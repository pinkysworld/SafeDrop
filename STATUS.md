# SafeDrop Project Status

Last updated: 2026-03-18

## Current Sprint: Sprint 2 — Link Lifecycle ✅ Complete

### Overall Progress
- **Sprint 0**: ✅ Complete — Repo structure, docs, vocabulary, /health endpoint, data model structs
- **Sprint 1**: ✅ Complete — Chunk store, manifest builder, audit pipeline
- **Sprint 2**: ✅ Complete — Share lifecycle, resumable download, delivery receipts
- **Sprint 3–5**: ⬚ Not started

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
- ✅ Share issuance with token-based access (`POST /api/v1/shares`)
- ✅ Share metadata retrieval (`GET /api/v1/shares/{id}`)
- ✅ Share listing (`GET /api/v1/shares`)
- ✅ Share revocation/expiry (`POST /api/v1/shares/{id}/expire`)
- ✅ Full file download via capability token (`GET /s/{token}`)
- ✅ Per-segment resumable download (`GET /s/{token}?segment=N`)
- ✅ Resume info for interrupted transfers (`POST /api/v1/shares/{id}/resume`)
- ✅ Segment acknowledgment (`POST /api/v1/shares/{id}/ack`)
- ✅ Delivery receipt generation with BLAKE3 hashing
- ✅ Evidence retrieval (`GET /api/v1/evidence/{share_id}`)
- ✅ Download limit enforcement (share exhaustion)
- ✅ 9 integration tests passing (health + Sprint 2 lifecycle)
- ✅ Website deployed to GitHub Pages

### What's Not Working Yet
- ⬚ UPnP/NAT-PMP reachability (Sprint 3)
- ⬚ External signed reachability probe (Sprint 3)
- ⬚ Encrypted relay fallback (Sprint 3)
- ⬚ Reachability descriptor API (Sprint 3)
- ⬚ Admin UI (Sprint 4)
- ⬚ Evidence export bundles (Sprint 4)
- ⬚ Hybrid PQ crypto (Sprint 5)
- ⬚ Reproducible builds (Sprint 5)
- ⬚ Formal TLA+ spec (Sprint 5)

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
