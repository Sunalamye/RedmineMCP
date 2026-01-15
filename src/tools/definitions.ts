/**
 * MCP Tools Definitions
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

export const TOOLS = [
  // Issues
  {
    name: "redmine_get_issues",
    description: "List issues with optional filters",
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
    description: "Get issue details with journals and attachments",
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_update_issue",
    description: "Update issue (add notes, change status/assignee)",
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
    description: "Get issue history/comments",
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },

  // Projects & Users
  {
    name: "redmine_get_projects",
    description: "List all projects",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_project_members",
    description: "Get project members",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_current_user",
    description: "Get current authenticated user",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_users",
    description: "List users with optional filters",
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
    description: "Get user details with groups/memberships",
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Trackers & Statuses
  {
    name: "redmine_get_trackers",
    description: "List all trackers",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_statuses",
    description: "List all issue statuses",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_priorities",
    description: "List all issue priorities",
    inputSchema: { type: "object", properties: {} },
  },

  // Time Entries
  {
    name: "redmine_get_time_entries",
    description: "List time entries with optional filters",
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
    description: "Create time entry for issue or project",
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
    description: "List time entry activity types",
    inputSchema: { type: "object", properties: {} },
  },

  // Versions
  {
    name: "redmine_get_versions",
    description: "List project versions/milestones",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_version",
    description: "Get version details",
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Issue Relations
  {
    name: "redmine_get_issue_relations",
    description: "Get issue relations (blocks, duplicates, etc)",
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },
  {
    name: "redmine_create_issue_relation",
    description: "Create relation between issues",
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
    description: "Delete issue relation",
    inputSchema: {
      type: "object",
      properties: { relation_id: { type: "number" } },
      required: ["relation_id"],
    },
  },

  // Issue Categories
  {
    name: "redmine_get_issue_categories",
    description: "Get project issue categories",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },

  // Wiki
  {
    name: "redmine_get_wiki_pages",
    description: "List wiki pages in project",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_wiki_page",
    description: "Get wiki page content",
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
    description: "Update wiki page content",
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
    description: "Get project files",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_attachment",
    description: "Get attachment metadata",
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_upload",
    description: "Upload file to Redmine",
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
    description: "Download attachment to local path",
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
    description: "Full-text search across issues/wiki/news",
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
    description: "List saved queries",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_roles",
    description: "List all roles",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_groups",
    description: "List all groups",
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_news",
    description: "List news (optionally by project)",
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
    },
  },

  // Log Viewer
  {
    name: "redmine_log_viewer",
    description: "Get Log Viewer URL (open=true opens browser)",
    inputSchema: {
      type: "object",
      properties: { open: { type: "boolean" } },
    },
  },
] as const satisfies readonly ToolDefinition[];

export type ToolName = (typeof TOOLS)[number]["name"];
