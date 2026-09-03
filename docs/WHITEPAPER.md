# KwaaiNet: A Trust-Centered Decentralized AI Network

**Technical Whitepaper — Version 0.3**
**Kwaai Foundation — March 2026**

---

## Abstract

KwaaiNet is an open-source, decentralized AI network that turns heterogeneous commodity hardware into a shared AI computer governed by verifiable trust rather than corporate policy. Its architecture places cryptographic identity and attestation at the center of three intersecting capabilities: distributed transformer inference, encrypted knowledge retrieval, and encrypted peer-to-peer networking. This paper describes the design, the protocol decisions, the current implementation status, and the research directions that remain open.

---

## 1. Introduction

The rapid deployment of large language models has consolidated AI infrastructure around a small number of hyperscale cloud providers. This concentration creates risks that are structural, not incidental: users and institutions that depend on centralized inference have no verifiable guarantee about data handling, no recourse against jurisdictional interference, and no resilience against provider failure. Even when model weights are published under open licenses, the infrastructure that serves them remains opaque and centralized.

Prior work on decentralized AI infrastructure has addressed individual aspects of this problem. Petals [1] demonstrated that large transformer models can be run collaboratively across volunteer hardware by pipelining activations through a chain of servers, each holding a contiguous range of transformer blocks. Federated learning [2] distributes training while keeping data local but does not address inference or knowledge retrieval. Secure multi-party computation [3] and homomorphic encryption [4] provide cryptographic privacy for computation and data, but have generally been studied in isolation from the networking and identity challenges of a production peer-to-peer system.

KwaaiNet combines these strands into a unified architecture grounded in one design bet: that **trust, identity, and attestation must be first-class primitives**, not retrofitted policies. The system is implemented in Rust, exposes an OpenAI-compatible HTTP API so that existing tooling requires no modification, and maintains wire-format compatibility with the Python Hivemind DHT network [5] so that nodes are immediately visible on existing network maps.

---

## 2. Architecture Overview

Each KwaaiNet node is a single Rust process (`kwaainet`) composed of three subsystems and a mandatory trust core. The sketch below is illustrative — it names the architectural roles rather than types in the tree, and `carbon_tracker` in particular has no implementation today:

```rust
pub struct DistributedAINode {
    // Branch: Shared Compute
    inference_engine: CandelEngine,

    // Branch: P2P Network
    p2p_network: P2PNetwork,

    // Branch: Shared Secure Storage
    kb_engine: Option<KnowledgeBaseEngine>,

    // Center: Decentralized Trust
    identity: Option<IdentityProvider>,
    encryption_layer: E2EEncryption,
    trust_graph_client: Option<TrustGraphClient>,

    carbon_tracker: EnvironmentalMetrics,
    contribution_tracker: ResourceMetrics,
}
```

The three subsystems may be enabled independently: a node without a GPU can contribute storage and routing without serving inference. However, all participation — inference scheduling, storage placement, and connection gating — flows through the trust core. This paper treats each subsystem in turn, beginning with the trust core that underlies all of them.

---

## 3. Decentralized Trust

### 3.1 Persistent Cryptographic Identity

Each node generates an Ed25519 keypair [6] on first launch and writes it as a protobuf-encoded binary to `~/.kwaainet/identity.key`. The Ed25519 scheme was chosen for its compact key and signature sizes (32-byte public keys, 64-byte signatures), fast verification, and resistance to side-channel attacks compared with ECDSA [7]. The same key file is passed to the embedded `go-libp2p-daemon` [8] via its `-id` flag, ensuring that the libp2p `PeerId` and the application-layer identity are derived from the same keypair.

Two identifiers are derived deterministically from this keypair:

- **`PeerId`** — the libp2p peer identifier used for Kademlia DHT routing [9], stream multiplexing, and protocol dispatch.
- **`did:peer:`** — a W3C Decentralized Identifier [10] anchored to the same public key, used as the subject and issuer of Verifiable Credentials.

The persistence of this keypair is not merely a convenience: without a stable identity, each restart would produce a new `PeerId`, orphaning any credentials issued to the previous DID. The credential-identity binding would be broken every time the node restarts, making trust accumulation impossible.

### 3.2 Verifiable Credentials

