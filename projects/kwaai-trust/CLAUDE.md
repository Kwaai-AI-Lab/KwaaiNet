# kwaai-trust — Claude Code Instructions

## Project scope

kwaai-trust owns all cryptographic identity and credential logic: Ed25519 keypairs, `did:peer:` DIDs,
W3C Verifiable Credentials (VC wallet), local trust scoring, and WASM packaging for browser nodes.
It does **not** handle P2P transport, inference, or storage.

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-trust` | `core/crates/kwaai-trust/` | Identity, VC wallet, trust scoring |
| `kwaai-wasm` | `core/crates/kwaai-wasm/` | WASM bindings for browser nodes |

CLI entry points in `core/crates/kwaai-cli/src/`:
- `identity.rs` — `kwaainet identity` subcommands (create, show, export)
- `trust.rs` — `kwaainet trust` subcommands (score, issue, verify)

## Build & test

```bash
# Build only trust crates
cargo build -p kwaai-trust -p kwaai-wasm

# Run tests
cargo test -p kwaai-trust

# WASM build (requires wasm-pack)
wasm-pack build core/crates/kwaai-wasm --target bundler
```

## Current state

**Shipped:**
- Ed25519 keypair at `~/.kwaainet/identity.key`, derived `PeerId` and `did:peer:` DID
- VC wallet at `~/.kwaainet/credentials/` with types: `FiduciaryPledgeVC`, `VerifiedNodeVC`,
  `UptimeVC`, `ThroughputVC`, `EventAttendeeVC`, `PeerEndorsementVC`
- Local trust scoring with time decay and tiers: `Unknown → Known → Verified → Trusted`
- Active participation in Trust over IP (ToIP) Decentralized Trust Graph

**In progress / planned:**
- Testable Credential (TC) layer and PVP-1 protocol for contextual evaluation
- EigenTrust-style propagation across the network
- Sybil resistance hardening

## Key source files

| File | Description |
|------|-------------|
| `kwaai-trust/src/did.rs` | DID generation, serialization, `did:peer:` method |
| `kwaai-trust/src/credential.rs` | VC types, issuance, signing, wallet |
| `kwaai-trust/src/trust_score.rs` | Local score computation, time decay, tier enum |
| `kwaai-trust/src/verify.rs` | VC verification, signature checks |
| `kwaai-trust/src/storage.rs` | Credential wallet persistence (`~/.kwaainet/credentials/`) |
| `kwaai-cli/src/identity.rs` | `kwaainet identity` command handler |
| `kwaai-cli/src/trust.rs` | `kwaainet trust` command handler |

## Do not

- Do not add P2P transport code here — that belongs in `kwaai-network`
- Do not hardcode credential types; they must be extensible for future VC schemas
- Do not store credentials unencrypted on disk
- Do not move credential wallet path away from `~/.kwaainet/credentials/` without a migration path

## Full project context

`projects/kwaai-trust/` — requirements, design docs, roadmap, TODO
