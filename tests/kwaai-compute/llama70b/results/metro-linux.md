# metro-linux results

Run by a Claude Code session on metro-linux (RHEL 9, RTX A5000 24 GB, 93 GiB RAM), on branch `test/llama70b-heterogeneous` at `b8294892`. Times are UTC.

## Phase 1: baseline, backups, download

### 1. Baseline (2026-11-15 01:19 UTC)

```
$ kwaainet --version
kwaainet 0.7.3
[exit 0]

$ kwaainet status

╭─────────────────────────────────────────────────────────────────────╮
│                       📊 KwaaiNet Node Status                       │
╰─────────────────────────────────────────────────────────────────────╯

  🟢 Status:  Running
  🔢 PID:     925146
  ⏱️  Uptime:  3d 1h 31m 1s
  💻 CPU:     0.0%
  🧠 Memory:  204 MB

  🟢 Shard:   Running (PID 925227)
─────────────────────────────────────────────────────────────────────
[exit 0]

$ free -g
               total        used        free      shared  buff/cache   available
Mem:              93          11           5           0          77          82
Swap:              3           0           3
[exit 0]

$ nvidia-smi
Sat Nov 14 17:19:40 2026       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 595.99.02              Driver Version: 595.99.02      CUDA Version: 13.2     |
+-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  NVIDIA RTX A5000               Off |   00000000:15:00.0  On |                  Off |
| 30%   33C    P8             22W /  230W |    5566MiB /  24564MiB |     11%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+

+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI              PID   Type   Process name                        GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|    0   N/A  N/A           10771      G   /usr/libexec/Xorg                       117MiB |
|    0   N/A  N/A           10870      G   /usr/bin/gnome-shell                    270MiB |
|    0   N/A  N/A          166999      G   ...rack-uuid=3190708988185955192        109MiB |
|    0   N/A  N/A          925227      C   ...zarassool/.cargo/bin/kwaainet       4990MiB |
+-----------------------------------------------------------------------------------------+
[exit 0]

$ df -h ~
Filesystem           Size  Used Avail Use% Mounted on
/dev/mapper/rl-root  850G  334G  517G  40% /
[exit 0]

```

Before the test, the 8B node was serving a shard: blocks 24–32 of `unsloth/Llama-3.1-8B-Instruct` on the GPU, which accounts for 4,990 MiB of the GPU memory above. Of the live `config.yaml`, the relevant lines were `model: unsloth/Llama-3.1-8B-Instruct`, `blocks: 8`, `start_block: 24`, `use_gpu: true` and `public_name: metro-linux`.

**PASS:** the baseline is recorded. The node runs 0.7.3 with its shard up.

### 2. Backups

```
$ cp ~/.kwaainet/config.yaml ~/config.70b-backup.yaml
[exit 0]
$ cp ~/.kwaainet/run/start-args.json ~/start-args.70b-backup.json
[exit 0]
$ cat ~/start-args.70b-backup.json
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
[exit 0]
$ sha256sum (originals vs backups)
614868f0a41a0994863e00a03efa873ffc63132d97e6cd78be6e145ffbec2e00  /home/rezarassool/.kwaainet/config.yaml
614868f0a41a0994863e00a03efa873ffc63132d97e6cd78be6e145ffbec2e00  /home/rezarassool/config.70b-backup.yaml
96fa4abd885c06f42f7ebfada653e1fe0832f1c2a6f2e81b65339e4d8dfe828a  /home/rezarassool/.kwaainet/run/start-args.json
96fa4abd885c06f42f7ebfada653e1fe0832f1c2a6f2e81b65339e4d8dfe828a  /home/rezarassool/start-args.70b-backup.json
```

**PASS:** both backups are byte-identical to the originals, and the start-args contents are above (`"shard": true`, everything else default).

### 3. Download, blocks 40–80, under `KWAAINET_HOME=~/.kwaainet-70b-dl`

