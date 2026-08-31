# TODO

## Build

- [ ] **Runtime CUDA binding — single binary for all machines** — Currently we ship separate CPU and CUDA binaries, requiring GPU detection in the installer and updater. The fundamental problem: a CUDA-linked binary fails to start on machines without CUDA libs because the dynamic linker resolves symbols at load time, before `main()` runs. Fix: use `libloading` to `dlopen` CUDA libs lazily at first inference call rather than linking at compile time. The binary starts on any machine; if `libcuda.so` / `libcublas.so` are found at runtime, GPU is used — otherwise CPU falls back silently. This is how PyTorch/TensorFlow ship a single binary. Trade-off: every CUDA call must go through dynamically-resolved function pointers rather than direct symbol references; candle's `cuda` feature assumes compile-time linkage so this requires wrapping the CUDA backend. A lighter middle ground: a thin launcher binary that `exec`s `kwaainet-cuda` or `kwaainet-cpu` after GPU detection — still two underlying binaries but the user only ever touches one entry point. Both options eliminate the installer timing gap (CUDA build takes ~90 min in CI, fast path publishes immediately).

- [ ] **Release build takes ~1.5 hours** — `[profile.release]` in `core/Cargo.toml` uses `lto = true` (fat LTO) + `codegen-units = 1`. Switch to `lto = "thin"`: thin LTO is parallel (5–10× faster link) with <5% runtime impact for this workload (I/O-bound: network, Ollama calls, redb). The `[profile.ci]` profile is already fast (`lto = false`, `codegen-units = 16`) and is unaffected.

## Housekeeping

- [ ] **Finish moving `~/Source` to `/Volumes/WD2`** — rsync copy already done and verified (checksums match, file counts match). Step remaining: `rm -rf ~/Source` then `ln -s /Volumes/WD2/Source ~/Source`. WD2 must be mounted before deleting the original. Frees ~49 GB on internal SSD.

## Installation

- [ ] **Bundle `p2pd` in release tarball** — `DAEMON_BINARY_PATH` is baked in at compile time (`env!("P2PD_PATH")`) pointing to the build output dir. When `kwaainet` is installed on a clean machine the path doesn't exist and the node fails to start. Fix options: (1) include `p2pd` alongside `kwaainet` in the release archive, (2) resolve at runtime by searching `~/.local/bin`, `/usr/local/bin`, same dir as `kwaainet` binary, then fall back to compile-time path.

- [ ] **p2pd crashes silently on Rocky Linux** — `kwaainet start` spawns p2pd but it dies before creating the Unix socket (`/tmp/kwaai-p2pd.sock`). kwaainet then fails with "Connection error: No such file or directory". The daemon's stderr is swallowed so the actual crash reason is invisible. Fix: (1) capture p2pd stderr for the first few seconds after spawn and log it, (2) add a startup health check that waits for the socket with a timeout and reports the daemon's exit code/stderr if it dies, (3) investigate whether the musl-static Go binary has runtime issues on Rocky Linux (glibc 2.28). Reported on Rocky Linux (metro@kwaai, peer `...5bZ251`).

- [ ] **`kwaainet update` fails in restricted environments (Kasm, containers)** — `install_update()` in `updater.rs` downloads the installer script to `std::env::temp_dir()` which may be read-only or noexec in containerized environments like Kasm workspaces. The cargo-dist installer also uses `mktemp` inside `~/.cargo/bin/` for atomic writes, which fails if that dir has restrictive permissions. Fix: (1) respect `TMPDIR` env var in `updater.rs`, fall back to `~/.kwaainet/tmp/` if system temp is unwritable, (2) handle permission errors gracefully with a helpful message suggesting `TMPDIR=~/tmp kwaainet update`, (3) consider adding `--install-dir` flag to `kwaainet update` for non-standard layouts. Reported on Rocky Linux (stale binaries in `/home/linuxbrew/`, `~/.local/bin/`, `/usr/local/bin/` shadowing `~/.cargo/bin/`) and Kasm (permission denied on `mktemp` in `~/.cargo/bin/`).

## map.kwaai.ai

The map moved to its own repo: https://github.com/Kwaai-AI-Lab/KwaaiNetMap

v1 — the Python service that serves `/api/v1/state` — is on `main` there. v2,
the Rust API and React SPA that used to be `core/crates/map-server` and
`apps/map` here, is parked on the `map-v2` branch along with the backlog that
used to fill this section.

## Bootstrap Server v2 — Rust Port & Radical Reimagination

> Full requirements: `docs/BOOTSTRAP_SERVER_V2_REQUIREMENTS.md`
> Replaces: OpenAI-Petal `docker/kwaainet_bootstrap/` (50-line shell wrapper around `petals.cli.run_dht`)
> New crate: `core/crates/kwaai-bootstrap/`

