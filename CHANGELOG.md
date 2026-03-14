# Changelog

All notable changes to SafeDrop will be documented in this file.

## [Unreleased]

### Added
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