The per-file progress lines are omitted, about 4.9 M lines of them; the ✓ line for each file is kept.

```
$ kwaainet config set model unsloth/Llama-3.3-70B-Instruct  [start 01:19:59Z]
2026-11-15T01:19:59.999926Z  INFO kwaainet::config: Created default config at /home/rezarassool/.kwaainet-70b-dl/config.yaml
╭─────────────────────────────────────────────────────────────────────╮
│                      ⚙️  Configuration Updated                      │
╰─────────────────────────────────────────────────────────────────────╯
  ✅ Set model = unsloth/Llama-3.3-70B-Instruct
─────────────────────────────────────────────────────────────────────
[exit 0 in 1s, end 01:20:00Z]

$ kwaainet shard download --start-block 40 --blocks 32  [start 01:20:00Z]
╭─────────────────────────────────────────────────────────────────────╮
│                    Downloading HuggingFace Model                    │
╰─────────────────────────────────────────────────────────────────────╯
  Model: unsloth/Llama-3.3-70B-Instruct
  Blocks: [40, 72) of 80 (selective download)
  Commit: 99cd0d2c829e
  Files:  19 (selective for blocks [40, 72))
  Dest:   /home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f
  [ 1/19] config.json  ✓  891 B
  [ 2/19] generation_config.json  ✓  234 B
  [ 3/19] model-00015-of-00030.safetensors  ✓  4.3 GB
  [ 4/19] model-00016-of-00030.safetensors  ✓  4.3 GB
  [ 5/19] model-00017-of-00030.safetensors  ✓  4.3 GB
  [ 6/19] model-00018-of-00030.safetensors  ✓  4.7 GB
  [ 7/19] model-00019-of-00030.safetensors  ✓  4.6 GB
  [ 8/19] model-00020-of-00030.safetensors  ✓  4.3 GB
  [ 9/19] model-00021-of-00030.safetensors  ✓  4.3 GB
  [10/19] model-00022-of-00030.safetensors  ✓  4.3 GB
  [11/19] model-00023-of-00030.safetensors  ✓  4.7 GB
  [12/19] model-00024-of-00030.safetensors  ✓  4.6 GB
  [13/19] model-00025-of-00030.safetensors  ✓  4.3 GB
  [14/19] model-00026-of-00030.safetensors  ✓  4.3 GB
  [15/19] model-00027-of-00030.safetensors  ✓  4.3 GB
  [16/19] model.safetensors.index.json  — already cached (58 KB)
  [17/19] special_tokens_map.json  ✓  454 B
  [18/19] tokenizer.json  ✓  16 MB
  [19/19] tokenizer_config.json  ✓  54 KB
  ✅ Saved to: /home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f
  💡 Start serving: kwaainet shard serve --model-path "/home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f"
─────────────────────────────────────────────────────────────────────
[exit 0 in 2220s, end 01:57:00Z]

$ kwaainet shard download --start-block 72 --blocks 8  [start 01:57:00Z]
╭─────────────────────────────────────────────────────────────────────╮
│                    Downloading HuggingFace Model                    │
╰─────────────────────────────────────────────────────────────────────╯
  Model: unsloth/Llama-3.3-70B-Instruct
  Blocks: [72, 80) of 80 (selective download)
  Commit: 99cd0d2c829e
  Files:  10 (selective for blocks [72, 80))
  Dest:   /home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f
  [ 1/10] config.json  — already cached (891 B)
  [ 2/10] generation_config.json  — already cached (234 B)
  [ 3/10] model-00027-of-00030.safetensors  — already cached (4.3 GB)
  [ 4/10] model-00028-of-00030.safetensors  ✓  4.7 GB
  [ 5/10] model-00029-of-00030.safetensors  ✓  4.6 GB
  [ 6/10] model-00030-of-00030.safetensors  ✓  2.0 GB
  [ 7/10] model.safetensors.index.json  — already cached (58 KB)
  [ 8/10] special_tokens_map.json  — already cached (454 B)
  [ 9/10] tokenizer.json  — already cached (16 MB)
  [10/10] tokenizer_config.json  — already cached (54 KB)
  ✅ Saved to: /home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f
  💡 Start serving: kwaainet shard serve --model-path "/home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f"
─────────────────────────────────────────────────────────────────────
[exit 0 in 478s, end 02:04:58Z]

overall exit 0
```

