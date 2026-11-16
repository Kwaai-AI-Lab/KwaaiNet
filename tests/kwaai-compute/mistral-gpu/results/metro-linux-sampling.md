# metro-linux: last-node sampling (PR #304), Small 24B

2026-11-16, metro-linux (RTX A5000, no host CUDA toolkit). Every command below is logged with its UTC start time and exit code.

## Result: node READY on the PR #304 build; smoke test (step 6) FAILED: the chain has no server for blocks 0–8

- **Head:** `H=6d92149d812adb01cd5c446f1d0ff74d3944525c` (`feat/sample-on-last-node`). It is one comment-only commit after 22583ea8, which this node ran overnight.
- **Store path:** `/nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet`.
- **Build:** 1000 s (16 min 40 s), exit 0, under `MemoryMax=48G`. `ldd` is clean: `libcuda.so.1` comes from the package's `libexec/cuda-driver-shim/`, and nothing is `not found`. `~/.cargo/bin/kwaainet` was not touched (still dated Nov 4).
- **Node:** ready at 19:31:17Z: `Blocks: [24, 40)`, `Device: CUDA (GPU 0)`, `Loading norm + lm_head`, `Shard [24..40) ready — embedding=false final_head=true`. The shard process holds **21,240 MiB** on the GPU (21,703 / 24,564 MiB on the card). The config was not changed.
- **Smoke test:** `shard run` found only 2 servers, metro-win (8–24) and metro-linux (24–40), and failed with `Error: No server covers block 0 — chain has a gap`. `shard chain --total-blocks 40` confirms `2 server(s) — 32/40 blocks covered`. professor_reza (blocks 0–8, the Mac) is not on the DHT. Following the instructions, I stopped there and did not work around it: no text, no ms/token, and so no comparison with Phase 4b. The request never reached this node; `shard.log` has no hop lines for it.

**One deviation, agreed with Reza beforehand:** step 2's `git worktree add ../KwaaiNet-lastsample "$H"` would have failed, because that path already held the overnight worktree at 22583ea8. Reza chose to check out `$H` in place with `git checkout --detach`.

Also seen: `start` printed `Mistral-Small-24B-Instruct-2501  2 server(s)  ✗ not installed` while the weights are cached and the node then loaded them. That's the same cosmetic map-reading issue as `Map entry: Llama-3.1-8B-Instruct`. There was also the usual harmless `inference-mux … failed after 5 attempts` warning.

## Command log