The trust system implements W3C Verifiable Credentials Data Model 1.1 [11]. A `VerifiableCredential` carries a JSON-LD structure with `@context`, `type`, `issuer`, `issuanceDate`, optional `expirationDate`, and a `credentialSubject` block containing the attestation claims. Proofs use Ed25519 signatures over a canonical JSON serialization of the credential, allowing offline verification against the issuer's DID.

KwaaiNet defines six credential types, each representing a distinct tier of behavioral attestation:

| Credential | Issuer | Semantic Meaning |
|---|---|---|
| `FiduciaryPledgeVC` | GliaNet | Operator signed the GliaNet Fiduciary Pledge |
| `VerifiedNodeVC` | Kwaai Foundation | Node passed Foundation onboarding |
| `UptimeVC` | Bootstrap servers | Node maintained uptime ≥ threshold over N days |
| `ThroughputVC` | Peer-witnessed | Throughput within X% of advertised value |
| `SummitAttendeeVC` | Summit organizer | Attendance at a Kwaai Personal AI Summit |
| `PeerEndorsementVC` | Any peer | Peer-to-peer reliability endorsement |

Credentials are stored in a local wallet at `~/.kwaainet/credentials/` and can be imported, listed, and verified entirely offline. A `BindingVC` type additionally links a passkey `did:key:` identity to a node's `did:peer:` DID, enabling user-to-node attribution without requiring users to manage raw keypairs.

### 3.3 Local Trust Score

Trust scores are computed locally by the querying node — there is no central registry. The score is a function of a node's verifiable credentials: their types, their issuers, and how recently they were issued. Credentials decay in value over time, keeping the trust graph current without requiring frequent re-issuance, drawing on the intuition from PageRank-derived reputation systems that stale information should contribute less [12].

As an illustrative example, a time-decayed weighted sum over credential types might look like:

```
score = min(1.0, Σ weight(vc_type) × 0.5^(age_days / 365))
```

where each credential type carries a weight reflecting its attestation strength — a peer-witnessed throughput benchmark contributing less than a signed fiduciary pledge, for instance — and the exponential term gives roughly a one-year half-life. Scores map to human-readable tiers (Unknown, Known, Verified, Trusted) that gate access to sensitive workloads.

The exact weighting scheme, decay function, and tier boundaries are under active development and will be finalized as the credential ecosystem matures.

### 3.4 Planned: EigenTrust Propagation

The current score is credential-only. Phase 4 will add transitive peer endorsements via a variant of the EigenTrust algorithm [13], which was originally developed to suppress malicious peers in Gnutella-scale P2P file-sharing networks. EigenTrust's key property is Sybil resistance: endorsements from low-trust nodes propagate proportionally less, so an attacker who creates many colluding identities cannot manufacture high trust scores for arbitrary peers. The planned formula:

```
Phase 4 score =
  w₁ × DirectPeerRatings
  w₂ × CredentialScore
  w₃ × TransitiveEndorsements  (weight × 0.5 per hop)
  × TimeDecay(age_of_assertions)
```

Until Phase 4 ships, the `peer_endorsement_contribution` field in the score struct is always `0.0`, and the system falls back entirely to the credential-weighted baseline.

---

## 4. Shared Compute

### 4.1 Block-Sharded Distributed Inference

KwaaiNet implements Petals-style pipeline parallelism [1] at the transformer block level. A model's layers are divided into contiguous ranges; each range is served by a different node. A coordinator discovers the block chain from the DHT, opens a session with each server in sequence, and pipes activation tensors from one to the next until the final node produces logits.

**Model format.** Distributed inference operates on models stored in the SafeTensors format [14], which provides memory-mapped, zero-copy tensor loading and does not execute arbitrary code on deserialization — a meaningful security property in a network of heterogeneous operators. GGUF models [15] are supported for single-node inference only; their quantization schemes are not currently compatible with the hidden-state wire format.

**Attention and positional encoding.** The shard implementation handles the standard Llama-family architecture [16]: Rotary Positional Encoding (RoPE) [17], in which position information is injected by rotating query and key vectors in pairs of dimensions with position-dependent angles; Grouped Query Attention (GQA) [18], which reduces KV-cache memory by sharing a smaller number of key-value heads across multiple query heads; and SwiGLU activation in the feed-forward sublayers [19]. Correct broadcast semantics are required for both RoPE (query has 32 heads; the cosine/sine tables have 1) and causal attention masking (score tensor shape `[1, 32, s, s]` vs. mask shape `[1, 1, s, s]`) — the implementation uses explicit broadcast operations throughout rather than relying on operator overloading, following the `candle` framework's conventions [20].

