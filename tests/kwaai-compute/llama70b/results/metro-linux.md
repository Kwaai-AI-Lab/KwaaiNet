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
