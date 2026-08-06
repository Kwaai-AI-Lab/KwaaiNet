# Bootstrap Server v2 — Requirements & Design

> **Status:** Design phase
> **Replaces:** OpenAI-Petal `docker/kwaainet_bootstrap/` (Python, wraps `petals.cli.run_dht`)
> **Target:** Rust crate `core/crates/kwaai-bootstrap/`
> **Strategic priority:** High — bootstrap is the single point of failure for the entire network

---

## 1. Why Rewrite?

### Current state

The existing bootstrap server is a 50-line shell script that:
1. Decodes a Base64 RSA-2048 private key from an env var
2. Shells out to `python -m petals.cli.run_dht`
3. Exits if Petals is updated incorrectly

This is a thin wrapper around a Python library. Every problem with it is external — no observability, no control plane, no operator interface, and zero ability to evolve the protocol.

### Why this matters architecturally

Bootstrap servers are the **single point of entry** for new peers. Every node that joins KwaaiNet connects to bootstrap-1 or bootstrap-2 first. This creates:

| Risk | Severity |
|------|----------|
| A single DDoS can take the network dark | Critical |
| All joining peers reveal their real IP to bootstrap | High |
| Fixed peer IDs = targetable by adversaries | High |
| No load distribution — 2 servers for 10 000 nodes | Medium |
| No insight into what traffic is bootstrap vs. DHT amplification | Medium |
| Python dependency chain breaks on Petals updates | Low |

### Opportunity

A ground-up Rust rewrite lets us rethink the entire model:

- **Federation**: anyone can run a bootstrap node; the network routes around failures
- **Privacy**: onion-routed introductions; bootstrap never learns who is talking to whom
- **Scalability**: stateless introduction layer; DHT state is distributed, not concentrated
- **Anti-abuse**: PoW challenges, rate limiting, trust-gated fast-lane
- **Observability**: Prometheus metrics, structured logs, health endpoint

---

## 2. Architecture Overview

```
                    ┌──────────────────────────────────────┐
                    │       Bootstrap Cluster              │
                    │                                      │
  New peer          │  ┌──────────┐    ┌──────────┐       │
  joining   ──────▶ │  │ Entry    │    │ Entry    │  ...  │
                    │  │ Node A   │    │ Node B   │       │
                    │  │ (Kwaai)  │    │ (Kwaai)  │       │
                    │  └────┬─────┘    └────┬─────┘       │
                    │       │  Gossip       │              │
                    │  ┌────▼─────────────▼────┐          │
                    │  │  Control Plane Mesh   │          │
                    │  │  (SWIM / Raft-lite)   │          │
                    │  └───────────────────────┘          │
                    └──────────────────────────────────────┘
                            │
                    ┌───────▼───────────────────┐
                    │  Community Bootstrap Nodes │
                    │  (federation, permissioned)│
                    └───────────────────────────┘
```

### Three logical layers

**Layer 1 — Introduction** (`kwaai-bootstrap-entry`)
Receives new peer connections, issues a challenge, hands off a peer-list response. Stateless per request. Horizontally scalable. Does NOT store DHT state.

**Layer 2 — DHT fabric** (`kwaai-bootstrap-dht`)
Runs a full Kademlia DHT node that persists routing table state. Separate from the introduction layer so introduction servers can be ephemeral/serverless while DHT state is durable.

**Layer 3 — Control plane** (`kwaai-bootstrap-ctl`)
Admin API + Prometheus metrics endpoint. Manages the federation list. Issues signed attestations for community bootstrap operators.

---

## 3. Core Design Decisions

### 3.1 Stateless introduction servers

The introduction server does exactly one thing: given a new peer, return a list of existing peers to connect to. It keeps no persistent state — it queries the DHT fabric for a fresh peer sample on each request. This means introduction servers can be:

- Run on serverless / edge (Cloudflare Workers compatible subset)
- Horizontally scaled behind an anycast address
- Killed and restarted without affecting network state
- Replaced entirely without a migration

### 3.2 Signed peer-lists — no trust required

Every peer-list response is signed by the bootstrap keypair. Peers verify the signature before using the list. This means:

- A compromised Anycast router cannot inject fake peer lists
- Community bootstrap nodes sign with their own key, auditable on-chain
- Peers cache signed lists locally — they can rejoin without contacting bootstrap if they have a recent signed list (gossip convergence as fallback)

### 3.3 Privacy-preserving introductions

