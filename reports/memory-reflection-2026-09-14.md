# Memory reflection — 2026-09-14

Checked at HEAD `e66dd62273f367fbee4a18ad4cf4607bbc489926` (= `origin/main`).

## Mechanical — checker output, verbatim

```
Checking 13 memory files

0 broken reference(s), 0 drift warning(s) across 13 files
```

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned nothing — the most recent commit
(`e66dd62`) is from 2026-09-11 19:21 -0700, about 56 hours before this run. No commit
in the window, so there is nothing a commit in the window could have invalidated.

That leaves the commit-attribution requirement of this section technically unmet, but
three verified-false claims turned up anyway while cross-checking code for the
accretion pass below. They predate the window (all traceable to commit `ec51c1a`,
2026-08-31, or code that landed since), so they are reported here as findings, not as
answers to "what did last night's commits break":

- **`projects/kwaai-network/CLAUDE.md`**, Infrastructure section ("Fix needed: p2p
  relay routing for inference requests…") and Current State → In progress/planned
  ("P2P relay routing for inference (route through p2p network instead of direct
  TCP)"). This is shipped, not planned: `p2p://PEER_ID` / `mux://PEER_ID` inference
  routing is fully implemented in `kwaai-cli/src/ollama_proxy.rs`,
  `inference_mux.rs`, and `capacity_lease.rs`, wired into `--inference-url(s)` in
  `cli.rs`. Landed in `ec51c1a` (2026-08-31). Both `projects/kwaai-compute/CLAUDE.md`
  and `projects/kwaai-knowledge/CLAUDE.md` already document it as working
  infrastructure with real peer-ID examples — this file is the odd one out.
  Commit `1a9d2d6` ("docs: say what the code actually does", #188, 2026-09-04)
  touched this same file to fix two other stale claims and missed this one.

- **`projects/kwaai-storage/CLAUDE.md`**, Current State → In progress/planned:
  "`vpk discover` — query DHT for available VPK nodes (stub implemented, needs DHT
  integration)". `discover()`/`discover_nodes()` in `kwaai-cli/src/vpk.rs` is a
  complete implementation: it connects to the running p2pd daemon, issues a real DHT
  `FindRequest`, and returns live results in both human and JSON form. Not a stub.
  (The neighboring claim about `vpk shard`/`vpk resolve` is still accurate — the code
  itself prints "planned for Phase 3" at runtime.)

- **`core/crates/kwaai-inference/CLAUDE.md:20`** and
  **`projects/kwaai-compute/CLAUDE.md:88`**: both cite the RoPE `broadcast_mul` fix at
  `shard.rs:104`. Line 104 is now mid-way through building `sin4`; the actual
  `broadcast_mul` call is at line 109. The surrounding code hasn't changed since
  `ec51c1a` (2026-08-31) — the two docs just never got the line number right, or it
  drifted from an earlier edit. The mechanical checker only verifies that a *file*
  reference resolves, not a line number, so this class of drift is structurally
  invisible to it.

## Accretion — what could be removed and why

- **Hardcoded peer IDs, five times over.** The same three GPU peer IDs
  (`metro-linux`, `metro-win`, `jerome`) are typed out in full five times across two
  files: once in a table plus once in an example command in
  `projects/kwaai-compute/CLAUDE.md`, and once in a table plus twice in example
  commands in `projects/kwaai-knowledge/CLAUDE.md`. A single peer-ID rotation
  requires five synchronized edits. Worth collecting in one place both files can
  point at.

- **`projects/kwaai-compute/CLAUDE.md:75`** points to
  `~/.claude/plans/cached-jingling-creek.md` for the session-pool/LRU-eviction plan.
  That path is local to whoever wrote it — outside the repo, unreadable by any other
  contributor or session (confirmed absent in this checkout's home directory). Either
  inline a one-line summary of the plan or drop the pointer.

- **`projects/kwaai-trust/CLAUDE.md:56-57`** — duplicate table row for the same file:

  ```
  | `kwaai-cli/src/identity.rs` | `kwaainet identity` command handler |
  | `kwaai-cli/src/identity.rs` | `kwaainet identity` handler — DID, VC import/list/verify |
  ```

  Both landed in `ec51c1a` and have sat unmerged for two weeks. One should go.

- **`projects/kwaai-knowledge/GLOSSARY.md:151`** — "Ontology… New in this branch;
  see `plans/PerKBOntology-plan.md`." `ontology.rs` and the feature it describes have
  been on `main` since `ec51c1a` (2026-08-31, ~2 weeks). There is no branch anymore;
  the phrasing reads as more provisional than the shipped code actually is.

## Nothing found

- No CLI subcommand renames or removals in the window (there were no commits in the
  window at all): `vpk`, `shard`, `identity`, `reputation` subcommands in
  `cli.rs`/`vpk.rs`/`shard_cmd.rs`/`identity.rs`/`reputation_cmd.rs` all match what
  their respective `CLAUDE.md` files claim.
- `kwaai-knowledge/CLAUDE.md`'s "Do not enable unfiltered relation extraction for 8B
  models" note already references the Phase-4 pipeline that fixed it — this is the
  pattern the task description warned about, but it's already correctly caveated, not
  stale.
- cargo-dist version (`0.31.0`), D6 eval question count (40), and the `vpk shard`/
  `vpk resolve` "planned" claims all check out against the repo/code as documented.
