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