Timings:
- Blocks 40–72 took **2220 s (37 min)**, for 13 safetensors files.
- Blocks 72–80 took **478 s (8 min)**, for 3 new files; file 27 was shared and already cached.
- In total it took **45 min**, averaging about 26 MB/s.

Snapshot:
- **Path:** `/home/rezarassool/.cache/huggingface/hub/models--unsloth--Llama-3.3-70B-Instruct/snapshots/99cd0d2c829e92a67c844f9144c2509632e5c87f`
- **Size:** 69 GB, which is the whole HF cache entry for this model. It is `model-00015` through `model-00030-of-00030.safetensors` plus `config.json`, `generation_config.json`, `tokenizer.json`, `tokenizer_config.json`, `special_tokens_map.json` and `model.safetensors.index.json`.

I checked the index's `weight_map` against each file's safetensors header:

```
tensors needed 362 in files model-00015 … model-00030 (16 files)
missing files []
tensors absent from headers 0
layers 40 - 79 count 40
has model.norm True has lm_head True
config.json True tokenizer.json True
```

After the download:
- The live home was not touched: `sha256sum -c` of `~/.kwaainet/config.yaml` and `~/.kwaainet/run/start-args.json` against the pre-download sums gave `OK` for both. `config set model` wrote only to `~/.kwaainet-70b-dl/config.yaml`.
- The 8B node is still Running (PID 925146) with its shard up (PID 925227).
- Disk usage is now 403 GB used and 447 GB free. RAM is 82 GiB available.

**PASS:** both downloads completed. The snapshot has `config.json`, `tokenizer.json`, `model.norm`, `lm_head` and every tensor of `model.layers.40` through `model.layers.79`.

### Phase 1 summary

- Baseline: **PASS**
- Backups: **PASS**
- Download, blocks 40–80: **PASS**. It took 45 min and the snapshot is 69 GB.

## Phase 2: bring up both nodes

### Node B: `~/.kwaainet`, GPU, blocks 72–80

Run in a shell with none of the node-A variables set (`env | grep KWAAINET` was empty).

```
$ kwaainet stop  [02:57:30Z]
│                      🛑 Stopping KwaaiNet Node                      │
2026-11-15T02:57:30.579368Z  INFO kwaainet::daemon: Stopping daemon PID 925146
  ✅ KwaaiNet daemon stopped
[exit 0]

$ kwaainet config set contribute.auto_update false  [02:57:31Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set contribute.auto_update = false
[exit 0]

$ kwaainet config set model unsloth/Llama-3.3-70B-Instruct  [02:57:31Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set model = unsloth/Llama-3.3-70B-Instruct
[exit 0]

$ kwaainet config set use_gpu true  [02:57:31Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set use_gpu = true
[exit 0]

$ kwaainet config set start_block 72  [02:57:31Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set start_block = 72
[exit 0]

$ kwaainet config set blocks 8  [02:57:31Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set blocks = 8
[exit 0]

```

Config diff against the Phase 1 backup:

```
1c1
< model: unsloth/Llama-3.1-8B-Instruct
---
> model: unsloth/Llama-3.3-70B-Instruct
3c3
< start_block: 24
---
> start_block: 72
54a55,57
> contribute:
>   storage: true
>   auto_update: false
```

Note: `blocks: 8` and `use_gpu: true` were already the values in the 8B config, so they do not show in the diff. Setting `contribute.auto_update` created a `contribute:` section, and that also wrote out `storage: true`, a default the original file did not spell out. Phase 4's restore copies the backup back, which undoes this.

