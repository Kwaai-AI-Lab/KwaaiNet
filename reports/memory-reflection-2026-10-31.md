# Memory Reflection — 2026-10-31

Checked at HEAD `ee626f3f0c4e41822e5877ce2b8cfd5ffd8fb32c` (branch `main`).

## Mechanical — checker output, verbatim

```
Checking 56 memory and plan files

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
    warn    `rag_cmd.rs:7372` — `extract_rc_windows` is defined at rag_cmd.rs:7646
    warn    `rag_cmd.rs:7598` — `call_llm_for_relations` is defined at rag_cmd.rs:7872
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
    warn    `ingestion.rs:1633` — `extract_entity_centric` is defined at ingestion.rs:1860
    warn    `ingestion.rs:271` — `extract_and_store_entities_pub` is defined at ingestion.rs:356
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
  projects/kwaai-knowledge/plans/confidence-hybrid-extraction.md
    warn    file reference does not resolve: `d6_family_tree.yaml` (proposed?)
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_experiments_log.md` (proposed?)
  projects/kwaai-knowledge/plans/coref-pronoun-resolution.md
    warn    `src/graph.rs:1461` — `GraphStore::link_chunk` is defined at src/graph.rs:1671
    warn    `src/graph.rs:4832` — `GraphStore::all_chunk_entity_pairs` is defined at src/graph.rs:5157
    warn    `src/ingestion.rs:1890` — `refine_low_confidence_entities` is defined at src/ingestion.rs:2122
    warn    `src/meta_store.rs:177` — `MetaStore::all_chunks` is defined at src/meta_store.rs:193
    warn    `src/ner.rs:267` — `ner::resolve_pronouns` is defined at src/ner.rs:344
    warn    `src/rag_cmd.rs:6508` — `FAMILY_TRIGGERS` is defined at src/rag_cmd.rs:6782
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

0 broken reference(s), 134 warning(s) across 56 files
```

0 broken references. All 134 warnings are in `projects/*/plans/*.md` design documents (mostly proposed-but-not-yet-created files, and a batch of line-number drift in function references) — none fall in the twelve `CLAUDE.md` files or `GLOSSARY.md`. Not independently re-verified warning-by-warning; the checker's own output stands as recorded.

## Semantic — claims now false

**Nothing found.** Only two commits landed in the last 36 hours:

- `ee626f3` — ci: per-job cargo cache keys, saved from main only (#263) — touches only `.github/workflows/ci.yml`.
- `798e869` — fix(nix): stop release version bumps from rebuilding all dependencies (#262) — touches only `distrib/nix/crane.nix`.

Both are CI/Nix build-infrastructure changes with no effect on CLI subcommands, documented defaults/thresholds/flags, or any behavior described in a memory file. Grepped all twelve `CLAUDE.md` files and `GLOSSARY.md` for `ci.yml`, `crane.nix`, cargo-cache and Nix-build terms referenced in these commits — no hits. Neither commit invalidates any documented claim.

## Accretion — what could be removed and why

A full read of all twelve `CLAUDE.md` files plus `GLOSSARY.md` turned up six items, all verified against the code or the plans directories (not just suspected):

1. **Session-local scratch path baked into three tracked files.** `projects/kwaai-compute/CLAUDE.md:75`, `projects/kwaai-compute/roadmap.md:17`, and `projects/kwaai-compute/TODO.md:11` all point to `~/.claude/plans/cached-jingling-creek.md` as the plan for "Dedicated inference thread with session pool, LRU eviction." That path is a Claude Code session's own local scratch file, never committed to this repo — it won't exist for any other contributor or machine, and won't survive that local file being cleaned up. Should become a real path under `projects/kwaai-compute/plans/`, or the reference should be dropped if no such plan was ever checked in.

2. **Stale line-number anchors for the RoPE fix — confirmed wrong today.** `core/crates/kwaai-inference/CLAUDE.md:20` and `projects/kwaai-compute/CLAUDE.md:88` both say the fix is "at `shard.rs:104`: `x1.broadcast_mul(&cos4)` not `x1 * cos4`." Checked `core/crates/kwaai-inference/src/shard.rs`: the `broadcast_mul` calls are now at lines 108–116; line 104 is unrelated code. This is exactly the anchor rot the root `CLAUDE.md`'s own PR-review section warns about ("`service.rs:1653` dies on the next rebase ... name the function, not the line") — these two lines should name the function instead.

3. **`projects/kwaai-network/CLAUDE.md` describes p2p relay routing as still "in progress," but sibling docs treat it as already the preferred path.** Its "Current state → In progress/planned" list and its Metro-machines section both frame "p2p relay routing for inference (route through p2p network instead of direct TCP)" as a needed fix for broken DNS. But `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md` both document `p2p://PEER_ID` / `mux://PEER_ID` inference routing as working today and explicitly preferred ("no IP/DNS needed"), with worked example commands. This section in `kwaai-network/CLAUDE.md` reads like it hasn't been updated since the workaround it's waiting on actually shipped.

4. **Duplicate table row in `projects/kwaai-trust/CLAUDE.md`** (lines 56–57): the "Key source files" table lists `kwaai-cli/src/identity.rs` twice back to back, once as "`kwaainet identity` command handler" and once as "`kwaainet identity` handler — DID, VC import/list/verify." Copy-paste leftover; should be merged into one row.

5. **`projects/kwaai-trust/CLAUDE.md`'s "Shipped" section doesn't reflect the newest trust plan's decision to retire what it calls shipped.** Lines 35–39 list the home-grown Ed25519/DID/VC wallet (`FiduciaryPledgeVC`, `VerifiedNodeVC`, etc.) as shipped. But `projects/kwaai-trust/plans/DTGWG-TrustStack-plan.md` (re-synced 2026-10-01, the most recent trust-domain plan) explicitly commits to "adopt the DTGWG identity/VC stack, retiring our home-grown DID/VC code," and states it "supersedes the trust sections of `PublicRelease-plan.md`." The CLAUDE.md's "Shipped" framing doesn't mention this decision at all — worth a line noting the home-grown wallet is slated for replacement, even before the migration itself lands.

6. **`projects/kwaai-knowledge/GLOSSARY.md` §9 reading order has fallen behind the plans directory.** It recommends `PerKBOntology-plan.md`, `DreamRAG-Ontology-Eval-Compression.md`, and `OntologySession-assessment.md` (all from 2026-09-11) but omits `plans/OntologySelection-plan.md` (2026-10-20, status "design agreed"), which says it "completes `PerKBOntology-plan.md`'s ... never-built 'arm D'" — i.e. it's the direct continuation of the one plan the list does recommend. Minor, but worth adding.

Design note, not a verified staleness finding: hardcoded peer IDs for `metro-linux`/`metro-win`/`jerome` appear identically in three files (`projects/kwaai-compute/CLAUDE.md`, `projects/kwaai-knowledge/CLAUDE.md`). No evidence they're currently wrong, but a hardware swap would require editing three places by hand.

Files read in full and found clean, with no accretion issues: root `./CLAUDE.md`, `core/crates/kwaai-p2p/CLAUDE.md`, `core/crates/kwaai-rag/CLAUDE.md`, `core/crates/kwaai-storage/CLAUDE.md`, `projects/kwaai-storage/CLAUDE.md`, `projects/kwaai-platform/CLAUDE.md`, `projects/kwaai-knowledge/CLAUDE.md`.
