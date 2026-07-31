# patches/

Build-time patched dependencies. Nothing here is vendored source: the repo
carries only the patch files and a checksum-pinned fetch script; the expanded
crate sources are produced locally and gitignored.

## multistream-select (slash-less protocol IDs)

`multistream-select 0.13.0` (from [rust-libp2p], MIT) with **one behavioral
change**, applied via `[patch.crates-io]` in `core/Cargo.toml`:

Upstream rejects any protocol name that does not start with `/`, in both
`TryFrom` impls in `src/protocol.rs`. The hivemind network negotiates **bare
handler names** (`DHTProtocol.rpc_store`, `hello`) as libp2p protocol IDs,
which go-libp2p accepts — the restriction is local to rust-libp2p, not part of
the wire protocol. The patch relaxes validation to what the message framing
actually requires: non-empty UTF-8 containing no newline.

Without it a native node can neither call a hivemind unary handler
(dialer-side rejection before anything is sent) nor serve one to a Go peer
(listener-side rejection while parsing the proposal). Interop proof that Go
peers negotiate slash-less IDs: `kwaai-network-tests/tests/07_wire_interop.rs`
(`slashless_protocol_negotiates`).

The entire delta is `multistream-select.patch` (~23 changed lines, one file).

### Fresh checkout

`cargo` cannot parse the workspace until the patched source exists:

```sh
bash core/patches/fetch-multistream-select.sh
```

`setup.sh` and every CI workflow run this automatically. The script pins the
crates.io tarball by sha256 and is an instant no-op once the source is present
and matches the patch.

### Upgrading / removing

- When bumping the workspace `libp2p` version, check which multistream-select
  version it pulls, update `VERSION`/`SHA256` in the fetch script, and re-apply
  (the patch is two small hunks; rebase it if upstream moved).
- If upstream relaxes the check (or makes it configurable — the intended
  endgame, see `docs/NATIVE_P2P_MIGRATION.md`), delete this directory and the
  `[patch.crates-io]` entry.

[rust-libp2p]: https://github.com/libp2p/rust-libp2p
