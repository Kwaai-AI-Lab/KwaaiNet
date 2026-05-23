# kwaai-compute — TODO

## Bug fixes
- [ ] Metro machine connectivity — DNS resolves to router; Ollama bound to localhost. Blocked on kwaai-network p2p relay work.

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
