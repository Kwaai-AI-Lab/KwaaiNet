![Kwaai](assets/kwaai-logo.png)

# KwaaiNet

**A Decentralized AI Network Built on Verifiable Trust**

_Audience: anyone meeting KwaaiNet for the first time — funders, partners, prospective operators_

## The problem

Today, almost every AI service runs inside a handful of cloud providers. When you use an AI tool, your questions, documents, and data pass through infrastructure you cannot inspect, governed by policies you have to take on trust. If the provider changes its terms, faces a government order, or simply goes down, your access disappears. Open model weights do not solve this: the computers that run them are still controlled by someone else.

## What KwaaiNet does

KwaaiNet turns ordinary computers — laptops, workstations, edge servers — into a shared AI network that anyone can join, audit, and trust. There is no central company running it. Nodes cooperate directly, and every node proves what it is and what it can do before being allowed to participate.

## Three capabilities, one trust core

**Run AI models together.** Any machine with a model installed can serve requests for the whole network, so capacity comes from ordinary hardware rather than a data centre. Splitting a single oversized model across several machines — passing results between them like an assembly line — also works, and is the more ambitious goal, but it is still experimental. Your queries travel between nodes encrypted, and a machine holding a middle slice of a model sees neither your prompt nor the final answer.

**Store and search private knowledge — privately.** Storage nodes hold only numeric vectors and return only matches; your documents never leave your machine, and no storage host ever receives the text of a document or a query. The next step, already designed, is for those vectors to be scrambled on your side before upload, so the host cannot read them either.

**Connect nodes without a central server.** Nodes find each other using a distributed directory — the same technology behind BitTorrent — so there is no single address that can be blocked or shut down. All traffic between nodes is encrypted end-to-end.

Every one of these capabilities is governed by a trust layer at the center. Each node has a permanent cryptographic identity and collects signed certificates — from the Kwaai Foundation, from peers, from uptime monitors — that prove its reliability. Other nodes verify those certificates locally, without asking any central authority.

## Who it is for

- **Individuals** who want to run AI tools without surrendering their data to a cloud provider.
- **Community organizations and nonprofits** that need AI infrastructure but cannot afford or trust hyperscale cloud.
- **Regulated industries** (healthcare, legal, government) that must keep data within a defined boundary and under verifiable control.
- **Researchers and developers** building privacy-preserving AI applications who need an open, auditable foundation.

## Status

KwaaiNet is running today. Shipped: whole-model inference contributed by any node with a model installed, encrypted node identity, verifiable credential wallets, an OpenAI-compatible API for existing AI tools, multi-tenant vector storage, and automatic network self-organization.

In progress: splitting a single model across machines works on Linux with NVIDIA GPUs but is not yet dependable enough to rely on, and does not work on Apple Silicon. Client-side encryption of stored vectors, trust-gated routing, and confidential-computing enclaves for inference are designed but not yet shipped.

---

Open source · Apache 2.0 · [github.com/Kwaai-AI-Lab/KwaaiNet](https://github.com/Kwaai-AI-Lab/KwaaiNet) · Kwaai Foundation (nonprofit)
