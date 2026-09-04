# Changelog

## Status of this file

**This file stopped being maintained after v0.3.7 (2026-03-04).** The project has shipped
188 tagged releases since, up to v0.6.8, none of them recorded here. Anyone reading down
from the top would take a March 2026 release as current.

Until per-release notes resume, the accurate sources are:

- **[Releases](https://github.com/Kwaai-AI-Lab/KwaaiNet/releases)** — every tag, with
  install instructions and assets.
- **`git log --oneline v0.3.7..HEAD`** — the actual change history. Commit subjects follow
  `type(scope): summary` and reference their PR.
- **[projects/*/plans/](../projects)** — design and outcome documents for the larger pieces
  of work.

The entries below are retained as the historical record up to v0.3.7. They are accurate for
their releases and have not been edited.

---

## [Unreleased]

### Removed

- The Go `p2pd` daemon. Every node runs the in-process rust-libp2p stack, which has
  been the default since v0.6.0; the child process, its build hook and its release
  plumbing are gone.
- Go is no longer a build dependency of KwaaiNet.
- Release archives and the generated installers no longer ship a `p2pd` binary beside
  `kwaainet`. An upgraded install keeps the old file until `kwaainet uninstall` removes it.

### Changed

- `native_p2p` in `config.yaml` is ignored; setting it to `false` logs a warning and the
  node starts natively anyway. `kwaainet config set native_p2p` is rejected.
- `kwaainet setup --get-deps` is a hidden no-op kept for one release — there is nothing
  left to download.

The historical record up to v0.3.7 follows.

---

## [v0.3.7] - 2026-03-04

### Fixed
- **Gap filling never errors on full coverage** — `shard serve --auto` previously crashed with "all blocks already served" when joining a healthy network. It now joins the least-covered block window as a redundant node instead.
- **Self-exclusion in gap detection** — stale DHT entries for our old block range (TTL up to 360 s) no longer falsely mark blocks as covered after a rebalance restart.
- **Thundering herd prevention** — rebalance timer now starts with a 0–60 s jitter derived from the node's peer ID, spreading simultaneous check-and-move storms across the fleet.
- **Deterministic test peers** — `fake_peer(n)` in unit tests now produces a stable, distinct `PeerId` for each `n` via a seeded Ed25519 keypair.

### Internal
- New pure function `rebalancer::pick_gap_from_chain()` extracts all gap-selection logic so it is unit-testable without a live daemon. 9 rebalancer unit tests total (was 5).

## [v0.3.6] - 2026-03-04

### Fixed
- Throughput cache lookup falls back to single-entry when the model key in the cache doesn't match exactly.
- `discover_chain` now counts legacy nodes that omit the `peer_id` field in their DHT announcement.
- CI: fixed crates.io publish order and workspace dependency versioning.
- CI: fixed already-published skip pattern for crates.io v2 error message.
- CI: fixed `kwaai-cli` → `kwaainet` package name in publish list.

## [v0.3.5] - 2026-03-04

### Fixed
- Throughput is refreshed from the benchmark cache on every DHT re-announce (was stale after restarts).
- `kwaainet update` now always does a live version check, bypassing the 24-hour cache.

## [v0.3.4] - 2026-03-03

### Changed
- `shard serve` is now map-first: the node registers its RPC handler and appears on the map immediately while the model loads in the background. Requests during warmup receive a structured "warming up" error.
- Lazy weight loading — only the safetensors shards needed for the assigned block range are loaded.

### Fixed
- All nodes were picking block 0 due to an incorrect DHT prefix fallback.
- `shard status` now always shows the effective DHT prefix.

## [v0.3.3] - 2026-03-03

### Added
- Update hint shown after each command when a newer version is available.

### Fixed
- CI: `publish-crates` is non-blocking (`continue-on-error`).

## [v0.3.2] - 2026-03-03

### Added
- Dynamic block rebalancing (`shard serve --auto-rebalance`): nodes periodically check DHT coverage and move to fill gaps when their current range has sufficient redundancy.

### Fixed
- FP16/fullFP16 SIMD enabled for `aarch64-linux-gnu` builds.
- Local in-process inference, chat template handling, and KV-cache TTL.
- Vendor OpenSSL for musl targets to fix cross-compilation.
- Use MUSL targets + rustls to fix glibc/libssl install failures.
- Extended regular expressions (PCRE not supported on macOS).
- Go Linux aarch64 daemon build.

## [v0.3.1] - 2026-03-02

### Added
- `kwaainet update` installs the new release automatically (not just checks).

### Changed
- Summit server: polished help text, error messages, and exit codes.

### Fixed
- `shard --help` now renders the long description correctly.
- All clippy warnings resolved across the workspace.

## [v0.3.0] - 2026-03-01

### Added
- Distributed block sharding (`kwaainet shard serve/run/chain`) — Petals-style transformer block serving over libp2p.
- OpenAI-compatible shard API (`kwaainet shard api`) — streaming SSE + non-streaming HTTP endpoints.
- Sampling parameters: `--temperature`, `--top-p`, `--top-k`.
- `kwaainet shard download` — downloads HuggingFace SafeTensors snapshots directly (no `huggingface-cli` required).
- cargo-dist release pipeline: shell/PowerShell/Homebrew installers for macOS, Linux, and Windows.
- p2pd bundled inside platform archives and patched into installer scripts.
- `kwaainet setup --get-deps` as a fallback to fetch p2pd post-install.
