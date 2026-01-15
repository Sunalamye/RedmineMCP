/**
 * Log Viewer WebSocket Server
 *
 * 環境變數:
 *   LOG_VIEWER - 啟用/停用 (預設: true)
 *   LOG_VIEWER_PORT - 伺服器端口 (預設: 3456)
 *   LOG_VIEWER_OPEN - 自動開啟瀏覽器 (預設: true)
 *   LOG_VIEWER_HISTORY - 保留最近 N 筆日誌 (預設: 500)
 */

import { logger, type LogEntry } from "../logger.js";
import { LOG_VIEWER_HTML } from "./ui.js";

// 配置
const CONFIG = {
  enabled: process.env.LOG_VIEWER !== "false",
  basePort: parseInt(process.env.LOG_VIEWER_PORT || "3456", 10),
  autoOpen: process.env.LOG_VIEWER_OPEN !== "false" && !process.env.CI,
  historySize: parseInt(process.env.LOG_VIEWER_HISTORY || "500", 10),
  maxPortRetries: 10,
};

// 運行時狀態
let currentPort: number | null = null;
let serverInstance: ReturnType<typeof Bun.serve> | null = null;

// 日誌歷史 (環形緩衝區)
const logHistory: LogEntry[] = [];

function addToHistory(entry: LogEntry): void {
  logHistory.push(entry);
  if (logHistory.length > CONFIG.historySize) {
    logHistory.shift();
  }
}

// 脫敏處理 - 移除可能的 API token
function sanitize(entry: LogEntry): LogEntry {
  return {
    ...entry,
    message: entry.message.replace(/([a-f0-9]{20,})/gi, "[REDACTED]"),
    raw: entry.raw.replace(/([a-f0-9]{20,})/gi, "[REDACTED]"),
  };
}

// 驗證 Host header (防止 DNS rebinding 攻擊)
function isValidHost(req: Request): boolean {
  const host = req.headers.get("host") || "";
  const hostname = host.split(":")[0];
  return ["127.0.0.1", "localhost"].includes(hostname);
}

// 嘗試在指定端口啟動伺服器
function tryStartServer(port: number): ReturnType<typeof Bun.serve> | null {
  try {
    return Bun.serve({
      hostname: "127.0.0.1",
      port,

      fetch(req, server) {
        if (!isValidHost(req)) {
          return new Response("Forbidden", { status: 403 });
        }

        const url = new URL(req.url);

        // WebSocket 升級
        if (url.pathname === "/ws") {
          const upgraded = server.upgrade(req, { data: {} });
          if (upgraded) return undefined;
          return new Response("WebSocket upgrade failed", { status: 400 });
        }

        // 主頁面
        if (url.pathname === "/" || url.pathname === "/index.html") {
          return new Response(LOG_VIEWER_HTML, {
            headers: {
              "Content-Type": "text/html; charset=utf-8",
              "Access-Control-Allow-Origin": "null",
            },
          });
        }

        // API: 取得歷史日誌
        if (url.pathname === "/api/history") {
          return new Response(JSON.stringify(logHistory.map(sanitize)), {
            headers: {
              "Content-Type": "application/json",
              "Access-Control-Allow-Origin": "null",
            },
          });
        }

        // API: 取得伺服器資訊
        if (url.pathname === "/api/info") {
          return new Response(
            JSON.stringify({
              port: currentPort,
              url: getLogViewerUrl(),
              entries: logHistory.length,
              maxHistory: CONFIG.historySize,
            }),
            {
              headers: {
                "Content-Type": "application/json",
                "Access-Control-Allow-Origin": "null",
              },
            }
          );
        }

        return new Response("Not Found", { status: 404 });
      },

      websocket: {
        open(ws) {
          ws.subscribe("logs");
          for (const entry of logHistory) {
            ws.send(JSON.stringify(sanitize(entry)));
          }
        },
        close(ws) {
          ws.unsubscribe("logs");
        },
        message() {},
      },
    });
  } catch {
    return null;
  }
}

/** 取得 Log Viewer URL (如果已啟動) */
export function getLogViewerUrl(): string | null {
  if (!currentPort) return null;
  return `http://localhost:${currentPort}`;
}

/** 取得 Log Viewer 端口 (如果已啟動) */
export function getLogViewerPort(): number | null {
  return currentPort;
}

/** 檢查 Log Viewer 是否正在運行 */
export function isLogViewerRunning(): boolean {
  return serverInstance !== null;
}

/** 啟動 Log Viewer */
export function startLogViewer(): void {
  if (!CONFIG.enabled) {
    console.error("[LOG-VIEWER] 已停用 (LOG_VIEWER=false)");
    return;
  }

  // 嘗試多個端口
  for (let i = 0; i < CONFIG.maxPortRetries; i++) {
    const port = CONFIG.basePort + i;
    const server = tryStartServer(port);

    if (server) {
      serverInstance = server;
      currentPort = port;

      // 訂閱 Logger 事件
      logger.on("log", (entry: LogEntry) => {
        addToHistory(entry);
        server.publish("logs", JSON.stringify(sanitize(entry)));
      });

      const url = getLogViewerUrl()!;
      console.error(`[LOG-VIEWER] 運行於 ${url}`);

      // 自動開啟瀏覽器
      if (CONFIG.autoOpen) {
        const cmd =
          process.platform === "darwin"
            ? ["open", url]
            : process.platform === "win32"
              ? ["cmd", "/c", "start", url]
              : ["xdg-open", url];

        Bun.spawn(cmd, { stdout: "ignore", stderr: "ignore" });
      }

      return;
    }
  }

  console.error(
    `[LOG-VIEWER] 啟動失敗: 端口 ${CONFIG.basePort}-${CONFIG.basePort + CONFIG.maxPortRetries - 1} 都被佔用`
  );
}
