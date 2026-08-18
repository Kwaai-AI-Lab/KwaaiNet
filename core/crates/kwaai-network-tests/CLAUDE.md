# kwaai-network-tests crate

Connectivity, uptime, and robustness test suite for the network pillar. Not a library others
depend on — `publish = false`, it exists to exercise `kwaai-p2p`, `kwaai-hivemind-dht`,
`kwaai-p2p-daemon`, and `kwaai-rpc` together.

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/harness.rs` | `TestNode` — spins up p2pd instances, each with its own tmpdir/socket path |
| `src/metrics.rs` | Appends one JSON line per run to `tests/kwaai-network/results/metrics.jsonl` |
| `src/bin/metrics_report.rs` | `metrics-report` binary — summarises the metrics JSONL |
| `tests/01_unit_dht.rs` … `03_unit_rpc.rs` | Unit tier: codec, value, storage, protocol |
| `tests/04_integration_daemon.rs`, `05_integration_relay.rs` | Daemon spawn, DHT, relay topology |
| `tests/06_network_bootstrap.rs` | Live bootstrap + peer-count metrics |

## Three test tiers — all off by default

| Tier | Gate | What runs |
|------|------|-----------|
| unit | (always) | codec, value, storage, protocol |
| integration | `KWAAI_INTEGRATION_TESTS=1` | daemon spawn, DHT, relay topology |
| network | `KWAAI_NETWORK_TESTS=1` | real bootstrap, peer count metrics |

macOS caps unix-socket paths at 104 bytes — the harness's per-node tmpdir exists to stay under it.

## Build

```bash
cargo test -p kwaai-network-tests                                          # unit only
KWAAI_INTEGRATION_TESTS=1 cargo test -p kwaai-network-tests                # + daemon spawn
KWAAI_INTEGRATION_TESTS=1 KWAAI_NETWORK_TESTS=1 cargo test -p kwaai-network-tests   # all tiers
```
