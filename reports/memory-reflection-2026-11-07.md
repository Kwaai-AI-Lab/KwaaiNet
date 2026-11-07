# Memory reflection — 2026-11-07

Checked out at `e98ca37` (= `origin/main`).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1116
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
    warn    file reference does not resolve: `d6_doc_meta.yaml` (proposed?)
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_entity_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
    warn    `graph.rs:1912` — `coref_candidates_for_chunk` is defined at graph.rs:2122
  projects/kwaai-knowledge/plans/AutoDescriptions-plan.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/results/eval_log.md` (proposed?)
  projects/kwaai-knowledge/plans/D6-FullAB-results.md
    warn    file reference does not resolve: `D6.yaml` (proposed?)
    warn    file reference does not resolve: `ontologies/D6.yaml` (proposed?)
  projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md
    warn    `analysis/analyze.py` — no file at that path (moved? found tests/kwaai-knowledge/eval2/pilot/d6_noseed_analyze.py)
    warn    file reference does not resolve: `pilot/ontology_pilot.py` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
  projects/kwaai-knowledge/plans/D6-OntologyAB-testplan.md
    warn    file reference does not resolve: `ontologies/D6.yaml` (proposed?)
  projects/kwaai-knowledge/plans/D6-eval-report-20260609.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-AIAS2026-manuscript-plan.md
    warn    `../papers/aias2026/figures/gen_paper_figures.py` — no file at that path (moved? found projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py)
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `eval2_ctl_20260826_220033.md` (proposed?)
    warn    file reference does not resolve: `eval_D6_r26_dream_t0_20260615_232346.md` (proposed?)
    warn    file reference does not resolve: `eval_D6_r27_postdream_20260615_235134.md` (proposed?)
    warn    file reference does not resolve: `eval_D6_r37_dream6_20260617.md` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md
    warn    `analysis/analyze.py` — no file at that path (moved? found tests/kwaai-knowledge/eval2/pilot/d6_noseed_analyze.py)
    warn    `analysis/figures.py` — no file at that path (moved? found projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py)
    warn    file reference does not resolve: `export.py` (proposed?)
    warn    file reference does not resolve: `review_cli.py` (proposed?)
    warn    function reference does not resolve: `normalize_hash()` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-Eval3-plan.md
    warn    file reference does not resolve: `D6.yaml` (proposed?)
    warn    file reference does not resolve: `eval2/prereg_v3.md` (proposed?)
    warn    file reference does not resolve: `gold/freeze.py` (proposed?)
    warn    file reference does not resolve: `gold/review.py` (proposed?)
    warn    file reference does not resolve: `results/eval3/after_pilot.sh` (proposed?)
    warn    file reference does not resolve: `results/ontology_pilot/report.md` (proposed?)
    warn    file reference does not resolve: `work/gold/run_manhattan_v3_gold.sh` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-Intern-Curriculum.md
    warn    file reference does not resolve: `Methods.md` (proposed?)
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-Ontology-Eval-Compression.md
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_entity_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_entity_type_schemas.yaml` (proposed?)
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
  projects/kwaai-knowledge/plans/OntologySelection-plan.md
    warn    file reference does not resolve: `D6.yaml` (proposed?)
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `results/eval3/after_pilot.sh` (proposed?)
    warn    file reference does not resolve: `results/ontology_pilot_v2/report.md` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/results/ontology_pilot/report.md` (proposed?)
  projects/kwaai-knowledge/plans/OntologySession-assessment.md
    warn    file reference does not resolve: `D6.yaml` (proposed?)
    warn    file reference does not resolve: `ontologies/D6.yaml` (proposed?)
  projects/kwaai-knowledge/plans/PerKBOntology-plan.md
    warn    file reference does not resolve: `d6_entity_type_schemas.yaml` (proposed?)
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_entity_schema.yaml` (proposed?)
    warn    `graph.rs:1215` — `upsert_relation` is defined at graph.rs:1425
    warn    `graph.rs:365` — `KBEntityTypeSchema` is defined at graph.rs:432
    warn    `graph.rs:5563` — `extract_from_text` is defined at graph.rs:6063
    warn    `graph.rs:600` — `expected_fields` is defined at graph.rs:718
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/results/eval_log.md` (proposed?)
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
    warn    `rag_cmd.rs:7372` — `extract_rc_windows` is defined at rag_cmd.rs:7666
    warn    `rag_cmd.rs:7598` — `call_llm_for_relations` is defined at rag_cmd.rs:7892
    warn    `sequence.rs:777` — `extract_kinship_interactions` is defined at sequence.rs:796
  projects/kwaai-knowledge/plans/Phase5-TrainedClassifiers-plan.md
    warn    file reference does not resolve: `core/crates/kwaai-rag/src/entity_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `core/crates/kwaai-rag/src/relation_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `gap_analysis.py` (proposed?)
    warn    file reference does not resolve: `gap_analysis2.py` (proposed?)
    warn    file reference does not resolve: `relation_type_classifier.rs` (proposed?)
    warn    file reference does not resolve: `scripts/classifier_train_common.py` (proposed?)
    warn    file reference does not resolve: `scripts/export_entity_training_data.py` (proposed?)
    warn    file reference does not resolve: `scripts/train_entity_classifier.py` (proposed?)
    warn    file reference does not resolve: `scripts/train_relation_classifier.py` (proposed?)
  projects/kwaai-knowledge/plans/Phase6-PersistentMentionIndex-plan.md
    warn    `ingestion.rs:1633` — `extract_entity_centric` is defined at ingestion.rs:1888
    warn    `ingestion.rs:271` — `extract_and_store_entities_pub` is defined at ingestion.rs:371
  projects/kwaai-knowledge/plans/PredicateAsVector-research.md
    warn    file reference does not resolve: `DreamRAG-Forgetting-plan.md` (proposed?)
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
  projects/kwaai-knowledge/plans/confidence-hybrid-extraction.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_experiments_log.md` (proposed?)
  projects/kwaai-knowledge/plans/coref-pronoun-resolution.md
    warn    `src/graph.rs:1461` — `GraphStore::link_chunk` is defined at src/graph.rs:1671
    warn    `src/graph.rs:4832` — `GraphStore::all_chunk_entity_pairs` is defined at src/graph.rs:5157
    warn    `src/ingestion.rs:1890` — `refine_low_confidence_entities` is defined at src/ingestion.rs:2150
    warn    `src/meta_store.rs:177` — `MetaStore::all_chunks` is defined at src/meta_store.rs:193
    warn    `src/ner.rs:267` — `ner::resolve_pronouns` is defined at src/ner.rs:344
    warn    `src/rag_cmd.rs:6508` — `FAMILY_TRIGGERS` is defined at src/rag_cmd.rs:6802
  projects/kwaai-knowledge/plans/d6-person-entity-experiments.md
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_experiments_log.md` (proposed?)
  projects/kwaai-knowledge/plans/d6-rag-accuracy-improvement.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_relation_hard_cases.md` (proposed?)
    warn    function reference does not resolve: `is_family_query()` (proposed?)
    warn    `retriever.rs:456` — `resolve_author_relative` is defined at retriever.rs:591
  projects/kwaai-knowledge/plans/entity-type-schema-validation.md
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_entity_schema.yaml` (proposed?)
  projects/kwaai-knowledge/plans/hierarchical-summarization.md
    warn    function reference does not resolve: `search_summaries()` (proposed?)
    warn    `dream_tasks.rs:236` — `call_llm` is defined at dream_tasks.rs:258
  projects/kwaai-knowledge/plans/structure-aware-ingestion.md
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `doc_schema.yaml` (proposed?)
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
  projects/kwaai-trust/plans/DTGWG-TrustStack-plan.md
    warn    file reference does not resolve: `TokenEconomy-plan.md` (proposed?)
    warn    file reference does not resolve: `bindings/libp2p/0.1/spec.md` (proposed?)
    warn    `dtgwg-trust-tasks-tf/trust-tasks-rs/src/transport.rs` — no file at that path (moved? found core/crates/kwaai-p2p/src/transport.rs)
    warn    file reference does not resolve: `release-plz.toml` (proposed?)
    warn    `trust-tasks-rs/src/transport.rs` — no file at that path (moved? found core/crates/kwaai-p2p/src/transport.rs)
  projects/kwaai-trust/plans/TrustTasksLibp2p-implementation-plan.md
    warn    file reference does not resolve: `bindings/libp2p/0.1/spec.md` (proposed?)
    warn    file reference does not resolve: `bindings/tsp/0.1/spec.md` (proposed?)
    warn    file reference does not resolve: `consume.rs` (proposed?)
    warn    file reference does not resolve: `deny.toml` (proposed?)
    warn    file reference does not resolve: `freshness.rs` (proposed?)
    warn    file reference does not resolve: `kwaai-cli/src/trust_tasks.rs` (proposed?)
    warn    file reference does not resolve: `replay.rs` (proposed?)
    warn    `trust-tasks-rs/src/transport.rs` — no file at that path (moved? found core/crates/kwaai-p2p/src/transport.rs)
    warn    file reference does not resolve: `trust_tasks.rs` (proposed?)

0 broken reference(s), 135 warning(s) across 57 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

Exit code 0. All 135 warnings are in `projects/*/plans/*.md` files (none in the twelve
`CLAUDE.md` files or `GLOSSARY.md`), and all are either forward references to files a plan
proposes creating (the checker marks these `(proposed?)`) or line-number drift on functions
that still exist at a different line. No broken references.

One warning stood out as actionable rather than cosmetic and was followed up on in
Accretion below: `RAGPerformanceReport-20260712.md` — status says "not yet committed" but
`core/crates/kwaai-rag/src` has changed since.

## Semantic — claims now false, with file, line, and the commit responsible

No commits landed in the last 36 hours: `git log --since="36 hours ago"` is empty. The
latest commit on `main` is `e98ca37` (2026-11-04T18:28:04-08:00, #274), over 60 hours old.
With no new commits there is nothing a commit could have made untrue since the last
window, so this section has nothing to report this run.

As a spot check (not triggered by a recent commit, but relevant to this section's intent),
I verified the specific "do not re-enable X where X has since been made safe" pattern the
task description calls out as the known failure mode: `projects/kwaai-knowledge/CLAUDE.md`
§"Do not" still says not to enable *unfiltered* relation extraction for 8B models, but
explicitly names the Phase-4 axiomatic pipeline (lexical trigger pre-filter + confidence
split) as the sanctioned alternative — i.e. this file already accounts for the machinery
that would otherwise make the warning stale. No action needed.

## Accretion — what could be removed and why

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — "**Status**:
  code complete, not yet committed" (referring to the redb→rusqlite WAL migration). This is
  now stale: `core/crates/kwaai-rag/Cargo.toml` depends on `rusqlite` with no `redb`
  reference anywhere in `core/`, and `cache.rs`, `feedback.rs`, and `graph.rs` all set
  `PRAGMA journal_mode=WAL`, matching the plan's described end-state. The crate is at
  `v0.7.3`, well past this report's 2026-07-12 date. The line should be updated to say the
  migration landed (and ideally name the commit), or the status paragraph removed since the
  migration is now just how the code works, not a pending change.

- **Hardcoded peer IDs** in `projects/kwaai-compute/CLAUDE.md` and
  `projects/kwaai-knowledge/CLAUDE.md` (`metro-linux`, `metro-win`, `jerome` — three
  `12D3Koo...` peer IDs, duplicated verbatim across both files). I can't verify from the
  repo alone whether these machines are still live or reachable — that's infrastructure
  state, not something git tracks. Flagging per the task's named example pattern, but this
  needs a human (or a p2p relay check) to confirm before touching it, not a repo-only
  read. One file not corroborating this is `RAGPerformanceReport-20260712.md:106-114`,
  which (as of July) already recorded `jerome` as "❌ Offline... excluded from pipeline" —
  so the two `CLAUDE.md` files listing `jerome` as a plain infrastructure entry, with no
  offline annotation, may already be the stale ones. Worth a maintainer check rather than
  a unilateral edit.

- No plan documents found that are clearly superseded by a later plan on the same topic,
  and no sections describing long-finished work that should be trimmed, beyond the one
  above.

## Nothing found

No broken references (mechanical). No commits in the window, so no semantic drift to
report from new code.
