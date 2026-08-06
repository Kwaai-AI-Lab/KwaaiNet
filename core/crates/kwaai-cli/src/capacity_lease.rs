//! Capacity Lease — admission control for a remote peer's GPU inference slots.
//!
//! Problem: two independent processes (e.g. a long-running `rag graph build`
//! job and a separate `rag chat` session) can both dial the same remote peer
//! concurrently via `p2p://`/`mux://`. Neither knows the other exists, and
//! today's per-process `CircuitBreaker` can't tell "peer busy with someone
//! else's work" apart from "peer unreachable" — it just trips and reports a
//! misleading error.
//!
//! Capacity Lease closes this with an explicit negotiation step before
//! dispatch: a requester asks the remote peer for a slot; the peer grants or
//! denies atomically against a `Semaphore` sized to its real concurrency
//! (e.g. `OLLAMA_NUM_PARALLEL`), never a stale DHT-announced number. A grant
//! is a time-boxed lease, implicitly renewed by ongoing use and otherwise
//! left to expire — no persistence, no explicit release required for
//! correctness (though callers should still release promptly when done).
//!
//! This module is transport-agnostic: `LeaseTable` is the server-side
//! admission gate, `LeaseHolder` is what a client keeps after a successful
//! negotiation. Wiring into `mux://` and `p2p://` happens in
//! `inference_mux.rs`/`ollama_proxy.rs`.

use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use tokio::sync::{Semaphore, TryAcquireError};

use crate::circuit_breaker::CircuitBreaker;

/// Dedicated unary protocol for the `p2p://` (non-persistent-stream) path,
/// negotiated once per `resolve_inference_urls()` call rather than once per
/// request. The `mux://` path instead rides lease frames on its own
/// already-open stream (see `inference_mux.rs`) and doesn't use this.
pub const CAPACITY_LEASE_PROTO: &str = "/kwaai/capacity-lease/1.0.0";

/// Wire payload for a `CAPACITY_LEASE_PROTO` unary request. The response is
/// a bare encoded `NegotiationOutcome` — a unary call's request/response
/// pairing is already the correlation mechanism, unlike the mux path's
/// frames, which need an explicit `request_id` to multiplex over one
/// shared stream.
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaseRequestPayload {
    pub model: Option<String>,
}

/// Default lease TTL, renewed implicitly by every request dispatched under
/// it. Sized against this codebase's other timeouts (e.g. the 30s HTTP-read
/// deadline in `ollama_proxy::handle_connection`) — long enough to tolerate
/// a normal inter-chunk local-processing gap in a `dream run`/`graph build`
/// job, short enough that a crashed holder's slot isn't stuck for long.
pub const LEASE_TTL_SECS: u64 = 30;

/// Default concurrent-slot count when the environment doesn't specify
/// `OLLAMA_NUM_PARALLEL` — Ollama's own default.
pub const DEFAULT_MAX_CONCURRENT: usize = 4;

/// Server-minted, process-unique lease identifier. Never persisted, never
/// reused across process restarts — mirrors `CircuitBreaker`'s per-process
/// model (see `circuit_breaker.rs`).
pub type LeaseId = u64;

/// Opaque connection identity, transport-agnostic: the mux path keys this to
/// a per-connection counter (so a dead stream's leases can be reclaimed
/// eagerly); the p2p unary path mints one per `resolve_inference_urls()`
/// call, since there's no persistent connection to key off of there.
pub type ConnectionId = u64;

/// What the client learns from a successful negotiation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LeaseGrant {
    pub lease_id: LeaseId,
    pub ttl_secs: u32,
}

/// Negotiation outcome. `PeerUnreachable` is never sent on the wire — it's
/// synthesized client-side when the negotiation call itself fails (timeout,
/// transport error) — but it lives in this enum so every call site collapses
/// "did the call succeed" and "what did the peer say" into one match, with
/// no separate error path to remember to wire into the circuit breaker.
///
/// Breaker interaction (this is the entire point of the feature): only
/// `PeerUnreachable` should call `breaker.record_failure()`. Every `Denied*`
/// variant is proof the peer is alive and answered correctly — callers
/// should route those to `breaker.record_success()` (plain "no" is not
/// "down") and, for `DeniedAtCapacity` specifically, to
/// `CircuitBreaker::record_capacity_denied()` for a separate busy-backoff
/// that never touches the breaker's failure counter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NegotiationOutcome {
    Granted(LeaseGrant),
    DeniedAtCapacity { retry_after_secs: u32 },
    DeniedWrongModel,
    DeniedLeaseUnknown,
    DeniedProtocolUnsupported,
    PeerUnreachable,
}

