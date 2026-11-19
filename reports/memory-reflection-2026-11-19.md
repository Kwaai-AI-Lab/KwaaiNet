# Memory reflection — 2026-11-19

Checked at `460c69b6f71763644734c92251b961806f69d720` (HEAD == `origin/main`).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files
```

All 135 warnings are in `projects/*/plans/*.md` design docs (proposed files/functions,
one "no file at that path (moved?)" each) — none touch the twelve `CLAUDE.md` files or
`GLOSSARY.md`. 0 broken references.

## Semantic — claims now false, with file, line, and the commit responsible

Six commits landed in the last 36 hours: #307, #279, #275, #283, #306, #268 (full list
in the commit log; #268 is the big one — forward-chain decode routing,
`core/crates/kwaai-cli/src/routing/*`, +2730/-522 across 22 files). I checked each
against the CLAUDE.md files for the areas it touches (kwaai-network, kwaai-p2p,
kwaai-compute, kwaai-inference, kwaai-storage) — crate/file tables, RPC protocol IDs,
CLI subcommands (`shard chain` still exists, cli.rs:834), DHT wire format, "Do not"
lists.

**Nothing in this window's commits makes any of those files' claims false.** The new
forward-chain routing is additive (star stays the default/fallback) and isn't
documented anywhere yet, so there's nothing to contradict.

One pre-existing false claim turned up while reading #283's diff, not caused by it:

- `projects/kwaai-storage/CLAUDE.md:63` — `vpk discover — query DHT for available VPK
  nodes (stub implemented, needs DHT integration)`. Not true: `discover_nodes` in
  `core/crates/kwaai-cli/src/vpk.rs:530` is a real DHT `rpc_find` over the running
  node's p2pd IPC socket, decoding `FoundDictionary`/`FoundRegular` responses. #283 only
  refined its expiry filtering (adds `now`, skips expired subkeys) — the full
  implementation predates it. This repo's shallow clone only reaches back to
  2026-09-25 and `vpk.rs`'s own history in that range is just #283 and one unrelated
  docs commit, so I can't name the commit that actually implemented `discover` — it's
  older than this window. Flagging it here since it's a real false claim, not a
  suspicion.

## Accretion — what could be removed and why

- `projects/kwaai-compute/CLAUDE.md:75` — "plan: `~/.claude/plans/cached-jingling-creek.md`"
  points outside the repo; that path doesn't exist in this checkout (or presumably
  anyone else's). Dead for any reader but the one session that wrote it.
- The three metro-machine peer IDs are hardcoded in both
  `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md`
  (D6 rebuild command). `projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md`
  already named this exact duplication as a "still open" pruning target when it
  proposed this reflection routine — repeating it here rather than re-litigating it.

## Nothing found

No renamed/deleted CLI subcommand, no changed default/threshold/cap, no inverted
behaviour, and no "do not do X" superseded by new machinery in this window's six
commits.
