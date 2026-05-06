//! Local peer reputation store — subjective, evidence-based trust scores.
//!
//! Each node maintains its own view of peer reliability: latency, availability,
//! and observed vs. claimed throughput.  No global scores, no central authority.
//!
//! Persisted to `~/.kwaainet/reputation.json`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use crate::config::kwaainet_dir;

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

pub fn reputation_file() -> PathBuf {
    kwaainet_dir().join("reputation.json")
}

// ---------------------------------------------------------------------------
// Observation (single measurement sample)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerObservation {
    pub timestamp_secs: u64,
    pub latency_ms: f64,
    pub success: bool,
    /// Tokens/sec measured by the caller for this hop (None when not applicable).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_tps: Option<f64>,
    /// Tokens/sec the peer claimed in its DHT announcement (None when unknown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_tps: Option<f64>,
}

// ---------------------------------------------------------------------------
// PeerRecord (ring buffer of observations per peer)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRecord {
    pub peer_id_b58: String,
    pub public_name: String,
    pub observations: VecDeque<PeerObservation>,
    pub first_seen_secs: u64,
    pub last_seen_secs: u64,
}

impl PeerRecord {
    fn new(peer_id_b58: &str, public_name: &str, now: u64) -> Self {
        Self {
            peer_id_b58: peer_id_b58.to_string(),
            public_name: public_name.to_string(),
            observations: VecDeque::new(),
            first_seen_secs: now,
            last_seen_secs: now,
        }
    }
}

// ---------------------------------------------------------------------------
// TrustTier
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustTier {
    Unknown,
    Known,
    Verified,
    Trusted,
}

impl TrustTier {
    pub fn as_str(self) -> &'static str {
        match self {
            TrustTier::Unknown => "UNKNOWN",
            TrustTier::Known => "KNOWN",
            TrustTier::Verified => "VERIFIED",
            TrustTier::Trusted => "TRUSTED",
        }
    }
}

// ---------------------------------------------------------------------------
// TrustScore
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TrustScore {
    pub score: f64,
    pub tier: TrustTier,
    pub sample_count: usize,
    pub avg_latency_ms: f64,
    pub availability: f64,
    pub throughput_ratio: Option<f64>,
}

// ---------------------------------------------------------------------------
// ReputationStore
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationStore {
    peers: HashMap<String, PeerRecord>,
    #[serde(default = "default_max_obs")]
    max_observations: usize,
}

fn default_max_obs() -> usize {
    100
}

impl Default for ReputationStore {
    fn default() -> Self {
        Self {
            peers: HashMap::new(),
            max_observations: 100,
        }
    }
}

impl ReputationStore {
    #[cfg(test)]
    pub fn with_max_observations(max: usize) -> Self {
        Self {
            peers: HashMap::new(),
            max_observations: max,
        }
    }

    // ── Persistence ──────────────────────────────────────────────────────────

