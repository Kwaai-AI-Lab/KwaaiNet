# Memory reflection — 2026-10-29

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` (exit 0):

```
Checking 56 memory and plan files
...
0 broken reference(s), 134 warning(s) across 56 files
```

All 134 warnings are in `projects/kwaai-knowledge/plans/` — overwhelmingly
`(proposed?)` references to files not yet created, plus a handful of
function/line-number drift in plan docs (the checker already labels these as
hints, not failures). None touch the 12 `CLAUDE.md` files or `GLOSSARY.md`.
Full warning list is in the command output above (omitted here per the
checker's own framing: drift is a prompt to re-read, not a failure).

One warning is worth a note on its own, see Accretion below:
`RAGPerformanceReport-20260712.md` — "status says 'not yet committed' but
core/crates/kwaai-rag/src has changed since — recheck whether it has landed."

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` shows 8 commits (#245, #246, #242,
c3154e5, #252, #253, #234, #251, #254). I checked each against the 12
`CLAUDE.md` files and `GLOSSARY.md` for the four shapes named in the brief
(removed/renamed CLI subcommand, changed default/threshold/cap, a "do not X"
superseded by new machinery, inverted behaviour):

- #254 (bearer-token auth on every gRPC call) and #245/#246 (new gRPC RAG and
  DHT-diagnostics endpoints) — no memory file mentions gRPC, auth, loopback,
  or TCP credentials at all, so nothing was asserted that these commits could
  contradict. (This is a documentation gap, not a false claim — not reporting
  it as a finding.)
- #251 (graph build no longer silently drops chunks' extraction) — checked
  `projects/kwaai-knowledge/CLAUDE.md` and `GLOSSARY.md` for extraction-rate
  or confidence-default claims this could invalidate. The one close match,
  the "do not enable unfiltered relation extraction for 8B models" rule
  (`projects/kwaai-knowledge/CLAUDE.md:150`), already names its own fix
  (Phase-4 axiomatic pipeline) — it's current, not stale. No numeric recall
  claim in either file was traceable to the pre-fix pipeline.
- #234 (auto-update), #253/#242 (trust-stack diagrams/plan), #252 (CI apt-get
  fix) — none of the five CLAUDE.md files that reference updater, trust, or
  CI content assert anything these commits contradict; the changes are
  additive or fix non-doc-claimed defects.

**Nothing found.** No memory file now asserts something a commit from the
last 36 hours made untrue.

## Accretion — what could be removed and why

- **`RAGPerformanceReport-20260712.md:130`** — "Status: code complete, not
  yet committed" (SQLite WAL migration). Verified against the code: `rusqlite`
  with WAL pragmas (`journal_mode=WAL`, `synchronous=NORMAL`) is live in
  `core/crates/kwaai-rag/Cargo.toml` and `src/{meta_store,graph,cache,feedback}.rs`.
  This has been committed for months; the line is stale. Low stakes — it's a
  dated historical report, not an instruction — but worth a one-line fix or a
  "superseded" banner next time someone opens the file.
- **D6 rebuild command's hardcoded p2p peer IDs** (`projects/kwaai-knowledge/CLAUDE.md:39,103-105,111`)
  — already flagged as debt in `projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md`
  (2026-08-26) and still unaddressed two months later. Not re-litigating the
  case here, just noting it hasn't been actioned.
- **`projects/kwaai-network/CLAUDE.md:31`** — "(swapped from A6000 2026-08)"
  on metro-linux's GPU line. The swap is now ~3 months past; the parenthetical
  has outlived its usefulness and could just become "A5000 24GB."

Nothing else in the 56-file set looked clearly superseded or dead; the large
`projects/kwaai-knowledge/plans/` warning cluster is proposed-future-file
noise the checker already classifies correctly, not actionable accretion on
its own.
