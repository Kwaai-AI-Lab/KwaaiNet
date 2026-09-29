# Memory reflection — 2026-09-29

## Mechanical — checker output, verbatim

```
Checking 49 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1082
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
  projects/kwaai-knowledge/plans/DreamRAG-AIAS2026-manuscript-plan.md
    warn    `../papers/aias2026/figures/gen_paper_figures.py` — no file at that path (moved? found projects/kwaai-knowledge/papers/aias2026/figures/gen_paper_figures.py)
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
  projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md
    warn    function reference does not resolve: `copy_metrics()` (proposed?)
    warn    function reference does not resolve: `pick_best_relation_thresholds()` (proposed?)
    warn    function reference does not resolve: `print_metrics()` (proposed?)
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
  projects/kwaai-knowledge/plans/d6-rag-accuracy-improvement.md
    warn    file reference does not resolve: `tests/kwaai-knowledge/d6_relation_hard_cases.md` (proposed?)
    warn    function reference does not resolve: `is_family_query()` (proposed?)
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

0 broken reference(s), 46 warning(s) across 49 files
```

Exit code 0. All 46 warnings are either `(proposed?)` placeholders in forward-looking plan docs (expected — the file/function doesn't exist yet because the plan hasn't been executed) or, per manual triage below, false positives against files that live in a separate private repo (PHE) the checker can't see. Two warnings pointed at genuine drift; both are covered under Semantic.

## Semantic — claims now false

`git log --since="36 hours ago" --stat` returned no commits — the latest commit on `main` (`486bcf6a`) is from 2026-09-25, four days ago. There is nothing in this window to review by the letter of the method.

Manually triaging the checker's `(moved?)` warnings (as opposed to the expected `(proposed?)` ones) surfaced two claims that are false independent of any recent commit — worth reporting even though no single recent commit caused them:

- **`projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md:152-167`** — Phase 3 is marked `STATUS: ✅ COMPLETED (2026-06-26)`, but its own "Changes" step 3 and "Files" line still describe the deliverable as "Implement `KBEntityTypeSchema` struct in a new `schema.rs` module" / "new `core/crates/kwaai-rag/src/schema.rs`". No `schema.rs` was ever created. Verified: `KBEntityTypeSchema` is defined at `core/crates/kwaai-rag/src/graph.rs:365`, not in a standalone module. `doc_schema.rs` (what the checker's fuzzy match found) is an unrelated file — per `projects/kwaai-knowledge/CLAUDE.md:137` it holds `DocSchema`/`auto_detect_schema`/`parse_index_seeds`, a different feature (document section-skip rules, not entity typing). The doc should say the schema landed in `graph.rs`, not leave the completed phase describing a module that doesn't exist.
- **`projects/kwaai-compute/plans/MacOllamaStopgap-plan.md:62`** — cites `KwaaiNetConfig::announce_state()` at `config.rs:1059` to justify "No announce changes needed"; the function is actually at `core/crates/kwaai-cli/src/config.rs:1082`. The underlying design conclusion isn't in question, just the line pointer — worth a quick fix per the project's own "anchor to symbols, not line numbers" convention, since this doc still uses a bare line number.

Two other `(moved?)` warnings looked like drift but are not, on inspection:
- `MemoryIntegrity-growth-cycle.md`'s `src/mcp/server.rs` reference and `VPK-CrateIntegration-plan.md`'s `src/vpk/metrics.rs` reference both describe files in the separate private PHE repo (explicitly noted in each doc's own header/body), not this repo — the checker can only resolve against KwaaiNet's own tree and coincidentally matched unrelated same-named files here. Not a doc problem.
- `DreamRAG-AIAS2026-manuscript-plan.md`'s `../papers/aias2026/...` reference resolves correctly relative to the plan doc's own location; the checker resolves it against repo root, producing a spurious warning.

Also checked, per the task brief's own example: `projects/kwaai-knowledge/CLAUDE.md:145` currently reads "Do not enable *unfiltered* relation extraction for 8B models" and explicitly credits the Phase-4 axiomatic pipeline (lexical trigger pre-filter + confidence-tiered LLM pass) as the machinery that makes constrained extraction viable. This is already correctly qualified and not stale — it is the corrected version of the exact trap the task description warned about, not an instance of it.

## Accretion — what could be removed and why

- **Duplicated infra table**: the same three peer IDs (metro-linux, metro-win, jerome) and the same multi-machine build command are hardcoded in both `projects/kwaai-compute/CLAUDE.md:31-36` and `projects/kwaai-knowledge/CLAUDE.md:96-107`. Two copies of physical-machine infrastructure details means a hardware change (or an ID rotation) has two places to miss instead of one. Candidate to consolidate into a single source (e.g. the compute project doc, referenced by the knowledge doc) rather than copy-pasted.
- **`projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md`** is fully shipped — all 6 phases marked `✅ COMPLETED` between 2026-06-25 and 2026-06-26, three months stale relative to today, sitting in `plans/` alongside active forward-looking plans. Combined with the stale `schema.rs` claim above, this is a good candidate to either fix in place or fold into a short completed-work summary rather than leave as a live plan doc.

No hardcoded absolute local paths (`/Users/`, `/home/`) or other stray peer IDs were found outside the one documented macOS `codesign` build step (`projects/kwaai-platform/CLAUDE.md`). No other plan file under `plans/` (37 checked) showed a fully-shipped, three-months-stale pattern like the one above. Root `CLAUDE.md` and the per-project `CLAUDE.md` files themselves showed no leftover-cruft sections or broken cross-references on inspection.

## Nothing found

No commits landed in the 36-hour window, so there is no renamed/deleted CLI subcommand, changed default/threshold, or inverted behavior to check from recent history.
