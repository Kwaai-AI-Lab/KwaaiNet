# Memory reflection — 2026-11-17

Base commit: `e98ca3733da35f6e5dea1abb70fd45f735d2fd67` (origin/main, 2026-11-04)

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
```

Exit code 0. All 135 warnings are either `(proposed?)` references inside `plans/` documents (files/functions a plan proposes to create, correctly not yet existing) or line-number drift on symbols that still exist elsewhere in the same file — the checker treats both as warnings, not breaks, and that's the right call. No broken references.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` on `main` returns **no commits** — the last commit on this branch is `e98ca373` (2026-11-04), 13 days before this run. So step 2's commit-driven method has nothing to attribute this cycle: no memory file was invalidated by anything that landed in the last 36 hours.

While reading the twelve `CLAUDE.md` files and `GLOSSARY.md` directly against the current code (the part a commit-diff can't do, and the part the mechanical checker can't do either — these are well-formed claims that are just wrong), three pre-existing false claims surfaced. None are new; all predate the window, but they're the kind of thing this reflection exists to catch, so they're recorded here rather than dropped for not fitting the 36-hour frame:

1. **`core/crates/kwaai-storage/CLAUDE.md:3-4,16`** — describes the crate as "multi-tenant **homomorphic-encrypted** vector storage" and lists `src/vectors.rs` as "Homomorphic vector search." Actually false: `core/crates/kwaai-storage/src/lib.rs:4-6` states the crate "deliberately knows nothing about encryption: vectors are opaque `Vec<f32>`"; `vectors.rs` stores and searches plaintext `f32` vectors by cosine similarity, with no encryption code anywhere in the crate. The sibling `projects/kwaai-storage/CLAUDE.md` already has it right: "vectors are plaintext f32 today — encryption is planned." Both files were last touched by commit `b9a7343` (2026-09-18, "docs: drop the Petals bridge roadmap and its bootstrap example", #229), which corrected the project-level doc but left the crate-level one stale — a contradiction between two memory files that has stood for two months.

2. **`projects/kwaai-storage/CLAUDE.md:40,43`** — Build & test section runs `python3 tests/kwaai-storage/mock-vpk-health.py` and `bash tests/kwaai-storage/vpk-lan-test.sh`. Neither path exists; both scripts now live at `tests/kwaai-network/mock-vpk-health.py` and `tests/kwaai-network/vpk-lan-test.sh`. Same commit `b9a7343` moved the test directory without updating this doc. Note: `check_memory_integrity.py` does not catch this — it resolves backtick-wrapped references, not bare paths inside fenced ` ```bash ` blocks, so this is a real blind spot in the checker, not just a missed run.

3. **`projects/kwaai-knowledge/GLOSSARY.md:90`** — "35 in the global list, 14 of them kinship." The kinship count (14) is still correct; the total is not. `RELATION_TYPES` in `core/crates/kwaai-rag/src/graph.rs:40-91` now holds 44 entries (14 family + 7 agent + 3 spatial/biographical + 5 structural + 5 temporal + 5 semantic + 5 informational — counted directly). The glossary line was last written in `b9a7343` (2026-09-18); the relation list grew afterward, in #248 (2026-10-20, "every stage uses the KB's ontology, chosen per document at ingest") and/or #251 (2026-10-28, "graph build no longer silently loses chunks' extraction"), without the glossary count being updated either time.

## Accretion — what could be removed and why

1. **`projects/kwaai-trust/CLAUDE.md:56-57`** — duplicate table row, both pointing to `kwaai-cli/src/identity.rs` with near-identical descriptions (`kwaainet identity command handler` / `kwaainet identity handler — DID, VC import/list/verify`). Clerical duplication from commit `b9a7343`. Merge into one row.

2. **`projects/kwaai-network/CLAUDE.md:30-34`** ("Metro machines" block) — hardcodes operational state that is not checkable from this repo and will drift silently: a GPU swap dated "2026-08", a claimed broken-DNS fallback to `192.168.1.1`, and Ollama bind configuration. If this is still accurate it belongs in an ops runbook, not in a file every agent in this repo treats as ground truth; if it's stale, nothing here would tell you. Recommend moving it out of `CLAUDE.md` or marking it explicitly as "check before trusting."

3. **`projects/kwaai-network/CLAUDE.md:21`** — credits `node.rs` with defining `DHTServerInfo` and `VpkInfo`. Both are actually `pub struct` in `core/crates/kwaai-cli/src/announce.rs`; `node.rs` only imports them. `announce.rs` isn't named in any `CLAUDE.md`. Minor misattribution, worth a one-line fix next time this file is touched.

4. **`projects/kwaai-compute/CLAUDE.md:32-34`** — hardcoded libp2p peer IDs for `metro-linux`/`metro-win`/`jerome`. Infrastructure-specific and unverifiable from the repo; rots silently the moment a machine is replaced or re-keyed.

5. **`projects/kwaai-compute/CLAUDE.md:75`** — points to a KV-cache-pool plan at `~/.claude/plans/cached-jingling-creek.md`, a path outside the repo on one person's machine. Meaningless to any other session reading this file; either inline the plan or drop the reference.

6. **`projects/kwaai-knowledge/GLOSSARY.md:151`** — marks the per-KB ontology feature "New in this branch." It landed via #248 (2026-10-20), about four weeks ago, and is now the stable mechanism used throughout ingest/extraction/scoring, not a fresh addition. Cosmetic, low priority, but worth dropping on the next edit to that section.

7. **`projects/kwaai-compute/CLAUDE.md`** CLI entry-points table documents `ShardAction::{serve,run,status,chain}` but not the newer `api`/`download`/`gap`/`circuit` subcommands. Not false, just incomplete — a follow-up addition rather than a correction.

## Nothing found

The remaining nine `CLAUDE.md` files (`core/crates/kwaai-inference`, `core/crates/kwaai-p2p`, `core/crates/kwaai-trust`, `projects/kwaai-trust`, `projects/kwaai-network` outside the items above, `projects/kwaai-platform`, root `CLAUDE.md`'s build commands and project map, `projects/kwaai-knowledge/CLAUDE.md`'s source-file table, entity-cap logic, and CLI flags) were checked against the current code — file paths, function/struct names, CLI subcommands, default values, and "do not" guidance all still hold. Nothing false found there.
