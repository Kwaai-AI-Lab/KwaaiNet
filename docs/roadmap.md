# Roadmap: Layer 8 destination vs current implementation

This roadmap defines KwaaiNet's direction by comparing:

- The **aspirational Layer 8 architecture** from the Layer 8 and node architecture whitepapers.
- The **current Rust implementation** (CLI, node, crates, and deployed network).
- The **gap** between the two, expressed as roadmap items and contribution opportunities.

It is a living document and should be updated whenever new capabilities ship or the design evolves.

---

## 1. Trust: decentralized Layer 8 trust graph

### 1.1 Destination (what Layer 8 wants)

KwaaiNet's trust layer is designed as a five-layer pipeline:

1. **Cryptographic identity** — Ed25519 keypair, `PeerId`, and `did:peer:` DID anchor all claims.
2. **Verifiable Credentials (VCs)** — W3C VCs for fiduciary pledges, uptime, throughput, node verification, events, and peer endorsements.
3. **Local trust score** — Time-decayed, credential-weighted score computed independently by each node; no central registry.
4. **Testable Credentials (TCs)** — Forward-looking contextual evaluation (PVP-1) that tests "does this claim apply to my workload now?".
5. **EigenTrust-style propagation** — Transitive, Sybil-resistant trust propagation across the network.

Together, these provide complete Layer 8 trust: cryptographic history plus contextual meaning and transitive endorsements.

### 1.2 Current implementation

Today, the Rust node and ecosystem provide:

- Ed25519 keypair at `~/.kwaainet/identity.key` and derived `PeerId` / `did:peer:` DID.
- W3C VC wallet at `~/.kwaainet/credentials/` with concrete types such as `FiduciaryPledgeVC`, `VerifiedNodeVC`, `UptimeVC`, `ThroughputVC`, `EventAttendeeVC`, and `PeerEndorsementVC`.
- Local trust scoring logic with time decay and tiers (`Unknown`, `Known`, `Verified`, `Trusted`).
- Active participation in Trust over IP (ToIP) Decentralized Trust Graph work for interoperability.

### 1.3 Roadmap gaps and contributions

**Gaps**

- Implement the **Testable Credential (TC) layer** and PVP-1 protocol for contextual evaluation.
- Design and implement **EigenTrust-style propagation** integrated with the existing trust graph.
- Harden **Sybil resistance** and cross-domain trust exchange in line with ToIP patterns.

**Contribution ideas**

- Prototype TC and PVP-1 as a Rust module, with a small CLI for evaluating example credentials.
- Implement and benchmark different EigenTrust variants on top of existing trust data.
- Help define interoperability formats with ToIP (schemas, DID/VC profiles, cross-network trust import/export).

---

## 2. Compute: shared, sharded LLM infrastructure

### 2.1 Destination

Compute is meant to turn heterogeneous devices into a single Layer 8 inference and training fabric:

- Petals-style block-sharded inference for large open models.
- Decentralized training on sharded weights.
- Safe, trust-gated tool-calling for agents.
- End-to-end secure chat-completion and RAG pipelines, including KV-cache protections.

### 2.2 Current implementation

As of the latest `kwaainet` crate and node releases:

- CandelEngine implements **block-sharded inference** over SafeTensors models.
- Modern model features: RoPE positional encoding, Grouped Query Attention (GQA), SwiGLU, per-session KV cache with TTL, temperature/top-k/top-p sampling.
- OpenAI-compatible HTTP API (`/v1/models`, `/v1/chat/completions` with streaming) for inference.
- Smart model selection and node health tracking in the network.

### 2.3 Roadmap gaps and contributions

**Gaps**

- Design and implement **decentralized training** on sharded weights.
- Develop **trust-routed inference** and KV-cache scrambling or other mitigations for collusion attacks.
- Implement **tool-calling mediated by the trust graph** (credential-gated tool access, audit trails).

**Contribution ideas**

- Extend CandelEngine to support training loops over the existing sharding protocol.
- Explore KV-cache scrambling / partitioning strategies and their security/performance trade-offs.
- Prototype a trust-gated tool-calling layer (e.g. simple "tool registry" filtered by trust tier / credential type).

---

## 3. Storage: Virtual Private Knowledge (VPK) and distributed memory

### 3.1 Destination

KwaaiNet aims to treat personal and organizational knowledge as a distributed, privacy-preserving Layer 8 system:

