//! 環境變數配置

use crate::credential::{CredentialFile, CredentialSource, LocalConfig, ResolvedCredential};
use crate::error::{RedmineError, Result};
use std::env;
use tracing::Level;

const DEFAULT_LOG_FILE: &str = "/tmp/redmine-mcp.log";

/// 應用程式配置
#[derive(Debug, Clone)]
pub struct Config {
    pub redmine_url: String,
    pub redmine_token: String,
    pub log_file: String,
    pub log_level: Level,
}

impl Config {
    /// 從環境變數載入配置（MCP 模式用）
    pub fn from_env() -> Result<Self> {
        let redmine_url = env::var("REDMINE_URL")
            .map_err(|_| RedmineError::Config("缺少環境變數: REDMINE_URL".into()))?;
        let redmine_token = env::var("REDMINE_TOKEN")
            .map_err(|_| RedmineError::Config("缺少環境變數: REDMINE_TOKEN".into()))?;

        Ok(Self {
            redmine_url: redmine_url.trim_end_matches('/').to_string(),
            redmine_token,
            log_file: Self::default_log_file(),
            log_level: Self::default_log_level(),
        })
    }

    /// 分層解析配置（CLI 模式用）
    /// 優先序：CLI flags > env vars > --profile > .redmine > global default
    pub fn resolve(
        cli_url: Option<&str>,
        cli_token: Option<&str>,
        cli_profile: Option<&str>,
    ) -> Result<(Self, ResolvedCredential)> {
        // 1. CLI flags
        if let (Some(url), Some(token)) = (cli_url, cli_token) {
            let resolved = ResolvedCredential {
                url: url.to_string(),
                token: token.to_string(),
                source: CredentialSource::CliFlags,
            };
            return Ok((Self::from_resolved(&resolved), resolved));
        }

        // 2. Env vars
        if let (Ok(url), Ok(token)) = (env::var("REDMINE_URL"), env::var("REDMINE_TOKEN")) {
            let resolved = ResolvedCredential {
                url, token,
                source: CredentialSource::EnvVars,
            };
            return Ok((Self::from_resolved(&resolved), resolved));
        }

        let cred_file = CredentialFile::load();

        // 3. --profile flag
        if let Some(profile_name) = cli_profile {
            if let Some(profile) = cred_file.as_ref().and_then(|cf| cf.get_profile(profile_name)) {
                let resolved = ResolvedCredential {
                    url: profile.url.clone(),
                    token: profile.token.clone(),
                    source: CredentialSource::GlobalProfile {
                        profile_name: profile_name.to_string(),
                    },
                };
                return Ok((Self::from_resolved(&resolved), resolved));
            }
            return Err(RedmineError::Config(
                format!("Profile '{profile_name}' 不存在，請用 `redmine login -g --profile {profile_name}` 建立"),
            ));
        }

        // 4. Local .redmine
        if let Some((config_path, local_config)) = LocalConfig::find() {
            if let Some(profile) = cred_file.as_ref().and_then(|cf| cf.get_profile(&local_config.profile)) {
                let resolved = ResolvedCredential {
                    url: profile.url.clone(),
                    token: profile.token.clone(),
                    source: CredentialSource::LocalProfile {
                        profile_name: local_config.profile,
                        config_path,
                    },
                };
                return Ok((Self::from_resolved(&resolved), resolved));
            }
            return Err(RedmineError::Config(
                format!(".redmine 指向 profile '{}' 但不存在於 credentials.toml", local_config.profile),
            ));
        }

        // 5. Global default
        if let Some((name, profile)) = cred_file.as_ref().and_then(|cf| cf.get_default()) {
            let resolved = ResolvedCredential {
                url: profile.url.clone(),
                token: profile.token.clone(),
                source: CredentialSource::GlobalDefault {
                    profile_name: name.to_string(),
                },
            };
            return Ok((Self::from_resolved(&resolved), resolved));
        }

        Err(RedmineError::Config(
            "找不到憑證。請用 `redmine login --url URL --token TOKEN` 登入".into(),
        ))
    }

    fn from_resolved(resolved: &ResolvedCredential) -> Self {
        Self {
            redmine_url: resolved.url.trim_end_matches('/').to_string(),
            redmine_token: resolved.token.clone(),
            log_file: Self::default_log_file(),
            log_level: Self::default_log_level(),
        }
    }

    fn default_log_file() -> String {
        env::var("LOG_FILE").unwrap_or_else(|_| DEFAULT_LOG_FILE.into())
    }

    fn default_log_level() -> Level {
        env::var("LOG_LEVEL")
            .map(|s| Self::parse_log_level(&s))
            .unwrap_or(Level::INFO)
    }

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
