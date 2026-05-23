# kwaai-network — TODO

## Bug fixes
- [ ] **Metro DNS broken** — `metro-linux` and `metro-win` both resolve to `192.168.1.1` (router). Need to discover actual IPs (ARP table, manual check) and persist in compute CLAUDE.md.
- [ ] **Ollama localhost binding** — metro machines need `OLLAMA_HOST=0.0.0.0` before Ollama can serve remote requests.

## Features
- [ ] P2P relay routing for inference — route `shard run` through libp2p circuit relay instead of direct TCP
- [ ] `kwaainet_version` field in `DHTServerInfo` wire format (backward-compatible)
- [ ] Intent-casting schema prototype (JSON schema for intents and replies)
- [ ] `kwaainet p2p peers` — show connected peers with trust tier and version

## Tests
- [ ] DHT announce + discover round-trip integration test (two local nodes)
- [ ] Circuit relay test: node behind NAT (simulated) connects via relay
- [ ] Hivemind compatibility test: FoundDictionary (rt=2) decode round-trip

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
- [ ] Document actual metro machine IPs once discovered
