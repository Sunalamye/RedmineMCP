/**
 * MCP Tool Handlers
 */

import {
  RedmineClient,
  type IssueListParams,
  type IssueUpdateParams,
  type TimeEntryParams,
  type TimeEntryListParams,
  type IssueRelationParams,
  type WikiPageParams,
  type SearchParams,
} from "../redmine-client.js";
import { log } from "../logger.js";
import {
  getLogViewerUrl,
  getLogViewerPort,
  isLogViewerRunning,
} from "../log-viewer/server.js";

// ============================================================================
// Logging Helpers
// ============================================================================

/** Redact sensitive data from logs */
function redact(obj: unknown): unknown {
  if (obj === null || obj === undefined) return obj;
  if (typeof obj === "string") {
    return obj.replace(/([a-f0-9]{20,})/gi, "[REDACTED]");
  }
  if (Array.isArray(obj)) {
    return obj.map(redact);
  }
  if (typeof obj === "object") {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj)) {
      if (/token|key|password|secret|credential|api_key/i.test(key)) {
        result[key] = "[REDACTED]";
      } else {
        result[key] = redact(value);
      }
    }
    return result;
  }
  return obj;
}

/** Truncate large response for logging */
function truncateForLog(data: unknown, maxLength = 2000): string {
  const str = JSON.stringify(redact(data));
  if (str.length <= maxLength) return str;
  return str.substring(0, maxLength) + `... [truncated ${str.length - maxLength} chars]`;
}
import { type ToolName } from "./definitions.js";

export interface ToolResult {
  content: Array<{ type: "text"; text: string }>;
  isError?: boolean;
  [key: string]: unknown;
}

type Args = Record<string, unknown> | undefined;
type Handler = (client: RedmineClient, args: Args) => Promise<unknown>;

function require<T>(args: Args, key: string): T {
  const value = args?.[key] as T;
  if (value === undefined || value === null) {
    throw new Error(`Missing required parameter: ${key}`);
  }
  return value;
}

