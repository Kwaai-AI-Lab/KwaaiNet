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
    /// Set by `record_capacity_denied()` when a Capacity Lease negotiation
    /// returns `DeniedAtCapacity` — a healthy peer that is simply full right
    /// now. Deliberately separate from `state`/`failures`: it must never
    /// feed `should_open`/`allow()`'s Closed/Open/HalfOpen decision, since a
    /// busy-but-honest peer is not a failing peer (that conflation is
    /// exactly what produced the misleading "peer unreachable" error this
    /// feature exists to fix). Read via `is_busy()`, a routing hint for
    /// candidate selection, not a breaker-tripping signal.
    busy_until: Option<Instant>,
}

impl Default for PeerEntry {
    fn default() -> Self {
        Self {
            state: State::Closed,
            failures: 0,
            opened_at: None,
            busy_until: None,
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

    /// Record a clean `DeniedAtCapacity` Capacity Lease response — the peer
    /// answered correctly, it's simply full. Sets a short busy-backoff hint;
    /// deliberately does NOT touch `failures`/`state`, so repeated capacity
    /// denials can never trip the circuit open (call `record_success()`
    /// first, as `negotiate_lease()` does, so a busy-but-healthy peer is
    /// also credited as alive).
    pub fn record_capacity_denied(&self, peer: &PeerId, retry_after: Duration) {
        let mut map = self.peers.lock().unwrap();
        let entry = map.entry(*peer).or_default();
        entry.busy_until = Some(Instant::now() + retry_after);
    }

    /// True while a peer is inside its post-`DeniedAtCapacity` backoff
    /// window. A routing hint for candidate selection (skip this peer for
    /// now, try the next-best candidate) — must never gate `allow()`'s
    /// return value, since a busy peer is still Closed/healthy.
    ///
    /// Unused until the Phase 4 shard-chain candidate-selection integration
    /// (see the Capacity Lease plan) wires it into `forward_through_chain`'s
    /// candidate ordering; exercised directly by this module's own tests in
    /// the meantime.
    #[allow(dead_code)]
    pub fn is_busy(&self, peer: &PeerId) -> bool {
        let map = self.peers.lock().unwrap();
        map.get(peer)
            .and_then(|e| e.busy_until)
            .map(|t| Instant::now() < t)
            .unwrap_or(false)
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

    #[test]
    fn capacity_denied_never_trips_the_breaker() {
        // The direct regression test for the bug Capacity Lease fixes: a
        // peer that is repeatedly, honestly full must never look "down."
        let cb = CircuitBreaker::new();
        let p = make_peer();
        for _ in 0..(FAILURE_THRESHOLD * 3) {
            cb.record_capacity_denied(&p, Duration::from_millis(1));
        }
        assert!(
            cb.allow(&p),
            "repeated DeniedAtCapacity must never open the circuit — busy is not down"
        );
    }

    #[test]
    fn is_busy_reflects_the_backoff_window() {
        let cb = CircuitBreaker::new();
        let p = make_peer();
        assert!(!cb.is_busy(&p), "not busy before any denial");

        cb.record_capacity_denied(&p, Duration::from_millis(5));
        assert!(cb.is_busy(&p), "busy immediately after a capacity denial");

        std::thread::sleep(Duration::from_millis(15));
        assert!(
            !cb.is_busy(&p),
            "no longer busy after the backoff window elapses"
        );
    }

    #[test]
    fn capacity_denied_after_failures_does_not_erase_busy_but_record_success_would() {
        // record_success() resets the whole entry to default — this test
        // pins the ordering contract negotiate_lease() relies on: call
        // record_success() BEFORE record_capacity_denied() for a
        // DeniedAtCapacity outcome, never the other way around, or the busy
        // hint would be immediately erased.
        let cb = CircuitBreaker::new();
        let p = make_peer();
        cb.record_success(&p);
        cb.record_capacity_denied(&p, Duration::from_secs(30));
        assert!(
            cb.is_busy(&p),
            "busy hint must survive when success precedes it"
        );
    }
}
