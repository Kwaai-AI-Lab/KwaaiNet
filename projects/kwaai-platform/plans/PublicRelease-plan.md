# KwaaiNet Public Release — four blocks of functionality

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

---

## Block 1 — Unified Feed · `kwaai-feed` + `kwaai-twin` · **XL**

The product. A single prioritised stream replacing app-switching.

Two agents with **different trust boundaries**, which is why this is two crates:

- **`kwaai-twin`** — external-facing. Receives all inbound, verifies origin, deletes
  suspicious, checks provenance, scores importance/credibility, negotiates access
  (CC in phase 2), and proactively discovers information. Speaks p2p twin-to-twin.
- **`kwaai-feed`** — local. The VA role plus the feed itself: item model, store,
  connectors, prioritisation engine, owner rules, mode awareness, summarisation.

Scope for the release:

- Item model + store; connectors for **email + calendar**
- Prioritisation: owner-defined rules (deterministic, override) → AI learning from
  behaviour → urgency (sender-declared, modulated by Twin; deadline-driven; dynamic
  decay for neglected contacts) → sender reputation
- UI contract: single-line previews, hover-to-expand without marking read,
  click-to-engage, reappearance of unresolved items, notification batching,
  on-demand summarisation, **prioritisation transparency** (why is this here?)
- Outbox with follow-up tracking

Reuse: `kwaai-rag` for summaries and contextual assistance; `kwaai-inference` for
prioritisation and composition; `kwaai-trust::reputation` for sender trust (note it is
local-subjective by design — see `docs/reputation.md`); `kwaai-p2p` unary for
twin-to-twin.

Deferred to phase 2: all CC negotiation, Indiscriminate Price, vendor intent-pitching.

## Block 2 — Tokenomics · `kwaai-ledger` (exists) → `kwaai-economy` · **S then L–XL**

**Phase 1 — land the ledger (S).** Rebase `feat/kwaai-ledger` onto `main` (16 ahead /
22 behind), re-verify cross-platform, merge. Delete `summit-server` and its 13 external
references (`Dockerfile.eve`, root `CLAUDE.md`, `CONTRIBUTORS.md`,
`.github/workflows/ci-kwaai-platform.yml`, `docs/{ARCHITECTURE,TODO,BOOTSTRAP_SERVER_V2_REQUIREMENTS}.md`,
five `projects/kwaai-platform/` files). Purge Verida from ~10 docs.

**Phase 2 — the token economy (L–XL).** Per `TokenEconomy-plan.md`: trust (local,
non-transferable, earned by uptime/routing) kept strictly separate from miles
(network-wide, minted by an issuer at settlement, earned by provable work). Needs a new
issuer service — the trust assumption is not new, since the two bootstrap servers are
already fully trusted. **CC rides on this phase**, adding escrow, reclaim, and tipping.
The cash-out path carries real regulatory weight and must be designed in, not bolted on.

## Block 3 — UX · `kwaai-api` **M** + browser-first journey **L**

- **`kwaai-api` (M)** — Rust crate exposing the node's HTTP/WS API. Consumed by the
  existing Node backend, by the Flutter client, and by the Feed UI. API only; the Node
  backend and React frontend stay.
- **Browser-first journey (L)** — the public release's front door. Requires first
  **revising** `MASS_ADOPTION_STRATEGY.md`: it is a year old and its architecture
  assumes Verida throughout, which is now dropped. The WASM thesis (via
  `core/crates/kwaai-wasm`) survives that revision; the data-sovereignty layer needs a
  replacement answer.
- Thick client stays in `KwaaiNetGUI` (Darren), consuming `kwaai-api`.

## Block 4 — Agentic harness · `kwaai-agent` · **M** adopting, **L** building

Define a narrow `Harness` trait — tool registry, act loop, memory, channels — then
spike three adapters against **one** real task: the Twin vetting and scoring an inbound
message.

| Candidate | Shape | Fit |
|---|---|---|
| **Claude** (Agent SDK / MCP) | Mature, hosted models, strong tool-calling | Lowest risk; weakest on local-first sovereignty |
| **OpenClaw** | MIT, local-first, model-agnostic (Ollama), **gateway already aggregates WhatsApp/Discord/Slack** | Closest fit — its gateway *is* much of the Feed's connector problem |
| **OmegaClaw** (SingularityNET/ASI) | Hyperon neural-symbolic, NAL + PLN, goal-autonomous, three-tier memory | Most ambitious, most research-grade, highest risk |

Deliver the trait plus a written recommendation with evidence. Also expose KwaaiNet
capabilities (RAG, graph, shard inference, storage) over **MCP** so external agents can
drive the network — there is no MCP implementation in the tree today, only two doc
mentions.

---

## Build order

Dependencies, not the numbering above:

```
0. Foundation      land PR #107, fix issue #108 -> completes the v0.6 cutover,
                   which is what takes P2P to MVP
1. kwaai-ledger    rebase + merge; delete summit-server; purge Verida
2. kwaai-trust     ToIP DTGWG alignment; fix did:key/did:peer + canonicalisation
   kwaai-agent     Harness trait + 3 spikes -> pick one
3. kwaai-api       API surface; absorb map-server as a Rust, node-served map
4. kwaai-feed      VA, store, prioritisation, email+calendar connectors
   kwaai-twin      vetting, provenance, twin-to-twin p2p  (needs step 2 trust)
5. browser journey revise adoption doc, then build the front door
6. kwaai-economy   miles + issuer -> then CC in the Feed

deliberately off the release path:
   VPK integration      -> the sovereignty story; decide if the launch claims it
   sharded on Metal     -> blocked on drivers; a scale feature, not a gate
```

Steps 2 and 3 can run in parallel. Step 0 is small and already in flight; #108 matters
here because it can latch a circuit breaker against a healthy peer until restart, which
would be indefensible in a consumer product.

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
5. **Twin-to-twin protocol** — agent-to-agent negotiation is unspecified. Check
   DTGWG's **Trust Task Protocols** and **Agent Names** task forces before designing
   one: agent-to-agent addressing and trust interactions are exactly their scope, and
   an interoperable Twin is worth more than a bespoke one.
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
