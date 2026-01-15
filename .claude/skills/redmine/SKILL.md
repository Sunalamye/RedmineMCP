---
name: redmine
version: 1.0.0
description: Redmine integration skill - URL parsing, issue query and management
allowed-tools:
  # Issues
  - mcp__redmine__redmine_get_issue
  - mcp__redmine__redmine_get_issues
  - mcp__redmine__redmine_update_issue
  - mcp__redmine__redmine_get_journals
  # Projects
  - mcp__redmine__redmine_get_projects
  - mcp__redmine__redmine_get_project_members
  # Trackers & Statuses
  - mcp__redmine__redmine_get_trackers
  - mcp__redmine__redmine_get_statuses
  - mcp__redmine__redmine_get_priorities
  # Users
  - mcp__redmine__redmine_get_current_user
  - mcp__redmine__redmine_get_users
  - mcp__redmine__redmine_get_user
  # Time Entries
  - mcp__redmine__redmine_get_time_entries
  - mcp__redmine__redmine_create_time_entry
  - mcp__redmine__redmine_get_time_entry_activities
  # Versions
  - mcp__redmine__redmine_get_versions
  - mcp__redmine__redmine_get_version
  # Issue Relations
  - mcp__redmine__redmine_get_issue_relations
  - mcp__redmine__redmine_create_issue_relation
  - mcp__redmine__redmine_delete_issue_relation
  # Issue Categories
  - mcp__redmine__redmine_get_issue_categories
  # Wiki
  - mcp__redmine__redmine_get_wiki_pages
  - mcp__redmine__redmine_get_wiki_page
  - mcp__redmine__redmine_update_wiki_page
  # Files & Attachments
  - mcp__redmine__redmine_get_files
  - mcp__redmine__redmine_get_attachment
  - mcp__redmine__redmine_upload
  - mcp__redmine__redmine_download
  # Search
  - mcp__redmine__redmine_search
  # Generic Request (Escape Hatch)
  - mcp__redmine__redmine_request
  # Others
  - mcp__redmine__redmine_get_queries
  - mcp__redmine__redmine_get_roles
  - mcp__redmine__redmine_get_groups
  - mcp__redmine__redmine_get_news
---

# Redmine Skill

Redmine integration skill with 31 APIs, supporting URL parsing and natural language queries.

> **This skill is the documentation source for all Redmine MCP tools.**

## Quick Commands

| User Says | Action |
|-----------|--------|
| my tasks | `redmine_get_issues({ assigned_to_id: "me", status_id: "open" })` |
| project list | `redmine_get_projects()` |
| who am I | `redmine_get_current_user()` |
| reply to Issue #123 | `redmine_update_issue({ id: 123, notes: "..." })` |
| log time | `redmine_create_time_entry({ hours: N, ... })` |
| search "keyword" | `redmine_search({ q: "keyword" })` |
| download attachment | `redmine_download({ attachment_id: N, save_path: "/path" })` |

## URL Parsing

Detect Redmine URL type and automatically call corresponding API:

```
/issues/{id}              → redmine_get_issue(id)
/projects/{pid}/issues    → redmine_get_issues(project_id)
/projects/{pid}/issues?...→ redmine_get_issues + parse filters
```

### Execution Flow

1. Run `bun run {baseDir}/scripts/parse-url.ts "<url>"`
2. Determine which API to call based on `type`
3. Call MCP API and format results

### Filter Mapping

| Web Query | API Parameter |
|-----------|---------------|
| `op[status_id]=o` | `status_id: "open"` |
| `op[status_id]=c` | `status_id: "closed"` |
| `op[assigned_to_id]=!` & `v[]=42` | `assigned_to_id: "!42"` |

See full mapping at `{baseDir}/references/operators.md`.

## Core API Parameters

### redmine_get_issues

Get issues list with various filters.

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `project_id` | string | Project ID or identifier | `"my-project"` |
| `tracker_id` | number | Tracker ID | `20` |
| `status_id` | string | Status: open/closed/*/number | `"open"` |
| `assigned_to_id` | string | Assignee: me/ID/!ID | `"me"`, `"!42"` |
| `limit` | number | Max results (max 100) | `25` |
| `offset` | number | Skip count (pagination) | `0` |
| `sort` | string | Sort field | `"updated_on:desc"` |

### redmine_get_issue

Get single issue details with journals and attachments.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | ✓ | Issue ID |

### redmine_update_issue

Update issue: add notes, change status, assignee, progress.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | number | ✓ | Issue ID |
| `notes` | string | | Notes (supports Textile markup) |
| `status_id` | number | | Status ID |
| `assigned_to_id` | number | | Assignee ID |
| `done_ratio` | number | | Progress percentage (0-100) |
| `priority_id` | number | | Priority ID |

### redmine_create_time_entry

Create time entry.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `hours` | number | ✓ | Hours spent |
| `issue_id` | number | | Issue ID (or project_id) |
| `project_id` | string | | Project ID |
| `activity_id` | number | | Activity type ID |
| `comments` | string | | Comments |
| `spent_on` | string | | Date (YYYY-MM-DD), defaults to today |

### redmine_search

Full-text search across Issues, Wiki, News, etc.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `q` | string | ✓ | Search keyword |
| `scope` | string | | all/issues/wiki_pages/news/... |
| `project_id` | string | | Limit to project |
| `limit` | number | | Max results |
| `offset` | number | | Skip count |

### redmine_create_issue_relation

Create issue relation.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `issue_id` | number | ✓ | Source Issue ID |
| `issue_to_id` | number | ✓ | Target Issue ID |
| `relation_type` | string | ✓ | Relation type |
| `delay` | number | | Delay days (precedes/follows) |

**Relation types:** `relates`, `duplicates`, `blocks`, `blocked`, `precedes`, `follows`, `copied_to`, `copied_from`

## Output Format

### Single Issue

```
**Issue #{id}**
| Field | Value |
|-------|-------|
| Subject | {subject} |
| Status | {status} |
| Assignee | {assigned_to} |
| Progress | {done_ratio}% |
```

### Issues List

```
| ID | Subject | Status | Assigned | Updated |
|----|---------|--------|----------|---------|
```

## Resource Index

| Resource | Purpose | Load When |
|----------|---------|-----------|
| `{baseDir}/references/api-reference.md` | Full 31 API parameter docs | Need advanced params |
| `{baseDir}/references/file-operations.md` | File upload/download | Handle attachments |
| `{baseDir}/references/advanced-api.md` | redmine_request usage | Use generic API |
| `{baseDir}/references/textile-syntax.md` | Textile markup syntax | Write formatted content |
| `{baseDir}/references/examples.md` | Usage examples | Learn usage |
| `{baseDir}/references/operators.md` | URL filter mapping | Parse URL params |
| `{baseDir}/scripts/parse-url.ts` | URL parsing script | Auto-parse URLs |

---

**Redmine Skill v1.0.0**
