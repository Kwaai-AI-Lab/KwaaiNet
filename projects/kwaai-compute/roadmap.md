# kwaai-compute — Roadmap

## Shipped

- CandelEngine: block-sharded inference over SafeTensors models
- RoPE positional encoding (broadcast_mul fix — `x1.broadcast_mul(&cos4)` not `x1 * cos4`)
- Grouped Query Attention (GQA), SwiGLU MLP
- Per-session KV cache with 60s TTL and session HashMap in `TransformerShard`
- OpenAI-compatible HTTP API: `/v1/models`, `/v1/chat/completions` (streaming SSE)
- Shard chain discovery via DHT block keys (`{prefix}.{block_n}`)
- Argmax greedy sampler; temperature/top-k/top-p sampling
- Single-node full-forward-pass mode (all blocks on one machine)
- `kwaainet shard serve/run/status/chain`

## In progress

- **Dedicated inference thread with session pool** — `LlamaContext` reuse, LRU eviction, prefix-matching for session continuation. Plan: `~/.claude/plans/cached-jingling-creek.md`. Files: `llama_local.rs`, `shard_api.rs`.
- **Metro machine connectivity** — both `metro-linux` (A6000) and `metro-win` (A5000) have DNS/routing issues; Ollama bound to localhost. Fix via p2p relay (kwaai-network dependency).

## Planned

- **Trust-routed inference** — credential-gated shard selection; minimum trust tier per request
- **KV-cache scrambling** — mitigation for collusion attacks between shard holders
- **Trust-gated tool calling** — credential-gated tool access with audit trails
- **`kwaainet_version` in DHTServerInfo** — surface shard peer versions for compatibility checking

## Research / future

- Decentralized training on sharded weights (extend CandelEngine with training loops)
- MLX backend for Apple Silicon (`docs/MLX_BACKEND_PLAN.md`)
- GGUF distributed mode (currently single-node only due to format constraints)
- Metal performance optimisation for macOS (`docs/METAL_PERFORMANCE_ANALYSIS.md`)
