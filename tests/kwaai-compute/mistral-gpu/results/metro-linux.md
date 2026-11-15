# metro-linux results

Run by a Claude Code session on metro-linux (RHEL 9, RTX A5000 24 GB, driver 595.99.02, 93 GiB RAM), on branch `test/mistral-gpu-3node` at `e14c71e0`. Times are UTC, 2026-11-15.

## Phase 0: leave the 70B

Before starting, I stopped the 30 s memory sampler left over from the 70B test. Both 70B backups were present, and their checksums match the ones taken at the 70B test's Phase 1 (`614868f0…` for the config, `96fa4abd…` for start-args).

```
## node A shell
KWAAINET_GRPC_PORT=8094
KWAAINET_HOME=/home/rezarassool/.kwaainet-70b-cpu
KWAAINET_PORT=8081
KWAAINET_SOCKET=/home/rezarassool/.kwaainet-70b-cpu/node.sock
$ kwaainet stop  [04:24:02Z]
│                      🛑 Stopping KwaaiNet Node                      │
2026-11-15T04:24:02.088459Z  INFO kwaainet::daemon: Stopping daemon PID 1079739
  ✅ KwaaiNet daemon stopped
─────────────────────────────────────────────────────────────────────
[exit 0]

## clean shell
$ kwaainet stop  [04:24:04Z]
│                      🛑 Stopping KwaaiNet Node                      │
2026-11-15T04:24:04.691815Z  INFO kwaainet::daemon: Stopping daemon PID 1064630
  ✅ KwaaiNet daemon stopped
─────────────────────────────────────────────────────────────────────
[exit 0]

$ rm -rf ~/.kwaainet-70b-cpu  [04:24:09Z]
[exit 0]

$ rm -rf ~/kwaainet-cpu ~/kwaainet-cpu-target  [04:24:09Z]
[exit 0]

$ pgrep -fa kwaainet  [04:24:09Z]
[exit 0]

$ nvidia-smi  [04:24:09Z]
Sat Nov 14 20:24:09 2026       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 595.99.02              Driver Version: 595.99.02      CUDA Version: 13.2     |
+-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  NVIDIA RTX A5000               Off |   00000000:15:00.0  On |                  Off |
| 30%   34C    P5             26W /  230W |     443MiB /  24564MiB |      5%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+
+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI              PID   Type   Process name                        GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|    0   N/A  N/A           10771      G   /usr/libexec/Xorg                        93MiB |
|    0   N/A  N/A           10870      G   /usr/bin/gnome-shell                    178MiB |
|    0   N/A  N/A          166999      G   ...rack-uuid=3190708988185955192        100MiB |
+-----------------------------------------------------------------------------------------+
[exit 0]

$ free -g  [04:24:10Z]
               total        used        free      shared  buff/cache   available
Mem:              93           7          52           0          35          86
Swap:              3           3           0
[exit 0]

$ ls -d ~/.kwaainet-70b-cpu ~/kwaainet-cpu ~/kwaainet-cpu-target  [04:24:10Z]
ls: cannot access '/home/rezarassool/.kwaainet-70b-cpu': No such file or directory
ls: cannot access '/home/rezarassool/kwaainet-cpu': No such file or directory
ls: cannot access '/home/rezarassool/kwaainet-cpu-target': No such file or directory
[exit 2]

$ sha256sum ~/config.70b-backup.yaml ~/start-args.70b-backup.json  [04:24:10Z]
614868f0a41a0994863e00a03efa873ffc63132d97e6cd78be6e145ffbec2e00  /home/rezarassool/config.70b-backup.yaml
96fa4abd885c06f42f7ebfada653e1fe0832f1c2a6f2e81b65339e4d8dfe828a  /home/rezarassool/start-args.70b-backup.json
[exit 0]

```

`pgrep -fa kwaainet` exited 0 without printing anything. The match was the wrapper shell running it, because its own command line contains "kwaainet". A check that cannot match itself confirms nothing is left:

```
$ ps -eo comm | grep -c '^kwaainet$'
0
$ ss -ltn | grep -E ':(8080|8081|8093|8094)\b'      # no output: the ports are free
```

`~/.kwaainet-70b-dl`, the 70B download home, is left for Phase 5, which removes it. The 70B weights stay in the HF cache.

**PASS:**
- No `kwaainet` process remains.
- The GPU is at its 443 MiB desktop baseline: Xorg, gnome-shell and one browser process.
- RAM is 86 GiB available.
- `~/.kwaainet-70b-cpu`, `~/kwaainet-cpu` and `~/kwaainet-cpu-target` are deleted.
- The 70B backups are kept, unchanged.

## Phase 1: build and download Nemo

### 1. Build

**The instructed build fails on this host.** metro-linux has no CUDA toolkit: there is no `nvcc` and no `/usr/local/cuda`. This is a known fact about this machine; CUDA builds here have only ever worked inside the manylinux container or under Nix.

The worktree is `../KwaaiNet-headdim` at `origin/fix/config-head-dim`, `e0d17e26`, which includes `3e092c0c`. `core/patches/` exists in the worktree, but only with its tracked scripts and `.patch` files. So I symlinked the three git-ignored directories (`cudarc`, `libp2p-kad`, `multistream-select`) from the main clone. The note's "symlink `core/patches/`" would put the link inside the existing directory.

