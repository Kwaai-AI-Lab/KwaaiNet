# KwaaiNet Token Economy — utility miles over a local trust fabric

## Context

KwaaiNet now has a working **bilateral** credit system: co-signed work receipts, verified
live across macOS, Linux and Windows (`feat/kwaai-ledger`, `ffbe735`). Credits are pairwise
and non-transferable — you can only draw on someone you have already served.

The ambition is larger: KwaaiNet as digital public infrastructure with a **utility token that
behaves like frequent-flyer miles** — earn broadly, redeem broadly, with buying and cashing
out as tightly controlled edge cases. That requires network-wide fungibility, which the
current design deliberately avoids because it needs either consensus or a trusted party.

This plan takes the second route, which the codebase's trust topology already implies, and
splits the problem into two currencies that must never merge:

- **Trust** — local, observed, non-transferable. Earned by uptime and routing quality.
  Governs *how much exposure* a peer will carry for you.
- **Miles** — network-wide, minted by an issuer at periodic settlement. Earned by
  cryptographically provable work. Governs *what you can redeem*.

Keeping these apart is the core insight of the design. The contributions that are easiest to
fake (uptime, routing) earn only the thing that cannot be faked to a peer who is measuring you
directly. The contributions that are cryptographically self-evidencing (tokens, embeddings)
earn the thing that is transferable.

### Decisions locked with the user

| Decision | Choice |
|---|---|
| First milestone | **Standing now, transfers later** — forward-compatible artifacts |
| Counterparty | **Peers transact, Kwaai settles periodically** — receipts are the settlement input |
| Uptime / routing | **Earn trust, never currency** |
| What trust controls | **Unsettled credit limit** |
| Trust scope | **Local-subjective**, enriched by transitive query per `docs/reputation.md` §4 |
| Sybil gate | **Proof-of-resource** unlocks the welcome balance |
| Minting rule | **Diversity-weighted net contribution** |
| Pricing | **Issuer rate card** (award chart) |
| Bootstrap / relay operators | **Issuer grants, attested out-of-band** |

### Three prior decisions this reverses, deliberately

1. *"No faucet — pure zero-sum, total supply exactly 0"* → a welcome balance means minting.
2. *"No external convertibility, permanently"* → buy / cash-out return as controlled edge cases.
3. *"summit-server deleted, no central component"* → miles requires an issuer. Note the trust
   assumption is **not new**: the two Kwaai bootstrap servers are already fully trusted for all
   DHT state, and the DHT is entirely unauthenticated.

The cash-out path carries regulatory weight that airline miles avoid precisely *because* they
cannot be cashed out. It drives real architecture — transfer controls, identity requirements,
reserve accounting — and should not be bolted on later. Flagged, not resolved here.

## Architecture

### The join: signed balance certificates

This is the piece that makes "peers transact, Kwaai settles periodically" concrete.

At each settlement epoch the issuer signs a **balance certificate** per member —
`{owner_did, miles, epoch, expires_at, sig}` — using the same canonical-encoding rules as
`kwaai-ledger` (positional msgpack, no maps, no floats; see `kwaai-ledger/src/lib.rs:34-58`).

A consumer presents its certificate when requesting service. The provider serves up to
`cert.miles + trust_credit_limit(peer)` and accumulates ordinary co-signed receipts. At the
next settlement both parties' receipts are submitted, netted, and fresh certificates issued.

Consequences worth being explicit about:

- **Intra-epoch overspend is possible.** A node holding 100 miles can present the same
  certificate to five providers at once. This is detection-not-prevention: settlement sees the
  overspend, and the trust-derived credit limit bounds the loss per provider. Shorter epochs
  tighten it. This is an accepted property, not an oversight.
- **Replay is already solved.** `receipt_id` is a content address and the store's primary key
  (`kwaai-ledger/src/store.rs`), so double-submission to the clearing house is rejected by the
  existing uniqueness constraint.
- **Offline tolerance is free.** The issuer is never in the request path, so peers keep
  transacting through an outage and reconcile later.

### Layer 1 — Trust (local, per `docs/reputation.md`)

`docs/reputation.md` is already the written spec. Implement it rather than redesign it. The
gap between doc and code is large:

| Spec says | Code today |
|---|---|
| `s_throughput = observed / claimed` | field exists, **never populated** |
| `s_uptime` | not measured at all |
| VC component, 1-year decay | exists, **never combined** with metrics |
| §4 transitive query | no protocol |
| §5 endorsement accountability | not implemented |
| decay on observations | **none** — a year-old sample counts fully |

