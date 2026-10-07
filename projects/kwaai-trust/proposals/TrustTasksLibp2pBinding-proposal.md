<!--
POST AS: a new reply comment in dtgwg-trust-tasks-tf#248 — REVISION 3, NOT YET POSTED.
Revision 2 was posted as a reply on 2026-08-19 (17:02 UTC); its text is this file at
commit aec4a589 (`git show aec4a589:projects/kwaai-trust/proposals/TrustTasksLibp2pBinding-proposal.md`).
These HTML comments do not render on GitHub — paste the whole file.
Paragraphs are deliberately unwrapped: GitHub joins soft-wrapped lines, and long
lines survive copy-paste into the comment box unchanged.
Images are absolute raw.githubusercontent.com URLs pinned to a commit SHA, so they render
in the discussion as well as in the repo. If the diagrams change, re-render the PNGs and
update the SHA (see projects/kwaai-trust/design/DTGWG-TrustStack-diagrams.md).
-->

Thanks @stormer78 for the go-ahead on our call. This revision does three things: it shows how we would build the binding, in five diagrams; it corrects two statements in our last reply, one of them about KwaaiNet itself; and it asks you to confirm a few points here in writing, so the decision is on the record where you asked for it to be.

## What we got wrong in the first version

Our case rested on three claimed gaps. All three were mistaken:

- **"None of the existing bindings derives sender identity from a cryptographic transport handshake."** Not true — the delivery implementation is swappable.
- **"Neither party needs a public endpoint" as a libp2p advantage.** DIDComm and TSP are messages and can travel peer-to-peer. Mediators are a choice, not a requirement, and multi-relay hopping provides sender privacy that plain libp2p circuit relay does not.
- **"Full-duplex removes polling or mediator-callback patterns."** Mediators already use bi-directional websockets with event-based delivery.

The error was ours: we read the HTTPS binding specification closely and then generalised its properties to DIDComm and TSP, which we had not read. Those are messaging protocols with their own identity and routing models, and the generalisation does not hold.

**libp2p is not technically superior to what Trust Tasks already supports.** The honest case is narrower, and it is below.

## Two corrections to our last reply

**1. "KwaaiNet already issues `did:peer`, so this needs no change on our side."** Wrong. Our identifier is the string `did:peer:` followed by the raw base58 PeerId, with no numalgo and no multibase/multicodec encoding. It is not a conformant `did:peer`. We will replace it, and we have revised which DID the binding uses (below).

**2. "Addressing maps cleanly onto `did:peer`" as the VID.** The routing half of that holds; the identity half does not. A VID must be compared by exact string (§4.8) and, if a binding lets `proof` be omitted, derived deterministically from the transport principal (§9.1.1). A `did:peer:2` that carries multiaddrs changes whenever a node's addresses change, so it cannot be that identifier.

## The revised proposal: start with the bridge

> a real innovation here would be a bridge between libp2p and TSP/DIDComm where you could mix the protocols together. We do this already with TSP+DIDComm where you can use TSP for routing, and the final delivery is via DIDComm for example. So you could use TSP for routing, carrying a libp2p payload or vice-versa.

This is a better proposition than another transport binding, and we would rather pursue it. Composing protocols so each does what it is best at — TSP routing with libp2p delivery, or libp2p routing carrying a TSP payload — is more interesting than adding a fifth way to move the same document. A libp2p binding is the substrate it needs, so we propose building that first and designing the bridge on top of it.

## The honest case for a libp2p binding underneath it

**libp2p is a transport a lot of people already run.** Meeting an existing deployment where it is has value even when it offers no new capability — much the reason a REST binding would be worth having. We are aware of at least one other libp2p stack in this group, which suggests the constituency is real.

**libp2p identity converts cleanly to a VID.** For the common key types (Ed25519, secp256k1, ECDSA) a PeerId is an encoding of the peer's public key, and every connection is authenticated against it by Noise. The binding can therefore derive the peer's VID as a `did:key` deterministically and verify it offline, and an in-band `did:key`, `did:peer:0` or `did:peer:2` naming the same key can be accepted as consistent. Routing information keeps your reason for `did:peer`: a node can publish an optional `did:peer:2` whose service endpoint carries its multiaddrs, which is where the bridge needs it.

