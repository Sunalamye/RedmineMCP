//! Redmine MCP Server
//!
//! 環境變數:
//!   REDMINE_URL      - Redmine 網址 (必填)
//!   REDMINE_TOKEN    - API Token (必填)
//!   LOG_FILE         - 日誌檔案路徑 (預設: /tmp/redmine-mcp.log)
//!   LOG_LEVEL        - 日誌等級: debug, info, warn, error (預設: info)
//!   LOG_VIEWER       - 啟用 Log Viewer (預設: true)
//!   LOG_VIEWER_PORT  - Log Viewer 埠號 (預設: 3456)
//!   LOG_VIEWER_OPEN  - 自動開啟瀏覽器 (預設: true)

use redmine_mcp::{Config, RedmineClient, RedmineMcpServer, start_log_viewer, get_log_viewer_url};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use tracing::{debug, error, info};

fn main() -> anyhow::Result<()> {
    // 初始化日誌 (輸出到 stderr)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    info!("=== MCP 伺服器啟動中 ===");
    debug!("工作目錄: {:?}", std::env::current_dir()?);

    // 載入配置
    let config = Config::from_env()?;
    debug!("REDMINE_URL: {}", config.redmine_url);
    debug!("REDMINE_TOKEN: (已設定)");

    info!("環境變數檢查通過");

    // 建立 Redmine 客戶端
    let client = RedmineClient::new(&config.redmine_url, &config.redmine_token)?;

    // 建立 MCP Server
    let server = RedmineMcpServer::new(client);

    // 使用 tokio runtime 執行
    let rt = tokio::runtime::Runtime::new()?;

    // 啟動 Log Viewer
    rt.block_on(start_log_viewer());
    if let Some(url) = get_log_viewer_url() {
        info!("Log Viewer: {}", url);
    }

    // 測試連線
    info!("測試 Redmine 連線...");
    let login_result = rt.block_on(async {
        server.call_tool("redmine_get_current_user", None).await
    });
    info!(
        "Redmine 連線{}",
        if login_result.is_error { "失敗" } else { "成功" }
    );

    info!("伺服器已就緒，等待請求中");

    // 主迴圈：處理 JSON-RPC 請求
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("讀取輸入錯誤: {}", e);
                continue;
            }
        };

        if line.is_empty() {
            continue;
        }

        debug!("收到請求: {}", line);

        let request: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                error!("JSON 解析錯誤: {}", e);
                let error_response = json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                });
                writeln!(stdout, "{}", error_response)?;
                stdout.flush()?;
                continue;
            }
        };

        let id = request.get("id").cloned();
        let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("");

        let response = match method {
            "initialize" => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "redmine-mcp",
                            "version": env!("CARGO_PKG_VERSION")
                        }
                    }
                })
            }
            "notifications/initialized" | "initialized" => {
                // 不需要回應
                continue;
            }
            "tools/list" => {
                let tools = redmine_mcp::tools::get_tool_definitions();
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "tools": tools
                    }
                })
            }
            "tools/call" => {
                let params = request.get("params");
                let tool_name = params
                    .and_then(|p| p.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let tool_args = params.and_then(|p| p.get("arguments")).cloned();

                let result = rt.block_on(server.call_tool(tool_name, tool_args));

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": result.to_json()
                })
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                })
            }
        };

        debug!("回應: {}", response);
        writeln!(stdout, "{}", response)?;
        stdout.flush()?;
    }

    Ok(())
}