```
$ cd ../KwaaiNet-headdim/core
$ CUDA_COMPUTE_CAP=80 CUDAFORGE_THREADS=4 RAYON_NUM_THREADS=4 systemd-run --user --scope -q -p MemoryMax=40G -p MemorySwapMax=0 \
    cargo build --release -p kwaainet --features cuda -j 8
error: failed to run custom build command for `cudarc v0.19.7 (/home/rezarassool/Documents/KwaaiNet-headdim/core/patches/cudarc)`
  ...
  `nvcc --version` failed.
  Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
cargo exit 101 in 48s
```

The memory caps and the cudaforge thread limits were added because uncapped CUDA builds have run this box out of memory. `--features cuda-no-flash` is no way round it, since cudarc and candle-kernels need `nvcc` too.

**What was built instead, with Reza's choice:** the flake's `kwaainet-cuda` package, the Nix CUDA package from #260, built from the same worktree. It brings its own CUDA 12.8 and needs no host toolkit. It builds with `--features cuda`, which includes flash-attn, for **compute capability 8.6**, the A5000's native capability and the flake's default, not the release's 8.0.

The exact steps to repeat it:

```bash
git fetch origin
git worktree add ../KwaaiNet-headdim origin/fix/config-head-dim
cd ../KwaaiNet-headdim
# The core/patches/ symlinks were in place but untracked, and a flake built from a git tree sees only tracked files,
# so this build did not use them. crane.nix supplies the patched crates itself.
systemd-run --user --scope -q -p MemoryMax=48G -p MemorySwapMax=0 \
  nix build .#kwaainet-cuda --cores 16 --max-jobs 1 -L -o result-cuda
K=$(readlink -f result-cuda)/bin/kwaainet
```

Build log summary:

```
start 04:26:27Z at e0d17e26
building '/nix/store/3k0hnnbmfzhp6yid37pwj1pjwv8v0b9i-kwaainet-deps-0.0.0.drv'...
kwaainet-deps>     Finished `release` profile [optimized] target(s) in 18m 53s
kwaainet-deps>     Finished `release` profile [optimized] target(s) in 2m 41s
kwaainet-deps>     Finished `release` profile [optimized] target(s) in 51.54s
building '/nix/store/llpfdikqgzk86dr16pr81cxrncy9whf9-kwaainet-0.7.3.drv'...
kwaainet>     Finished `release` profile [optimized] target(s) in 15m 45s
nix exit 0 in 2314s, end 05:05:01Z
/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3
```

The build took **2314 s (38.5 min)**:
- 22.5 min of CUDA dependencies. These were not cached, because the dependency derivation for this lock had not been built here before.
- 15.75 min for the kwaainet package, mostly link-time optimisation.

It stayed within the 48 GB cap. The 8B node was stopped throughout.

The commit is `e0d17e26`, and **`K=/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet`**. The installed `~/.cargo/bin/kwaainet` was not touched.

#### Does `$K` see the GPU outside Nix?

A Nix-built binary does not search the host's library paths, so on a non-NixOS host it can pick up the toolkit's stub `libcuda`. That fails at CUDA initialisation, or falls back silently to CPU. The package works round this in `distrib/nix/crane.nix`'s `postFixup`: it symlinks only `libcuda.so.1`, `libnvidia-ml.so.1` and `libnvidia-ptxjitcompiler.so.1` from `cudaDriverLibPath` (`/usr/lib64`, set in `flake.nix`) into a private shim directory placed first on the RUNPATH. It does not put `/usr/lib64` itself on the RUNPATH, because the host's glibc there would shadow Nix's.

I checked it from a plain shell, with no Nix environment and no `LD_LIBRARY_PATH`:

```
K=/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet
(no LD_LIBRARY_PATH / KWAAINET_* set)

$ ls -l /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/
total 187504
-r-xr-xr-x. 1 rezarassool rezarassool 192001632 Dec 31  1969 kwaainet
[exit 0]

$ file -L /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet | cut -c1-200
/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /nix/store/jms7zxzm7w1whczwny5m3gkgdjg
[exit 0]

$ /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet --version
kwaainet 0.7.3
[exit 0]

$ git -C /home/rezarassool/Documents/KwaaiNet-headdim log --oneline -1
e0d17e26 test(inference): cover non-square attention in MLX; keep the candle block test off flash-attn
[exit 0]

$ ldd /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet | grep -i -E 'cuda|cublas|curand|nvidia|not found'
	libnvidia-ptxjitcompiler.so.1 => /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/libexec/cuda-driver-shim/libnvidia-ptxjitcompiler.so.1 (0x00007f8339c00000)
	libcudart.so.12 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcudart.so.12 (0x00007f8339800000)
	libcuda.so.1 => /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/libexec/cuda-driver-shim/libcuda.so.1 (0x00007f8333c00000)
	libcurand.so.10 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcurand.so.10 (0x00007f832b000000)
	libcublas.so.12 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcublas.so.12 (0x00007f8323e00000)
	libcublasLt.so.12 => /nix/store/3zp1b44ylx3q204yvmbp6c01rg7f1i3z-cuda12.8-libcublas-12.8.4.1-lib/lib/libcublasLt.so.12 (0x00007f82f1600000)
[exit 0]

$ patchelf --print-rpath /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet 2>/dev/null || readelf -d /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet | grep -E 'RUNPATH|RPATH'
 0x000000000000001d (RUNPATH)            Library runpath: [/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/libexec/cuda-driver-shim:/nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib:/nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/stubs:/nix/store/jms7zxzm7w1whczwny5m3gkgdjghmi2r-glibc-2.42-51/lib:/nix/store/ab3753m6i7isgvzphlar0a8xb84gl96i-gcc-15.2.0-lib/lib]
[exit 0]

$ ls -l .../libexec/cuda-driver-shim/
total 0
lrwxrwxrwx. 1 rezarassool rezarassool 23 Dec 31  1969 libcuda.so.1 -> /usr/lib64/libcuda.so.1
lrwxrwxrwx. 1 rezarassool rezarassool 28 Dec 31  1969 libnvidia-ml.so.1 -> /usr/lib64/libnvidia-ml.so.1
lrwxrwxrwx. 1 rezarassool rezarassool 40 Dec 31  1969 libnvidia-ptxjitcompiler.so.1 -> /usr/lib64/libnvidia-ptxjitcompiler.so.1

$ readlink -f .../cuda-driver-shim/libcuda.so.1
/usr/lib64/libcuda.so.595.99.02

$ KWAAINET_HOME=$(mktemp -d) RUST_LOG=info $K generate no-such-model:probe x
│                        🧠 KwaaiNet Generate                         │
  Model:  no-such-model:probe
  Prompt: "x"
2026-11-15T05:05:21.714190Z  INFO kwaai_inference: CUDA device detected
2026-11-15T05:05:22.053601Z  INFO kwaai_inference::engine: Inference engine initialised on Cuda(0)
  ❌ Cannot locate Ollama manifest for 'no-such-model:probe'
[exit 0]
files written to the temp home: 1
```

