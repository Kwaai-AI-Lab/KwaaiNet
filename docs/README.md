# KwaaiNet Documentation

Welcome to the KwaaiNet docs. This folder helps you understand what KwaaiNet is, how to run a node, how to build on it, and how to contribute to the Layer 8 roadmap.

KwaaiNet is a decentralized AI node architecture for **Layer 8** — the trust and intelligence layer above the traditional network stack — developed by the [Kwaai Foundation](https://www.kwaai.ai), a 501(c)(3) nonprofit AI lab.

---

## Who these docs are for

We write for three main audiences:

- **Node operators / DevOps** — You want to run one or more KwaaiNet nodes (on-prem, cloud, or home hardware), monitor them, and understand their trust and security properties.
- **Application developers** — You want to call KwaaiNet via an OpenAI-compatible API, integrate with existing apps/agents, or use Virtual Private Knowledge (VPK) as a backend.
- **Core contributors / protocol researchers** — You want to work on the Layer 8 stack itself: trust graph mechanics, sharded inference, VPK distribution, and intent-casting.

Each page will label its primary audience at the top to make navigation easier.

---

## Documentation map

### 1. Getting started

- [`getting-started-node.md`](getting-started-node.md)
  Run your first KwaaiNet node, generate identity, and confirm it is connected.
- [`api-quickstart.md`](api-quickstart.md)
  Call the OpenAI-compatible HTTP API (`/v1/models`, `/v1/chat/completions`) from Python/JS.

### 2. Concepts

- [`ARCHITECTURE.md`](ARCHITECTURE.md)
  Big-picture view of a KwaaiNet node: trust, compute, storage, network, and the Layer 8 stack.
- `trust-and-identity.md` *(planned)*
  Ed25519 identities, `did:peer:` DIDs, Verifiable Credentials, and local trust scores.
- `knowledge-and-VPK.md` *(planned)*
  Virtual Private Knowledge, encrypted vector search, and distributed personal AI memory.
- `network-and-intent-routing.md` *(planned)*
  libp2p + Kademlia DHT, trust-gated routing, and intent-casting as a Layer 8 business protocol.

### 3. How-to guides

- `run-multi-node.md` *(planned)*
  Connect multiple nodes, pin models, and observe shard chains in action.
- `deploy-cloud-node.md` *(planned)*
  Run a node on common cloud providers with recommended configs and security practices.
- `connect-ui-and-agents.md` *(planned)*
  Use KwaaiNet as a backend for existing UI/agent frameworks (e.g., OpenAI-compatible tools).

### 4. Reference

- `docker-images.md` *(planned)*
  Overview of official Docker images (bootstrap, node, etc.) and key env vars.
- `config-reference.md` *(planned)*
  Node configuration, CLI flags, and file layout under `~/.kwaainet/`.
- `api-reference.md` *(planned)*
  HTTP endpoints, request/response schemas, and error semantics.

### 5. Roadmap and contributing

- [`roadmap.md`](roadmap.md)
  Gap-based roadmap: aspirational Layer 8 architecture vs. current Rust implementation vs. planned work.
- [`contributor-guide.md`](contributor-guide.md)
  How to contribute code, docs, research, and operator feedback; "1 hour / 1 day / 1 week" paths.

---

## Background reading

For deeper context, we recommend:

- **[WHITEPAPER.md](WHITEPAPER.md)** — Layer 8: The Decentralized AI Trust Layer. High-level whitepaper on Layer 8, the trust pipeline, and governance.
- **[ARCHITECTURE.md](ARCHITECTURE.md)** — Technical description of node lobes, CandelEngine, VPK, and P2P networking.
- **[kwaai.ai/kwaainet](https://www.kwaai.ai/kwaainet)** — Public overview of KwaaiNet and its role in the Kwaai ecosystem.

These docs aim to translate those papers into practical guidance for running nodes, building apps, and extending the Layer 8 stack.
