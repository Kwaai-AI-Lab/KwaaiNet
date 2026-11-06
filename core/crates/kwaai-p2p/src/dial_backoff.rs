//! Per-peer dial backoff: a negative cache of peers whose dials failed, so a
//! kad walk stops re-dialing (and spending relay circuits on) the same dead peers.
//! Service-originated dials are [`Behaviour::exempt`]; they carry their own policy.

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fmt;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use libp2p::core::{transport::PortUse, Endpoint};
use libp2p::swarm::{
    behaviour::{ConnectionClosed, ConnectionEstablished, DialFailure, FromSwarm},
    dummy, ConnectionDenied, ConnectionId, DialError, NetworkBehaviour, THandler, THandlerInEvent,
    THandlerOutEvent, ToSwarm,
};
use libp2p::{Multiaddr, PeerId};

/// First backoff for a peer we have connected to before.
pub const SEEN_BASE: Duration = Duration::from_secs(30);
/// First backoff for a peer we have never connected to: no evidence it exists.
pub const UNSEEN_BASE: Duration = Duration::from_secs(2 * 60);
/// Floor once a relay says the peer holds no reservation there.
pub const GONE_FLOOR: Duration = Duration::from_secs(10 * 60);
/// Ceiling on any backoff.
pub const MAX_BACKOFF: Duration = Duration::from_secs(30 * 60);
/// Peers tracked at once, per map; a memory bound, oldest out.
const MAX_TRACKED: usize = 4096;

/// What a failed dial says about the peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Says nothing about the peer (our own refusal, a dial condition, no addresses).
    Ignore,
    /// The peer did not answer.
    Failed,
    /// A relay reported no reservation for the peer: it has most likely left.
    Gone,
}

/// Classify a dial error.
pub fn classify(error: &DialError) -> Verdict {
    match error {
        DialError::Transport(_) => {
            let gone = libp2p::relay::outbound::hop::ConnectError::NoReservation.to_string();
            if error.to_string().contains(&gone) {
                Verdict::Gone
            } else {
                Verdict::Failed
            }
        }
        DialError::WrongPeerId { .. } => Verdict::Failed,
        _ => Verdict::Ignore,
    }
}

#[derive(Debug, Clone)]
struct Entry {
    failures: u32,
    until: Instant,
}

/// The backoff bookkeeping, with time passed in so it can be tested.
#[derive(Debug, Default)]
pub struct DialBackoff {
    entries: HashMap<PeerId, Entry>,
    /// Peers we have held a connection to, with when it was last established.
    seen: HashMap<PeerId, Instant>,
    /// Peers with a live connection: a dial to them is never refused.
    connected: HashMap<PeerId, usize>,
}

impl DialBackoff {
    /// How long until `peer` may be dialed again, if it is in backoff.
    pub fn remaining(&self, peer: &PeerId, now: Instant) -> Option<Duration> {
        if self.connected.contains_key(peer) {
            return None;
        }
        let until = self.entries.get(peer)?.until;
        (now < until).then(|| until - now)
    }

    /// Count a failure for `peer`, returning the backoff it now serves.
    pub fn record_failure(&mut self, peer: PeerId, verdict: Verdict, now: Instant) -> Duration {
        if verdict == Verdict::Ignore || self.connected.contains_key(&peer) {
            return Duration::ZERO;
        }
        let seen = self.seen.contains_key(&peer);
        let failures = self.entries.get(&peer).map_or(0, |e| e.failures) + 1;
        let delay = backoff_for(failures, seen, verdict == Verdict::Gone);
        self.prune(now);
        self.entries.insert(
            peer,
            Entry {
                failures,
                until: now + delay,
            },
        );
        delay
    }

    /// A connection to `peer` was established: it is alive, forget its failures.
    pub fn on_connected(&mut self, peer: PeerId, now: Instant) {
        self.entries.remove(&peer);
        *self.connected.entry(peer).or_default() += 1;
        if self.seen.len() >= MAX_TRACKED && !self.seen.contains_key(&peer) {
            evict_oldest(&mut self.seen);
        }
        self.seen.insert(peer, now);
    }

