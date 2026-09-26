# Memory reflection — 2026-09-26

Reviewed at `486bcf6a17e31481731f8b084ab232b927c573c5` (origin/main, confirmed via fresh `git fetch`).

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

All 49 files are `plans/*.md`; none of the 46 warnings touch a CLAUDE.md or GLOSSARY.md. Every warning is either a "(proposed?)" reference to a file/function that hasn't been built yet (expected for design docs) or a path that resolved to a moved file. One of the latter is a false positive worth naming: `DreamRAG-AIAS2026-manuscript-plan.md`'s `../papers/aias2026/figures/gen_paper_figures.py` is written as inline code, not a Markdown link, and does resolve correctly relative to the plan file — the checker appears to resolve bare-backtick paths against the repo root rather than the referring file, which is why it warns here. Not a doc problem.

## Semantic — claims now false, with file, line, and the commit responsible

**1. Tracked `.docx` outputs now contradict the root CLAUDE.md's "no .docx is tracked" rule — and the commits that did it said so.**

`CLAUDE.md:226-231` states: *"No `.docx` is tracked... The one thing that may legitimately be tracked is a pandoc reference template — a `.docx` that is an input to a build, not an output of one."*

Commit `3fb0185` (2026-09-25) added `projects/kwaai-knowledge/plans/submission54-manuscript.docx`, a build **output** of `build_manuscript_docx.py`, with the commit message itself noting: *"Committed at the author's request, despite CLAUDE.md's 'no .docx is tracked' rule; regenerate it from the .md, never edit it."* Commit `7f976b0` added a second build output, `submission54-4page.docx`, the same way. Commits `c42dd5e` and `51b35b4` (also in this window) relocated both files but kept them tracked; they are still tracked at HEAD:

```
projects/kwaai-knowledge/papers/aias2026/submission54-4page.docx
projects/kwaai-knowledge/papers/aias2026/submission54-manuscript.docx
```

This is a deliberate, acknowledged exception, not an accident — but the rule as written in `CLAUDE.md` no longer describes the repository. Either the rule needs a carve-out for author-requested submission artifacts, or these two files should move to a gitignored `rendered/`-style location once the submission is final. Not fixing the doc myself per this task's scope (report only).

*(For contrast, `_easychair_template.docx` and `projects/kwaai-knowledge/plans/_base.docx`, also tracked, are template **inputs** to a build and do fit the existing exception — no issue there.)*

**2. `projects/kwaai-knowledge/CLAUDE.md`'s "100w" chunk-size claim is false, and this window's commit says so explicitly — but the discrepancy predates the window.**

`projects/kwaai-knowledge/CLAUDE.md:59` — *"Chunk size | 100w sentence-aligned"* — and `:150` — *"Do not change chunk size away from 100w without re-running Phase 2 experiments."*

Commit `3fb0185` (in this window) records as a manuscript finding: *"Chunking is <=800 characters, not the '100w' that projects/kwaai-knowledge/CLAUDE.md claims."* I verified against the code: `core/crates/kwaai-rag/src/chunker.rs:38` sets `chunk_size: 800` as the default, and the chunker operates in `.chars().count()`, not words (e.g. `chunker.rs:408`, `:448`). So the claim is confirmed false.

Important caveat: `git blame` shows both the `CLAUDE.md` "100w" line and the `chunk_size: 800` code date to the same commit, `19e96ae` (2026-09-09) — over two weeks before this reflection's 36-hour window. No commit in the last 36 hours *made* this claim false; the AIAS2026 manuscript work in this window only *discovered and documented* a pre-existing discrepancy. Flagging it here anyway since it's now a confirmed, well-formed false claim in an authoritative CLAUDE.md file, not a suspicion.

No other findings recorded in the manuscript commits (Dream-merge description handling, RRF score magnitudes, `scorer.rs`'s per-type vs per-KB ontology tables) correspond to any claim in a CLAUDE.md or GLOSSARY.md file — checked by grep, no matches. Those are new observations, not contradictions of existing memory.

The `release(v0.7.2)` commit (`2a19d81`) itself only touches `Cargo.toml`/`Cargo.lock` version pins — it's a version bump, not a behavior change. The behavior changes it summarizes (#231 process supervision, #228 ephemeral start flags, #226 `config set` guard, etc.) all merged on 2026-09-19 through 2026-09-22, outside this window, and none of them are referenced anywhere in the twelve CLAUDE.md files (checked by grep across the plausibly-affected ones: root, kwaai-platform, kwaai-network, kwaai-compute, kwaai-inference, kwaai-p2p), so there's nothing to invalidate there.

## Accretion — what could be removed and why

Nothing found that clearly outlived its use in this pass. The hardcoded lab peer IDs in `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md` (metro-linux/metro-win/jerome) and the DNS note in `projects/kwaai-network/CLAUDE.md` look like they could go stale, but I have no way to verify from this checkout whether those machines are still live — flagging as something a human would need to check, not reporting it as a finding.

The two tracked-docx items above are really both a semantic (rule now false) and an accretion (should these files eventually be dropped from git once the submission is finalized) question at once; recorded once above rather than twice.

## Nothing found

The mechanical checker is clean of broken references, and no CLAUDE.md/GLOSSARY.md claim was invalidated by an in-window code change beyond the two items above.
