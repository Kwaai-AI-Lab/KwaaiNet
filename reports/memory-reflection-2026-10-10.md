# Memory reflection — 2026-10-10

Checked out at `d46bb4a` (origin/main tip). Scope: 12 `CLAUDE.md` files + `projects/kwaai-knowledge/GLOSSARY.md`.

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

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```
Exit code 0. Every one of the 101 warnings is in `projects/*/plans/*.md` (proposed/future file
names not yet created, or function/line citations that drifted by a few lines after edits) —
**none** fall in the 12 `CLAUDE.md` files or `GLOSSARY.md` that are this report's actual focus.
One is worth a human look regardless of scope: `RAGPerformanceReport-20260712.md` is flagged as
possibly claiming "not yet committed" for code that has since changed — see that file if anyone
is relying on its landed/not-landed status.

## Semantic — claims now false

`git log --since="36 hours ago" --stat` returned **no commits** — the last commit (`d46bb4a`) landed
2026-10-07, three days ago. So there is nothing to diff-check for commit-caused staleness. Given that,
I instead spot-verified existing claims in the 12 `CLAUDE.md` files + `GLOSSARY.md` against the current
code (not gated to the empty 36h window), since that's the actual failure mode this routine exists to
catch. Four confirmed:

1. **`core/crates/kwaai-storage/CLAUDE.md`, lines 3–4 and 15** — claims the crate implements
   "multi-tenant **homomorphic-encrypted** vector storage" and that `src/vectors.rs` does "Homomorphic
   vector search." False: `core/crates/kwaai-storage/src/lib.rs:5–9` states "The crate deliberately
   knows nothing about encryption: vectors are opaque `Vec<f32>`…", and `vectors.rs` does plain cosine
   similarity with no encryption. The sibling doc `projects/kwaai-storage/CLAUDE.md` already has this
   right ("vectors are plaintext f32 today — encryption is planned"; `vectors.rs` → "Cosine vector
   search (plaintext)"). The crate-level file is the stale one.

2. **`projects/kwaai-network/CLAUDE.md`, lines 31–33 and 63** — "Fix needed: p2p relay routing for
   inference requests" and, under "In progress / planned," "P2P relay routing for inference (route
   through p2p network instead of direct TCP)." This is shipped, not planned:
   `core/crates/kwaai-cli/src/ollama_proxy.rs:608–626` implements `p2p://PEER_ID` / `mux://PEER_ID`
   resolution and proxying to remote Ollama, used throughout `rag_cmd.rs` and `shard_cmd.rs`. It's
   documented as working, default infrastructure in `projects/kwaai-compute/CLAUDE.md` (lines 24–36)
   and `projects/kwaai-knowledge/CLAUDE.md` (lines 93–116), with real `--inference-urls p2p://…`
   commands. Even this same file's own "Do not" list (line 92: "Do not send inference requests
   directly over TCP… use p2p relay") already assumes the relay works. The "in progress" framing
   should go, or be replaced with whatever narrower piece (if any) is still outstanding.

3. **`projects/kwaai-knowledge/GLOSSARY.md`, line 90** — "35 in the global list, 14 of them kinship."
   The kinship count is right; the total isn't. `core/crates/kwaai-rag/src/graph.rs`'s
   `RELATION_TYPES` const (lines 40–92) currently has **44** entries (14 kinship + 7 agent + 3
   spatial/biographical + 5 structural + 5 temporal + 5 semantic + 5 informational — counted
   directly). The structural/temporal/semantic/informational groups were added after this line was
   written and the total was never updated.

4. **`projects/kwaai-knowledge/CLAUDE.md`, lines 76–77** — the per-type recall table (under a section
   literally titled "Phase 3 confirmed") lists "Legislation: TBD (Phase 3 — KB schema injected, r108
   pending)" and the same for Publication. r108 is not pending — it ran and is reported on in
   `projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md:136,268` ("D6 rebuild (r108) runs with
   `--timeline`" / "Phase 3+4+5 (r108, fresh rebuild): 135/222 = 60.8%") and again in
   `RAGPerformanceReport-20260712.md:46` (Phase 3 overall 88.9–89.5%), both from months before today.
   Either the Legislation/Publication recall numbers were never extracted from that run and should say
   so, or they were and this table was never filled in.

## Accretion — what could be removed or reworded

- **`projects/kwaai-compute/CLAUDE.md:75`, `TODO.md:11`, `roadmap.md:17`** — all three point to
  `~/.claude/plans/cached-jingling-creek.md` for the session-pool/LRU-eviction plan. That's an
  absolute path under one person's home directory, not a repo path — it can't resolve for anyone
  else, including any agent working from a fresh checkout (confirmed: no such file exists anywhere
  in this tree). Either inline the plan's substance into the repo (e.g. under
  `projects/kwaai-compute/plans/`) or drop the dangling pointer from all three files.

- **`projects/kwaai-trust/CLAUDE.md:56–57`** — `kwaai-cli/src/identity.rs` is listed twice in the "Key
  source files" table with near-duplicate descriptions ("`kwaainet identity` command handler" /
  "`kwaainet identity` handler — DID, VC import/list/verify"). Harmless but should be one row.

- **`projects/kwaai-storage/CLAUDE.md`** — scope (line 6) and "Shipped" both say vectors are stored
  as plaintext f32 *today*, but "Do not" (line 90) reads as a present-tense rule: "Do not store
  unencrypted vectors for `eve`/`bob` modes" — which the shipped code is currently doing. Worth
  rewording to make clear this is a constraint for once PHE encryption lands, not one already in
  force, so it doesn't read as self-contradicting or get silently ignored as already-violated.

Nothing else in the twelve files looked like dead weight — no other hardcoded peer IDs, superseded
plan documents, or long-finished work sections turned up (the three Metro-machine peer IDs in
`kwaai-compute/CLAUDE.md` and `kwaai-knowledge/CLAUDE.md` are still used in commands across recent
plan docs, so they're live, not stale).