Because `s_throughput` defaults to `0.0` and is never populated (`reputation.rs:273`), every
peer is capped at ~0.70 and **no node can currently reach `Trusted`**. Fixing this is a
prerequisite for trust gating anything.

**The VC component must start disabled (α = 0).** `kwaai-trust` has no issuer≠subject check and
no allow-list, so a node can self-issue a full credential set and reach score 1.0; signatures
are never verified on any automated path. Its `CredentialSubject.claims` is also a `HashMap`
with `serde(flatten)`, giving non-deterministic signing preimages. Until those are fixed, trust
must derive from observed metrics only.

### Layer 2 — Receipts (built; coverage is the problem)

The mechanism is sound and live. The coverage is not, and this is urgent independent of the
economy:

- **Streaming bypass.** `parse_token_counts` (`ledger_node.rs:391`) parses the whole body, so
  `"stream": true` NDJSON yields `None` → no claim → **free inference**. The dominant remote-GPU
  consumers in this repo already stream: `kwaai-rag/src/graph.rs:5297`,
  `kwaai-rag/src/sequence.rs:283` and `:1837`. Every dream cycle and graph build is currently
  unbilled. The final NDJSON line carries `eval_count`/`prompt_eval_count`, so verifiability
  survives the fix.
- **`p2p://` ollama-proxy** has no ledger at all.
- **`shard-proxy`** — which `resolve_inference_urls` *prefers* — has no lease, no ledger, and
  `shard_api` fabricates counts as `text.len()/4` with `prompt_tokens: 0`.
- **Storage cannot be billed at all**: `/kwaai/storage/1.0.0` has no authorization, and
  `ListTenants` returns every tenant UUID to any anonymous caller, making the bearer capability
  meaningless. This is a security bug regardless of billing.
- **Only one protocol authenticates its caller.** `add_unary_handler_with_peer` exists
  (`kwaai-p2p-daemon/src/persistent.rs:376`) but has zero adopters outside the mux stream.

**Block sharding stays out of the mint.** Quantity is verifiable but *correctness is not* — a
lazy hop can return zeros of the right shape at near-zero cost, indistinguishable from honest
work. Minting against it without redundant execution or spot-checks would be paying for nothing.

**Embeddings should be added early**: `embeddings.len() × dim` is directly countable from the
delivered bytes, making them *cleaner* to meter than chat tokens.

### Layer 3 — Miles (new)

A Kwaai-operated settlement service. Responsibilities:

1. **Receipt intake** — members submit signed receipts. Dedupe on `receipt_id`; verify both
   signatures with the existing `Receipt::verify()`.
2. **Minting** — diversity-weighted net contribution. Mint on net position across *distinct*
   counterparties, discounting dense clusters that trade mostly with each other.
3. **Rate card** — signed, versioned award chart mapping each resource to miles
   (per 1k tokens, per GB-month, per embedding batch). One comparable unit across otherwise
   incommensurable resources.
4. **Certificates** — issue signed balance certs per epoch.
5. **Welcome balance** — released only on proof-of-resource.
6. **Grants** — out-of-band credits for bootstrap and relay operators.

**Wash trading is the attack minting introduces.** Two colluding identities can trade trivial
work and farm the reward; the existing self-dealing exclusion only catches same-DID. The
defence is the one advantage of central settlement: it sees the whole receipt graph at once, so
collusion rings appear as dense subgraphs with little external connectivity — something no
individual peer could ever detect.

## Phases

**Phase A — Metering coverage.** No economics; makes the numbers real.
- Fix the streaming bypass (parse the final NDJSON line).
- Meter embeddings.
- Adopt `add_unary_handler_with_peer` on `ollama-proxy`, `shard-proxy`, `block_rpc`, `storage`.
- Fix storage authorization; scope or remove `ListTenants`.
- Per-peer lease accounting (`LeaseRow` keys `connection_id`, and the unary path hardcodes
  `UNARY_CONNECTION_ID = 0` for every caller).

**Phase B — Trust ("standing").** Delivers user-visible value with no currency.
- Populate `observed_tps`/`claimed_tps`; add uptime sampling; add time decay.
- Combine metrics with the VC component behind a configurable α, **defaulting to 0**.
- New `/kwaai/trust-query/1.0.0` unary protocol for §4 transitive enrichment — signed
  responses, weighted by the asker's own trust in the responder. Not gossip.