The `generate` probe initialises the inference engine before it looks up the model. So it shows CUDA works without loading anything, and it wrote only to a temporary home, which was then removed. Note that `libcuda.so.1` resolves to the shim, and the shim to `/usr/lib64/libcuda.so.595.99.02`, the real driver, not the stub in `cuda-merged-12.8/lib/stubs`, which comes later on the RUNPATH.

Phase 2 still has to show it under load: `shard.log` must say `Device: CUDA`, and `nvidia-smi` must show the shard process itself holding about 10 GB. A CPU fallback would also load and look healthy, as node A did in the 70B test.

**PASS (built by a different route than instructed):**
- `$K` is built from `e0d17e26` with `--features cuda`, including flash-attn, for compute capability 8.6.
- `ldd` shows no `not found`.
- CUDA initialises on `Cuda(0)` through the real driver.

**Correction for `metro-linux.md` Phase 1.1:** this box cannot run a host `cargo build --features cuda`. Use the Nix steps above, or the manylinux and CUDA 12.6 container that release CI uses.

### 2. Download, blocks 24–40, under `KWAAINET_HOME=~/.kwaainet-mistral-dl`

`$K` did not exist yet, so the download ran with the installed `kwaainet` 0.7.3. `fix/config-head-dim` changes only `kwaai-inference/src/shard.rs` and `mlx_shard.rs` (`git diff --stat c48c3b8e origin/fix/config-head-dim`), so the download code is the same. The steps ran in parallel with the build.

```
KWAAINET_HOME=/home/rezarassool/.kwaainet-mistral-dl
$ kwaainet config set model unsloth/Mistral-Nemo-Instruct-2407  [start 04:25:04Z]
2026-11-15T04:25:05.031293Z  INFO kwaainet::config: Created default config at /home/rezarassool/.kwaainet-mistral-dl/config.yaml
│                      ⚙️  Configuration Updated                      │
  ✅ Set model = unsloth/Mistral-Nemo-Instruct-2407
─────────────────────────────────────────────────────────────────────
[exit 0 in 1s, end 04:25:05Z]

$ kwaainet shard download --start-block 24 --blocks 8  [start 04:25:05Z]
│                    Downloading HuggingFace Model                    │
  Model: unsloth/Mistral-Nemo-Instruct-2407
  Blocks: [24, 32) of 32 (selective download)
  Commit: e9976040f42d
  Files:  9 (selective for blocks [24, 32))
  Dest:   /home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5
  [ 1/9] config.json  ✓  728 B
  [ 2/9] generation_config.json  ✓  162 B
  [ 3/9] model-00003-of-00005.safetensors  ✓  4.6 GB
  [ 4/9] model-00004-of-00005.safetensors  ✓  4.6 GB
  [ 5/9] model-00005-of-00005.safetensors  ✓  4.6 GB
  [ 6/9] model.safetensors.index.json  — already cached (29 KB)
  [ 7/9] special_tokens_map.json  ✓  552 B
  [ 8/9] tokenizer.json  ✓  16 MB
  [ 9/9] tokenizer_config.json  ✓  177 KB
  ✅ Saved to: /home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5
  💡 Start serving: kwaainet shard serve --model-path "/home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5"
─────────────────────────────────────────────────────────────────────
[exit 0 in 917s, end 04:40:22Z]

$ kwaainet shard download --start-block 24 --blocks 16  [start 04:40:22Z]
│                    Downloading HuggingFace Model                    │
  Model: unsloth/Mistral-Nemo-Instruct-2407
  Blocks: [24, 40) of 40 (selective download)
  Commit: e9976040f42d
  Files:  9 (selective for blocks [24, 40))
  Dest:   /home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5
  [ 1/9] config.json  — already cached (728 B)
  [ 2/9] generation_config.json  — already cached (162 B)
  [ 3/9] model-00003-of-00005.safetensors  — already cached (4.6 GB)
  [ 4/9] model-00004-of-00005.safetensors  — already cached (4.6 GB)
  [ 5/9] model-00005-of-00005.safetensors  — already cached (4.6 GB)
  [ 6/9] model.safetensors.index.json  — already cached (29 KB)
  [ 7/9] special_tokens_map.json  — already cached (552 B)
  [ 8/9] tokenizer.json  — already cached (16 MB)
  [ 9/9] tokenizer_config.json  — already cached (177 KB)
  ✅ Saved to: /home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5
  💡 Start serving: kwaainet shard serve --model-path "/home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5"
─────────────────────────────────────────────────────────────────────
[exit 0 in 0s, end 04:40:22Z]

overall exit 0
```

