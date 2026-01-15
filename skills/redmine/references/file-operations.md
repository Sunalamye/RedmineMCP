# File Operations

Redmine file upload/download operations.

## Upload (redmine_upload)

Upload file to Redmine and get attachment token.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| file_path | string | ✓ | Local file full path |
| description | string | | File description |

### Response

```json
{
  "upload": {
    "id": 12345,
    "token": "abc123..."
  }
}
```

### Workflow

1. Upload file to get token
2. Use token to attach to Issue or other resource

```typescript
// Step 1: Upload file
const upload = await redmine_upload({
  file_path: "/path/to/document.pdf",
  description: "Project specification"
});

// Step 2: Attach to Issue
await redmine_update_issue({
  id: 123,
  uploads: [{ token: upload.upload.token }]
});
```

### Notes

- File must exist locally
- Token is single-use
- Supports any file type

## Download (redmine_download)

Download Redmine attachment to local.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| attachment_id | number | ✓ | Attachment ID |
| save_path | string | ✓ | Save path (including filename) |

### Response

```json
{
  "saved_to": "/path/to/file.pdf",
  "filename": "original_filename.pdf"
}
```

### Getting Attachment ID

Get attachment list from Issue details:

```typescript
const issue = await redmine_get_issue({ id: 123 });
// issue.attachments[0].id → Attachment ID
```

### Usage Example

```typescript
// Download attachment
const result = await redmine_download({
  attachment_id: 456,
  save_path: "/tmp/downloaded_file.pdf"
});

console.log(result.saved_to);  // /tmp/downloaded_file.pdf
console.log(result.filename);  // original_name.pdf
```

### Notes

- Target directory must exist
- Will overwrite existing file
- Requires attachment read permission

## Complete Workflows

### Scenario: Download all Issue attachments

```typescript
// 1. Get Issue details (with attachments)
const issue = await redmine_get_issue({ id: 123 });

// 2. Download each attachment
for (const att of issue.attachments || []) {
  await redmine_download({
    attachment_id: att.id,
    save_path: `/tmp/issue_123/${att.filename}`
  });
}
```

### Scenario: Upload screenshot to Issue

```typescript
// 1. Upload image
const upload = await redmine_upload({
  file_path: "/tmp/screenshot.png",
  description: "Bug reproduction screenshot"
});

// 2. Attach to Issue with note
await redmine_update_issue({
  id: 123,
  notes: "Attached bug reproduction screenshot",
  uploads: [{ token: upload.upload.token }]
});
```
