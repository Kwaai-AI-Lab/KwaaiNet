//! Peers behind our own router: when their private addresses may be dialed.
//!
//! A per-peer exception to [`AddrPolicy`]: [`same_nat`] needs a connection *we
//! dialed* that lands behind our router, [`admit`] bounds the addresses to our
//! own subnets, and [`LanPeers`] keeps them out of kad and every announce path.

use std::collections::{HashMap, HashSet, VecDeque};
use std::net::IpAddr;
use std::time::{Duration, Instant};

use std::task::{Context, Poll};

use libp2p::core::{transport::PortUse, Endpoint};
use libp2p::swarm::{
    ConnectionDenied, ConnectionId, FromSwarm, NetworkBehaviour, THandler, THandlerInEvent,
    THandlerOutEvent, ToSwarm,
};
use libp2p::{mdns, multiaddr::Protocol, Multiaddr, PeerId};

use crate::addresses::{is_circuit, AddrPolicy};
use crate::learned_addrs::LearnedAddrs;

/// How long an interface scan is trusted before the next use re-reads it.
pub const RESCAN_AFTER: Duration = Duration::from_secs(60);

/// Peers remembered as not speaking our kad protocol, so mDNS stops re-dialing them.
pub const MAX_REFUSED: usize = 256;

/// Why we believe a peer shares our router: where a direct connection we
/// dialed, and the noise handshake authenticated, actually landed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evidence {
    /// We reached it over loopback: the peer is on this host.
    SameHost,
    /// We reached it at a private address inside one of our subnets.
    OnLink,
    /// We reached it at our own public IP, as other peers observe it.
    SharedPublicIp,
}

/// One subnet an interface of ours sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subnet {
    addr: IpAddr,
    prefix: u8,
}

impl Subnet {
    /// `None` for a prefix wider than a plausible LAN (/8 on v4, /32 on v6),
    /// which would otherwise make most of the address space "on link".
    pub fn new(addr: IpAddr, prefix: u8) -> Option<Self> {
        let (min, max) = match addr {
            IpAddr::V4(_) => (8, 32),
            IpAddr::V6(_) => (32, 128),
        };
        (min..=max)
            .contains(&prefix)
            .then_some(Self { addr, prefix })
    }

    pub fn contains(&self, ip: IpAddr) -> bool {
        match (self.addr, ip.to_canonical()) {
            (IpAddr::V4(net), IpAddr::V4(ip)) => {
                let mask = u32::MAX << (32 - u32::from(self.prefix));
                u32::from(net) & mask == u32::from(ip) & mask
            }
            (IpAddr::V6(net), IpAddr::V6(ip)) => {
                let mask = u128::MAX
                    .checked_shl(128 - u32::from(self.prefix))
                    .unwrap_or(0);
                u128::from(net) & mask == u128::from(ip) & mask
            }
            _ => false,
        }
    }
}

/// The subnets of this host's interfaces, re-read at most every [`RESCAN_AFTER`].
#[derive(Debug, Default)]
pub struct LocalSubnets {
    nets: Vec<Subnet>,
    scanned: Option<Instant>,
    /// Set by [`Self::fixed`]: tests pin the set rather than read the host's.
    pinned: bool,
}

impl LocalSubnets {
    /// A fixed set that never rescans.
    pub fn fixed(nets: Vec<Subnet>) -> Self {
        Self {
            nets,
            scanned: None,
            pinned: true,
        }
    }

    /// Re-read the interfaces if the last scan is older than [`RESCAN_AFTER`].
    pub fn refresh(&mut self, now: Instant) {
        if self.pinned
            || self
                .scanned
                .is_some_and(|at| now.saturating_duration_since(at) < RESCAN_AFTER)
        {
            return;
        }
        self.scanned = Some(now);
        match if_addrs::get_if_addrs() {
            Ok(ifaces) => {
                self.nets = ifaces
                    .iter()
                    .filter_map(|iface| match &iface.addr {
                        if_addrs::IfAddr::V4(a) => Subnet::new(IpAddr::V4(a.ip), a.prefixlen),
                        if_addrs::IfAddr::V6(a) => Subnet::new(IpAddr::V6(a.ip), a.prefixlen),
                    })
                    .collect();
            }
            // Keep the previous set: an empty one would only disable LAN dials.
            Err(e) => tracing::debug!(error = %e, "could not list network interfaces"),
        }
    }

    pub fn contains(&self, ip: IpAddr) -> bool {
        self.nets.iter().any(|net| net.contains(ip))
    }
}

