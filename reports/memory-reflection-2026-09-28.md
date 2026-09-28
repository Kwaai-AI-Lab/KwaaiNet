# Memory reflection — 2026-09-28

Reviewed at `486bcf6a17e31481731f8b084ab232b927c573c5` (origin/main HEAD at run time).

## Mechanical — checker output, verbatim

```
Checking 49 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
  projects/kwaai-knowledge/plans/DreamRAG-AIAS2026-manuscript-plan.md
    warn    `../papers/aias2026/figures/gen_paper_figures.py` — no file at that path (moved? found projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py)
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

0 broken reference(s), 46 warning(s) across 49 files
```

Exit code 0. All 46 warnings are in `plans/` documents, and the checker's own `(proposed?)` tag is correct for the large majority: those name files/functions the plan proposes to create, not ones that once existed. The three `(moved? found ...)` warnings (`AutoDeriveSeededFacts-plan.md`, `MemoryIntegrity-growth-cycle.md`, `VPK-CrateIntegration-plan.md`) are genuine renames worth a look next pass, but none are in the twelve `CLAUDE.md` files or `GLOSSARY.md`, so nothing authoritative is broken.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned no commits. The last commit on `main` is `486bcf6` at 2026-09-25 16:08:39 -0700 (~2026-09-25 23:08 UTC), roughly 59 hours before this run — outside the window. Nothing to check: no commit could have invalidated a memory-file claim, so nothing was invalidated.

I did spot-check the one class of claim the task background calls out by name — a "do not do X" rule that new machinery might have outdated — against `projects/kwaai-knowledge/CLAUDE.md:145`: "Do not enable *unfiltered* relation extraction for 8B models." This is already correctly qualified: it names the Phase-4 pipeline (`--relation-threshold-high`) as the sanctioned alternative, and I confirmed `relation_threshold_high`/`relation_threshold_low` are live CLI flags (`core/crates/kwaai-cli/src/cli.rs:1534,1539,2021,2029`) wired into `extract_relations_axiomatic()` (`core/crates/kwaai-cli/src/rag_cmd.rs:9977`). Not a finding — noted only because it's the same shape of claim the task asked me to check for.

## Accretion — what could be removed and why

**`projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md` is a proposal for work that has already shipped, and should be marked done or archived.** The plan (last touched 2026-09-14) is written entirely in the future tense — "Create: `core/crates/kwaai-rag/src/relation_extract.rs`", "two new flags on `Rebuild` and `GraphAction::Build`", "New orchestration function `extract_relations_axiomatic()`". All three already exist in the checked-out tree:
- `core/crates/kwaai-rag/src/relation_extract.rs` exists.
- `relation_threshold_high` / `relation_threshold_low` are defined as CLI flags at `core/crates/kwaai-cli/src/cli.rs:1534,1539` (on `Rebuild`) and `:2021,2029` (on `GraphAction::Build`), and threaded through `rag_cmd.rs`.
- `extract_relations_axiomatic()` is defined at `core/crates/kwaai-cli/src/rag_cmd.rs:9977` and called from both construction sites the plan specifies.
- `projects/kwaai-knowledge/plans/Phase6-PersistentMentionIndex-plan.md:5` independently corroborates this, opening with "Validating Phase 4's relation extraction against real D6 candidates today surfaced..." — i.e. a later plan already treats Phase 4 as shipped and in active use.

None of this makes the plan's *content* wrong (it's a design record, and an accurate one), but leaving it phrased as an open proposal in an active `plans/` directory is exactly the "section describing work long finished" the task asked me to flag. I did not touch it — recommend a one-line status header (e.g. "Status: shipped, see `relation_extract.rs`") or a move to a `plans/done/` (or equivalent) location, whichever this repo's convention prefers; I don't know of an existing convention for that and would want it confirmed before acting.

One thing I noticed but am not confident is stale, so I'm not calling it a finding: the p2p peer-ID table (`metro-linux`/`metro-win`/`jerome`) is duplicated verbatim in both `projects/kwaai-compute/CLAUDE.md:32-36` and `projects/kwaai-knowledge/CLAUDE.md:96-106`. The two copies still agree with each other, so there's no internal contradiction, but I have no way from the repo alone to confirm those peer IDs are still the live addresses for those machines — that needs someone with access to the actual p2p network to check.

## Nothing found

No commits landed in the last 36 hours, so the Semantic section has nothing new to report beyond the spot-check above.
