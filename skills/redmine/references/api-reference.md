# Redmine MCP API Reference

Complete API tool list, **35 tools** total (Rust implementation).

## Issues

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_issues` | Get issues list | - |
| `redmine_get_issue` | Get single issue details | `id` |
| `redmine_update_issue` | Update issue | `id` |
| `redmine_get_journals` | Get issue history | `issue_id` |

### redmine_get_issues Parameters

| Parameter | Description | Example |
|-----------|-------------|---------|
| `project_id` | Project ID | `"my-project"` |
| `status_id` | Status | `"open"`, `"closed"`, `"*"` |
| `tracker_id` | Tracker ID | `20` |
| `assigned_to_id` | Assignee | `"me"`, `"!42"` |
| `limit` | Max results (max 100) | `25` |
| `offset` | Skip count | `0` |
| `sort` | Sort field | `"updated_on:desc"` |

### redmine_update_issue Parameters

| Parameter | Description |
|-----------|-------------|
| `id` | Issue ID (required) |
| `notes` | Notes (supports Textile) |
| `status_id` | Status ID |
| `assigned_to_id` | Assignee ID |
| `done_ratio` | Progress (0-100) |
| `priority_id` | Priority ID |

---

## Time Entries

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_time_entries` | Get time entries | - |
| `redmine_create_time_entry` | Create time entry | `hours` |
| `redmine_get_time_entry_activities` | Get activity types | - |

### redmine_create_time_entry Parameters

| Parameter | Description |
|-----------|-------------|
| `issue_id` | Issue ID (or project_id) |
| `project_id` | Project ID |
| `hours` | Hours (required) |
| `activity_id` | Activity type ID |
| `comments` | Comments |
| `spent_on` | Date (YYYY-MM-DD) |

---

## Versions

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_versions` | Get versions list | `project_id` |
| `redmine_get_version` | Get version details | `id` |

---

## Issue Relations

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_issue_relations` | Get relations | `issue_id` |
| `redmine_create_issue_relation` | Create relation | `issue_id`, `issue_to_id`, `relation_type` |
| `redmine_delete_issue_relation` | Delete relation | `relation_id` |

### Relation Types

| Type | Description |
|------|-------------|
| `relates` | Related to |
| `duplicates` | Duplicates |
| `blocks` | Blocks |
| `precedes` | Precedes |
| `follows` | Follows |
| `copied_to` | Copied to |
| `copied_from` | Copied from |

---

## Wiki

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_wiki_pages` | Get pages list | `project_id` |
| `redmine_get_wiki_page` | Get page content | `project_id`, `title` |
| `redmine_update_wiki_page` | Update page | `project_id`, `title`, `text` |

---

## Projects & Users

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_projects` | Get projects list | - |
| `redmine_get_project_members` | Get project members | `project_id` |
| `redmine_get_current_user` | Get current user | - |
| `redmine_get_users` | Get users list | - |
| `redmine_get_user` | Get user details | `id` |

---

## Files & Attachments

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_files` | Get project files | `project_id` |
| `redmine_get_attachment` | Get attachment info | `id` |
| `redmine_upload` | Upload file to Redmine | `file_path` |
| `redmine_download` | Download attachment | `attachment_id`, `save_path` |

---

## Search

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_search` | Full-text search | `q` |

### redmine_search Parameters

| Parameter | Description |
|-----------|-------------|
| `q` | Search keyword (required) |
| `scope` | Search scope: all/issues/wiki_pages/news |
| `project_id` | Limit to project |
| `limit` | Max results |
| `offset` | Skip count |

---

## Generic API

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_request` | Generic API call | `path` |

### redmine_request Parameters

| Parameter | Description |
|-----------|-------------|
| `path` | API path (required), e.g. `/issues.json` |
| `method` | HTTP method: get/post/put/delete |
| `data` | Request body (for POST/PUT) |
| `params` | Query parameters |

See `advanced-api.md` for detailed examples.

---

## Log Viewer

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_log_viewer` | Get Log Viewer URL | - |

### redmine_log_viewer Parameters

| Parameter | Description |
|-----------|-------------|
| `open` | Open browser (default: false) |

### Response

```json
{
  "url": "http://localhost:3456",
  "opened": true
}
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LOG_VIEWER` | `true` | Enable/disable Log Viewer |
| `LOG_VIEWER_PORT` | `3456` | Server port |
| `LOG_VIEWER_OPEN` | `true` | Auto open browser on startup |

---

## Others

| Tool | Description | Required Params |
|------|-------------|-----------------|
| `redmine_get_trackers` | Trackers list | - |
| `redmine_get_statuses` | Status list | - |
| `redmine_get_priorities` | Priorities list | - |
| `redmine_get_issue_categories` | Issue categories | `project_id` |
| `redmine_get_queries` | Saved queries | - |
| `redmine_get_roles` | Roles list | - |
| `redmine_get_groups` | Groups list | - |
| `redmine_get_news` | News list | - |

---

## Tool Count Summary

| Category | Count |
|----------|-------|
| Issues | 4 |
| Time Entries | 3 |
| Versions | 2 |
| Issue Relations | 3 |
| Wiki | 3 |
| Projects & Users | 5 |
| Files & Attachments | 4 |
| Search | 1 |
| Generic API | 1 |
| Log Viewer | 1 |
| Others | 8 |
| **Total** | **35** |