/// The IP of a direct address's first component, mapped v6 folded to v4.
pub fn leading_ip(addr: &Multiaddr) -> Option<IpAddr> {
    match addr.iter().next()? {
        Protocol::Ip4(a) => Some(IpAddr::V4(a)),
        Protocol::Ip6(a) => Some(IpAddr::V6(a).to_canonical()),
        _ => None,
    }
}

/// Unicast and usable without an interface scope: no link-local, whose v6 form
/// a multiaddr cannot carry the zone for.
fn is_plain_unicast(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(a) => {
            !(a.is_unspecified() || a.is_multicast() || a.is_broadcast() || a.is_link_local())
        }
        IpAddr::V6(a) => !(a.is_unspecified() || a.is_multicast() || a.is_unicast_link_local()),
    }
}

/// Whether `ip` is a public address under `policy` (the `only_global_ips` tier).
fn is_public(ip: IpAddr, policy: &AddrPolicy) -> bool {
    policy.announceable(&Multiaddr::from(ip))
}

/// Our public IPs as at least `min` peers *other than* `exclude` report seeing
/// us at, so the peer under test cannot vouch for itself.
pub fn observed_public_ips(
    observed: &HashMap<Multiaddr, HashSet<PeerId>>,
    exclude: &PeerId,
    min: usize,
    policy: &AddrPolicy,
) -> HashSet<IpAddr> {
    let mut by_ip: HashMap<IpAddr, HashSet<PeerId>> = HashMap::new();
    for (addr, observers) in observed {
        let Some(ip) = leading_ip(addr).filter(|ip| !is_circuit(addr) && is_public(*ip, policy))
        else {
            continue;
        };
        by_ip
            .entry(ip)
            .or_default()
            .extend(observers.iter().filter(|p| *p != exclude));
    }
    by_ip
        .into_iter()
        .filter(|(_, observers)| observers.len() >= min.max(1))
        .map(|(ip, _)| ip)
        .collect()
}

/// What an outbound direct connection to `remote` proves about the peer on its
/// far end. Inbound ones are not asked: a forwarder or SNAT picks their source.
pub fn same_nat(
    remote: &Multiaddr,
    our_public: &HashSet<IpAddr>,
    subnets: &LocalSubnets,
    policy: &AddrPolicy,
) -> Option<Evidence> {
    if is_circuit(remote) {
        return None;
    }
    let ip = leading_ip(remote)?;
    if ip.is_loopback() {
        return Some(Evidence::SameHost);
    }
    if is_public(ip, policy) {
        return our_public.contains(&ip).then_some(Evidence::SharedPublicIp);
    }
    (is_plain_unicast(ip) && subnets.contains(ip)).then_some(Evidence::OnLink)
}

/// Whether a peer that `evidence` places behind our router may be dialed at
/// `addr`: a plain direct address the ordinary policy would refuse to store,
/// inside one of our subnets. Loopback only for a peer on this host.
pub fn admit(
    addr: &Multiaddr,
    evidence: Evidence,
    subnets: &LocalSubnets,
    policy: &AddrPolicy,
) -> bool {
    if is_circuit(addr) || !policy.dialable(addr) || policy.announceable(addr) {
        return false;
    }
    let ips = addr
        .iter()
        .filter(|p| matches!(p, Protocol::Ip4(_) | Protocol::Ip6(_)))
        .count();
    let mut parts = addr.iter();
    let (Some(ip), true) = (leading_ip(addr), ips == 1) else {
        return false;
    };
    parts.next();
    if !matches!(parts.next(), Some(Protocol::Tcp(p) | Protocol::Udp(p)) if p != 0) {
        return false;
    }
    if ip.is_loopback() {
        return evidence == Evidence::SameHost;
    }
    is_plain_unicast(ip) && subnets.contains(ip)
}

/// LAN dial candidates per peer: what [`admit`] let through, and nothing else.
/// Never read by kad, the learned-address store, or any announce path.
#[derive(Debug)]
pub struct LanPeers {
    addrs: LearnedAddrs,
    refused: HashSet<PeerId>,
    refused_order: VecDeque<PeerId>,
}

impl LanPeers {
    pub fn new(local: PeerId) -> Self {
        Self {
            addrs: LearnedAddrs::new(local),
            refused: HashSet::new(),
            refused_order: VecDeque::new(),
        }
    }

    /// Keep `addrs` for `peer`; returns how many were new. Refused peers keep nothing.
    pub fn insert(&mut self, peer: PeerId, addrs: impl IntoIterator<Item = Multiaddr>) -> usize {
        if self.refused.contains(&peer) {
            return 0;
        }
        self.addrs.insert(peer, addrs)
    }