### Phase 1 — Drop-in Rust replacement
- [ ] **New crate `kwaai-bootstrap`** — `core/crates/kwaai-bootstrap/`; add to workspace; CLI: `kwaainet bootstrap serve / keygen / status`.
- [ ] **Ed25519 key management** — replace RSA-2048 with Ed25519; same libp2p peer ID derivation (`SHA256(protobuf(pubkey))`); key stored at `~/.kwaainet/bootstrap-identity.bin`.
- [ ] **Existing peer ID compatibility** — wrap existing RSA keys or run graceful handoff window so established nodes do not need config changes.
- [ ] **Kademlia DHT node** — reuse `kwaai-hivemind-dht`; persist routing table to SQLite; survives restart in < 2 s.
- [ ] **Health endpoint** — `GET /health → { status, peers_known, uptime_secs }`; Prometheus metrics on `:9090`.
- [ ] **Docker image + deploy** — `Dockerfile.bootstrap`, add to `deploy-map.yml` or separate `deploy-bootstrap.yml`.
- [ ] **Verify drop-in** — existing `kwaainet` nodes connect without config change after DNS swap.

### Phase 2 — Privacy layer
- [ ] **PoW challenge** — adaptive difficulty (target 50 ms solve on modern CPU); stateless verify (recompute from timestamp epoch); no heap alloc on bad requests.
- [ ] **TrustedNodeVC fast-lane** — peers with valid VC skip PoW; verified against trust registry signature, no live RPC.
- [ ] **Encrypted introduction protocol** — ephemeral X25519 key exchange; response encrypted to ephemeral pubkey; bootstrap never logs joining IPs; `intent_hash = SHA256(model_prefix)`.
- [ ] **Signed peer-list responses** — Ed25519 signature over response; peers verify before using list; protects against BGP hijack / MITM.
- [ ] **No persistent IP logging** — enforce in code (`log_joins = false` default); document in privacy policy.
- [ ] **Rate limiting at socket layer** — 10 introductions/IP/min, 1000/IP/hr; enforced before any heap allocation.

### Phase 3 — Federation
- [ ] **`_kwaai.bootstrap.nodes` DHT key** — community bootstrap nodes self-register; existing nodes auto-discover via DHT crawl.
- [ ] **`BootstrapOperatorVC` issuance** — `kwaainet bootstrap register --vc <path>` submits to summit-server; returns `BootstrapAdmissionVC`.
- [ ] **Federation API** — `GET /api/federation/nodes`, `POST /api/federation/register`, `DELETE /api/federation/nodes/:peer_id` (Kwaai admin).
- [ ] **VC revocation** — revoked `BootstrapAdmissionVC` causes nodes to stop using that bootstrap within 60 s (next DHT crawl).
- [ ] **Geographic distribution** — deploy Kwaai-operated nodes in US-West, US-East, EU-West, AP-Southeast; anycast DNS (`bootstrap.kwaai.ai`).
- [ ] **Community capacity target** — design for ≥ 50% of introductions served by community nodes.

### Phase 4 — Peer cache gossip (bootstrap-free rejoin)
- [ ] **Peer cache gossip protocol** — nodes broadcast signed peer-list snapshot to 3 random peers every 5 min; receivers merge, keep 100 most recent entries.
- [ ] **Signed cache entries** — originating peer signs; receiver verifies before merging; wire format MessagePack.
- [ ] **Bootstrap-free rejoin** — on restart, attempt cached peers before contacting bootstrap; only truly new nodes require bootstrap.
- [ ] **Cache persistence** — store peer cache at `~/.kwaainet/peer-cache.json`; survives node restart.
- [ ] **Deprecate always-bootstrap-on-restart** — update `kwaainet start` to try peer cache first; bootstrap is fallback.

---

## Inference Throughput — Path to Conversational Speed

Current distributed inference runs at ~0.17 tok/s (2-hop chain) vs ~3 tok/s local. Target: 10+ tok/s. The bottleneck is **compute time per hop** (~500ms for 24 blocks), not network (~10ms RTT) or serialization (~8KB hidden state at F16). Improvements below are ordered by impact and dependency.

### Tier 1 — Instrumentation (prerequisite for everything else)

- [ ] **Per-hop timing in `handle_inference_request()`** — Add `Instant::now()` around the forward call in `block_rpc.rs:handle_inference_request()` and log `forward_ms`, `serialize_ms`, `total_ms`. Without this we can't distinguish compute vs overhead. Also add timing in `forward_through_chain()` to measure per-hop wall time from coordinator's perspective (includes network RTT). Output: `tracing::info!` with structured fields so operators can diagnose their nodes.

- [ ] **`kwaainet shard run --stats`** — Print per-token timing summary after generation: prefill time, average decode time, tok/s, per-hop latency breakdown. Reuses the instrumentation above. Changes to `shard_cmd.rs` (`cmd_shard_run`), `cli.rs` (add `--stats` flag to `ShardRunArgs`).

### Tier 2 — Reduce per-block compute time

- [ ] **Profile individual block timing** — Add per-block `Instant` in `run_blocks()` (`shard.rs:574`). Identify if specific blocks (attention, MLP, RoPE) are disproportionately slow. Check for CPU fallback in candle Metal backend (some ops may silently fall back to CPU, causing GPU→CPU→GPU round-trips).

- [ ] **Fused attention kernel** — candle's default attention is separate matmul + scale + mask + softmax + matmul. Investigate `candle_flash_attn` or a custom Metal kernel for fused scaled-dot-product attention. Expected 2-3x speedup on attention-bound blocks.

- [ ] **Reduce KV-cache mutex contention** — `run_blocks()` holds `sessions.lock()` for the entire block iteration (`shard.rs:580`). For 24 blocks that's one long critical section. Consider per-session `RwLock` or moving session lookup outside the hot loop.

### Tier 3 — Pipeline tokens across hops (~2-3x)

