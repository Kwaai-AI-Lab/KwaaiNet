# kwaai-network — Roadmap

## Shipped

- libp2p Kademlia DHT, Hivemind wire compatibility (rt=1 FoundRegular, rt=2 FoundDictionary)
- Circuit relay for residential NAT traversal
- Yamux stream multiplexing
- p2pd daemon via Unix domain socket
- `DHTServerInfo` Ext(64) msgpack wire format with `start_block`, `end_block`, `peer_id`, `vpk` fields
- VPK node advertisement: `_kwaai.vpk.nodes` DHT record
- Shard chain discovery via block DHT keys
- Intent-based routing for inference: model + trust tier + latency constraints → shard chain selection
- `kwaainet p2p status`

## In progress

- **P2P relay routing for inference** — route `shard run` requests through the p2p network rather than direct TCP, so metro machines with localhost-bound Ollama can still serve requests. DNS issue: `metro-linux`/`metro-win` both resolve to `192.168.1.1`. Fix: route inference via libp2p circuit relay.
- `kwaainet_version` field in `DHTServerInfo` — surface peer build version in `shard chain` output

## Planned

- **Intent-casting schemas** — JSON/Protobuf schemas for machine-readable intents (needs, offers, goals) and responses
- **Intent bus prototype** — narrow use case: evaluation infrastructure intent broadcast
- Person-anchored identity binding for intents (legal responsibility)
- Governance patterns for neutral routing (prevent routing capture)

## Research / future

- Economic settlement layer for intent fulfillment
- Cross-network federation (connect KwaaiNet DHT to other open networks)
- WebRTC mesh for browser-to-browser P2P
