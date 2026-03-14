# SafeDrop Project Status

Last updated: 2026-03-14

## Current Sprint: Sprint 1 — Storage & Evidence Baseline

### Overall Progress
- **Sprint 0**: ✅ Complete — Repo structure, docs, vocabulary, /health endpoint, data model structs
- **Sprint 1**: 🔨 In Progress — Chunk store, manifest builder, audit pipeline
- **Sprint 2–5**: ⬚ Not started

### What's Working Right Now
- ✅ Repository structure with all specification documents
- ✅ Rust project compiles and runs
- ✅ `GET /health` returns system status JSON
- ✅ Data model structs for Manifest, Share, AuditEvent, Receipt, Chunk
- ✅ Configuration system with CLI args and environment variables
- ✅ Typed error handling across all modules
- ✅ Content-addressed chunk store with BLAKE3 hashing
- ✅ Deterministic manifest generation from file ingestion
- ✅ Append-only audit event log with checkpoint hashing
- ✅ File import API endpoint (`POST /api/v1/files/import`)
- ✅ Audit events API endpoint (`GET /api/v1/audit/events`)
- ✅ Website deployed to GitHub Pages

### What's Not Working Yet
- ⬚ Share issuance and expiry (Sprint 2)
- ⬚ Resumable segmented transfer (Sprint 2)
- ⬚ Delivery receipts (Sprint 2)
- ⬚ UPnP/NAT-PMP reachability (Sprint 3)
- ⬚ Relay fallback (Sprint 3)
- ⬚ Admin UI (Sprint 4)
- ⬚ Evidence export bundles (Sprint 4)

### Known Limitations
- Storage is filesystem-only; no database backend yet
- No authentication on admin endpoints (development mode)
- No TLS built-in; expects reverse proxy for production

### Architecture Decisions
| Decision | Choice | Rationale |
|----------|--------|-----------|
| Language | Rust | Memory safety, single binary, async performance |
| Web framework | Axum | Tokio-native, tower middleware, type-safe routing |
| Hash algorithm | BLAKE3 | Fast, parallelizable, streaming, no length extension |
| Serialization | JSON + serde | Readable evidence, standard tooling |
| Chunk size | 1 MiB fixed | Simple, good for resume, parallelizable |
| Audit format | Append-only JSONL | Human-readable, streamable, easy to verify |

### Honest Assessment
This is a pre-alpha project. The core storage and evidence models are implemented but have not been tested under adversarial conditions. The threat model is documented but enforcement is minimal at this stage. No external security audit has been performed.
