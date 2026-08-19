# Proposal: a libp2p transport binding for Trust Tasks

**To:** LF ToIP Decentralized Trust Graph Working Group, Trust Tasks Task Force
**From:** Kwaai AI Lab
**Date:** 2026-08-19
**Status:** **shared with the DTGWG weekly call, 2026-08-19.** Awaiting task-force
response on the four asks below. Superseded by that response, not by this draft.

---

## Summary

Kwaai AI Lab proposes to contribute a **libp2p transport binding** for the Trust
Tasks framework — a binding specification under
`bindings/libp2p/<version>/spec.md` and a `trust-tasks-libp2p` crate alongside
the existing `-https`, `-didcomm` and `-tsp` implementations.

Trust Tasks currently ships bindings for HTTPS, DIDComm v1/v2, push and TSP.
None of them carries Trust Tasks between two parties where **neither has a
reachable public endpoint**, and none derives sender identity from a
cryptographic transport handshake. A libp2p binding addresses both.

We are asking the task force for agreement in principle, a reserved binding slug
and URI, and guidance on the target framework version — not for a decision on
merge today.

## Why libp2p, specifically

### 1. Transport-derived identity that is cryptographic, not configured

SPEC §4.8.1 defines the precedence of in-band over transport-derived identity,
and the HTTPS binding satisfies it with a bearer token mapped to a *Verifiable
Identifier*. That mapping is a deployment concern: its strength depends on how
the token was issued and how carefully the receiver maintains the mapping.

libp2p authenticates **every** connection with a Noise handshake over the peer's
long-term key. The peer ID *is* the public key. Transport-derived identity is
therefore established cryptographically before the first byte of a Trust Task
document is read, and the peer ID resolves to a *Verifiable Identifier* by
derivation from that same key rather than by a lookup table.

We think that makes libp2p a useful second data point for §4.8.1 — a binding
where transport-derived identity is strong enough that the precedence rule is
doing real work, rather than deferring to in-band identity by default.

### 2. Neither party needs a public endpoint

The HTTPS binding requires a reachable server; DIDComm in practice usually does
too, via a mediator. That excludes a class of deployment the DTGWG charter
speaks directly to: two agents, each on a personal device, each behind NAT, with
no server between them.

libp2p handles this natively — circuit relay for reachability plus DCUtR hole
punching to upgrade to a direct connection where the network allows it. A trust
graph in which "all parties control their own subgraph" is more credible when
two parties can exchange Trust Tasks without either of them running
infrastructure.

### 3. Full-duplex and multiplexed

A libp2p connection carries many concurrent streams in both directions. A
consumer behind NAT can *receive* Trust Tasks, not only send them, over the same
connection it opened. For long-lived agent-to-agent relationships this removes
the polling or mediator-callback patterns the request/response bindings imply.

## What Kwaai brings

- **A production libp2p deployment.** KwaaiNet is a decentralised AI fabric built
  on rust-libp2p 0.56 — Kademlia DHT, circuit relay, AutoNAT, DCUtR, Noise,
  yamux — running across macOS, Linux and Windows nodes in the field.
- **Operational experience under real network conditions.** Those nodes sit
  behind residential NAT, reach each other over circuit relay, and upgrade to
  direct connections via DCUtR. We maintain a tiered integration harness and
  measure per-call round-trip latency in the field, so we can characterise a
  binding's cost rather than assert it.
- **Four Kwaai volunteers already participate in DTGWG.** This contribution is
  intended as ongoing participation rather than a code drop.

## What we propose to deliver

| Deliverable | Shape |
| --- | --- |
| `bindings/libp2p/0.1/spec.md` | Binding specification following the HTTPS binding's structure — YAML front matter, document carriage, identity handling under §4.8.1, error mapping to the §8.3 vocabulary |
| `trust-tasks-libp2p` crate | Client and listener over a libp2p `Swarm`, mirroring the `HttpsClient` / `HttpsServer` shape |
| Binding URI | `https://trusttasks.org/binding/libp2p/0.1` (slug `libp2p`, subject to the task force's preference) |
| Interop evidence | Round-trip against an existing binding — the relevant comparison for a transport binding — plus measured latency over direct and relayed connections |

We would follow `CONTRIBUTING-SPECS.md`: fork, branch, folder, `npm run build`
to validate, PR for CODEOWNERS routing. DCO and CLA as required.

## What we are asking for

1. **Agreement in principle** that a libp2p binding is in scope for the task
   force, before we invest in the specification.
2. **A reserved slug and binding URI**, so the work targets a stable identifier.
3. **Target framework version.** The HTTPS binding targets framework `0.2`;
   we would like to know whether a new binding should target `0.2` or a later
   draft.
4. **Maintenance in-tree, published to crates.io.** The existing bindings are
   versioned in lockstep with `trust-tasks-rs` — `-https`, `-didcomm`, `-proof`
   and `-tsp` each have thirteen releases tracking the core's minor line. We are
   explicitly asking for the same treatment rather than maintaining a binding
   out-of-tree, and we should be transparent that this is as much to our benefit
   as to the framework's: it is the mechanism by which upstream refactors keep
   our binding working. We note `bindings/push` exists in-repo without a
   published crate, so publication is worth agreeing rather than assuming.

## Open questions we would raise, not resolve

- **Identity precedence.** Does a cryptographically-authenticated transport
  change how §4.8.1 should be applied, or is the existing precedence rule
  sufficient as written? We have a view but not a strong one, and it is properly
  the task force's call.
- **Relayed connections.** When a connection traverses a circuit relay, the relay
  sees traffic shape and endpoints though not content. Whether that warrants
  anything in the binding specification is worth discussing.
- **Peer ID to VID — which DID method?** Both `did:key` and `did:peer` are
  derivable from the same Ed25519 key, and the choice is not obvious. KwaaiNet
  uses `did:peer` today; the framework's other bindings lean toward `did:key`.
  Whichever the task force prefers, key rotation and multi-key peers deserve a
  stated rule rather than an implied one. We would rather the binding specify
  this than leave each implementation to guess.
- **Framework stability.** `trust-tasks-rs` has published 86 versions since May
  2026, moving 0.5 → 0.9 in eight days. We are comfortable tracking that, but the
  binding's own version cadence should be agreed rather than discovered.

## Timing

Kwaai's committed engineering scope for 2026 does not include this work; our
year-end release is platform hardening. We would expect to begin the
specification once the task force agrees in principle, with implementation
following the framework reaching a stable enough surface — realistically Q1
2027. We would rather say that plainly than commit to a date we would miss.

---

### Internal notes

*(These were present in the version shared on 2026-08-19. Retained rather than
quietly deleted, so the record matches what the working group actually saw.)*

- This was shared without prior review by the four Kwaai DTGWG volunteers.
- Licence checked: framework source is Apache-2.0 (`SOURCE_CODE.md`, and
  `license = "Apache-2.0"` in `Cargo.toml`); specifications are OWFa 1.0.
  Compatible with KwaaiNet's MIT, and contributing requires DCO plus CLA.
- The ask in point 4 (in-tree, published) is the commercially meaningful one for
  us. Worth deciding whether to lead with it or leave it where it is.
- No claim in this document should outrun what we can show. The interop harness
  and the production deployment are real; the Q1 2027 timing is a genuine
  estimate, not a hedge.
