# native-p2p cross-OS stress test — findings (2026-08-10)

Fleet under test: macOS (this Mac), metro-linux, metro-win — all three running the
native rust-libp2p stack (`native_p2p: true`) — against the live production fleet of
12 nodes (7 linux, 3 macOS, 2 windows), most still on the Go `p2pd` path.

Branch: `native-p2p-pr7-nat-traversal` @ 8822d4d (tip of the #79–#86 stack).

---

## BLOCKER (FIXED) — the native path destroyed its own circuit addresses

**A native node could not reach a relay-only peer that go-libp2p reached fine.**

Symptom: dialing metro-win from the native path failed inside rust-libp2p with

```
Failed to negotiate transport protocol(s):
  (/ip4/18.219.43.67/tcp/8000/p2p-circuit/p2p/12D3KooWLMizEbVi… : Missing relay peer id.)
```

while the macOS **p2pd** node reached the same peer 3/3 at the same moment.

### Root cause — ours, not go-libp2p's

My first reading was that the remote advertised a malformed address and go-libp2p was
merely more tolerant. That was wrong. **We mangled the address ourselves on the way
out of the routing table.**

`P2PService::known_addresses` ended with `strip_p2p()` on every address. `strip_p2p`
removes *every* `/p2p` component — correct for a direct address, destructive for a
circuit, which carries two with different jobs:

```
/ip4/<relay-ip>/tcp/<port>/p2p/<relay>/p2p-circuit/p2p/<dest>
                          ^^^^^^^^^^^^             ^^^^^^^^^^
                          which relay to           who to reach
                          dial *through*           through it
```

Stripping both left `/ip4/…/tcp/8000/p2p-circuit`; the caller re-attached the
destination, producing the address rust-libp2p refuses. Its own comment named it the
single chokepoint — *"the one place every consumer (connect, routed dial, the GUI's
peer table) reads them back"* — so one line broke every relay-only dial at once, and
`peers find` displayed the mangled form, which is what made it look remote-caused.

A second latent bug sat next to it: `dial()` keyed the routing-table entry off
`peer_id_from_multiaddr`, which returns the **first** `/p2p` — on a circuit that is
the *relay*, so the circuit got filed under the relay's key and the destination was
left with no route.

### The fix

`core/crates/kwaai-p2p/src/addresses.rs`:

- **`strip_dest_p2p`** — drops the destination `/p2p` while preserving the relay hop;
  delegates to `strip_p2p` for non-circuit addresses, so `circuit_listen_addr` and the
  relay-manager call sites keep their existing behaviour.
- **`dest_peer_id`** — the peer an address *reaches*, as opposed to the one it routes
  through. `None` for a circuit naming a relay but no destination (our own listen addr).

`core/crates/kwaai-p2p/src/service.rs` uses both in `known_addresses` and `dial`.

### Verified live

| | before | after |
|---|---|---|
| DHT hands back | `/ip4/…/tcp/8000/p2p-circuit/p2p/<dest>` | `/ip4/…/tcp/8000/p2p/<relay>/p2p-circuit/p2p/<dest>` |
| hello to metro-win, discovery only | 0/3 timeout | **5/5 `Response (2 bytes): ok`** |
| metro-win over a 120s sweep | 0% | **100%** |
| metro-linux over a 120s sweep | 60% | **100%** |
| fleet overall | 56.5% | **68.4%** |

No other peer moved: every 100% stayed 100%, and the peers that are 0% on both stacks
are unchanged (peer-side, see below).

### Regression cover

Seven tests in `addresses.rs`, named for the behaviour rather than the function:

- `strip_dest_p2p_keeps_the_relay_and_drops_the_destination`
- `a_stripped_circuit_readdressed_to_its_destination_is_dialable` — the exact
  strip-then-re-attach round trip `known_addresses` and its callers perform
- `the_old_strip_p2p_would_have_broken_the_circuit` — pins *why* the new function
  exists, and that the two genuinely differ on a circuit
- `strip_dest_p2p_matches_strip_p2p_on_direct_addresses`
- `dest_peer_id_reads_through_the_relay_not_the_relay_itself` — also documents the
  `peer_id_from_multiaddr` trap
- `dest_peer_id_is_none_when_a_circuit_names_no_destination`
- `dest_peer_id_on_a_direct_address_is_the_peer`

Full suites after the change: **kwaai-p2p + kwaai-hivemind-dht all green**, and all
**27 interop tests** (07–13) still pass.

### Why it mattered for Phase 5

`force_private` defaults to true, so relay-only is the **normal** state for a NATed
node, not an edge case. Before the fix, native nodes silently saw a smaller network
than p2pd nodes on the same fleet — no error loud enough to notice, just peers that
were never there.

---

## FIXED — an address filed under the wrong peer never got evicted

Found 2026-08-10 by the bake-off harness, *after* the circuit-address fix landed.

