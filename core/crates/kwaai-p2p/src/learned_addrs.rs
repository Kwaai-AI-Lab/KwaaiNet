//! The peerstore rust-libp2p does not have.
//!
//! go-libp2p keeps every address it has ever learned for a peer in a peerstore,
//! and its routed host consults that before anything else. rust-libp2p keeps
//! addresses only inside behaviours: kad's k-buckets and identify's per-
//! connection cache. Neither is a store.
//!
//! * A k-bucket holds **k = 20** peers, and a *disconnected* entry is the first
//!   one replaced — so a peer we were explicitly told how to reach is evicted
//!   by ordinary churn, precisely while it is not connected and we most need
//!   its address.
//! * kad answers `FIND_NODE` from those buckets alone
//!   (`find_closest_local_peers`), where the kad-DHT spec §6.1.1 says a server
//!   MUST answer for a peer held in its *peerstore* "even if the target node
//!   isn't a DHT Server or only advertises private addresses". That missing
//!   half is why a NATed peer cannot be resolved by PeerId at all, which is
//!   what this whole branch is about.
//! * kad never re-dials a tabled peer whose address failed, so simply calling
//!   `kad.add_address` with a learned address is *not* an equivalent: once that
//!   address fails, the entry is inert until something evicts it.
//!
//! So the service keeps its own map, filled from the signed records peers
//! publish about themselves ([`crate::peer_record`]) and from callers that
//! supply addresses with a connect request — Go's `peer.AddrInfo{ID, Addrs}`,
//! which is exactly the shape the old p2pd `doConnect` took. `dial_routed`
//! consults it before the k-bucket, so it survives a dropped connection: the
//! next routed request re-dials from the same addresses rather than starting
//! again from "no addresses". When the planned §6.1.1 `FIND_NODE` change lands,
//! it will answer from this same map.
//!
//! Two bounds, because this is fed by remote claims:
//!
//! * per peer, [`MAX_ADDRS_PER_PEER`], oldest evicted;
//! * across peers, [`MAX_PEERS`], oldest *first inserted* evicted.
//!
//! There is no TTL. Staleness is handled reactively instead, by
//! [`LearnedAddrs::forget`] on a dial failure — the same evidence-based policy
//! the routing table uses, and for the same reason: only a failed dial
//! distinguishes an address that is stale from one that merely looks odd.

use std::collections::{HashMap, VecDeque};

use libp2p::{Multiaddr, PeerId};

use crate::addresses::{strip_dest_p2p, strip_p2p};

/// Cap on learned addresses per peer.
///
/// The same six as the routing table's `MAX_ADDRESSES_PER_PEER`, and for the
/// same reason: every entry is dialed on the next attempt, so an unbounded
/// list fans one dial out across a peer's entire history.
///
/// At the cap the **oldest** address is evicted. Refusing the new one instead
/// would be simpler but wrong: a peer that legitimately moves — a NATed node
/// rotating onto a new relay, which is routine — would be frozen at the six
/// addresses it happened to publish first and could never become reachable
/// again. Oldest-out keeps the list tracking where the peer is *now*.
pub const MAX_ADDRS_PER_PEER: usize = 6;

/// Cap on how many peers the map holds.
///
/// A memory bound, not a policy: at six addresses of ~40 bytes this is well
/// under a megabyte, and it is comfortably larger than the neighbourhood a
/// node actually talks to (the routing table itself tops out around the same
/// order). Eviction is by *first insertion*, not by use — an LRU would need a
/// touch on every read from the dial path, and the cheap approximation is
/// enough for a bound that a real node is not expected to reach.
pub const MAX_PEERS: usize = 1024;

/// Addresses learned about other peers, bounded (see the module docs).
///
/// Addresses are stored **bare**, with the destination `/p2p/<peer>` stripped
/// and a circuit's relay hop kept — the convention `candidate_addresses` hands
/// out and `dial_routed` re-attaches the id to.
#[derive(Debug)]
pub struct LearnedAddrs {
    /// Our own peer id, so a record naming us can never be stored. An address
    /// of ours filed under someone else's id reaches our own listener and
    /// fails `WrongPeerId`; the address-level guard is `OwnAddresses`, this is
    /// the peer-level one.
    local: PeerId,
    peers: HashMap<PeerId, Vec<Multiaddr>>,
    /// Peers in first-insertion order, oldest at the front. Kept in step with
    /// `peers`: exactly one entry per key.
    order: VecDeque<PeerId>,
}

