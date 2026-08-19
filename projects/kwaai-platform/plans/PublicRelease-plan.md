# KwaaiNet Public Release — four blocks of functionality


## Plan of record

**Committed scope for 2026: Rung 1 — Secure Network.** The v0.6 cutover,
authorization, the ledger landed, and `kwaai-api`. Approved 2026-08-18.

Everything else on this page is upside, not commitment.

| Rung | Adds | Solo weeks | Volunteer weeks needed | Status |
|---|---|---|---|---|
| **1 · Secure Network** | Cutover, authorization, ledger, `kwaai-api` | 14 | **0** | **Committed** |
| 2 · Browser front door | Public, browser-reachable node | 19 | 2 | Likely — one engaged volunteer month |
| 3 · Feed Preview | Agent harness, email connector, ranking | 33 | 16 | Pursue, do not announce |

Rung 1 is the only scope that depends on nobody volunteering. Build so rungs 2
and 3 attach without rework, and let availability decide where the year ends.


**Status:** approved 2026-08-18. Sizes are t-shirt, not estimates.

## Context

KwaaiNet today is a developer-grade decentralised AI fabric: p2p transport, DHT,
distributed inference, RAG, storage, identity. It has no consumer-facing product, no
accounting for the compute nodes give each other, and no agent layer. This plan sizes
the four blocks needed to turn it into a public release.

The four blocks are **not peers**, and that is the single most important finding from
reading the requirements. The Unified Feed is the *integrating application* — it
consumes the agent harness (the Twin and the VA **are** agents), the ledger (CC is
money), and the UX (the Feed **is** the primary UI). The other three are its
substrate. Build order therefore differs from the numbering below, which is kept as
the user stated it.

