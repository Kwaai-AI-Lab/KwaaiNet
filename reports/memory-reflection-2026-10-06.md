# Memory reflection — 2026-10-06

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

All 12 `CLAUDE.md` files and `GLOSSARY.md` came back clean (zero warnings) — every warning is in a `projects/*/plans/*.md` document, where the checker expects drift because plans describe proposed work. None of the "(proposed?)" warnings above are findings; they're exactly what the checker is designed to not flag as broken. One is worth a human look: `RAGPerformanceReport-20260712.md`'s "not yet committed" status, which the checker flags as possibly stale given later changes to `kwaai-rag/src` — I did not chase this further since it's a plan doc, not one of the twelve authoritative `CLAUDE.md` files this reflection is scoped to.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned no commits — the last commit (`97d0b1e`, 2026-10-01) is almost five days old. There is nothing to check against a recent commit; this section is empty by construction, not by omission.

## Accretion — what could be removed and why

- **`projects/kwaai-knowledge/CLAUDE.md:71-72`** — "Legislation: TBD (Phase 3 — KB schema injected, r108 pending)" and "Publication: TBD (Phase 3 — KB schema injected, r108 pending)" are stale. `tests/kwaai-knowledge/results/eval_log.md` shows r108 ran on 2026-06-26 (Legislation/Publication types were part of that very run), and the eval log continues through at least r111. "r108 pending" has been false for roughly four months. Either fill in the actual per-type recall from the eval log or drop the line.

- **`projects/kwaai-network/CLAUDE.md:63`** vs. **`projects/kwaai-compute/CLAUDE.md:24-28`** and **`projects/kwaai-knowledge/CLAUDE.md:90-94`** — a direct contradiction. `kwaai-network/CLAUDE.md` lists "P2P relay routing for inference (route through p2p network instead of direct TCP)" under "In progress / planned", i.e. not shipped. But `kwaai-compute/CLAUDE.md` and `kwaai-knowledge/CLAUDE.md` both document `p2p://`/`mux://` inference routing as already-shipped, preferred infrastructure, with working example commands. I verified against the code: `core/crates/kwaai-cli/src/ollama_proxy.rs`, `inference_mux.rs`, and `capacity_lease.rs` are substantial, mature modules implementing exactly this (lease negotiation, mux proxying, p2p unary transport) — the feature is shipped. `kwaai-network/CLAUDE.md`'s "planned" line should be moved to "Shipped" or removed. (I could not pin this to a specific commit — the file history collapses to one large squash-merge commit, `952f737`, from the September PR queue described in the root `CLAUDE.md`, so I can't tell whether the doc was ever accurate or just copied forward through that merge.)

- **`core/crates/kwaai-trust/CLAUDE.md:56-57`** — the "Key source files" table lists `kwaai-cli/src/identity.rs` twice, with two slightly different descriptions ("`kwaainet identity` command handler" and "`kwaainet identity` handler — DID, VC import/list/verify"). Looks like an edit added the more detailed row without removing the original. Drop one.

- **`projects/kwaai-compute/CLAUDE.md:30-36`** and **`projects/kwaai-knowledge/CLAUDE.md:96-108`** — the same three peer IDs (metro-linux, metro-win, jerome) are hardcoded identically in both files (plus repeated again inline in each file's example commands — four copies total between the two files). `projects/kwaai-network/CLAUDE.md`'s own "Metro machines" section only names metro-linux and metro-win (no jerome) and separately tracks a DNS-broken status for them. If any of these peer IDs ever rotate (reinstall, key regen), there are four call sites to update and no single source of truth. Worth consolidating to one reference, e.g. in `kwaai-network/CLAUDE.md`, with the others pointing at it.

- **`core/crates/kwaai-inference/CLAUDE.md:20`** and **`projects/kwaai-compute/CLAUDE.md:88`** — both cite "RoPE fix at `shard.rs:104`: `x1.broadcast_mul(&cos4)`". The actual call is at `shard.rs:109` (and a second, symmetric one at `:116`); the fix itself is still there, just a few lines off from a rebase. Harmless today but will keep drifting — the mechanical checker doesn't catch this form (a quoted code snippet, not a named-symbol reference) since it isn't a form it parses.

- **`projects/kwaai-compute/CLAUDE.md:75`** — "Dedicated inference thread with session pool, LRU eviction (plan: `~/.claude/plans/cached-jingling-creek.md`)" points at a path under the user's home directory, outside the repo and outside git. Nobody else who reads this file can resolve it, and it will silently go stale the moment that local plan file is edited, renamed, or deleted. If the plan matters, it belongs under `projects/kwaai-compute/plans/` in-repo; otherwise drop the pointer.

Everything else read as consistent with the code I spot-checked (glossary term count matches the claimed "63 terms" exactly at 63 via the em-dash definition pattern; `docs/REVIEW_INTEGRITY_RETROSPECTIVE.md` referenced from the root `CLAUDE.md` exists; the `kwaai-knowledge/CLAUDE.md` "Do not re-enable unfiltered relation extraction" line already references the Phase-4 pipeline that fixed it, so it is not the stale pattern the task background warned about).

## Nothing found

No commits landed in the last 36 hours, so Semantic has nothing to report this cycle. I did not find any broken (as opposed to warned) references from the mechanical checker.
