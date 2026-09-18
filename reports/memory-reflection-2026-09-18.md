# Nightly memory reflection — 2026-09-18

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

0 broken reference(s), 45 warning(s) across 48 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

Exit code 0. All 45 warnings are in `projects/*/plans/*.md` design/proposal documents, not in the twelve `CLAUDE.md` files or `GLOSSARY.md`. Almost all are references marked `(proposed?)` — files/functions the plan describes building, not claims of current state. The three non-"proposed" warnings are pre-existing line-number drift or file moves in plan docs (`config.rs:1059`, `schema.rs`→`doc_schema.rs`, `metrics.rs` location), already the checker's own "moved?" case, not new breakage. Zero broken references.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` shows 7 commits, PRs #220–#226, all by RezaRassool, all in `core/crates/kwaai-cli`, `kwaai-inference`, `README.md`, and installer scripts. None touch `kwaai-rag`/`kwaai-knowledge` domain code. Checked each against all 12 `CLAUDE.md` files and `GLOSSARY.md` for a claim it could invalidate:

- **#224** (`ea76bf0`, ollama.rs): model detection switched from a filesystem scan to `GET /api/tags`. No `CLAUDE.md` documents the detection mechanism.
- **#223** (`49014ee`, main.rs): changed the `kwaainet status` shard hint text. No `CLAUDE.md` quotes that hint.
- **#225**/**#226** (`7163508`, `e7c581d`, shard_cmd.rs/main.rs): auto-pin messaging and `config set` refusing while the daemon/shard is running. `projects/kwaai-compute/CLAUDE.md`'s "Shard serve command" example uses explicit `--start-block`/`--blocks` flags and isn't affected; no file documents `config set`'s locking semantics for this to invalidate.
- **#222** (`76ab7e1`, kwaai-inference/lib.rs): reinstall hint changed `curl | bash` → `curl | sh`. No `CLAUDE.md` quotes the old form; `README.md` already said `| sh`.
- **#221** (`5504c76`, installer/README): musl-CUDA-fallback fix, cargo-env sourcing. No `CLAUDE.md` describes installer internals at this level.
- **#220** (`38c453c`, root `CLAUDE.md`): added the "cap a pass at 5 PRs" rule directly to `CLAUDE.md` — self-consistent, not a claim about code.

**Nothing found.** None of this window's commits made a memory-file claim false.

## Accretion — what could be removed and why

1. **Internal contradiction in `projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md`.** §4b is headed *"Reflection — proposed, not built"*, but §6, later in the same file, is headed *"The reflection half — implemented"* and names this exact nightly job (routine `trig_01TuA53doMcSPCTNNNyqJEuc`, `0 10 * * *` UTC). §4b's heading is now contradicted by §6 two pages later — one line fix (e.g. "proposed, not built" → "proposed; built, see §6").

2. **Un-pruned example the same plan doc already flagged, still live.** That plan (written 2026-09-01) cites as its accretion example: *"The D6 rebuild command in `projects/kwaai-knowledge/CLAUDE.md` carries three hardcoded p2p peer IDs; nothing has ever removed anything from that file."* Verified still true today: the same three IDs (`12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs`, `...LMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE`, `...DyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd`) appear twice in `projects/kwaai-knowledge/CLAUDE.md` (the rebuild command at lines 32–34 and the machine table at lines 98–106), and a third time in `projects/kwaai-compute/CLAUDE.md`'s machine table. Three copies of the same infrastructure fact, no single source of truth (the plan itself proposed the glossary as that anchor — hasn't happened). Not verifiable as *wrong* from this checkout (no DHT access to confirm the peers are still live), only as duplicated and un-anchored.

3. **Absolute path outside the repo.** `projects/kwaai-compute/CLAUDE.md` points to a plan at `~/.claude/plans/cached-jingling-creek.md`. That path does not exist in this checkout/environment, and being outside the repo it's invisible to `check_memory_integrity.py` — it can go stale with no signal at all. Unsure whether it still exists on the machine that wrote it; worth confirming, or moving the plan under `projects/kwaai-compute/plans/` if it's meant to stay valid.

## Nothing found

No broken references (mechanical). No memory-file claim invalidated by the last 36 hours of commits (semantic).
