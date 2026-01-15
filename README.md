# Redmine MCP

MCP Server for Redmine integration, supporting Claude Code and OpenCode.

## 功能

- 取得 Issues 列表（支援篩選條件）
- 取得單一 Issue 詳情（含歷史紀錄與附件）
- 取得專案、追蹤標籤、狀態列表
- 取得專案成員
- 取得當前使用者資訊

## 下載

| 版本 | 檔案 | 大小 | 需求 |
|------|------|------|------|
| 原始碼版 | `redmine-mcp-src.zip` | 12K | 需安裝 Bun |
| macOS 執行檔 | `redmine-mcp-macos.zip` | 21M | 免安裝，直接執行 |

## 安裝

### 快速安裝（推薦）

下載任一版本後，執行安裝腳本：

```bash
unzip redmine-mcp-*.zip
./install.sh
```

安裝腳本會引導你：
1. 選擇 Claude Code 或 OpenCode
2. 指定安裝目錄
3. 輸入 Redmine URL 和 API Token
4. 自動產生設定檔

### 手動安裝

#### 方式 A：使用執行檔

1. 下載 `redmine-mcp-macos.zip`
2. 解壓縮
3. 跳至「取得 API Token」步驟

#### 方式 B：使用原始碼

1. 下載 `redmine-mcp-src.zip`
2. 解壓縮
3. 安裝 Bun：

**macOS / Linux:**
```bash
curl -fsSL https://bun.sh/install | bash
```

**Windows:**
```powershell
powershell -c "irm bun.sh/install.ps1 | iex"
```

4. 安裝依賴：
```bash
bun install
```

## 取得 API Token

1. 登入 Redmine
2. 點擊右上角帳號 → **我的帳戶**
3. 在頁面右側找到 **API 存取金鑰**
4. 點擊 **顯示** 按鈕取得你的 API Token

```
我的帳戶頁面
├── 左側：個人資訊
└── 右側：API 存取金鑰
         └── [顯示] 按鈕 ← 點這裡
```

## 設定 MCP

### Claude Code

建立 `.mcp.json`：

**執行檔版：**
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

**原始碼版：**
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

建立 `opencode.json`：

**執行檔版：**
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

**原始碼版：**
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

## 使用方式

### 啟動 MCP Server

```bash
bun run src/index.ts
```

### 可用工具

| Tool | Description |
|------|-------------|
| `redmine_get_issues` | 取得 Issues 列表 |
| `redmine_get_issue` | 取得單一 Issue 詳情 |
| `redmine_get_projects` | 取得專案列表 |
| `redmine_get_trackers` | 取得追蹤標籤列表 |
| `redmine_get_statuses` | 取得狀態列表 |
| `redmine_get_project_members` | 取得專案成員 |
| `redmine_get_current_user` | 取得當前使用者 |

### 範例

```
# 取得所有開啟的 Issues
redmine_get_issues(status_id: "open")

# 取得特定專案的 Issues
redmine_get_issues(project_id: "my-project")

# 取得單一 Issue
redmine_get_issue(id: 12345)

# 取得指派給自己的 Issues
redmine_get_issues(assigned_to_id: "me")

# 排除特定使用者
redmine_get_issues(assigned_to_id: "!42")
```

## 檔案說明

| 檔案 | 說明 |
|------|------|
| `install.sh` | 互動式安裝腳本 |
| `USAGE.md` | 詳細使用指南 |
| `redmine-mcp` | 執行檔（僅 macOS 版） |
| `.mcp-E.json` | Claude Code 設定範例（執行檔版） |
| `.mcp-E-src.json` | Claude Code 設定範例（原始碼版） |
| `opencodeE.json` | OpenCode 設定範例（執行檔版） |
| `opencodeE-src.json` | OpenCode 設定範例（原始碼版） |
| `src/index.ts` | MCP Server 主程式 |
| `src/redmine-client.ts` | Redmine API Client |
| `.claude/skills/redmine/` | Redmine URL 解析 Skill |

## 注意事項

- `.mcp.json` 和 `opencode.json` 包含你的 API Token，**請勿提交到版本控制**
- 範例檔案 `.mcp-E.json` 和 `opencodeE.json` 不含真實 Token，可安全分享

## 環境變數

| 變數 | 說明 |
|------|------|
| `REDMINE_URL` | Redmine 網址 |
| `REDMINE_TOKEN` | API Token |

## License

MIT
