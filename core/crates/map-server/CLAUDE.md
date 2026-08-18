# map-server crate

Public API behind map.kwaai.ai. A background task crawls the DHT via the running p2pd every 60 s
and refreshes an in-memory node cache; axum serves stats, the node list, and a live WebSocket
stream off that cache.

**Full project context:** `projects/kwaai-platform/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/main.rs` | axum server wiring, background crawler task |
| `src/crawler.rs` | Dials each bootstrap peer via p2pd every 60 s |
| `src/cache.rs` | `NodeCache` — in-memory nodes with TTL eviction |
| `src/routes.rs` | HTTP + WebSocket handlers |
| `src/state.rs` | Shared application state |

## Endpoints

| Route | Description |
|-------|-------------|
| `GET /api/stats` | Aggregated network stats (node count, tps, coverage) |
| `GET /api/nodes` | All known peers with trust tier + shard info |
| `WS /api/live` | Real-time stats stream (5 s deltas) |

A peer missing from map.kwaai.ai means the crawler could not reach it — that is a reachability
signal, not proof the node crashed.

## Build

```bash
cargo build -p map-server
cargo test -p map-server
```
