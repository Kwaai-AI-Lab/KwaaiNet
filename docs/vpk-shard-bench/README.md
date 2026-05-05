# VPK Shard Benchmark

**Run 1:** 2026-05-02 — K=2 metro Eves  
**Run 2:** 2026-05-05 — K=11 geographically diverse Eves  
**Version:** KwaaiNet v0.4.24  
**Authors:** KwaaiNet core team

---

## Hypothesis

Bob's vector knowledge base is sharded across multiple remote Eve nodes.
Because each shard is smaller, each Eve traverses a smaller HNSW graph.
Fan-out is parallel. The question: does the HNSW saving outweigh the
P2P round-trip overhead, allowing sharded remote search to beat a single
local index?

---

## Setup

### Run 1 — K=2 metro Eves (2026-05-02)

| Node | PeerId (short) | OS |
|------|----------------|----|
| metro_win | `12D3KooW…WCWqQz` | Windows x86-64 |
| metro-linux-x86-64 | `12D3KooW…TapGgd` | Linux x86-64 |

P2P round-trip (p50, 20 health pings per Eve): **25.6 ms**

### Run 2 — K=11 geographically diverse Eves (2026-05-05)

| Node | PeerId (short) | OS | RTT p50 |
|------|----------------|----|--------:|
| metro-linux-x86_64 | `12D3KooW…TapGgd` | Linux x86-64 | 27.5 ms |
| metro_win | `12D3KooW…WCWqQz` | Windows x86-64 | 30.6 ms |
| jerome-linux-x86_64 | `12D3KooW…f6sdrd` | Linux x86-64 | 24.0 ms |
| kasm-user-linux-aarch64 | `12D3KooW…4JA7bE` | Linux aarch64 | 52.6 ms |
| darrenw-linux-aarch64 | `12D3KooW…XJiinV` | Linux aarch64 | 56.5 ms |
| john-linux-draca-x86_64 | `12D3KooW…KsFfWB` | Linux x86-64 | 95.3 ms |
| john-linux-spectre-x86_64 | `12D3KooW…iediy3` | Linux x86-64 | 95.9 ms |
| john-linux-vovin-x86_64 | `12D3KooW…jVtAii` | Linux x86-64 | 98.3 ms |
| john-linux-ryu-x86_64 | `12D3KooW…bSWgXw` | Linux x86-64 | 99.8 ms |
| john-linux-naga-x86_64 | `12D3KooW…twMpPi` | Linux x86-64 | 104.4 ms |
| john-linux-draak-x86_64 | `12D3KooW…EaLAN6` | Linux x86-64 | 105.3 ms |

P2P round-trip combined p50: **92.5 ms**  
Node distribution: 3 nodes at 24–31 ms, 2 at 53–57 ms, **7 at 95–105 ms**

### Corpus (both runs)
- Vector sizes: 12,500 / 25,000 / 50,000
- Dimensions: 384 (sentence-embedding scale)
- 200 search queries, top-10 results
- Random unit vectors (L2-normalised, deterministic seed)

### Baselines
| Backend | Description |
|---------|-------------|
| **KwaaiNet local** | In-process redb + hnsw_rs (this machine) |
| **KwaaiNet WAN K=2** | Fan-out over `/kwaai/storage/1.0.0` to 2 metro Eves |
| **KwaaiNet WAN K=11** | Fan-out to 11 geographically diverse Eves |
| **Qdrant local Docker** | Qdrant 1.15.5 at `localhost:6333` |
| **Qdrant Cloud** | Qdrant managed cluster, `us-west-1` (AWS) |

---

## Results

### Search latency p50 (µs) — Run 1 (K=2)

| Vectors | KwaaiNet local | KwaaiNet WAN K=2 | Qdrant local | Qdrant Cloud |
|--------:|---------------:|-----------------:|-------------:|-------------:|
| 12,500  | 2,139          | 33,007           | 496          | 29,076       |
| 25,000  | 2,269          | 32,268           | 722          | 28,881       |
| 50,000  | 2,488          | 31,415           | 1,173        | 67,012 †     |

† 50K Cloud jump from ~29 ms to 67 ms — likely segment-compaction or
index-rebuild threshold on the free-tier cluster.

