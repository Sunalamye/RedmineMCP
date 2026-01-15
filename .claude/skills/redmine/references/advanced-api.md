# Advanced API (redmine_request)

通用 Redmine API 呼叫工具，用於未實作的 API 端點。

## 使用時機

- 需要呼叫尚未封裝的 Redmine API
- 需要自訂 API 參數組合
- 探索新 API 功能

## 參數

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| path | string | ✓ | API 路徑，如 `/issues.json` |
| method | string | | HTTP 方法：get, post, put, delete（預設 get） |
| data | object | | 請求內容（用於 POST/PUT） |
| params | object | | 查詢參數 |

## 回傳

```json
{
  "status_code": 200,
  "body": { ... },
  "error": ""
}
```

## 使用範例

### GET 請求

```typescript
// 取得專案成員（含分頁）
const result = await redmine_request({
  path: "/projects/myproject/memberships.json",
  params: { limit: "100", offset: "0" }
});
```

### POST 請求

```typescript
// 建立 Issue
const result = await redmine_request({
  path: "/issues.json",
  method: "post",
  data: {
    issue: {
      project_id: 1,
      tracker_id: 1,
      subject: "新功能請求",
      description: "詳細說明..."
    }
  }
});
```

### PUT 請求

```typescript
// 更新版本
const result = await redmine_request({
  path: "/versions/5.json",
  method: "put",
  data: {
    version: {
      status: "closed",
      due_date: "2024-12-31"
    }
  }
});
```

### DELETE 請求

```typescript
// 刪除附件
const result = await redmine_request({
  path: "/attachments/123.json",
  method: "delete"
});
```

## 常用未封裝 API

### Issue Watchers

```typescript
// 新增 watcher
await redmine_request({
  path: "/issues/123/watchers.json",
  method: "post",
  data: { user_id: 5 }
});

// 移除 watcher
await redmine_request({
  path: "/issues/123/watchers/5.json",
  method: "delete"
});
```

### Custom Fields

```typescript
// 更新自訂欄位
await redmine_request({
  path: "/issues/123.json",
  method: "put",
  data: {
    issue: {
      custom_fields: [
        { id: 1, value: "高優先" },
        { id: 2, value: ["選項A", "選項B"] }
      ]
    }
  }
});
```

### Project Modules

```typescript
// 取得專案詳情（含 modules）
await redmine_request({
  path: "/projects/myproject.json",
  params: { include: "trackers,issue_categories,enabled_modules" }
});
```

### Memberships

```typescript
// 新增專案成員
await redmine_request({
  path: "/projects/myproject/memberships.json",
  method: "post",
  data: {
    membership: {
      user_id: 5,
      role_ids: [3, 4]
    }
  }
});
```

## 錯誤處理

```typescript
const result = await redmine_request({ path: "/issues/99999.json" });

if (result.status_code === 404) {
  console.log("Issue not found");
} else if (result.error) {
  console.log("Error:", result.error);
} else {
  console.log("Success:", result.body);
}
```

## 注意事項

1. **路徑格式**：必須包含 `.json` 後綴
2. **權限**：遵循 Redmine 權限設定
3. **HTTP 方法**：大小寫不敏感
4. **回傳解析**：自動解析 JSON 回應

## Redmine REST API 文檔

完整 API 參考：https://www.redmine.org/projects/redmine/wiki/Rest_api
