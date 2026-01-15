---
name: redmine
version: 1.0.0
description: Redmine 操作技能 - URL 解析、Issue 查詢與管理 (project)
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

Redmine 整合操作技能（31 個 API），支援 URL 解析與自然語言查詢。

> **此 Skill 是所有 Redmine MCP 工具的文檔來源。**

## 快速指令

| 使用者說 | 執行動作 |
|---------|----------|
| 我的工作 | `redmine_get_issues({ assigned_to_id: "me", status_id: "open" })` |
| 專案列表 | `redmine_get_projects()` |
| 誰是我 | `redmine_get_current_user()` |
| 回覆 Issue #123 | `redmine_update_issue({ id: 123, notes: "..." })` |
| 記錄工時 | `redmine_create_time_entry({ hours: N, ... })` |
| 搜尋「關鍵字」 | `redmine_search({ q: "關鍵字" })` |
| 下載附件 | `redmine_download({ attachment_id: N, save_path: "/path" })` |

## URL 解析

偵測 Redmine URL 類型，自動呼叫對應 API：

```
/issues/{id}              → redmine_get_issue(id)
/projects/{pid}/issues    → redmine_get_issues(project_id)
/projects/{pid}/issues?...→ redmine_get_issues + 解析篩選條件
```

### 執行流程

1. 執行 `bun run {baseDir}/scripts/parse-url.ts "<url>"`
2. 依 `type` 判斷呼叫哪個 API
3. 呼叫 MCP API 並格式化結果

### 篩選條件對照

| Web Query | API Parameter |
|-----------|---------------|
| `op[status_id]=o` | `status_id: "open"` |
| `op[status_id]=c` | `status_id: "closed"` |
| `op[assigned_to_id]=!` & `v[]=42` | `assigned_to_id: "!42"` |

完整對照表見 `{baseDir}/references/operators.md`。

## 核心 API 參數

### redmine_get_issues

取得 Issues 列表，支援多種篩選條件。

| 參數 | 類型 | 說明 | 範例 |
|------|------|------|------|
| `project_id` | string | 專案 ID 或識別碼 | `"my-project"` |
| `tracker_id` | number | 追蹤標籤 ID | `20` |
| `status_id` | string | 狀態：open/closed/*/數字 | `"open"` |
| `assigned_to_id` | string | 指派者：me/ID/!ID | `"me"`, `"!42"` |
| `limit` | number | 筆數上限 (max 100) | `25` |
| `offset` | number | 跳過筆數（分頁） | `0` |
| `sort` | string | 排序欄位 | `"updated_on:desc"` |

### redmine_get_issue

取得單一 Issue 詳情，含 journals 和 attachments。

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| `id` | number | ✓ | Issue ID |

### redmine_update_issue

更新 Issue：新增備註、變更狀態、指派者、完成度。

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| `id` | number | ✓ | Issue ID |
| `notes` | string | | 備註（支援 Textile 富文本） |
| `status_id` | number | | 狀態 ID |
| `assigned_to_id` | number | | 指派者 ID |
| `done_ratio` | number | | 完成百分比 (0-100) |
| `priority_id` | number | | 優先權 ID |

### redmine_create_time_entry

建立工時記錄。

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| `hours` | number | ✓ | 工時（小時） |
| `issue_id` | number | | Issue ID（與 project_id 二選一） |
| `project_id` | string | | 專案 ID |
| `activity_id` | number | | 活動類型 ID |
| `comments` | string | | 備註 |
| `spent_on` | string | | 日期 (YYYY-MM-DD)，預設今天 |

### redmine_search

全文搜尋 Issues、Wiki、News 等。

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| `q` | string | ✓ | 搜尋關鍵字 |
| `scope` | string | | all/issues/wiki_pages/news/... |
| `project_id` | string | | 限定專案 |
| `limit` | number | | 筆數上限 |
| `offset` | number | | 跳過筆數 |

### redmine_create_issue_relation

建立 Issue 關聯。

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| `issue_id` | number | ✓ | 來源 Issue ID |
| `issue_to_id` | number | ✓ | 目標 Issue ID |
| `relation_type` | string | ✓ | 關聯類型 |
| `delay` | number | | 延遲天數（precedes/follows） |

**關聯類型：** `relates`, `duplicates`, `blocks`, `blocked`, `precedes`, `follows`, `copied_to`, `copied_from`

## 輸出格式

### 單一 Issue

```
**Issue #{id}**
| 欄位 | 值 |
|------|-----|
| 標題 | {subject} |
| 狀態 | {status} |
| 指派 | {assigned_to} |
| 進度 | {done_ratio}% |
```

### Issues 列表

```
| ID | Subject | Status | Assigned | Updated |
|----|---------|--------|----------|---------|
```

## Resource Index

| Resource | Purpose | Load When |
|----------|---------|-----------|
| `{baseDir}/references/api-reference.md` | 完整 31 個 API 參數說明 | 需要進階參數 |
| `{baseDir}/references/file-operations.md` | 檔案上傳/下載操作 | 處理附件 |
| `{baseDir}/references/advanced-api.md` | redmine_request 進階用法 | 使用通用 API |
| `{baseDir}/references/textile-syntax.md` | Textile 富文本語法 | 撰寫格式化內容 |
| `{baseDir}/references/examples.md` | 使用範例 | 學習用法 |
| `{baseDir}/references/operators.md` | URL 篩選條件對照表 | 解析 URL 參數 |
| `{baseDir}/scripts/parse-url.ts` | URL 解析腳本 | 自動解析 URL |

---

**Redmine Skill v1.0.0**
