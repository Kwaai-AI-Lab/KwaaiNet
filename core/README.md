# KwaaiNet Core

Rust core for KwaaiNet sovereign AI infrastructure — P2P inference, distributed block sharding, and privacy-preserving vector storage.

## Status

| Component | Status | Description |
|-----------|--------|-------------|
| P2P Networking | ✅ Working | libp2p swarm, Kademlia DHT, peer discovery |
| Hivemind/Petals compatibility | ✅ Working | DHT + RPC; nodes visible on map.kwaai.ai |
| Block sharding (Petals-style) | ✅ Working | Distributed transformer inference across nodes |
| Storage fabric (Eve role) | ✅ Working | Multi-tenant vector storage, no Docker required |
| Tensor Operations | ✅ Working | Candle ML framework, Metal/CUDA/CPU |
| 8-bit Quantization | ✅ Working | ~4x compression with minimal accuracy loss |
| cargo-dist releases | ✅ Working | Single-binary installs for mac/linux/windows |
| WASM Build | 🔧 Scaffold | Browser bindings (interface defined) |

## Install

```bash
# macOS / Linux — installs kwaainet + p2pd to ~/.cargo/bin
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh

# Homebrew
brew install Kwaai-AI-Lab/tap/kwaainet
```

Or build from source:

```bash
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git
cd KwaaiNet/core
cargo build --release -p kwaainet
```

## Running a Node

```bash
# One-time identity setup
kwaainet setup

# Start as background daemon (P2P node + storage API if configured)
kwaainet start --daemon
kwaainet status
kwaainet logs --follow
kwaainet stop
```

## Storage Fabric (Eve role)

Eve nodes store opaque float vectors on behalf of Bob nodes. Bob embeds documents locally — Eve never sees the text, only the vectors. Search returns IDs and scores; Bob resolves them from his own knowledge base.

**No Docker. No PostgreSQL. Pure embedded Rust (hnsw_rs + redb).**

### Quick start

```bash
# 1. Initialise — creates ~/.kwaainet/storage/ and saves config
kwaainet storage init --capacity-gb 10

# Point at an external or secondary drive
kwaainet storage init --capacity-gb 500 --data-dir /Volumes/MyDrive/kwaainet

# Advertise this Eve publicly so Bob nodes can find it via DHT
kwaainet storage init --capacity-gb 10 --endpoint http://YOUR_PUBLIC_IP:7432

# 2. Check store health, tenant count, and disk usage
kwaainet storage status

# 3. Run the storage API (foreground — Ctrl+C to stop)
kwaainet storage serve

# Or run everything as a background daemon
kwaainet start --daemon
```

### Storage API (REST)

The API runs on port 7432 by default.

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/health` | Health, peer ID, tenant count, capacity |
| `POST` | `/api/tenants` | Create a tenant for a Bob node |
| `GET` | `/api/tenants/:id` | Tenant stats (vector count, bytes) |
| `DELETE` | `/api/tenants/:id` | Delete a tenant and all its vectors |
| `POST` | `/api/tenants/:id/vectors` | Upload a batch of vectors |
| `POST` | `/api/tenants/:id/search` | ANN search — returns IDs + scores only |
| `DELETE` | `/api/tenants/:id/vectors` | Delete vectors by ID |

### Integration tests

```bash
# Local test — no Docker, no network partner needed
bash tests/local-storage-test.sh

# With pre-built binary (skip cargo build)
bash tests/local-storage-test.sh --skip-build

# LAN test — run on Eve machine first, then Bob machine
bash tests/storage-lan-test.sh eve
bash tests/storage-lan-test.sh bob <eve-ip>
```

## Block Sharding (Petals-style distributed inference)

```bash
# Terminal 1 — serve transformer blocks 0-31
kwaainet shard serve --start-block 0 --blocks 32

# Terminal 2 — run a prompt through the distributed chain
kwaainet shard run "What is the capital of France?"

# Show which peers cover which blocks
kwaainet shard chain --total-blocks 32
```

## Crate Structure

```
core/
├── crates/
│   ├── kwaai-cli/           # kwaainet binary — all CLI commands
│   ├── kwaai-storage/       # Embedded vector store (hnsw_rs + redb)
│   ├── kwaai-p2p/           # P2P networking (libp2p, Kademlia DHT)
│   ├── kwaai-p2p-daemon/    # go-libp2p-daemon wrapper (p2pd)
│   ├── kwaai-hivemind-dht/  # Hivemind/Petals DHT protocol
│   ├── kwaai-inference/     # ML inference engine (Candle, block sharding)
│   ├── kwaai-distributed/   # Distributed ML (MoE, gradient averaging)
│   ├── kwaai-compression/   # Gradient compression (8-bit quantization)
│   └── kwaai-wasm/          # Browser WASM bindings
├── examples/                # Runnable examples
└── tests/                   # Integration tests
```

## Crate Overview

### kwaai-cli

The `kwaainet` binary — manages everything:
- Node lifecycle: `start`, `stop`, `status`, `logs`, `restart`
- Storage fabric: `storage init/status/serve/destroy`
- Block sharding: `shard serve/run/chain`
- Identity/DID management: `identity show`
- VPK discovery: `vpk discover/status`
- Auto-update: `kwaainet update`
- Service install: `kwaainet service install` (launchd/systemd)

### kwaai-storage

Embedded multi-tenant vector store — no system dependencies:
- **hnsw_rs** — pure Rust HNSW approximate nearest-neighbour search (same algorithm as pgvector)
- **redb** — pure Rust ACID embedded KV store for tenant metadata and vector persistence
- Per-tenant in-memory index, rebuilt from redb on startup
- Full REST API via Axum (`run_storage_api`)
- Search by Index privacy protocol — Eve returns only `(id, score)`, never embeddings

### kwaai-p2p

P2P networking layer using libp2p:
- Kademlia DHT for peer discovery and DHT announcements
- Hivemind/Petals RPC protocol for map.kwaai.ai integration
- TCP + Noise encryption + Yamux multiplexing

### kwaai-inference

ML inference using Candle:
- GGUF and SafeTensors model loading
- Distributed block sharding (Petals-style forward passes)
- Per-block KV-cache with session management
- Multi-device: CPU, CUDA, Metal

### kwaai-distributed

Distributed ML operations:
- Mixture of Experts (MoE) with TopK routing
- Decentralised parameter averaging
- Fault-tolerant expert routing

### kwaai-compression

Communication optimisation:
- Blockwise 8-bit quantisation (~4x compression)
- Sparse gradient compression (Top-K selection)

## Building

```bash
# Prerequisites: Rust stable toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cd core

# Full workspace build
cargo build --release

# CLI only (faster)
cargo build --release -p kwaainet

# Run tests
cargo test

# Lint
cargo clippy
```

## Documentation

- [Two-Machine Test](examples/TWO_MACHINE_TEST.md)
- [Examples README](examples/README.md)

## License

MIT License — see [LICENSE](../LICENSE)
