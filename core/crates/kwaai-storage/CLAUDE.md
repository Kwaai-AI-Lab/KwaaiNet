# kwaai-storage crate

This crate implements VPK (Virtual Private Knowledge): multi-tenant homomorphic-encrypted vector
storage, DHT advertisement, and VPK health check integration.

**Full project context:** `projects/kwaai-storage/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/api.rs` | VPK HTTP API handlers |
| `src/db.rs` | SQLite database layer |
| `src/tenant.rs` | Multi-tenant isolation |
| `src/vectors.rs` | Homomorphic vector search |

## Build

```bash
cargo build -p kwaai-storage
cargo test -p kwaai-storage
```
