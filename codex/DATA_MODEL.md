
# Data Model

## Suggested manifest schema (versioned)

```json
{
  "schema_version": "1",
  "manifest_id": "sha256-or-blake3-root",
  "created_at": "RFC3339 timestamp",
  "content_type": "application/pdf",
  "size_bytes": 12345,
  "chunking": {
    "algorithm": "fixed-1MiB",
    "chunk_count": 4
  },
  "chunks": [
    {"index": 0, "hash": "...", "size": 1048576}
  ],
  "file_name_alias": "contract-Q1.pdf",
  "hash_algorithm": "blake3"
}
```

## Suggested share schema

```json
{
  "schema_version": "1",
  "share_id": "uuid-or-hash",
  "manifest_id": "...",
  "created_at": "RFC3339 timestamp",
  "expires_at": "RFC3339 timestamp",
  "recipient_scope": "anonymous | one_time_secret | named",
  "path_preference": "direct_first",
  "relay_allowed": true,
  "download_limit": 1
}
```

## Evidence events

- `manifest_committed`
- `share_issued`
- `reachability_confirmed`
- `download_started`
- `segment_acknowledged`
- `download_completed`
- `share_expired`
- `object_deleted`
