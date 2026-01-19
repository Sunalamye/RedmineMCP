//! Log Viewer HTTP/WebSocket Server
//!
//! 環境變數:
//!   LOG_VIEWER      - 啟用/停用 (預設: true)
//!   LOG_VIEWER_PORT - 伺服器埠號 (預設: 3456)
//!   LOG_VIEWER_OPEN - 自動開啟瀏覽器 (預設: true)

mod ui;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

/// 日誌項目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub tool: Option<String>,
    pub message: String,
    pub duration_ms: Option<u64>,
    pub raw: String,
}

/// Log Viewer 配置
#[derive(Debug, Clone)]
pub struct LogViewerConfig {
    pub enabled: bool,
    pub port: u16,
    pub auto_open: bool,
    pub max_port_retries: u16,
}

impl Default for LogViewerConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("LOG_VIEWER")
                .map(|v| v != "false")
                .unwrap_or(true),
            port: std::env::var("LOG_VIEWER_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3456),
            auto_open: std::env::var("LOG_VIEWER_OPEN")
                .map(|v| v != "false")
                .unwrap_or(true)
                && std::env::var("CI").is_err(),
            max_port_retries: 10,
        }
    }
}

/// Log Viewer 共享狀態
pub struct LogViewerState {
    history: RwLock<Vec<LogEntry>>,
    tx: broadcast::Sender<LogEntry>,
    port: u16,
}

impl LogViewerState {
    fn new(port: u16) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            history: RwLock::new(Vec::with_capacity(500)),
            tx,
            port,
        }
    }

    /// 添加日誌
    pub fn add_log(&self, entry: LogEntry) {
        {
            let mut history = self.history.write();
            history.push(entry.clone());
            if history.len() > 500 {
                history.remove(0);
            }
        }
        let _ = self.tx.send(entry);
    }

    /// 取得歷史記錄
    fn get_history(&self) -> Vec<LogEntry> {
        self.history.read().clone()
    }

    /// 訂閱日誌
    fn subscribe(&self) -> broadcast::Receiver<LogEntry> {
        self.tx.subscribe()
    }
}

/// 全域 Log Viewer 狀態
static LOG_VIEWER: std::sync::OnceLock<Arc<LogViewerState>> = std::sync::OnceLock::new();

/// 取得 Log Viewer URL
pub fn get_log_viewer_url() -> Option<String> {
    LOG_VIEWER.get().map(|s| format!("http://localhost:{}", s.port))
}

/// 記錄日誌到 Log Viewer
pub fn log_to_viewer(
    level: &str,
    tool: Option<&str>,
    message: &str,
    duration_ms: Option<u64>,
) {
    if let Some(state) = LOG_VIEWER.get() {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            level: level.to_string(),
            tool: tool.map(|s| s.to_string()),
            message: sanitize_message(message),
            duration_ms,
            raw: message.to_string(),
        };
        state.add_log(entry);
    }
}

/// 隱藏敏感資訊
fn sanitize_message(msg: &str) -> String {
    // 隱藏長度 >= 20 的十六進位字串 (可能是 token)
    let re = regex::Regex::new(r"[a-f0-9]{20,}").unwrap_or_else(|_| {
        regex::Regex::new(r"^$").unwrap()
    });
    re.replace_all(msg, "[REDACTED]").to_string()
}

/// 啟動 Log Viewer
pub async fn start_log_viewer() -> Option<u16> {
    let config = LogViewerConfig::default();

    if !config.enabled {
        info!("[LOG-VIEWER] 已停用 (LOG_VIEWER=false)");
        return None;
    }

    for i in 0..config.max_port_retries {
        let port = config.port + i;
        if try_start_server(port, config.auto_open).await {
            return Some(port);
        }
    }

    error!(
        "[LOG-VIEWER] 啟動失敗: 埠號 {}-{} 皆被占用",
        config.port,
        config.port + config.max_port_retries - 1
    );
    None
}

async fn try_start_server(port: u16, auto_open: bool) -> bool {
    let state = Arc::new(LogViewerState::new(port));

    // 設置全域狀態
    if LOG_VIEWER.set(state.clone()).is_err() {
        return false;
    }

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/api/history", get(history_handler))
        .route("/api/info", get(info_handler))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(_) => return false,
    };

    let url = format!("http://localhost:{}", port);
    info!("[LOG-VIEWER] 運行於 {}", url);

    // 自動開啟瀏覽器
    if auto_open {
        let _ = open::that(&url);
    }

    // 在背景執行伺服器
    tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            error!("[LOG-VIEWER] 伺服器錯誤: {}", e);
        }
    });

    true
}

// HTTP Handlers

async fn index_handler() -> impl IntoResponse {
    Html(ui::LOG_VIEWER_HTML)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<LogViewerState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: Arc<LogViewerState>) {
    // 發送歷史記錄
    for entry in state.get_history() {
        if let Ok(json) = serde_json::to_string(&entry) {
            if socket.send(Message::Text(json.into())).await.is_err() {
                return;
            }
        }
    }

    // 訂閱新日誌
    let mut rx = state.subscribe();

    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(entry) => {
                        if let Ok(json) = serde_json::to_string(&entry) {
                            if socket.send(Message::Text(json.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
        }
    }
}

async fn history_handler(
    State(state): State<Arc<LogViewerState>>,
) -> impl IntoResponse {
    Json(state.get_history())
}

async fn info_handler(
    State(state): State<Arc<LogViewerState>>,
) -> impl IntoResponse {
    let history = state.history.read();
    Json(serde_json::json!({
        "port": state.port,
        "url": format!("http://localhost:{}", state.port),
        "entries": history.len(),
        "maxHistory": 500
    }))
}
