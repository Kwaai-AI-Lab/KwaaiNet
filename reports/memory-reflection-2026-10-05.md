# Memory reflection — 2026-10-05

Checked out at `origin/main` @ `97d0b1e235f4d2bd9dbf8645013e288e3d026ad9` (2026-10-01).

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files
...
0 broken reference(s), 65 warning(s) across 51 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

Exit code 0. All 65 warnings are in `projects/*/plans/*.md` (proposed-but-not-yet-written
files, or line numbers that drifted after later edits) — none are in the 12 `CLAUDE.md`
files or `GLOSSARY.md`. I spot-checked a sample against current source (see Accretion)
rather than re-verifying all 65; none looked urgent enough to justify the time against
tonight's "be brief" mandate. Full list is in the run above if a future pass wants it.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned **no commits** — the last commit on
`main` is `97d0b1e` from 2026-10-01 16:31 PDT, about 89 hours before this run, not 36.
There is nothing to check by the method Step 2 specifies.

One stale claim turned up anyway, via the mechanical checker's own warning rather than
the 36-hour diff, so I'm reporting it here instead of silently sitting on it:

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** reads
  `**Status**: code complete, not yet committed.` for the redb→rusqlite/WAL migration.
  This is false: `rusqlite` has been in `core/crates/kwaai-rag/Cargo.toml` since commit
  `952f737` (2026-09-09, "docs: record what went wrong reviewing the September PR
  queue" — a squash-merge batch, not literally about this migration), and
  `meta_store.rs` currently opens `.db` files with a migration check for legacy
  `.redb` files. The status line is about four weeks stale.

Nothing else: no deleted/renamed CLI subcommand, no default/threshold/flag change, and
no inverted "do not" guidance found. On that last point specifically — the one example
in the task brief ("do not re-enable relation extraction for 8B models") is already
current in `projects/kwaai-knowledge/CLAUDE.md`'s Do-not section: it names the Phase-4
axiomatic pipeline as the safe alternative, so that stale pattern has already been fixed
here, not newly found tonight.

## Accretion — what could be removed and why

- **The `RAGPerformanceReport-20260712.md:130` status line above** should just be fixed
  (change to "Status: committed, `952f737`") or the line deleted if the report is
  otherwise frozen as a point-in-time snapshot — leaving it as "not yet committed" will
  keep misleading anyone who reads only this file.
- **Same file's GPU table** (`jerome — ❌ Offline; 148+ consecutive "routing: not found"
  failures`) is from 2026-07-12, nearly three months old. I can't verify current peer
  status from this environment (no live p2p access), so I'm not asserting it's wrong —
  only flagging that a three-month-old infra snapshot sitting next to the still-active
  `projects/kwaai-knowledge/CLAUDE.md` peer-ID table (which lists `jerome` with no
  offline note) is exactly the kind of silently-aging claim this reflection exists to
  catch. Worth a live check next time someone touches that infra.
- **Hardcoded peer IDs** for `metro-linux`, `metro-win`, `jerome` are duplicated
  verbatim in both `projects/kwaai-compute/CLAUDE.md` and
  `projects/kwaai-knowledge/CLAUDE.md`. Peer IDs regenerate if a node's identity key is
  ever reset, and nothing would flag it here if that happened — not a finding against
  today's content (I have no evidence these are wrong), just noting the duplication and
  the silent-staleness risk for whoever next reinstalls one of those machines.
- Everything else scanned (the 65 checker warnings, the `plans/` directory generally)
  read as either genuinely-proposed future work (correctly unresolvable) or
  already-landed experiment logs that record their own results inline (e.g.
  `D6-NoSeed-Dream-plan.md`, which documents a trial whose fixes shipped in `97d0b1e`
  but which already contains its own dated result sections rather than stale
  forward-looking claims) — not accretion in the sense this step is after.

## Nothing found

No broken references (0, per checker). No CLI subcommand renames/removals, no
default/threshold/flag drift, and no inverted guidance found — Step 2's full 36-hour
window had zero commits to check in the first place.
