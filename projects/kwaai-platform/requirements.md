# kwaai-platform — Requirements

## Purpose

Deliver a single, self-installing `kwaainet` binary that integrates all KwaaiNet domains and
runs reliably on every supported platform — including auto-update, bundled dependencies, and
a consistent user experience.

## Functional requirements

1. **FR-P1** Provide a single `kwaainet` binary that dispatches all subcommands: identity, trust, p2p, shard, rag, vpk, setup, node.
2. **FR-P2** Persist configuration at `~/.kwaainet/config.yaml`; config fields use `#[serde(default)]` with `skip_serializing_if` so missing fields don't invalidate old configs.
3. **FR-P3** `kwaainet setup --get-deps` downloads and installs `p2pd` for the current platform without requiring the user to visit GitHub.
4. **FR-P4** Shell and PowerShell installers (cargo-dist) must install both `kwaainet` and `p2pd` in a single invocation.
5. **FR-P5** Homebrew tap formula (`Kwaai-AI-Lab/homebrew-tap`) must stay in sync with releases.
6. **FR-P6** Print a clear warning when `p2pd` is not found at startup; direct user to `kwaainet setup --get-deps`.
7. **FR-P7** map-server and summit-server must start, serve their endpoints, and respond to health checks.
8. **FR-P8** Auto-start service (`kwaainet start --daemon`) must work on macOS (launchd), Linux (systemd), and Windows (service).
9. **FR-P9** All terminal output uses the display module helpers; no raw `println!` in command handlers.

## Non-functional requirements

- **NFR-P1** Binary size: `kwaainet` < 30MB stripped on all targets.
- **NFR-P2** `kwaainet --help` must respond in < 100ms.
- **NFR-P3** Release pipeline (cargo-dist + p2pd injection) must produce verified SHA256 checksums.
- **NFR-P4** Tag pattern `v[0-9]*.[0-9]*.[0-9]*` must match two-digit patch versions (v0.1.10+).
- **NFR-P5** Targets: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows.

## Out of scope

- Business logic — all domain logic lives in the domain crates.
- GUI / web frontend.
- Mobile (iOS, Android).

## Dependencies

- All other KwaaiNet projects — kwaai-platform is the integration layer.
