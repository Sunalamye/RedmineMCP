#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== 設置 Redmine 測試環境 ==="
echo ""

# 啟動 Docker Compose
echo "[1/4] 啟動 Docker 容器..."
docker compose up -d

echo "[2/4] 等待 Redmine 啟動 (可能需要 1-2 分鐘)..."
max_attempts=60
attempt=0
while ! curl -s http://localhost:3000/ > /dev/null 2>&1; do
    attempt=$((attempt + 1))
    if [ $attempt -ge $max_attempts ]; then
        echo "Error: Redmine 未能在時間內啟動"
        docker compose logs redmine
        exit 1
    fi
    echo -n "."
    sleep 2
done
echo ""
echo "Redmine 已啟動!"

echo "[3/4] 設置管理員 API Token..."
# 使用 Rails console 創建 API token
docker compose exec -T redmine bash -c "
cd /usr/src/redmine
bundle exec rails runner '
user = User.find(1) # admin user
token = user.api_key
if token.nil? || token.empty?
  user.create_api_key
  token = user.api_key
end
puts \"API_TOKEN=#{token}\"
' 2>/dev/null | grep API_TOKEN
" > /tmp/redmine_token.txt 2>/dev/null || true

if [ -f /tmp/redmine_token.txt ] && grep -q "API_TOKEN=" /tmp/redmine_token.txt; then
    source /tmp/redmine_token.txt
    echo "API Token: $API_TOKEN"
    rm /tmp/redmine_token.txt
else
    echo "Warning: 無法自動獲取 API Token"
    echo "請手動登入 http://localhost:3000 (admin/admin)"
    echo "然後到 My account -> API access key 獲取 Token"
    API_TOKEN=""
fi

echo ""
echo "[4/8] 啟用 REST API 和模組..."
docker compose exec -T redmine bash -c "
cd /usr/src/redmine
bundle exec rails runner '
# 啟用 REST API
Setting.rest_api_enabled = \"1\"