**Inter-node transport protocol.** The activation transport uses a custom libp2p protocol:

```
/kwaai/inference/1.0.0
```

Messages are serialized using MessagePack [21], chosen for its compact binary representation and broad multi-language support. Tensor data is packed as raw little-endian bytes: token IDs as `u32-LE`, hidden states and logits as `f16-LE` (IEEE 754 half-precision [22]). Each exchange carries a `session_id: u64` that links to a per-session KV cache on the serving node; sessions expire after 600 seconds of inactivity and are garbage-collected by a background task.

The message flow is:

```
Coordinator                          Block Server
  │── InferenceRequest (msgpack) ──────────────▶│
  │   {session_id, seq_pos,                      │
  │    payload_type, shape, data}                │
  │                                              │  runs local blocks
  │◀── InferenceResponse (msgpack) ─────────────│
  │   {session_id, response_type,                │
  │    shape, data}                              │
```

The first node in the chain receives `payload_type: TokenIds`; every subsequent node receives `payload_type: HiddenStates`. The last node responds with `response_type: Logits`.

**DHT advertisement.** When a node starts serving blocks, it publishes a DHT record using the Hivemind wire format [5], which KwaaiNet extends with additional fields while maintaining backward compatibility (unknown map keys are silently ignored by legacy clients):

```
Key:   SHA1(msgpack("{model_prefix}.{block_index}"))
Value: Ext(64, msgpack([state_i32, throughput_f64,
         {start_block, end_block, peer_id, public_name, …}]))
```

The `peer_id` field in the map payload is a KwaaiNet extension that allows chain discovery to recover node addresses directly from DHT responses without a separate lookup round-trip.

**Warm-up state.** A node registers on the DHT and marks itself visible (`state = 0`) before weights have finished loading. The inference handler is backed by an `Arc<RwLock<Option<TransformerShard>>>` lazy cell — requests that arrive during warm-up block on the read lock until the shard is populated, so the global chain view remains consistent and coordinators do not need to handle a separate "node loading" error case.

**Dynamic rebalancing.** A background rebalancer periodically queries the DHT, identifies coverage gaps — block ranges with fewer than the target number of serving nodes — and restarts the local shard at a gap range. To prevent thundering-herd reconvergence when many nodes simultaneously detect the same gap, each node adds a jitter delay of 0–60 seconds derived deterministically from the last byte of its `PeerId`. This avoids synchronized re-announcements while remaining reproducible across restarts [23].

### 4.2 Sampling and Decoding

The final logit vector is sampled locally by the coordinator. The implementation supports temperature scaling, top-k filtering, and nucleus (top-p) sampling [24]. Temperature $T$ rescales logits before the softmax: low temperatures concentrate probability mass on the highest-scoring tokens (approaching greedy decoding at $T \to 0$); high temperatures flatten the distribution, increasing diversity. Top-k restricts sampling to the $k$ highest-probability tokens; top-p dynamically selects the smallest set of tokens whose cumulative probability exceeds threshold $p$, adapting the effective vocabulary to the model's confidence at each step.

### 4.3 OpenAI-Compatible API

`kwaainet shard api` exposes an HTTP server with a drop-in OpenAI-compatible interface:

```
GET  /v1/models
POST /v1/chat/completions    (streaming SSE + non-streaming)
POST /v1/completions         (streaming SSE + non-streaming)
```

The server discovers the shard chain once at startup and serializes concurrent requests through a single `Arc<Mutex<P2PClient>>` to avoid session-ID collisions on the DHT. Existing clients — editors, agents, evaluation harnesses — require no modification to use a KwaaiNet-backed inference endpoint.

### 4.4 Secret Custom Embeddings

For RAG workloads requiring stronger privacy guarantees than standard embedding storage provides, KwaaiNet supports tenant-specific embedding projection. A base embedding model produces a standard dense vector; the tenant applies a private linear projection that maps it into a tenant-specific latent space before storage or transmission.

