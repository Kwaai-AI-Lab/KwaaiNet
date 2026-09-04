# kwaai-p2p-daemon

The p2pd control-socket protocol, in both directions: the client external
processes use to drive a running node, and the server the node itself hosts.

The wire format is the go-libp2p-daemon protobuf control protocol, which keeps
the socket compatible with the Hivemind/Petals tooling that speaks it.

## Prerequisites

Rust only. `build.rs` compiles the vendored `proto/p2pd.proto` with
`prost-build`, downloading `protoc` into `OUT_DIR` when it is not on PATH.

## Halves

- **Client** (`client.rs`, `persistent.rs`, `dht.rs`, `stream.rs`) — identify,
  connect, list_peers, DHT verbs, stream handlers, and the persistent-connection
  unary sub-protocol.
- **Server** (`server.rs`) — the node's own implementation, translating the same
  bytes into `kwaai_p2p::NetworkHandle` calls. External processes (`shard serve`,
  `storage serve`, `rag`, `p2p`/`status`, inference-mux, the map's DHT crawler)
  attach to the socket and act as the node's peer identity.

## Usage

```rust,no_run
use kwaai_p2p_daemon::{P2PClient, DEFAULT_SOCKET_NAME};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Attach to the socket a running node serves.
    let mut client = P2PClient::connect(DEFAULT_SOCKET_NAME).await?;

    let peer_id = client.identify().await?;
    println!("Our peer ID: {}", peer_id);

    let key = b"/my-namespace/my-key".to_vec();
    client
        .dht_put_value(key.clone(), b"my-value".to_vec(), Some(60))
        .await?;
    let result = client.dht_get_value(key, Some(30)).await?;
    println!("Retrieved value: {:?}", result.value);
    Ok(())
}
```

## Operations

- **Basic**: IDENTIFY, CONNECT, DISCONNECT, STREAM_OPEN, STREAM_HANDLER
- **DHT**: PUT_VALUE, GET_VALUE, FIND_PEER, FIND_PROVIDERS, PROVIDE
- **Persistent**: unary call/response over a single connection

## Platform Support

- **Windows**: TCP on loopback (`/ip4/127.0.0.1/tcp/5005`)
- **Linux/macOS**: Unix domain sockets (`/unix/tmp/kwaai-p2pd.sock`)

## Testing

```bash
cargo test -p kwaai-p2p-daemon
```

`tests/control_server.rs` drives the real `P2PClient` against a `ControlServer`
backed by a real `NetworkService`.

## License

MIT