The first step took 917 s (15.3 min), at about 16 MB/s. The second step reported `[24, 40) of 40` and had nothing left to fetch: all 9 files it selected were already cached.

I checked the index's `weight_map` against each file's safetensors header:

```
728 config.json
162 generation_config.json
4907529456 model-00003-of-00005.safetensors
4907529456 model-00004-of-00005.safetensors
4907496272 model-00005-of-00005.safetensors
29894 model.safetensors.index.json
552 special_tokens_map.json
181364 tokenizer_config.json
17078292 tokenizer.json
14G	/home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5
config: num_hidden_layers 40 hidden 5120 heads 32 head_dim 128
needed tensors 146 files ['model-00003-of-00005.safetensors', 'model-00004-of-00005.safetensors', 'model-00005-of-00005.safetensors']
lm_head.weight -> model-00005-of-00005.safetensors | model.norm.weight -> model-00005-of-00005.safetensors
absent from headers 0
layers 24 - 39 count 16
```

The snapshot is `/home/rezarassool/.cache/huggingface/hub/models--unsloth--Mistral-Nemo-Instruct-2407/snapshots/e9976040f42d8a5590ed58cefe973061b3f973b5`.

`config.json` has `head_dim: 128`, while `hidden_size / num_attention_heads` is 5120 / 32 = 160. That mismatch is the case `fix/config-head-dim` is for.

**PASS:**
- The second download says `Blocks: [24, 40) of 40`.
- `model-00003` through `model-00005-of-00005.safetensors` total 14.72 GB, alongside `config.json` and `tokenizer.json`.
- `lm_head.weight` and `model.norm.weight` are in `model-00005`, and all 146 tensors for layers 24–39 are present.

### Phase 1 summary

- Phase 0, leave the 70B: **PASS**
- Build, instructed `cargo build --features cuda`: **FAIL**. There is no `nvcc` on this host.
- Build via Nix `kwaainet-cuda`, at Reza's choice: **PASS**. It took 38.5 min, uses compute capability 8.6, and CUDA initialises through the host driver.
- Download, blocks 24–40: **PASS**. It took 15.3 min and is 14.72 GB.

**metro-linux BUILT+DOWNLOADED.**

## Phase 2: serve Nemo

Run with `K=/nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet` in the default home `~/.kwaainet`, with no `KWAAINET_*` variables set and no kwaainet process running beforehand.

```
env: 0 KWAAINET_* vars set
$ $K config set contribute.auto_update false
  ✅ Set contribute.auto_update = false
[exit 0]
$ $K config set model unsloth/Mistral-Nemo-Instruct-2407
  ✅ Set model = unsloth/Mistral-Nemo-Instruct-2407
[exit 0]
$ $K config set use_gpu true
  ✅ Set use_gpu = true
[exit 0]
$ $K config set start_block 24
  ✅ Set start_block = 24
[exit 0]
$ $K config set blocks 16
  ✅ Set blocks = 16
[exit 0]

$ diff ~/config.70b-backup.yaml ~/.kwaainet/config.yaml
1,2c1,2
< model: unsloth/Llama-3.1-8B-Instruct
< blocks: 8
---
> model: unsloth/Mistral-Nemo-Instruct-2407
> blocks: 16
54a55,57
> contribute:
>   storage: true
>   auto_update: false
```

The diff is against the 8B backup, which Phase 5 restores. `start_block: 24` and `use_gpu: true` match the 8B's own values, so they do not show. The `contribute:` section is the one first added in the 70B test.

```
$ $K start --daemon --shard  [05:28:29Z]
│                       🗺  Reading Network Map                        │
  Local models (1):
    • llama3.1:8b
  Network map (2 model(s)):
    Llama-3.1-8B-Instruct                       12 server(s)  ✓ have locally
    Mistral-Nemo-Instruct-2407                   1 server(s)  ✗ not installed
  ✅ Confirmed model: unsloth/Mistral-Nemo-Instruct-2407
    Map entry:  Llama-3.1-8B-Instruct  (12 server(s))
─────────────────────────────────────────────────────────────────────
│                      🚀 Starting KwaaiNet Node                      │
  Model:   unsloth/Mistral-Nemo-Instruct-2407
  Blocks:  16
  Port:    8080
  Peers:   2
  Name:    metro-linux
─────────────────────────────────────────────────────────────────────
  ✅ KwaaiNet daemon started (PID 1113257)
  ✅ Shard serving: the daemon starts and supervises it
  💡 Shard logs:   kwaainet logs --shard
  💡 Storage not initialised — skipping. Run: kwaainet storage init
  💡 Check status: kwaainet status
  💡 View logs:    kwaainet logs
  💡 Stop daemon:  kwaainet stop
─────────────────────────────────────────────────────────────────────
[exit 0]
```

