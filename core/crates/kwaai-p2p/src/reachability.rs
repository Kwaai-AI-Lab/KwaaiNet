//! Am I reachable from the outside, and how do I know?
//!
//! Four independent sources can claim to answer that, and they disagree. The
//! operator declares a port forward; UPnP negotiates a mapping with the
//! gateway; AutoNAT gets a peer to dial us back; a crowd of identify responses
//! all report the same observed address. Each is evidence of a different
//! quality, so the machine is a **priority ladder**, not a last-writer-wins
//! flag:
//!
//! | source | who says so | can be demoted by |
//! | --- | --- | --- |
//! | [`Source::Declared`] | the operator, via `external_addr` | nothing |
//! | [`Source::AutoNat`] | a peer that actually dialed us back | AutoNAT itself |
//! | [`Source::Upnp`] | the local gateway | UPnP expiry, AutoNAT `Private` |
//! | [`Source::IdentifyConsensus`] | ≥ N peers reporting the same address | AutoNAT `Private`, and any stronger source |
//!
//! Two rules are worth stating outright because they look like bugs:
//!
//! **`Declared` outranks `force_private`.** Setting both is contradictory and
//! logs a warning, but the declaration wins. An operator who has typed in their
//! public address knows something a probe cannot disprove — and a failed
//! dialback is evidence about the *prober*, not only about us.
//!
//! **UPnP expiry returns to `Unknown`, not `Private`.** A lapsed port mapping
//! means we stopped knowing, not that we learned we are unreachable. Demoting
//! straight to Private would start relay reservations on a node that may still
//! be perfectly dialable; returning to Unknown re-opens the question and lets
//! the grace timer or AutoNAT answer it properly.
//!
//! The whole thing is deliberately I/O-free: it takes events and returns
//! [`Effect`]s describing what the caller should do to the swarm. That is what
//! makes every rule testable without a network.

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use libp2p::{Multiaddr, PeerId};
use tracing::{debug, info, warn};

use crate::addresses::is_announceable_with;

/// How long to wait for real evidence before falling back to identify
/// consensus. Long enough for AutoNAT's boot delay plus a probe round-trip
/// (5s + one 30s retry), short enough that a node is not stuck Unknown — and
/// therefore not announcing — for a noticeable part of its first announce
/// interval.
pub const IDENTIFY_GRACE: Duration = Duration::from_secs(45);

/// Where a reachability verdict came from. Ordered weakest to strongest; the
/// `Ord` derive *is* the priority ladder, so a stronger source cannot be
/// overwritten by a weaker one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    /// ≥ `identify_min_confirmations` distinct peers reported this address.
    /// The weakest claim: nobody has actually dialed us here.
    IdentifyConsensus,
    /// The local IGD gateway says it mapped this address to us.
    Upnp,
    /// A peer dialed us back at this address and got through.
    AutoNat,
    /// The operator configured it. Not evidence at all — an instruction.
    Declared,
}

/// What we currently believe about our own reachability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reachability {
    /// No verdict yet. Announce is deferred; kad stays in client mode.
    Unknown,
    /// Reachable at `addr`, on the strength of `source`.
    Public { addr: Multiaddr, source: Source },
    /// Not reachable directly. Relay reservations are wanted.
    Private,
}

/// The coarse shape of [`Reachability`], for consumers that care whether to
/// announce rather than which address won.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReachabilityKind {
    Unknown,
    Public,
    Private,
}

/// What the announce loop needs to know, and nothing else.
///
/// The DHT record (`DHTServerInfo`) carries the node's *circuit* addresses —
/// a dialer cannot reconstruct which relay a peer holds a reservation on, so
/// the record has to say — but no direct ones: those still travel by
/// `Swarm::add_external_address` plus an identify push, entirely separate from
/// announcing. So this struct tracks exactly what the record depends on: the
/// reachability verdict, and a fingerprint of the confirmed circuit set.
///
/// That keeps the channel quiet on everything else. Identify can push a dozen
/// times and a direct address can churn without waking the announce loop. What
/// *does* wake it is a reservation moving between relays: the published
/// circuit is then wrong until the next tick, up to a full TTL of dials at a
/// relay that answers `no reservation for destination`, so the record follows
/// the reservation rather than waiting for the clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnounceState {
    /// Public, Private, or not yet known.
    pub reachability: ReachabilityKind,
    /// Whether at least one circuit reservation is confirmed. Goes straight
    /// into the DHT record's `using_relay`.
    pub using_relay: bool,
    /// Whether there is any point announcing yet. False only while reachability
    /// is Unknown — a node that does not know where it stands should not be
    /// telling the network it is Direct.
    pub announceable: bool,
    /// Order-independent fingerprint of the confirmed circuit addresses, `0`
    /// when there are none. A hash rather than the list so the state stays
    /// `Copy` and cheap to compare; the record reads the addresses themselves
    /// from the swarm at announce time.
    pub circuits: u64,
    /// Increments on every published change, so a consumer can tell "no change"
    /// from "changed back to what it was".
    pub epoch: u64,
}

