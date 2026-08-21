# Macs serve whole models via Ollama, not slow shards

**Status:** approved 2026-08-21. Stopgap until an Apple fast path lands; see #117.

## Context

A Mac on KwaaiNet today serves transformer blocks through candle at **2–4 tok/s**
while the same machine runs the same model through Ollama at **47.6 tok/s** —
measured back to back on `rezas-mini-2`, same prompt, same 24 tokens:

| Path | Prefill (22 tok) | Decode |
|---|---|---|
| candle `auto` (Metal) | 5223 ms | 2.3 tok/s |
| candle `--no-gpu` (CPU) | 1627 ms | 4.2 tok/s |
| **Ollama (llama.cpp Metal)** | **102 ms** | **47.6 tok/s** |

Metal is *slower than CPU* on the candle path, so `DeviceType::detect_best()`
deliberately skips it and Macs land on CPU. Worse, gap-filling can hand a Mac a
**partial** range (#117) that Metal cannot serve on GPU at all, and `shard chain`
still shows it as healthy coverage.

**This is a stopgap, not the fix.** Three Apple paths exist and all are dark in
the shipped binary:

| Path | State | Why it is off |
|---|---|---|
| candle Metal | compiled (`features = ["metal"]`) | Skipped at runtime — 10× slower than CPU for decode |
| MLX (`mlx_shard.rs`, 1106 lines, **does** shard) | `mlx` feature off | Abandoned 2026-03-29: MLX recompiles its graph per eval, 100 s/forward. Two fix attempts failed |
| llama.cpp (`llama_local.rs`) | `llama-cpp` feature off — `default = ["storage", "rag"]` | Only wired into `grpc_server` streaming, never the serving path |

Ollama is llama.cpp underneath. It is already installed on these machines, it is
already fast, and **every node already registers `/kwaai/ollama-proxy/1.0.0`**
(`node.rs:340`, `node_native.rs:687`) — that is the path the D6 rebuild used. So
the quick win is to stop Macs pretending to be block servers and let them serve
whole models through the engine that already works.

## Decisions

| Decision | Choice |
|---|---|
| macOS block serving | **Stop entirely.** No slow candle path advertised |
| Whole-model serving | Local Ollama over the existing ollama-proxy protocol |
| Ollama absent | **Degrade loudly** — do not serve, say what to install. Never serve slowly while looking healthy |
| Scope | macOS only. Linux/CUDA behaviour unchanged |

## Changes

### 1. Skip block serving on macOS — `shard_cmd.rs`

The registration point is `shard_cmd.rs:362-366`:

```rust
let handler = make_block_rpc_handler(shard_cell.clone(), device.clone());
client.add_unary_handler(crate::block_rpc::INFERENCE_PROTO, handler, false)
```

On macOS, skip device detection, the model load, and this registration. Keep the
`OLLAMA_PROXY_PROTO` and `SHARD_PROXY_PROTO` registrations that follow at
`:382-398` — they are what make the node useful.

Because no shard loads, `ShardManager::shard_is_ready()` stays false and
`KwaaiNetConfig::announce_state()` (`config.rs:794`) already returns `0` rather
than `2`. **No announce changes needed** — the existing "online, no shard"
state is exactly right.

This also removes Macs from the auto-assign path, so **#117 stops being reachable
on macOS** as a side effect: a Mac can no longer be handed a partial range.

### 2. Probe Ollama, and be explicit either way

Before registering the proxy handler on macOS, check the local Ollama on
`cfg.ollama_port` (`config.rs:227`, default 11434). Reuse
`ollama::list_local_models()` (`ollama.rs:110`) so the check covers *and the
configured model is present*, not merely that the port answers.

- **Reachable, model present** — register the proxy, log that this node serves
  the whole model via Ollama and roughly what that is worth.
- **Absent** — do not register the proxy either. Log plainly that macOS requires
  Ollama until a native fast path lands, name the model to pull, and exit
  non-zero from `shard serve` so it is visible in a service log rather than
  silently idle.

Advertising a proxy that cannot answer is the same failure class as advertising
slow blocks, so both paths stay honest.

### 3. Say what is actually happening — `shard_cmd.rs` status

`shard status` currently prints the configured `GPU:` flag while the process runs
on CPU — that is #118, and it misled us for an hour today. On macOS it should
report the serving mode plainly:

```
Mode:    whole model via Ollama (block sharding unsupported on Metal)
Ollama:  reachable, llama3.1:8b present
```

Fold the #118 fix in here rather than leaving two half-truths.

## What this does not do

- **Does not make Metal shard.** #117 stays open as the real fix; this removes
  its blast radius on macOS.
- **Does not revive MLX or llama.cpp in-process.** Both remain viable long-term
  answers — MLX especially, since it already implements sharding and only failed
  on graph recompilation. Reviving either is a separate piece of work.
- **Does not add Ollama to the installer.** Worth doing (`feedback_end_user_experience`:
  installers must handle dependencies), but it is packaging work, not this change.

## Verification

Run on `rezas-mini-2`, which reproduces every symptom today:

1. **Block path gone.** `kwaainet shard serve` on macOS registers no
   `INFERENCE_PROTO` handler; `kwaainet shard chain` no longer lists this node
   among block servers.
2. **Inference path alive and fast.**
   `kwaainet p2p probe --peer <self-or-peer> --proto /kwaai/ollama-proxy/1.0.0`
   answers, and a real generation through the p2p path lands near the **47.6
   tok/s** Ollama baseline rather than 2–4 — measure with ≥14 tokens, since
   shorter runs inflate the figure 2–5× (`project_p2p_relay_token_latency`).
3. **Loud degradation.** Stop Ollama, restart `shard serve`, confirm it refuses
   with an actionable message and registers nothing.
4. **Linux unaffected.** metro-linux still serves blocks and still appears in
   `shard chain` with its range.
5. **Regression tests** for the platform gate and the probe outcome, in the style
   of the `start_block_pinning` tests added for #116.

## Open

- **Discovery.** A Mac serving via ollama-proxy is reachable at `p2p://<peer-id>`
  but is not discoverable the way block servers are through `shard chain`. Fine
  for RAG, which names peers explicitly; a gap for anything that wants to *find*
  a whole-model node. A capability flag in `DHTServerInfo` would fix it — there
  is precedent in `lease_v1` (`announce.rs:145-155`), and unknown map keys are
  ignored by legacy Hivemind clients, so it extends safely.
- **Coverage.** Macs stop contributing block coverage. Given they were
  contributing at 2–4 tok/s, that is a gain, but worth watching `shard chain`
  during rollout.
