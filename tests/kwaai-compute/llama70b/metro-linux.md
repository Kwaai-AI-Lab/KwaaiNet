# metro-linux: two nodes, blocks 40–80

Read [README.md](README.md) first; its rules apply. Write everything to `tests/kwaai-compute/llama70b/results/metro-linux.md`.

This machine runs **two** KwaaiNet nodes:

| Node | Home | Device | Blocks |
|---|---|---|---|
| B | the existing `~/.kwaainet` | GPU | 72–80, plus `lm_head` and the final norm |
| A | a new `~/.kwaainet-70b-cpu` | CPU | 40–72 |

Two nodes need separate homes, sockets and ports, or the second refuses to start. Node A takes about 55 GB of RAM.

**This box has run out of memory before.** Watch `free -g` while node A loads. Stop node A at once if available memory drops below 8 GiB.

## Phase 1: baseline, backups, download (the 8B node keeps running)

1. Baseline:
   ```bash
   kwaainet --version; kwaainet status
   free -g; nvidia-smi; df -h ~
   ```
   Record all of it.
2. Backups:
   ```bash
   cp ~/.kwaainet/config.yaml ~/config.70b-backup.yaml
   cp ~/.kwaainet/run/start-args.json ~/start-args.70b-backup.json
   cat ~/start-args.70b-backup.json
   ```
   Record the start-args contents.
3. Download blocks 40–80 (about 70 GB) under a throwaway home. Both nodes share the one Hugging Face cache.
   ```bash
   export KWAAINET_HOME=$HOME/.kwaainet-70b-dl
   kwaainet config set model unsloth/Llama-3.3-70B-Instruct
   kwaainet shard download --start-block 40 --blocks 32
   kwaainet shard download --start-block 72 --blocks 8
   unset KWAAINET_HOME
   ```
   The model is set in config first because `shard download` takes the block total from config, and would otherwise clamp the range to 32 blocks.

   **PASS:** both downloads complete. The snapshot has `config.json`, `tokenizer.json`, `model.norm`, `lm_head` and the safetensors files containing `model.layers.40` through `model.layers.79`. Record the time taken and the snapshot path and size.
4. Commit and push: `test(70b): metro-linux download`. Tell Reza the download is done.

## Phase 2: bring up both nodes (stops the 8B here)

Wait for Reza to say go. Bring up node B first, then node A.

### Node B: the existing node, GPU, blocks 72–80

Use a shell **without** the node-A variables.

1. Switch:
   ```bash
   kwaainet stop
   kwaainet config set contribute.auto_update false
   kwaainet config set model unsloth/Llama-3.3-70B-Instruct
   kwaainet config set use_gpu true
   kwaainet config set start_block 72
   kwaainet config set blocks 8
   kwaainet start --daemon --shard
   tail -f ~/.kwaainet/logs/shard.log      # until ready
   ```
2. **PASS:** `Shard ready  (8 blocks, is_first=false, is_last=true)`. Record `nvidia-smi` (expect about 16 GB used) and `free -g`.

### Node A: new home, CPU, blocks 40–72

Use a **separate** shell. Every node-A command needs all four variables set in it:

1. Set up and start:
   ```bash
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
   tail -f $KWAAINET_HOME/logs/shard.log   # until ready
   ```
   In a third shell: `watch -n 5 free -g`.
2. **PASS** when all of these hold:
   - node A logs `Shard ready  (32 blocks, is_first=false, is_last=false)`;
   - available memory stayed above 8 GiB throughout;
   - node B is still running (`kwaainet status` in node B's shell).
3. Record node A's load time, its shard process RSS (`ps -o rss= -p $(cat $KWAAINET_HOME/run/shard.pid)`), and `free -g` after load.
4. If node A fails because of a port or socket clash, record the exact error and stop. Don't try other ports without asking Reza.

The fresh home generates its own identity, so node A gets a new peer ID. Record it from its `kwaainet status` or the log. It has no storage initialised, so it doesn't compete with node B for storage port 7432.

### Report

Commit and push: `test(70b): metro-linux ready`. Tell Reza: **"metro-linux READY: node A CPU 40–72, node B GPU 72–80"**, with the `nvidia-smi` and `free -g` figures.

## Phase 3: during Reza's runs

Leave both nodes running. Every 30 s or so while Reza runs prompts, record `free -g`, `nvidia-smi`, and whether anything swaps. Note anything new in either `shard.log`, such as errors or "Message too large". Commit and push: `test(70b): metro-linux runs`.

## Phase 4: restore (only when Reza says so)

1. Node A, in a shell with the four variables set:
   ```bash
   kwaainet stop
   ```
2. Then, in a clean shell:
   ```bash
   unset KWAAINET_HOME KWAAINET_SOCKET KWAAINET_PORT KWAAINET_GRPC_PORT
   rm -rf ~/.kwaainet-70b-cpu ~/.kwaainet-70b-dl
   kwaainet stop
   cp ~/config.70b-backup.yaml ~/.kwaainet/config.yaml
   cp ~/start-args.70b-backup.json ~/.kwaainet/run/start-args.json
   kwaainet restart
   kwaainet status
   ```
   **PASS:** node A is gone, and node B is back to the 8B with its original flags, as in the Phase 1 baseline.
3. Leave the 70B weights (about 70 GB) in the Hugging Face cache unless Reza says to delete them.
4. Commit and push: `test(70b): metro-linux restored`.
