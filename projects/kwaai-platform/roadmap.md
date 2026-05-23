# kwaai-platform — Roadmap

## Shipped

- Single `kwaainet` binary dispatching all subcommands
- `~/.kwaainet/config.yaml` persistence with backward-compatible serde defaults
- cargo-dist 0.31.0 release pipeline: aarch64/x86_64 macOS, x86_64/aarch64 Linux, x86_64 Windows
- Shell + PowerShell installers; Homebrew tap (`Kwaai-AI-Lab/homebrew-tap`)
- p2pd injection into release archives (repack after `dist build`)
- Shell installer patch: `_bins="kwaainet p2pd"`
- `kwaainet setup --get-deps` — downloads p2pd for current platform
- Auto-start service (`--daemon`) on macOS (launchd), Linux (systemd), Windows
- map-server and summit-server (deployed)
- Tag trigger pattern fixed: `v[0-9]*.[0-9]*.[0-9]*` supports two-digit patch
- Ad-hoc codesign step in release.yml for Gatekeeper compatibility

## In progress

- None active (platform is stable)

## Planned

- **Proper Apple code signing** — Apple Developer Program ($99/yr) + notarization in release.yml; required for manual installs without quarantine workaround
- **`kwaainet update`** — self-update command that fetches and installs latest release
- **RISCV target** — `release-riscv.yml` in progress

## Research / future

- GUI wrapper (Tauri?) for non-developer users
- Mobile distribution (iOS/Android) — needs kwaai-wasm maturity first
- Docker image in release pipeline (currently gated, job preserved in release.yml)
