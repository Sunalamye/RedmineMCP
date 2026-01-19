//! MCP 工具定義

use serde_json::{json, Value};

/// 工具定義常量
pub static TOOL_DEFINITIONS: &[Value] = &[];

/// 建立工具定義
pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        // Issues
        json!({
            "name": "redmine_get_issues",
            "description": "Issues 列表",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string" },
                    "tracker_id": { "type": "number" },
                    "status_id": { "type": "string" },
                    "assigned_to_id": { "type": "string" },
                    "limit": { "type": "number" },
                    "offset": { "type": "number" },
                    "sort": { "type": "string" }
                }
            }
        }),
        json!({
            "name": "redmine_get_issue",
            "description": "Issue 詳情",
            "inputSchema": {
                "type": "object",
                "properties": { "id": { "type": "number" } },
                "required": ["id"]
            }
        }),
        json!({
            "name": "redmine_update_issue",
            "description": "更新 Issue",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "id": { "type": "number" },
                    "notes": { "type": "string" },
                    "status_id": { "type": "number" },
                    "assigned_to_id": { "type": "number" },
                    "done_ratio": { "type": "number" },
                    "priority_id": { "type": "number" }
                },
                "required": ["id"]
            }
        }),
        json!({
            "name": "redmine_get_journals",
            "description": "Issue 歷史",
            "inputSchema": {
                "type": "object",
                "properties": { "issue_id": { "type": "number" } },
                "required": ["issue_id"]
            }
        }),

        // Projects & Users
        json!({
            "name": "redmine_get_projects",
            "description": "專案列表",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_project_members",
            "description": "專案成員",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } },
                "required": ["project_id"]
            }
        }),
        json!({
            "name": "redmine_get_current_user",
            "description": "當前使用者",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_users",
            "description": "使用者列表",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "status": { "type": "number" },
                    "name": { "type": "string" },
                    "group_id": { "type": "number" },
                    "limit": { "type": "number" },
                    "offset": { "type": "number" }
                }
            }
        }),
        json!({
            "name": "redmine_get_user",
            "description": "使用者詳情",
            "inputSchema": {
                "type": "object",
                "properties": { "id": { "type": "number" } },
                "required": ["id"]
            }
        }),

        // Trackers & Statuses
        json!({
            "name": "redmine_get_trackers",
            "description": "追蹤標籤",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_statuses",
            "description": "狀態列表",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_priorities",
            "description": "優先權",
            "inputSchema": { "type": "object", "properties": {} }
        }),

        // Time Entries
        json!({
            "name": "redmine_get_time_entries",
            "description": "工時列表",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string" },
                    "user_id": { "type": "string" },
                    "from": { "type": "string" },
                    "to": { "type": "string" },
                    "limit": { "type": "number" },
                    "offset": { "type": "number" }
                }
            }
        }),
        json!({
            "name": "redmine_create_time_entry",
            "description": "建立工時",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "issue_id": { "type": "number" },
                    "project_id": { "type": "string" },
                    "hours": { "type": "number" },
                    "activity_id": { "type": "number" },
                    "comments": { "type": "string" },
                    "spent_on": { "type": "string" }
                },
                "required": ["hours"]
            }
        }),
        json!({
            "name": "redmine_get_time_entry_activities",
            "description": "活動類型",
            "inputSchema": { "type": "object", "properties": {} }
        }),

        // Versions
        json!({
            "name": "redmine_get_versions",
            "description": "版本列表",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } },
                "required": ["project_id"]
            }
        }),
        json!({
            "name": "redmine_get_version",
            "description": "版本詳情",
            "inputSchema": {
                "type": "object",
                "properties": { "id": { "type": "number" } },
                "required": ["id"]
            }
        }),

        // Issue Relations
        json!({
            "name": "redmine_get_issue_relations",
            "description": "Issue 關聯",
            "inputSchema": {
                "type": "object",
                "properties": { "issue_id": { "type": "number" } },
                "required": ["issue_id"]
            }
        }),
        json!({
            "name": "redmine_create_issue_relation",
            "description": "建立關聯",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "issue_id": { "type": "number" },
                    "issue_to_id": { "type": "number" },
                    "relation_type": { "type": "string" },
                    "delay": { "type": "number" }
                },
                "required": ["issue_id", "issue_to_id", "relation_type"]
            }
        }),
        json!({
            "name": "redmine_delete_issue_relation",
            "description": "刪除關聯",
            "inputSchema": {
                "type": "object",
                "properties": { "relation_id": { "type": "number" } },
                "required": ["relation_id"]
            }
        }),

        // Issue Categories
        json!({
            "name": "redmine_get_issue_categories",
            "description": "Issue 分類",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } },
                "required": ["project_id"]
            }
        }),

        // Wiki
        json!({
            "name": "redmine_get_wiki_pages",
            "description": "Wiki 列表",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } },
                "required": ["project_id"]
            }
        }),
        json!({
            "name": "redmine_get_wiki_page",
            "description": "Wiki 內容",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string" },
                    "title": { "type": "string" }
                },
                "required": ["project_id", "title"]
            }
        }),
        json!({
            "name": "redmine_update_wiki_page",
            "description": "更新 Wiki",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project_id": { "type": "string" },
                    "title": { "type": "string" },
                    "text": { "type": "string" },
                    "comments": { "type": "string" }
                },
                "required": ["project_id", "title", "text"]
            }
        }),

        // Files & Attachments
        json!({
            "name": "redmine_get_files",
            "description": "專案檔案",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } },
                "required": ["project_id"]
            }
        }),
        json!({
            "name": "redmine_get_attachment",
            "description": "附件資訊",
            "inputSchema": {
                "type": "object",
                "properties": { "id": { "type": "number" } },
                "required": ["id"]
            }
        }),
        json!({
            "name": "redmine_upload",
            "description": "上傳檔案",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "file_path": { "type": "string" },
                    "description": { "type": "string" }
                },
                "required": ["file_path"]
            }
        }),
        json!({
            "name": "redmine_download",
            "description": "下載附件",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "attachment_id": { "type": "number" },
                    "save_path": { "type": "string" }
                },
                "required": ["attachment_id", "save_path"]
            }
        }),

        // Search
        json!({
            "name": "redmine_search",
            "description": "全文搜尋",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "q": { "type": "string" },
                    "scope": { "type": "string" },
                    "project_id": { "type": "string" },
                    "limit": { "type": "number" },
                    "offset": { "type": "number" }
                },
                "required": ["q"]
            }
        }),

        // Others
        json!({
            "name": "redmine_get_queries",
            "description": "已存查詢",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_roles",
            "description": "角色列表",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_groups",
            "description": "群組列表",
            "inputSchema": { "type": "object", "properties": {} }
        }),
        json!({
            "name": "redmine_get_news",
            "description": "新聞列表",
            "inputSchema": {
                "type": "object",
                "properties": { "project_id": { "type": "string" } }
            }
        }),

        // Generic API
        json!({
            "name": "redmine_request",
            "description": "通用 API",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "method": { "type": "string" },
                    "data": { "type": "object" },
                    "params": { "type": "object" }
                },
                "required": ["path"]
            }
        }),

        // Log Viewer
        json!({
            "name": "redmine_log_viewer",
            "description": "Log Viewer URL (open=true 開啟瀏覽器)",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "open": { "type": "boolean" }
                }
            }
        }),
    ]
}
