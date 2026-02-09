//! Redmine MCP Server & CLI
//!
//! MCP Server for Redmine API integration, providing 35 tools.
//! CLI mode for direct terminal access (like gh/glab).

pub mod config;
pub mod credential;
pub mod error;
pub mod client;
pub mod tools;
pub mod log_viewer;
pub mod cli;

pub use config::Config;
pub use error::{RedmineError, Result};
pub use client::RedmineClient;
pub use tools::RedmineMcpServer;
pub use log_viewer::{start_log_viewer, get_log_viewer_url, log_to_viewer};
