# CLAUDE.md

Guidance for Claude Code when working with this repository.

## Project Overview

Redmine MCP Server - High-performance Rust implementation with 35 API tools.

## Commands

```bash
cd rust
cargo build --release    # Build release binary
cargo run               # Run MCP server
./build-release.sh      # Cross-compile for multiple platforms
```

## Architecture

```
rust/src/
├── main.rs            # Entry point, JSON-RPC loop
├── lib.rs             # Library exports
├── config.rs          # Configuration (env vars)
├── error.rs           # Error types (thiserror)
├── client/
│   ├── mod.rs         # Redmine API client (async reqwest)
│   └── types.rs       # API response types (serde)
├── log_viewer/
│   ├── mod.rs         # HTTP/WebSocket server (axum)
│   └── ui.rs          # HTML/CSS/JS for web UI
└── tools/
    ├── mod.rs         # MCP tool implementations
    └── params.rs      # Tool definitions (JSON Schema)
```

**Data flow:** stdin → JSON-RPC parse → `tools/mod.rs` match → `RedmineClient` async → Redmine API → JSON response → stdout

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `REDMINE_URL` | Yes | - | Redmine base URL |
| `REDMINE_TOKEN` | Yes | - | API token |
| `LOG_LEVEL` | No | `info` | debug/info/warn/error |
| `LOG_VIEWER` | No | `true` | Enable Log Viewer |
| `LOG_VIEWER_PORT` | No | `3456` | Log Viewer port |
| `LOG_VIEWER_OPEN` | No | `true` | Auto open browser |

## Key MCP Tools (35 total)

| Category | Tools |
|----------|-------|
| Issues | `redmine_get_issues`, `redmine_get_issue`, `redmine_update_issue`, `redmine_get_journals` |
| Time | `redmine_get_time_entries`, `redmine_create_time_entry`, `redmine_get_time_entry_activities` |
| Projects | `redmine_get_projects`, `redmine_get_project_members` |
| Users | `redmine_get_current_user`, `redmine_get_users`, `redmine_get_user` |
| Wiki | `redmine_get_wiki_pages`, `redmine_get_wiki_page`, `redmine_update_wiki_page` |
| Files | `redmine_get_files`, `redmine_get_attachment`, `redmine_upload`, `redmine_download` |
| Search | `redmine_search` |
| Generic | `redmine_request` (custom API calls) |
| Utilities | `redmine_log_viewer` |

## Custom Commands

### /redmine

Parses Redmine web URL and executes corresponding API call.

| Web UI Parameter | API Parameter |
|-----------------|---------------|
| `op[status_id]=o` | `status_id: "open"` |
| `op[status_id]=c` | `status_id: "closed"` |
| `v[tracker_id][]=20` | `tracker_id: 20` |
| `op[assigned_to_id]=!` & `v[]=42` | `assigned_to_id: "!42"` |

## Skill Reference

Full API documentation: `skills/redmine/references/api-reference.md`
