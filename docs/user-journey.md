# KwaaiNet Player Journey
## From Invitation to Architect — a Game Design Framework

**Version:** 1.0  
**Date:** 2026-05-04  
**Status:** Product Design — v1 draft

---

## Design Principles

KwaaiNet's user progression borrows deliberately from game design:

- **Every action has a visible reward.** XP, a new badge, a higher trust tier, a new capability unlocked.
- **The next objective is always clear.** At every level, the player knows exactly what to do next and why it matters.
- **Progress is permanent.** Verifiable Credentials in the wallet are the on-chain equivalent of achievements — they travel with the player across restarts, device changes, and network migrations.
- **The network effect is the game.** The platform gets more valuable as more players reach higher levels. Inviting others and endorsing peers are first-class game actions.
- **No level is a dead end.** Each level is genuinely useful on its own; higher levels add power, not just status.

---

## The World Map

```
LEVEL 0          LEVEL 1          LEVEL 2          LEVEL 3          LEVEL 4          LEVEL 5
────────         ────────         ────────         ────────         ────────         ────────
VISITOR    ───▶  MEMBER     ───▶  CITIZEN    ───▶  CONTRIBUTOR ──▶  GUARDIAN   ───▶  ARCHITECT
                                                                     
Browser          PassKey          Node on          Serving          Eve + Pledge     Governance
only             identity         network          inference        operator         + Builder

0 XP             50 XP            500 XP           5,000 XP         25,000 XP        100,000 XP
```

---

## Level 0 — Visitor

> *"You've been invited to a different kind of AI."*

### Entry
- Receives an invitation link, or discovers KwaaiNet organically.
- No account, no install, no commitment.

### Experience
- **Browser-only.** The Kwaai web app runs a lightweight WASM inference node directly in the browser tab — no server required.
- Can send prompts to the distributed network and receive responses.
- Sees the live network map at map.kwaai.ai — nodes by location, throughput, trust tier.
- Sees a "shadow profile": *"Your device could contribute approximately X tok/s to the network."*
- Every response carries a footnote: which nodes served the inference, their trust tier, and whether vectors left the network.

### Objectives
| # | Objective | XP |
|---|-----------|-----|
| 1 | Send your first inference request | 5 XP |
| 2 | View the network map | 5 XP |
| 3 | Explore your shadow profile | 5 XP |
| 4 | Read your first inference provenance receipt | 5 XP |

### Rewards
- 20 XP (enough to see progress toward Level 1)
- Unlocks: invitation code to share with one other person

### Boss Moment — the gate to Level 1
**Register a PassKey.** One tap with Face ID / Touch ID / Windows Hello. No email, no password.

### What stops players here
- Don't yet understand why a node matters.
- Friction: "I just want to try the AI, not run infrastructure."
- **Design response:** The shadow profile is the hook — show them concretely what their machine could do, and what they'd earn for doing it. The invite they can give a friend is a social hook.

---

## Level 1 — Member

> *"You exist on the network. Your AI remembers you."*

### Entry
- Completed PassKey registration.
- Has a persistent session identity (FIDO2 credential, device-bound or cloud-synced).

### Experience
- Personal AI: conversation history persists across sessions (stored locally, encrypted with PassKey-derived key).
- Can configure privacy level: Anonymous → Basic → Personalized.
- Appears on map.kwaai.ai with a placeholder node badge (grey, "Member — not yet running a node").
- Can earn XP by using the AI — inference requests contribute to the network's collective usage signal.
- Receives a `MemberVC` issued by the Kwaai platform on PassKey verification.

### Objectives
| # | Objective | XP |
|---|-----------|-----|
| 1 | Complete 10 inference sessions | 10 XP |
| 2 | Configure your privacy level | 10 XP |
| 3 | Check your trust wallet (`kwaainet identity show`) | 10 XP |
| 4 | Invite one friend (they must reach Level 1) | 50 XP |
| 5 | Attend a Kwaai Personal AI Summit (earns `SummitAttendeeVC`) | 100 XP |