    pub fn get(&self, peer: &PeerId) -> &[Multiaddr] {
        self.addrs.get(peer)
    }

    pub fn forget(&mut self, peer: &PeerId, failed: &[Multiaddr]) -> usize {
        self.addrs.forget(peer, failed)
    }

    /// Drop `peer` and stop keeping addresses for it: it does not speak our kad.
    pub fn refuse(&mut self, peer: PeerId) {
        self.addrs.remove(&peer);
        if self.refused.insert(peer) {
            self.refused_order.push_back(peer);
            if self.refused_order.len() > MAX_REFUSED {
                if let Some(oldest) = self.refused_order.pop_front() {
                    self.refused.remove(&oldest);
                }
            }
        }
    }

    pub fn is_refused(&self, peer: &PeerId) -> bool {
        self.refused.contains(peer)
    }
}

/// mDNS, minus its two side doors into the swarm's dial path: it neither
/// answers dial-by-PeerId nor emits `NewExternalAddrOfPeer`. Discoveries reach
/// the service as events, where [`admit`] filters them.
pub struct Discovery(mdns::tokio::Behaviour);

impl Discovery {
    pub fn new(local: PeerId) -> std::io::Result<Self> {
        mdns::tokio::Behaviour::new(mdns::Config::default(), local).map(Self)
    }
}

impl NetworkBehaviour for Discovery {
    type ConnectionHandler = <mdns::tokio::Behaviour as NetworkBehaviour>::ConnectionHandler;
    type ToSwarm = mdns::Event;

    fn handle_established_inbound_connection(
        &mut self,
        id: ConnectionId,
        peer: PeerId,
        local: &Multiaddr,
        remote: &Multiaddr,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        self.0
            .handle_established_inbound_connection(id, peer, local, remote)
    }

    fn handle_established_outbound_connection(
        &mut self,
        id: ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role: Endpoint,
        port_use: PortUse,
    ) -> Result<THandler<Self>, ConnectionDenied> {
        self.0
            .handle_established_outbound_connection(id, peer, addr, role, port_use)
    }

    fn on_swarm_event(&mut self, event: FromSwarm) {
        self.0.on_swarm_event(event)
    }

