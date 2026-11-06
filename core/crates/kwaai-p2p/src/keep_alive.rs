//! Keep a peer's connections open while application traffic is flowing.
//!
//! The swarm's idle timeout is global. A peer we exchange requests with — a
//! hop on an inference route — holds a lease instead: while it lasts, every
//! connection to that peer reports keep-alive, so a pause between requests
//! does not cost a fresh dial (and, for a NATed peer, a fresh relay circuit).
//! The service renews the lease on every application request, in or out.

use std::collections::HashMap;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use futures::FutureExt;
use libp2p::core::transport::PortUse;
use libp2p::core::upgrade::DeniedUpgrade;
use libp2p::core::{Endpoint, Multiaddr};
use libp2p::swarm::handler::ConnectionEvent;
use libp2p::swarm::{
    ConnectionDenied, ConnectionHandler, ConnectionHandlerEvent, ConnectionId, FromSwarm,
    NetworkBehaviour, SubstreamProtocol, THandler, THandlerInEvent, THandlerOutEvent, ToSwarm,
};
use libp2p::PeerId;

/// Above this many leases, expired ones for disconnected peers are dropped.
const LEASE_PRUNE_AT: usize = 1024;

/// A lease deadline in milliseconds since the behaviour's epoch; 0 is none.
type Deadline = Arc<AtomicU64>;

pub struct Behaviour {
    epoch: Instant,
    /// When the lease map was last pruned, so a large map is not rescanned per request.
    pruned_at: Instant,
    ttl: Duration,
    leases: HashMap<PeerId, Deadline>,
    connected: HashMap<PeerId, usize>,
}

impl Behaviour {
    /// `ttl` is how long one request keeps the peer's connections open.
    pub fn new(ttl: Duration) -> Self {
        Self {
            epoch: Instant::now(),
            pruned_at: Instant::now(),
            ttl,
            leases: HashMap::new(),
            connected: HashMap::new(),
        }
    }

    fn now_ms(&self) -> u64 {
        self.epoch.elapsed().as_millis() as u64
    }

    /// Hold `peer`'s connections open for another `ttl`.
    pub fn lease(&mut self, peer: PeerId) {
        if self.ttl.is_zero() {
            return;
        }
        if self.leases.len() >= LEASE_PRUNE_AT && self.pruned_at.elapsed() >= self.ttl / 4 {
            self.pruned_at = Instant::now();
            let now = self.now_ms();
            let connected = &self.connected;
            self.leases
                .retain(|p, d| connected.contains_key(p) || d.load(Ordering::Relaxed) > now);
        }
        let until = self.now_ms() + self.ttl.as_millis() as u64;
        self.leases
            .entry(peer)
            .or_default()
            .fetch_max(until, Ordering::Relaxed);
    }

    /// End `peer`'s lease; its connections fall back to the idle timeout.
    pub fn release(&mut self, peer: &PeerId) {
        if let Some(deadline) = self.leases.get(peer) {
            deadline.store(0, Ordering::Relaxed);
        }
    }

    /// Whether `peer` currently holds a lease.
    pub fn is_leased(&self, peer: &PeerId) -> bool {
        let now = self.now_ms();
        self.leases
            .get(peer)
            .is_some_and(|d| d.load(Ordering::Relaxed) > now)
    }

    fn handler(&mut self, peer: PeerId) -> Handler {
        *self.connected.entry(peer).or_default() += 1;
        Handler {
            epoch: self.epoch,
            deadline: Arc::clone(self.leases.entry(peer).or_default()),
            timer: None,
        }
    }
}

pub struct Handler {
    epoch: Instant,
    deadline: Deadline,
    /// Wakes the connection when the lease runs out, so the idle timer starts then.
    timer: Option<(u64, Pin<Box<tokio::time::Sleep>>)>,
}

impl ConnectionHandler for Handler {
    type FromBehaviour = Infallible;
    type ToBehaviour = Infallible;
    type InboundProtocol = DeniedUpgrade;
    type OutboundProtocol = DeniedUpgrade;
    type InboundOpenInfo = ();
    type OutboundOpenInfo = ();

    fn listen_protocol(&self) -> SubstreamProtocol<Self::InboundProtocol> {
        SubstreamProtocol::new(DeniedUpgrade, ())
    }

    fn connection_keep_alive(&self) -> bool {
        self.deadline.load(Ordering::Relaxed) > self.epoch.elapsed().as_millis() as u64
    }

    fn on_behaviour_event(&mut self, event: Self::FromBehaviour) {
        match event {}
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ConnectionHandlerEvent<Self::OutboundProtocol, (), Self::ToBehaviour>> {
        let deadline = self.deadline.load(Ordering::Relaxed);
        if self.timer.as_ref().map(|(at, _)| *at) != Some(deadline) {
            let wake_at = tokio::time::Instant::from_std(self.epoch)
                + Duration::from_millis(deadline.saturating_add(1));
            self.timer = Some((deadline, Box::pin(tokio::time::sleep_until(wake_at))));
        }
        if let Some((_, sleep)) = self.timer.as_mut() {
            // Ready only means the lease ran out; the swarm re-reads keep-alive after this poll.
            let _ = sleep.poll_unpin(cx);
        }
        Poll::Pending
    }

    fn on_connection_event(
        &mut self,
        _event: ConnectionEvent<Self::InboundProtocol, Self::OutboundProtocol>,
    ) {
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = Handler;
    type ToSwarm = Infallible;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        peer: PeerId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(self.handler(peer))
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        peer: PeerId,
        _addr: &Multiaddr,
        _role_override: Endpoint,
        _port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        Ok(self.handler(peer))
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        if let FromSwarm::ConnectionClosed(closed) = event {
            if closed.remaining_established == 0 {
                self.connected.remove(&closed.peer_id);
                if !self.is_leased(&closed.peer_id) {
                    self.leases.remove(&closed.peer_id);
                }
            }
        }
    }

    fn on_connection_handler_event(
        &mut self,
        _peer: PeerId,
        _connection: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        match event {}
    }

    fn poll(
        &mut self,
        _cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lease_lasts_its_ttl_and_release_ends_it() {
        let mut b = Behaviour::new(Duration::from_secs(60));
        let peer = PeerId::random();
        assert!(!b.is_leased(&peer));
        b.lease(peer);
        assert!(b.is_leased(&peer));
        b.release(&peer);
        assert!(!b.is_leased(&peer));
    }

    #[test]
    fn a_zero_ttl_never_leases() {
        let mut b = Behaviour::new(Duration::ZERO);
        let peer = PeerId::random();
        b.lease(peer);
        assert!(!b.is_leased(&peer));
    }

    #[test]
    fn a_connection_shares_its_peers_lease() {
        let mut b = Behaviour::new(Duration::from_secs(60));
        let peer = PeerId::random();
        let handler = b.handler(peer);
        assert!(!handler.connection_keep_alive());
        b.lease(peer);
        assert!(handler.connection_keep_alive());
        b.release(&peer);
        assert!(!handler.connection_keep_alive());
    }
}
