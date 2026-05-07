# TODO

## Installation

- [ ] **Bundle `p2pd` in release tarball** — `DAEMON_BINARY_PATH` is baked in at compile time (`env!("P2PD_PATH")`) pointing to the build output dir. When `kwaainet` is installed on a clean machine the path doesn't exist and the node fails to start. Fix options: (1) include `p2pd` alongside `kwaainet` in the release archive, (2) resolve at runtime by searching `~/.local/bin`, `/usr/local/bin`, same dir as `kwaainet` binary, then fall back to compile-time path.

- [ ] **p2pd crashes silently on Rocky Linux** — `kwaainet start` spawns p2pd but it dies before creating the Unix socket (`/tmp/kwaai-p2pd.sock`). kwaainet then fails with "Connection error: No such file or directory". The daemon's stderr is swallowed so the actual crash reason is invisible. Fix: (1) capture p2pd stderr for the first few seconds after spawn and log it, (2) add a startup health check that waits for the socket with a timeout and reports the daemon's exit code/stderr if it dies, (3) investigate whether the musl-static Go binary has runtime issues on Rocky Linux (glibc 2.28). Reported on Rocky Linux (metro@kwaai, peer `...5bZ251`).

- [ ] **`kwaainet update` fails in restricted environments (Kasm, containers)** — `install_update()` in `updater.rs` downloads the installer script to `std::env::temp_dir()` which may be read-only or noexec in containerized environments like Kasm workspaces. The cargo-dist installer also uses `mktemp` inside `~/.cargo/bin/` for atomic writes, which fails if that dir has restrictive permissions. Fix: (1) respect `TMPDIR` env var in `updater.rs`, fall back to `~/.kwaainet/tmp/` if system temp is unwritable, (2) handle permission errors gracefully with a helpful message suggesting `TMPDIR=~/tmp kwaainet update`, (3) consider adding `--install-dir` flag to `kwaainet update` for non-standard layouts. Reported on Rocky Linux (stale binaries in `/home/linuxbrew/`, `~/.local/bin/`, `/usr/local/bin/` shadowing `~/.cargo/bin/`) and Kasm (permission denied on `mktemp` in `~/.cargo/bin/`).

## map.kwaai.ai — Public Web UI

> v1 shipped: DHT crawler (`core/crates/map-server`), React SPA (`apps/map`).
> The items below are grouped by phase and approximate complexity.

### Backend — map-server (`core/crates/map-server`)

#### Quick wins
- [ ] **Remove debug log spam** — `crawler.rs` still emits per-result `DEBUG` lines for every DHT response. Gate behind `tracing::trace!` or remove before production deploy.
- [ ] **Fix `-0.0` tokens/sec** — `cache.rs` sums an empty set of floats, producing `-0.0`. Guard with `if nodes.is_empty() { 0.0 } else { sum }`.
- [ ] **Rate-limit `/api/nodes`** — response can be large. Add `tower_http::limit` or cache the serialised JSON for 5 s.
- [ ] **CORS lockdown** — `ALLOWED_ORIGINS` env var exists but defaults to `*`. Set to `https://map.kwaai.ai` in the production Dockerfile / compose file.
- [ ] **Graceful shutdown** — handle `SIGTERM` so the server drains in-flight WebSocket connections before exit (important for zero-downtime deploys).

#### Crawler improvements
- [ ] **Accurate trust tier** — current `tier_from_vc_count(n)` is a rough proxy. Import `kwaai-trust::TrustScore::from_credentials()` and decode the `trust_attestations` field from the DHT value to compute a real score.
- [ ] **Version field** — decode `version` from the DHT map and expose it in `NodeEntry`. Needed for `map.kwaai.ai` to surface stale nodes (see MEMORY.md "Peer version visibility").
- [ ] **Crawl own local storage** — in addition to querying bootstrap peers, query the local `DHTStorage` (via p2pd) so the running node always appears even if bootstrap propagation lags.
- [ ] **Configurable bootstrap peers** — read `~/.kwaainet/config.yaml` at startup (or accept a `--config` flag) to inherit the user's `initial_peers` list rather than using only defaults.
- [ ] **Crawl all registered models dynamically** — `_petals.models` registry discovery is implemented but only runs once. Re-query every crawl cycle so newly registered models are picked up without a server restart.
- [ ] **Persist cache across restarts** — write the node cache to a small SQLite file (via `rusqlite`) so the map is not empty for the first 60 s after a restart.