    fn on_connection_handler_event(
        &mut self,
        peer: PeerId,
        id: ConnectionId,
        event: THandlerOutEvent<Self>,
    ) {
        self.0.on_connection_handler_event(peer, id, event)
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<ToSwarm<mdns::Event, THandlerInEvent<Self>>> {
        loop {
            match self.0.poll(cx) {
                Poll::Ready(ToSwarm::NewExternalAddrOfPeer { .. }) => continue,
                other => return other,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ma(s: &str) -> Multiaddr {
        s.parse().expect("test multiaddr")
    }

    fn net(s: &str, prefix: u8) -> Subnet {
        Subnet::new(s.parse().expect("test ip"), prefix).expect("plausible prefix")
    }

    /// A home LAN, a docker bridge and a v6 ULA LAN.
    fn home() -> LocalSubnets {
        LocalSubnets::fixed(vec![
            net("192.168.1.20", 24),
            net("172.17.0.1", 16),
            net("fd12:3456:789a:1::20", 64),
        ])
    }

    fn strict() -> AddrPolicy {
        AddrPolicy {
            strict: true,
            ipv6: true,
        }
    }

    fn ours(ips: &[&str]) -> HashSet<IpAddr> {
        ips.iter().map(|s| s.parse().unwrap()).collect()
    }

    // -- the evidence -----------------------------------------------------

    #[test]
    fn a_connection_from_our_own_public_ip_is_a_shared_router() {
        let remote = ma("/ip4/203.0.114.9/tcp/51000");
        assert_eq!(
            same_nat(&remote, &ours(&["203.0.114.9"]), &home(), &strict()),
            Some(Evidence::SharedPublicIp)
        );
    }

    #[test]
    fn a_different_public_ip_is_no_evidence() {
        let remote = ma("/ip4/203.0.114.10/tcp/51000");
        assert_eq!(
            same_nat(&remote, &ours(&["203.0.114.9"]), &home(), &strict()),
            None
        );
        assert_eq!(same_nat(&remote, &ours(&[]), &home(), &strict()), None);
    }

    #[test]
    fn a_private_peer_on_one_of_our_subnets_is_on_link() {
        let remote = ma("/ip4/192.168.1.31/tcp/8080");
        assert_eq!(
            same_nat(&remote, &ours(&[]), &home(), &strict()),
            Some(Evidence::OnLink)
        );
    }

    #[test]
    fn a_private_peer_outside_our_subnets_is_no_evidence() {
        let remote = ma("/ip4/10.9.8.7/tcp/8080");
        assert_eq!(same_nat(&remote, &ours(&[]), &home(), &strict()), None);
    }

    #[test]
    fn loopback_is_the_same_host() {
        let remote = ma("/ip4/127.0.0.1/tcp/8080");
        assert_eq!(
            same_nat(&remote, &ours(&[]), &home(), &strict()),
            Some(Evidence::SameHost)
        );
    }

    /// The circuit's IP is the relay's, whoever is on the far end.
    #[test]
    fn a_relayed_connection_is_never_evidence() {
        let relay = PeerId::random();
        let remote = ma(&format!(
            "/ip4/192.168.1.31/tcp/8080/p2p/{relay}/p2p-circuit"
        ));
        assert_eq!(same_nat(&remote, &ours(&[]), &home(), &strict()), None);
        let public = ma(&format!(
            "/ip4/203.0.114.9/tcp/8000/p2p/{relay}/p2p-circuit"
        ));
        assert_eq!(
            same_nat(&public, &ours(&["203.0.114.9"]), &home(), &strict()),
            None
        );
    }

    // -- whose observation counts ----------------------------------------

    #[test]
    fn the_peer_under_test_cannot_vouch_for_our_public_ip() {
        let liar = PeerId::random();
        let honest = PeerId::random();
        let mut observed: HashMap<Multiaddr, HashSet<PeerId>> = HashMap::new();
        observed.insert(ma("/ip4/203.0.114.9/tcp/4001"), HashSet::from([liar]));
        assert!(observed_public_ips(&observed, &liar, 1, &strict()).is_empty());

        observed.insert(ma("/ip4/203.0.114.9/tcp/4002"), HashSet::from([honest]));
        assert_eq!(
            observed_public_ips(&observed, &liar, 1, &strict()),
            ours(&["203.0.114.9"])
        );
        assert!(
            observed_public_ips(&observed, &liar, 2, &strict()).is_empty(),
            "one honest observer is below a threshold of two"
        );
    }

    /// Distinct ports on one IP are one public IP, counted across observers.
    #[test]
    fn observers_are_pooled_per_ip_not_per_port() {
        let (a, b) = (PeerId::random(), PeerId::random());
        let observed = HashMap::from([
            (ma("/ip4/203.0.114.9/tcp/4001"), HashSet::from([a])),
            (ma("/ip4/203.0.114.9/tcp/4002"), HashSet::from([b])),
        ]);
        assert_eq!(
            observed_public_ips(&observed, &PeerId::random(), 2, &strict()),
            ours(&["203.0.114.9"])
        );
    }

    #[test]
    fn a_private_observation_is_not_our_public_ip() {
        let observed = HashMap::from([(
            ma("/ip4/192.168.1.20/tcp/4001"),
            HashSet::from([PeerId::random(), PeerId::random()]),
        )]);
        assert!(observed_public_ips(&observed, &PeerId::random(), 1, &strict()).is_empty());
    }

    // -- which addresses the evidence admits ------------------------------

    #[test]
    fn admits_lan_addresses_inside_our_subnets() {
        for addr in [
            "/ip4/192.168.1.31/tcp/8080",
            "/ip4/172.17.0.5/tcp/8080",
            "/ip4/192.168.1.31/udp/8080/quic-v1",
            "/ip6/fd12:3456:789a:1::31/tcp/8080",
        ] {
            assert!(
                admit(&ma(addr), Evidence::SharedPublicIp, &home(), &strict()),
                "{addr}"
            );
        }
    }

    /// The spoofing case: a peer claiming a private address on a LAN we are
    /// not on must not make us dial it.
    #[test]
    fn refuses_a_private_address_outside_our_subnets() {
        for addr in [
            "/ip4/10.0.0.5/tcp/8080",
            "/ip4/192.168.2.31/tcp/8080",
            "/ip6/fd99::31/tcp/8080",
        ] {
            assert!(
                !admit(&ma(addr), Evidence::OnLink, &home(), &strict()),
                "{addr}"
            );
        }
    }

    #[test]
    fn refuses_loopback_unless_the_peer_is_on_this_host() {
        let lo = ma("/ip4/127.0.0.1/tcp/8081");
        assert!(!admit(&lo, Evidence::OnLink, &home(), &strict()));
        assert!(!admit(&lo, Evidence::SharedPublicIp, &home(), &strict()));
        assert!(admit(&lo, Evidence::SameHost, &home(), &strict()));
    }

    #[test]
    fn refuses_what_is_not_a_plain_lan_address() {
        let relay = PeerId::random();
        for addr in [
            // Public: the ordinary path stores these, so this one must not.
            "/ip4/8.8.8.8/tcp/8080".to_string(),
            // Unusable without a zone, or not unicast.
            "/ip4/169.254.3.4/tcp/8080".to_string(),
            "/ip6/fe80::1/tcp/8080".to_string(),
            "/ip4/224.0.0.251/udp/5353".to_string(),
            "/ip4/0.0.0.0/tcp/8080".to_string(),
            // No transport, port zero, mixed, or a circuit.
            "/ip4/192.168.1.31".to_string(),
            "/ip4/192.168.1.31/tcp/0".to_string(),
            "/ip4/192.168.1.31/tcp/1/ip4/192.168.1.32/tcp/2".to_string(),
            format!("/ip4/192.168.1.31/tcp/8080/p2p/{relay}/p2p-circuit"),
        ] {
            assert!(
                !admit(&ma(&addr), Evidence::SameHost, &home(), &strict()),
                "{addr}"
            );
        }
    }

    #[test]
    fn a_v4_only_node_admits_no_v6() {
        let v4_only = AddrPolicy {
            strict: true,
            ipv6: false,
        };
        let ula = ma("/ip6/fd12:3456:789a:1::31/tcp/8080");
        assert!(!admit(&ula, Evidence::OnLink, &home(), &v4_only));
    }

    /// Under `only_global_ips: false` reserved space is announceable, so it
    /// takes the ordinary path, not this one.
    #[test]
    fn the_lenient_tier_leaves_reserved_space_to_the_ordinary_path() {
        let lenient = AddrPolicy {
            strict: false,
            ipv6: true,
        };
        let subnets = LocalSubnets::fixed(vec![net("198.18.0.1", 24)]);
        let addr = ma("/ip4/198.18.0.40/tcp/8080");
        assert!(!admit(&addr, Evidence::OnLink, &subnets, &lenient));
        assert!(admit(&addr, Evidence::OnLink, &subnets, &strict()));
    }

    #[test]
    fn mapped_v6_is_judged_as_the_v4_it_carries() {
        let remote = ma("/ip6/::ffff:192.168.1.31/tcp/8080");
        assert_eq!(
            same_nat(&remote, &ours(&[]), &home(), &strict()),
            Some(Evidence::OnLink)
        );
    }

    // -- subnets ------------------------------------------------------------

    #[test]
    fn implausibly_wide_subnets_are_ignored() {
        assert!(Subnet::new("10.0.0.1".parse().unwrap(), 0).is_none());
        assert!(Subnet::new("10.0.0.1".parse().unwrap(), 7).is_none());
        assert!(Subnet::new("fd00::1".parse().unwrap(), 16).is_none());
        assert!(Subnet::new("10.0.0.1".parse().unwrap(), 8).is_some());
    }

    #[test]
    fn subnet_edges_are_exact() {
        let lan = net("192.168.1.20", 24);
        assert!(lan.contains("192.168.1.0".parse().unwrap()));
        assert!(lan.contains("192.168.1.255".parse().unwrap()));
        assert!(!lan.contains("192.168.2.0".parse().unwrap()));
        assert!(!lan.contains("fd12::1".parse().unwrap()));
        let host = net("192.168.1.20", 32);
        assert!(host.contains("192.168.1.20".parse().unwrap()));
        assert!(!host.contains("192.168.1.21".parse().unwrap()));
    }

    // -- the store ----------------------------------------------------------

    #[test]
    fn a_refused_peer_keeps_nothing_and_stays_refused() {
        let mut lan = LanPeers::new(PeerId::random());
        let peer = PeerId::random();
        lan.insert(peer, [ma("/ip4/192.168.1.31/tcp/8080")]);
        lan.refuse(peer);
        assert!(lan.get(&peer).is_empty());
        assert_eq!(lan.insert(peer, [ma("/ip4/192.168.1.31/tcp/8080")]), 0);
        assert!(lan.is_refused(&peer));
    }

    #[test]
    fn the_refused_set_is_bounded() {
        let mut lan = LanPeers::new(PeerId::random());
        let first = PeerId::random();
        lan.refuse(first);
        for _ in 0..MAX_REFUSED {
            lan.refuse(PeerId::random());
        }
        assert!(!lan.is_refused(&first), "oldest refusal evicted");
        assert_eq!(lan.refused.len(), MAX_REFUSED);
    }
}
