# Contributor guide

KwaaiNet is an open, decentralized AI node architecture for Layer 8, built by a nonprofit lab and a growing community. This guide explains how to contribute effectively, whether you have an hour, a day, or a week to invest.

---

## 1. Ways you can contribute

There are many ways to help move KwaaiNet toward the full Layer 8 vision.

- **Run nodes and share feedback** — Operate a node (or several), report bugs, and provide performance and UX feedback.
- **Improve docs and onboarding** — Clarify concepts, fix gaps, and add examples in the README and `docs/`.
- **Contribute to the Layer 8 stack** — Work on trust graph mechanics, sharded inference, VPK distribution, or intent-casting.
- **Integrate apps and tools** — Connect UIs, agents, and existing OpenAI-compatible tooling to KwaaiNet.
- **Advance governance and standards** — Help align KwaaiNet with ToIP, Mozilla-style trustworthy AI practices, SingularityNET ecosystems, and IEEE P7012.

---

## 2. If you have 1 hour, 1 day, or 1 week

### 2.1 1 hour

- Read the root `README.md`, `docs/ARCHITECTURE.md`, and `docs/roadmap.md` to understand the Layer 8 destination vs current implementation.
- File issues for unclear docs, confusing error messages, or rough edges in the CLI experience.
- Suggest small doc improvements (typo fixes, clarifications, missing links) as quick PRs.

### 2.2 1 day

- Run a node and document any missing steps or platform-specific gotchas.
- Add a focused doc page or section (e.g., "common errors when starting a node," "how trust tiers are computed" with examples).
- Implement a small feature or bugfix that is clearly scoped in an existing issue, especially those tagged "good first issue".

### 2.3 1 week or more

- Choose one roadmap area (trust, compute, storage, network, governance), discuss design with maintainers, and work on a multi-PR feature or prototype.
- Build an integration (UI, agent framework, RAG pipeline) and contribute configuration + docs so others can replicate it.
- Co-develop standards-aligned proposals (e.g., ToIP trust graph profiles, P7012-aligned privacy terms for Layer 8 agents).

---

## 3. Where to start: map to the roadmap

Use [`docs/roadmap.md`](roadmap.md) to pick work that matters.

- **Trust** — Testable Credentials, PVP-1 protocol, EigenTrust propagation, Sybil resistance.
- **Compute** — Decentralized training, KV-cache scrambling, safer tool-calling.
- **Storage** — Cross-node VPK shard placement, DHT-backed knowledge resolution, HE performance.
- **Network** — Intent-casting schemas and bus, neutral routing and marketplace governance.
- **Governance** — Documenting governance structures and connecting trust signals to real-world accountability.

When in doubt, open an issue or join community channels to confirm that your idea aligns with the current phase and priorities.

---

## 4. Development workflow

1. **Discuss design for larger changes**
   For substantial features, open an issue or use existing discussion threads to align on approach before coding.

2. **Fork and branch**
   Fork the repository, create a feature branch, and keep your changes focused and small where possible.

3. **Code and test**
   Follow the existing Rust style and patterns in the `kwaainet` crate and related modules. Add or update tests where appropriate; make sure the test suite passes before opening a PR.

4. **Write or update docs**
   If you change behavior, update `README.md`, `docs/`, and `docs/roadmap.md` to keep architecture and roadmap accurate.

5. **Open a pull request**
   Use a descriptive title and a short description explaining what changed and which part of the roadmap it moves forward. Link relevant issues and mention maintainers if review from a specific perspective (trust, compute, storage, network) is needed.

---

## 5. Design principles to keep in mind

KwaaiNet is guided by a few core principles drawn from the Layer 8 whitepaper and node architecture:

- **Person-anchored, not anonymous** — Every node and agent is ultimately accountable to a human or organization, with cryptographic identity and verifiable credentials.
- **Trust-first, then performance** — Improvements should respect and extend the trust pipeline (identity → VCs → local score → TCs → EigenTrust) rather than bypass it.
- **Owners, not renters** — Design for people and organizations who want to own and co-govern their AI infrastructure, not just consume APIs.
- **Interoperate with open standards** — Align with ecosystems such as Trust Over IP, Mozilla's trustworthy AI work, SingularityNET, and IEEE P7012 where possible.

Changes that move KwaaiNet closer to the full Layer 8 architecture while respecting these principles are especially welcome.

---

## 6. Community and collaboration

KwaaiNet is part of a broader movement to build open, decentralized AI infrastructure.

You can:

- Follow project updates and join discussions via channels listed on [kwaai.ai](https://www.kwaai.ai).
- Engage with related initiatives through the Linux Foundation Trust Over IP working groups, Mozilla's trustworthy AI communities, SingularityNET forums, and IEEE standards groups.

If you're unsure where to plug in, open an issue titled "How can I help?" with a brief note about your background and interests, and maintainers can help you find a good starting point.
