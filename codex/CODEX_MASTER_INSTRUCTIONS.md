
# SafeDrop Codex Master Instructions

This file is the primary prompt pack for AI coding agents. The mission is to let multiple agents contribute without fragmenting the architecture or accidentally pushing the project into research-only territory before the product foundations exist.

## 1. Mission

Build SafeDrop as a single executable that lets a user:

1. start the server on a home PC,
2. upload or select a file,
3. generate a link,
4. make that link reachable while the host is online,
5. deliver the file with cryptographically anchored evidence,
6. inspect the resulting evidence in a plain-language admin console.

## 2. Architectural contract

Required layers:

- protocol facade (web UI + API)
- content-addressed storage
- evidence and audit layer
- dynamic access engine
- privacy and policy layer
- admin and explanation console

Non-negotiable system qualities:

- one binary for default deployment
- local-first operation
- append-only evidence
- feature-flagged frontier research
- explicit versioning of proof and export formats
- strong defaults with reversible operator control

## 3. Sequencing rules

### Product before frontier crypto

Implement in this order:

1. hash-addressed chunks and manifests
2. upload/download/link lifecycle
3. audit events and signed receipts
4. resumable segmented transfer
5. reachability probes and path descriptors
6. encrypted relay fallback
7. explanation cards and export bundles
8. hybrid PQ and reproducible builds
9. optional succinct proofs and frontier privacy modes

### Networking path order

1. local/private access for development
2. public bind if already reachable
3. UPnP / NAT-PMP mapping
4. signed external reachability check
5. ICE / STUN assisted direct path
6. relay fallback
7. route diversity and relay honesty research

### Evidence path order

1. audit log
2. signed receipts
3. export bundle
4. receipt aggregation
5. deletion evidence
6. succinct proof wrappers

## 4. Rust coding rules

- Prefer stable Rust and version-pinned toolchains.
- Use `Result` and typed errors.
- Keep cryptographic primitives behind interfaces so policy and migration can evolve.
- All external formats must be versioned from day one.
- All time comparisons must be explicit about monotonic versus wall-clock behavior.
- Use property tests for manifest and chunk invariants.
- Add fuzz targets to parsers and export bundle loaders.
- Add integration tests for interrupted transfers and resume.

## 5. Review checklist for generated code

- Does the feature strengthen the MVP path or distract from it?
- Can the operator explain the resulting evidence to a non-expert?
- Can the system still function without the experimental feature flag?
- Is there a plausible downgrade or rollback strategy?
- Did the change increase metadata leakage or operational complexity?

## 6. Required issue template fields

Every issue written for an AI agent must include:

- problem statement
- invariants touched
- files or modules expected
- tests to add
- evidence or logging changes
- feature flag requirements
- acceptance criteria
- explicit out-of-scope list
