# kwaai-platform — TODO

## Features
- [ ] Apple code signing — Developer Program + notarization in release.yml (long-term fix for Gatekeeper)
- [ ] `kwaainet update` — self-update command (fetch and install latest release)
- [ ] RISCV target — complete `release-riscv.yml`

## Tests
- [ ] Installer smoke test: curl install → `kwaainet --version` + `p2pd --version` both pass
- [ ] `kwaainet setup --get-deps` smoke test on clean macOS, Linux, Windows

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
