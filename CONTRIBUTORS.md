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

> **Context**: The current `kwaai-inference` crate is a stub. `engine.rs` returns
> hardcoded strings; model loading stores empty weight vectors; the tokenizer is
> byte-level only. The node never calls the inference crate at all — it is purely
> a DHT announcer. The Python/Petals equivalent achieves ~1500 tokens/s on a Mac
> mini; the Rust node produces ~100 tokens/s of fake output. All items below are
> needed to close that gap.

**Model loading**
- [ ] Implement real weight loading from SafeTensors (`candle-core`) in `engine.rs`
- [ ] Implement GGUF model loading for quantized models (`candle-transformers`)
- [ ] Replace `_weights: Vec::new()` with an actual loaded model struct

**Tokenizer**
- [ ] Replace the byte-level placeholder tokenizer with a real BPE tokenizer
      (e.g. `tokenizers` crate, or load vocab from HuggingFace model repo)

**Forward pass & generation**
- [ ] Implement a real autoregressive token generation loop in `engine.rs`
- [ ] Add KV-cache support for efficient multi-turn generation
- [ ] Implement temperature / top-p / top-k sampling
- [ ] Wire `kwaai-inference` into the node's RPC handlers so inference requests
      actually reach the engine (currently the RPC path never calls the crate)

**Apple Silicon / Metal acceleration**
- [ ] Enable the existing `metal` feature flag (`candle-core/metal`) by default on macOS
- [ ] Verify `candle_core::Device::Metal` is selected at runtime on Apple Silicon
- [ ] Benchmark Metal vs CPU on Mac mini and document results

**Benchmarking**
- [ ] Implement real tokens/s benchmark in `kwaai-inference/benches/inference_bench.rs`
      (currently a TODO stub)
- [ ] Add performance regression gate to CI

**Longer-term**
- [ ] Benchmark Candle vs. llama.cpp bindings vs. ONNX Runtime and document results
- [ ] Streaming token output over the RPC interface
- [ ] Multi-model routing (select model by capability or load)

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

### P2P Networking (Hivemind / libp2p)
- [ ] Implement NAT traversal improvements (relay fallback, hole-punching)
- [ ] Add peer reputation / scoring system
- [ ] DHT optimisations for large peer sets (>1 000 nodes)
- [ ] Write integration tests for multi-node scenarios

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
