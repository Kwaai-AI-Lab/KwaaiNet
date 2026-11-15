# metro-win results: Mistral on three GPUs

Machine: metro-win, Windows 11 Pro for Workstations, NVIDIA RTX A5000 (24 GB, WDDM, driver 581.42, CUDA 13.0), 190 GB RAM. Installed `kwaainet 0.7.3`.

## Phase 0: leave the 70B: PASS

2026-11-14 20:23 local (04:23Z).

Backups are kept:

```
Name                       Length LastWriteTime
config.70b-backup.yaml       1969 11/12/2026 4:58:21 PM
start-args.70b-backup.json    250 11/12/2026 4:59:12 PM
```

`kwaainet stop`:

```
INFO kwaainet::daemon: Stopping daemon PID 15720
  ✅ KwaaiNet daemon stopped
```

`kwaainet status`: `🔴 Status: Not running`, `⚫ Shard: Not running`, `🔴 Storage: Not running`.

- `Get-Process kwaainet`: 0 processes.
- `nvidia-smi`: 687 MiB / 24564 MiB in use. The only processes are desktop C+G ones (explorer, VS Code, Windows Terminal, the KwaaiNet GUI window and similar); none is a kwaainet compute process.
- Free RAM is 176.3 GB, against 124.7 GB while the 70B shard was loaded.

Note: on Windows, `kwaainet stop` runs `taskkill /F` (`daemon.rs`, `stop_pid`), so the node exits without writing its `state = -1` tombstone. Its 70B records stay "online" in the DHT. Because the bootstraps keep serving a subkey after it expires, they can stay there long after the 360 s TTL. Both problems were found during the 70B test; this test did not cause them.

## Phase 1: build and download Nemo

### 1. Build

Worktree: `git worktree add ..\KwaaiNet-headdim origin/fix/config-head-dim`, at `e0d17e26 test(inference): cover non-square attention in MLX; keep the candle block test off flash-attn`. `git merge-base --is-ancestor 3e092c0c origin/fix/config-head-dim` succeeded, so it contains 3e092c0c.

`core\patches\` in the worktree had only the scripts and `.patch` files. Each `.patch` and `fetch-*.sh` was byte-identical (`cmp`) to the main clone's, so the patched `cudarc`, `libp2p-kad` and `multistream-select` sources were copied over from `C:\Users\METRO_WIN11\Source\KwaaiNet\core\patches\`.

**First attempt: failed on the build environment, not the code.** It ran in a plain shell with `CUDA_COMPUTE_CAP=80`, and stopped after 67 s:

```
nvcc fatal   : Cannot find compiler 'cl.exe' in PATH
Error: CompilationFailed { path: "src\affine.cu", message: "nvcc error:\n\n" }
```

The instructions assume MSVC is on `PATH`. `docs/runbooks/build-on-metro-win.md` says nvcc needs the MSVC environment and caps parallelism. So the second attempt runs from a `cmd` that first calls `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat`, with the same feature and compute capability:

```
set CUDA_COMPUTE_CAP=80
cargo build --release -p kwaainet --features cuda-windows -j 16
```

**Second attempt: PASS.** `Finished release profile [optimized] target(s) in 14m 37s`, ending 20:40:21 local, RC=0.

- `$K` = `C:\Users\METRO_WIN11\Source\KwaaiNet-headdim\core\target\release\kwaainet.exe`, 58,597,888 bytes.
- `& $K --version`: `kwaainet 0.7.3`. The branch has not bumped the version, so the commit is what identifies this build.
- `git -C .. log --oneline -1`: `e0d17e26`, which contains 3e092c0c.
- `dumpbin /dependents $K` lists `nvcuda.dll`, `cublas64_12.dll` and `curand64_10.dll`, so this is a CUDA build.
- The installed `kwaainet` is untouched: `~\.cargo\bin\kwaainet.exe` is 58,590,208 bytes, dated Nov 4, `kwaainet 0.7.3`.

### 2. Download: PASS

Run with `$env:KWAAINET_HOME = "$env:USERPROFILE\.kwaainet-mistral-dl"`, from 04:41:04Z to 04:46:36Z: **332 s**.

```
  Model: unsloth/Mistral-Nemo-Instruct-2407
  Blocks: [8, 24) of 32 (selective download)
  Commit: e9976040f42d
  Files:  8 (selective for blocks [8, 24))
  [ 1/8] config.json  ✓  728 B
  [ 2/8] generation_config.json  ✓  162 B
  [ 3/8] model-00002-of-00005.safetensors  ✓  4.6 GB
  [ 4/8] model-00003-of-00005.safetensors  ✓  4.6 GB
  [ 5/8] model.safetensors.index.json  — already cached (29 KB)
  [ 6/8] special_tokens_map.json  ✓  552 B
  [ 7/8] tokenizer.json  ✓  16 MB
  [ 8/8] tokenizer_config.json  ✓  177 KB
