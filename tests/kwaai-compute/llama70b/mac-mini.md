# Mac mini: blocks 0–8 on MLX, and the coordinator

Read [README.md](README.md) first; its rules apply. Write the node's record to `results/mac-mini.md`, and the test runs to `results/runs.md`.

## Binary

This uses a `--features mlx` build of `main`, in the worktree `../KwaaiNet-mlx`:

```
K=/Users/rezarassool/Source/KwaaiNet-mlx/core/target/release/kwaainet
```

Building it on macOS 26 needed three things:
- Apple's Metal Toolchain: `xcodebuild -downloadComponent MetalToolchain`;
- `MACOSX_DEPLOYMENT_TARGET=14.0`, so MLX's `fence.metal` compiles;
- linking `$(xcrun clang --print-runtime-dir)/libclang_rt.osx.a`, for `___isPlatformVersionAtLeast`. Pass it via `cargo rustc ... -- -C link-arg=<path>`.

`mlx.metallib` is copied beside the binary.

## Phase 1: backups and download

1. Back up `~/.kwaainet/config.yaml` and `~/.kwaainet/run/start-args.json`.
2. Download blocks 0–8 (about 16 GB) under a throwaway home:
   ```
   KWAAINET_HOME=$HOME/.kwaainet-70b-dl $K config set model unsloth/Llama-3.3-70B-Instruct
   KWAAINET_HOME=$HOME/.kwaainet-70b-dl $K shard download --start-block 0 --blocks 8
   ```
   **PASS:** `config.json`, `tokenizer.json`, `embed_tokens`, and the files containing `model.layers.0` through `model.layers.7`.

## Phase 2: switch (once both metros report READY)

1. Stop Ollama. It frees memory, and otherwise `whole_model.ready` makes the node announce ONLINE before the MLX shard has loaded.
2. Switch:
   ```
   $K stop
   $K config set contribute.auto_update false
   $K config set model unsloth/Llama-3.3-70B-Instruct
   $K config set shard.backend mlx
   $K config set start_block 0
   $K config set blocks 8
   $K start --daemon --shard
   ```
3. **PASS:** `MLX: Shard [0..8) ready`.
   **If it fails** on memory (MLX's GPU working-set limit), use the fallback layout in the plan: the Mac takes 0–4 and every other range shifts down by 4.

## Phase 3: coverage and runs

1. Coverage:
   ```
   $K shard chain --total-blocks 80
   ```
   **Gate:** 80/80 blocks, by exactly the four planned nodes.
2. Runs (greedy, the default):
   ```
   $K shard run "Name three rivers in Africa." --total-blocks 80 --max-tokens 32 --stats
   $K shard run "Write a haiku about distributed computing." --total-blocks 80 --max-tokens 32 --stats
   $K shard run "What is the capital of Australia, and why?" --total-blocks 80 --max-tokens 48 --stats
   ```
   **PASS:**
   - the pinned path is Mac, then metro-win, then metro-linux-cpu, then metro-linux;
   - every answer is coherent and complete;
   - there are no hop errors.

   Record per-hop times, prefill, decode average and memory peaks in `results/runs.md`.
3. When the runs are recorded, tell both metro sessions to restore.

## Phase 4: restore

`$K stop`. Restore both backups. `kwaainet start --daemon --shard` with the installed binary. Restart Ollama. Delete `~/.kwaainet-70b-dl`.
