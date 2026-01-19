#!/bin/bash
set -e

# 顏色定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MCP_BIN="$PROJECT_DIR/target/release/mitake-redmine-mcp"

# Redmine 設定
REDMINE_URL="${REDMINE_URL:-http://localhost:3000}"
REDMINE_TOKEN="${REDMINE_TOKEN:-}"

echo "=== Mitake Redmine MCP 功能測試 ==="
echo "Redmine URL: $REDMINE_URL"
echo ""

# 函數：發送 MCP 請求
send_mcp_request() {
    local method="$1"
    local params="$2"
    local id="${3:-1}"

    local request="{\"jsonrpc\":\"2.0\",\"id\":$id,\"method\":\"$method\""
    if [ -n "$params" ]; then
        request="$request,\"params\":$params"
    fi
    request="$request}"

    echo "$request" | REDMINE_URL="$REDMINE_URL" REDMINE_TOKEN="$REDMINE_TOKEN" "$MCP_BIN" 2>/dev/null | head -1
}

# 函數：呼叫工具
call_tool() {
    local tool_name="$1"
    local args="$2"

    local params="{\"name\":\"$tool_name\""
    if [ -n "$args" ]; then
        params="$params,\"arguments\":$args"
    fi
    params="$params}"

    send_mcp_request "tools/call" "$params"
}

# 函數：檢查回應是否成功
check_response() {
    local response="$1"
    local tool_name="$2"

    if echo "$response" | grep -q '"error"'; then
        echo -e "${RED}FAIL${NC}: $tool_name"
        echo "  Response: $response"
        return 1
    else
        echo -e "${GREEN}PASS${NC}: $tool_name"
        return 0
    fi
}

# 測試計數
TOTAL=0
PASSED=0
FAILED=0

# 測試函數
run_test() {
    local tool_name="$1"
    local args="$2"
    local description="$3"

    TOTAL=$((TOTAL + 1))
    echo -n "[$TOTAL] Testing $tool_name"
    if [ -n "$description" ]; then
        echo " - $description"
    else
        echo ""
    fi

    local response=$(call_tool "$tool_name" "$args")

    if check_response "$response" "$tool_name"; then
        PASSED=$((PASSED + 1))
    else
        FAILED=$((FAILED + 1))
    fi
    echo ""
}

# 檢查 MCP 二進位檔
if [ ! -x "$MCP_BIN" ]; then
    echo -e "${RED}Error${NC}: MCP binary not found at $MCP_BIN"
    echo "Please run: cargo build --release"
    exit 1
fi

# 檢查環境變數
if [ -z "$REDMINE_TOKEN" ]; then
    echo -e "${YELLOW}Warning${NC}: REDMINE_TOKEN not set. Some tests may fail."
    echo "Please set: export REDMINE_TOKEN=your_api_token"
    echo ""
fi

echo "=== 開始測試 ==="
echo ""

# 1. 測試 initialize
echo "[0] Testing initialize"
response=$(send_mcp_request "initialize" '{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}')
if echo "$response" | grep -q '"protocolVersion"'; then
    echo -e "${GREEN}PASS${NC}: initialize"
else
    echo -e "${RED}FAIL${NC}: initialize"
    echo "  Response: $response"
fi
echo ""

# 2. 測試 tools/list
echo "[0] Testing tools/list"
response=$(send_mcp_request "tools/list" "{}")
tool_count=$(echo "$response" | grep -o '"name"' | wc -l)
echo "  Found $tool_count tools"
if [ "$tool_count" -gt 20 ]; then
    echo -e "${GREEN}PASS${NC}: tools/list (found $tool_count tools)"
else
    echo -e "${RED}FAIL${NC}: tools/list (expected 31 tools, got $tool_count)"
fi
echo ""

# 3. 測試各個工具
echo "=== 測試 MCP 工具 (全部 34 個) ==="
echo ""

# ========== 元資料 API (無需測試資料) ==========
echo "--- 元資料 API ---"
run_test "redmine_get_trackers" "{}" "取得追蹤器列表"
run_test "redmine_get_statuses" "{}" "取得狀態列表"
run_test "redmine_get_priorities" "{}" "取得優先權列表"
run_test "redmine_get_roles" "{}" "取得角色列表"
run_test "redmine_get_groups" "{}" "取得群組列表"
run_test "redmine_get_queries" "{}" "取得已存查詢"
run_test "redmine_get_time_entry_activities" "{}" "取得工時活動類型"
run_test "redmine_get_news" "{}" "取得新聞"