`start` again prints `Map entry: Llama-3.1-8B-Instruct` after confirming a different model, the same cosmetic mismatch seen in the 70B test.

`shard.log` from this start, with blocks 25–38 omitted:

```
2026-11-15T05:28:30.531615Z  INFO kwaai_inference: CUDA device detected
2026-11-15T05:28:30.846362Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 500ms
2026-11-15T05:28:31.348131Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 1s
2026-11-15T05:28:32.349496Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 2s
2026-11-15T05:28:34.350820Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 4s
2026-11-15T05:28:38.352259Z  WARN kwaainet::shard_cmd: inference-mux server failed to start after 5 attempts: register inference-mux stream handler
│                      🧩 KwaaiNet Shard Server                       │
  Blocks:      [24, 40)
  Device:      CUDA (GPU 0)
  Model:       unsloth/Mistral-Nemo-Instruct-2407
  ✅ Node registered on protocol /kwaai/inference/1.0.0 — appearing on map.
  💡 Loading model in background. Requests return 'warming up' until ready.
─────────────────────────────────────────────────────────────────────
  💡 Local bypass server on 127.0.0.1:37547
  💡 Rebalancing disabled — this node pins blocks [24, 40). Unset start_block to allow moves.
  💡 Loading shard (3 file(s), blocks [24, 40))…
2026-11-15T05:28:38.353129Z  INFO kwaai_inference::shard: Loading shard [24..40) of 40: hidden=5120 heads=32 (8 kv)
2026-11-15T05:28:38.353517Z  INFO kwaai_inference::shard:   Loading block 24
2026-11-15T05:28:40.325836Z  INFO kwaai_inference::shard:   Loading block 39
2026-11-15T05:28:40.454312Z  INFO kwaai_inference::shard:   Loading norm + lm_head
2026-11-15T05:28:41.477971Z  INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=131072, bos=Some(1), eos=Some(2)
2026-11-15T05:28:41.687449Z  INFO kwaai_inference::shard: Shard [24..40) ready — embedding=false final_head=true
  ✅ Shard ready  (16 blocks, is_first=false, is_last=true)
2026-11-15T05:28:42.043934Z  INFO kwaainet::daemon: Sent SIGHUP to daemon PID 1113257 — re-announce will follow
```

The `inference-mux server failed to start` warning is the same pre-existing collision with the daemon's own inference-mux recorded in the 70B test. The shard still loads and registers.

GPU memory sampled every 2 s during the load, including the shard process's own figure:

```
05:28:29 avail=87708M gpu_total=449 MiB procs=[]
05:28:31 avail=87412M gpu_total=673 MiB procs=[1113360, 216 MiB;]
05:28:34 avail=87428M gpu_total=673 MiB procs=[1113360, 216 MiB;]
05:28:36 avail=87427M gpu_total=673 MiB procs=[1113360, 216 MiB;]
05:28:38 avail=87422M gpu_total=673 MiB procs=[1113360, 216 MiB;]
05:28:40 avail=87383M gpu_total=8961 MiB procs=[1113360, 8664 MiB;]
05:28:42 avail=87152M gpu_total=12257 MiB procs=[1113360, 11800 MiB;]
```

Checks after load:

```
$ ps -o pid,args -p $(cat ~/.kwaainet/run/shard.pid)
    PID COMMAND
1113360 /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet shard serve --auto-rebalance
[exit 0]

$ ps -o pid,args -p $(cat ~/.kwaainet/run/kwaainet.pid)
    PID COMMAND
1113257 /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet run-node --shard
[exit 0]

$ /nix/store/460frpdijpwl6fcz89wg49hsp9jbdjlf-kwaainet-0.7.3/bin/kwaainet status
│                       📊 KwaaiNet Node Status                       │
  🟢 Status:  Running
  🔢 PID:     1113257
  ⏱️  Uptime:  29s
  💻 CPU:     0.0%
  🧠 Memory:  203 MB
  🟢 Shard:   Running (PID 1113360)
─────────────────────────────────────────────────────────────────────
[exit 0]

$ nvidia-smi
Sat Nov 14 21:28:58 2026       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 595.99.02              Driver Version: 595.99.02      CUDA Version: 13.2     |
+-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  NVIDIA RTX A5000               Off |   00000000:15:00.0  On |                  Off |
| 30%   38C    P5             47W /  230W |   12257MiB /  24564MiB |      1%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+
+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI              PID   Type   Process name                        GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|    0   N/A  N/A           10771      G   /usr/libexec/Xorg                        93MiB |
|    0   N/A  N/A           10870      G   /usr/bin/gnome-shell                    176MiB |
|    0   N/A  N/A          166999      G   ...rack-uuid=3190708988185955192        107MiB |
|    0   N/A  N/A         1113360      C   ...f-kwaainet-0.7.3/bin/kwaainet      11800MiB |
+-----------------------------------------------------------------------------------------+
[exit 0]

$ free -g
               total        used        free      shared  buff/cache   available
Mem:              93           8          33           0          52          85
Swap:              3           3           0
[exit 0]

$ grep -c -E 'shape mismatch|panicked|ERROR' /tmp/claude-1000/-home-rezarassool-Documents-KwaaiNet/2b13942b-285f-4b6a-8d0f-c2eeeae4aeeb/scratchpad/mistral/p2-shardlog.txt
0
[exit 1]

```

