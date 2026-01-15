# Redmine Skill 使用範例

## Issue 操作

### 查詢 Issues

```
# 我的開啟 Issues
redmine_get_issues({ assigned_to_id: "me", status_id: "open" })

# 專案的所有 Issues
redmine_get_issues({ project_id: "my-project" })

# 排除特定人員
redmine_get_issues({ assigned_to_id: "!42", status_id: "open" })
```

### 更新 Issue

```
# 新增備註
redmine_update_issue({ id: 12345, notes: "已修復" })

# 變更狀態 + 備註
redmine_update_issue({ id: 12345, status_id: 5, notes: "已解決" })

# 調整完成度
redmine_update_issue({ id: 12345, done_ratio: 50 })

# 富文本備註
redmine_update_issue({
  id: 12345,
  notes: "h2. 修復說明\n\n*問題已修復*\n\n# 修改了 A\n# 修改了 B"
})
```

---

## Time Entry 操作

### 查詢工時

```
# 我的工時
redmine_get_time_entries({ user_id: "me" })

# 專案本月工時
redmine_get_time_entries({
  project_id: "my-project",
  from: "2024-01-01",
  to: "2024-01-31"
})
```

### 記錄工時

```
# Issue 工時
redmine_create_time_entry({
  issue_id: 12345,
  hours: 2,
  comments: "修復 Bug"
})

# 專案工時
redmine_create_time_entry({
  project_id: "my-project",
  hours: 4,
  activity_id: 9
})
```

---

## Issue Relations 操作

```
# 建立阻擋關聯
redmine_create_issue_relation({
  issue_id: 12345,
  issue_to_id: 12346,
  relation_type: "blocks"
})

# 建立先後關聯（延遲 3 天）
redmine_create_issue_relation({
  issue_id: 12345,
  issue_to_id: 12347,
  relation_type: "precedes",
  delay: 3
})

# 刪除關聯
redmine_delete_issue_relation({ relation_id: 123 })
```

---

## Wiki 操作

```
# 取得頁面列表
redmine_get_wiki_pages({ project_id: "my-project" })

# 取得頁面內容
redmine_get_wiki_page({ project_id: "my-project", title: "HomePage" })

# 更新頁面
redmine_update_wiki_page({
  project_id: "my-project",
  title: "API文件",
  text: "h1. API 文件\n\n內容...",
  comments: "更新 API 說明"
})
```

---

## URL 解析範例

貼上 Redmine URL 自動轉換為 API 呼叫：

| URL | 轉換結果 |
|-----|----------|
| `/issues/12345` | `redmine_get_issue({ id: 12345 })` |
| `/projects/xxx/issues?op[status_id]=o` | `redmine_get_issues({ project_id: "xxx", status_id: "open" })` |
| `/projects/xxx/issues?op[assigned_to_id]=!&v[assigned_to_id][]=42` | `redmine_get_issues({ project_id: "xxx", assigned_to_id: "!42" })` |
