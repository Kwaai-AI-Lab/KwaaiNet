# kwaai-trust crate

This crate implements Ed25519 identity, `did:peer:` DIDs, W3C VC wallet, and local trust scoring.
It is a leaf crate — no KwaaiNet dependencies.

**Full project context:** `projects/kwaai-trust/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/did.rs` | DID generation, `did:peer:` method |
| `src/credential.rs` | VC types, issuance, signing, wallet |
| `src/trust_score.rs` | Local score computation, time decay, tier enum |
| `src/verify.rs` | VC verification, Ed25519 signature checks |
| `src/storage.rs` | Credential wallet persistence |

## Build

```bash
cargo build -p kwaai-trust
cargo test -p kwaai-trust
```
