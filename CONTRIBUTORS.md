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

### Inference Engine (Candle)
- [ ] Benchmark Candle vs. other backends (llama.cpp, ONNX) and document results
- [ ] Add quantized model (GGUF / GGML) loading support
- [ ] Streaming token output over the RPC interface
- [ ] Multi-model routing (select model by capability or load)

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