/// Fingerprint a circuit set so that the same relays in any order compare
/// equal and any addition, removal or swap does not.
pub fn circuit_fingerprint(circuits: &[Multiaddr]) -> u64 {
    use std::hash::{DefaultHasher, Hash, Hasher};
    if circuits.is_empty() {
        return 0;
    }
    let mut keys: Vec<String> = circuits.iter().map(ToString::to_string).collect();
    keys.sort_unstable();
    let mut h = DefaultHasher::new();
    keys.hash(&mut h);
    h.finish()
}

impl AnnounceState {
    /// The state before anything is known.
    pub fn initial() -> Self {
        Self {
            reachability: ReachabilityKind::Unknown,
            using_relay: false,
            announceable: false,
            circuits: 0,
            epoch: 0,
        }
    }

    /// Derive the state from a reachability verdict and the confirmed circuit
    /// set, preserving `epoch` (the sender bumps it only on a real change).
    pub fn derive(reachability: &Reachability, circuits: &[Multiaddr], epoch: u64) -> Self {
        let kind = reachability.kind();
        Self {
            reachability: kind,
            // Only a *confirmed* reservation counts. A requested-but-unanswered
            // one is not a way for anybody to reach us, and announcing
            // `using_relay` on the strength of one would tell the map a node is
            // reachable when it is not.
            using_relay: !circuits.is_empty(),
            announceable: kind != ReachabilityKind::Unknown,
            circuits: circuit_fingerprint(circuits),
            epoch,
        }
    }

    /// Whether two states differ in anything a consumer acts on — i.e.
    /// everything but the epoch.
    pub fn differs(&self, other: &Self) -> bool {
        self.reachability != other.reachability
            || self.using_relay != other.using_relay
            || self.announceable != other.announceable
            || self.circuits != other.circuits
    }
}

impl Reachability {
    pub fn kind(&self) -> ReachabilityKind {
        match self {
            Reachability::Unknown => ReachabilityKind::Unknown,
            Reachability::Public { .. } => ReachabilityKind::Public,
            Reachability::Private => ReachabilityKind::Private,
        }
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Reachability::Public { .. })
    }

    /// The source backing a Public verdict, if any.
    pub fn source(&self) -> Option<Source> {
        match self {
            Reachability::Public { source, .. } => Some(*source),
            _ => None,
        }
    }
}

/// What the caller should do to the swarm as a result of a transition.
///
/// The machine never touches the swarm itself — every side effect is named
/// here and applied by [`crate::service::NetworkService`]. AutoNAT in
/// particular does *not* confirm its own verdict into the swarm's external
/// address set (0.12 emits no `ExternalAddrConfirmed`), so
/// [`Effect::ConfirmExternal`] is how a `Public` verdict becomes real.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// `Swarm::add_external_address(addr)` — publishes it over identify and
    /// flips kad to server mode.
    ConfirmExternal(Multiaddr),
    /// `Swarm::remove_external_address(addr)` — we no longer stand behind it.
    RetractExternal(Multiaddr),
}

/// The reachability state machine. See the module docs for the rules.
pub struct ReachabilityState {
    current: Reachability,
    /// Terminal Private unless an `external_addr` overrides it.
    force_private: bool,
    /// Parsed `external_addr`, pinning Public from startup.
    declared: Option<Multiaddr>,
    /// Distinct-observer threshold for the identify-consensus fallback.
    min_confirmations: usize,
    /// Whether the reserved IPv4 ranges count as routable.
    require_global_ips: bool,
    /// Whether the grace period has elapsed. Before it does, an absence of
    /// evidence is not evidence of absence.
    grace_elapsed: bool,
}

