# Two-Machine Test Guide

Simple test to verify P2P tensor exchange between two machines.

## Prerequisites

Both machines need:
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repository
git clone https://github.com/Kwaai-AI-Lab/KwaaiNet.git
cd KwaaiNet/core

# Build (first build takes a few minutes)
cargo build --release --example tensor_exchange
```

## Test 1: Basic Tensor Exchange

### Machine A (Receiver)

```bash
# Get your IP address
# macOS:
ipconfig getifaddr en0

# Linux:
hostname -I | awk '{print $1}'

# Start the receiver on port 4001
cd KwaaiNet/core
cargo run --release --example tensor_exchange -- --listen 4001
```

You'll see output like:
```
KwaaiNet Tensor Exchange Demo
==============================

Mode: RECEIVER
Waiting for tensor data...

Listening on: /ip4/0.0.0.0/tcp/4001

Connect with: --connect /ip4/192.168.1.100/tcp/4001/p2p/12D3KooW...
```

**Copy the full connect address** (starts with `/ip4/...`)

### Machine B (Sender)

Replace `<MACHINE_A_ADDRESS>` with the connect address from Machine A:

```bash
cd KwaaiNet/core
cargo run --release --example tensor_exchange -- \
  --connect /ip4/<MACHINE_A_IP>/tcp/4001/p2p/<PEER_ID> \
  --send
```

### Expected Output

**Machine B (Sender):**
```
Mode: SENDER
Connecting to: /ip4/192.168.1.100/tcp/4001/p2p/12D3KooW...

Creating test tensor...
Compressing tensor...
  Original: 8192 bytes
  Compressed: 2176 bytes
  Ratio: 3.77x

Sending tensor to peer...

  RECEIVED ACK:
  For tensor: gradient_layer_1
  Status: received

==============================
Tensor exchange successful!
```

**Machine A (Receiver):**
```
  RECEIVED TENSOR:
  Name: gradient_layer_1
  Shape: [32, 64]
  Metadata: training_step_42
  Compressed size: 2176 bytes
  Compression ratio: 3.77x

  DECOMPRESSED:
  Elements: 2048
  Mean: 0.0023
  Min:  -2.8134
  Max:  2.9012
  Sample values: [0.123, -0.456, ...]

  ACK sent to sender
```

## Test 2: DHT Key-Value Store

### Machine A

```bash
cargo run --release --example dht_node -- --listen 4001 --put mykey "Hello from Machine A"
```

Copy the bootstrap address.

### Machine B

```bash
cargo run --release --example dht_node -- \
  --bootstrap /ip4/<MACHINE_A_IP>/tcp/4001/p2p/<PEER_ID> \
  --get mykey
```

Should print: `SUCCESS: Retrieved 'Hello from Machine A'`

## Test 3: Peer Discovery

### Machine A (Provider)

```bash
cargo run --release --example peer_discovery -- --listen 4001 --provide inference:llama2
```

### Machine B (Discoverer)

```bash
cargo run --release --example peer_discovery -- \
  --bootstrap /ip4/<MACHINE_A_IP>/tcp/4001/p2p/<PEER_ID> \
  --find inference:llama2
```

Should print: `SUCCESS: Found 1 provider(s)`

## Troubleshooting

### Connection Refused
- Ensure firewall allows port 4001: `sudo ufw allow 4001/tcp` (Linux)
- Check Machine A is running and listening
- Verify IP address is correct (use local network IP, not 127.0.0.1)

### Timeout
- Machines must be on same network (or have public IPs)
- Check no NAT/firewall blocking the connection
- Try a different port if 4001 is in use

### Build Errors
- Ensure Rust is up to date: `rustup update`
- On macOS, may need Xcode tools: `xcode-select --install`
- On Linux, may need: `sudo apt install build-essential pkg-config libssl-dev`

## Network Configurations

### Same Local Network
Use local IPs (192.168.x.x, 10.x.x.x, etc.)

### Different Networks (Advanced)
Options:
1. Port forwarding on router
2. Use a VPN (Tailscale, WireGuard)
3. Cloud VM with public IP

## Quick Verification Script

Save as `test_connection.sh` on Machine B:

```bash
#!/bin/bash
MACHINE_A_ADDR="$1"

if [ -z "$MACHINE_A_ADDR" ]; then
  echo "Usage: ./test_connection.sh /ip4/<IP>/tcp/4001/p2p/<PEER_ID>"
  exit 1
fi

echo "Testing tensor exchange..."
cargo run --release --example tensor_exchange -- --connect "$MACHINE_A_ADDR" --send

echo ""
echo "Test complete!"
```

Run with:
```bash
chmod +x test_connection.sh
./test_connection.sh "/ip4/192.168.1.100/tcp/4001/p2p/12D3KooW..."
```
