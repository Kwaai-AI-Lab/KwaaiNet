# Mistral Small 24B across three GPUs on three platforms

**Status:** plan, 2026-11-14. It replaces the [Llama 3.3 70B plan](Llama70B-Heterogeneous-plan.md), which failed (see "Why the 70B run was dropped").

## Goal

Same intent as before: one model that no single participating GPU can hold, split by blocks across a Mac (MLX), a Windows box (CUDA) and a Linux box (CUDA).

The scope is narrower:
- **GPU only.**
- **One node per machine.**

## Models

Both models are run in 16-bit. Neither is gated.

| Step | Model | Size | Layers | Why |
|---|---|---|---|---|
| 1. Shakedown | `unsloth/Mistral-Nemo-Instruct-2407` (12B) | 24.5 GB | 40 | Same `head_dim` mismatch as Small 24B (128, against hidden ÷ heads = 160), so it exercises the fix at half the download. It is still more than one 24 GB GPU holds. |
| 2. Target | `unsloth/Mistral-Small-24B-Instruct-2501` | 47 GB | 40 | Twice any single GPU. A current general chat model, under Apache 2.0. |

Both have hidden size 5120, 32 heads, 8 KV heads, `head_dim` 128 and vocab 131072, and both have untied `lm_head`. So one layout serves both, and only `model` changes between step 1 and step 2.

## Code needed

**`fix/config-head-dim` (`3e092c0c`).** Both block backends used to derive `head_dim` as hidden ÷ heads and build `q_proj`/`o_proj` square. Mistral models failed to load on shape. The fix reads `head_dim` from `config.json` and falls back to the quotient, so Llama models are unchanged. All three machines build from this branch.

**Mac only:** a local patch in `../KwaaiNet-mlx` raises `node_native.rs → REQUEST_TIMEOUT` from 30 s to 300 s. Every hop is called from the client's node, so only the Mac needs it (#134).

## Layout

| Order | Machine | Backend | Blocks | Nemo 12B | Small 24B |
|---|---|---|---|---|---|
| 1 | Mac mini, 24 GB unified | MLX | 0–8, plus embeddings | 5.7 GB | 10.2 GB |
| 2 | metro-win, A5000 24 GB | CUDA (`cuda-windows`) | 8–24 | 8.7 GB | 17.8 GB |
| 3 | metro-linux, A5000 24 GB | CUDA (`cuda`) | 24–40, plus norm and `lm_head` | 10.1 GB | 19.2 GB |

Every node serves 8 or 16 blocks, both of which are valid counts. The Mac stays well under the 15 GB at which it stalled in the 70B run.

**Downloads.** These are whole safetensors files, so they overlap at the range edges.

| Machine | Nemo 12B | Small 24B |
|---|---|---|
| Mac | 9.8 GB | 14.3 GB |
| metro-win | 9.8 GB | 19.2 GB |
| metro-linux | 14.7 GB | 23.1 GB |

## Known limits that apply

- **The `shard download` clamp.** Before a model is in the local cache, `KwaaiNetConfig::model_total_blocks` guesses the block count from the name, and anything without "70b" or "13b" counts as 32. So metro-linux's 24–40 would be clamped to 24–32 and miss `lm_head`. Workaround: download 24–32 first, which writes `config.json`, then 24–40. To be fixed separately.
- **No Mistral chat template in `shard run`.** It knows only Llama 3 and ChatML. Prompts are written with `[INST] … [/INST]` by hand; the tokenizer has both as special tokens, and BOS is added by the code.
- **Prompts under about 300 tokens.** Same control-socket limit as before; at hidden size 5120 it is less tight.
- **Long context:** no `rope_scaling` concern. Nemo's RoPE table is sized to `max_position_embeddings` (1,024,000), about 260 MB on the CUDA nodes. That is affordable and noted, not fixed.

## Runs

From the Mac. Use `--total-blocks 40` for both models.

```
$K shard chain --total-blocks 40
$K shard run "[INST]Name three rivers in Africa.[/INST]" --total-blocks 40 --max-tokens 32 --stats
$K shard run "[INST]Write a haiku about distributed computing.[/INST]" --total-blocks 40 --max-tokens 32 --stats
$K shard run "[INST]What is the capital of Australia, and why?[/INST]" --total-blocks 40 --max-tokens 48 --stats
```

## Pass criteria (each model)

1. `shard chain` shows 40/40 blocks, from exactly the three nodes.
2. Every run's pinned path is Mac, then metro-win, then metro-linux.
3. All three answers are coherent and correct, with no hop errors.
4. Every node runs on its GPU. No GPU memory is over 23 GB, and nothing runs out of memory.
5. Per-hop times and tokens per second, from `--stats`, over at least 14 tokens.

## Why the 70B run was dropped

Recorded in `test/llama70b-heterogeneous`:
- 64 of its 80 blocks had to run on CPU, because blocks are 16-bit only and 70B is 141 GB.
- A 16-token CPU prefill through 32 blocks takes longer than the 30 s per-hop timeout (#134).
- A Linux CUDA build can't serve blocks on CPU at all (#291).
- The Mac's 8 blocks (15 GB) stalled under memory pressure until other apps were closed.
