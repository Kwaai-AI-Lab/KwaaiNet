# metro-win: blocks 8–40 on CPU

Read [README.md](README.md) first; its rules apply. Write everything to `tests/kwaai-compute/llama70b/results/metro-win.md`.

**Why this machine runs on CPU:** it has 190 GB of RAM, and Windows can run only one KwaaiNet node (the control socket is hard-coded to `127.0.0.1:5005`). A CPU node can hold 32 blocks (about 55 GB); the GPU could hold only 8. So the GPU sits idle for this test.

## Phase 1: baseline, backups, download (the 8B node keeps running)

1. Baseline:
   ```powershell
   kwaainet --version; kwaainet status
   nvidia-smi
   (Get-CimInstance Win32_OperatingSystem | Select-Object @{n='FreeGB';e={[math]::Round($_.FreePhysicalMemory/1MB,1)}})
   Get-PSDrive C
   ```
   Record all of it.
2. Backups:
   ```powershell
   Copy-Item $env:USERPROFILE\.kwaainet\config.yaml $env:USERPROFILE\config.70b-backup.yaml
   Copy-Item $env:USERPROFILE\.kwaainet\run\start-args.json $env:USERPROFILE\start-args.70b-backup.json
   Get-Content $env:USERPROFILE\start-args.70b-backup.json
   ```
   Record the start-args contents.
3. Download blocks 8–40 (about 55 GB) under a throwaway home:
   ```powershell
   $env:KWAAINET_HOME = "$env:USERPROFILE\.kwaainet-70b-dl"
   kwaainet config set model unsloth/Llama-3.3-70B-Instruct
   kwaainet shard download --start-block 8 --blocks 32
   Remove-Item Env:KWAAINET_HOME
   ```
   The model is set in config first because `shard download` takes the block total from config, and would otherwise clamp the range to 32 blocks.

   **PASS:** the download completes. The snapshot has `config.json`, `tokenizer.json` and the safetensors files containing `model.layers.8` through `model.layers.39`. Record the time taken and the snapshot path and size.
4. Commit and push: `test(70b): metro-win download`. Tell Reza the download is done.

## Phase 2: switch to the 70B (stops the 8B on metro-win)

Wait for Reza to say go.

1. Switch:
   ```powershell
   kwaainet stop
   kwaainet config set contribute.auto_update false
   kwaainet config set model unsloth/Llama-3.3-70B-Instruct
   kwaainet config set use_gpu false
   kwaainet config set start_block 8
   kwaainet config set blocks 32
   kwaainet start --daemon --shard
   ```
2. Watch the shard log until it reports ready:
   ```powershell
   Get-Content $env:USERPROFILE\.kwaainet\logs\shard.log -Tail 20 -Wait
   ```
   **PASS:** `Shard ready  (32 blocks, is_first=false, is_last=false)`, with no errors before it.
3. Record:
   - the load time;
   - the shard process's memory: `Get-Process kwaainet | Format-Table Id,@{n='WS_GB';e={[math]::Round($_.WS/1GB,1)}}`;
   - free RAM;
   - `nvidia-smi`, which should show no new GPU memory in use.
4. Commit and push: `test(70b): metro-win ready`. Tell Reza: **"metro-win READY: blocks 8–40 on CPU"**, with the memory figures.

## Phase 3: during Reza's runs

Leave the node running. Every 30 s or so while Reza runs prompts, record the shard process's memory and free RAM, and note whether anything swaps. Note anything new in `shard.log`, such as errors or "Message too large". Commit and push: `test(70b): metro-win runs`.

## Phase 4: restore (only when Reza says so)

1. Put the 8B node back:
   ```powershell
   kwaainet stop
   Copy-Item $env:USERPROFILE\config.70b-backup.yaml $env:USERPROFILE\.kwaainet\config.yaml -Force
   Copy-Item $env:USERPROFILE\start-args.70b-backup.json $env:USERPROFILE\.kwaainet\run\start-args.json -Force
   kwaainet restart
   kwaainet status
   Remove-Item -Recurse $env:USERPROFILE\.kwaainet-70b-dl
   ```
   **PASS:** the 8B node is running with the original flags, as in the Phase 1 baseline.
2. Leave the 70B weights (about 55 GB) in the Hugging Face cache unless Reza says to delete them.
3. Commit and push: `test(70b): metro-win restored`.
