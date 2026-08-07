# KwaaiNet Ledger — internal work credits via co-signed receipts

## Context

KwaaiNet nodes contribute GPU compute and storage to each other with **no accounting
whatsoever**. A verified sweep of the workspace found zero currency, balance, ledger,
accounting, payment, earn, or redeem primitives — no module, struct, field, or table.
The only existing resource accounting is a per-tenant storage byte cap
(`storage_rpc.rs`, itself unauthenticated) and an ephemeral in-process GPU-slot
semaphore (`capacity_lease.rs`, shipped as v0.5.5). Nodes therefore have no way to
earn recognition for what they give or to be held to what they take.

This release adds an internal credit unit backed by **co-signed work receipts**: when
one node serves another, both sign a compact record of what was delivered. Netting
those receipts pairwise *is* a bilateral mutual-credit system — real accounting, fully
decentralized, with no server, no consensus, and no double-spend surface.

**Critically, this codebase has no agreed-between-peers state of any kind today** — no
pubsub/gossip (only `pubsub: None` placeholders), no consensus, no signed DHT records,
no signed receipts, and `reputation.rs` is local-subjective by explicit design. This
feature introduces the first shared-state primitive, so it is deliberately scoped to
the one shape that needs no agreement protocol: a receipt is agreed by exactly the two
parties to it, and by nobody else.

### Decisions locked in

| Decision | Choice | Consequence |
|---|---|---|
| Scope | **Receipts only, no mint** | Fully decentralized; credits are relationship-scoped, not yet network-fungible |
| Transferability | **Work-only, non-transferable** | Internal resource accounting, not a payment instrument; minimal regulatory surface |
| Issuance | **No faucet — pure zero-sum** | Total supply is exactly 0; Sybil resistance is a non-issue this release |
| Naming | **`ledger` / `credits`** | Avoids "wallet" (already the VC store) and "token" (already an LLM token) |
| `summit-server` | **Delete from the repo** | Undeployed prototype, not in the build graph; removes the only VC issuer |
| Verida / VDA coin | **Not adopted — purge the docs** | Confirms external convertibility is permanently out, not merely deferred |

## Also in this release: two removals

Both are decided, and both are prerequisites for the ledger docs being coherent — we
are introducing a real credit unit into a repo that currently documents two *other*
aspirational value systems that are now dead.

**Remove `core/crates/summit-server/` (9 files).** It is not a workspace member
(`core/Cargo.toml:3-18` lists 14 crates, not including it), uses
`version.workspace = true` which cannot resolve outside a workspace, has **0 hits in
`Cargo.lock`**, and has no deployment artifact anywhere — i.e. it does not compile and
is not shipped. It is referenced from **13 locations outside its own directory** that
must be cleaned up alongside it: `Dockerfile.eve`, root `CLAUDE.md`, `CONTRIBUTORS.md`,
`.github/workflows/ci-kwaai-platform.yml` (which triggers on
`core/crates/summit-server/**` but only ever runs `cargo … -p kwaainet`),
`docs/{ARCHITECTURE,TODO,BOOTSTRAP_SERVER_V2_REQUIREMENTS}.md`, and five
`projects/kwaai-platform/` files including its `.code-workspace`.

Note the knock-on effect: summit-server is the **only VC issuer in the system**
(`vc_issuer.rs`). Deleting it means nothing issues Verifiable Credentials at all, so the
`did:key` vs `did:peer` verification mismatch in deferred item 3 stops being a live
production problem — though the underlying verifier limitation and the canonicalization
bug remain real and still block any *future* VC-carrying-weight work.

**Purge Verida / VDA from the docs (10 files, ~277 references, zero code).** Verified:
**no `.rs` file anywhere references Verida or VDA** — the two `package-lock.json`
"matches" are coincidental base64 substrings inside `sha512-` integrity hashes. So this
is a documentation-only change with no code impact: delete the two dedicated files
`docs/VERIDA_INTEGRATION.md` (58 refs) and `docs/VERIDA_ARCHITECTURE.md` (71 refs), and
prune references from `docs/{DATA_FLOWS,CHALLENGE_ARCHITECTURES,DEPLOYMENT_ARCHITECTURE,ARCHITECTURE,INTEGRATIONS,HIVEMIND_RUST_ARCHITECTURE}.md`,
`CONTRIBUTORS.md`, and `core/.claude/SESSION_STATE.md`.

