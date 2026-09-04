# Archived documentation

_Audience: anyone tracing a design decision back to when it was made_

These documents described KwaaiNet at a point in the past. They are **not maintained** and
several of them assert things that are no longer true — WebRTC transport, `p2pd` as the live
networking path, `redb` as the storage engine, block sharding as production-ready. They are
kept because they record why the architecture went the way it did, not because they describe
the system you are running.

For current documentation, start at [docs/README.md](../README.md).

| Document | Describes the system as of | Lines |
|---|---|---|
| [CHALLENGE_ARCHITECTURES.md](CHALLENGE_ARCHITECTURES.md) | 2026-02-19 | 1257 |
| [HIVEMIND_RUST_ARCHITECTURE.md](HIVEMIND_RUST_ARCHITECTURE.md) | 2026-02-19 | 1158 |
| [DEPLOYMENT_ARCHITECTURE.md](DEPLOYMENT_ARCHITECTURE.md) | 2025-11-20 | 1063 |
| [CANDLE_ENGINE.md](CANDLE_ENGINE.md) | 2026-02-19 | 955 |
| [VERIDA_INTEGRATION.md](VERIDA_INTEGRATION.md) | 2026-02-19 | 862 |
| [DATA_FLOWS.md](DATA_FLOWS.md) | 2026-02-19 | 850 |
| [VERIDA_ARCHITECTURE.md](VERIDA_ARCHITECTURE.md) | 2026-02-19 | 843 |
| [PETALS_PROTOCOL_COMPLETE.md](PETALS_PROTOCOL_COMPLETE.md) | 2026-03-17 | 782 |
| [FEATURE_GAP_ANALYSIS.md](FEATURE_GAP_ANALYSIS.md) | 2026-03-17 | 712 |

Nothing here should be cited as the current design. If one of these is the only place a
decision is written down, that is a gap worth closing in the live docs rather than a reason
to treat the archive as current.