# 確保有預設的時間活動類型
if TimeEntryActivity.count == 0
  TimeEntryActivity.create!(name: \"Development\", is_default: true, active: true)
  TimeEntryActivity.create!(name: \"Design\", active: true)
end
puts \"REST API enabled, activities configured\"
' 2>/dev/null
" || echo "Warning: 無法自動啟用 REST API"

echo "[5/8] 創建測試專案..."
if [ -n "$API_TOKEN" ]; then
    # 創建測試專案（啟用所有模組）
    curl -s -X POST "http://localhost:3000/projects.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"project":{"name":"Test Project","identifier":"test-project","is_public":true,"enabled_module_names":["issue_tracking","time_tracking","wiki","files","repository","boards"]}}' > /dev/null 2>&1 || true
    echo "  專案已創建"

    # 創建版本
    echo "[6/8] 創建測試版本..."
    curl -s -X POST "http://localhost:3000/projects/test-project/versions.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"version":{"name":"v1.0.0","status":"open","sharing":"none"}}' > /dev/null 2>&1 || true
    echo "  版本已創建"

    # 創建 Issue 分類
    curl -s -X POST "http://localhost:3000/projects/test-project/issue_categories.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"issue_category":{"name":"Test Category"}}' > /dev/null 2>&1 || true
    echo "  分類已創建"

    echo "[7/8] 創建測試 Issues..."
    # 創建第一個 Issue
    ISSUE1=$(curl -s -X POST "http://localhost:3000/issues.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"issue":{"project_id":"test-project","subject":"Test Issue 1","description":"First test issue for MCP testing","priority_id":2}}' 2>/dev/null | grep -o '"id":[0-9]*' | head -1 | cut -d: -f2)

    # 創建第二個 Issue（用於關聯測試）
    ISSUE2=$(curl -s -X POST "http://localhost:3000/issues.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"issue":{"project_id":"test-project","subject":"Test Issue 2","description":"Second test issue for relation testing","priority_id":2}}' 2>/dev/null | grep -o '"id":[0-9]*' | head -1 | cut -d: -f2)

    echo "  Issue #$ISSUE1 和 #$ISSUE2 已創建"

    # 更新 Issue（生成 Journal）
    if [ -n "$ISSUE1" ]; then
        curl -s -X PUT "http://localhost:3000/issues/$ISSUE1.json" \
            -H "X-Redmine-API-Key: $API_TOKEN" \
            -H "Content-Type: application/json" \
            -d '{"issue":{"notes":"Adding a comment for journal testing"}}' > /dev/null 2>&1 || true
        echo "  Issue #$ISSUE1 已更新（生成 Journal）"
    fi

    # 創建 Issue 關聯
    if [ -n "$ISSUE1" ] && [ -n "$ISSUE2" ]; then
        curl -s -X POST "http://localhost:3000/issues/$ISSUE1/relations.json" \
            -H "X-Redmine-API-Key: $API_TOKEN" \
            -H "Content-Type: application/json" \
            -d "{\"relation\":{\"issue_to_id\":$ISSUE2,\"relation_type\":\"relates\"}}" > /dev/null 2>&1 || true
        echo "  Issue 關聯已創建"
    fi

    echo "[8/8] 創建 Wiki 和工時..."
    # 創建 Wiki 頁面
    curl -s -X PUT "http://localhost:3000/projects/test-project/wiki/TestPage.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{"wiki_page":{"text":"# Test Wiki Page\n\nThis is test content for MCP testing.\n\n## Section 1\n\nSome text here."}}' > /dev/null 2>&1 || true
    echo "  Wiki 頁面已創建"

    # 取得時間活動 ID
    ACTIVITY_ID=$(curl -s "http://localhost:3000/enumerations/time_entry_activities.json" \
        -H "X-Redmine-API-Key: $API_TOKEN" 2>/dev/null | grep -o '"id":[0-9]*' | head -1 | cut -d: -f2)

    # 創建工時記錄
    if [ -n "$ISSUE1" ] && [ -n "$ACTIVITY_ID" ]; then
        curl -s -X POST "http://localhost:3000/time_entries.json" \
            -H "X-Redmine-API-Key: $API_TOKEN" \
            -H "Content-Type: application/json" \
            -d "{\"time_entry\":{\"issue_id\":$ISSUE1,\"hours\":1.5,\"activity_id\":$ACTIVITY_ID,\"comments\":\"Test time entry\"}}" > /dev/null 2>&1 || true
        echo "  工時記錄已創建"
    fi

    # 上傳測試檔案
    echo "Test file content for MCP testing" > /tmp/mcp_test_upload.txt
    UPLOAD_TOKEN=$(curl -s -X POST "http://localhost:3000/uploads.json?filename=test-file.txt" \
        -H "X-Redmine-API-Key: $API_TOKEN" \
        -H "Content-Type: application/octet-stream" \
        --data-binary @/tmp/mcp_test_upload.txt 2>/dev/null | grep -o '"token":"[^"]*"' | cut -d'"' -f4)

    if [ -n "$UPLOAD_TOKEN" ] && [ -n "$ISSUE1" ]; then
        curl -s -X PUT "http://localhost:3000/issues/$ISSUE1.json" \
            -H "X-Redmine-API-Key: $API_TOKEN" \
            -H "Content-Type: application/json" \
            -d "{\"issue\":{\"uploads\":[{\"token\":\"$UPLOAD_TOKEN\",\"filename\":\"test-file.txt\",\"content_type\":\"text/plain\"}]}}" > /dev/null 2>&1 || true
        echo "  測試檔案已上傳"
    fi
    rm -f /tmp/mcp_test_upload.txt

    echo ""
    echo "測試資料已全部創建"
fi

echo ""
echo "=== 設置完成 ==="
echo ""
echo "Redmine URL: http://localhost:3000"
echo "預設帳號: admin / admin"
if [ -n "$API_TOKEN" ]; then
    echo ""
    echo "執行測試:"
    echo "  export REDMINE_URL=http://localhost:3000"
    echo "  export REDMINE_TOKEN=$API_TOKEN"
    echo "  ./test_mcp.sh"
fi
echo ""
echo "停止環境: docker compose down"
echo "查看日誌: docker compose logs -f"
