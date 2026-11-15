# Llama 3.3 70B across a Mac, a Windows box and a Linux box

**Status:** plan, 2026-11-14. Phase A (manual, 16-bit, current code) is ready to run.

## Goal

Show that KwaaiNet runs one model that no single participating machine can hold, split by blocks across machines with different hardware, operating systems, backends and memory sizes.

The model is `unsloth/Llama-3.3-70B-Instruct`: 80 blocks, hidden size 8192, 30 safetensors files, about 141 GB in 16-bit. It is not gated, so no Hugging Face token is needed.

## Machines (measured 2026-11-14)

| Machine | OS | Accelerator | System RAM | Free disk | Backend |
|---|---|---|---|---|---|
| Mac mini | macOS | M-series, 24 GB unified | (same 24 GB) | 65 GB | MLX (`--features mlx` build) |
| metro-win | Windows 11 | RTX A5000, 24 GB | 190 GB (175 free) | 509 GB | Candle, CPU for this test |
| metro-linux | Linux | RTX A5000, 24 GB | 93 GiB (82 available) | 517 GB | Candle CUDA + Candle CPU |

None can hold the 141 GB model alone. metro-win's RAM comes closest, and the test does not rely on that.

## Constraints in the current code

All checked on `main` at `c48c3b8e`.

1. **Blocks load only in 16-bit.** `kwaai-inference/src/shard.rs → TransformerShard::load` sets `dtype: DType::F16`; MLX converts to `Float16`. The block path has no quantization. One 70B block is about 1.71 GB.
2. **A node serves 4, 8, 16 or 32 blocks.** `shard_cmd.rs → snap_to_valid_blocks` rounds anything else to the nearest of `VALID_BLOCK_COUNTS`.
3. **Keep prompts under about 300 tokens, chat template included.** Activations go over the control socket as a msgpack integer array, about 24.6 KB per token at hidden size 8192. The socket rejects messages over 10 MB (`kwaai-p2p-daemon/src/server.rs → MAX_MESSAGE_SIZE`), and prefill is not chunked. So a hop fails at roughly 320–426 prompt tokens.
4. **Llama 3 `rope_scaling` is not applied** by either backend. It matters at long context, and 8B already runs this way. The short prompts here are not affected much.
5. **Windows runs one node.** Its control socket is hard-coded to `127.0.0.1:5005`, so `KWAAINET_SOCKET` is ignored there.
6. **The first node also holds the embeddings, and the last holds `lm_head`**, about 2.1 GB each.
7. **No memory check before load.** A node that over-commits finds out when it runs out of memory.

## Layout

| Order | Node | Device | Blocks | Approx. memory |
|---|---|---|---|---|
| 1 | Mac mini | GPU (MLX) | 0–8 (8), plus embeddings | 16 GB of 24 |
| 2 | metro-win | CPU | 8–40 (32) | 55 GB of 190 |
| 3 | metro-linux node A (new home) | CPU | 40–72 (32) | 55 GB of 82 available |
| 4 | metro-linux node B (default home) | GPU | 72–80 (8), plus `lm_head` | 16 GB of 24 |

**Expected speed: 8–10 s per token.** 64 of the 80 blocks are on CPU. Each block is about 4× the size of an 8B block, and metro-win measured about 40 ms per 8B block on CPU.

**Fallback if the Mac can't hold 8 blocks.** MLX's default limit on how much memory the GPU may use is below 24 GB. If the Mac fails to load:
- Mac takes 0–4.
- metro-win takes 4–36.
- metro-linux A takes 36–68.
- metro-linux B (GPU) takes 68–76.
- A third metro-linux CPU node, with its own home, takes 76–80 plus `lm_head`.

## Downloads

Each node fetches only its own blocks, through `hf::download_for_blocks`: the safetensors files that contain its layers, plus config and tokenizer files. A file holding layers from two ranges is downloaded by both nodes.

| Machine | Blocks | About |
|---|---|---|
| Mac | 0–8 | 16 GB |
| metro-win | 8–40 | 55 GB |
| metro-linux | 40–80 | 70 GB (one cache, shared by both nodes) |

`shard download` takes the block total from `cfg.model`, not from a flag. **Set the model in config before downloading**, or the range is clamped to 32 blocks.

Start the downloads first; they are the long pole.

## Steps

### 0. On every node

- Back up `config.yaml` and `run/start-args.json`.
- `kwaainet config set contribute.auto_update false`, so nothing updates mid-test.
- `kwaainet config set model unsloth/Llama-3.3-70B-Instruct`. Every node and the client must use this exact string; it gives the DHT prefix `Llama-3-3-70B-Instruct`.

Each node stops serving the 8B while it serves the 70B. Tell the fleet beforehand; the other 8B servers keep that model covered.

### 1. Mac mini (MLX, blocks 0–8)

- Build: the `--features mlx` binary in `../KwaaiNet-mlx`. Building it here needed three things:
  - Apple's Metal Toolchain (`xcodebuild -downloadComponent MetalToolchain`);
  - `MACOSX_DEPLOYMENT_TARGET=14.0`;
  - linking `$(xcrun clang --print-runtime-dir)/libclang_rt.osx.a`.

  #255 covers the second.