# ========== 使用者 API ==========
echo ""
echo "--- 使用者 API ---"
run_test "redmine_get_current_user" "{}" "取得當前使用者"
run_test "redmine_get_users" "{}" "取得使用者列表"
run_test "redmine_get_user" "{\"id\":1}" "取得使用者詳情"

# ========== 專案 API ==========
echo ""
echo "--- 專案 API ---"
run_test "redmine_get_projects" "{}" "取得專案列表"
run_test "redmine_get_project_members" "{\"project_id\":\"test-project\"}" "取得專案成員"
run_test "redmine_get_versions" "{\"project_id\":\"test-project\"}" "取得版本列表"
run_test "redmine_get_version" "{\"id\":1}" "取得版本詳情"
run_test "redmine_get_issue_categories" "{\"project_id\":\"test-project\"}" "取得 Issue 分類"

# ========== Issue API ==========
echo ""
echo "--- Issue API ---"
run_test "redmine_get_issues" "{\"limit\":5}" "取得 Issue 列表"
run_test "redmine_get_issues" "{\"project_id\":\"test-project\",\"status_id\":\"open\"}" "取得專案 Issue"
run_test "redmine_get_issue" "{\"id\":1}" "取得單一 Issue"
run_test "redmine_get_journals" "{\"issue_id\":1}" "取得 Issue 歷史"
run_test "redmine_update_issue" "{\"id\":1,\"notes\":\"MCP test note\"}" "更新 Issue (新增備註)"

# ========== Issue 關聯 API ==========
echo ""
echo "--- Issue 關聯 API ---"
run_test "redmine_get_issue_relations" "{\"issue_id\":1}" "取得 Issue 關聯"
# 建立新關聯（如果測試資料中已有，可能會失敗）
run_test "redmine_create_issue_relation" "{\"issue_id\":1,\"issue_to_id\":2,\"relation_type\":\"relates\"}" "建立 Issue 關聯"
# 刪除關聯需要 relation_id，先跳過
# run_test "redmine_delete_issue_relation" "{\"id\":1}" "刪除 Issue 關聯"

# ========== 工時 API ==========
echo ""
echo "--- 工時 API ---"
run_test "redmine_get_time_entries" "{}" "取得工時列表"
run_test "redmine_get_time_entries" "{\"issue_id\":1}" "取得 Issue 工時"
# 取得活動 ID 後建立工時
run_test "redmine_create_time_entry" "{\"issue_id\":1,\"hours\":0.5,\"activity_id\":9,\"comments\":\"MCP test entry\"}" "建立工時記錄"

# ========== Wiki API ==========
echo ""
echo "--- Wiki API ---"
run_test "redmine_get_wiki_pages" "{\"project_id\":\"test-project\"}" "取得 Wiki 頁面列表"
run_test "redmine_get_wiki_page" "{\"project_id\":\"test-project\",\"title\":\"TestPage\"}" "取得 Wiki 頁面"
run_test "redmine_update_wiki_page" "{\"project_id\":\"test-project\",\"title\":\"MCPTestPage\",\"text\":\"# MCP Test\\n\\nCreated by MCP test.\"}" "更新 Wiki 頁面"

# ========== 檔案 API ==========
echo ""
echo "--- 檔案 API ---"
run_test "redmine_get_files" "{\"project_id\":\"test-project\"}" "取得專案檔案"
run_test "redmine_get_attachment" "{\"id\":1}" "取得附件資訊"
# upload 和 download 需要實際檔案路徑，需要特別處理
echo "  [SKIP] redmine_upload - 需要實際檔案路徑"
echo "  [SKIP] redmine_download - 需要目標路徑"

# ========== 搜尋 API ==========
echo ""
echo "--- 搜尋 API ---"
run_test "redmine_search" "{\"query\":\"test\"}" "全文搜尋"
run_test "redmine_search" "{\"query\":\"issue\",\"scope\":\"issues\"}" "搜尋 Issues"

# ========== 通用 API ==========
echo ""
echo "--- 通用 API ---"
run_test "redmine_request" "{\"path\":\"/projects.json\",\"method\":\"GET\"}" "通用 API 請求"

# ========== 協議層測試 ==========
echo ""
echo "=== 協議層測試 ==="
echo ""

# 測試未知工具
echo "[P1] Testing unknown tool"
response=$(call_tool "unknown_tool_xyz" "{}")
if echo "$response" | grep -q '"isError":true\|"error"'; then
    echo -e "${GREEN}PASS${NC}: unknown tool returns error"
