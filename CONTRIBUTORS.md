# Contributors

## Current Contributors

| Name | GitHub | Email | Role |
|------|--------|-------|------|
| Reza Rassool | [@RezaRassool](https://github.com/RezaRassool) | reza@kwaai.ai | Founder / Core |
| Metro | — | — | Core |
| Balaji | [@xspanbalaji](https://github.com/xspanbalaji) | balaji@xspani.ai | Contributor |

---

## Contributor TODO List

The following areas need contributors. Pick what interests you and open a PR or discussion.

### Core Runtime (Rust / WASM)
- [ ] Improve WASM binary size and startup performance
- [ ] Add `no_std` support for embedded/edge targets
- [ ] Harden error types across the crate (consistent `KwaaiError` coverage)
- [ ] Implement graceful shutdown / node lifecycle management
- [ ] Write fuzz tests for p2p message parsing

### Inference Engine (Candle) — **critical path**

> **Context** (updated 2026-02-27): Local single-node inference is now real —
> GGUF/SafeTensors model loading via `candle-transformers`, BPE tokenizer from
> GGUF vocab or `tokenizer.json`, autoregressive `generate()` loop, and Metal GPU
> acceleration on Apple Silicon all work. The node serves the full model locally
> and exposes an OpenAI-compatible API.
>
> What is **not yet implemented** is true Petals-style distributed block sharding.
> Each node currently runs the full model independently. The `start_block`/`end_block`
> fields in DHT announcements are metadata only — no activation tensors are exchanged
> between nodes. The `forward()` method in `engine.rs` returns an error stub and the
> `kwaai-distributed` MoE routing returns input unchanged. The items below are needed
> to close that gap.

**Model loading**
- [x] Implement real weight loading from SafeTensors (`candle-core`) in `engine.rs`
- [x] Implement GGUF model loading for quantized models (`candle-transformers`)
- [x] Replace `_weights: Vec::new()` with an actual loaded model struct

**Tokenizer**
- [x] Replace the byte-level placeholder tokenizer with a real BPE tokenizer
      (`tokenizers` crate; loaded from GGUF vocab or `tokenizer.json`)

**Forward pass & generation**
- [x] Implement a real autoregressive token generation loop in `engine.rs`
- [ ] Add KV-cache support for efficient multi-turn generation
- [ ] Implement temperature / top-p / top-k sampling
- [ ] Wire `kwaai-inference` into the node's RPC handlers so inference requests
      actually reach the engine (currently the RPC path never calls the crate)

**Apple Silicon / Metal acceleration**
- [x] Enable the `metal` feature flag (`candle-core/metal`) on macOS — 33+ tok/s on M4 Pro
- [x] Verify `candle_core::Device::Metal` is selected at runtime on Apple Silicon
- [ ] Benchmark Metal vs CPU on Mac mini and document results

**Benchmarking**
- [ ] Implement real tokens/s benchmark in `kwaai-inference/benches/inference_bench.rs`
      (currently a TODO stub)
- [ ] Add performance regression gate to CI

**Longer-term**
- [ ] Benchmark Candle vs. llama.cpp bindings vs. ONNX Runtime and document results
- [ ] Streaming token output over the RPC interface (currently returns full completion)
- [ ] Multi-model routing (select model by capability or load)
- [ ] Test inference end-to-end using [Open WebUI](https://github.com/open-webui/open-webui) as the frontend against `kwaainet serve` (`/v1/chat/completions`)

### Distributed Block Sharding (Petals model) — **next major milestone**

> **Context**: The Petals/Hivemind model shards a single transformer across multiple
> nodes. Node A loads blocks 0–7, processes the input embeddings, and forwards the
> resulting activation tensor to Node B which processes blocks 8–15, and so on until
> the final node produces logits. This enables running models too large for any single
> machine. The KwaaiNet Rust implementation has all the networking infrastructure in
> place (DHT block announcements, RPC handlers, p2p daemon) but does not yet perform
> any inter-node activation routing. All items below are needed to make real sharding
> work.

- [ ] **Partial model loading** — load only transformer blocks `start_block..end_block`
      instead of the full model; slice the weight tensors at load time in `loader.rs`
- [ ] **Inference RPC message type** — define a protobuf/msgpack `InferenceRequest`
      carrying `(session_id, sequence_position, activation_tensor)` for node-to-node
      forwarding; add request/response types alongside the existing Hivemind RPC messages
      in `kwaai-p2p-daemon`
- [ ] **Wire `forward()` in `engine.rs`** — receive an activation tensor via RPC, run it
      through the local block slice, return the output tensor; replace the current error stub
- [ ] **Activation handoff** — after processing local blocks, send the output tensor to the
      next node in the block chain; discover the next node from the DHT block registry
- [ ] **Client-side coordinator** — logic to find which nodes hold which blocks for a given
      model (via DHT lookup), chain the RPC calls in order, and reassemble the final logits
      for sampling; lives in `kwaai-distributed/src/coordinator.rs`
- [ ] **Session management** — track in-flight inference sessions (sequence length, KV cache
      state) across the multi-node chain; handle node dropout / re-routing mid-sequence
- [ ] **Real MoE expert routing** — replace the uniform stub in `kwaai-distributed/src/moe.rs`
      with real softmax + top-k gating and remote expert calls via P2P for MoE models
      (e.g. Mixtral)
- [ ] **End-to-end test** — two-node integration test on localhost: node A serves blocks 0–3,
      node B serves blocks 4–7, client sends a prompt and receives correct output

### Windows Support — **needs a Windows dev machine**

> **Context**: The codebase builds and runs on Windows for dev/testing, but several
> production-critical features are Unix-only stubs. All items below require testing
> and iteration on a real Windows machine (not WSL).

**Graceful shutdown (Priority 1)**
- [ ] Replace `taskkill /F` in `daemon.rs` with a named-event or named-pipe signal
      so the node can flush DHT announcements and close peer connections cleanly
- [ ] Wire the Windows shutdown signal into the `shutdown_signal()` future in `node.rs`
      (currently only `Ctrl+C` is caught; SIGTERM equivalent is missing)

**Daemon instance locking (Priority 1)**
- [ ] Replace the `flock` no-op in `daemon.rs::try_acquire_lock` with a Windows
      named mutex (`CreateMutexW`) so a second `kwaainet start` fails fast instead
      of colliding on the same port

**Auto-start service integration (Priority 2)**
- [ ] Implement `WindowsServiceManager` in `service.rs` (currently returns
      `Err("not supported")`) — install/uninstall/status via the Windows Service
      Control Manager API or a bundled NSSM/winsw wrapper

**Home directory (Priority 2)**
- [ ] Fix `dirs_sys::home_dir()` in `config.rs` to fall back to `USERPROFILE` on
      Windows so config paths resolve correctly on systems where `HOME` is not set

**Validation**
- [ ] Smoke-test `kwaainet start`, `status`, `stop`, `serve` end-to-end on Windows 10/11
- [ ] Add Windows to the CI platform matrix (see Testing section below)

### Trust Graph (kwaai-trust) — **Phase 2 next up**

> **Context**: The `kwaai-trust` crate ships Phase 1 (VC data model, `did:peer:`
> utilities, credential storage, Ed25519 verification, weighted trust scoring,
> `kwaainet identity` CLI). The items below are the Phase 2–4 work needed to
> make trust a live network feature rather than a local tool.

**Phase 2 — Credential issuance (Q2 2026)**
- [ ] Build the summit on-ramp server that issues `SummitAttendeeVC` on QR scan
      (signs with its own Ed25519 keypair; returns VC JSON for the attendee to import)
- [ ] Build the GliaNet pledge endpoint that issues `FiduciaryPledgeVC`
      (`kwaainet pledge sign` flow — submits pledge hash, receives signed VC)
- [ ] Build the Kwaai Foundation onboarding endpoint that issues `VerifiedNodeVC`
- [ ] Implement `kwaainet pledge sign` CLI command
- [ ] Expose trust tier and badge data in the map.kwaai.ai health-monitor API
      (parse `trust_attestations` field from DHT `ServerInfo`; verify signatures)

**Phase 3 — Automated issuance (Q3 2026)**
- [ ] Bootstrap server uptime tracking: auto-issue `UptimeVC` after N days of
      observed availability (threshold and period configurable in governance config)
- [ ] Peer throughput witnessing: nodes that forward inference requests record
      measured vs announced throughput; issue `ThroughputVC` when within tolerance
- [ ] Implement VC revocation check (issuer publishes revocation list; verifier
      consults it before accepting a credential)

**Phase 4 — EigenTrust propagation (Q3 2026)**
- [ ] `PeerEndorsementVC` issuance flow: after N successful inference transactions,
      the requesting node offers a signed endorsement to the serving node
- [ ] Implement `TrustScore::from_endorsement_graph()` — 2-hop EigenTrust propagation
      over the endorsement graph stored in the credential store
- [ ] Sybil resistance: weight endorsements by endorser's own trust score
- [ ] Persist the endorsement graph locally for offline score queries

**Phase 5 — Optional DID binding (Q4 2026)**
- [ ] `kwaainet identity link --did did:vda:...` — bind a user-level DID to the
      node's `did:peer:` via a signed assertion; store the binding VC locally
- [ ] Support `did:web`, ENS, and `did:ion` as external identity anchors
- [ ] Update trust score to incorporate bound DID's reputation (if available)

**Infrastructure**
- [ ] Trust registry contract / JSON-LD context at `https://kwaai.ai/credentials/v1`
      listing authoritative issuers per VC type and their public key DIDs
- [ ] VC schema validation (JSON Schema per credential type)
- [ ] Expand `kwaai-trust` unit tests: sign → verify round-trip, expiry edge cases,
      malformed proof rejection, time-decay boundary values

### P2P Networking (Hivemind / libp2p)
- [ ] Implement NAT traversal improvements (relay fallback, hole-punching)
- [ ] DHT optimisations for large peer sets (>1 000 nodes)
- [ ] Write integration tests for multi-node scenarios
- [ ] Port conflict resolution for multiple nodes behind the same WAN IP — detect when `public_ip` matches another already-announced node on the same port, warn the user, and suggest an alternate port (`kwaainet config --set port <N>`)
- [x] Replace hardcoded 30s bootstrap wait with peer-count polling — announce as soon as ≥1 bootstrap peer is connected (typically 2–5s); reduces `kwaainet start --daemon` → node-on-map time from ~46s to ~15s
- [ ] Fix `kwaainet stop` hang — daemon does not exit cleanly when stop command is issued; currently requires `kill -9`; investigate whether the shutdown signal is not being delivered or the run-node loop is blocking on a future that doesn't respect cancellation

### Storage Integrations
- [ ] IPFS storage provider (implement `StorageProvider` trait)
- [ ] OrbitDB storage provider
- [ ] Solid Protocol pod storage provider
- [ ] Filecoin persistent storage provider

### Identity Integrations
- [ ] WebAuthn / PassKey identity provider (implement `IdentityProvider` trait)
- [ ] ENS (Ethereum Name Service) identity provider
- [ ] Improve Verida DID documentation with working end-to-end example

### Browser / Web
- [ ] WASM bundle size audit and tree-shaking
- [ ] Service Worker integration for background node operation
- [ ] WebRTC mesh connection reliability improvements
- [ ] Browser extension scaffold (Chrome / Firefox)

### Mobile
- [ ] iOS proof-of-concept (Swift + WASM)
- [ ] Android proof-of-concept (Kotlin + WASM)
- [ ] React Native bridge scaffold

### Environmental / Carbon Tracking
- [ ] Integrate Energy Origin Certificate API
- [ ] Add Renewable Energy Credit (REC) verification
- [ ] Carbon leaderboard UI component

### Testing & Quality
- [ ] Raise unit test coverage to ≥ 80% across all crates
- [ ] Add CI platform matrix (Linux, macOS, Windows, WASM)
- [ ] End-to-end test harness for multi-node inference
- [ ] Performance regression benchmarks in CI
- [ ] Smoke-test `summit-server` Docker image end-to-end (passkey registration → VC issuance → node bind)
- [ ] Add Docker build to CI so image is validated on every PR, not only on release tags

### Cross-Compilation
- [ ] Cross-compile `x86_64-apple-darwin` (Intel Mac) from `macos-latest` (Apple Silicon) runner — requires `GOARCH=amd64` for p2pd and verifying no C dependency issues with the macOS SDK
- [ ] Cross-compile `x86_64-unknown-linux-gnu` from macOS or Windows using `cross` or `zig cc` as the linker
- [ ] Cross-compile `x86_64-pc-windows-msvc` from Linux using `cross` + MinGW or MSVC sysroot
- [ ] Once cross-compilation is proven, collapse the 4-platform matrix to a single `ubuntu-latest` runner to cut CI time and eliminate runner availability issues (e.g. `macos-13` deprecation)

### Release & Distribution
- [ ] Smoke-test Windows binary end-to-end (currently marked `experimental` in release.yml)
- [x] Write `install.sh` auto-detect script (detects platform, downloads correct binary, installs both kwaainet + p2pd)
- [ ] Test install on fresh Ubuntu VM / Docker container
- [ ] Test install on macOS Intel from binary download
- [ ] Verify `kwaainet setup` wizard works after fresh binary install

### VPK — Phase 2 & 3
- [ ] Phase 2: Cross-node Eve sharding (`kwaainet vpk shard --kb-id <id> --eve-count N`)
- [ ] Phase 3: DHT FIND on `_kwaai.vpk.kb.{kb_id}` for shard topology recovery (`kwaainet vpk resolve`)
- [ ] PHE/VPK repo: add `peer_id`, `mode` fields to config.rs
- [ ] PHE/VPK repo: multi-tenant DB schema (tenant_id column on documents, index_mapping, audit_log)
- [ ] PHE/VPK repo: `GET /api/health` returning peer_id, tenant_count, capacity_gb_available

### Documentation
- [ ] API reference (auto-generated via `cargo doc`, published to docs.rs)
- [ ] Quickstart tutorial (zero to first inference in 5 minutes)
- [ ] Integration cookbook (one page per storage/identity provider)
- [ ] Architecture decision records (ADRs) for past major choices
- [ ] Video walkthrough of local dev setup

### Community & Ecosystem
- [ ] Example project: personal AI assistant using KwaaiNet
- [ ] Example project: collaborative document summarisation
- [ ] Discord bot for CI status / contributor stats
- [ ] Contributor onboarding checklist / mentorship pairing

---

## How to Claim a TODO

1. Open a [GitHub Issue](https://github.com/Kwaai-AI-Lab/kwaainet/issues) describing the work you plan to do.
2. Tag it with the relevant area label (e.g. `area: storage`, `area: p2p`).
3. Mention this file so we can check it off when your PR merges.

See [CONTRIBUTING.md](CONTRIBUTING.md) for full guidelines.
