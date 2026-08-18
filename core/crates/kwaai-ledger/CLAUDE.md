# kwaai-ledger crate

This crate implements KwaaiNet internal work credits: a provider-signed **quote**, a provider-signed
**claim** about work delivered, and a consumer-counter-signed **receipt**. Netting receipts pairwise
is a bilateral mutual-credit system — no mint, no balance, no network-wide fungibility, total supply
exactly zero. It needs no consensus, which is why the scope was chosen.

**Full project context:** `projects/kwaai-trust/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/lib.rs` | `LeaseQuote`, `SignedLeaseGrant`, `WorkClaimPayload`, receipts, canonical-encoding rules |
| `src/economy.rs` | Pluggable currency backends — receipts record work, an economy prices it |
| `src/store.rs` | Local per-node persistence for every receipt this node is a party to |

## Canonical encoding

Signing uses rmp-serde's **positional** (non-named) encoding, not the named form. See the
canonical-encoding rules in `lib.rs` — a mismatch here silently breaks signature verification
across peers. `receipt_id` and `response_digest` are SHA-256.

Payload version is a policy decision, not a decode failure (4a94e88), and an older peer's
artifact must not cost the caller its response (b50b318).

## Build

```bash
cargo build -p kwaai-ledger
cargo test -p kwaai-ledger
```