```
$ kwaainet start --daemon --shard  [02:57:39Z]
│                       🗺  Reading Network Map                        │
  Local models (1):
    • llama3.1:8b
  Network map (1 model(s)):
    Llama-3.1-8B-Instruct                       12 server(s)  ✓ have locally
  ✅ Confirmed model: unsloth/Llama-3.3-70B-Instruct
    Map entry:  Llama-3.1-8B-Instruct  (12 server(s))
─────────────────────────────────────────────────────────────────────
│                      🚀 Starting KwaaiNet Node                      │
  Model:   unsloth/Llama-3.3-70B-Instruct
  Blocks:  8
  Port:    8080
  Peers:   2
  Name:    metro-linux
─────────────────────────────────────────────────────────────────────
  ✅ KwaaiNet daemon started (PID 1064630)
  ✅ Shard serving: the daemon starts and supervises it
  💡 Shard logs:   kwaainet logs --shard
  💡 Storage not initialised — skipping. Run: kwaainet storage init
  💡 Check status: kwaainet status
  💡 View logs:    kwaainet logs
  💡 Stop daemon:  kwaainet stop
─────────────────────────────────────────────────────────────────────
[exit 0]
```

The start output says `Map entry: Llama-3.1-8B-Instruct (12 server(s))` right after `Confirmed model: unsloth/Llama-3.3-70B-Instruct`. The map entry names the 8B model, not the 70B. Node A's start shows the same, even though the map listed `Llama-3.3-70B-Instruct  1 server(s)` by then. This looks cosmetic, as the shard loaded the 70B blocks, but it is worth a look.

`shard.log` from this start, after `kwaainet stop` ended the 8B shard:

```
2026-11-15T02:57:40.800621Z  INFO kwaai_inference: CUDA device detected
2026-11-15T02:57:41.138884Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 500ms
2026-11-15T02:57:41.639777Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 1s
2026-11-15T02:57:42.642097Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 2s
2026-11-15T02:57:44.643561Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 4s
2026-11-15T02:57:48.644944Z  WARN kwaainet::shard_cmd: inference-mux server failed to start after 5 attempts: register inference-mux stream handler
╭─────────────────────────────────────────────────────────────────────╮
│                      🧩 KwaaiNet Shard Server                       │
╰─────────────────────────────────────────────────────────────────────╯
  Blocks:      [72, 80)
  Device:      CUDA (GPU 0)
  Model:       unsloth/Llama-3.3-70B-Instruct
  ✅ Node registered on protocol /kwaai/inference/1.0.0 — appearing on map.
  💡 Loading model in background. Requests return 'warming up' until ready.
─────────────────────────────────────────────────────────────────────
  💡 Local bypass server on 127.0.0.1:37829
  💡 Rebalancing disabled — this node pins blocks [72, 80). Unset start_block to allow moves.
  💡 Loading shard (16 file(s), blocks [72, 80))…
2026-11-15T02:57:48.646316Z  INFO kwaai_inference::shard: Loading shard [72..80) of 80: hidden=8192 heads=64 (8 kv)
2026-11-15T02:57:48.647102Z  INFO kwaai_inference::shard:   Loading block 72
2026-11-15T02:57:49.060137Z  INFO kwaai_inference::shard:   Loading block 73
2026-11-15T02:57:49.456602Z  INFO kwaai_inference::shard:   Loading block 74
2026-11-15T02:57:49.849081Z  INFO kwaai_inference::shard:   Loading block 75
2026-11-15T02:57:50.250790Z  INFO kwaai_inference::shard:   Loading block 76
2026-11-15T02:57:50.657083Z  INFO kwaai_inference::shard:   Loading block 77
2026-11-15T02:57:51.055003Z  INFO kwaai_inference::shard:   Loading block 78
2026-11-15T02:57:51.448127Z  INFO kwaai_inference::shard:   Loading block 79
2026-11-15T02:57:51.834949Z  INFO kwaai_inference::shard:   Loading norm + lm_head
2026-11-15T02:57:53.033089Z  INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=128256, bos=Some(128000), eos=Some(128009)
2026-11-15T02:57:53.251018Z  INFO kwaai_inference::shard: Shard [72..80) ready — embedding=false final_head=true
  ✅ Shard ready  (8 blocks, is_first=false, is_last=true)
2026-11-15T02:57:53.730894Z  INFO kwaainet::daemon: Sent SIGHUP to daemon PID 1064630 — re-announce will follow
```

