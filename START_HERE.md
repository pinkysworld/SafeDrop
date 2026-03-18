
# SafeDrop Launch Pack - Start Here

This package gives you four things in one place:

1. a submission-ready **first research paper** written for the current IJRC / IJRCom style expectations,
2. an expanded **50-track research agenda**,
3. a full **AI-agent / Codex instruction pack** so coding can start immediately,
4. a complete **static website** that you can upload to GitHub Pages.

## Recommended order

1. Read `paper/SafeDrop_IJRC_Design_Paper.pdf`.
2. Read `research/SafeDrop_50_Track_Master_Agenda.md`.
3. Read `codex/CODEX_MASTER_INSTRUCTIONS.md` and `codex/AGENTS.md`.
4. Open `website/index.html` locally, then upload the `website/` folder to a GitHub repository.

## Immediate next actions

- Create a public GitHub repository named `SafeDrop`.
- Copy the `website/` folder into the repository root or `docs/` folder for GitHub Pages.
- Use the files in `codex/PROMPTS/` as task prompts for coding agents.
- Implement the first thin vertical slice in this order:
  - audit model and manifests,
  - resumable transfer path,
  - expiring capability links,
  - UPnP reachability plus external probe,
  - encrypted relay fallback,
  - explanation cards.

## Package structure

- `paper/` - full research paper in DOCX, PDF, and Markdown source.
- `research/` - 50-track agenda in Markdown, CSV, and JSON.
- `codex/` - agent charter, architecture, threat model, API, sprint plan, prompts.
- `website/` - static site ready for GitHub Pages deployment.

## Important note

The paper deliberately makes no fabricated empirical claims. It is positioned as a rigorous system-design paper with explicit evaluation criteria, implementation sequencing, and an honest research roadmap.
