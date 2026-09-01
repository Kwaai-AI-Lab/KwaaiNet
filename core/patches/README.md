# patches/

Build-time patched dependencies. Nothing here is vendored source: the repo
carries only the patch files and checksum-pinned fetch scripts; the expanded
crate sources are produced locally and gitignored.

## Fresh checkout

`cargo` cannot parse the workspace until *every* patched source exists, so run
the umbrella script — never a per-crate one:

```sh
bash core/patches/fetch-patches.sh
```

`setup.sh` and every CI workflow run this automatically. Each per-crate script
pins its crates.io tarball by sha256 and is an instant no-op once the source is
present and matches the patch.

A per-crate script run on its own **succeeds and exits 0** while leaving the
other patched crates absent; the omission surfaces much later as a workspace
parse failure with nothing pointing back at the patch step. They warn about
this when invoked directly.

Nix does not run these scripts: a flake build reads the git tree and their
output is gitignored, so `nix/crane.nix` materializes each patched crate from
the same pinned tarball and patch file. Adding a patched crate means adding it
there too.

## libp2p-kad (multi-protocol names)

`libp2p-kad 0.48.0` (from [rust-libp2p], MIT) with **one API restoration**,
applied via `[patch.crates-io]` in `core/Cargo.toml`:

Kad's negotiation machinery holds a `Vec<StreamProtocol>` and offers every
entry on both inbound and outbound streams, but upstream removed the public
`set_protocol_names` setter, leaving `Config::new` with a single name. The
patch restores the setter on `Config` (and adds one on `ProtocolConfig`,
where the field lives) — no behavioral change to negotiation itself.

We need it for the kad protocol migration: nodes serve and offer
`/kwaai/kad/1.0.0` *and* the legacy `/ipfs/kad/1.0.0`
(`NetworkConfig::kad_protocols`), so upgraded peers negotiate the kwaai name
while peers that predate it still match on the legacy one. Serving the legacy
name on a public address is what let the global IPFS DHT absorb the
bootstraps (2026-08-31: several hundred foreign peers per bootstrap, p2pd
OOM-killed every 30–90 min), so bootstrap-grade nodes configure the kwaai
name alone — which is only possible if the protocol list is settable.

The entire delta is `libp2p-kad.patch` (two added methods, two files).

### Upgrading / removing the kad patch

Same drill as multistream-select below; if upstream reintroduces a public
multi-name API, delete `libp2p-kad.patch`, `fetch-libp2p-kad.sh`, the
`[patch.crates-io]` entry, and the line in `fetch-patches.sh`.

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

### Upgrading / removing

- When bumping the workspace `libp2p` version, check which multistream-select
  version it pulls, update `VERSION`/`SHA256` in the fetch script, and re-apply
  (the patch is two small hunks; rebase it if upstream moved).
- If upstream relaxes the check (or makes it configurable — the intended
  endgame, see `docs/NATIVE_P2P_MIGRATION.md`), delete this directory and the
  `[patch.crates-io]` entry.

[rust-libp2p]: https://github.com/libp2p/rust-libp2p
