# Getting started: run a KwaaiNet node

_Audience: node operators, developers who want to try KwaaiNet locally_

This guide walks you through installing the `kwaainet` CLI, starting a node, and making a test chat-completion request against its OpenAI-compatible API.

> This is an early guide. Exact commands and flags may change; always check `kwaainet --help` and the latest release notes.

---

## 1. Prerequisites

You'll need:

- A Linux, macOS, or Windows machine — x86_64 or ARM64 (including Raspberry Pi 4/5 and Apple Silicon).
- An Internet connection (for DHT bootstrap and optional model downloads).
- Basic terminal access and permissions to install binaries.

GPU is not required for a basic test, but some models will run much faster with GPU acceleration.

---

## 2. Install the `kwaainet` CLI

KwaaiNet ships as a single native binary (`kwaainet`) that manages identity, the node daemon, and the OpenAI-compatible API.

### 2.1 Shell installer (macOS / Linux — recommended)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh
```

### 2.2 PowerShell installer (Windows)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1 | iex"
```

### 2.3 Homebrew (macOS / Linux — optional)

```bash
brew install kwaai-ai-lab/tap/kwaainet
```

### 2.4 cargo binstall (downloads prebuilt binary via Rust toolchain)

```bash
cargo binstall kwaainet
```

### 2.5 Build from source (Rust toolchain required)

```bash
cargo install --git https://github.com/Kwaai-AI-Lab/KwaaiNet kwaainet
```

Confirm the install:

```bash
kwaainet --help
```

You should see usage information for commands such as `setup`, `start`, `stop`, `status`, `logs`, and `serve`.

---

## 3. Initialize your node

Before running as a daemon, initialize identity, configuration, and dependencies.

```bash
kwaainet setup
```

This will:

- Generate an Ed25519 keypair at `~/.kwaainet/identity.key`.
- Derive your `PeerId` and `did:peer:` DID from that keypair.
- Create default config files and select a smart default node name (e.g. `alice-linux-aarch64`).
- Download and configure required dependencies (e.g. `p2pd`) if missing.

> If `p2pd` is not found after setup, run `kwaainet setup --get-deps` to download it automatically.

You can inspect your identity:

```bash
kwaainet identity show
```

This prints your node's `PeerId`, `did:peer:` DID, and other identity details anchored in the trust core.

---

## 4. Start the node daemon

Start the node and join the network:

```bash
# Foreground
kwaainet start

# Or run as a background daemon
kwaainet start --daemon
```

On startup, the node will:

- Connect to bootstrap peers and stabilize DHT connections.
- Measure local compute and network bandwidth.
- Discover available models (local or network) and pick a suitable default.
- Register blocks, model info, and trust attestations with bootstrap peers.
- Expose an OpenAI-compatible HTTP API (default port `11435` — check `kwaainet --help` for the current default).

Check status and follow logs:

```bash
kwaainet status
kwaainet logs --follow
```

If everything is working, you should see logs about DHT bootstrap, model discovery, and API readiness.

---

## 5. Make a test API call

KwaaiNet provides an OpenAI-compatible API for models it serves, so most OpenAI clients work by just changing the base URL.

### 5.1 List available models

```bash
curl http://localhost:11435/v1/models
```

You should see a JSON list of available models, including any local model (e.g. `llama3.1:8b`) or network-shared models.

### 5.2 Send a chat-completion request

```bash
MODEL_ID="llama3.1:8b"   # replace with a model from /v1/models

curl http://localhost:11435/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"${MODEL_ID}\",
    \"messages\": [
      {\"role\": \"user\", \"content\": \"Hello, KwaaiNet!\"}
    ]
  }"
```

The node will:

1. Interpret this as an intent ("run MODEL_ID with default trust and latency constraints").
2. Resolve a shard chain of nodes that satisfy trust and capability requirements.
3. Run CandelEngine's block-sharded inference and stream back a completion.

### 5.3 Using the OpenAI Python client

```python
from openai import OpenAI

client = OpenAI(
    base_url="http://localhost:11435/v1",
    api_key="sk-local"  # placeholder, not checked
)

resp = client.chat.completions.create(
    model="llama3.1:8b",  # replace with your model
    messages=[{"role": "user", "content": "Hello, KwaaiNet from Python!"}],
)

print(resp.choices[0].message.content)
```

---

## 6. Confirm your node on the map (optional)

If your configuration allows public discovery, your node can appear on the KwaaiNet map as a visible participant in the Layer 8 fabric.

Once the node is stable:

1. Visit [map.kwaai.ai](https://map.kwaai.ai) and look for your node name.
2. Over time, trust attestations (VCs, uptime, throughput) may show up as badges next to your node.

---

## 7. Next steps

- [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) — See how trust, compute, storage, and network fit together inside the node.
- [`docs/roadmap.md`](roadmap.md) — Understand which parts of the Layer 8 design are implemented vs in progress vs research.
- [`docs/contributor-guide.md`](contributor-guide.md) — Help advance the trust graph, VPK, intent-casting, or other roadmap items.
