//! Per-peer circuit breaker for P2P inference proxies.
//!
//! Trips on connection/timeout failures (not Ollama-level HTTP errors).
//! One `CircuitBreaker` is created per inference session and shared across
//! all proxy accept loops via `Arc`.
//!
//! States:
//!   Closed  → normal; all requests pass through
//!   Open    → tripped; requests fail fast with HTTP 503 (no P2P round-trip)
//!   HalfOpen→ cooldown elapsed; one probe request allowed through
//!
//! Thresholds: 3 failures → Open; 30 s cooldown before HalfOpen.

use libp2p::PeerId;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tracing::{info, warn};

const FAILURE_THRESHOLD: u32 = 3;
const OPEN_DURATION: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, PartialEq, Debug)]
enum State {
    Closed,
    Open,
    HalfOpen,
}

struct PeerEntry {
    state: State,
    failures: u32,
    opened_at: Option<Instant>,
}

impl Default for PeerEntry {
    fn default() -> Self {
        Self {
            state: State::Closed,
            failures: 0,
            opened_at: None,
        }
    }
}

pub struct CircuitBreaker {
    peers: Mutex<HashMap<PeerId, PeerEntry>>,
    /// Cooldown after tripping before transitioning to HalfOpen.
    /// Fixed at `OPEN_DURATION` in production; overrideable in tests.
    open_duration: Duration,
}

impl CircuitBreaker {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            peers: Mutex::new(HashMap::new()),
            open_duration: OPEN_DURATION,
        })
    }

    /// Returns `true` when a request should be forwarded to the peer.
    /// Returns `false` when the circuit is open and the cooldown has not elapsed.
    /// Transitions Open → HalfOpen automatically when the cooldown expires.
    pub fn allow(&self, peer: &PeerId) -> bool {
        let mut map = self.peers.lock().unwrap();
        let entry = map.entry(*peer).or_default();
        match entry.state {
            State::Closed => true,
            State::Open => {
                let elapsed = entry
                    .opened_at
                    .map(|t| t.elapsed() >= self.open_duration)
                    .unwrap_or(false);
                if elapsed {
                    info!(
                        "circuit breaker: {} → HalfOpen (cooldown elapsed)",
                        peer.to_base58()
                    );
                    entry.state = State::HalfOpen;
                    true
                } else {
                    false
                }
            }
            // HalfOpen: let exactly one probe through (caller decides outcome).
            State::HalfOpen => true,
        }
    }

    /// Record a successful P2P round-trip. Closes the circuit and resets counters.
    pub fn record_success(&self, peer: &PeerId) {
        let mut map = self.peers.lock().unwrap();
        let entry = map.entry(*peer).or_default();
        if entry.state != State::Closed {
            info!(
                "circuit breaker: {} → Closed (peer recovered)",
                peer.to_base58()
            );
        }
        *entry = PeerEntry::default();
    }

    /// Record a connection/timeout failure. Opens the circuit after
    /// `FAILURE_THRESHOLD` failures, or immediately if currently HalfOpen.
    pub fn record_failure(&self, peer: &PeerId) {
        let mut map = self.peers.lock().unwrap();
        let entry = map.entry(*peer).or_default();
        entry.failures += 1;
        let should_open = entry.failures >= FAILURE_THRESHOLD || entry.state == State::HalfOpen;
        if should_open && entry.state != State::Open {
            warn!(
                "circuit breaker: {} → Open ({} consecutive connection failure(s)); \
                 will retry in {}s",
                peer.to_base58(),
                entry.failures,
                self.open_duration.as_secs(),
            );
            entry.state = State::Open;
            entry.opened_at = Some(Instant::now());
        }
    }
}

#[cfg(test)]
impl CircuitBreaker {
    fn new_with_cooldown(cooldown: Duration) -> Arc<Self> {
        Arc::new(Self {
            peers: Mutex::new(HashMap::new()),
            open_duration: cooldown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    fn make_peer() -> PeerId {
        Keypair::generate_ed25519().public().to_peer_id()
    }

    #[test]
    fn closed_by_default() {
        let cb = CircuitBreaker::new();
        let p = make_peer();
        assert!(cb.allow(&p));
    }

    #[test]
    fn trips_after_threshold() {
        let cb = CircuitBreaker::new();
        let p = make_peer();
        for _ in 0..FAILURE_THRESHOLD {
            assert!(cb.allow(&p));
            cb.record_failure(&p);
        }
        assert!(!cb.allow(&p), "must be Open after threshold failures");
    }

    #[test]
    fn fast_fail_beyond_threshold() {
        let cb = CircuitBreaker::new();
        let p = make_peer();
        for _ in 0..FAILURE_THRESHOLD {
            cb.record_failure(&p);
        }
        for _ in 0..10 {
            assert!(!cb.allow(&p));
        }
    }

    #[test]
    fn success_before_threshold_resets_counter() {
        let cb = CircuitBreaker::new();
        let p = make_peer();
        cb.record_failure(&p);
        cb.record_failure(&p);
        cb.record_success(&p);
        // Need FAILURE_THRESHOLD more failures to trip again.
        for _ in 0..FAILURE_THRESHOLD - 1 {
            assert!(cb.allow(&p));
            cb.record_failure(&p);
        }
        assert!(cb.allow(&p), "only 2 failures after reset — still Closed");
    }

    #[test]
    fn halfopen_success_closes() {
        let cb = CircuitBreaker::new_with_cooldown(Duration::from_millis(1));
        let p = make_peer();
        for _ in 0..FAILURE_THRESHOLD {
            cb.record_failure(&p);
        }
        assert!(!cb.allow(&p), "Open immediately after tripping");

        std::thread::sleep(Duration::from_millis(5));

        assert!(cb.allow(&p), "HalfOpen after cooldown");
        cb.record_success(&p);
        assert!(cb.allow(&p), "Closed after HalfOpen success");
    }

    #[test]
    fn halfopen_failure_reopens() {
        let cb = CircuitBreaker::new_with_cooldown(Duration::from_millis(1));
        let p = make_peer();
        for _ in 0..FAILURE_THRESHOLD {
            cb.record_failure(&p);
        }
        std::thread::sleep(Duration::from_millis(5));

        assert!(cb.allow(&p), "HalfOpen probe allowed");
        cb.record_failure(&p);
        assert!(!cb.allow(&p), "re-opened after HalfOpen failure");
    }

    #[test]
    fn peers_are_independent() {
        let cb = CircuitBreaker::new();
        let p1 = make_peer();
        let p2 = make_peer();
        for _ in 0..FAILURE_THRESHOLD {
            cb.record_failure(&p1);
        }
        assert!(!cb.allow(&p1), "p1 tripped");
        assert!(cb.allow(&p2), "p2 unaffected by p1 failures");
    }
}
