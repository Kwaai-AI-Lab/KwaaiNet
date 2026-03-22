# Reputation in the KwaaiNet trust fabric

In KwaaiNet, **reputation** is how each node summarizes its experience of other nodes in the Layer 8 trust fabric. It is always **local** (per-node), built from **objective evidence**, and used to make routing and scheduling decisions without any central registry.

---

## 1. Local, subjective views — no central scores

Every node maintains its **own** reputation view of peers:

- There is **no global reputation score** and no central authority that assigns or revokes trust tiers.
- Each node computes a **local trust score** per peer, using its own weights and risk tolerance.
- Two nodes can legitimately disagree about the same peer because they have seen different behavior.

Reputation is therefore "what this node believes about that peer right now, based on what it has seen and what it has heard from others," not a universal fact.

---

## 2. Evidence: claims vs observed behavior

For each peer, a node tracks two main evidence streams:

**Claims (assertions)**
- Verifiable Credentials (VCs) and self-descriptions: "I can do 10 tokens/s," "I maintain 99.9% uptime," "I host model X."
- Encoded as W3C VCs like `ThroughputVC`, `UptimeVC`, `VerifiedNodeVC`, `PeerEndorsementVC`.

**Observed behavior (measurements)**
- What actually happens when the evaluator uses the peer:
  - Effective throughput (tokens/s) vs claimed.
  - Latency / jitter.
  - Job success/failure rates.
  - Uptime and availability over time.

**Example:**

- Eve claims 10 tokens/s throughput.
- Bob runs several jobs and measures an average of 6 tokens/s.
- Bob records a **fulfilled-promise ratio** of `6 / 10 = 0.6` for Eve's throughput claim (60%).

This attribute-level score becomes part of Eve's local reputation on Bob's node, independent of what any credential says.

---

## 3. Computing local trust scores

Each node combines credential evidence and behavioral evidence into a single **local trust score** per peer, which maps into tiers: `Unknown`, `Known`, `Verified`, and `Trusted`.

**Credential component** (from the whitepaper):

```
score_vc = min(1.0, Σ weight(vc_type) × 0.5^(age_days / 365))
```

Older VCs naturally lose weight unless renewed.

**Metrics component** (illustrative):

```
s_throughput  = observed / claimed throughput  (clamped to [0, 1])
s_uptime      = normalized uptime fraction
s_availability = fraction of successful requests

score_metrics = w_t × s_throughput + w_u × s_uptime + w_a × s_availability
```

**Overall local trust score:**

```
trust_local = α × score_vc + (1 - α) × score_metrics
```

Weights and thresholds are **configurable per node**, so an operator can choose how much to rely on credentials vs live performance.

These scores are discretized into tiers, which are used when routing intents (e.g. "only use nodes with trust tier ≥ Verified").

---

## 4. Transitive trust: asking others "what do you think?"

Local reputation can be enriched by asking **trusted peers for their view of another node**, in an EigenTrust-style propagation step.

**Example with Alice, Bob, Eve:**

- Bob has no direct experience with Eve, but he trusts Alice.
- Bob asks Alice: "What is your trust score for Eve, and what is it based on?"
- Alice returns her local score and, optionally, attribute-level metrics and credential summaries, signed with her identity key.

Bob then combines:

- **Direct evidence** — Bob's own measurements of Eve (if any).
- **Credential evidence** — VCs Bob has verified for Eve.
- **Transitive evidence** — Alice's (and others') scores for Eve, weighted by Bob's trust in each recommender.

**EigenTrust-style formula:**

```
T_B_E_direct  = Bob's direct score for Eve
T_B_i         = Bob's score for introducer i
T_i_E         = introducer i's score for Eve

T_B_E_trans   = Σ_i  T_B_i × T_i_E

trust_B_E     = w_direct × T_B_E_direct + w_trans × T_B_E_trans
```

This remains **Bob's local view**; no global scores are created.

---

## 5. Accountability for endorsements

Endorsements themselves affect a node's reputation. When Alice recommends Eve to Bob:

- If Eve behaves as Alice indicated (meets throughput, reliability, and uptime expectations), Alice's **endorsement reliability** improves.
- If Eve repeatedly under-performs relative to Alice's endorsement, Bob can **lower Alice's endorsement score**, even if Alice's own node performs well.

Over time, each node tracks:

- How well a peer behaves **directly** (jobs it runs).
- How well that peer's **endorsements** hold up when acted upon.

The planned EigenTrust propagation layer explicitly includes a term for transitive trust, so that:

- Nodes that recommend reliable peers gain reputational weight.
- Nodes that frequently oversell or misrepresent peers see their recommendations discounted by the network.

This aligns incentives: nodes should only make endorsements they are willing to stand behind, because those endorsements feed back into their own reputation in the trust fabric.

---

## 6. Role of reputation in routing and scheduling

Reputation is used at several key decision points:

- **Intent routing** — When resolving an intent like "run model X with minimum trust tier Verified, max latency Y," nodes filter candidates by local trust tier and pick shard chains that satisfy both trust and capability constraints.
- **Shard selection and rebalancing** — Nodes may bias shard selection toward peers with a strong track record for that model or workload profile.
- **Future features** — Tool-calling, VPK shard placement, and intent-casting marketplaces can all be gated or weighted by local reputation, ensuring that sensitive operations are preferentially routed through highly trusted nodes.

In all cases, reputation remains **decentralized, evidence-based, and local**, forming a trust fabric that guides Layer 8 behavior without ever collapsing into a single global score.
