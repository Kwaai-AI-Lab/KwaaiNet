# Memory reflection — 2026-09-21

Checked out at `19f288d405d07c4f4b16e87aaf3383a7558ad4dc` (origin/main tip).

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

Exit code 0. All 45 warnings are in `plans/` documents and are either explicitly
proposed-but-unbuilt paths or moved-file guesses the checker already resolves;
none are broken references in a `CLAUDE.md` or `GLOSSARY.md`.

## Semantic — claims now false, with file, line, and the commit responsible

One commit landed in the last 36 hours: `19f288d` "feat(cli): start flags are
ephemeral, and every relaunch reuses them (#228)". It reworks how `start`,
`run-node`, `restart`, `reconnect`, `update` and the auto-update respawn handle
node flags (`StartOverrides`, `run/start-args.json`, new `grpc_port` and
`shard.backend` config keys), and touches `cli.rs`, `config.rs`, `daemon.rs`,
`grpc_server.rs`, `main.rs`, `node.rs`, `node_native.rs`, `shard_cmd.rs`.

No subcommand was renamed or deleted — `start`, `run-node`, `restart`,
`reconnect`, `update`, `stop`, `config set` all still exist under the same
names. Checked every `CLAUDE.md`/`GLOSSARY.md` for the terms this commit
touches (`contribute.shards`, `contribute.storage`, `shard.backend`,
`grpc_port`, `--shard`, `--no-contribute`, `--force-blocks`, `StartOverrides`,
`config set` key lists) — none of the twelve `CLAUDE.md` files or
`GLOSSARY.md` assert anything about this behavior. The commit's own diff
already updated `README.md` (the `--shard`/`--no-contribute` persistence note
and the `shard.backend` note) in the same commit, so nothing there went stale
either.

**Nothing found.**

## Accretion — what could be removed and why

- `projects/kwaai-compute/CLAUDE.md` → "In progress / planned" section, the
  line `Dedicated inference thread with session pool, LRU eviction (plan:
  ~/.claude/plans/cached-jingling-creek.md)`. That path is outside the repo
  (a session-local Claude Code plan file, not a tracked document), does not
  exist on this checkout, and has no history in `git log --all`. It cannot be
  read by another agent or session, and the mechanical checker cannot see it
  either since it's outside the repo root. Either inline the actual plan
  content into the `CLAUDE.md` (or a tracked `projects/kwaai-compute/plans/`
  doc) or drop the parenthetical.

No other clearly-outlived content found in this pass. The hardcoded peer IDs
for `metro-linux`/`metro-win`/`jerome` in `kwaai-compute/CLAUDE.md` and
`kwaai-knowledge/CLAUDE.md` are duplicated across two files but I have no way
to confirm from this checkout whether they're still live — flagging as
unverified rather than as a finding.

## Nothing found

Semantic section is clean: the one commit in the last 36 hours kept its own
documentation (`README.md`) in sync, and no memory file makes claims about the
behavior it changed.