- Derive a credit limit from trust and enforce it in the lease/admission path.
- Optionally §5 endorsement accountability.

**Phase C — Miles.** The currency.
- Settlement service, rate card, receipt submission, minting, balance certificates.
- Proof-of-resource gate and welcome balance.
- `kwaainet miles show / statement`.

**Phase D — Redemption controls.** Transfers between members, then gated buy / cash-out. Not
designed here; requires the regulatory position settled first.

## Critical files

- `core/crates/kwaai-cli/src/ledger_node.rs` — `parse_token_counts` (:391) streaming fix;
  rate-card pricing replaces `DEFAULT_PRICE_MICRO_PER_1K_TOKENS` (:50)
- `core/crates/kwaai-cli/src/reputation.rs` — decay, uptime, tps population, α-blend
- `core/crates/kwaai-trust/src/trust_score.rs`, `verify.rs` — issuer≠subject, allow-list, and
  actually calling `verify()` before scoring
- `core/crates/kwaai-p2p-daemon/src/persistent.rs:376` — `add_unary_handler_with_peer`, the
  prerequisite for every unauthenticated protocol below
- `core/crates/kwaai-cli/src/ollama_proxy.rs`, `block_rpc.rs`, `storage_rpc.rs` — authenticate
  callers; `storage_rpc.rs:237` `ListTenants` leak
- `core/crates/kwaai-cli/src/capacity_lease.rs` — per-peer keying; trust-derived credit limit
- `core/crates/kwaai-ledger/` — reuse the canonical-encoding discipline verbatim for
  certificates and rate cards
- `docs/reputation.md` — the trust spec; implement, don't redesign

## Verification

- Per phase: `cargo test -p kwaai-ledger`, `cargo test -p kwaainet`,
  `cargo clippy --all-targets`, `cargo fmt --check`. Two `grpc_server::tests` failures are
  **pre-existing and environmental** (a running daemon holds port 8093).
- **Phase A metering proof**: run a real `dream run` / `graph build` against a remote peer and
  confirm receipts are now produced for streamed calls — today that produces none. Compare
  billed tokens against Ollama's own reported counts.
- **Two-node local harness** (`~/.claude/projects/.../memory/project_two_node_local_p2p_test.md`):
  two real nodes on one Mac over real libp2p, each with its own `KWAAINET_HOME` and ledger.
  This is what caught the dropped-`ReceiptAck` bug; use it for every cross-peer change.
- **Cross-machine**: metro-linux and metro-win, both on v0.5.5 and already reconciling exactly
  (76/4 and 57/3). No SSH — Reza runs commands directly.
- **Wash-trading test**: two colluding identities trading trivial work must earn materially less
  than two independent nodes doing the same volume against diverse counterparties.
- **Sybil test**: N fresh identities without proof-of-resource must mint zero.

## Explicitly not in scope

- **Consensus, chain, or global replicated state.** The DHT is unauthenticated, author-mutable,
  360s TTL, mediated by two bootstrap servers — unusable as a ledger substrate.
- **Gossip.** No pubsub primitive exists; p2pd is not even launched with it enabled. The
  transitive-trust protocol is deliberately request/response instead.
- **Minting for block-shard hops, relay bandwidth, or storage-at-rest** until challenge
  protocols exist. Relay is opaque inside p2pd and endpoints cannot prove which relay carried
  traffic.
- **Proof of correct computation.** Receipts prove delivery and mutual agreement, nothing about
  output quality.
- **Transfers, buy-in, cash-out** — Phase D, gated on the regulatory position.

## Open questions

1. **Where does the settlement service live?** A new crate in this repo, an extension of
   `map-server` (which today has no signing key, no write path, no persistence), or a separate
   Kwaai-operated service outside this repo. Affects whether Phase C is in this workspace at all.
2. **Settlement cadence.** Directly sets maximum unsettled exposure and intra-epoch overspend
   risk. Needs a number.
3. **Do miles expire?** Breakage is core to the airline model and caps issuer liability;
   expiry also keeps credits circulating rather than hoarded.
4. **`map.kwaai.ai` endpoint mismatch** — the CLI fetches `/api/v1/state` but the `map-server`
   crate serves `/api/stats` and `/api/nodes` with a different shape. The deployed service is
   evidently not this crate. Worth resolving before adding economic surfaces to it.
