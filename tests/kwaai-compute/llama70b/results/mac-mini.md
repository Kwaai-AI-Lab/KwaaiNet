# Mac mini results

## Phase 1: download (2026-11-14)

The live node kept serving 8B blocks 0–16 throughout. Backups of `config.yaml` and `run/start-args.json` were taken before the 8B MLX test.

- Command: `KWAAINET_HOME=~/.kwaainet-70b-dl $K shard download --start-block 0 --blocks 8`
- Time: 17:25:41 to 17:35:42 PST, **10 min**.
- Snapshot: `~/.cache/huggingface/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f`. `HF_HOME=~/.cache/huggingface` is set on this machine, so there is no `hub/` level in the path.
- Contents: `config.json`, `tokenizer.json`, `tokenizer_config.json`, `special_tokens_map.json`, the index, and `model-00001` through `model-00004` of 30. **18 GB.** The plan estimated 16 GB; the extra is other layers that share those files.
- **PASS:** the 73 tensors for `model.layers.0`–`7` plus `model.embed_tokens` are all present (checked against `model.safetensors.index.json`).
- Free disk: 65 GiB before, 35 GiB after.

## Phase 2: switch (run first, before the metros, by Reza's choice)

2026-11-15 02:58 UTC (18:58 PST).

1. Ollama: `osascript quit` was cancelled by the app (`User canceled. (-128)`), so it was stopped with `pkill -TERM`. Both Ollama processes exited.
2. `$K stop` → daemon PID 13881 stopped. `config set`: `contribute.auto_update false`, `model unsloth/Llama-3.3-70B-Instruct`, `shard.backend mlx`, `start_block 0`, `blocks 8`, each ✅.
3. `$K start --daemon --shard` at 02:58:23. The log said `Rebalancing disabled — this node pins blocks [0, 8)`.

```
02:58:31 MLX: Loading [0..8) of 80: h=8192 heads=64(8 kv)
02:59:44 MLX: test matmul [8192,28672] took 12.8ms
02:59:47 MLX: Shard [0..8) ready — emb=true head=false compiled=false
  ✅ Shard ready  (8 blocks, is_first=true, is_last=false)
```

**PASS:** the shard loaded in 84 s. The fallback layout is not needed.

Memory after load:
- the shard process (PID 8843) has a footprint of 15 GB, and its peak was also 15 GB;
- system-wide memory free was 33%;
- swap was 30.2 of 30.7 GB used. That is cumulative pressure from two rust-analyzer processes (5.4 and 5.0 GB) and Chrome (about 8 GB across helpers), not only from the shard.
