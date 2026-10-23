# Trust Tasks ↔ KwaaiNet node-to-node interactions

*2026-10-14. Upstream catalogue read at `trustoverip/dtgwg-trust-tasks-tf@15b91d1f` (634 distinct
task slugs, latest version of each). KwaaiNet inventory read from the code on
`docs/dtgwg-trust-stack-plan`; the trust gaps below were re-checked against the code.
Companion to `TrustTasksLibp2p-implementation-plan.md` and `DTGWG-TrustStack-plan.md`.*

## Summary

- **34 of 634 tasks fit a node-to-node exchange KwaaiNet has or plans.** Five of them are
  framework tasks (error, next-step, control, discovery, ok) that the binding carries
  anyway.
- **155 more fit only if KwaaiNet runs as a Verifiable Trust Community (VTC):** the
  community, rather than a peer, is one party. That is a design decision we have not made
  (see "The VTC question").
- **445 are out of scope for node-to-node use:**
  - human, browser or wallet ceremonies: passkeys, out-of-band auth, step-up, persona
  - DID hosting: webvh, did-management
  - forge governance: git-ns
  - administration of mediators, vaults, key custodians, websites and external accounts
- **None of our data-plane traffic should become Trust Tasks.** That covers block RPC,
  the inference proxy and mux, Kademlia, identify, relay, DCUtR, AutoNAT, and DHT
  store/find. The data plane is per token or per request, and libp2p already
  authenticates it by PeerId. What it lacks is authorization, which is a handler fix, not
  a Trust Task.
- **Upstream has no task for most of our planned economic interactions:**
  - capacity lease
  - the ledger quote, claim and receipt
  - intent routing
  - EigenTrust score queries
  - federated RAG query and rerank
  - vector search

  These would be Kwaai-authored task specs, an opportunity as much as a gap.

## Strong fits: tasks that match an exchange we plan

| KwaaiNet interaction | Status | Trust Task(s) | Fit | Notes |
|---|---|---|---|---|
| Peer endorsement (`PeerEndorsementVC`, EigenTrust input) | types only | **`vtc/relationships/request` 0.2** | exact | "A member asks another member to issue them a VRC, and receives the signed VRC in the response." A decline is `trust-task-error` `…:declined`. This is our first-task candidate almost word for word, with a credential type upstream already defines. Despite the `vtc/` prefix it is a member-to-member exchange; `publish` (lodging the VRC with a community) is a separate, optional step. |
| Mutual endorsement | — | `vrc/relationships/propose` + `vrc/relationships/issue` | good | Peer-to-peer and mutual, with a pairwise DID and optional witnessing (`witness/session`). Heavier than `request`; use it if endorsements should be reciprocal. |
| Credential verification on demand (replaces unverified `trust_attestations` in announcements) | planned (§3 credential exchange) | `credential-exchange/query` → `credential-exchange/present` | good | DCQL query with nonce and stated purpose → OID4VP presentation. Lets a client ask a candidate inference or storage peer for, say, its VerifiedNode credential *before* routing, instead of trusting an unsigned DHT blob. The `pending/*` tasks cover holder-side consent, which a headless node would auto-decide by policy. |
| UptimeVC (bootstrap-issued), ThroughputVC (peer-witnessed) | types only | `credential-exchange/offer` → `request` → `issue` | good | OID4VCI flow with key binding. The issuer is a bootstrap or witness node, the holder the measured node. |
| Trust-filtered routing ("is this peer recognised / authorised for model X?") | planned (`reputation.md` §6) | `registry/recognition`, `registry/authorization` (TRQP), `vtc/recognition/check` | good if a registry exists | Asks a trust registry, not the peer. Needs Kwaai, or a community, to run a registry; the bootstraps are the obvious host. |
| Which tasks does this peer support? | — | `trust-task-discovery` | exact | Task-level complement to identify's protocol list; needed once more than one task exists. |
| Cancel or suspend long work | — | `trust-task-control` | good | For long-running tasks (a credential issuance awaiting a decision; later, leased jobs). Not for inference requests. |
| Refusals and multi-step answers | — | `trust-task-error`, `trust-task-next-step` | framework | Every task uses them. `ok` is retired. |
| Ledger audit chain (`ledger.db`, branch `feat/kwaai-ledger`) | planned | `audit/list`, `audit/verify` | partial | Upstream's "walk an append-only hash chain and report the first break" matches what a counterparty would want from a ledger. Our receipts are bilateral, not one maintainer's log, so this covers verification, not settlement. |
| Proof of work done (`WorkClaim` / `ReceiptAck`) | planned | `process-attestation`, `trust-ceremony-receipt` | weak | Same shape (challenge-bound, evidentiary response with mandatory proof), but meant for authorship processes and ceremonies. Better as a Kwaai task modelled on `process-attestation`'s proof and `issuedAt` rules. |
| Storage tenant authorization (today: anyone with a tenant UUID can read, write or delete; `ListTenants` lists all) | planned (storage Phase 2 auth) | `acl/grant`, `acl/revoke`, `acl/show`, `acl/list` | partial | Tenant ownership and delegation as ACL entries held by the storage node (Eve). The control plane fits; per-call checks stay in the storage RPC handler. |
| Node attestation (what binary and config a peer runs) | — | `vta/attestation/status`, `report`, `config-report` | future | TEE-bound. Our nodes would answer "no TEE" today. Relevant if confidential inference arrives. |
| Owner approval of a privileged remote action | — | `task-consent/request`, `decision`, `granted` | future | E.g. an owner's phone approving a remote `/api/pull` or config change. Nothing like it exists or is planned. |
| Liveness at the task layer | — | `messaging/ping` | redundant | libp2p ping and `rpc_ping` already do this. |

