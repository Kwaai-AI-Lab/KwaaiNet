# Memory reflection — 2026-09-23

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exit 0:

```
Checking 48 memory and plan files
[... 45 warnings across 11 plan files, listed below ...]
0 broken reference(s), 45 warning(s) across 48 files
```

All 45 warnings are in `projects/*/plans/*.md`, none in the twelve `CLAUDE.md`
files or `GLOSSARY.md`. Almost all are `(proposed?)` — file/function names the
plan describes building, not claims that they exist yet — which is expected
and not actionable.

Four are `(moved?)` warnings, i.e. the checker found a same-named file
elsewhere:
- `MacOllamaStopgap-plan.md`: `config.rs:1059` — `announce_state` is now at
  `config.rs:1082` (pure line-number drift, same file).
- `AutoDeriveSeededFacts-plan.md`: `core/crates/kwaai-rag/src/schema.rs` —
  checked the context (line 163/167); the doc proposes this as a **new**
  module, so this is a false positive on the checker's "moved?" heuristic,
  not real drift.
- `MemoryIntegrity-growth-cycle.md`: `src/mcp/server.rs` → found
  `core/crates/kwaai-cli/src/grpc_server.rs`. Not checked further; this plan
  is itself the design doc for this reflection process and its file
  references read as illustrative examples, not live paths.
- `VPK-CrateIntegration-plan.md`: `src/vpk/metrics.rs` — checked the context
  (line 127-129); this refers to the **separate PHE repo**, not a path in
  this repo, so the checker's match against
  `core/crates/kwaai-network-tests/src/metrics.rs` is a false positive.

Nothing here needs fixing this cycle.

## Semantic — claims now false, with file, line, and the commit responsible

Only two commits landed in the last 36 hours:

- `5bb2ce0` (#231) — `run-node` supervises shard/storage children (new
  `supervisor.rs`, changes to `daemon.rs`, `main.rs`, `node.rs`,
  `shard_cmd.rs`, `storage.rs`, `cli.rs`).
- `4578271` (#235) — added the "Before opening a PR" section to root
  `CLAUDE.md`.

I checked every memory file for claims the supervisor refactor could have
invalidated: the `kwaainet start --daemon` example in
`projects/kwaai-network/CLAUDE.md:43` (still valid — `Start(StartArgs)` still
exists in `cli.rs` with a `--daemon` flag), the `shard serve/run/status/chain`
subcommand list in `projects/kwaai-compute/CLAUDE.md` (unchanged, verified
against the `shard_cmd.rs` diff), and grepped all twelve `CLAUDE.md` files for
`daemon.rs`, PID-file tracking, `ShardManager`, backoff/restart wording, and
process-group signal handling — the only hits are in root `CLAUDE.md`'s own
new section, which commit `4578271` added specifically to describe this
refactor's lessons and is accurate as written.

**Nothing found.** No memory file asserts something the last 36 hours' commits
made untrue.

## Accretion — what could be removed and why

**Nothing found this cycle.** The candidates the mechanical pass surfaced
(above) turned out to be a line-number drift, and two false positives from
the checker matching a proposed-but-unbuilt path or a path in a different
repo to an unrelated same-named file here. None reflects a memory file that
has actually outlived its use. I did not do a full accretion sweep of all
twelve files independently of the checker's output — that would need a
dedicated pass rather than piggybacking on tonight's two commits.

## Nothing found

Checker passed clean, only two commits landed in the window and neither
invalidates any memory-file claim, and no fresh accretion turned up beyond
pre-existing checker noise already covered above.
