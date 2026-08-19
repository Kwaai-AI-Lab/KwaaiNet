# DTGWG response and our reply — libp2p binding

**Source:** Glenn Gore (Affinidi), 2026-08-19, comments on the shared proposal.
**Discussion:** https://github.com/trustoverip/dtgwg-trust-tasks-tf/discussions/248

All four asks answered affirmatively — **and every technical argument we made for
libp2p was rebutted.** The process outcome is good; the case we built for it was
not. Both are recorded here, because Glenn's original point was that decisions
should be documented and linkable from the PR.

## The four asks — all answered

| Ask | Answer |
|-----|--------|
| 1. In scope? | **Yes.** *"Agreed that libp2p binding would be in scope of Trust Tasks and be a good addition."* |
| 2. Reserved slug and URI? | **Yes**, with a naming question — see below. *"I am happy to reserve a slug and URI for libp2p."* |
| 3. Target framework version? | **Latest, not `0.2`.** *"It should target the latest framework, where possible, given that this is pre-1.0, and the spec is moving fast and has breaking changes."* |
| 4. In-tree maintenance? | **Yes, and stronger than we asked.** *"The maintainers of trust-tasks would take on the responsibility of ensuring the binding is kept in sync with the features of Trust Tasks."* |

Ask 4 is the significant one. We asked to be maintained in lockstep; the answer
is that *they* carry the sync burden. That removes the pre-1.0 churn risk which
was the main reservation in our own planning.

## Correction: our technical case was wrong on all three points

Glenn rebutted every differentiator in the proposal. Recording this plainly,
because the shared document still carries the incorrect claims.

**"None derives sender identity from a cryptographic transport handshake."**
> *"This is technically not true. You can swap the delivery implementation."* (9:29)

**"Neither party needs a public endpoint."**
> *"I would state this differently. With DIDComm and TSP you also do not need to
> use a mediator. They are just messages, you can send them peer-to-peer if you
> want. We use mediators/relays though where they can be neutrally hosted for
> public good and you can nest (hide) behind multi relay hopping so no one knows
> where you are and as each client connects out to a mediator, it also operates
> behind NAT etc — you can punch holes through networks."* (9:23)

Worse for us than merely being wrong: multi-relay hopping gives *sender privacy*
that plain libp2p circuit relay does not.

**"Full-duplex removes the polling or mediator-callback patterns."**
> *"this is how the mediator works — there is no polling… It is bi-directional
> websockets with event based delivery."* (9:25)

### Where the error came from

I read the HTTPS binding specification closely and then generalised its
properties — bearer-token identity, a reachable server, request/response — to
DIDComm and TSP, which I did not read. Those are messaging protocols with their
own identity and routing models, and the generalisation does not hold. The
lesson is narrow and worth keeping: **do not characterise a peer technology from
a sibling's specification.**

## The constructive redirect: a bridge, not another transport

> *"a real innovation here would be a bridge between libp2p and TSP/DIDComm where
> you could mix the protocols together. We do this already with TSP+DIDComm where
> you can use TSP for routing, and the final delivery is via DIDComm for example.
> So you could use TSP for routing, carrying a libp2p payload or vice-versa."* (9:24)

This is the part worth pursuing. It is a genuinely different proposition from
"another transport binding": composing protocols so each does what it is best at
— TSP routing with libp2p delivery, or libp2p routing carrying a TSP payload.

It also plays to what KwaaiNet actually has. Our contribution is not a novel
transport; it is a production libp2p fabric that could be one leg of such a
bridge. And it fits the shim architecture below rather than fighting it.

**We should lead the revised proposal with the bridge**, and treat the plain
binding as the necessary substrate beneath it rather than as the headline.

## The architecture guidance — a shim, not a full binding

> "There is an option here to have the libp2p binding be a shim that creates a
> payload, with a trait overlay that allows a pluggable libp2p behind the shim.
> This would allow fast-moving changes to the shim when the Trust Task spec adds
> an attribute, while keeping control of the heavy lifting protocol elsewhere (as
> the TSP and DIDComm bindings do)."

This is how `trust-tasks-tsp` already works — it wraps `affinidi-tsp` rather than
implementing TSP. **We should adopt it**, for three reasons:

1. **It shrinks our deliverable.** `trust-tasks-libp2p` becomes a thin shim, not
   a transport implementation. Our rust-libp2p sits behind the trait.
2. **It decouples release cadences.** Spec changes hit the shim, which they
   maintain. Our stack moves on its own schedule.
