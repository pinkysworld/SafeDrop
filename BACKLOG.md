# SafeDrop Backlog

Last updated: 2026-03-18

## Sprint 0 — Repository & Vocabulary Freeze

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-001 | Bootstrap repo and documentation | bootstrap | ✅ Done | README, AGENTS, site scaffold, no code secrets |
| SD-002 | Define manifest and share schemas | storage | ✅ Done | Schema docs and serde-ready structs |

## Sprint 1 — Storage & Evidence Baseline

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-003 | Implement chunk ingestion path | storage | ✅ Done | Chunk verification and deterministic manifest generation |
| SD-004 | Build audit event append pipeline | evidence | ✅ Done | Signed checkpoint export |

## Sprint 2 — Link Lifecycle

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-005 | Implement share issuance and expiry | evidence | ✅ Done | Expired shares rejected and logged |
| SD-006 | Add resumable segmented transfer | storage | ✅ Done | Interrupted transfers restart safely |
| SD-007 | Add final delivery receipts | evidence | ✅ Done | Human-readable completion evidence |

## Sprint 3 — Reachability

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-008 | Implement UPnP/NAT-PMP attempts | network | ⬚ Planned | Reachability attempt logged |
| SD-009 | Build external signed reachability probe | network | ⬚ Planned | Descriptor proves selected path |
| SD-010 | Encrypted relay fallback skeleton | network | ⬚ Planned | Relay has no plaintext access |
| SD-011 | Publish reachability descriptor API | network | ⬚ Planned | Recipient sees path type |

## Sprint 4 — Explainability & Polish

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-012 | Admin console evidence cards | admin_ui | ⬚ Planned | Plain-language evidence summaries |
| SD-013 | Export bundle generation | evidence | ⬚ Planned | Bundle validates against schema |

## Sprint 5 — Research Acceleration

| ID | Title | Agent | Status | Acceptance |
|----|-------|-------|--------|------------|
| SD-014 | Hybrid PQ crypto interface | release | ⬚ Planned | Policy versioning and fallback |
| SD-015 | Reproducible build manifest | release | ⬚ Planned | Published hashes and verification |
| SD-016 | Minimal formal spec | formal_methods | ⬚ Planned | TLA+ lifecycle model |
