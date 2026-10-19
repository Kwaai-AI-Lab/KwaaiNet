# Memory reflection — 2026-10-19

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files
...
0 broken reference(s), 101 warning(s) across 51 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 101 warnings fall inside `projects/kwaai-knowledge/plans/*.md` — proposed files not yet created, or line numbers that drifted by a few lines within a function that still exists under the same name. None touch the twelve `CLAUDE.md` files or `GLOSSARY.md`.

## Semantic — claims now false, with file, line, and the commit responsible

No commits landed in the last 36 hours (`git log --since="36 hours ago"` is empty; HEAD at `913e0bc`, 2026-10-14, is 5 days old). There is nothing a recent commit could have invalidated, so this section is empty by construction rather than by a clean read of history.

I did check the one case the task brief specifically flagged as a past example — `projects/kwaai-knowledge/CLAUDE.md:150`, "Do not enable *unfiltered* relation extraction for 8B models." This line has already been updated past that failure mode: it now names the Phase-4 pipeline (`--relation-threshold-high`) that makes filtered extraction safe, and that flag is real and wired up in `core/crates/kwaai-cli/src/rag_cmd.rs`. Nothing to fix here.

## Accretion — what could be removed and why

**`projects/kwaai-storage/CLAUDE.md:63`** lists `vpk discover` as "stub implemented, needs DHT integration" under **In progress / planned**. Reading `core/crates/kwaai-cli/src/vpk.rs::discover` (and the `discover_nodes` helper it calls, ~line 530) shows a complete implementation: it connects to the running node's p2pd, issues a real `DHTProtocol.rpc_find` over the bootstrap peers, decodes `FindResponse`, dedupes by peer ID, and drops self — including inline commentary explaining the SHA1-DHTID conversion and why individual bootstrap failures are swallowed rather than propagated. This reads as finished work misfiled under "planned." I can't pin the exact landing commit — this clone's history is shallow (50 commits) and `git log --follow` on the file only surfaces one entry, a large unrelated commit (`13515c2`, #193) that happens to touch this file too — but the code itself is unambiguous. Recommend moving this bullet to **Shipped** (or removing it, since the main bullet list already covers DHT advertising).

**Duplicated hardware table.** The same three-row P2P node table (`metro-linux`, `metro-win`, `jerome` with their `12D3KooW...` peer IDs) is copy-pasted into both `projects/kwaai-compute/CLAUDE.md` (lines 32–36) and `projects/kwaai-knowledge/CLAUDE.md` (lines 103–111), in slightly different formats (bare peer ID vs. `p2p://`-prefixed). I can't verify from this checkout whether these three peer IDs are still live — that needs a DHT lookup or asking whoever administers those boxes. Flagging as *unsure, needs checking* rather than asserting staleness. Independent of liveness, the duplication itself is worth collapsing to one source (e.g. the compute doc links to it) so a change to one doesn't silently leave the other wrong.

**`~/.claude/plans/cached-jingling-creek.md`** is referenced from three places (`projects/kwaai-compute/CLAUDE.md:75`, `TODO.md:11`, `roadmap.md:17`) for the still-unimplemented session-pool/LRU work (verified: no `LRU`, `eviction`, or `SessionPool` hits anywhere in `core/crates/kwaai-inference/src/`). The claim itself is accurate, but the path points into a local Claude Code plans directory that isn't part of this repo, so no other agent or contributor can actually open it. Not new accretion — it's consistently used across three files, so likely an intentional convention — but worth noting as something future cleanup could resolve by committing the plan into `projects/kwaai-compute/plans/` if it's meant to be shared.

## Nothing found

- No broken mechanical references in the twelve `CLAUDE.md` files or `GLOSSARY.md`.
- No commits in the last 36 hours, so no code-invalidated claims to report this cycle.
- `GLOSSARY.md` was spot-checked (section headers, mechanical pass) but not read term-by-term against current code; I'm not confident enough to say it's clean, just that nothing surfaced.
