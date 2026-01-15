# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MCP Server for Redmine integration. Provides 31 API tools for Claude Code and OpenCode.

## Commands

```bash
bun run start      # Run MCP server
bun run dev        # Watch mode
bun run build      # Build to dist/
bun run typecheck  # Type check
```

## Architecture

```
src/
├── index.ts           # Entry point, MCP server setup
├── logger.ts          # Singleton logger (file + console)
├── redmine-client.ts  # Redmine API client (all HTTP calls)
└── tools/
    ├── definitions.ts # Tool schemas (inputSchema for each tool)
    └── handlers.ts    # Tool handlers (switch on tool name)
```

**Data flow:** MCP request → `handlers.ts` → `RedmineClient` method → Redmine API → JSON response

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `REDMINE_URL` | Yes | Redmine base URL |
| `REDMINE_TOKEN` | Yes | API token |
| `LOG_FILE` | No | Log file path (default: `/tmp/redmine-mcp.log`) |
| `LOG_LEVEL` | No | debug/info/warn/error (default: info) |

## Custom Commands

### /redmine

Parses Redmine web URL and executes corresponding API call.

**URL Parameter Mapping:**

| Web UI | API |
|--------|-----|
| `op[status_id]=o` | `status_id: "open"` |
| `op[status_id]=c` | `status_id: "closed"` |
| `v[tracker_id][]=20` | `tracker_id: 20` |
| `op[assigned_to_id]=!` & `v[]=42` | `assigned_to_id: "!42"` |

## Key MCP Tools

- `redmine_get_issues` - List issues with filters (project_id, status_id, assigned_to_id, tracker_id)
- `redmine_get_issue` - Single issue with journals and attachments
- `redmine_update_issue` - Add notes, change status/assignee
- `redmine_create_time_entry` - Log time entries
- `redmine_search` - Full-text search across issues/wiki/news
- `redmine_upload` / `redmine_download` - File operations

Full API reference: `.claude/skills/redmine/references/api-reference.md`
