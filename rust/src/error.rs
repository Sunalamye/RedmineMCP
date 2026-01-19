//! 錯誤類型定義

use thiserror::Error;

/// Redmine API 錯誤類型
#[derive(Error, Debug)]
pub enum RedmineError {
    /// HTTP 請求錯誤
    #[error("HTTP 錯誤: {0}")]
    Http(#[from] reqwest::Error),

    /// 認證失敗 (401)
    #[error("認證失敗: API Key 無效")]
    Unauthorized,

    /// 權限不足 (403)
    #[error("權限不足")]
    Forbidden,

    /// 資源不存在 (404)
    #[error("資源不存在: {resource} (id: {id})")]
    NotFound { resource: String, id: String },

    /// 驗證錯誤 (422)
    #[error("驗證錯誤: {message}")]
    ValidationError { message: String, errors: Vec<String> },

    /// 伺服器錯誤 (5xx)
    #[error("伺服器錯誤 ({status}): {message}")]
    ServerError { status: u16, message: String },

    /// JSON 解析錯誤
    #[error("JSON 解析錯誤: {0}")]
    Json(#[from] serde_json::Error),

    /// URL 解析錯誤
    #[error("URL 錯誤: {0}")]
    Url(#[from] url::ParseError),

    /// IO 錯誤
    #[error("IO 錯誤: {0}")]
    Io(#[from] std::io::Error),

    /// 檔案不存在
    #[error("檔案不存在: {path}")]
    FileNotFound { path: String },

    /// 缺少必要參數
    #[error("缺少必要參數: {0}")]
    MissingParam(String),

    /// 未知工具
    #[error("未知的工具: {0}")]
    UnknownTool(String),

    /// 設定錯誤
    #[error("設定錯誤: {0}")]
    Config(String),
}

/// Result 類型別名
pub type Result<T> = std::result::Result<T, RedmineError>;

impl RedmineError {
    /// 從 HTTP 回應建構錯誤
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();

        match status {
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound {
                resource: "Resource".into(),
                id: "unknown".into(),
            },
            422 => Self::ValidationError {
                message: body.clone(),
                errors: Self::parse_validation_errors(&body),
            },
            500..=599 => Self::ServerError {
                status,
                message: body,
            },
            _ => Self::ServerError {
                status,
                message: format!("Unexpected status: {body}"),
            },
        }
    }

    /// 解析 Redmine 驗證錯誤
    fn parse_validation_errors(body: &str) -> Vec<String> {
        serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|v| v.get("errors")?.as_array().cloned())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    }
}
