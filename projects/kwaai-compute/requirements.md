# kwaai-compute — Requirements

## Purpose

Turn heterogeneous GPU devices into a single shared inference fabric by implementing Petals-style
block-sharded LLM inference over the KwaaiNet P2P network.

## Functional requirements

1. **FR-C1** Load any HuggingFace SafeTensors model and partition transformer blocks across one or more nodes (`shard serve --start-block N --blocks M`).
2. **FR-C2** Execute a forward pass through a remote shard via the `/kwaai/inference/1.0.0` RPC protocol (msgpack, F16-LE tensor bytes).
3. **FR-C3** Coordinate a full inference chain: `shard run "prompt"` discovers the chain, routes tokens through all shards, and returns the generated text.
4. **FR-C4** Support RoPE positional encoding, Grouped Query Attention (GQA), and SwiGLU MLP with correct broadcasting semantics.
5. **FR-C5** Maintain per-session KV cache with configurable TTL (default 60s); support concurrent sessions.
6. **FR-C6** Expose an OpenAI-compatible HTTP API: `GET /v1/models`, `POST /v1/chat/completions` (streaming SSE).
7. **FR-C7** Support single-node full-model inference (all blocks on one machine) as a degenerate case.
8. **FR-C8** Show shard chain coverage: `shard chain --total-blocks N` lists which peers cover which blocks.

## Non-functional requirements

- **NFR-C1** Forward pass latency per shard: < 500ms on NVIDIA A6000 for llama3.1:8b at batch size 1.
- **NFR-C2** RPC serialization overhead: < 5ms for a 4096-token F16 tensor.
- **NFR-C3** Memory: partial model load must not materialize weights outside the assigned block range.
- **NFR-C4** KV cache TTL eviction must not leak memory — session map must be bounded.
- **NFR-C5** SafeTensors only for distributed mode; GGUF supported for single-node only.
- **NFR-C6** Targets: aarch64/x86_64 macOS (Metal), x86_64/aarch64 Linux (CUDA/CPU), x86_64 Windows.

## Out of scope

- Training — planned, not current sprint.
- Trust-gated tool calling — planned.
- KV-cache scrambling / collusion resistance — planned.
- GGUF multi-node sharding — not supported by design.

## Dependencies

- **kwaai-network** — shard chain discovery via DHT; RPC transport via libp2p streams.
- **kwaai-trust** — trust tier used for shard peer selection (minimum tier filter).
