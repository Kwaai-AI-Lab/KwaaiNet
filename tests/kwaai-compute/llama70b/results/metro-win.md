# metro-win results: Llama 3.3 70B, blocks 8–40 on CPU

- Branch `test/llama70b-heterogeneous` at `b829489209f2e3c9aba8a5d379984e1defaf3108`.
- Machine: metro-win, Windows 11 Pro for Workstations 10.0.22631, NVIDIA RTX A5000 (24 GB), 190.7 GB RAM.

## Phase 1: baseline, backups, download

Run on 2026-11-14, 17:24–17:56 local time (UTC−8).

### 1. Baseline: PASS (recorded)

```
kwaainet 0.7.3
  🟢 Status:  Running
  🔢 PID:     22564
  ⏱️  Uptime:  2d 25m 21s
  💻 CPU:     0.0%
  🧠 Memory:  220 MB
  🟢 Shard:   Running (PID 22436)
  🟢 Storage: Running (PID 14236, port 7432)
```

`nvidia-smi`, captured at 17:24:33:

```
NVIDIA-SMI 581.42   Driver Version: 581.42   CUDA Version: 13.0
GPU 0  NVIDIA RTX A5000  WDDM | 00000000:15:00.0 On
30%  31C  P8  14W / 230W | 18305MiB / 24564MiB | 6%  Default
Processes (C = compute): 22436  C  ...WIN11\.cargo\bin\kwaainet.exe  (the 8B shard)
                         plus desktop C+G processes (explorer, VS Code, Terminal, KwaaiNet GUI test build, …)
```

The 8B shard runs on the GPU: `shard.log` shows `Device: CUDA (GPU 0)`.

Free RAM and disk:

```
FreeGB
------
 175.3

Name Used (GB) Free (GB) Provider   Root
---- --------- --------- --------   ----
C       421.47    509.09 FileSystem C:\
```

### 2. Backups: PASS

```
Copy-Item ~\.kwaainet\config.yaml          ~\config.70b-backup.yaml        (1969 bytes, mtime 2026-11-12 16:58:21; SHA-256 matches the original)
Copy-Item ~\.kwaainet\run\start-args.json  ~\start-args.70b-backup.json
```

Before copying, I checked that neither target existed (`Test-Path` → False, False).

`start-args.70b-backup.json`:

```json
{
  "model": null,
  "blocks": null,
  "port": null,
  "grpc_port": null,
  "no_gpu": false,
  "public_name": null,
  "public_ip": null,
  "announce_addr": null,
  "identity_key": null,
  "no_relay": false,
  "shard": true,
  "no_contribute": false
}
```

### 3. Download blocks 8–40: PASS

```
$env:KWAAINET_HOME = "$env:USERPROFILE\.kwaainet-70b-dl"
kwaainet config set model unsloth/Llama-3.3-70B-Instruct
  INFO kwaainet::config: Created default config at C:\Users\METRO_WIN11\.kwaainet-70b-dl\config.yaml
  ✅ Set model = unsloth/Llama-3.3-70B-Instruct          (exit 0)
kwaainet shard download --start-block 8 --blocks 32
  Model: unsloth/Llama-3.3-70B-Instruct
  Blocks: [8, 40) of 80 (selective download)
  Commit: 99cd0d2c829e
  Files:  18 (selective for blocks [8, 40))
  Dest:   C:\Users\METRO_WIN11\.cache/huggingface/hub\models--unsloth--Llama-3.3-70B-Instruct\snapshots\99cd0d2c829e92a67c844f9144c2509632e5c87f
  [ 1/18] config.json  ✓  891 B
  [ 2/18] generation_config.json  ✓  234 B
  [ 3/18] model-00004-of-00030.safetensors  ✓  4.6 GB
  … [14/18] model-00015-of-00030.safetensors  ✓  4.3 GB   (12 weight files, 00004–00015)
  [15/18] model.safetensors.index.json  — already cached (58 KB)
  [16/18] special_tokens_map.json  ✓  454 B
  [17/18] tokenizer.json  ✓  16 MB
  [18/18] tokenizer_config.json  ✓  54 KB
  ✅ Saved to: C:\Users\METRO_WIN11\.cache/huggingface/hub\models--unsloth--Llama-3.3-70B-Instruct\snapshots\99cd0d2c829e92a67c844f9144c2509632e5c87f
download exit=0
Remove-Item Env:KWAAINET_HOME     (confirmed removed)
```

On the "NativeCommandError" for `config set`: PowerShell shows that when a native command writes to stderr (here, the INFO line). The command exited 0.

- **Time:** 29.4 min (17:25:01 → 17:54:26), about 33 MB/s on average.
- **Snapshot path:** `C:\Users\METRO_WIN11\.cache\huggingface\hub\models--unsloth--Llama-3.3-70B-Instruct\snapshots\99cd0d2c829e92a67c844f9144c2509632e5c87f`
- **Snapshot size:** 57,564,294,066 bytes, which is 57.6 GB or 53.6 GiB (`du -sbL`). The plan estimated about 55 GB. Whole safetensors files are fetched, so files 00004 and 00015 also carry tensors from outside 8–40.

**Contents check.** I mapped every `model.layers.N.*` tensor in `model.safetensors.index.json` to its file:

```
layers covered: 8 - 39 count 32
tensors for layers 8-39: 288
missing: 0 []
files needed  = files present = model-00004 … model-00015-of-00030.safetensors
num_hidden_layers: 80  torch_dtype: bfloat16
tokenizer.json: 17209920 bytes
```

`config.json` and `tokenizer.json` are present, and every tensor for `model.layers.8` through `model.layers.39` is in a downloaded file.

### Live node untouched: PASS

Checked after the download:

```
🟢 Status: Running   🔢 PID: 22564   ⏱️ Uptime: 2d 55m 40s   🟢 Shard: Running (PID 22436)   🟢 Storage: Running (PID 14236, port 7432)
~/.kwaainet/config.yaml          mtime 2026-11-12 16:58:21   cmp == config.70b-backup.yaml
~/.kwaainet/run/start-args.json  mtime 2026-11-12 16:59:12   cmp == start-args.70b-backup.json
free RAM 175.1 GB; C: free 455.22 GB
```

The throwaway home `~\.kwaainet-70b-dl\config.yaml` has `model: unsloth/Llama-3.3-70B-Instruct`, plus defaults (`blocks: 8`, `port: 8080`). It is not used by the live node, and Phase 4 removes it.

### Notes

- `config.yaml`'s mtime of 2026-11-12 16:58 predates this test. The live config was last rewritten by the CLI's own `restart` during the PR #272/#273 tests. It still holds `model: unsloth/Llama-3.1-8B-Instruct`, `blocks: 32`.
- A KwaaiNetGUI test build (`kwaainet-gui-fix-test\app5`, PID 9168) is open on the desktop from earlier testing.