The `grep -c` is 0 (it exits 1 when nothing matches): the log has no `shape mismatch`, no `panicked` and no `ERROR`.

Measurements:
- **Load time:** 3.3 s from `Loading shard` (05:28:38.35) to `Shard ready` (05:28:41.69), and 13 s from `start`.
- **The shard process on the GPU:** PID 1113360 holds **11,800 MiB**, out of 12,257 MiB in use on the card. This is the process's own use, so it is not a CPU fallback.
  - That is a little over the ~10 GB expected. From `config.json`'s shapes, the weights alone are about 10.1 GB in bf16: 16 blocks of about 272.6 M parameters, or 8.7 GB, plus `lm_head` at 131,072 × 5,120, or 1.3 GB.
  - The rest is the CUDA context and cuBLAS working memory.
- **RAM:** 85 GiB available.

**PASS:**
- `shard.log` has `Device: CUDA (GPU 0)` and then `Shard ready  (16 blocks, is_first=false, is_last=true)`, with no load error and no `q_proj: shape mismatch`.
- `nvidia-smi` shows the shard process holding 11.8 GB.
- Both the daemon and the shard child run the `$K` path.

**metro-linux READY (Nemo, GPU 24–40).**

## Phase 3: Nemo runs

A sampler recorded `free`, `nvidia-smi` (the whole card and per process) and `vmstat` swap activity every 30 s, from 05:29:19 to 06:02:43. Reza reported the runs done at about 06:02.

### Memory and GPU

```
05:29:19 avail=87249M swap_used=3353M gpu=[12266 MiB, 4 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:29:50 avail=87338M swap_used=3353M gpu=[12253 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:30:21 avail=87379M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:30:52 avail=87388M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:31:23 avail=87407M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:31:54 avail=87452M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:32:25 avail=87478M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:32:56 avail=87488M swap_used=3353M gpu=[12245 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:33:28 avail=87503M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:33:59 avail=87524M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:34:30 avail=87551M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:35:01 avail=87556M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:35:32 avail=87569M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:36:03 avail=87579M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:36:34 avail=87590M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:37:05 avail=87594M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:37:36 avail=87595M swap_used=3353M gpu=[12275 MiB, 0 %] procs=[1113360, 11800 MiB;] si=0 so=0
05:38:07 avail=87386M swap_used=3353M gpu=[12281 MiB, 0 %] procs=[1113360, 11806 MiB;] si=0 so=0
05:38:38 avail=87411M swap_used=3352M gpu=[12281 MiB, 0 %] procs=[1113360, 11806 MiB;] si=0 so=0
05:39:09 avail=87429M swap_used=3352M gpu=[12281 MiB, 5 %] procs=[1113360, 11806 MiB;] si=0 so=0
05:39:40 avail=87433M swap_used=3352M gpu=[12281 MiB, 0 %] procs=[1113360, 11806 MiB;] si=0 so=0
05:40:11 avail=87436M swap_used=3352M gpu=[11097 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:40:42 avail=87449M swap_used=3352M gpu=[11097 MiB, 16 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:41:13 avail=87449M swap_used=3352M gpu=[11097 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:41:44 avail=87460M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=4 so=0
05:42:16 avail=87474M swap_used=3352M gpu=[11094 MiB, 17 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:42:47 avail=87475M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:43:18 avail=87477M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:43:49 avail=87485M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:44:20 avail=87491M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:44:51 avail=87498M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:45:22 avail=87504M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:45:53 avail=87505M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:46:24 avail=87507M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:46:55 avail=87508M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:47:26 avail=87510M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:47:57 avail=87508M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:48:28 avail=87507M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:48:59 avail=87508M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:49:30 avail=87513M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:50:01 avail=87503M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:50:32 avail=87501M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:51:04 avail=87499M swap_used=3352M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:51:35 avail=87488M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:52:06 avail=87479M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:52:37 avail=87488M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:53:08 avail=87465M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:53:39 avail=87475M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:54:10 avail=87474M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:54:41 avail=87464M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:55:12 avail=87443M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:55:43 avail=87443M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:56:14 avail=87448M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:56:45 avail=87451M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:57:16 avail=87452M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=16 so=0
05:57:47 avail=87457M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:58:18 avail=87455M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:58:49 avail=87456M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:59:20 avail=87451M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
05:59:52 avail=87447M swap_used=3342M gpu=[11094 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:00:23 avail=87441M swap_used=3342M gpu=[11096 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:00:54 avail=87429M swap_used=3342M gpu=[11096 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:01:25 avail=87426M swap_used=3342M gpu=[11096 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:01:56 avail=87422M swap_used=3342M gpu=[11096 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:02:27 avail=87450M swap_used=3337M gpu=[11074 MiB, 0 %] procs=[1113360, 10622 MiB;] si=0 so=0
06:02:58 avail=87399M swap_used=3337M gpu=[11082 MiB, 15 %] procs=[1113360, 10622 MiB;] si=0 so=0
```

Over the 65 samples:
- **RAM:** at least 87,249 MiB (85 GiB) available throughout.
- **Swap:** flat at about 3.35 GiB used. That is left over from the 70B test; `so` was 0 in every sample and `si` totalled 20 pages, so **nothing swapped**.
- **GPU memory:** at most **12,281 MiB** on the card. The shard process itself held 11,800–11,806 MiB until 05:40, then **10,622 MiB** for the rest of the runs. It released about 1.2 GB between 05:39:09 and 05:40:42, during the second burst, and did not grow again.
- **GPU utilisation:** at most 17% in any sample. The 30 s samples mostly fall between forward passes, which last about 18 ms each.

