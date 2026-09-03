# Changelog

All notable changes to KwaaiNet are documented here.

## [Unreleased]

### Added
- **IPv6 support**, behind a new `ipv6: auto | true | false` config key (default `auto`). `auto` binds `/ip6/::` and runs IPv4-only if the host refuses; `true` makes a refused v6 bind a startup error, so a dual-stack deployment cannot silently come up serving half the network; `false` opens no v6 listener and drops v6 addresses from the dial and announce sets. Also a cargo feature, on by default in `kwaai-p2p` and `kwaainet` — compiled out, the key still parses and reads as `false`.
- `is_routable_v6` / `is_globally_routable_v6` mirror the IPv4 classifiers, including the carve-out: unique-local `fc00::/7` and the documentation and benchmarking prefixes are announceable unless `require_global_ips` is set. IPv4-mapped addresses are classified as the IPv4 address they carry.
- `SelfStatus.ipv6` on the gRPC network snapshot (`"off"`, `"active"` or `"unavailable"`), so a v6 bind the host refused is visible rather than merely absent. `kwaainet p2p info` prints the configured mode.

### Changed
- The node's own local servers — the gRPC control port, the OpenAI-compatible HTTP APIs and the storage health endpoint — now bind IPv6 alongside IPv4 on the same port. A client whose `localhost` resolves to `::1` first, which is the default on Windows and most modern Linux distributions, previously got connection refused from a daemon that was running.
- A v6 `public_ip` is announced as `/ip6/…` instead of being formatted into an `/ip4/` multiaddr that does not parse, which left a correctly configured node announcing nothing.
- `port_in_use` probes both address families, so the "already running" message is not defeated by a squatter holding only the v6 half.

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