    pub fn load() -> Self {
        let path = reputation_file();
        if !path.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(&path)
            .context("read reputation file")
            .and_then(|s| serde_json::from_str(&s).context("parse reputation json"))
        {
            Ok(store) => store,
            Err(e) => {
                tracing::warn!("Could not load reputation store: {e} — starting fresh");
                Self::default()
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = reputation_file();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json).context("write reputation file")
    }

    // ── Mutation ─────────────────────────────────────────────────────────────

    pub fn record(&mut self, peer_id_b58: &str, public_name: &str, obs: PeerObservation) {
        self.record_no_save(peer_id_b58, public_name, obs);
        let _ = self.save();
    }

    fn record_no_save(&mut self, peer_id_b58: &str, public_name: &str, obs: PeerObservation) {
        let now = obs.timestamp_secs;
        let record = self
            .peers
            .entry(peer_id_b58.to_string())
            .or_insert_with(|| PeerRecord::new(peer_id_b58, public_name, now));

        // Update display name if it changed.
        if !public_name.is_empty() {
            record.public_name = public_name.to_string();
        }
        record.last_seen_secs = now;

        // Cap ring buffer at max_observations.
        if record.observations.len() >= self.max_observations {
            record.observations.pop_front();
        }
        record.observations.push_back(obs);
    }

    pub fn reset(&mut self, peer_id_b58: &str) -> bool {
        let removed = self.peers.remove(peer_id_b58).is_some();
        if removed {
            let _ = self.save();
        }
        removed
    }

    // ── Scoring ──────────────────────────────────────────────────────────────

    /// Compute a local trust score for a peer.
    ///
    /// Scoring formula (from reputation.md):
    /// ```text
    /// s_availability  = successes / total
    /// s_throughput    = (avg_observed_tps / claimed_tps).clamp(0, 1)   (0 when no tps data)
    /// s_latency       = exp(−avg_latency_ms / 500)
    /// score_metrics   = 0.5 × s_avail + 0.3 × s_tput + 0.2 × s_lat
    /// ```
    pub fn score(&self, peer_id_b58: &str) -> TrustScore {
        let Some(record) = self.peers.get(peer_id_b58) else {
            return TrustScore {
                score: 0.0,
                tier: TrustTier::Unknown,
                sample_count: 0,
                avg_latency_ms: 0.0,
                availability: 0.0,
                throughput_ratio: None,
            };
        };

        let n = record.observations.len();
        if n == 0 {
            return TrustScore {
                score: 0.0,
                tier: TrustTier::Unknown,
                sample_count: 0,
                avg_latency_ms: 0.0,
                availability: 0.0,
                throughput_ratio: None,
            };
        }

        let successes = record.observations.iter().filter(|o| o.success).count() as f64;
        let s_availability = successes / n as f64;

        let total_latency: f64 = record.observations.iter().map(|o| o.latency_ms).sum();
        let avg_latency_ms = total_latency / n as f64;
        let s_latency = (-avg_latency_ms / 500.0_f64).exp();

        // Throughput ratio: use observations that have both observed and claimed tps.
        let tps_pairs: Vec<(f64, f64)> = record
            .observations
            .iter()
            .filter_map(|o| o.observed_tps.zip(o.claimed_tps))
            .collect();

        let throughput_ratio = if tps_pairs.is_empty() {
            None
        } else {
            let avg_observed: f64 = tps_pairs.iter().map(|(o, _)| o).sum::<f64>() / tps_pairs.len() as f64;
            let avg_claimed: f64 = tps_pairs.iter().map(|(_, c)| c).sum::<f64>() / tps_pairs.len() as f64;
            Some((avg_observed / avg_claimed).clamp(0.0, 1.0))
        };
        let s_throughput = throughput_ratio.unwrap_or(0.0);

        let score = 0.5 * s_availability + 0.3 * s_throughput + 0.2 * s_latency;

        let tier = if n < 5 {
            TrustTier::Unknown
        } else if score < 0.40 {
            TrustTier::Known
        } else if score < 0.70 {
            TrustTier::Verified
        } else {
            TrustTier::Trusted
        };

        TrustScore {
            score,
            tier,
            sample_count: n,
            avg_latency_ms,
            availability: s_availability,
            throughput_ratio,
        }
    }

    // ── Queries ──────────────────────────────────────────────────────────────

    /// All known peers with their computed trust score, sorted by score descending.
    pub fn all_scored(&self) -> Vec<(String, TrustScore)> {
        let mut scored: Vec<(String, TrustScore)> = self
            .peers
            .keys()
            .map(|id| (id.clone(), self.score(id)))
            .collect();
        scored.sort_by(|a, b| b.1.score.partial_cmp(&a.1.score).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }

    pub fn peer_record(&self, peer_id_b58: &str) -> Option<&PeerRecord> {
        self.peers.get(peer_id_b58)
    }
}

// ---------------------------------------------------------------------------
// Timestamp helper
// ---------------------------------------------------------------------------

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn obs(latency_ms: f64, success: bool) -> PeerObservation {
        PeerObservation {
            timestamp_secs: 1_000_000,
            latency_ms,
            success,
            observed_tps: None,
            claimed_tps: None,
        }
    }

    #[test]
    fn unknown_with_no_data() {
        let store = ReputationStore::default();
        let s = store.score("nonexistent");
        assert_eq!(s.tier, TrustTier::Unknown);
        assert_eq!(s.sample_count, 0);
    }

    #[test]
    fn unknown_with_few_samples() {
        let mut store = ReputationStore::default();
        for _ in 0..4 {
            store.record_no_save("peer1", "Alice", obs(10.0, true));
        }
        let s = store.score("peer1");
        assert_eq!(s.tier, TrustTier::Unknown);
        assert_eq!(s.sample_count, 4);
    }

    #[test]
    fn trusted_with_good_data() {
        let mut store = ReputationStore::default();
        for _ in 0..10 {
            store.record_no_save("peer2", "Bob", obs(50.0, true));
        }
        let s = store.score("peer2");
        // availability=1.0, latency=exp(-0.1)≈0.905, tput=0 → score≈0.5+0+0.181≈0.681
        assert!(s.score >= 0.60, "expected ≥ 0.60, got {}", s.score);
        assert!(s.tier >= TrustTier::Verified);
    }

    #[test]
    fn known_with_high_latency_and_failures() {
        let mut store = ReputationStore::default();
        for i in 0..10 {
            store.record_no_save("peer3", "Eve", obs(2000.0, i % 2 == 0));
        }
        let s = store.score("peer3");
        // availability≈0.5, latency=exp(-4)≈0.018, tput=0 → score≈0.25+0+0.0036≈0.25
        assert!(s.score < 0.40, "expected < 0.40, got {}", s.score);
        assert_eq!(s.tier, TrustTier::Known);
    }

    #[test]
    fn throughput_ratio_recorded() {
        let mut store = ReputationStore::default();
        for _ in 0..10 {
            store.record_no_save(
                "peer4",
                "Test",
                PeerObservation {
                    timestamp_secs: 1_000_000,
                    latency_ms: 50.0,
                    success: true,
                    observed_tps: Some(6.0),
                    claimed_tps: Some(10.0),
                },
            );
        }
        let s = store.score("peer4");
        assert!((s.throughput_ratio.unwrap() - 0.6).abs() < 0.01);
    }

    #[test]
    fn ring_buffer_capped() {
        let mut store = ReputationStore::with_max_observations(5);
        for i in 0..10u64 {
            store.record_no_save(
                "peer5",
                "Capped",
                PeerObservation {
                    timestamp_secs: i,
                    latency_ms: 10.0,
                    success: true,
                    observed_tps: None,
                    claimed_tps: None,
                },
            );
        }
        let record = store.peer_record("peer5").unwrap();
        assert_eq!(record.observations.len(), 5);
        // Should contain the 5 most recent (timestamps 5..9)
        assert_eq!(record.observations.front().unwrap().timestamp_secs, 5);
    }

    #[test]
    fn reset_clears_peer() {
        let mut store = ReputationStore::default();
        store.record_no_save("peer6", "Clear", obs(10.0, true));
        assert!(store.peer_record("peer6").is_some());
        store.reset("peer6");
        assert!(store.peer_record("peer6").is_none());
    }
}
