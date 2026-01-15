/**
 * Redmine API Client
 * Using API Key authentication
 *
 * Refactored version: Using generic methods to reduce code duplication
 */

import { log } from "./logger.js";

// ============================================================================
// Logging Helpers
// ============================================================================

/** Redact sensitive data from logs */
function redact(obj: unknown): unknown {
  if (obj === null || obj === undefined) return obj;
  if (typeof obj === "string") {
    // Redact long hex strings (tokens, API keys)
    return obj.replace(/([a-f0-9]{20,})/gi, "[REDACTED]");
  }
  if (Array.isArray(obj)) {
    return obj.map(redact);
  }
  if (typeof obj === "object") {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj)) {
      // Redact known sensitive fields
      if (/token|key|password|secret|credential/i.test(key)) {
        result[key] = "[REDACTED]";
      } else {
        result[key] = redact(value);
      }
    }
    return result;
  }
  return obj;
}

/** Truncate large response bodies for logging */
function truncateForLog(data: unknown, maxLength = 2000): string {
  const str = JSON.stringify(redact(data));
  if (str.length <= maxLength) return str;
  return str.substring(0, maxLength) + `... [truncated ${str.length - maxLength} chars]`;
}

// ============================================================================
// Types & Interfaces
// ============================================================================

export interface RedmineConfig {
  baseUrl: string;
  apiKey: string;
}

export interface RedmineIssue {
  id: number;
  project: { id: number; name: string };
  tracker: { id: number; name: string };
  status: { id: number; name: string };
  priority: { id: number; name: string };
  author: { id: number; name: string };
  assigned_to?: { id: number; name: string };
  subject: string;
  description: string;
  done_ratio: number;
  created_on: string;
  updated_on: string;
  fixed_version?: { id: number; name: string };
  category?: { id: number; name: string };
}

export interface IssueListParams {
  project_id?: string;
  tracker_id?: number;
  status_id?: string | number;
  assigned_to_id?: string | number;
  limit?: number;
  offset?: number;
  sort?: string;
}

export interface IssueUpdateParams {
  notes?: string;
  status_id?: number;
  assigned_to_id?: number;
  done_ratio?: number;
  priority_id?: number;
}

export interface TimeEntryParams {
  issue_id?: number;
  project_id?: string;
  hours: number;
  activity_id?: number;
  comments?: string;
  spent_on?: string;
}

export interface TimeEntryListParams {
  project_id?: string;
  user_id?: string | number;
  from?: string;
  to?: string;
  limit?: number;
  offset?: number;
}

export interface IssueRelationParams {
  issue_to_id: number;
  relation_type: string;
  delay?: number;
}

export interface WikiPageParams {
  text: string;
  comments?: string;
}

export interface SearchParams {
  scope?: string;
  project_id?: string;
  limit?: number;
  offset?: number;
}

// ============================================================================
// Helper Types
// ============================================================================

interface IdName {
  id: number;
  name: string;
}

interface PaginatedResponse {
  total_count: number;
  offset: number;
  limit: number;
}

// ============================================================================
// RedmineClient
// ============================================================================

export class RedmineClient {
  private config: RedmineConfig;

  constructor(config: RedmineConfig) {
    this.config = config;
  }

  // ==========================================================================
  // Core API Methods (Key to reducing duplication)
  // ==========================================================================

  private async apiFetch(path: string, options: RequestInit = {}): Promise<Response> {
    const url = `${this.config.baseUrl}${path}`;
    const method = options.method || "GET";
    const headers = new Headers(options.headers);
    headers.set("X-Redmine-API-Key", this.config.apiKey);
    headers.set("Content-Type", "application/json");
    headers.set("User-Agent", "RedmineMCP/1.0");

    // Log request
    const startTime = Date.now();
    log.debug(`[HTTP] ${method} ${url}`);
    if (options.body) {
      log.debug(`[HTTP] Request Body: ${truncateForLog(JSON.parse(options.body as string))}`);
    }

    const res = await fetch(url, { ...options, headers });
    const elapsed = Date.now() - startTime;

    // Log response status
    log.debug(`[HTTP] Response ${res.status} ${res.statusText} (${elapsed}ms)`);

    return res;
  }

  /** GET request - Auto error handling and JSON parsing */
  private async get<T>(path: string, errorMsg: string): Promise<T> {
    const res = await this.apiFetch(path);
    if (!res.ok) {
      const errorText = await res.text();
      log.debug(`[HTTP] Error Body: ${errorText}`);
      throw new Error(`${errorMsg}: ${res.status}`);
    }
    const data = await res.json();
    log.debug(`[HTTP] Response Body: ${truncateForLog(data)}`);
    return data;
  }