- [ ] **Overlapped pipeline** — While Peer B processes token N (blocks 24–31), send token N+1 to Peer A (blocks 0–23). Both peers work in parallel instead of sequentially. Requires: (1) coordinator sends token N+1 to Peer A before receiving token N's logits from Peer B, (2) Peer A can start forward pass while previous token's hidden states are still in flight to Peer B. Changes to `forward_through_chain()` in `shard_cmd.rs` — replace sequential loop with async pipeline using `tokio::spawn` per hop. The inference loop in `cmd_shard_run()` would issue the next token's embedding call concurrently with the current token's tail hop.

### Tier 4 — Speculative decoding (~2-4x)

- [ ] **Draft-model speculation** — First peer generates K candidate tokens using argmax (no sampling) after its forward pass, sends all K hidden states in one batch to the next peer. The final peer validates candidates against its own forward pass and accepts the longest correct prefix. Amortizes per-token network cost by K. Requires: new `PayloadType::SpeculativeBatch` in `block_rpc.rs`, modified `forward_through_chain()` to handle batch validation, and rollback logic when candidates are rejected.

### Tier 5 — Serialization and protocol optimization

- [ ] **Zero-copy F16 tensor transfer** — Replace per-element `to_le_bytes()` iteration in `tensor_to_f16_bytes()` (`block_rpc.rs:111`) with unsafe `slice::from_raw_parts` transmute. Saves ~0.3ms per hop per token. Also applies to `f16_bytes_to_tensor()`.

- [ ] **Binary protocol** — Replace msgpack named fields with positional binary encoding (fixed header: session_id u64 + seq_pos u32 + payload_type u8 + shape_len u8 + shape bytes + data bytes). Saves ~80-120 bytes overhead per message and eliminates serde dispatch. Consider only if Tier 1 instrumentation shows serialization > 5% of per-token time.

---

## Benchmark — Realistic Shard Throughput

- [x] **Rewrite `kwaainet benchmark` to use TransformerShard** — ✅ Done in v0.3.33. Now uses `TransformerShard::forward_full()` (same path as `shard run --local`). Reports prefill + decode tok/s separately. Defaults to CPU (candle Metal is 650x slower for decode). `--gpu` flag to opt in.

---

## Metal GPU Performance — 650x Decode Slowdown

> Full analysis: [`docs/METAL_PERFORMANCE_ANALYSIS.md`](METAL_PERFORMANCE_ANALYSIS.md)

Metal decode is 130s/token while CPU is 0.2s/token. Prefill is the opposite (Metal 23x faster). Root cause: candle 0.8.4's Metal backend launches 23 separate GPU kernels per transformer block per token — for single-token decode the kernel launch overhead (~100µs each) dwarfs the actual compute (~1.6µs per matmul). PyTorch's Metal backend fuses these into ~3-4 kernels via `scaled_dot_product_attention`. Tiered fix plan:

- [ ] **Tier 0: Diagnostic instrumentation** — Add per-block and per-op timing inside `ShardBlock::forward()` and GPU→CPU transfer timing. Determine whether the 130s is in the forward pass or the `to_device(CPU)` pipeline stall. (1 day)

- [ ] **Tier 1: Eliminate GPU→CPU round-trip** — Do argmax/sampling on GPU instead of transferring [1,1,128000] logits to CPU every token. Only transfer the single u32 token ID back. Could eliminate pipeline stall entirely. (1 day)

- [ ] **Tier 2: Hybrid Metal prefill + CPU decode** — Use Metal for prefill (23x faster), then transfer hidden states to CPU and decode on CPU (4.8 tok/s). Best of both worlds with minimal code change. (1 day)

- [ ] **Tier 3: Fused Metal kernels** — Combine QKV projection (3→1 kernel), RoPE (8→1), attention (6→1), and in-place KV-cache (eliminate `Tensor::cat` allocation+copy). Reduces from ~23 to ~8 kernel launches per block. (1 week)

- [ ] **Tier 4: Upgrade candle or custom MPS kernels** — Check if candle 0.9+ has Metal decode fixes. Write custom Metal shading language kernels for the attention hot path. (1-2 weeks)

- [ ] **Tier 5: MLX backend** — Apple's MLX framework is purpose-built for Apple Silicon (unified memory, lazy eval, automatic fusion). Reports ~30-40 tok/s Llama 8B on M2 Pro. Research spike complete: mlx-rs v0.25.3 has all needed ops (SafeTensors, RMSNorm, RoPE, Attention, KV-cache, SiLU). Requires porting ~200 line Python Llama to Rust, adding `DeviceType::Mlx` variant, and `brew install mlx` as system dep. Feature-gated: `cfg(target_os = "macos")`. See full plan: [`docs/MLX_BACKEND_PLAN.md`](MLX_BACKEND_PLAN.md). (1-2 weeks)

---

## Bug Reporting — `kwaainet bug`

- [ ] **`kwaainet bug "comment"`** — create a GitHub issue with node diagnostics. Bundles user comment, version/OS/arch, peer ID, node name, and last 100 lines of `kwaainet.log` into a markdown issue body with `<details>` log block. Auth via `GITHUB_TOKEN` env var or `--token` flag. Supports `--dry-run` and `--log-lines N`. Config opt-in: `bug_reporting: true` (default until v1.0, `kwaainet config --set bug_reporting false` to disable). New files: `bug.rs` handler; changes to `cli.rs`, `config.rs`, `main.rs`. No new deps (reqwest + serde_json already available). See plan: `.claude/plans/glistening-seeking-dijkstra.md`.

