# Reliable LLM Inference Hosting — GPU Host Supervisor Design

## Status: COMPLETE (v0.4.118)

All four host-side improvements implemented and shipped. See implementation notes below.

---

## Context

kwaainet remote inference works through a three-layer stack on the GPU host:
1. **p2pd** (go-libp2p daemon) — handles all P2P transport, relay, and stream routing
2. **kwaainet run-node** — registers `/kwaai/ollama-proxy/1.0.0`, `/kwaai/shard-proxy/1.0.0`, and `/kwaai/inference-mux/1.0.0` handlers with p2pd
3. **Ollama** — serves the LLM locally at `http://localhost:11434`

When a client uses `p2p://PEER_ID`, it opens a libp2p unary RPC to the host's registered handler, which forwards to Ollama and relays the response back. The `mux://PEER_ID` variant multiplexes concurrent requests over a single persistent yamux stream.

**Current reliability gaps on the host side:**
- p2pd crashes are detected at the 300s re-announce tick, not sooner
- Ollama is not supervised at all — if it crashes, requests silently fail
- The health monitor logs intent to reconnect but does NOT actually restart anything
- Relay circuit expiry (libp2p circuit relay v2 reservation timeout) can cause the host to become silently unreachable until the next IDENTIFY probe (5 min)
- No heartbeat signal to clients — they learn of host death only when their next request fails

The user's question ("what managing process is required?") and goal ("circuits should automatically heal, minimize downtime") point to a single coherent answer: **the GPU host needs a supervised process tree where each component is health-checked and auto-restarted, and where the P2P circuit is actively maintained.**

---

## What Needs to Run on the GPU Host (Answer to the Question)

```
┌─────────────────────────────────────────────────┐
│             kwaainet daemon (run-node)           │
│  ┌───────────────┐   ┌─────────────────────────┐│
│  │  p2pd         │   │  Ollama health watcher  ││
│  │  (go-libp2p)  │   │  (new: polls :11434)    ││
│  └───────────────┘   └─────────────────────────┘│
│  ┌────────────────────────────────────────────┐ │
│  │  Registered handlers (per run_node startup)│ │
│  │  • /kwaai/ollama-proxy/1.0.0              │ │
│  │  • /kwaai/shard-proxy/1.0.0               │ │
│  │  • /kwaai/inference-mux/1.0.0             │ │
│  └────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
          ↓ forwards to
┌──────────────────────┐
│  Ollama              │
│  localhost:11434     │
│  (must be running)   │
└──────────────────────┘
```

The managing process is `kwaainet run-node` (started via `kwaainet start --daemon`). It is the single entry point that must supervise all three sub-components.

---

## Reliability Improvements to Implement

### 1. Ollama Health Watcher (new — highest priority)

Add an `ollama_watcher` task inside `run_node()` that:
- Polls `GET http://localhost:11434/api/tags` every 15 seconds
- On 3 consecutive failures: logs ERROR + optionally spawns `ollama serve` if configured
- On recovery: logs INFO + re-announces to DHT (so clients know host is back)
- Configuration: `ollama_manage: true/false` in `~/.kwaainet/config.yaml` — if true, kwaainet owns Ollama lifecycle

**Files:** `core/crates/kwaai-cli/src/node.rs` — add `spawn_ollama_watcher()` task alongside existing `spawn_shard_serve()` (~line 600).

### 2. Shrink p2pd Crash Detection Window

Current: p2pd crash detected at 300s re-announce tick.
Fix: add a dedicated `p2pd_heartbeat` task that calls `daemon.is_running()` every **10 seconds** and triggers immediate restart (same `restart_p2pd()` path already in `node.rs:944-974`).

**Files:** `core/crates/kwaai-cli/src/node.rs` — add `spawn_p2pd_heartbeat()` loop.

### 3. Relay Circuit Keepalive

libp2p circuit relay v2 reservations expire after a fixed TTL (~2 min idle). When the relay circuit drops, the host becomes unreachable until the next IDENTIFY cycle.

Fix: add a keepalive task that sends a trivial DHT ping to any bootstrap peer every **60 seconds**. This keeps the relay stream alive without a new reservation negotiation.

**Files:** `core/crates/kwaai-cli/src/node.rs` — add `spawn_relay_keepalive()` using the existing `client.find_peers()` call (which already keeps the DHT connection warm).

### 4. Re-announce on Recovery