    /// One connection to `peer` closed; `remaining` is how many are still open.
    pub fn on_closed(&mut self, peer: &PeerId, remaining: usize) {
        if remaining == 0 {
            self.connected.remove(peer);
        } else {
            self.connected.insert(*peer, remaining);
        }
    }

    /// Drop entries whose backoff ended long enough ago that a new failure
    /// should start the schedule over, then hold the map under its cap.
    fn prune(&mut self, now: Instant) {
        if self.entries.len() < MAX_TRACKED {
            return;
        }
        self.entries
            .retain(|_, e| now.saturating_duration_since(e.until) < MAX_BACKOFF);
        while self.entries.len() >= MAX_TRACKED {
            let Some(oldest) = self
                .entries
                .iter()
                .min_by_key(|(_, e)| e.until)
                .map(|(p, _)| *p)
            else {
                break;
            };
            self.entries.remove(&oldest);
        }
    }
}

fn evict_oldest(map: &mut HashMap<PeerId, Instant>) {
    if let Some(oldest) = map.iter().min_by_key(|(_, at)| **at).map(|(p, _)| *p) {
        map.remove(&oldest);
    }
}

/// The backoff after `failures` consecutive failures.
pub fn backoff_for(failures: u32, seen: bool, gone: bool) -> Duration {
    let base = if seen { SEEN_BASE } else { UNSEEN_BASE };
    let doublings = failures.saturating_sub(1).min(16);
    let delay = base.saturating_mul(1 << doublings).min(MAX_BACKOFF);
    match (gone, seen) {
        (false, _) => delay,
        (true, true) => delay.max(GONE_FLOOR),
        (true, false) => MAX_BACKOFF,
    }
}

/// Why a dial was refused.
#[derive(Debug)]
pub struct InBackoff {
    pub peer: PeerId,
    pub remaining: Duration,
}

impl fmt::Display for InBackoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is in dial backoff for another {}s after failed dials",
            self.peer,
            self.remaining.as_secs()
        )
    }
}

impl std::error::Error for InBackoff {}

/// The swarm-facing gate over [`DialBackoff`].
#[derive(Debug, Default)]
pub struct Behaviour {
    state: DialBackoff,
    exempt: HashSet<ConnectionId>,
    /// Targets of our own lookups: the walk must be free to dial the peer it seeks.
    sought: HashMap<PeerId, usize>,
}

impl Behaviour {
    /// Let the dial with `id` through regardless of backoff. Must be followed by
    /// [`Self::clear_exempt`] once `Swarm::dial` returns.
    pub fn exempt(&mut self, id: ConnectionId) {
        self.exempt.insert(id);
    }

    pub fn clear_exempt(&mut self, id: ConnectionId) {
        self.exempt.remove(&id);
    }

    /// Let any dial to `peer` through until a matching [`Self::unseek`].
    pub fn seek(&mut self, peer: PeerId) {
        *self.sought.entry(peer).or_default() += 1;
    }

    pub fn unseek(&mut self, peer: &PeerId) {
        if let Some(n) = self.sought.get_mut(peer) {
            *n -= 1;
            if *n == 0 {
                self.sought.remove(peer);
            }
        }
    }

    /// See [`DialBackoff::remaining`].
    pub fn remaining(&self, peer: &PeerId) -> Option<Duration> {
        self.state.remaining(peer, Instant::now())
    }