---

## Storage Fabric — Multi-Tenant Eve Nodes

> Full plan: [`docs/storage-fabric-plan.md`](storage-fabric-plan.md)
> PHE repo: `github.com/Kwaai-AI-Lab/PHE`
> Benchmarking rationale: `github.com/Kwaai-AI-Lab/vector_dbs_benchmarking`

### Phase 0 — Eve operator CLI (KwaaiNet repo)

- [x] **`kwaainet storage init` command** — provision local PostgreSQL+pgvector, create database, save `StorageConfig` to `~/.kwaainet/config.yaml`. New file: `storage.rs`. Detects PG via Homebrew paths, validates pgvector version match, idempotent.
- [x] **`StorageConfig` in config.rs** — add `storage: Option<StorageConfig>` to `KwaaiNetConfig` with fields: `pg_url`, `data_path`, `capacity_gb`, `pg_port` (default 5433).
- [x] **`kwaainet storage status/start/stop/destroy`** — PG lifecycle management commands. `status` shows DB size, tenant count (vector tables), VPK health. `destroy` with confirmation prompt.
- [ ] **PG+PHE lifecycle in `kwaainet start/stop`** — when `storage` config exists, start/stop PostgreSQL and PHE alongside p2pd. Wire into `node.rs` daemon lifecycle.
- [x] **`Storage(StorageArgs)` in cli.rs + main.rs** — add `Storage` command variant with `Init/Status/Start/Stop/Destroy` subcommands.

### Phase 1 — Database migration for multi-tenancy (PHE repo)

- [ ] **`migrations/20260404_004_multi_tenancy.sql`** — create `tenants` table (tenant_id UUID, peer_id, display_name, capacity_limit_mb, status, timestamps). Add nullable `tenant_id` column to `documents`, `index_mapping`, `audit_log`, `encryption_keys`.
- [ ] **Default tenant bootstrap in `database/mod.rs`** — on first startup after migration, auto-create a "default" tenant and adopt existing NULL-tenant rows. Ensures seamless upgrade from single-tenant.
- [ ] **`TenantQueries` in `database/queries.rs`** — CRUD operations for tenants table: create, get, list, update status, soft-delete.

### Benchmark — Flat vs HNSW crossover point (KwaaiNet repo)

- [ ] **PGVector flat vs HNSW benchmark** — Measure query latency (P50/P95) and insert throughput for flat scan vs HNSW index at 1k, 5k, 10k, 25k, 50k vectors (384-dim, f32) on consumer hardware. When Bob shards a 1GB KB across 100 Eves, each Eve holds ~2,500 vectors — HNSW warm-up penalty likely makes flat search faster at that scale. Find the exact crossover point. Use `kwaai-storage` crate's `VectorStore` directly. Compare with benchmarking study results.
- [ ] **Auto-index threshold in VectorStore** — After benchmark, change `ensure_table()` to create tables without HNSW index by default. Add background task or trigger to create HNSW index when vector count exceeds threshold (e.g., 10k). Make threshold configurable per-tenant or globally.

### Phase 2 — PGVector Eve storage backend (PHE repo)

- [ ] **`VectorStorage` trait in `vectordb/client.rs`** — extract async trait with `upload_vectors`, `search`, `delete_vectors`, `health_check`, `clear`, `count` methods. Rename `VectorDBClient` → `InMemoryVectorStorage`.
- [ ] **`PgVectorClient` in `vectordb/pgvector.rs`** — PGVector-backed storage. Per-tenant table (`eve_vectors_{tenant_hex8}`) with HNSW index (m=16, ef_construction=64). Methods: batch INSERT, `ORDER BY embedding <=> $1` search, `pg_total_relation_size()` capacity tracking.
- [ ] **`EveStorageConfig` in `config.rs`** — backend enum (InMemory/PgVector), `pgvector_url`, HNSW params with defaults.
- [ ] **Update `ShardManager` in `shard/manager.rs`** — change `Shard.client` from `Arc<VectorDBClient>` to `Arc<dyn VectorStorage>`. Accept factory fn or pre-built storage backends.
- [ ] **Add `pgvector = "0.4"` to `Cargo.toml`** — verify compatibility with existing `tokio-postgres 0.7`.

### Phase 3 — Tenant-aware VPK core (PHE repo)

- [ ] **`TenantManager` in `tenant/manager.rs`** — create, delete (soft), list, get, stats, check_capacity. Uses `TenantQueries`.
- [ ] **`TenantContext` in `vpk/core.rs`** — per-tenant crypto state: key_id, DimensionalScrambling, EncryptionPipeline, IndexScrambler, ShardManager. Lazy-loaded via `ensure_tenant_context(tenant_id)`.
- [ ] **Thread `tenant_id` through VPK methods** — `upload_document`, `upload_documents`, `query` all gain `tenant_id: Uuid` parameter. `VPK` struct gains `tenant_manager` and `tenant_contexts: HashMap<Uuid, TenantContext>`.
- [ ] **Tenant-scope `database/queries.rs`** — all query structs gain `WHERE tenant_id = $N` clauses. Existing parameterless methods route to default tenant.
- [ ] **Per-tenant index mapping caches** — `index/mapping.rs` caches become `HashMap<Uuid, HashMap<i64, i64>>`.