```
$ gh pr view 304 --json headRefOid --jq .headRefOid  [19:14:00Z]
6d92149d812adb01cd5c446f1d0ff74d3944525c
[exit 0]

$ git fetch origin  [19:14:00Z]
[exit 0]

$ git -C ../KwaaiNet-lastsample checkout --detach 6d92149d812adb01cd5c446f1d0ff74d3944525c  [19:14:01Z]
Previous HEAD position was 22583ea8 feat(shard): the last node samples the next token instead of returning logits
HEAD is now at 6d92149d docs(shard): say why prefill chunks also ask the last node to sample
[exit 0]

$ git -C ../KwaaiNet-lastsample log --oneline -1  [19:14:01Z]
6d92149d docs(shard): say why prefill chunks also ask the last node to sample
[exit 0]

$ git -C ../KwaaiNet-lastsample status --short  [19:14:01Z]
[exit 0]

$ systemd-run --user --scope -q -p MemoryMax=48G -p MemorySwapMax=0 nix build .#kwaainet-cuda --cores 16 --max-jobs 1 -L -o result-cuda  [19:14:08Z]
kwaainet> checking for references to /build/ in /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3...
kwaainet> patching script interpreter paths in /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3
kwaainet> stripping (with command strip and flags -S -p) in  /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin
[exit 0]  elapsed 1000s  (full log: sp-build.txt, 201 lines)

$ readlink -f result-cuda  [19:30:53Z]
/nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3
[exit 0]

K=/nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet
$ ldd /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet | grep -E 'cuda|not found'  [19:30:53Z]
	libnvidia-ptxjitcompiler.so.1 => /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/libexec/cuda-driver-shim/libnvidia-ptxjitcompiler.so.1 (0x00007fe0eb800000)
	libcudart.so.12 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcudart.so.12 (0x00007fe0eb400000)
	libcuda.so.1 => /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/libexec/cuda-driver-shim/libcuda.so.1 (0x00007fe0e5800000)
	libcurand.so.10 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcurand.so.10 (0x00007fe0dcc00000)
	libcublas.so.12 => /nix/store/25wzj612dc79hniya13a74lmjhk0wcxx-cuda-merged-12.8/lib/libcublas.so.12 (0x00007fe0d5a00000)
	libcublasLt.so.12 => /nix/store/3zp1b44ylx3q204yvmbp6c01rg7f1i3z-cuda12.8-libcublas-12.8.4.1-lib/lib/libcublasLt.so.12 (0x00007fe0a3200000)
[exit 0]

$ ldd /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet | grep -c 'not found'  [19:30:53Z]
0
[exit 1]

$ ps -eo pid,args | grep -E '^ *[0-9]+ \S*kwaainet' | grep -v grep  [19:30:53Z]
1206824 /nix/store/9fmkghw6598hgcigrwnwqm3rwia6h7rx-kwaainet-0.7.3/bin/kwaainet run-node --shard
1206906 /nix/store/9fmkghw6598hgcigrwnwqm3rwia6h7rx-kwaainet-0.7.3/bin/kwaainet shard serve --auto-rebalance
[exit 0]

$ grep -n -E '^model:|^blocks:|^start_block:|^use_gpu:|auto_update' ~/.kwaainet/config.yaml  [19:30:54Z]
1:model: unsloth/Mistral-Small-24B-Instruct-2501
2:blocks: 16
3:start_block: 24
5:use_gpu: true
57:  auto_update: false
[exit 0]

$ ls -la ~/.cargo/bin/kwaainet  [19:30:54Z]
-rwxr-xr-x. 1 rezarassool rezarassool 191631464 Nov  4 18:27 /home/rezarassool/.cargo/bin/kwaainet
[exit 0]

$ /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet stop  [19:31:01Z]


2026-11-16T19:31:01.986556Z  INFO kwaainet::daemon: Stopping daemon PID 1206824
  ✅ KwaaiNet daemon stopped
[exit 0]

$ ps -eo comm | grep -c '^kwaainet$'  [19:31:03Z]
0
[exit 1]

$ /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet start --daemon --shard  [19:31:03Z]


  Local models (1):
    • llama3.1:8b

  Network map (2 model(s)):
    Llama-3.1-8B-Instruct                       13 server(s)  ✓ have locally
    Mistral-Small-24B-Instruct-2501              2 server(s)  ✗ not installed

  ✅ Confirmed model: unsloth/Mistral-Small-24B-Instruct-2501
    Map entry:  Llama-3.1-8B-Instruct  (13 server(s))


  Model:   unsloth/Mistral-Small-24B-Instruct-2501
  Blocks:  16
  Port:    8080
  Peers:   2
  Name:    metro-linux

  ✅ KwaaiNet daemon started (PID 1265644)
  ✅ Shard serving: the daemon starts and supervises it
  💡 Shard logs:   kwaainet logs --shard
  💡 Storage not initialised — skipping. Run: kwaainet storage init
  💡 Check status: kwaainet status
  💡 View logs:    kwaainet logs
  💡 Stop daemon:  kwaainet stop
[exit 0]

$ ps -eo pid,args | grep -E '^ *[0-9]+ \S*kwaainet' | grep -v grep  [19:31:23Z]
1265644 /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet run-node --shard
1265735 /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet shard serve --auto-rebalance
[exit 0]

$ /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet logs --shard 2>&1 | sed 's/\x1b\[[0-9;]*m//g' | grep -E 'Device:|Blocks:|Loading norm|Shard ready|Shard \[|ERROR|panicked' | tail -8  [19:31:23Z]
2026-11-16T04:05:03.408709Z  INFO kwaai_inference::shard: Shard [24..40) ready — embedding=false final_head=true
  ✅ Shard ready  (16 blocks, is_first=false, is_last=true)
  Blocks:      [24, 40)
  Device:      CUDA (GPU 0)
2026-11-16T19:31:16.673400Z  INFO kwaai_inference::shard:   Loading norm + lm_head
2026-11-16T19:31:17.744720Z  INFO kwaai_inference::shard: Shard [24..40) ready — embedding=false final_head=true
  ✅ Shard ready  (16 blocks, is_first=false, is_last=true)
[exit 0]

$ nvidia-smi --query-compute-apps=pid,process_name,used_memory --format=csv  [19:31:23Z]
pid, process_name, used_gpu_memory [MiB]
1265735, /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet, 21240 MiB
[exit 0]

$ nvidia-smi --query-gpu=memory.used,memory.total --format=csv  [19:31:23Z]
memory.used [MiB], memory.total [MiB]
21703 MiB, 24564 MiB
[exit 0]

$ /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet shard run "[INST]Summarize the critique of the standard model of physics[/INST]" --max-tokens 128 --stats  [19:31:35Z]


  Model:        unsloth/Mistral-Small-24B-Instruct-2501
  DHT prefix:   Mistral-Small-24B-Instruct-2501
  Total blocks: 40
  Prompt:       "[INST]Summarize the critique of the standard model of physics[/INST]"

  Discovering block circuit from DHT…2 node(s)
  ⚠️  Block circuit has gaps — inference may be incomplete.

  [ 1] blocks   8– 23  12D3KooWLMizEbVi…  (metro-win-rtx-a5000/v0.7.3)  UNKNOWN
  [ 2] blocks  24– 39  12D3KooWA33TMz7s…  (metro-linux/v0.7.3)  UNKNOWN

2026-11-16T19:31:37.598617Z  INFO kwaai_inference::tokenizer: Loaded tokenizer.json: vocab=131072, bos=Some(1), eos=Some(2)
  Input tokens: 14
  Session ID:   15003688258757835287
  Max tokens:   128
Error: No server covers block 0 — chain has a gap (or all candidates blacklisted)
[exit 1]

$ /nix/store/w0a93sxg9mzw4165y1psmcpvzal6asa6-kwaainet-0.7.3/bin/kwaainet shard chain --total-blocks 40  [19:31:42Z]


  Model prefix: Mistral-Small-24B-Instruct-2501
  Querying 40 blocks from DHT…

    2 server(s) — 32/40 blocks covered

  START  END    PEER ID (prefix)   TRUST      NAME
  ─────────────────────────────────────────────────────────────────
      8     24  12D3KooWLMizEbVi…  UNKNOWN    metro-win-rtx-a5000/v0.…
     24     40  12D3KooWA33TMz7s…  UNKNOWN    metro-linux/v0.7.3

  Blocks: [░░░░░░░░████████████████████████████████]

  ⚠️  Gaps detected: 8 block(s) not served
[exit 0]

```
