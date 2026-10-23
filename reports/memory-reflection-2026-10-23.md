# Memory reflection — 2026-10-23

Checked out at `8de6705` (origin/main, HEAD detached, matches).

## Mechanical — checker output, verbatim

```
Checking 53 memory and plan files
...
0 broken reference(s), 117 warning(s) across 53 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 117 warnings are in `projects/*/plans/*.md` files, and almost
all are `(proposed?)` file references — paths the plan itself hasn't landed yet, or
paths in the private, gitignored D6 corpus (`d6_family_tree.yaml`, `D6.yaml`, etc. —
`projects/kwaai-knowledge/CLAUDE.md` already documents these as intentionally absent
from a fresh clone). Not treated as actionable; the checker's own "(proposed?)" tag
already distinguishes these from real drift. The one warning that is actual drift is
covered under Accretion below (`RAGPerformanceReport-20260712.md`).

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned no commits. The most recent commit
(`8de6705`, "stream distributed-inference routing events to gRPC clients", #244) landed
2026-10-21 14:47 -0700, about 36h16m before this run — just outside the window. Nothing
to check against for this step.

## Accretion — what could be removed and why

1. **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — `**Status**:
   code complete, not yet committed.` for the redb→rusqlite WAL migration. This landed
   months ago: `core/crates/kwaai-rag/Cargo.toml` has carried `rusqlite` (no `redb`) since
   commit `2a19d81` ("release(v0.7.2): run-node owns its children and remembers its start
   flags", 2026-09-25), and `meta_store.rs`/`graph.rs` now contain the legacy-migration
   warning (`"Legacy redb store detected... Run kwaainet rag rebuild to migrate"`) that
   only exists post-migration. The status line is ~4 months stale and should say "landed
   in v0.4.148" (or whichever release actually shipped it) rather than "not yet committed".

2. **`core/crates/kwaai-trust/CLAUDE.md`, "Key source files" table** — `kwaai-cli/src/identity.rs`
   is listed twice (lines 56–57, "command handler" and "handler — DID, VC import/list/verify").
   Duplicate row, no new information in the second; one should be dropped.

3. **`core/crates/kwaai-inference/CLAUDE.md:20`** (and the copy at
   `projects/kwaai-compute/CLAUDE.md:88`) — "RoPE fix at `shard.rs:104`:
   `x1.broadcast_mul(&cos4)` not `x1 * cos4`." Checked against the file: line 104 is
   `let sin4 = sin` (part of the surrounding setup); the actual `x1.broadcast_mul(&cos4)`
   call is at line 109 (a second one, `x2.broadcast_mul(&cos4)`, at line 116). Minor —
   the file and the fix are both still real — but the pinned line number has drifted and
   is wrong today. Worth a one-line fix in both copies. (The mechanical checker doesn't
   catch this because it only tracks `file:line` references against known function/const
   symbols, not bare line numbers inside prose.)

4. **`~/.claude/plans/cached-jingling-creek.md`**, referenced from three places —
   `projects/kwaai-compute/CLAUDE.md:75`, `projects/kwaai-compute/TODO.md:11`, and
   `projects/kwaai-compute/roadmap.md:17` — all pointing at the same per-developer,
   outside-the-repo Claude Code plan file for the same not-yet-started "dedicated
   inference thread" item. Flagging as unsure rather than removing: I can't check
   whether that plan file still exists (it's local to whoever's machine wrote it, not
   in this container), so I can't tell if the pointer is live or already orphaned. What
   I'd need to check: ask whoever owns that plan whether `cached-jingling-creek.md` is
   still around, or whether the work it describes has since been captured somewhere
   durable (a repo-tracked plan) that these three files should point at instead.

Not flagged: the `metro-linux`/`metro-win` DNS-broken notes and peer IDs in
`projects/kwaai-network/CLAUDE.md`, `projects/kwaai-compute/CLAUDE.md`, and
`projects/kwaai-knowledge/CLAUDE.md` are exactly the kind of hardcoded infra detail
likely to drift, but verifying them means reaching those machines, which this run
can't do. Noting it so a future pass (or Reza) can confirm the peer IDs and DNS status
are still current.

## Nothing found

No broken references (0/0). No commits in the last 36 hours, so no semantic claim to
check against a specific commit this run.
