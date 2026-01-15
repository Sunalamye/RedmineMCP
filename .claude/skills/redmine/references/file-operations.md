# File Operations

Redmine 檔案上傳/下載操作說明。

## Upload (redmine_upload)

上傳檔案至 Redmine，取得 attachment token。

### 參數

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| file_path | string | ✓ | 本地檔案完整路徑 |
| description | string | | 檔案說明 |

### 回傳

```json
{
  "upload": {
    "id": 12345,
    "token": "abc123..."
  }
}
```

### 使用流程

1. 上傳檔案取得 token
2. 用 token 附加到 Issue 或其他資源

```typescript
// Step 1: 上傳檔案
const upload = await redmine_upload({
  file_path: "/path/to/document.pdf",
  description: "專案規格書"
});

// Step 2: 附加到 Issue
await redmine_update_issue({
  id: 123,
  uploads: [{ token: upload.upload.token }]
});
```

### 注意事項

- 檔案必須存在於本地
- Token 為一次性使用
- 支援任意檔案類型

## Download (redmine_download)

下載 Redmine 附件至本地。

### 參數

| 參數 | 類型 | 必填 | 說明 |
|------|------|------|------|
| attachment_id | number | ✓ | 附件 ID |
| save_path | string | ✓ | 儲存路徑（含檔名） |

### 回傳

```json
{
  "saved_to": "/path/to/file.pdf",
  "filename": "original_filename.pdf"
}
```

### 取得附件 ID

從 Issue 詳情取得附件列表：

```typescript
const issue = await redmine_get_issue({ id: 123 });
// issue.attachments[0].id → 附件 ID
```

### 使用範例

```typescript
// 下載附件
const result = await redmine_download({
  attachment_id: 456,
  save_path: "/tmp/downloaded_file.pdf"
});

console.log(result.saved_to);  // /tmp/downloaded_file.pdf
console.log(result.filename);  // original_name.pdf
```

### 注意事項

- 目標目錄必須存在
- 會覆蓋同名檔案
- 需要附件讀取權限

## 完整工作流程

### 情境：下載 Issue 所有附件

```typescript
// 1. 取得 Issue 詳情（含附件）
const issue = await redmine_get_issue({ id: 123 });

// 2. 逐一下載
for (const att of issue.attachments || []) {
  await redmine_download({
    attachment_id: att.id,
    save_path: `/tmp/issue_123/${att.filename}`
  });
}
```

### 情境：上傳螢幕截圖到 Issue

```typescript
// 1. 上傳圖片
const upload = await redmine_upload({
  file_path: "/tmp/screenshot.png",
  description: "Bug 重現截圖"
});

// 2. 附加到 Issue 並加入備註
await redmine_update_issue({
  id: 123,
  notes: "附上 Bug 重現截圖",
  uploads: [{ token: upload.upload.token }]
});
```
