# Memory reflection — 2026-10-21

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exited 0 (no broken references).

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
```

0 broken references — nothing mechanically dead. The 117 warnings are almost entirely two shapes that are not errors: (1) function/line references a few lines off their target (normal churn, checker already treats these as warnings not failures), and (2) file paths in `plans/*.md` marked `(proposed?)` — these are forward-looking design docs describing files that don't exist *yet*, which is correct for a plan. One warning is a genuine standing flag worth repeating here because it names exactly the kind of drift this reflection exists to catch:

> `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md` — status says "not yet committed" but `core/crates/kwaai-rag/src` has changed since — recheck whether it has landed.

I did not chase this down — it's outside the 36-hour window this run covers and the checker already surfaces it on every run. Flagging it for a future reflection (or for whoever owns that plan doc) rather than re-investigating it here.

## Semantic — claims now false, with file, line, and the commit responsible

Only one commit landed in the last 36 hours: `662caa6` "feat(kwaai-rag): every stage uses the KB's ontology, chosen per document at ingest (#248)", 2026-10-20 16:18 -0700. One finding:

**`projects/kwaai-knowledge/plans/OntologySelection-plan.md`, line 3** (added by this same commit):
> Status: **design agreed** (2026-09-27); §7 decisions 1–3 settled, 4 open. **No code written.**

This is false as of the commit that introduced the file. The same commit also adds `core/crates/kwaai-rag/src/ontology_select.rs` (928 lines — `Library::load_dir`, `coverage`, `select`, `detect_doc_type`, `DocOntologies`, etc.), the `kwaainet rag graph schema select` subcommand (`cli.rs`, `SchemaAction::Select`), and `--ontology-library`/`--own-corpus` on `rag ingest`. The commit message confirms this shipped as a real feature ("Per-document ontology selection at `rag ingest`", reviewed across three rounds with fixes and regression tests, plus a list of known-remaining issues). The plan's own status line was not updated to match the code it shipped alongside. Whoever owns this doc should change the status line to reflect that phase (design → implemented, with the "found and not fixed" list from the `#248` commit message carried over as the new open-items list) rather than leave it reading as unstarted.

Everything else this commit touched checked out:
- `projects/kwaai-knowledge/CLAUDE.md`'s "Full pipeline test" line was itself edited by this commit to redact the real D6 file path to `"<D6 manuscript, private copy>"`, consistent with the commit's "Prepared for the public tree: No D6 material is added" note and the file's own "D6 is a private corpus and must never be committed to this public repo" rule. Not stale — this is the fix.
- `core/crates/kwaai-rag/CLAUDE.md`'s "Do not enable *unfiltered* relation extraction for 8B models" guidance already describes the Phase-4 lexical-trigger/confidence-split machinery as the answer; this commit's "ontology's trigger phrases widen the compiled relation gate and never replace it" fix is consistent with, not contradicted by, that section.
- `--reset-graph`'s help text changed (now also keeps ontology/entity-schemas/doc-metadata, not just chunks), but no CLAUDE.md or GLOSSARY.md documents its old behavior — only a usage example invokes the flag, without describing what it preserves. Nothing to fix there.

## Accretion — what could be removed and why

- `projects/kwaai-knowledge/plans/PerKBOntology-plan.md` (last touched 2026-09-10) names "Arm D — selected" as its only unbuilt piece, deferred until "B has produced two modules worth selecting between." `OntologySelection-plan.md` (added this commit) says explicitly it "completes `PerKBOntology-plan.md`." With #248 landed, PerKBOntology-plan.md's open item may now be resolved — worth a status-line update once its owner confirms Arm D's precondition was actually met, rather than leaving two plan docs that each gesture at the other's completion status without either being updated. I'm not certain enough to call this stale outright (I didn't verify the "two modules" precondition against the shipped ontology library), so flagging rather than asserting.
- Four other ontology plan docs (`D6-OntologyAB-testplan.md`, `DreamRAG-Ontology-Eval-Compression.md`, `OntologySession-assessment.md`, plus `PerKBOntology-plan.md` above) are all last-touched 2026-09-10, a month before per-document ontology selection shipped. None of these were checked line-by-line this run (out of scope for a 36-hour window), but given how much #248 changed underneath them, they're due for a reconciliation pass the next time someone works in this area.
- `core/crates/kwaai-rag/CLAUDE.md`'s "Key source files" table doesn't list the new `ontology_select.rs` (928 lines, added this commit). The table doesn't claim to be exhaustive ("these are the ones worth reading first"), so this isn't a false claim — but a 928-line new module doing ontology selection is plausibly exactly the kind of file that table exists to surface. Worth adding next time that file is touched.

## Nothing found

The mechanical checker found 0 broken references (unchanged from its normal baseline of proposed-file and off-by-a-few-lines warnings). No renamed/deleted CLI subcommands, inverted behaviors, or now-safe "do not" rules turned up in the single commit from the last 36 hours beyond the one status-line finding above.
