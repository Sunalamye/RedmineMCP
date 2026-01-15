# Redmine Query Operators

## Operator Codes

| Code | Meaning | API Mapping |
|------|---------|-------------|
| `o` | Open | `status_id: "open"` |
| `c` | Closed | `status_id: "closed"` |
| `*` | Any/All | `status_id: "*"` |
| `=` | Equals | Direct value |
| `!` | Not equals | Prefix `!` to value |
| `!*` | None/Empty | Empty string |

## Filter Fields

| Field | Type | Example Values |
|-------|------|----------------|
| `status_id` | Status | `o`, `c`, `*`, `7` |
| `tracker_id` | Number | `20` (Bug-iOS) |
| `assigned_to_id` | User | `me`, `42`, `!42` |
| `author_id` | User | `me`, `42` |
| `fixed_version_id` | Number | Version ID |
| `priority_id` | Number | Priority ID |

## Query String Pattern

```
f[]={field}              # Active filter
op[{field}]={operator}   # Operator
v[{field}][]={value}     # Value(s)
```

## URL Encoding

| Encoded | Decoded |
|---------|---------|
| `%3D` | `=` |
| `%21` | `!` |
| `%5B` | `[` |
| `%5D` | `]` |
