# kwaai-trust — TODO

## Features
- [ ] Testable Credential (TC) layer — PVP-1 protocol for contextual credential evaluation
- [ ] EigenTrust-style propagation — transitive Sybil-resistant trust across network
- [ ] Sybil resistance hardening aligned with ToIP patterns
- [ ] Cross-network trust import/export (ToIP interoperability schemas)

## Tests
- [ ] Unit tests for trust score time decay (assert tier demotions over synthetic timestamps)
- [ ] VC round-trip test: issue → sign → serialise → deserialise → verify
- [ ] WASM smoke test: identity creation and trust score in headless browser

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