Symptom: unary calls to metro-win fail, intermittently and then persistently,
with the native node dialing **its own machine**:

```
Dial error: Unexpected peer ID 12D3KooWAourfFox… at
  /ip4/127.0.0.1/tcp/8080/p2p/12D3KooWLMizEbVi…
```

`/ip4/127.0.0.1/tcp/8080` is the *local* production p2pd node, filed in kad under
**metro-win's** peer id. Once that entry exists every dial to metro-win hits the
local node, gets the wrong peer id back, and fails — the perfectly good circuit
addresses in the DHT are never tried. It clears when the entry is evicted, which
is why reachability flaps: metro-win measured 100%, then 0%, then 100%, then 0%
across four runs an hour apart.

### Why the existing filter does not catch it

`known_addresses` filters non-announceable addresses (loopback included) on the
way out, and its comment calls itself "the one place every consumer reads them
back". That is not quite true. The routed dispatch path calls
`unary.send_request(peer, …)`, which dials **by PeerId** — so the swarm resolves
addresses from the behaviours directly, and kad hands over everything it holds,
including the loopback entry our filter never sees. The filter guards our read
path, not libp2p's own address resolution.

This is the same class the branch already has two commits against ("cap
routing-table addresses learned from identify", "filter kad addresses on the way
out, not just on the way in"). The remaining hole is the by-PeerId dial.

### The fix, and the approach that did not work

First attempt was a proactive sweep: before each routed dispatch, evict anything
kad held for the peer that failed `is_announceable`. It passed the existing tests
and was **wrong**. `is_announceable` answers *"is this worth telling the world
about"*, not *"can I dial it"* — and loopback and LAN addresses are exactly how
two nodes on one machine, or one subnet, reach each other. That sweep would have
broken the two-nodes-on-one-Mac setup while fixing the fleet. The existing tests
did not catch it only because `dht_find_peer` does not route through the path the
sweep was added to.

What distinguishes a poisoned address from a legitimate loopback one is not its
shape — it is that dialing it reaches **somebody else**. So the eviction is
evidence-based: on `DialError::WrongPeerId`, drop that address for that peer.
Nothing is evicted on suspicion; an address is removed only once it has proven it
does not belong to the peer it was filed under.

That also covers the case a shape-based filter never could: an address that is
perfectly routable and simply belongs to a different node.

### Regression cover

`kwaai-p2p/tests/swarm.rs::an_address_that_answers_with_the_wrong_peer_id_is_evicted`
reproduces the production shape — Bob's real address filed in kad under a ghost
peer id, then a unary call to the ghost (the by-PeerId path). It asserts the
entry is gone afterwards.

Verified it actually catches the bug: with the eviction removed the test fails
(times out after 20s waiting for the entry to disappear); with it, it passes.

### Verified live

Rebuilt, restarted, and re-run through the bake-off harness — the same harness
that found it:

| | before | after |
|---|---|---|
| visibility parity vs p2pd | missing 1 peer | **missing 0** |
| metro-win reachability | 0% (flapping 100%/0% across runs) | **100%** |
| scorecard verdict | FAIL | **PASS** |

## No memory leak, no FD leak — 60-minute soak

macOS native node, 8 workers, payloads 64B → 1MiB, **60,945 RPCs**.

| | start | end | verdict |
|---|---|---|---|
| RSS | 19.0 MB | 55.3 MB | peaks 65 MB at ~17 min, then plateaus |
| second-half RSS slope | | **−1.2 MB/hour** | stable — the climb was warm-up, not a leak |
| FDs | 40 | 50 | flat |
| threads | 16 | 16 | flat |
| process | alive | alive | survived all 60,945 calls |

The node logged **one** warning across the entire run, and it was an unrelated gRPC
startup error.

---

## Native vs p2pd A/B — no native-specific regression

Identical 12-peer sweep from both stacks on the same machine, ~880 RPCs each:

| peer | os | native | p2pd | verdict |
|---|---|---|---|---|
| christophe-linux-x86_64 (×2) | linux | 100% | 100% | both fine |
| john-linux-arach | linux | 100% | 100% | both fine |
| john-linux-draca | linux | 100% | 100% | both fine |
| gandalf_macosx64 | macos | 100% | 100% | both fine |
| redmond-win-amd64 | win | 100% | 100% | both fine |
| john-linux-naga | linux | 0% | 0% | unreachable on **both** — peer-side |
| john-linux-spectre | linux | 0% | 0% | unreachable on **both** — peer-side |
| projxai-macos | macos | 0% | 0% | unreachable on **both** — peer-side |
| metro-win | win | 0% | *(later 100%)* | the relay-peer-id blocker above |

Every peer reachable at all is at 100% on both paths, across all three OSes. Every
failure except metro-win is 0% on **both** paths — those peers advertise direct
addresses that no longer answer, which is peer-side and predates this branch.

Latency scales cleanly with payload (p50, native, over the hour):

| payload | p50 | p95 |
|---|---|---|
| 64 B | 245 ms | 461 ms |
| 4 KiB | 248 ms | 476 ms |
| 64 KiB | 307 ms | 565 ms |
| 256 KiB | 507 ms | 871 ms |
| 1 MiB | 1078 ms | 1754 ms |

---

## Fleet-transparency check — passed

After metro-linux and metro-win switched to the native stack, both kept the **same
peer ID**, stayed `online` on map.kwaai.ai with `using_relay=true` and blocks 0–32,
and no `p2pd` process remained. The rest of the fleet cannot tell the stack changed.

---

## RAG graph build: native vs p2pd — no meaningful difference (it is inference-bound)

Interleaved A/B, identical binary, identical KB and chunks, same remote GPU
(metro-linux, `llama3.1:8b`), 20 chunks per run, 3 reps per arm, arm order
alternated each rep. The only variable is which node answers on `KWAAINET_SOCKET`.

| arm | runs (s) | mean | entities extracted |
|---|---|---|---|
| p2pd | 98.1, 89.3, 94.5 | **94.0 s** | 23, 28, 25 (avg 25.3) |
| native | 105.7, 91.2, 101.1 | **99.3 s** | 32, 26, 32 (avg 30.0) |

Native is 5.6% slower on raw wall-clock — but it also extracted ~19% more
entities, i.e. generated more output tokens, which is the thing that actually
costs time. Per entity the ordering reverses (3.75 s vs 3.32 s). With n=3 and a
89–106 s spread on both arms, **the honest read is that the two are equivalent for
RAG graph build**: the work is dominated by remote LLM inference, and the p2p
transport is a rounding error on a multi-second call.

## Transport latency: native pays about one extra round trip per unary call

The RAG result does not mean the stacks are equal at the transport layer. Measured
by interleaving single unary calls through each socket:

| peer | p2pd median | native median | native advantage | paths (native / p2pd) |
|---|---|---|---|---|
| christophe | 173.1 ms | **94.5 ms** | **+78.7 ms** | direct / relay |
| redmond-win | 177.2 ms | **120.9 ms** | **+56.2 ms** | direct / relay |
| metro-linux | **183.6 ms** | 333.4 ms | −149.8 ms | direct / relay |
| gandalf | **207.9 ms** | 397.3 ms | −189.4 ms | direct / direct |

Two things are going on, and they pull in opposite directions.

**1. Native gets far better paths.** 19 connections, all direct, versus p2pd's 7
(4 direct, 3 relay). On the three peers where p2pd relays through AWS, its latency
clusters at 173–184 ms regardless of how near the peer is — the relay hop dominates.

**2. Native pays roughly 2 network round trips per call where p2pd pays 1.**
gandalf is the clean control: both stacks hold a *direct* connection to it, and
native takes **1.91×** p2pd's time. `unary.rs` explains it — "one request and one
response per stream", and "every outbound request here negotiates exactly one
protocol, the one the caller asked for". A hivemind call *is* its protocol, so each
call opens a fresh stream and runs multistream-select again; p2pd's client
multiplexes over a persistent, already-upgraded connection.

Halving each native figure recovers a plausible direct RTT (47 / 61 / 167 / 199 ms)
that tracks how far away each peer is, which is what the 2-round-trip model
predicts.

Net effect: **native wins on nearby peers and loses on distant ones.** The
per-call penalty scales with RTT, so it is invisible on a multi-second LLM call
(the 5 s gap over 20 chunks is ~250 ms/chunk, consistent with the measured
~150 ms transport delta) and very visible on chatty, low-latency RPC.

Control-socket IPC is *not* the cause — a network-free `p2p info` round trip is
9.7 ms native vs 10.8 ms p2pd, i.e. native is marginally faster there.

Worth noting the mechanism is inherent to the hivemind wire shape rather than a
defect: protocol-per-request is what makes the go-libp2p interop work. Amortising
it would mean caching negotiated streams per (peer, protocol), which is a real
design change, not a tweak.

## Build portability notes

- **Go is still required to build this branch.** `kwaai-p2p-daemon/build.rs` clones and
  builds go-libp2p-daemon even though the native path never launches it. Removal is
  Phase 5.
- `core/patches/fetch-multistream-select.sh` is bash-only with no PowerShell
  equivalent, and CI runs it only on `ubuntu-latest`. Windows builds must use Git Bash.
- There is no `origin/native-p2p` branch — the stack tip is
  `origin/native-p2p-pr7-nat-traversal`.

---

## Reproducing

```bash
bash stress_agent.sh --targets targets.tsv --duration 3600 --workers 8 \
     --label <machine>-native --node-pid "$(pgrep -f 'kwaainet run-node' | head -1)"
```

Writes `results.jsonl` (one line per RPC and per resource sample) and a `progress.json`
refreshed every 15s.
