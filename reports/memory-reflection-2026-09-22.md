# Memory reflection — 2026-09-22

Checked out at `19f288d` (origin/main, HEAD detached, matches origin exactly).

## Mechanical — checker output, verbatim

```
Checking 48 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
  projects/kwaai-knowledge/plans/DreamRAG-Intern-Curriculum.md
    warn    file reference does not resolve: `Methods.md` (proposed?)
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-Ontology-Eval-Compression.md
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
  projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md
    warn    file reference does not resolve: `hivemind.rs` (proposed?)
    warn    file reference does not resolve: `network.rs` (proposed?)
    warn    file reference does not resolve: `path/file.rs` (proposed?)
    warn    file reference does not resolve: `reports/memory-reflection-YYYY-MM-DD.md` (proposed?)
    warn    file reference does not resolve: `src/api/mod.rs` (proposed?)
    warn    file reference does not resolve: `src/hivemind.rs` (proposed?)
    warn    `src/mcp/server.rs` — no file at that path (moved? found core/crates/kwaai-cli/src/grpc_server.rs)
    warn    file reference does not resolve: `src/network.rs` (proposed?)
    warn    file reference does not resolve: `trust.rs` (proposed?)
    warn    function reference does not resolve: `fn_name()` (proposed?)
  projects/kwaai-knowledge/plans/PerKBOntology-plan.md
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
  projects/kwaai-knowledge/plans/Phase5-TrainedClassifiers-plan.md
    warn    file reference does not resolve: `core/crates/kwaai-rag/src/entity_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `core/crates/kwaai-rag/src/relation_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `gap_analysis.py` (proposed?)
    warn    file reference does not resolve: `gap_analysis2.py` (proposed?)
    warn    file reference does not resolve: `relation_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `scripts/classifier_train_common.py` (proposed?)
    warn    file reference does not resolve: `scripts/export_entity_training_data.py` (proposed?)
    warn    file reference does not resolve: `scripts/train_entity_classifier.py` (proposed?)
    warn    file reference does not resolve: `scripts/train_relation_classifier.py` (proposed?)
  projects/kwaai-knowledge/plans/d6-rag-accuracy-improvement.md
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_relation_hard_cases.md` (proposed?)
    warn    function reference does not resolve: `is_family_query()` (proposed?)
  projects/kwaai-knowledge/plans/hierarchical-summarization.md
    warn    function reference does not resolve: `search_summaries()` (proposed?)
  projects/kwaai-network/plans/InferenceHostSupervisor-plan.md
    warn    file reference does not resolve: `node_cmd.rs` (proposed?)
    warn    function reference does not resolve: `restart_p2pd()` (proposed?)
    warn    function reference does not resolve: `spawn_p2pd_heartbeat()` (proposed?)
    warn    function reference does not resolve: `spawn_relay_keepalive()` (proposed?)
    warn    function reference does not resolve: `spawn_shard_serve()` (proposed?)
  projects/kwaai-platform/plans/PublicRelease-plan.md
    warn    file reference does not resolve: `LICENSE.md` (proposed?)
    warn    file reference does not resolve: `Ledger-plan.md` (proposed?)
    warn    file reference does not resolve: `OpenAI-Petal/MASS_ADOPTION_STRATEGY.md` (proposed?)
    warn    file reference does not resolve: `SOURCE_CODE.md` (proposed?)
    warn    file reference does not resolve: `TokenEconomy-plan.md` (proposed?)
    warn    file reference does not resolve: `economy.rs` (proposed?)
    warn    file reference does not resolve: `projects/kwaai-trust/plans/Ledger-plan.md` (proposed?)
    warn    file reference does not resolve: `projects/kwaai-trust/plans/TokenEconomy-plan.md` (proposed?)
  projects/kwaai-storage/plans/VPK-CrateIntegration-plan.md
    warn    `src/vpk/metrics.rs` — no file at that path (moved? found core/crates/kwaai-network-tests/src/metrics.rs)

0 broken reference(s), 45 warning(s) across 48 files
```

Exit 0, 0 broken references. All 45 warnings are in `plans/*.md` files and are either
`(proposed?)` forward references to not-yet-written files/functions, or one-line
drift (`config.rs:1059` vs `:1082`) already legible as "hint, not truth" per this
repo's own convention. None of the 12 `CLAUDE.md` files or `GLOSSARY.md` produced a
warning.

