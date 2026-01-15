/**
 * Log Viewer WebSocket Server
 *
 * Environment variables:
 *   LOG_VIEWER - Enable/disable (default: true)
 *   LOG_VIEWER_PORT - Server port (default: 3456)
 *   LOG_VIEWER_OPEN - Auto open browser (default: true)
 *   LOG_VIEWER_HISTORY - Keep last N log entries (default: 500)
 */

import { logger, type LogEntry } from "../logger.js";
import { LOG_VIEWER_HTML } from "./ui.js";

const CONFIG = {
  enabled: process.env.LOG_VIEWER !== "false",
  basePort: parseInt(process.env.LOG_VIEWER_PORT || "3456", 10),
  autoOpen: process.env.LOG_VIEWER_OPEN !== "false" && !process.env.CI,
  historySize: parseInt(process.env.LOG_VIEWER_HISTORY || "500", 10),
  maxPortRetries: 10,
};

let currentPort: number | null = null;
let serverInstance: ReturnType<typeof Bun.serve> | null = null;
const logHistory: LogEntry[] = [];

function addToHistory(entry: LogEntry): void {
  logHistory.push(entry);
  if (logHistory.length > CONFIG.historySize) {
    logHistory.shift();
  }
}

function sanitize(entry: LogEntry): LogEntry {
  return {
    ...entry,
    message: entry.message.replace(/([a-f0-9]{20,})/gi, "[REDACTED]"),
    raw: entry.raw.replace(/([a-f0-9]{20,})/gi, "[REDACTED]"),
  };
}

function isValidHost(req: Request): boolean {
  const host = req.headers.get("host") || "";
  const hostname = host.split(":")[0];
  return ["127.0.0.1", "localhost"].includes(hostname);
}

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

        if (url.pathname === "/ws") {
          const upgraded = server.upgrade(req, { data: {} });
          if (upgraded) return undefined;
          return new Response("WebSocket upgrade failed", { status: 400 });
        }

        if (url.pathname === "/" || url.pathname === "/index.html") {
          return new Response(LOG_VIEWER_HTML, {
            headers: {
              "Content-Type": "text/html; charset=utf-8",
              "Access-Control-Allow-Origin": "null",
            },
          });
        }

        if (url.pathname === "/api/history") {
          return new Response(JSON.stringify(logHistory.map(sanitize)), {
            headers: {
              "Content-Type": "application/json",
              "Access-Control-Allow-Origin": "null",
            },
          });
        }

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

export function getLogViewerUrl(): string | null {
  return currentPort ? `http://localhost:${currentPort}` : null;
}

export function getLogViewerPort(): number | null {
  return currentPort;
}

export function isLogViewerRunning(): boolean {
  return serverInstance !== null;
}

export function startLogViewer(): void {
  if (serverInstance) return;

  if (!CONFIG.enabled) {
    console.error("[LOG-VIEWER] Disabled (LOG_VIEWER=false)");
    return;
  }

  for (let i = 0; i < CONFIG.maxPortRetries; i++) {
    const port = CONFIG.basePort + i;
    const server = tryStartServer(port);

    if (server) {
      serverInstance = server;
      currentPort = port;

      logger.on("log", (entry: LogEntry) => {
        addToHistory(entry);
        server.publish("logs", JSON.stringify(sanitize(entry)));
      });

      const url = getLogViewerUrl()!;
      console.error(`[LOG-VIEWER] Running at ${url}`);

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
    `[LOG-VIEWER] Failed to start: ports ${CONFIG.basePort}-${CONFIG.basePort + CONFIG.maxPortRetries - 1} are all in use`
  );
}
