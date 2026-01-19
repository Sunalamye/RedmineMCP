//! 環境變數配置

use crate::error::{RedmineError, Result};
use std::env;
use tracing::Level;

/// 應用程式配置
#[derive(Debug, Clone)]
pub struct Config {
    /// Redmine 網址
    pub redmine_url: String,
    /// Redmine API Token
    pub redmine_token: String,
    /// 日誌檔案路徑
    pub log_file: String,
    /// 日誌等級
    pub log_level: Level,
}

impl Config {
    /// 從環境變數載入配置
    pub fn from_env() -> Result<Self> {
        let redmine_url = env::var("REDMINE_URL")
            .map_err(|_| RedmineError::Config("缺少環境變數: REDMINE_URL".into()))?;

        let redmine_token = env::var("REDMINE_TOKEN")
            .map_err(|_| RedmineError::Config("缺少環境變數: REDMINE_TOKEN".into()))?;

        let log_file = env::var("LOG_FILE")
            .unwrap_or_else(|_| "/tmp/mitake-redmine-mcp.log".into());

        let log_level = env::var("LOG_LEVEL")
            .map(|s| Self::parse_log_level(&s))
            .unwrap_or(Level::INFO);

        Ok(Self {
            redmine_url: redmine_url.trim_end_matches('/').to_string(),
            redmine_token,
            log_file,
            log_level,
        })
    }

    /// 解析日誌等級
    fn parse_log_level(s: &str) -> Level {
        match s.to_lowercase().as_str() {
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" | "warning" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        }
    }
}