Recent work has demonstrated that plain text can often be reconstructed with high fidelity from standard embeddings alone [25], making raw embedding storage a meaningful privacy risk for sensitive corpora. A private projection matrix raises the bar for inversion: an adversary who obtains stored vectors from a compromised storage node cannot reconstruct useful approximations without also possessing the projection, substantially complicating reconstruction attacks. The projection itself is never stored on network nodes; only the tenant holds it.

---

## 5. Shared Secure Storage

### 5.1 Virtual Private Knowledge (VPK)

The Shared Secure Storage layer is implemented as an embedded Rust crate (`kwaai-storage`) running in-process alongside the KwaaiNet node. The storage engine uses `redb` for durable vector persistence and `hnsw_rs` for approximate nearest-neighbour search. Each Eve node manages multiple tenants in isolated per-tenant HNSW indices.

The integration has two channels:

1. **PeerId binding.** Each Eve node's storage capability is bound to the node's `PeerId`. Tenants are created with the owner's `PeerId` as the authoritative identity, enabling future credential-gated access policies.

2. **DHT advertisement.** Before each DHT announcement cycle, the node queries the embedded storage engine directly and includes the capability metadata in two DHT records:
   - The per-block `Ext(64)` record gains a `"vpk"` field.
   - A separate `_kwaai.vpk.nodes` registry stores a per-peer dictionary of VPK capability maps, each entry containing `mode`, `capacity_gb`, `tenant_count`, and `vpk_version`. **No IP addresses or HTTP endpoints are included** — nodes are identified solely by PeerId.

**Remote access** follows the same model as the inference layer: remote Bobs connect to Eve nodes using the `/kwaai/storage/1.0.0` libp2p protocol, addressed by PeerId. The libp2p transport (circuit relay, Noise encryption, Yamux multiplexing) handles NAT traversal and authenticated transport without requiring port forwarding. An HTTP health API is available on `127.0.0.1:7432` for local operator tooling only; it is never exposed remotely.

Any node on the network can run `kwaainet vpk discover` to locate VPK-capable peers without running VPK itself.

**Empirical sharding note (May 2026).** Bench runs with K=2 metro Eves (WAN RTT p50 = 25.6 ms) and K=11 geographically diverse Eves (WAN RTT p50 = 92.5 ms) show that fan-out query latency is dominated entirely by P2P round-trip time. HNSW compute per shard is 1–2.5 ms; the WAN overhead is 10–50× larger at any tested corpus size. No practical shard count K overcomes this gap on internet-connected nodes — the breakeven corpus for K=2 WAN sharding is approximately 2⁶³ vectors. Cross-node sharding is a **capacity mechanism** — distributing a corpus that exceeds a single Eve node's RAM — not a query latency optimisation. LAN deployments (RTT ≤ 1 ms) are the sole exception: they break even at K ≈ 11.

### 5.2 Multi-Tenant Isolation

VPK nodes serve three roles:

| Mode | Role |
|---|---|
| `bob` | Data owner — embeds and submits documents; seals the vectors in the designed pipeline of §5.3 |
| `eve` | Storage provider — holds tenant-scoped vector shards, never the documents |
| `both` | Serves both roles simultaneously |

Every document, embedding, and metadata entry carries a `tenant_id` bound to its originating DID. All query execution is scoped to that identifier. Multiple tenants may share a physical Eve node without access to each other's data; the `tenant_id` column propagates through all storage, index, and audit tables.

A DHT-backed shard manager is planned to maintain the mapping from tenant knowledge bases to Eve nodes, handle replication, and rebalance on churn — applying the same gap-filling logic described for inference block sharding. Today a Bob node discovers Eve nodes by PeerId through the DHT and uses one; `vpk shard` and `vpk resolve` are stubs.

### 5.3 Homomorphic Encryption for Confidential Vector Search

