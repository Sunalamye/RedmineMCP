/**
 * Logger 模組 - Singleton Pattern + EventEmitter
 *
 * 環境變數:
 *   LOG_FILE - 日誌輸出檔案路徑 (預設: /tmp/redmine-mcp.log)
 *   LOG_LEVEL - 日誌等級: debug, info, warn, error (預設: info)
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

    // 寫入檔案
    fs.appendFileSync(this.logFile, raw + "\n");
    console.error(`[${levelTag}] ${msg}`);

    // 解析工具名稱和執行時間
    const entry = this.parseLogEntry(timestamp, level, msg, raw);

    // 發送事件給 WebSocket 訂閱者
    this.emit("log", entry);
  }

  private parseLogEntry(timestamp: string, level: LogLevel, msg: string, raw: string): LogEntry {
    const entry: LogEntry = { timestamp, level, message: msg, raw };

    // 解析 [請求] tool_name {...}
    const requestMatch = msg.match(/^\[請求\]\s+(\w+)/);
    if (requestMatch) {
      entry.tool = requestMatch[1];
    }

    // 解析 [回應] tool_name 成功 (123ms)
    const responseMatch = msg.match(/^\[回應\]\s+(\w+)\s+成功\s+\((\d+)ms\)/);
    if (responseMatch) {
      entry.tool = responseMatch[1];
      entry.duration_ms = parseInt(responseMatch[2], 10);
    }

    // 解析 [錯誤] tool_name 失敗: ... (123ms)
    const errorMatch = msg.match(/^\[錯誤\]\s+(\w+)\s+失敗:.*\((\d+)ms\)/);
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

  /** 取得目前日誌檔案路徑 */
  getLogFile(): string {
    return this.logFile;
  }

  /** 取得目前日誌等級 */
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