EXIT 0
```

**Snapshot:** `C:\Users\METRO_WIN11\.cache\huggingface\hub\models--unsloth--Mistral-Nemo-Instruct-2407\snapshots\e9976040f42d8a5590ed58cefe973061b3f973b5`. It is 9.2 GiB (9.83 GB):

| File | Bytes |
|---|---|
| `model-00002-of-00005.safetensors` | 4,907,529,424 |
| `model-00003-of-00005.safetensors` | 4,907,529,456 |
| `tokenizer.json` | 17,078,292 |
| `model.safetensors.index.json` | 29,894 |
| `config.json` | 728 |

**Contents check.** I mapped every `model.layers.N.*` tensor with 8 ≤ N < 24 in `model.safetensors.index.json` to its file:
- `model-00002` holds layers 8–15;
- `model-00003` holds layers 15–23;
- every layer from 8 to 23 is covered, and no file is missing.

`.index.json` was "already cached", from an earlier fetch of this repo's index.

**The live home was not touched:** `~\.kwaainet\config.yaml` is still dated 18:58 (the 70B test's switch). The throwaway `config set model` wrote only to `~\.kwaainet-mistral-dl\config.yaml`.

**metro-win BUILT+DOWNLOADED.**

## Phase 2: serve Nemo: PASS

Live home (`KWAAINET_HOME` unset). Every command ran with `$K`: `config set contribute.auto_update false`, `model unsloth/Mistral-Nemo-Instruct-2407`, `use_gpu true`, `start_block 8`, `blocks 16`, then `start --daemon --shard` at 05:28:45.6Z. The daemon got PID 23972.

New lines in `shard.log`:

```
05:28:55.653Z WARN kwaainet::shard_cmd: inference-mux server failed to start after 5 attempts: register inference-mux stream handler
  Blocks:      [8, 24)
  Device:      CUDA (GPU 0)
  Model:       unsloth/Mistral-Nemo-Instruct-2407
  ✅ Node registered on protocol /kwaai/inference/1.0.0 — appearing on map.
  💡 Rebalancing disabled — this node pins blocks [8, 24). Unset start_block to allow moves.
  💡 Loading shard (2 file(s), blocks [8, 24))…
05:28:55.660Z INFO kwaai_inference::shard: Loading shard [8..24) of 40: hidden=5120 heads=32 (8 kv)
05:29:02.728Z INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=131072, bos=Some(1), eos=Some(2)
05:29:02.880Z INFO kwaai_inference::shard: Shard [8..24) ready — embedding=false final_head=false
  ✅ Shard ready  (16 blocks, is_first=false, is_last=false)