impl ReachabilityState {
    /// Build the machine from config. Returns the initial state alongside the
    /// effects to apply at startup — a declared address is confirmed
    /// immediately, before any peer has been dialed.
    pub fn new(
        force_private: bool,
        declared: Option<Multiaddr>,
        min_confirmations: usize,
        require_global_ips: bool,
    ) -> (Self, Vec<Effect>) {
        if force_private && declared.is_some() {
            warn!(
                "both force_private and external_addr are set; the declared address wins — \
                 force_private cannot make a node with a known public address unreachable"
            );
        }

        let (current, effects) = match &declared {
            Some(addr) => (
                Reachability::Public {
                    addr: addr.clone(),
                    source: Source::Declared,
                },
                vec![Effect::ConfirmExternal(addr.clone())],
            ),
            // force_private starts Private rather than Unknown so relay
            // reservations begin at t=0 instead of after the grace period —
            // that immediacy is the whole point of the flag.
            None if force_private => (Reachability::Private, Vec::new()),
            None => (Reachability::Unknown, Vec::new()),
        };

        if let Reachability::Public { addr, source } = &current {
            info!(%addr, ?source, "reachability pinned at startup");
        }

        (
            Self {
                current,
                force_private,
                declared,
                min_confirmations: min_confirmations.max(1),
                require_global_ips,
                grace_elapsed: false,
            },
            effects,
        )
    }

    /// The current verdict.
    pub fn current(&self) -> &Reachability {
        &self.current
    }

    /// Whether a declared address pins us Public. Nothing can move us then, so
    /// the caller can skip work.
    fn is_pinned(&self) -> bool {
        self.declared.is_some()
    }

    /// AutoNAT reports us publicly reachable at `addr`.
    ///
    /// Ignored under `force_private` — that flag exists precisely because
    /// AutoNAT can read a NAT-PMP mapping as public reachability and thereby
    /// stop relay circuits from ever forming.
    pub fn on_autonat_public(&mut self, addr: Multiaddr) -> Vec<Effect> {
        if self.is_pinned() {
            debug!(%addr, "autonat says public; already pinned by a declared address");
            return Vec::new();
        }
        if self.force_private {
            info!(%addr, "autonat says public, but force_private is set — staying private");
            return Vec::new();
        }
        // With `only_global_ips: false` (the default, so the RFC2544 test beds
        // work) autonat itself does no address-class filtering, and a dialback
        // from a peer on our own LAN can "confirm" an RFC1918 address. Promoting
        // it would advertise a LAN address to the whole network and tear down
        // relay circuits — the Direct-but-unreachable failure mode.
        if !is_announceable_with(&addr, self.require_global_ips) {
            info!(%addr, "autonat says public at a non-announceable address; ignoring");
            return Vec::new();
        }
        self.promote(addr, Source::AutoNat)
    }

    /// AutoNAT reports us unreachable.
    ///
    /// This demotes any source *weaker than or equal to* AutoNat — its own
    /// earlier verdict, a UPnP mapping, an identify consensus — but never a
    /// declared address.
    pub fn on_autonat_private(&mut self) -> Vec<Effect> {
        if self.is_pinned() {
            debug!("autonat says private; ignored, a declared address pins us public");
            return Vec::new();
        }
        self.demote("autonat")
    }

    /// UPnP mapped us at `addr`. It has already confirmed the address into the
    /// swarm itself, so there is no `ConfirmExternal` to emit here.
    ///
    /// Only promotes from Unknown: a UPnP mapping is a statement about the
    /// gateway, and if AutoNAT has already told us a real dialback failed, the
    /// gateway's opinion does not overturn it.
    pub fn on_upnp_external(&mut self, addr: Multiaddr) -> Vec<Effect> {
        if self.is_pinned() || self.force_private {
            return Vec::new();
        }
        if !matches!(self.current, Reachability::Unknown) {
            debug!(%addr, current = ?self.current, "upnp mapping noted; a stronger verdict stands");
            return Vec::new();
        }
        // Same guard as the autonat path: a gateway can report an internal or
        // carrier-grade address, and standing on it would be worse than Unknown.
        if !is_announceable_with(&addr, self.require_global_ips) {
            info!(%addr, "upnp mapped a non-announceable external address; ignoring");
            return Vec::new();
        }
        info!(%addr, "upnp mapping accepted as our external address");
        self.current = Reachability::Public {
            addr,
            source: Source::Upnp,
        };
        // upnp confirms its own address (behaviour.rs does it on mapping), so
        // emitting ConfirmExternal here would be a duplicate.
        Vec::new()
    }

