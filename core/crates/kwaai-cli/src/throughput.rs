//! Throughput cache and the Petals-style effective-throughput formula.
//!
//! ## What we measure
//!
//! ```text
//! effective_tps = min(compute_tps, network_rps × relay_penalty)
//!
//! network_rps   = min(download_bps, upload_bps) / (hidden_dim × 16)
//!                 ← hidden_dim F16 elements per token, 16 bits each
//!
//! relay_penalty = 0.2  if routing through a relay peer
//!                 1.0  if direct connection
//! ```
//!
//! ## Cache file
//!
//! `~/.kwaainet/throughput_cache.json` — keyed by model name.
//! Format: `{ "model-name": { "compute_tps": <f64>, "hidden_size": <usize> } }`
//!
//! Older entries written as plain `f64` by the previous version are silently
//! ignored (the deserializer returns `None` for those keys).

use crate::config::kwaainet_dir;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// ── Cache entry ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputEntry {
    /// Decode throughput in tokens/second measured by `kwaainet generate`.
    pub compute_tps: f64,
    /// Hidden dimension of the model architecture (number of F16 elements per
    /// token passed between layers). Used in the network-bandwidth formula.
    pub hidden_size: usize,
}

// ── File path ─────────────────────────────────────────────────────────────────

pub fn cache_file() -> PathBuf {
    kwaainet_dir().join("throughput_cache.json")
}

// ── Persist / load ────────────────────────────────────────────────────────────

/// Persist a decode throughput measurement for `model`.
pub fn save(model: &str, compute_tps: f64, hidden_size: usize) -> Result<()> {
    let path = cache_file();
    std::fs::create_dir_all(path.parent().expect("cache_file has a parent"))?;

    let mut cache: HashMap<String, ThroughputEntry> = load_cache();
    cache.insert(model.to_string(), ThroughputEntry { compute_tps, hidden_size });
    std::fs::write(&path, serde_json::to_string_pretty(&cache)?)?;
    Ok(())
}

/// Load the cached throughput entry for `model`.
///
/// Returns `None` if the file doesn't exist, the model has no entry, or the
/// entry was written by an older version of kwaainet (plain `f64` format).
pub fn load(model: &str) -> Option<ThroughputEntry> {
    load_cache().remove(model)
}

fn load_cache() -> HashMap<String, ThroughputEntry> {
    let text = match std::fs::read_to_string(cache_file()) {
        Ok(t) => t,
        Err(_) => return HashMap::new(),
    };
    // If the file contains the old format (`"model": <f64>`) the deserializer
    // will return an error and we start fresh, which is the right behaviour.
    serde_json::from_str(&text).unwrap_or_default()
}

// ── Network measurement ───────────────────────────────────────────────────────

/// Penalty applied to `network_rps` when the peer connection routes through a
/// relay (same constant as Petals).
pub const RELAY_PENALTY: f64 = 0.2;

/// Estimate download bandwidth by fetching a small test payload from
/// Cloudflare's speed-test endpoint.  Returns bits-per-second, or `0.0` if
/// the measurement fails (no connectivity, firewall, etc.).
///
/// We use download as a proxy for both directions. Upload is typically the
/// bottleneck on residential connections; the `min()` in the formula means we
/// are intentionally conservative.
pub async fn measure_download_bps() -> f64 {
    // Cloudflare __down: returns exactly N random bytes with no caching.
    const URL: &str = "https://speed.cloudflare.com/__down?bytes=1048576";
    const TEST_BYTES: usize = 1_048_576; // 1 MiB

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(_) => return 0.0,
    };

    let start = std::time::Instant::now();
    let resp = match client.get(URL).send().await {
        Ok(r) => r,
        Err(_) => return 0.0,
    };
    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(_) => return 0.0,
    };

    let secs = start.elapsed().as_secs_f64();
    if secs <= 0.0 || bytes.len() < TEST_BYTES / 2 {
        return 0.0; // incomplete response
    }

    (bytes.len() as f64 * 8.0) / secs // bits per second
}

// ── Petals formula ────────────────────────────────────────────────────────────

/// Compute effective throughput using the Petals formula.
///
/// ```text
/// network_rps   = download_bps / (hidden_size × 16)
/// effective_tps = min(compute_tps, network_rps × relay_penalty)
/// ```
///
/// If `download_bps == 0` (measurement failed), network is not the bottleneck
/// and we return `compute_tps` unchanged — a conservative but safe fallback.
pub fn effective_tps(entry: &ThroughputEntry, download_bps: f64, using_relay: bool) -> f64 {
    let penalty = if using_relay { RELAY_PENALTY } else { 1.0 };

    if download_bps <= 0.0 || entry.hidden_size == 0 {
        return entry.compute_tps;
    }

    // Each token exchange transfers `hidden_size` F16 elements = hidden_size × 16 bits.
    let network_rps = download_bps / (entry.hidden_size as f64 * 16.0);
    entry.compute_tps.min(network_rps * penalty)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_tps_compute_bound() {
        // 100 Mbps network, 4096 hidden: network_rps = 1526, ×0.2 = 305
        // compute_tps = 20  → bottleneck is compute
        let entry = ThroughputEntry { compute_tps: 20.0, hidden_size: 4096 };
        let tps = effective_tps(&entry, 100_000_000.0, true);
        assert!((tps - 20.0).abs() < 0.01, "expected 20.0, got {tps}");
    }

    #[test]
    fn test_effective_tps_network_bound() {
        // Very slow network: 1 Mbps, 4096 hidden: network_rps ≈ 15.3, ×0.2 ≈ 3.1
        // compute_tps = 100 → bottleneck is network
        let entry = ThroughputEntry { compute_tps: 100.0, hidden_size: 4096 };
        let tps = effective_tps(&entry, 1_000_000.0, true);
        // network_rps = 1_000_000 / (4096 * 16) = 15.26, × 0.2 = 3.05
        assert!(tps < 5.0, "expected network-bound (<5), got {tps}");
        assert!(tps > 2.0, "expected >2, got {tps}");
    }

    #[test]
    fn test_effective_tps_no_relay() {
        // Same as above but no relay → penalty = 1.0 → network_rps ≈ 15.3
        let entry = ThroughputEntry { compute_tps: 100.0, hidden_size: 4096 };
        let tps = effective_tps(&entry, 1_000_000.0, false);
        assert!(tps > 14.0 && tps < 16.0, "expected ~15.3, got {tps}");
    }

    #[test]
    fn test_effective_tps_no_network_data() {
        // download_bps == 0 → fall back to compute_tps
        let entry = ThroughputEntry { compute_tps: 7.5, hidden_size: 4096 };
        assert_eq!(effective_tps(&entry, 0.0, true), 7.5);
    }
}
