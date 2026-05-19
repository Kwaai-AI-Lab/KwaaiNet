# KwaaiNet — Claude Code Instructions

## PR Triage (run at the start of every session)

Reza is responsible for merging all community pull requests. At the start of each conversation,
run `gh pr list` and surface any open PRs. If there are open PRs, call them out clearly so the
backlog doesn't grow silently.

```bash
gh pr list
```

Flag any PR that has been open longer than 7 days as overdue.

## TODO

- **Build time ~1.5 hrs**: `[profile.release]` in `core/Cargo.toml` uses `lto = true` + `codegen-units = 1`. Switch to `lto = "thin"` — 5–10× faster link, <5% runtime impact for this workload.
- **Alias embedding gap**: after `graph alias-scan --auto` merges TLSA/NUSAS/NEF, graph retrieval for abbreviation queries regresses (M19: 54.3% vs M17 best 56.9%). Fix: embed aliases in entity text string, e.g. `"{name} ({alias}): description"`.
- **Dream RAG Phase 3**: quality gate — snapshot + rollback if graph health score drops >5% after a cycle.