> **Implementation status.** This section describes the designed pipeline, not what the reference
> node ships today. The partially homomorphic schemes below — dimensional scrambling, noise
> injection, and ROME — are implemented and evaluated in the Kwaai PHE project; the KwaaiNet node
> does not yet apply them, so vectors currently reach a storage node as raw embeddings. In both
> cases the storage node receives neither documents nor queries. Integration is designed in
> [KwaaiNet #170](https://github.com/Kwaai-AI-Lab/KwaaiNet/pull/170).

The knowledge plane is designed around a pipeline that preserves confidentiality through the full retrieval path, drawing on the body of work on homomorphic encryption [4, 26] and encrypted similarity search [27].

**Preparation (data owner / Bob).**
1. Compute dense embeddings for each document.
2. Apply the tenant-specific secret custom embedding projection (§4.4), followed by dimensional scrambling and carefully tuned noise injection.
3. Encrypt the resulting vectors with a homomorphic scheme that supports approximate inner-product similarity without decryption.

**Encrypted storage (storage nodes / Eve).**
Eve nodes store only encrypted, scrambled, noise-injected embeddings and minimal encrypted metadata, partitioned into tenant-scoped shards. Eve nodes never hold plaintexts, embedding parameters, or decryption keys.

**Encrypted search (querying party / Alice).**
Alice's query is embedded and transformed under the same tenant secrets, then evaluated homomorphically against the stored encrypted vectors. Eve nodes compute approximate encrypted similarity scores — they observe neither the query embedding nor the document embeddings in plaintext.

**Result handling.**
Eve returns encrypted scores and document indices. Only key-holding parties — Bob, an authorized delegate, or a confidential-computing enclave — decrypt scores, select top-*k* results, and route approved context to the language model. The LLM itself receives only plaintext excerpts that Bob's key-holder chose to disclose.

This pipeline is similar in spirit to cuVS-backed encrypted retrieval [28], but operates in a decentralized multi-tenant setting without a trusted coordinator.

### 5.4 Inversion and Wire-Tap Resistance

Two independent properties compound to resist passive adversaries:

- **Inversion resistance.** The combination of secret projection, dimensional scrambling, and noise injection makes high-quality embedding inversion substantially harder than against standard publicly-produced embeddings. The secret projection alone does not constitute information-theoretic security, but it eliminates the black-box inversion attack demonstrated in [25] by requiring the adversary to also recover the projection matrix.

- **Wire-tap resistance via index-based retrieval.** Similarity search returns compact document indices, not content. A network adversary observing query traffic captures encrypted scores and integer indices — not document text — then must separately compromise the content channel to reconstruct the retrieved passages. This reduces leakage from traffic-pattern analysis compared with approaches that stream full document content in response to similarity queries.

Trust scoring governs which Eve nodes are eligible to hold a given tenant's shards, ensuring that Shared Secure Storage always intersects the central trust layer for placement and access decisions.

---

## 6. P2P Network

### 6.1 Transport Stack

All inter-node communication uses **libp2p** [8] as the networking substrate, run in-process through rust-libp2p (the default since v0.6.0; the earlier embedded `go-libp2p-daemon` remains selectable with `native_p2p: false`). libp2p provides transport-layer encryption via Noise [29], stream multiplexing via Yamux, NAT traversal via circuit relay, and a protocol negotiation layer (multistream-select) that maps human-readable protocol strings to typed handlers.

Three application-layer protocols are currently registered:

- `DHTProtocol.rpc_find` — Hivemind DHT lookups; wire-format compatible with Python Hivemind bootstrap nodes.
- `/kwaai/inference/1.0.0` — block-shard activation transfers (§4.1).
- `/kwaai/storage/1.0.0` — storage fabric operations (§5.1): tenant CRUD, vector upload/search/delete, and health queries. All operations use a msgpack envelope `{ op, tenant_id, payload }`. Nodes are addressed by PeerId; the libp2p relay provides NAT traversal with no open ports required.

### 6.2 Distributed Hash Table

KwaaiNet's DHT uses the Kademlia algorithm [9] as implemented by the Hivemind protocol [5]. Kademlia organizes nodes in a binary tree keyed by XOR distance, providing O(log N) lookup time for N participating nodes and tolerating concurrent failures without a central coordinator. Keys are 20-byte DHTID values derived by SHA-1 hashing msgpack-encoded key strings — the same derivation used by Python Hivemind — ensuring interoperability:

```
DHTID(key) = SHA1(msgpack(key))
```

KwaaiNet nodes appear on external network visualization tools (e.g., `map.kwaai.ai`) without additional bridging, because they participate in the same DHT namespace as the broader Hivemind network.

Bootstrap peers are drawn from a configurable `initial_peers` list in `~/.kwaainet/config.yaml`; if empty, the network defaults to known Petals bootstrap addresses. DHT records carry a 360-second TTL and are re-announced on a background loop.

### 6.3 Intent Routing

Clients express intent rather than binding to fixed servers: "run model X over these token IDs", "RAG over tenant Y with minimum trust tier Z". The network resolves each intent into concrete nodes:

1. Query the DHT for all block records matching the target model prefix.
2. Decode each `Ext(64)` record to extract peer IDs, block ranges, and announced throughput.
3. Rank candidates by block coverage, using the local trust score as a tie-breaker.
4. Construct the inference chain using a widest-coverage-first ordering; skip nodes that fail RPC calls with a logged warning rather than aborting the request.

Trust currently **weights** selection rather than gating it: a lower-trust node is preferred less, but is still used when it is the only peer covering a block range. Filtering against a minimum tier declared in the intent is planned rather than shipped. It requires a configurable minimum tier, and it requires enough of the fleet to have earned one — a node stays at `Unknown` until it has accumulated several recorded observations, so gating today would reject most of the network and leave the block range uncovered.

The design intent for that filtering is that enforcement remains decentralized: no single gateway applies it, and each node applies its own local view of trust rather than deferring to a shared authority.

---

## 7. Security Properties and Limitations

### 7.1 Properties Provided

- **Stable cryptographic identity** with a verifiable attestation chain anchored in W3C standard credentials.
- **Encrypted transport** for all inter-node traffic via Noise and TLS 1.3.
- **Partial inference privacy.** Intermediate shard nodes receive only hidden-state activation tensors — not the original prompt tokens — and return hidden states rather than generated text. An adversary operating a single middle shard observes neither the input nor the output of an inference request in plaintext.
- **Tenant-isolated storage.** Every vector carries a `tenant_id` bound to its originating DID and all query execution is scoped to it, so tenants sharing an Eve node cannot read each other's data. The storage node holds vectors and returns `(id, score)`; it never receives documents or queries. Sealing those vectors so the host cannot read them either is designed but not yet shipped — see §5.3.
- **Local trust computation** — no central authority can grant or revoke trust; each node computes its own view.

### 7.2 Known Limitations

**Collusion attack on inference.** An adversary who controls both the first shard node (which sees input token IDs) and the last shard node (which produces output logits) can reconstruct the prompt-to-completion pair. Trust tiers mitigate this risk by restricting which nodes may serve boundary positions for sensitive workloads, but do not eliminate it. The long-term mitigation is confidential-computing enclaves (e.g., Intel TDX, AMD SEV-SNP) for inference shards, which the architecture is designed to accommodate.

**Unsigned DHT records.** DHT entries in the current Hivemind wire format are not cryptographically signed by their authors. A node with DHT write access can publish false capability records — advertising blocks it does not serve, or VPK capacity it does not have. Trust scoring provides partial defense (low-trust nodes are deprioritized), but the underlying DHT protocol does not enforce record authenticity.

**Phase 4 peer endorsements not yet implemented.** Current trust scores are credential-only; the EigenTrust propagation layer that provides Sybil resistance for peer endorsement graphs is planned for Phase 4.

**Cross-node VPK shard placement.** Eve node discovery is operational (`kwaainet vpk discover`), but cross-node shard splitting — placing a tenant's encrypted knowledge base across multiple Eve nodes with redundancy — is targeted for Phase 2 of the VPK roadmap.

**WAN sharding does not reduce search latency.** Fan-out query time is bounded by the slowest shard's RTT, not the mean. Empirical measurements (May 2026, K=2 and K=11 WAN Eve fleets) record P2P overhead at 25.6 ms and 92.5 ms respectively. HNSW compute savings from sharding are B × log₂K µs — always smaller than one round-trip at any tested corpus size. Sharding should be used to distribute a corpus that exceeds a single Eve's RAM, not to improve query speed on WAN-connected nodes. See [VPK shard benchmark](../docs/vpk-shard-bench/README.md) for full data.

**Node version opacity.** Nodes running older builds silently fail RPC calls when the serving protocol is not registered. The `kwaainet_version` field in the DHT wire format is tracked as a near-term addition to make version skew visible in `kwaainet shard chain` output.

---

## 8. Implementation Status

| Feature | Status |
|---|---|
| Ed25519 persistent identity + `did:peer:` DID | Shipped |
| W3C VC wallet (import, list, verify, Ed25519 proof) | Shipped |
| Credential-weighted trust score with time decay | Shipped |
| DHT-based shard discovery (Hivemind-compatible) | Shipped |
| Block-sharded distributed inference over libp2p | Shipped |
| RoPE + GQA + SwiGLU shard implementation (SafeTensors) | Shipped |
| Session KV cache (600 s TTL) | Shipped |
| Temperature / top-k / top-p sampling | Shipped |
| Dynamic gap-filling rebalancer with per-node jitter | Shipped |
| OpenAI-compatible shard API (streaming SSE + non-streaming) | Shipped |
| HuggingFace snapshot download (`kwaainet shard download`) | Shipped |
| VPK DHT advertisement + health polling | Shipped |
| VPK node discovery (`kwaainet vpk discover`) | Shipped |
| EigenTrust peer-graph propagation | Phase 4 |
| Cross-node VPK Eve shard placement | Phase 2 |
| DHT-backed VPK KB resolution | Phase 3 |
| Confidential-computing enclave support for inference | Research |
| Signed DHT records | Research |
| Homomorphic encryption backend | Research |

---

## 9. Conclusion

KwaaiNet is not a research prototype of decentralized AI — it is a working system running on commodity hardware today. Its defining architectural commitment is to treat trust as infrastructure rather than policy: every node has a verifiable identity, every claim about uptime or throughput is a signed credential, and every scheduling and placement decision is computed locally from first principles rather than delegated to a central authority.

The encrypted storage and inference pipelines are deliberately incremental. The current system already separates activation tensors from plaintext prompts at intermediate shard nodes, and stores knowledge bases with strict tenant isolation on nodes that never receive the source documents. The path to full end-to-end confidentiality — homomorphic encrypted vector search, confidential-computing inference enclaves, and EigenTrust-grounded Sybil resistance — is well-defined and laid on a foundation that is already running at scale.

KwaaiNet is open-source, developed by a nonprofit AI lab, and designed to run on hardware already owned by individuals, communities, and institutions. It is intended to remain that way.

---

## References

[1] A. Borzunov, D. Baranchuk, T. Dettmers, M. Riabinin, Y. Belkada, A. Chumachenko, P. Samygin, and C. Raffel, "Petals: Collaborative Inference and Fine-tuning of Large Models," in *Proceedings of the 61st Annual Meeting of the Association for Computational Linguistics (ACL 2023)*, 2023.

[2] B. McMahan, E. Moore, D. Ramage, S. Hampson, and B. A. y Arcas, "Communication-Efficient Learning of Deep Networks from Decentralized Data," in *Proceedings of the 20th International Conference on Artificial Intelligence and Statistics (AISTATS)*, 2017, pp. 1273–1282.

[3] A. C. Yao, "Protocols for Secure Computations," in *Proceedings of the 23rd IEEE Symposium on Foundations of Computer Science (FOCS)*, 1982, pp. 160–164.

[4] C. Gentry, "A Fully Homomorphic Encryption Scheme," Ph.D. dissertation, Stanford University, 2009.

[5] M. Riabinin and E. Gusev, "Towards Crowdsourced Training of Large Neural Networks using Decentralized Mixture-of-Experts," in *Advances in Neural Information Processing Systems (NeurIPS 2021)*, 2021.

[6] D. J. Bernstein, N. Duif, T. Lange, P. Schwabe, and B.-Y. Yang, "High-speed high-security signatures," in *Cryptographic Hardware and Embedded Systems (CHES 2011)*, Lecture Notes in Computer Science, vol. 6917, Springer, 2011, pp. 124–142.

[7] D. R. L. Brown, "SEC 2: Recommended Elliptic Curve Domain Parameters," Certicom Research, Standards for Efficient Cryptography, Version 2.0, 2010.

[8] Protocol Labs, "libp2p: A Modular, p2p Networking Stack," [Online]. Available: https://libp2p.io, 2019.

[9] P. Maymounkov and D. Mazières, "Kademlia: A Peer-to-peer Information System Based on the XOR Metric," in *Proceedings of the 1st International Workshop on Peer-to-Peer Systems (IPTPS)*, Lecture Notes in Computer Science, vol. 2429, Springer, 2002, pp. 53–65.

[10] M. Sporny, D. Longley, M. Sabadello, D. Reed, O. Steele, and C. Allen, "Decentralized Identifiers (DIDs) v1.0," W3C Recommendation, World Wide Web Consortium, July 2022.

[11] M. Sporny, D. Longley, D. Chadwick, O. Steele, and B. Zundel, "Verifiable Credentials Data Model v1.1," W3C Recommendation, World Wide Web Consortium, March 2022.

[12] L. Page, S. Brin, R. Motwani, and T. Winograd, "The PageRank Citation Ranking: Bringing Order to the Web," Stanford InfoLab Technical Report, 1999.

[13] S. D. Kamvar, M. T. Schlosser, and H. Garcia-Molina, "The EigenTrust Algorithm for Reputation Management in P2P Networks," in *Proceedings of the 12th International World Wide Web Conference (WWW 2003)*, 2003, pp. 640–651.

[14] HuggingFace, "SafeTensors: A simple, safe way to store and distribute tensors," [Online]. Available: https://github.com/huggingface/safetensors, 2022.

[15] B. Schmidt, "GGUF: GPT-Generated Unified Format," llama.cpp project specification, [Online]. Available: https://github.com/ggerganov/llama.cpp, 2023.

[16] H. Touvron, L. Martin, K. Stone, P. Albert, A. Almahairi, Y. Babaei, N. Bashlykov, S. Batra, P. Bhargava, S. Bhosale et al., "Llama 2: Open Foundation and Fine-Tuned Chat Models," arXiv preprint arXiv:2307.09288, 2023.

[17] J. Su, Y. Lu, S. Pan, A. Murtadha, B. Wen, and Y. Liu, "RoFormer: Enhanced Transformer with Rotary Position Embedding," *Neurocomputing*, vol. 568, p. 127063, 2024.

[18] J. Ainslie, J. Lee-Thorp, M. de Jong, Y. Zemlyanskiy, F. Lebrón, and S. Sanghai, "GQA: Training Generalized Multi-Query Transformer Models from Multi-Head Checkpoints," in *Proceedings of the 2023 Conference on Empirical Methods in Natural Language Processing (EMNLP)*, 2023, pp. 4895–4901.

[19] N. Shazeer, "GLU Variants Improve Transformer," arXiv preprint arXiv:2002.05202, 2020.

[20] HuggingFace, "Candle: A Minimalist ML Framework for Rust," [Online]. Available: https://github.com/huggingface/candle, 2023.

[21] S. Furuhashi, "MessagePack: It's like JSON. But fast and small," [Online]. Available: https://msgpack.org, 2008.

[22] IEEE, "IEEE Standard for Floating-Point Arithmetic," IEEE Std 754-2019, 2019.

[23] M. Castro, P. Druschel, Y. C. Hu, and A. Rowstron, "Exploiting Network Proximity in Distributed Hash Tables," in *Proceedings of the 1st International Workshop on Peer-to-Peer Systems (IPTPS)*, 2002. *(Jitter and backoff strategies for DHT convergence.)*

[24] A. Holtzman, J. Buys, L. Du, M. Forbes, and Y. Choi, "The Curious Case of Neural Text Degeneration," in *International Conference on Learning Representations (ICLR 2020)*, 2020.

[25] J. Morris, V. Kuleshov, V. Shmatikov, and A. Rush, "Text Embeddings Reveal (Almost) As Much As Text," in *Proceedings of the 2023 Conference on Empirical Methods in Natural Language Processing (EMNLP)*, 2023, pp. 12448–12460.

[26] Z. Brakerski and V. Vaikuntanathan, "Fully Homomorphic Encryption from Ring-LWE and Security for Key Dependent Messages," in *Advances in Cryptology (CRYPTO 2011)*, Lecture Notes in Computer Science, vol. 6841, Springer, 2011, pp. 505–524.

[27] W.-J. Lu, Z. Huang, C. Hong, Y. Ma, and H. Qu, "PEGASUS: Bridging Polynomial and Non-polynomial Evaluations in Homomorphic Encryption," in *IEEE Symposium on Security and Privacy (S&P 2021)*, 2021.

[28] NVIDIA, "cuVS: GPU-Accelerated Vector Search," [Online]. Available: https://github.com/rapidsai/cuvs, 2024.

[29] T. Perrin, "The Noise Protocol Framework," [Online]. Available: https://noiseprotocol.org/noise.html, 2018.

---

*Source: [github.com/Kwaai-AI-Lab/KwaaiNet](https://github.com/Kwaai-AI-Lab/KwaaiNet) | License: Apache 2.0 | Developed by the Kwaai Foundation*