#### API additions (v2)
- [ ] **`GET /api/nodes/:peer_id`** — individual node detail page (trust certs, uptime history, version).
- [ ] **`GET /api/coverage`** — block coverage bitmap as a compact JSON array; used by a future coverage heatmap widget.
- [ ] **`POST /api/v1/state`** — receive heartbeat pings from running nodes (the `health_monitoring.api_endpoint` already points here). Validate and upsert into cache so nodes are visible immediately on start rather than waiting for the 60 s crawl.
- [ ] **WebSocket auth** — currently `/api/live` is fully open. For v2 operator dashboard, add an optional `?token=` query param checked against a shared secret.

---

### Frontend — React SPA (`apps/map`)

#### Quick wins
- [ ] **Favicon** — `public/favicon.svg` is referenced in `index.html` but not created. Add an SVG version of the Kwaai tree logo.
- [ ] **Negative-zero display** — mirror the backend fix; also guard `tokens_per_sec.toFixed(1)` to show `0.0` not `-0.0`.
- [ ] **WebSocket reconnect indicator** — show a subtle "reconnecting…" badge in the hero stat bar when `connected === false`, instead of silently showing stale numbers.
- [ ] **Responsive nav** — mobile hamburger menu; nav links currently hidden on small screens.
- [ ] **Accessibility** — add `aria-label` to all icon-only buttons (copy, remove drive, CTAs). Run `axe` or Lighthouse audit.

#### HeroSection / NetworkGraph
- [ ] **Node tooltip on hover** — show peer name, trust tier, throughput, version in a floating tooltip when hovering a graph node.
- [ ] **Click-to-highlight** — clicking a node in the graph highlights its block range in a coverage bar below the counter chips.
- [ ] **Live pulse animation** — nodes with `throughput > 0` should visually pulse; currently the glow is drawn every frame. Implement a CSS animation driven by `throughput` magnitude.
- [ ] **Coverage bar** — a horizontal bar below the stats chips showing block 0–79 coloured by how many nodes cover each block (green = covered, red = gap). Uses `GET /api/coverage`.
- [ ] **Node count history sparkline** — tiny 24 h sparkline next to the node counter, fed by a rolling window stored in localStorage.

#### BenchmarkSection
- [ ] **WebGL fallback** — the CPU fallback is very slow on low-end machines. Implement a WebGL GEMM path using `twgl.js` as the middle tier between WebGPU and pure CPU.
- [ ] **Calibrate tps estimate** — the 128×128 GEMM → token/sec extrapolation is rough. Gather empirical data from known hardware (M2, RTX 4090, etc.) to fit a better conversion factor.
- [ ] **Storage: show browser quota vs disk quota** — `navigator.storage.estimate().quota` is typically 60 % of available disk space. Show both figures and clarify the difference to the user.
- [ ] **Persist benchmark results** — save to `localStorage` so the results panel is visible on return visits without re-running.
- [ ] **Share results** — "Share my score" button that generates a shareable URL with tps/storage params encoded (no server needed).

#### TrustGraphSection
- [ ] **Real endorsement edges** — `PeerEndorsementVC` relationships from `/api/nodes` should draw directed edges between nodes, not just adjacency edges. Decode the VC subject/issuer DIDs and map to peer IDs.
- [ ] **Tier filter** — checkbox row to show/hide nodes by tier (Unknown / Known / Verified / Trusted).
- [ ] **ToIP explainer** — add a collapsible "How it works" panel below the graph explaining the 4-layer ToIP stack in plain language, linking to `docs/WHITEPAPER.md`.

#### InstallSection
- [ ] **Live installer URL** — fetch latest release tag from GitHub API (`/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest`) and inject the real version into the install commands instead of hardcoded `main` branch URL.
- [ ] **Node live detection** — after install, poll `/api/nodes` every 10 s for a peer ID stored in `localStorage` post-install. When found, show a "Your node is live!" celebration toast and unlock the `node_live` gamification state.
- [ ] **Gamification state machine** — implement the full `teaser → benchmarked → installed → node_live` progression with achievement badge chips (see plan). State persisted in `localStorage`.
- [ ] **Windows PowerShell copy** — the copy button currently copies the `curl` command even when Windows tab is selected. Fix to copy the `irm | iex` command.

---

### Infrastructure & Deployment