3. **It matches the existing pattern**, so it needs no special pleading.

**This supersedes the deliverable in the proposal**, which described a client and
listener over a libp2p `Swarm` mirroring `HttpsClient`/`HttpsServer`. That was the
HTTPS model; the TSP model is the right one here.

## Open questions — resolved

**§4.8.1 stays as written.** *"It should not change 4.8.1, if addressing in libp2p
can be converted to a DID/VID then you will get greater interoperability (i.e.
libp2p connection within a VTC could still talk to a TSP connected peer in the
same VTC)."* Better than we hoped: convert libp2p addressing to a DID/VID and you
get cross-binding interoperability inside a VTC, not merely a working libp2p path.

**Relay privacy needs nothing special.** *"Nothing special apart from explaining
how libp2p works… that is not the responsibility of Trust Tasks to help someone
pick libp2p or TSP or DIDComm or REST."* Document the properties, do not
editorialise about the choice.

**DID method: `did:peer` recommended — which is what we already use.** *"Trust
Tasks does not care… if you are looking for an ephemeral self-supporting DID then
I would strongly suggest `did:peer` as it allows you more flexibility to put some
routing information in the DID vs. `did:key` which would be very limiting from a
routing perspective."*

Three consequences for us:

- KwaaiNet already issues `did:peer` (`kwaai-trust/src/did.rs`). No change needed,
  and the earlier draft's `did:key` assumption was wrong in exactly the direction
  Glenn warns against.
- **Routing information in the DID is the interesting part.** `did:peer` can carry
  service endpoints, so a peer's multiaddrs — including circuit-relay addresses —
  can travel in the DID itself. That is a real fit for libp2p rather than a
  compromise.
- It settles the `did:key` vs `did:peer` verification mismatch recorded in
  `Ledger-plan.md`, in favour of what we already do.

## Still open: is `libp2p` the right slug?

> "Is 'libp2p' the right protocol name (aka DIDComm or TSP)?"

A fair challenge: DIDComm and TSP are messaging protocols with defined envelopes;
libp2p is a networking stack that carries many protocols.

**Our answer should be that `libp2p` is right, by exact analogy with `https`.**
The HTTPS binding is named for the transport and then specifies the particulars —
`POST` to `/trust-tasks`. A libp2p binding is named for the transport and
specifies the libp2p **protocol ID** that carries a Trust Task document, e.g.
`/trust-tasks/1.0.0`, negotiated by multistream-select on a libp2p stream.

So the parallel is `https` : `/trust-tasks` :: `libp2p` : `/trust-tasks/1.0.0`.
Worth stating plainly in the reply, and inviting correction — Glenn asked the
question, so he may have a reason to prefer otherwise.

## What we should do next

1. **Correct the shared document before anything else.** It still asserts three
   rebutted claims to a public audience. Rewrite "Why libp2p, specifically"
   around the bridge, and drop the differentiators rather than defending them.
2. **Reply in the Discussion**, and lead with the concession. Accept the
   corrections plainly, take the bridge idea, accept the shim architecture,
   confirm `did:peer`, and answer the slug question. Conceding accurately is
   worth more here than being right — it is the first substantive exchange we
   have had with this group.
3. **Fix the public comment.** Discussion #248's follow-up links the Google Doc
   including Reza's `ouid`. Replace it with the GitHub proposal link once #113
   merges.
4. **Ask Glenn to repost his comments in the thread**, or paste them with
   attribution. They exist only in a Google Doc, which defeats his own point.
5. **Nothing changes in the 2026 plan.** Still Q1 2027; rung 1 is unaffected.

## What is actually left of our case

Worth being clear-eyed, because the revised proposal has to stand on this:

- **Not** unique cryptographic transport identity. Delivery is swappable.
- **Not** unique NAT traversal or serverless operation. DIDComm and TSP do
  peer-to-peer, and mediators add sender privacy we do not have.
- **Not** unique full-duplex or event-driven delivery. Mediators are
  bi-directional websockets already.
- **Yes:** a bridge composing libp2p with TSP/DIDComm — Glenn's own suggestion,
  and he called it "a real innovation".
- **Yes:** a production libp2p fabric to be one leg of that bridge, with
  `did:peer` already in use and multiaddrs that can ride in the DID.
- **Yes:** a constituency. We are not the only libp2p stack in the group.

The honest framing for the revised proposal is that libp2p is a transport many
people already run, so meeting them where they are has value — not that it is
technically superior to what Trust Tasks already supports.
