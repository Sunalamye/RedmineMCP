# Redmine MCP API Reference

完整 API 工具列表，共 27 個。

## Issues

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_issues` | 取得 Issues 列表 | - |
| `redmine_get_issue` | 取得單一 Issue 詳情 | `id` |
| `redmine_update_issue` | 更新 Issue | `id` |
| `redmine_get_journals` | 取得 Issue 歷史記錄 | `issue_id` |

### redmine_get_issues 參數

| 參數 | 說明 | 範例 |
|------|------|------|
| `project_id` | 專案 ID | `"my-project"` |
| `status_id` | 狀態 | `"open"`, `"closed"`, `"*"` |
| `tracker_id` | 追蹤標籤 ID | `20` |
| `assigned_to_id` | 指派者 | `"me"`, `"!42"` |
| `limit` | 筆數 (max 100) | `25` |
| `offset` | 跳過筆數 | `0` |
| `sort` | 排序 | `"updated_on:desc"` |

### redmine_update_issue 參數

| 參數 | 說明 |
|------|------|
| `id` | Issue ID（必填） |
| `notes` | 備註（支援 Textile） |
| `status_id` | 狀態 ID |
| `assigned_to_id` | 指派者 ID |
| `done_ratio` | 完成度 (0-100) |
| `priority_id` | 優先權 ID |

---

## Time Entries

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_time_entries` | 取得工時記錄 | - |
| `redmine_create_time_entry` | 建立工時記錄 | `hours` |
| `redmine_get_time_entry_activities` | 取得活動類型 | - |

### redmine_create_time_entry 參數

| 參數 | 說明 |
|------|------|
| `issue_id` | Issue ID（與 project_id 二選一） |
| `project_id` | 專案 ID |
| `hours` | 工時（必填） |
| `activity_id` | 活動類型 ID |
| `comments` | 備註 |
| `spent_on` | 日期 (YYYY-MM-DD) |

---

## Versions

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_versions` | 取得版本列表 | `project_id` |
| `redmine_get_version` | 取得版本詳情 | `id` |

---

## Issue Relations

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_issue_relations` | 取得關聯 | `issue_id` |
| `redmine_create_issue_relation` | 建立關聯 | `issue_id`, `issue_to_id`, `relation_type` |
| `redmine_delete_issue_relation` | 刪除關聯 | `relation_id` |

### 關聯類型

| 類型 | 說明 |
|------|------|
| `relates` | 相關 |
| `duplicates` | 重複 |
| `blocks` | 阻擋 |
| `precedes` | 先於 |
| `follows` | 後於 |
| `copied_to` | 複製至 |
| `copied_from` | 複製自 |

---

## Wiki

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_wiki_pages` | 取得頁面列表 | `project_id` |
| `redmine_get_wiki_page` | 取得頁面內容 | `project_id`, `title` |
| `redmine_update_wiki_page` | 更新頁面 | `project_id`, `title`, `text` |

---

## Projects & Users

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_projects` | 取得專案列表 | - |
| `redmine_get_project_members` | 取得專案成員 | `project_id` |
| `redmine_get_current_user` | 取得當前使用者 | - |
| `redmine_get_users` | 取得使用者列表 | - |
| `redmine_get_user` | 取得使用者詳情 | `id` |

---

## Others

| 工具 | 說明 | 必填參數 |
|------|------|----------|
| `redmine_get_trackers` | 追蹤標籤列表 | - |
| `redmine_get_statuses` | 狀態列表 | - |
| `redmine_get_priorities` | 優先權列表 | - |
| `redmine_get_issue_categories` | Issue 分類 | `project_id` |
| `redmine_get_queries` | 已存查詢 | - |
| `redmine_get_roles` | 角色列表 | - |
| `redmine_get_groups` | 群組列表 | - |
| `redmine_get_files` | 專案檔案 | `project_id` |
| `redmine_get_attachment` | 附件資訊 | `id` |
| `redmine_get_news` | 新聞列表 | - |
