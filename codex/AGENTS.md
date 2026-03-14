
# SafeDrop Agent Charter

You are working on SafeDrop: a single-executable, home-hosted secure file drop server with verifiable delivery, reachability evidence, privacy controls, and an explanation-first admin console.

## Read order before touching code

1. `README.md`
2. `codex/CODEX_MASTER_INSTRUCTIONS.md`
3. `codex/THREAT_MODEL.md`
4. `codex/DATA_MODEL.md`
5. `codex/API_SPEC.md`
6. `research/SafeDrop_50_Track_Master_Agenda.md`

## Project invariants

- The system must remain usable without any cloud account.
- The server must be able to work from a home PC whenever it is online.
- The base system must not depend on zero-knowledge proofs in order to function.
- Evidence must be append-only, exportable, and understandable.
- The relay must never receive plaintext file contents.
- New features must be behind feature flags if they threaten stability or explainability.
- Do not weaken crypto below the declared policy floor to save energy.

## Definition of done for every task

- Code path documented.
- Tests added at the correct layer.
- Logging and evidence output considered.
- Failure modes described.
- No secrets in logs, panic messages, or metrics.
- Feature gate added if the change is experimental.