## How it would work, in five diagrams

The diagrams are coloured by owner. **Green** is the crate we would contribute to your workspace, `trust-tasks-libp2p`. **Grey** is Trust Tasks core and existing crates (`trust-tasks-rs`, `dtg-credentials`, the Affinidi stack). **Blue** is the node application — KwaaiNet in these diagrams, but any libp2p stack plays the same part. Names marked *(proposed)* do not exist yet. The Mermaid source is [here](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/DTGWG-TrustStack-diagrams.md).

### 1. Data flow: one Trust Task between two peers

![Data flow of one Trust Task between two peers](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/img/dtgwg-trust-stack/1-data-flow.png)

The sending application hands a Trust Task to the crate. The crate sets the issuer to the `did:key` derived from the node's own PeerId, attaches any credential, frames the envelope and writes it to a libp2p stream on the binding's protocol ID. The stream crosses the network directly or through a circuit relay; either way it is end-to-end Noise. On the receiving side the application passes three things to the crate: the stream, the Noise-authenticated PeerId, and whether the path was relayed. The crate rejects oversize frames, cross-checks the document's issuer against the transport identity per §4.8.1, and hands the document to `consume_inbound` for replay and freshness checks before the application sees it. A mismatched issuer gets an `identity_mismatch` error back on the same stream.

What matters for maintenance is the boundary: **the application's networking code only ever touches a stream, a PeerId and a direct/relayed flag.** It is an in-process interface, function calls inside the application, not a separate service. Everything that is Trust Tasks transport lives in the crate or in core, so when the framework changes the binding, the crate changes and the applications' networking code does not. One honest caveat: an application's task handlers still read and build Trust Task documents using the framework's types, so a breaking change to those types does reach them. That is inherent to speaking the protocol, and it is the same for every binding; what the crate removes is everything below the handlers.

### 2. Entities: identity, tasks and credentials

![Entity relationships](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/img/dtgwg-trust-stack/2-entity-relationships.png)

Each node has exactly one VID, a `did:key` derived from its PeerId, and optionally a `did:peer:2` routing DID; dialable addresses otherwise travel in libp2p signed peer records. A Trust Task names an issuer and a recipient VID, belongs to a thread that carries its lifecycle state, and travels in a binding envelope. Credentials follow the DTG credential specification: an issuer and a subject VID, the required `issuerScope`, an `eddsa-jcs-2022` Data Integrity proof, and for statement credentials a predicate the verifier accepts. A credential can be bound to the Trust Task that produced it through `taskContext`.

### 3. A request, its response, and a rejected sender

![Request, response and identity mismatch](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/img/dtgwg-trust-stack/3-sequence-request-response.png)

The coloured bands show the same ownership split. Once the stream is open, the crate checks the frame size and resolves the parties. If the in-band issuer matches the transport identity, or was omitted and is filled in from it, core checks replay and freshness and the application answers on the same stream. If it does not match, the crate's `reject()` returns `identity_mismatch` and the application never sees the task. Closing, resetting or dropping a stream ends the exchange but changes no task state.

### 4. Work that outlives its stream

![Late response on a fresh stream](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/img/dtgwg-trust-stack/4-sequence-late-response.png)

A task can outlive the connection that started it. The consumer can report `executing` and the first stream can close. When the result is ready the consumer opens a new stream, and the producer matches it to the task by `id` and `threadId`. libp2p has no correlation identifiers of its own, so the binding specification would say this explicitly, as §9.1 requires. Either side can open a stream later, for example to send a `trust-task-control` suspend or cancel.

### 5. Issuing and verifying a credential

![Credential issued and verified](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/90e772d85a6e3521499f31fa7c16491e09e59e47/projects/kwaai-trust/design/img/dtgwg-trust-stack/5-sequence-credential-issue-verify.png)

