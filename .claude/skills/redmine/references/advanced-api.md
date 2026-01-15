# Advanced API (redmine_request)

Generic Redmine API call tool for unimplemented API endpoints.

## When to Use

- Need to call Redmine API not yet wrapped
- Need custom API parameter combinations
- Explore new API features

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| path | string | ✓ | API path, e.g. `/issues.json` |
| method | string | | HTTP method: get, post, put, delete (default: get) |
| data | object | | Request body (for POST/PUT) |
| params | object | | Query parameters |

## Response

```json
{
  "status_code": 200,
  "body": { ... },
  "error": ""
}
```

## Usage Examples

### GET Request

```typescript
// Get project members (with pagination)
const result = await redmine_request({
  path: "/projects/myproject/memberships.json",
  params: { limit: "100", offset: "0" }
});
```

### POST Request

```typescript
// Create Issue
const result = await redmine_request({
  path: "/issues.json",
  method: "post",
  data: {
    issue: {
      project_id: 1,
      tracker_id: 1,
      subject: "New feature request",
      description: "Detailed description..."
    }
  }
});
```

### PUT Request

```typescript
// Update version
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

### DELETE Request

```typescript
// Delete attachment
const result = await redmine_request({
  path: "/attachments/123.json",
  method: "delete"
});
```

## Common Unwrapped APIs

### Issue Watchers

```typescript
// Add watcher
await redmine_request({
  path: "/issues/123/watchers.json",
  method: "post",
  data: { user_id: 5 }
});

// Remove watcher
await redmine_request({
  path: "/issues/123/watchers/5.json",
  method: "delete"
});
```

### Custom Fields

```typescript
// Update custom fields
await redmine_request({
  path: "/issues/123.json",
  method: "put",
  data: {
    issue: {
      custom_fields: [
        { id: 1, value: "High Priority" },
        { id: 2, value: ["Option A", "Option B"] }
      ]
    }
  }
});
```

### Project Modules

```typescript
// Get project details (with modules)
await redmine_request({
  path: "/projects/myproject.json",
  params: { include: "trackers,issue_categories,enabled_modules" }
});
```

### Memberships

```typescript
// Add project member
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

## Error Handling

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

## Notes

1. **Path format**: Must include `.json` suffix
2. **Permissions**: Follows Redmine permission settings
3. **HTTP method**: Case insensitive
4. **Response parsing**: Automatically parses JSON response

## Redmine REST API Documentation

Full API reference: https://www.redmine.org/projects/redmine/wiki/Rest_api