    /// A UPnP mapping lapsed.
    ///
    /// Back to Unknown, not Private — see the module docs. If the mapping was
    /// not what we were standing on, nothing happens.
    pub fn on_upnp_expired(&mut self, addr: &Multiaddr) -> Vec<Effect> {
        if self.current.source() != Some(Source::Upnp) {
            return Vec::new();
        }
        let Reachability::Public { addr: current, .. } = &self.current else {
            return Vec::new();
        };
        if current != addr {
            return Vec::new();
        }
        info!(%addr, "upnp mapping expired; reachability is unknown again");
        // force_private nodes never got here (on_upnp_external refuses them),
        // so Unknown is right — and the grace timer or autonat will resolve it.
        self.current = Reachability::Unknown;
        vec![Effect::RetractExternal(addr.clone())]
    }

    /// The grace period elapsed. Fall back to identify consensus.
    ///
    /// `observed` is the service's live observed-address map: address → the set
    /// of distinct peers that reported it. Distinctness is what makes this
    /// evidence — one peer reporting an address may be describing a NAT mapping
    /// only that peer can use.
    pub fn on_grace_elapsed(
        &mut self,
        observed: &HashMap<Multiaddr, HashSet<PeerId>>,
    ) -> Vec<Effect> {
        self.grace_elapsed = true;
        if self.is_pinned() || self.force_private {
            return Vec::new();
        }
        if !matches!(self.current, Reachability::Unknown) {
            // Real evidence arrived during the grace period; nothing to do.
            return Vec::new();
        }

        match self.identify_consensus(observed) {
            Some((addr, observers)) => {
                info!(
                    %addr, observers,
                    "no autonat verdict within the grace period; promoting on identify consensus"
                );
                self.promote(addr, Source::IdentifyConsensus)
            }
            None => {
                info!(
                    "no reachability evidence within the grace period; assuming private \
                     and seeking a relay"
                );
                self.current = Reachability::Private;
                Vec::new()
            }
        }
    }

    /// The best identify-consensus candidate: the announceable observed address
    /// with the most distinct observers, provided it clears the threshold.
    fn identify_consensus(
        &self,
        observed: &HashMap<Multiaddr, HashSet<PeerId>>,
    ) -> Option<(Multiaddr, usize)> {
        observed
            .iter()
            .filter(|(addr, _)| is_announceable_with(addr, self.require_global_ips))
            .map(|(addr, observers)| (addr.clone(), observers.len()))
            .filter(|(_, n)| *n >= self.min_confirmations)
            // Ties broken by address so the choice is deterministic — a
            // HashMap's iteration order is not, and a verdict that changes
            // between runs on identical evidence is a debugging nightmare.
            .max_by(|a, b| {
                a.1.cmp(&b.1)
                    .then_with(|| b.0.to_string().cmp(&a.0.to_string()))
            })
    }

    /// Move to `Public { addr, source }` if `source` is at least as strong as
    /// what we already have.
    fn promote(&mut self, addr: Multiaddr, source: Source) -> Vec<Effect> {
        // Belt-and-braces: every caller checks `is_pinned()` first, but the
        // Declared invariant (an operator's address is never retracted) is
        // enforced here at the chokepoint so a future caller can't miss it.
        if self.is_pinned() && source != Source::Declared {
            debug!(%addr, ?source, "declared address pins reachability; promotion ignored");
            return Vec::new();
        }
        if let Reachability::Public {
            addr: current_addr,
            source: current_source,
        } = &self.current
        {
            if *current_source > source {
                debug!(
                    %addr, ?source, current = ?current_source,
                    "ignoring a weaker reachability claim"
                );
                return Vec::new();
            }
            if current_addr == &addr {
                // Same address, better (or equal) evidence. The swarm already
                // has the address; retracting and re-confirming it would churn
                // identify pushes and briefly drop kad out of server mode for
                // no gain. Record the stronger source and emit nothing.
                if *current_source != source {
                    debug!(%addr, from = ?current_source, to = ?source, "reachability source upgraded");
                    self.current = Reachability::Public { addr, source };
                }
                return Vec::new();
            }
            // Same source, new address (a re-NAT, a changed port forward): the
            // old one has to be retracted or identify keeps advertising it.
            let old = current_addr.clone();
            info!(from = %old, to = %addr, ?source, "external address changed");
            self.current = Reachability::Public {
                addr: addr.clone(),
                source,
            };
            return vec![Effect::RetractExternal(old), Effect::ConfirmExternal(addr)];
        }

        info!(%addr, ?source, "reachability: public");
        self.current = Reachability::Public {
            addr: addr.clone(),
            source,
        };
        vec![Effect::ConfirmExternal(addr)]
    }