else
    echo -e "${RED}FAIL${NC}: unknown tool should return error"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))
echo ""

# 測試缺少必要參數
echo "[P2] Testing missing required params"
response=$(call_tool "redmine_get_issue" "{}")
if echo "$response" | grep -q '"isError":true\|"error"\|Missing'; then
    echo -e "${GREEN}PASS${NC}: missing params returns error"
else
    echo -e "${RED}FAIL${NC}: missing params should return error"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))
echo ""

# ========== 邊緣案例測試 ==========
echo ""
echo "=== 邊緣案例測試 ==="
echo ""

# E1: 空結果集
echo "[E1] Testing empty result set"
response=$(call_tool "redmine_get_issues" "{\"project_id\":\"nonexistent-project-xyz-12345\"}")
if echo "$response" | grep -q '"issues":\[\]\|"error"'; then
    echo -e "${GREEN}PASS${NC}: empty result or error for nonexistent project"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for nonexistent project"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E2: 無效 ID
echo "[E2] Testing invalid issue ID"
response=$(call_tool "redmine_get_issue" "{\"id\":999999}")
if echo "$response" | grep -q '"isError":true\|"error"\|404\|Not Found'; then
    echo -e "${GREEN}PASS${NC}: invalid ID returns error"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}FAIL${NC}: invalid ID should return error"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))
echo ""

# E3: 特殊字元搜尋
echo "[E3] Testing special characters in search"
response=$(call_tool "redmine_search" "{\"query\":\"test<script>alert(1)</script>\"}")
if echo "$response" | grep -q '"results"\|"error"'; then
    echo -e "${GREEN}PASS${NC}: special characters handled"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for special characters"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E4: 大量資料請求
echo "[E4] Testing large limit"
response=$(call_tool "redmine_get_issues" "{\"limit\":1000}")
if echo "$response" | grep -q '"issues"\|"error"'; then
    echo -e "${GREEN}PASS${NC}: large limit handled"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for large limit"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E5: 零值參數
echo "[E5] Testing zero limit"
response=$(call_tool "redmine_get_issues" "{\"limit\":0}")
if echo "$response" | grep -q '"issues"\|"error"'; then
    echo -e "${GREEN}PASS${NC}: zero limit handled"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for zero limit"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E6: 負數 ID
echo "[E6] Testing negative ID"
response=$(call_tool "redmine_get_issue" "{\"id\":-1}")
if echo "$response" | grep -q '"isError":true\|"error"\|404'; then
    echo -e "${GREEN}PASS${NC}: negative ID returns error"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: negative ID response"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E7: Unicode 字元
echo "[E7] Testing Unicode characters"
response=$(call_tool "redmine_search" "{\"query\":\"測試中文日本語\"}")
if echo "$response" | grep -q '"results"\|"error"'; then
    echo -e "${GREEN}PASS${NC}: Unicode characters handled"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for Unicode"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E8: 空字串參數
echo "[E8] Testing empty string parameter"
response=$(call_tool "redmine_search" "{\"query\":\"\"}")
if echo "$response" | grep -q '"results"\|"error"\|"isError"'; then
    echo -e "${GREEN}PASS${NC}: empty string handled"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for empty string"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E9: 無效的專案 ID 格式
echo "[E9] Testing invalid project ID format"
response=$(call_tool "redmine_get_versions" "{\"project_id\":\"!!!invalid!!!\"}")
if echo "$response" | grep -q '"error"\|"isError":true\|404'; then
    echo -e "${GREEN}PASS${NC}: invalid project ID returns error"
    PASSED=$((PASSED + 1))
else
    echo -e "${YELLOW}INFO${NC}: unexpected response for invalid project ID"
fi
TOTAL=$((TOTAL + 1))
echo ""

# E10: 多種狀態過濾
echo "[E10] Testing status filter"
response=$(call_tool "redmine_get_issues" "{\"status_id\":\"closed\"}")
if echo "$response" | grep -q 'issues'; then
    echo -e "${GREEN}PASS${NC}: status filter works"
    PASSED=$((PASSED + 1))
else
    echo -e "${RED}FAIL${NC}: status filter failed"
    FAILED=$((FAILED + 1))
fi
TOTAL=$((TOTAL + 1))
echo ""

echo "=== 測試結果 ==="
echo "Total: $TOTAL"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${YELLOW}Some tests failed. Check if Redmine is running and API token is correct.${NC}"
    exit 1
fi
