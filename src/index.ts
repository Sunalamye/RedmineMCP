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

import { log } from "./logger.js";
import { startLogViewer } from "./log-viewer/server.js";

log.info("=== MCP Server Starting ===");
log.debug(`Working directory: ${process.cwd()}`);
log.debug(`Node version: ${process.version}`);
log.debug(`Start arguments: ${process.argv.join(" ")}`);

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { RedmineClient } from "./redmine-client.js";
import { TOOLS } from "./tools/definitions.js";
import { createToolHandler } from "./tools/handlers.js";

log.info("Modules loaded");

const REDMINE_URL = process.env.REDMINE_URL;
const REDMINE_TOKEN = process.env.REDMINE_TOKEN;

log.debug(`REDMINE_URL: ${REDMINE_URL || "(not set)"}`);
log.debug(`REDMINE_TOKEN: ${REDMINE_TOKEN ? "(set)" : "(not set)"}`);

if (!REDMINE_URL || !REDMINE_TOKEN) {
  log.error("Missing required environment variables: REDMINE_URL, REDMINE_TOKEN");
  console.error("Missing required environment variables:");
  console.error("  REDMINE_URL, REDMINE_TOKEN");
  process.exit(1);
}

log.info("Environment variables validated");

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
  log.info("Starting main process...");

  // Start Log Viewer (before other initializations)
  startLogViewer();

  log.info("Testing Redmine connection...");
  const loggedIn = await redmineClient.login();
  log.info(loggedIn ? "Redmine connection successful" : "Initial connection test failed");

  const transport = new StdioServerTransport();
  await server.connect(transport);
  log.info("Server ready, waiting for requests");
}

log.info("Calling main...");
main().catch((error) => {
  log.error(`Fatal error: ${error}`);
  console.error("[RedmineMCP] Fatal error:", error);
  process.exit(1);
});
