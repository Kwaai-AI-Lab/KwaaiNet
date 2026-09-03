# IP Address Discovery and p2pd Restart Safety

This document explains how a KwaaiNet node discovers its public IP address, how
that address propagates to `map.kwaai.ai`, and the safety mechanism that
prevents p2pd restarts from disrupting in-flight inference.

---

## Overview

For a node to appear **Online** on the map and be reachable for PING/FIND
operations, the Hivemind DHT record for that node must contain a publicly
routable multiaddr. There are three ways this address can be established,
in priority order:

| Priority | Source | Config field | Behaviour |
|----------|--------|--------------|-----------|
| 1 | Manual multiaddr | `announce_addr` | Passed directly to p2pd at startup; no discovery needed |
| 2 | Manual IP | `public_ip` | Formatted as `/ip4/<ip>/tcp/<port>` and passed to p2pd |
| 3 | **IDENTIFY (default)** | _(neither set)_ | p2pd starts with no announce addr; address discovered dynamically |

When neither `announce_addr` nor `public_ip` is set the node relies entirely
on the IDENTIFY-based discovery path described below.

---

## Phase 1 — p2pd starts without an announce address

p2pd is spawned with only a host listen address (`/ip4/0.0.0.0/tcp/<port>`).
At this point the daemon has no knowledge of its public address and will not
advertise one in DHT records.

## Phase 2 — DHT bootstrap

The node dials each configured bootstrap peer and waits until at least one
connection is confirmed. Bootstrap peers are the entry point into the Hivemind
DHT and are also the peers that will run the IDENTIFY protocol with us.

## Phase 3 — IDENTIFY polling (external address discovery)