The `inference-mux server failed to start after 5 attempts` warning is not new. The same `shard.log` had it 8 times before this test, including the 8B shard's start on 2026-11-11, each followed by `Shard ready`. The daemon serves `/kwaai/inference-mux/1.0.0` itself (`kwaainet.log`: `inference-mux: serving /kwaai/inference-mux/1.0.0 natively`), so the shard's own registration of the handler collides with it.

After load:
- Load took 5 s, from `Loading shard` at 02:57:48 to ready at 02:57:53.
- `nvidia-smi` showed **18,366 MiB / 24,564 MiB** used. About 0.5 GB of that is Xorg and GNOME, so the shard accounts for about 17.8 GB, against the ~16 GB expected.
- `free -g` showed available 82 GiB and swap 0 GiB used.
- `kwaainet status`: Running, PID 1064630, Shard Running (PID 1064719).

**PASS:** `Shard ready  (8 blocks, is_first=false, is_last=true)`.

### Node A: `~/.kwaainet-70b-cpu`, CPU, blocks 40–72

Run from a script with all four variables exported. A watchdog in the same script sampled `free` every 2 s and would have run `kwaainet stop` for node A if available memory had dropped below 8 GiB.

```
KWAAINET_GRPC_PORT=8094
KWAAINET_HOME=/home/rezarassool/.kwaainet-70b-cpu
KWAAINET_PORT=8081
KWAAINET_SOCKET=/home/rezarassool/.kwaainet-70b-cpu/node.sock
$ kwaainet config set model unsloth/Llama-3.3-70B-Instruct  [02:58:28Z]
2026-11-15T02:58:28.539218Z  INFO kwaainet::config: Created default config at /home/rezarassool/.kwaainet-70b-cpu/config.yaml
│                      ⚙️  Configuration Updated                      │
  ✅ Set model = unsloth/Llama-3.3-70B-Instruct
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet config set public_name metro-linux-cpu  [02:58:28Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set public_name = metro-linux-cpu
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet config set use_gpu false  [02:58:28Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set use_gpu = false
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet config set start_block 40  [02:58:28Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set start_block = 40
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet config set blocks 32  [02:58:28Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set blocks = 32
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet config set contribute.auto_update false  [02:58:28Z]
│                      ⚙️  Configuration Updated                      │
  ✅ Set contribute.auto_update = false
─────────────────────────────────────────────────────────────────────
[exit 0]

$ kwaainet start --daemon --shard  [02:58:29Z]
│                       🗺  Reading Network Map                        │
  Local models (1):
    • llama3.1:8b
  Network map (2 model(s)):
    Llama-3.1-8B-Instruct                       12 server(s)  ✓ have locally
    Llama-3.3-70B-Instruct                       1 server(s)  ✗ not installed
  ✅ Confirmed model: unsloth/Llama-3.3-70B-Instruct
    Map entry:  Llama-3.1-8B-Instruct  (12 server(s))
─────────────────────────────────────────────────────────────────────
│                      🚀 Starting KwaaiNet Node                      │
  Model:   unsloth/Llama-3.3-70B-Instruct
  Blocks:  32
  Port:    8081
  Peers:   2
  Name:    metro-linux-cpu
─────────────────────────────────────────────────────────────────────
  ✅ KwaaiNet daemon started (PID 1065960)
  ✅ Shard serving: the daemon starts and supervises it
  💡 Shard logs:   kwaainet logs --shard
  💡 Storage not initialised — skipping. Run: kwaainet storage init
  💡 Check status: kwaainet status
  💡 View logs:    kwaainet logs
  💡 Stop daemon:  kwaainet stop
─────────────────────────────────────────────────────────────────────
[exit 0]

```

