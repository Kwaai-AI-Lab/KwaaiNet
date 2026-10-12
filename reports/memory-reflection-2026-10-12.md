# Memory Reflection — 2026-10-12

Checked out at `d46bb4a` (origin/main, 2026-10-07).

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files
... (101 warnings across 51 files, listed by file below)
0 broken reference(s), 101 warning(s) across 51 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

Full per-file warning list (unabridged, as produced by `python3 scripts/check_memory_integrity.py`,
exit 0):

- `projects/kwaai-compute/plans/MacOllamaStopgap-plan.md` — `config.rs:1059` vs actual `config.rs:1082` for `KwaaiNetConfig::announce_state` (line drift only, function unchanged).
- `projects/kwaai-knowledge/plans/*` (18 files) — mostly line-number drift on function references (`graph.rs`, `rag_cmd.rs`, `sequence.rs`, `retriever.rs`, `meta_store.rs`) and references to D6 corpus files (`d6_family_tree.yaml`, `d6_doc_schema.yaml`, etc.) that the checker correctly flags as "proposed?" — these live in the private, gitignored D6 copy per `projects/kwaai-knowledge/CLAUDE.md`, so a fresh clone never has them. Not stale; expected by design.
- `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md` — flagged by the checker as "status says not yet committed but core/crates/kwaai-rag/src has changed since — recheck whether it has landed." **Investigated — see Semantic/Accretion below: the checker's suspicion is correct, this is genuinely stale.**
- `projects/kwaai-network/plans/InferenceHostSupervisor-plan.md`, `projects/kwaai-platform/plans/PublicRelease-plan.md`, `projects/kwaai-storage/plans/VPK-CrateIntegration-plan.md` — proposed/moved file references, same pattern as above (plan docs describing future or renamed files).

No broken references. All warnings are either (a) "proposed?" paths in plan docs for files that live outside the public repo by design, (b) harmless line-number drift on functions that still exist under the same name, or (c) the one genuine case called out below.

## Semantic — claims now false, with file, line, and the commit responsible

**No commits landed on `main` in the last 36 hours** (`git log --since="36 hours ago"` is empty;
the latest commit, `d46bb4a`, is from 2026-10-07, five days ago). There is nothing for this step
to check this run — no commit in the window could have invalidated a memory file's claim.

One item from outside the 36-hour window, surfaced by the checker rather than the commit scan,
checked anyway since it was already in hand:

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** states "**Status**:
  code complete, not yet committed" about the redb→rusqlite/WAL migration described just above it
  (lines 118–129: `MetaStore: Mutex<SafeConn>`, WAL pragmas, `.redb`→`.db`). This is false as of
  now: `rusqlite = { version = "0.31", features = ["bundled"] }` is in
  `core/crates/kwaai-rag/Cargo.toml`, and `core/crates/kwaai-rag/src/meta_store.rs:88` contains
  the exact `PRAGMA journal_mode=WAL;` pragma the plan describes. The migration landed in commit
  `4aec1f0` ("ci: fail a PR that inserts a Rust item into an existing doc block (#214)",
  2026-09-10) — over a month before this status line was last touched. This is a dated snapshot
  report, not a living doc, so the fix is to add a landed-status note rather than rewrite it (see
  Accretion).

Also checked directly, since the task background names it as the canonical example of this kind
of staleness: `projects/kwaai-knowledge/CLAUDE.md`'s "Do not" section ("Do not enable *unfiltered*
relation extraction for 8B models") already documents the Phase-4 lexical-trigger/confidence-split
pipeline as the sanctioned alternative — it is not making the stale claim the example warns about.
Nothing to fix there.

## Accretion — what could be removed and why

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — as above, a "not yet
  committed" status that has been landed for ~5 weeks. This is a dated snapshot report (filename
  carries the date, not a living plan), so the right fix is a one-line addendum ("Landed in #214,
  2026-09-10") rather than editing the historical status, and it's a small enough edit that it's
  worth doing on the next pass through this file rather than leaving it to rot further.

- **Duplicated, unflagged-offline infra table.** `projects/kwaai-compute/CLAUDE.md:32-36` and
  `projects/kwaai-knowledge/CLAUDE.md:103-111` both hardcode the same three peer IDs
  (metro-linux, metro-win, jerome) as if all three are equally available. The dated
  `RAGPerformanceReport-20260712.md:114` records that **jerome was offline** at the time ("148+
  consecutive 'routing: not found' failures; excluded from pipeline") with an open TODO to
  SSH in and investigate. Neither CLAUDE.md file carries that caveat — a reader following either
  table today has no way to know jerome needs checking before use. I can't verify jerome's current
  status from this checkout (no live access to the p2p network); flagging this as something to
  verify rather than asserting it's still broken. If it's still offline, both tables should say so
  inline (or jerome should be dropped from them); if it's back, nothing to do. This is also a
  second instance of the general risk this section exists to flag: a hardcoded peer ID, duplicated
  across two files, that depends on physical infrastructure state the repo has no way to track.

Nothing else stands out as pure accretion this pass — the rest of the 101 checker warnings are
either design-intentional (private-corpus "proposed?" paths) or cosmetic line drift, not content
that asserts something false.

## Nothing found

No commits landed in the 36-hour window this run covers, so Step 2's commit-driven scan is clean
by construction — there was nothing to check. The eleven other CLAUDE.md files (all but
`projects/kwaai-knowledge/CLAUDE.md` and `projects/kwaai-compute/CLAUDE.md`) and
`projects/kwaai-knowledge/GLOSSARY.md` turned up no contradicted claims, no broken "do not" rules,
and no superseded/deprecated language on a targeted grep (`superseded|deprecated|no longer|
obsolete|outdated`) beyond the root `CLAUDE.md`'s own documented-and-resolved `.docx` lesson.
