# kwaai-trust — Requirements

## Purpose

Provide cryptographic identity, W3C Verifiable Credentials, and local trust scoring so every
KwaaiNet node can prove who it is and evaluate how much to trust other nodes — without a central authority.

## Functional requirements

1. **FR-T1** Generate and persist an Ed25519 keypair at `~/.kwaainet/identity.key` on first run; never overwrite an existing key without explicit user confirmation.
2. **FR-T2** Derive a `PeerId` and `did:peer:` DID from the keypair; expose both via `kwaainet identity show`.
3. **FR-T3** Maintain a VC wallet at `~/.kwaainet/credentials/`; support issue, store, list, and verify operations.
4. **FR-T4** Support at minimum these VC types: `FiduciaryPledgeVC`, `VerifiedNodeVC`, `UptimeVC`, `ThroughputVC`, `EventAttendeeVC`, `PeerEndorsementVC`.
5. **FR-T5** Compute a local trust score for any `PeerId` using time-decayed, credential-weighted logic; return one of four tiers: `Unknown`, `Known`, `Verified`, `Trusted`.
6. **FR-T6** Sign any VC with the node's Ed25519 keypair; verify the signature before accepting any incoming VC.
7. **FR-T7** Export identity and credentials in a portable format for backup and migration.
8. **FR-T8** WASM build must expose identity and trust scoring to browser-based nodes.

## Non-functional requirements

- **NFR-T1** Key derivation and signing must complete in < 10ms on all supported platforms.
- **NFR-T2** Trust score recomputation must complete in < 50ms for up to 1,000 stored credentials.
- **NFR-T3** Credential wallet must survive process restarts; writes must be atomic (no partial files on crash).
- **NFR-T4** WASM bundle must be < 500KB gzipped.
- **NFR-T5** Targets: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows, WASM.

## Out of scope

- P2P transport for credential exchange — that is kwaai-network's responsibility.
- Testable Credentials (TC) / PVP-1 — planned, not current sprint.
- EigenTrust propagation — planned, not current sprint.

## Dependencies

- None — kwaai-trust is a leaf crate with no KwaaiNet dependencies.
