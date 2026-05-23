# kwaai-p2p crate

This crate implements the P2P transport: libp2p Kademlia DHT (Hivemind-compatible), circuit relay
for NAT traversal, Yamux stream multiplexing, and the p2pd daemon.

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/network.rs` | libp2p swarm, behaviour, event loop |
| `src/dht.rs` | Kademlia DHT: find/put record |
| `src/hivemind.rs` | Hivemind DHT wire format (Ext64 msgpack) |
| `src/transport.rs` | Transport stack, relay, Yamux |
| `src/protocol.rs` | RPC protocol definitions |

## DHT compatibility

Hivemind responses: rt=1 FoundRegular, rt=2 FoundDictionary.
Unknown map keys in DHTServerInfo are silently ignored — safe to add new fields.

## Build

```bash
cargo build -p kwaai-p2p -p kwaai-p2p-daemon
cargo test -p kwaai-p2p
```
