# KwaaiNet Core

Rust/WASM core implementation for KwaaiNet sovereign AI infrastructure.

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
| Petals Integration | ✅ Working | DHT bootstrap via Petals network |
| WASM Build | 🔧 Scaffold | Browser bindings (interface defined) |

**Latest (Nov 22, 2025):**
- ✅ Two-machine P2P tensor exchange verified
- ✅ Connected to Petals/Hivemind network via shared DHT
- ✅ Transport layer (TCP/noise/yamux) compatible with Petals
- ✅ Hivemind protocol module added (ServerInfo, MessagePack serialization)
- ✅ `petals_visible` example: DHT announcement for map.kwaai.ai discovery
- 🚧 Petals protocol bridge in progress ([roadmap](docs/PETALS_BRIDGE_ROADMAP.md))

## Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git
cd KwaaiNet/core
cargo build --release
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

# Make node visible on map.kwaai.ai
cargo run --release --example petals_visible -- --name "My-Node"
```

## Crate Structure

```
core/
├── crates/
│   ├── kwaai-p2p/           # P2P networking (libp2p, Kademlia DHT)
│   ├── kwaai-inference/     # ML inference engine (Candle)
│   ├── kwaai-distributed/   # Distributed ML (MoE, averaging)
│   ├── kwaai-compression/   # Gradient compression (8-bit quantization)
│   └── kwaai-wasm/          # Browser WASM bindings
├── examples/                # 10 runnable examples
└── tests/                   # Integration tests
```

## Crate Overview

### kwaai-p2p

P2P networking layer using libp2p:
- Kademlia DHT for peer discovery
- Request-response protocol for tensor exchange
- TCP transport for native applications
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

## For Hackathon Contributors

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
