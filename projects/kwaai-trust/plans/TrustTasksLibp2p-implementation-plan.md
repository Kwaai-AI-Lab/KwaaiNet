# trust-tasks-libp2p — implementation plan (Phase 1 spike → upstream PR)

Drafted 2026-10-14. This expands Phases 1–2 of
[`DTGWG-TrustStack-plan.md`](DTGWG-TrustStack-plan.md) into buildable work. That plan stays
the record of *why* (crate-level integration, `did:key` VID, the wire format); this one says
*what to build, in what order, and how we know it works*.

Upstream facts were read from `trustoverip/dtgwg-trust-tasks-tf` at `15b91d1f`
(2026-10-14); KwaaiNet facts from `origin/main` at `913e0bc4`.

## Where things stand

- **Approval:** verbal only (Glenn Gore, 2026-10-01). Revision 3 of the proposal was posted
  to discussion #248 on 2026-10-14 asking for written confirmation of six points. **That reply
  gates the upstream PR (Phase 2), not the spike.** The spike happens in a fork and costs
  nothing upstream if the answer changes.
- **Upstream pace:** `trust-tasks-rs` is **0.31.1**; ~40 releases since 2026-09-26, a breaking
  minor every 1–3 days. But the parts this binding touches are quiet:
  `trust-tasks-rs/src/transport.rs` last changed 2026-08-26 (#274); `consume.rs`,
  `replay.rs`, `freshness.rs` are unchanged since August. The churn is in generated spec
  types. So the transport glue is stable; what moves is the version number we must track.
- **Toolchain:** upstream MSRV is **1.95**; our `rust-toolchain.toml` still pins **1.93.1**.
  The crate builds in the fork on its own toolchain, but nothing in KwaaiNet can depend on it
  until the bump lands.
- **No libp2p anything upstream** — no slug, crate, issue or code.

## Corrections to the parent plan

1. **Target framework version.** The parent plan says the binding "targets framework 0.7.0".
   Upstream CI (`scripts/build-registry.mjs → checkTargetFrameworkForm`) requires
   `targetFrameworkVersion` to name a *published* release with an envelope schema under
   `specs/_framework/`; those stop at **0.6** (0.7.0 is a Working Draft). Every existing
   binding declares `"0.5.0"`. The spec must declare a published release in three-part form
   and state it in bold in the prose (`Targets **framework `0.6.0`**`) — the check compares
   the two. Which one (0.5.0 like its siblings, or 0.6.0) is a question for Glenn.
2. **No Go daemon path.** The parent plan's §9.1.1 item 8 note mentions "any local client of
   the `kwaai-p2p-daemon` control socket". The Go p2pd mode is gone (`node.rs` ignores
   `native_p2p: false`); what remains is our Rust control socket, and the note should say so.
3. **DCO.** Upstream's `CLAUDE.md` makes a `Signed-off-by:` trailer mandatory on every commit,
   checked in CI — in addition to the OWF/ToIP CLA. Every commit in the fork needs it.

## The crate: `trust-tasks-libp2p`

Lives at `trust-tasks-libp2p/` in the upstream workspace (fork first), spec at
`bindings/libp2p/0.1/spec.md`. Layout modelled on `trust-tasks-tsp` (modules `error`,
`handler`, `inbound`, `pack`).

### Dependencies

All crates.io, all licences on upstream's `deny.toml` allow-list, no git sources:

| Crate | Why | Note |
|---|---|---|
| `trust-tasks-rs` | `TransportHandler`, `consume_inbound`, document types | `{ path = "../trust-tasks-rs", version = "0.31" }` like the siblings |
| `libp2p-identity` | `PeerId`, `PublicKey` | `default-features = false, features = ["ed25519", "peerid"]`; secp256k1/ecdsa behind features |
| `futures` | `AsyncRead`/`AsyncWrite` — our `RawStream` is `libp2p::swarm::Stream`, which is futures-io | no tokio dependency; the crate stays runtime-agnostic |
| `unsigned-varint` | frame length prefix | `features = ["futures"]` |
| `prost` | the two-field `TrustTaskFrame` | **derive only, hand-written struct — no `build.rs`, no `protoc`** |
| `flate2` | DEFLATE | `default-features = false, features = ["rust_backend"]` (miniz_oxide, pure Rust) |
| `multibase` | `did:key` encode/decode | or hand-roll base58btc + the multicodec prefix; decide by dep weight |
| `futures-timer` | per-frame read timeout without a runtime | |
| `thiserror`, `serde_json`, `chrono` | as the siblings | |

Optional feature `swarm` (later, not the spike): a ready-made `libp2p-stream` endpoint for
stacks with no stream layer of their own. It is the only part that would pull in
`libp2p-swarm`, so it stays off by default.

### Public API (sketch)

