<!--
REVISION 4, DRAFT, NOT POSTED (2026-10-14). Adds the decision that KwaaiNet becomes a
Verifiable Trust Community, peer endorsement via vtc/relationships/request as the first task,
diagrams 5 and 6, a correction of the framework version, and three new questions.
REVISION 3 was POSTED to dtgwg-trust-tasks-tf#248 on 2026-10-14 (16:53 UTC) from commit
8a7792ce, without this HTML header:
https://github.com/trustoverip/dtgwg-trust-tasks-tf/discussions/248#discussioncomment-18929785
(`git show 8a7792ce:projects/kwaai-trust/proposals/TrustTasksLibp2pBinding-proposal.md`).
Revision 2 was posted as a reply on 2026-08-19 (17:02 UTC); its text is this file at
commit aec4a589.
These HTML comments do not render on GitHub — paste the whole file.
Paragraphs are deliberately unwrapped: GitHub joins soft-wrapped lines, and long
lines survive copy-paste into the comment box unchanged.
Images are absolute raw.githubusercontent.com URLs pinned to a commit SHA, so they render
in the discussion as well as in the repo. If the diagrams change, re-render the PNGs and
update the SHA (see projects/kwaai-trust/design/DTGWG-TrustStack-diagrams.md).
-->

Thanks @stormer78 for the go-ahead on our call. This revision does four things: it shows how we would build the binding, in six diagrams; it corrects three statements we made earlier, one of them about KwaaiNet itself; it says how KwaaiNet will use Trust Tasks, as a Verifiable Trust Community; and it asks you to confirm a few points here in writing, so the decision is on the record where you asked for it to be.

## What changed since revision 3

- **KwaaiNet will become a Verifiable Trust Community.** We mapped the 634 Trust Task specifications in the registry onto the node-to-node exchanges KwaaiNet has or plans. 34 fit directly between two nodes, and about 155 more apply once the network itself is a community. That settles it: the community model is where the registry's identity tasks live, so that is how we will use them.
- **Our first real task is one you already have.** Peer endorsement uses `vtc/relationships/request` 0.2 unchanged: a node asks a peer for a Verifiable Relationship Credential and receives it in the response. The prototype therefore needs no new task specification, only the binding. Diagram 5 now shows it, and the new diagram 6 shows the community.
- **A correction: the framework version.** We wrote that the binding would target framework 0.7.0. That is a Working Draft with no published envelope schema, and your registry checks require a published release. We will target a published one; see the questions at the end.
- **Three new questions,** listed under "Still open".

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

## How it would work, in six diagrams

The diagrams are coloured by owner. **Green** is the crate we would contribute to your workspace, `trust-tasks-libp2p`. **Grey** is Trust Tasks core and existing crates (`trust-tasks-rs`, `dtg-credentials`, the Affinidi stack). **Blue** is the node application — KwaaiNet in these diagrams, but any libp2p stack plays the same part. Names marked *(proposed)* do not exist yet. The Mermaid source is [here](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/DTGWG-TrustStack-diagrams.md).

### 1. Data flow: one Trust Task between two peers

![Data flow of one Trust Task between two peers](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/1-data-flow.png)

The sending application hands a Trust Task to the crate. The crate sets the issuer to the `did:key` derived from the node's own PeerId, attaches any credential, wraps the document in a small protobuf frame (compressed when worthwhile) and writes it to a libp2p stream on the binding's protocol ID. The stream crosses the network directly or through a circuit relay; either way it is end-to-end Noise. On the receiving side the application passes three things to the crate: the stream, the Noise-authenticated PeerId, and whether the path was relayed. The crate rejects oversize frames (and oversize documents after decompression), cross-checks the document's issuer against the transport identity per §4.8.1, and hands the document to `consume_inbound` for replay and freshness checks before the application sees it. A mismatched issuer gets an `identity_mismatch` error back on the same stream.

