# kwaai-distributed crate

Distributed ML operations following Hivemind patterns: Mixture of Experts across network nodes,
decentralized parameter averaging with no master node, and fault tolerance when nodes drop.

**Full project context:** `projects/kwaai-compute/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/moe.rs` | Mixture of Experts — distributes expert sublayers across the network |
| `src/averaging.rs` | Decentralized gradient/parameter averaging, no central server |
| `src/expert.rs` | Expert registration and management |
| `src/coordinator.rs` | Distributed operations coordinator |

## Scope

This is the **training** side of the compute pillar (decentralized model training is still
"coming" on the roadmap). Sharded *inference* lives in `kwaai-inference`.

## Build

```bash
cargo build -p kwaai-distributed
cargo test -p kwaai-distributed
```
