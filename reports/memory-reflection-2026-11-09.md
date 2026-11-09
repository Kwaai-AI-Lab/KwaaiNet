# Memory reflection — 2026-11-09

Scope: root `CLAUDE.md`, the 11 other `CLAUDE.md` files, and
`projects/kwaai-knowledge/GLOSSARY.md`, checked against `origin/main` at
`e98ca3733da35f6e5dea1abb70fd45f735d2fd67`.

## Mechanical — checker output, verbatim

`scripts/check_memory_integrity.py` exists and ran clean (exit 0):

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

**Read on the output:** 0 broken references. Of the 135 warnings, 134 sit inside
`projects/*/plans/*.md` files, and the great majority are tagged `(proposed?)` by
the checker itself — forward-looking references in design/eval plans, not claims
about current state. I did not re-flag those. Two are worth a human look but are
not memory-file bugs per se: `RAGPerformanceReport-20260712.md`'s "not yet
committed" status note (the checker already surfaces this — `core/crates/kwaai-rag/src`
has moved on since), and the five line-number drifts in `coref-pronoun-resolution.md`
and `PerKBOntology-plan.md` where the named function still exists but has moved —
harmless under "anchor findings to symbols, not line numbers" since these name the
function correctly and only the line number is stale.

## Semantic — claims now false

No commits landed in the last 36 hours (`git log --since="36 hours ago"` on
`origin/main` is empty; the last commit, `e98ca373`, is from 2026-11-04). So there
is nothing to check under Step 2 as scoped — no commit in the window could have
invalidated a memory file.

While verifying Step 3 (accretion) against the current code, two claims turned up
that are false today regardless of the 36-hour window. I'm reporting them here
since they're exactly the kind of drift Step 2 is meant to catch, but I could not
pin the invalidating commit: this repo's history was squashed on 2026-08-28 (see
`backup/main-before-reset-20260828`), and `git log` on every file below bottoms out
at the squash commit `e0764ea` with no later touch, so the original change predates
what this checkout can see.

1. **`projects/kwaai-network/CLAUDE.md:31-33`** says of the metro GPU machines:
   > `metro-linux` / `metro-win` — **DNS broken** ... Fix needed: p2p relay routing
   > for inference requests (user intent: "use p2p relay")

   This is no longer true. `core/crates/kwaai-cli/src/ollama_proxy.rs:608,617`
   implements exactly this: `strip_prefix("mux://")` and `strip_prefix("p2p://")`
   resolve a peer ID over the relay to the remote Ollama instance. Confirming this,
   **`projects/kwaai-compute/CLAUDE.md:24-28`** already documents it as shipped,
   with the same two machines by peer ID and concrete `--inference-urls` usage. The
   `kwaai-network` doc's "fix needed" framing is stale; the `kwaai-compute` doc has
   already moved past it. Whoever reads `kwaai-network/CLAUDE.md` first would think
   this is still an open problem.

2. **`projects/kwaai-knowledge/GLOSSARY.md:90`** says:
   > `measured_by`. 35 in the global list, 14 of them kinship.

   The kinship count (14) is right. The global count is not: `RELATION_TYPES` in
   `core/crates/kwaai-rag/src/graph.rs:40-85` now holds 44 entries (14 kinship + 7
   agent + 3 spatial + 5 structural + 5 temporal + 5 semantic + 5 informational —
   counted directly from the array). `GLOSSARY.md` has no edits at all since the
   squash commit, so "35" is simply an uncorrected count from before this checkout's
   visible history; I can't tell from here which PR grew the list past 35.

One more, found the same way, that's a reality gap rather than a drifted number:

