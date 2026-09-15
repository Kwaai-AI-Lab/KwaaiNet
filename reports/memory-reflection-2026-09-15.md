# Memory reflection — 2026-09-15

Scope: default branch (`main`), commits from the 36 hours preceding this run
(`git log --since="36 hours ago"`, 2026-09-14 17:19–19:30 PDT, 4 commits: #217, #213, #189, #164/#169 — #164 and #169 landed back-to-back).

## Mechanical — checker output, verbatim

```
Checking 48 memory and plan files

  projects/kwaai-compute/plans/MacOllamaStopgap-plan.md
    warn    `config.rs:1059` — `KwaaiNetConfig::announce_state` is defined at config.rs:1043
  projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md
    warn    `core/crates/kwaai-rag/src/schema.rs` — no file at that path (moved? found core/crates/kwaai-rag/src/doc_schema.rs)
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
  projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
    warn    status says "not yet committed" but core/crates/kwaai-rag/src has changed since — recheck whether it has landed
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

0 broken reference(s), 46 warning(s) across 48 files
```

Zero errors. All 46 warnings are on `plans/` files, which the checker (since #164) treats as
warn-only because plans legitimately name not-yet-created files. One warning is worth a human
look even though it's not new: `RAGPerformanceReport-20260712.md` flags itself as possibly
stale ("not yet committed" but `kwaai-rag/src` has since changed) — not caused by a commit in
this window, so left for the next pass rather than chased here.

## Semantic — claims now false, with file, line, and the commit responsible

**`core/crates/kwaai-storage/CLAUDE.md`, lines 3 and 15 — still claims VPK ships homomorphic
search; commit `1c0b342` (#169) fixed every sibling doc but this one.**

PR #169 ("docs(storage): stop claiming homomorphic search ships") corrected the same overclaim
in `projects/kwaai-storage/CLAUDE.md`, `roadmap.md`, `design/overview.md`, and
`design/data-flows.md` — its own diff touches exactly those four files. It did not touch
`core/crates/kwaai-storage/CLAUDE.md`, which still reads:

- line 3: "This crate implements VPK (Virtual Private Knowledge): multi-tenant
  **homomorphic-encrypted** vector storage, ..."
- line 15: `| \`src/vectors.rs\` | **Homomorphic vector search** |`

Checked against the code #169 cites: `core/crates/kwaai-storage/src/db.rs` stores
`Hnsw<'static, f32, DistCosine>` over plaintext `f32` embeddings (confirmed by reading the
file — `use hnsw_rs::...DistCosine`, `pub hnsw: Hnsw<'static, f32, DistCosine>`), and
`src/vectors.rs` contains no reference to encryption at all (`grep -i
"encrypt\|homomorphic"` returns nothing). The corrected sibling file now says "vectors are
plaintext f32 today — encryption is planned, see roadmap.md" and "VPK: multi-tenant vectors
(plaintext today)" for the same crate. `core/crates/kwaai-storage/CLAUDE.md` is the crate-level
memory file a session lands on when working directly in `core/crates/kwaai-storage/`, so it
carries the same overclaim #169 set out to remove, just one directory over.

No other memory file was invalidated by this window's commits. I checked `dht_server` (#217,
`ac1ea5e`) and `enable_quic`/QUIC defaults (#189, `9ad8fbc`) against all 13 memory files
(`grep -rn` across the 12 CLAUDE.md files and the glossary) — neither term appears anywhere
outside the crate source and #217/#189's own field-doc comments, so neither commit's behavior
change left a stale claim in a memory file to find.

## Accretion — what could be removed and why

Nothing found this pass. I looked for the specific patterns called out (hardcoded peer IDs,
absolute paths, stale command references, superseded plans, finished-work sections) but didn't
find anything I could confirm as dead rather than merely old:

- `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md` hardcode three
  `12D3Koo...` peer IDs for named lab machines (metro-linux, metro-win, jerome). This is the
  shape of thing the task asked me to flag, but I have no way to check from here whether these
  peers are still live, and unlike the homomorphic-search claim above I have no commit or code
  evidence either way — flagging without evidence would be exactly the kind of manufactured
  finding this report is supposed to avoid. Worth a human check (or a future pass with p2p
  access) rather than a claim here.
- `docs/REVIEW_INTEGRITY_RETROSPECTIVE.md` (added by #213) notes its own open loose end: a
  proposed 5-PR review-batch cap needs reconciling with an unpushed 3-PR merge cap on a branch
  not present in this checkout. Not a memory file and not actionable from here.

## Nothing found

- Mechanical: 0 broken references.
- Semantic: only one invalidated claim found (above); #213, #189's and #217's own in-repo field
  docs, and #164's checker changes did not leave any other memory file stale.
- Accretion: no confirmed-dead content to remove this pass.
