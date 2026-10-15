# Memory Reflection — 2026-10-15

Reviewed at `913e0bc` (origin/main). No prior reflection report exists in this
repo, so this is the first run.

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exits 0:

```
Checking 51 memory and plan files
...
0 broken reference(s), 101 warning(s) across 51 files
```

All 101 warnings are in `projects/kwaai-knowledge/plans/*.md` — file/function
references the checker itself marks `(proposed?)`, i.e. forward references to
work the plan describes but hasn't landed, plus a few `funcname()` drifts
where a referenced function moved a handful of lines (e.g.
`graph.rs:1912` → `coref_candidates_for_chunk` now at `graph.rs:1998`). None
of this is in the twelve CLAUDE.md files or GLOSSARY.md. No broken
references anywhere.

## Semantic — claims now false, with file, line, and the commit responsible

Only two commits landed in the last 36 hours:

- `913e0bc` fix(updater): don't self-update a musl/non-x86_64 host to the
  gnu-cuda build (#236) — changes `core/crates/kwaai-cli/src/updater.rs`.
- `74b8f84` test(kwaai-rag): neutral surname in the namesake-ordering
  fixture (#241) — test-only change to `core/crates/kwaai-rag/tests/suite.rs`.

Checked both against the twelve CLAUDE.md files and GLOSSARY.md:

- `projects/kwaai-platform/CLAUDE.md` and root `CLAUDE.md` mention
  `updater.rs` only as "self-update: release lookup, download, binary swap" —
  no claim about which build variant it chooses, so #236 doesn't contradict
  anything documented.
- No memory file mentions `graph_of_namesakes`, the `Gool`/`Doe` fixture
  surname, or anything #241 touches.

**Nothing found.**

## Accretion — what could be removed and why

- `projects/kwaai-compute/CLAUDE.md:75` — "Dedicated inference thread with
  session pool, LRU eviction (plan: `~/.claude/plans/cached-jingling-creek.md`)"
  points at a path under the author's local `~/.claude/`, outside this
  repository. No other contributor or agent (including this one) can resolve
  it, and `check_memory_integrity.py` doesn't flag it because it isn't a
  repo-relative path. I confirmed the feature itself is still unshipped (no
  `LRU`/session-pool code in `core/crates/kwaai-inference/src/*.rs` or
  `shard_cmd.rs`), so the line is accurate, just pointing at a dead
  reference for anyone but its author. Worth either inlining a one-line
  summary of the plan or dropping the path.

- The GPU-machine peer-ID table (`metro-linux`, `metro-win`, `jerome`) is
  duplicated verbatim in two places: `projects/kwaai-compute/CLAUDE.md:30-34`
  and `projects/kwaai-knowledge/CLAUDE.md:103-105` (plus the matching
  `--inference-urls` example on each). Two copies of the same peer IDs will
  drift the next time a machine is re-provisioned and only one file gets
  updated. I can't verify from the repo whether these peer IDs are still
  live (that needs a DHT lookup or SSH to the machines, which I don't have
  from here) — flagging the duplication, not the values. Worth making one
  file the source of truth and having the other link to it.

No other candidates found: the remaining CLAUDE.md files and GLOSSARY.md are
short (22-303 lines each) and I didn't find finished-work sections, stale
defaults, or dead "do not" guidance in them.
