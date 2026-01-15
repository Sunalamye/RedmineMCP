/**
 * MCP Tools 定義
 */

export interface ToolDefinition {
  name: string;
  description: string;
  inputSchema: {
    type: "object";
    properties: Record<string, { type: string }>;
    required?: string[];
  };
}

const TOOL_HINT = "（用 /redmine 查看詳細說明）";

export const TOOLS: ToolDefinition[] = [
  // Issues
  {
    name: "redmine_get_issues",
    description: `Issues 列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        project_id: { type: "string" },
        tracker_id: { type: "number" },
        status_id: { type: "string" },
        assigned_to_id: { type: "string" },
        limit: { type: "number" },
        offset: { type: "number" },
        sort: { type: "string" },
      },
    },
  },
  {
    name: "redmine_get_issue",
    description: `Issue 詳情 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_update_issue",
    description: `更新 Issue ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        id: { type: "number" },
        notes: { type: "string" },
        status_id: { type: "number" },
        assigned_to_id: { type: "number" },
        done_ratio: { type: "number" },
        priority_id: { type: "number" },
      },
      required: ["id"],
    },
  },
  {
    name: "redmine_get_journals",
    description: `Issue 歷史 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },

  // Projects & Users
  {
    name: "redmine_get_projects",
    description: `專案列表 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_project_members",
    description: `專案成員 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_current_user",
    description: `當前使用者 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_users",
    description: `使用者列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        status: { type: "number" },
        name: { type: "string" },
        group_id: { type: "number" },
        limit: { type: "number" },
        offset: { type: "number" },
      },
    },
  },
  {
    name: "redmine_get_user",
    description: `使用者詳情 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Trackers & Statuses
  {
    name: "redmine_get_trackers",
    description: `追蹤標籤 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_statuses",
    description: `狀態列表 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_priorities",
    description: `優先權 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },

  // Time Entries
  {
    name: "redmine_get_time_entries",
    description: `工時列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        project_id: { type: "string" },
        user_id: { type: "string" },
        from: { type: "string" },
        to: { type: "string" },
        limit: { type: "number" },
        offset: { type: "number" },
      },
    },
  },
  {
    name: "redmine_create_time_entry",
    description: `建立工時 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        issue_id: { type: "number" },
        project_id: { type: "string" },
        hours: { type: "number" },
        activity_id: { type: "number" },
        comments: { type: "string" },
        spent_on: { type: "string" },
      },
      required: ["hours"],
    },
  },
  {
    name: "redmine_get_time_entry_activities",
    description: `活動類型 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },

  // Versions
  {
    name: "redmine_get_versions",
    description: `版本列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_version",
    description: `版本詳情 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Issue Relations
  {
    name: "redmine_get_issue_relations",
    description: `Issue 關聯 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },
  {
    name: "redmine_create_issue_relation",
    description: `建立關聯 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        issue_id: { type: "number" },
        issue_to_id: { type: "number" },
        relation_type: { type: "string" },
        delay: { type: "number" },
      },
      required: ["issue_id", "issue_to_id", "relation_type"],
    },
  },
  {
    name: "redmine_delete_issue_relation",
    description: `刪除關聯 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { relation_id: { type: "number" } },
      required: ["relation_id"],
    },
  },

  // Issue Categories
  {
    name: "redmine_get_issue_categories",
    description: `Issue 分類 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },

  // Wiki
  {
    name: "redmine_get_wiki_pages",
    description: `Wiki 列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_wiki_page",
    description: `Wiki 內容 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        project_id: { type: "string" },
        title: { type: "string" },
      },
      required: ["project_id", "title"],
    },
  },
  {
    name: "redmine_update_wiki_page",
    description: `更新 Wiki ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        project_id: { type: "string" },
        title: { type: "string" },
        text: { type: "string" },
        comments: { type: "string" },
      },
      required: ["project_id", "title", "text"],
    },
  },

  // Files & Attachments
  {
    name: "redmine_get_files",
    description: `專案檔案 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_attachment",
    description: `附件資訊 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_upload",
    description: `上傳檔案 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        file_path: { type: "string" },
        description: { type: "string" },
      },
      required: ["file_path"],
    },
  },
  {
    name: "redmine_download",
    description: `下載附件 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        attachment_id: { type: "number" },
        save_path: { type: "string" },
      },
      required: ["attachment_id", "save_path"],
    },
  },

  // Search
  {
    name: "redmine_search",
    description: `全文搜尋 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: {
        q: { type: "string" },
        scope: { type: "string" },
        project_id: { type: "string" },
        limit: { type: "number" },
        offset: { type: "number" },
      },
      required: ["q"],
    },
  },

  // Others
  {
    name: "redmine_get_queries",
    description: `已存查詢 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_roles",
    description: `角色列表 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_groups",
    description: `群組列表 ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_news",
    description: `新聞列表 ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
    },
  },

  // Log Viewer
  {
    name: "redmine_log_viewer",
    description: "取得 Log Viewer 網址並可選擇開啟瀏覽器 (open=true 開啟瀏覽器) [MCP內部工具]",
    inputSchema: {
      type: "object",
      properties: {
        open: { type: "boolean" },
      },
    },
  },
];
