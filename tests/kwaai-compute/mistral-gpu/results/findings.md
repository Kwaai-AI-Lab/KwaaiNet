# Findings: Mistral on three GPUs, Mac (MLX) + metro-win (CUDA) + metro-linux (CUDA)

Written 2026-11-15 from `results/runs.md`, `mac-mini.md`, `metro-win.md` and `metro-linux.md` on this branch. The code under test is `fix/config-head-dim` at `e0d17e26` (PR #292).

## Verdict

**The goal is met.** Two models that no single participating GPU can hold, Mistral Nemo 12B (24.5 GB) and Mistral Small 24B (47 GB), ran by blocks across three GPUs on three platforms. Every pass criterion in the plan was met, with one exception: the prompts were Reza's three rather than the plan's three.

| Criterion | Nemo 12B | Small 24B |
|---|---|---|
| 1. `shard chain` 40/40 from exactly three nodes | PASS | PASS |
| 2. Pinned path Mac → metro-win → metro-linux every run | PASS (4/4 Mac-client runs, 3/3 metro-linux-client runs) | PASS (4/4) |
| 3. Coherent, correct answers, no hop errors | PASS, with `[INST]` added by hand. **FAIL from the GUI** (F3) | PASS (`[INST]` by hand) |
| 4. Every node on its GPU, no card over 23 GB, no OOM | PASS. Peak card use 12.3 GB (metro-linux) | PASS, **tight**: metro-linux card at 21.7 GB, 2.8 GiB free (F5) |
| 5. Per-hop stats over ≥14 tokens | PASS | PASS |

The 70B attempt failed on CPU blocks, timeouts, #291 and Mac memory pressure. This run hit none of those. It used GPU only, one node per machine, and kept each node's footprint well inside its card.

## Headline numbers

| | Nemo 12B | Small 24B |
|---|---|---|
| Decode, client on the Mac | 242–330 ms/tok (3.0–3.9 tok/s) | 305–387 ms/tok (2.5–3.3 tok/s) |
| Decode, client on metro-linux | **114–119 ms/tok (~8 tok/s)** | not run |
| Prefill, 8–14 tokens, warm | 0.6–0.7 s | 0.6–0.9 s |
| Cold first run (Mac pages in MLX weights) | +1.0 s prefill | +1.8 s prefill |
| Shard load to ready | Mac 13 s · win 7.2 s · linux 3.3 s | Mac 25 s · win 13.4 s · linux 5.0 s |

## Findings

### F1. The fix in #292 works on all three backends and both models. Merge it.

There was no `q_proj: shape mismatch` on any node, in either model:
- MLX on macOS;
- CUDA on Windows (`cuda-windows`, without flash-attn);
- CUDA on Linux (`cuda`, with flash-attn, built by Nix).

The answers are fluent. **The numerics also agree across backends.** For the same prompt over the same three nodes, the Mac as client and metro-linux as client produced identical text, word for word (`runs.md` Nemo runs 1–2, `metro-linux.md` Phase 3b run 1). Decoding is greedy, so any disagreement in the MLX or CUDA attention math would have shown up as a different token.

PR #292 is `MERGEABLE/CLEAN` at `e0d17e26`.

### F2. The slow last hop is most likely the 256 KB of logits it returns per token, not metro-linux's GPU.

metro-linux's own forward time is steady at **about 18 ms** per token for blocks 24–40 plus `lm_head` (1,215 hops, median 17.6 ms, p99 18.1 ms). Yet the Mac client sees that hop take **136–229 ms**, with maxima up to 538 ms.

| Hop seen by the client | Client on the Mac | Client on metro-linux |
|---|---|---|
| Mac, 0–8 | ~21 ms | ~50 ms |
| metro-win, 8–24 | 83–121 ms | 38–43 ms |
| metro-linux, 24–40 + head | 136–229 ms | ~25 ms |
| **Decode per token** | **242–330 ms** | **114–119 ms** |

The last node is the only one whose response is large. From `block_rpc.rs` and `shard.rs → forward_last`:
- a middle hop returns one hidden state, `[1, 1, 5120]` f16 = **10 KB**;
- the last hop returns full-vocabulary logits, `[1, 1, 131072]` f16 = **256 KB**, about 25 times more, **on every token**.

When the client is on metro-linux, that 256 KB never leaves the machine, so the hop costs about 25 ms. When the client is the Mac, it crosses the relayed WAN path every token. The extra 75–155 ms beyond the RTT matches moving 256 KB at roughly 13–27 Mbit/s.

The earlier measurement in memory, 411 ms/tok with metro-linux CUDA on the 8B, has the same shape: Llama 3's vocabulary is 128,256.

**Status:** strongly indicated, not yet isolated. The metro-linux-client runs move the client next to the logits *and* change every RTT at once. The clean test is a client on the Mac with sampling moved to the last node, or a top-k-only logits response.

**Fix direction:** have the last node return the sampled token id, or top-k logits, instead of the full vocabulary. The request already carries what is needed: the client samples with greedy, top-k or top-p settings that could be sent along. This would cut per-token traffic on the last hop from 256 KB to bytes. It matters more than #268's forward-chain routing for this layout, because a chained last node still returns the logits to the client.

### F3. Chat from the GUI is broken for Mistral (and any model that isn't Llama 3 or ChatML): #293.

The GUI's chat goes gRPC `ShardRun` → `shard_cmd::run_streaming`. That path:
- **has no Mistral template**, so the prompt goes in raw;
- **decodes greedily** with no repetition penalty;
- **stops silently at 200 tokens.**

With a raw prompt the model continues it as a document: 2 of 3 GUI answers looped or rambled. Reproduced word for word from the Mac CLI with raw prompts, and fixed by wrapping in `[INST]` (`runs.md`). The HTTP APIs (`shard_api.rs`, `api.rs`) and `grpc_server.rs → build_prompt` assume Llama 3 for every model.

Until #293 is fixed, a Mistral demo must use the CLI with `[INST]` wrapping.

### F4. metro-win's server-side time is about 2.4 times metro-linux's for the same 16 blocks on the same GPU model. Unexplained.

Both machines have an RTX A5000 and serve 16 Nemo blocks:
- **metro-win:** `hop timing` `fwd_ms` median 44.2 ms, p90 75.7 ms (1,024 steps);
- **metro-linux:** median 17.6 ms, and it also computes `lm_head`.

`fwd_ms` covers the lock, deserialisation, the forward pass and the copy back to the host, so it is comparable between the two. Candidates, none tested:
- no flash-attn on Windows (`cuda-windows` maps to no-flash);
- kernels for compute capability 8.0 on metro-win, against Nix's native 8.6;
- WDDM scheduling and GPU clocks dropping between tokens at about 2–3 tok/s.

metro-win's hop seen from the metro-linux client (38–43 ms in total) is *below* its Phase 3 median. That points at clock or idle behaviour rather than a fixed compute cost. This is worth one controlled run (same client, nvidia-smi clocks logged) before blaming the build.

### F5. GPU headroom on the last node is the binding constraint for Small 24B.

| Node | Model | Weights (estimated) | Shard on GPU | Card total | Free |
|---|---|---|---|---|---|
| metro-linux 24–40 + head | Nemo | 10.1 GB | 11.8 GB, then 10.6 GB | 12.3 GB | ~12 GB |
| metro-linux 24–40 + head | Small 24B | 19.1 GB | **21.2 GB** | **21.7 GB** | **2.8 GiB** |
| metro-win 8–24 | Nemo | 8.7 GB | ≈9.4 GB | 10.1 GB | ~14 GB |
| metro-win 8–24 | Small 24B | 17.8 GB | ≈19.1 GB | 19.8 GB | ~4.7 GB |
| Mac 0–8 + embeddings | Nemo / Small 24B | 5.7 / 10.2 GB | 5.5 / 9.9 GB footprint | n/a, 24 GB unified | 35% system free |

- **The CUDA shards carry 1.3–2.1 GB over their weights** (CUDA context, cuBLAS workspace). The plan's estimates left this out.
- **With Small 24B, metro-linux has 2.8 GiB left for KV cache.** That is fine for these short prompts with one session. Long contexts or concurrent sessions could run it out of memory. Small 24B's KV cache is 40 layers × 8 KV heads × 128 × 2 (K and V) × 2 bytes, about 160 KB per token for all 40 layers, or 64 KB per token for this node's 16 blocks. So 2.8 GiB is roughly 45k tokens across all sessions, if the cache grows as needed. Activations and allocator fragmentation will take some of that.
- **A balanced layout would move blocks off the head node,** e.g. 0–10 / 10–26 / 26–40. Today's valid counts (4, 8, 16, 32) don't allow that split; the block-count restriction is what forces the imbalance.
- **The Mac was never under pressure** at 10 GB of shard. The 70B run stalled at 15 GB with rust-analyzer and Chrome open.

### F6. The Mac's 300 s `REQUEST_TIMEOUT` patch wasn't needed for GPU-only runs.

The slowest decode step in any run (all three hops) was 778 ms, and the slowest prefill 2.8 s, far under the default 30 s. The patch only matters for CPU blocks (#134). The Mac binary was built with it, but no run depended on it.

### F7. The test instructions were wrong about how to build on both metros.

- **metro-win:** `nvcc` needs MSVC on `PATH`. The build must run from a `vcvars64.bat` shell (as `docs/runbooks/build-on-metro-win.md` says). The first attempt failed with `Cannot find compiler 'cl.exe'`.
- **metro-linux:** there is no host CUDA toolkit, so `cargo build --features cuda` cannot work. The Nix `kwaainet-cuda` package (#260) built it in 38.5 min, for compute capability 8.6. Its `postFixup` driver shim made CUDA initialise through the real host driver (verified: `ldd` clean, `Device: CUDA (GPU 0)`, and the shard process itself holding GPU memory).
- **Both:** `core/patches/` is git-ignored, so a new worktree needs the patched crates linked or copied in. A Nix flake build ignores them, because `crane.nix` brings its own.

Any future multi-machine test should link the per-machine build runbook rather than restate a build command.

### F8. Smaller defects seen along the way

| # | Defect | Status |
|---|---|---|
| a | `shard download` guesses 32 blocks from the model name before `config.json` is cached, so a first download past block 32 is clamped and misses `lm_head`. The two-step workaround worked, and step 1 even fetched the head file by accident. | **Not filed** |
| b | A Windows `kwaainet stop` is `taskkill /F`, so it writes no tombstone. Combined with the bootstraps serving expired subkeys, the map showed metro-win on the 8B for hours. | #294, #286. Fix in **#295**, tested end to end on metro-win during Phase 4 (four scenarios plus the negative cases, all PASS) |
| c | Three node starts on metro-win within six minutes each failed their first announce (`Failed to negotiate transport protocol(s)` on both bootstraps), then recovered in 2–5 min. Possibly the bootstraps rate-limiting reconnects. | Not investigated, not filed |
| d | The headline tok/s in `shard run --stats` disagrees with its own timing block (3.4 against 3.0 for 128 tokens in 42.6 s). | Cosmetic, not filed |
| e | One answer ended in `���`, probably an emoji split across byte-level tokens and decoded one token at a time. | Unverified, not filed |
| f | `inference-mux server failed to start after 5 attempts` on every shard start, on every OS. The shard works regardless. | Pre-existing, not filed |
| g | `temperature 1.0` means greedy in `sample_token`. OpenAI clients send 1.0 to mean "sample normally". | In the #293 comment |

## Recommendations, in order

1. **Merge #292** (F1). It is the only code this test needed.
2. **Fix #293** before any Mistral demo from the GUI (F3). The minimum is an `[INST]` probe in the three copies of the template selection in `shard_cmd.rs`. Better is one shared function that renders `tokenizer_config.json`'s `chat_template`.
3. **Test, then fix, the logits return** (F2): sample on the last node, or return top-k. This is probably the largest single speed-up available to every sharded model, about 2× for this layout, judging by the metro-linux-client runs.
4. **File F8a**, the `shard download` clamp. It is a one-line read of `config.json`, or a fetch of it before the range check.
5. **Run one controlled metro-win timing test** (F4) before changing the Windows build.
6. **For larger models, plan GPU headroom** at about 2 GB per CUDA shard over its weights (F5). Consider allowing block counts other than 4, 8, 16 and 32, so the head node can take fewer blocks.

## What would make this a demo

- #292 and #293 merged, so the GUI chats correctly with Small 24B.
- F2 fixed, so a Mac client gets the ~8 tok/s that the metro-linux client saw.
- Small 24B run from the GUI on all three machines, with a longer answer (`max_tokens` passed through, per #293), and GPU memory watched on metro-linux.
