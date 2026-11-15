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

## Phase 4: Mistral Small 24B

**Download: PASS.** `shard download --start-block 0 --blocks 8` under `~/.kwaainet-mistral-dl`, 22:13:51–22:28:21 PST (14.5 min). It averaged about 11 MB/s, with a slow patch near 8 GB. Got `model-00001` through `model-00003-of-00010.safetensors` (3 × 4.78 GB = 14.3 GB), `config.json`, `tokenizer.json` and `tokenizer_config.json`. Snapshot: `~/.cache/huggingface/models--unsloth--Mistral-Small-24B-Instruct-2501/snapshots/2eddef09…`. 19 GiB of disk left.

**Serve: PASS.** `$K stop`, `config set model unsloth/Mistral-Small-24B-Instruct-2501`, `$K start --daemon --shard`, with the same `start_block 0` and `blocks 8`:
- `MLX: Loading [0..8) of 40: h=5120 heads=32(8 kv)`, then `MLX: Shard [0..8) ready — emb=true head=false`, 25 s after start (06:28:44 → 06:29:08 UTC);
- the shard (PID 60780, `target-nemo` binary) has a `phys_footprint` of **9863 MB**, as expected (~10 GB); system memory is 35% free;
- the tokenizer has vocab 131072, BOS 1, EOS 2.

**mac-mini READY (Small 24B, MLX 0–8).**

## Phase 5: restore

1. `$K stop`: the daemon stopped and no `kwaainet` process remained.
2. Restored `~/.kwaainet/config.yaml` and `run/start-args.json` from the pre-test backups (`cmp` matches). Model `unsloth/Llama-3.1-8B-Instruct`, blocks 0–32.
3. Started with the **installed** binary, `~/.cargo/bin/kwaainet` 0.7.3, `kwaainet start --daemon --shard`: daemon PID 94220, shard PID 94304, storage PID 94261, all from `~/.cargo/bin/kwaainet`.
4. Launched Ollama 0.40.6. The shard's first readiness check ran before Ollama was up (`Ollama is not ready — refusing to advertise`), then retried and reported `Ollama ready` and `Serving whole-model inference over /kwaai/ollama-proxy/1.0.0`. This is the pre-test mode: whole model through Ollama.
5. Deleted `~/.kwaainet-mistral-dl` and `~/.kwaainet-70b-dl`.

**PASS:** the Mac is back to its pre-test state. Left in place: the Nemo and Small 24B weights in `~/.cache/huggingface` (9.8 GB + 14.3 GB, 19 GiB disk free) and the `KwaaiNet-mlx` build worktree.