/// One row in the server-side lease table. Dropping `_permit` returns the
/// slot to the `Semaphore` — this is the only mechanism that releases a
/// slot; Release/Expiry/stream-death all reduce to "remove this row."
struct LeaseRow {
    connection_id: ConnectionId,
    #[allow(dead_code)]
    // surfaced for future diagnostics/DeniedWrongModel checks at the table level
    model: String,
    expires_at: Instant,
    _permit: tokio::sync::OwnedSemaphorePermit,
}

/// Server-side admission gate + lease table for one Ollama instance.
///
/// One instance lives for the lifetime of the node process and is shared
/// via `Arc` across every transport that dispatches to that instance (both
/// the mux server and the p2p unary handler) — the semaphore must model the
/// actual shared resource (the whole Ollama process's concurrency), not any
/// one connection or protocol, or two independent callers would each believe
/// they hold exclusive capacity while still colliding inside Ollama.
pub struct LeaseTable {
    semaphore: Arc<Semaphore>,
    rows: Mutex<HashMap<LeaseId, LeaseRow>>,
    next_lease_id: AtomicU64,
    model: String,
}

impl LeaseTable {
    pub fn new(model: String, max_concurrent: usize) -> Arc<Self> {
        Arc::new(Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            rows: Mutex::new(HashMap::new()),
            next_lease_id: AtomicU64::new(1),
            model,
        })
    }

    /// Non-blocking admission: `try_acquire_owned()` only, never `.acquire()`
    /// — a queued negotiation on a long-running-request workload would block
    /// for minutes with the requester unable to distinguish "queued, will
    /// succeed" from "hung," reintroducing the exact misleading-latency
    /// problem this feature exists to remove. Losers get `DeniedAtCapacity`
    /// and are expected to fall through to their own next-best candidate.
    ///
    /// `requested_model: None` skips the model check entirely — today's
    /// `mux://`/`p2p://` proxies are generic HTTP tunnels that don't know
    /// which model a request targets (that's embedded in the forwarded JSON
    /// body, not visible at this layer), so a caller that can't yet supply a
    /// model negotiates for "any." `Some(model)` performs the strict check,
    /// for callers that do know their target model.
    pub fn try_grant(
        &self,
        requested_model: Option<&str>,
        connection_id: ConnectionId,
        ttl: Duration,
    ) -> NegotiationOutcome {
        if let Some(requested) = requested_model {
            if requested != self.model {
                return NegotiationOutcome::DeniedWrongModel;
            }
        }

        match self.semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                let lease_id = self.next_lease_id.fetch_add(1, Ordering::Relaxed);
                let expires_at = Instant::now() + ttl;
                self.rows.lock().unwrap().insert(
                    lease_id,
                    LeaseRow {
                        connection_id,
                        model: requested_model.unwrap_or(&self.model).to_string(),
                        expires_at,
                        _permit: permit,
                    },
                );
                NegotiationOutcome::Granted(LeaseGrant {
                    lease_id,
                    ttl_secs: ttl.as_secs() as u32,
                })
            }
            Err(TryAcquireError::NoPermits) => NegotiationOutcome::DeniedAtCapacity {
                // No queueing, so no real queue-position ETA to offer — a
                // short fixed hint is enough for callers to back off briefly
                // before trying the next candidate or retrying this one.
                retry_after_secs: 5,
            },
            Err(TryAcquireError::Closed) => NegotiationOutcome::DeniedAtCapacity {
                retry_after_secs: 5,
            },
        }
    }

    /// Implicit renewal: call on every request successfully dispatched under
    /// an existing `lease_id`. Bumps `expires_at`; returns `false` (callers
    /// should treat this as `DeniedLeaseUnknown`) if the row is gone —
    /// already expired, released, or never existed (e.g. post server-restart).
    pub fn renew(&self, lease_id: LeaseId, ttl: Duration) -> bool {
        let mut rows = self.rows.lock().unwrap();
        match rows.get_mut(&lease_id) {
            Some(row) => {
                row.expires_at = Instant::now() + ttl;
                true
            }
            None => false,
        }
    }

    /// Explicit release (Resolution phase) — drops the row/permit
    /// immediately, independent of TTL. A no-op if the lease is already gone.
    pub fn release(&self, lease_id: LeaseId) {
        self.rows.lock().unwrap().remove(&lease_id);
    }

    /// Reclaim every lease tied to a dead connection, without waiting out
    /// the TTL. The mux path calls this the instant its stream dies —
    /// crash-scoped detection the p2p unary path (no persistent connection)
    /// cannot provide, which is the whole reason mux is the primary carrier
    /// for this protocol.
    pub fn release_connection(&self, connection_id: ConnectionId) {
        self.rows
            .lock()
            .unwrap()
            .retain(|_, row| row.connection_id != connection_id);
    }

    /// Sweep every row whose TTL has lapsed. This is the crash-safety
    /// backstop: even a client that vanishes without triggering
    /// `release_connection` (e.g. the p2p unary path, or a mux stream that
    /// hangs rather than closing) has its slot reclaimed once its lease's
    /// `expires_at` passes. Safe to call opportunistically (e.g. from within
    /// `try_grant`) and/or from a coarse periodic timer; not required for
    /// correctness on every call, only eventually.
    pub fn sweep_expired(&self) {
        let now = Instant::now();
        self.rows
            .lock()
            .unwrap()
            .retain(|_, row| row.expires_at > now);
    }

    /// Spawn a background task that calls `sweep_expired()` on a fixed
    /// cadence for the life of the process. This is what actually makes
    /// `sweep_expired()` load-bearing rather than dead code: without it, a
    /// lease from a caller that vanishes without cleanly closing its
    /// connection (a killed process, a network partition, or any p2p unary
    /// caller — which has no persistent stream for `release_connection` to
    /// key off of at all) would never be reclaimed. Call once per
    /// `LeaseTable` right after construction; the returned handle can be
    /// dropped (fire-and-forget, matching this codebase's existing pattern
    /// for other background accept-loop tasks).
    pub fn spawn_periodic_sweep(
        self: &Arc<Self>,
        interval: Duration,
    ) -> tokio::task::JoinHandle<()> {
        let table = self.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            loop {
                ticker.tick().await;
                table.sweep_expired();
            }
        })
    }

    #[cfg(test)]
    fn active_lease_count(&self) -> usize {
        self.rows.lock().unwrap().len()
    }
}

