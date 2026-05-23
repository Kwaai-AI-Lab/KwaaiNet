# kwaai-storage — Claude Code Instructions

## Project scope

kwaai-storage owns Virtual Private Knowledge (VPK): multi-tenant homomorphic-encrypted vector
storage bound to node identity, cross-node shard placement (planned), and DHT-backed knowledge
base resolution (planned). It does **not** own the RAG/knowledge pipeline (kwaai-knowledge)
or P2P transport (kwaai-network) — VPK uses both.

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-storage` | `core/crates/kwaai-storage/` | VPK: multi-tenant vectors, homomorphic search |

CLI entry points in `core/crates/kwaai-cli/src/`:
- `vpk.rs` — `kwaainet vpk enable/disable/status/discover/shard/resolve`

The PHE (Personal Homomorphic Encryption) service runs as a **separate process** (separate repo)
bound to the node's `PeerId`.

## VPK roles

| Role | Description |
|------|-------------|
| `bob` | Personal knowledge base (private) |
| `eve` | Encrypted inference — shares with trusted peers |
| `both` | Combined personal + encrypted inference |

## Build & test

```bash
# Build storage crate
cargo build -p kwaai-storage

# VPK status
kwaainet vpk status

# Run mock VPK health server (for testing)
python3 tests/kwaai-storage/mock-vpk-health.py  # serves on port 7432

# LAN test (two machines: machine-a and machine-b)
bash tests/kwaai-storage/vpk-lan-test.sh
```

## DHT wire format

`_kwaai.vpk.nodes` key: subkey=msgpack(peer_id_base58),
value=msgpack({mode, endpoint, capacity_gb, tenant_count, vpk_version}), TTL=360s

VPK health endpoint: `GET /api/health` → `{status, version, peer_id, tenant_count, capacity_gb_available}`

## Current state

**Shipped:**
- VPK process binds to node `PeerId`
- Multi-tenant vector tables with homomorphic-encrypted search
- DHT advertising: `_kwaai.vpk.nodes` record
- `kwaainet vpk enable/disable/status` commands
- `vpk_info` field in `DHTServerInfo` wire format

**In progress / planned:**
- `vpk discover` — query DHT for available VPK nodes (stub implemented, needs DHT integration)
- `vpk shard` / `vpk resolve` — Phase 2/3 cross-node placement and DHT resolution
- Database migration: `tenant_id` column on documents, index_mapping, audit_log in PHE repo
- `peer_id`, `mode` fields in PHE `src/config.rs`

## PHE repo changes needed (separate repo, not this one)

- `src/config.rs`: add `peer_id`, `mode` fields
- DB migration: tenant_id column
- `src/api/mod.rs`: tenant_id param on all endpoints
- `src/mcp/server.rs`: tenant_id on all four MCP tools
- `GET /api/health`: return status, version, peer_id, tenant_count, capacity_gb_available

## Key source files

| File | Description |
|------|-------------|
| `kwaai-storage/src/api.rs` | VPK HTTP API handlers |
| `kwaai-storage/src/db.rs` | SQLite database layer |
| `kwaai-storage/src/tenant.rs` | Multi-tenant isolation |
| `kwaai-storage/src/vectors.rs` | Homomorphic vector search |
| `kwaai-cli/src/vpk.rs` | `kwaainet vpk` command handlers |
| `kwaai-cli/src/node.rs` | `VpkInfo`, `check_vpk_health()`, DHT announcement |

## Do not

- Do not merge PHE repo changes into this repo — they are a separate service
- Do not store unencrypted vectors for `eve`/`bob` modes
- Do not change VPK DHT record TTL without updating both announcement and discovery code

## Full project context

`projects/kwaai-storage/` — requirements, design docs, roadmap, TODO