- VPK as a multi-tenant knowledge base bound to node identity and credentials.
- Homomorphic-encrypted vector search across untrusted nodes.
- Cross-node shard placement for redundancy and locality (Phase 2).
- DHT-backed resolution of knowledge bases for fully distributed personal AI memory and parallel retrieval (Phase 3).

### 3.2 Current implementation

Currently, VPK:

- Runs as a separate process bound to the node's `PeerId`.
- Supports roles `bob` (personal), `eve` (encrypted inference), and `both` (shared).
- Performs homomorphic-encrypted vector search over multi-tenant vector tables.
- Advertises presence and health via the DHT; cross-node shard placement is not yet deployed.

### 3.3 Roadmap gaps and contributions

**Gaps**

- Implement **cross-node VPK shard placement** and policies for redundancy, locality, and trust.
- Implement **DHT-backed knowledge base resolution** for fully distributed personal AI memory.
- Strengthen homomorphic encryption tooling and performance for large-scale use.

**Contribution ideas**

- Design shard placement strategies (e.g. trust-weighted, geography-aware, capacity-aware) and implement an initial prototype.
- Implement DHT record schemas and resolution logic for discovering and aggregating VPK shards.
- Benchmark and tune the encrypted vector search pipeline on realistic workloads.

---

## 4. Network: P2P fabric and intent-casting

### 4.1 Destination

The network layer should provide a credibly neutral Layer 8 business protocol:

- Trust-gated, intent-based routing for inference and knowledge.
- Intent-casting: people and their agents broadcast machine-readable intents (needs, offers, goals) and find verifiable, trust-scored counterparties.
- Economic settlement and audit trails that reflect the trust graph and protect participants from fraud.

### 4.2 Current implementation

The shipping network stack includes:

- libp2p with Kademlia DHT (Hivemind-compatible) for discovery and record distribution.
- Circuit relay for residential NAT traversal, Yamux for stream multiplexing.
- Intent-based routing for inference: "model X, minimum trust tier T, max latency Y" → shard chain selection.

### 4.3 Roadmap gaps and contributions

**Gaps**

- Define and implement **intent-casting schemas** (for intents and responses).
- Bind intents tightly to person-anchored identities and legal responsibility.
- Design governance patterns to keep marketplaces credibly neutral and prevent routing capture.

**Contribution ideas**

- Draft and test initial JSON / Protobuf schemas for intents and replies.
- Implement a prototype "intent bus" on top of libp2p for a narrow use case (e.g., evaluation infra).
- Explore neutral governance and routing policies, drawing on Project VRM and related work.

---

## 5. Governance and ecosystem

### 5.1 Destination

KwaaiNet is intended to be governed as shared infrastructure ("Linux of AI"), not a single-vendor product:

- Corporate membership tiers (Bronze, Silver, Gold, Platinum) aligned with Trust Equation 2.0 "mutual benefit" levels.
- Individual membership as a way for people to co-own the infrastructure they use.
- Integration with broader open AI ecosystems and standards (ToIP, open model communities, etc.).

### 5.2 Current implementation

Today:

- Kwaai Foundation operates as a 501(c)(3) nonprofit stewarding KwaaiNet.
- Membership and ecosystem work (alliances, standards bodies, open-source collaborations) are ongoing.
- Governance details are primarily documented in narrative/whitepaper form, not yet codified as repo docs.

### 5.3 Roadmap gaps and contributions

**Gaps**

- Document **governance structures** and decision-making processes in this repo.
- Clarify how trust artifacts (VCs, TCs, EigenTrust signals) connect to governance actions.
- Provide guidance for partners integrating their own governance or compliance requirements.

**Contribution ideas**

- Help draft governance and contributor docs that mirror the principles in the whitepapers.
- Propose mechanisms for mapping technical trust signals into concrete governance actions (e.g., access tiers, escalation paths).

---

## 6. How to use this roadmap as a contributor

If you're looking for where to help:

- **1 hour** — File issues clarifying docs, identify inconsistencies between code and this roadmap, suggest small API or UX improvements.
- **1 day** — Pick a small gap (e.g., a missing config option in docs, a KV-cache experiment, a DHT record improvement) and propose a focused PR.
- **1 week+** — Align with one of the destination areas (trust, compute, storage, network, governance), discuss design on community channels, and work on a multi-PR feature or research prototype.

Please sync on design in issues or discussions before starting larger features so that work stays aligned with the Layer 8 vision and existing implementation.