### `shard.log` during the runs

The log has 1,219 new lines from Phase 2's ready line to 06:02:43:
- 1,215 `hop timing` lines;
- 4 `GC evicted 1 stale sessions` lines.

There are **no errors or warnings**: `grep -i -E 'error|warn|too large|panick'` matched 0 lines, so there is no `Message too large` and no `shape mismatch`.

Forward time per hop for this node's blocks `[24..40)`, with `payload="HiddenStates"`:

```
hops 1215  min 17.0  median 17.6  p99 18.1  max 346.1  mean 17.8 (ms)
```

The six hops slower than 20 ms are each the first or second hop of a burst, which is the prefill over the whole prompt:

```
05:37:58.541  346.1 ms   (very first forward pass on this node: CUDA kernel load/JIT)
05:38:00.620   22.6 ms
05:40:34.242   39.3 ms
05:43:27.891   38.4 ms
05:51:39.132   40.5 ms
06:00:39.366   38.3 ms
```

Grouped by gaps longer than 5 s, the hops fall into five bursts:

```
burst at 05:37:58  200 hops, 105.2 s
burst at 05:40:34  200 hops, 100.8 s
burst at 05:43:27  200 hops, 100.4 s
burst at 05:51:39  423 hops, 134.2 s
burst at 06:00:39  192 hops,  55.6 s
```

In each burst this node's 16 blocks took about 18 ms per token, but tokens arrived about every 0.5 s. So almost all of each run's time was spent outside metro-linux. The 423-hop burst at 05:51 is probably two runs back to back. Reza's `runs.md` has the actual prompts.

The `GC evicted 1 stale sessions` lines came at 05:50:12, 05:52:42, 05:55:12 and 06:02:42. They are routine cleanup of finished sessions, and 6 sessions were still held at the last one.

**PASS:**
- No errors in `shard.log`.
- No swapping.
- GPU memory peaked at 12.3 GB on the card and 11.8 GB for the shard process.
- Every hop served from CUDA in about 17–18 ms, apart from the first.

## Phase 3b: the same prompts run from metro-linux as the client

Reza asked for a sharded run from this machine with three prompts: "Summarize the critique of the standard model of physics", "is that your reals hair?" and "How do civilizations collapse?". They are the Mac's Phase 3 prompts, so I used the Mac's flags, `[INST]…[/INST]` wrapping and `--max-tokens 128 --stats`, so the runs are comparable. Each prompt is used exactly as Reza wrote it, so prompt 2 says "reals" where the Mac's said "real", and prompt 3 ends with "?".

The client is `$K shard run` on metro-linux, at 2026-11-15 06:05 UTC, with the same three serving nodes as the Mac's runs.

**Gate: PASS.**

```
$ $K shard chain --total-blocks 40
  Model prefix: Mistral-Nemo-Instruct-2407
  Querying 40 blocks from DHT…
    3 server(s) — 40/40 blocks covered
  START  END    PEER ID (prefix)   TRUST      NAME
  ─────────────────────────────────────────────────────────────────
      0      8  12D3KooWAourfFox…  UNKNOWN    professor_reza/v0.7.3
      8     24  12D3KooWLMizEbVi…  UNKNOWN    metro-win-rtx-a5000/v0.…
     24     40  12D3KooWA33TMz7s…  UNKNOWN    metro-linux/v0.7.3
  Blocks: [████████████████████████████████████████]
  ✅ Full model coverage — distributed inference ready
─────────────────────────────────────────────────────────────────────
```

### Summary

| Run | Prompt | Tokens in / out | Prefill | Decode avg (min–max) | Mac hop | metro-win hop | metro-linux hop | Total |
|---|---|---|---|---|---|---|---|---|
| 1 | Summarize the critique of the standard model of physics | 14 / 128 | 1102 ms | 119 ms (101–133) | 50 ms | 43 ms | 25 ms | 16.3 s |
| 2 | is that your reals hair? | 10 / 40 (EOS) | 469 ms | 114 ms (107–122) | 49 ms | 38 ms | 25 ms | 4.9 s |
| 3 | How do civilizations collapse? | 9 / 128 | 553 ms | 118 ms (106–128) | 50 ms | 42 ms | 24 ms | 15.6 s |

**Result: PASS.**
- All three runs completed over the three GPUs, with no timeout or error.
- The output is coherent English and on topic.
- Run 2 stopped on EOS at 40 tokens; runs 1 and 3 hit the 128-token cap.

The metro-linux shard's own `shard.log` for the same runs, as forward time per hop:

| Run | Hops logged | fwd ms min / median / max |
|---|---|---|
| 1 | 128 | 17.6 / 17.8 / 33.9 (the max is prefill) |
| 2 | 40 | 17.6 / 17.8 / 18.9 |
| 3 | 128 | 17.6 / 17.8 / 19.6 |

The log has no errors or warnings for any run. The client's per-hop figures for this node, about 25 ms, are about 18 ms of compute plus about 7 ms of local RPC overhead.

### Compared with the Mac's Phase 3 (`runs.md`)