const handlers: Record<ToolName, Handler> = {
  // Issues
  redmine_get_issues: (client, args) => {
    const params: IssueListParams = {
      project_id: args?.project_id as string | undefined,
      tracker_id: args?.tracker_id as number | undefined,
      status_id: args?.status_id as string | undefined,
      assigned_to_id: args?.assigned_to_id as string | undefined,
      limit: (args?.limit as number) || 25,
      offset: args?.offset as number | undefined,
      sort: args?.sort as string | undefined,
    };
    return client.getIssues(params);
  },

  redmine_get_issue: (client, args) => client.getIssue(require<number>(args, "id")),

  redmine_update_issue: (client, args) => {
    const params: IssueUpdateParams = {
      notes: args?.notes as string | undefined,
      status_id: args?.status_id as number | undefined,
      assigned_to_id: args?.assigned_to_id as number | undefined,
      done_ratio: args?.done_ratio as number | undefined,
      priority_id: args?.priority_id as number | undefined,
    };
    return client.updateIssue(require<number>(args, "id"), params);
  },

  redmine_get_journals: (client, args) => client.getJournals(require<number>(args, "issue_id")),

  // Issue Relations
  redmine_get_issue_relations: (client, args) => client.getIssueRelations(require<number>(args, "issue_id")),

  redmine_create_issue_relation: (client, args) => {
    const params: IssueRelationParams = {
      issue_to_id: args?.issue_to_id as number,
      relation_type: args?.relation_type as string,
      delay: args?.delay as number | undefined,
    };
    return client.createIssueRelation(require<number>(args, "issue_id"), params);
  },

  redmine_delete_issue_relation: (client, args) =>
    client.deleteIssueRelation(require<number>(args, "relation_id")),

  // Projects
  redmine_get_projects: (client) => client.getProjects(),
  redmine_get_project_members: (client, args) => client.getProjectMembers(require<string>(args, "project_id")),
  redmine_get_issue_categories: (client, args) => client.getIssueCategories(require<string>(args, "project_id")),
  redmine_get_versions: (client, args) => client.getVersions(require<string>(args, "project_id")),
  redmine_get_version: (client, args) => client.getVersion(require<number>(args, "id")),

  // Users
  redmine_get_current_user: (client) => client.getCurrentUser(),

  redmine_get_users: (client, args) =>
    client.getUsers({
      status: args?.status as number | undefined,
      name: args?.name as string | undefined,
      group_id: args?.group_id as number | undefined,
      limit: args?.limit as number | undefined,
      offset: args?.offset as number | undefined,
    }),

  redmine_get_user: (client, args) => client.getUser(require<number>(args, "id")),

  // Enumerations
  redmine_get_trackers: (client) => client.getTrackers(),
  redmine_get_statuses: (client) => client.getStatuses(),
  redmine_get_priorities: (client) => client.getPriorities(),
  redmine_get_time_entry_activities: (client) => client.getTimeEntryActivities(),
  redmine_get_roles: (client) => client.getRoles(),
  redmine_get_groups: (client) => client.getGroups(),
  redmine_get_queries: (client) => client.getQueries(),

  // Time Entries
  redmine_get_time_entries: (client, args) => {
    const params: TimeEntryListParams = {
      project_id: args?.project_id as string | undefined,
      user_id: args?.user_id as string | undefined,
      from: args?.from as string | undefined,
      to: args?.to as string | undefined,
      limit: args?.limit as number | undefined,
      offset: args?.offset as number | undefined,
    };
    return client.getTimeEntries(params);
  },

  redmine_create_time_entry: (client, args) => {
    const params: TimeEntryParams = {
      issue_id: args?.issue_id as number | undefined,
      project_id: args?.project_id as string | undefined,
      hours: args?.hours as number,
      activity_id: args?.activity_id as number | undefined,
      comments: args?.comments as string | undefined,
      spent_on: args?.spent_on as string | undefined,
    };
    return client.createTimeEntry(params);
  },

  // Wiki
  redmine_get_wiki_pages: (client, args) => client.getWikiPages(require<string>(args, "project_id")),

  redmine_get_wiki_page: (client, args) => {
    const projectId = require<string>(args, "project_id");
    const title = require<string>(args, "title");
    return client.getWikiPage(projectId, title);
  },

  redmine_update_wiki_page: (client, args) => {
    const projectId = require<string>(args, "project_id");
    const title = require<string>(args, "title");
    const params: WikiPageParams = {
      text: args?.text as string,
      comments: args?.comments as string | undefined,
    };
    return client.updateWikiPage(projectId, title, params);
  },

  // Files & Attachments
  redmine_get_files: (client, args) => client.getFiles(require<string>(args, "project_id")),
  redmine_get_attachment: (client, args) => client.getAttachment(require<number>(args, "id")),

  redmine_upload: (client, args) => {
    const filePath = require<string>(args, "file_path");
    return client.uploadFile(filePath, args?.description as string | undefined);
  },

  redmine_download: (client, args) => {
    const attachmentId = require<number>(args, "attachment_id");
    const savePath = require<string>(args, "save_path");
    return client.downloadAttachment(attachmentId, savePath);
  },

  // Search
  redmine_search: (client, args) => {
    const query = require<string>(args, "q");
    const params: SearchParams = {
      scope: args?.scope as string | undefined,
      project_id: args?.project_id as string | undefined,
      limit: args?.limit as number | undefined,
      offset: args?.offset as number | undefined,
    };
    return client.search(query, params);
  },

  // News
  redmine_get_news: (client, args) => client.getNews(args?.project_id as string | undefined),

  // Log Viewer
  redmine_log_viewer: async (_client, args) => {
    const running = isLogViewerRunning();
    const url = getLogViewerUrl();
    const port = getLogViewerPort();

    if (!running || !url) {
      return {
        running: false,
        message: "Log Viewer not running, set LOG_VIEWER=true to enable",
      };
    }

    if (args?.open === true) {
      const cmd =
        process.platform === "darwin"
          ? ["open", url]
          : process.platform === "win32"
            ? ["cmd", "/c", "start", url]
            : ["xdg-open", url];
      Bun.spawn(cmd, { stdout: "ignore", stderr: "ignore" });
    }

    return {
      running: true,
      url,
      port,
      message: args?.open ? "Browser opened" : "Log Viewer running",
    };
  },
};

export function createToolHandler(redmineClient: RedmineClient) {
  return async (name: string, args: Args): Promise<ToolResult> => {
    const startTime = Date.now();
    log.info(`[Request] ${name} ${args ? JSON.stringify(args) : ""}`);

    try {
      const handler = handlers[name as ToolName];
      if (!handler) throw new Error(`Unknown tool: ${name}`);

      const result = await handler(redmineClient, args);
      const elapsed = Date.now() - startTime;
      log.info(`[Response] ${name} success (${elapsed}ms)`);
      log.debug(`[Response] ${name} body: ${truncateForLog(result)}`);

      return {
        content: [{ type: "text", text: JSON.stringify(result, null, 2) }],
      };
    } catch (error) {
      const elapsed = Date.now() - startTime;
      const message = error instanceof Error ? error.message : String(error);
      log.error(`[Error] ${name} failed: ${message} (${elapsed}ms)`);

      return {
        content: [{ type: "text", text: `Error: ${message}` }],
        isError: true,
      };
    }
  };
}
