# Memory Reflection — 2026-10-22

Checked out at `8de6705` (origin/main, confirmed up to date). Commits in the last 36h: `8de6705`
(#244, RPC event streaming) and `662caa6` (#248, per-document ontology selection).

## Mechanical — checker output, verbatim

```
Checking 53 memory and plan files
... (117 warnings, all in projects/kwaai-knowledge/plans/*.md: forward-declared future
     files/functions marked "(proposed?)", and a handful of line-number drift on symbols
     that have since moved within their file)
0 broken reference(s), 117 warning(s) across 53 files
```

All 117 warnings are pre-existing plan-doc drift (the checker's own design treats these as
"reflect, don't necessarily fix" — see its closing line). None are new since the last run; none
trace to either of the last 36h's commits. Full raw output is in the session transcript, omitted
here to avoid padding a report where nothing changed.

## Semantic — claims now false

**Nothing found.** Both recent commits touched their own documentation in the same commit and
got it right:

- `662caa6` rewrote `GraphAction::BuildGraph`'s `--reset-graph` help text in `cli.rs` to match
  what `GraphStore::clear_graph_data` (`core/crates/kwaai-rag/src/graph.rs:4731`, unchanged by
  this commit) actually clears (entities, relations, mentions, timeline — not chunks, ontology,
  entity schemas or doc metadata). Checked `projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md:31`,
  which states the older, less complete version of this claim ("wipes entities and relations but
  keeps chunks") — it's incomplete but not false, and it's a dated historical design note, not
  live guidance, so not counted as a finding.
- `662caa6` also fixed the one line in `projects/kwaai-knowledge/CLAUDE.md` that was already
  wrong (a `--file` flag that doesn't exist — `ingest` takes `file` positionally, confirmed at
  `cli.rs:1216-1218`) as part of the same diff.
- `8de6705`'s new gRPC event streaming is opt-in (`ShardRunRequest.events`) and additive only —
  diffed `shard_cmd.rs` for removed functions: none. `projects/kwaai-compute/CLAUDE.md`'s
  `shard serve/run/status/chain` subcommand list is still accurate.
- Checked the "do not enable unfiltered relation extraction for 8B models" guidance in
  `projects/kwaai-knowledge/CLAUDE.md:150` (the shape of claim the task asked me to watch for)
  against `662caa6`'s ontology-trigger change. Not invalidated: the commit message itself says
  ontology triggers *widen* the existing lexical-trigger gate and never replace it, so the
  Phase-4 pipeline this guidance points to is still the correct answer.
- The PR's own "found and not fixed" list (dream/`graph score` still use the KB-wide ontology
  union, not each document's) is a known limitation stated in the commit message, not a
  contradicted memory-file claim — checked GLOSSARY.md and found no claim asserting the
  opposite.

## Accretion

1. **`projects/kwaai-compute/CLAUDE.md:75`** — `Dedicated inference thread with session pool,
   LRU eviction (plan: ~/.claude/plans/cached-jingling-creek.md)`. That path is some past
   session's local Claude Code plan cache, not a repo path; it doesn't exist in this checkout
   and can't exist in anyone else's. Either replace with a real repo-relative doc or drop the
   pointer.

2. **`projects/kwaai-network/CLAUDE.md:63`** vs. the same file's own `:92` — "In progress /
   planned" lists *"P2P relay routing for inference (route through p2p network instead of
   direct TCP)"* as not yet done, but the `Do not` section two lines away already treats it as
   shipped policy ("Do not send inference requests directly over TCP to metro machines; use p2p
   relay"), and `projects/kwaai-compute/CLAUDE.md` / `projects/kwaai-knowledge/CLAUDE.md` both
   document working `p2p://`/`mux://` inference routing with examples, backed by
   `core/crates/kwaai-cli/src/inference_mux.rs` (exists, implements `mux://`). This isn't tied
   to either recent commit — it's older drift — but it's a self-contradiction within one file
   and worth a line move from "planned" to "shipped" or deletion.

3. **Hardcoded peer-ID table** (metro-linux/metro-win/jerome with full `12D3KooW...` strings) is
   duplicated verbatim in `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md`
   (machine names without the IDs also appear a third time in `projects/kwaai-network/CLAUDE.md`).
   Can't verify from the repo whether these peer IDs are still live; flagging the duplication
   itself as a sync liability, not asserting they're stale.

4. **`projects/kwaai-trust/CLAUDE.md:56-57`** — `identity.rs` is listed twice in the "Key source
   files" table, once as `kwaainet identity command handler` and once as `handler — DID, VC
   import/list/verify`. Collapse to one row.

5. **`projects/kwaai-knowledge/plans/D6-eval-report-20260609.md`** (moderate confidence, headers
   only) — a June 2026 snapshot (63.1% recall) that reads superseded by later dated results
   (`D6-FullAB-results.md`, Aug; `RAGPerformanceReport-20260712.md`, Jul 12). No open action
   items found in what was sampled. Worth a closer read before archiving rather than acting on
   this alone.
