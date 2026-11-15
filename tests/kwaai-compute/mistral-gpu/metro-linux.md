# metro-linux: blocks 24–40 on the GPU (CUDA), one node

Read [README.md](README.md) first; its rules apply. Write everything to `tests/kwaai-compute/mistral-gpu/results/metro-linux.md`.

This test runs **one** node, in the default home `~/.kwaainet`. The 70B test's second node (node A, `~/.kwaainet-70b-cpu`) is removed in Phase 0.

**This box runs out of memory on uncapped builds.** Always build with `-j 8`.

## Phase 0: leave the 70B

1. Stop node A. Use a shell with its four variables set:
   ```bash
   export KWAAINET_HOME=$HOME/.kwaainet-70b-cpu
   export KWAAINET_SOCKET=$KWAAINET_HOME/node.sock
   export KWAAINET_PORT=8081
   export KWAAINET_GRPC_PORT=8094
   kwaainet stop
   ```
2. Then, in a clean shell:
   ```bash
   unset KWAAINET_HOME KWAAINET_SOCKET KWAAINET_PORT KWAAINET_GRPC_PORT
   kwaainet stop
   rm -rf ~/.kwaainet-70b-cpu
   pgrep -fa kwaainet; nvidia-smi; free -g
   ```
   **PASS:** no `kwaainet` process remains, and the GPU and RAM are free. Keep `~/config.70b-backup.yaml` and `~/start-args.70b-backup.json`. If Phase 2b's `~/kwaainet-cpu` binary and `~/kwaainet-cpu-target` exist, delete them.

## Phase 1: build and download Nemo

1. Build from the code branch. Use a separate worktree, so this checkout stays on the test branch:
   ```bash
   git fetch origin
   git worktree add ../KwaaiNet-headdim origin/fix/config-head-dim
   cd ../KwaaiNet-headdim/core
   # core/patches/ is git-ignored: symlink it from your main clone if it is missing
   CUDA_COMPUTE_CAP=80 cargo build --release -p kwaainet --features cuda -j 8
   K=$PWD/target/release/kwaainet
   $K --version
   git -C .. log --oneline -1          # expect 3e092c0c or later
   ```
   These are the release CUDA build's feature and compute capability. `cuda` includes flash-attn, which is right on the GPU, and its kernels take a while to compile. If the flash-attn compile fails, record the error and use `--features cuda-no-flash` instead. Record the build time and the commit.
2. Download blocks 24–40 under a throwaway home. This takes **two steps on purpose**. Before the model is in the cache, `shard download` guesses 32 blocks from the name, which would clamp 24–40 to 24–32 and leave out `lm_head`. The first step writes `config.json`; the second then reads the real total of 40.
   ```bash
   export KWAAINET_HOME=$HOME/.kwaainet-mistral-dl
   $K config set model unsloth/Mistral-Nemo-Instruct-2407
   $K shard download --start-block 24 --blocks 8     # [24, 32) of 32: writes config.json
   $K shard download --start-block 24 --blocks 16    # must say [24, 40) of 40
   unset KWAAINET_HOME
   ```
   **PASS:**
   - the second download says `Blocks: [24, 40) of 40`;
   - `model-00003` through `model-00005-of-00005.safetensors` are present, about 14.7 GB, along with `config.json` and `tokenizer.json`;
   - the index's `lm_head.weight` and `model.norm.weight` are in a downloaded file.

   Record the times and the snapshot path.
3. Commit and push `test(mistral): metro-linux built+downloaded`. Tell Reza.

## Phase 2: serve Nemo (once Reza says go)

```bash
$K config set contribute.auto_update false
$K config set model unsloth/Mistral-Nemo-Instruct-2407
$K config set use_gpu true
$K config set start_block 24
$K config set blocks 16
$K start --daemon --shard
tail -f ~/.kwaainet/logs/shard.log      # until ready
```

**PASS:**
- the log has `Device: CUDA`, then `Shard ready  (16 blocks, is_first=false, is_last=true)`, with no load error. In particular there is no `q_proj: shape mismatch`;
- `nvidia-smi` shows about 10 GB in use;
- `ps -o args= -p $(cat ~/.kwaainet/run/shard.pid)` shows the `$K` path.

Commit and push `test(mistral): metro-linux nemo ready`. Tell Reza **"metro-linux READY (Nemo, GPU 24–40)"**, with the `nvidia-smi` figure.

## Phase 3: Nemo runs

While Reza runs prompts, record `nvidia-smi` and `free -g` about every 30 s, and anything new in `shard.log`. Commit and push `test(mistral): metro-linux nemo runs`.

## Phase 4: Mistral Small 24B (once Reza says go)

1. Download, again in two steps:
   ```bash
   export KWAAINET_HOME=$HOME/.kwaainet-mistral-dl
   $K config set model unsloth/Mistral-Small-24B-Instruct-2501
   $K shard download --start-block 24 --blocks 8
   $K shard download --start-block 24 --blocks 16    # must say [24, 40) of 40
   unset KWAAINET_HOME
   ```
   **PASS:** `model-00006` through `model-00010-of-00010.safetensors`, about 23.1 GB.
2. Switch:
   ```bash
   $K stop
   $K config set model unsloth/Mistral-Small-24B-Instruct-2501
   $K start --daemon --shard
   ```
   **PASS:** `Shard ready  (16 blocks, is_first=false, is_last=true)`, and `nvidia-smi` shows about 19–20 GB, under 23 GB.
   **If it runs out of GPU memory,** record the exact error and stop. Don't change blocks.
3. Commit and push `test(mistral): metro-linux small24b ready`. Tell Reza **"metro-linux READY (Small 24B)"**. During Reza's runs, record as in Phase 3.

## Phase 5: restore (only when Reza says so)

```bash
$K stop
cp ~/config.70b-backup.yaml ~/.kwaainet/config.yaml
cp ~/start-args.70b-backup.json ~/.kwaainet/run/start-args.json
kwaainet restart
kwaainet status
rm -rf ~/.kwaainet-mistral-dl ~/.kwaainet-70b-dl
```

**PASS:** the installed `kwaainet` is serving the 8B, blocks 24–32 on the GPU, as in the 70B test's Phase 1 baseline. Leave the weights in the cache unless Reza says otherwise. Commit and push `test(mistral): metro-linux restored`.
