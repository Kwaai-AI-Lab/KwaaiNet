# Memory reflection — 2026-09-30

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

0 broken references. All 46 warnings are on files under `plans/`, which the checker
itself treats as advisory, not failures. Most are `(proposed?)` references to files
that don't exist yet by design (the plan proposes them). A handful are genuine
"moved" flags worth a human glance next time those specific plans are touched
(`schema.rs`→`doc_schema.rs`, `src/mcp/server.rs`→`grpc_server.rs`,
`src/vpk/metrics.rs`→`core/crates/kwaai-network-tests/src/metrics.rs`, a one-line
drift in `MacOllamaStopgap-plan.md`), but none of them are in a `CLAUDE.md` or the
glossary, and none change a claim a reader would act on today. No `CLAUDE.md` shows
60+ days of drift this run.

## Semantic — claims now false, with file, line, and the commit responsible

**Nothing found.** `git log --since="36 hours ago" --stat` returned no commits.

One honest caveat: the checkout I was given is a detached `HEAD` at `486bcf6`,
5 days old (last commit 2026-09-25T16:08:39-07:00), sitting 8 commits ahead of the
local `main`/`origin/main` pointer (`5bb2ce0`) on unpushed `docs(knowledge)` work
about the AIAS+ 2026 submission. Those 8 commits are docs-only (paper text, response
letter, figure/reference fixes) and don't touch any `CLAUDE.md` or the glossary, so
they don't change this section's answer. But if a newer default-branch state exists
elsewhere that this checkout doesn't have, this run wouldn't see it — worth checking
that the checkout used for future runs is current before trusting a repeated
"nothing found" here.

## Accretion — what could be removed and why

**The one item worth naming is already on record and still there.**
`projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md` (§2, written
2026-08-26) flagged that `projects/kwaai-knowledge/CLAUDE.md`'s D6 multi-machine
build example hardcodes three P2P peer IDs (`metro-linux`, `metro-win`, `jerome`,
lines 96–104) as an example of a file that "only ever accretes." Five weeks later
that table and example command are unchanged. I can't verify from this checkout
whether those peer IDs are still live (that needs the running daemon), so I'm not
asserting they're wrong — only that this is the concrete accretion candidate the
project's own prior reflection already identified and nothing has acted on it since.

Nothing else stood out: no hardcoded absolute paths (`/home/`, `/Users/`) in any
`CLAUDE.md` or the glossary, no plan doc whose own content claims it was
superseded, no CLAUDE.md over the 60-day drift threshold this run.

## Nothing found

No broken references, no falsified claims from the last 36 hours (none existed to
check), no `CLAUDE.md`/glossary drift over threshold.