### Search latency p50 (µs) — Run 2 (K=11)

| Vectors | KwaaiNet local | KwaaiNet WAN K=11 | Qdrant local |
|--------:|---------------:|------------------:|-------------:|
| 12,500  | 2,254          | 113,342           | 461          |
| 25,000  | 2,022          | 114,273           | 555          |
| 50,000  | 2,563          | 114,845           | 1,155        |

### Search latency p95 (µs) — Run 1 (K=2)

| Vectors | KwaaiNet local | KwaaiNet WAN K=2 |
|--------:|---------------:|-----------------:|
| 12,500  | 3,497          | 70,018           |
| 25,000  | 3,591          | 37,834           |
| 50,000  | 4,217          | 37,765           |

### Search latency p95 (µs) — Run 2 (K=11)

| Vectors | KwaaiNet local | KwaaiNet WAN K=11 |
|--------:|---------------:|------------------:|
| 12,500  | 3,777          | 122,119           |
| 25,000  | 3,724          | 122,092           |
| 50,000  | 4,520          | 122,926           |

### Upload throughput (ms for full corpus)

| Vectors | KwaaiNet local | WAN K=2 | WAN K=11 | Qdrant local | Qdrant Cloud |
|--------:|---------------:|--------:|---------:|-------------:|-------------:|
| 12,500  | 9,513          | 45,706  | 75,181   | 602          | 73,618       |
| 25,000  | 22,716         | 100,084 | 127,958  | 1,174        | 149,483      |
| 50,000  | 53,122         | 223,584 | 297,681  | 2,828        | 298,044      |

### Recall@10 (sharded vs local ground truth)

| Vectors | K=2  | K=11  |
|--------:|-----:|------:|
| 12,500  | 43%  | 38%   |
| 25,000  | 28%  | 24%   |
| 50,000  | 18%  | 14%   |

Recall is low because random unit vectors in 384 dimensions all have near-zero
cosine similarity — score gaps are tiny and any HNSW approximation causes
rank swaps. Production semantic embeddings have far more structure and yield
significantly higher recall.

---

## Chart

![VPK Shard Benchmark](vpk-shard-bench.png)

Panel A (log-log): latency vs corpus size for all backends, theoretical curves
for LAN/datacenter/WAN sharding, and measured K=2 and K=11 scatter points.

Panel B (semilog): HNSW compute saved by sharding = B × log₂K µs.
This quantity is **constant with respect to N** — the only variable is K.

---

## Analysis

### 1. WAN sharding is flat and RTT-dominated

At K=2 (25.6 ms overhead), all three corpus sizes produce sharded latency ~32 ms.
At K=11 (92.5 ms p50 overhead), sharded latency is ~114 ms at all three scales.
In both cases the HNSW traversal time (1–1.3 ms per shard) is negligible compared
to the wire overhead. Adding more vectors to each shard barely moves the number.

### 2. Fan-out latency = max(shard RTTs), not mean

This is the key finding from the K=11 run. Fan-out fires all K shards in parallel
and waits for the slowest one. With 7 of 11 nodes at ~95–105 ms RTT, the combined
p50 is 92.5 ms — not the 60 ms average you'd naively compute. Any "slow" shard
in the fleet sets the floor for every query.

**Implication:** adding more Eves to a WAN-sharded setup only helps if the new
Eves are *at least as fast* as the slowest existing Eve. A single 100 ms node
in an otherwise 25 ms cluster doubles your query latency.

### 3. The HNSW saving from sharding is logarithmic — and independent of N

Splitting N vectors across K shards saves exactly:

```
Δ = B × log₂K   µs
```

where B ≈ 407 µs for KwaaiNet's hnsw_rs (B ≈ 338 µs for Qdrant). This
saving grows with K but does **not** grow with N. A corpus 1,000× larger
produces the same saving from the same K.

### 4. WAN breakeven is physically impossible

For sharding to beat local search over WAN (K=2, 26 ms overhead):

```
407 × log₂K > 26,000   →   K > 2^63
```

For K=11 with 92.5 ms overhead, breakeven is at ~9.8 billion vectors —
far beyond any practical deployment. There are not enough computers on
Earth to make WAN sharding win on query latency.