After any p2pd restart or Ollama recovery, immediately call `announce()` rather than waiting for the next 300s tick. This minimizes the window where clients try to reach a host that has re-appeared but hasn't re-announced yet.

This already exists (`retry_announce` path in `run_node`) but is not triggered from the Ollama watcher. Wire it in.

### 5. Host Health Endpoint (CLI command)

Add `kwaainet node health` command that prints the status of each component:
- p2pd: running/stopped, uptime, peer count
- Ollama: reachable/unreachable, model count
- Circuit: direct/relay, relay peer, relay expiry ETA
- Last announce: timestamp

**Files:** `core/crates/kwaai-cli/src/cli.rs` (new subcommand), `node.rs` (query via gRPC IPC).

---

## Files to Modify

| File | Change |
|------|--------|
| `core/crates/kwaai-cli/src/node.rs` | Add `spawn_ollama_watcher()`, `spawn_p2pd_heartbeat()`, `spawn_relay_keepalive()`; wire recovery re-announce |
| `core/crates/kwaai-cli/src/config.rs` | Add `ollama_manage: bool` and `ollama_port: u16` fields to `NodeConfig` |
| `core/crates/kwaai-cli/src/cli.rs` | Add `kwaainet node health` subcommand |
| `core/crates/kwaai-cli/src/rag_cmd.rs` or new `node_cmd.rs` | Implement health query handler |

---

## Test Setup (Host-First Approach)

Start from the GPU host side:
```bash
# 1. On GPU host: start daemon with Ollama management
kwaainet start --daemon

# 2. Kill Ollama deliberately to test watcher
pkill ollama
# Expect: ERROR log within 45s, optional auto-respawn, re-announce

# 3. Kill p2pd deliberately
pkill p2pd
# Expect: heartbeat detects within 10s, restarts, re-registers handlers

# 4. Check status from host
kwaainet node health

# 5. From client: verify inference resumes automatically
kwaainet rag chat --kb D6 "test" --inference-url "p2p://PEER_ID"
```

---

## Out of Scope (This Plan)

- Client-side circuit breaker (separate concern)
- inference-mux reconnection improvements (already has `is_dead()` path)
- Streaming / SSE response support over P2P
- Multi-GPU load balancing

---

## Implementation Notes (v0.4.118)

### What was built

**1. Ollama health watcher** (`node.rs`, before event loop)
- `tokio::spawn` background task; polls `localhost:<ollama_port>/api/tags` every 15 s
- 3 consecutive failures → WARN + optional `ollama serve` spawn (if `ollama_manage=true`)
- Down→up transition → sends `()` on `ollama_recovery_tx` channel to main loop
- Main loop `select!` arm receives the signal and calls `announce()` immediately

**2. p2pd heartbeat** (`node.rs`, new `select!` arm)
- `tokio::time::interval(10s)` → polls `daemon.is_running()` every 10 s
- On crash: calls existing `restart_p2pd()` and resets `next_announce` to 30 s
- Detection window: 10 s (was 300 s)

**3. Relay circuit keepalive** (`node.rs`, new `select!` arm)
- `tokio::time::interval(60s)` → calls `client.identify()` every 60 s
- Keeps p2pd unix socket warm and exercises p2pd ↔ relay TCP connection
- Prevents idle NAT mappings from expiring between 300 s announces

**4. `kwaainet node health` command** (`cli.rs` + `main.rs`)
- Live probe of all three components
- p2pd: socket connect + `identify()` for peer ID, `list_peers()` for peer count
- Ollama: HTTP probe at configured port; shows model count from `/api/tags`
- Shows `ollama_manage` flag status

**5. Config additions** (`config.rs`)
- `ollama_manage: bool` — if true, kwaainet will spawn `ollama serve` on Ollama failure
- `ollama_port: u16` — default 11434; used by watcher and `node health` command

### Config example

```yaml
# ~/.kwaainet/config.yaml
ollama_manage: true   # kwaainet will restart Ollama if it crashes
ollama_port: 11434    # default; only needed if using a non-standard port
```

### Test procedure

```bash
# On GPU host — start daemon
kwaainet start --daemon

# Check live status
kwaainet node health

# Kill Ollama to test watcher
pkill ollama
# → ERROR within 45 s; if ollama_manage=true, re-spawns automatically

# Kill p2pd to test heartbeat
pkill p2pd
# → WARN within 10 s; p2pd restarts; re-announce at 30 s mark
```