### Rewards
- 50–180 XP depending on objectives completed
- `MemberVC` in wallet (+0.05 trust score weight)
- `SummitAttendeeVC` on summit attendance (+0.10 trust score weight)
- Access to longer context windows (vs anonymous Level 0)
- Unlock: install guide + setup wizard becomes available

### Boss Moment — the gate to Level 2
**Install `kwaainet`, run `kwaainet setup`, start the daemon, and appear on map.kwaai.ai.**

### What stops players here
- "Installing software feels like work."
- **Design response:** The setup wizard (from `NODE_UI_PLANNING.md`) does it in 4 clicks. The boss moment is celebrated with a confetti animation and a map notification: *"Your node just went live."*

---

## Level 2 — Citizen

> *"You are the network. Your node serves your community."*

### Entry
- `kwaainet` installed, daemon running, node visible on map.kwaai.ai.

### Experience
- Node appears on the map with a **blue Citizen badge**.
- Can see live stats: tokens served, uptime %, peers connected, trust score.
- **VPK Bob role**: private knowledge base active. Documents encrypted locally before leaving the device; query results return only IDs and scores, never raw content.
- Participates in distributed inference as a **consumer** — requests route preferentially through higher-trust nodes.
- 24-hour uptime earns first `UptimeVC` (provisional, reissued daily).

### XP Sources (ongoing)
| Action | XP rate |
|--------|---------|
| Node online | 1 XP / hour |
| Inference request served (even small) | 1 XP / 50 tokens served |
| 24h continuous uptime | 25 XP bonus |
| 7-day streak (node online ≥ 20h/day) | 100 XP bonus |
| First `UptimeVC` earned | 50 XP |

### Objectives
| # | Objective | XP |
|---|-----------|-----|
| 1 | Stay online for 72 continuous hours | 75 XP |
| 2 | Serve your first 10,000 tokens | 200 XP |
| 3 | Enable VPK (`kwaainet vpk enable --mode bob`) | 25 XP |
| 4 | Endorse one peer (`kwaainet identity endorse <peer_id>`) | 25 XP |
| 5 | Reach trust tier **Known** (score ≥ 0.10) | 50 XP |

### Rewards
- `UptimeVC` (provisional → permanent after 30 days, +0.10 trust weight)
- Map badge upgrades to **blue Citizen**
- Unlock: shard serving configuration in the UI
- Eligible for: inference circuit priority routing

### Boss Moment — the gate to Level 3
**Serve 100,000 tokens over at least 30 days of uptime. Earn `ThroughputVC`.**

### What stops players here
- Node goes offline when the laptop closes.
- **Design response:** "Always-on" setup guide (systemd / launchd / Windows service), surfaced prominently in the dashboard when uptime drops. UI shows: *"You're 3 days of uptime away from your ThroughputVC."*

---

## Level 3 — Contributor

> *"The network is stronger because you're in it."*

### Entry
- 30+ days uptime, 100K tokens served, `ThroughputVC` earned.

### Experience
- Serving as a **block shard server**: `kwaainet shard serve` hosts one or more transformer blocks and participates in distributed inference chains.
- Node appears on map with a **green Contributor badge** and throughput stat.
- Peers actively route inference requests through the node.
- Eligible for **enterprise inference workloads** (HIPAA/GDPR track once `VerifiedNodeVC` obtained).
- Can participate in `kwaainet shard circuit` — named, stable peer paths that other users can pin to for consistent sessions.
- Earns `ThroughputVC` (peer-witnessed, Kwaai Foundation co-signed).

### XP Sources (ongoing)
| Action | XP rate |
|--------|---------|
| Node online | 1 XP / hour |
| Tokens served as block shard | 1 XP / 20 tokens served |
| 30-day uptime streak | 500 XP bonus |
| `ThroughputVC` issued | 200 XP |
| Peer endorsement received | 50 XP each |
| Another player reaches Level 2 via your invite chain | 100 XP |

