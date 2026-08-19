# DTGWG response and our reply — libp2p binding

**Source:** Glenn Gore (Affinidi), 2026-08-19, comments on the shared proposal.
**Discussion:** https://github.com/trustoverip/dtgwg-trust-tasks-tf/discussions/248

All four asks answered affirmatively, with architectural guidance that makes the
work smaller than proposed. Recorded here because Glenn's original point was that
decisions should be documented and linkable from the PR.

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

1. **Reply in the Discussion** — answer the slug question, accept the shim
   architecture, confirm `did:peer`, and thank him for taking on sync.
2. **Fix the public comment.** The follow-up comment on discussion #248 links the
   Google Doc including Reza's `ouid`. Replace it with the GitHub proposal link
   once #113 merges.
3. **Ask Glenn to repost these comments in the thread**, or paste them ourselves
   with attribution — they are currently only in a Google Doc, which defeats his
   own point about documenting the decision where the PR can link to it.
4. **Update the proposal** to reflect the shim architecture and the resolved
   questions, so the document and the decision do not drift.
5. **Nothing changes in the 2026 plan.** This remains Q1 2027 work; rung 1 is
   unaffected.
