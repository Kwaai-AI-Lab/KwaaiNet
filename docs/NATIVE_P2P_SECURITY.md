# Native-p2p: security analysis to date

Status of `main` at `8ad4680` (native-p2p stack 7/7 landed), plus the two
follow-on PRs under review (#94 bootstrap node, #96 decentralized placement).

**Scope and method.** This is what surfaced from reviewing the eight-PR stack and
from running the native path against the live fleet for a sustained session — not
a dedicated threat model, not an audit, and no fuzzing or adversarial testing.
Every claim below was re-verified against `main` at the time of writing, with
file:line references so they can be re-checked when the code moves. Section 6
lists what has *not* been looked at, which is the part most worth acting on.

---

## 1. What the migration changes about the trust boundary

On the p2pd path, a Go daemon terminates every remote connection. Malformed
frames, hostile peers and protocol abuse hit `p2pd` first, in a separate process
with its own memory. The Rust node speaks only to that daemon over a local
control socket.

On the native path that intermediary is gone. The Rust process now terminates
remote connections directly, and everything an untrusted peer sends is parsed
in-process: multistream-select negotiation, the hivemind unary wire envelope,
DHT store/find payloads, identify records, relay circuit requests.

That is the headline change. It is not an argument against the migration — the
Go daemon was never a security boundary anyone designed, and removing a whole
language runtime removes a class of problems too. But it means input handling in
`kwaai-p2p` and `kwaai-hivemind-dht` is now directly exposed where it previously
was not, and should be read that way.

---

## 2. Open issues

### 2.1 No record validators — any peer can write any key, on every native node

**Status: open. Highest-priority item in this document.**

Hivemind gates DHT writes behind a `RecordValidator`. This port does not
implement one, and says so honestly (`kwaai-hivemind-dht/src/server.rs:8`):

> Record validators/signatures are **not** implemented

The migration doc frames this as a bootstrap prerequisite — *"a Rust bootstrap
must not ship without validators"*. **That framing is too narrow.** On `main`
today, `node_native.rs:180` calls `spawn_dht_service` **unconditionally**: every
native node registers `rpc_store` / `rpc_find` / `rpc_ping` and accepts stores
from any peer that can reach it. The exposure is not waiting on a bootstrap; it
shipped with the stack.

Note the wording: not *unauthenticated* writes, which would imply a check being
skipped. There is no authentication layer at all. `RequestAuthInfo` on the wire
is a hivemind-compatible shape that nothing reads, so the accurate statement is
simply that **any peer able to open a connection can write any key**.

There is also **no operator control over this, and none coming** — the
`dht_server` config key does not exist on `main` (0 occurrences in
`kwaai-cli/src/config.rs`; the `dht_server` in `kwaai-p2p`'s `NetworkConfig` is a
different setting, selecting libp2p-kad server mode). #94 briefly proposed an
operator-facing key and then dropped it, correctly — see 2.2. Validators are now
the only answer.

What stands in for authentication today is capacity bounding:

| bound | value | file |
|---|---|---|
| `MAX_STORE_KEYS_PER_REQUEST` | 1024 | `hivemind-dht/src/server.rs:65` |
| `DEFAULT_STORAGE_SIZE` | 1,048,576 entries | `server.rs:59` |
| `DEFAULT_CACHE_SIZE` | 32,768 | `server.rs:47` |

Those prevent unbounded memory growth. They do not prevent a peer from writing
*content* — overwriting or crowding out records for keys it has no relationship
to. Impact scales with how much the network trusts DHT contents:
today those records are largely block-coverage and node-discovery metadata, so
the practical damage is misrouting and pollution rather than compromise. That
changes if anything security-relevant ever lands in a DHT record.

**Recommendation.** Treat validators as a prerequisite for (a) running any Rust
bootstrap, (b) enabling `decentralized_dht`, and (c) placing any
trust-establishing data in DHT records. There is no interim mitigation to fall
back on: the one that was proposed would not have worked, and was withdrawn
rather than fixed (2.2).

### 2.2 The proposed `dht_server: false` did not stop the node serving DHT writes

**Status: resolved by withdrawal. Raised on #94 (CHANGES_REQUESTED); the key was
dropped from the branch entirely rather than fixed. Never on `main`.**

As proposed, the config key documented itself as:

> rpc_ping/rpc_store/rpc_find is governed by `dht_server`.

It is not. `config.dht_server` flows only into `NetworkConfig.dht_server`, which
selects libp2p-kad **server mode** (`behaviour.rs:131`). The hivemind RPC
handlers are registered unconditionally, with no guard in the enclosing scope.

| | governed by `dht_server`? |
|---|---|
| libp2p-kad server mode | yes |
| hivemind `rpc_store`/`rpc_find`/`rpc_ping` | **no — always on** |

This mattered because, with validators absent, the key would have been the only
lever an operator had — and one who set it believing the documentation would get
no protection and no warning. A silently ineffective security control is worse
than an absent one.

**Why withdrawal was the right resolution, and not merely the cheap one.** The
gate was first added as suggested, then removed on a stronger argument: turning
off the handlers is invisible to *placement*. `gather_candidates`
(`announce.rs:620`) draws from the routing table with no filter on who serves, so
a non-serving node is still chosen among the *k* nearest, its store fails, and
the loss surfaces only as an aggregate `total_shortfall` (`announce.rs:796`) that
names neither the peer nor the cause. Python hivemind avoids this — a
`client_mode` node sends an empty `node_info`, so peers never route to it — and
this port has no equivalent. A gate fixing the local half while leaving the
network still routing to the node would have been the same class of defect the
finding was about.

One caveat on the rationale as recorded in `BOOTSTRAP.md`: it also argues kad's
auto-mode makes the key unnecessary, reaching client-or-server from observed
reachability. That is true of `kwaai-p2p` as a library but not of anything
shipped — `node_native.rs` hardcodes `dht_server: true`, so `behaviour.rs:131`
always pins server mode and the auto-mode branch is unreachable for a kwaainet
node. The accurate statement is that serving is unconditional with no
evidence-driven fallback, which argues for withdrawal more strongly, not less.

### 2.3 O(n) eviction under the write lock — availability cliff at capacity

**Status: open, tracked since #83.**

`LocalStorage::enforce_capacity` selects its eviction victim with
`.iter().min_by(…)` while holding the write lock, against a 1,048,576-entry
bound. Below capacity it never runs. At capacity, every insert scans ~1M entries
under the lock, serialising the node's whole DHT service.

The shape matters: not gradual degradation, but no cost at all and then a cliff,
reached exactly when a node is busiest. An ordinary node will not fill a million
entries. A bootstrap will, and a fleet using decentralized placement (#96) makes
it likelier that *some* node does.

Combined with 2.1, any writer can push a node toward that cliff deliberately — bounded by `MAX_STORE_KEYS_PER_REQUEST` per request, but not
overall. Python hivemind uses an expiration heap; an expiration-ordered index
here would remove both the cliff and that lever.

### 2.4 A DNS dependency on the dial path carries an advisory with no fix

**Status: open. `cargo audit` 0.22.2 against 1,216 advisories, run after the
original draft — this section replaces the gap §6 used to record.**

The native path is **206 crates**, of which `libp2p 0.56` contributes 19
sub-crates. Four carry advisories:

| crate | advisory | assessment |
|---|---|---|
| `hickory-proto 0.25.2` | RUSTSEC-2026-0119 (O(n²) name compression, CPU exhaustion) and **RUSTSEC-2026-0118 (NSEC3 proof validation enters an unbounded loop; no fixed version exists)** | the one that matters |
| `crossbeam-epoch 0.9.18` | RUSTSEC-2026-0204, invalid pointer dereference in the `Debug` impl for `Atomic`/`Shared` | low: requires an already-invalid pointer to be formatted. Fix is a patch bump to ≥0.9.20 |
| `bincode 1.3.3` | RUSTSEC-2025-0141, unmaintained | warning |
| `anyhow 1.0.102` | RUSTSEC-2026-0190, unsound `Error::downcast_mut()` | warning |

`hickory` arrives as `libp2p[dns] → libp2p-dns 0.44 → hickory-resolver 0.25 →
hickory-proto`, which puts it on the **untrusted-input path**: DNS responses are
attacker-influenceable, and name resolution runs on every dial to a `/dns/`
multiaddr — which the bootstrap addresses are (`/dns/bootstrap-1.kwaai.ai/…`).
The feature is therefore load-bearing and cannot simply be dropped.

Two reasons the exposure is probably narrower than the advisory titles suggest,
both **worth confirming rather than assuming**: 0119 is in message *encoding*
while a resolver predominantly decodes, and 0118 is DNSSEC NSEC3 validation,
which a stub resolver does not normally perform. Neither has a clean remedy here
— `libp2p-dns 0.44` pins `hickory-resolver 0.25`, so the upgrade must come from
upstream libp2p rather than from this repo.

Two structural notes fall out of the same scan:

- **The `dns` feature is declared in only one of the two places `libp2p` is
  configured** — present in `[workspace.dependencies]` (`core/Cargo.toml:174`),
  absent from `[dependencies]` (`:52`). Cargo unions features so nothing is
  broken today, but the one dependency with an unfixable advisory is reached
  through a feature list that two hand-maintained declarations already disagree
  about.
- **Twelve crates build at two or more versions**, including `yamux` at both
  0.12.1 and 0.13.10 — two implementations of the muxer that frames every
  connection, linked into one binary — plus `prost` 0.12/0.14 and
  `unsigned-varint` 0.7/0.8, both on the wire-framing path.

The workspace's high-severity findings are **not** on the p2p path: `lopdf`
(7.5, stack overflow on nested PDF objects, via `pdf-extract` — the dependency
#87 upgrades) and `quick-xml` (two at 7.5), both in `kwaai-rag`'s document
ingestion. Recorded here only so the boundary is explicit.

Note what an advisory scan is: a check against *known, reported* problems in
dependencies. It says nothing about this repo's own code, and nothing about
unreported problems in theirs.

---

## 3. Issues found and resolved during the stack

Recorded because the *class* recurs, not just the instances. All three were
availability bugs found by running the stack against the live fleet rather than
by reading it, and all three were invisible to the node experiencing them.

### 3.1 Circuit addresses stripped of their relay hop — resolved

`known_addresses` ran every address through `strip_p2p`, which removes *every*
`/p2p` component. A circuit address carries two with different jobs, and deleting
the relay hop produced an address rust-libp2p refuses (`Missing relay peer id`).
Every relay-only peer became undialable from the native path while remaining
reachable from p2pd. Fixed by `strip_dest_p2p` / `dest_peer_id`
(`kwaai-p2p/src/addresses.rs`).

### 3.2 An address filed under the wrong peer was never evicted — resolved

A live address recorded in kad under a different peer's id made every dial to
that peer land on whoever actually owned the address, fail `WrongPeerId`, and
leave the entry in place to be retried forever. Observed in production as
`/ip4/127.0.0.1/tcp/8080` filed under a remote peer, so calls to that peer hit
the local node. Fixed by evidence-based eviction on `WrongPeerId`
(`kwaai-p2p/src/service.rs`).

Worth noting the fix is deliberately *reactive*: the first attempt filtered
addresses that failed `is_announceable`, which would also have discarded
legitimate loopback and LAN peers. Only a failed dial distinguishes a poisoned
address from a valid local one.

### 3.3 Relay-only peers accepted as relay candidates — resolved

`note_identify` used `is_announceable` to choose relay candidates. That returns
true for *any* circuit address, so a peer itself reachable only via a relay was
accepted, and `circuit_listen_addr` appended onto its circuit address to produce
a doubly-nested address `listen_on` rejects. A node whose bootstrap reservation
lapsed rotated onto such candidates and never regained an inbound path. Fixed by
`is_relay_candidate_addr` and by making `circuit_listen_addr` return `Option`.

**The pattern across all three:** a predicate written to answer one question
("is this address worth advertising?") reused to answer a different one ("can I
dial this?" / "can I reserve on this?"). Worth watching for in review.

### 3.4 The observability gap that made all three expensive

In every case the affected node reported completely healthy: status green, DHT
announces succeeding, relay reservations logged. Nothing surfaced "I have no
inbound path" or "I cannot reach any peer". One instance ran for hours before
anyone noticed, and it was only diagnosable because an address string in an
otherwise-empty error message gave it away — `service.rs` logged the listen
failure as `error = %e`, which printed nothing at all.

**Recommendation.** A reachability signal that goes red when reachability is
`Private` and zero circuits are held would have caught all three. Separately,
`%e` on error types across the crate should be swept to `?e` — `{e}` on an
`anyhow::Error` prints only the outermost frame, and this bit twice in one week
(also fixed in #78).

---

## 4. Input handling: what is bounded, and what stands behind it

The bounds below are what currently substitutes for peer authentication. They are
mostly good, and were mostly present before review.

| surface | bound | file |
|---|---|---|
| wire frame length | 10 MiB, checked before allocation | `hivemind-dht/src/wire.rs:51` |
| varint prefix | `MAX_VARINT_LEN`, read byte-at-a-time so it cannot over-consume | `wire.rs` |
| DHT keys per store request | 1024 | `server.rs:65` |
| DHT storage entries | 1,048,576 (see 2.3) | `server.rs:59` |
| addresses learned per peer | 6, capped against identify flooding | `kwaai-p2p/src/service.rs:86` |
| inbound unary streams | `max_concurrent_streams`, then bounded polite refusal (`REFUSAL_SLOTS = 16`), then drop | `kwaai-p2p/src/unary.rs:322,348` |
| outbound unary calls | 30s `request_timeout` | `unary.rs` |
| capacity lease TTL | 30s, renewed per request | `kwaai-cli/src/capacity_lease.rs:60` |

Two are worth calling out as well designed:

**Caller identity is taken from the connection, never from the frame**
(`unary.rs:169`). On a raw stream the `callUnary.peer` field arrives exactly as
the caller sent it, so trusting it would let any peer claim any identity. This
was identified in Phase 0 and handled correctly.

**The unary refusal tier** converts "at capacity" from a caller-side hang into an
immediate error, so a loaded responder does not stall every peer for their full
timeout. That is a denial-of-service mitigation as much as an ergonomic one.

### The multistream-select patch

The stack carries a two-hunk patch (`core/patches/multistream-select.patch`)
relaxing upstream's requirement that protocol IDs begin with `/`, because
hivemind negotiates bare handler names. Reviewed on #91; the relaxation is
narrow and the replacement rule is the one the framing actually requires:

```rust
if value.is_empty() || value.contains('\n') { return Err(InvalidProtocol) }
```

Non-empty, no embedded newline — a newline terminates a multistream-select
message, so a name containing one could otherwise inject a message boundary. This
is validation *replaced*, not removed, and the constraint that matters for
framing safety is retained. The crate source is fetched by checksum-pinned script
rather than vendored, and the checksum is verified before extraction.

---

## 5. Prerequisites before the follow-on work ships

Neither #94 nor #96 can be enabled safely without 2.1, and both make its blast
radius larger:

- **#94 (bootstrap node)** — a bootstrap is the deployment that reaches the
  storage bound, so 2.3 becomes load-bearing. It ships with no operator control
  over serving, the proposed one having been withdrawn (2.2).
- **#96 (decentralized placement)** — defaults to `false`, correctly. Enabling it
  changes the topology from "two operator-run bootstraps hold nearly everything"
  to "every reachable node stores keys it did not choose, written by peers whose
  writes nothing checks". That is a different risk profile, and the flag's doc
  comment should say so where an operator will see it.

`decentralized_dht: false` and `native_p2p: false` are both correct defaults, and
`node.rs` (the p2pd path the fleet runs) is untouched by #96 — so nothing changes
for existing nodes on upgrade.

---

## 6. Not analysed

Stated plainly, because the absence of findings in these areas is not evidence
of their absence:

- **No fuzzing** of the wire decoder, msgpack/DHT payload parsing, or identify
  record handling. `wire.rs` is bounded and reviewed, but never fuzzed.
- **No adversarial peer testing.** Everything was exercised against a cooperative
  fleet. Nobody has run a deliberately malicious peer against a native node —
  malformed frames, protocol-ID abuse, oversized or deeply-nested payloads,
  identify flooding beyond the address cap.
- **Relay abuse not examined.** The node runs a relay *server* (`relay_server`
  defaults true in `NetworkConfig`); its resource limits and whether it can be
  used to amplify or proxy traffic were not reviewed.
- **The capacity-lease admission gate** was reviewed for correctness (it silently
  lost work; fixed in #89) but not as an abuse control. Whether a peer can
  monopolise leases or deny inference capacity to others is unexamined.
- **Trust/VC integration** (`kwaai-trust`) was not in scope of any of these PRs
  and its interaction with the native path is unreviewed.
- **`only_global_ips` defaults to `true`** (`kwaai-p2p/src/config.rs`), so
  IANA-reserved space is rejected like private space. A node that sets it
  `false` announces such addresses; that is a per-deployment decision and not
  examined further here.
- **Licence and supply-chain policy** is unexamined: `cargo deny` was not run, so
  nothing here speaks to licence compatibility, banned crates, or source
  provenance beyond the one vendored dependency noted below. The advisory scan
  that *was* run is at 2.4, and covers only known-and-reported vulnerabilities.
- **The vendored `multistream-select`** (`patches/multistream-select`, crates.io
  0.13.0 with its protocol-name validation relaxed for hivemind's slash-less
  names) is the one dependency not resolved from the registry. The fetch script
  pins a checksum, but the patched code itself was not re-reviewed here — and it
  sits directly on the negotiation path for every inbound stream.

The two cheapest high-value additions would be a fuzz target over `wire.rs`'s
decoder and a deliberately hostile peer in the interop tier — both are small next
to the ~2,800 lines of interop tests the stack already carries.