`shard.log` (blocks 41–70 omitted):

```
2026-11-15T02:58:29.956332Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 500ms
2026-11-15T02:58:30.457177Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 1s
2026-11-15T02:58:31.458473Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 2s
2026-11-15T02:58:33.459747Z  WARN kwaainet::shard_cmd: inference-mux server failed to start: register inference-mux stream handler — retrying in 4s
2026-11-15T02:58:37.460770Z  WARN kwaainet::shard_cmd: inference-mux server failed to start after 5 attempts: register inference-mux stream handler
  Blocks:      [40, 72)
  Device:      CPU
  💡 Loading model in background. Requests return 'warming up' until ready.
  💡 Loading shard (16 file(s), blocks [40, 72))…
2026-11-15T02:58:37.462458Z  INFO kwaai_inference::shard: Loading shard [40..72) of 80: hidden=8192 heads=64 (8 kv)
2026-11-15T02:58:37.463200Z  INFO kwaai_inference::shard:   Loading block 40
2026-11-15T03:03:02.362113Z  INFO kwaai_inference::shard:   Loading block 71
2026-11-15T03:03:12.006172Z  INFO kwaai_inference::shard: Shard [40..72) ready — embedding=false final_head=false
  ✅ Shard ready  (32 blocks, is_first=false, is_last=false)
```

Measurements:
- **Load time:** 275 s from `Loading shard` (02:58:37) to `Shard ready` (03:03:12), and 285 s from `kwaainet start`.
- **Peer ID:** `12D3KooWJH5FGgJ8YeyKxzZZBPH4muwJ4Lv9GTaioqGEjtXCRQS3`, a fresh identity in the new home.
- **Shard process RSS:** `ps -o rss= -p $(cat $KWAAINET_HOME/run/shard.pid)` gave `52304264` KiB, which is **49.9 GiB**.
- **Available memory:** 84,302 MiB at the start, and **35,586 MiB (34.8 GiB) at its lowest**, at 03:03:12 when the shard became ready. That is from 143 samples, 2 s apart; it never came near 8 GiB.
- **Ports:** node A listens on 8081 (p2p) and 127.0.0.1/::1 8094 (gRPC), and node B on 8080 and 8093. There was no port or socket clash.

`free -g` after load:

```
               total        used        free      shared  buff/cache   available
Mem:              93          59           1           0          34          34
Swap:              3           3           0
```

Swap went from 0.9 GiB used before the test to full (about 4.0 GiB of 4 GiB) while node A loaded. The kernel moved idle pages out to make room. `vmstat` afterwards showed `si=32 so=0`, so nothing is being swapped out now. I'm watching it in Phase 3.

Node B was still Running after node A loaded: PID 1064630, Shard PID 1064719, and `nvidia-smi` at 18,272 MiB.

**PASS:**
- Node A logged `Shard ready  (32 blocks, is_first=false, is_last=false)`.
- Available memory stayed at or above 34.8 GiB throughout.
- Node B was still running.

### Phase 2 summary

- Node B, GPU 72–80: **PASS**. Ready in 5 s, using 18.4 GB of GPU memory.
- Node A, CPU 40–72: **PASS**. Ready in 275 s, with an RSS of 49.9 GiB and at least 34.8 GiB available throughout.
- Swap filled during node A's load, but nothing was being swapped out afterwards.

**metro-linux READY: node A CPU 40–72, node B GPU 72–80.**
