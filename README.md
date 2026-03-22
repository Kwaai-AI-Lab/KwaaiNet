# KwaaiNet

[![KwaaiNet — Sovereign AI Infrastructure](https://img.youtube.com/vi/ES9iQWkAFeY/maxresdefault.jpg)](https://youtu.be/ES9iQWkAFeY)

KwaaiNet is a decentralized AI node architecture for **Layer 8** — the trust and intelligence layer above the traditional network stack — built by the [Kwaai Foundation](https://www.kwaai.ai), a 501(c)(3) nonprofit AI lab focused on democratizing AI.

Each KwaaiNet node combines:

- A **decentralized trust graph** (cryptographic identity, verifiable credentials, local trust scores).
- **Shared, sharded LLM compute** over heterogeneous CPUs/GPUs using Petals-style distributed inference.
- **Secure multi-tenant knowledge storage** via Virtual Private Knowledge (VPK) with encrypted vector search.
- **Intent-based, peer-to-peer networking** that routes based on "what I need" (model, trust tier, latency), not just IP addresses.

From an app's point of view, KwaaiNet looks like a familiar chat-completion style HTTP API. Under the hood, it is a person-anchored Layer 8 fabric where every node is tied to an accountable human or organization.

---

## Why KwaaiNet?

Today's "Layer 8" — the AI and agent layer that mediates how people see information and act in the world — is mostly provided by closed platforms you rent and cannot inspect.

KwaaiNet offers an alternative:

- **Owners, not renters** — Run intelligent agents on infrastructure you and your community own and govern, instead of renting access to proprietary stacks.
- **Trust-first, not anonymous compute** — Every node carries an Ed25519-anchored identity, W3C Verifiable Credentials, and a local, time-decayed trust score; there is no central trust registry.
- **Knowledge as a first-class, private citizen** — VPK lets you shard encrypted knowledge across nodes and query it without exposing raw content.
- **Intent-based networking** — Nodes route requests based on intents like "model X, minimum trust tier Verified, max latency Y," making the network semantic and economic, not just transport.

For the full architectural and philosophical context, see:

- **Layer 8: The Decentralized AI Trust Layer** (whitepaper) — available via the [Kwaai website](https://www.kwaai.ai/kwaainet).
- **KwaaiNet: Decentralized AI Node Architecture for Layer 8** (technical architecture) — available via the [Kwaai website](https://www.kwaai.ai/kwaainet).

---

## Project status: where we are now

KwaaiNet is under active development. The Rust CLI and node implementation already ship many core capabilities; others are in progress or still research.

Today, a KwaaiNet node can:

- Run as a native Rust binary (`kwaainet`) with pre-built cross-platform releases.
- Generate a persistent Ed25519 keypair at `~/.kwaainet/identity.key` and derive a stable `PeerId` / `did:peer:` DID.
- Maintain a local W3C Verifiable Credential wallet under `~/.kwaainet/credentials/` with credential types like `FiduciaryPledgeVC`, `VerifiedNodeVC`, `UptimeVC`, `ThroughputVC`, `EventAttendeeVC`, and `PeerEndorsementVC`.
- Compute a local, time-decayed trust score for peers, grouped into tiers (`Unknown`, `Known`, `Verified`, `Trusted`).
- Join a libp2p + Kademlia DHT swarm compatible with Petals/Hivemind for node discovery and health checks.
- Serve and consume **block-sharded LLM inference** (CandelEngine): SafeTensors loading, RoPE, GQA, SwiGLU, per-session KV-cache, and temperature/top-k/top-p sampling, exposed through an OpenAI-compatible HTTP API.
- Auto-detect local models and network state to smart-select what to serve, and appear on the public map when properly configured at [map.kwaai.ai](https://map.kwaai.ai).

See the [latest GitHub Release](https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest) for the most recent feature list and release notes.

---

## Quickstart: run a node and make a request

This quickstart shows how to install the native Rust CLI, start a node, and send a simple chat-completion request against its OpenAI-compatible endpoint.

> **Note:** Exact flags and defaults may evolve. Check `kwaainet --help` for current options.

### 1. Install the `kwaainet` CLI

**Shell installer (macOS / Linux):**

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh
```

**PowerShell installer (Windows):**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1 | iex"
```

**Homebrew (macOS / Linux — optional):**

```bash
brew install kwaai-ai-lab/tap/kwaainet
```

**Build from source (Rust toolchain required):**

```bash
cargo install --git https://github.com/Kwaai-AI-Lab/KwaaiNet kwaainet
```

Then confirm:

```bash
kwaainet --help
```

### 2. Initialize and start a node

Initialize node identity and config:

```bash
kwaainet setup
```

This generates `~/.kwaainet/identity.key` (Ed25519 keypair) and creates a default config with a smart default node name (e.g. `alice-linux-aarch64`).

> If `kwaainet start` reports that `p2pd` is missing (e.g. manual install from a `.tar.xz`), run `kwaainet setup --get-deps` to download and install it automatically.

Start the node:

```bash
kwaainet start --daemon
```

The node will:

- Connect to bootstrap peers and announce itself on the DHT.
- Load or download model shards (depending on your configuration).
- Expose an HTTP API compatible with the OpenAI chat-completion interface.

### 3. Call the OpenAI-compatible API

```bash
curl http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "your-model-id",
    "messages": [
      {"role": "user", "content": "Hello, KwaaiNet!"}
    ]
  }'
```

This sends a chat-completion request to your local node, which may route it through a shard chain of other nodes depending on configuration and trust requirements.

More detailed getting-started guides (including Windows/macOS specifics, Docker, and advanced configuration) will live under `docs/` in future PRs.

---

## Roadmap: destination vs current implementation

KwaaiNet's roadmap is defined as the **gap** between the aspirational Layer 8 architecture in the whitepapers and the currently shipping Rust implementation.

| Area    | Aspirational (whitepapers)                                                                 | Current implementation (Rust node)                                       |
|---------|--------------------------------------------------------------------------------------------|---------------------------------------------------------------------------|
| Trust   | 5-layer trust pipeline including Testable Credentials (PVP-1) and EigenTrust propagation. | Identity + VC wallet + local time-decayed trust scores shipped; ToIP work in progress. |
| Compute | Sharded inference, decentralized training, safe tool-calling with trust-gated policies.   | Petals-style block-sharded inference and OpenAI-compatible API shipped. |
| Storage | Fully distributed personal AI memory via cross-node VPK sharding and DHT-backed resolution. | VPK process, roles (bob/eve/both), encrypted vector search, and DHT advertisement shipped. |
| Network | Intent-casting as a Layer 8 business protocol with economic settlement and neutrality guarantees. | libp2p + Kademlia DHT, trust-gated routing by model/trust/latency shipped. |

A more detailed, living roadmap (with phases, issues, and contribution ideas) will be maintained in `docs/roadmap.md` and linked here in a follow-up PR.

---

## Who is building KwaaiNet?

KwaaiNet is developed by the **[Kwaai Foundation](https://www.kwaai.ai)**, a 501(c)(3) nonprofit AI lab and proud signatory of the [GliaNet Fiduciary Pledge](https://www.glianetalliance.org/pledge).

- **Mission:** democratize AI by building open, person-anchored infrastructure and Personal AI systems.
- **Values:** personal control, self-sovereign identity, transparency, openness.
- **Role of KwaaiNet:** serve as the decentralized AI trust and compute layer (Layer 8) for the broader Kwaai ecosystem and allied open-source projects.

Learn more at [kwaai.ai](https://www.kwaai.ai) and the [Kwaai-AI-Lab GitHub organization](https://github.com/Kwaai-AI-Lab).

---

## Contributing

KwaaiNet is a volunteer-driven project and welcomes contributions from node operators, application developers, protocol researchers, and documentation writers.

Until a dedicated `CONTRIBUTING.md` and `docs/contributor-guide.md` are added, you can:

- Explore [open issues and discussions](https://github.com/Kwaai-AI-Lab/KwaaiNet/issues) in this repository.
- Join Kwaai community channels listed on [kwaai.ai](https://www.kwaai.ai) to coordinate on roadmap items and design questions.
- Open PRs that:
  - Improve docs and onboarding.
  - Harden distributed inference and networking.
  - Prototype pieces of the trust, VPK, and intent-casting roadmap.

Future PRs will add:

- `docs/README.md` — docs index and audience map.
- `docs/architecture-overview.md` — node lobes, Layer 8 stack, and subsystems.
- `docs/roadmap.md` — detailed gap-based roadmap derived from the whitepapers and crate releases.
- `CONTRIBUTING.md` and `docs/contributor-guide.md` — practical on-ramp for new contributors.
