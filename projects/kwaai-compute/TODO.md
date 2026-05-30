# kwaai-compute — TODO

## Bug fixes
- [ ] Metro machine connectivity — DNS resolves to router; Ollama bound to localhost. Blocked on kwaai-network p2p relay work.
- [ ] **Inference-mux silent failure after auto-update restart** — `start_inference_mux_server()` in `shard_cmd.rs:377` uses `.map_err(...).ok()` which swallows the error when `register_stream_handler` fails on startup (race with p2pd, or first-boot timing). Result: shard serve runs normally but `/kwaai/inference-mux/1.0.0` is never registered; mux:// clients get `stream_open_raw` errors. Fix: either retry registration with backoff, or propagate the error and let the caller decide (currently it's a soft failure). Workaround: manual restart of kwaainet after update.

## Known platform behavior
- [ ] **Windows: model cold-loads on every `shard run --local`** — On Linux, OS page cache keeps mmap'd model weights in RAM after process exit so subsequent runs are near-instant. On Windows the page cache evicts file-backed pages on process exit, causing a full cold load every invocation. Not a bug in kwaainet — it's OS mmap behavior. Fix: always use `shard serve` (keeps model in a live process, page cache irrelevant) rather than `shard run --local` on Windows. Alternative: `OLLAMA_KEEP_ALIVE=-1` if using Ollama as local backend. Do NOT pursue `VirtualLock` — requires elevated privileges.

## Features
- [ ] Dedicated inference thread with session pool (LRU eviction, prefix-match session reuse) — plan at `~/.claude/plans/cached-jingling-creek.md`. Files: `llama_local.rs`, `shard_api.rs`
- [ ] Trust-routed shard selection — filter peers by minimum trust tier
- [ ] `kwaainet_version` in `DHTServerInfo` for peer version visibility
- [ ] KV-cache scrambling for collusion resistance

## Tests
- [ ] Two-shard integration test (two local `shard serve` instances + one `shard run`)
- [ ] Broadcast semantics unit test (assert `broadcast_mul` vs `*` shape mismatch detection)
- [ ] Session KV-cache TTL eviction test

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
- [ ] Document metro machine actual IPs once p2p relay is working
