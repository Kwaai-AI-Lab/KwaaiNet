# GitHub Discussion draft — libp2p binding

**Where:** `trustoverip/dtgwg-trust-tasks-tf` → Discussions → **Ideas**
**Status:** **ON HOLD — do not post yet.** See "Erika Bjune" below.

**Why this exists:** Glenn Gore's feedback on the 2026-08-19 call — *"it seems to
me that this should be a Discussion on the TF GH so that the decision is
documented and the PR can be linked against it."* He is right, and he authored
the HTTPS binding, so this follows his process rather than ours. He also asked a
clarifying question — *"You are talking about a Trust Task Binding for libp2p?"* —
which the opening line answers directly.

Keep it short. It is a Discussion opener asking one decision, not the proposal
document.

---

## Title

`Proposal: a libp2p transport binding`

## Body

Yes — a **Trust Task Binding** for libp2p, in the sense of
[`bindings/`](https://github.com/trustoverip/dtgwg-trust-tasks-tf/tree/main/bindings)
and https://trusttasks.org/bindings, alongside `https`, `didcomm`, `push` and
`tsp`. Raising it here per Glenn's suggestion on the 19 August call, so the
decision is documented and a PR can be linked against it.

### What

A binding specification at `bindings/libp2p/0.1/spec.md` plus a
`trust-tasks-libp2p` crate, following the shape of the HTTPS binding — a client
and a listener over a libp2p `Swarm`, mirroring `HttpsClient` / `HttpsServer`.

### Why libp2p rather than another transport

Two gaps in the current set, both specific:

**Transport-derived identity is cryptographic.** [SPEC §4.8.1](https://github.com/trustoverip/dtgwg-trust-tasks-tf/blob/main/SPEC.md#481-precedence-of-in-band-over-transport-derived-identity)
governs precedence of in-band over transport-derived identity. The HTTPS binding
satisfies it with a bearer token mapped to a VID, where the strength of the
mapping is a deployment concern. libp2p authenticates every connection with a
Noise handshake over the peer's long-term key — the peer ID *is* the public key —
so transport-derived identity is established before the first byte of a Trust
Task document is read. That may make it a useful second data point for §4.8.1:
a binding where the precedence rule is doing real work rather than deferring to
in-band identity by default.

**Neither party needs a public endpoint.** HTTPS requires a reachable server, and
DIDComm in practice usually needs a mediator. libp2p reaches peers behind NAT via
circuit relay and upgrades to direct connections with DCUtR. For a decentralised
trust graph where all parties hold their own subgraph, two agents on personal
devices exchanging Trust Tasks without either running infrastructure seems worth
supporting.

Secondary: a libp2p connection is full-duplex and multiplexed, so a consumer
behind NAT can receive Trust Tasks over the connection it opened, rather than
polling or relying on mediator callbacks.

### What we are asking

Whether a libp2p binding is **in scope for the task force**. If it is, we would
also like to settle a reserved slug and binding URI (`libp2p` /
`https://trusttasks.org/binding/libp2p/0.1`), and which framework version a new
binding should target — the HTTPS binding targets `0.2`.

### Open questions we would rather the task force decide

- **Which DID method for the peer ID → VID mapping?** Both `did:key` and
  `did:peer` derive from the same Ed25519 key. Key rotation and multi-key peers
  need a stated rule either way, and we would rather the binding specify it than
  have each implementation guess.
- **Relayed connections.** A circuit relay sees traffic shape and endpoints,
  though not content. Whether that warrants anything in the binding spec is a
  question for the group.

### Who is proposing

Kwaai AI Lab. We run [KwaaiNet](https://github.com/Kwaai-AI-Lab/KwaaiNet), a
decentralised AI fabric on rust-libp2p — Kademlia DHT, circuit relay, AutoNAT,
DCUtR, Noise, yamux — across macOS, Linux and Windows nodes behind residential
NAT. Four Kwaai people already participate in DTGWG, and we would want this
maintained in-tree and published alongside the other bindings rather than carried
out-of-tree, so upstream changes keep it working.

Longer write-up:
[`TrustTasksLibp2pBinding-proposal.md`](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/projects/kwaai-trust/proposals/TrustTasksLibp2pBinding-proposal.md)

Happy to be told this belongs somewhere else in the process.

---

## Blocking: talk to Erika Bjune first

On the same call (2026-08-19, 08:41), Erika Bjune: *"Hi Reza, we really need to
talk. Ours is a libp2p stack and we're working on these issues as well."*

**Do not post this as a Kwaai proposal until that conversation happens.** Posting
hours after someone says they are already working on the problem reads as
claiming the `libp2p` slug ahead of them, which is a poor first move in a group
we want a long relationship with.

It is also a worse proposal alone than it would be together. A task force
deciding whether to reserve a binding identifier wants evidence of more than one
implementation. Two independent libp2p stacks asking for the same binding turns
"Kwaai wants this" into "the libp2p constituency needs this" — a much easier yes.

Three ways this could go, in rough order of preference:

1. **Joint Discussion**, co-authored. Strongest signal, and it makes the slug
   reservation obviously worth doing.
2. **We post, naming her effort** and inviting her in. Faster, still collegial,
   but it puts Kwaai's name on the identifier first.
3. **She posts, we support.** Fine if her work is further along than ours — the
   binding existing matters more to us than whose name is on it, given we are not
   scheduled to build it until Q1 2027 anyway.

Worth establishing early: whether her stack is rust-libp2p or go-libp2p. If Go,
the binding needs to specify the wire contract carefully enough that both
implementations interoperate — which is a better specification either way, and is
exactly the kind of thing that justifies a binding spec rather than one crate.

---

## Notes for Reza before posting

- **Link only works once #113 is merged to `main`.** Until then it 404s — merge
  first, or drop the link.
- **Do not link the Google Doc.** The share URL carries your `ouid`, and this
  Discussion is public.
- The in-tree/publication ask is softened here to one clause at the end. In the
  full proposal it is ask 4 with the reasoning spelled out. That seemed right for
  an opener — it is the commercially meaningful ask, and leading with it in a
  first Discussion post would read badly.
- Glenn is at Affinidi, who maintain `affinidi-tsp` behind the `trust-tasks-tsp`
  binding. He wrote the HTTPS binding spec. Worth @-mentioning him as the person
  who suggested the Discussion.