```rust
pub const BINDING_URI: &str = "https://trusttasks.org/binding/libp2p/0.1";
pub const PROTOCOL_ID: &str = "/trust-tasks/0.1";

pub enum Mode { Direct, Relayed }

pub mod vid {
    /// did:key for an identity-multihash PeerId (Ed25519, secp256k1, ECDSA); None for RSA.
    pub fn peer_id_to_did_key(peer: &PeerId) -> Option<String>;
    /// The public key a did:key / did:peer:0 / did:peer:2 names, decoded locally (no I/O).
    pub fn did_public_key(did: &str) -> Option<PublicKey>;
}

pub mod frame {
    pub enum Compression { None, Deflate }
    pub struct Limits { pub max_frame: usize, pub max_document: usize, pub read_timeout: Duration }
    pub async fn write_frame<W: AsyncWrite + Unpin>(w: &mut W, document: &[u8], c: Compression) -> Result<(), FrameError>;
    /// Rejects an oversize length prefix before allocating, and an oversize document while inflating.
    pub async fn read_frame<R: AsyncRead + Unpin>(r: &mut R, limits: &Limits) -> Result<Option<Vec<u8>>, FrameError>;
}

/// One per stream: the parties libp2p authenticated for it.
pub struct Libp2pHandler { local_vid: String, remote_vid: Option<String>, mode: Mode }
impl TransportHandler for Libp2pHandler {
    fn binding_uri(&self) -> &str { BINDING_URI }
    fn derive_parties(&self) -> TransportContext { /* issuer = remote, recipient = local */ }
    fn resolve_parties<P>(&self, doc: &TrustTask<P>) -> Result<ResolvedParties, ConsistencyError>;
}

/// Mirrors `TspConsumer`: in-memory replay guard + consequential freshness by default.
pub struct Libp2pConsumer { /* checks, limits */ }
impl Libp2pConsumer {
    pub fn new(local: &PeerId) -> Self;
    pub fn with_replay_guard(self, g: Arc<dyn ReplayGuard>) -> Self;
    pub fn with_limits(self, l: Limits) -> Self;
    /// Read frames until half-close; run each through consume_inbound; write the response
    /// (or error) back on the same stream.
    pub async fn serve<S, P, R, F, Fut>(&self, stream: S, remote: PeerId, mode: Mode, handler: F) -> Result<(), ServeError>;
}

/// Producer side: stamp the issuer if absent, frame, write; optionally read one response.
pub async fn send<S, P: Serialize>(stream: &mut S, local: &PeerId, doc: &mut TrustTask<P>, c: Compression) -> Result<(), SendError>;
pub async fn request<S, P: Serialize, R: DeserializeOwned>(stream: S, local: &PeerId, doc: TrustTask<P>, limits: &Limits) -> Result<TrustTask<R>, RequestError>;
```

### Behaviour that needs care

- **`resolve_parties` override — the one place we diverge from the default.** The default
  compares in-band and transport VIDs by exact string. Ours first checks the default; if the
  strings differ and the in-band issuer is a `did:key`, `did:peer:0` or `did:peer:2` whose
  single authentication key equals the PeerId's key (`vid::did_public_key`), the parties are
  consistent and the **in-band string** is returned (§4.8.1: in-band is authoritative).
  Anything else is `IssuerMismatch` → `identity_mismatch`, which the default
  `TransportHandler::reject` already routes to the transport-authenticated peer.
- **Lifecycle (§4.12, §11.7 — MUST).** Stream close, reset and connection drop map to *no*
  state. A frame that cannot be decoded is a binding error: if the bytes parse as a document
  with an `id`, answer `malformedRequest`; otherwise reset the stream. Late responses and
  `trust-task-control` documents go on a fresh stream, correlated by `id`/`threadId`.
- **Limits.** 1 MiB for the frame and for the inflated document; the decoder reads through
  `take(max_document + 1)` and fails on the extra byte. An unknown `compression` value is a
  binding error, not "treat as NONE".
- **When to compress.** `NONE` below a threshold (start 1 KiB; the spike measures it).
  The API makes the caller choose, so a document mixing secrets with attacker-supplied
  content is never compressed by default.
- **Proofs.** `ProofPolicy::Verify(&Verifier::for_did_key())` for `did:key` issuers. A
  `did:peer:0` *issuer* is not in the spike: upstream's did:peer:0 resolution goes through
  did:key and may return `did:key:` verification-method ids, which would fail
  `sign_trust_task`'s issuer = VM-DID check. Test it before the spec promises it.

### The spec: `bindings/libp2p/0.1/spec.md`

