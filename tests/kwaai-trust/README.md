# tests/kwaai-trust

Integration and evaluation scripts for the Trust pillar — identity, VC wallet, trust scoring,
reputation, and work receipts (`kwaai-ledger`).

Unit tests live with their crates (`cargo test -p kwaai-trust -p kwaai-ledger`). This directory is
for the things that need a running node, a second peer, or fixture data: credential round-trips
across two identities, receipt quote/claim/counter-sign exchanges, trust-tier transitions.

Referenced as the `tests` folder by `projects/kwaai-trust/kwaai-trust.code-workspace`.
