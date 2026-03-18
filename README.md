
# SafeDrop Launch Pack

SafeDrop is a one-binary, home-hosted secure file drop server with auditable delivery evidence, privacy-aware sharing, and dynamic reachability that works whenever the host PC is online.

## Project status

| Sprint | Focus | Status |
|--------|-------|--------|
| Sprint 0 | Repository, docs, vocabulary, schemas | **Complete** |
| Sprint 1 | Chunk store, manifests, audit log | Planned — next |
| Sprint 2 | Share links, expiry, resumable transfer, receipts | Planned |
| Sprint 3 | UPnP/NAT-PMP, probes, relay fallback | Planned |
| Sprint 4 | Admin dashboard, evidence cards, export bundles | Planned |
| Sprint 5 | Hybrid PQ, reproducible builds, formal spec | Planned |

> Last updated: 2026-03-18

## Included deliverables

- **Research paper**: `research/SafeDrop_IJRC_Design_Paper.md`
- **50-track research agenda**: `research/SafeDrop_50_Track_Master_Agenda.md`
- **Agent instructions**: `codex/CODEX_MASTER_INSTRUCTIONS.md`
- **Static website**: `Docs/` (ready for GitHub Pages)
- **Issue backlog**: `codex/ISSUE_MAP.csv` (16 issues across 6 sprints)
- **Threat model**: `codex/THREAT_MODEL.md`
- **API specification**: `codex/API_SPEC.md`
- **Data model**: `codex/DATA_MODEL.md`

## Repository layout

```text
SafeDrop_Launch_Pack/
  codex/                 # AI-agent instructions, specs, and prompts
    PROMPTS/             # Per-agent prompt files
    AGENTS.md            # Agent charter and invariants
    API_SPEC.md          # REST API specification
    CODEX_MASTER_INSTRUCTIONS.md
    DATA_MODEL.md        # Manifest, share, and event schemas
    IMPLEMENTATION_SPRINTS.md  # Sprint plan with status tracking
    ISSUE_MAP.csv        # 16-issue backlog with status
    SYSTEM_OVERVIEW.md
    TEST_AND_EVAL_PLAN.md
    THREAT_MODEL.md
  Docs/                  # Static website (GitHub Pages ready)
    assets/              # CSS, JS, data
    downloads/           # Downloadable spec files
    index.html           # Home page
    architecture.html    # System architecture
    roadmap.html         # Implementation roadmap
    research.html        # 50-track browser
    paper.html           # Design paper
    build.html           # Build guide
  research/              # Research agenda and track data
  CHANGELOG.md           # Release history
  START_HERE.md          # Quick-start guide
```

## Quick start

1. Read `START_HERE.md` for orientation
2. Review `codex/CODEX_MASTER_INSTRUCTIONS.md` for architecture rules
3. Check `codex/ISSUE_MAP.csv` for the current backlog
4. Open `Docs/index.html` in a browser for the project website

## License note

This pack is a project planning and research authoring bundle. Add the open-source license of your choice before public release of code or documents.
