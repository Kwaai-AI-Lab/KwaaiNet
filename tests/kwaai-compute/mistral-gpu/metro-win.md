# metro-win: blocks 8–24 on the GPU (CUDA)

Read [README.md](README.md) first; its rules apply. Write everything to `tests/kwaai-compute/mistral-gpu/results/metro-win.md`.

## Phase 0: leave the 70B

```powershell
kwaainet stop
kwaainet status
nvidia-smi
```

**PASS:** the node is stopped and no `kwaainet` process holds GPU or RAM. Keep `~\config.70b-backup.yaml` and `~\start-args.70b-backup.json`.

## Phase 1: build and download Nemo

1. Build from the code branch. Use a separate worktree, so this checkout stays on the test branch:
   ```powershell
   git fetch origin
   git worktree add ..\KwaaiNet-headdim origin/fix/config-head-dim
   cd ..\KwaaiNet-headdim\core
   # core\patches\ is git-ignored: copy or junction it from your main clone if it is missing
   $env:CUDA_COMPUTE_CAP = "80"
   cargo build --release -p kwaainet --features cuda-windows
   $K = "$PWD\target\release\kwaainet.exe"
   & $K --version
   git -C .. log --oneline -1          # expect 3e092c0c or later
   ```
   These are the same feature and compute capability as the release CUDA build. Record the build time and the commit.
2. Download blocks 8–24 under a throwaway home:
   ```powershell
   $env:KWAAINET_HOME = "$env:USERPROFILE\.kwaainet-mistral-dl"
   & $K config set model unsloth/Mistral-Nemo-Instruct-2407
   & $K shard download --start-block 8 --blocks 16
   Remove-Item Env:KWAAINET_HOME
   ```
   **PASS:**
   - the output says `Blocks: [8, 24) of 32`. 32 is the name-based guess and is harmless here, since 24 < 32;
   - `model-00002` and `model-00003-of-00005.safetensors`, `config.json` and `tokenizer.json` are present, about 9.8 GB in all.

   Record the time and the snapshot path.
3. Commit and push `test(mistral): metro-win built+downloaded`. Tell Reza.

## Phase 2: serve Nemo (once Reza says go)

```powershell
& $K config set contribute.auto_update false
& $K config set model unsloth/Mistral-Nemo-Instruct-2407
& $K config set use_gpu true
& $K config set start_block 8
& $K config set blocks 16
& $K start --daemon --shard
Get-Content $env:USERPROFILE\.kwaainet\logs\shard.log -Tail 20 -Wait
```

**PASS:**
- the log has `Device: CUDA`, then `Shard ready  (16 blocks, is_first=false, is_last=false)`, with no load error. In particular there is no `q_proj: shape mismatch`, which is the bug the branch fixes;
- `nvidia-smi` shows the shard using about 9 GB.

Record the load time and `nvidia-smi`. Commit and push `test(mistral): metro-win nemo ready`. Tell Reza **"metro-win READY (Nemo, GPU 8–24)"**.

## Phase 3: Nemo runs

While Reza runs prompts, record `nvidia-smi` about every 30 s, and anything new in `shard.log`. Commit and push `test(mistral): metro-win nemo runs`.

## Phase 4: Mistral Small 24B (once Reza says go)

1. Download. The 8B and Nemo nodes are not involved:
   ```powershell
   $env:KWAAINET_HOME = "$env:USERPROFILE\.kwaainet-mistral-dl"
   & $K config set model unsloth/Mistral-Small-24B-Instruct-2501
   & $K shard download --start-block 8 --blocks 16
   Remove-Item Env:KWAAINET_HOME
   ```
   **PASS:** `model-00003` through `model-00006-of-00010.safetensors`, about 19.2 GB.
2. Switch:
   ```powershell
   & $K stop
   & $K config set model unsloth/Mistral-Small-24B-Instruct-2501
   & $K start --daemon --shard
   ```
   **PASS:** `Shard ready  (16 blocks, …)`, and `nvidia-smi` shows about 18 GB, with total GPU use under 23 GB.
3. Commit and push `test(mistral): metro-win small24b ready`. Tell Reza **"metro-win READY (Small 24B)"**. During Reza's runs, record as in Phase 3.

## Phase 5: restore (only when Reza says so)

```powershell
& $K stop
Copy-Item $env:USERPROFILE\config.70b-backup.yaml $env:USERPROFILE\.kwaainet\config.yaml -Force
Copy-Item $env:USERPROFILE\start-args.70b-backup.json $env:USERPROFILE\.kwaainet\run\start-args.json -Force
kwaainet restart
kwaainet status
Remove-Item -Recurse $env:USERPROFILE\.kwaainet-mistral-dl, $env:USERPROFILE\.kwaainet-70b-dl
```

**PASS:** the installed `kwaainet` is serving the 8B on the GPU, as in the 70B test's Phase 1 baseline. Leave the Mistral weights in the cache unless Reza says otherwise. Commit and push `test(mistral): metro-win restored`.
