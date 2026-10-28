# Memory reflection — 2026-10-28

Scope: `main` @ `c6da5da`. `git log --since="36 hours ago"` is empty — the last
commit (`c6da5da`, 2026-10-23) is five days old, so Step 2 as specified
(claims a *recent commit* made untrue) has nothing to check. While reading all
13 memory files for Step 3 I found several claims that are false independent
of the 36h window; they're reported under Semantic below rather than silently
dropped, with the commit history that explains each one where I could find it.

Note on git history: this checkout is shallow (50 commits, oldest
`b402540`, 2026-09-10). Where a claim predates that boundary I say so instead
of guessing a commit.

## Mechanical — checker output, verbatim

```
Checking 53 memory and plan files
...
0 broken reference(s), 117 warning(s) across 53 files
```

All 117 warnings fall inside `projects/*/plans/*.md`, which the checker
deliberately treats as "proposed" references (per its own header comment) —
none are in the 13 always-loaded files (12 `CLAUDE.md` + `GLOSSARY.md`), which
come back fully clean.

## Semantic — claims now false

No commit in the last 36 hours, so none of these were caused by one — all
predate this window. Found by reading the files against current code.

1. **`core/crates/kwaai-storage/CLAUDE.md:3,15`** — describes the crate as
   "multi-tenant **homomorphic-encrypted** vector storage" and
   `src/vectors.rs` as "**Homomorphic vector search**". This is the exact
   overclaim that commit `1c0b342` (2026-09-14, PR #169, "docs(storage): stop
   claiming homomorphic search ships") corrected — but only in
   `projects/kwaai-storage/CLAUDE.md`, `design/data-flows.md`,
   `design/overview.md` and `roadmap.md` (per `git show --stat 1c0b342`). The
   crate-level shortcut file wasn't in that commit's diff and still has the
   pre-fix wording. Verified against code: `grep -i 'homomorphic\|phe'
   core/crates/kwaai-storage/src/*.rs` returns nothing; `vectors.rs` stores
   raw `f32` bytes (`f32s_to_bytes`) with plain cosine search, matching PR
   #169's own description ("the host stores opaque f32 and ranks with plain
   cosine").

2. **`projects/kwaai-knowledge/GLOSSARY.md:36`** — "**VPK** — the encrypted
   multi-tenant storage fabric Eve nodes run." Same overclaim, also missed by
   `1c0b342` (not in that commit's file list either).

3. **`projects/kwaai-storage/CLAUDE.md:90`** ("Do not" section) — "Do not
   store unencrypted vectors for `eve`/`bob` modes" directly contradicts
   lines 6 and 14 of the *same file* ("vectors are plaintext f32 today —
   encryption is planned" / "multi-tenant vectors (plaintext today)"), which
   is the corrected wording PR #169 itself put in place. The "Do not" line
   looks like it survived that fix unreconciled — as written, following it
   is impossible given the state the rest of the file describes.

4. **`projects/kwaai-network/CLAUDE.md:62-63`** — lists "P2P relay routing
   for inference (route through p2p network instead of direct TCP)" under
   "In progress / planned." It's shipped: `kwaai-cli/src/inference_mux.rs`,
   `ollama_proxy.rs`, and the `p2p://`/`mux://` URL schemes are live in
   `rag_cmd.rs`, `shard_cmd.rs`, and `capacity_lease.rs`. Both
   `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md`
   (the latter last touched 2026-10-20, the most recently edited memory file
   in the repo) already document it as shipped, preferred infrastructure with
   working peer IDs and example commands. I can't name the commit that
   shipped it — the feature already existed at this checkout's shallow
   boundary (`b402540`, 2026-09-10) — but the "planned" status is actively
   contradicted by two sibling docs and by the code today.

## Accretion — what could be removed or fixed

1. **`projects/kwaai-knowledge/CLAUDE.md:76-77`** — Legislation/Publication
   per-type recall rows read "TBD (Phase 3 — KB schema injected, r108
   pending)". r108 isn't pending:
   `plans/AutoDeriveSeededFacts-plan.md` (itself last touched 2026-09-10)
   already reports r108's completed result (135/222 = 60.8%, later superseded
   by r110's 149/222). I could not find a per-type recall number for
   Legislation/Publication recorded anywhere in the repo, so I'm not
   confident whether the right fix is to fill in a measured value or just
   drop the stale "pending" annotation — flagging rather than guessing a
   number.

2. **`projects/kwaai-compute/CLAUDE.md:75`** — points to
   `~/.claude/plans/cached-jingling-creek.md` for the session-pool/LRU
   eviction plan. That path is under a personal home directory, isn't part of
   the repo, and doesn't exist in this container (confirmed:
   `ls ~/.claude/plans/cached-jingling-creek.md` → no such file). Meaningless
   to any other contributor or future session; should point into the repo or
   be dropped.

3. **`projects/kwaai-trust/CLAUDE.md:56-57`** — `kwaai-cli/src/identity.rs`
   is listed twice in the "Key source files" table with two slightly
   different descriptions. Harmless but should be merged into one row.

## Nothing found

- No broken references anywhere (0/117 checker warnings are in the
  always-loaded files).
- No renamed/deleted CLI subcommands found contradicting documented ones.
- No commits landed in the review window, so no commit-driven invalidation
  to report under Step 2 as scoped.