Once a bootstrap peer is connected, the libp2p
[IDENTIFY protocol](https://github.com/libp2p/specs/blob/master/identify/README.md)
fires automatically. Each peer that connects reports back what address it
_observed_ us connecting from — i.e., our public IP and port as seen from the
internet.

`collect_observed_addresses()` polls `identify_with_addrs()` in a loop,
filtering out private/loopback addresses. It requires **two independent
confirmations** from distinct peers before accepting an address as authoritative.
This guards against a single rogue or misconfigured peer reporting a wrong
address.

```
Bootstrap peer A  ──identify──▶  "I see you at /ip4/203.0.113.1/tcp/8080"
Bootstrap peer B  ──identify──▶  "I see you at /ip4/203.0.113.1/tcp/8080"
                                                          ↓
                                              2 confirmations → accepted
```

Polling runs for up to 10 seconds. If no confirmed address is found within that
window a warning is logged and the node falls back to relay mode.

## Phase 4 — p2pd restart with announce address

Once the address is confirmed, p2pd is restarted with the discovered multiaddr
passed as its `--announceAddr` flag. This causes p2pd to include the address in
all DHT advertisements and IDENTIFY responses going forward.

Before shutdown, DHT stream handlers (`DHTProtocol.rpc_ping/store/find`) are
explicitly unregistered so the listener port is freed cleanly. After the new
p2pd starts the handlers are re-registered with the new client.

**The startup restart (Step 5)** happens before the event loop begins, so no
inference traffic can be in-flight at this point.

## Phase 5 — DHT announcement

With p2pd now advertising the correct address, the node announces itself to the
DHT. `map.kwaai.ai` reads these records and marks the node **Online**. PING and
FIND operations from other peers now route to the correct address.

---

## Periodic address refresh

Network conditions change — DHCP leases expire, NAT mappings shift, the node
moves between networks. Every 5 minutes (when running without an explicit
`announce_addr`) the node re-runs IDENTIFY polling with the same 2-confirmation
requirement.

If the observed address has changed a **deferred restart** is scheduled (see
below). If the address is unchanged nothing happens.

---

## Deferred restart safety mechanism

### The problem

Restarting p2pd tears down:

- All active peer connections and relay circuits
- DHT handler registrations (these must be re-registered after restart)
- Any libp2p streams currently being forwarded

If a p2pd restart fires during an in-flight inference request the stream
carrying that request is torn down. The coordinator's retry logic handles
this, but the disruption is unnecessary and wastes KV-cache state on the
downstream shard.

### The solution

When IDENTIFY detects an address change inside the event loop, the new
addresses are stored in `pending_restart` rather than triggering an immediate
restart:

```
IDENTIFY tick  →  address changed  →  pending_restart = Some(new_addrs)
                                       (log: "restart deferred until idle")
```

At the next **reannounce tick** (every 120 seconds), before the regular DHT
announcement, the event loop checks two conditions:

1. `pending_restart` is `Some(_)` — there is a restart waiting
2. `active_rpc_streams == 0` — no DHT RPC handler tasks are currently running

If both are true the restart proceeds. If streams are still active the tick logs
the count and tries again at the next tick.

```
Reannounce tick
  ├─ pending_restart? yes
  ├─ active_rpc_streams? 0  ──▶  restart p2pd now
  │                               re-register handlers
  │                               pending_restart = None
  │                               continue with normal re-announce
  └─ active_rpc_streams? N  ──▶  log "N stream(s) active, retrying next tick"
```

`active_rpc_streams` is an `Arc<AtomicUsize>` incremented when `accept()`
returns a new stream and decremented when the spawned handler task completes.

### What this guarantees

- A restart will never fire while a DHT RPC (ping/store/find) is in progress.
- Restarts are applied within at most `120 + 300 = 420 seconds` of an address
  change in the worst case (IDENTIFY fires at the end of its 5-minute window,
  all streams happen to be active for the next full 120-second reannounce
  interval).
- Handler re-registration always happens as part of the restart, eliminating
  the previous bug where handlers were lost after a periodic restart.

---

## Remaining risks

| Risk | Likelihood | Mitigation |
|------|-----------|------------|
| `shard serve` inference stream torn down by startup restart (Step 5) | Low — startup precedes inference | `shard serve` is a separate process; its handler survives p2pd restart. The coordinator retries on stream error. |
| Relay circuit torn down mid-inference (relay-mode nodes only) | Low — only affects nodes with no confirmed public address | Direct-reachable nodes are unaffected. Relay nodes already tolerate circuit loss. |
| Address change goes undetected if IDENTIFY peers are slow | Medium | 10-second timeout at startup; 8-second timeout on periodic checks. Bootstrap peers are expected to be stable. |
| Node stays on stale address longer than expected if always busy | Low | In practice DHT RPC streams complete in milliseconds. Sustained load would need to span multiple 120-second ticks. |
| `announce_addr` or `public_ip` set but wrong (manual misconfiguration) | User error | IDENTIFY path is bypassed entirely when either field is set. The node relies on the user-provided value. |

---

## IPv6

Everything above describes discovery of one address. A dual-stack node has two,
and the `ipv6` config key decides whether the second one exists at all.

| value | listeners | no IPv6 loopback, or a refused v6 bind | v6 addresses from peers |
|---|---|---|---|
| `auto` (default) | `/ip4/0.0.0.0` + `/ip6/::` | one warning, run IPv4-only | accepted |
| `true` | same | **startup error** | accepted |
| `false` | `/ip4/0.0.0.0` only | n/a | dropped from the dial and announce sets |

Availability is decided by binding a concrete address — `[::1]:0` — **before**
any listener is opened, not by whether the listener bound. Binding the
unspecified `[::]` succeeds on Linux even with
`net.ipv6.conf.all.disable_ipv6=1`, so a node with IPv6 switched off at the
kernel would otherwise report itself `active` and advertise a dual stack it
does not have.

`true` exists because the two failures are not equally visible. A node that
wanted v6 and quietly came up v4-only looks healthy from every angle — it
announces, it serves, it holds reservations — while being unreachable to the
half of the network that only has v6. On a host provisioned with a delegated
prefix, that is a deployment error worth refusing to start over.

Both families listen on the same port number. libp2p-tcp and libp2p-quic set
`IPV6_V6ONLY` on their v6 sockets, so `0.0.0.0` and `::` do not collide; the
node's own local servers (gRPC, the HTTP APIs, the storage health endpoint) set
it explicitly through `kwaai-cli::net`, since the OS default differs by
platform.

`kwaainet p2p info` prints the configured mode. What actually happened is in
the network snapshot's `ipv6` field — `off`, `active` or `unavailable` — which
is the one that distinguishes "v6 is disabled" from "v6 was wanted and the host
refused".

### The classifier, and why it accepts ULAs

`is_routable_v6` mirrors `is_routable_v4`, including its deliberate carve-out.
Rejected: unspecified, loopback, multicast, link-local `fe80::/10`, the
deprecated site-local `fec0::/10` and IPv4-compatible `::a.b.c.d` forms, and the
discard-only `100::/64`. An IPv4-mapped `::ffff:a.b.c.d` is classified as the v4
address it carries, so the v4 filters cannot be walked past by respelling an
address.

**Accepted:** unique-local `fc00::/7`, documentation `2001:db8::/32` and
`3fff::/20`, and benchmarking `2001:2::/48`. This is the same decision as
accepting RFC2544 `198.18/15` on the v4 side, and for the same reason: a docker
bridge hands out ULAs unless it is given a delegated global prefix, so the v6
test bed lives on `fdc6:1200::/64` and would classify itself unreachable
otherwise.

`is_globally_routable_v6` is the strict form, reached by `require_global_ips`.
It rejects all of the above plus the ranges that name a v4 endpoint or an
overlay rather than a host on the v6 internet: Teredo `2001::/32`, 6to4
`2002::/16`, and ORCHID `2001:10::/28` and `2001:20::/28`. Turn it on for a node
on the real internet, where a documentation-range address in the announce set
could only ever be a misconfiguration.

### Rolling v6 out to the fleet

The compiled-in bootstrap literals in `kwaai-p2p::config` are IPv4 and stay
that way — changing them is a release, and a release is the slowest lever
available. The fast one is `/dnsaddr/`: `KWAAI_BOOTSTRAP_SERVERS_DNS` resolves
TXT records at `_dnsaddr.bootstrap.kwaai.ai`, so adding a bootstrap's v6
transport is a DNS edit that every already-deployed node picks up on its next
resolve. Add the `/ip6/` record first, confirm nodes are reaching it, and only
then consider whether the v4 literal is still carrying anyone.
