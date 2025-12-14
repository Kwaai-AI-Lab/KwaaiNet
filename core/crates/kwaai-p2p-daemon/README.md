# kwaai-p2p-daemon

Rust wrapper for [go-libp2p-daemon](https://github.com/learning-at-home/go-libp2p-daemon), providing full compatibility with the Hivemind/Petals DHT network.

## Prerequisites

This crate **requires Go 1.13 or later** to build. The build process will compile the go-libp2p-daemon from source.

### Installing Go

#### Windows
1. Download installer from: https://golang.org/dl/
2. Run the installer (go1.x.x.windows-amd64.msi)
3. Verify installation: `go version`

#### Linux
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install golang-go

# Fedora/RHEL
sudo dnf install golang

# Arch Linux
sudo pacman -S go
```

#### macOS
```bash
brew install go
```

### Verifying Installation

After installing Go, verify it's in your PATH:

```bash
go version
# Should output: go version go1.x.x ...
```

## How It Works

1. **Build Time**: The `build.rs` script:
   - Checks for Go toolchain
   - Clones go-libp2p-daemon repository
   - Compiles the p2pd daemon binary
   - Generates Rust protobuf code from p2pd.proto
   - Places binary in `target/<profile>/p2pd(.exe)`

2. **Runtime**: The Rust code:
   - Spawns the p2pd daemon process
   - Communicates via IPC (named pipes on Windows, Unix sockets on Linux/macOS)
   - Sends protobuf messages for DHT operations, stream handling, etc.

## Usage

```rust
use kwaai_p2p_daemon::P2PDaemon;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start the daemon
    let daemon = P2PDaemon::builder()
        .dht(true)
        .relay(true)
        .spawn()
        .await?;

    // Get a client to communicate with it
    let mut client = daemon.client().await?;

    // Use the client
    let peer_id = client.identify().await?;
    println!("Our peer ID: {}", peer_id);

    // DHT operations
    let key = b"/my-namespace/my-key".to_vec();
    let value = b"my-value".to_vec();

    // Store value in DHT
    client.dht_put_value(key.clone(), value, Some(60)).await?;

    // Retrieve value from DHT
    let result = client.dht_get_value(key, Some(30)).await?;
    println!("Retrieved value: {:?}", result.value);

    // Keep daemon running
    daemon.wait().await?;
    Ok(())
}
```

## Features

### Basic Operations
- ✅ **IDENTIFY**: Get our peer ID
- ✅ **CONNECT**: Connect to a peer
- ✅ **DISCONNECT**: Disconnect from a peer
- ✅ **STREAM_OPEN**: Open a stream to a peer

### DHT Operations
- ✅ **PUT_VALUE**: Store a value in the DHT
- ✅ **GET_VALUE**: Retrieve a value from the DHT
- ✅ **FIND_PEER**: Find peer addresses
- ✅ **FIND_PROVIDERS**: Find content providers
- ✅ **PROVIDE**: Announce content provision

### Future Operations
- ⏳ **STREAM_HANDLER**: Register protocol handlers
- ⏳ **PUBSUB**: Pub/sub messaging

## Testing

After installing Go, test the daemon:

```bash
cd core

# Basic daemon test
cargo run --example daemon_test

# DHT operations test
cargo run --example dht_test
```

**daemon_test** will:
1. Build the Go daemon from source
2. Spawn the daemon process
3. Connect via IPC
4. Send an IDENTIFY request
5. Verify the response

**dht_test** will:
1. Spawn daemon with DHT enabled
2. Test PUT_VALUE operation
3. Test GET_VALUE operation
4. Demonstrate DHT usage patterns

## Platform Support

- **Windows**: ✅ TCP socket (`/ip4/127.0.0.1/tcp/5005`) - Go daemon doesn't support Windows named pipes in multiaddr format
- **Linux**: ✅ Unix domain sockets (`/unix/tmp/kwaai-p2pd.sock`)
- **macOS**: ✅ Unix domain sockets (`/unix/tmp/kwaai-p2pd.sock`)

## Architecture

```
┌─────────────────────────────────────────────┐
│  Your Rust Application                      │
│  ┌────────────────────────────────────────┐ │
│  │  kwaai-p2p-daemon (Rust)               │ │
│  │  • Daemon lifecycle                    │ │
│  │  • Protobuf client                     │ │
│  │  • Async API                           │ │
│  └─────────────┬──────────────────────────┘ │
│                │ IPC (named pipe / socket)   │
└────────────────┼─────────────────────────────┘
                 │
┌────────────────▼─────────────────────────────┐
│  go-libp2p-daemon (Go)                       │
│  • libp2p networking                         │
│  • Protocol negotiation                      │
│  • DHT operations                            │
│  • Stream multiplexing                       │
│  • NAT traversal / Relay                     │
└──────────────────────────────────────────────┘
                 │
                 │ libp2p protocols
                 ▼
      [ Petals Network Nodes ]
```

## Why This Approach?

Instead of reimplementing the Hivemind protocol in pure Rust (which would require matching exact protocol negotiation behavior), we wrap the battle-tested Go daemon that's already used by Petals nodes. This gives us:

✅ **100% Petals compatibility** (uses same daemon as Python)
✅ **No Python dependency** (Rust manages daemon lifecycle)
✅ **Production-ready** (tested by thousands of Petals nodes)
✅ **Full feature set** (DHT, relay, NAT traversal, etc.)

## License

MIT
