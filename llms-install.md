# Redmine MCP Server - Installation Instructions

This document helps AI assistants (like Cline) automatically set up the Redmine MCP server.

## Prerequisites

- **Bun** runtime (or Node.js 18+)
- **Redmine** instance with REST API enabled
- **API Token** from Redmine (My account > API access key)

## Installation Steps

### Step 1: Clone the repository

```bash
git clone https://github.com/soane/redmine-mcp.git
cd redmine-mcp
```

### Step 2: Install dependencies

```bash
bun install
```

Or with npm:
```bash
npm install
```

### Step 3: Configure MCP client

Add to your MCP configuration file:

**For Claude Code** (`.mcp.json` or `~/.claude/settings.json`):

```json
{
  "mcpServers": {
    "redmine": {
      "command": "bun",
      "args": ["run", "/path/to/redmine-mcp/src/index.ts"],
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

**For Cline** (VS Code settings):

```json
{
  "cline.mcpServers": {
    "redmine": {
      "command": "bun",
      "args": ["run", "/path/to/redmine-mcp/src/index.ts"],
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

## Environment Variables

| Variable | Required | Description |
|----------|----------|-------------|
| `REDMINE_URL` | Yes | Your Redmine instance URL |
| `REDMINE_TOKEN` | Yes | API token from My account page |
| `LOG_LEVEL` | No | debug/info/warn/error (default: info) |

## Getting Your API Token

1. Log in to your Redmine instance
2. Click your username (top-right) → **My account**
3. Find **API access key** on the right sidebar
4. Click **Show** to reveal your token

## Available Tools (34 total)

### Issues
- `redmine_get_issues` - List issues with filters
- `redmine_get_issue` - Get single issue details
- `redmine_update_issue` - Update issue (notes, status, assignee)
- `redmine_get_journals` - Get issue history

### Time Tracking
- `redmine_get_time_entries` - List time entries
- `redmine_create_time_entry` - Log time
- `redmine_get_time_entry_activities` - List activity types

### Projects & Users
- `redmine_get_projects` - List projects
- `redmine_get_project_members` - List project members
- `redmine_get_current_user` - Current user info
- `redmine_get_users` - List users
- `redmine_get_user` - Get user details

### Wiki & Files
- `redmine_get_wiki_pages` - List wiki pages
- `redmine_get_wiki_page` - Get wiki page content
- `redmine_update_wiki_page` - Update wiki page
- `redmine_upload` - Upload file
- `redmine_download` - Download attachment

### Search & Metadata
- `redmine_search` - Full-text search
- `redmine_get_trackers` - List trackers
- `redmine_get_statuses` - List statuses
- `redmine_get_priorities` - List priorities

## Verification

After installation, verify the server works:

```bash
# Test the server starts correctly
bun run src/index.ts
```

The server should output: `Redmine MCP Server running on stdio`

## Troubleshooting

**API Token not working:**
- Ensure REST API is enabled in Redmine: Administration → Settings → API → Enable REST web service

**Connection refused:**
- Check `REDMINE_URL` is correct and accessible
- Verify there's no firewall blocking the connection

**Permission denied:**
- Your API token may not have sufficient permissions
- Check your Redmine user role and project memberships
