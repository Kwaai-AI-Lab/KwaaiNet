<!--
POST AS: a reply comment in dtgwg-trust-tasks-tf#248
These HTML comments do not render on GitHub — paste the whole file.
Paragraphs are deliberately unwrapped: GitHub joins soft-wrapped lines, and long
lines survive copy-paste into the comment box unchanged.
-->

Thanks @stormer78 — the review corrected three things we had wrong and pointed at a better idea than the one we brought. Here is the revised version.

## What we got wrong

Our case rested on three claimed gaps. All three were mistaken:

- **"None of the existing bindings derives sender identity from a cryptographic transport handshake."** Not true — the delivery implementation is swappable.
- **"Neither party needs a public endpoint" as a libp2p advantage.** DIDComm and TSP are messages and can travel peer-to-peer. Mediators are a choice, not a requirement, and multi-relay hopping provides sender privacy that plain libp2p circuit relay does not.
- **"Full-duplex removes polling or mediator-callback patterns."** Mediators already use bi-directional websockets with event-based delivery.

The error was ours: we read the HTTPS binding specification closely and then generalised its properties to DIDComm and TSP, which we had not read. Those are messaging protocols with their own identity and routing models, and the generalisation does not hold.

**libp2p is not technically superior to what Trust Tasks already supports.** The honest case is narrower, and it is below.

## The revised proposal: start with the bridge

> a real innovation here would be a bridge between libp2p and TSP/DIDComm where you could mix the protocols together. We do this already with TSP+DIDComm where you can use TSP for routing, and the final delivery is via DIDComm for example. So you could use TSP for routing, carrying a libp2p payload or vice-versa.

This is a better proposition than another transport binding, and we would rather pursue it. Composing protocols so each does what it is best at — TSP routing with libp2p delivery, or libp2p routing carrying a TSP payload — is more interesting than adding a fifth way to move the same document.

It also matches what we can actually offer. Kwaai's contribution is not a novel transport; it is a production libp2p fabric that could serve as one leg of such a bridge, plus people to do the work.

## The honest case for a libp2p binding underneath it

Narrower than our first version, and we think it still holds:

**libp2p is a transport a lot of people already run.** Meeting an existing deployment where it is has value even when it offers no new capability — much the reason a REST binding would be worth having. We are aware of at least one other libp2p stack in this group, which suggests the constituency is real.

**Addressing maps cleanly onto `did:peer`.** A peer's multiaddrs, including circuit-relay addresses, can ride in the DID's service endpoints. That makes the libp2p-addressing-to-VID conversion concrete rather than aspirational, which matters for the cross-binding interoperability below.

That is the whole argument. A binding is worth having because people are already on libp2p, not because libp2p does something the others cannot.

## Accepting the guidance

| Guidance | Our response |
| --- | --- |
| Shim producing the payload, trait overlay, pluggable libp2p behind it | Agreed — and a better fit than the `HttpsClient`/`HttpsServer` shape we first described. Keeps spec churn in the shim and the transport out of the framework's way |
| Target the latest framework, not `0.2` | Agreed |
| §4.8.1 unchanged; convert libp2p addressing to a DID/VID | Agreed. A libp2p peer and a TSP peer in the same VTC being able to talk is a better outcome than a special case |
| Document relay properties rather than editorialise | Agreed — explain how libp2p behaves, let implementers choose |
| Maintainers keep the binding in sync with the framework | Thank you. That was our main reservation about tracking a pre-1.0 framework, and it resolves it |

## Answering the two questions

**"Is `libp2p` the right protocol name (aka DIDComm or TSP)?"**

We think so, by analogy with `https`. That binding is named for the transport and then specifies the particulars — `POST` to `/trust-tasks`. A libp2p binding would be named for the transport and specify the libp2p **protocol ID** carrying a Trust Task document — say `/trust-tasks/1.0.0` — negotiated by multistream-select on a libp2p stream:

| Binding | Names | Specifies |
| --- | --- | --- |
| `https` | the transport | `POST /trust-tasks` |
| `libp2p` | the transport | protocol ID `/trust-tasks/1.0.0` |

That said, you raised it, so you may have a reason to prefer otherwise. And if the bridge is the more interesting artefact, the naming question may want settling in that context instead.

**Which DID method?**

`did:peer`, on your recommendation and for your reason — routing information in the DID. KwaaiNet already issues `did:peer`, so this needs no change on our side.

## What we would contribute

- A production rust-libp2p fabric — Kademlia DHT, circuit relay, AutoNAT, DCUtR, Noise, yamux — running across macOS, Linux and Windows nodes behind residential NAT, with a tiered integration harness and measured per-call latency.
- Four Kwaai people already participating in DTGWG.
- Work on the shim, and on the bridge if the task force wants to pursue it.

## Still open

- **Whether to lead with the bridge or the binding.** The bridge is the more valuable artefact; the binding may be the necessary substrate. We have no strong view on sequencing and would follow the task force's.
- **Whether another group should lead.** We are not the only libp2p stack here. If someone else is further along, we would rather support their specification than advance our own — the binding existing matters more to us than whose name is on it.

## Timing

This is not in Kwaai's committed engineering scope for 2026; our year-end release is platform hardening. Realistically we would begin in Q1 2027. We would rather say that plainly than commit to a date we would miss — and it is another reason we are glad for someone else to lead if they are ready sooner.
