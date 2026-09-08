# Serving on a Jetson Orin

Measured 2026-09-08 on a Jetson Orin Nano (8 GB variant, 7 GB usable, L4T
r36.5 / JetPack 6, CUDA 12.6), serving blocks 0–8 of Llama-3.1-8B-Instruct
in f16 inside a three-node chain.

## Build

```bash
cd core
export PATH=/usr/local/cuda/bin:$HOME/.cargo/bin:$PATH   # nvcc is not on PATH for non-interactive shells
core/patches/fetch-patches.sh                            # once; fetches the patched crates, cudarc included
CUDA_COMPUTE_CAP=87 cargo build --release -p kwaainet --bin kwaainet --features cuda-no-flash -j 4
```

- `cuda-no-flash` is plain candle CUDA. `cuda` adds flash-attn, which is a far
  longer build and was not tried on sm_87.
- ~25 minutes warm; the final LTO link needs about 4 GB, so build with no
  shard server resident — the OOM killer takes the 4.5 GB server first.

## Run

```bash
CUDARC_DISABLE_ASYNC_ALLOC=1 kwaainet shard serve --use-gpu --start-block 0 --blocks 8 --model-path <snapshot>
```

- The variable is required: without it the stream-ordered allocator caps at
  ~2.5 GB on this board and the load dies at block 3 (see
  `core/patches/README.md`).
- The first request after a fresh binary spends 15–40 s JIT-compiling PTX
  into `~/.nv/ComputeCache`; later requests are normal. A client with a 30 s
  call timeout may fail that first request.
- Only the safetensors files carrying the shard's layers need to be present;
  the loader reads per block from the index.

## What to expect

| | CPU | GPU |
|---|---|---|
| decode, 8 blocks + embeddings | ~440 ms/token | ~150 ms/token (median 156, min 109) |
| prefill forward, 17 tokens | 730 ms | 120–170 ms |
| resident memory | 4.5 GB | 5.25 GB (unified memory; GPU buffers count) |

Decode is memory-bandwidth bound; the board's ~68 GB/s puts the floor for
this shard near 66 ms, so there is roughly a 2× left in candle's per-op
CUDA path (no fused attention at batch size 1). Four Orins at 8 blocks each
cover the model.
