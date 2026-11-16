//! Kademlia with lean dialing.
//!
//! A kad walk dials every peer it learns of to ask it FIND_NODE. For a NATed
//! peer the only addresses are relay circuits, and each one spends a request
//! from that relay's per-source-IP circuit budget, which every node behind
//! one home router shares. The walk does not need those peers: whatever they
//! would answer, the relays they reserve on answer too.
//!
//! [`Behaviour`] wraps `kad::Behaviour` and rewrites each dial kad starts so
//! it carries only direct addresses; a peer with none is refused, which kad
//! sees as an ordinary dial failure. The target of a running lookup is
//! exempt: kad only reports peers that answered. So is the liveness check on
//! a full bucket's oldest entry, which would otherwise lose its slot. Other
//! dials that borrow kad's addresses get one circuit per relay. Everything
//! else passes straight through, and the wrapper derefs to the inner
//! behaviour so callers are unchanged.

use std::{
    collections::HashSet,
    fmt,
    ops::{Deref, DerefMut},
    task::{Context, Poll},
};

use libp2p::{
    core::{transport::PortUse, Endpoint},
    kad::{self, store::MemoryStore},
    swarm::{
        dial_opts::DialOpts, ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, THandler,
        THandlerInEvent, THandlerOutEvent, ToSwarm,
    },
    Multiaddr, PeerId,
};

use crate::addresses::{is_circuit, one_circuit_per_relay};

/// Upper bound on refusals awaiting their dial; each is consumed within one swarm poll.
const MAX_PENDING_REFUSALS: usize = 1024;

pub struct Behaviour {
    inner: kad::Behaviour<MemoryStore>,
    refuse: HashSet<ConnectionId>,
    /// Whether the swarm can dial QUIC, which decides the circuit form kept per relay.
    dials_quic: bool,
}

/// Why a kad-started dial was refused.
#[derive(Debug)]
pub struct OnlyCircuitAddresses(pub PeerId);

impl fmt::Display for OnlyCircuitAddresses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "kad does not dial {} through a relay", self.0)
    }
}

impl std::error::Error for OnlyCircuitAddresses {}

impl Behaviour {
    pub fn new(inner: kad::Behaviour<MemoryStore>, dials_quic: bool) -> Self {
        Self {
            inner,
            refuse: HashSet::new(),
            dials_quic,
        }
    }

    /// The addresses kad would offer for `peer`, the same set it hands the swarm.
    fn kad_addrs(&mut self, peer: PeerId) -> Vec<Multiaddr> {
        self.inner
            .handle_pending_outbound_connection(
                ConnectionId::new_unchecked(0),
                Some(peer),
                &[],
                Endpoint::Dialer,
            )
            .unwrap_or_default()
    }

    /// Whether a running lookup is *for* `peer`. kad reports only peers that
    /// answered, so the target of a lookup must stay dialable by its circuit.
    fn is_lookup_target(&self, peer: &PeerId) -> bool {
        let key = peer.to_bytes();
        self.inner.iter_queries().any(
            |q| matches!(q.info(), kad::QueryInfo::GetClosestPeers { key: k, .. } if *k == key),
        )
    }

    /// Whether `peer` is the disconnected oldest entry of a bucket holding a
    /// pending newcomer: kad dials it to check liveness, and evicts it if unanswered.
    fn is_liveness_check(&mut self, peer: &PeerId) -> bool {
        self.inner.kbucket(*peer).is_some_and(|bucket| {
            bucket.has_pending()
                && bucket.iter().next().is_some_and(|oldest| {
                    oldest.node.key.preimage() == peer
                        && oldest.status == kad::NodeStatus::Disconnected
                })
        })
    }

    /// Rewrites a dial kad started (always by peer id alone); `None` leaves it as built.
    fn rewrite(&mut self, opts: &DialOpts) -> Option<DialOpts> {
        let peer = opts.get_peer_id()?;
        if self.is_lookup_target(&peer) || self.is_liveness_check(&peer) {
            return None;
        }
        let mut direct: Vec<Multiaddr> = Vec::new();
        for addr in self.kad_addrs(peer) {
            if !is_circuit(&addr) && !direct.contains(&addr) {
                direct.push(addr);
            }
        }
        if direct.is_empty() {
            if self.refuse.len() >= MAX_PENDING_REFUSALS {
                self.refuse.clear();
            }
            self.refuse.insert(opts.connection_id());
            tracing::trace!(%peer, "kad dial skipped: circuit addresses only");
            return None;
        }
        Some(DialOpts::peer_id(peer).addresses(direct).build())
    }
}

