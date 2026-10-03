# Memory reflection — 2026-10-03

Checked out `main` at `97d0b1e235f4d2bd9dbf8645013e288e3d026ad9`.

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
    warn    `graph.rs:1912` — `coref_candidates_for_chunk` is defined at graph.rs:1998
  projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md
    warn    `analysis/analyze.py` — no file at that path (moved? found tests/kwaai-knowledge/eval2/pilot/d6_noseed_analyze.py)
    warn    file reference does not resolve: `pilot/ontology_pilot.py` (proposed?)
  projects/kwaai-knowledge/plans/DreamRAG-AIAS2026-manuscript-plan.md
    warn    `../papers/aias2026/figures/gen_paper_figures.py` — no file at that path (moved? found projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py)
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
    warn    `graph.rs:1215` — `upsert_relation` is defined at graph.rs:1301
    warn    `graph.rs:5563` — `extract_from_text` is defined at graph.rs:5661
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
    warn    `rag_cmd.rs:7372` — `extract_rc_windows` is defined at rag_cmd.rs:7385
    warn    `rag_cmd.rs:7598` — `call_llm_for_relations` is defined at rag_cmd.rs:7611
    warn    `sequence.rs:777` — `extract_kinship_interactions` is defined at sequence.rs:796
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
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
  projects/kwaai-knowledge/plans/coref-pronoun-resolution.md
    warn    `src/graph.rs:1461` — `GraphStore::link_chunk` is defined at src/graph.rs:1547
    warn    `src/graph.rs:4832` — `GraphStore::all_chunk_entity_pairs` is defined at src/graph.rs:4929
    warn    `src/meta_store.rs:177` — `MetaStore::all_chunks` is defined at src/meta_store.rs:193
    warn    `src/rag_cmd.rs:6508` — `FAMILY_TRIGGERS` is defined at src/rag_cmd.rs:6521
  projects/kwaai-knowledge/plans/d6-rag-accuracy-improvement.md
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_relation_hard_cases.md` (proposed?)
    warn    function reference does not resolve: `is_family_query()` (proposed?)
    warn    `retriever.rs:456` — `resolve_author_relative` is defined at retriever.rs:591
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

0 broken reference(s), 65 warning(s) across 51 files
```

Exit code 0. All 65 warnings are line-drift or proposed-but-unbuilt references inside `plans/` documents — none are in the twelve `CLAUDE.md` files or `GLOSSARY.md`. One warning is worth acting on and is carried into Accretion below: the checker's own flag on `RAGPerformanceReport-20260712.md`'s stale "not yet committed" status.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago"` shows two commits: `e4e9aa9` (nix build fix, `distrib/nix/crane.nix` + `packages.nix`) and `97d0b1e` (kwaai-rag retrieval/dream fixes + Eval v2 harness, #239).

Checked both against all twelve `CLAUDE.md` files and `GLOSSARY.md`, specifically for claims touching the files `97d0b1e` changed (`cli.rs`, `bm25.rs`, `dream.rs`, `graph.rs`, `iterative.rs`, `meta_store.rs`, `query_understand.rs`, `retriever.rs`, `sequence.rs`):

- `projects/kwaai-knowledge/CLAUDE.md` "Do not" section already qualifies the relation-extraction warning as *unfiltered* extraction and names the Phase-4 axiomatic pipeline as the fix — not invalidated by this commit, which didn't touch that pipeline.
- No `CLAUDE.md`/`GLOSSARY.md` documents graph-retrieval ordering, `entity_chunks()`, RRF tie-breaking, or the `--no-relations` dream-cycle bug that `97d0b1e` fixed. The one doc that *did* describe that bug, `projects/kwaai-knowledge/plans/D6-NoSeed-Dream-plan.md`, was added by this same commit and is self-consistent: its "Context" section describes the bug as motivation, followed by a "Result (2026-09-29)" section confirming the fix. Not stale.
- No `CLAUDE.md` documents the nix/crane build internals `e4e9aa9` touched.
- The new `--mode graph-only` and `--dump-jsonl`/`--run-tag` eval flags added to `cli.rs` aren't mentioned in `projects/kwaai-knowledge/CLAUDE.md`'s mode list — an omission, not a false claim, so not reported here as a semantic finding (see Accretion for the related staleness this surfaced).

**Nothing found** in this section: no memory-file claim was made false by the commits in the last 36 hours.

## Accretion — what could be removed and why

1. **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — "Status: code complete, not yet committed" for the redb→SQLite/WAL migration. This is stale: `rusqlite` has been a direct dependency of `kwaai-rag` for months, `meta_store.rs` (the file this note is about) has been touched by commits as recent as `97d0b1e` (2026-10-01), and `projects/kwaai-knowledge/d6_accuracy_progress.md:438` already correctly says "SQLite since July 2026." The checker itself flags this line. Fix: update the status line or mark the whole report historical — not caused by a recent commit, just never corrected after the migration landed.

2. **Hardcoded peer IDs for `jerome`** in `projects/kwaai-knowledge/CLAUDE.md:100` and `projects/kwaai-compute/CLAUDE.md:34`, both listing jerome in the P2P GPU relay table with no caveat, including in "all three GPUs" example build commands. `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:114` records jerome as offline with "148+ consecutive 'routing: not found' failures; excluded from pipeline" as of 2026-07-12. I could not verify current p2p status from this session (no access to ping the peer), so I'm not asserting the CLAUDE.md tables are wrong today — only flagging that two authoritative files present jerome as an available GPU node with no caveat, while the most recent written record of its status says otherwise. Worth a 30-second check (`kwaainet p2p status` or similar) before the next time someone copies that example command.

## Nothing found

No broken references (checker exit 0, 0 broken reference(s)). No commit in the last 36 hours made any `CLAUDE.md`/`GLOSSARY.md` claim false.
