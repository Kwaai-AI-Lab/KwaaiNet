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
