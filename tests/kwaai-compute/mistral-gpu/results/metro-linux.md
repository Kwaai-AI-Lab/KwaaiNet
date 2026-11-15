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
# core/patches/{cudarc,libp2p-kad,multistream-select} are not needed for the Nix build (the flake vendors its own)
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
