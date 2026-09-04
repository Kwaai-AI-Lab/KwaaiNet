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
   ALL unary protocols, and a hand-rolled `raw_stream::Behaviour` for raw-stream
   protocols (inference-mux, block_rpc, the `rpc_*` forwarding path). *(Revised in Phase 3
   — the original plan said `libp2p_stream`, which cannot express slash-less protocol
   IDs for the same reason it was ruled out for unary. `raw_stream` reuses
   `unary::Protocols` for negotiation and diverges after it; see Phase 3 below.)*
   Transport: SwarmBuilder TCP+noise+yamux + dns + relay client. TCP-only
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

*Done so far:* `unary::Behaviour` (see the revision note under design decision 2) and its
**handler dispatch through `NetworkHandle`**. `KwaaiBehaviour` composes a `unary` field
built from `NetworkConfig::request_timeout`; the service loop owns a protocol → sender
dispatch map kept in lockstep with the behaviour's inbound protocol set, and the handle
exposes `call_unary_handler` / `add_unary_handler` / `remove_unary_handler` under their
`P2PClient` names. Outbound calls need no pending-map entry — `send_request` owns the
oneshot and resolves it on every path — and inbound dispatch uses an unbounded channel plus
one task per call, so neither a slow handler nor a dead one can stall the select loop.
Gated by `kwaai-p2p/tests/service_unary.rs` and `09_service_unary_interop.rs` (a full
`NetworkService` against a real p2pd, both directions).

*Also done:* **DHT serving** (`kwaai-p2p/src/dht_service.rs`) — `spawn_dht_service`
registers `DHTProtocol.rpc_{ping,store,find}` as unary handlers over
`kwaai-hivemind-dht`'s `DHTStorage`, plus a 60 s maintenance task that feeds the
storage's routing table from `NetworkHandle::routing_peers()` (a new kad
k-bucket read) and runs `cleanup_expired()`. Nothing called `update_peers` or
`cleanup_expired` before. `rpc_ping` answers `available = false` — hivemind sets
true only after dialing the caller back, which is Phase 4 reachability work; the
field feeds the *caller's* confidence in its own reachability, so under-claiming
is the safe direction. Gated by `kwaai-p2p/tests/dht_service.rs` (in-process) and
`10_native_dht_interop` (a real p2pd storing into and reading from our native
node — the direction the `stream.rs` bug used to break).

**Announce records** are extracted into `kwaai-cli/src/announce.rs`:
`build_announce_records` / `build_unannounce_records` / `send_records_via_handle`
alongside the record value types moved verbatim from `node.rs`. The rewire landed in
Phase 4 slice 1: `run_node` uses these when `native_p2p` is set, on the same
300 s ± 30 s cadence, and the p2pd path still uses its own.

*Still open in this phase:* the announce **watch channel** — re-announcing on an address
change rather than only on the timer — which belongs with the address discovery deferred
to the Phase 4 NAT slice.

#### Verified live against the production bootstraps

`kwaai-p2p/tests/live_dht_announce.rs` (gated `#[ignore]` **and**
`KWAAI_LIVE_DHT_WRITE=1`, because it writes to the real DHT) ran green:

- **The RSA dial path works.** Both Python bootstraps (`Qm…`, RSA-2048) complete
  the noise handshake from rust-libp2p in ~200 ms. This was the single biggest
  Phase 1 unknown and it is now closed empirically. Identify reports
  `agent_version=p2pd/0.1` and a protocol list without any `DHTProtocol.*` entry
  (those are served by the Python process behind the daemon, not advertised).
- **A Python bootstrap accepts our native `rpc_store`**: `store_ok = [true]`,
  ~170 ms.
- **A subkeyed record reads back as rt=2 `FOUND_DICTIONARY`** with the subkey and
  value byte-identical, ~167 ms.
- **The tombstone rule is confirmed as documented**: a store with an expiration
  *strictly greater* than the live record replaces it. A store can never delete —
  the key remains `FOUND`, only the value changes.

Two findings that contradict prior assumptions:

- **`rpc_store` is single-hop.** A record stored on bootstrap #1 is *not* visible
  on bootstrap #2. It writes to the receiving node and replicates nothing;
  hivemind's `DHTNode.store` gets redundancy by calling `rpc_store` on each of
  the `k` nearest nodes from the **client** side. Today's `send_to_bootstrap`
  fan-out to every bootstrap is therefore load-bearing, not belt-and-braces —
  removing it in favour of "store once and let the DHT propagate" would silently
  halve our record redundancy.
- **The live bootstraps answer `NOT_FOUND` with an empty nearest-peers list.**
  Hivemind's `rpc_find` does attach nearest peers (`protocol.py:362-364`) and our
  `DHTStorage` does too, so this reflects those deployments' routing tables
  rather than the protocol. Worth knowing before relying on a bootstrap to route
  an iterative lookup: it will not.

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

*Done so far:* **`ControlServer`** (`kwaai-p2p-daemon/src/server.rs`) — binds the p2pd
control socket (`/unix/<path>`, plus the `/ip4/…/tcp/<port>` form the Windows client
already speaks), accepts concurrent clients, and translates into `NetworkHandle`. Verb
coverage:

| verb | status |
| --- | --- |
| IDENTIFY, CONNECT, DISCONNECT, LIST_PEERS | served |
| DHT FIND_PEER | served (via `dht_find_peer`) |
| PERSISTENT_CONN_UPGRADE + add/remove_unary_handler, call_unary_handler, unaryResponse | served |
| PERSISTENT `cancel` | accepted as a no-op — `NetworkHandle` has no mid-flight abort; calls are bounded by `request_timeout` |
| STREAM_OPEN, STREAM_HANDLER, REMOVE_STREAM_HANDLER | served — pipe mode, see below |
| DHT put/get/provide/find_providers/get_closest/search/pubkey | **stubbed** `"not supported"` (Go's own wording for unsupported DHT verbs) |
| PUBSUB, CONNMANAGER | **stubbed** `"not supported"` — never implemented by the client either |

Response shapes follow the Go source rather than the proto, because several are
load-bearing and invisible in the schema: the add/remove-handler ACK is a
`PersistentConnectionResponse` with a `callId` and **no** message arm (Go's
`okUnaryCallResponse`), which is exactly what `persistent.rs` decodes as success;
`IdentifyResponse.id` is raw peer-ID bytes and `addrs` are binary multiaddrs; simple verbs
error as `Response{ERROR}` while persistent verbs error as `daemonError`.

**Handler ownership is per socket connection** and released on disconnect for any reason —
this is the stale-handler-after-crash fix. `balanced` is deliberately ignored (no call site
passes `true`; honouring it would mean a round-robin fairness policy for an unused mode).
Call-ID correlation state is per connection so two clients cannot collide on a UUID, and
inbound dispatch mints its own call IDs so two remote callers cannot either.

*Also fixed here:* a Phase 2 bug this work surfaced — `unary::Handler` correlated outbound
streams to pending requests **by position**, but `FullyNegotiatedOutbound` arrives in
completion order, not emission order. Two concurrent calls to *different* protocols on one
connection could therefore swap replies. Now matched by negotiated protocol.
Gated by `service_unary.rs::concurrent_calls_to_different_protocols_do_not_cross_talk`.

*Also done (slice 2):* **pipe mode** — `STREAM_OPEN` / `STREAM_HANDLER` /
`REMOVE_STREAM_HANDLER`, the raw byte relay `inference_mux` and `node.rs`'s `rpc_*`
forwarding run on.

The libp2p half is a new `kwaai-p2p/src/raw_stream.rs`: a sibling of `unary::Behaviour`
that reuses its `Protocols` upgrade and `UnaryProtocol` (so slash-less names still
negotiate, which `libp2p_stream` cannot express) and then hands the negotiated `Stream`
out untouched — no framing, no per-call timeout, no callId correlation. A raw stream may
live for a node's whole session, which is exactly the lifetime model the unary path exists
to prevent, so folding a "raw mode" flag into `unary::Handler` would have put two unrelated
lifetime policies behind one set of queues. `NetworkHandle` gains `open_raw_stream` /
`accept_streams` / `remove_stream_handler`; the unary path is untouched.

The **fd handoff** was the restructure slice 1 flagged. A successful `STREAM_OPEN` is
terminal for the connection exactly as `PERSISTENT_CONN_UPGRADE` is (Go writes the
response, calls `doStreamPipe(c, s)`, returns — `conn.go:59-73`), so the socket's two
halves must be rejoined into one duplex stream. Slice 1 boxed the write half behind
`Arc<Mutex<Box<dyn AsyncWrite>>>`, and a boxed trait object cannot be reunited with
anything. Resolved by keeping the socket's *concrete* type — a `ClientSocket` enum over
Unix/TCP that splits inside `ConnState` and reunites on the way into pipe mode — with the
writer slot becoming an `Option` that pipe mode takes. A late frame write then finds `None`
and errors, which is correct: the socket is no longer a frame channel.

Backpressure both ways is `tokio::io::copy_bidirectional`, whose flow control *is* awaiting
each write before reading more, so a slow consumer stops the producer at the socket rather
than accumulating in-process; yamux's window applies on top. Half-closes propagate, so a
client that signals "request complete" by closing its write half still receives its reply.
`STREAM_HANDLER` dials the client's listener per inbound stream and writes the
length-delimited `StreamInfo` prologue every consumer already parses; a dial-back failure
**resets** the inbound stream, matching Go's `handleStream`.

Stream-handler registrations are **owned by the connection** and released on disconnect —
a deliberate divergence from Go, whose `d.handlers` map is process-global and outlives the
registering client, so a crashed `shard serve` there leaves the daemon advertising a
protocol whose forwarding address refuses connections (every inbound stream then costs a
dial timeout instead of a negotiation refusal). This matches the unary discipline from
slice 1.

*Gated by:* `kwaai-p2p/tests/raw_stream.rs` (12 tests: slash-less negotiation, protocol
preference lists, 4 MiB each way concurrently, half-close as EOF with the reverse direction
live, concurrent streams, dial-on-demand, clean refusals),
`kwaai-p2p-daemon/tests/control_server.rs` (19 in-process tests driving the **unmodified**
`P2PClient` against a `ControlServer` over a real `NetworkService` — 8 of them pipe mode,
including a 2 MiB relay through two `copy_bidirectional` hops and the dial-back reset),
`11_control_server_interop` (the unary cross-implementation matrix against a real p2pd) and
`12_pipe_mode_interop` (the same matrix for pipe mode: native client → p2pd stream handler,
p2pd client → native stream handler, `StreamInfo` prologue parity, and 1 MiB each way
across the boundary where `io.Copy` meets `copy_bidirectional`).

*Still open in this phase:* threading the caller's `PeerId` into `add_unary_handler_boxed`
so the dispatched `callUnary.peer` can carry it (Go rewrites that field,
`persistent_stream.go:298`; we currently send it empty) — note the *stream* path already
carries it, since `StreamInfo.peer` comes from the connection; the harness `TestNode`
variant that runs the *existing* client-side tiers against the new server; and deciding per
remaining DHT verb whether to back it with `kad` record/provider APIs or delete the client
method at cutover.

*Testable:* the full existing daemon-client test suite green against the new server;
multi-client smoke: `kwaainet status`, `p2p peers list`, `shard serve` (register handler +
receive inbound call), `storage serve`, inference-mux `stream_open_raw`, two clients
concurrently; handler deregistration on client disconnect (kill `storage serve`, verify
the protocol is refused afterward).

*Expected issues, and how they landed:* **riskiest sub-piece** — bidirectional callUnary
routing across concurrent socket clients (handler ownership, disconnect cleanup, UUID
call-ID collisions between independent clients): resolved in slice 1 by per-connection
correlation state. `stream_open_raw` pipe-mode fidelity (raw byte relay, backpressure both
ways): resolved in slice 2; the real obstacle turned out not to be the relay but the **fd
handoff** — slice 1's boxed writer could not be reunited with its reader — see the slice-2
notes above. Subtle p2pd response-shape details existing clients depend on (error
encodings, IdentifyResponse addr byte format): covered by the interop tiers. **Windows
TCP-socket path parity remains untested** — the `ClientSocket` TCP arm exists and the relay
is transport-agnostic, but every test binds a unix socket; tracked in the follow-ups above.

### Phase 4 — node.rs integration behind flag + NAT traversal

`native_p2p` selects the new path in run_node; hello/ollama-proxy/shard-proxy via
`add_unary_handler`, inference-mux via `accept_streams`. autonat + upnp + RelayManager +
dcutr + announce watch channel.

*Done so far (slice 1 — the run_node integration):* **`native_p2p`** (`kwaai-cli/src/
config.rs`, default false, settable via `kwaainet config set`) selects
`node_native::run_native_node`. `NativeNode::start` assembles what Phases 1–3 built:
the swarm on the configured port, `spawn_dht_service` over a `DHTStorage` (bootstrap-grade
serving, the native equivalent of p2pd's `-b`), this node's own handlers, and a
`ControlServer` on the same socket path p2pd would have used — resolved through the same
`KWAAINET_SOCKET`-or-default rule every client uses, so the GUI, `kwaainet p2p …`,
`shard serve` and the map crawler are unchanged. **No p2pd binary need be present.**

Handler registration by protocol:

| protocol | p2pd path | native path |
| --- | --- | --- |
| `DHTProtocol.rpc_{ping,store,find}` | TCP listener registered as a *stream handler*; p2pd forwards each request with a `StreamInfo` prologue and a `PersistentConnectionRequest` wrapper | `spawn_dht_service` → `add_unary_handler`, in-swarm, no wrapper |
| `/kwaai/p2p/hello/1.0.0` | `P2PClient::add_unary_handler` | `NetworkHandle::add_unary_handler` |
| `/kwaai/ollama-proxy/1.0.0`, `/kwaai/shard-proxy/1.0.0` | same | same, same handler bodies |
| `/kwaai/inference-mux/1.0.0` | TCP listener + `register_stream_handler`, prologue consumed per stream | `accept_streams`, the libp2p stream directly |

The **peer ID is identical on both paths** — same key file, same libp2p protobuf encoding
(`kwaai-p2p::identity` and `kwaai-cli::NodeIdentity` are compatible on-disk by
construction), so a migrating node keeps its DID, its credentials and its map entry.
Announce and unannounce go through `announce.rs` byte-for-byte on the same 300 s ± 30 s
jittered cadence against the 360 s TTL, with the `state = -1` tombstone on clean shutdown,
and the per-bootstrap timings still feed the reputation store.

Watchdog machinery is **absent, not stubbed**: no `find_p2pd_binary`, no crash detection,
no `restart_p2pd*`, no 10 s heartbeat, no 60 s socket keepalive — there is no child
process. Everything genuinely shared is shared rather than copied (`SigHup`, the Ollama
watcher, the auto-update respawn, and the announce-input helpers were extracted from
`run_node` in the same series), and `run_node`'s prologue and tail cover both paths.

Gated by `13_native_node_assembly` — two native nodes where one announces and the other
serves the record back as `FOUND_DICTIONARY` with the value byte-identical, an unchanged
`P2PClient` doing identify + a unary round trip against the assembled node's own control
socket, a check that the socket and the swarm are *one* node (a `ControlServer` on a
different handle would pass every tier-11 test and still be broken), and the key-file
identity property. A `kwaainet run-node` spawn test was deliberately skipped: it writes
the real PID file, binds the real socket and takes the fixed gRPC port, so it collides
with a developer's running node and with a second copy of itself, and would add only
argument parsing and the config round trip over what tier 13 covers.

*Deferred to slice 2 (the NAT slice) — a native node is currently reachable only if it is
directly dialable:*

- **autonat + upnp + RelayManager + dcutr.** `no_relay`, `force_private` and
  `trusted_relays` are inert on the native path and documented as such on the config field.
- **IDENTIFY-driven address discovery and the announce watch channel.** The p2pd path
  polls observed addresses, restarts the daemon with new announce addrs, and defers that
  restart while RPC streams are in flight. Natively there is no restart to defer — an
  address change should re-announce in place — so the whole `discover_and_restart_with_announce`
  / `pending_restart` / `collect_observed_addresses` cluster has no native counterpart yet.
  Until it lands, a native node announces `announce_addr`/`public_ip` or warns that it has
  none, and `using_relay` is simply "no address configured" rather than
  `all_addrs_are_relay` over discovered addresses.
- **`rpc_ping` `validate=true` dial-back** (tracked in the follow-ups above) — the
  reachability state machine it belongs to is this slice.

*API gaps in `kwaai-p2p` the NAT slice will need:* nothing blocked slice 1, but the native
path found no equivalent of p2pd's announce-address override — `NetworkConfig` has
`listen_addrs` but no "advertise this instead" field, so `announce_addr`/`public_ip`
currently only reach the DHT record and never the swarm's own address book. The NAT slice
needs that (external-address confirmation) plus a `NetworkHandle` event stream for address
changes, since today's `observed_addrs()`/`listen_addrs()` are poll-only.

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

- AutoNAT v1 service actually answering on bootstraps (fallback: identify-confirmation
  path already kept; `force_private=true` default makes this low-risk).
- Health-probe "protocol not supported" string vs rust-libp2p's unknown-protocol error
  surface (the health patch matches on a substring).

## Tracked follow-ups (from the Phase 2/3 adversarial review)

- **Record validators/signatures** (`kwaai-hivemind-dht/src/server.rs` module docs):
  hivemind gates writes behind `RecordValidator`; this port does not. The primary tier
  is capacity-bounded as an interim abuse guard — a Rust bootstrap must not ship
  without validators.
- **Eviction index**: `LocalStorage::enforce_capacity` is O(n) per eviction under the
  write lock (Python uses a heap). Add an expiration-ordered index before bootstrap-scale
  load.
- **`rpc_ping` `validate=true`** returns `available=false` without the reverse-ping;
  affects only caller routing confidence. Revisit with the reachability work (Phase 4).
- **`callUnary.peer` on IPC-dispatched inbound calls** is sent empty by the
  ControlServer (Go rewrites it to the caller's ID). No current handler reads it; thread
  the caller PeerId through before any handler authenticates callers.
- **Windows TCP parity for pipe mode is untested.** `ClientSocket` has a TCP arm and the
  relay is transport-agnostic, but every test in this tier binds a unix socket. The
  Windows client path (`/ip4/127.0.0.1/tcp/5005`) needs a run on Windows CI before the
  cutover, or at minimum a TCP-bound variant of the `control_server.rs` pipe tests.
- **Stream-handler `balanced` is ignored**, as on the unary path: Go keeps a round-robin
  list of forwarding addresses per protocol, we keep one owner and refuse the second. No
  call site in this codebase passes `true`.

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

- `core/crates/kwaai-p2p/src/*` — rewritten (service/handle/behaviour/unary/raw_stream/relay_manager/addresses)
- `core/crates/kwaai-hivemind-dht/src/{wire.rs(new),codec.rs,server.rs,lib.rs}`
- `core/crates/kwaai-p2p-daemon/src/{server.rs(new),daemon.rs(delete),stream.rs(delete at cutover)}`, `build.rs`
- `core/crates/kwaai-cli/src/{node.rs,node_native.rs(new),announce.rs,config.rs,inference_mux.rs,daemon.rs,setup.rs,identity.rs}`
- `core/crates/kwaai-network-tests/src/harness.rs`
- `{flake.nix,nix/p2pd.nix,nix/crane.nix,Makefile,scripts/build-p2pd.sh,.github/workflows/release.yml,core/Cargo.toml}`
- kwaaiai-env: `{docker-compose.yml,docker/p2pd-patched/,patches/go-multiaddr/,docker/nat-test/config-node-*.yaml,docs/nat-test-topology.md}`
- KwaaiNetGUI: `.github/workflows/release.yml` (p2pd copy conditional)

## Phase 4 slice 2 — NAT traversal design findings (2026-07-31)

Source-verified design pass over libp2p 0.53.2 (autonat 0.12 / relay 0.17.2 / dcutr 0.11 /
upnp 0.2.2), the old p2pd spawn flags, and the kwaaiai-env nat-test topology, ahead of
implementation. These findings correct several Phase 4 "expected issues" and set the
implementation slicing.

### Ground truths (from source, not assumption)

- **G1 — the announce record carries no multiaddrs.** `DHTServerInfo` has no address
  field; peers resolve addresses via Kademlia + identify. The "announce watch channel"
  therefore only carries `using_relay` + an announceable flag — address publication is
  `Swarm::add_external_address` + identify push (already enabled). This shrinks the
  flapping surface dramatically and removes the restart-to-change-addrs motivation.
- **G2 — relay reservation refresh-on-expiry is already in libp2p** (renew at 3/4 TTL,
  keep-alive while live). Only relay-*restart* recovery is ours to build.
- **G3 — reservation loss surfaces as `SwarmEvent::ListenerClosed`**, not as a
  relay-client event (`relay::client::Event` has no failure variant). Refusal/timeout →
  `ListenerClosed{reason: Err}`; relay connection death → `ListenerClosed{reason: Ok}`.
  The RelayManager is keyed on `ListenerId`, which makes duplicate circuit listens
  structurally impossible.
- **G4 — autonat 0.12 never calls `ExternalAddrConfirmed`**; confirming a
  `NatStatus::Public(addr)` into the swarm's address set is our job. (relay-client and
  upnp do confirm their own addrs.)
- **G5 — exactly two RFC2544 (198.18/15) rejection sites in all of rust-libp2p 0.53**:
  autonat's `is_benchmarking` behind `Config::only_global_ips` (default true), and a upnp
  gateway check that is unreachable in the nat-test bed (no SSDP responder). kad,
  identify, swarm, core, dcutr: zero address-class filtering (greps recorded). The whole
  "RFC2544 classified unreachable somewhere" worry reduces to one config bool.
- **G6 — identify advertises `listen ∪ external` addresses with no filter knob**, so a
  NATed node leaks RFC1918 listen addrs to every peer (see risk R1).
- **G7 — the nat-test trusted relay is node-a**, a KwaaiNet node, not a bootstrap; the
  production bootstraps have a documented RESERVATION_REFUSED history. Refusal is a
  normal outcome, not an error.
- **G8 — p2pd was never spawned with a hole-punching flag**, so there is no dcutr parity
  baseline; reframe acceptance as an absolute floor (≥7/10 cone-NAT upgrades over 10
  trials, measured and recorded, not gating).
- **G9 — `force_private` defaults to true** in kwaai-cli config; Private is the *default*
  reachability state and AutoNAT is a promotion mechanism for opt-outs only.

### Design decisions

- **Behaviour additions**: `autonat` (client+server; `only_global_ips` from new
  `only_global_ips` config, default false; boot_delay 5s, retry 30s, refresh 5min,
  keep `confidence_max 3` as the flap damper), `relay_client` (via
  `SwarmBuilder::with_relay_client` — the `with_behaviour` closure becomes two-arg),
  `Toggle<relay::Behaviour>` hop server (default on, parity with `!no_relay`),
  `dcutr`, `Toggle<upnp>` (off in tests).
- **RelayManager** lives inside `NetworkService` as a plain state machine polled from the
  select loop (needs `&mut Swarm` + swarm-level listener events). Slots keyed by
  `ListenerId`; one reservation per relay; backoff `min(30s·2^(n−1), 15min)` ±20% jitter
  with rotation orthogonal to backoff. Candidates: configured `trusted_relays` first,
  then identify-discovered hop-capable peers (protocol list contains the hop protocol) —
  explicitly *not* kad routing-table probing (that is the `-relayDiscovery` we always
  disabled). Trusted relays are dialed alongside bootstraps to cut reservation latency.
- **Reachability state machine** (`Unknown | Public{addr, source} | Private`, sources
  `Declared > AutoNat > Upnp > IdentifyConsensus`): declared `external_addr` pins Public
  and outranks `force_private` (with a warning); autonat Private demotes
  IdentifyConsensus but never Declared; upnp expiry returns to Unknown, not Private.
  Identify-consensus fallback: after a 45s grace still Unknown, promote the
  highest-distinct-observer announceable observed addr if it has ≥
  `identify_min_confirmations` observers, else Private. Whether bootstraps answer
  autonat dialbacks is the one empirical unknown — measured by a read-only live identify
  snapshot as implementation commit 1; the design is safe either way because
  `use_connected: true` makes every autonat-speaking peer a probe target and an
  unanswered probe leaves status Unknown (never a false flip).
- **Announce watch channel**: `watch::Sender<AnnounceState{reachability_kind,
  using_relay, announceable, epoch}>` — no multiaddrs (G1). Equality-gated sends; the
  10s settle debounce lives in the run_node consumer, not the service. Closes slice 1's
  known gaps (degraded `using_relay`, no re-announce on address change) and replaces the
  p2pd path's restart cluster and relay-addr disk cache (which existed only for p2pd's
  ~7-minute AutoRelay latency and would be harmful now).
- **`addresses.rs` classifier**: port of node.rs `is_announceable_addr` /
  `is_globally_routable_v4` (which already deliberately accept 198.18/15 and the
  RFC5737 doc ranges for the test beds) into kwaai-p2p, with a golden test pinning
  `198.18.0.20` as routable. `only_global_ips` opt-in tightens.
- **Follow-up closure for free**: with reachability in hand, `dht_service`'s `rpc_ping`
  can answer `available = is Public` instead of hardcoded false (open item above), via a
  new `NetworkHandle::reachability()`.

### New risks found

- **R1 (high)** — identify's unfiltered listen-addr advertisement (G6) is the direct
  cause of "Direct-but-unreachable" map entries. Accepted for this slice (go-libp2p
  leaked the same before `-announceAddrs`); a ~60-line filtering identify wrapper is a
  tracked follow-up. If a NATed node shows "Direct" with a 192.168.x address, this is why.
- **R2 (medium)** — `default_trusted_relays()` points at the production bootstraps with
  their refusal history; with an active RelayManager that yields a visible
  backoff-rotate loop and no relay. Fix in the CLI wiring commit: empty default +
  identify hop discovery as the real supply; `trusted_relays` becomes a pure operator
  override.
- **R3 (medium)** — `observed_addrs` in the service never expires entries; the
  identify-consensus fallback would latch a stale address after a network move. Fixed by
  pruning observers on `ConnectionClosed` (also fixes the latent staleness in the
  existing `ObservedAddrs` handle command).
- **R4 (low)** — autonat 0.12 is unconditionally also a dialback *server*
  (`ProtocolSupport::Full`); bounded by its default throttles and observed-IP
  substitution. Keep the throttles at defaults.
- **R5 (low)** — `with_relay_client` changes the `with_behaviour` closure arity; the
  compile error points at the closure, not the cause.
- **R6 (low)** — autonat/dcutr enter Cargo.lock for the first time; nix/crane vendoring
  must be regenerated in the same commit or offline CI breaks.

### Implementation slicing (9 commits, only the last-but-one touches node.rs)

1. live bootstrap protocol-list snapshot test (answers the autonat unknown, read-only);
2. `addresses.rs` classifier + tests; 3. behaviours added inert (Cargo/behaviour/config/
builder); 4. reachability state machine + observed-addrs pruning + `rpc_ping available`;
5. RelayManager + in-process relay tests (restart recovery, refusal rotation, slot
bounds); 6. announce watch channel; 7. dcutr plumbing tests; 8. CLI wiring into the
native run_node path (`trusted_relays`/`no_relay`/`force_private`/`public_ip` →
`NetworkConfig`, watch-driven announce loop, empty relay default); 9. doc update.
In-process tests cover relay lifecycle, reachability rules, and dcutr plumbing; real
hole punching and the (a)–(f) acceptance items remain kwaaiai-env nat-test topology work.

### Slice 2 landed (2026-07-31)

All nine planned commits are in (`b5d4cdc..3e2000d` + the settle-deadline fix).
Full gate green: workspace build, unit suites, integration tiers 01–13 vs real p2pd,
fmt/clippy clean (one pre-existing wire.rs type-complexity warning).

- **The bootstrap-autonat unknown resolved favourably**: both production bootstraps
  advertise `/libp2p/autonat/1.0.0`, both relay-hop/stop, and `/libp2p/dcutr`
  (read-only identify snapshot; full lists in commit `b5d4cdc`). A native node gets two
  AutoNAT servers from its first dial; identify consensus is a true fallback. Hop
  advertisement is not a promise — the RESERVATION_REFUSED history stands, and refusal
  rotates rather than fails.
- Identify now records per-peer protocol lists (`NetworkHandle::peer_protocols`),
  dropped on last disconnect; relay-hop discovery and AutoNAT probe targeting read it.
- The announce settle window is a deadline armed in the watch arm and awaited in its own
  select branch — an inline sleep would have blocked shutdown/SIGHUP for the window.

**Remaining Phase 4 work**: the kwaaiai-env nat-test topology acceptance run —
items (a)–(f) — which requires coordinating with the in-flight work in that repo,
plus the tracked follow-ups above (identify RFC1918 address filter, record validators,
eviction index, callUnary.peer threading, Windows TCP pipe tests).

### Slice 2 adversarial review (2026-07-31)

Findings fixed with regression tests (`ffb9b30`, `83ad739`, `1ba9f21`): the settle arm
gated re-announce on `using_relay` alone (reachability-only transitions never
re-published; epoch-gated now, and the 300 s tick folds in current state); the watch arm
silently died with the service task (now shuts the node down); AutoNAT/UPnP promotions
bypassed the announceability classifier (a LAN dialback could advertise an RFC1918
address fleet-wide and tear down circuits); relay-candidate eviction mid-dial stranded
the slot and froze rotation/backoff.

Accepted as minor follow-ups: in-flight relay dials are forgotten (not aborted) on a
Public flip — the service's connected-check compensates; `pick_candidate` reclones the
candidate list per call; two relay tests need a positive control / an explicit timeout
to fail rather than hang (`a_loopback_relay_is_never_discovered_by_identify`,
`a_node_with_no_relay_candidates_does_not_spin`).

### Live-fleet incident and fix: the dictionary-subkey type freeze (2026-07-31)

First real-network validation of the native node surfaced a wire bug invisible to every
self-consistent test: our `FOUND_DICTIONARY` blobs wrapped subkeys in msgpack **bin**,
while hivemind re-encodes its (deserialized) subkeys as msgpack **str**. A Python
hivemind client merging the same record from a bootstrap (str subkey) and from us
(bytes subkey) hits `TypeError: '<' not supported between 'bytes' and 'str'` in its
candidate heap, the traversal worker dies, and — because hivemind never resolves the
outer future and the health-map updater wraps no timeout around `dht.get` — the
map.kwaai.ai crawler froze. Reproduced twice against production (map freezes within one
crawl cycle of a healthy native node joining) and then locally with a stock hivemind
1.1.10.post2 client (full traceback). Fixed in `f872e2a`: subkeys now serialize as the
msgpack object their raw bytes encode (str stays str; non-msgpack bytes still pass as
bin); verified live — the same crawl completes in 0.7s with the native node in the
traversal and its subkey present.

Lessons recorded: (1) symmetric serialize/parse tests cannot catch asymmetric-encoding
bugs — a Python-hivemind golden interop check belongs in the follow-ups; (2) the
hivemind/health-service side has a real robustness gap (a poisoned record type hangs
the crawler forever; the updater has no timeout and the map UI shows no staleness) —
worth reporting upstream; (3) debugging artifact: a node run under CodeLLDB freezes on
panic/pause and becomes a half-dead peer holding sockets in CLOSE_WAIT.