**Problem**: today, bootstrap sees the real IP of every joining peer and knows which model they want to run.

**Solution**: two-phase introduction with a mixing step:

```
Phase 1: Peer → Bootstrap (encrypted to bootstrap pubkey)
         { ephemeral_pubkey, challenge_response, intent_hash }
         "intent" = H(model_prefix) — bootstrap sees the hash, not the model name

Phase 2: Bootstrap → Peer (encrypted to ephemeral_pubkey)
         { peer_list: [addr, peer_id, ...], signed_by: bootstrap_keypair }
         Peer list is a random sample — bootstrap does not know which peer the
         joining node will actually connect to first.
```

No persistent log of joining IPs. Session state is in-memory only, discarded after 60 s. Operators may configure `log_joins = false` (default) or `log_joins = true` for debugging.

### 3.4 Anti-abuse: proof-of-work challenges

Joining the DHT is cheap — a single UDP packet. This makes bootstrap servers prime targets for amplification attacks. Every introduction request must include a valid PoW solution:

```
challenge = H(timestamp_rounded_to_30s || client_ip_prefix || bootstrap_pubkey)
solution  = nonce such that H(challenge || nonce) has difficulty leading zeros
difficulty = adaptive (targets 50 ms solve time on a modern CPU)
```

Legitimate nodes solve this once per epoch (30 s). Bots doing amplification attacks must solve it for each packet, burning CPU. The challenge is stateless on the bootstrap side (verified by recomputation, not lookup).

Fast-lane exception: peers with a `KwaaiNet:TrustedNodeVC` can present their VC and skip PoW. The VC is verified against the trust registry signature, not a live RPC call.

### 3.5 Federation — community bootstrap operators

Bootstrap operators register by:
1. Running `kwaainet bootstrap register --stake <vc>` — submits a `BootstrapOperatorVC` to a trust registry (**to be chosen**: this role was assigned to summit-server, which has been removed)
2. Receiving a signed `BootstrapAdmissionVC` authorising them to serve introductions
3. Publishing their multiaddr in the `_kwaai.bootstrap.nodes` DHT key

Existing nodes periodically refresh their bootstrap list from `_kwaai.bootstrap.nodes`, so new community bootstrap servers are automatically discovered without config changes.

Revocation: if a bootstrap operator misbehaves (returning bad peer lists, logging IPs in violation of privacy policy), the trust registry revokes their BootstrapAdmissionVC. Nodes will stop using revoked bootstrap servers within one DHT crawl cycle (~60 s).

### 3.6 Geographic distribution and anycast

Two Kwaai-operated bootstrap servers are not enough for global low-latency joins. The entry layer should be deployable as:

- **Anycast IP** — one global IP, traffic routed to nearest PoP by BGP
- **DNS-based** — `bootstrap.kwaai.ai` with low-TTL regional CNAME targets
- **Community nodes** — geographically distributed, trusted via VC

Target regions for Kwaai-operated nodes: US-West, US-East, EU-West, AP-Southeast.

### 3.7 Bootstrap-free rejoin via gossip

A fully bootstrapped peer maintains a local signed peer cache (`~/.kwaainet/peer-cache.json`). On restart, if bootstrap servers are unreachable, the peer attempts direct connections to cached peers. If any cached peer is live, normal DHT routing resumes without ever touching bootstrap. This eliminates the "bootstrap is down, network is dark" failure mode for established nodes. Only truly new nodes (first ever connection) require bootstrap.

---

## 4. Functional Requirements

### 4.1 Introduction service

- Accept libp2p `Identify` + custom `Introduce` protocol on port 8000 (TCP) and 8001 (QUIC)
- Respond with a signed peer-list of N=20 random peers from DHT routing table
- PoW verification (adaptive difficulty)
- TrustedNodeVC fast-lane bypass
- Rate limit: 10 introductions/IP/minute, 1000/IP/hour
- No persistent session state
- Health endpoint: `GET /health` → `{"status":"ok","peers_known":N,"uptime_secs":T}`

### 4.2 DHT node

- Full Kademlia implementation (reuse `kwaai-hivemind-dht` crate)
- Persist routing table to SQLite (`~/.kwaainet/bootstrap-routing.db`) — survives restart
- Replicate DHT records to at least 3 other bootstrap nodes
- Expose DHT metrics: routing table size, put/get RPS, replication lag

### 4.3 Federation control plane

