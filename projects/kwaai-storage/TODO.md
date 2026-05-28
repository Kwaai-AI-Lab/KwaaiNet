# kwaai-storage — TODO

## Features
- [ ] `vpk discover` — complete DHT integration (stub exists in `vpk.rs`)
- [ ] `vpk shard` — Phase 2: cross-node shard placement with trust-weighted policies
- [ ] `vpk resolve` — Phase 3: DHT-backed knowledge base resolution
- [ ] PHE repo: `peer_id`/`mode` config fields, `tenant_id` DB column, health endpoint additions

## Tests
- [ ] VPK health check integration test (use `tests/kwaai-storage/mock-vpk-health.py`)
- [ ] Multi-tenant isolation test (tenant A cannot read tenant B's vectors)
- [ ] DHT announce + discover round-trip for VPK nodes

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