/// Client-side holder for a granted lease, transport-agnostic. Tracks
/// whether a keepalive probe is due — the fallback renewal path used only
/// when no real request has gone out in `ttl/2`, per the implicit-renewal
/// design (real traffic renews for free; this just covers idle gaps in a
/// long-running job like `dream run`/`graph build`).
#[derive(Debug)]
pub struct LeaseHolder {
    pub lease_id: LeaseId,
    ttl: Duration,
    last_request_sent_at: Mutex<Instant>,
}

impl LeaseHolder {
    pub fn new(grant: LeaseGrant) -> Self {
        Self {
            lease_id: grant.lease_id,
            ttl: Duration::from_secs(grant.ttl_secs as u64),
            last_request_sent_at: Mutex::new(Instant::now()),
        }
    }

    /// True once `ttl/2` has elapsed since the last real request — the
    /// caller should send a `LeaseKeepalive` frame before issuing the next
    /// real request to avoid the lease lapsing during a slow local gap.
    pub fn needs_keepalive_probe(&self) -> bool {
        self.last_request_sent_at.lock().unwrap().elapsed() >= self.ttl / 2
    }

    /// Record that a real request just went out under this lease — the
    /// implicit-renewal bookkeeping counterpart to the server's `renew()`.
    pub fn mark_request_sent(&self) {
        *self.last_request_sent_at.lock().unwrap() = Instant::now();
    }
}

// ── Shared client-side breaker mapping ───────────────────────────────────────

