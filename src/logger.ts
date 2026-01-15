/**
 * Logger Module - Singleton Pattern + EventEmitter
 *
 * Environment variables:
 *   LOG_FILE - Log output file path (default: /tmp/redmine-mcp.log)
 *   LOG_LEVEL - Log level: debug, info, warn, error (default: info)
 */

import * as fs from "fs";
import { EventEmitter } from "events";

export type LogLevel = "debug" | "info" | "warn" | "error";

export interface LogEntry {
  timestamp: string;
  level: LogLevel;
  message: string;
  tool?: string;
  duration_ms?: number;
  raw: string;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

class Logger extends EventEmitter {
  private static instance: Logger | null = null;
  private logFile: string;
  private logLevel: LogLevel;

  private constructor() {
    super();
    this.logFile = process.env.LOG_FILE || "/tmp/redmine-mcp.log";
    this.logLevel = (process.env.LOG_LEVEL as LogLevel) || "info";
  }

  static getInstance(): Logger {
    if (!Logger.instance) {
      Logger.instance = new Logger();
    }
    return Logger.instance;
  }

  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVELS[level] >= LOG_LEVELS[this.logLevel];
  }

  private write(level: LogLevel, msg: string): void {
    if (!this.shouldLog(level)) return;

    const timestamp = new Date().toISOString();
    const levelTag = level.toUpperCase().padEnd(5);
    const raw = `[${timestamp}] [${levelTag}] ${msg}`;

    // Write to file
    fs.appendFileSync(this.logFile, raw + "\n");
    console.error(`[${levelTag}] ${msg}`);

    // Parse tool name and execution time
    const entry = this.parseLogEntry(timestamp, level, msg, raw);

    // Emit event to WebSocket subscribers
    this.emit("log", entry);
  }

  private parseLogEntry(timestamp: string, level: LogLevel, msg: string, raw: string): LogEntry {
    const entry: LogEntry = { timestamp, level, message: msg, raw };

    // Parse [Request] tool_name {...}
    const requestMatch = msg.match(/^\[Request\]\s+(\w+)/);
    if (requestMatch) {
      entry.tool = requestMatch[1];
    }

    // Parse [Response] tool_name success (123ms)
    const responseMatch = msg.match(/^\[Response\]\s+(\w+)\s+success\s+\((\d+)ms\)/);
    if (responseMatch) {
      entry.tool = responseMatch[1];
      entry.duration_ms = parseInt(responseMatch[2], 10);
    }

    // Parse [Error] tool_name failed: ... (123ms)
    const errorMatch = msg.match(/^\[Error\]\s+(\w+)\s+failed:.*\((\d+)ms\)/);
    if (errorMatch) {
      entry.tool = errorMatch[1];
      entry.duration_ms = parseInt(errorMatch[2], 10);
    }

    return entry;
  }

  debug(msg: string): void {
    this.write("debug", msg);
  }

  info(msg: string): void {
    this.write("info", msg);
  }

  warn(msg: string): void {
    this.write("warn", msg);
  }

  error(msg: string): void {
    this.write("error", msg);
  }

  /** Get current log file path */
  getLogFile(): string {
    return this.logFile;
  }

  /** Get current log level */
  getLogLevel(): LogLevel {
    return this.logLevel;
  }
}

export const logger = Logger.getInstance();

export const log = {
  debug: logger.debug.bind(logger),
  info: logger.info.bind(logger),
  warn: logger.warn.bind(logger),
  error: logger.error.bind(logger),
};