### Phase 4 — Eve HTTP API for remote Bobs (PHE repo)

- [ ] **Tenant CRUD routes in `api/mod.rs`** — `POST /api/tenants` (create), `GET /api/tenants` (list), `GET /api/tenants/{id}` (info+stats), `DELETE /api/tenants/{id}` (soft-delete).
- [ ] **Per-tenant vector routes** — `POST /api/tenants/{id}/vectors` (upload encrypted), `POST /api/tenants/{id}/search` (search encrypted), `DELETE /api/tenants/{id}/vectors` (delete). All endpoints accept already-encrypted data.
- [ ] **Enhanced `/api/health` response** — add `tenant_count`, `total_vectors`, `capacity_gb_total`, `capacity_gb_available`, `version`, `peer_id` fields. Must be compatible with KwaaiNet's `check_vpk_health()` in `node.rs`.
- [ ] **Backward-compatible legacy routes** — existing `/api/upload`, `/api/search` continue to work via default tenant.
- [ ] **Tenant auth (Phase 1)** — tenant creation returns `tenant_secret` (UUID). Subsequent requests include `Authorization: Bearer <tenant_secret>`. All endpoints accept optional auth header. `peer_id` recorded in audit log.

### Phase 5 — Remote shard client (PHE repo)

- [ ] **`RemoteEveClient` in `vectordb/remote.rs`** — implements `VectorStorage` over HTTP. Maps trait methods to Eve's tenant API: `upload_vectors` → `POST /api/tenants/{id}/vectors`, `search` → `POST /api/tenants/{id}/search`, etc.
- [ ] **Update `ShardEndpoint` in `config.rs`** — `endpoint` now carries remote Eve URL + tenant_id for production use.
- [ ] **Bob's workflow integration** — discover Eves via DHT → create tenants → configure shard_configs → encrypt locally → fan out to remote Eves → merge results.

### Phase 6 — KwaaiNet CLI updates (KwaaiNet repo)

- [ ] **`TenantAction` subcommands in `cli.rs`** — `kwaainet vpk tenant create/list/info/delete` with `--eve-endpoint` flag (omitted = local, provided = remote Eve).
- [ ] **Tenant CLI handlers in `vpk.rs`** — `tenant_create()`, `tenant_list()`, `tenant_info()`, `tenant_delete()`. HTTP calls to Eve tenant API, formatted with `display.rs`.
- [ ] **Enhanced `vpk status`** — show per-tenant breakdown from enhanced health endpoint.
- [ ] **DHT advertisement update in `node.rs`** — add `total_vectors: u64` to `VpkInfo`, emit in `to_msgpack_value()`.

### Scientific Experiments — Fabric Benefits Beyond Latency

> Plan: `.claude/plans/our-recent-finding-is-concurrent-lampson.md`
> Context: VPK shard bench (K=2, K=11) proved WAN sharding gives no latency benefit.
> These experiments measure the three *other* benefits of a multi-node fabric.

#### Experiment 1 — Elastic Capacity (H1)

**Hypothesis:** Total storable vectors scales linearly with N Eves; a Bob can store a corpus
larger than any single Eve's quota with no data loss and no change to search accuracy.

- [ ] **`vpk_capacity_bench.rs`** — new file: `run_capacity_bench(args)` fills each discovered Eve to its declared limit (batch uploads until rejection), records accepted/rejected counts and effective utilisation.
- [ ] **`CapacityBenchArgs` struct** — fields: `eve_peer_ids`, `batch_size_mb`, `dimensions: u32` (default 384), `json_output: bool`.
- [ ] **Wire into `vpk.rs`** — add `CapacityBench(CapacityBenchArgs)` to `VpkAction`; add `kwaainet vpk capacity-bench` to `cli.rs`; dispatch in `main.rs`.
- [ ] **Run experiment** — discover live Eves, fill N=1/2/4/8, plot total accepted vs N Eves.
- [ ] **Publish results** — `docs/vpk-capacity-bench/README.md` with capacity-vs-N table, effective utilisation %, upload throughput per Eve.

#### Experiment 2 — Read Availability Under Node Failure (H2)

**Hypothesis:** R=2 replication delivers 100% correct search results when one Eve is
unreachable, with ≤5% latency overhead (parallel fan-out, first-response-wins).

- [ ] **`rpc_upload_vectors_replicated()` in `storage_rpc.rs`** — fan-out write to `peers: &[PeerId]`, return `Ok` if ≥1 succeeds.
- [ ] **`rpc_search_vectors_with_fallback()` in `storage_rpc.rs`** — race all peers with `tokio::select!`, return first successful response; record which peer responded.
- [ ] **`kwaainet vpk replicate` subcommand** — copies all vectors for a `--kb-id` from `--from` PeerId to `--to` PeerId via chunked search+upload.
- [ ] **`kwaainet vpk failover-search` subcommand** — searches with automatic fallback across `--eve-peer-ids`, prints responding peer and latency delta.
- [ ] **`tests/vpk-redundancy-test.sh`** — 2-machine script: upload to R=2, kill primary, run 100 queries, verify 0 failures.
- [ ] **Run experiment** — measure success rate (R=1 vs R=2 under failure) and latency (healthy vs degraded).
- [ ] **Publish results** — `docs/vpk-redundancy-bench/README.md`.