What matters for maintenance is the boundary: **the application's networking code only ever touches a stream, a PeerId and a direct/relayed flag.** It is an in-process interface, function calls inside the application, not a separate service. Everything that is Trust Tasks transport lives in the crate or in core, so when the framework changes the binding, the crate changes and the applications' networking code does not. One honest caveat: an application's task handlers still read and build Trust Task documents using the framework's types, so a breaking change to those types does reach them. That is inherent to speaking the protocol, and it is the same for every binding; what the crate removes is everything below the handlers.

### 2. Entities: identity, tasks and credentials

![Entity relationships](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/2-entity-relationships.png)

Each node has exactly one VID, a `did:key` derived from its PeerId, and optionally a `did:peer:2` routing DID; dialable addresses otherwise travel in libp2p signed peer records. A Trust Task names an issuer and a recipient VID, belongs to a thread that carries its lifecycle state, and travels in a binding frame that carries the document's JSON bytes and a compression flag. Credentials follow the DTG credential specification: an issuer and a subject VID, the required `issuerScope`, an `eddsa-jcs-2022` Data Integrity proof, and for statement credentials a predicate the verifier accepts. A credential can be bound to the Trust Task that produced it through `taskContext`. A node may be a member of a community, which issues membership and statement credentials, publishes members' relationship credentials as its trust graph, and has its keys held by a Verifiable Trust Agent (dashed: still open for us).

### 3. A request, its response, and a rejected sender

![Request, response and identity mismatch](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/3-sequence-request-response.png)

The coloured bands show the same ownership split. Once the stream is open, the crate checks the frame size and resolves the parties. If the in-band issuer matches the transport identity, or was omitted and is filled in from it, core checks replay and freshness and the application answers on the same stream. If it does not match, the crate's `reject()` returns `identity_mismatch` and the application never sees the task. Closing, resetting or dropping a stream ends the exchange but changes no task state.

### 4. Work that outlives its stream

![Late response on a fresh stream](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/4-sequence-late-response.png)

A task can outlive the connection that started it. The consumer can report `executing` and the first stream can close. When the result is ready the consumer opens a new stream, and the producer matches it to the task by `id` and `threadId`. libp2p has no correlation identifiers of its own, so the binding specification would say this explicitly, as §9.1 requires. Either side can open a stream later, for example to send a `trust-task-control` suspend or cancel.

### 5. Peer endorsement: the first task, using `vtc/relationships/request`

![Peer endorsement via vtc/relationships/request](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/5-sequence-peer-endorsement.png)

A node asks a peer to endorse it with `vtc/relationships/request` 0.2, unchanged, over the binding. The issuing node runs the inbound pipeline and decides by its own policy. If it agrees, it builds a Verifiable Relationship Credential whose issuer is its own `did:key` and whose subject is the requester's, signs it with `dtg-credentials` (JCS canonicalisation, `eddsa-jcs-2022`), and returns it in the response on the same stream. The requester verifies the proof and that issuer and subject match the task's parties, and stores the credential only if both hold. Publishing it to the community with `vtc/relationships/publish` is a separate, optional step. A decline is a `trust-task-error` with `vtc/relationships/request:declined`.

### 6. KwaaiNet as a Verifiable Trust Community

![KwaaiNet as a Verifiable Trust Community](https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/892ffb91c15e3084ae4dc1ff8ae09be03e850ba3/projects/kwaai-trust/design/img/dtgwg-trust-stack/6-kwaainet-vtc.png)

KwaaiNet's nodes become the community's members. Tasks between two nodes, such as endorsement and credential presentation (`credential-exchange/query`, then `present`), travel over the libp2p binding. Tasks with the community travel to the community's service: joining (`vtc/join-requests/submit`), publishing relationship credentials, receiving membership and node credentials (`vtc/endorsements/issue`), and recognition checks before a node is trusted with work (`registry/recognition`). An earlier, central credential issuer of ours is retired in favour of it. The dashed box is our open question: in OpenVTC's implementation every community sits on a Verifiable Trust Agent, and we have not yet decided how we will provide one.

