# Memory reflection — 2026-09-25

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

0 broken references. All 45 warnings are in `plans/` documents, not the 12 authoritative `CLAUDE.md` files or `GLOSSARY.md`; the great majority are explicitly-marked `(proposed?)` future-file references in unimplemented plans, which is expected. Four are worth a closer look but are low stakes since plan docs aren't loaded as ambient context the way `CLAUDE.md` is:
- `MacOllamaStopgap-plan.md` cites `config.rs:1059` for `announce_state`, actually at `config.rs:1082` — 23-line drift, harmless.
- `AutoDeriveSeededFacts-plan.md` and `MemoryIntegrity-growth-cycle.md` cite files that were since renamed/moved (`schema.rs` → `doc_schema.rs`; `src/mcp/server.rs` → `core/crates/kwaai-cli/src/grpc_server.rs`).
- `VPK-CrateIntegration-plan.md` cites `src/vpk/metrics.rs`, now at `core/crates/kwaai-network-tests/src/metrics.rs`.

## Semantic — claims now false, with file, line, and the commit responsible

No commits landed in the last 36 hours (latest commit `5bb2ce0`, 2026-09-22 19:24 UTC; now 2026-09-25 10:06 UTC — a ~63 hour gap), so there is nothing in this window to check a memory file's claims against. Nothing found.

## Accretion — what could be removed and why

1. **`plans/d6-rag-accuracy-improvement.md` (repo root) is a stale duplicate, superseded by `projects/kwaai-knowledge/plans/d6-rag-accuracy-improvement.md`.** The root copy still contains the leftover instruction *"Rename this file to `d6-rag-accuracy-improvement.md` after exiting plan mode"* and describes a 52.6% baseline recall with zero relations in the graph. The `projects/kwaai-knowledge/` copy (last touched 2026-09-14, twelve days after the root copy's last touch on 2026-09-02) has a progress tracker showing recall at 63.1% and several phases already implemented. The root file was never removed after its content was superseded. Recommend deleting `plans/d6-rag-accuracy-improvement.md` and keeping only the `projects/kwaai-knowledge/` version.

2. **`plans/windows-self-update-redesign.md` describes finished work as still pending.** It's marked `**Status:** Ready to implement`, targeting branch `fix/updater-ps1-move-retry` / PR #65, and its "Problem" section describes the Windows updater generating a detached PowerShell script (`CREATE_NO_WINDOW`, silent `Move-Item` retries, self-deleting script). I checked `core/crates/kwaai-cli/src/updater.rs` as it exists today: `install_update()` on Windows now does an in-process, synchronous `std::fs::rename` + `std::fs::copy` binary swap with no PS1 generation at all; the only remaining `CREATE_NO_WINDOW` use is in `nvidia_smi_windows()`, unrelated to installation. `git log --diff-filter=A` shows `updater.rs` was added in its current synchronous form in commit `05d6925` (2026-09-02) — the same date the plan file was last touched — so the redesign this plan proposes is already shipped. The plan is a completed item still sitting under `plans/` as "ready to implement." Recommend either deleting it or moving it to a "done" location with a note of which commit closed it.

**Flagged but not confirmed stale (would need information I don't have access to here):**

- `projects/kwaai-compute/CLAUDE.md:32-36` and `projects/kwaai-knowledge/CLAUDE.md:98-106` hardcode three lab peer IDs (`metro-linux`, `metro-win`, `jerome`). These are exactly the kind of accretion risk called out for this pass, but I have no way from this checkout to confirm whether those machines are still live p2p nodes or have been decommissioned/renamed. Worth a human check against current fleet inventory next time one of those machines changes.
- `projects/kwaai-knowledge/GLOSSARY.md:19` claims "Sixteen exist" knowledge bases under `rag_kbs` in `~/.kwaainet/config.yaml`. That config lives on a developer machine, not in this repo, so I can't verify the current count from here. If it drifts, it's a one-line fix.

## Nothing found

Step 2 (semantic drift from recent commits): nothing found — no commits in the last 36 hours to check.