- [ ] **`deploy-map.yml` GitHub Actions workflow** — on push to `main`, build `Dockerfile.map-server` and `Dockerfile.map-frontend`, push to GHCR, SSH-deploy to `map.kwaai.ai` host.
- [ ] **`docker-compose.map.yml`** — single compose file to run `map-server` + `nginx` frontend + `kwaainet` (for p2pd) on the production host. Include health checks and restart policies.
- [ ] **TLS / HTTPS** — `docker/map-nginx.conf` serves plain HTTP. Add Certbot / Let's Encrypt auto-renewal for `map.kwaai.ai`.
- [ ] **Environment secrets** — document required env vars (`BIND_ADDR`, `ALLOWED_ORIGINS`, `BOOTSTRAP_PEERS`, `TOTAL_BLOCKS`) in a `.env.example` file.
- [ ] **Map-server in workspace dist config** — `map-server` is not in `[workspace.metadata.dist]` targets. Decide whether to ship it as a release binary or Docker-only.

---

### v2 — Operator Dashboard (auth required)

- [ ] **Passkey / WebAuthn registration** — reuse `summit-server` WebAuthn flow. Add `POST /api/auth/begin` + `/complete` to map-server, or proxy to summit-server.
- [ ] **Operator node binding** — `POST /api/node/claim` lets an authenticated user claim their peer ID. Stored in SQLite alongside the node cache.
- [ ] **Private stats panel** — authenticated route `/dashboard` showing uptime history, per-block throughput, earnings ledger, and VC status for the operator's own node.
- [ ] **VC issuance trigger** — operator dashboard shows a "Request VerifiedNodeVC" button that initiates the issuance flow via summit-server.

---

### v3 — Trust Registry

- [ ] **TRQP endpoint** — `GET /api/trust/:did` implements the Trust Registry Query Protocol so other agents can verify KwaaiNet node DIDs against the live registry.
- [ ] **Verifiable Relationship Credentials** — extend the DHT wire format to carry `PeerEndorsementVC` data between peers. Visualise the resulting endorsement graph on the map.
- [ ] **EigenTrust propagation** — implement Phase 4 of `kwaai-trust` (transitive endorsement scoring) and feed scores into the D3 trust graph node sizes.

---

## map.kwaai.ai — v2 Rewrite

> Full requirements: `docs/MAP_SERVER_V2_REQUIREMENTS.md`
> Replaces: OpenAI-Petal `docker/kwaainet_health/` (Python/Flask + vanilla JS + Leaflet)
> Source to port from: `../OpenAI-Petal`

### Map View (Phase 2 — port + upgrade from OpenAI-Petal)
- [ ] **Improved geo-location accuracy** — upgrade IP→lat/lon resolution; fall back to `announce_addr` multiaddr parsing before IP-geolocation DB lookup.
- [ ] **Pin images** *(optional)* — custom marker icons per trust tier (Unknown / Known / Verified / Trusted); SVG icons served from `/static/`.
- [ ] **Traffic heatmap** — overlay tokens/sec per geographic cell using a weighted heatmap layer (MapLibre `heatmap` layer or Leaflet.heat); intensity driven by `throughput_tps` field from `/api/nodes`.
- [ ] **Bootstrap diagnostics on mouse-over** — hovering a bootstrap peer marker shows a tooltip with: peer_id, uptime, peers_known, last_seen, latency to server; data from `GET /api/bootstrap/:peer_id`.

### Table View (Phase 2 — port + upgrade from OpenAI-Petal)
- [ ] **Summary stats row** — pinned header row showing network totals: total nodes (direct + relay counts separate), total tokens/sec, total storage capacity (GB).
- [ ] **Direct vs relay node count** — distinguish `direct` (reachable via public IP) from `relay` (circuit-relay only) in both the summary row and per-row badge.
- [ ] **Total tokens/sec column** — per-node `throughput_tps` column with network total in the summary row; fix `-0.0` display.
- [ ] **Total storage capacity column** — per-node `capacity_gb` from VPK info; network total in summary row.
- [ ] **Version column** — decode `kwaainet_version` from DHT map (see MEMORY.md "Peer version visibility"); highlight stale versions in amber.
- [ ] **Uptime column** — `first_seen` → uptime duration string (e.g. `3d 14h`); updated live via WebSocket.
- [ ] **Blocks capability column** — show block capability as a compact chip list (e.g. `0–31`, `32–63`) rather than raw start/end integers; colour-coded by coverage gaps.

