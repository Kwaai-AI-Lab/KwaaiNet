# kwaai-provenance crate

Native provenance primitives for the knowledge layer: deterministic claim and evidence
identity, an append-only lineage log, and a policy gate (Disabled / Observe / Enforce).
Standalone — nothing in the workspace depends on it yet.

**Full project context:** `projects/kwaai-knowledge/` — CLAUDE.md, GLOSSARY.md, and
`plans/NativeProvenance-plan.md` (requirements, design, decisions, limitations, rollout).

## Key source files

| File | Description |
|------|-------------|
| `src/schema.rs` | `Claim`, `EvidenceRef`, `SupportStatus`, `VerificationResult`, `SCHEMA_VERSION` |
| `src/identity.rs` | Domain-separated, length-prefixed SHA-256 IDs: `claim_id`, `evidence_id` |
| `src/lineage.rs` | `LineageEvent`, `LogEntry`, `LineageLog` — proposed/committed/failed, cycle rejection, ancestry, `traceability_rate` |
| `src/gate.rs` | `Gate`, `GateMode`, `Policy`, `Decision`, the `Verifier` trait, `UnresolvedVerifier` |

## Build

```bash
cargo build -p kwaai-provenance
cargo test -p kwaai-provenance
cargo clippy -p kwaai-provenance --all-targets -- -D warnings
```

## Current state

Phase 1 of the plan: in-memory primitives only. No persistence, no semantic verifier,
no quarantine store, no integration with `kwaai-rag`.

## Do not

- Do not add a dependency on `kwaai-rag` (or any KwaaiNet crate). Integration goes in a
  feature-gated adapter on the `kwaai-rag` side, in its own PR.
- Do not let a verifier return `Supported` because evidence IDs are present. Association
  is not support; that conflation is what this crate exists to separate.
- Do not hash `EvidenceRef.parent_ids` into the evidence ID, or treat it as history.
  Derivations belong in `LineageLog`.
- Do not count proposed or failed events as lineage.
- Do not change an ID scheme without bumping its domain string (`...:v1` → `...:v2`).
  Existing IDs would otherwise silently change meaning.
- Do not commit JSONL logs produced from a private KB (D6).
