#!/bin/bash

# Redmine MCP Installer
# 支援 Claude Code 和 OpenCode

set -e

echo "============================================"
echo "  Redmine MCP 安裝程式"
echo "============================================"
echo ""

# 檢測是否為執行檔版本
if [ -f "./redmine-mcp" ]; then
    IS_EXECUTABLE=true
    echo "✓ 偵測到執行檔版本"
else
    IS_EXECUTABLE=false
    echo "✓ 偵測到原始碼版本"

    # 檢查 Bun 是否已安裝
    if ! command -v bun &> /dev/null; then
        echo ""
        echo "⚠ 未偵測到 Bun，是否要安裝？(y/n)"
        read -r INSTALL_BUN
        if [ "$INSTALL_BUN" = "y" ] || [ "$INSTALL_BUN" = "Y" ]; then
            echo "正在安裝 Bun..."
            curl -fsSL https://bun.sh/install | bash
            export BUN_INSTALL="$HOME/.bun"
            export PATH="$BUN_INSTALL/bin:$PATH"
        else
            echo "請先安裝 Bun 後再執行此腳本"
            exit 1
        fi
    fi
fi

echo ""

# 選擇工具
echo "請選擇要設定的工具："
echo "  1) Claude Code"
echo "  2) OpenCode"
echo ""
read -p "選擇 (1 或 2): " TOOL_CHOICE

case $TOOL_CHOICE in
    1)
        TOOL_NAME="Claude Code"
        CONFIG_FILE=".mcp.json"
        ;;
    2)
        TOOL_NAME="OpenCode"
        CONFIG_FILE="opencode.json"
        ;;
    *)
        echo "無效選擇"
        exit 1
        ;;
esac

echo ""
echo "✓ 選擇了 $TOOL_NAME"
echo ""

# 輸入安裝目錄
echo "請輸入 MCP 安裝目錄（按 Enter 使用預設位置）"
if [ "$TOOL_CHOICE" = "1" ]; then
    DEFAULT_DIR="$HOME/.claude/mcp/redmine"
else
    DEFAULT_DIR="$HOME/.opencode/mcp/redmine"
fi
echo "預設: $DEFAULT_DIR"
read -p "安裝目錄: " INSTALL_DIR

if [ -z "$INSTALL_DIR" ]; then
    INSTALL_DIR="$DEFAULT_DIR"
fi

# 展開 ~ 符號
INSTALL_DIR="${INSTALL_DIR/#\~/$HOME}"

echo ""
echo "✓ 安裝目錄: $INSTALL_DIR"
echo ""

# 輸入 Redmine 設定
read -p "Redmine URL: " REDMINE_URL
if [ -z "$REDMINE_URL" ]; then
    echo "⚠ Redmine URL 不可為空"
    exit 1
fi

echo ""
echo "請輸入你的 Redmine API Token"
echo "（在 Redmine → 我的帳戶 → 右側 API 存取金鑰 → 顯示）"
read -p "API Token: " REDMINE_TOKEN

if [ -z "$REDMINE_TOKEN" ]; then
    echo "⚠ API Token 不可為空"
    exit 1
fi

echo ""
echo "============================================"
echo "  確認安裝設定"
echo "============================================"
echo "工具:        $TOOL_NAME"
echo "安裝目錄:    $INSTALL_DIR"
echo "Redmine URL: $REDMINE_URL"
echo "API Token:   ${REDMINE_TOKEN:0:8}..."
echo "============================================"
echo ""
read -p "確認安裝？(y/n): " CONFIRM

if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    echo "安裝已取消"
    exit 0
fi

# 建立目錄
echo ""
echo "正在建立目錄..."
mkdir -p "$INSTALL_DIR"

# 複製檔案
echo "正在複製檔案..."
if [ "$IS_EXECUTABLE" = true ]; then
    cp ./redmine-mcp "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/redmine-mcp"
else
    cp -r ./src "$INSTALL_DIR/"
    cp ./package.json "$INSTALL_DIR/"
    cp ./tsconfig.json "$INSTALL_DIR/" 2>/dev/null || true

    # 安裝依賴
    echo "正在安裝依賴..."
    cd "$INSTALL_DIR" && bun install
    cd - > /dev/null
fi

# 複製 skill（如果存在）
if [ -d "./.claude/skills/redmine" ]; then
    echo "正在複製 Redmine skill..."
    mkdir -p "$INSTALL_DIR/.claude/skills"
    cp -r ./.claude/skills/redmine "$INSTALL_DIR/.claude/skills/"
fi

