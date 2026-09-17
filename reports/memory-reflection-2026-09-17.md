# Memory reflection — 2026-09-17

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exit 0. 0 broken references, 46 warnings across 48 files.

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

Nearly all warnings are plan documents citing not-yet-built ("proposed") files/functions, which is expected for plan docs. Two were followed up on below because they read as claims about present reality rather than proposals: the `RAGPerformanceReport` status line (Semantic) and the `config.rs:1059` line-number drift (harmless — `announce_state` still exists, just 16 lines further down; not worth a separate entry).

## Semantic — claims now false, with file, line, and the commit responsible

Two commits landed in the last 36 hours: `78d303d` (fix(kad): find a NATed peer from the k-bucket, not a patched peerstore, #216) and `ec3dc24` (release v0.7.1, version-bump only). `78d303d` removed `Behaviour::set_peerstore_addresses`, `refresh_peerstore`, `peer_listen_addrs`, `vouchable_listen_addrs`, reverted `libp2p-kad.patch` to the `#167` delta, and renamed `find_node_peerstore.rs` → `find_node_circuit.rs`.

Checked every CLAUDE.md and GLOSSARY.md for references to those symbols, the old test file name, the peerstore patch mechanism, or the `v0.7.0`/`v0.7.1` version strings: no hits. None of the twelve CLAUDE.md files or GLOSSARY.md describe kad/peerstore internals at that level of detail — `core/crates/kwaai-p2p/CLAUDE.md` and `projects/kwaai-network/CLAUDE.md` only name source files, not their internals.

**Nothing found in this section for the 36-hour window.** No memory-file claim was made false by either commit.

## Accretion — what could be removed and why

1. **`core/crates/kwaai-storage/CLAUDE.md` — wrong, not just stale.** Lines 3–4 and the `src/vectors.rs` table row describe the crate as "multi-tenant homomorphic-encrypted vector storage" / "Homomorphic vector search." Verified against `core/crates/kwaai-storage/src/lib.rs:1-6`, which states the crate "deliberately knows nothing about encryption: vectors are opaque `Vec<f32>`... a client that seals them before upload gets a host that cannot tell the difference," and against `src/vectors.rs`, which implements plain cosine-similarity HNSW search with no encryption code. The sibling `projects/kwaai-storage/CLAUDE.md` already has it right: "vectors are plaintext f32 today — encryption is planned, see roadmap.md" and "The PHE (Partial Homomorphic Encryption) service runs as a **separate process** (separate repo)." The crate-level file should be brought in line with the project-level one.

2. **`projects/kwaai-network/CLAUDE.md` — "in progress" item has shipped.** Line 63 lists "P2P relay routing for inference (route through p2p network instead of direct TCP)" under **In progress / planned**, and line 33 says "Fix needed: p2p relay routing for inference requests." But `core/crates/kwaai-cli/src/ollama_proxy.rs` (present, with a full doc comment describing `p2p://PEER_ID` resolution and protocol auto-negotiation) and `inference_mux.rs` implement exactly this, and it's wired into `rag_cmd.rs` and `shard_cmd.rs`. `projects/kwaai-knowledge/CLAUDE.md`'s own D6 rebuild command already uses `--inference-urls "p2p://12D3KooW...` as working infrastructure, not a plan. The v0.7.1 release notes (commit `ec3dc24`) describe "remote inference from this 0.7.1 build to that node, via `mux://<peer-id>`... 113.8/112.5/111.5 tok/s" as a verified capability. Three independent sources agree this shipped; the "in progress" framing in this one file is stale. (The file these functions first appear at in `git log` is `c3359b6`, a repo-restructuring squash-merge, so that specific SHA is not necessarily true origin — but that doesn't change that the feature is live today.)

3. **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — "Status: code complete, not yet committed" for the redb→rusqlite WAL migration. Verified: `core/crates/kwaai-rag/Cargo.toml` depends on `rusqlite` (bundled), no `redb` dependency remains, and the only remaining `redb` references in `core/crates/kwaai-rag/src` are legacy-store detection code (`meta_store.rs`, `graph.rs`) that tells a user to run `rag rebuild` to migrate — i.e. the migration is done, not pending. It landed at `c3359b6` (Aug 31 2026), well before this plan doc's own "not yet committed" line was presumably left unedited. Flagged by the checker as a status claim to recheck; recheck confirms it should read "committed."

4. **Duplicated hardcoded peer IDs.** The same three metro-machine peer-ID rows (`12D3KooWCzuh...`, `12D3KooWLMiz...`, `12D3KooWDyPJ...`) are hand-copied into both `projects/kwaai-compute/CLAUDE.md` (lines 32-34) and `projects/kwaai-knowledge/CLAUDE.md` (lines 98-100), plus the example commands on lines 36 and 106 respectively. Two independent copies of hardware-specific literals will drift the next time a machine is replaced (as `metro-linux`'s GPU already was once, per the network file's own "swapped from A6000 2026-08" note). Not a false claim today, but a duplication that removes any single point to update — worth consolidating to one source (e.g. the network file, which already owns "Metro machines" as a section) with the other two referencing it.

5. **`projects/kwaai-compute/CLAUDE.md:75`** references `plan: ~/.claude/plans/cached-jingling-creek.md` — an absolute path under one machine's home directory, outside the repo and unresolvable by any other engineer or agent. Candidate for removal or replacement with a repo-tracked plan path.

6. **Not independently verified, flagging as unsure rather than asserting:** `projects/kwaai-network/CLAUDE.md:31-32` states `metro-linux`/`metro-win` DNS "resolves to 192.168.1.1 (router)" and that `metro-linux` was "swapped from A6000 2026-08." These are point-in-time infrastructure notes I have no way to confirm or refute from the repo alone — would need to check current DNS behavior on those hosts or ask whoever administers them. Not reporting as stale, just noting it's aged and un-checkable from here.

## Nothing found

No memory-file claim was invalidated by either of the two commits landed in the last 36 hours (`78d303d`, `ec3dc24`) — see Semantic section above.