## Semantic — claims now false, with file, line, and the commit responsible

**Nothing found.** `git log --since="36 hours ago" --stat` returned no commits — the
most recent commit on `main` (`19f288d`, "feat(cli): start flags are ephemeral, and
every relaunch reuses them (#228)") landed 2026-09-19 19:34 -0700, roughly 55 hours
before this run. There is no change in the window to check memory claims against.

## Accretion — what could be removed and why

1. **`projects/kwaai-network/CLAUDE.md` — stale "not built yet" note for a shipped
   feature.** Lines 30–33 (Infrastructure) say:
   > Fix needed: p2p relay routing for inference requests (user intent: "use p2p relay")
   and lines 62–63 (Current state → In progress/planned) list:
   > P2P relay routing for inference (route through p2p network instead of direct TCP)

   This is already shipped. `p2p://PEER_ID` / `mux://PEER_ID` inference routing is
   implemented in `core/crates/kwaai-cli/src/capacity_lease.rs` and
   `core/crates/kwaai-cli/src/shard_cmd.rs` (e.g. `shard_cmd.rs` around the
   `mux://` proxy resolution and the `p2p://auto` peer-selection helper), and is
   documented as **Shipped** in two sibling files that this same repo trusts:
   `projects/kwaai-compute/CLAUDE.md` ("GPU inference machines (via p2p relay — no
   IP/DNS needed)") and `projects/kwaai-knowledge/CLAUDE.md` (working
   `--inference-urls "p2p://…"` examples in the D6 rebuild command). The same
   file's own "Do not" list even assumes it's shipped: "Do not send inference
   requests directly over TCP to metro machines; use p2p relay." This predates the
   repo's available history (earliest commit in this checkout, `acb791d`, already
   contains the capacity-lease code), so no specific invalidating commit can be
   named — it's just never been corrected since. Worth cutting the "Fix needed" /
   "In progress" lines so the file stops contradicting its own neighbors and its
   own do-not list.

   The adjacent "DNS broken: resolves to 192.168.1.1 (router)" workaround notes for
   `metro-linux`/`metro-win` (lines 31–32) look like a leftover from before p2p
   routing existed — if inference now goes over the p2p relay rather than direct
   TCP/DNS, a broken-DNS note for those hosts may no longer matter. Flagging rather
   than asserting: I did not find code that still depends on direct DNS resolution
   of these hostnames, but I also can't prove a negative here — worth a maintainer's
   five-second check before deleting.

2. **`core/crates/kwaai-trust/CLAUDE.md` — duplicate table row.** The "Key source
   files" table lists `kwaai-cli/src/identity.rs` twice back to back:
   > `kwaai-cli/src/identity.rs` | `kwaainet identity` command handler
   > `kwaai-cli/src/identity.rs` | `kwaainet identity` handler — DID, VC import/list/verify

   Same file, two rows, second one more descriptive. Looks like an edit added a
   better row without deleting the old one. Merge to one row.

3. **`projects/kwaai-compute/CLAUDE.md` — session-local plan path checked into a
   shared file.** Line 75:
   > (plan: `~/.claude/plans/cached-jingling-creek.md`)

   This is a path under one contributor's home directory; it does not exist in
   this checkout's environment (`~/.claude/plans/` isn't even present here). A
   home-relative, auto-named plan-tool path baked into a doc every agent reads is
   exactly the kind of reference that can never resolve for anyone but its author
   and will silently rot once even they can't find it. Either inline the plan's
   substance or drop the pointer.

## Nothing found

No CLAUDE.md or GLOSSARY.md reference to a CLI subcommand, flag, or file was found
broken beyond the three items above. Spot-checked against `cli.rs` and confirmed
still present: `rag connect-eve` (`ConnectEve`), `vpk discover` (`Discover`),
`shard chain` (`Chain`/`ShardChainArgs`), `rag graph seed`/`rag rebuild`/`rag
dream`/`rag eval`, and the `--no-relations` / `--relation-threshold-high` flags
referenced in `projects/kwaai-knowledge/CLAUDE.md`'s do-not list. The RoPE fix
pointer in `core/crates/kwaai-inference/CLAUDE.md` (`shard.rs:104`,
`broadcast_mul` not `*`) still matches the code at that location.