#### Experiment 3 — Data Durability via Async Backup (H3)

**Hypothesis:** Periodic backup to a cold Eve enables full corpus recovery after permanent
primary destruction, with data-loss window bounded to the backup interval.

- [ ] **`GET /api/tenants/{id}/export` in `kwaai-storage/src/api.rs`** — streams all `(doc_id, vector)` pairs as msgpack chunks (1000 vectors per chunk); memory-bounded.
- [ ] **`vpk_backup.rs`** — new file: `backup(kb_id, from_peer, to_peer)` exports via relay + uploads to backup Eve; `restore(kb_id, from_peer, to_peer)` exports from backup, uploads to target.
- [ ] **`kwaainet vpk backup` and `kwaainet vpk restore` subcommands** — wire into `vpk.rs`/`cli.rs`/`main.rs`.
- [ ] **`tests/vpk-backup-test.sh`** — upload 25K vectors, backup, upload 500 more (post-backup window), destroy primary, restore, measure recall vs pre-backup vs post-backup corpus.
- [ ] **Run experiment** — measure recovery time at 10K/25K/50K vectors, quantify data-loss window.
- [ ] **Publish results** — `docs/vpk-durability-bench/README.md`.

---

## Peer Reputation

- [ ] **Background peer discovery + ping loop** — Nodes only accumulate reputation data for bootstrap peers (passively, every 300 s via STORE RPC timing) and for inference peers actually used in `shard run`. All other network nodes remain invisible in the reputation list until a user happens to route inference through them. Fix: add a `tokio::spawn` background loop in `run_node()` that fires every ~30 minutes. Each cycle: (1) sends `FindRequest` for a sample of block keys (`{prefix}.0`, `{prefix}.N/2`, `{prefix}.N-1`) to bootstrap peers to get the current FoundDictionary of all serving peers, (2) for each discovered peer_id (excluding self) calls `call_unary_handler(&peer_id_bytes, "DHTProtocol.rpc_find", &probe_bytes)` with a 5 s timeout — p2pd handles routing via peer_id, no multiaddr needed, (3) records `PeerObservation { latency_ms, success }` in the local `ReputationStore`. The FoundDictionary parsing logic should be extracted from `shard_cmd.rs::discover_chain()` into a shared utility rather than duplicated. Frequency chosen to balance freshness vs. network load: 30 min gives ~48 passive pings/day per peer, well below any reasonable rate limit.

## RAG Application Stack

> Full implementation plan: `.claude/plans/loook-at-https-github-com-kwaai-ai-lab-k-parsed-breeze.md`
>
> Builds on top of the existing storage fabric (`kwaainet storage`, port 7432) and OpenAI-compatible
> inference API (`kwaainet shard api`, port 8080). Embedding via Ollama `nomic-embed-text` (768-dim).
> UI: minimal embedded HTML at `/` served by `kwaainet rag serve`, plus OpenWebUI as a production
> frontend pointing at port 9090 as a custom OpenAI base URL.
>
> **Critical constraint**: Eve stores only opaque `(i64, Vec<f32>)` tuples. All chunk text and
> metadata live locally on Bob at `~/.kwaainet/rag/<tenant_id>.redb` (redb sidecar).

### Phase 1 — Personal Knowledge Base (new crate `kwaai-rag`)

- [ ] **New crate `crates/kwaai-rag`** — library crate with no binary; add to workspace `[members]` and `[workspace.dependencies]`. Deps: `anyhow`, `serde`, `serde_json`, `tokio`, `reqwest`, `redb = "2"`, `uuid`, `tracing`, `sha2`, `base64`.

- [ ] **`chunker.rs`** — `ChunkConfig { chunk_size: 800, chunk_overlap: 200 }`, `Chunk { id: i64, text, doc_name, chunk_index, page_num? }`. `split_text()` sliding window over Unicode scalars. `chunk_id()` = truncate(SHA-256(doc_name + "::" + chunk_index), 8 bytes) → i64 — stable across re-ingest.

- [ ] **`embedder.rs`** — `EmbedClient { base_url, model: "nomic-embed-text" }`. `embed_one()` and `embed_batch()` POST to `localhost:11434/api/embed`. At serve-startup: test-embed `""`, assert `len == 768`, clear error if Ollama not running.

- [ ] **`meta_store.rs`** — redb sidecar at `~/.kwaainet/rag/<tenant_id>.redb`. Compound key = `tenant_uuid[16] + chunk_id_i64_le[8]` for single-file multi-tenant storage. `ChunkMeta { doc_name, chunk_index, text, page_num?, ingested_at }`. Methods: `put_chunks`, `get_chunks`, `list_docs`, `delete_doc` (returns `Vec<i64>` for Eve deletion).

- [ ] **`ingestion.rs`** — `ingest_text(cfg, doc_name, text, upload_fn, progress?)`. `upload_fn` closure is provided by the CLI caller (closes over `P2PClient + Eve PeerId`) — keeps `kwaai-rag` free of `kwaai-p2p-daemon` dependency. Batch size: 256 vectors per RPC call.

- [ ] **`retriever.rs`** — `retrieve(query, cfg, embed, meta, search_fn)`. `search_fn` closure same pattern as `upload_fn`. Returns `Vec<RetrievedChunk { chunk_meta, score, source_kb? }>`.

