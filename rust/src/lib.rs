//! Redmine MCP Server
//!
//! MCP Server for Redmine API integration, providing 34 tools.

pub mod config;
pub mod error;
pub mod client;
pub mod tools;
pub mod log_viewer;

pub use config::Config;
pub use error::{RedmineError, Result};
pub use client::RedmineClient;
pub use tools::RedmineMcpServer;
pub use log_viewer::{start_log_viewer, get_log_viewer_url, log_to_viewer};
