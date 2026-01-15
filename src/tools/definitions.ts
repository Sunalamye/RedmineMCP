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

const TOOL_HINT = "(use /redmine for details)";

export const TOOLS: ToolDefinition[] = [
  // Issues
  {
    name: "redmine_get_issues",
    description: `List issues ${TOOL_HINT}`,
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
    description: `Get issue details ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_update_issue",
    description: `Update issue ${TOOL_HINT}`,
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
    description: `Get issue history ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },

  // Projects & Users
  {
    name: "redmine_get_projects",
    description: `List projects ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_project_members",
    description: `Get project members ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_current_user",
    description: `Get current user ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_users",
    description: `List users ${TOOL_HINT}`,
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
    description: `Get user details ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Trackers & Statuses
  {
    name: "redmine_get_trackers",
    description: `List trackers ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_statuses",
    description: `List statuses ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_priorities",
    description: `List priorities ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },

  // Time Entries
  {
    name: "redmine_get_time_entries",
    description: `List time entries ${TOOL_HINT}`,
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
    description: `Create time entry ${TOOL_HINT}`,
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
    description: `List activity types ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },

  // Versions
  {
    name: "redmine_get_versions",
    description: `List versions ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_version",
    description: `Get version details ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },

  // Issue Relations
  {
    name: "redmine_get_issue_relations",
    description: `Get issue relations ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { issue_id: { type: "number" } },
      required: ["issue_id"],
    },
  },
  {
    name: "redmine_create_issue_relation",
    description: `Create relation ${TOOL_HINT}`,
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
    description: `Delete relation ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { relation_id: { type: "number" } },
      required: ["relation_id"],
    },
  },

  // Issue Categories
  {
    name: "redmine_get_issue_categories",
    description: `Get issue categories ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },

  // Wiki
  {
    name: "redmine_get_wiki_pages",
    description: `List wiki pages ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_wiki_page",
    description: `Get wiki page content ${TOOL_HINT}`,
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
    description: `Update wiki page ${TOOL_HINT}`,
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
    description: `Get project files ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
      required: ["project_id"],
    },
  },
  {
    name: "redmine_get_attachment",
    description: `Get attachment info ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { id: { type: "number" } },
      required: ["id"],
    },
  },
  {
    name: "redmine_upload",
    description: `Upload file ${TOOL_HINT}`,
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
    description: `Download attachment ${TOOL_HINT}`,
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
    description: `Full-text search ${TOOL_HINT}`,
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
    description: `List saved queries ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_roles",
    description: `List roles ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_groups",
    description: `List groups ${TOOL_HINT}`,
    inputSchema: { type: "object", properties: {} },
  },
  {
    name: "redmine_get_news",
    description: `List news ${TOOL_HINT}`,
    inputSchema: {
      type: "object",
      properties: { project_id: { type: "string" } },
    },
  },

  // Log Viewer
  {
    name: "redmine_log_viewer",
    description: "Get Log Viewer URL and optionally open browser (open=true opens browser) [MCP internal tool]",
    inputSchema: {
      type: "object",
      properties: {
        open: { type: "boolean" },
      },
    },
  },
];