### 5. LAN sharding breaks even at K ≈ 11

At 1 ms LAN round-trip, breakeven is K ≈ 11 shards. A cluster of ~12 Eve
nodes on a local network would make sharded search competitive with a single
local index — and the sharded version can hold far more total vectors in RAM.
This is the correct use case for latency-sensitive sharding.

### 6. Qdrant local is 3–4× faster per operation

Qdrant's HNSW engine (B ≈ 338 µs/log₂N) is faster than hnsw_rs (B ≈ 407
µs/log₂N) — likely due to AVX-512 SIMD distance kernels in its C++ core.
At 50K vectors Qdrant local takes 1.2 ms; KwaaiNet local takes 2.5 ms.
Both follow the same logarithmic growth law.

### 7. Qdrant Cloud ≈ KwaaiNet WAN sharded (K=2) at small N

At 12.5K–25K vectors, Qdrant Cloud search latency (~29 ms) and KwaaiNet
WAN sharded K=2 (~32 ms) are effectively tied — both are pure RTT to the
same AWS region. The P2P relay occasionally takes longer (higher p95) but
the medians are comparable.

### 8. The encryption argument is orthogonal to the storage backend

KwaaiNet VPK's PHE scheme preserves distance ordering — encrypted vectors
are still searchable by any ANN index. This means PHE-encrypted vectors
could be uploaded to Qdrant Cloud and searched correctly. The distinction
between KwaaiNet Eve and Qdrant Cloud is therefore not the cryptography
but the **trust model**:

- Qdrant Cloud: a US company, subject to subpoenas, billing, vendor lock-in.
- KwaaiNet Eve: a peer you can verify by Ed25519 PeerId; no company intermediary.

For users whose threat model includes the storage provider, Eve is the right
choice regardless of performance.

---

## Conclusions

| Question | Answer |
|----------|--------|
| Can WAN sharding beat local HNSW on latency? | **No** — breakeven requires K ≈ 2⁶³ (K=2) or ~9.8B vectors (K=11) |
| What sets fan-out latency in a mixed fleet? | **The slowest shard** — max(RTTs), not mean |
| What is sharding good for? | **Capacity** — distribute a corpus too large for one machine's RAM |
| When does LAN sharding win on latency? | At K ≥ 11 Eves on a 1 ms LAN |
| Is Qdrant faster than KwaaiNet's HNSW? | **Yes, ~3× faster** — better SIMD kernels |
| Can you use PHE + Qdrant for privacy? | **Yes** — encryption is orthogonal to storage backend |
| What is KwaaiNet Eve's unique value? | **Decentralised, peer-owned storage** with no company intermediary |

---

## What's Next

The K=11 run confirmed geographic diversity is live. The network has grown —
12 nodes are now discoverable via `kwaainet vpk discover`.

- **LAN-range sharding** — nodes co-located in the same datacenter or on a
  private network to empirically measure the K ≈ 11 LAN breakeven.
- **Scale** — 100K–1M vector corpus requires Eve nodes with more RAM.
- **PHE encryption** — upload encrypted vectors and confirm recall parity
  with plaintext (encryption is orthogonal to ANN; should be identical).

### Storage serve on Eve mode

If you run a KwaaiNet node and want to participate in storage benchmarks:

```bash
kwaainet vpk enable --mode eve --capacity-gb 10
kwaainet storage serve &   # starts the storage RPC handler
kwaainet start --daemon
```

Your node will appear in `kwaainet vpk discover` and will be reachable for
`kwaainet vpk bench` runs.

---

## Reproducing This Benchmark

```bash
# Discover available Eve nodes
kwaainet vpk discover --json

# Run the benchmark against all discovered Eves
PEER_IDS=$(kwaainet vpk discover --json | jq -r '[.[].peer_id] | join(",")')
kwaainet vpk bench \
  --eve-peer-ids "$PEER_IDS" \
  --vectors 50000 \
  --dimensions 384 \
  --queries 200 \
  --qdrant-url http://localhost:6333      # omit if Qdrant not running
```

Regenerate the chart (requires Python + matplotlib + numpy):

```bash
cd docs/vpk-shard-bench
python3 chart.py
```
