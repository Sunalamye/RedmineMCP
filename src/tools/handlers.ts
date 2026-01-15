/**
 * MCP Tool Handlers
 * 使用 Handler Map 模式取代 switch statement
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

export interface ToolResult {
  content: Array<{ type: "text"; text: string }>;
  isError?: boolean;
  [key: string]: unknown;
}

type Args = Record<string, unknown> | undefined;
type Handler = (client: RedmineClient, args: Args) => Promise<unknown>;

// =============================================================================
// Helper Functions
// =============================================================================

function requireId(args: Args): number {
  const id = args?.id as number;
  if (!id) throw new Error("缺少必要參數: id");
  return id;
}

function requireIssueId(args: Args): number {
  const issueId = args?.issue_id as number;
  if (!issueId) throw new Error("缺少必要參數: issue_id");
  return issueId;
}

function requireProjectId(args: Args): string {
  const projectId = args?.project_id as string;
  if (!projectId) throw new Error("缺少必要參數: project_id");
  return projectId;
}

// =============================================================================
// Handler Map
// =============================================================================

const handlers: Record<string, Handler> = {
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

  redmine_get_issue: (client, args) => client.getIssue(requireId(args)),

  redmine_update_issue: (client, args) => {
    const params: IssueUpdateParams = {
      notes: args?.notes as string | undefined,
      status_id: args?.status_id as number | undefined,
      assigned_to_id: args?.assigned_to_id as number | undefined,
      done_ratio: args?.done_ratio as number | undefined,
      priority_id: args?.priority_id as number | undefined,
    };
    return client.updateIssue(requireId(args), params);
  },

  redmine_get_journals: (client, args) => client.getJournals(requireIssueId(args)),

  // Issue Relations
  redmine_get_issue_relations: (client, args) => client.getIssueRelations(requireIssueId(args)),

  redmine_create_issue_relation: (client, args) => {
    const params: IssueRelationParams = {
      issue_to_id: args?.issue_to_id as number,
      relation_type: args?.relation_type as string,
      delay: args?.delay as number | undefined,
    };
    return client.createIssueRelation(requireIssueId(args), params);
  },

  redmine_delete_issue_relation: (client, args) => {
    const relationId = args?.relation_id as number;
    if (!relationId) throw new Error("缺少必要參數: relation_id");
    return client.deleteIssueRelation(relationId);
  },

  // Projects
  redmine_get_projects: (client) => client.getProjects(),
  redmine_get_project_members: (client, args) => client.getProjectMembers(requireProjectId(args)),
  redmine_get_issue_categories: (client, args) => client.getIssueCategories(requireProjectId(args)),
  redmine_get_versions: (client, args) => client.getVersions(requireProjectId(args)),
  redmine_get_version: (client, args) => client.getVersion(requireId(args)),

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

  redmine_get_user: (client, args) => client.getUser(requireId(args)),

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
  redmine_get_wiki_pages: (client, args) => client.getWikiPages(requireProjectId(args)),

  redmine_get_wiki_page: (client, args) => {
    const projectId = requireProjectId(args);
    const title = args?.title as string;
    if (!title) throw new Error("缺少必要參數: title");
    return client.getWikiPage(projectId, title);
  },

  redmine_update_wiki_page: (client, args) => {
    const projectId = requireProjectId(args);
    const title = args?.title as string;
    if (!title) throw new Error("缺少必要參數: title");
    const params: WikiPageParams = {
      text: args?.text as string,
      comments: args?.comments as string | undefined,
    };
    return client.updateWikiPage(projectId, title, params);
  },

  // Files & Attachments
  redmine_get_files: (client, args) => client.getFiles(requireProjectId(args)),
  redmine_get_attachment: (client, args) => client.getAttachment(requireId(args)),

  redmine_upload: (client, args) => {
    const filePath = args?.file_path as string;
    if (!filePath) throw new Error("缺少必要參數: file_path");
    return client.uploadFile(filePath, args?.description as string | undefined);
  },

  redmine_download: (client, args) => {
    const attachmentId = args?.attachment_id as number;
    const savePath = args?.save_path as string;
    if (!attachmentId) throw new Error("缺少必要參數: attachment_id");
    if (!savePath) throw new Error("缺少必要參數: save_path");
    return client.downloadAttachment(attachmentId, savePath);
  },

  // Search
  redmine_search: (client, args) => {
    const query = args?.q as string;
    if (!query) throw new Error("缺少必要參數: q");
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

  // Log Viewer (不需要 client)
  redmine_log_viewer: async (_client, args) => {
    const running = isLogViewerRunning();
    const url = getLogViewerUrl();
    const port = getLogViewerPort();

    if (!running || !url) {
      return {
        running: false,
        message: "Log Viewer 未啟動，請設定 LOG_VIEWER=true",
      };
    }

    // 可選：開啟瀏覽器
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
      message: args?.open ? "已開啟瀏覽器" : "Log Viewer 運行中",
    };
  },
};

// =============================================================================
// Main Handler
// =============================================================================

export function createToolHandler(redmineClient: RedmineClient) {
  return async (name: string, args: Args): Promise<ToolResult> => {
    const startTime = Date.now();
    log.info(`[請求] ${name} ${args ? JSON.stringify(args) : ""}`);

    try {
      const handler = handlers[name];
      if (!handler) throw new Error(`未知的工具: ${name}`);

      const result = await handler(redmineClient, args);
      const elapsed = Date.now() - startTime;
      log.info(`[回應] ${name} 成功 (${elapsed}ms)`);

      return {
        content: [{ type: "text", text: JSON.stringify(result, null, 2) }],
      };
    } catch (error) {
      const elapsed = Date.now() - startTime;
      const message = error instanceof Error ? error.message : String(error);
      log.error(`[錯誤] ${name} 失敗: ${message} (${elapsed}ms)`);

      return {
        content: [{ type: "text", text: `錯誤: ${message}` }],
        isError: true,
      };
    }
  };
}