Front matter `slug: libp2p, version: "0.1", status: draft, targetFrameworkVersion` (see
correction 1), `bindingURI`, no `envelopeType` (as `push/0.1` — we have no envelope), authors.
Sections follow `bindings/tsp/0.1/spec.md`: 1 Binding URI · 2 Document carriage (protocol ID,
the frame, compression, limits) · 3 Identity mapping (PeerId → did:key; accepted in-band forms)
· 4 Error mapping + 4.1 Lifecycle mapping table · 5 Modes (direct, relayed: both end-to-end
Noise; default relay limits 128 KiB / 2 min can truncate) · 6 Responses (same stream; late
responses on a fresh stream) · 7 Security profile — all eight §9.1.1 items, **per mode** — +
7.1 duplicate-execution record · 8 Discovery · 9 Versioning · 10 References. Plus the
`website/assets/bindings.js` entry, or `checkBindingRegistry` fails.

## KwaaiNet side (spike branch)

1. **Toolchain bump PR, first and separately** (parent plan, Immediate action 1): pin
   ≥ 1.95, every release job green (CUDA, musl, riscv, Jetson, Windows MSVC, cargo-dist).
   Needs Darren; nothing below compiles against the crate until it lands.
2. **`kwaai-p2p`: a relayed flag on streams.** `raw_stream::Behaviour` today discards the
   connection addresses (`handle_established_*_connection` takes `_local_addr`,
   `_remote_addr`) and the `ConnectionId`. Record `relayed` per `ConnectionId`
   (`addresses::is_circuit(local_addr) || is_circuit(remote_addr)` inbound — an inbound
   circuit's `send_back_addr` is a bare `/p2p/<peer>`, so `local_addr` is the tell — and
   `is_circuit(addr)` outbound); add `relayed: bool` to `InboundStream`; carry it in
   `OpenResult` and out of `NetworkHandle::open_raw_stream`. Note outbound always uses
   `connections[0]`, which may still be the relayed one after a DCUtR upgrade. Useful on its
   own, so it can land on `main` ahead of the rest.
3. **`kwaai-cli/src/trust_tasks.rs`**, modelled on
   `inference_mux.rs → start_native_inference_mux_server`: `accept_streams([PROTOCOL_ID])`,
   one task per stream calling `Libp2pConsumer::serve`, handlers converting to our own types
   at the boundary (parent plan §2 caveat). Dependency on the fork by **git rev**, spike
   branch only.
4. **First real Trust Task** (exit criterion e) — see Decisions.

## Order of work

| # | Step | Where | Done when |
|---|---|---|---|
| 0 | Fork the upstream repo; branch pinned to a recorded upstream SHA; DCO sign-off configured | GitHub | fork CI green on an untouched branch |
| 1 | `vid` module + tests (Ed25519/secp256k1/ECDSA round trips, RSA → None, did:peer:0/2 key decode) | fork | unit tests |
| 2 | `frame` module + tests (round trip with and without DEFLATE, document bytes identical after inflate; oversize prefix; decompression bomb; unknown compression value; truncated frame; timeout) | fork | unit tests |
| 3 | `Libp2pHandler` + `resolve_parties` override + tests (match, omitted issuer filled, mismatch → `identity_mismatch` routed to the transport peer) | fork | unit tests |
| 4 | `Libp2pConsumer::serve` / `send` / `request` over an in-memory duplex (`futures::io` pipe); replay and freshness via upstream's guard | fork | end-to-end test in the crate, no libp2p swarm |
| 5 | Spec draft + `bindings.js` entry | fork | `npm run check-bindings` green |
| 6 | Toolchain bump | KwaaiNet PR | merged |
| 7 | Relayed flag in `kwaai-p2p` + test extending `tests/relay.rs` (`a_second_peer_can_dial_us_through_the_circuit` is the template) | KwaaiNet PR | merged |
| 8 | `trust_tasks.rs` + two-node test: direct, then through a relay configured with libp2p's default limits (`relay_max_circuit_bytes` 128 KiB, `relay_max_circuit_duration` 2 min) | KwaaiNet spike branch | exit criteria a–c |
| 9 | Measurement: document sizes and a latency split (network, proof verify, JSON parse, compression) | spike branch | threshold chosen; exit criterion c2 |
| 10 | `dtg-credentials` issue → verify, and one upstream vector | spike branch | exit criterion d |
| 11 | First real Trust Task end to end | spike branch | exit criterion e |
| 12 | Go/no-go writeup → Phase 2 (upstream PR) only with Glenn's written confirmation | plans/ | decision recorded |

Steps 1–5 need no KwaaiNet change and can start now; 6 and 7 can run alongside them.

## Keeping up with upstream

Rebase the fork onto upstream `main` weekly, and before the PR. Because `transport.rs` and
`consume.rs` are quiet, a rebase is normally a version bump in our `Cargo.toml`; the CI
`publication-order` job and release-plz `semver_check` will say if not. Our spike branch
pins the fork by rev, so an upstream release never breaks KwaaiNet without a deliberate bump.

