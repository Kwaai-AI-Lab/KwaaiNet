# kwaai-p2p-daemon crate

Rust wrapper around the `go-libp2p-daemon` binary (p2pd), which runs as a separate process and
gives us full Hivemind/Petals DHT compatibility. Communication is over IPC: named pipes on
Windows (`//./pipe/name`), Unix domain sockets elsewhere (`/tmp/name.sock`).

**Full project context:** `projects/kwaai-network/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/daemon.rs` | `P2PDaemon` lifecycle — spawn, monitor, shut down the p2pd process |
| `src/client.rs` | Async IPC client over pipe/socket |
| `src/dht.rs` | High-level DHT operations |
| `src/persistent.rs` | Hivemind's unary handler pattern over go-libp2p-daemon |
| `src/stream.rs` | Decoding daemon-forwarded streams |
| `src/hello.rs`, `src/protocol.rs`, `src/error.rs` | Handshake, protocol IDs, errors |
| `proto/` | go-libp2p-daemon protobuf definitions |

## Gotchas

Default socket is `/tmp/kwaai-p2pd.sock` (`DEFAULT_SOCKET_NAME`). macOS caps unix-socket paths at
104 bytes — the binding constraint when running two nodes on one Mac.

p2pd is bundled into release archives and installers; see `projects/kwaai-platform/` for that
pipeline. The native-libp2p migration that would replace this crate is tracked in
`projects/kwaai-network/plans/`.

## Build

```bash
cargo build -p kwaai-p2p-daemon
cargo test -p kwaai-p2p-daemon
```
