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
output is gitignored, so `distrib/nix/crane.nix` materializes each patched crate from
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

We need it for the kad protocol migration, and **only the bootstrap-grade
build uses it**: `cargo build --release -p kwaainet --features
kad-multi-protocol` serves `/kwaai/kad/1.0.0` *and* the legacy
`/ipfs/kad/1.0.0`, bridging peers that predate the kwaai name while the
fleet upgrades. A **stock build serves the kwaai name alone** and never
touches this patched API — that is what keeps `cargo publish` working, and
it means a bootstrap built with default features silently cuts off every
legacy peer; the migration-window bootstraps must be the feature build.
Serving the legacy name on a public address is what let the global IPFS DHT
absorb the old bootstraps (2026-08-31: several hundred foreign peers each,
p2pd OOM-killed every 30–90 min), which is why the stock default is
kwaai-only and there is no runtime knob (`docs/BOOTSTRAP.md` has the
rollout order). The rename ends recruitment as a *DHT server* — the
routing-table/OOM vector — but foreign peers still connect below kad, so
connection tables fill regardless; that half belongs to KwaaiNet#174.

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

### libp2p-kad: FIND_NODE answers from a peerstore

Upstream serves an inbound FIND_NODE from its k-buckets alone, so a peer
that never got a bucket slot — every fleet peer, once the buckets filled
with Amino entries — is unfindable while it sits connected to the node being
asked. go-libp2p's `handleFindPeer` includes the target "if present in
peerstore, even if it is self, the requester, or not a DHT server". The patch
adds `Behaviour::set_peerstore_addresses` and prepends the target to the
reply when the owner has vouched for it; kwaai-p2p feeds it each connected
peer's identify addresses and a circuit through itself for peers holding a
relay reservation here. Same `libp2p-kad.patch`, regenerated with
`diff -u` against the pristine crate.

## cudarc (sync-allocation opt-out)

`cudarc 0.19.7` (from [cudarc], MIT/Apache-2.0) with one four-line change:
the runtime choice between `cuMemAllocAsync` and `cuMemAlloc` also honours the
environment variable `CUDARC_DISABLE_ASYNC_ALLOC`. Upstream picks the async,
stream-ordered allocator whenever the device reports memory-pool support and
offers no way to say no. On Jetson Orin (L4T r36 / CUDA 12.6) that pool
hard-caps at roughly a third of system memory — 2.5 GB on a 7 GB Orin Nano,
~20 GB on a 64 GB AGX Orin (NVIDIA/TensorRT-LLM#10894) — while plain
`cuMemAlloc` reaches nearly all of it, so an 8-block f16 shard (4.5 GB) fails
to load with `CUDA_ERROR_OUT_OF_MEMORY`. With the variable set, allocation is
synchronous and the shard loads. Unset, the build is byte-for-byte upstream
behaviour on every platform. Only pulled in by the CUDA features.

`kwaai-inference` sets the variable itself on a Jetson (Linux aarch64 with
`/etc/nv_tegra_release`) when it is unset, so a service-managed Orin needs no
unit-file edit. CI checks that the patch is still what cargo resolves, so a
candle bump past cudarc 0.19 fails there instead of silently dropping it.

[cudarc]: https://github.com/coreylowman/cudarc
