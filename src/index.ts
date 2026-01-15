#!/usr/bin/env bun
/**
 * Redmine MCP Server
 *
 * Environment variables:
 *   REDMINE_URL   - Redmine URL (required)
 *   REDMINE_TOKEN - API Token (required)
 *   LOG_FILE      - Log file path (default: /tmp/redmine-mcp.log)
 *   LOG_LEVEL     - Log level: debug, info, warn, error (default: info)
 *   LOG_VIEWER    - Enable Log Viewer (default: true)
 *   LOG_VIEWER_PORT - Log Viewer port (default: 3456)
 *   LOG_VIEWER_OPEN - Auto open browser (default: true)
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { log } from "./logger.js";
import { startLogViewer } from "./log-viewer/server.js";
import { RedmineClient } from "./redmine-client.js";
import { TOOLS } from "./tools/definitions.js";
import { createToolHandler } from "./tools/handlers.js";

const REDMINE_URL = process.env.REDMINE_URL;
const REDMINE_TOKEN = process.env.REDMINE_TOKEN;

if (!REDMINE_URL || !REDMINE_TOKEN) {
  console.error("Missing required environment variables: REDMINE_URL, REDMINE_TOKEN");
  process.exit(1);
}

const redmineClient = new RedmineClient({
  baseUrl: REDMINE_URL,
  apiKey: REDMINE_TOKEN,
});

const handleTool = createToolHandler(redmineClient);

const server = new Server(
  { name: "redmine-mcp", version: "1.0.0" },
  { capabilities: { tools: {} } }
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({ tools: TOOLS }));

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  return handleTool(name, args as Record<string, unknown> | undefined);
});

async function main(): Promise<void> {
  startLogViewer();

  const connected = await redmineClient.login();
  log.info(connected ? "Redmine connection successful" : "Redmine connection failed");

  const transport = new StdioServerTransport();
  await server.connect(transport);
  log.info("MCP Server ready");
}

main().catch((error) => {
  console.error("[RedmineMCP] Fatal error:", error);
  process.exit(1);
});