- Stop Ollama for the test. It frees memory, and otherwise `whole_model.ready` makes the node announce ONLINE before the MLX shard has loaded.
- Commands:
  ```
  K=/Users/rezarassool/Source/KwaaiNet-mlx/core/target/release/kwaainet
  kwaainet stop
  $K config set shard.backend mlx
  $K config set start_block 0
  $K config set blocks 8
  $K shard download --start-block 0 --blocks 8
  $K start --daemon --shard
  ```

### 2. metro-win (CPU, blocks 8–40)

```
kwaainet stop
kwaainet config set use_gpu false
kwaainet config set start_block 8
kwaainet config set blocks 32
kwaainet shard download --start-block 8 --blocks 32
kwaainet start --daemon --shard
```

The GPU sits idle. Using it as well needs the Windows `KWAAINET_SOCKET` fix (Phase B).

### 3. metro-linux node B (GPU, blocks 72–80): the existing default home

```
kwaainet stop
kwaainet config set use_gpu true
kwaainet config set start_block 72
kwaainet config set blocks 8
kwaainet shard download --start-block 40 --blocks 32   # node A's range: one shared cache
kwaainet shard download --start-block 72 --blocks 8
kwaainet start --daemon --shard
```

### 4. metro-linux node A (CPU, blocks 40–72): a new home

Every command for this node needs all four variables set, so that it uses its own config, identity, socket and ports:

```
export KWAAINET_HOME=$HOME/.kwaainet-70b-cpu
export KWAAINET_SOCKET=$KWAAINET_HOME/node.sock
export KWAAINET_PORT=8081
export KWAAINET_GRPC_PORT=8094

kwaainet config set model unsloth/Llama-3.3-70B-Instruct
kwaainet config set public_name metro-linux-cpu
kwaainet config set use_gpu false
kwaainet config set start_block 40
kwaainet config set blocks 32
kwaainet config set contribute.auto_update false
kwaainet start --daemon --shard
```

- The fresh home generates its own identity, so the node gets a new peer ID.
- It has no storage initialised, so no second storage server competes for port 7432.
- The weights come from the shared Hugging Face cache.

### 5. Check coverage

From the Mac:

```
$K shard chain --total-blocks 80
```

`shard chain` defaults to 32 blocks, so pass 80.

**Gate:** all 80 blocks covered, by exactly these four nodes.

## Run

From the Mac, three short prompts with greedy decoding, the default:

```
$K shard run "Name three rivers in Africa." --total-blocks 80 --max-tokens 32 --stats
$K shard run "Write a haiku about distributed computing." --total-blocks 80 --max-tokens 32 --stats
$K shard run "What is the capital of Australia, and why?" --total-blocks 80 --max-tokens 48 --stats
```

While it runs, record each machine's peak memory: Activity Monitor; Task Manager or `Get-Process`; `free -g` and `nvidia-smi`.

**Optional reference.** metro-win can run Ollama's `llama3.3:70b` 4-bit on CPU in its 190 GB of RAM. Compare answers for meaning only: it is a different quantization, so the tokens won't match.

## Pass criteria

1. `shard chain` shows 80 of 80 blocks across the four nodes. No single machine holds more than 40 blocks.
2. Every run's pinned path is the four planned nodes in order: Mac, metro-win, metro-linux A, metro-linux B.
3. All three prompts produce coherent, correct answers to `--max-tokens` or EOS, with no hop errors and no KV resets.
4. No machine swaps or runs out of memory. Peak memory is recorded per machine.
5. Per-hop decode times and the overall tokens per second are recorded with `--stats`, measured over at least 14 tokens.

Write the results to `tests/kwaai-compute/results/llama70b-heterogeneous-<date>.md`: per-hop timings, memory peaks, the three transcripts, and anything that failed.

## Restore

On each node:
1. `kwaainet stop`.
2. Put back the saved `config.yaml` and `run/start-args.json`.
3. `kwaainet start --daemon` with the node's usual flags.

On metro-linux, also stop node A with its four variables set, then delete `~/.kwaainet-70b-cpu`. The 70B weights can stay in the cache or be removed: about 16, 55 and 70 GB.

## Phase B: make it fast enough to show

In order of value:

1. **8-bit blocks.** Candle's quantized `QMatMul` (q8_0) and MLX `quantize` halve block memory. The three GPUs then hold about 40 blocks between them, and the rest fit comfortably on CPU. 4-bit would put all 80 on the three GPUs.
2. **Any block count.** Drop `VALID_BLOCK_COUNTS` rounding, so each GPU takes the 11–12 blocks it can hold instead of 8.
3. **Windows honours `KWAAINET_SOCKET`**, so metro-win can run a GPU node next to its CPU node.
4. **Chunked prefill, or a `serde_bytes` wire encoding.** This lifts the roughly 300-token prompt limit. Note that `serde_bytes` changes the wire format.

## Later: automatic placement

The manual steps above are what an automated protocol would do for the operator:
- a client publishes an intent ("serve `Llama-3.3-70B-Instruct`");
- nodes reply with the span they can hold, given their memory and device;
- the client or a coordinator assigns spans to cover all 80 blocks;
- each node downloads and serves its span.

The per-node capacity figures and placement rules from this run are what that protocol needs as input.