### Objectives
| # | Objective | XP |
|---|-----------|-----|
| 1 | Serve 1,000,000 tokens total | 500 XP |
| 2 | Maintain 90-day uptime (≥ 20h/day average) | 500 XP |
| 3 | Receive 3 peer endorsements | 150 XP |
| 4 | Pass `VerifiedNodeVC` onboarding with Kwaai Foundation | 300 XP |
| 5 | Reach trust tier **Verified** (score ≥ 0.40) | 200 XP |

### Rewards
- `ThroughputVC` (+0.20 trust score weight — highest single observable metric)
- `VerifiedNodeVC` on passing Kwaai onboarding (+0.25 trust score weight)
- Map badge upgrades to **green Contributor** with tok/s stat
- Priority routing: requests preferentially flow through Contributor+ nodes
- Access to enterprise workload queue

### Boss Moment — the gate to Level 4
**Enable Eve storage, initialize a VPK storage node with ≥ 10 GB capacity, and sign the GliaNet Fiduciary Pledge.**

---

## Level 4 — Guardian

> *"You hold the network's knowledge, and your word is your bond."*

### Entry
- Contributor with `VerifiedNodeVC`.
- Eve storage node running with ≥ 10 GB committed.
- GliaNet Fiduciary Pledge signed.

### Experience
- Node serves as a **VPK Eve**: stores opaque vectors for Bob nodes across the network, and never receives their documents. (Sealing those vectors client-side so the host cannot read them is designed but not yet wired in.)
- Appears on map with a **purple Guardian badge** and a shield icon (Fiduciary Pledge).
- Eligible for HIPAA and GDPR-classified workloads.
- `FiduciaryPledgeVC` issued by the GliaNet Foundation (+0.30 trust weight — the single highest-weight VC in the system).
- Eve storage tenants accumulate; each tenant is a Bob node that trusts this Guardian with their encrypted knowledge.
- Governance weight: 1 vote per 1,000 XP in community proposals.

### XP Sources (ongoing)
| Action | XP rate |
|--------|---------|
| Node online (inference + storage) | 2 XP / hour |
| Eve tenant served (active) | 10 XP / tenant / day |
| Storage contributed | 5 XP / GB·day |
| Tokens served as block shard | 1 XP / 20 tokens |
| New peer endorsement received | 75 XP each |
| Another player reaches Level 3 via your invite chain | 250 XP |

### Objectives
| # | Objective | XP |
|---|-----------|-----|
| 1 | Serve 10 active VPK tenants concurrently | 500 XP |
| 2 | Commit 100 GB of Eve storage | 500 XP |
| 3 | Reach trust tier **Trusted** (score ≥ 0.70) | 500 XP |
| 4 | Serve 10,000,000 tokens total | 1,000 XP |
| 5 | Receive 10 peer endorsements from Contributor+ nodes | 750 XP |

### Rewards
- `FiduciaryPledgeVC` (+0.30 trust weight)
- Purple Guardian badge with shield
- Enterprise workload eligibility
- Governance participation (proposal + vote)
- Access to: **Builder API** (extended node configuration, custom intent routing rules)
- Leaderboard appearance on map.kwaai.ai (top 50 nodes by trust score)

### Boss Moment — the gate to Level 5
**Reach 100,000 XP, maintain Trusted tier for 180 days, and make a meaningful contribution to the platform (code PR merged, or demonstrated leadership in the community).**

---

## Level 5 — Architect

> *"You built this place."*

### Entry
- 100,000 XP
- Trusted tier sustained for 180 days
- Meaningful platform contribution (merged PR, published integration, community leadership role)

