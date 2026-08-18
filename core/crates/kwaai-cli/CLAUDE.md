# kwaainet crate (kwaai-cli)

The `kwaainet` binary — every user-facing command, the node runtime, and the daemon. The package
is named **`kwaainet`**, not `kwaai-cli`; build with `-p kwaainet`.

This crate is cross-cutting: the platform pillar owns its shell (dispatch, config, install,
update, display), while each domain pillar owns its own command modules. See the ownership split
below before editing.

**Full project context:** `projects/kwaai-platform/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Platform-owned source files

| File | Description |
|------|-------------|
| `src/main.rs` | Dispatch for every subcommand |
| `src/cli.rs` | clap `Command` enum + `Args` structs |
| `src/config.rs` | `KwaaiNetConfig` — YAML at `~/.kwaainet/config.yaml` |
| `src/updater.rs` | Self-update / release channel |
| `src/service.rs` | Auto-start service management |
| `src/setup.rs`, `src/uninstall.rs` | Install and removal flows |
| `src/daemon.rs` | Daemon process lifecycle |
| `src/display.rs` | `print_box_header/success/error/info/warning/separator` |
| `src/progress.rs`, `src/monitor.rs`, `src/health.rs` | Progress bars, monitoring, health checks |
| `src/api.rs`, `src/map.rs` | Local HTTP API, map.kwaai.ai client |

## Modules owned by other pillars

| Pillar | Modules | Project docs |
|--------|---------|--------------|
| Trust | `identity.rs`, `trust.rs`, `reputation.rs`, `reputation_cmd.rs`, `ledger_cmd.rs`, `ledger_node.rs` | `projects/kwaai-trust/` |
| Network | `p2p_cmd.rs`, `node.rs`, `grpc_server.rs` | `projects/kwaai-network/` |
| Compute | `shard_cmd.rs`, `shard_api.rs`, `block_rpc.rs`, `inference_mux.rs`, `rebalancer.rs`, `calibration.rs`, `capacity_lease.rs`, `throughput.rs`, `hf.rs`, `llama_local.rs`, `ollama.rs`, `ollama_proxy.rs`, `circuit_breaker.rs` | `projects/kwaai-compute/` |
| Storage | `vpk.rs`, `vpk_bench.rs`, `storage.rs`, `storage_rpc.rs` | `projects/kwaai-storage/` |
| Knowledge | `rag_cmd.rs`, `rag_api.rs` | `projects/kwaai-knowledge/` |

## Adding a subcommand

Add `Args` + `Action` to `cli.rs`, create the handler module, add `mod X;` and dispatch in
`main.rs`. Follow the `identity.rs` pattern: `pub async fn run(args: XArgs) -> Result<()>`.
Config fields use `#[serde(default)]` with `skip_serializing_if = "Option::is_none"` for optionals.

## Build

```bash
cd core && cargo build -p kwaainet --release
cp core/target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet   # macOS 26+ required
```

Always test with the built binary before committing, tagging, or pushing.