/// Apply the standard Capacity Lease → `CircuitBreaker` taxonomy to a
/// negotiation outcome — shared by every transport's negotiation path
/// (`mux://`'s `InferenceMuxClient::negotiate_lease` and `p2p://`'s
/// `negotiate_lease_unary` below) so the mapping is defined exactly once.
///
/// Only a genuine transport failure (`PeerUnreachable`) trips the breaker.
/// Every answered outcome — `Granted` or any `Denied*` — is proof the peer
/// is alive and functioning correctly; conflating "peer said no" with "peer
/// never responded" is exactly the bug this feature exists to fix.
/// `DeniedAtCapacity` additionally records a separate busy-backoff hint
/// that never touches the breaker's failure counter.
pub fn apply_breaker_outcome(
    outcome: &NegotiationOutcome,
    peer_id: &PeerId,
    breaker: &CircuitBreaker,
) {
    match outcome {
        NegotiationOutcome::PeerUnreachable => breaker.record_failure(peer_id),
        NegotiationOutcome::DeniedAtCapacity { retry_after_secs } => {
            breaker.record_success(peer_id);
            breaker.record_capacity_denied(peer_id, Duration::from_secs(*retry_after_secs as u64));
        }
        NegotiationOutcome::Granted(_)
        | NegotiationOutcome::DeniedWrongModel
        | NegotiationOutcome::DeniedLeaseUnknown
        | NegotiationOutcome::DeniedProtocolUnsupported => breaker.record_success(peer_id),
    }
}

// ── p2p:// unary transport ────────────────────────────────────────────────────

/// Build a unary handler for `CAPACITY_LEASE_PROTO`, forwarding negotiation
/// requests to `lease_table`. Register with
/// `client.add_unary_handler(CAPACITY_LEASE_PROTO, handler, false)` —
/// mirrors `ollama_proxy::make_ollama_proxy_handler`'s shape exactly so the
/// two protocols register identically in `node.rs`/`shard_cmd.rs`.
///
/// Unary callers have no persistent connection for `release_connection` to
/// key eager reclaim off of, so this path relies entirely on the periodic
/// TTL sweep (`spawn_periodic_sweep`) for crash recovery — a fixed
/// placeholder `connection_id` is used since nothing ever targets it for
/// reclaim.
#[allow(clippy::type_complexity)]
pub fn make_capacity_lease_handler(
    lease_table: Arc<LeaseTable>,
) -> impl Fn(
    Vec<u8>,
) -> Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    const UNARY_CONNECTION_ID: ConnectionId = 0;
    move |data: Vec<u8>| {
        let lease_table = lease_table.clone();
        Box::pin(async move {
            let req: LeaseRequestPayload = match rmp_serde::from_slice(&data) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!("capacity-lease server: bad request: {e}");
                    return Err(kwaai_p2p_daemon::error::Error::Protocol(format!(
                        "bad request: {e}"
                    )));
                }
            };
            let outcome = lease_table.try_grant(
                req.model.as_deref(),
                UNARY_CONNECTION_ID,
                Duration::from_secs(LEASE_TTL_SECS),
            );
            rmp_serde::to_vec_named(&outcome)
                .map_err(|e| kwaai_p2p_daemon::error::Error::Protocol(e.to_string()))
        })
    }
}

