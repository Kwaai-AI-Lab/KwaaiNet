# Memory reflection — 2026-11-21

Scope: root `CLAUDE.md`, 11 project/crate `CLAUDE.md` files, `projects/kwaai-knowledge/GLOSSARY.md`.
Checked out at `main` @ `abc4b5db2dde2f3ac2ee127da2a5340327585719`.

## Mechanical — checker output, verbatim

`scripts/check_memory_integrity.py` exists and ran clean:

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files
```

All 135 warnings land in `projects/*/plans/*.md` (forward references to proposed-but-not-yet-created
files, and two `line:function` references drifted by edits) — none in the 12 `CLAUDE.md` files or
`GLOSSARY.md`. Full warning list omitted here since it's unchanged noise from the plan corpus, not a
finding; rerun the script for the live list.

## Semantic — claims now false

`git log --since="36 hours ago" --stat` on `main` returns nothing — the last commit is
`abc4b5d` (2026-11-16 17:47 PST, five days ago). There is no commit in the window to check memory
claims against, so step 2 as scoped (claims invalidated by a recent commit) found nothing.

While verifying accretion candidates below, I did find two claims that are false *today*,
independent of any recent commit — reported here since they fit the "described behaviour a commit
inverted" pattern, just not a commit from the last 36h:

- **`projects/kwaai-network/CLAUDE.md`, "In progress / planned"**: lists "P2P relay routing for
  inference (route through p2p network instead of direct TCP)" as not yet done. It's done:
  `call_block_forward` in `core/crates/kwaai-cli/src/block_rpc.rs` sends every inter-shard
  `InferenceRequest`/`InferenceResponse` via `P2PClient::call_unary_handler` over the `/kwaai/inference/1.0.0`
  protocol — the only TCP listener in that path (`start_local_inference_server` in `shard_cmd.rs`)
  binds `127.0.0.1:0`, a local-loopback bridge, not a cross-node transport. The same file's "Do not"
  line ("Do not send inference requests directly over TCP to metro machines; use p2p relay") already
  describes this as current practice, so the file contradicts itself. Move the bullet to "Shipped."
  I could not date when this became true: `git blame` attributes the whole file to one commit
  (`7f976b0`, 2026-09-25) that also introduces `block_rpc.rs` wholesale, which looks like a monorepo
  squash artifact rather than real simultaneity — treat the dating as unknown, not as evidence either way.

- **`projects/kwaai-storage/CLAUDE.md`, "In progress / planned"**: lists "`vpk discover` — query DHT
  for available VPK nodes (stub implemented, needs DHT integration)". It isn't a stub:
  `discover_nodes()` in `core/crates/kwaai-cli/src/vpk.rs` performs a real DHT `FIND` (`DHTProtocol.rpc_find`)
  against bootstrap peers, SHA1-hashes the peer ID into a Hivemind-compatible DHTID, dedupes and
  filters results, and `kwaainet vpk discover` wires it end-to-end with JSON and human output. Move
  to "Shipped." By contrast I checked the other two bullets in the same block — `vpk shard` and
  `vpk resolve` — and both genuinely print "not yet implemented" and return early, so those are
  accurate as still-planned.

Everything else in "In progress / planned" across the other `CLAUDE.md` files I spot-checked
(`kwaai-trust`'s EigenTrust/PVP-1/Sybil-resistance items, `kwaai-network`'s intent-casting and
`kwaainet_version` items) has no matching code yet — those claims stand.

## Accretion — what could be removed and why

- **`~/.claude/plans/cached-jingling-creek.md`**, referenced from `CLAUDE.md`:75, `projects/kwaai-compute/TODO.md`:11,
  and `projects/kwaai-compute/roadmap.md`:17 as "the plan" for the dedicated-inference-thread work.
  There is no `.claude/` directory anywhere in this repo. The path is local to whoever ran Claude Code's
  plan mode on their own machine; it isn't a repo path, isn't shared, and the checker has no way to flag
  it as broken since it never tries to resolve `~`-paths. Three files currently point a reader at a file
  none of them can open. Same pattern, lower stakes, in `docs/TODO.md`:132,200,248 (`.claude/plans/...`),
  outside the 12-file authoritative set but worth the same fix: either inline the plan's substance or
  drop the reference.
- **`projects/kwaai-trust/CLAUDE.md`, "Key source files" table**: `kwaai-cli/src/identity.rs` is listed
  twice, once as "`kwaainet identity` command handler" and again two rows later as "`kwaainet identity`
  handler — DID, VC import/list/verify". Harmless but it's a copy-paste leftover; collapse to one row.
- **Hardcoded peer IDs** for `metro-linux`/`metro-win`/`jerome` appear in both `projects/kwaai-compute/CLAUDE.md`
  and `projects/kwaai-knowledge/CLAUDE.md` (duplicated, not just cross-referenced). I can't verify from
  here whether these three peer IDs are still live — that needs someone who can reach the actual nodes
  (`kwaainet p2p info` run against each machine, or a DHT lookup). Flagging as "check, don't yet remove":
  if any of these machines has been reprovisioned, its identity key regenerates and the old ID becomes a
  silent dead end with no checker coverage, same blind spot as the plan-path issue above.

## Nothing found

No renamed/removed CLI subcommand still documented, no default/threshold/cap mismatch, and no
"do not do X" guidance made obsolete by new machinery, found in this pass.