The issuer builds a statement credential whose issuer and subject are both `did:key` VIDs and signs it with `dtg-credentials` (JCS canonicalisation, `eddsa-jcs-2022`). It travels to the subject as a Trust Task payload over the binding. The subject verifies the proof, the `issuerScope` and the predicate, and stores the credential only if all three pass.

## Accepting the guidance

| Guidance | Our response |
| --- | --- |
| Shim producing the payload, trait overlay, pluggable libp2p behind it | Agreed in substance, with one refinement for your view. Rather than libp2p behind a trait, **one complete `trust-tasks-libp2p` crate in your workspace that depends on no libp2p networking stack** — only PeerId types and any async byte stream. Any libp2p implementation hands it a `(PeerId, stream)` pair, so nobody has to implement a trait and no stack's libp2p version is forced on another. An optional feature can provide a ready-made endpoint for stacks without their own stream layer |
| Target the latest framework, not `0.2` | Agreed — framework 0.7.0 today |
| §4.8.1 unchanged; convert libp2p addressing to a DID/VID | Agreed. The PeerId-to-`did:key` mapping above is that conversion |
| Document relay properties rather than editorialise | Agreed. One property the specification will state plainly: relays running libp2p's default limits (128 KiB, 2 minutes per circuit) can cut a large document off |
| Maintainers keep the binding in sync with the framework | Thank you. We would also like to **co-maintain the crate** — named in CODEOWNERS, with Kwaai owning libp2p-side changes — so the work does not fall only on you |

## Answering the two questions

**"Is `libp2p` the right protocol name (aka DIDComm or TSP)?"**

We think so, by analogy with `https`. That binding is named for the transport and then specifies the particulars — `POST` to `/trust-tasks`. A libp2p binding would be named for the transport and specify the libp2p **protocol ID** carrying a Trust Task document, negotiated by multistream-select on a libp2p stream. We now propose `/trust-tasks/0.1`, so the protocol ID moves with the binding version (`https://trusttasks.org/binding/libp2p/0.1`) rather than claiming a `1.0.0` the binding has not reached:

| Binding | Names | Specifies |
| --- | --- | --- |
| `https` | the transport | `POST /trust-tasks` |
| `libp2p` | the transport | protocol ID `/trust-tasks/0.1` |

That said, you raised it, so you may have a reason to prefer otherwise.

**Which DID method?**

`did:key` for the VID, derived from the PeerId; `did:peer:2` as an optional routing DID, for the routing reason you gave. See the two corrections above for why we changed this.

## What we would contribute

- **`trust-tasks-libp2p`**, built in your workspace layout and CI from the start, released in your `core` group, with `bindings/libp2p/0.1/spec.md` and its registry entry.
- **Co-maintenance** of that crate, and test vectors so other libp2p stacks (Go, JS) can check their own implementations against it.
- A production rust-libp2p fabric to test it on — Kademlia DHT, circuit relay, AutoNAT, DCUtR, Noise, yamux — running across macOS, Linux and Windows nodes behind residential NAT.
- Four Kwaai people already participating in DTGWG, and work on the bridge design once the binding exists.

## Please confirm here in writing

Your approval on the call is what we are working from. Since this discussion is where the decision should be recorded, could you confirm, or correct, these points here?

1. A `libp2p` binding slug and URI `https://trusttasks.org/binding/libp2p/0.1`.
2. One complete `trust-tasks-libp2p` crate in your workspace over a `(PeerId, stream)` seam, with no dependency on a libp2p networking stack, released in the `core` group.
3. Kwaai as co-maintainer of that crate.
4. `did:key` derived from the PeerId as the VID, with `did:peer:2` optional for routing.
5. Protocol ID `/trust-tasks/0.1`.

## Still open

- **Whether another group should lead.** We are not the only libp2p stack here. If someone else is further along, we would rather support their specification than advance our own — the binding existing matters more to us than whose name is on it.

## Timing

We will start now with a prototype: the crate built in a fork of your workspace, exercised between our own nodes, directly and through a relay. If that holds up, we would open the pull request in November or December 2026, and integrate it into KwaaiNet in Q1 2027. The bridge design follows the binding.