Sources read: `Unified Feed w_Use Cases.pdf` (20pp, Apr 2025); on branch
`feat/kwaai-ledger`: `projects/kwaai-trust/plans/Ledger-plan.md` and
`projects/kwaai-trust/plans/TokenEconomy-plan.md`; and the LF ToIP [Decentralized
Trust Graph Working Group](https://lf-toip.atlassian.net/wiki/spaces/HOME/pages/257785857/Decentralized+Trust+Graph+Working+Group).

## What already exists — do not rebuild

| Asset | State | Consequence |
|---|---|---|
| **`core/crates/kwaai-ledger`** | **Built** on `feat/kwaai-ledger` — `lib.rs` 1244 / `store.rs` 578 / `economy.rs` 618 lines, plus `kwaai-cli/src/{ledger_cmd,ledger_node}.rs`. Already a workspace member. Branch is 16 ahead / 22 behind `main`. | Block 2 phase 1 is a **land-and-harden**, not a build |
| `Ledger-plan.md`, `TokenEconomy-plan.md` | Detailed, decisions locked | Reuse verbatim; do not re-litigate |
| `systems/node-dashboard/` | React frontend + Node.js `backend/server.js` | Stays; new crate is API-only |
| `apps/map/` | React + d3 force graph (map.kwaai.ai) | Unaffected |
| `KwaaiNetGUI` repo | Flutter desktop, split out in `1a2d34ea` | Darren continues there |
| `core/crates/kwaai-wasm` | Exists | The browser-first path |
| `kwaai-rag`, `kwaai-inference`, `kwaai-p2p`, `kwaai-trust` (incl. `reputation.rs`) | Shipping | Feed substrate: summarisation, prioritisation, twin-to-twin, sender trust |
| `core/crates/summit-server/` | Not a workspace member, 0 hits in `Cargo.lock`, does not compile | **Delete** (confirmed). It is the only VC issuer (`vc_issuer.rs`) — phase 2 will need a real issuer, but not this prototype |
| Verida / VDA | ~277 doc references, **zero `.rs` references** | **Not used.** Purge per `Ledger-plan.md` |

## Decisions locked

| Decision | Choice |
|---|---|
| Feed MVP and CC | **Ship without CC.** Prioritise on owner rules + AI learning/mode + urgency/deadline + reputation. CC lands with tokenomics phase 2 |
| Harness selection | **Time-boxed bake-off** behind an adapter trait — Claude / OpenClaw / OmegaClaw |
| First Feed sources | **Email + calendar** |
| Web UI | Keep the Node backend; the new crate is **API-only** |
| Thick client | Darren continues Flutter in `KwaaiNetGUI`; public release needs a **browser-first** journey |
| summit-server | Delete |
| Verida | Not used |
| Browser/adoption doc | `OpenAI-Petal/MASS_ADOPTION_STRATEGY.md` (Sep 2025) is ~1yr old and Verida-dependent — **needs revision** before it guides the UX block |

### Why CC is deferred — the tension worth understanding

`Ledger-plan.md` locks credits as **work-only, non-transferable, no mint, pure
zero-sum**. CC in the Feed doc is the opposite: value attached to a message between
**strangers**, held in escrow, reclaimable if unopened, credited to the recipient on
open, tippable, and quoted as an "Indiscriminate Price". That is transferable money and
it needs the issuer from `TokenEconomy-plan.md`. Deferring CC keeps the
zero-sum-ledger-first ordering intact and still leaves a Feed that delivers the
document's core promise.

## Sizing scale — calibrated against what already exists

T-shirt sizes below are anchored to measured bodies of functionality in the
current tree, so they mean something. Src LOC excludes tests.

| Body | Src LOC | Test LOC | Test fns | Size |
|---|---|---|---|---|
| Knowledge / RAG (`kwaai-rag` + `rag_cmd`, `rag_api`) | 35,209 | 4,115 | 359 | **XL** |
| P2P network + DHT (`kwaai-p2p`, `-daemon`, `-hivemind-dht`, `-rpc` + `node`, `p2p_cmd`) | 22,370 | 10,570 | ~290 | **XL** |
| Distributed inference (`kwaai-inference`, `-distributed`, `-compression` + `shard_*`, `grpc_server`) | 17,617 | 0 | 82 | **L** |
| Platform / CLI shell (`main`, `cli`, `config`, `updater` + `map-server`) | 8,429 | 0 | ~94 | **M** |
| Storage / VPK (`kwaai-storage` + `vpk*`, `storage_rpc`) | 4,141 | 618 | 58 | **M** |
| Ledger (unmerged, `feat/kwaai-ledger`) | ~2,440 | — | — | **S–M** |
| Trust / identity (`kwaai-trust`, `-wasm` + `identity`, `reputation`) | 2,156 | 0 | 6 | **S** |

Roughly: **S** ≈ 1–3k · **M** ≈ 4–9k · **L** ≈ 15–20k · **XL** ≈ 20k+.

**Two caveats, because LOC misleads in opposite directions here.** P2P is 22k
lines against RAG's 35k but was the harder build — compare the test ratios, 47%
against 17%. Distributed-systems work costs far more per line, and PR #107 /
issue #108 are the class of bug that lives there. Sized by *difficulty* rather
than volume, P2P is the XL and RAG is a large-but-tractable pipeline with fast
feedback loops. Separately, distributed inference carries **no dedicated `tests/`
directory at all** — 17.6k lines on 82 inline tests, the thinnest coverage of any
major body, which is worth knowing independently of this plan.

### What the calibration changed

Sizes were first assigned by feel and then checked against the table above. Two
were wrong in ways worth recording:

| Block | First estimate | Calibrated | Why it moved |
|---|---|---|---|
| Unified Feed | XL | **XL** | Unchanged. Plausibly the largest body attempted here |
| Tokenomics ph.1 | M | **S** | ~2,440 lines already written on `feat/kwaai-ledger`. "Already built" was under-weighted |
| Tokenomics ph.2 | XL | **L–XL** | Issuer + minting + escrow + settlement — larger than Storage/VPK's 4.1k, plus regulatory work |
| UX | L | **M + L** | Was one size for two separable pieces. `kwaai-api` is M against the 8.4k Platform shell; the browser journey is the L |
| Agentic harness | L | **M** adopting | Integrating OpenClaw behind an adapter is much less than building a runtime. Stays **L** only if the bake-off says build our own |

The pattern in both errors is the same: sizing the *problem* rather than the
*remaining work*. Worth repeating the check before committing to a schedule.

## State of the existing bodies — what is left

Size says what is built; this says what remains. Some of it gates the release.

| Body | Size | Maturity | Remaining |
|---|---|---|---|
| Knowledge / RAG | XL | **Production** | Nothing gating the release |
| P2P network + DHT | XL | **MVP after the v0.6 cutover** | Land #107, fix #108, complete the cutover |
| Distributed inference | L | **Incomplete — paused Feb 2026** | Sharded inference does not work on Mac Metal; paused awaiting driver support |
| Platform / CLI shell | M | **API planned** | `map-server` is Python in the OpenPetal project — rewrite in Rust; ultimately each node discovers and serves its own map |
| Storage / VPK | M | **Needs VPK integration** | See the data-sovereignty item below |
| Ledger | S–M | **Written, unmerged; plan good** | Rebase and land |
| Trust / identity | S | **Needs ToIP DTGWG update** | Adopt VRC/PHC/r-cards; peer-to-peer issuance, no central authority |

### The Mac Metal gap is scoped, not a release blocker

Worth stating precisely, because "distributed inference is broken on Mac" reads
like a release blocker and is not one. What does not work on Metal is **sharded**
inference — the Petals-style block path. **Full-model inference on Macs works
today** via `kwaai-cli/src/llama_local.rs`, which uses llama-cpp-2 with Metal
acceleration.

So a Mac user in the public release can run a model locally or reach one over
p2p. What they cannot do is *participate in sharded inference of a model too
large for one device*. That makes sharding a **scale feature, not a gate**, as
long as the Feed's inference needs are served by full-model local or remote p2p
— which they are. Confirm rather than assume when the Feed's inference path is
designed.

### Trust: ToIP DTGWG alignment is conformance, not redesign

**There is no central credential authority. Trust is relative and subjective.**
That is the model, and KwaaiNet already matches it — `reputation.rs` is
local-subjective by explicit design. So this is conformance work, not a rewrite.

The Decentralized Trust Graph Working Group (formed March 2025, joint ToIP/DIF)
standardises *portable* trust graphs between people, groups, organisations and AI
agents, on W3C DIDs and Verifiable Credentials, where **all parties control their
own subgraph** and **credentials are issued peer-to-peer rather than by
centralised authorities**.

Primitives worth designing against rather than reinventing:

| Primitive | What it is | Where it lands for us |
|---|---|---|
| **VRC** — Verifiable Relationship Credential | Attests a trust connection between two entities | The Twin's sender reputation |
| **PHC** — Personhood Credential | Verifies an individual is a person | Sybil resistance, incl. the phase-2 welcome balance |
| **r-cards** | Portable relationship data | Trust that travels with the user, not the node |
| **Social vouching / out-of-band introduction** | How trust is established without an authority | How an *unknown* sender earns a path in |
| **Trust task protocols**, **Agent Names** | Active task forces | Twin-to-twin negotiation and addressing — see open item 5 |
| Zero-knowledge proofs | Privacy-preserving assertions | Disclosing trust without disclosing the graph |

Two known defects gate any of this, both recorded in `Ledger-plan.md`: the
`did:key` vs `did:peer` verification mismatch, and a canonicalisation bug. Both
already block VC-carrying-weight work.

**Kwaai is a participant, not an adopter.** We have four volunteers in the working
group, which means influence over specs still in draft and early access to the
reference code — and it makes contributing back the cheaper path than diverging.

**DTGWG ships code, not only specifications.** Repos under
[`github.com/trustoverip`](https://github.com/trustoverip):

| Repo | What | Language |
|---|---|---|
| `dtgwg-trust-tasks-tf` | **Trust Tasks** — transport-agnostic protocol for managing tasks | **Rust** (`trust-tasks-rs`, plus TS bindings, DIDComm + HTTPS transports) |
| `dtgwg-cred-tf` | Credentials task force (the specified credential set) | spec |
| `dtgwg-zkp-tf` | ZKP conformance requirements | spec |
| `aimwg-tsp-enabled-ai-agent-protocols` | TSP-enabled **AI agent** protocols | spec/JS |
| `tswg-tsp-specification` | Trust Spanning Protocol | spec/JS |

`trust-tasks-rs` is the one to look at first: a Trust Task is a self-contained,
transport-agnostic JSON document describing finite work, schema-validated, with a
live registry at `trusttasks.org/registry`. Specs move draft → candidate →
standard.

**The fit is unusually good.** Trust Tasks is deliberately transport-agnostic and
ships DIDComm and HTTPS bindings; KwaaiNet has a p2p unary transport that already
works. So `kwaai-twin` should *consume* `trust-tasks-rs` and carry it over
`kwaai-p2p` rather than invent a twin-to-twin protocol — and a libp2p transport
binding is then an obvious contribution back, which is exactly what having four
volunteers in the group is for.

**Licence: checked and clear.** GitHub reports NOASSERTION only because the repo
splits its licensing across two files. Verified:

| | Licence | Verdict against our MIT |
|---|---|---|
| Source code (`SOURCE_CODE.md`, and `license = "Apache-2.0"` in `Cargo.toml`) | **Apache-2.0** | **Compatible.** Permissive, no copyleft. Standard practice in the Rust ecosystem |
| Specifications (`LICENSE.md`) | **OWFa 1.0** | Grants copyright *and patent* rights for implementing conformantly — a benefit |

crates.io metadata agrees: `trust-tasks-rs` 0.9.0 is published as `Apache-2.0`.
Depending on it does not affect KwaaiNet staying MIT. Apache-2.0 also carries an
explicit **patent grant**, which plain-MIT dependencies do not.

Two conditions rather than blockers:

- If we ever *vendor* their source the way we vendor multistream-select, Apache-2.0
  §4 requires keeping the licence, attributing, and stating changes. There is no
  upstream `NOTICE` file, so nothing to propagate. Depending via crates.io avoids
  this entirely — prefer that.
- DCO plus CLA apply to **contributing back**, not to consuming. Relevant when we
  offer the libp2p binding, and our four volunteers are presumably already covered.

**It is a real, used ecosystem, and pre-1.0.** `trust-tasks-rs` has ~28.8k
downloads, with `trust-tasks-https` (12.2k), `trust-tasks-proof` (11.8k),
`trust-tasks-capability-client` (8.1k), `trust-tasks-didcomm` (4.6k) and
`trust-tasks-tsp` alongside it. Note the transport bindings are **separate
crates** — which is exactly the seam a `trust-tasks-libp2p` contribution would
slot into. The caveat is 0.9.0: pre-1.0, so expect API churn and pin
accordingly.

**Block 1 depends on this.** The Feed needs *domain-specific* sender reputation —
"Alice's opinion on X but not Y" — which is precisely a subjective trust graph.
The Twin cannot vet or prioritise properly without it.

**And it resolves a design question in the Feed.** A trust graph handles *known*
senders; the Indiscriminate Price handles the rest. Those are not competing
mechanisms — the CC price is the economic fallback for exactly the case where no
trust path exists, and social vouching is the non-economic one. Unknown senders
either pay or get vouched for.

### The map becomes a node capability, not a service

"Each node discovers and serves its own map" folds `map-server` into `kwaai-api`
rather than keeping a separately deployed service, and the Python original lives
out of tree in OpenPetal. This removes another central component — the same
direction as deleting summit-server — and is why the UX block owns the rewrite.

### VPK integration answers the post-Verida sovereignty question

Open item 4 asks what replaces Verida for data sovereignty now that it is
dropped. Storage needing VPK integration and that question are the same question:
`kwaai-storage`'s multi-tenant encrypted vector storage is the sovereign-data
story. It is **not** on the Feed MVP path, but it is on the *narrative* path for
a release that claims sovereignty — decide deliberately which of those the launch
needs.

## DTGWG code availability — what it changes

Reviewed after confirming Kwaai participates in the working group. The question
is which blocks shrink because code already exists.

| Available | Licence | Maturity | Effect on our sizing |
|---|---|---|---|
| `trust-tasks-rs` 0.9.0 + `-https`, `-didcomm`, `-tsp`, `-proof`, `-capability-client` | Apache-2.0 | ~28.8k downloads, pre-1.0 | **Twin protocol: adopt, do not design.** Transport bindings are separate crates, so `trust-tasks-libp2p` is the contribution seam |
| `ssi` 0.16.0 (Spruce) | Apache-2.0 | ~182k downloads | Candidate fix for the `did:key`/`did:peer` mismatch and canonicalisation bug — adopt a maintained DID/VC stack rather than repairing ours |
| VTI — `vta-sdk`, `vti-common`, `vtc-service` ([OpenVTC](https://github.com/OpenVTC/verifiable-trust-infrastructure)) | Apache-2.0 | 1,478 commits, active | Reference implementation of VTA/VTC. **Probably not adopted** — service- and organisation-oriented (incl. AWS Nitro enclaves), where we need node-local personal agents. Worth reading, not depending on |
| `dtgwg-cred-tf`, `dtgwg-zkp-tf`, `aimwg-tsp-enabled-ai-agent-protocols` | OWFa 1.0 | spec, active | Specifications to conform to; no code to adopt |

**Net effect: two blocks shrink, one is unaffected.**

- **`kwaai-twin`** loses protocol design and implementation — the largest and
  riskiest part of it. It becomes a Trust Tasks integration carried over
  `kwaai-p2p`, plus Feed-specific vetting policy.
- **`kwaai-trust`** shifts from "repair our DID/VC implementation" to "adopt a
  maintained one and conform". Same size, materially higher confidence.
- **`kwaai-agent`** is untouched — DTGWG has no harness.

### Staying current with their improvements

Yes — but it depends entirely on *how* we consume it, and the release data says
the naive approach would hurt.

**The churn is real.** `trust-tasks-rs` has published **86 versions since
2026-05-18**, going 0.5 → 0.6 → 0.7 → 0.8 → 0.9 in **eight days**. Under 0.x
semver every one of those minor bumps is a breaking change.

**But the transport bindings are maintained in lockstep**, and that is the
important signal: `-https`, `-didcomm`, `-tsp` and `-proof` each have exactly 13
versions, all first published 2026-05-18, all now at 0.9.0. They bump only on
core's minor line. In other words, when upstream breaks the transport trait, they
fix every binding in their own workspace at the same time.

That gives four rules:

1. **Depend via crates.io. Never vendor.** We already carry one patched
   dependency (`patches/multistream-select`) and it costs a patch file, a
   checksum-pinned fetch script, and a re-apply chore on every libp2p bump. Do
   not repeat that here — a fork stops improvements reaching us by definition.
2. **Contribute `trust-tasks-libp2p` upstream, not in-tree.** This is the whole
   mechanism. **No p2p binding exists today** — `bindings/` holds `didcomm`,
   `didcomm-v1`, `https`, `push` and `tsp`, and only eight `trust-tasks-*` crates
   are published (`push` is in-repo but unpublished, so not every binding reaches
   crates.io). The gap is real and the shape is well defined: implement the
   transport trait directly over libp2p, exactly as `trust-tasks-https` does over
   HTTP. Note TSP is **not** a shortcut — `trust-tasks-tsp` merely wraps
   `affinidi-tsp`, a third-party implementation, so TSP is a peer binding rather
   than an abstraction we could slot libp2p underneath. In their workspace, their refactors carry our binding along, as
   they have for the other four bindings across 13 lockstep releases. In ours, we
   absorb every break ourselves. Contributing is the cheap path, not the generous
   one.
3. **Pin exactly and upgrade deliberately.** At this cadence, floating is
   thrash. Batch upgrades, and use our four working-group volunteers as the early
   warning for breaking changes — that is a concrete, ongoing use for that seat.
4. **Isolate the dependency behind a thin internal adapter** inside `kwaai-twin`,
   the same shape as the `Harness` trait in Block 4. Churn then hits one module
   rather than the crate.

**The timing works in our favour.** `kwaai-twin` sits at step 4 of the build
order — months out on either velocity scenario. `trust-tasks-rs` went from 0.1.0
to 0.9.0 in three months; on that trajectory it plausibly reaches 1.0 well before
we couple to it deeply. Spike it early to prove scope fit, then integrate for
real once the API settles.

Residual risk remains the **scope fit**, not the churn: Trust Tasks models
*finite work*, and Feed access negotiation may not be that shape. Prove that on
the spike before committing.

## Schedule — availability, not velocity

Kwaai is volunteer-driven. The binding constraint is not how fast anyone codes;
it is whether they are there that month. Planning by aggregate velocity produces
a schedule that only works if everyone shows up, which is the wrong shape for
this organisation.

### Correction: August was not a slowdown

An earlier revision read "34 commits in August against 347 in May" as the team
stalling and hedged every date on it. **That was wrong.** Reza's authoring
dropped; Darren shipped **24,430 LOC** that month — the whole native-p2p stack,
#75 through #86 — as squash-merged PRs that commit counts structurally cannot
see. August was the second-strongest month of the year.

| Month | Reza | Darren | Team |
|---|---|---|---|
| May 2026 | 35,361 | 3,789 | 39,150 |
| June | 16,222 | 0 | 16,222 |
| July | 9,837 | 0 | 9,837 |
| August | 3,011 | 24,430 | 27,441 |

Team output averages **23,162 LOC/month** over the last four months. Velocity
was never the problem, so the dates below are built on availability instead.

### Two contributors, two shapes

Totals are closer than commit counts suggest — 675 LOC/commit against 163 is
what squash-merging looks like. The *shapes* are not close.

- **Reza — continuous.** Nine active months of twelve, 10–16k LOC/month
  sustained, 35k peak, while also carrying review, ops and planning. **The only
  capacity that can be committed to a date.**
- **Darren — bursty.** Four active months of twelve, but 24k in a peak one; he
  lands whole subsystems at a time. The risk is presence, not rate: an engaged
  month is worth one of Reza's, an absent month is worth zero.
- **Christophe — onboarding.** Senior, and already runs nodes, so not cold on
  the domain. Treat as upside until demonstrated.

### The rungs

17 effective weeks remain to 31 December (19.3 calendar, less holidays).
Durations are one contributor on one body, anchored on `kwaai-rag` — 28.5k LOC
over 3.4 months, the one contiguous XL build in the tree.

| Rung | Solo | Volunteer weeks needed | Confidence |
|---|---|---|---|
| 1 · Secure Network | 14 wk | 0 | **Commit** — 3 weeks spare |
| 2 · Browser front door | 19 wk | 2 | Likely — inside one engaged month |
| 3 · Feed Preview | 33 wk | 16 | Fragile — two people sustained through Q4 |

### Contributor fit

Christophe's specialities are **networking, cyber and IoT**, which map almost
exactly onto rung 1's riskiest piece.

| Who | Shape | Best brief | Why |
|---|---|---|---|
| **Christophe** | New, senior | **Authorization**, then **#108** | Signed DHT writes, tenant ownership and per-peer quotas are a security brief in a networking stack — his two specialities intersecting. #108 (loopback dial) is a small, security-adjacent networking bug: an ideal first merge |
| **Darren** | Bursty, lands subsystems | **Cutover completion**, #107 | He built the native stack; finishing it needs no ramp and suits a burst |
| **Reza** | Continuous | **`kwaai-api`**, ledger, coordination | The API's shape is decided once for all three rungs, so it belongs with the contributor who will still be here in rung 3 |

**This is upside, not dependency — and that is the point.** Rung 1 was sized for
Reza solo at 14 weeks. If Christophe delivers authorization, Reza's critical path
drops to `foundation → ledger → kwaai-api` = **9 weeks**, and the year looks very
different:

- Rung 1 completes around **week 9**, leaving 8 weeks of slack
- Rung 2 (browser front door) lands around **week 14** — comfortably inside
- Rung 3 still misses; it needs a further 14 weeks and would land at week 28

If Christophe does not materialise, Reza absorbs authorization and rung 1 still
fits at 14 weeks with 3 to spare. **The commitment does not move either way**,
which is exactly the property a volunteer plan needs.

Two notes beyond rung 1. His cyber background suits an ongoing **security review**
role — the tree currently has no authorization anywhere, and PR #98's native-p2p
security analysis has no standing owner. And **IoT is where the platform is
heading**: the mass-adoption thesis is browser plus mobile plus edge, carried by
`kwaai-wasm`. Not rung 1 work, but worth him knowing it is on the map — a
volunteer who can see their speciality in the roadmap stays.

### Designing for volunteers rather than around them

A volunteer schedule fails when one person's disappearance blocks three others.
Keep the critical path on the continuous contributor, and give volunteers work
that is **bounded, specified up front, and merge-ready in isolation** — exactly
the shape of Darren's August: eight PRs, one subsystem, landed in a burst.

Two pieces fit that shape well:

- **Authorization** — self-contained, spec-able before anyone is available, and
  testable against the existing interop harness.
- **`trust-tasks-libp2p`** — bounded, upstream, and carrying the external credit
  volunteers are motivated by.

Note that **four Kwaai volunteers are already committed to the LF ToIP working
group.** That is real capacity, deliberately spent on standards influence rather
than the release. Probably correct — but it should be a conscious choice.

### What would move these dates

- **Availability, first and last.** Re-read who has shown up at week 6, not how
  fast they went.
- **Christophe ramping.** If authorization lands with him, rung 2 becomes likely
  and rung 1 finishes five weeks early. If it does not, nothing breaks.
- **#107 unreviewed.** Step 0 of every rung, green and idle.
- **DHT signing vs Go peers.** The one estimate that could move; signatures
  should ride as an ignored extension field, unproven until the interop harness
  says so. Prove it early.
- **Not on this path:** VPK integration, Metal sharding, CC.


## Build order

Dependencies, not the numbering above:

```
RUNG 1 — COMMITTED, 2026
0. Foundation      land PR #107, fix issue #108 -> completes the v0.6 cutover,
                   which is what takes P2P to MVP
1. authorization   signed DHT writes, storage tenant ownership, per-peer quota
2. kwaai-ledger    rebase + merge; delete summit-server; purge Verida
3. kwaai-api       API surface; absorb map-server as a Rust, node-served map
                   -- SHIPPABLE HERE --

RUNG 2 — one engaged volunteer month
4. browser journey revise the adoption doc, then build the front door

RUNG 3 — two volunteers sustained through Q4
5. kwaai-trust     ToIP DTGWG alignment; fix did:key/did:peer + canonicalisation
   kwaai-agent     Harness trait + 3 spikes -> pick one
6. kwaai-feed      VA, store, prioritisation, email connector

AFTER 2026
7. kwaai-twin      vetting, provenance, twin-to-twin over trust-tasks (needs 5)
8. kwaai-economy   miles + issuer -> then CC in the Feed

deliberately off the release path:
   VPK integration      -> the sovereignty story; decide if the launch claims it
   sharded on Metal     -> blocked on drivers; a scale feature, not a gate
```

Steps 1, 2 and 3 are independent of each other and are the natural volunteer
briefs — authorization especially, being spec-able before anyone is available.
Step 0 is small and already in flight; #108 matters here because it can latch a
circuit breaker against a healthy peer until restart, which would be
indefensible in anything public.

**Rungs 2 and 3 must attach without rework.** That is the constraint rung 1's
design has to satisfy: `kwaai-api` carries the surface the browser front door and
the Feed both consume, so its shape is decided once, in rung 1, for all three.

## Verification

- **Ledger**: `cargo test -p kwaai-ledger`; re-run the cross-platform receipt exchange
  (macOS/Linux/Windows) the branch already proved; confirm `summit-server` deletion
  leaves `cargo test --workspace --no-run` green and CI's
  `ci-kwaai-platform.yml` trigger updated.
- **Harness**: each adapter runs the same Twin-vetting task on the same fixture inbox;
  compare accuracy, latency, token cost, and lines of integration glue.
- **Feed**: end-to-end against a real mailbox — ingest → Twin vets → VA prioritises →
  single-line previews. Assert owner rules always override learned ranking, and that
  the transparency view names the actual deciding factor.
- **API**: existing Node backend switched to `kwaai-api` with no frontend change.
- **Whole-system**: the D6 harness pattern
  (`tests/kwaai-knowledge/native_p2p_d6_rebuild.sh`) is the model — telemetry to a
  progress JSON, and a completeness audit before trusting any quality metric.

## Open items

1. **Feed connector auth** — email/calendar means OAuth against Google/Microsoft, with
   token storage and platform ToS review. Not yet scoped.
2. **CC regulatory** — escrow plus cash-out is money transmission in many
   jurisdictions. Needs an answer before phase 2 ships, not after.
3. **VC issuer** — deleting summit-server leaves nothing issuing Verifiable
   Credentials. Fine now; phase 2's issuer must fill this.
4. **Data sovereignty after Verida** — the adoption strategy leaned on it for the
   storage/identity story. This is the same question as "storage needs VPK
   integration": `kwaai-storage`'s multi-tenant encrypted vector storage is the
   candidate. Decide whether the launch *claims* sovereignty, because that decides
   whether VPK is on the release path or after it.
5. **Twin-to-twin protocol — likely solved, do not invent one.** DTGWG's
   `dtgwg-trust-tasks-tf` is a transport-agnostic protocol for exactly this, and it
   is **Rust**. Evaluate `trust-tasks-rs` carried over `kwaai-p2p` before designing
   anything bespoke; also check the Agent Names task force for addressing. Remaining
   question is scope fit — Trust Tasks models *finite work*, and Feed access
   negotiation may or may not fit that shape.
6. **Two different "issuers" — do not conflate them.** An earlier draft of this plan
   treated deleting summit-server as leaving a credential gap. That was wrong.
   DTGWG issues credentials **peer-to-peer with no central authority**, so removing
   the only central VC issuer is *aligned with* the model, not a hole in it — each
   node attests for itself. What phase 2 needs is an **economic** issuer to mint
   miles, which is a deliberate, separate trust assumption and already argued in
   `TokenEconomy-plan.md`. Identity credentials: decentralised. Currency: issued.
7. **Sharded inference on Metal** — paused since Feb 2026 awaiting driver support.
   Scoped out of the release above, but it is the one item whose unblocking is
   *outside our control*, so it should be re-checked rather than assumed dormant.