## Accepting the guidance

| Guidance | Our response |
| --- | --- |
| Shim producing the payload, trait overlay, pluggable libp2p behind it | Agreed in substance, with one refinement for your view. Rather than libp2p behind a trait, **one complete `trust-tasks-libp2p` crate in your workspace that depends on no libp2p networking stack** — only PeerId types and any async byte stream. Any libp2p implementation hands it a `(PeerId, stream)` pair, so nobody has to implement a trait and no stack's libp2p version is forced on another. An optional feature can provide a ready-made endpoint for stacks without their own stream layer |
| Target the latest framework, not `0.2` | Agreed — the latest **published** release. We first wrote 0.7.0, but that is a Working Draft with no published envelope schema, so it would fail your registry checks. 0.5.0, as the other bindings declare, or 0.6.0: which would you prefer? |
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

## Wire format: binary framing, JSON document — please weigh in

The document stays exactly what §4.1 says it is: a UTF-8 JSON object, so proofs, task digests and `idConflict` behave as specified and a bridge can pass documents to TSP or DIDComm untouched. What we propose to make binary is only the framing around it, following libp2p's own convention (identify, kad and relay all send unsigned-varint length-prefixed protobuf):

```proto
syntax = "proto3";

message TrustTaskFrame {
  bytes       document    = 1;  // the Trust Task document: UTF-8 JSON (SPEC §4.1), encoded per `compression`
  Compression compression = 2;  // how `document` is encoded on the wire
}

enum Compression {
  COMPRESSION_NONE    = 0;  // the document bytes exactly as the producer serialised them
  COMPRESSION_DEFLATE = 1;  // raw DEFLATE (RFC 1951) of those bytes
}
```

Three choices in that, each open to your view:

- **No envelope type.** On TSP and DIDComm the `{type, document}` envelope exists because the payload has no content type. On libp2p, the protocol ID negotiated for the stream already identifies the content, so the frame carries the document directly. Across 256 example documents from the registry (median 497 B), the JSON envelope would have been 15% of the bytes.
- **A compression flag.** Receivers must support `NONE` and `DEFLATE`, so nothing needs negotiating; the sender decides per frame. DEFLATE is the baseline because Go, Rust and browsers all ship it; zstd could be added later. Compression saves about 38% on those examples, so we would use it above a size threshold rather than by default, and the binding would advise against compressing documents that mix secrets with attacker-influenced content, since compressed length can leak inside an encrypted channel.
- **Limits.** 1 MiB for the frame and for the document after decompression, enforced while decompressing.

The document is never transcoded: `payload` and `ext` are open JSON, and a protobuf representation of them (`google.protobuf.Struct`) turns every number into a double, which would break JCS-based signatures.

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
6. The wire format above: a protobuf frame with no envelope type, the JSON document carried as bytes, and a `NONE`/`DEFLATE` compression flag.

## Still open

- **Which framework release to target:** 0.5.0, like the other bindings, or 0.6.0?
- **Identifiers when a node joins a community.** `vtc/join-requests/submit` declares the applicant's identifier as pairwise, but the binding's VID, a `did:key` derived from the PeerId, is public by construction. Should a node join with a separate pairwise key bound to its PeerId by a credential, or is a public identifier acceptable here with the justification §13.1 asks for?
- **Does a community need its own Verifiable Trust Agent?** OpenVTC places every community on a VTA that holds its keys. Is that a requirement of the community model, or a choice of that implementation? Our nodes each hold one key and would not run a VTA themselves; we would welcome your view on whether that is sound.
- **Whether another group should lead.** We are not the only libp2p stack here. If someone else is further along, we would rather support their specification than advance our own — the binding existing matters more to us than whose name is on it.

## Timing

We will start now with a prototype: the crate built in a fork of your workspace, exercised between our own nodes, directly and through a relay. If that holds up, we would open the pull request in November or December 2026, and integrate it into KwaaiNet in Q1 2027. The bridge design follows the binding.
