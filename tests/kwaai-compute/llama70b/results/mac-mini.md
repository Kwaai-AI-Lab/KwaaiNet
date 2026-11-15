# Mac mini results

## Phase 1: download (2026-11-14)

The live node kept serving 8B blocks 0–16 throughout. Backups of `config.yaml` and `run/start-args.json` were taken before the 8B MLX test.

- Command: `KWAAINET_HOME=~/.kwaainet-70b-dl $K shard download --start-block 0 --blocks 8`
- Time: 17:25:41 to 17:35:42 PST, **10 min**.
- Snapshot: `~/.cache/huggingface/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f`. `HF_HOME=~/.cache/huggingface` is set on this machine, so there is no `hub/` level in the path.
- Contents: `config.json`, `tokenizer.json`, `tokenizer_config.json`, `special_tokens_map.json`, the index, and `model-00001` through `model-00004` of 30. **18 GB.** The plan estimated 16 GB; the extra is other layers that share those files.
- **PASS:** the 73 tensors for `model.layers.0`–`7` plus `model.embed_tokens` are all present (checked against `model.safetensors.index.json`).
- Free disk: 65 GiB before, 35 GiB after.
