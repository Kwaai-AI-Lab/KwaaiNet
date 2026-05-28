# kwaai-trust — Roadmap

## Shipped

- Ed25519 keypair + `PeerId` + `did:peer:` DID generation and persistence
- VC wallet with types: FiduciaryPledgeVC, VerifiedNodeVC, UptimeVC, ThroughputVC, EventAttendeeVC, PeerEndorsementVC
- Local trust score: time-decayed, credential-weighted, four tiers (Unknown/Known/Verified/Trusted)
- VC signing and verification (Ed25519)
- `kwaainet identity show/create/export`
- `kwaainet trust score/issue/verify`
- WASM bindings for browser nodes (`kwaai-wasm`)
- ToIP Decentralized Trust Graph participation

## In progress

- None active (trust layer is stable baseline)

## Planned

- **Testable Credentials (TC) / PVP-1** — contextual evaluation: "does this claim apply to my workload now?" — see whitepaper §Trust layer, PVP-1 spec
- **EigenTrust-style propagation** — transitive, Sybil-resistant trust across the network
- **Sybil resistance hardening** — cross-domain trust exchange aligned with ToIP patterns
- **Cross-network trust import/export** — interoperability formats with ToIP (DID/VC profiles)

## Research / future

- Credential revocation registry (on-chain or DHT-anchored)
- Zero-knowledge proofs for credential attribute disclosure without revealing full VC
- Trust-gated capability grants (link trust tier to allowed operations)