- [ ] **`prompt.rs`** — `build_rag_prompt()` and `build_chat_messages()` with numbered context blocks injected as system message. `max_context_chars: 6000` default (configurable via `--max-context` on `rag serve`).

- [ ] **`crates/kwaai-cli/src/rag_cmd.rs`** — subcommands wired to `kwaainet rag`:
  - `rag init --eve-peer-id <PEER> [--capacity-mb 2048] [--embed-model nomic-embed-text]` → `rpc_create_tenant(..vector_dimension:768..)` + MetaStore create + write `~/.kwaainet/rag/config.toml`. Dimension guard: before first upload verify tenant dimension == 768 via `rpc_list_tenants`.
  - `rag ingest <file>` → txt/md via `read_to_string`; PDF via `lopdf` (optional feature `rag-pdf`); calls `ingest_text` with progress bar.
  - `rag query <text> [--top-k 5]` → retrieve + display chunks, no LLM (debug tool).
  - `rag chat` → interactive REPL: retrieve → `build_chat_messages` → POST `localhost:8080/v1/chat/completions` → stream SSE.
  - `rag docs` → `meta_store.list_docs()`.
  - `rag delete-doc <name> [--yes]` → `rpc_delete_vectors(ids)` + `meta_store.delete_doc()`.
  - `rag serve [--port 9090] [--inference-url http://localhost:8080]` → starts `rag_api` HTTP server.

- [ ] **`crates/kwaai-cli/src/rag_api.rs`** — Axum HTTP server shared state `RagState { tenant_id, eve_peer, p2p_client, embed_client, meta_store, inference_base_url, top_k, http_client }`. Routes:
  - `GET /v1/models` → `"kwaai-rag"`
  - `POST /v1/chat/completions` → RAG pipeline: extract last user message → retrieve → `build_chat_messages` → proxy to shard API (transparent SSE streaming via reqwest; reuse pattern from `shard_api.rs:508-569`)
  - `POST /v1/completions` → same pipeline
  - `POST /api/ingest` → multipart file upload
  - `GET /api/docs` → list docs
  - `DELETE /api/docs/:name` → delete doc
  - `GET /` → `include_str!("../assets/rag_ui.html")` minimal chat UI

- [ ] **`config.rs`** — add `RagConfig { tenant_id: Option<Uuid>, eve_peer_id: Option<String>, embed_model: String, inference_url: String }` with `Default`; `pub rag: Option<RagConfig>` with `#[serde(default)]` in `KwaaiNetConfig`.

- [ ] **`cli.rs` + `main.rs`** — add `Rag(RagArgs)` to `Command` enum; `RagArgs`/`RagAction`; `mod rag_cmd; mod rag_api;` + dispatch arm.

- [ ] **`kwaai-storage/src/tenant.rs`** — add `vector_dimension: u32` to `TenantInfo` and `TenantRecord` (needed for dimension guard in `rag init`).

- [ ] **OpenWebUI integration** — document: run `kwaainet rag serve --port 9090`, then in OpenWebUI Settings → Connections add `http://localhost:9090` as custom OpenAI base URL; select model `kwaai-rag`. File upload via embedded UI or `kwaainet rag ingest`; OpenWebUI file uploads use its own pipeline, not KwaaiNet VPK.

### Phase 2 — Federated Multi-KB RAG

- [ ] **`crates/kwaai-rag/src/federation.rs`** — `advertise_kb(client, kb_id, tenant_id)` announces on DHT key `_kwaai.vpk.kb.<kb_id>` (SHA1-msgpack hashing, same as `vpk_dht_id()` pattern). `resolve_kb(client, kb_id) → Vec<KbEndpoint { eve_peer_id, tenant_id, owner_peer_id, display_name }>`.

