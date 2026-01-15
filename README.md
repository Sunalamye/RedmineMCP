# Redmine MCP

MCP Server for Redmine integration. Works with Claude Code and OpenCode.

## Features

- Get issues list with filters (project, status, assignee, tracker)
- Get single issue details with history and attachments
- Update issues (add notes, change status, assignee, etc.)
- Log time entries
- Search across issues, wiki, and news
- Manage wiki pages
- Upload and download attachments
- 34 API tools in total

## Download

| Version | File | Size | Requirements |
|---------|------|------|--------------|
| Source | `redmine-mcp-src.zip` | 12K | Bun required |
| macOS Binary | `redmine-mcp-macos.zip` | 21M | Ready to run |

## Installation

### Quick Install (Recommended)

Download and run the install script:

```bash
unzip redmine-mcp-*.zip
./install.sh
```

The script will guide you through:
1. Choose Claude Code or OpenCode
2. Set installation directory
3. Enter Redmine URL and API Token
4. Generate configuration file

### Manual Installation

#### Option A: Binary Version

1. Download `redmine-mcp-macos.zip`
2. Extract the archive
3. Continue to "Getting Your API Token"

#### Option B: Source Version

1. Download `redmine-mcp-src.zip`
2. Extract the archive
3. Install Bun:

**macOS / Linux:**
```bash
curl -fsSL https://bun.sh/install | bash
```

**Windows:**
```powershell
powershell -c "irm bun.sh/install.ps1 | iex"
```

4. Install dependencies:
```bash
bun install
```

## Getting Your API Token

1. Log in to Redmine
2. Click your account name (top-right) → **My account**
3. Find **API access key** on the right side
4. Click **Show** to reveal your token

```
My account page
├── Left: Personal information
└── Right: API access key
         └── [Show] button ← Click here
```

## Configuration

### Environment Variables

| Variable | Description |
|----------|-------------|
| `REDMINE_URL` | Your Redmine URL |
| `REDMINE_TOKEN` | Your API Token |

### Claude Code

Create `.mcp.json` in your project directory:

**Binary version:**
```json
{
  "mcpServers": {
    "redmine": {
      "command": "/path/to/redmine-mcp",
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

**Source version:**
```json
{
  "mcpServers": {
    "redmine": {
      "command": "bun",
      "args": ["run", "/path/to/src/index.ts"],
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

### OpenCode

Create `opencode.json` in your project directory:

**Binary version:**
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

**Source version:**
```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "redmine": {
      "type": "local",
      "command": ["bun", "run", "./src/index.ts"],
      "enabled": true,
      "environment": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-api-token-here"
      }
    }
  }
}
```

## Usage

### Available Tools

| Tool | Description |
|------|-------------|
| `redmine_get_issues` | Get issues list |
| `redmine_get_issue` | Get single issue details |
| `redmine_update_issue` | Update issue (notes, status, etc.) |
| `redmine_get_projects` | Get projects list |
| `redmine_get_trackers` | Get trackers list |
| `redmine_get_statuses` | Get status list |
| `redmine_get_project_members` | Get project members |
| `redmine_get_current_user` | Get current user info |
| `redmine_create_time_entry` | Log time entry |
| `redmine_search` | Full-text search |

### Examples

```
# Get all open issues
redmine_get_issues(status_id: "open")

# Get issues for a specific project
redmine_get_issues(project_id: "my-project")

# Get single issue details
redmine_get_issue(id: 12345)

# Get issues assigned to me
redmine_get_issues(assigned_to_id: "me")

# Exclude specific user
redmine_get_issues(assigned_to_id: "!42")

# Add a note to an issue
redmine_update_issue(id: 12345, notes: "Task completed")

# Log 2 hours on an issue
redmine_create_time_entry(issue_id: 12345, hours: 2, comments: "Bug fix")
```

## Security Notes

- `.mcp.json` and `opencode.json` contain your API token
- **Do not commit these files to version control**
- Use example files as templates (they contain no real tokens)

## License

MIT
