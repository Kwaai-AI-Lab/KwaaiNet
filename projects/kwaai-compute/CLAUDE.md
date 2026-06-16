# kwaai-compute — Claude Code Instructions

## Project scope

kwaai-compute owns distributed LLM inference: Petals-style block-sharded inference (CandelEngine),
model loading (SafeTensors + GGUF), RoPE/GQA/SwiGLU attention, per-session KV cache, OpenAI-compatible
HTTP API, and the compression/distributed crates. It does **not** own P2P routing (kwaai-network)
or the RAG knowledge layer (kwaai-knowledge).

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-inference` | `core/crates/kwaai-inference/` | CandelEngine, shard logic, model loading |
| `kwaai-compression` | `core/crates/kwaai-compression/` | Weight compression utilities |
| `kwaai-distributed` | `core/crates/kwaai-distributed/` | Distributed coordination |

CLI entry points in `core/crates/kwaai-cli/src/`:
- `shard_cmd.rs` — `kwaainet shard serve/run/status/chain`
- `block_rpc.rs` — RPC protocol `/kwaai/inference/1.0.0`, InferenceRequest/Response (msgpack)

## Infrastructure

**GPU inference machines** (via p2p relay — no IP/DNS needed):

Use `p2p://PEER_ID` or `mux://PEER_ID` as `--inference-urls`. The running kwaainet daemon
routes requests over the p2p relay to the remote Ollama instance. `mux://` is preferred
(concurrent, saturates OLLAMA_NUM_PARALLEL); `p2p://` for sequential use.

| Machine | Peer ID | GPU |
|---------|---------|-----|
| metro-linux | `12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` | A6000 48GB |
| metro-win   | `12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE` | A5000 |
| jerome      | `12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd` | |

Example: `--inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"`

**Shard serve command** (run on each metro machine):
```bash
kwaainet shard serve --start-block 0 --blocks 16   # first half
kwaainet shard serve --start-block 16 --blocks 16  # second half
```

**Shard run** (coordinator, runs locally):
```bash
kwaainet shard run "What is the capital of France?"
kwaainet shard chain --total-blocks 32
```

## Build & test

```bash
# Build inference crates
cargo build -p kwaai-inference -p kwaai-compression

# Build the full binary (includes shard commands)
cargo build -p kwaainet

# Smoke test (single-node, full forward pass)
kwaainet shard serve --start-block 0 --blocks 32 &
kwaainet shard run "What is the capital of France?"
```

## Current state

**Shipped:**
- CandelEngine: block-sharded inference over SafeTensors models
- RoPE positional encoding (broadcast_mul fix), Grouped Query Attention (GQA), SwiGLU MLP
- Per-session KV cache with 60s TTL, session HashMap in `TransformerShard`
- OpenAI-compatible HTTP API: `/v1/models`, `/v1/chat/completions` (streaming)
- Shard chain discovery via DHT block keys (`{prefix}.{block_n}`)
- Temperature, top-k, top-p sampling; argmax greedy sampler for chain mode

**In progress / planned:**
- Dedicated inference thread with session pool, LRU eviction (plan: `~/.claude/plans/cached-jingling-creek.md`)
- Trust-gated tool calling
- Decentralized training on sharded weights
- KV-cache scrambling for collusion resistance

## Key source files

| File | Description |
|------|-------------|
| `kwaai-inference/src/shard.rs` | `TransformerShard`: partial SafeTensors loading, KV cache, forward passes |
| `kwaai-inference/src/engine.rs` | CandelEngine: model orchestration, sampling |
| `kwaai-inference/src/model.rs` | Model architecture definitions |
| `kwaai-inference/src/loader.rs` | SafeTensors + GGUF model loading |
| `kwaai-inference/src/shard.rs:104` | RoPE broadcast fix: `broadcast_mul` not `*` |
| `kwaai-cli/src/shard_cmd.rs` | `kwaainet shard` command handlers, chain discovery |
| `kwaai-cli/src/block_rpc.rs` | RPC protocol, InferenceRequest/Response, F16-LE tensor bytes |

## Candle broadcasting rule

candle's `*` and `+` require **exact** shape match. Use `broadcast_mul`/`broadcast_add` when
dimensions differ (e.g. query has 32 heads, cos/sin has 1 head → must broadcast over n_heads dim).

## Do not

- Do not use GGUF for distributed sharding — SafeTensors only; GGUF is single-node only
- Do not use candle private internals from `candle_transformers` — use `candle_nn` primitives
- Do not hardcode model block counts — derive from `--blocks` flag or config

## Full project context

`projects/kwaai-compute/` — requirements, design docs, roadmap, TODO
