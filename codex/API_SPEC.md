
# API Spec (Initial Draft)

## Public recipient endpoints

- `GET /health`
- `GET /s/{share_token}` - recipient landing page
- `GET /api/v1/shares/{share_id}` - metadata permitted to the recipient
- `GET /api/v1/shares/{share_id}/download` - ranged or segmented download
- `POST /api/v1/shares/{share_id}/resume` - request resumable segments
- `POST /api/v1/shares/{share_id}/ack` - segment or final receipt acknowledgment

## Owner/admin endpoints

- `POST /api/v1/files/import`
- `POST /api/v1/shares`
- `POST /api/v1/shares/{share_id}/expire`
- `GET /api/v1/audit/events`
- `GET /api/v1/evidence/{share_id}`
- `POST /api/v1/reachability/check`
- `GET /api/v1/research/tracks`

## Response principles

- Every response returns a version field.
- Every failure response includes a stable machine code.
- Security-sensitive endpoints must never reveal whether a hidden share exists unless the caller proves authorization.
- Range and resume endpoints must be idempotent where practical.