- `GET /api/federation/nodes` — list of all active community bootstrap nodes with VC status
- `POST /api/federation/register` — submit BootstrapOperatorVC, returns admission VC on success
- `DELETE /api/federation/nodes/:peer_id` — revoke a community node (Kwaai admin only)
- Prometheus metrics: `kwaai_bootstrap_introductions_total`, `kwaai_bootstrap_pow_failures_total`, `kwaai_bootstrap_peers_known`, `kwaai_bootstrap_federation_size`

### 4.4 Peer cache gossip

- Nodes broadcast their current peer-list snapshot to 3 random peers every 5 minutes
- Receiving peers merge into their cache, keeping the 100 most recently seen entries
- Signed by originating peer — receivers verify before merging
- Wire format: MessagePack `{ts, signer_peer_id, sig, peers: [{peer_id, multiaddr, last_seen}]}`

### 4.5 Key management

- Ed25519 keypair (replace current RSA-2048 — shorter, faster, constant-time)
- Keypair generated on first start if not present; stored at `~/.kwaainet/bootstrap-identity.bin`
- Peer ID derived as `SHA2-256(protobuf(Ed25519_pubkey))` — compatible with existing libp2p peer IDs
- Key rotation: new keypair can be pre-registered in DHT before cutover, old keypair signs a "I am moving to new key" attestation for 7 days

---

## 5. Non-Functional Requirements

### 5.1 Performance targets

| Metric | Target |
|--------|--------|
| Introduction latency p50 | < 20 ms |
| Introduction latency p99 | < 100 ms |
| Concurrent introductions | 10 000 / server |
| DHT record propagation | < 5 s across all bootstrap nodes |
| Memory per introduction server | < 64 MB |
| Restart time (with cached routing table) | < 2 s |

### 5.2 Resilience

- Introduction servers restart automatically on panic (systemd `Restart=always`)
- DHT nodes must maintain at least 3-way replication for any stored record
- If all Kwaai bootstrap servers go dark, established peers can still communicate via gossip peer cache
- Community bootstrap nodes provide ≥ 50% of introduction capacity by design (not a fallback)

### 5.3 Privacy guarantees (published in privacy policy)

- Bootstrap servers **do not log** joining peer IPs in any persistent store (enforced by code, not policy)
- `intent_hash` (model hash) is retained in memory for PoW state only, discarded after 60 s
- No analytics beacon, no third-party SDK in the introduction server
- Operators may audit this guarantee by building from source (reproducible builds)

### 5.4 Security

- All inter-bootstrap traffic encrypted with Noise_XX (libp2p standard)
- Admin API behind mTLS with a Kwaai-issued client certificate
- PoW difficulty auto-adjusts to prevent CPU exhaustion under attack
- Rate limits enforced at the socket layer before any allocation (no heap allocation on bad requests)
- Bootstrap peer IDs are long-lived (rotation announced ≥ 7 days in advance) — removes the incentive for targeted attacks based on predictable peer ID churn

---

## 6. Wire Protocol

### Introduction request (client → bootstrap)

```
MessagePack array:
[
  version: u8,                  // protocol version = 1
  ephemeral_pubkey: bytes[32],  // X25519, for response encryption
  pow_nonce: u64,               // PoW solution
  intent_hash: bytes[32],       // SHA256(model_prefix), optional
  vc_token: bytes | null        // TrustedNodeVC, optional fast-lane
]
```

### Introduction response (bootstrap → client)

```
MessagePack array:
[
  version: u8,
  timestamp: u64,               // unix seconds
  peers: [                      // up to 20 peers
    { peer_id: bytes, multiaddrs: [string] }
  ],
  next_epoch_challenge: bytes[32], // pre-compute PoW for next 30s window
  signature: bytes[64]          // Ed25519 over canonical msgpack of above
]
```

Response is encrypted to `ephemeral_pubkey` using X25519-Salsa20-Poly1305 (NaCl box). The bootstrap server never learns which symmetric key is used — only the ephemeral pubkey.

---

## 7. Crate Structure

```
core/crates/kwaai-bootstrap/
├── Cargo.toml
└── src/
    ├── main.rs          — CLI: bootstrap serve / bootstrap keygen / bootstrap status
    ├── server.rs        — tokio listener, dispatch to introduction / DHT / admin
    ├── introduction.rs  — stateless introduction handler, PoW verification
    ├── pow.rs           — adaptive PoW challenge/verify
    ├── federation.rs    — community node registry, VC verification
    ├── gossip.rs        — peer cache gossip protocol
    ├── keys.rs          — Ed25519 key management, peer ID derivation
    ├── metrics.rs       — Prometheus exposition
    └── config.rs        — YAML config, env var overrides
```

