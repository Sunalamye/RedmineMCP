# Redmine Skill Usage Examples

## Issue Operations

### Query Issues

```
# My open issues
redmine_get_issues({ assigned_to_id: "me", status_id: "open" })

# All project issues
redmine_get_issues({ project_id: "my-project" })

# Exclude specific user
redmine_get_issues({ assigned_to_id: "!42", status_id: "open" })
```

### Update Issue

```
# Add a note
redmine_update_issue({ id: 12345, notes: "Fixed" })

# Change status + add note
redmine_update_issue({ id: 12345, status_id: 5, notes: "Resolved" })

# Update progress
redmine_update_issue({ id: 12345, done_ratio: 50 })

# Rich text note
redmine_update_issue({
  id: 12345,
  notes: "h2. Fix Summary\n\n*Issue fixed*\n\n# Modified file A\n# Modified file B"
})
```

---

## Time Entry Operations

### Query Time Entries

```
# My time entries
redmine_get_time_entries({ user_id: "me" })

# Project monthly time
redmine_get_time_entries({
  project_id: "my-project",
  from: "2024-01-01",
  to: "2024-01-31"
})
```

### Log Time

```
# Issue time entry
redmine_create_time_entry({
  issue_id: 12345,
  hours: 2,
  comments: "Bug fix"
})

# Project time entry
redmine_create_time_entry({
  project_id: "my-project",
  hours: 4,
  activity_id: 9
})
```

---

## Issue Relations Operations

```
# Create blocking relation
redmine_create_issue_relation({
  issue_id: 12345,
  issue_to_id: 12346,
  relation_type: "blocks"
})

# Create precedence relation (3 day delay)
redmine_create_issue_relation({
  issue_id: 12345,
  issue_to_id: 12347,
  relation_type: "precedes",
  delay: 3
})

# Delete relation
redmine_delete_issue_relation({ relation_id: 123 })
```

---

## Wiki Operations

```
# Get page list
redmine_get_wiki_pages({ project_id: "my-project" })

# Get page content
redmine_get_wiki_page({ project_id: "my-project", title: "HomePage" })

# Update page
redmine_update_wiki_page({
  project_id: "my-project",
  title: "API-Docs",
  text: "h1. API Documentation\n\nContent...",
  comments: "Updated API docs"
})
```

---

## URL Parsing Examples

Paste Redmine URL to automatically convert to API call:

| URL | Result |
|-----|--------|
| `/issues/12345` | `redmine_get_issue({ id: 12345 })` |
| `/projects/xxx/issues?op[status_id]=o` | `redmine_get_issues({ project_id: "xxx", status_id: "open" })` |
| `/projects/xxx/issues?op[assigned_to_id]=!&v[assigned_to_id][]=42` | `redmine_get_issues({ project_id: "xxx", assigned_to_id: "!42" })` |
