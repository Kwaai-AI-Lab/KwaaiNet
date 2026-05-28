# KwaaiNet — Claude Code Instructions

## PR Triage (run at the start of every session)

Reza is responsible for merging all community pull requests. At the start of each conversation,
run `gh pr list` and surface any open PRs. If there are open PRs, call them out clearly so the
backlog doesn't grow silently.

```bash
gh pr list
```

Flag any PR that has been open longer than 7 days as overdue.

---

## Workspace

Rust workspace root: `core/`. Build the CLI binary:

```bash
cd core && cargo build -p kwaainet --release
cp core/target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet  # macOS 26+ required
```

Run all tests: `cd core && cargo test`

---

## Project map

| Project | Crates | CLI files | Docs |
|---------|--------|-----------|------|
| **kwaai-trust** | kwaai-trust, kwaai-wasm | identity.rs, trust.rs | `projects/kwaai-trust/` |
| **kwaai-network** | kwaai-p2p, kwaai-p2p-daemon, kwaai-hivemind-dht, kwaai-rpc | p2p_cmd.rs, node.rs | `projects/kwaai-network/` |
| **kwaai-compute** | kwaai-inference, kwaai-compression, kwaai-distributed | shard_cmd.rs, block_rpc.rs | `projects/kwaai-compute/` |
| **kwaai-storage** | kwaai-storage | vpk.rs | `projects/kwaai-storage/` |
| **kwaai-knowledge** | kwaai-rag | rag_cmd.rs, rag_api.rs | `projects/kwaai-knowledge/` |
| **kwaai-platform** | kwaai-cli, map-server, summit-server | main.rs, config.rs, updater.rs | `projects/kwaai-platform/` |

**For domain work, see `projects/{project}/CLAUDE.md`** — each contains: scope, crate ownership, infrastructure details, build commands, current state, key files, and do-not list.

Per-crate shortcuts also exist: `core/crates/{crate}/CLAUDE.md` points back to the project.

---

## Tests

`tests/{project}/` — integration and evaluation scripts per domain.
Most active: `tests/kwaai-knowledge/` — D6 eval, entity extraction experiments, family tree.