# 產生設定檔
echo "正在產生設定檔..."

if [ "$TOOL_CHOICE" = "1" ]; then
    # Claude Code
    if [ "$IS_EXECUTABLE" = true ]; then
        cat > "$INSTALL_DIR/$CONFIG_FILE" << EOF
{
  "mcpServers": {
    "redmine": {
      "command": "$INSTALL_DIR/redmine-mcp",
      "env": {
        "REDMINE_URL": "$REDMINE_URL",
        "REDMINE_TOKEN": "$REDMINE_TOKEN"
      }
    }
  }
}
EOF
    else
        cat > "$INSTALL_DIR/$CONFIG_FILE" << EOF
{
  "mcpServers": {
    "redmine": {
      "command": "bun",
      "args": ["run", "$INSTALL_DIR/src/index.ts"],
      "env": {
        "REDMINE_URL": "$REDMINE_URL",
        "REDMINE_TOKEN": "$REDMINE_TOKEN"
      }
    }
  }
}
EOF
    fi
else
    # OpenCode
    if [ "$IS_EXECUTABLE" = true ]; then
        cat > "$INSTALL_DIR/$CONFIG_FILE" << EOF
{
  "\$schema": "https://opencode.ai/config.json",
  "mcp": {
    "redmine": {
      "type": "local",
      "command": ["$INSTALL_DIR/redmine-mcp"],
      "enabled": true,
      "environment": {
        "REDMINE_URL": "$REDMINE_URL",
        "REDMINE_TOKEN": "$REDMINE_TOKEN"
      }
    }
  }
}
EOF
    else
        cat > "$INSTALL_DIR/$CONFIG_FILE" << EOF
{
  "\$schema": "https://opencode.ai/config.json",
  "mcp": {
    "redmine": {
      "type": "local",
      "command": ["bun", "run", "$INSTALL_DIR/src/index.ts"],
      "enabled": true,
      "environment": {
        "REDMINE_URL": "$REDMINE_URL",
        "REDMINE_TOKEN": "$REDMINE_TOKEN"
      }
    }
  }
}
EOF
    fi
fi

echo ""
echo "============================================"
echo "  ✓ 安裝完成！"
echo "============================================"
echo ""
echo "設定檔位置: $INSTALL_DIR/$CONFIG_FILE"
echo ""

if [ "$TOOL_CHOICE" = "1" ]; then
    echo "使用方式："
    echo "  方式 1: 複製設定檔到專案目錄"
    echo "    cp $INSTALL_DIR/$CONFIG_FILE /your/project/.mcp.json"
    echo ""
    echo "  方式 2: 複製設定內容到現有 .mcp.json"
    echo "    cat $INSTALL_DIR/$CONFIG_FILE"
else
    echo "使用方式："
    echo "  方式 1: 複製設定檔到專案目錄"
    echo "    cp $INSTALL_DIR/$CONFIG_FILE /your/project/opencode.json"
    echo ""
    echo "  方式 2: 複製設定內容到現有 opencode.json"
    echo "    cat $INSTALL_DIR/$CONFIG_FILE"
fi

echo ""
echo "可用工具（共 31 個）："
echo ""
echo "  Issues:"
echo "    - redmine_get_issues: 取得 Issues 列表"
echo "    - redmine_get_issue: 取得單一 Issue 詳情"
echo "    - redmine_update_issue: 更新 Issue（新增備註、變更狀態等）"
echo "    - redmine_get_journals: 取得 Issue 歷史記錄"
echo ""
echo "  Time Entries:"
echo "    - redmine_get_time_entries: 取得工時記錄"
echo "    - redmine_create_time_entry: 建立工時記錄"
echo "    - redmine_get_time_entry_activities: 取得活動類型"
echo ""
echo "  Versions:"
echo "    - redmine_get_versions: 取得版本列表"
echo "    - redmine_get_version: 取得版本詳情"
echo ""
echo "  Relations:"
echo "    - redmine_get_issue_relations: 取得關聯"
echo "    - redmine_create_issue_relation: 建立關聯"
echo "    - redmine_delete_issue_relation: 刪除關聯"
echo ""
echo "  Wiki:"
echo "    - redmine_get_wiki_pages: 取得 Wiki 列表"
echo "    - redmine_get_wiki_page: 取得 Wiki 內容"
echo "    - redmine_update_wiki_page: 更新 Wiki"
echo ""
echo "  Others: projects, members, users, trackers, statuses,"
echo "          priorities, categories, queries, roles, groups,"
echo "          files, attachments, news, search, upload, download"
echo ""
