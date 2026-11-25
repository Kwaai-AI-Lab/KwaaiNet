# kwaai-provenance

Provenance primitives for KwaaiNet's knowledge graph. The crate answers two questions
the graph cannot answer today:

1. **Where did this fact come from?** Every extraction, derivation, entity merge, claim
   rewrite and prune is recorded as an immutable lineage event, so a fact can be traced
   back — or shown not to trace back — to original source evidence.
2. **Is it supported, and what should happen to it?** A verifier reports a status
   (supported / unsupported / unresolved); a separate policy maps that status to a
   decision (allow / reject / quarantine). The gate runs in one of three modes:

| Mode | Calls verifier | Effect on the mutation |
|------|----------------|------------------------|
| Disabled | no | none (baseline behaviour) |
| Observe | yes | none; the recommendation is recorded |
| Enforce | yes | the policy decision is applied |

Evidence *association* is not *support*. The only verifier shipped,
`UnresolvedVerifier`, never reports `Supported`.

## Status

Experimental, standalone, not published (`publish = false`). Lineage and gate outcomes
can be persisted as JSONL and replayed with full validation. Nothing in the workspace
calls it yet. See
[`projects/kwaai-knowledge/plans/NativeProvenance-plan.md`](../../../projects/kwaai-knowledge/plans/NativeProvenance-plan.md)
for requirements, design diagrams, decisions, limitations and the rollout plan.

## Example

```rust
use kwaai_provenance::gate::{Decision, Gate, GateMode, UnresolvedVerifier};
use kwaai_provenance::lineage::{LineageEvent, LineageEventKind, LineageLog};
use kwaai_provenance::{Claim, ClaimKind};

let claim = Claim::new(ClaimKind::Relation, "alice", "works_at", "kwaai");

let mut log = LineageLog::new();
let event = LineageEvent::new(
    LineageEventKind::Extract, ["orig:chunk-1"], [claim.id.as_str()], "ingest", None, 0,
)?;
let id = log.propose(event)?;
log.commit(&id)?;
assert!(log.traces_to(&claim.id, |a| a.starts_with("orig:")));

let out = Gate::new(GateMode::Observe).check(&claim, &[], &UnresolvedVerifier)?;
assert_eq!(out.recommended, Some(Decision::Quarantine));
assert_eq!(out.applied, Decision::Allow);
```

The same example runs as the crate-level doctest in `src/lib.rs`.

## Modules

| Module | Contents |
|--------|----------|
| `schema` | Record types and `SCHEMA_VERSION` |
| `identity` | Deterministic SHA-256 IDs for claims and evidence |
| `lineage` | Lineage events, the append-only log, ancestry and traceability queries |
| `gate` | Modes, policy, decisions, the `Verifier` trait |
| `persist` | JSONL records, writer, strict reader, replay, torn-tail repair, digest, atomic snapshot |

## Persisting a run

```rust
use kwaai_provenance::persist::{read_file, JsonlWriter, TailPolicy};

let mut out = JsonlWriter::append_to(path, 0)?;
// after each propose / commit / fail:
out.sync_lineage(&log)?;
// after each gate check:
out.write_gate(&outcome)?;
out.sync_all()?;

// later, or from another process:
let read = read_file(path, TailPolicy::Strict)?;
let log = read.replay()?;          // re-validates every entry
let outcomes = read.gate_outcomes();
```

A file that ends in a torn line (interrupted write) is refused for appending. Read it
with `TailPolicy::DropTornTail` to see what was lost, then call `truncate_torn_tail`.

## Build and test

```bash
cd core
cargo test -p kwaai-provenance
```