  /** GET request with query parameters */
  private async getWithQuery<T>(basePath: string, params: object, errorMsg: string): Promise<T> {
    const query = this.buildQueryString(params);
    const path = query ? `${basePath}?${query}` : basePath;
    return this.get<T>(path, errorMsg);
  }

  /** POST request - Returns created resource */
  private async post<T>(path: string, body: object, errorMsg: string): Promise<T> {
    const res = await this.apiFetch(path, {
      method: "POST",
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      const errorText = await res.text();
      log.debug(`[HTTP] Error Body: ${errorText}`);
      throw new Error(`${errorMsg}: ${res.status} - ${errorText}`);
    }
    const data = await res.json();
    log.debug(`[HTTP] Response Body: ${truncateForLog(data)}`);
    return data;
  }

  /** PUT/DELETE request - Returns success status only */
  private async mutate(
    path: string,
    method: "PUT" | "DELETE",
    body?: object,
    errorMsg = "Operation failed"
  ): Promise<{ success: boolean }> {
    const res = await this.apiFetch(path, {
      method,
      body: body ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) {
      const errorText = await res.text();
      log.debug(`[HTTP] Error Body: ${errorText}`);
      throw new Error(`${errorMsg}: ${res.status} - ${errorText}`);
    }
    log.debug(`[HTTP] Response: success (no body)`);
    return { success: true };
  }

  private buildQueryString(params: object): string {
    const queryParams = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
      if (value !== undefined && value !== null) {
        queryParams.set(key, String(value));
      }
    }
    return queryParams.toString();
  }

  // ==========================================================================
  // Auth
  // ==========================================================================

  async login(): Promise<boolean> {
    try {
      const res = await this.apiFetch("/users/current.json");
      console.error(res.ok ? "[RedmineClient] Authentication successful" : `[RedmineClient] Authentication failed: ${res.status}`);
      return res.ok;
    } catch (error) {
      console.error("[RedmineClient] Authentication error:", error);
      return false;
    }
  }

  // ==========================================================================
  // Issues
  // ==========================================================================

  async getIssues(params: IssueListParams = {}): Promise<{ issues: RedmineIssue[] } & PaginatedResponse> {
    return this.getWithQuery("/issues.json", params, "Failed to get issues");
  }

  async getIssue(id: number): Promise<{ issue: RedmineIssue }> {
    return this.get(`/issues/${id}.json?include=journals,attachments`, `Failed to get issue ${id}`);
  }

  async updateIssue(id: number, params: IssueUpdateParams): Promise<{ success: boolean }> {
    const body: Record<string, unknown> = {};
    if (params.notes) body.notes = params.notes;
    if (params.status_id) body.status_id = params.status_id;
    if (params.assigned_to_id) body.assigned_to_id = params.assigned_to_id;
    if (params.done_ratio !== undefined) body.done_ratio = params.done_ratio;
    if (params.priority_id) body.priority_id = params.priority_id;
    return this.mutate(`/issues/${id}.json`, "PUT", { issue: body }, `Failed to update issue ${id}`);
  }

  async getJournals(issueId: number): Promise<{
    issue: {
      id: number;
      journals: Array<{
        id: number;
        user: IdName;
        notes: string;
        created_on: string;
        details: Array<{ property: string; name: string; old_value: string | null; new_value: string | null }>;
      }>;
    };
  }> {
    return this.get(`/issues/${issueId}.json?include=journals`, "Failed to get journals");
  }

  // ==========================================================================
  // Issue Relations
  // ==========================================================================

  async getIssueRelations(issueId: number): Promise<{
    relations: Array<{ id: number; issue_id: number; issue_to_id: number; relation_type: string; delay: number | null }>;
  }> {
    return this.get(`/issues/${issueId}/relations.json`, "Failed to get relations");
  }

  async createIssueRelation(issueId: number, params: IssueRelationParams): Promise<{ relation: Record<string, unknown> }> {
    return this.post(`/issues/${issueId}/relations.json`, { relation: params }, "Failed to create relation");
  }

  async deleteIssueRelation(relationId: number): Promise<{ success: boolean }> {
    return this.mutate(`/relations/${relationId}.json`, "DELETE", undefined, "Failed to delete relation");
  }

  // ==========================================================================
  // Projects
  // ==========================================================================

  async getProjects(): Promise<{
    projects: Array<{ id: number; name: string; identifier: string; description: string }>;
    total_count: number;
  }> {
    return this.get("/projects.json?limit=100", "Failed to get projects");
  }

  async getProjectMembers(projectId: string): Promise<{
    memberships: Array<{ id: number; user?: IdName; roles: IdName[] }>;
  }> {
    return this.get(`/projects/${projectId}/memberships.json?limit=100`, "Failed to get members");
  }

  async getIssueCategories(projectId: string): Promise<{
    issue_categories: Array<{ id: number; project: IdName; name: string; assigned_to?: IdName }>;
  }> {
    return this.get(`/projects/${projectId}/issue_categories.json`, "Failed to get issue categories");
  }

  async getVersions(projectId: string): Promise<{
    versions: Array<{
      id: number;
      project: IdName;
      name: string;
      description: string;
      status: string;
      due_date: string | null;
      sharing: string;
      created_on: string;
      updated_on: string;
    }>;
    total_count: number;
  }> {
    return this.get(`/projects/${projectId}/versions.json`, "Failed to get versions");
  }

  async getVersion(id: number): Promise<{ version: Record<string, unknown> }> {
    return this.get(`/versions/${id}.json`, `Failed to get version ${id}`);
  }

  // ==========================================================================
  // Enumerations
  // ==========================================================================

  async getTrackers(): Promise<{ trackers: IdName[] }> {
    return this.get("/trackers.json", "Failed to get trackers");
  }

  async getStatuses(): Promise<{ issue_statuses: Array<{ id: number; name: string; is_closed: boolean }> }> {
    return this.get("/issue_statuses.json", "Failed to get statuses");
  }

  async getPriorities(): Promise<{ issue_priorities: Array<{ id: number; name: string; is_default: boolean }> }> {
    return this.get("/enumerations/issue_priorities.json", "Failed to get priorities");
  }

  async getTimeEntryActivities(): Promise<{ time_entry_activities: Array<{ id: number; name: string; is_default: boolean }> }> {
    return this.get("/enumerations/time_entry_activities.json", "Failed to get activities");
  }

  async getRoles(): Promise<{ roles: IdName[] }> {
    return this.get("/roles.json", "Failed to get roles");
  }

  async getGroups(): Promise<{ groups: IdName[] }> {
    return this.get("/groups.json", "Failed to get groups");
  }

  async getQueries(): Promise<{ queries: Array<{ id: number; name: string; is_public: boolean; project_id: number | null }> }> {
    return this.get("/queries.json", "Failed to get queries");
  }

  // ==========================================================================
  // Time Entries
  // ==========================================================================

  async getTimeEntries(params: TimeEntryListParams = {}): Promise<
    {
      time_entries: Array<{
        id: number;
        project: IdName;
        issue?: { id: number };
        user: IdName;
        activity: IdName;
        hours: number;
        comments: string;
        spent_on: string;
        created_on: string;
        updated_on: string;
      }>;
    } & PaginatedResponse
  > {
    return this.getWithQuery("/time_entries.json", params, "Failed to get time entries");
  }

  async createTimeEntry(params: TimeEntryParams): Promise<{ time_entry: { id: number } }> {
    return this.post("/time_entries.json", { time_entry: params }, "Failed to create time entry");
  }

  // ==========================================================================
  // Users
  // ==========================================================================

  async getCurrentUser(): Promise<{ user: { id: number; login: string; firstname: string; lastname: string } }> {
    return this.get("/users/current.json", "Failed to get current user");
  }

  async getUsers(params: { status?: number; name?: string; group_id?: number; limit?: number; offset?: number } = {}): Promise<{
    users: Array<{ id: number; login: string; firstname: string; lastname: string; mail: string; created_on: string; status: number }>;
    total_count: number;
  }> {
    return this.getWithQuery("/users.json", params, "Failed to get users");
  }

  async getUser(id: number): Promise<{
    user: {
      id: number;
      login: string;
      firstname: string;
      lastname: string;
      mail: string;
      created_on: string;
      last_login_on: string;
      groups?: IdName[];
      memberships?: Array<{ project: IdName; roles: IdName[] }>;
    };
  }> {
    return this.get(`/users/${id}.json?include=groups,memberships`, "Failed to get user");
  }

  // ==========================================================================
  // Wiki
  // ==========================================================================

  async getWikiPages(projectId: string): Promise<{
    wiki_pages: Array<{ title: string; version: number; created_on: string; updated_on: string }>;
  }> {
    return this.get(`/projects/${projectId}/wiki/index.json`, "Failed to get wiki pages");
  }

  async getWikiPage(projectId: string, title: string): Promise<{
    wiki_page: { title: string; text: string; version: number; author: IdName; created_on: string; updated_on: string };
  }> {
    return this.get(`/projects/${projectId}/wiki/${encodeURIComponent(title)}.json`, "Failed to get wiki page");
  }

  async updateWikiPage(projectId: string, title: string, params: WikiPageParams): Promise<{ success: boolean }> {
    return this.mutate(
      `/projects/${projectId}/wiki/${encodeURIComponent(title)}.json`,
      "PUT",
      { wiki_page: params },
      "Failed to update wiki page"
    );
  }

  // ==========================================================================
  // Files & Attachments
  // ==========================================================================

  async getFiles(projectId: string): Promise<{
    files: Array<{ id: number; filename: string; filesize: number; content_type: string; description: string; content_url: string; created_on: string }>;
  }> {
    return this.get(`/projects/${projectId}/files.json`, "Failed to get files");
  }

  async getAttachment(id: number): Promise<{
    attachment: { id: number; filename: string; filesize: number; content_type: string; description: string; content_url: string; author: IdName; created_on: string };
  }> {
    return this.get(`/attachments/${id}.json`, "Failed to get attachment");
  }

  async uploadFile(filePath: string, description?: string): Promise<{ upload: { id: number; token: string } }> {
    const file = Bun.file(filePath);
    if (!(await file.exists())) {
      throw new Error(`File does not exist: ${filePath}`);
    }

    const content = await file.arrayBuffer();
    const filename = filePath.split("/").pop() || "file";

    const queryParams = new URLSearchParams();
    queryParams.set("filename", filename);
    if (description) queryParams.set("description", description);

    const url = `${this.config.baseUrl}/uploads.json?${queryParams.toString()}`;
    const startTime = Date.now();
    log.debug(`[HTTP] POST ${url} (file: ${filename}, size: ${content.byteLength} bytes)`);

    const res = await fetch(url, {
      method: "POST",
      headers: {
        "X-Redmine-API-Key": this.config.apiKey,
        "Content-Type": "application/octet-stream",
      },
      body: content,
    });

    const elapsed = Date.now() - startTime;
    log.debug(`[HTTP] Response ${res.status} ${res.statusText} (${elapsed}ms)`);

    if (!res.ok) {
      const errorText = await res.text();
      log.debug(`[HTTP] Error Body: ${errorText}`);
      throw new Error(`Failed to upload file: ${res.status} - ${errorText}`);
    }
    const data = await res.json();
    log.debug(`[HTTP] Response Body: ${truncateForLog(data)}`);
    return data;
  }

  async downloadAttachment(attachmentId: number, savePath: string): Promise<{ saved_to: string; filename: string }> {
    const attachmentInfo = await this.getAttachment(attachmentId);
    const { content_url, filename } = attachmentInfo.attachment;

    const startTime = Date.now();
    log.debug(`[HTTP] GET ${content_url} (download attachment)`);

    const res = await fetch(content_url, {
      headers: { "X-Redmine-API-Key": this.config.apiKey },
    });

    const elapsed = Date.now() - startTime;
    log.debug(`[HTTP] Response ${res.status} ${res.statusText} (${elapsed}ms)`);

    if (!res.ok) {
      log.debug(`[HTTP] Download failed`);
      throw new Error(`Failed to download attachment: ${res.status}`);
    }

    const content = await res.arrayBuffer();
    log.debug(`[HTTP] Downloaded ${content.byteLength} bytes, saving to ${savePath}`);

    await Bun.write(savePath, content);
    return { saved_to: savePath, filename };
  }

  // ==========================================================================
  // News
  // ==========================================================================

  async getNews(projectId?: string): Promise<{
    news: Array<{ id: number; project: IdName; author: IdName; title: string; summary: string; description: string; created_on: string }>;
  }> {
    const path = projectId ? `/projects/${projectId}/news.json` : "/news.json";
    return this.get(path, "Failed to get news");
  }

  // ==========================================================================
  // Search
  // ==========================================================================

  async search(query: string, params: SearchParams = {}): Promise<
    { results: Array<{ id: number; title: string; type: string; url: string; description: string; datetime: string }> } & PaginatedResponse
  > {
    return this.getWithQuery("/search.json", { q: query, ...params }, "Failed to search");
  }

}
