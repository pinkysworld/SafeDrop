# Changelog

All notable changes to SafeDrop will be documented in this file.

## [Unreleased]

### Added — Sprint 2: Link Lifecycle
- Share issuance with token-based capability URLs (`POST /api/v1/shares`)
- Share metadata retrieval (`GET /api/v1/shares/{id}`)
- Share listing (`GET /api/v1/shares`)
- Share revocation/expiry (`POST /api/v1/shares/{id}/expire`)
- Full file download via capability token (`GET /s/{token}`)
- Per-segment resumable download (`GET /s/{token}?segment=N`)
- Resume info endpoint for interrupted transfers (`POST /api/v1/shares/{id}/resume`)
- Segment acknowledgment with hash verification (`POST /api/v1/shares/{id}/ack`)
- Delivery receipt generation with BLAKE3 integrity hash
- Evidence retrieval endpoint (`GET /api/v1/evidence/{share_id}`)
- Download limit enforcement with automatic share exhaustion
- Filesystem-backed ShareStore with per-share JSON persistence
- 8 new integration tests covering full share lifecycle

### Added — Sprint 0 + Sprint 1
- Project repository structure and documentation
- Full specification: AGENTS.md, CODEX_MASTER_INSTRUCTIONS.md, API_SPEC.md, DATA_MODEL.md, THREAT_MODEL.md
- Rust project with Axum web framework
- `GET /health` endpoint returning system status
- Content-addressed chunk store with BLAKE3 hashing
- Deterministic manifest generation from file ingestion
- Append-only audit event log with BLAKE3 checkpoint chaining
- `POST /api/v1/files/import` for file ingestion
- `GET /api/v1/audit/events` for audit log inspection
- Typed error handling with structured error responses
- Configuration via CLI arguments and environment variables
- 50-track research agenda and IJRC design paper
- Project website with architecture, roadmap, research, and build guide pages
- GitHub issue templates for agent tasks
- Backlog (BACKLOG.md) and status tracking (STATUS.md)

### Infrastructure
- Cargo.toml with pinned dependency versions
- .gitignore for Rust, IDE, and OS artifacts
- MIT license
