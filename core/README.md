# KwaaiNet Core

Rust/WASM core implementation for KwaaiNet sovereign AI infrastructure.

## ⚠️ Project Mission

**This project is replacing the Python Hivemind/Petals stack with Rust/WASM.**

The user already has working bare metal nodes on map.kwaai.ai built with Python code from the [OpenAI-Petal project](https://github.com/Kwaai-AI-Lab/OpenAI-Petal). The mission is to:
1. Achieve 100% protocol compatibility with Petals/Hivemind DHT and RPC
2. Replace Python dependencies with pure Rust implementation
3. Enable WASM deployment for browser-based nodes
4. Do NOT go backward by adding Docker or Python dependencies

**Reference Implementation**: `/Users/rezarassool/Source/OpenAI-Petal/` - Python wrapper that calls standard `petals.cli.run_server`

## Status

| Component | Status | Description |
|-----------|--------|-------------|
| P2P Networking | ✅ Working | libp2p swarm, Kademlia DHT, peer discovery |
| Tensor Operations | ✅ Working | Candle ML framework integration |
| 8-bit Quantization | ✅ Working | 3.8x compression with minimal accuracy loss |
| Sparse Gradients | ✅ Working | Top-K compression for bandwidth efficiency |
| Expert Registry | ✅ Working | MoE infrastructure with fault tolerance |
| Parameter Averaging | ✅ Working | Decentralized gradient averaging |
| P2P Tensor Exchange | ✅ Working | Compressed tensor transmission between nodes |
| Petals Integration | ✅ Complete | DHT + RPC handler for map.kwaai.ai visibility |
| WASM Build | 🔧 Scaffold | Browser bindings (interface defined) |

**Latest (Dec 3, 2025):**
- ✅ **Hivemind RPC protocol implementation complete**
- ✅ Nodes respond to health monitor queries via `/hivemind/0.0.0/rpc`
- ✅ Full compatibility with Petals/Hivemind protocol (MessagePack + protobuf)
- ✅ KwaaiNet nodes now visible on map.kwaai.ai
- ✅ libp2p 0.53 Codec trait correctly implemented
- ✅ `petals_visible` example with integrated RPC handler ready to run

## Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git
cd KwaaiNet/core
cargo build --release

# Build the kwaainet CLI
cargo build --release -p kwaai-cli
```

### Running a node with `kwaainet`

```bash
# One-time setup
./target/release/kwaainet setup

# Start in foreground
./target/release/kwaainet start

# Or as a background daemon
./target/release/kwaainet start --daemon
./target/release/kwaainet status
./target/release/kwaainet logs --follow
./target/release/kwaainet stop
```

## Runnable Examples

### P2P Networking (Days 1-3)

```bash
# Day 1: Basic P2P node
cargo run --example p2p_node

# Day 2: DHT key-value store
cargo run --example dht_node -- --listen 4001 --put mykey "hello world"

# Day 3: Capability-based peer discovery
cargo run --example peer_discovery -- --listen 4001 --provide inference:llama2
```

### ML Operations (Days 4-5)

```bash
# Day 4: Tensor operations with Candle
cargo run --example tensor_ops

# Day 5: Neural network forward pass
cargo run --example forward_pass
```

### Compression (Days 6-7)

```bash
# Day 6: 8-bit blockwise quantization
cargo run --example quantization

# Day 7: Top-K sparse gradient compression
cargo run --example sparse_gradients
```

### Distributed ML (Days 8-9)

```bash
# Day 8: MoE expert registry and routing
cargo run --example expert_registry

# Day 9: Decentralized parameter averaging
cargo run --example local_averaging
```

### P2P Tensor Exchange (Day 10)

```bash
# Terminal 1 (receiver):
cargo run --release --example tensor_exchange -- --listen 4001

# Terminal 2 (sender):
cargo run --release --example tensor_exchange -- \
  --connect /ip4/<IP>/tcp/4001/p2p/<PEER_ID> --send
```

See [examples/TWO_MACHINE_TEST.md](examples/TWO_MACHINE_TEST.md) for multi-machine testing.

### Petals Network Integration

```bash
# Test Petals DHT connectivity
cargo run --example petals_dht

# Make node visible on map.kwaai.ai (full RPC implementation)
cargo run --release --example petals_visible -- \
  --name "My-KwaaiNode" \
  --model "Llama-3.3-70B-Instruct" \
  --port 31337

# Your node will:
# 1. Connect to Petals DHT via bootstrap servers
# 2. Announce itself with ServerInfo
# 3. Accept RPC queries from health monitor
# 4. Appear on map.kwaai.ai within 5-10 minutes
```

## Crate Structure

```
core/
├── crates/
│   ├── kwaai-cli/           # kwaainet CLI binary (start/stop/status/config/...)
│   ├── kwaai-p2p/           # P2P networking (libp2p, Kademlia DHT)
│   ├── kwaai-p2p-daemon/    # go-libp2p-daemon wrapper
│   ├── kwaai-hivemind-dht/  # Hivemind/Petals DHT protocol
│   ├── kwaai-inference/     # ML inference engine (Candle)
│   ├── kwaai-distributed/   # Distributed ML (MoE, averaging)
│   ├── kwaai-compression/   # Gradient compression (8-bit quantization)
│   └── kwaai-wasm/          # Browser WASM bindings
├── examples/                # Runnable examples
└── tests/                   # Integration tests
```

## Crate Overview

### kwaai-cli

The `kwaainet` binary — a native Rust CLI for managing KwaaiNet nodes:
- Start/stop/restart daemon lifecycle with PID and lock files
- YAML config management at `~/.kwaainet/config.yaml`
- Hardware calibration (RAM-based block count estimation)
- Health monitoring with exponential backoff reconnection
- Auto-start service management (launchd on macOS, systemd on Linux)
- GitHub Releases update checker

### kwaai-p2p

P2P networking layer using libp2p:
- Kademlia DHT for peer discovery
- Request-response protocol for tensor exchange
- **Hivemind RPC protocol** for Petals/map.kwaai.ai integration
- TCP transport with Noise encryption and Yamux multiplexing
- NAT traversal support

### kwaai-inference

ML inference engine using Candle:
- GGUF model loading
- Text generation
- Resource-aware scheduling
- Multi-device support (CPU, CUDA, Metal)

### kwaai-distributed

Distributed ML operations (Hivemind patterns):
- Mixture of Experts (MoE) with TopK routing
- Decentralized parameter averaging
- Fault-tolerant expert routing with fallbacks
- Expert registry for local/remote experts

### kwaai-compression

Communication optimization:
- Blockwise 8-bit quantization (~4x compression)
- Sparse gradient compression (Top-K selection)
- Error feedback for accuracy preservation

### kwaai-wasm

Browser integration:
- WebAssembly bindings
- JavaScript API
- Web Worker support

## Building

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target (optional, for browser builds)
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Native Build

```bash
cd core
cargo build --release
```

### WASM Build

```bash
cd core/crates/kwaai-wasm
wasm-pack build --target web --release
```

### Run Tests

```bash
cd core
cargo test
```

## For Contributors

The codebase provides working interfaces for you to build upon:

1. **P2P Layer** (`kwaai-p2p`): Implement custom protocols on top of libp2p
2. **Inference** (`kwaai-inference`): Add model formats, optimize inference
3. **Distributed** (`kwaai-distributed`): Implement full MoE forward pass, improve averaging
4. **Compression** (`kwaai-compression`): Add new compression algorithms
5. **WASM** (`kwaai-wasm`): Complete browser bindings

Each crate has defined traits - implement them to extend functionality.

## Development

### Code Style

```bash
# Format code
cargo fmt

# Run lints
cargo clippy
```

### Adding a New Crate

1. Create directory: `crates/kwaai-newcrate/`
2. Add `Cargo.toml` with workspace dependencies
3. Add to workspace members in root `Cargo.toml`
4. Implement with proper error handling and documentation

## Documentation

- [Examples README](examples/README.md) - Detailed example documentation
- [Two-Machine Test](examples/TWO_MACHINE_TEST.md) - Multi-node testing guide
- [Hivemind Architecture](../docs/HIVEMIND_RUST_ARCHITECTURE.md) - Distributed ML design
- [Candle Engine](../docs/CANDLE_ENGINE.md) - Inference engine details

## License

MIT License - see [LICENSE](../LICENSE)
