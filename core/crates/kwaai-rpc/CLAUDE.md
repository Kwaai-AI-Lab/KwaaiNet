# kwaai-rpc crate

Thin crate: the gRPC IPC surface for the `kwaainet` daemon. Proto definitions live in
`proto/kwaai.proto`; `build.rs` generates tonic code and `lib.rs` re-exports it under a stable
path so callers never reach into `tonic::include_proto!` themselves.

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `proto/kwaai.proto` | The actual service + message definitions — edit here, not in generated code |
| `src/lib.rs` | Re-exports generated tonic code as `kwaai_rpc::v1` |
| `build.rs` | tonic-build codegen |

## Consumers

Server side lives in the daemon (`kwaai-cli/src/grpc_server.rs`); the client side is the Flutter
GUI and CLI talking over a Unix socket.

## prost version split

This crate's tonic-generated types are on **prost 0.13**, while `kwaai-p2p` and
`kwaai-hivemind-dht` are on **prost 0.12**. Crates depending on both alias them
(`prost013 = { package = "prost", version = "0.13" }`) — see `kwaai-network-tests/Cargo.toml`.

## Build

```bash
cargo build -p kwaai-rpc
cargo test -p kwaai-rpc
```
