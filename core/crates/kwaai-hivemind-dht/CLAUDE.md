# kwaai-hivemind-dht crate

Rust implementation of the Hivemind DHT protocol used by the Petals network: `DHTValue` wrappers
with expiration timestamps, the unified FIND RPC (value retrieval + routing in one call), batch
STORE, MessagePack for values, protobuf for RPC framing.

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/protocol.rs` | Protocol message definitions |
| `src/codec.rs` | Wire format for DHT RPC messages over libp2p |
| `src/value.rs` | `DHTValue` types and expiration handling |
| `src/client.rs` | `HivemindDHT` — get/store operations |
| `src/server.rs` | Responds to inbound FIND and STORE requests |
| `proto/` | Protobuf definitions for the RPC framing |

## Wire compatibility

Hivemind responses: `rt=1` FoundRegular, `rt=2` FoundDictionary. DHT RPC uses **raw prost with no
varint length prefix**. Unknown map keys in `DHTServerInfo` are silently ignored by legacy Hivemind
clients — new fields are safe to add.

This crate is on **prost 0.12**; `kwaai-rpc` is on 0.13. Crates using both must alias.

## Build

```bash
cargo build -p kwaai-hivemind-dht
cargo test -p kwaai-hivemind-dht
```