    /// Count a failure that never reached a dial (e.g. a lookup found nothing).
    pub fn record_failure(&mut self, peer: PeerId, verdict: Verdict) -> Duration {
        self.state.record_failure(peer, verdict, Instant::now())
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = dummy::ConnectionHandler;
    type ToSwarm = Infallible;

    fn handle_established_inbound_connection(
        &mut self,
        _: ConnectionId,
        _: PeerId,
        _: &Multiaddr,
        _: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        _: &[Multiaddr],
        _: Endpoint,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        if self.exempt.contains(&connection_id) {
            return Ok(vec![]);
        }
        let Some(peer) = maybe_peer.filter(|p| !self.sought.contains_key(p)) else {
            return Ok(vec![]);
        };
        match self.state.remaining(&peer, Instant::now()) {
            Some(remaining) => {
                tracing::trace!(%peer, secs = remaining.as_secs(), "dial refused: peer in backoff");
                Err(ConnectionDenied::new(InBackoff { peer, remaining }))
            }
            None => Ok(vec![]),
        }
    }

    fn handle_established_outbound_connection(
        &mut self,
        _: ConnectionId,
        _: PeerId,
        _: &Multiaddr,
        _: Endpoint,
        _: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(dummy::ConnectionHandler)
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        let now = Instant::now();
        match event {
            FromSwarm::ConnectionEstablished(ConnectionEstablished { peer_id, .. }) => {
                self.state.on_connected(peer_id, now);
            }
            FromSwarm::ConnectionClosed(ConnectionClosed {
                peer_id,
                remaining_established,
                ..
            }) => self.state.on_closed(&peer_id, remaining_established),
            FromSwarm::DialFailure(DialFailure {
                peer_id: Some(peer),
                error,
                connection_id,
            }) => {
                self.exempt.remove(&connection_id);
                let verdict = classify(error);
                let delay = self.state.record_failure(peer, verdict, now);
                if !delay.is_zero() {
                    tracing::debug!(%peer, ?verdict, secs = delay.as_secs(), "dial failed — backing off");
                }
            }
            _ => {}
        }
    }

    fn on_connection_handler_event(
        &mut self,
        _: PeerId,
        _: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        match event {}
    }

    fn poll(&mut self, _: &mut Context<'_>) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::core::transport::TransportError;

    fn peer() -> PeerId {
        PeerId::random()
    }

    fn transport_error(inner: Box<dyn std::error::Error + Send + Sync>) -> DialError {
        let addr: Multiaddr = "/ip4/198.51.100.1/tcp/8000".parse().unwrap();
        DialError::Transport(vec![(
            addr,
            TransportError::Other(std::io::Error::other(inner)),
        )])
    }

    #[test]
    fn schedule_doubles_from_the_seen_base_to_the_cap() {
        assert_eq!(backoff_for(1, true, false), SEEN_BASE);
        assert_eq!(backoff_for(2, true, false), SEEN_BASE * 2);
        assert_eq!(backoff_for(3, true, false), SEEN_BASE * 4);
        assert_eq!(backoff_for(40, true, false), MAX_BACKOFF);
        assert_eq!(backoff_for(u32::MAX, false, false), MAX_BACKOFF);
    }

    #[test]
    fn a_never_seen_peer_starts_higher() {
        assert_eq!(backoff_for(1, false, false), UNSEEN_BASE);
        assert!(UNSEEN_BASE > SEEN_BASE);
    }

    #[test]
    fn gone_jumps_to_the_floor_or_straight_to_the_cap_when_never_seen() {
        assert_eq!(backoff_for(1, true, true), GONE_FLOOR);
        assert_eq!(backoff_for(1, false, true), MAX_BACKOFF);
    }

    #[test]
    fn failures_back_off_and_expire() {
        let mut b = DialBackoff::default();
        let p = peer();
        let t0 = Instant::now();
        assert_eq!(b.remaining(&p, t0), None);

        assert_eq!(b.record_failure(p, Verdict::Failed, t0), UNSEEN_BASE);
        assert_eq!(b.remaining(&p, t0), Some(UNSEEN_BASE));
        assert_eq!(b.remaining(&p, t0 + UNSEEN_BASE), None);

        // The count survives expiry, so the next failure waits longer.
        let t1 = t0 + UNSEEN_BASE;
        assert_eq!(b.record_failure(p, Verdict::Failed, t1), UNSEEN_BASE * 2);
    }

    #[test]
    fn a_connection_clears_the_backoff_and_marks_the_peer_seen() {
        let mut b = DialBackoff::default();
        let p = peer();
        let t0 = Instant::now();
        b.record_failure(p, Verdict::Failed, t0);
        b.on_connected(p, t0);
        assert_eq!(b.remaining(&p, t0), None);
        b.on_closed(&p, 0);
        assert_eq!(b.record_failure(p, Verdict::Failed, t0), SEEN_BASE);
    }

    #[test]
    fn a_connected_peer_is_never_refused_or_penalised() {
        // A failed hole punch while the relayed connection is up must not
        // block dcutr's next attempt.
        let mut b = DialBackoff::default();
        let p = peer();
        let t0 = Instant::now();
        b.on_connected(p, t0);
        assert_eq!(b.record_failure(p, Verdict::Gone, t0), Duration::ZERO);
        assert_eq!(b.remaining(&p, t0), None);
        b.on_closed(&p, 1);
        assert_eq!(b.record_failure(p, Verdict::Failed, t0), Duration::ZERO);
        b.on_closed(&p, 0);
        assert_eq!(b.record_failure(p, Verdict::Failed, t0), SEEN_BASE);
    }

    #[test]
    fn ignored_errors_leave_no_entry() {
        let mut b = DialBackoff::default();
        let p = peer();
        let t0 = Instant::now();
        assert_eq!(b.record_failure(p, Verdict::Ignore, t0), Duration::ZERO);
        assert_eq!(b.remaining(&p, t0), None);
    }

    #[test]
    fn classify_spots_a_missing_reservation_through_the_wrapping() {
        use libp2p::relay::{client::transport::Error, outbound::hop::ConnectError};
        let gone = transport_error(Box::new(Error::Connect(ConnectError::NoReservation)));
        assert_eq!(classify(&gone), Verdict::Gone);

        let limited = transport_error(Box::new(Error::Connect(
            ConnectError::ResourceLimitExceeded,
        )));
        assert_eq!(classify(&limited), Verdict::Failed);

        let refused = transport_error(Box::new(std::io::Error::from(
            std::io::ErrorKind::ConnectionRefused,
        )));
        assert_eq!(classify(&refused), Verdict::Failed);
    }

    #[test]
    fn classify_ignores_errors_that_say_nothing_about_the_peer() {
        assert_eq!(classify(&DialError::NoAddresses), Verdict::Ignore);
        assert_eq!(classify(&DialError::Aborted), Verdict::Ignore);
        let denied = DialError::Denied {
            cause: ConnectionDenied::new(InBackoff {
                peer: peer(),
                remaining: Duration::from_secs(1),
            }),
        };
        assert_eq!(classify(&denied), Verdict::Ignore);
    }

    #[test]
    fn the_gate_refuses_a_peer_in_backoff_unless_the_dial_is_exempt() {
        let mut gate = Behaviour::default();
        let p = peer();
        gate.record_failure(p, Verdict::Failed);

        let id = ConnectionId::new_unchecked(1);
        assert!(gate
            .handle_pending_outbound_connection(id, Some(p), &[], Endpoint::Dialer)
            .is_err());

        gate.exempt(id);
        assert!(gate
            .handle_pending_outbound_connection(id, Some(p), &[], Endpoint::Dialer)
            .is_ok());
        gate.clear_exempt(id);

        // Nor is the target of one of our own lookups, until the lookup ends.
        gate.seek(p);
        gate.seek(p);
        gate.unseek(&p);
        assert!(gate
            .handle_pending_outbound_connection(id, Some(p), &[], Endpoint::Dialer)
            .is_ok());
        gate.unseek(&p);
        assert!(gate
            .handle_pending_outbound_connection(id, Some(p), &[], Endpoint::Dialer)
            .is_err());

        // A dial with no peer id has nothing to look up.
        assert!(gate
            .handle_pending_outbound_connection(id, None, &[], Endpoint::Dialer)
            .is_ok());
    }

    #[test]
    fn the_map_stays_bounded() {
        let mut b = DialBackoff::default();
        let t0 = Instant::now();
        for _ in 0..MAX_TRACKED + 10 {
            b.record_failure(peer(), Verdict::Failed, t0);
        }
        assert!(b.entries.len() <= MAX_TRACKED);
    }
}
