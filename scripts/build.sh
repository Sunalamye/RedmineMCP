#!/bin/bash
# Redmine MCP 打包腳本
# Usage: ./scripts/build.sh [src|macos|all]

set -e

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 取得腳本所在目錄的上層（專案根目錄）
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

echo "============================================"
echo "  Redmine MCP 打包工具"
echo "============================================"
echo ""
echo "專案目錄: $PROJECT_DIR"
echo ""

# 建立 dist 目錄
mkdir -p dist

# 驗證 skill 檔案（檢查已知的 bug pattern）
validate_skill() {
    echo -e "${YELLOW}[驗證]${NC} 檢查 SKILL.md..."

    if grep -q '`!` + `' .claude/skills/redmine/SKILL.md 2>/dev/null; then
        echo -e "${RED}[錯誤]${NC} SKILL.md 包含有問題的 pattern: \`!\` + \`"
        echo "請修正後再執行打包"
        exit 1
    fi

    if grep -q '`!` + `' CLAUDE.md 2>/dev/null; then
        echo -e "${RED}[錯誤]${NC} CLAUDE.md 包含有問題的 pattern: \`!\` + \`"
        echo "請修正後再執行打包"
        exit 1
    fi

    echo -e "${GREEN}[通過]${NC} Skill 檔案驗證通過"
}

# 打包原始碼版
build_src() {
    echo ""
    echo -e "${YELLOW}[打包]${NC} 原始碼版 (src.zip)..."

    rm -f dist/redmine-mcp-src.zip

    zip -9 -r dist/redmine-mcp-src.zip \
        CLAUDE.md \
        README.md \
        package.json \
        tsconfig.json \
        install.sh \
        src/ \
        docs/ \
        .claude/ \
        -x "*.DS_Store" -x "*__MACOSX*"

    local size=$(ls -lh dist/redmine-mcp-src.zip | awk '{print $5}')
    echo -e "${GREEN}[完成]${NC} dist/redmine-mcp-src.zip ($size)"
}

# 編譯執行檔
build_binary() {
    echo ""
    echo -e "${YELLOW}[編譯]${NC} 執行檔..."

    if ! command -v bun &> /dev/null; then
        echo -e "${RED}[錯誤]${NC} 找不到 bun，請先安裝 bun"
        exit 1
    fi

    bun build src/index.ts --compile --minify --outfile dist/redmine-mcp

    local size=$(ls -lh dist/redmine-mcp | awk '{print $5}')
    echo -e "${GREEN}[完成]${NC} dist/redmine-mcp ($size)"
}

# 打包 macOS 版
build_macos() {
    echo ""
    echo -e "${YELLOW}[打包]${NC} macOS 執行檔版 (macos.zip)..."

    # 檢查執行檔是否存在
    if [ ! -f "dist/redmine-mcp" ]; then
        echo -e "${YELLOW}[注意]${NC} 執行檔不存在，先編譯..."
        build_binary
    fi

    rm -f dist/redmine-mcp-macos.zip

    # 建立暫存目錄結構
    local tmp_dir=$(mktemp -d)

    cp dist/redmine-mcp "$tmp_dir/"
    cp install.sh "$tmp_dir/"
    cp -r docs "$tmp_dir/" 2>/dev/null || true
    cp -r .claude "$tmp_dir/"

    # 移除 .DS_Store
    find "$tmp_dir" -name ".DS_Store" -delete

    # 打包
    (cd "$tmp_dir" && zip -9 -r "$PROJECT_DIR/dist/redmine-mcp-macos.zip" .)

    # 清理
    rm -rf "$tmp_dir"

    local size=$(ls -lh dist/redmine-mcp-macos.zip | awk '{print $5}')
    echo -e "${GREEN}[完成]${NC} dist/redmine-mcp-macos.zip ($size)"
}

# 顯示打包結果
show_summary() {
    echo ""
    echo "============================================"
    echo "  打包結果"
    echo "============================================"
    echo ""
    ls -lh dist/*.zip dist/redmine-mcp 2>/dev/null | awk '{print $9, $5}'
    echo ""

    # 顯示 SKILL 版本
    local skill_version=$(grep "^version:" .claude/skills/redmine/SKILL.md | awk '{print $2}')
    echo "SKILL 版本: v$skill_version"
    echo ""
}

# 主程式
case "${1:-all}" in
    src)
        validate_skill
        build_src
        show_summary
        ;;
    macos)
        validate_skill
        build_macos
        show_summary
        ;;
    binary)
        build_binary
        show_summary
        ;;
    all)
        validate_skill
        build_src
        build_binary
        build_macos
        show_summary
        ;;
    *)
        echo "Usage: $0 [src|macos|binary|all]"
        echo ""
        echo "  src    - 只打包原始碼版 (src.zip)"
        echo "  macos  - 只打包 macOS 執行檔版 (macos.zip)"
        echo "  binary - 只編譯執行檔"
        echo "  all    - 全部打包 (預設)"
        exit 1
        ;;
esac

echo -e "${GREEN}完成！${NC}"
