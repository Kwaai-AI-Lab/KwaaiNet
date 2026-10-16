# Memory Reflection — 2026-10-16

Checked out at `main` @ `913e0bc` (matches `origin/main`).

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
    warn    file reference does not resolve: `d6_doc_meta.yaml` (proposed?)
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_entity_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
    warn    `graph.rs:1912` — `coref_candidates_for_chunk` is defined at graph.rs:1998
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
  projects/kwaai-knowledge/plans/OntologySession-assessment.md
    warn    file reference does not resolve: `D6.yaml` (proposed?)
    warn    file reference does not resolve: `ontologies/D6.yaml` (proposed?)
  projects/kwaai-knowledge/plans/PerKBOntology-plan.md
    warn    file reference does not resolve: `d6_entity_type_schemas.yaml` (proposed?)
    warn    file reference does not resolve: `ontology.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_entity_schema.yaml` (proposed?)
    warn    `graph.rs:1215` — `upsert_relation` is defined at graph.rs:1301
    warn    `graph.rs:5563` — `extract_from_text` is defined at graph.rs:5661
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/results/eval_log.md` (proposed?)
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
    warn    `rag_cmd.rs:7372` — `extract_rc_windows` is defined at rag_cmd.rs:7385
    warn    `rag_cmd.rs:7598` — `call_llm_for_relations` is defined at rag_cmd.rs:7611
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
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
  projects/kwaai-knowledge/plans/confidence-hybrid-extraction.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_experiments_log.md` (proposed?)
  projects/kwaai-knowledge/plans/coref-pronoun-resolution.md
    warn    `src/graph.rs:1461` — `GraphStore::link_chunk` is defined at src/graph.rs:1547
    warn    `src/graph.rs:4832` — `GraphStore::all_chunk_entity_pairs` is defined at src/graph.rs:4929
    warn    `src/meta_store.rs:177` — `MetaStore::all_chunks` is defined at src/meta_store.rs:193
    warn    `src/rag_cmd.rs:6508` — `FAMILY_TRIGGERS` is defined at src/rag_cmd.rs:6521
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

0 broken reference(s), 101 warning(s) across 51 files
```

0 broken references. All 101 warnings are in `projects/*/plans/*.md` — forward-looking plan docs, not the 12 CLAUDE.md files or GLOSSARY.md the checker was built to protect against regressions in. Most are "(proposed?)" references to files a plan describes building, not claims about current state; a few are line-number drift in docs/plans pointing at functions that have since shifted a few lines (expected churn, not semantic breakage). Not re-litigated individually since they're pre-existing and not something to act on under "report only."

## Semantic — claims now false, with file, line, and the commit responsible

No commits landed in the last 36 hours. Latest commit on `main` is `913e0bc` (2026-10-14 09:04:39 -0700, "fix(updater): don't self-update a musl or non-x86_64 host to the gnu-cuda build (#236)"), roughly 49 hours before this run — outside the window. Nothing to check against.

## Accretion — what could be removed and why

Nothing found tonight. Spot-checked the 12 CLAUDE.md files and GLOSSARY.md for self-flagged staleness markers (TODO/FIXME/deprecated/superseded/outdated language) — only generic "roadmap, TODO" directory pointers turned up, not stale claims. Did not run a full line-by-line pass of all twelve files tonight since nothing changed upstream to prompt one; if a deeper accretion sweep is wanted independent of recent commits, that's a separate, larger task (the kind the August audit did) rather than part of this nightly diff-driven check.

## Nothing found

Checker: clean (0 broken references; the 101 warnings are known plan-doc drift, not CLAUDE.md/GLOSSARY.md regressions). Semantic: no commits in the 36-hour window to check. Accretion: no self-evidently stale claims surfaced in a spot check.
