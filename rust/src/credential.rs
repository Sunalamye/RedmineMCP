//! 憑證管理 — 多 Profile 支援
//!
//! 全域：~/.config/redmine/credentials.toml（含 token，0o600）
//! 本地：.redmine（只含 profile 名稱，可 commit）

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ============ Global Credentials ============

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CredentialFile {
    #[serde(default)]
    pub default_profile: Option<String>,
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub url: String,
    pub token: String,
}

impl CredentialFile {
    pub fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("redmine").join("credentials.toml"))
    }

    pub fn load() -> Option<Self> {
        let path = Self::path()?;
        let content = std::fs::read_to_string(&path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::path().ok_or_else(|| anyhow::anyhow!("無法取得設定目錄"))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }
        Ok(())
    }

    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    pub fn get_default(&self) -> Option<(&str, &Profile)> {
        let name = self.default_profile.as_deref()?;
        self.profiles.get(name).map(|p| (name, p))
    }

    pub fn remove() -> anyhow::Result<()> {
        if let Some(path) = Self::path() {
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }
        Ok(())
    }
}

// ============ Local Config (.redmine) ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    pub profile: String,
}

impl LocalConfig {
    /// 從 CWD 向上搜尋 .redmine（類似 .git 搜尋）
    pub fn find() -> Option<(PathBuf, Self)> {
        let mut current = std::env::current_dir().ok()?;
        loop {
            let config_path = current.join(".redmine");
            if config_path.exists() {
                let content = std::fs::read_to_string(&config_path).ok()?;
                let config: Self = toml::from_str(&content).ok()?;
                return Some((config_path, config));
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    pub fn save_to_cwd(&self) -> anyhow::Result<PathBuf> {
        let path = std::env::current_dir()?.join(".redmine");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;
        Ok(path)
    }

    pub fn remove_from_cwd() -> anyhow::Result<bool> {
        let path = std::env::current_dir()?.join(".redmine");
        if path.exists() {
            std::fs::remove_file(&path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// ============ Credential Resolution ============

#[derive(Debug, Clone)]
pub struct ResolvedCredential {
    pub url: String,
    pub token: String,
    pub source: CredentialSource,
}

#[derive(Debug, Clone)]
pub enum CredentialSource {
    CliFlags,
    EnvVars,
    LocalProfile { profile_name: String, config_path: PathBuf },
    GlobalDefault { profile_name: String },
    GlobalProfile { profile_name: String },
}

impl std::fmt::Display for CredentialSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CliFlags => write!(f, "CLI flags"),
            Self::EnvVars => write!(f, "環境變數"),
            Self::LocalProfile { profile_name, config_path } => {
                write!(f, "local .redmine → profile '{profile_name}' ({})", config_path.display())
            }
            Self::GlobalDefault { profile_name } => {
                write!(f, "global default → profile '{profile_name}'")
            }
            Self::GlobalProfile { profile_name } => {
                write!(f, "--profile '{profile_name}'")
            }
        }
    }
}
