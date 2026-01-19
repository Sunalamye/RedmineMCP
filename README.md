# Redmine MCP

MCP Server for Redmine integration. A high-performance Rust implementation with 35 API tools.

## Features

- **35 API Tools** - Complete Redmine API coverage
- **Log Viewer** - Real-time web-based log viewer with WebSocket support
- **Cross-Platform** - Pre-built binaries for macOS, Linux, and Windows
- **High Performance** - Native Rust implementation with async I/O

### Tool Categories

| Category | Tools |
|----------|-------|
| Issues | get_issues, get_issue, update_issue, get_journals |
| Time Entries | get_time_entries, create_time_entry, get_activities |
| Projects & Users | get_projects, get_members, get_users, get_current_user |
| Wiki | get_wiki_pages, get_wiki_page, update_wiki_page |
| Files | get_files, get_attachment, upload, download |
| Search | search (full-text across issues/wiki/news) |
| Generic | request (custom API calls) |
| Utilities | log_viewer |

## Download

Pre-built binaries are available from the [Releases](https://github.com/soane/redmine-mcp/releases) page.

| Platform | File | Architecture |
|----------|------|--------------|
| macOS | `redmine-mcp-*-macos-arm64` | Apple Silicon (M1/M2/M3) |
| Linux | `redmine-mcp-*-linux-arm64` | ARM64 |
| Windows | `redmine-mcp-*-windows-x64.exe` | x64 |

## Quick Start

### 1. Get Your API Token

1. Log in to your Redmine instance
2. Go to **My account** (click your username)
3. Find **API access key** on the right sidebar
4. Click **Show** to reveal your token

### 2. Configure MCP Client

#### Claude Code

Create `.mcp.json` in your project directory:

```json
{
  "mcpServers": {
    "redmine": {
      "command": "/path/to/redmine-mcp",
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here",
        "LOG_VIEWER": "true"
      }
    }
  }
}
```

#### OpenCode

Create `opencode.json` in your project directory:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "redmine": {
      "type": "local",
      "command": ["/path/to/redmine-mcp"],
      "enabled": true,
      "environment": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `REDMINE_URL` | Yes | - | Redmine instance URL |
| `REDMINE_TOKEN` | Yes | - | API token |
| `LOG_LEVEL` | No | `info` | Log level: debug/info/warn/error |
| `LOG_VIEWER` | No | `true` | Enable Log Viewer web UI |
| `LOG_VIEWER_PORT` | No | `3456` | Log Viewer server port |
| `LOG_VIEWER_OPEN` | No | `true` | Auto-open browser on startup |

## Usage Examples

```bash
# Get all open issues
redmine_get_issues(status_id: "open")

# Get issues assigned to me
redmine_get_issues(assigned_to_id: "me")

# Get single issue with history
redmine_get_issue(id: 12345)

# Add a note to an issue
redmine_update_issue(id: 12345, notes: "Task completed")

# Log time on an issue
redmine_create_time_entry(issue_id: 12345, hours: 2, comments: "Bug fix")

# Full-text search
redmine_search(q: "authentication", scope: "issues")

# Open Log Viewer in browser
redmine_log_viewer(open: true)
```

## Log Viewer

The built-in Log Viewer provides real-time monitoring of MCP requests:

- **Real-time updates** via WebSocket
- **Filter by level** (DEBUG/INFO/WARN/ERROR)
- **Filter by tool** name
- **JSON viewer** for request/response data
- **Auto-redaction** of sensitive data (API tokens)

Access at `http://localhost:3456` when the server is running.

## Building from Source

### Prerequisites

- Rust 1.75+ (with `cargo`)
- For cross-compilation: `cargo-zigbuild`

### Build

```bash
cd rust

# Debug build
cargo build

# Release build
cargo build --release

# Cross-compile for multiple platforms
./build-release.sh
```

## Testing

A Docker Compose file is provided for local testing:

```bash
cd rust/test
docker-compose up -d

# Access Redmine at http://localhost:3000
# Default login: admin / admin

# Enable API: Administration → Settings → API → Enable REST web service
```

## Security Notes

- `.mcp.json` and `opencode.json` contain your API token
- **Do not commit these files to version control**
- The Log Viewer automatically redacts API tokens in logs

## License

MIT
