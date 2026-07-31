# multistream-select (patched)

Verbatim crates.io source of `multistream-select 0.13.0` (from the
[rust-libp2p](https://github.com/libp2p/rust-libp2p) repository, MIT license)
with **one behavioral change**, applied via `[patch.crates-io]` in
`core/Cargo.toml`.

## The change

Upstream rejects any protocol name that does not start with `/`, in both
`TryFrom` impls in `src/protocol.rs` (marked `KWAAI PATCH`). The hivemind
network negotiates **bare handler names** (`DHTProtocol.rpc_store`, `hello`) as
libp2p protocol IDs, which go-libp2p accepts — the restriction is local to
rust-libp2p, not part of the wire protocol. Validation is relaxed to what the
message framing actually requires: non-empty UTF-8 containing no newline.

Without this patch a native node can neither call a hivemind unary handler
(dialer-side rejection before anything is sent) nor serve one to a Go peer
(listener-side rejection while parsing the proposal).

Context: `docs/NATIVE_P2P_MIGRATION.md`, Phase 2. The interop proof that Go
peers negotiate slash-less IDs is `kwaai-network-tests/tests/07_wire_interop.rs`
(`slashless_protocol_negotiates`).

## Removing it

If upstream ever relaxes the check (or exposes it as a config), drop this
directory and the `[patch.crates-io]` entry. When bumping the workspace
`libp2p` version, check which `multistream-select` version it pulls and re-apply
the two-hunk patch to that version's source.
