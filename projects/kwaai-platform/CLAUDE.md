# kwaai-platform — Claude Code Instructions

## Project scope

kwaai-platform owns the CLI binary (`kwaainet`), summit-server, release distribution
(cargo-dist), and auto-update infrastructure. It is the integration layer that dispatches all
subcommands — it does not implement domain logic, only wires crates together.

## Crate ownership

| Crate | Path | Description |
|-------|------|-------------|
| `kwaai-cli` | `core/crates/kwaai-cli/` | Binary: all command dispatch |
| `summit-server` | `core/crates/summit-server/` | Summit coordination server |

Primary CLI files in `core/crates/kwaai-cli/src/`:
- `main.rs` — top-level dispatch, `mod` declarations
- `cli.rs` — clap `Command` enum + `Args` structs for all subcommands
- `config.rs` — `KwaaiNetConfig` (YAML at `~/.kwaainet/config.yaml`)
- `display.rs` — `print_box_header`, `print_success`, `print_error`, `print_info`, `print_warning`, `print_separator`
- `service.rs` — auto-start service management (launchd / systemd / Windows service)
- `updater.rs` — self-update: release lookup, download, binary swap

## Build & release

```bash
# Build the CLI binary
cd core && cargo build -p kwaainet --release

# Install + codesign (macOS 26+)
cp core/target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet

# Full workspace test
cd core && cargo test

# Release (cargo-dist) — run in CI, not manually
cargo dist build
```

## Release distribution (cargo-dist 0.31.0)

- Config: `core/Cargo.toml` `[workspace.metadata.dist]`
- Targets: aarch64/x86_64 apple-darwin, x86_64/aarch64 linux-gnu, x86_64-pc-windows-msvc
- Installers: shell + PowerShell + Homebrew tap (Kwaai-AI-Lab/homebrew-tap)
- `HOMEBREW_TAP_TOKEN` secret required on repo
- Shell installer patched for NVIDIA detection in `build-global-artifacts`

**Tag trigger pattern**: `v[0-9]*.[0-9]*.[0-9]*` — supports two-digit patch (v0.1.10+)

**Critical**: multi-line `python3 -c "..."` with zero-indented lines inside YAML block scalar causes
silent tag-event failures. Always write python3 patches as one-liners in release.yml.

## Adding a new subcommand

1. Add `XArgs` + `XAction` to `cli.rs`
2. Create handler module `x_cmd.rs` with `pub async fn run(args: XArgs) -> Result<()>`
3. Add `mod x_cmd;` to `main.rs`
4. Add dispatch arm in `main.rs` `match` block
5. Follow the `identity.rs` pattern for module structure

## Key source files

| File | Description |
|------|-------------|
| `kwaai-cli/src/main.rs` | Top-level dispatch |
| `kwaai-cli/src/cli.rs` | All clap Args/Command structs |
| `kwaai-cli/src/config.rs` | `KwaaiNetConfig`, YAML persistence |
| `kwaai-cli/src/display.rs` | All terminal output helpers |
| `kwaai-cli/src/service.rs` | Daemon / service management |
| `kwaai-cli/src/updater.rs` | Self-update |
| `.github/workflows/release.yml` | cargo-dist release pipeline |

## Config fields pattern

```rust
#[serde(default)]
pub field: Option<T>,
// + skip_serializing_if = "Option::is_none"
```

## Do not

- Do not add business logic to `main.rs` — it dispatches only
- Do not skip `codesign` after build on macOS 26+ — Gatekeeper will kill the binary
- Do not use `--no-verify` to skip git hooks
- Do not force-push to main

## Full project context

`projects/kwaai-platform/` — requirements, design docs, roadmap, TODO