3. **`projects/kwaai-trust/CLAUDE.md:65`** — "Do not store credentials unencrypted
   on disk" is written as a standing rule, with nothing elsewhere in the file
   flagging the current implementation as falling short of it. But
   `core/crates/kwaai-trust/src/storage.rs` stores each credential as a plain JSON
   file (file's own doc comment: "no database, no index, just files"; zero
   `encrypt`/`cipher`/`aes` hits in the file), and `core/crates/kwaai-cli/src/identity.rs:79-81`
   writes `identity.key` as raw protobuf bytes with no encryption step. Either the
   rule is aspirational and should say so, or credentials really are unencrypted on
   disk today and that's a gap worth someone's attention — I can't tell which from
   the doc alone, but the doc currently reads as asserting the invariant already
   holds.

   For contrast, `projects/kwaai-storage/CLAUDE.md:90` has a similarly-worded rule
   ("Do not store unencrypted vectors for `eve`/`bob` modes"), but that file is
   explicit elsewhere (lines 6, 57, 83) that vectors are plaintext today and
   encryption is roadmap work — so that one reads as a forward guardrail, not a
   false claim. I'm not flagging it.

## Accretion — what could be removed and why

1. **Dangling session-local plan path, repeated in three files**:
   `~/.claude/plans/cached-jingling-creek.md` appears in
   `projects/kwaai-compute/CLAUDE.md:75`, `projects/kwaai-compute/TODO.md:11`, and
   `projects/kwaai-compute/roadmap.md:17`, all citing it as the plan for the
   dedicated-inference-thread work. That path does not exist anywhere in this
   container (checked `~/.claude/plans/` and a filesystem-wide search) — it's a
   Claude Code session-local plan file from whoever's machine generated it,
   committed as if it were a repo-relative path. It will never resolve for anyone
   else working from a different machine or session. The mechanical checker
   doesn't catch `~/`-rooted paths. Worth either inlining the plan's substance into
   the roadmap/TODO entries or removing the dead citation.

2. **`projects/kwaai-storage/CLAUDE.md:40,43`** point at
   `tests/kwaai-storage/mock-vpk-health.py` (port 7432) and
   `tests/kwaai-storage/vpk-lan-test.sh`. Neither file is at that path — both now
   live at `tests/kwaai-network/mock-vpk-health.py` and
   `tests/kwaai-network/vpk-lan-test.sh` (confirmed, including the port-7432
   detail). `tests/kwaai-storage/` holds a different, unrelated set of scripts
   today (`local-storage-test.sh`, `storage-lan-test.sh`, etc.). The mechanical
   checker misses this because the references sit inside a ` ```bash ` fence, not a
   markdown link it parses. This is a real case of "commands referencing files that
   moved."

3. **`projects/kwaai-trust/CLAUDE.md:56-57`** — `kwaai-cli/src/identity.rs` is
   listed twice in the "Key source files" table, once as `kwaainet identity`
   command handler and once as `kwaainet identity` handler — DID, VC
   import/list/verify. Reads as a merge artifact; one row should go.

## Nothing found

- No broken mechanical references (0/135 warnings are breaks; all are drift
  warnings, and almost all are in forward-looking plan docs that are tagged as
  such).
- No commits in the last 36 hours, so Step 2 as literally scoped (a memory claim a
  recent commit made untrue) has nothing to check this cycle.
- I did not find a renamed/deleted CLI subcommand still documented as existing, nor
  a "do not do X" warning where X has since been made safe by new machinery (the
  credentials case above is the inverse shape: a "do not" rule the code already
  violates, not an outdated warning against something now-safe).

## Unverifiable from this checkout

- GLOSSARY.md's KB count ("sixteen exist"), D6's question count ("40"), and the
  relation-density numbers (D6 0.225, DeepSea 0.000) are claims about private,
  gitignored data (`~/.kwaainet/config.yaml`, the D6 corpus) that isn't in this
  container. I can't confirm or refute them from here.
- The hardcoded peer-ID tables for `metro-linux`/`metro-win`/`jerome` in
  `kwaai-compute` and `kwaai-knowledge` CLAUDE.md assume those machines are still
  live and still hold those keys. I have no network access from this checkout to
  confirm; if any have been decommissioned or re-keyed, those tables are stale, but
  I have no evidence either way.
