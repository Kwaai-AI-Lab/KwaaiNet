# kwaai-compression crate

Compression utilities for distributed ML transfer: blockwise 8-bit quantization (~4x, minimal
accuracy loss), sparse top-K gradient compression, and delta encoding. Leaf crate over candle
tensors — no KwaaiNet dependencies.

**Full project context:** `projects/kwaai-compute/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/quantization.rs` | `BlockwiseQuantizer` — Hivemind-style blockwise 8-bit |
| `src/sparse.rs` | Top-K selection and other sparsification methods |
| `src/lib.rs` | `Compressor` trait, `CompressedData` |
| `benches/` | Criterion benchmarks |

## Build

```bash
cargo build -p kwaai-compression
cargo test -p kwaai-compression
cargo bench -p kwaai-compression
```