### Backend hardening (Phase 1)
- [ ] **WebSocket diff protocol** — replace full-snapshot push every 5 s with compact add/update/remove diffs; reduces bandwidth ~95% for established connections.
- [ ] **`GET /api/nodes/:peer_id`** — individual node detail: VC list, 24 h throughput history, uptime.
- [ ] **`GET /api/coverage`** — block 0–79 coverage bitmap (`u8[80]` count per block); drives coverage heatmap widget.
- [ ] **`POST /api/v1/state`** — heartbeat ingest from running nodes; upserts into cache immediately so nodes appear within seconds of starting, not after the 60 s crawl.
- [ ] **SQLite persistence** — write node cache + 24 h throughput history to `rusqlite`; no cold-start blank map after server restart.
- [ ] **Rate-limit `/api/nodes`** — 5 s pre-serialised byte cache; single goroutine reserialises; `tower` middleware.
- [ ] **CORS lockdown** — set `ALLOWED_ORIGINS=https://map.kwaai.ai` in production Dockerfile.

### 3D Globe hero (Phase 2)
- [ ] **Replace flat NetworkGraph with `globe.gl`** — Three.js 3D globe, nodes as luminous dots (size = throughput, colour = trust tier), rotating hero element.
- [ ] **Inference arc animation** — geodesic lines connecting coordinator → shard nodes during active sessions; synthesised from `/api/live` session data.
- [ ] **Coverage heatmap bar** — blocks 0–79 coloured by coverage count (green ≥ 2 nodes, amber = 1, red = gap); clicking a block highlights serving nodes on globe.
- [ ] **Node detail side panel** — click globe node → slide-in panel with peer_id, blocks, throughput, version, VPK, VC count, first/last seen, 24 h sparkline.
- [ ] **Operator search + deep link** — search by peer_id / public_name / IP; `?node=Qm...` URL; globe flies to matching node.
- [ ] **OffscreenCanvas worker** — Three.js globe in offscreen canvas to keep main thread free.

### Engagement & conversion (Phase 3)
- [ ] **Real endorsement edges in trust graph** — decode `PeerEndorsementVC` subject/issuer DIDs from `/api/nodes`; draw directed arrows; tier filter checkboxes.
- [ ] **WebGL GEMM benchmark fallback** — twgl.js middle tier between WebGPU and CPU for Safari/Firefox users.
- [ ] **Benchmark localStorage persist + share URL** — save results across sessions; "Share my score" encodes tps/storage in URL params.
- [ ] **Calibrated tps conversion factor** — empirical data from known hardware (M2, RTX 4090, A100) to fit better tokens/sec estimate.
- [ ] **Install funnel: GitHub Release API version** — fetch latest tag dynamically instead of hardcoded `main` branch URL.
- [ ] **Gamification state machine** — `teaser → benchmarked → installed → node_live`; achievement chip badges; `node_live` detected by polling `/api/nodes` for stored peer ID.
- [ ] **Windows PowerShell copy fix** — copy button copies curl command even on Windows tab; fix to copy `irm | iex` command.

### Polish & accessibility (Phase 4)
- [ ] **Framer Motion entrance animations** — panels slide/fade in; globe nodes lerp to new positions.
- [ ] **`prefers-reduced-motion` support** — disable arc animations and globe auto-rotation.
- [ ] **WCAG 2.1 AA audit** — `aria-label` on all icon-only buttons; keyboard nav for globe; contrast check.
- [ ] **Mobile responsive nav** — hamburger menu; sections stack on small screens.
- [ ] **Favicon** — `public/favicon.svg` SVG version of Kwaai tree logo.
- [ ] **Lighthouse ≥ 90** — bundle < 300 KB gzip JS, < 50 KB CSS; virtual DOM diff only for side panel.

---

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

### Phase 3 — GraphRAG + Reasoning Harnesses

- [ ] **`crates/kwaai-rag/src/graph.rs`** — `KgNode { id, label, node_type, chunk_ids }`, `KgEdge { source_id, target_id, relation }`, `KnowledgeGraph`. Built during ingestion via structured-output entity extraction (calls `shard_api` — no new services). Persisted as a third redb table in the MetaStore sidecar.

- [ ] **`crates/kwaai-rag/src/reasoning.rs`** — `ReasoningStrategy` enum: `Naive`, `GraphExpanded`, `ChainOfThought`, `Agentic { max_steps }`. Graph-augmented retrieval: top-K dense chunks → expand entity neighborhood (1–2 hops) → add neighborhood chunks to context.

- [ ] **`--strategy` flag on `kwaainet rag serve`** — `--strategy graph` enables GraphRAG; `--strategy cot` enables chain-of-thought prompting.

---

## Networking

- [ ] **Fix relay fallback** — `metro@kwaai` (peer `...5bZ251`) connects via p2p-circuit relay through `76.91.214.120` instead of direct on configured public IP `75.141.127.202:8080`. Node should establish a direct connection. Investigate NAT traversal / port forwarding and `announceAddrs` config.
