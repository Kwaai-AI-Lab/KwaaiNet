# KwaaiNet Core

Rust/WASM core implementation for KwaaiNet sovereign AI infrastructure.

## Crate Structure

```
core/
├── crates/
│   ├── kwaai-p2p/           # P2P networking (libp2p, Kademlia DHT)
│   ├── kwaai-inference/     # ML inference engine (Candle)
│   ├── kwaai-distributed/   # Distributed ML (MoE, averaging)
│   ├── kwaai-compression/   # Gradient compression (8-bit quantization)
│   └── kwaai-wasm/          # Browser WASM bindings
├── examples/                # Usage examples
└── tests/                   # Integration tests
```

## Building

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
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

## Crate Overview

### kwaai-p2p

P2P networking layer using libp2p:
- Kademlia DHT for peer discovery
- WebRTC transport for browsers
- TCP/QUIC for native applications
- NAT traversal

### kwaai-inference

ML inference engine using Candle:
- GGUF model loading
- Text generation
- Resource-aware scheduling
- Multi-device support (CPU, CUDA, Metal)

### kwaai-distributed

Distributed ML operations (Hivemind patterns):
- Mixture of Experts (MoE)
- Decentralized parameter averaging
- Fault-tolerant expert routing

### kwaai-compression

Communication optimization:
- Blockwise 8-bit quantization
- Sparse gradient compression (top-K)
- ~4x bandwidth reduction

### kwaai-wasm

Browser integration:
- WebAssembly bindings
- JavaScript API
- Web Worker support

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

## License

MIT License - see [LICENSE](../LICENSE)
