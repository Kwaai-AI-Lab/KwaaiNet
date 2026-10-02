# kwaai-trust — adopt the DTGWG trust stack (plan, re-synced 2026-10-01)

Diagrams (data flow, entities, sequences): [`design/DTGWG-TrustStack-diagrams.md`](../design/DTGWG-TrustStack-diagrams.md).

## Context

Kwaai proposed a libp2p binding for Trust Tasks to the LF ToIP Decentralized Trust
Graph WG (Kwaai-AI-Lab/KwaaiNet#113; upstream discussion #248). Glenn Gore (maintainer)
has given **verbal approval on a conference call of the revised proposal**: a libp2p
binding maintained in-tree upstream, with the libp2p ↔ TSP/DIDComm *bridge* as the
headline. Decisions for this plan (Reza, 2026-10-01):

- **Scope:** contribute the binding **and adopt the DTGWG identity/VC stack**, retiring
  our home-grown DID/VC code. The bridge is designed, not built.
- **Timing:** spike now (Q4 2026), node integration Q1 2027 as told to the WG. Rung 1
  (2026 committed scope) is untouched.

The re-sync shows the August picture was wrong in ways that change the work. This plan
supersedes the trust sections of `PublicRelease-plan.md` and corrects
`proposals/TrustTasksLibp2pBinding-response.md`.

## What the re-sync found

**Upstream** (read 2026-10-01 via gh api / crates.io):

| | August record | Now |
|---|---|---|
| `trust-tasks-rs` | "0.9.0" (wrong — was 0.17.3) | **0.26.0**, ~3 releases/day, breaking minor bumps through September |
| Crates | rs + https/didcomm/tsp/proof/capability-client | + `trust-tasks` facade, `-didcomm-v1`, `-ceremony`; Go/Dart/TS clients |
| Framework | 0.2 | **WD 0.7.0**; SPEC generated from `dtgwg-trust-tasks-spec` |
| libp2p | — | **nothing**: no slug, code or issue; **#248 has no maintainer reply on GitHub** |
| Toolchain | — | MSRV **1.95** (trust-tasks edition 2021; Affinidi + `dtg-credentials` edition 2024) |
| Credentials | VRC / PHC / r-cards | **dtgwg-cred-spec WD 0.6.0**: VMC VRC VDC VIC VPC VSC VAC; PHC = a VMC profile; r-card removed (a VDS). VC 2.0, `DataIntegrityProof` `eddsa-jcs-2022`, `issuerScope` required |
| Credential code | Spruce `ssi` | **`dtg-credentials` 0.13.0 (OpenVTC)** on the **Affinidi** stack; `ssi` has no did:peer → **drop `ssi`** |

**Binding mechanics:** one sync trait, `trust_tasks_rs::TransportHandler`
(upstream `dtgwg-trust-tasks-tf/trust-tasks-rs/src/transport.rs`); "transports that perform I/O do it outside the trait".
Existing bindings are hard-wired to one Affinidi crate (handler / pack / inbound / error,
`BINDING_URI`, `{type, document}` envelope); `push` is a binding with no crate.
Registration is a PR adding `bindings/libp2p/<M.m>/spec.md` + `website/assets/bindings.js`;
OWF CLA 1.0; conventional commits. SPEC §9.1 (carriage, correlation, lifecycle MUST),
§9.1.1 (VID mapping + per-mode allowances when `proof` may be omitted), §4.8.1
(transport identity only cross-checks), §4.12/§11 (task lifecycle).

**Our side** (verified on `main` `97d0b1e2`):

- **`rust-toolchain.toml` pins `channel = "1.93.1"`** — CI and release builds use it
  despite `dtolnay/rust-toolchain@stable`. No 1.95 dependency builds until it is bumped.
- `did.rs` emits `did:peer:<base58 PeerId>` — **not conformant did:peer**. Our #248
  comment ("KwaaiNet already issues did:peer, no change needed") is false.
- `credential.rs` signs `serde_json::to_string` over a flattened `HashMap` → **≥2-claim
  VCs fail verification at random**; no round-trip tests.
- `verify.rs` resolves only that form (did:key issuers always fail), never checks
  `verificationMethod` against `issuer`; `TrustScore::from_credentials` never verifies.
- `trust_attestations` are announced unverified and consumed by nothing.
- Unary handlers drop the caller PeerId; **raw streams carry it**
  (`kwaai-p2p/src/handle.rs` `open_raw_stream`/`accept_streams`, `raw_stream.rs`
  `InboundStream{peer,proto,stream}`) but expose no direct/relayed flag.
- `identity.key` 0644, plaintext wallet ignoring `$KWAAINET_HOME`; `kwaai-wasm` is a stub;
  trust docs claim commands/properties that do not exist.
- `summit-server` (non-member; only `ci-kwaai-platform.yml` builds it) issues
  BindingVCs tying the old DID form to passkey `did:key`s; Darren's
  `chore/remove-summit-server-and-verida` and `feat/kwaai-ledger` (146 behind main)
  are pending.

## Approach

### 1. Identity — `did:key` as the derived VID, routing DID optional
- The binding maps any PeerId with an identity-multihash key (Ed25519, secp256k1, ECDSA —
  what Go/JS stacks use too) to **`did:key`**: deterministic, exact-string comparable,
  verified offline by upstream's `trust_tasks_proof::affinidi::Verifier::for_did_key()`
  without the resolver-cache dependency.
- The binding overrides `resolve_parties` so an in-band `did:key`, `did:peer:0` or
  `did:peer:2` is accepted when its single authentication key equals the PeerId's key
  (local decode, no I/O). did:peer:4 is unsupported by Affinidi (`affinidi-did-common`
  supports numalgo 0 and 2); the standalone `did-peer` crate is stale.
- **`did:peer:2`** (keys + a service entry with multiaddrs) is the optional routing DID
  Glenn recommended, used by the bridge; addresses otherwise stay in signed peer records
  (`kwaai_p2p::peer_record`, `announce.rs::signed_dial_addrs`).
- Correct our #248 statement in writing. These helpers ship **in `trust-tasks-libp2p`**
  (§2); `kwaai_trust::{peer_id_to_did, did_to_peer_id}` are replaced by calls to it, with
  only a read-only parser for the legacy form left in our tree. RSA bootstraps get no VID.

### 2. Integration model — one contributed crate that we use, co-maintained by Kwaai

**Principle: all Trust Tasks logic lives in a crate in the upstream workspace; KwaaiNet
holds only a seam that never changes when Trust Tasks does.** Upstream maintainers carry
it through their lockstep releases (their ask-4 commitment); Kwaai co-maintains it
(CODEOWNERS for the crate, one of our four WG volunteers as reviewer) and owns libp2p-side
changes. Our re-sync cost becomes a version-group bump, not code.

Options considered:

| Model | What lives in our repo | Re-sync cost | Verdict |
|---|---|---|---|
| Pure shim upstream + our carrier/handlers (previous draft) | carrier, envelope handling, consumer wiring | every trust-tasks change touches our code | rejected |
| Crate owns a libp2p `NetworkBehaviour`/Swarm | nothing, but swarm coupled to their libp2p version | every libp2p bump must match ours (we patch kad + multistream-select) | rejected |
| **Complete crate over a stable seam: `(PeerId, AsyncRead+AsyncWrite stream)`** | ~one small module: open/accept a stream, hand it over | version bump only | **chosen** |

**`trust-tasks-libp2p` (upstream workspace, `bindings/libp2p/0.1`) contains everything:**
  - `Libp2pHandler` implementing `TransportHandler`, incl. the `resolve_parties` override;
  - VID mapping PeerId ↔ `did:key` (+ acceptance of matching `did:peer:0/2`) — so
    `kwaai-trust` keeps **no** DID code of its own;
  - the frame codec, client (`send`/request-response over a stream) and server
    (`serve(stream, remote_peer, mode, handler)`) built on their `consume_inbound`,
    replay guard and freshness policy, lifecycle/correlation per the spec;
  - dependencies: `trust-tasks-rs` (workspace), `libp2p-identity` (PeerId/keys — the most
    stable libp2p crate), `futures`, `unsigned-varint`; **optional `swarm` feature** giving
    a turnkey `libp2p-stream` endpoint for stacks without their own stream layer (Go/JS
    peers use the spec + vectors);
  - the bridge, later, as a feature of the same crate (`tsp-bridge`), not a new codebase.
- KwaaiNet depends on it via the `trust-tasks` facade (`features = ["libp2p"]`); our
  `kwaai-p2p` glue is `accept_streams(["/trust-tasks/0.1"])` → `serve(...)` and
  `open_raw_stream(peer, ...)` → `send(...)`. Our `RawStream` already implements the seam.
- Credentials likewise by crate: `dtg-credentials` consumed as-is (OpenVTC); no fork, no
  wrapper beyond mapping our types.
- **Wire (spec):** protocol ID proposed `/trust-tasks/0.1` (confirm with WG); a stream
  carries a sequence of unsigned-varint-framed JSON envelopes until half-close; late
  responses, `trust-task-next-step`, control and errors after `executing` go on a fresh
  stream opened by whichever side sends, correlated by `id`/`threadId` (libp2p has no
  correlation of its own — say so); stream reset / connection drop / half-close map to
  **no** lifecycle state; max frame 1 MiB, rejected before allocation; read timeouts.
- **Modes:** direct and circuit-relayed, both end-to-end Noise (unlike TSP routed); state
  that third-party relays' default limits (128 KiB / 2 min) can cut documents off.
  §9.1.1 item 8: the authenticated principal is the **node**, not the process — any local
  client of the `kwaai-p2p-daemon` control socket can register the protocol.
- Targets framework 0.7.0; joins the release-plz `core` group, facade feature, bindings.js.

### 3. KwaaiNet integration (Q1 2027)
- **Stream seam only** in `kwaai-p2p`: hand `InboundStream{peer, stream}` and
  `open_raw_stream` results to `trust-tasks-libp2p`; add `relayed: bool` (or
  `ConnectedPoint`) to both so the crate can apply per-mode rules. No Trust Tasks types
  cross into `kwaai-p2p`; task handlers live in the consuming crate (`kwaai-cli` now,
  `kwaai-twin` later).
- **Credentials:** `dtg-credentials` + `affinidi-data-integrity` (`eddsa-jcs-2022`, VC 2.0)
  replace `credential.rs`/`verify.rs`. Mapping:

  | Ours | DTG |
  |---|---|
  | PeerEndorsementVC | VSC `endorses/1` |
  | UptimeVC, ThroughputVC (self-measured) | VSC with **Kwaai-namespaced** predicates + verifier accept-list (`witnessed/1` requires `taskContext`) |
  | FiduciaryPledgeVC | VSC |
  | SummitAttendeeVC | VMC, `issuerScope: public` |
  | VerifiedNodeVC | VMC |
  | BindingVC (passkey human authorises node) | VDC grant/accept |

  No signature migration: re-issue; legacy files read-only. BindingVC re-issue depends on
  summit-server's fate — decide re-issue vs a one-time server-signed bridge attestation
  once Darren's removal PR is settled.
- **Verification mandatory** in `TrustScore::from_credentials` and for `trust_attestations`;
  VC weight in reputation stays α = 0 until issuer policy exists (`TokenEconomy-plan.md` (branch `feat/kwaai-ledger`)).
- Hygiene: key file 0600 + atomic writes; wallet honours `$KWAAINET_HOME`.
- **Dependencies:** crates.io only, never vendor; caret ranges (`^0.26`, excludes 0.27),
  committed lockfile, upgrade trust-tasks-* / dtg-credentials / affinidi-* **as one group**;
  CI check `cargo tree -d` shows a single `affinidi-data-integrity`. Keep the resolver's
  network features off (their rustls defaults to aws-lc-rs; our reqwest uses ring); no
  `pq` features; pass keys between ed25519-dalek 2 (libp2p) and 3 (Affinidi) as raw
  32-byte seeds; set getrandom `wasm_js` for the wasm32 job.

### 4. Bridge (headline; design only)
- `affinidi-tsp` pack/unpack is transport-agnostic with Nested/Routed modes, so libp2p
  streams can carry TSP envelopes and TSP routing can carry a libp2p-originated Trust
  Task. Deliver a design note using `did:peer:2` routing DIDs and OpenVTC
  `tsp-conformance` vectors; check whether Affinidi's did:peer:0 derives an X25519
  `keyAgreement` (needed for TSP HPKE). Implementation is a later decision.

## Immediate actions

1. **Toolchain heads-up (Darren):** Trust Tasks crates (`trust-tasks-rs` 0.26,
   `dtg-credentials` 0.13 on the Affinidi stack) need **rustc ≥ 1.95**. Our
   `rust-toolchain.toml` pins **1.93.1**, and CI/release builds honour it despite
   `@stable`, so nothing builds them until we bump it — every job incl. CUDA, riscv,
   manylinux_2_28/cargo-dist, musl, Windows MSVC, Jetson needs checking. The bump lands as
   its own PR before the spike needs CI.
2. Phase 0 below.

## Phases

| Phase | When | Deliverable | Exit criterion |
|---|---|---|---|
| 0 · Re-sync the record | Oct 2026 | Correct `-response.md`, `PublicRelease-plan.md` trust sections (versions, the contributed crate supersedes rule 2, `ssi` → Affinidi, credential set), kwaai-trust `CLAUDE.md`/`roadmap.md`/`TODO.md`; #248 reply: correct the did:peer claim, ask Glenn to confirm in writing (slug; a complete `trust-tasks-libp2p` crate over a `(PeerId, stream)` seam in their workspace, published in the `core` group; Kwaai as co-maintainer; VID form; protocol ID) | Written maintainer confirmation incl. VID form and co-maintenance |
| 1 · Toolchain + spike | Oct–Nov 2026 | PR bumping `rust-toolchain.toml` to ≥ 1.95 with all release jobs green; `trust-tasks-libp2p` developed **in a fork of the upstream workspace** (so it is born in their layout and CI), consumed by a KwaaiNet spike branch through a git dependency, two local nodes | (a) round trip direct **and** through a default-limit relay; (b) mismatched in-band issuer rejected with `identity_mismatch`, error routed to the sender; (c) oversize frame rejected; (d) `dtg-credentials` issue→verify, plus one upstream vector verifies; (e) one real node-to-node interaction expressed as a Trust Task with a defined lifecycle end — else no-go |
| 2 · Upstream PR | Nov–Dec 2026 | OWF CLA signed first; security review of the §9.1.1 text; `bindings/libp2p/0.1/spec.md` + `trust-tasks-libp2p` (+ `libp2p` facade feature, release-plz `core` group, CODEOWNERS naming a Kwaai maintainer); vectors exercised by a second implementation (go-libp2p or js-libp2p, or Affinidi/OpenVTC) | Merged and **published on crates.io in their `core` group**; Kwaai listed as co-maintainer |
| 3 · Node integration | Q1 2027 | Switch the git dependency to the crates.io release; stream seam + relayed flag in `kwaai-p2p`; did:key VID via the crate; DTG credential stack; verified attestations; summit-server decision; docs match code | Fresh-`KWAAINET_HOME` start-up; tasks exchanged on the fleet over direct and relayed paths; credentials verify against upstream's implementation |
| 4 · Bridge | after Q1 2027 | Design note → decision | — |

## Critical files

- Ours: `rust-toolchain.toml`; `core/crates/kwaai-trust/src/{did,credential,verify,storage,trust_score}.rs`;
  `core/crates/kwaai-p2p/src/{handle,raw_stream,peer_record}.rs`;
  `core/crates/kwaai-cli/src/{identity,node,announce,reputation}.rs`;
  `core/crates/summit-server/src/vc_issuer.rs`; `projects/kwaai-trust/**`;
  `projects/kwaai-platform/plans/PublicRelease-plan.md`.
- Upstream (`trustoverip/dtgwg-trust-tasks-tf`): `trust-tasks-rs/src/transport.rs`; `trust-tasks-proof/src/affinidi/`;
  `trust-tasks-tsp/` (layout reference); `bindings/{tsp,push}/0.1/spec.md`;
  `website/assets/bindings.js`; `release-plz.toml`.

## Verification

- Spike: `trust-tasks-libp2p` unit tests (frame codec limits, envelope round trip, §4.8.1 mismatch,
  replay guard); two-node test with paired `KWAAINET_HOME`s, then via a relay with libp2p
  default limits; `dtg-credentials` round trip and upstream vectors.
- Upstream: their CI (`cargo test --workspace`, clippy `-D warnings`, `cargo deny`,
  `npm run check-bindings`); second-implementation run of the vectors.
- Integration: `cargo fmt/clippy --all-targets/test`, `cargo tree -d` affinidi check,
  fresh-install start-up test, fleet exchange over p2p.

## Not verified yet

Breaking-minor count since 09-06; X25519 keyAgreement from Affinidi did:peer:0;
transitive dependency count / binary size; credentials held by deployed nodes;
riscv/Jetson behaviour on 1.95.