impl LearnedAddrs {
    pub fn new(local: PeerId) -> Self {
        Self {
            local,
            peers: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Learn `addrs` for `peer`, newest last. Returns how many were new.
    ///
    /// Empty addresses and our own peer id are refused; duplicates (compared
    /// with the destination id stripped, so the same address in two
    /// publishing conventions cannot occupy two slots) are ignored rather than
    /// re-ordered, so a repeated announcement does not count as churn.
    pub fn insert(&mut self, peer: PeerId, addrs: impl IntoIterator<Item = Multiaddr>) -> usize {
        if peer == self.local {
            return 0;
        }
        let mut added = 0;
        for addr in addrs {
            let addr = strip_dest_p2p(&addr);
            if addr.is_empty() {
                continue;
            }
            let entry = match self.peers.get_mut(&peer) {
                Some(entry) => entry,
                None => {
                    self.evict_oldest_peer_if_full();
                    self.order.push_back(peer);
                    self.peers.entry(peer).or_default()
                }
            };
            if entry.contains(&addr) {
                continue;
            }
            if entry.len() >= MAX_ADDRS_PER_PEER {
                entry.remove(0);
            }
            entry.push(addr);
            added += 1;
        }
        added
    }

    /// Everything we have learned for `peer`, oldest first.
    pub fn get(&self, peer: &PeerId) -> &[Multiaddr] {
        self.peers.get(peer).map_or(&[], Vec::as_slice)
    }

    /// Drop the learned addresses in `failed`, and the peer's entry with them
    /// if that empties it.
    ///
    /// Called from the dial-failure path, mirroring what the routing table
    /// does: an address that failed to connect is the only evidence available
    /// that it is stale, and a rotated-away circuit would otherwise stick
    /// forever. Comparison strips every `/p2p` component, because a dial
    /// reports the fully-qualified address it actually attempted.
    pub fn forget(&mut self, peer: &PeerId, failed: &[Multiaddr]) -> usize {
        let Some(entry) = self.peers.get_mut(peer) else {
            return 0;
        };
        let before = entry.len();
        entry.retain(|a| !failed.iter().any(|f| strip_p2p(f) == strip_p2p(a)));
        let dropped = before - entry.len();
        if entry.is_empty() {
            self.peers.remove(peer);
            self.order.retain(|p| p != peer);
        }
        dropped
    }

    /// How many peers the map currently holds.
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    fn evict_oldest_peer_if_full(&mut self) {
        while self.peers.len() >= MAX_PEERS {
            match self.order.pop_front() {
                Some(oldest) => {
                    self.peers.remove(&oldest);
                }
                None => break,
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

    fn map() -> LearnedAddrs {
        LearnedAddrs::new(PeerId::random())
    }

    #[test]
    fn keeps_what_it_was_given() {
        let mut m = map();
        let peer = PeerId::random();
        m.insert(peer, [ma("/ip4/203.0.113.7/tcp/4001")]);
        assert_eq!(m.get(&peer), [ma("/ip4/203.0.113.7/tcp/4001")]);
    }

    /// The destination id is stripped on the way in, so the same address in
    /// the two publishing conventions is one entry, not two.
    #[test]
    fn strips_the_destination_id_and_dedupes() {
        let mut m = map();
        let peer = PeerId::random();
        m.insert(
            peer,
            [
                ma(&format!("/ip4/203.0.113.7/tcp/4001/p2p/{peer}")),
                ma("/ip4/203.0.113.7/tcp/4001"),
            ],
        );
        assert_eq!(m.get(&peer), [ma("/ip4/203.0.113.7/tcp/4001")]);
    }

    /// A circuit keeps its relay hop — without it rust-libp2p refuses the dial
    /// outright ("Missing relay peer id").
    #[test]
    fn keeps_a_circuits_relay_hop() {
        let mut m = map();
        let peer = PeerId::random();
        let relay = PeerId::random();
        m.insert(
            peer,
            [ma(&format!(
                "/ip4/76.13.5.74/tcp/4001/p2p/{relay}/p2p-circuit/p2p/{peer}"
            ))],
        );
        assert_eq!(
            m.get(&peer),
            [ma(&format!(
                "/ip4/76.13.5.74/tcp/4001/p2p/{relay}/p2p-circuit"
            ))]
        );
    }

    /// A peer that moves must not be frozen at its first six addresses.
    #[test]
    fn per_peer_cap_evicts_the_oldest() {
        let mut m = map();
        let peer = PeerId::random();
        for i in 0..(MAX_ADDRS_PER_PEER + 2) {
            m.insert(peer, [ma(&format!("/ip4/203.0.113.{i}/tcp/4001"))]);
        }
        let held = m.get(&peer);
        assert_eq!(held.len(), MAX_ADDRS_PER_PEER);
        assert_eq!(held[0], ma("/ip4/203.0.113.2/tcp/4001"), "oldest two gone");
        assert_eq!(
            held[MAX_ADDRS_PER_PEER - 1],
            ma(&format!(
                "/ip4/203.0.113.{}/tcp/4001",
                MAX_ADDRS_PER_PEER + 1
            )),
            "the newest is kept"
        );
    }

    #[test]
    fn total_cap_evicts_the_oldest_peer() {
        let mut m = map();
        let first = PeerId::random();
        m.insert(first, [ma("/ip4/203.0.113.1/tcp/4001")]);
        for _ in 1..MAX_PEERS {
            m.insert(PeerId::random(), [ma("/ip4/203.0.113.2/tcp/4001")]);
        }
        assert_eq!(m.len(), MAX_PEERS);

        m.insert(PeerId::random(), [ma("/ip4/203.0.113.3/tcp/4001")]);
        assert_eq!(m.len(), MAX_PEERS, "the map stays bounded");
        assert!(
            m.get(&first).is_empty(),
            "the first peer inserted is the one evicted"
        );
    }

    /// Our own addresses filed under our own id would only ever dial our own
    /// listener.
    #[test]
    fn refuses_our_own_peer_id() {
        let local = PeerId::random();
        let mut m = LearnedAddrs::new(local);
        assert_eq!(m.insert(local, [ma("/ip4/203.0.113.7/tcp/4001")]), 0);
        assert!(m.get(&local).is_empty());
    }

    /// A dial reports the fully-qualified address it attempted; the stored one
    /// is bare. They must still match, or a stale circuit sticks forever.
    #[test]
    fn forget_drops_the_failed_address_however_it_is_qualified() {
        let mut m = map();
        let peer = PeerId::random();
        m.insert(
            peer,
            [
                ma("/ip4/203.0.113.7/tcp/4001"),
                ma("/ip4/203.0.113.8/tcp/4001"),
            ],
        );

        let dropped = m.forget(
            &peer,
            &[ma(&format!("/ip4/203.0.113.7/tcp/4001/p2p/{peer}"))],
        );
        assert_eq!(dropped, 1);
        assert_eq!(m.get(&peer), [ma("/ip4/203.0.113.8/tcp/4001")]);
    }

    #[test]
    fn forget_removes_a_peer_whose_last_address_failed() {
        let mut m = map();
        let peer = PeerId::random();
        m.insert(peer, [ma("/ip4/203.0.113.7/tcp/4001")]);
        m.forget(&peer, &[ma("/ip4/203.0.113.7/tcp/4001")]);
        assert!(m.is_empty(), "an empty entry must not linger");
    }

    /// An address with no transport cannot be dialed, only mistaken for
    /// reachability.
    #[test]
    fn refuses_an_address_that_is_only_a_peer_id() {
        let mut m = map();
        let peer = PeerId::random();
        assert_eq!(m.insert(peer, [ma(&format!("/p2p/{peer}"))]), 0);
        assert!(m.is_empty());
    }
}