    /// Drop to Private, retracting whatever address we were standing on.
    fn demote(&mut self, why: &str) -> Vec<Effect> {
        // Same chokepoint guard as `promote`: a declared address never demotes.
        if self.is_pinned() {
            debug!(why, "declared address pins reachability; demotion ignored");
            return Vec::new();
        }
        match std::mem::replace(&mut self.current, Reachability::Private) {
            Reachability::Public { addr, source } => {
                info!(%addr, ?source, why, "reachability: private");
                // Only retract what we confirmed. UPnP owns its own address in
                // the swarm's set and retracts it on expiry; taking it out from
                // under upnp would leave its bookkeeping pointing at nothing.
                if source == Source::Upnp {
                    Vec::new()
                } else {
                    vec![Effect::RetractExternal(addr)]
                }
            }
            Reachability::Unknown => {
                info!(why, "reachability: private");
                Vec::new()
            }
            Reachability::Private => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ma(s: &str) -> Multiaddr {
        s.parse().unwrap()
    }

    fn circuit(relay: u8) -> Multiaddr {
        ma(&format!(
            "/ip4/198.18.0.{relay}/tcp/4001/p2p/{}/p2p-circuit",
            peer(relay)
        ))
    }

    /// The case the fingerprint exists for: a reservation moving from one relay
    /// to another leaves `using_relay` true throughout, so before this only the
    /// clock would have republished the record — with the old relay in it.
    #[test]
    fn a_rotation_between_relays_is_a_change_worth_announcing() {
        let before = AnnounceState::derive(&Reachability::Private, &[circuit(1)], 0);
        let after = AnnounceState::derive(&Reachability::Private, &[circuit(2)], 0);
        assert!(
            before.using_relay && after.using_relay,
            "the flag alone cannot see it"
        );
        assert!(before.differs(&after));
    }

    /// Gaining or losing one of several also counts — the record lists them all.
    #[test]
    fn adding_or_dropping_a_circuit_is_a_change() {
        let one = AnnounceState::derive(&Reachability::Private, &[circuit(1)], 0);
        let two = AnnounceState::derive(&Reachability::Private, &[circuit(1), circuit(2)], 0);
        assert!(one.differs(&two));
        assert!(two.differs(&one));
    }

    /// The relay manager hands the set back from a map, so order is arbitrary
    /// and must not look like churn.
    #[test]
    fn the_same_circuits_in_another_order_are_not_a_change() {
        let a = AnnounceState::derive(&Reachability::Private, &[circuit(1), circuit(2)], 0);
        let b = AnnounceState::derive(&Reachability::Private, &[circuit(2), circuit(1)], 0);
        assert!(!a.differs(&b));
        assert_eq!(a.circuits, b.circuits);
    }

    /// No circuits is the zero fingerprint and `using_relay` false, so the
    /// initial state and a node that never reserved compare equal.
    #[test]
    fn no_circuits_matches_the_initial_state() {
        let none = AnnounceState::derive(&Reachability::Private, &[], 0);
        assert_eq!(none.circuits, 0);
        assert!(!none.using_relay);
        let mut initial = AnnounceState::initial();
        initial.reachability = ReachabilityKind::Private;
        initial.announceable = true;
        assert!(!none.differs(&initial));
    }

    fn peer(n: u8) -> PeerId {
        let mut bytes = [0u8; 32];
        bytes[0] = n;
        libp2p::identity::Keypair::ed25519_from_bytes(bytes)
            .unwrap()
            .public()
            .to_peer_id()
    }

    fn observed(entries: &[(&str, &[u8])]) -> HashMap<Multiaddr, HashSet<PeerId>> {
        entries
            .iter()
            .map(|(addr, peers)| (ma(addr), peers.iter().map(|n| peer(*n)).collect()))
            .collect()
    }

    fn plain() -> ReachabilityState {
        ReachabilityState::new(false, None, 2, false).0
    }

    // -- force_private ---------------------------------------------------

    #[test]
    fn force_private_is_terminal_against_autonat_public() {
        let (mut state, effects) = ReachabilityState::new(true, None, 2, false);
        assert!(effects.is_empty());
        // Private from t=0, not Unknown: reservations start immediately, which
        // is the entire reason the flag exists.
        assert_eq!(*state.current(), Reachability::Private);

        let effects = state.on_autonat_public(ma("/ip4/1.2.3.4/tcp/8080"));
        assert!(
            effects.is_empty(),
            "force_private must swallow the promotion"
        );
        assert_eq!(*state.current(), Reachability::Private);
    }

    #[test]
    fn force_private_also_refuses_upnp_and_identify_consensus() {
        let (mut state, _) = ReachabilityState::new(true, None, 2, false);
        assert!(state
            .on_upnp_external(ma("/ip4/1.2.3.4/tcp/8080"))
            .is_empty());
        assert_eq!(*state.current(), Reachability::Private);

        let obs = observed(&[("/ip4/1.2.3.4/tcp/8080", &[1, 2, 3])]);
        assert!(state.on_grace_elapsed(&obs).is_empty());
        assert_eq!(*state.current(), Reachability::Private);
    }

    #[test]
    fn autonat_public_at_a_lan_address_is_ignored() {
        // With `only_global_ips: false` (our default, for the RFC2544 beds)
        // autonat does no address filtering of its own: a dialback from a peer
        // on our LAN can "confirm" an RFC1918 address. Promoting it would
        // advertise a LAN address fleet-wide and tear down relay circuits.
        let mut state = plain();
        let effects = state.on_autonat_public(ma("/ip4/192.168.1.50/tcp/8080"));
        assert!(effects.is_empty(), "an RFC1918 address must not promote");
        assert_eq!(*state.current(), Reachability::Unknown);

        // The same verdict at an announceable address still promotes — the
        // guard is the classifier, not autonat suppression.
        let effects = state.on_autonat_public(ma("/ip4/198.18.0.40/tcp/8080"));
        assert!(!effects.is_empty());
        assert!(matches!(
            state.current(),
            Reachability::Public {
                source: Source::AutoNat,
                ..
            }
        ));
    }

    #[test]
    fn upnp_mapping_at_a_cgnat_address_is_ignored() {
        // A gateway behind carrier-grade NAT reports a 100.64/10 external
        // address; standing on it is worse than staying Unknown.
        let mut state = plain();
        let effects = state.on_upnp_external(ma("/ip4/100.64.0.9/tcp/8080"));
        assert!(effects.is_empty());
        assert_eq!(*state.current(), Reachability::Unknown);
    }

    // -- declared --------------------------------------------------------

    #[test]
    fn declared_external_addr_outranks_force_private_with_a_warning() {
        let addr = ma("/ip4/203.0.113.7/tcp/8080");
        let (mut state, effects) = ReachabilityState::new(true, Some(addr.clone()), 2, false);
        assert_eq!(effects, vec![Effect::ConfirmExternal(addr.clone())]);
        assert_eq!(state.current().source(), Some(Source::Declared));

        // And nothing subsequently moves it.
        assert!(state.on_autonat_private().is_empty());
        assert_eq!(state.current().source(), Some(Source::Declared));
        assert!(state.on_upnp_external(ma("/ip4/1.2.3.4/tcp/1")).is_empty());
        assert_eq!(
            *state.current(),
            Reachability::Public {
                addr,
                source: Source::Declared
            }
        );
    }

    #[test]
    fn autonat_private_demotes_identify_consensus_but_not_declared() {
        // Consensus is demotable…
        let mut state = plain();
        let obs = observed(&[("/ip4/203.0.113.7/tcp/8080", &[1, 2])]);
        state.on_grace_elapsed(&obs);
        assert_eq!(state.current().source(), Some(Source::IdentifyConsensus));
        let effects = state.on_autonat_private();
        assert_eq!(
            effects,
            vec![Effect::RetractExternal(ma("/ip4/203.0.113.7/tcp/8080"))]
        );
        assert_eq!(*state.current(), Reachability::Private);

        // …declared is not.
        let addr = ma("/ip4/203.0.113.7/tcp/8080");
        let (mut declared, _) = ReachabilityState::new(false, Some(addr), 2, false);
        assert!(declared.on_autonat_private().is_empty());
        assert!(declared.current().is_public());
    }

    #[test]
    fn autonat_public_overrides_identify_consensus() {
        let mut state = plain();
        state.on_grace_elapsed(&observed(&[("/ip4/203.0.113.7/tcp/8080", &[1, 2])]));
        assert_eq!(state.current().source(), Some(Source::IdentifyConsensus));

        // Same address, better evidence: no churn in the swarm's address set,
        // but the source is upgraded so autonat now owns the verdict.
        let effects = state.on_autonat_public(ma("/ip4/203.0.113.7/tcp/8080"));
        assert!(effects.is_empty(), "same address, nothing to re-confirm");
        assert_eq!(state.current().source(), Some(Source::AutoNat));
    }

    #[test]
    fn re_confirming_the_same_address_emits_nothing() {
        // Repeated identical verdicts are the common case — autonat re-probes
        // every refresh interval. Each one retracting and re-confirming the
        // address would churn identify pushes and briefly drop kad out of
        // server mode, for nothing.
        let mut state = plain();
        let addr = ma("/ip4/203.0.113.7/tcp/8080");
        assert_eq!(
            state.on_autonat_public(addr.clone()),
            vec![Effect::ConfirmExternal(addr.clone())]
        );
        for _ in 0..5 {
            assert!(state.on_autonat_public(addr.clone()).is_empty());
        }
    }

    #[test]
    fn a_weaker_source_cannot_overwrite_a_stronger_one() {
        let mut state = plain();
        state.on_autonat_public(ma("/ip4/203.0.113.7/tcp/8080"));
        // upnp is weaker than autonat and must not steal the verdict.
        assert!(state
            .on_upnp_external(ma("/ip4/198.51.100.1/tcp/8080"))
            .is_empty());
        assert_eq!(state.current().source(), Some(Source::AutoNat));
    }

    #[test]
    fn a_changed_address_from_the_same_source_retracts_the_old_one() {
        let mut state = plain();
        state.on_autonat_public(ma("/ip4/203.0.113.7/tcp/8080"));
        // A re-NAT or a changed port forward. Leaving the old address confirmed
        // would keep identify advertising somewhere we are not.
        let effects = state.on_autonat_public(ma("/ip4/203.0.113.8/tcp/8080"));
        assert_eq!(
            effects,
            vec![
                Effect::RetractExternal(ma("/ip4/203.0.113.7/tcp/8080")),
                Effect::ConfirmExternal(ma("/ip4/203.0.113.8/tcp/8080")),
            ]
        );
    }

    // -- upnp ------------------------------------------------------------

    #[test]
    fn upnp_expiry_returns_to_unknown_not_private() {
        let mut state = plain();
        let addr = ma("/ip4/203.0.113.7/tcp/8080");
        // upnp confirms its own address into the swarm, so no effect here.
        assert!(state.on_upnp_external(addr.clone()).is_empty());
        assert_eq!(state.current().source(), Some(Source::Upnp));

        let effects = state.on_upnp_expired(&addr);
        assert_eq!(effects, vec![Effect::RetractExternal(addr)]);
        // Unknown, not Private: we stopped knowing, we did not learn we are
        // unreachable. Demoting to Private here would start relay reservations
        // on a node that may still be perfectly dialable.
        assert_eq!(*state.current(), Reachability::Unknown);
    }

    #[test]
    fn upnp_expiry_for_an_address_we_are_not_using_is_ignored() {
        let mut state = plain();
        state.on_upnp_external(ma("/ip4/203.0.113.7/tcp/8080"));
        assert!(state
            .on_upnp_expired(&ma("/ip4/198.51.100.1/tcp/8080"))
            .is_empty());
        assert_eq!(state.current().source(), Some(Source::Upnp));
    }

    #[test]
    fn demoting_upnp_does_not_retract_its_address() {
        // upnp owns its address in the swarm's set and retracts it on expiry.
        // Retracting it from under upnp would leave its bookkeeping pointing at
        // an address the swarm no longer has.
        let mut state = plain();
        state.on_upnp_external(ma("/ip4/203.0.113.7/tcp/8080"));
        assert!(state.on_autonat_private().is_empty());
        assert_eq!(*state.current(), Reachability::Private);
    }

    // -- identify consensus ----------------------------------------------

    #[test]
    fn identify_fallback_requires_min_confirmations_distinct_observers() {
        let mut state = plain();
        // One observer is not consensus — it may be describing a NAT mapping
        // only that peer can use.
        let effects = state.on_grace_elapsed(&observed(&[("/ip4/203.0.113.7/tcp/8080", &[1])]));
        assert!(effects.is_empty());
        assert_eq!(*state.current(), Reachability::Private);

        let mut state = plain();
        state.on_grace_elapsed(&observed(&[("/ip4/203.0.113.7/tcp/8080", &[1, 2])]));
        assert_eq!(state.current().source(), Some(Source::IdentifyConsensus));
    }

    #[test]
    fn identify_fallback_ignores_non_announceable_addresses() {
        let mut state = plain();
        // Ten peers all seeing our LAN address is ten peers on our LAN, not
        // evidence of public reachability.
        let effects = state.on_grace_elapsed(&observed(&[
            ("/ip4/192.168.1.10/tcp/8080", &[1, 2, 3, 4, 5]),
            ("/ip4/127.0.0.1/tcp/8080", &[6, 7, 8]),
        ]));
        assert!(effects.is_empty());
        assert_eq!(*state.current(), Reachability::Private);
    }

    #[test]
    fn identify_fallback_picks_the_most_observed_candidate() {
        let mut state = plain();
        state.on_grace_elapsed(&observed(&[
            ("/ip4/203.0.113.7/tcp/8080", &[1, 2]),
            ("/ip4/198.51.100.1/tcp/8080", &[3, 4, 5]),
            ("/ip4/192.168.1.10/tcp/8080", &[1, 2, 3, 4, 5, 6]),
        ]));
        assert_eq!(
            *state.current(),
            Reachability::Public {
                addr: ma("/ip4/198.51.100.1/tcp/8080"),
                source: Source::IdentifyConsensus,
            }
        );
    }

    #[test]
    fn require_global_ips_excludes_the_reserved_ranges_from_consensus() {
        // Permissive: the RFC2544 test-bed address is a valid candidate.
        let mut state = plain();
        state.on_grace_elapsed(&observed(&[("/ip4/198.18.0.30/tcp/8080", &[1, 2])]));
        assert!(state.current().is_public());

        // Strict: it is not, and with nothing else on offer we go Private.
        let mut strict = ReachabilityState::new(false, None, 2, true).0;
        strict.on_grace_elapsed(&observed(&[("/ip4/198.18.0.30/tcp/8080", &[1, 2])]));
        assert_eq!(*strict.current(), Reachability::Private);
    }

    #[test]
    fn evidence_during_the_grace_period_wins_over_the_fallback() {
        let mut state = plain();
        state.on_autonat_public(ma("/ip4/203.0.113.7/tcp/8080"));
        // The timer still fires; it must not overwrite a real verdict with a
        // consensus one, even where consensus disagrees.
        let effects =
            state.on_grace_elapsed(&observed(&[("/ip4/198.51.100.1/tcp/8080", &[1, 2, 3])]));
        assert!(effects.is_empty());
        assert_eq!(
            *state.current(),
            Reachability::Public {
                addr: ma("/ip4/203.0.113.7/tcp/8080"),
                source: Source::AutoNat,
            }
        );
    }

    #[test]
    fn the_consensus_choice_is_deterministic_under_a_tie() {
        // HashMap iteration order is not stable, and a verdict that differs
        // between runs on identical evidence is miserable to debug.
        for _ in 0..16 {
            let mut state = plain();
            state.on_grace_elapsed(&observed(&[
                ("/ip4/203.0.113.7/tcp/8080", &[1, 2]),
                ("/ip4/198.51.100.1/tcp/8080", &[3, 4]),
            ]));
            assert_eq!(
                *state.current(),
                Reachability::Public {
                    addr: ma("/ip4/198.51.100.1/tcp/8080"),
                    source: Source::IdentifyConsensus,
                }
            );
        }
    }

    #[test]
    fn source_ordering_is_the_priority_ladder() {
        assert!(Source::Declared > Source::AutoNat);
        assert!(Source::AutoNat > Source::Upnp);
        assert!(Source::Upnp > Source::IdentifyConsensus);
    }
}
