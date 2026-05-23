# kwaai-inference crate

This crate implements CandelEngine: Petals-style block-sharded LLM inference over SafeTensors models.
RoPE, GQA, SwiGLU, per-session KV cache, OpenAI-compatible HTTP API.

**Full project context:** `projects/kwaai-compute/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/shard.rs` | `TransformerShard`: partial SafeTensors load, KV cache, forward passes |
| `src/engine.rs` | CandelEngine: model orchestration, sampling |
| `src/model.rs` | Model architecture definitions |
| `src/loader.rs` | SafeTensors + GGUF model loading |

## Candle broadcasting rule

Use `broadcast_mul`/`broadcast_add` when shapes differ — candle `*`/`+` require exact match.
RoPE fix at `shard.rs:104`: `x1.broadcast_mul(&cos4)` not `x1 * cos4`.

## Build

```bash
cargo build -p kwaai-inference
cargo test -p kwaai-inference
```
