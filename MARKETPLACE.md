# Marketplace 提交指南

本文檔說明如何將 Redmine MCP 發布到各個 marketplace。

## 發布順序（建議）

1. **Claude Code Plugin Marketplace** - 完整 MCP + Skills 整合
2. **Smithery.ai** - 最大的 MCP 生態系統
3. **Cline Marketplace** - 400萬+ VS Code 用戶
4. **npm** - 標準 Node.js 安裝方式
5. **PulseMCP / LobeHub** - 被動發現

---

## 1. Claude Code Plugin Marketplace

**目標：** [github.com/anthropics/claude-plugins-official](https://github.com/anthropics/claude-plugins-official)

### 提交步驟

```bash
# 1. Fork the repository
# 2. 在 external_plugins/ 目錄新增你的 plugin

# 3. 創建 PR
git clone https://github.com/YOUR_USERNAME/claude-plugins-official
cd claude-plugins-official
mkdir -p external_plugins/redmine-mcp

# 4. 添加必要文件
cp /path/to/redmine-mcp/.claude-plugin/plugin.json external_plugins/redmine-mcp/
# 添加 README 和其他資訊

# 5. 提交 PR
git add .
git commit -m "Add redmine-mcp plugin"
git push origin main
# 在 GitHub 上創建 Pull Request
```

### 所需文件
- `.claude-plugin/plugin.json` ✅ 已創建
- `skills/` 目錄 ✅ 已創建

---

## 2. Smithery.ai

**目標：** [smithery.ai](https://smithery.ai/)

### 提交步驟

```bash
# 1. 安裝 Smithery CLI
npm install -g @smithery/cli

# 2. 登入
smithery login

# 3. 發布
cd /path/to/redmine-mcp
smithery publish
```

### 所需文件
- `smithery.yaml` ✅ 已創建
- `Dockerfile` ✅ 已創建

### 注意事項
- Smithery 優先支援 HTTP transport（STDIO 已棄用）
- 需要 Docker 支援

---

## 3. Cline MCP Marketplace

**目標：** [github.com/cline/mcp-marketplace](https://github.com/cline/mcp-marketplace)

### 提交步驟

1. 準備 400x400 PNG logo
2. 在 GitHub 創建 Issue，使用以下模板：

```markdown
## MCP Server Submission

**Name:** redmine-mcp
**Description:** MCP Server for Redmine with 34 API tools
**Repository:** https://github.com/soane/redmine-mcp
**Logo:** [附上 400x400 PNG]

### Features
- Issue tracking and management
- Time entry logging
- Wiki page management
- File upload/download
- Full-text search
- 34 API tools total

### Installation
See llms-install.md for AI-assisted setup instructions.
```

### 所需文件
- `llms-install.md` ✅ 已創建
- README.md ✅ 已存在
- 400x400 PNG logo ⚠️ 需要創建

---

## 4. npm 發布

### 提交步驟

```bash
# 1. 確保已登入 npm
npm login

# 2. 檢查 package.json
npm pack --dry-run

# 3. 發布
npm publish
```

### 所需文件
- `package.json` ✅ 已更新
  - 已添加 bin, files, keywords, repository 等欄位

### 發布後使用

```json
{
  "mcpServers": {
    "redmine": {
      "command": "npx",
      "args": ["-y", "redmine-mcp"],
      "env": {
        "REDMINE_URL": "https://your-redmine.example.com",
        "REDMINE_TOKEN": "your-token"
      }
    }
  }
}
```

---

## 5. PulseMCP

**目標：** [pulsemcp.com/servers](https://www.pulsemcp.com/servers)

### 提交步驟

1. 訪問 PulseMCP 網站
2. 點擊 "Submit Server"
3. 填寫表單：
   - Repository URL: `https://github.com/soane/redmine-mcp`
   - Description: MCP Server for Redmine with 34 API tools

無需額外文件，PulseMCP 會自動索引 GitHub repo。

---

## 6. LobeHub MCP

**目標：** [lobehub.com/mcp](https://lobehub.com/mcp)

### 提交步驟

1. 訪問 LobeHub MCP marketplace
2. 按照社區提交流程

---

## 文件清單

| 文件 | 用途 | 狀態 |
|------|------|------|
| `.claude-plugin/plugin.json` | Claude Code Plugin | ✅ |
| `smithery.yaml` | Smithery | ✅ |
| `Dockerfile` | Smithery | ✅ |
| `llms-install.md` | Cline | ✅ |
| `package.json` | npm | ✅ |
| `.mcp.json.example` | 使用者範例 | ✅ |
| `skills/redmine/` | Claude Code Skills | ✅ |
| Logo (400x400 PNG) | Cline | ⚠️ 需要 |

---

## GitHub Repository 優化

發布前建議優化 GitHub repo：

```bash
# 添加 topics
# 在 GitHub repo 設定頁面添加以下 topics:
# mcp, redmine, claude-code, cline, ai-tools, issue-tracking
```

---

## 維護建議

1. **版本同步** - 更新時同時更新所有平台的版本號
2. **CHANGELOG** - 保持更新日誌
3. **文檔** - 確保 README 和各平台文檔同步