/// Negotiate a lease over `CAPACITY_LEASE_PROTO` — the `p2p://` counterpart
/// to `InferenceMuxClient::negotiate_lease`, called once per
/// `resolve_inference_urls()` invocation rather than once per request.
/// Applies `apply_breaker_outcome` so both transports feed the breaker
/// identically.
pub async fn negotiate_lease_unary(
    client: &kwaai_p2p_daemon::P2PClient,
    peer_id: PeerId,
    model: Option<&str>,
    breaker: &CircuitBreaker,
) -> NegotiationOutcome {
    let payload = LeaseRequestPayload {
        model: model.map(String::from),
    };
    let req_bytes = match rmp_serde::to_vec_named(&payload) {
        Ok(b) => b,
        Err(_) => {
            let outcome = NegotiationOutcome::PeerUnreachable;
            apply_breaker_outcome(&outcome, &peer_id, breaker);
            return outcome;
        }
    };

    // Negotiation is a pure in-memory check on the remote side (no LLM call
    // involved) — a short timeout is enough, keeping a genuinely
    // unreachable peer from stalling anywhere near as long as a real
    // inference request is allowed to take.
    let outcome = match tokio::time::timeout(
        Duration::from_secs(10),
        client.call_unary_handler(&peer_id.to_bytes(), CAPACITY_LEASE_PROTO, &req_bytes),
    )
    .await
    {
        Ok(Ok(resp_bytes)) => rmp_serde::from_slice::<NegotiationOutcome>(&resp_bytes)
            .unwrap_or(NegotiationOutcome::PeerUnreachable),
        _ => NegotiationOutcome::PeerUnreachable,
    };

    apply_breaker_outcome(&outcome, &peer_id, breaker);
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL: &str = "llama3.1:8b";

    #[tokio::test]
    async fn periodic_sweep_reclaims_an_abandoned_lease() {
        // The crash-safety backstop for callers that vanish without ever
        // triggering release_connection (a killed process, a network
        // partition, or any p2p-unary caller, which has no persistent
        // stream to key a release off of at all).
        // Wide margins (this test flaked at 5ms ttl / 60ms wait under heavy
        // parallel test-suite + background-build CPU contention on this
        // machine) — the sweeper only needs one tick past the ttl, but
        // scheduling jitter under real load can be tens of ms, not just a
        // couple, so give it a full order of magnitude more room than that.
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let ttl = Duration::from_millis(20);
        let grant = table.try_grant(Some(MODEL), 1, ttl);
        assert!(matches!(grant, NegotiationOutcome::Granted(_)));

        let handle = table.spawn_periodic_sweep(Duration::from_millis(10));

        // Give the sweeper many ticks to run past the lease's TTL, without
        // ever calling sweep_expired()/release() ourselves.
        tokio::time::sleep(Duration::from_millis(200)).await;

        assert!(
            matches!(
                table.try_grant(Some(MODEL), 2, ttl),
                NegotiationOutcome::Granted(_)
            ),
            "the periodic sweeper must reclaim an abandoned, expired lease on its own"
        );
        handle.abort();
    }

    #[test]
    fn two_concurrent_grants_for_last_slot_one_wins() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let a = table.try_grant(Some(MODEL), 1, Duration::from_secs(30));
        let b = table.try_grant(Some(MODEL), 2, Duration::from_secs(30));

        let granted = matches!(a, NegotiationOutcome::Granted(_)) as u8
            + matches!(b, NegotiationOutcome::Granted(_)) as u8;
        let denied = matches!(a, NegotiationOutcome::DeniedAtCapacity { .. }) as u8
            + matches!(b, NegotiationOutcome::DeniedAtCapacity { .. }) as u8;
        assert_eq!(
            granted, 1,
            "exactly one of two concurrent grants for a single slot must succeed"
        );
        assert_eq!(
            denied, 1,
            "the loser must get DeniedAtCapacity, not some other outcome"
        );
    }

    #[test]
    fn expiry_reclaims_slot_without_explicit_release() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let short_ttl = Duration::from_millis(20);
        let grant = table.try_grant(Some(MODEL), 1, short_ttl);
        assert!(matches!(grant, NegotiationOutcome::Granted(_)));
        assert!(matches!(
            table.try_grant(Some(MODEL), 2, short_ttl),
            NegotiationOutcome::DeniedAtCapacity { .. }
        ));

        std::thread::sleep(Duration::from_millis(150));
        table.sweep_expired();

        assert!(
            matches!(
                table.try_grant(Some(MODEL), 3, short_ttl),
                NegotiationOutcome::Granted(_)
            ),
            "slot must be free again after the original lease's TTL lapses"
        );
    }

    #[test]
    fn renew_extends_expiry_past_the_original_deadline() {
        // Generous margins throughout (this test flaked at 15ms/10ms/10ms
        // under full-suite parallel load — scheduler jitter easily eats a
        // few ms of a `std::thread::sleep`, so timing assertions need
        // headroom well beyond the nominal durations, not just a hair's
        // width more than them).
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let ttl = Duration::from_millis(100);
        let grant = match table.try_grant(Some(MODEL), 1, ttl) {
            NegotiationOutcome::Granted(g) => g,
            other => panic!("expected Granted, got {other:?}"),
        };

        std::thread::sleep(Duration::from_millis(50));
        assert!(
            table.renew(grant.lease_id, ttl),
            "renew on a live lease must succeed"
        );

        // Elapsed since grant is now ~125ms: past the ORIGINAL 100ms ttl
        // (with a 25ms margin) but well before the renewed deadline of
        // ~150ms (renew happened at T=50ms, +100ms ttl) — also a 25ms
        // margin, symmetric headroom against scheduler jitter either way.
        std::thread::sleep(Duration::from_millis(75));
        table.sweep_expired();
        assert_eq!(
            table.active_lease_count(),
            1,
            "a renewed lease must survive past its original (pre-renewal) deadline"
        );
    }

    #[test]
    fn release_connection_reclaims_before_ttl() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let long_ttl = Duration::from_secs(30);
        let grant = table.try_grant(Some(MODEL), /* connection_id */ 42, long_ttl);
        assert!(matches!(grant, NegotiationOutcome::Granted(_)));

        table.release_connection(42);

        assert!(
            matches!(
                table.try_grant(Some(MODEL), 43, long_ttl),
                NegotiationOutcome::Granted(_)
            ),
            "dead-connection reclaim must free the slot well before the 30s TTL"
        );
    }

    #[test]
    fn denied_lease_unknown_on_stale_or_never_granted_renew() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        assert!(
            !table.renew(999, Duration::from_secs(30)),
            "renewing a lease_id that was never granted must fail, not panic or silently succeed"
        );

        let ttl = Duration::from_millis(20);
        let grant = match table.try_grant(Some(MODEL), 1, ttl) {
            NegotiationOutcome::Granted(g) => g,
            other => panic!("expected Granted, got {other:?}"),
        };
        std::thread::sleep(Duration::from_millis(150));
        table.sweep_expired();
        assert!(
            !table.renew(grant.lease_id, ttl),
            "renewing an already-expired lease_id must fail"
        );
    }

    #[test]
    fn wrong_model_denied_without_consuming_a_permit() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let outcome = table.try_grant(Some("some-other-model"), 1, Duration::from_secs(30));
        assert!(matches!(outcome, NegotiationOutcome::DeniedWrongModel));

        // The single slot must still be available for a correctly-modeled request.
        assert!(matches!(
            table.try_grant(Some(MODEL), 2, Duration::from_secs(30)),
            NegotiationOutcome::Granted(_)
        ));
    }

    #[test]
    fn none_model_bypasses_the_model_check() {
        // Callers that don't know the target model (e.g. the mux proxy,
        // which is a generic HTTP tunnel) negotiate with None and must not
        // be denied on model grounds regardless of what the table serves.
        let table = LeaseTable::new(MODEL.to_string(), 1);
        assert!(matches!(
            table.try_grant(None, 1, Duration::from_secs(30)),
            NegotiationOutcome::Granted(_)
        ));
    }

    #[test]
    fn release_frees_the_slot_immediately() {
        let table = LeaseTable::new(MODEL.to_string(), 1);
        let grant = match table.try_grant(Some(MODEL), 1, Duration::from_secs(30)) {
            NegotiationOutcome::Granted(g) => g,
            other => panic!("expected Granted, got {other:?}"),
        };
        assert!(matches!(
            table.try_grant(Some(MODEL), 2, Duration::from_secs(30)),
            NegotiationOutcome::DeniedAtCapacity { .. }
        ));

        table.release(grant.lease_id);

        assert!(matches!(
            table.try_grant(Some(MODEL), 3, Duration::from_secs(30)),
            NegotiationOutcome::Granted(_)
        ));
    }

    #[test]
    fn lease_holder_needs_keepalive_after_ttl_half_idle() {
        let holder = LeaseHolder::new(LeaseGrant {
            lease_id: 1,
            ttl_secs: 0,
        });
        // ttl_secs: 0 -> ttl/2 = 0, so any elapsed time triggers the probe —
        // exercises the boundary without needing a real multi-second sleep.
        std::thread::sleep(Duration::from_millis(1));
        assert!(holder.needs_keepalive_probe());

        holder.mark_request_sent();
        // Immediately after marking activity, elapsed (~0) should not yet
        // exceed ttl/2 (0) — this is a >= comparison so a zero-ttl holder is
        // an intentionally degenerate case; use a nonzero ttl for the
        // "does NOT need a probe right after activity" assertion instead.
        let holder = LeaseHolder::new(LeaseGrant {
            lease_id: 2,
            ttl_secs: 30,
        });
        assert!(
            !holder.needs_keepalive_probe(),
            "must not need a probe immediately after grant"
        );
        holder.mark_request_sent();
        assert!(
            !holder.needs_keepalive_probe(),
            "must not need a probe immediately after activity"
        );
    }
}
