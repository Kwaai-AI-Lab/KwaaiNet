//! kwaai-network-tests — connectivity, uptime, and robustness test suite
//!
//! # Three test tiers
//!
//! | Tier        | Gate env var               | What runs                          |
//! |-------------|----------------------------|------------------------------------|
//! | unit        | (always)                   | codec, value, storage, protocol    |
//! | integration | `KWAAI_INTEGRATION_TESTS=1`| daemon spawn, DHT, relay topology  |
//! | network     | `KWAAI_NETWORK_TESTS=1`    | real bootstrap, peer count metrics |
//!
//! # Running
//!
//! ```bash
//! # Unit tests only
//! cd core && cargo test -p kwaai-network-tests
//!
//! # Unit + integration
//! cd core && KWAAI_INTEGRATION_TESTS=1 cargo test -p kwaai-network-tests
//!
//! # All tiers
//! cd core && KWAAI_INTEGRATION_TESTS=1 KWAAI_NETWORK_TESTS=1 cargo test -p kwaai-network-tests
//!
//! # View historical metrics
//! cd core && cargo run -p kwaai-network-tests --bin metrics-report
//! ```

pub mod harness;
pub mod metrics;

/// Returns true when integration (daemon-spawn) tests should run.
pub fn integration_enabled() -> bool {
    std::env::var("KWAAI_INTEGRATION_TESTS").is_ok()
}

/// Returns true when network (live bootstrap) tests should run.
pub fn network_enabled() -> bool {
    std::env::var("KWAAI_NETWORK_TESTS").is_ok()
}

/// Convenience: skip a test at runtime if a tier is not enabled.
///
/// Call at the start of integration/network tests:
/// ```ignore
/// require_integration!();
/// require_network!();
/// ```
#[macro_export]
macro_rules! require_integration {
    () => {
        if !$crate::integration_enabled() {
            eprintln!(
                "Skipping integration test — set KWAAI_INTEGRATION_TESTS=1 to enable"
            );
            return;
        }
    };
}

#[macro_export]
macro_rules! require_network {
    () => {
        if !$crate::network_enabled() {
            eprintln!(
                "Skipping network test — set KWAAI_NETWORK_TESTS=1 to enable"
            );
            return;
        }
    };
}