Reuses existing workspace crates:
- `kwaai-hivemind-dht` — Kademlia DHT
- `kwaai-p2p` — libp2p transport
- `kwaai-trust` — VC verification
- `kwaai-p2p-daemon` — p2pd socket client

---

## 8. CLI Interface

```bash
# Generate a new bootstrap identity
kwaainet bootstrap keygen --output ~/.kwaainet/bootstrap-identity.bin

# Start bootstrap server
kwaainet bootstrap serve \
  --identity ~/.kwaainet/bootstrap-identity.bin \
  --bind 0.0.0.0:8000 \
  --quic 0.0.0.0:8001 \
  --admin 127.0.0.1:9090 \
  --federation                     # enable community node registration

# Show bootstrap server status
kwaainet bootstrap status

# Register as a community bootstrap operator
kwaainet bootstrap register \
  --vc path/to/BootstrapOperatorVC.json \
  --bootstrap /dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/Qm...

# Show current federation (all registered bootstrap nodes)
kwaainet bootstrap peers
```

---

## 9. Migration Path

### Phase 1 — Drop-in replacement (no protocol change)
1. Build `kwaai-bootstrap` crate with existing Kademlia + libp2p
2. Same peer IDs as current Python servers (same private key, converted to Ed25519 or wrapped)
3. Deploy alongside Python bootstrap; swap DNS once stable
4. Verify: existing `kwaainet` nodes connect without config change

### Phase 2 — Privacy layer
1. Add PoW challenge
2. Add encrypted introduction request/response
3. Add intent_hash
4. Update `kwaainet` client to use new protocol (backward-compatible: falls back to plain if bootstrap does not support v1)

### Phase 3 — Federation
1. `_kwaai.bootstrap.nodes` DHT key
2. Community registration endpoint
3. BootstrapOperatorVC issuance (issuer TBD — summit-server removed)
4. `kwaainet` auto-discovers community nodes

### Phase 4 — Peer cache gossip
1. Gossip protocol in `kwaainet` node
2. Bootstrap-free rejoin for established peers
3. Deprecate requirement to always contact bootstrap on restart

---

## 10. Success Metrics

| Metric | Baseline | Target |
|--------|----------|--------|
| Bootstrap servers as SPOF | Yes (2 servers) | No (federation) |
| Introduction latency p99 | ~800 ms (Python) | < 100 ms |
| Nodes that can rejoin without bootstrap | 0% | > 90% (established) |
| Joining peer IPs logged persistently | Yes | No |
| Community-operated bootstrap capacity | 0% | ≥ 50% of introductions |
| Bootstrap server memory | ~200 MB (Python) | < 64 MB |
| Supported concurrent introductions | ~200 | 10 000 |

---

## 11. Security Threat Model

| Threat | Mitigation |
|--------|-----------|
| DDoS on bootstrap IP | Anycast + rate limiting + PoW |
| Sybil attack (fake community nodes) | BootstrapAdmissionVC required, revocable |
| Peer list poisoning | Signed responses; peers verify before using |
| IP harvesting of joining peers | Ephemeral encrypted introductions, no persistent IP log |
| Bootstrap key compromise | Key rotation protocol; 7-day overlap |
| BGP hijack of bootstrap anycast | Signed peer lists; cached lists as fallback |
| Amplification via bootstrap | PoW at socket layer; no heap alloc on bad requests |
| Eclipse attack on new node | Return random sample, not closest-only peers |

---

## 12. Open Questions

1. **Ed25519 ↔ existing RSA peer IDs**: Are the two existing Python bootstrap peer IDs pinned in node configs across the fleet? If so, we must either keep the same derived peer IDs or run a graceful handoff window where both old and new peer IDs are announced.

2. **QUIC transport**: libp2p QUIC support in Rust is stable but adds ~15 MB to binary. Is the latency improvement (no handshake RTT) worth it for bootstrap specifically?

3. **Federation governance**: who decides which community operators get BootstrapAdmissionVCs? This was to be a Kwaai-controlled summit-server, which has since been removed — so the near-term issuer is now an open question, with a DAO or multi-sig still the longer-term goal.

4. **PoW accessibility**: adaptive PoW may disadvantage very low-power devices (Raspberry Pi). Should there be a VC-free but bandwidth-limited slow-lane?
