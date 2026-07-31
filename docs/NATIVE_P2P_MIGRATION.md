# Native P2P Migration: Removing the Go p2p-daemon

Status: **in progress** — implementation on `feature/native-p2p`.

## Context

The `kwaainet` node currently runs **no in-process libp2p at all**. It spawns the Go
`go-libp2p-daemon` (hivemind fork, pinned `v0.5.0.hivemind1`) as a child process and drives
it over a protobuf control socket (`/tmp/kwaai-p2pd.sock` / `KWAAINET_SOCKET`). This makes Go
a hard build **and** runtime dependency (five separate build paths ship the binary), forces
awkward workarounds (full daemon restart to change announce addrs; `connect_peer()` crashing
p2pd goroutines, `node.rs:1635`; global kill-by-name of orphaned `p2pd` that can kill other
nodes' daemons), and leaves all NAT/relay/AutoNAT capability in Go. Goal: a native
rust-libp2p node, daemon removed entirely.

Note: `docs/FEATURE_GAP_ANALYSIS.md` and `docs/HIVEMIND_RUST_ARCHITECTURE.md` describe the
native-Rust stack as if it already exists — they are aspirational. This document reflects
the verified current state.

## Current state (verified)

- `kwaai-p2p-daemon` = p2pd control-protocol **client** (identify, connect/disconnect,
  list_peers, stream handlers, stream_open, persistent-conn unary handlers, DHT
  put/get/find_peer/find_providers/provide). Its `build.rs` clones + `go build`s p2pd
  (hard-fails without Go). Pubsub/connmanager/4 DHT verbs unimplemented — all unused.
- `kwaai-p2p` (rust-libp2p 0.53.2) = **dead prototype**: kad+identify+stub, TCP/noise/yamux,
  no event loop, DHT reads local-only, ephemeral keys. Only consumer is an example.
- `kwaai-hivemind-dht` = hivemind DHT protobuf types (`DHTProtocol.rpc_{ping,store,find}`,
  MessagePack values, SHA1(msgpack) 20-byte DHT IDs, f64 expiration). Gaps: storage ignores
  subkeys (Petals module records are DictionaryDHTValues — mandatory), never returns
  `FoundDictionary`, nearest-peers not XOR-sorted. Its 8-byte-BE+marker framing in
  `codec.rs` is a kwaai invention that does NOT match the hivemind wire — replace, don't
  reuse.
- `kwaai-cli/src/node.rs` run_node(): spawn p2pd → register handlers (rpc_* forwarded to a
  localhost TCP listener; unary handlers hello/ollama-proxy/shard-proxy/inference-mux) →
  bootstrap dial → IDENTIFY self-discovery → **restart p2pd** with discovered addrs →
  announce + 300 s re-announce loop, unannounce (state=-1) on shutdown.
- The p2pd socket is a **multi-process service bus**: map-server crawler, `shard serve`,
  `storage serve`, `rag`, `p2p`, `status`, vpk_bench, block_rpc, ollama_proxy, shard_api,
  rag_api, inference_mux all attach as clients from other processes — and some **register
  inbound handlers** (act as the node's peer identity). Any replacement must preserve this.
- Python side (bootstraps = upstream petals `run_dht` via hivemind pip, RSA-2048 Qm… peer
  IDs; health service) keeps its own p2pd — **out of scope**. Interop constraints:
  TCP+noise+yamux dialability, hivemind `DHTProtocol.rpc_*` protocols, Kademlia peer
  routing (`/ipfs/kad/1.0.0` — the go daemon uses defaults, no ProtocolPrefix), clean
  protocol refusal for the health probe.

## Feature gap: daemon features in use vs rust-libp2p

| Daemon feature in use | rust-libp2p status |
| --- | --- |
| TCP v4/v6 + noise + yamux | ✅ |
| Identify + observed-addr discovery | ✅ `libp2p-identify` |
| Kademlia server mode, `/ipfs/kad/1.0.0` | ✅ `libp2p-kad` defaults interop |
| Circuit relay v2 client + service | ✅ `libp2p-relay` |
| dcutr hole punching | ✅ `libp2p-dcutr` |
| AutoNAT (v1 — what go-libp2p speaks) | ✅ `libp2p-autonat` |
| UPnP (`-natPortMap`) | ✅ `libp2p-upnp` |
| **AutoRelay** (`-autoRelay -trustedRelays -relayDiscovery=false`) | ❌ **hand-roll** a RelayManager |
| `-forceReachabilityPrivate` (config `force_private`, default true) | replaced by explicit reachability state machine |
| RSA bootstrap peer IDs | ✅ `rsa` feature already enabled |
| Unary handlers / persistent conn | wire format resolved (below); becomes `request_response` behaviour |
| Pubsub, connmanager, QUIC | unused — out of scope |
| Hivemind DictionaryDHTValue/subkeys | ❌ gap in our own `kwaai-hivemind-dht` — must implement |
| 198.18.0.0/15 routability (nat-test bed) | rust-libp2p has **no default address blacklist** (`global_only` is opt-in) → the go-multiaddr patch apparatus retires |

## Critical wire-format finding (verified in proto + Go source)

Hivemind's unary RPC wrapper **is on the libp2p wire between peers**: one uvarint-delimited
`PersistentConnectionRequest{callId, callUnary{peer, proto, data}}` out, one uvarint-delimited
`PersistentConnectionRequest{callId, unaryResponse: CallUnaryResponse}` back, on a stream
whose protocol ID is the bare handler name (no leading slash, e.g. `DHTProtocol.rpc_store`).
`data` is the raw application protobuf.

**Live interop bug found:** `wrap_stream_handler_response`
(`kwaai-p2p-daemon/src/stream.rs:162-186`) writes a `PersistentConnectionResponse`
(oneof field 2 = `callUnaryResponse`), but Go callers decode the reply as
`PersistentConnectionRequest`, whose field 2 is `AddUnaryHandlerRequest` with a
proto2-`required` `balanced` field → unmarshal failure. Every inbound rpc_ping/store/find
served by a Rust node to a Go-daemon caller errors out today. The native codec must write
`PersistentConnectionRequest{callId, unaryResponse}`; Phase 0 proves this against a real
p2pd.

## Design decisions

1. **In-process swarm** lives in a gutted-and-rewritten `kwaai-p2p` crate (zero production
   consumers today; keep `config.rs` skeleton). New files: `behaviour.rs`, `service.rs`
   (single swarm task with command/handler-response/swarm-event/maintenance select loop),
   `handle.rs` (`NetworkHandle` — clonable facade mirroring `P2PClient` method names),
   `relay_manager.rs`, `addresses.rs`. Delete `network.rs`, `hivemind.rs`, `rpc.rs`,
   `dht.rs`, `protocol.rs`.

2. **Behaviour composition**: ping, identify, kad(MemoryStore, auto client/server mode +
   `dht_server` config override), autonat(v1), relay client, `Toggle<relay server>`
   (`!config.no_relay`), dcutr, `Toggle<upnp>`, a **hand-rolled `unary::Behaviour`** for
   ALL unary protocols, `libp2p_stream` for slashed raw-stream protocols (inference-mux,
   block_rpc). Transport: SwarmBuilder TCP+noise+yamux + dns + relay client. TCP-only
   (fleet is TCP). Workspace libp2p features add: `ping, dcutr, autonat, upnp, dns`.

   *(Revised in Phase 2 — the original plan said `request_response<HivemindUnaryCodec>`,
   which turned out to be structurally wrong for hivemind: its protocol list is fixed at
   construction and `send_request` cannot pick a protocol per request, while a hivemind
   call IS its protocol and Phase 3 registers handlers at runtime. `kwaai-p2p/src/unary.rs`
   keeps `request_response`'s architecture — pending queues, dial-on-demand, worker
   futures — with per-request negotiation via `UnaryProtocol(Arc<str>)` and a shared
   dynamic inbound protocol set. Slash-less negotiation additionally needs the vendored
   `multistream-select` patch; see Resolved verification items.)*

3. **IPC for external processes — node-hosted p2pd-protocol control socket.** The node
   serves the existing p2pd protobuf control protocol on the same socket path
   (`KWAAINET_SOCKET` / `/tmp/kwaai-p2pd.sock`, TCP 5005 on Windows), translating into
   `NetworkHandle` calls. All ~15 external client files (`client.rs`, `persistent.rs`,
   `dht.rs` and every call site) stay **byte-for-byte unchanged**; external processes keep
   acting as the node's peer identity, including `add_unary_handler` from
   `shard serve`/`storage serve` (handler ownership tracked per socket connection;
   deregister on disconnect — fixes today's stale-handler-after-crash bug). Serve only the
   used subset; error on pubsub/connmgr. New: `kwaai-p2p-daemon/src/server.rs`.

4. **Hivemind wire codec** in `kwaai-hivemind-dht/src/wire.rs` + minimal
   `proto/persistent_conn.proto` (wire-compatible subset: PersistentConnectionRequest,
   CallUnaryRequest/Response, Cancel, DaemonError; workspace prost 0.12 — kills the
   0.12/0.13 split rationale). Delete the 8-byte-BE/marker framing; keep DHT prost types.
   10 MiB inbound cap (parity). Fix the slashed constants in `lib.rs:31-33` (bare names).

5. **DHT semantics completion** in `kwaai-hivemind-dht/src/server.rs`:
   `Stored::{Regular, Dictionary(BTreeMap<subkey, (value, expiration)>)}`; per-subkey
   freshness merge; `FoundDictionary` responses (msgpack DictionaryDHTValue layout —
   verify against hivemind `dht/protocol.py` + live bootstrap capture); XOR-distance
   nearest-peer selection over hivemind node IDs, k=20, fed from kad routing-table events.
   Validators: follow-up, not parity-blocking.

6. **NAT traversal**: reachability state machine (Unknown/Public/Private; `force_private`
   short-circuits to Private = `-forceReachabilityPrivate` parity). AutoNAT probes against
   bootstraps; identify observed-addrs + UPnP external addrs are candidates, promoted to
   confirmed external addresses. RelayManager: reserve `/p2p-circuit` on up to 2 trusted
   relays (or hop-capable bootstraps) when Private; backoff + rotation on loss; dcutr
   upgrades relayed inbound to direct. **Announce lifecycle via
   `watch::Receiver<AnnounceState>` — the entire p2pd restart dance is deleted**
   (`restart_p2pd`, `restart_p2pd_with_addrs`, `discover_and_restart_with_announce`,
   relay-addr cache, p2pd heartbeat/watchdog select arms, `find_p2pd_binary`,
   `kill_orphaned_p2pd`).

7. **Identity**: pass `NodeIdentity` keypair (`kwaai-cli/src/identity.rs`, already
   Ed25519+RSA-capable protobuf encoding) to `SwarmBuilder::with_existing_identity`;
   consider moving the loader into `kwaai-p2p` for map-server reuse.

8. **Clean cut, no `--use-p2pd` runtime fallback** in shipped releases. Transition safety =
   unchanged IPC protocol + last p2pd-based release tag remains installable (GUI pins via
   `.kwaainet-version`) + dev-window flag on the feature branch only
   (`native_p2p` config / `KWAAINET_NATIVE_P2P=1`, flipped then removed at cutover).

9. **KwaaiNetGUI is runtime-transparent; the coupling is packaging-only.** No GUI source
   touches p2pd (the `kwaainet` binary spawns it). But the GUI release workflow extracts
   `p2pd`/`p2pd.exe` from the KwaaiNet release archive under strict error modes — an
   archive without p2pd hard-fails the GUI build. The "gate" is one small change: make the
   p2pd copy conditional-on-presence, landed before the first p2pd-free KwaaiNet tag;
   delete the conditional later.

10. **Forward-looking: Rust bootstrap nodes (design constraint, not in scope).** The
    OpenAI-Petal Python bootstraps are themselves slated for replacement by KwaaiNet, and
    the hivemind DHT is a known feature gap there. Decisions here must not paint us into a
    corner: implement DHT storage/serving to *bootstrap grade* — real subkey/dictionary
    semantics, XOR-sorted nearest peers fed from the kad routing table (not a flat list),
    capacity/eviction that survives being a network root, and the `dht_server` mode knob.
    Known additional gaps for a future Rust bootstrap, tracked but not built now: hivemind
    record validators/signatures, petals `ReachabilityProtocol` (bootstraps serve it via
    `attach_to_dht`), AutoNAT service + relay-hop service at bootstrap scale, RSA identity
    generation (loading already works), and the health service's
    `connect/disconnect/list_peers` probe surface.

## Phasing (each independently landable)

### Phase 0 — wire codec + interop proof (no behavior change)

`kwaai-hivemind-dht/src/wire.rs` + `proto/persistent_conn.proto`; golden-byte tests.
Interop test in `kwaai-network-tests`: real p2pd ↔ raw rust stream with new codec, both
directions. Hot-fix the `stream.rs` response-wrapper bug in the p2pd path regardless —
it's a live network bug. **This gates everything.**

*Testable:* golden-byte unit tests of frame encode/decode; p2pd-in-the-loop round trips
(rust codec calls a handler registered via P2PClient; p2pd `call_unary_handler` hits a
raw-rust-served stream); explicit repro-then-fix test for the response-wrapper bug;
empirical check that multistream-select negotiates the slash-less `DHTProtocol.rpc_ping`.

*Expected issues:* proto2 `required` fields vs prost defaults (missing `balanced`/`callId`
must fail loudly, not default); uvarint framing edge cases (multi-byte lengths, partial
reads); the possibility that some deployed peer *depends* on the buggy responder format
(unlikely — Go callers error out — but check the map-server crawler's find path);
hivemind's Python client setting fields the Go daemon overwrites (e.g. `peer` in
CallUnary) — match Go behavior, not Python's.

### Phase 1 — swarm skeleton

Rewrite `kwaai-p2p` (ping/identify/kad only), `NetworkService`/`NetworkHandle`, identity
reuse from `kwaai-cli/src/identity.rs`.

*Testable:* two-in-process-swarm unit tests (dial, identify exchange, kad lookup); against
the live network or compose stack: dial bootstraps (RSA Qm… peers), `list_peers` shows
them, kad FIND_PEER resolves a known peer, observed addrs captured from identify.

*Expected issues:* RSA handshake with the Python bootstraps (rust-libp2p `rsa` feature is
enabled but this path has never run in production); kad protocol-config drift (confirm
`/ipfs/kad/1.0.0` and default query parallelism against go-libp2p-kad-dht); event-loop
starvation mistakes (blocking calls inside the select loop); channel deadlocks between
`NetworkHandle` calls and swarm events (command/oneshot leak on error paths).

### Phase 2 — hivemind unary RPC native

`request_response<HivemindUnaryCodec>` behaviour + handler dispatch; DHT serving + announce
via `NetworkHandle`.

*Testable:* node announces to bootstraps and shows on the health map; `rpc_find` against a
Python bootstrap returns our record; a p2pd-based caller successfully calls our `rpc_ping`
(previously broken inbound path); re-announce loop and clean unannounce (state=-1)
observed on the bootstrap.

*Expected issues:* `UnaryProtocol(Arc<str>)` slash-less negotiation end-to-end; per-request
timeout semantics vs today's 30 s `send_to_bootstrap`; large-message caps (10 MiB parity);
connect-before-call semantics (`call_unary_handler` must dial+resolve like Go's
`host.NewStream`, or announces to not-yet-connected bootstraps fail); TTL/expiration
skew — bootstraps reject TTLs shorter than the existing record (`node.rs:1381` note).

### Phase 3 — IPC control-socket server + conformance rig

`kwaai-p2p-daemon/src/server.rs`; rework `kwaai-network-tests/src/harness.rs`
(`new_relay_server`/`new_dht_client`/`new_nat_client` spawn kwaainet nodes or in-process
swarms instead of `DaemonBuilder`); existing client-side test bodies become the
conformance suite — old tests, new server.

*Testable:* the full existing daemon-client test suite green against the new server;
multi-client smoke: `kwaainet status`, `p2p peers list`, `shard serve` (register handler +
receive inbound call), `storage serve`, inference-mux `stream_open_raw`, two clients
concurrently; handler deregistration on client disconnect (kill `storage serve`, verify
the protocol is refused afterward).

*Expected issues:* **riskiest sub-piece** — bidirectional callUnary routing across
concurrent socket clients (handler ownership, disconnect cleanup, UUID call-ID collisions
between independent clients); `stream_open_raw` pipe-mode fidelity (raw byte relay between
unix socket and libp2p stream, backpressure both ways); subtle p2pd response-shape details
existing clients depend on (error encodings, IdentifyResponse addr byte format); Windows
TCP-socket path parity.

### Phase 4 — node.rs integration behind flag + NAT traversal

`native_p2p` selects the new path in run_node; hello/ollama-proxy/shard-proxy via
`add_unary_handler`, inference-mux via `accept_streams`. autonat + upnp + RelayManager +
dcutr + announce watch channel.

*Testable (kwaaiai-env nat-test topology):* (a) direct node announces
public_ip:public_port (incl. node-d asymmetric port map), (b) NATed↔NATed dcutr
hole-punch (verify direct transport addr, not just connectivity), (c) symmetric-NAT node
relay-only via trusted relay, (d) address change re-announces without restart, (e)
unchanged map-server crawler sees all nodes (proves IPC compat end-to-end), (f) one
p2pd-based node kept in the topology for the mixed-fleet window, fully interoperating.

*Expected issues:* hand-rolled RelayManager edge cases (reservation refresh on relay
restart, candidate rotation, duplicate circuit listens); dcutr success rate vs go-libp2p
(measure, don't assume); AutoNAT v1 dialback against Python-bootstrap p2pd (if unanswered,
identify-confirmation fallback must engage); RFC2544 addresses being classified
unreachable somewhere in rust-libp2p (autonat/kad address scoring) — the nat-test bed will
surface it; announce flapping if the address-book confirmation threshold is too eager.

### Phase 5 — DHT parity + cutover + teardown

- Dictionary/subkeys/XOR landed and verified against hivemind Python.
- Flip default; delete: p2pd spawn path + watchdogs + `daemon.rs` + `kill_orphaned_p2pd` +
  setup.rs p2pd download (`setup --get-deps` → no-op success) + `setup.sh:265`.
- `kwaai-p2p-daemon/build.rs` → protoc-only (delete Go check/clone/build/P2PD_PATH); keep
  crate as the IPC client+server (`client.rs`/`persistent.rs`/`dht.rs`/`server.rs`/proto).
- Nix: delete `nix/p2pd.nix`; `flake.nix` drop p2pd outputs; `crane.nix` drop
  `P2PD_BIN`/stub/`$out/bin/p2pd` symlink (keep pre-generated proto copy); `Makefile` p2pd
  targets.
- cargo-dist: `core/Cargo.toml` extra-artifacts, `scripts/build-p2pd.sh`, `release.yml`
  setup-go + both p2pd-injection steps (incl. dist-manifest hash patching — native
  checksums become correct again).
- **GUI packaging tolerance (runtime is transparent):** make KwaaiNetGUI release
  workflow's p2pd copy conditional-on-presence FIRST, then tag KwaaiNet vNEXT, then bump
  `.kwaainet-version`; delete the conditional later. Also drop `p2pd` reaping in
  OpenAI-Petal installers (`daemon_utils.py`, `Installer/*/kwaainet/daemon.py`).
- kwaaiai-env: delete `docker/p2pd-patched/` + `patches/go-multiaddr/allow-rfc2544.patch`,
  remove `p2pd-patched`/`nat-test-overlay-p2pd` services + depends_on edges + all
  `GOLOG_LOG_LEVEL` envs (→ `RUST_LOG`); **keep** the bootstrap p2pd overlay patch (Python
  side still needs it); update `patches/README.md`, `docs/nat-test-topology.md`, nat-test
  node configs (preserve port/public_ip/public_port/announce_addr semantics).

*Testable:* dictionary/subkey golden tests against hivemind Python semantics (capture
`_petals.models` / `{prefix}.{block}` records from a live bootstrap; two peers storing
under the same key accumulate, not overwrite); XOR nearest-peer ordering unit tests; full
workspace builds **without a Go toolchain**; untar each release archive → `kwaainet` only,
no `p2pd`, checksums valid without hash-patching; `kwaainet setup --get-deps` succeeds as
a no-op; GUI CI dry-run against the release-candidate tag; final nat-test + compose
acceptance re-run.

*Expected issues:* hivemind's "regular store" subkey sentinel and DictionaryDHTValue
msgpack layout not matching assumptions (verify against `hivemind/dht/protocol.py` first);
nix/crane stub still referencing P2PD env vars breaking sandbox builds; cargo-dist
regeneration clobbering hand-edited release.yml sections; stale `.kwaainet-version` or
Homebrew-tap references to p2pd checksums; OpenAI-Petal installers killing the new
single-binary node if reaping logic is only half-removed.

**Riskiest steps:** (1) Phase 0's interop gate — if the wire assumptions are wrong nothing
else lands; (2) Phase 3's persistent-conn server semantics for concurrent external
clients; (3) dcutr/relay behavior parity under real NATs (mitigated by Phase 4 acceptance
gates).

## Open verification items

- Hivemind's "regular store" subkey sentinel + exact DictionaryDHTValue msgpack layout
  (read hivemind `dht/protocol.py`; golden-capture against a live bootstrap).
- AutoNAT v1 service actually answering on bootstraps (fallback: identify-confirmation
  path already kept; `force_private=true` default makes this low-risk).
- Health-probe "protocol not supported" string vs rust-libp2p's unknown-protocol error
  surface (the health patch matches on a substring).

## Resolved verification items (Phase 0, `07_wire_interop` against a real p2pd)

- Slash-less protocol IDs negotiate fine on the go-libp2p wire
  (`slashless_protocol_negotiates`) — but rust-libp2p's *local* validation rejects them
  at three points in `multistream-select` (dial-side and listen-side `Protocol::try_from`
  plus the proposal-recognition rule in `Message::decode`). Resolved by a vendored
  two-file patch: `core/patches/multistream-select/` via `[patch.crates-io]`. The wire
  format itself is unaffected.
- The wire wrapper is exactly as described above (both directions proven; the old
  `PersistentConnectionResponse` reply shape is provably rejected by Go callers).
- **proto2 `required` is enforced by Go on unmarshal**: a frame omitting an empty
  `peer`/`data` field is dropped and the stream reset. The wire.rs prost types carry
  `required` labels so empty fields are still encoded.
- **`callUnary.peer` is only rewritten to the caller's ID on the unary-handler dispatch
  path.** On a raw stream handler it arrives exactly as the caller sent it (the callee's
  ID, per Go's caller convention). A native responder must take caller identity from the
  libp2p connection, never from this field.

## Critical files

- `core/crates/kwaai-p2p/src/*` — rewritten (service/handle/behaviour/relay_manager/addresses)
- `core/crates/kwaai-hivemind-dht/src/{wire.rs(new),codec.rs,server.rs,lib.rs}`
- `core/crates/kwaai-p2p-daemon/src/{server.rs(new),daemon.rs(delete),stream.rs(delete at cutover)}`, `build.rs`
- `core/crates/kwaai-cli/src/{node.rs,daemon.rs,setup.rs,identity.rs}`
- `core/crates/kwaai-network-tests/src/harness.rs`
- `{flake.nix,nix/p2pd.nix,nix/crane.nix,Makefile,scripts/build-p2pd.sh,.github/workflows/release.yml,core/Cargo.toml}`
- kwaaiai-env: `{docker-compose.yml,docker/p2pd-patched/,patches/go-multiaddr/,docker/nat-test/config-node-*.yaml,docs/nat-test-topology.md}`
- KwaaiNetGUI: `.github/workflows/release.yml` (p2pd copy conditional)
