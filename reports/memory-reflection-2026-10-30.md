# Memory Reflection — 2026-10-30

Scope: default branch at `ee626f3` (origin/main). Commits reviewed: `git log --since="36 hours ago"`
(6 commits, 2026-10-28 22:30 PT through 2026-10-29 20:02 PT): #254, #259, #260, #261, #262, #263.

## Mechanical — checker output, verbatim

```
$ python3 scripts/check_memory_integrity.py
Checking 56 memory and plan files
```

0 broken reference(s), 134 warning(s) across 56 files — all 134 warnings are in
`projects/*/plans/*.md` (proposal/experiment-log documents describing not-yet-built files, or
functions that have since moved within a file). None of the 13 authoritative files (12 CLAUDE.md
+ `projects/kwaai-knowledge/GLOSSARY.md`) have any warning or broken reference. Full warning list
omitted here for brevity (available by re-running the command); representative examples:

- `projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md` — `core/crates/kwaai-rag/src/schema.rs` moved to `doc_schema.rs`; several proposed YAML/script files never materialized.
- `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md` — "status says 'not yet committed' but core/crates/kwaai-rag/src has changed since — recheck whether it has landed" (see Accretion).
- Several `plans/*.md` files cite line numbers for functions that have since shifted within the same file (e.g. `graph.rs:1912` vs current `2122`).

Exit code 0 (checker treats warnings as non-fatal drift, not failures).

## Semantic — claims now false

Nothing found. Checked all 13 authoritative memory files (12 CLAUDE.md + GLOSSARY.md) against
the 6 commits from the last 36 hours:

- **#254** `feat(grpc): require a per-start bearer token on every gRPC call` — no memory file
  documents the gRPC control API's auth model (searched for `grpc`, `bearer`, `token`,
  `unauthenticat` across all 13 files: no hits), so nothing to invalidate.
- **#259** `fix(kwaai-rag): embed the ontology modules from inside the crate` — moved
  `_module_*.yaml` from `tests/kwaai-knowledge/ontologies/` to
  `core/crates/kwaai-rag/ontologies/`, and changed the shipped ontology library from 18 to 15
  candidate modules. No CLAUDE.md or GLOSSARY.md references that old path or a module count
  (GLOSSARY.md's "17" at line 87 is the unrelated global *entity-type* default, not ontology
  library modules). Not invalidated.
- **#260** `feat(nix): kwaainet-cuda package`, **#261** `ci(release): reuse candle-flash-attn's
  CUDA kernels`, **#262** `fix(nix): stop release version bumps from rebuilding all
  dependencies`, **#263** `ci: per-job cargo cache keys` — all four are CI/Nix build-pipeline
  changes. No memory file describes Nix packaging, `ci.yml` cache behavior, or the CUDA release
  build in enough detail to be contradicted (`projects/kwaai-platform/CLAUDE.md:72` has a
  one-line table entry, "`.github/workflows/release.yml` | cargo-dist release pipeline" — still
  true; the PR only added CUDA-kernel reuse logic to it).

## Accretion

One item flagged by the checker is worth a human look, though it falls outside this run's
36-hour commit window and I did not fully chase it down:
`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130` still reads "**Status**:
code complete, not yet committed" for a SQLite-WAL thread-safety fix dated 2026-07-12 — over
three months old. I searched `git log --all` for `wal`/`thread.safety`/`sqlite` in commit
subjects and found nothing matching, which is consistent with either (a) it landed under an
unrelated commit message, or (b) it genuinely never shipped. I'm not confident which — it would
need someone who knows the fix to check `core/crates/kwaai-rag/src` directly rather than commit
subjects. Not a new finding (the checker already surfaces it every run); flagging so it doesn't
keep aging silently.

No other removal candidates found in the 13 authoritative files themselves — nothing in them
was made obsolete by this period's commits, and a broader sweep for dead absolute paths/peer IDs
turned up only items already self-flagged in-place (e.g. `projects/kwaai-network/CLAUDE.md`'s
"DNS broken" notes), not new drift.

## Nothing found

No broken references, no semantic contradictions from the last 36 hours' commits, and no new
accretion beyond the one checker-flagged item above.
