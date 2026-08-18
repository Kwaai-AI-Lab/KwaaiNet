# Proposal: eliminate the extra round trip on accepted unary calls

**For:** Darren (@dazwin) · **Status:** proposal, not yet implemented
**Blocks:** the 0.6 "drop p2pd, go native" cutover
**Related:** #100 (analysis), #99 (`p2p probe`, the tool that found it)
**Measured:** 2026-08-16 against the live fleet, from a Mac over the KwaaiNet relay

## The problem

The native path pays **two network round trips for an accepted unary call** where p2pd pays one. Measured from a Mac against metro-win, same peer, same relay, only `native_p2p` differing:

| call | p2pd | native |
|---|---|---|
| refused protocol | 158.9 ms | 157.7 ms |
| **accepted protocol** (`/kwaai/p2p/hello/1.0.0`) | **159.4 ms** | **320.1 ms** |
| 256KB payload | free | free |
| local socket write | 0.0–0.1 ms | 0.0–0.1 ms |
| **inference, 14-token decode** | **478 ms/tok** | **750 ms/tok** |

Compute is ruled out (identical remote peer), bandwidth is ruled out, local IPC is ruled out. Full write-up with method in #100.

**Why it blocks 0.6.** The release is scoped as "drop p2pd, go native", which means `native_p2p` defaults to `true`. Every generated token is one accepted unary call, so this lands on every token for every user: a release that removes a Go dependency and halves remote inference throughput in exchange.

A refusal costs the same on both stacks because it is answered on the *first* flight — there is never a second exchange to pay for. Only an accepted call reaches the step where native waits. That asymmetry is the diagnostic, and it is also why `p2p probe` in its default mode reports a healthy 158 ms on native and would have signed this off.

## Part 1 — the fix, measured

rust-libp2p defaults substream negotiation to `Version::V1`, which writes the multistream-select proposal, waits for confirmation, and only then sends the payload. `V1Lazy` buffers the proposal and flushes it with the first application data — 0-RTT negotiation, which is what go-libp2p does. One line, in the existing `.with_swarm_config(...)` chain in `kwaai-p2p/src/service.rs`:

```rust
.with_substream_upgrade_protocol_override(libp2p::core::upgrade::Version::V1Lazy)
```

| | p2pd | native V1 | native V1Lazy |
|---|---|---|---|
| accepted call | 159.4 ms | 320.1 ms | **158.0 ms** |
| inference | 478 ms/tok | 750 ms/tok | **483 ms/tok** |

The gap closes completely. **Interop is safe by construction**: the wire bytes are identical and a *listener* behaves as `V1` regardless, so only our dialer behaviour changes — mixed 0.5.5/0.6 fleets are unaffected. It applies when the dialer offers a single protocol, which every unary call here does.

## Part 2 — the complication, and where it is fixed

V1Lazy breaks exactly three tests, all asserting the protocol-refusal message:

```
service_unary::calling_an_unregistered_protocol_is_refused
service_unary::removing_a_handler_causes_a_clean_refusal
dht_service::removing_the_service_makes_calls_fail_cleanly
```

Their names show this is a deliberate property: the codebase wants "peer doesn't speak this" distinguishable from "something broke."

**The signal is not lost, only re-shaped.** Under V1 the swarm raises `StreamUpgradeError::NegotiationFailed`. Under V1Lazy the failure is deferred to first I/O and arrives as `StreamUpgradeError::Io(e)`, where `e` is `io::Error::new(ErrorKind::Other, NegotiationError::Failed)` — the enum is preserved as the io error's **inner error**, so it is recoverable by downcast.

Fix site is `kwaai-p2p/src/unary.rs:513-519`, where the `Io(e)` arm currently flattens to `UnaryError::Wire`:

```rust
StreamUpgradeError::Io(e) if is_negotiation_failure(&e) =>
    UnaryError::UnsupportedProtocol(message.proto.to_string()),
```

`libp2p_core::upgrade::NegotiationError` is re-exported (`libp2p-core/src/upgrade.rs:72`) and `kwaai-p2p` already imports from that module, so **no new dependency**. Worth walking the `source()` chain as well as attempting a direct downcast, in case the io error arrives nested.

`raw_stream.rs` looks like it has the same shape (`RawStreamError::Io` → `handle.rs:587`) and probably needs the identical treatment — I did not verify that one.

## Two dead ends, so you don't spend the time

- **Disambiguating via identify does not work.** `NetworkHandle::peer_protocols()` looks like the natural capability feed, but libp2p-identify pushes only on *listen-address* change (`push_listen_addr_updates`), not on protocol-set change, and we set a 5-minute identify interval. A peer that adds or removes a handler at runtime leaves the cached list stale for up to 5 minutes — and two of the three failing tests remove a handler at runtime, so they would still fail.
- **A per-call V1 retry to disambiguate is not expressible.** The version override is swarm-global (it lives on the connection pool config), and `SubstreamProtocol` in libp2p-swarm 0.47 has no per-call version field. It would need an upstream patch, and Part 2 makes it unnecessary.

## Judgement calls that are yours

1. **EOF folds into `Failed`.** `dialer_select.rs:172-175` deliberately treats a dropped stream as a graceful negotiation failure, so an abrupt peer disconnect would now report as "unsupported protocol". Upstream considers that correct; you may not. If it matters, the two cases are distinguishable earlier but not at this seam.
2. **Whether `UnsupportedProtocol` should stay authoritative for capability negotiation.** The ledger's `receipts_v1` degradation and capacity-lease's "a peer without the feature is served exactly as today" both lean on this distinction. Part 2 preserves it, but it is worth deciding whether feature detection should consult advertised protocols up front rather than discovering support by failing a call.
3. **Whether this ships with the cutover or ahead of it.** It is independent of the `native_p2p` default flip and could land first.

## Verifying

```bash
kwaainet p2p probe --peer <id>                                   # refused: 1 RTT
kwaainet p2p probe --peer <id> --proto /kwaai/p2p/hello/1.0.0    # accepted: should match
```

Equal medians = fixed. Requires #99. Then `cargo test -p kwaai-p2p` and a real `shard run` against a remote peer.

## Provenance

Found while verifying the native path ahead of the cutover, using `p2p probe` (#99). Analysis in #100. Everything above is measured on real hardware except where flagged: the `raw_stream.rs` parallel is unverified, and the EOF behaviour is read from upstream source rather than observed.

A local, unpushed commit with Part 1 exists on `fix/native-v1lazy-negotiation` — happy to push it as a starting point, or leave it to you.
