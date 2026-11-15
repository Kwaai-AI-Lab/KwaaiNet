# Mac mini results: blocks 0–8, MLX

Binary: `../KwaaiNet-mlx/core/target-nemo/release/kwaainet`. It is `main` at `c48c3b8e` plus three changes:
- `fix/config-head-dim` (`3e092c0c`);
- the local `REQUEST_TIMEOUT` of 300 s;
- comma-separated `--name-filter`.

## Phase 0

- 2026-11-15 04:21 UTC: the 70B node was stopped.
- 70B weights deleted, with Reza's OK: `~/.cache/huggingface/models--unsloth--Llama-3.3-70B-Instruct` (18 GB), plus `~/.kwaainet-70b-dl`.

## Phase 1: Nemo download — PASS

- 04:13:07 → 04:24:46 UTC, about 12 min.
- Downloaded `model-00001` (4.5 GB) and `model-00002-of-00005.safetensors` (4.6 GB), plus `config.json`, `tokenizer.json`, `tokenizer_config.json`, `generation_config.json` and `special_tokens_map.json`.
- Snapshot: `~/.cache/huggingface/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5`.
- Free disk afterwards: 35 GiB.

## Phase 2: serve Nemo — PASS (load)

This was started early, since the Mac was idle. It was the first load of real Mistral weights with the `head_dim` fix.

```
04:25:06 MLX: Loading [0..8) of 40: h=5120 heads=32(8 kv)
04:25:19 Loaded tokenizer.json: vocab=131072, bos=Some(1), eos=Some(2)
04:25:19 MLX: Shard [0..8) ready — emb=true head=false compiled=false
```

- The load took 13 s, and the shard's `phys_footprint` is 5541 MB.
- The tokenizer resolves Mistral's `<s>`/`</s>` as BOS/EOS.
- No forward pass has run yet; that needs the metros.