### Experience
- Top of the network. Leaderboard position visible globally.
- **Governance:** Full proposal and veto weight in Kwaai Foundation decisions.
- **Builder API:** Can register custom intent types, create named inference circuits available to the whole network, and publish node specializations.
- **Architect VC** issued by the Kwaai Foundation — the rarest credential in the system.
- Node appears on the map with a **gold Architect badge** and a special annotation.
- Can issue `PeerEndorsementVC` to other nodes — these carry higher weight than endorsements from lower levels.

### XP Sources
All Level 4 sources, plus:
| Action | XP rate |
|--------|---------|
| Governance proposal passed | 1,000 XP |
| PR merged to core repo | 500 XP |
| Integration published and adopted (≥ 5 nodes using it) | 1,000 XP |
| Summit speaker | 500 XP |

### No boss moment
Architects have arrived. The objective is to keep the network healthy and bring others up behind them.

---

## XP Reference Table

### One-time XP sources
| Event | XP | VC issued |
|-------|----|-----------|
| PassKey registration | 50 | `MemberVC` |
| Node first appears on map | 100 | — |
| First `UptimeVC` (24h) | 50 | `UptimeVC` (provisional) |
| `UptimeVC` permanent (30 days) | 200 | `UptimeVC` (permanent) |
| `ThroughputVC` earned | 200 | `ThroughputVC` |
| `VerifiedNodeVC` onboarding | 300 | `VerifiedNodeVC` |
| GliaNet Fiduciary Pledge signed | 500 | `FiduciaryPledgeVC` |
| Summit attendance | 100 | `SummitAttendeeVC` |
| Architect designation | — | `ArchitectVC` |

### Ongoing XP sources
| Action | Rate |
|--------|------|
| Node online (base) | 1 XP / hour |
| Node online (Guardian+, inference + storage) | 2 XP / hour |
| Inference tokens served (block shard) | 1 XP / 20 tokens |
| Inference tokens served (full model, Citizen) | 1 XP / 50 tokens |
| Eve tenant active | 10 XP / tenant / day |
| Eve storage committed | 5 XP / GB · day |
| Peer endorsement received (Contributor+) | 50–75 XP |
| Invitee reaches Level 1 | 50 XP |
| Invitee reaches Level 2 | 100 XP |
| Invitee reaches Level 3 | 250 XP |

### Streak bonuses
| Streak | Bonus |
|--------|-------|
| 24h continuous uptime | 25 XP |
| 7-day streak (≥ 20h/day) | 100 XP |
| 30-day streak | 500 XP |
| 90-day streak | 2,000 XP |
| 180-day streak | 5,000 XP |

---

## Trust Score — the in-game stat

XP is the progression metric. The **trust score** (0.0 – 1.0) is the in-game stat peers see and rely on. They are related but not the same: XP accumulates monotonically; trust score can decay if uptime falls or credentials age out.

```
trust_score = α × score_vc + (1 − α) × score_metrics

score_vc = min(1.0, Σ weight(vc_type) × 0.5^(age_days / 365))
```

### VC weights (contribution to trust score)
| VC | Weight | Issued by |
|----|--------|-----------|
| `FiduciaryPledgeVC` | +0.30 | GliaNet Foundation |
| `VerifiedNodeVC` | +0.25 | Kwaai Foundation |
| `ThroughputVC` | +0.20 | Peer-witnessed, co-signed |
| `UptimeVC` | +0.10 | Bootstrap servers |
| `SummitAttendeeVC` | +0.10 | Kwaai Foundation |
| `PeerEndorsementVC` | +0.05 each (cap 0.20) | Any Contributor+ node |
| `MemberVC` | +0.05 | Kwaai platform |

### Trust tiers (used in routing)
| Score | Tier | Routing eligibility |
|-------|------|---------------------|
| ≥ 0.70 | **Trusted** | Enterprise, HIPAA/GDPR, Eve storage |
| ≥ 0.40 | **Verified** | Priority routing, circuits |
| ≥ 0.10 | **Known** | General inference |
| < 0.10 | **Unknown** | Basic consumer only |

