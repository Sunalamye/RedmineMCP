//! MCP 工具模組

mod params;

pub use params::*;

use crate::client::*;
use crate::error::Result;
use crate::RedmineClient;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{error, info};

/// MCP Server 工具處理結果
#[derive(Debug)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    pub is_error: bool,
}

/// 工具回應內容
#[derive(Debug)]
pub struct ToolContent {
    pub r#type: String,
    pub text: String,
}

impl ToolResult {
    /// 建立成功回應
    pub fn success(data: impl serde::Serialize) -> Self {
        Self {
            content: vec![ToolContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&data).unwrap_or_default(),
            }],
            is_error: false,
        }
    }

    /// 建立錯誤回應
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent {
                r#type: "text".to_string(),
                text: format!("錯誤: {}", message.into()),
            }],
            is_error: true,
        }
    }

    /// 轉換為 JSON
    pub fn to_json(&self) -> Value {
        let content: Vec<Value> = self
            .content
            .iter()
            .map(|c| json!({ "type": c.r#type, "text": c.text }))
            .collect();

        if self.is_error {
            json!({ "content": content, "isError": true })
        } else {
            json!({ "content": content })
        }
    }
}

/// Redmine MCP Server
pub struct RedmineMcpServer {
    client: Arc<RedmineClient>,
}

impl RedmineMcpServer {
    /// 建立新的 MCP Server
    pub fn new(client: RedmineClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    /// 取得工具列表
    pub fn list_tools(&self) -> Vec<Value> {
        TOOL_DEFINITIONS.to_vec()
    }

    /// 執行工具
    pub async fn call_tool(&self, name: &str, args: Option<Value>) -> ToolResult {
        let start = std::time::Instant::now();
        info!("[請求] {} {:?}", name, args);

        let result = self.execute_tool(name, args).await;
        let elapsed = start.elapsed();

        match &result {
            Ok(_) => info!("[回應] {} 成功 ({:?})", name, elapsed),
            Err(e) => error!("[錯誤] {} 失敗: {} ({:?})", name, e, elapsed),
        }

        match result {
            Ok(data) => ToolResult::success(data),
            Err(e) => ToolResult::error(e.to_string()),
        }
    }

    /// 執行工具邏輯
    async fn execute_tool(&self, name: &str, args: Option<Value>) -> Result<Value> {
        let args = args.unwrap_or(json!({}));

        match name {
            // Issues
            "redmine_get_issues" => {
                let params: IssueListParams = serde_json::from_value(args)?;
                let result = self.client.get_issues(&params).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_issue" => {
                let id = get_required_u64(&args, "id")?;
                let result = self.client.get_issue(id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_update_issue" => {
                let id = get_required_u64(&args, "id")?;
                let params: IssueUpdateParams = serde_json::from_value(args)?;
                self.client.update_issue(id, &params).await?;
                Ok(json!({ "success": true }))
            }
            "redmine_get_journals" => {
                let issue_id = get_required_u64(&args, "issue_id")?;
                let result = self.client.get_journals(issue_id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Projects
            "redmine_get_projects" => {
                let result = self.client.get_projects().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_project_members" => {
                let project_id = get_required_str(&args, "project_id")?;
                let result = self.client.get_project_members(&project_id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Users
            "redmine_get_current_user" => {
                let result = self.client.get_current_user().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_users" => {
                let params: UserListParams = serde_json::from_value(args)?;
                let result = self.client.get_users(&params).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_user" => {
                let id = get_required_u64(&args, "id")?;
                let result = self.client.get_user(id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Metadata
            "redmine_get_trackers" => {
                let result = self.client.get_trackers().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_statuses" => {
                let result = self.client.get_statuses().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_priorities" => {
                let result = self.client.get_priorities().await?;
                Ok(serde_json::to_value(result)?)
            }

            // Time Entries
            "redmine_get_time_entries" => {
                let params: TimeEntryListParams = serde_json::from_value(args)?;
                let result = self.client.get_time_entries(&params).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_create_time_entry" => {
                let params: TimeEntryCreateParams = serde_json::from_value(args)?;
                let result = self.client.create_time_entry(&params).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_time_entry_activities" => {
                let result = self.client.get_time_entry_activities().await?;
                Ok(serde_json::to_value(result)?)
            }

            // Versions
            "redmine_get_versions" => {
                let project_id = get_required_str(&args, "project_id")?;
                let result = self.client.get_versions(&project_id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_version" => {
                let id = get_required_u64(&args, "id")?;
                let result = self.client.get_version(id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Relations
            "redmine_get_issue_relations" => {
                let issue_id = get_required_u64(&args, "issue_id")?;
                let result = self.client.get_issue_relations(issue_id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_create_issue_relation" => {
                let issue_id = get_required_u64(&args, "issue_id")?;
                let params: IssueRelationParams = serde_json::from_value(args)?;
                let result = self.client.create_issue_relation(issue_id, &params).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_delete_issue_relation" => {
                let relation_id = get_required_u64(&args, "relation_id")?;
                self.client.delete_issue_relation(relation_id).await?;
                Ok(json!({ "success": true }))
            }

            // Categories
            "redmine_get_issue_categories" => {
                let project_id = get_required_str(&args, "project_id")?;
                let result = self.client.get_issue_categories(&project_id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Wiki
            "redmine_get_wiki_pages" => {
                let project_id = get_required_str(&args, "project_id")?;
                let result = self.client.get_wiki_pages(&project_id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_wiki_page" => {
                let project_id = get_required_str(&args, "project_id")?;
                let title = get_required_str(&args, "title")?;
                let result = self.client.get_wiki_page(&project_id, &title).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_update_wiki_page" => {
                let project_id = get_required_str(&args, "project_id")?;
                let title = get_required_str(&args, "title")?;
                let params: WikiPageParams = serde_json::from_value(args)?;
                self.client
                    .update_wiki_page(&project_id, &title, &params)
                    .await?;
                Ok(json!({ "success": true }))
            }

            // Files
            "redmine_get_files" => {
                let project_id = get_required_str(&args, "project_id")?;
                let result = self.client.get_files(&project_id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_attachment" => {
                let id = get_required_u64(&args, "id")?;
                let result = self.client.get_attachment(id).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_upload" => {
                let file_path = get_required_str(&args, "file_path")?;
                let description = args.get("description").and_then(|v| v.as_str());
                let result = self.client.upload_file(&file_path, description).await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_download" => {
                let attachment_id = get_required_u64(&args, "attachment_id")?;
                let save_path = get_required_str(&args, "save_path")?;
                let result = self
                    .client
                    .download_attachment(attachment_id, &save_path)
                    .await?;
                Ok(json!({
                    "saved_to": result.saved_to,
                    "filename": result.filename
                }))
            }

            // Search
            "redmine_search" => {
                let q = get_required_str(&args, "q")?;
                let params: SearchParams = serde_json::from_value(args)?;
                let result = self.client.search(&q, &params).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Others
            "redmine_get_queries" => {
                let result = self.client.get_queries().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_roles" => {
                let result = self.client.get_roles().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_groups" => {
                let result = self.client.get_groups().await?;
                Ok(serde_json::to_value(result)?)
            }
            "redmine_get_news" => {
                let project_id = args.get("project_id").and_then(|v| v.as_str());
                let result = self.client.get_news(project_id).await?;
                Ok(serde_json::to_value(result)?)
            }

            // Generic
            "redmine_request" => {
                let path = get_required_str(&args, "path")?;
                let method = args
                    .get("method")
                    .and_then(|v| v.as_str())
                    .unwrap_or("GET");
                let data = args.get("data").cloned();
                let params = args
                    .get("params")
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                let result = self
                    .client
                    .request(&path, method, data.as_ref(), params.as_ref())
                    .await?;
                Ok(serde_json::to_value(result)?)
            }

            // Log Viewer
            "redmine_log_viewer" => {
                let open = args.get("open").and_then(|v| v.as_bool()).unwrap_or(false);
                if let Some(url) = crate::log_viewer::get_log_viewer_url() {
                    if open {
                        let _ = open::that(&url);
                    }
                    Ok(json!({ "url": url, "opened": open }))
                } else {
                    Ok(json!({ "error": "Log Viewer 未啟動", "hint": "設定 LOG_VIEWER=true" }))
                }
            }

            _ => Err(crate::error::RedmineError::UnknownTool(name.to_string())),
        }
    }
}

// Helper functions
fn get_required_u64(args: &Value, key: &str) -> Result<u64> {
    args.get(key)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| crate::error::RedmineError::MissingParam(key.to_string()))
}

fn get_required_str(args: &Value, key: &str) -> Result<String> {
    args.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| crate::error::RedmineError::MissingParam(key.to_string()))
}
