# kwaai-network — Claude Code Instructions

## Project scope

kwaai-network owns the P2P transport fabric: libp2p with Kademlia DHT (Hivemind-compatible),
circuit relay for NAT traversal, Yamux stream multiplexing, the p2pd daemon, RPC protocol,
and intent-based routing for inference and knowledge requests. It does **not** own inference
logic, storage, or identity — it uses them.

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-p2p` | `core/crates/kwaai-p2p/` | libp2p stack, DHT, relay, transport |
| `kwaai-p2p-daemon` | `core/crates/kwaai-p2p-daemon/` | Long-running p2pd process |
| `kwaai-hivemind-dht` | `core/crates/kwaai-hivemind-dht/` | Hivemind DHT compatibility layer |
| `kwaai-rpc` | `core/crates/kwaai-rpc/` | RPC protocol definitions |

CLI entry points in `core/crates/kwaai-cli/src/`:
- `p2p_cmd.rs` — `kwaainet p2p` subcommands
- `node.rs` — `DHTServerInfo`, DHT announcement, `run_node()`, `VpkInfo`

## Infrastructure

The p2p daemon runs as a Unix domain socket server:
- Socket: `/tmp/kwaai-p2pd.sock` (constant `DEFAULT_SOCKET_NAME`)
- VPK discovery sends `DHTProtocol.rpc_find` for `_kwaai.vpk.nodes`
- Circuit relay enables residential NAT traversal without port forwarding

**Metro machines** (used for inference routing):
- `metro-linux` — A6000 GPU, Ubuntu — **DNS broken**: resolves to 192.168.1.1 (router)
- `metro-win` — A5000 GPU, Windows — **DNS broken**: resolves to 192.168.1.1 (router)
- Fix needed: p2p relay routing for inference requests (user intent: "use p2p relay")
- Ollama on both machines bound to `127.0.0.1:11434` — needs `OLLAMA_HOST=0.0.0.0` to accept remote connections

## Build & test

```bash
# Build network crates
cargo build -p kwaai-p2p -p kwaai-p2p-daemon -p kwaai-hivemind-dht -p kwaai-rpc

# Run p2pd daemon
kwaainet start --daemon

# P2P status
kwaainet p2p status

# Tests
cargo test -p kwaai-p2p
```

## Current state

**Shipped:**
- libp2p Kademlia DHT with Hivemind wire compatibility
- Circuit relay for NAT traversal
- Yamux stream multiplexing
- `DHTServerInfo` Ext(64) wire format with `start_block`, `end_block`, `peer_id`, `vpk` fields
- VPK discovery via `_kwaai.vpk.nodes` DHT record
- `shard chain` discovery via block DHT keys

**In progress / planned:**
- P2P relay routing for inference (route through p2p network instead of direct TCP)
- Intent-casting schemas (JSON/Protobuf intents and replies)
- `kwaainet_version` field in `DHTServerInfo` for peer version visibility

## Key source files

| File | Description |
|------|-------------|
| `kwaai-p2p/src/network.rs` | libp2p swarm, behaviour, event loop |
| `kwaai-p2p/src/dht.rs` | Kademlia DHT operations, find/put record |
| `kwaai-p2p/src/hivemind.rs` | Hivemind DHT wire format compatibility |
| `kwaai-p2p/src/transport.rs` | Transport stack, relay, Yamux |
| `kwaai-p2p/src/protocol.rs` | RPC protocol definitions |
| `kwaai-cli/src/node.rs` | DHT announcement, `DHTServerInfo::to_msgpack()` |
| `kwaai-cli/src/p2p_cmd.rs` | `kwaainet p2p` command handlers |

## DHT wire format

`DHTServerInfo`: `Ext(64, msgpack([state_i32, throughput_f64, {start_block, end_block, peer_id, public_name, vpk, ...}]))`

Unknown map keys are silently ignored by legacy Hivemind clients — safe to extend.

`_kwaai.vpk.nodes`: subkey=msgpack(peer_id_base58), value=msgpack({mode, endpoint, capacity_gb, tenant_count, vpk_version}), TTL=360s

## Do not

- Do not hardcode bootstrap peer addresses — they live in config
- Do not send inference requests directly over TCP to metro machines; use p2p relay
- Do not break Hivemind DHT wire format compatibility (rt=1 FoundRegular, rt=2 FoundDictionary)

## Full project context

`projects/kwaai-network/` — requirements, design docs, roadmap, TODO
