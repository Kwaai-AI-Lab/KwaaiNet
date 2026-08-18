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

## Architecture — four pillars

The KwaaiNet node architecture has four pillars, plus a cross-cutting platform layer that hosts
them. Pillars describe **capabilities**; workspaces group the **crates** that implement them, and
the two do not map one-to-one (noted below where they diverge).

| Pillar | Capabilities | Workspace(s) |
|--------|--------------|--------------|
| **Trust** | Decentralized Trust Graph · Homomorphic encryption, provenance tracking · Wallet | `kwaai-trust` |
| **Compute** | Sharded LLM inference · Decentralized model training (coming) · Custom embedding · Tool calling | `kwaai-compute` |
| **Storage** | Knowledge base (graph, vector, SQL) · Multi-tenant vector DB hosting (PHE secured) · Distributed Hash Table | `kwaai-storage`, `kwaai-knowledge` |
| **Network** | P2P communication · Discovery · Intent casting | `kwaai-network` |
| _(cross-cutting)_ | CLI, config, install/update, CI, release, map.kwaai.ai | `kwaai-platform` |

**Where capability and crate diverge:**
- **DHT** is a Storage capability, but `kwaai-hivemind-dht` lives in the `kwaai-network` workspace
  — it is libp2p/Kademlia transport-coupled and ships with the p2p stack.
- **Custom embedding** is a Compute capability, but the embedder (`kwaai-rag/src/embedder.rs`)
  lives in `kwaai-knowledge` alongside the pipeline that calls it.

## Project map

| Project | Pillar | Crates | CLI files | Docs |
|---------|--------|--------|-----------|------|
| **kwaai-trust** | Trust | kwaai-trust, kwaai-ledger, kwaai-wasm | identity.rs, trust.rs, reputation*.rs, ledger*.rs | `projects/kwaai-trust/` |
| **kwaai-network** | Network | kwaai-p2p, kwaai-p2p-daemon, kwaai-hivemind-dht, kwaai-rpc, kwaai-network-tests | p2p_cmd.rs, node.rs, grpc_server.rs | `projects/kwaai-network/` |
| **kwaai-compute** | Compute | kwaai-inference, kwaai-compression, kwaai-distributed | shard_cmd.rs, block_rpc.rs | `projects/kwaai-compute/` |
| **kwaai-storage** | Storage | kwaai-storage | vpk.rs, storage_rpc.rs | `projects/kwaai-storage/` |
| **kwaai-knowledge** | Storage | kwaai-rag | rag_cmd.rs, rag_api.rs | `projects/kwaai-knowledge/` |
| **kwaai-platform** | cross-cutting | kwaai-cli (`kwaainet`), map-server | main.rs, cli.rs, config.rs, updater.rs | `projects/kwaai-platform/` |

All 15 workspace crates are claimed by exactly one project. **For domain work, see
`projects/{project}/CLAUDE.md`** — each contains: scope, crate ownership, infrastructure details,
build commands, current state, key files, and do-not list.

Every crate also has `core/crates/{crate}/CLAUDE.md` — a short pointer with that crate's key
source files and gotchas, naming the project it belongs to.

## Workspace layout

Each `projects/{project}/` follows the same structure — see `projects/README.md` for the contract:

```
projects/{project}/
  {project}.code-workspace   VS Code multi-root workspace (crates + kwaai-cli/src + docs + tests)
  CLAUDE.md                  scope, crate ownership, build, current state, do-not list
  requirements.md            what it must do
  roadmap.md                 phased plan
  TODO.md                    active checklist
  design/overview.md         architecture
  design/data-flows.md       sequence/data-flow diagrams
  plans/                     dated plans, proposals, reports
```

---

## Tests

`tests/{project}/` — integration and evaluation scripts per domain.
Most active: `tests/kwaai-knowledge/` — D6 eval, entity extraction experiments, family tree.