```

- `Device: CUDA (GPU 0)`.
- `Shard ready (16 blocks, is_first=false, is_last=false)`.
- `grep -cE "ERROR|panicked|shape mismatch"` over the new lines returns **0**, so there is no `q_proj: shape mismatch`.
- The loader reports `of 40` layers, which is Nemo's real depth. It is not the `of 32` name-based guess that `shard download` printed.
- The inference-mux WARN is the same one the 8B and 70B nodes logged on every start, each time followed by "Shard ready". It is not new on this branch.

**Load time:**
- `Loading shard` to `ready`: 7.2 s (05:28:55.66 → 05:29:02.88).
- `start` to ready: 17.3 s.

**GPU** (`nvidia-smi`, about 05:34Z):
- 10,125 MiB / 24,564 MiB in use, against 687 MiB at the Phase 0 baseline, so **≈ 9.4 GB for the shard**.
- `20536 C:\...\KwaaiNet-headdim\core\target\release\kwaainet.exe` is among the compute apps. WDDM doesn't report per-process GPU memory, so it shows `[N/A]`.

**Processes:** all three run from `$K`:
- 23972 `run-node --shard`;
- 23400 `storage serve`;
- 20536 `shard serve --auto-rebalance`.

The installed `kwaainet` is not running.

**Map** (`/api/v1/state`, crawl of 05:34:35Z):
- `model_reports` for `Mistral-Nemo-Instruct-2407` is **healthy**: professor_reza 0–8, metro-win-rtx-a5000 8–24 and metro-linux 24–40, all online.
- metro-win's `peers` row still says `Llama-3.1-8B-Instruct`, blocks 0–32. Reza saw this on the map page too. It is not a Nemo problem. It comes from two existing bugs:
  - The bootstraps still serve metro-win's 8B subkey long after it expired. It last read `state=2` with an expiration 56 minutes in the past, because the dictionary is kept fresh by other peers and expired subkeys inside it are never dropped.
  - A Windows `kwaainet stop` is `taskkill /F`, so no `state = -1` tombstone ever replaced it. The map's `build_snapshot` then gives the peer row the first online model, which is the 8B.
- The map's `model_reports` also still lists metro-win under the 8B, as 0–32 online.
- The GUI, which reads the node directly, shows the Nemo shard loaded.

**metro-win READY (Nemo, GPU 8–24).**

## Phase 3: Nemo runs: PASS

Sampled every 30 s from 05:36:04Z to 05:57:00Z (42 samples): `nvidia-smi` memory and utilisation, shard liveness (PID 20536), and new `shard.log` lines.

- **GPU memory:** 10,116–10,162 MiB throughout, flat. KV cache added at most about 40 MiB.
- **Utilisation:** peaked at 97% (the prefill at 05:38:07). It sat around 7–28% during decode.
- **Shard:** alive in every sample. No ERROR, panic or "Message too large".
- **The only non-timing lines** were three `GC evicted 1 stale sessions` (05:50, 05:52, 05:55).

Runs, from `kwaainet::block_rpc: hop timing` (a gap of more than 5 s starts a new run):

| Run | Time (Z) | Steps through 8–24 | Wall | Steps/s | First (prefill) step |
|---|---|---|---|---|---|
| 1 | 05:37:55–05:39:42 | 201 | 107 s | 1.88 | 536 ms |
| 2 | 05:40:32–05:42:13 | 200 | 101 s | 1.96 | 274 ms |
| 3 | 05:43:26–05:45:06 | 200 | 101 s | 1.98 | 286 ms |
| 4 | 05:51:37–05:53:51 | 423 | 134 s | 3.14 | 234 ms |

**metro-win compute** per step, over all 1,024 steps: median 44.2 ms, p90 75.7 ms. At about 2 steps/s end to end, most of each token's time is spent outside metro-win.

Runs 1–3 are the three prompts in the KwaaiNet GUI on this machine, each cut at exactly 200 tokens. Run 4's source wasn't recorded here: 423 steps at 3.1/s could be the Mac's run, or two sessions at once.

**GUI review, runs 1–3.** I took a screenshot at 05:45:06Z.
- Answer 1 ("Summarize the critique of the standard model of physics?") is fluent and on topic, so the head-dim fix holds. It is cut off mid-sentence.
- Answer 2 ("How do civiliations collapse?") repeats the question for 200 tokens.
- Answer 3 ("Is that your real hair?") rambles as a persona.

Cause: `run_streaming` sends Mistral prompts with no chat template, samples at temperature 1.0 with no top-k, top-p or repetition penalty, and stops silently at a default of 200 tokens. Filed as **#293**.

Filed alongside: **#294**. The map shows metro-win on the 8B because the bootstraps serve expired dictionary subkeys, and a Windows stop writes no tombstone.
