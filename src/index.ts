#!/usr/bin/env bun
/**
 * Redmine MCP Server
 *
 * 環境變數:
 *   REDMINE_URL   - Redmine 網址 (必填)
 *   REDMINE_TOKEN - API Token (必填)
 *   LOG_FILE      - 日誌檔案路徑 (預設: /tmp/redmine-mcp.log)
 *   LOG_LEVEL     - 日誌等級: debug, info, warn, error (預設: info)
 *   LOG_VIEWER    - 啟用 Log Viewer (預設: true)
 *   LOG_VIEWER_PORT - Log Viewer 端口 (預設: 3456)
 *   LOG_VIEWER_OPEN - 自動開啟瀏覽器 (預設: true)
 */

import { log } from "./logger.js";
import { startLogViewer } from "./log-viewer/server.js";

log.info("=== MCP 伺服器啟動中 ===");
log.debug(`工作目錄: ${process.cwd()}`);
log.debug(`Node 版本: ${process.version}`);
log.debug(`啟動參數: ${process.argv.join(" ")}`);

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { RedmineClient } from "./redmine-client.js";
import { TOOLS } from "./tools/definitions.js";
import { createToolHandler } from "./tools/handlers.js";

log.info("模組載入完成");

const REDMINE_URL = process.env.REDMINE_URL;
const REDMINE_TOKEN = process.env.REDMINE_TOKEN;

log.debug(`REDMINE_URL: ${REDMINE_URL || "(未設定)"}`);
log.debug(`REDMINE_TOKEN: ${REDMINE_TOKEN ? "(已設定)" : "(未設定)"}`);

if (!REDMINE_URL || !REDMINE_TOKEN) {
  log.error("缺少必要環境變數: REDMINE_URL, REDMINE_TOKEN");
  console.error("缺少必要環境變數:");
  console.error("  REDMINE_URL, REDMINE_TOKEN");
  process.exit(1);
}

log.info("環境變數檢查通過");

const redmineClient = new RedmineClient({
  baseUrl: REDMINE_URL,
  apiKey: REDMINE_TOKEN,
});

const handleTool = createToolHandler(redmineClient);

const server = new Server(
  {
    name: "redmine-mcp",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return { tools: TOOLS };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  return handleTool(name, args as Record<string, unknown> | undefined);
});

async function main(): Promise<void> {
  log.info("正在啟動主程序...");

  // 啟動 Log Viewer (在其他初始化之前)
  startLogViewer();

  log.info("測試 Redmine 連線...");
  const loggedIn = await redmineClient.login();
  log.info(loggedIn ? "Redmine 連線成功" : "初始連線測試失敗");

  const transport = new StdioServerTransport();
  await server.connect(transport);
  log.info("伺服器已就緒，等待請求中");
}

log.info("呼叫主程序...");
main().catch((error) => {
  log.error(`致命錯誤: ${error}`);
  console.error("[RedmineMCP] 致命錯誤:", error);
  process.exit(1);
});
