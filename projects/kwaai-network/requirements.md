# kwaai-network — Requirements

## Purpose

Provide a decentralized, Hivemind-compatible P2P fabric so KwaaiNet nodes can discover each other,
route inference and knowledge requests, and traverse residential NAT — without central servers.

## Functional requirements

1. **FR-N1** Operate a Kademlia DHT that is wire-compatible with the Python Hivemind DHT (rt=1 FoundRegular, rt=2 FoundDictionary).
2. **FR-N2** Announce node presence on DHT using `DHTServerInfo` Ext(64) msgpack wire format; re-announce on TTL expiry.
3. **FR-N3** Support circuit relay so nodes behind residential NAT can receive inbound connections.
4. **FR-N4** Run as a long-lived daemon (`p2pd`) via Unix domain socket (`/tmp/kwaai-p2pd.sock`).
5. **FR-N5** Expose `kwaainet p2p status` showing active peers, DHT records, and relay circuits.
6. **FR-N6** Advertise VPK nodes under `_kwaai.vpk.nodes` DHT key; support `vpk discover` command.
7. **FR-N7** Support shard chain discovery: find all `{prefix}.{n}` DHT keys and recover peer IDs.
8. **FR-N8** Support intent-based routing: "model X, minimum trust tier T" → shard chain selection.

## Non-functional requirements

- **NFR-N1** DHT announce must complete in < 5s on a stable network.
- **NFR-N2** Circuit relay handshake must complete in < 2s on residential broadband.
- **NFR-N3** p2pd must handle ≥ 100 concurrent connections without degradation.
- **NFR-N4** Adding new fields to `DHTServerInfo` must not break legacy Hivemind peers (unknown keys silently ignored).
- **NFR-N5** Targets: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows.

## Out of scope

- Inference execution — that is kwaai-compute.
- Storage encryption — that is kwaai-storage.
- Intent-casting schemas with economic settlement — planned, not current sprint.

## Dependencies

- **kwaai-trust** — PeerId derived from identity keypair; trust tier used in routing decisions.