## Conditional fits: if KwaaiNet is a Verifiable Trust Community

If the network itself (bootstraps plus governance) acts as a VTC, then 155 more tasks
apply. The ones that map onto something we have or plan:

| KwaaiNet concept | Trust Task family |
|---|---|
| Node joins the network, today via summit-server `/node/bind` and `FiduciaryPledgeVC` | `vtc/join-requests/{manifest, submit, status, accept}`, `vtc/members/vmc` |
| Network-issued node credentials (VerifiedNode, Uptime) | `vtc/endorsements/{issue, revoke, list}`, `vtc/endorsement-types/register` |
| EigenTrust's trust graph | `vtc/relationships/{publish, list, graph, revoke, suspend}` |
| Node key rotation (no path today: a new key is a new node) | `vtc/members/{rotate-challenge, rotate}` |
| Removing a misbehaving node | `vtc/members/{admin-remove, removal-notice}`, `vtc/endorsements/revoke` |
| Sybil resistance / vetting | `vtc/vetting/*`, `vetting/*` (human vetting; probably over-built for nodes) |
| Credential schemas for our VC types | `vtc/schemas/*` |
| Storage fabric (Bob stores encrypted data on Eve) | `rooms/*` (create, records put/get/list, blobs chunked upload/download, usage) + `vtc/rooms/limits/set` for quota. A partial match: rooms are MLS-sealed record stores with no vector search |

This is the bigger architectural question the catalogue raises. Upstream's centre of
gravity is the community, an operated service with an ACL, registry and status lists, and
most identity tasks assume one. The plan's model so far was a pure peer mesh.

**Decided 2026-10-14: KwaaiNet becomes a VTC.** `summit-server` was our first, abortive
attempt at one. Whether we also need a Verifiable Trust Agent (VTA) is still open: in
OpenVTC's reference implementation every VTC sits on top of one. See "Still open" in the
implementation plan.

## Not candidates: keep as libp2p and RPC

Each of these is already authenticated by the Noise PeerId. Several have real trust gaps,
but the fix for each is in our code, not a Trust Task.

| Interaction | Why not a Trust Task | The actual gap |
|---|---|---|
| `/kwaai/inference/1.0.0` block RPC | per token, per hop | handler can't see the caller (`add_unary_handler` takes `Fn(Vec<u8>)`) |
| `/kwaai/ollama-proxy/1.0.0`, `/kwaai/shard-proxy/1.0.0` | per request | same, and the proxy forwards **any** method and path to `localhost:11434`, including `/api/pull` |
| `/kwaai/inference-mux/1.0.0` | per request on a long-lived stream | knows the caller, but authorizes nothing; requests without a lease pass straight through |
| `/kwaai/capacity-lease/1.0.0` and mux lease frames | no upstream task; latency-adjacent | every unary lease shares `UNARY_CONNECTION_ID = 0`, so a lease is not tied to the peer that asked for it |
| Hivemind `rpc_store` / `rpc_find`, announce and tombstone records | records, not tasks | no record validators: anyone can overwrite or tombstone any subkey; only `signed_addrs` is signed; `trust_attestations` is published but no reader verifies it |
| `/kwaai/storage/1.0.0` vector ops | per query | no authentication of any kind (see ACL row above for the control plane) |
| Kademlia, identify, ping, AutoNAT, relay, DCUtR | libp2p substrate | — |
| `/kwaai/p2p/hello`, `/kwaai/probe-unrouted` | diagnostics | — |
| map crawler, health monitor, auto-update | not peer-to-peer | map trusts unsigned records |

## Gaps: candidate Kwaai-authored task specs

No upstream task covers these. Writing them would make the binding useful beyond identity,
and is where Kwaai would contribute specs as well as a crate.

1. **Capacity lease** (`kwaai/capacity/lease`): request a slot; grant, deny-at-capacity
   with `retry_after`, or deny-wrong-model. Only the negotiation; renewal stays in-band.
2. **Work receipt** (`kwaai/ledger/receipt`): signed claim for work served → counter-signed
   acknowledgement. Model the proof and `issuedAt` rules on `process-attestation`.
3. **Intent and offer** (`network-and-intent-routing.md` §4–5): publish an intent →
   offers with terms and VCs → contract.
4. **Reputation query** (`reputation.md` §4–5): "what is your score for Eve?" → a signed
   local score. Closest upstream is `vtc/relationships/graph`, which is
   community-published, not a peer's opinion.
5. **Federated knowledge-base query** (`/kwaai/rag/1.0.0` plan): top-k ids and scores, with
   an optional VC as authorization. Probably stays a plain protocol, with only its VC
   check borrowing `credential-exchange/present`.

## What this means for the implementation plan

Folded into the implementation plan's "Decided" and "Still open" sections on 2026-10-14.

- **First task (decided):** `vtc/relationships/request` 0.2, unchanged. The spike needs no
  new task spec, only the binding. `PeerEndorsementVC` becomes a VRC.
- **Second task (decided):** `credential-exchange/query` → `present`, so a client can
  verify a peer's credentials before routing. It replaces the unverified
  `trust_attestations` blob.
- **VTC (decided):** KwaaiNet becomes one, so join, network-issued credentials, removal
  and key rotation come from upstream's `vtc/*` tasks. Still open: the VTA, how nodes
  reach the VTC, and pairwise vs public identifiers.
- **Fix the transport gaps separately and first:**
  - the caller's PeerId in unary handlers
  - leases tied to the peer that asked
  - a method and path allow-list on the proxy
  - storage authentication
  - DHT record validators

  None of these needs Trust Tasks, and each one closes a hole that exists today.
