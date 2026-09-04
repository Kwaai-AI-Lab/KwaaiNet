# kwaai-p2p crate

This crate implements the P2P transport: libp2p Kademlia DHT (Hivemind-compatible), circuit relay
for NAT traversal, and Yamux stream multiplexing.

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/service.rs` | `NetworkService` — owns the swarm and its event loop |
| `src/behaviour.rs` | The composed libp2p `NetworkBehaviour` for a node |
| `src/handle.rs` | `NetworkHandle` — clonable facade over the swarm task |
| `src/dht_service.rs` | Serving the hivemind DHT natively: `rpc_ping` / `rpc_store` / `rpc_find` |
| `src/unary.rs` | Hivemind unary RPC as a `NetworkBehaviour` |
| `src/raw_stream.rs` | Raw libp2p streams as a `NetworkBehaviour` |
| `src/transport.rs` | Transport stack, relay, Yamux |
| `src/relay_manager.rs` | Circuit relay reservation and lifecycle |
| `src/addresses.rs` | Multiaddr construction and parsing |
| `src/reachability.rs` | NAT/reachability detection |

Hivemind DHT wire format lives in the `kwaai-hivemind-dht` crate, not here.

## DHT compatibility

Hivemind responses: rt=1 FoundRegular, rt=2 FoundDictionary.
Unknown map keys in DHTServerInfo are silently ignored — safe to add new fields.

## Build

```bash
cargo build -p kwaai-p2p -p kwaai-p2p-daemon
cargo test -p kwaai-p2p
```
