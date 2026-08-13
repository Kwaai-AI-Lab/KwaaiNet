# Bug: a relay-only peer is accepted as a relay candidate, so a node that loses its reservation never gets another

**Where:** `core/crates/kwaai-p2p/src/{relay_manager.rs,addresses.rs}` — native path only
**Branch:** `native-p2p-pr7-nat-traversal` (PR #86), the NAT slice that introduced both files
**Severity:** high — a NATed node silently becomes unreachable inbound and stays that way
**Found:** 2026-08-11, on metro-win, during a three-machine RAG build
**Diagnosis:** by the metro-win session, from that node's own logs; independently corroborated here
**Status:** fix written and tested locally, not landed — patch at the bottom, yours to take or rewrite

---

## Why this is a native-p2p bug specifically

`relay_manager.rs` exists only on the native path. The migration doc lists AutoRelay as
the single daemon feature with no rust-libp2p equivalent — "❌ **hand-roll** a
RelayManager" — and this is a defect in that hand-rolled replacement. go-libp2p's
AutoRelay does not have it, which is why metro-win was reachable on p2pd and lost its
circuit after switching to native.

## Symptom

metro-win got one working relay circuit at startup (23:09:12, on bootstrap-1
`QmQhRuhee…`) and used it for ~19 minutes. At 23:28 that reservation ended. Every
attempt since to obtain another failed — dozens of retries against ~14 candidates, all
`listen_on refused the circuit address`. It has been in that state continuously.

The node reports healthy throughout: `kwaainet status` is green and DHT re-announce keeps
succeeding, because announcing works fine. What is gone is any way for a peer to open an
inbound `p2p-circuit` connection to it.

From this Mac, metro-win times out on **both** stacks — that is not evidence the bug is
shared, it is the expected consequence: with no reservation there is nothing to dial,
whoever is asking.

## Root cause

Three things compose into it:

1. **`addresses.rs` — `is_announceable()` returns `true` for any address containing
   `/p2p-circuit`, unconditionally.** That is correct and deliberate for the question it
   was written to answer: *"is my own address worth telling the world about?"* Once we
   hold a reservation, our circuit address is exactly what peers need.

2. **`relay_manager.rs` — `note_identify()` reuses `is_announceable()` to decide whether
   a newly-discovered peer is a usable *relay candidate*.** This is the actual defect: it
   conflates two different questions. *"Is this address fine to advertise as mine"* is not
   *"can I dial this peer and reserve on it"*. A peer whose own advertised address is
   itself a circuit — i.e. a peer that is relay-only, which on this fleet is most of them —
   is accepted as a candidate.

3. **`addresses.rs` — `circuit_listen_addr()` appends `/p2p/<relay>/p2p-circuit` without
   checking whether the base already ends in `/p2p-circuit`.**

The result is a doubly-nested listen address:

```
/ip4/18.219.43.67/tcp/8000/p2p-circuit/p2p/12D3KooWAUGTd8Gs…/p2p-circuit
```

`Swarm::listen_on` correctly refuses it. The candidate goes on backoff, the cursor
rotates to the next candidate — which is usually also relay-only — and the cycle repeats
indefinitely.

The only candidates this path handles correctly are peers with clean, non-circuit
addresses: the two bootstraps. That is why the one reservation that ever worked was on a
bootstrap, and why nothing recovered once it lapsed.

## Corroborating measurement

From an unrelated macOS native node, 26h of logs, `why=` on 2818 reservation failures:

| failure | count | which relays |
|---|---|---|
| `listen_on refused the circuit address` | **2350** | **only fleet peers, never a bootstrap** |
| `the dial to the relay failed` | 355 | fleet peers |
| `the relay connection closed` | 87 | only bootstraps |
| `reservation ended` | 20 | only bootstraps |
| `Failed to get Reservation.` | 6 | only bootstraps |

The 2350 spread evenly (~200 each) across ~12 fleet peers, and the perfect split — the
nesting failure never touches a bootstrap, the relay-side failures never touch a fleet
peer — is the signature of this bug rather than of flaky relays.

Note the interaction: the ~113 bootstrap-side events are ordinary relay churn that a
healthy node absorbs by re-reserving elsewhere. This bug removes that fallback, so
*ordinary* churn becomes a permanent outage. Neither is sufficient alone.

## Recommendation

Three changes, in descending order of importance. The first alone fixes the outage.

### 1. Give the relay-candidate question its own predicate (the actual fix)

Do not reuse `is_announceable` for it. A relay must be **directly dialable**:

```rust
/// Whether a peer reachable at `addr` can serve as *our* relay.
pub fn is_relay_candidate_addr(addr: &Multiaddr) -> bool {
    is_announceable(addr) && !is_circuit(addr)
}
```

and in `note_identify`:

```rust
let Some(addr) = listen_addrs.iter().find(|a| is_relay_candidate_addr(a)) else {
    debug!(%peer, "peer offers relay hop but no directly-dialable address");
    return Vec::new();
};
```

A separate function rather than an inline `&& !is_circuit(..)` because the confusion here
was conceptual, not typographical — the two questions want two names, and the next person
to reach for `is_announceable` should find a reason not to.

### 2. Make the nested address unconstructible

`circuit_listen_addr` should refuse rather than silently produce an address that cannot
work. Returning `Option` forces the caller to decide:

```rust
pub fn circuit_listen_addr(relay_addr: &Multiaddr, relay: PeerId) -> Option<Multiaddr> {
    if is_circuit(relay_addr) { return None; }
    Some(strip_p2p(relay_addr).with(Protocol::P2p(relay)).with(Protocol::P2pCircuit))
}
```

with `on_relay_ready` treating `None` as a candidate failure (`self.fail(...)` +
`fill_slots`) so the slot is released instead of held by something that can never succeed.

### 3. Fix the swallowed error

`service.rs` logs the listen failure as `error = %e`, and it comes out **empty** — the
same swallowed-error pattern as the auto-update bug. `?e` (Debug) prints. Worth grepping
the crate for other `error = %e` on libp2p error types; the address in the message is what
gave this bug away, and next time there may not be one.

## What I would *not* do

Do not make `is_announceable` reject circuit addresses. It is right as it is, and the
comment above it explains why: classifying a circuit by the IP in front of it would reject
every reservation held on a relay reached over a LAN. The bug is the call site, not the
predicate.

## Tests

Three, one per layer, all failing before the fix and passing after:

- `addresses::a_circuit_address_is_never_a_relay_candidate` — asserts a circuit address
  stays announceable *and* is rejected as a candidate, pinning the distinction
- `addresses::circuit_listen_addr_refuses_to_nest_circuits` — the nested address is
  unconstructible; the direct case still builds
- `relay_manager::a_relay_that_is_itself_relay_only_is_not_a_candidate` — the end-to-end
  behaviour, using the real metro-win address shape
- `relay_manager::a_directly_dialable_relay_is_still_a_candidate` — the guard does not
  cost us ordinary relays

Verified the third genuinely catches the bug: reverting only the `note_identify` line
makes it fail, restoring it makes it pass.

Full `kwaai-p2p` + `kwaai-hivemind-dht` suites green (78 lib tests), interop suites green.

## Patch

`tests/kwaai-network/stress/relay-candidate-fix.patch` — 3 files, +139/-11, against
`native-p2p-pr7-nat-traversal` @ `633af3a`. Apply with `git apply`, or ignore it and write
your own; the diagnosis matters more than my diff.

## Still open, not addressed here

A node in this state reports healthy. `kwaainet status` is green, announce succeeds, and
nothing surfaces "I have no inbound path". Worth a reachability signal that goes red when
reachability is Private and zero circuits are held — the outage lasted hours precisely
because nothing said anything.