impl Deref for Behaviour {
    type Target = kad::Behaviour<MemoryStore>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Behaviour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = <kad::Behaviour<MemoryStore> as NetworkBehaviour>::ConnectionHandler;
    type ToSwarm = kad::Event;

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
        if self.refuse.remove(&connection_id) {
            if let Some(peer) = maybe_peer {
                return Err(ConnectionDenied::new(OnlyCircuitAddresses(peer)));
            }
        }
        self.inner
            .handle_pending_outbound_connection(
                connection_id,
                maybe_peer,
                addresses,
                effective_role,
            )
            .map(|addrs| one_circuit_per_relay(addrs, self.dials_quic))
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
        self.inner.on_swarm_event(event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.inner
            .on_connection_handler_event(peer_id, connection_id, event);
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<ToSwarm<kad::Event, THandlerInEvent<Self>>> {
        match self.inner.poll(cx) {
            Poll::Ready(ToSwarm::Dial { opts }) => {
                let opts = self.rewrite(&opts).unwrap_or(opts);
                Poll::Ready(ToSwarm::Dial { opts })
            }
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RELAY: &str = "QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc";

    fn wrapper_with(quic: bool, bucket_size: usize) -> Behaviour {
        let local = PeerId::random();
        let mut cfg = kad::Config::new(libp2p::StreamProtocol::new("/kwaai/kad/1.0.0"));
        cfg.set_kbucket_size(std::num::NonZeroUsize::new(bucket_size).unwrap());
        Behaviour::new(
            kad::Behaviour::with_config(local, MemoryStore::new(local), cfg),
            quic,
        )
    }

    fn wrapper() -> Behaviour {
        wrapper_with(false, kad::K_VALUE.get())
    }

    /// Marks `peer` connected, as the swarm would before kad sees it.
    fn connect(b: &mut Behaviour, peer: PeerId, addr: &Multiaddr) {
        let endpoint = libp2p::core::ConnectedPoint::Dialer {
            address: addr.clone(),
            role_override: Endpoint::Dialer,
            port_use: PortUse::Reuse,
        };
        b.on_swarm_event(FromSwarm::ConnectionEstablished(
            libp2p::swarm::behaviour::ConnectionEstablished {
                peer_id: peer,
                connection_id: ConnectionId::new_unchecked(99),
                endpoint: &endpoint,
                failed_addresses: &[],
                other_established: 0,
            },
        ));
    }

    /// The next dial kad emits, after the wrapper's rewrite.
    fn next_dial(b: &mut Behaviour) -> Option<DialOpts> {
        let mut cx = Context::from_waker(futures::task::noop_waker_ref());
        while let Poll::Ready(ev) = b.poll(&mut cx) {
            if let ToSwarm::Dial { opts } = ev {
                return Some(opts);
            }
        }
        None
    }

    fn circuit(host: &str, dest: PeerId) -> Multiaddr {
        format!("{host}/p2p/{RELAY}/p2p-circuit/p2p/{dest}")
            .parse()
            .unwrap()
    }

    fn pending(
        b: &mut Behaviour,
        id: ConnectionId,
        peer: PeerId,
    ) -> Result<Vec<Multiaddr>, ConnectionDenied> {
        b.handle_pending_outbound_connection(id, Some(peer), &[], Endpoint::Dialer)
    }

    #[test]
    fn a_kad_dial_keeps_only_direct_addresses() {
        let mut b = wrapper();
        let peer = PeerId::random();
        let direct: Multiaddr = "/ip4/203.0.113.7/tcp/8080".parse().unwrap();
        b.add_address(&peer, direct.clone());
        b.add_address(&peer, circuit("/ip4/198.51.100.2/tcp/8000", peer));

        let opts = b
            .rewrite(&DialOpts::peer_id(peer).build())
            .expect("rewritten");
        assert_eq!(opts.get_peer_id(), Some(peer));
        // An explicit list means the swarm discards every behaviour's additions,
        // so identify's cache cannot put the circuit back.
        let id = opts.connection_id();
        assert!(
            pending(&mut b, id, peer).is_ok(),
            "a rewritten dial is not refused"
        );
    }

    #[test]
    fn a_kad_dial_to_a_circuit_only_peer_is_refused() {
        let mut b = wrapper();
        let peer = PeerId::random();
        b.add_address(&peer, circuit("/ip4/198.51.100.2/tcp/8000", peer));

        let opts = DialOpts::peer_id(peer).build();
        let id = opts.connection_id();
        assert!(
            b.rewrite(&opts).is_none(),
            "kad's own dial is kept so it fails visibly"
        );
        assert!(
            pending(&mut b, id, peer).is_err(),
            "and the swarm refuses it"
        );
        assert!(
            pending(&mut b, id, peer).is_ok(),
            "a refusal is spent once, never leaked onto a later dial"
        );
    }

    #[test]
    fn the_target_of_a_lookup_stays_dialable_by_its_circuit() {
        let mut b = wrapper();
        let peer = PeerId::random();
        b.add_address(&peer, circuit("/ip4/198.51.100.2/tcp/8000", peer));
        b.get_closest_peers(peer);

        let opts = DialOpts::peer_id(peer).build();
        let id = opts.connection_id();
        assert!(b.rewrite(&opts).is_none());
        assert!(
            pending(&mut b, id, peer).is_ok(),
            "the lookup's own target is never refused"
        );
    }

    #[test]
    fn other_dials_still_get_kads_circuits_one_per_relay() {
        let mut b = wrapper();
        let peer = PeerId::random();
        b.add_address(&peer, circuit("/dnsaddr/bootstrap.kwaai.ai", peer));
        b.add_address(&peer, circuit("/ip4/198.51.100.2/tcp/8000", peer));

        let addrs = pending(&mut b, ConnectionId::new_unchecked(7), peer).expect("not refused");
        assert_eq!(addrs.len(), 1, "one circuit per relay: {addrs:?}");
        assert!(
            addrs[0].to_string().starts_with("/ip4/"),
            "IP form kept: {addrs:?}"
        );
    }

    #[test]
    fn the_kept_circuit_follows_the_swarms_quic_support() {
        for (quic, want) in [(false, "/dnsaddr/"), (true, "/ip4/")] {
            let mut b = wrapper_with(quic, kad::K_VALUE.get());
            let peer = PeerId::random();
            b.add_address(&peer, circuit("/ip4/198.51.100.2/udp/8000/quic-v1", peer));
            b.add_address(&peer, circuit("/dnsaddr/bootstrap.kwaai.ai", peer));

            let addrs = pending(&mut b, ConnectionId::new_unchecked(7), peer).expect("not refused");
            assert_eq!(addrs.len(), 1, "one circuit per relay: {addrs:?}");
            assert!(
                addrs[0].to_string().starts_with(want),
                "quic={quic} keeps {want}: {addrs:?}"
            );
        }
    }

    #[test]
    fn the_liveness_check_on_a_full_bucket_keeps_its_circuit() {
        // One slot per bucket, so a second peer in the same bucket contends for it.
        let mut b = wrapper_with(false, 1);
        let oldest = PeerId::random();
        let relayed = circuit("/ip4/198.51.100.2/tcp/8000", oldest);
        b.add_address(&oldest, relayed.clone());
        let range = b.kbucket(oldest).map(|k| k.range());
        let newcomer = std::iter::repeat_with(PeerId::random)
            .find(|p| b.kbucket(*p).map(|k| k.range()) == range)
            .unwrap();
        let direct: Multiaddr = "/ip4/203.0.113.9/tcp/8080".parse().unwrap();
        while next_dial(&mut b).is_some() {}

        connect(&mut b, newcomer, &direct);
        assert!(matches!(
            b.add_address(&newcomer, direct),
            kad::RoutingUpdate::Pending
        ));
        let opts = next_dial(&mut b).expect("kad checks the oldest entry");
        assert_eq!(opts.get_peer_id(), Some(oldest));
        let addrs = pending(&mut b, opts.connection_id(), oldest)
            .expect("a refused check would cost the oldest entry its slot");
        assert_eq!(addrs, vec![relayed]);
    }
}
