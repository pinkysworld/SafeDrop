# SafeDrop

**Home-hosted secure file sharing with proof-carrying delivery and verifiable reachability.**

SafeDrop is a single-executable file-sharing server that runs on a home PC whenever it is online. It combines content-addressed storage, signed delivery receipts, path-aware reachability, and an explanation-first admin console.

## Key Features

- **Content-addressed storage** — Files are split into BLAKE3-hashed chunks with deterministic manifests
- **Proof-carrying delivery** — Append-only audit events, signed receipts, and exportable evidence bundles
- **Home reachability** — Direct bind → UPnP/NAT-PMP → signed probe → encrypted relay fallback
- **Explanation-first UX** — Every cryptographic action is surfaced with human-readable explanations
- **Privacy controls** — Metadata aliases, size bucketing, consent tracking, and policy-governed exports
- **Single binary** — One executable, zero cloud accounts required

## Project Status

| Sprint | Focus | Status |
|--------|-------|--------|
| Sprint 0 | Repository, docs, vocabulary | ✅ Complete |
| Sprint 1 | Storage & evidence baseline | 🔨 In Progress |
| Sprint 2 | Share lifecycle | ⬚ Planned |
| Sprint 3 | Reachability | ⬚ Planned |
| Sprint 4 | Admin console & polish | ⬚ Planned |
| Sprint 5 | Research acceleration | ⬚ Planned |

See [STATUS.md](STATUS.md) for detailed progress and [BACKLOG.md](BACKLOG.md) for the full issue backlog.

## Quick Start

```bash
# Clone
git clone https://github.com/pinkysworld/SafeDrop.git
cd SafeDrop

# Build
cargo build --release

# Run
./target/release/safedrop

# Or with cargo
cargo run -- --port 8080
```

The server starts at `http://localhost:8080`. Visit `/health` to verify it's running.

## Architecture

SafeDrop is organized into explicit layers:

1. **Content-addressed storage** — Chunking, deduplication, deterministic manifests
2. **Evidence & audit** — Append-only log, signed receipts, delivery confirmations
3. **Share lifecycle** — Link creation, capability tokens, expiry, resumable transfer
4. **Reachability** — UPnP, NAT-PMP, signed probes, encrypted relay
5. **Privacy & policy** — Metadata controls, consent tracking, export bundles
6. **Explanation surface** — Admin dashboard, evidence cards, status explanations

See [codex/SYSTEM_OVERVIEW.md](codex/SYSTEM_OVERVIEW.md) for the full system model.

## Repository Structure

```
├── src/                  # Rust source code
│   ├── main.rs           # Entry point and server setup
│   ├── lib.rs            # Library root
│   ├── config.rs         # Configuration
│   ├── error.rs          # Error types
│   ├── storage/          # Content-addressed chunk store
│   ├── evidence/         # Audit log and receipts
│   ├── share/            # Share lifecycle
│   ├── network/          # Reachability engine
│   └── api/              # HTTP API handlers
├── tests/                # Integration tests
├── codex/                # AI agent instructions
├── paper/                # Research paper
├── research/             # 50-track research agenda
└── docs/                 # Project website
```

## Building for AI Agents

This project is designed to be built by AI coding agents. See:

- [codex/AGENTS.md](codex/AGENTS.md) — Agent charter and invariants
- [codex/CODEX_MASTER_INSTRUCTIONS.md](codex/CODEX_MASTER_INSTRUCTIONS.md) — Master build instructions
- [codex/PROMPTS/](codex/PROMPTS/) — Task-specific agent prompts
- [BACKLOG.md](BACKLOG.md) — Issue backlog with acceptance criteria

## Research

SafeDrop includes a 50-track research agenda and an IJRC-style design paper:

- [Research Agenda](research/SafeDrop_50_Track_Master_Agenda.md)
- [Design Paper](paper/SafeDrop_IJRC_Design_Paper.md)
- [Website](https://pinkysworld.github.io/SafeDrop/)

## License

MIT — see [LICENSE](LICENSE).
