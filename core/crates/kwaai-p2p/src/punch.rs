//! DCUtR with hole-punch counters.
//!
//! `dcutr::Behaviour` reports an outcome only on some paths: the side that
//! answers a punch gets no event when its dial fails. This wrapper counts the
//! punch dials themselves, which both sides make, by watching the dials dcutr
//! asks the swarm for and how each one ends.

use std::collections::HashMap;
use std::task::{Context, Poll};

use libp2p::core::transport::PortUse;
use libp2p::core::{Endpoint, Multiaddr};
use libp2p::dcutr;
use libp2p::swarm::{
    ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, THandler, THandlerInEvent,
    THandlerOutEvent, ToSwarm,
};
use libp2p::PeerId;

/// Hole-punch outcomes since the node started.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PunchStats {
    /// Punch dials started, plus punches that failed before any dial.
    pub attempted: u64,
    /// Punch dials that produced a direct connection.
    pub succeeded: u64,
    /// Punch dials that failed, plus punches that failed before any dial.
    pub failed: u64,
}

pub struct Behaviour {
    inner: dcutr::Behaviour,
    /// Punch dials still in flight, by the connection id dcutr gave them.
    dials: HashMap<ConnectionId, PeerId>,
    stats: PunchStats,
}

impl Behaviour {
    pub fn new(local_peer_id: PeerId) -> Self {
        Self {
            inner: dcutr::Behaviour::new(local_peer_id),
            dials: HashMap::new(),
            stats: PunchStats::default(),
        }
    }

    pub fn stats(&self) -> PunchStats {
        self.stats
    }
}

/// Whether a dcutr error came from its dial-retry limit — whose dials were
/// already counted — rather than from the protocol exchange before any dial.
/// Matches libp2p-dcutr 0.14's `InnerError::AttemptsExceeded` text.
fn after_dials(error: &dcutr::Error) -> bool {
    error.to_string().contains("dial attempts")
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = <dcutr::Behaviour as NetworkBehaviour>::ConnectionHandler;
    type ToSwarm = dcutr::Event;

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<(), ConnectionDenied> {
        self.inner
            .handle_pending_inbound_connection(connection_id, local_addr, remote_addr)
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        self.inner.handle_established_inbound_connection(
            connection_id,
            peer,
            local_addr,
            remote_addr,
        )
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[Multiaddr],
        effective_role: Endpoint,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        self.inner.handle_pending_outbound_connection(
            connection_id,
            maybe_peer,
            addresses,
            effective_role,
        )
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role_override: Endpoint,
        port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        self.inner.handle_established_outbound_connection(
            connection_id,
            peer,
            addr,
            role_override,
            port_use,
        )
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        match &event {
            FromSwarm::ConnectionEstablished(e)
                if self.dials.remove(&e.connection_id).is_some() =>
            {
                self.stats.succeeded += 1;
            }
            FromSwarm::DialFailure(f) if self.dials.remove(&f.connection_id).is_some() => {
                self.stats.failed += 1;
            }
            _ => {}
        }
        self.inner.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer: PeerId,
        connection_id: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.inner
            .on_connection_handler_event(peer, connection_id, event)
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<ToSwarm<Self::ToSwarm, THandlerInEvent<Self>>> {
        let event = std::task::ready!(self.inner.poll(cx));
        match &event {
            ToSwarm::Dial { opts } => {
                if let Some(peer) = opts.get_peer_id() {
                    self.dials.insert(opts.connection_id(), peer);
                    self.stats.attempted += 1;
                }
            }
            ToSwarm::GenerateEvent(dcutr::Event {
                result: Err(error), ..
            }) if !after_dials(error) => {
                self.stats.attempted += 1;
                self.stats.failed += 1;
            }
            _ => {}
        }
        Poll::Ready(event)
    }
}