---

## Daily Quests

Short-term engagement loops that reset every 24 hours:

| Quest | Condition | XP |
|-------|-----------|-----|
| **Morning shift** | Node online for first 4 hours of day | 10 XP |
| **Serve the chain** | Participate in ≥ 5 inference chains today | 15 XP |
| **Know thy network** | Check map.kwaai.ai and note one new node | 5 XP |
| **Pass it on** | Endorse a peer you've transacted with today | 20 XP |
| **Knowledge keeper** | Eve: serve ≥ 1 search query from a tenant | 15 XP |

---

## Weekly Challenges

| Challenge | Condition | XP |
|-----------|-----------|-----|
| **Backbone** | Achieve ≥ 150h uptime this week (out of 168) | 200 XP |
| **Throughput champion** | Serve more tokens than your 4-week rolling average | 150 XP |
| **Recruiter** | Bring one new player to Level 1 this week | 100 XP |
| **Knowledge node** | Eve: onboard ≥ 1 new tenant this week | 100 XP |

---

## The Invite Graph

Every player has an **invite code**. When someone uses your code and reaches Level 1, you earn XP. When they advance further, you earn again. This creates a visible lineage on the player profile:

```
You (Guardian, 28,000 XP)
├── Alice (Contributor, 7,200 XP)   +250 XP earned from Alice's Level 3
│   └── Dave (Citizen, 800 XP)     +100 XP earned from Dave's Level 2
└── Bob (Member, 120 XP)            +50 XP earned from Bob's Level 1
```

The invite graph is public and visible on map.kwaai.ai. Architects with large, deep invite trees have visible network impact — the leaderboard shows both personal XP and "network XP" (sum of your subtree).

---

## Level Summary Card

| | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|--|---------|---------|---------|---------|---------|---------|
| **Name** | Visitor | Member | Citizen | Contributor | Guardian | Architect |
| **XP gate** | 0 | 50 | 500 | 5,000 | 25,000 | 100,000 |
| **Map badge** | — | Grey | Blue | Green | Purple | Gold |
| **Boss moment** | — | PassKey | Node on map | 30d + 100K tokens | Eve + Pledge | 180d Trusted + contribution |
| **Key VC** | — | `MemberVC` | `UptimeVC` | `ThroughputVC` + `VerifiedNodeVC` | `FiduciaryPledgeVC` | `ArchitectVC` |
| **Trust tier** | — | Unknown | Known | Verified | Trusted | Trusted |
| **Inference** | Browser | Browser | Consumer | Block server | Block server | Block server |
| **Storage** | — | — | Bob (private KB) | Bob | Bob + Eve | Bob + Eve |
| **Governance** | — | — | — | — | 1 vote / 1K XP | Full weight |

---

## Open Questions for v2

1. **XP decay** — Should XP decay if a node goes offline for extended periods, or should it be a permanent ledger with a separate "active score"?
2. **PHE milestone** — When the PHE encryption layer ships (Phase 2), Eve operators who upgrade should receive a one-time `PHEUpgradeVC` and XP bonus for being early.
3. **Level 0 inference quality** — Should anonymous browser-only inference be throttled (e.g. shorter context, slower priority) to create genuine pull toward Level 1?
4. **Paid tier** — Is there a "fast lane" where organisations pay to get a `VerifiedNodeVC` without the 30-day uptime requirement? If so, does that VC carry lower trust weight?
5. **Sybil resistance** — A player could create many low-effort nodes to farm uptime XP. Mitigation: `VerifiedNodeVC` requires human onboarding; `FiduciaryPledgeVC` requires a legal entity. XP from unverified nodes is capped at Citizen level rewards.
6. **Mobile** — Level 0 works on mobile today (browser). A lightweight mobile node (background inference contributor) could sit between Level 0 and Level 1 — worth a half-level?