| | Mac as client (runs 1–4) | metro-linux as client |
|---|---|---|
| Decode avg | 242–330 ms/tok | **114–119 ms/tok** |
| Throughput | about 3–4 tok/s | **about 8 tok/s** |
| Mac hop (blocks 0–8) | about 21 ms | 49–50 ms |
| metro-win hop (8–24) | 83–93 ms | 38–43 ms |
| metro-linux hop (24–40) | 136–217 ms | 24–25 ms |

The same three servers ran 2.0–2.8× faster per token with the client here. Each node's hop got cheaper when the client was local to it and dearer when it was remote:
- The Mac's own hop went from 21 to 50 ms.
- metro-linux's went from 136–217 to 25 ms.

metro-linux's compute held at about 18 ms throughout, both in Phase 3 and here. So the hop figures are mostly the round trip between the client and each node, not compute. That supports what `runs.md` suspected: the slow, variable metro-linux hop seen from the Mac was the relayed path to this machine, not its GPU.

This is one set of runs a few minutes apart, so treat the comparison as indicative.

**Determinism across clients.** Run 1's text is **identical word for word** to the Mac's runs 1 and 2 for the same prompt, over the same three nodes. Greedy decoding gives the same numerics whichever machine runs the client. Runs 2 and 3 differ from the Mac's, but so do their prompts, so that says nothing either way:
- Run 2 has "reals hair?" against the Mac's "real hair". Here the model answered as if it had a profile picture and real hair. The Mac's prompt got "No, it's not. I'm a text-based AI…". This is the model's answer, not a fault in the chain.
- Run 3 has a trailing "?", which the Mac's prompt lacked. Both answers open with environmental degradation.

### Outputs (verbatim)

#### run 1: Summarize the critique of the standard model of physics  (start 06:05:15Z)

```
The Standard Model of Physics, which combines quantum mechanics and special relativity, has been incredibly successful in explaining three of the four known fundamental forces and classifying all known elementary particles. However, it faces several critiques and challenges:

1. **Incompleteness**: The Standard Model does not account for gravity, one of the four fundamental forces. This is often referred to as the "missing force." Efforts to incorporate gravity into the Standard Model have so far been unsuccessful, leading to the quest for a theory of quantum gravity.

2. **Dark Matter and Dark Energy**: Observations of the universe's large-scale structure and its accelerating expansion suggest the

```

#### run 2: is that your reals hair?  (start 06:05:35Z)

```
Yes, the hair you see in my profile picture is indeed my real hair! I don't have the ability to change my appearance, so what you see is what you get. ���

```

#### run 3: How do civilizations collapse?  (start 06:05:44Z)

```
Civilizations collapse due to a complex interplay of various factors, and there's no one-size-fits-all answer. Here are some of the most common reasons, drawing from the work of historians, archaeologists, and social scientists like Jared Diamond, Joseph Tainter, and Thomas Homer-Dixon:

1. **Environmental Degradation and Climate Change:**
 - Deforestation, soil erosion, and loss of biodiversity can lead to decreased agricultural productivity.
 - Climate change can cause droughts, floods, or other extreme weather events that disrupt food supplies and make it difficult for societies to persist.
 - Examples: The Mayan

```

Run 2's last characters print as `���`: invalid UTF-8, probably an emoji split across byte-level tokens and decoded one token at a time. I have not verified this, and it is cosmetic.

### Full `--stats` output

Run 1:

```
  ── Timing ────────────────────────────────────────────────
  Prefill:           1102ms  (14 input tokens)
  Decode avg:         119ms/tok  (min 101, max 133)
  Total:             16.3s
  Throughput:         7.9 tok/s
  Hops:                 3

  ── Per-hop (decode avg) ──────────────────────────────────
  [0] professor_reza/v0.7.3               blocks   0–  8 ( 8 blks)  avg     50ms  min 40  max 62
  [1] metro-win-rtx-a5000/v0.7.3          blocks   8– 24 (16 blks)  avg     43ms  min 35  max 52
  [2] metro-linux/v0.7.3                  blocks  24– 40 (16 blks)  avg     25ms  min 23  max 27
```

Run 2:

```
  ── Timing ────────────────────────────────────────────────
  Prefill:            469ms  (10 input tokens)
  Decode avg:         114ms/tok  (min 107, max 122)
  Total:              4.9s
  Throughput:         8.2 tok/s
  Hops:                 3

  ── Per-hop (decode avg) ──────────────────────────────────
  [0] professor_reza/v0.7.3               blocks   0–  8 ( 8 blks)  avg     49ms  min 43  max 54
  [1] metro-win-rtx-a5000/v0.7.3          blocks   8– 24 (16 blks)  avg     38ms  min 32  max 41
  [2] metro-linux/v0.7.3                  blocks  24– 40 (16 blks)  avg     25ms  min 24  max 26
```

Run 3:

```
  ── Timing ────────────────────────────────────────────────
  Prefill:            553ms  (9 input tokens)
  Decode avg:         118ms/tok  (min 106, max 128)
  Total:             15.6s
  Throughput:         8.2 tok/s
  Hops:                 3

  ── Per-hop (decode avg) ──────────────────────────────────
  [0] professor_reza/v0.7.3               blocks   0–  8 ( 8 blks)  avg     50ms  min 42  max 60
  [1] metro-win-rtx-a5000/v0.7.3          blocks   8– 24 (16 blks)  avg     42ms  min 36  max 50
  [2] metro-linux/v0.7.3                  blocks  24– 40 (16 blks)  avg     24ms  min 23  max 26
```

