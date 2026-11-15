# Mac mini: blocks 0–8 on MLX, and the coordinator

Read [README.md](README.md) first. Write the node's record to `results/mac-mini.md`, and the runs to `results/runs.md`.

## Binary

The binary is built in `../KwaaiNet-mlx` with `--features mlx` into a separate target dir:

```
K=/Users/rezarassool/Source/KwaaiNet-mlx/core/target-nemo/release/kwaainet
```

It is `main` at `c48c3b8e` plus three changes:
- the `fix/config-head-dim` change;
- the local `REQUEST_TIMEOUT` of 300 s;
- comma-separated `--name-filter`.

`mlx.metallib` is copied beside it. The macOS 26 build needs the same three things as before (Metal Toolchain, `MACOSX_DEPLOYMENT_TARGET=14.0`, `libclang_rt.osx.a`).

## Before Phase 2

- **Free memory.** Quit rust-analyzer and anything heavy, and keep Chrome small. Ollama stays stopped.
- **Disk.** Keep at least 20 GB free after the downloads. They are 9.8 GB for Nemo and 14.3 GB for Small 24B. The 70B blocks 0–8 (18 GB) can be deleted first if Reza agrees.

## Phase 1: download Nemo

```
KWAAINET_HOME=$HOME/.kwaainet-mistral-dl $K config set model unsloth/Mistral-Nemo-Instruct-2407
KWAAINET_HOME=$HOME/.kwaainet-mistral-dl $K shard download --start-block 0 --blocks 8
```

**PASS:** `model-00001` and `model-00002-of-00005.safetensors`, `config.json` and `tokenizer.json` are present.

## Phase 2: serve Nemo (once both metros report BUILT+DOWNLOADED)

```
$K stop
$K config set model unsloth/Mistral-Nemo-Instruct-2407
$K config set shard.backend mlx
$K config set start_block 0
$K config set blocks 8
$K start --daemon --shard
```

**PASS:** `MLX: Shard [0..8) ready`, and the shard's `footprint` is about 6 GB.

## Phase 3: Nemo runs (once both metros report READY)

```
$K shard chain --total-blocks 40
$K shard run "[INST]Name three rivers in Africa.[/INST]" --total-blocks 40 --max-tokens 32 --stats
$K shard run "[INST]Write a haiku about distributed computing.[/INST]" --total-blocks 40 --max-tokens 32 --stats
$K shard run "[INST]What is the capital of Australia, and why?[/INST]" --total-blocks 40 --max-tokens 48 --stats
```

The **gate** is 40/40 blocks from the three nodes. The **pass criteria** are in the plan.

The first run includes cold-start weight paging, so run prompt 1 twice and record both.

## Phase 4: Mistral Small 24B

Repeat Phases 1 to 3 with `unsloth/Mistral-Small-24B-Instruct-2501`. The block ranges are the same. Expect the shard to take about 10 GB.

## Phase 5: restore

1. `$K stop`.
2. Restore `~/.kwaainet/config.yaml` and `run/start-args.json` from the backups in the session scratchpad (`mac-shard-backup/`).
3. `kwaainet start --daemon --shard` with the installed binary.
4. Restart Ollama.
5. Delete `~/.kwaainet-mistral-dl` and `~/.kwaainet-70b-dl`.