- [ ] **P2P protocol `/kwaai/rag/1.0.0`** — Bob-to-Bob query relay. Request: `{ query_embedding: Vec<f32>, top_k: usize, tenant_id: Uuid, vc_token: Option<String> }`. Response: `{ results: Vec<(i64, f64)> }` — only IDs+scores, no chunk text (preserving Bob's privacy). Serving Bob validates optional VC from `kwaai-trust` before answering.

- [ ] **`rag_api.rs` federated fan-out** — `RagState` gains `Vec<KbEndpoint>`. `rag_chat_completions` fans out `retrieve()` across all endpoints concurrently via `tokio::join_all`, merge-sort by score, inject global top-K.

- [ ] **CLI additions** — `kwaainet rag kb-share --name <NAME>`, `kwaainet rag kb-list`, `--kb-ids` flag on `rag serve` for federated mode.

### Phase 2.5 — Document-Schema-Aware Ingestion

> Rationale: a flat PDF produces a uniform stream of chunks with no structural signal. A book has a
> Foreword (context-setting, low entity density), Chapters (primary content), an Index (entity name
> hints, near-zero prose), Footnotes (high specificity, easy to miss), and a Bibliography (relation
> hints). Treating them identically wastes LLM tokens on low-signal sections and misses structural
> metadata that improves entity extraction.

- [ ] **`doc_type.rs` — document type detection** — Detect `schema:Book`, `schema:ScholarlyArticle`,
  `schema:EmailMessage`, `schema:Report`, `schema:Legislation`, `schema:NewsArticle` from filename
  patterns, PDF metadata (`/Author`, `/Keywords`, `/Subject`), and structural heuristics (chapter
  heading density, reference list ratio, From:/To: lines). Return a `DocSchema` enum with known
  section roles per type (e.g. `Book` has `{Foreword, Preface, TableOfContents, Chapter, Appendix,
  Footnote, Bibliography, Index}`; `ScholarlyArticle` has `{Abstract, Introduction, Methods,
  Results, Discussion, References}`). Schema.org vocabulary as the canonical type names.

- [ ] **`section_tagger.rs` — structural section labelling** — Given a detected `DocSchema`, scan
  chunk text for structural markers (regex/heuristic): heading patterns (`Chapter \d+`, `CONTENTS`,
  `BIBLIOGRAPHY`, `INDEX`, numbered footnote markers, `Abstract`, `REFERENCES`). Tag each
  `Chunk` with a `section_type: Option<SectionType>` field and `section_index: Option<u32>` (chapter
  number, appendix letter, etc.). Store in chunk metadata in `meta_store.rs`.

- [ ] **`ingester.rs` — section-aware pipeline** — Before splitting text into chunks, call
  `section_tagger::label_sections()` to produce labelled segments. Pass `section_type` down through
  `split_text()` so each `Chunk` carries the label. Log section breakdown at ingest time:
  `"Detected schema:Book — 2 frontmatter, 18 chapters, 1 index, 1 bibliography"`.

- [ ] **Graph build section hints** — In `rag graph build`, use `section_type` to select the
  appropriate extraction prompt template:
  - `Chapter` → standard entity/relation extraction prompt
  - `Index` → entity-name-only extraction (no relations needed; each entry is a named entity hint)
  - `Bibliography` → person+work relation extraction (`{Person} authored {Work}`)
  - `Footnote` → high-precision extraction (small context, likely a specific fact)
  - `TableOfContents` / `Foreword` / `Preface` → skip entity extraction (low signal, high noise)
  Prompt templates live in a new `extract_prompts.rs` module; the build loop selects by
  `SectionType`.

- [ ] **`kwaainet rag ingest --doc-type <TYPE>`** — optional override flag (`book`, `paper`,
  `email`, `report`, `legislation`) so the user can force a schema when auto-detection is wrong.
  Auto-detected type is logged so users know what was inferred.

- [ ] **`kwaainet rag docs` — show section breakdown** — extend `list_docs()` output with a
  section summary column (e.g. `18 chapters, 1 index, 247 footnotes`) derived from stored
  `section_type` counts in `meta_store`.

### Phase 3 — GraphRAG + Reasoning Harnesses

- [ ] **`crates/kwaai-rag/src/graph.rs`** — `KgNode { id, label, node_type, chunk_ids }`, `KgEdge { source_id, target_id, relation }`, `KnowledgeGraph`. Built during ingestion via structured-output entity extraction (calls `shard_api` — no new services). Persisted as a third redb table in the MetaStore sidecar.

- [ ] **`crates/kwaai-rag/src/reasoning.rs`** — `ReasoningStrategy` enum: `Naive`, `GraphExpanded`, `ChainOfThought`, `Agentic { max_steps }`. Graph-augmented retrieval: top-K dense chunks → expand entity neighborhood (1–2 hops) → add neighborhood chunks to context.

- [ ] **`--strategy` flag on `kwaainet rag serve`** — `--strategy graph` enables GraphRAG; `--strategy cot` enables chain-of-thought prompting.

### Phase 4 — Dream RAG: continuous graph refinement (implemented Phase 1+2, see below)

- [ ] **Fix alias embedding gap** — `graph alias-scan --auto` merges abbreviation entities (e.g. TLSA, NEUM) into their canonical full-name nodes. After merge, the entity embedding is `"{canonical name}: description"` with no abbreviation in the vector, so queries containing "TLSA" miss the merged entity in graph search. M19 eval confirmed regression (54.3% vs 56.9% baseline). Fix: after `merge_entity_into()`, embed using `"{name} ({alias1}, {alias2}): description"` so abbreviation queries still resolve to the canonical node. Files: `kwaai-rag/src/graph.rs` (`merge_entity_into`), `kwaai-cli/src/rag_cmd.rs` (`cmd_alias_scan`).

- [ ] **Dream RAG quality gate** — add `kwaainet rag dream eval` subcommand that snapshots graph health score before a dream cycle and rolls back (restores from snapshot) if overall health drops >5% after the cycle. Prevents bad LLM completions from degrading the graph. Files: `kwaai-rag/src/dream.rs` (snapshot/restore), `kwaai-cli/src/cli.rs` + `rag_cmd.rs` (`dream eval` dispatch).

- [ ] **Dream RAG embed-model evaluation** — `kwaainet rag dream embed-eval` runs a before/after eval sweep comparing the current embed model against an alternative (e.g. mxbai-embed-large vs nomic-embed-text), re-embeds a sampled subset, measures retrieval quality delta, and adopts the new model with `--adopt` if it wins. Files: `kwaai-rag/src/dream.rs` or new `embed_eval.rs`.

---

## Networking

- [ ] **Fix relay fallback** — `metro@kwaai` (peer `...5bZ251`) connects via p2p-circuit relay through `76.91.214.120` instead of direct on configured public IP `75.141.127.202:8080`. Node should establish a direct connection. Investigate NAT traversal / port forwarding and `announceAddrs` config.