## Risks

- **No written confirmation.** If Glenn's reply changes the slug, VID form or wire format,
  steps 1–5 change shape. Kept cheap by building in a fork.
- **ed25519-dalek 2 and 3 side by side** once `dtg-credentials` (Affinidi) comes in at step
  10; keys cross as raw 32-byte seeds. Only step 10 is exposed, so it stays last.
- **Affinidi's rustls defaults to aws-lc-rs**; ours uses ring. Keep the resolver's network
  features off; `cargo tree -d` must show one `affinidi-data-integrity`.
- **did:peer:0 issuer ids** (above): untested; kept out of the spike's promises.
- **Relay limits** can cut a large document; the spec states it, and step 8 tests at the
  defaults.

## Decided (2026-10-14)

Mapping all 634 upstream tasks onto our node-to-node interactions
([`TrustTasks-KwaaiNet-interaction-map.md`](TrustTasks-KwaaiNet-interaction-map.md))
settled the first two:

1. **KwaaiNet becomes a Verifiable Trust Community (VTC).** `summit-server` was our first,
   abortive attempt at one. This puts about 155 upstream `vtc/*` and `rooms/*` tasks in
   reach: joining the network, network-issued node credentials, removing a node, key
   rotation, and the relationship graph EigenTrust needs.
2. **First real Trust Task (exit criterion e): peer endorsement, using upstream's
   `vtc/relationships/request` 0.2 unchanged.** A member asks another member for a
   Verifiable Relationship Credential (VRC) and gets the signed VRC back in the response;
   a refusal is a `trust-task-error` with `vtc/relationships/request:declined`. The spike
   therefore needs no new task spec, only the binding. Consequences:
   - `PeerEndorsementVC` maps to a **VRC**, not the VSC `endorses/1` the parent plan
     named. A VRC is also what `vtc/relationships/{publish, graph}` build the community's
     trust graph from (parent plan updated).
   - **Second task:** `credential-exchange/query` → `present`, so a client can check a
     peer's credentials before routing to it. It replaces the `trust_attestations` blob in
     announcements, which no reader verifies today.
   - **Capacity lease** has no upstream task. If it becomes one, it will be a Kwaai-authored
     spec covering only the negotiation; renewal stays inside the existing exchange.

## Still open

1. **Do we need a Verifiable Trust Agent (VTA), and where?** To discuss. The facts:
   - In OpenVTC's reference implementation (`OpenVTC/verifiable-trust-infrastructure`,
     Apache-2.0, Rust 1.95), **a VTC always sits on top of a VTA**. The VTA mints the
     VTC's keys at first boot (the `vtc-host` DID template), and one VTA can host many
     VTCs. A VTA custodies keys, DIDs and access policy for one organisational identity,
     optionally inside an AWS Nitro enclave.
   - **A community VTA** is required if we run OpenVTC's `vtc-service`. The real choice is
     to adopt OpenVTC (VTA and VTC) or to build our own VTC. Adopting matches our crate-level
     rule, since we already depend on their `dtg-credentials`.
   - **A VTA per node** looks unnecessary: a node's key stays in `identity.key`, and a
     key-custody service inside every node is a lot of machinery for one key. It becomes
     relevant only for operator multi-device custody, key rotation without a new identity,
     or TEE attestation (`vta/attestation/*`).
   - **Where it runs and who operates it:** a Kwaai-hosted server, or the bootstraps.
2. **How nodes reach the VTC.** OpenVTC's SDK ships REST and DIDComm clients, and the
   `vta/*` specs name HTTPS, DIDComm and TSP. A publicly hosted VTC can be reached over
   the existing HTTPS binding, which keeps our
   libp2p binding for node-to-node traffic. Only a VTC hosted *on* the bootstraps and
   reached over libp2p would need our binding.
3. **Pairwise vs public identifiers.** `vtc/join-requests/submit` declares the applicant
   `identifierScope: pairwise`. Our VID, `did:key` derived from the PeerId, is public by
   construction. The options:
   - join with a separate pairwise key, bound to the PeerId by a credential;
   - give the prose justification the spec requires for a `public` identifier.

   Ask Glenn.
4. **Who builds steps 1–5** (Darren knows the upstream crates; anyone can do 7).
5. **Target framework version** — ask Glenn alongside the written confirmation.

## Verification

- Fork: `cargo test --workspace --all-targets`, clippy `-D warnings` (default and
  `--all-features`), `cargo deny`, `cargo package --workspace`, `npm run check-bindings`,
  commit-lint on PR titles, DCO.
- KwaaiNet: `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test -p kwaainet`, `cargo test -p kwaai-p2p`; the two-node and relay tests run in
  CI on loopback (no fleet needed); then a fleet exchange with metro peers over p2p.
