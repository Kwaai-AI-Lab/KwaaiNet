# Memory Reflection — 2026-10-24

Checked out: `main` @ `c6da5da09f5e1741cf3b588929c65ec113b388cc`. Window for the semantic
pass: commits since 2026-10-22 ~22:00 UTC (`git log --since="36 hours ago"`), which contains
exactly one commit.

## Mechanical — checker output, verbatim

```
Checking 53 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
    warn    file reference does not resolve: `d6_doc_meta.yaml` (proposed?)
    warn    file reference does not resolve: `d6_doc_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_entity_schema.yaml` (proposed?)
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_family_tree.yaml` (proposed?)
    warn    `graph.rs:1912` — `coref_candidates_for_chunk` is defined at graph.rs:2004
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
    warn    `graph.rs:1215` — `upsert_relation` is defined at graph.rs:1307
    warn    `graph.rs:5563` — `extract_from_text` is defined at graph.rs:5945
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/results/eval_log.md` (proposed?)
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
    warn    `rag_cmd.rs:7372` — `extract_rc_windows` is defined at rag_cmd.rs:7538
    warn    `rag_cmd.rs:7598` — `call_llm_for_relations` is defined at rag_cmd.rs:7764
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
    warn    `ingestion.rs:1633` — `extract_entity_centric` is defined at ingestion.rs:1796
    warn    `ingestion.rs:271` — `extract_and_store_entities_pub` is defined at ingestion.rs:278
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
  projects/kwaai-knowledge/plans/confidence-hybrid-extraction.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_experiments_log.md` (proposed?)
  projects/kwaai-knowledge/plans/coref-pronoun-resolution.md
    warn    `src/graph.rs:1461` — `GraphStore::link_chunk` is defined at src/graph.rs:1553
    warn    `src/graph.rs:4832` — `GraphStore::all_chunk_entity_pairs` is defined at src/graph.rs:5039
    warn    `src/ingestion.rs:1890` — `refine_low_confidence_entities` is defined at src/ingestion.rs:2058
    warn    `src/meta_store.rs:177` — `MetaStore::all_chunks` is defined at src/meta_store.rs:193
    warn    `src/ner.rs:267` — `ner::resolve_pronouns` is defined at src/ner.rs:344
    warn    `src/rag_cmd.rs:6508` — `FAMILY_TRIGGERS` is defined at src/rag_cmd.rs:6674
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

0 broken reference(s), 117 warning(s) across 53 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 117 warnings are in `plans/` documents, not in the 12 CLAUDE.md files or
GLOSSARY.md. Two shapes worth separating:

- Most are `(proposed?)` — a plan naming a file it intends to create. Expected for forward-looking
  plan docs; not drift.
- A smaller set are `(moved? found ...)` — a plan naming a file that used to exist and has since
  moved. These are real, actionable drift; see **Accretion** below for the one I followed up on.

## Semantic — claims now false, with file, line, and the commit responsible

Only one commit landed in the window: `c6da5da` — "fix(ci): patch the Windows installer for CUDA,
and fail when a patch misses" (#247). It changed only `.github/workflows/release.yml` (plus
`CHANGELOG.md` and the two patch scripts, neither of which is a memory file).

No memory-file claim was made false by this commit. One claim is now **incomplete** rather than
false, and it's close enough to the bug this PR fixed that it's worth flagging:

- `projects/kwaai-platform/CLAUDE.md:47` — "Shell installer patched for NVIDIA detection in
  `build-global-artifacts`." Still true, but `c6da5da` changed `build-global-artifacts` to also
  patch `kwaainet-installer.ps1` (previously unpatched — the exact bug #247 fixes) and to `exit 1`
  if either installer is missing (previously just a warning). The line doesn't say anything
  false, but a reader would come away thinking only the shell installer matters here, which is the
  same blind spot that let the Windows installer ship unpatched for months. Worth a one-line
  update, not urgent.

No other memory file references `release.yml`, the patch scripts, `nvidia-smi`, `cublas`, or
`kwaainet-installer` (checked across all 12 CLAUDE.md files and GLOSSARY.md), so nothing else was
in reach of this commit.

## Accretion — what could be removed and why

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — "Status: code
  complete, not yet committed" for the redb→rusqlite WAL migration. The checker already flags this
  line and asks to recheck (see Mechanical output above); I did. It has landed: `Cargo.toml` for
  `kwaai-rag` depends on `rusqlite` (no `redb` dependency remains), and `meta_store.rs:79-81`
  contains a legacy-migration path that detects old `*.redb` files and tells the user to run
  `kwaainet rag rebuild`. The status line is stale and should be updated to "landed" or the report
  marked historical.

- **Duplicated infrastructure table** — the GPU peer-ID table (`metro-linux`, `metro-win`,
  `jerome`, with full peer IDs) is maintained identically in two places:
  `projects/kwaai-compute/CLAUDE.md:32-36` and `projects/kwaai-knowledge/CLAUDE.md:101-111`. Not
  wrong today, but two independent copies of hardware identity info will drift the next time a
  machine changes — one was already missed if either goes stale. Consider having one own it and
  the other reference it.

- **Malformed table, `projects/kwaai-knowledge/CLAUDE.md:101`** — the peer-ID table header declares
  4 columns (`Machine | Scheme | Peer ID | GPU`) but every data row (102-105) has only 3 cells.
  Renders wrong; worth a column fix next time that section is touched. Pre-existing, not caused by
  anything in the 36-hour window.

- The `(moved?)` warnings in the Mechanical output point at genuinely moved files inside `plans/`
  docs (e.g. `AutoDeriveSeededFacts-plan.md` → `schema.rs` is now `doc_schema.rs`;
  `D6-NoSeed-Dream-plan.md` and `DreamRAG-Eval2-plan.md` → `analysis/analyze.py` is now
  `tests/kwaai-knowledge/eval2/pilot/d6_noseed_analyze.py`). These are historical plan records, low
  stakes, and the checker already names the replacement — flagging here rather than editing, since
  the task is report-only.

Nothing found to remove in the 12 CLAUDE.md files or GLOSSARY.md beyond the two items above — no
dead "do not re-enable X" guidance, no absolute paths, no sections describing work that's finished
and could be dropped.
