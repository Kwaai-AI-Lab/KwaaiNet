# KwaaiNet Core Examples

Runnable examples demonstrating KwaaiNet functionality.

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Navigate to core directory
cd core
```

## Examples

### Day 1: P2P Node Startup

Basic P2P node that starts, listens, and logs peer information.

```bash
# Start a node on random port
cargo run --example p2p_node

# Start on specific port
cargo run --example p2p_node -- /ip4/0.0.0.0/tcp/4001

# Connect to another node
cargo run --example p2p_node -- /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID>
```

### Day 2: DHT Operations

Store and retrieve values via distributed hash table.

```bash
# Terminal 1: Start node and store a value
cargo run --example dht_node -- --listen 4001 --put hello world
# Note the bootstrap address printed

# Terminal 2: Connect and retrieve the value
cargo run --example dht_node -- --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID> --get hello
# Should print: SUCCESS: Retrieved 'world'
```

### Day 3: Peer Discovery

Discover peers by capability using DHT providers.

```bash
# Terminal 1: Register as provider for a capability
cargo run --example peer_discovery -- --listen 4001 --provide inference:llama2

# Terminal 2: Find providers for the capability
cargo run --example peer_discovery -- --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID> --find inference:llama2
# Should print: SUCCESS: Found 1 provider(s)
```

### Day 4: Tensor Operations

Basic Candle tensor operations.

```bash
cargo run --example tensor_ops
# Shows:
# - Tensor creation (zeros, ones, from_vec)
# - Matrix multiplication
# - Softmax
# - Element-wise operations
# - Reductions (sum, mean, max, min)
# - Performance benchmarks
```

### Day 5: Forward Pass

Simple neural network forward pass with Candle.

```bash
cargo run --example forward_pass
# Shows:
# - 2-layer MLP creation with Xavier initialization
# - Forward pass with ReLU activation
# - Performance benchmarking (10 iterations)
# - Throughput calculation (tokens/sec)
```

### Day 6: 8-bit Quantization

Blockwise 8-bit quantization for tensor compression.

```bash
cargo run --example quantization
# Shows:
# - Basic compress/decompress cycle
# - Block size comparison
# - Gradient compression simulation
# - Performance benchmarks
# - Bandwidth savings calculation
```

## Environment Variables

- `RUST_LOG=info` - Set log level (trace, debug, info, warn, error)

## Troubleshooting

**Port already in use**: Use a different port or let the system assign one with `/ip4/0.0.0.0/tcp/0`

**Connection refused**: Ensure the target node is running and the address is correct