`docs/DATA_FLOWS.md` matters most here (58 refs): it describes a "Connect Wallet" /
"Link Ethereum Wallet" / "VDA Wallet" progressive-auth flow that never had Rust behind
it. Left in place it would directly contradict this release — a reader would find three
different meanings of "wallet" in one repo, one of them an external coin we've now
declined. Removing it also collapses the naming collision from three meanings to one
(the VC credential store), which is why `ledger` remains the right name rather than
reclaiming "wallet".

## Architecture

### What a receipt is, and why it agrees itself

The hard part of metering is normally making provider and consumer agree on what was
delivered, when the provider is motivated to overstate. Here that is **already solved
by existing behavior**: every dispatch path sends `"stream": false`
(`rag_api.rs:320`, `rag_cmd.rs:1732/8273/8354`, `shard_cmd.rs:965`), and the Ollama
response body the consumer receives already carries `eval_count` / `prompt_eval_count`
— which `shard_cmd.rs:1003-1007` already parses.

So the receipt is signed over `sha256(response_body)` plus the counts parsed from that
same body. The consumer independently re-derives, from bytes it already holds, the exact
number it is about to sign. Inflating the count breaks the digest. No new
instrumentation, no trusted meter.

Three-step exchange per served request:

1. **Quote (at lease negotiation).** `capacity_lease`'s `LeaseGrant` gains a signature
   and a price, becoming a signed quote: `{lease_id, provider_did, consumer_did, model,
   price_micro_per_1k_tokens, ttl_secs, granted_at, nonce, provider_sig}`. The consumer
   verifies `consumer_did` is its own — this is what stops a grant being replayed
   against a different consumer, and is only possible after the prerequisite below.
   Price is settled *before* work, so the argument reduces to quantity alone.
2. **Claim (provider, after serving).** `WorkClaim {lease_id, request_id, provider_did,
   consumer_did, prompt_tokens, completion_tokens, response_digest, credits_owed,
   window, nonce, provider_sig}`. The response body itself goes back byte-for-byte
   unchanged — no wire change to the payload.
3. **Receipt (consumer counter-signs).** Consumer recomputes the digest and the counts,
   checks `credits_owed` against the quoted price, and counter-signs. Both sides persist.

A consumer who refuses to counter-sign leaves the provider holding an unpayable
provider-only claim. Unsigned claims are **recorded but never counted**; the provider
tracks the unsigned ratio per peer (a natural new tag alongside the existing
`LeaseOutcome` in `reputation.rs:33`) and can deny that peer future leases. Refusing to
sign costs access — the only enforcement available without an arbiter.

### Wire format: purpose-built structs, *not* Verifiable Credentials

