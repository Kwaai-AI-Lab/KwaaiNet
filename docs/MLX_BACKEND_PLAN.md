# MLX Backend Plan — Apple Silicon GPU Acceleration

## Why MLX

Candle's Metal backend gets 0.5 tok/s decode on Llama 8B (M-series). We default to CPU (4.9 tok/s) as a workaround. MLX — Apple's native ML framework — achieves **~30-40 tok/s** on the same hardware through:

- **Unified memory**: zero-copy between CPU and GPU (no `to_device()` stalls)
- **Lazy evaluation**: operations build a compute graph, fused automatically at eval time
- **Optimized transformer kernels**: attention, RoPE, MLP are hand-tuned for Apple Silicon

## Research Findings (mlx-rs v0.25.3)

| Criterion | Status |
|-----------|--------|
| SafeTensors loading | Supported natively |
| Transformer ops (Linear, RMSNorm, RoPE, Attention, SiLU, Softmax) | All available |
| KV-cache | Per-layer, rotating cache supported |
| Pre-built Llama model | Not in Rust — need to port ~200 lines from Python reference |
| Performance | ~30-40 tok/s Llama 8B on M1/M2 (Python benchmarks, Rust should be similar) |
| Platform | macOS only (Apple Silicon arm64, macOS >= 14.0) |
| System dependency | Requires `mlx-c` library (`brew install mlx`) |
| Maturity | v0.25.3, 285+ stars, 8+ contributors, active development |
| License | MIT / Apache 2.0 |

## Architecture

Keep candle as the primary backend for Linux/Windows. Add mlx-rs as a conditional macOS feature:

```
TransformerShard (existing API — unchanged)
├── CandleBackend (Linux, Windows, macOS fallback)
│   └── candle-core + candle-nn (existing code)
└── MlxBackend (macOS Apple Silicon only)
    └── mlx-rs + custom Llama implementation
```

### Feature gating

```toml
# kwaai-inference/Cargo.toml
[target.'cfg(target_os = "macos")'.dependencies]
mlx-rs = { version = "0.25", optional = true }

[features]
mlx = ["mlx-rs"]
```

### Device selection

```rust
pub enum DeviceType {
    Cpu,
    Cuda(usize),
    Metal(usize),     // candle Metal (legacy, slow for decode)
    Mlx,              // new — Apple MLX (fast for both prefill and decode)
}

impl DeviceType {
    pub fn detect_best() -> Self {
        #[cfg(feature = "mlx")]
        if mlx_available() { return Self::Mlx; }

        #[cfg(feature = "cuda")]
        if cuda_available() { return Self::Cuda(0); }

        Self::Cpu  // Metal skipped by default (too slow for decode)
    }
}
```

## Implementation Steps

### Phase 1: MLX Llama model (1 week)

**New file: `kwaai-inference/src/mlx_shard.rs`**

Port the ~200 line Python Llama reference to Rust using mlx-rs primitives:

1. `MlxTransformerBlock` — RMSNorm → Attention (RoPE + GQA + KV-cache) → SwiGLU MLP
2. `MlxTransformerShard` — embedding + blocks + norm + lm_head
3. `load()` — SafeTensors loading via mlx-rs
4. `forward_full()` / `forward_first()` / `forward_middle()` / `forward_last()` — same API as candle `TransformerShard`
5. Session/KV-cache management — same `HashMap<u64, Session>` pattern

### Phase 2: Integration (2-3 days)

1. Add `DeviceType::Mlx` variant to `kwaai-inference/src/lib.rs`
2. Modify `shard_cmd.rs` device selection to prefer MLX on macOS
3. Modify `shard serve` and `shard run --local` to use `MlxTransformerShard` when `DeviceType::Mlx`
4. Keep the existing `TransformerShard` (candle) as fallback
5. Update benchmark to test MLX path

### Phase 3: Testing and polish (2-3 days)

1. Benchmark: MLX vs CPU on same prompt
2. Verify distributed inference works (MLX shard serving blocks over p2pd)
3. Handle `mlx-c` not installed gracefully (fall back to CPU)
4. Update `kwaainet setup --get-deps` to install mlx-c on macOS
5. Documentation

## Dependencies for users

```bash
# macOS users need:
brew install mlx

# Verified by:
kwaainet setup --get-deps  # auto-installs if missing
```

## Expected Performance

| Backend | Prefill (tok/s) | Decode (tok/s) | Platform |
|---------|----------------|----------------|----------|
| CPU (candle) | 25 | 4.9 | All |
| Metal (candle) | 4715 | 0.5 | macOS (broken) |
| **MLX (target)** | **~500+** | **~30-40** | **macOS Apple Silicon** |
| CUDA (candle) | TBD | TBD | Linux NVIDIA |

## Risk Assessment

| Risk | Mitigation |
|------|-----------|
| mlx-rs API gaps | Python reference is comprehensive; mlx-rs wraps same C++ lib |
| mlx-c system dependency | `brew install mlx` is simple; add to `setup --get-deps` |
| Autograd differences | Inference-only, no training — autograd not needed |
| SafeTensors format mismatch | Both use standard HuggingFace format |
| Maintenance burden of two backends | Trait abstraction keeps them independent |