Receipts must **not** go through `kwaai-trust`'s `VerifiableCredential`, because that
container has a live nondeterministic-signature bug: `CredentialSubject.claims` is a
`HashMap` with `#[serde(flatten)]` (`credential.rs:88-94`) and `to_signing_bytes()`
(`credential.rs:215`) signs plain `serde_json::to_string` output with no
canonicalization. `RandomState` is seeded per instance, so signing in memory works
(`Clone` preserves the seed) but verifying a VC *deserialized from JSON* gets a fresh
seed — **any VC with ≥2 claims can fail verification at random**, and both VC types that
were being issued (before summit-server's removal, above) had exactly 2. No test covers
this. Single-claim VCs happen to be deterministic today, which is the only reason this
has not already bitten anyone.

Instead, in the new crate: **msgpack via `rmp_serde`** (already used throughout —
`capacity_lease.rs:426`, `inference_mux.rs:320`, so no new dependency), with two rules
that give canonicalization for free rather than requiring a canonicalization algorithm:

- **No maps, no `HashMap`, no `serde(flatten)`.** Fixed-order scalar fields only, so
  encoding is deterministic by declaration order.
- **No floats.** Integers only — tokens, bytes, milliseconds, micro-credits. A credit
  is a `u64` in its smallest unit. (`throughput: f64` is fine to *announce*; never to
  *sign*.)

### Hard prerequisite: handlers cannot currently identify their caller

`CallUnaryRequest.peer` (`p2pd.proto:193`, `required bytes peer = 1`) **is** populated
by the daemon with the authenticated caller, but `persistent.rs:157-158` clones only
`req.proto` and `req.data` and discards it, and `add_unary_handler` types handlers as
`F: Fn(Vec<u8>) -> Fut` (`persistent.rs:333`). **No unary handler in this codebase can
know who is calling it.** The mux path throws the same information away:
`inference_mux.rs::read_p2pd_stream_info` reads the StreamInfo prologue and discards it,
though `kwaai_p2p_daemon::stream::parse_stream_info` already decodes it.

Without this, every receipt would bind to a *self-declared* counterparty — exactly the
`storage_rpc::CreateTenantPayload.peer_id` hole (self-declared, never checked against
the stream peer, never signed). Fix it **additively**: add
`add_unary_handler_with_peer(proto, Fn(Vec<u8>, PeerId), balanced)` and implement the
existing `add_unary_handler` in terms of it, discarding the peer. There are **9 existing
call sites** (`node.rs` ×4, `shard_cmd.rs` ×4, `storage.rs` ×1) that must keep compiling
untouched.

### Key security must land with Phase 1, not later

`~/.kwaainet/identity.key` is written by a bare `std::fs::write` (`identity.rs:80`) —
**plaintext, no chmod (typically 0644, world-readable), no passphrase**. `NodeIdentity`
has no signing method at all, and its `keypair` field carries
`#[allow(dead_code)] // retained for Phase 4 peer endorsement signing`.

The non-obvious point: receipts are **durable and retroactively monetizable**. A key
stolen while receipts are "worth nothing" can be used to accumulate forged co-signed
receipts that acquire value the moment any redemption exists. Key protection therefore
ships with the *first signature*, not the first balance. Minimum bar: `0600` on write,
encryption at rest (also satisfies the project's own existing rule in
`projects/kwaai-trust/CLAUDE.md`, "Do not store credentials unencrypted on disk", today
violated), atomic temp-then-rename writes, and a **key-epoch field recorded in every
receipt** so rotation doesn't silently invalidate history (cheap now, a migration later).

## Phases

Mirrors the shape the Capacity Lease feature just shipped with (foundational types →
first transport → second transport → DHT flag → deferred).

The **two removals have no dependency on any phase** — they are pure deletions plus
reference cleanup, so they can land first as their own commit (cleanest: it makes the
subsequent ledger diffs easier to read, and gets a docs-coherence change out of the way
before adding a third meaning of "value" to the repo).

**Phase 0 — prerequisites + types. No network, no money.**
- Authenticated-peer plumbing: `add_unary_handler_with_peer` in
  `kwaai-p2p-daemon/src/persistent.rs`; use `stream::parse_stream_info` in the mux
  server instead of discarding it.
- `NodeIdentity::sign(&[u8]) -> [u8; 64]` + verify helper; `0600`; encryption at rest;
  atomic writes. Retires the `#[allow(dead_code)]` on `keypair`.
- New leaf crate **`core/crates/kwaai-ledger`** (added to the 14-member workspace list
  in `core/Cargo.toml:3-18`): `SignedLeaseGrant`, `WorkClaim`, `Receipt`, `receipt_id`
  (= `sha256(canonical_bytes)`), credit arithmetic in `u64` micro-credits.
- Tests: the **sign → serialize → deserialize → verify roundtrip-stability property
  test that `kwaai-trust` never had** (this is the test whose absence hid the VC bug),
  plus forgery/replay/tamper rejection.
- Shippable surface: `kwaainet ledger verify <file>`.

**Phase 1 — `mux://` receipts, persisted locally. This phase alone is a complete
bilateral mutual-credit system.**
- Extend `MuxFrame` additively with `WorkClaim` / `ReceiptAck` (same
  `#[serde(default)] Option<…>` compatibility pattern `lease_id` already established at
  `inference_mux.rs:57`).
- Sign the `LeaseGrant` quote in `capacity_lease.rs::try_grant`; keep the lease table
  **ephemeral** — the receipt is the durable artifact, so the audit trail survives a
  restart without making the admission gate crash-consistent.
- Persist to `~/.kwaainet/ledger.db` via `rusqlite 0.31` w/ `bundled` (already used by
  `kwaai-rag` and `kwaai-storage`; declared per-crate, not in workspace deps), with
  `UNIQUE(receipt_id)` for idempotent replay rejection.
- `kwaainet ledger show` — earned / spent / net per peer.

**Phase 2 — `p2p://` unary transport.** New `/kwaai/work-receipt/1.0.0`, same types,
mirroring how capacity-lease sequenced mux-then-unary.

**Phase 3 — DHT capability flag + price discovery.** `receipts_v1: bool` and
`price_micro_per_1k_tokens: i64` in the `DHTServerInfo` fields map — safely extensible
because unknown map keys are silently ignored by legacy Hivemind clients
(`node.rs:234-238`), the same pattern `lease_v1` and `vpk` already use. Advisory hints
only, never authoritative.

## Explicitly not in scope, and not guaranteed

- **No mint, no balances, no fungibility across the network.** Credits are pairwise.
  Earning from A does not let you spend at B this release.
- **No consensus, chain, gossip, or replicated state.** The DHT is unusable as ledger
  transport: records are author-mutable and unsigned (announcing arbitrary
  `trust_attestations` JSON already inflates a trust badge — `crawler.rs:333` literally
  counts array length), TTL is ~360s, and there is no broadcast primitive at all.
- **No proof of correct computation.** Receipts prove delivery and mutual agreement,
  nothing about output quality.
- **No transfers, no faucet, no dispute arbiter.**
- **No external convertibility — now permanently, not just deferred.** With Verida/VDA
  declined, credits are an internal accounting unit with no path to fiat or any
  on-chain token. This is the decision that keeps the design small: convertibility
  would have required transfers, real double-spend hardness, an AML posture, and
  probably consensus rather than pairwise receipts.
- **No W3C VC conformance / JSON-LD / SD-JWT / OpenID4VC / DID resolution / KYC.**
  Per OWF's own guidance for closed internal currencies, these are correctly skipped.
- **Storage credits are blocked**, not merely deferred: `storage_rpc.rs` accepts a
  self-declared `peer_id` never checked against the stream peer, and `tenant_id` is a
  bare bearer capability. You cannot bill for storage anyone holding a UUID can write to.

## Verification

- Per phase: `cargo test -p kwaai-ledger`, `cargo test -p kwaainet`,
  `cargo clippy -p kwaainet --all-targets`, `cargo fmt --check` — the full trio, since
  these phases touch shared code (`persistent.rs`, `identity.rs`, `capacity_lease.rs`)
  that other crates' tests exercise. Note two **pre-existing** environmental failures in
  `grpc_server::tests` (port 8093 held locally) that are unrelated to this work.
- The load-bearing unit test: a receipt that survives sign → msgpack → bytes → decode →
  verify **across process boundaries** (not just in-memory), which is precisely the
  property `kwaai-trust` lacks.
- Adversarial unit tests: tampered token count vs. digest; receipt replayed under a
  different `lease_id`; quote replayed against a different `consumer_did`; provider-only
  claim must never count toward earned.
- End-to-end on the two live remote peers already used to validate Capacity Lease
  (metro-linux, metro-win, both on `feat/capacity-lease` at v0.5.5): serve real
  inference between them and confirm both sides independently persist byte-identical
  receipts with matching `receipt_id`, and that netting agrees in both directions.
- Backward compat: a peer without `receipts_v1` must serve and be served exactly as
  today, with no receipt and no error — the same graceful-degradation bar Capacity
  Lease was held to.
- For the two removals: `cargo build --workspace` and the full test suite must pass
  unchanged (expected, since summit-server was never in the graph — a clean build is
  the proof it truly wasn't); `grep -ri "summit-server\|summit_server"` and
  `grep -ri "verida\|VDA"` must return nothing outside `Cargo.lock` hash noise; and
  `.github/workflows/ci-kwaai-platform.yml` must still parse and trigger correctly
  after its `core/crates/summit-server/**` path filter is removed.

## Critical files

- `core/crates/kwaai-ledger/` — **new crate**; add to `core/Cargo.toml:3-18` members
- `core/crates/kwaai-p2p-daemon/src/persistent.rs` — `:157-158`, `:326-335`; the
  authenticated-peer prerequisite that everything else depends on
- `core/crates/kwaai-cli/src/identity.rs` — `:80` key perms/encryption, add `sign()`
- `core/crates/kwaai-cli/src/capacity_lease.rs` — `LeaseGrant` → signed quote; price
  and receipt hooks in `try_grant`
- `core/crates/kwaai-cli/src/inference_mux.rs` — `MuxFrame` receipt frames;
  `read_p2pd_stream_info` must stop discarding StreamInfo; `call_ollama_local`'s
  verbatim body is the digest/token-count source
- `core/crates/kwaai-cli/src/node.rs` — `DHTServerInfo` `receipts_v1` flag (Phase 3)
- `core/crates/kwaai-cli/src/reputation.rs` — unsigned-claim ratio tag

Removals: `core/crates/summit-server/` (delete) plus its 13 referencing locations
(`Dockerfile.eve`, `CLAUDE.md`, `CONTRIBUTORS.md`,
`.github/workflows/ci-kwaai-platform.yml`, 3 `docs/`, 5 `projects/kwaai-platform/`);
`docs/VERIDA_INTEGRATION.md` and `docs/VERIDA_ARCHITECTURE.md` (delete) plus Verida
references in 8 more files, `docs/DATA_FLOWS.md` most importantly.

## Deferred — found during this work, not part of this release

1. **5 infinite-spin bugs from unhandled stdin EOF.** `read_line` returns `Ok(0)` at
   EOF, which an `is_err()` check misses; the empty line then hits a `continue` or a
   non-breaking `_` arm and re-prompts forever at ~100% CPU. Sites: `rag_cmd.rs:1570`
   (`cmd_chat` REPL — reproduced live, burned 47 min of CPU) and `:3573`, `:3692`,
   `:3810`, `:3887` (four `graph dedup` prompt loops). **Ctrl-D in a real terminal
   triggers these too**, not just piped input. Recommended fix: one shared helper taking
   `&mut impl BufRead` returning `Line(String) | Eof`, matching the existing house idiom
   `Ok(0) | Err(_) => break` (`ollama_proxy.rs:342`); that makes it one fix site instead
   of five and is unit-testable against `&b""[..]`. Note `rag_cmd.rs` has no
   `#[cfg(test)]` module at all today.
2. **`kwaai-trust` nondeterministic VC signatures** (`credential.rs:88-94` + `:215`) —
   described above. Must be fixed before any VC carries weight; fixing it invalidates
   already-issued signatures, so it needs a compatibility window. Single-claim VCs are
   deterministic today, which is a usable short-term mitigation.
3. **The VC verifier resolves `did:peer:` only** (`verify.rs:130`), so any VC issued
   under a `did:key:z…` returns `signature_valid: Some(false)`. This *was* live —
   summit-server issued exactly such VCs — and is defused by deleting it, but the
   limitation remains for any future issuer. Relatedly and independently:
   `TrustScore::from_credentials` (`trust_score.rs:49`) **never calls `verify()` at
   all**, so unsigned or forged VCs score identically to signed ones. Both must be
   fixed before a VC ever gates anything of value.
4. **Any future mint is new work, not an extension.** With summit-server deleted there
   is no central component and no VC issuer left in the system — worth recording so a
   later fungible-credits phase isn't planned on the assumption that one exists.
5. **`CredentialStore` ignores `$KWAAINET_HOME`** (`storage.rs:127-131`) unlike
   `config::kwaainet_dir()`, so identity key and credential store diverge when it's set;
   its `save()` is also a non-atomic `std::fs::write` (`storage.rs:43`), contradicting
   its own stated requirement NFR-T3.
