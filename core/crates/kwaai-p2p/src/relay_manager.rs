//! Holding circuit-relay reservations while we are unreachable.
//!
//! When reachability says Private, peers cannot dial us — so we ask a relay to
//! accept connections on our behalf and forward them. A reservation is
//! requested by *listening* on `<relay-addr>/p2p/<relay>/p2p-circuit`; libp2p
//! turns that into a `HOP RESERVE` and, if the relay agrees, the circuit
//! address becomes a listen address and is confirmed as an external one.
//!
//! # What this does not do
//!
//! **Refresh.** `libp2p-relay` already renews at 3/4 of the reservation TTL and
//! keeps the connection alive while the reservation holds. Re-implementing that
//! here would fight it. What is genuinely ours is *recovery*: a relay that
//! restarts, goes away, or refuses.
//!
//! # Why it is keyed on `ListenerId`
//!
//! `relay::client::Event` has no failure variant. A refused or timed-out
//! reservation surfaces as `SwarmEvent::ListenerClosed { reason: Err(_) }`, and
//! a relay whose connection dies surfaces as `ListenerClosed { reason: Ok(()) }`.
//! So the authoritative signal is a swarm-level listener event, and the slot map
//! is keyed by the `ListenerId` `listen_on` returned. That also makes duplicate
//! reservations on one relay structurally impossible rather than merely
//! guarded-against: candidate selection skips peers that already hold a slot,
//! and a slot exists from the moment `listen_on` is called, not from the moment
//! the relay says yes.
//!
//! # Backoff and rotation are orthogonal
//!
//! A relay that refuses gets exponential backoff (30s doubling to a 15-minute
//! ceiling, ±20% jitter so a fleet restarting together does not re-converge on
//! one relay). Meanwhile the cursor advances, so the *next* attempt goes to a
//! different candidate — refusal moves us on rather than stalling us. The
//! production bootstraps advertise relay hop and have historically refused
//! reservations, which is exactly the case this shape is for.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use libp2p::{core::transport::ListenerId, Multiaddr, PeerId};
use tracing::{debug, info, warn};

use crate::addresses::{circuit_listen_addr, is_relay_candidate_addr, strip_p2p};

/// Base backoff after a relay fails us, doubling per consecutive failure.
const BACKOFF_BASE: Duration = Duration::from_secs(30);
/// Ceiling on that doubling. A relay that has been down for 15 minutes is worth
/// retrying at that cadence forever — it may simply be being redeployed.
const BACKOFF_MAX: Duration = Duration::from_secs(15 * 60);
/// A reservation that held this long was real, so the relay's failure count is
/// forgiven. Without this a relay that works for hours and then restarts would
/// inherit the backoff from whatever went wrong the first time we met it.
const BACKOFF_RESET_AFTER: Duration = Duration::from_secs(60);
/// How long a dial to a relay may sit unresolved before its slot is reclaimed.
/// The swarm reports success or failure in practice, but a permanent slot leak
/// in a process expected to run for months needs more than "in practice".
const PENDING_DIAL_TIMEOUT: Duration = Duration::from_secs(60);
/// Cap on identify-discovered candidates. The list is a convenience, not a
/// routing table, and an unbounded one is a slow memory leak on a busy node.
const MAX_DISCOVERED: usize = 16;
/// The relay-hop protocol. A peer advertising this over identify is a candidate.
pub const RELAY_HOP_PROTOCOL: &str = "/libp2p/circuit/relay/0.2.0/hop";

/// What the manager wants the service to do to the swarm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelayAction {
    /// Dial `relay` at `relay_addr`. The reservation is requested once the
    /// connection is up, not now — see [`RelayAction::Listen`].
    Dial {
        relay: PeerId,
        relay_addr: Multiaddr,
    },
    /// `Swarm::listen_on(circuit_addr)`, then hand the resulting `ListenerId`
    /// back via [`RelayManager::note_listener`].
    ///
    /// Only ever emitted for a relay we are already connected to **and** have
    /// received an identify from — see [`RelayManager::on_relay_ready`] for why
    /// both halves matter.
    Listen {
        relay: PeerId,
        circuit_addr: Multiaddr,
    },
    /// `Swarm::remove_listener(id)` — we no longer want this reservation.
    StopListening(ListenerId),
}

/// Where a slot is in its lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SlotState {
    /// `listen_on` called, no circuit address yet.
    Reserving,
    /// The relay accepted; `Multiaddr` is the circuit address peers reach us at.
    Confirmed(Multiaddr),
}

#[derive(Debug, Clone)]
struct Slot {
    relay: PeerId,
    state: SlotState,
    /// When the reservation was requested — the clock for [`BACKOFF_RESET_AFTER`].
    opened_at: Instant,
}

/// A relay's failure history.
#[derive(Debug, Clone)]
struct Backoff {
    consecutive_failures: u32,
    /// Do not try this relay again before then.
    not_before: Instant,
}

/// Holds up to `max_slots` circuit reservations while enabled. See the module
/// docs for the shape and its reasons.
pub struct RelayManager {
    /// Operator-pinned relays, tried before discovered ones.
    configured: Vec<(PeerId, Multiaddr)>,
    /// Peers seen advertising relay hop over identify, newest last.
    discovered: Vec<(PeerId, Multiaddr)>,
    /// Rotation cursor over the combined candidate list.
    cursor: usize,
    /// Live slots, keyed by the listener that owns each reservation.
    slots: HashMap<ListenerId, Slot>,
    /// Relays we have dialed and intend to reserve on as soon as the
    /// connection is up. These hold a slot too — otherwise a second candidate
    /// would be picked while the first is still connecting, and the cap would
    /// be exceeded.
    pending: HashMap<PeerId, Instant>,
    backoff: HashMap<PeerId, Backoff>,
    max_slots: usize,
    /// Whether we currently want reservations at all — driven by reachability.
    enabled: bool,
    /// `require_global_ips`: which addresses a relay may be reached at.
    strict: bool,
}

impl RelayManager {
    /// Build from the configured relay list. Unparseable entries and those with
    /// no `/p2p/<peer-id>` are dropped with a warning rather than failing the
    /// node: a typo in one relay should not stop the other from working.
    pub fn new(trusted_relays: &[String], max_slots: usize, strict: bool) -> Self {
        let mut configured = Vec::new();
        for entry in trusted_relays {
            match entry.parse::<Multiaddr>() {
                Ok(addr) => match crate::addresses::peer_id_from_multiaddr(&addr) {
                    Some(peer) => configured.push((peer, strip_p2p(&addr))),
                    None => warn!(
                        %entry,
                        "trusted relay has no /p2p/<peer-id> component; a reservation needs to \
                         know which peer it is asking, so this entry is unusable"
                    ),
                },
                Err(e) => warn!(%entry, error = %e, "unparseable trusted relay address"),
            }
        }

        Self {
            configured,
            discovered: Vec::new(),
            cursor: 0,
            slots: HashMap::new(),
            pending: HashMap::new(),
            backoff: HashMap::new(),
            // Zero slots would silently disable relaying; one is a single point
            // of failure. Treat 0 as "the config meant 1".
            max_slots: max_slots.max(1),
            enabled: false,
            strict,
        }
    }

    /// Whether we currently want reservations.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Reservations that have been accepted, as circuit addresses.
    pub fn confirmed_addrs(&self) -> Vec<Multiaddr> {
        self.slots
            .values()
            .filter_map(|slot| match &slot.state {
                SlotState::Confirmed(addr) => Some(addr.clone()),
                SlotState::Reserving => None,
            })
            .collect()
    }

    /// Whether at least one reservation is live. This is what `using_relay`
    /// reports on the DHT record: a requested-but-unconfirmed reservation is
    /// not a way for anyone to reach us.
    pub fn has_circuit(&self) -> bool {
        self.slots
            .values()
            .any(|slot| matches!(slot.state, SlotState::Confirmed(_)))
    }

    /// Turn reservation-seeking on or off. Called on every reachability change:
    /// on when Private, off when Public.
    ///
    /// Disabling tears down every slot — a publicly reachable node holding
    /// circuits costs a relay real resources for no benefit, and leaves peers
    /// routing to it the slow way.
    pub fn set_enabled(&mut self, enabled: bool, now: Instant) -> Vec<RelayAction> {
        if self.enabled == enabled {
            return Vec::new();
        }
        self.enabled = enabled;
        if enabled {
            info!("reachability is private; seeking relay reservations");
            self.fill_slots(now)
        } else {
            info!("reachability is public; releasing relay reservations");
            let actions = self
                .slots
                .keys()
                .map(|id| RelayAction::StopListening(*id))
                .collect();
            self.slots.clear();
            // Dials in flight must be forgotten too, or a connection landing
            // after we went public would request a reservation we no longer
            // want. `on_relay_connected` checks `enabled` as well, so this is
            // belt and braces — but the belt is what keeps the slot accounting
            // honest if we flip back to Private.
            self.pending.clear();
            actions
        }
    }

    /// Record a peer that advertises relay hop over identify.
    ///
    /// This — not the kad routing table — is the supply of relay candidates.
    /// Probing the routing table is the `-relayDiscovery` behaviour the p2pd
    /// path always ran with disabled: it turns every DHT peer into a
    /// reservation attempt.
    pub fn note_identify(
        &mut self,
        peer: PeerId,
        protocols: &[String],
        listen_addrs: &[Multiaddr],
        now: Instant,
    ) -> Vec<RelayAction> {
        if !protocols.iter().any(|p| p == RELAY_HOP_PROTOCOL) {
            return Vec::new();
        }
        // Already a candidate — configured, or discovered earlier. Nothing to
        // add, but this identify may be the one that makes a pending
        // reservation negotiable, so hand it to `on_relay_ready`.
        if self.configured.iter().any(|(p, _)| *p == peer) {
            return self.on_relay_ready(peer, now);
        }
        if let Some(entry) = self.discovered.iter_mut().find(|(p, _)| *p == peer) {
            // Refresh the stored address only once the peer stops listening on
            // it: it was recorded at first sighting, so a relay that came back
            // somewhere else (a reborn bootstrap) would otherwise be re-dialled
            // at the stale address forever. Conditional because
            // `with_push_listen_addr_updates` means a relay pushes identify
            // whenever its listen set *changes* — merely gaining an address —
            // and replacing unconditionally would move us off one that works.
            let stored_still_listed = listen_addrs.iter().any(|a| strip_p2p(a) == entry.1);
            if !stored_still_listed {
                if let Some(fresh) = listen_addrs
                    .iter()
                    .find(|a| is_relay_candidate_addr(a, self.strict))
                {
                    debug!(%peer, from = %entry.1, to = %fresh, "relay candidate moved");
                    entry.1 = strip_p2p(fresh);
                }
            }
            return self.on_relay_ready(peer, now);
        }
        // A relay we can only reach at a LAN address is no use to peers who are
        // not on that LAN — and a relay we can only reach *through another
        // relay* is no use to anyone, ourselves included. `is_relay_candidate_addr`
        // is the second point: `is_announceable` passes any circuit address
        // unconditionally, which is right for advertising our own reserved
        // address and wrong here. Accepting one produced a nested
        // `<their-circuit>/p2p/<them>/p2p-circuit` that `listen_on` rejected on
        // every retry — see `is_relay_candidate_addr`.
        let Some(addr) = listen_addrs
            .iter()
            .find(|a| is_relay_candidate_addr(a, self.strict))
        else {
            debug!(%peer, "peer offers relay hop but no directly-dialable address");
            return Vec::new();
        };

        debug!(%peer, %addr, "discovered a relay candidate");
        self.discovered.push((peer, strip_p2p(addr)));
        if self.discovered.len() > MAX_DISCOVERED {
            let dropped = self.discovered.remove(0);
            // Oldest out: a candidate we have never successfully used and have
            // had on the list longest is the cheapest thing to forget.
            debug!(peer = %dropped.0, "relay candidate list full; forgetting the oldest");
            // The cursor indexes the combined `configured ++ discovered` list;
            // removing `discovered[0]` only shifts entries at or beyond
            // `configured.len()`, so a cursor inside the configured prefix
            // must not move.
            if self.cursor > self.configured.len() {
                self.cursor -= 1;
            }
        }
        self.fill_slots(now)
    }

    /// The service called `listen_on` for [`RelayAction::Listen`] and got `id`.
    ///
    /// The slot exists from this moment, which is what makes a duplicate
    /// reservation on one relay impossible: candidate selection skips peers
    /// that hold a slot, and that check now sees this one.
    pub fn note_listener(&mut self, id: ListenerId, relay: PeerId, now: Instant) {
        self.slots.insert(
            id,
            Slot {
                relay,
                state: SlotState::Reserving,
                opened_at: now,
            },
        );
    }

    /// `listen_on` itself failed — the address was rejected before a listener
    /// existed, so there is no `ListenerId` and no slot to close.
    pub fn note_listen_failed(&mut self, relay: PeerId, now: Instant) -> Vec<RelayAction> {
        self.fail(relay, "listen_on refused the circuit address", now);
        self.fill_slots(now)
    }

    /// A new listen address appeared. If it belongs to a tracked reservation,
    /// the relay has accepted.
    ///
    /// Returns whether this confirmed a slot, so the caller knows the announce
    /// state changed.
    pub fn on_new_listen_addr(&mut self, id: ListenerId, addr: &Multiaddr) -> bool {
        let Some(slot) = self.slots.get_mut(&id) else {
            return false;
        };
        if matches!(slot.state, SlotState::Confirmed(_)) {
            return false;
        }
        info!(relay = %slot.relay, %addr, "relay reservation confirmed");
        slot.state = SlotState::Confirmed(addr.clone());
        true
    }

    /// A listener closed. If it was one of ours the reservation is gone,
    /// whatever the reason: `Err` is a refusal or timeout, `Ok` is the relay's
    /// connection dying under a reservation that was working.
    ///
    /// Returns the actions to take and whether a *confirmed* reservation was
    /// lost (an unconfirmed one never affected the announce state).
    pub fn on_listener_closed(
        &mut self,
        id: ListenerId,
        reason: Result<(), &str>,
        now: Instant,
    ) -> (Vec<RelayAction>, bool) {
        let Some(slot) = self.slots.remove(&id) else {
            return (Vec::new(), false);
        };
        let was_confirmed = matches!(slot.state, SlotState::Confirmed(_));

        // A reservation that held long enough to be real earns forgiveness: the
        // relay works, it just went away. Without this a relay that served us
        // for hours would inherit the backoff from its first bad day.
        if was_confirmed && now.duration_since(slot.opened_at) >= BACKOFF_RESET_AFTER {
            debug!(relay = %slot.relay, "long-lived reservation ended; failure count reset");
            self.backoff.remove(&slot.relay);
            // Still count this one, or a relay that flaps on a 61-second cycle
            // gets retried instantly forever.
            self.fail(slot.relay, "reservation ended", now);
        } else {
            let why = match reason {
                Ok(()) => "the relay connection closed",
                Err(e) => e,
            };
            self.fail(slot.relay, why, now);
        }

        (self.fill_slots(now), was_confirmed)
    }

    /// Periodic tick. Retries candidates whose backoff has expired and fills
    /// any slot that is short — this is what recovers a node whose every
    /// candidate was in backoff when the last attempt was made.
    pub fn on_tick(&mut self, now: Instant) -> Vec<RelayAction> {
        self.backoff.retain(|_, b| b.not_before > now);

        // A dial we never heard back about would otherwise hold its slot
        // forever. In practice the swarm always reports one way or the other,
        // but "in practice" is not a reason to leave a permanent slot leak in a
        // process expected to run for months.
        let stalled: Vec<PeerId> = self
            .pending
            .iter()
            .filter(|(_, started)| now.duration_since(**started) >= PENDING_DIAL_TIMEOUT)
            .map(|(peer, _)| *peer)
            .collect();
        for relay in stalled {
            self.pending.remove(&relay);
            self.fail(relay, "the dial to the relay never resolved", now);
        }

        self.fill_slots(now)
    }

    /// Record a failure and extend the relay's backoff.
    fn fail(&mut self, relay: PeerId, why: &str, now: Instant) {
        let entry = self.backoff.entry(relay).or_insert(Backoff {
            consecutive_failures: 0,
            not_before: now,
        });
        entry.consecutive_failures = entry.consecutive_failures.saturating_add(1);
        let delay = backoff_delay(entry.consecutive_failures);
        entry.not_before = now + delay;
        // Move on to a different candidate. Rotation is independent of backoff:
        // refusal should advance us, not stall us on the same relay.
        self.cursor = self.cursor.wrapping_add(1);
        info!(
            %relay, why, failures = entry.consecutive_failures, retry_in = ?delay,
            "relay reservation failed"
        );
    }

    /// `relay` is connected **and** identify has confirmed it speaks hop. Ask
    /// for the reservation now.
    ///
    /// Both halves of that condition are load-bearing, and getting either wrong
    /// costs a full backoff cycle:
    ///
    /// - `listen_on` for an *unconnected* relay does not wait for the dial.
    ///   libp2p asks immediately, finds no connection, and closes the listener
    ///   with "Failed to get Reservation".
    /// - `listen_on` for a connected relay whose identify has not yet arrived
    ///   fails the same way. Until identify lands, the connection's supported
    ///   protocol set does not include hop, so the reservation cannot be
    ///   negotiated. This is why the trigger is the identify event and not
    ///   `ConnectionEstablished` — the difference is invisible against a fast
    ///   relay and a 30-second stall against a slow one.
    pub fn on_relay_ready(&mut self, relay: PeerId, now: Instant) -> Vec<RelayAction> {
        if !self.enabled || self.pending.remove(&relay).is_none() {
            return Vec::new();
        }
        let Some((_, relay_addr)) = self.candidate_addr(&relay) else {
            // The candidate was evicted from the discovered list while its dial
            // was in flight. The pending entry is already consumed; without a
            // fail() here the cursor and backoff never advance and the freed
            // slot waits for the next tick instead of refilling now.
            self.fail(relay, "the relay was evicted from the candidate list", now);
            return self.fill_slots(now);
        };
        let Some(circuit_addr) = circuit_listen_addr(&relay_addr, relay) else {
            // Belt and braces: `note_identify` keeps circuit-addressed peers off
            // the candidate list, so arriving here means one slipped through.
            // Fail it properly rather than handing `listen_on` an address it
            // rejects on every retry, forever, while the slot stays occupied.
            self.fail(
                relay,
                "the relay is itself only reachable through a circuit",
                now,
            );
            return self.fill_slots(now);
        };
        debug!(%relay, %circuit_addr, "relay connected; requesting the reservation");
        vec![RelayAction::Listen {
            relay,
            circuit_addr,
        }]
    }

    /// The dial to `relay` failed, so the reservation we were waiting on will
    /// never be requested. Count it against the relay and move on.
    pub fn on_relay_dial_failed(&mut self, relay: PeerId, now: Instant) -> Vec<RelayAction> {
        if self.pending.remove(&relay).is_none() {
            return Vec::new();
        }
        self.fail(relay, "the dial to the relay failed", now);
        self.fill_slots(now)
    }

    /// Dial candidates until every slot is spoken for or we run out.
    fn fill_slots(&mut self, now: Instant) -> Vec<RelayAction> {
        if !self.enabled {
            return Vec::new();
        }
        let mut actions = Vec::new();
        // Slots being filled — dialing or reserving — count towards the cap, so
        // a slow relay does not cause us to over-reserve while we wait for it.
        while self.slots.len() + self.pending.len() < self.max_slots {
            let Some((relay, relay_addr)) = self.pick_candidate(now) else {
                break;
            };
            self.pending.insert(relay, now);
            debug!(%relay, %relay_addr, "dialing a relay candidate");
            actions.push(RelayAction::Dial { relay, relay_addr });
        }
        actions
    }

    /// The stored address for a candidate, configured or discovered.
    fn candidate_addr(&self, peer: &PeerId) -> Option<(PeerId, Multiaddr)> {
        self.configured
            .iter()
            .chain(self.discovered.iter())
            .find(|(p, _)| p == peer)
            .cloned()
    }

    /// The next candidate not already in a slot, not being dialed, and not in
    /// backoff. Walks the whole list once from the cursor, so rotation is fair
    /// rather than always retrying the head.
    fn pick_candidate(&mut self, now: Instant) -> Option<(PeerId, Multiaddr)> {
        let candidates: Vec<(PeerId, Multiaddr)> = self
            .configured
            .iter()
            .chain(self.discovered.iter())
            .cloned()
            .collect();
        if candidates.is_empty() {
            return None;
        }

        for offset in 0..candidates.len() {
            let index = (self.cursor + offset) % candidates.len();
            let (peer, addr) = &candidates[index];

            if self.slots.values().any(|slot| slot.relay == *peer) {
                continue;
            }
            if self.pending.contains_key(peer) {
                continue;
            }
            if let Some(b) = self.backoff.get(peer) {
                if b.not_before > now {
                    continue;
                }
            }

            self.cursor = index + 1;
            return Some((*peer, addr.clone()));
        }
        None
    }
}

/// `30s · 2^(n−1)`, capped at 15 minutes, ±20%.
///
/// The jitter matters more than the curve: without it a fleet that restarted
/// together would re-converge on the same relay at the same instant every time,
/// which is precisely the load spike backoff exists to avoid.
fn backoff_delay(consecutive_failures: u32) -> Duration {
    let exponent = consecutive_failures.saturating_sub(1).min(16);
    let base = BACKOFF_BASE
        .saturating_mul(1u32 << exponent)
        .min(BACKOFF_MAX);
    jitter(base)
}

/// ±20% of `d`, from a cheap uncorrelated-per-call source. This does not need
/// to be unpredictable, only uncorrelated between nodes.
fn jitter(d: Duration) -> Duration {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    let mut hasher = RandomState::new().build_hasher();
    hasher.write_u64(d.as_millis() as u64);
    // 0..=40 → −20%..=+20%
    let percent = (hasher.finish() % 41) as i64 - 20;
    let millis = d.as_millis() as i64;
    let adjusted = millis + (millis * percent / 100);
    Duration::from_millis(adjusted.max(1) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(n: u8) -> PeerId {
        let mut bytes = [0u8; 32];
        bytes[0] = n;
        libp2p::identity::Keypair::ed25519_from_bytes(bytes)
            .unwrap()
            .public()
            .to_peer_id()
    }

    fn relay_entry(n: u8) -> String {
        format!("/ip4/198.51.100.{n}/tcp/8080/p2p/{}", peer(n))
    }

    fn listener(n: u64) -> ListenerId {
        // ListenerId has no public constructor; each `new` yields a fresh one,
        // which is all the tests need — distinct ids that behave like real ones.
        let _ = n;
        ListenerId::next()
    }

    fn enabled(trusted: &[String], max: usize) -> (RelayManager, Instant) {
        let now = Instant::now();
        let mut mgr = RelayManager::new(trusted, max, false);
        mgr.set_enabled(true, now);
        (mgr, now)
    }

    /// The relay a `Dial` action targets.
    fn dial_target(action: &RelayAction) -> PeerId {
        match action {
            RelayAction::Dial { relay, .. } => *relay,
            other => panic!("expected a Dial, got {other:?}"),
        }
    }

    /// Walk a candidate through dial → connected → listening, as the service
    /// does. Returns the ListenerId the reservation is tracked under.
    fn connect_and_listen(mgr: &mut RelayManager, relay: PeerId, now: Instant) -> ListenerId {
        let actions = mgr.on_relay_ready(relay, now);
        assert_eq!(actions.len(), 1, "connecting should request a reservation");
        let id = ListenerId::next();
        mgr.note_listener(id, relay, now);
        id
    }

    #[test]
    fn no_reservations_are_sought_while_disabled() {
        let mgr = RelayManager::new(&[relay_entry(1)], 2, false);
        assert!(!mgr.is_enabled());
        // A publicly reachable node holding circuits costs a relay real
        // resources for nothing.
        assert!(mgr.confirmed_addrs().is_empty());
    }

    #[test]
    fn enabling_requests_up_to_max_slots() {
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2), relay_entry(3)], 2);
        // `set_enabled` already returned the actions; re-enabling is a no-op.
        assert!(mgr.set_enabled(true, now).is_empty());

        let mut mgr2 =
            RelayManager::new(&[relay_entry(1), relay_entry(2), relay_entry(3)], 2, false);
        let actions = mgr2.set_enabled(true, now);
        assert_eq!(actions.len(), 2, "max_slots is 2, not 3: {actions:?}");
    }

    #[test]
    fn only_one_reservation_per_relay_even_when_listed_twice() {
        // A duplicated trusted_relays entry must not consume both slots on one
        // relay — that is two circuits with one point of failure.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[relay_entry(1), relay_entry(1)], 2, false);
        let actions = mgr.set_enabled(true, now);
        assert_eq!(actions.len(), 1, "one relay, one reservation: {actions:?}");
    }

    #[test]
    fn a_slot_is_taken_from_the_moment_listen_on_is_called() {
        // Not from the moment the relay says yes — otherwise a slow relay would
        // let us request a second reservation from it.
        let (mut mgr, now) = enabled(&[relay_entry(1)], 2);
        let id = listener(1);
        mgr.note_listener(id, peer(1), now);
        assert!(
            mgr.on_tick(now).is_empty(),
            "the only relay already holds a slot"
        );
        assert!(!mgr.has_circuit(), "reserving is not yet confirmed");
    }

    #[test]
    fn confirmation_makes_the_circuit_address_available() {
        let (mut mgr, now) = enabled(&[relay_entry(1)], 1);
        let id = listener(1);
        mgr.note_listener(id, peer(1), now);

        let circuit: Multiaddr = format!("/ip4/198.51.100.1/tcp/8080/p2p/{}/p2p-circuit", peer(1))
            .parse()
            .unwrap();
        assert!(mgr.on_new_listen_addr(id, &circuit));
        assert!(mgr.has_circuit());
        assert_eq!(mgr.confirmed_addrs(), vec![circuit.clone()]);
        // Idempotent: a repeated address must not re-report a change.
        assert!(!mgr.on_new_listen_addr(id, &circuit));
    }

    #[test]
    fn an_untracked_listener_is_ignored() {
        // The node's own TCP listeners produce these constantly.
        let (mut mgr, now) = enabled(&[relay_entry(1)], 1);
        let stranger = listener(99);
        assert!(!mgr.on_new_listen_addr(stranger, &"/ip4/127.0.0.1/tcp/1".parse().unwrap()));
        let (actions, lost) = mgr.on_listener_closed(stranger, Ok(()), now);
        assert!(actions.is_empty() && !lost);
    }

    #[test]
    fn losing_a_reservation_reports_it_and_seeks_a_replacement() {
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 1);
        let id = connect_and_listen(&mut mgr, peer(1), now);
        let circuit: Multiaddr = format!("/ip4/198.51.100.1/tcp/8080/p2p/{}/p2p-circuit", peer(1))
            .parse()
            .unwrap();
        mgr.on_new_listen_addr(id, &circuit);

        // The relay's connection died under a working reservation: Ok, not Err.
        let (actions, lost) = mgr.on_listener_closed(id, Ok(()), now);
        assert!(lost, "a confirmed reservation was lost");
        assert!(!mgr.has_circuit());
        assert_eq!(actions.len(), 1, "replacement sought: {actions:?}");
        // …and from the *other* relay: rotation moves us on.
        assert_eq!(dial_target(&actions[0]), peer(2));
    }

    #[test]
    fn losing_an_unconfirmed_reservation_is_not_an_announce_change() {
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 1);
        let id = listener(1);
        mgr.note_listener(id, peer(1), now);
        // Refused before it was ever confirmed — nobody could reach us through
        // it, so the announce state never depended on it.
        let (_, lost) = mgr.on_listener_closed(id, Err("RESERVATION_REFUSED"), now);
        assert!(!lost);
    }

    #[test]
    fn refusal_rotates_to_the_next_candidate() {
        // The production bootstraps advertise relay hop and have a documented
        // refusal history, so this is the common path, not the exotic one.
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 1);
        let first = connect_and_listen(&mut mgr, peer(1), now);

        let (actions, _) = mgr.on_listener_closed(first, Err("RESERVATION_REFUSED"), now);
        assert_eq!(actions.len(), 1);
        assert_eq!(
            dial_target(&actions[0]),
            peer(2),
            "a refusal must move us on, not retry the same relay"
        );
    }

    #[test]
    fn a_failed_relay_is_not_retried_until_its_backoff_expires() {
        let (mut mgr, now) = enabled(&[relay_entry(1)], 1);
        let id = connect_and_listen(&mut mgr, peer(1), now);
        mgr.on_listener_closed(id, Err("refused"), now);

        // Only candidate, and it is in backoff: idle rather than hammering it.
        assert!(mgr.on_tick(now).is_empty());
        assert!(mgr.on_tick(now + Duration::from_secs(10)).is_empty());
        // Past the maximum jittered first backoff (30s + 20%).
        assert_eq!(mgr.on_tick(now + Duration::from_secs(37)).len(), 1);
    }

    #[test]
    fn a_dial_that_never_resolves_gives_its_slot_back() {
        // If the swarm reported neither success nor failure, the slot would be
        // held forever — a permanent leak in a process expected to run for
        // months.
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 1);
        assert!(mgr.on_tick(now + Duration::from_secs(30)).is_empty());

        let actions = mgr.on_tick(now + PENDING_DIAL_TIMEOUT);
        assert_eq!(actions.len(), 1, "the slot is reclaimed: {actions:?}");
        assert_eq!(
            dial_target(&actions[0]),
            peer(2),
            "and rotation moves on to the other candidate"
        );
    }

    #[test]
    fn a_failed_dial_frees_the_slot_immediately() {
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 1);
        let actions = mgr.on_relay_dial_failed(peer(1), now);
        assert_eq!(actions.len(), 1);
        assert_eq!(dial_target(&actions[0]), peer(2));
    }

    #[test]
    fn identify_from_an_unrelated_peer_asks_for_nothing() {
        // Every connection the node makes flows through here.
        let (mut mgr, _now) = enabled(&[relay_entry(1)], 1);
        assert!(mgr.on_relay_ready(peer(200), _now).is_empty());
    }

    #[test]
    fn backoff_grows_and_is_capped() {
        assert!(backoff_delay(1) <= Duration::from_secs(36));
        assert!(backoff_delay(1) >= Duration::from_secs(24));
        assert!(backoff_delay(3) > backoff_delay(1));
        // The cap holds however many times we double, including absurdly.
        assert!(backoff_delay(64) <= Duration::from_secs(15 * 60 * 12 / 10));
    }

    #[test]
    fn a_long_lived_reservation_forgives_the_relays_history() {
        // A relay that served us for an hour and then restarted should not
        // inherit the backoff from whatever went wrong when we first met it.
        // Only that one relay, so rotation cannot hide the effect.
        let (mut mgr, now) = enabled(&[relay_entry(1)], 1);

        // Two early failures: without forgiveness the next backoff would be the
        // third step, ~2 minutes.
        // Two early failures: without forgiveness the next backoff would be the
        // third step, ~2 minutes. The first goes through the normal
        // dial-then-reserve path so the pending entry is consumed; the second
        // is driven directly, because by then the relay is in backoff and
        // `fill_slots` would not offer it again yet.
        let bad = connect_and_listen(&mut mgr, peer(1), now);
        mgr.on_listener_closed(bad, Err("refused"), now);
        let worse = ListenerId::next();
        mgr.note_listener(worse, peer(1), now);
        mgr.on_listener_closed(worse, Err("refused"), now);
        assert!(
            mgr.on_tick(now + Duration::from_secs(37)).is_empty(),
            "two failures should back off well past 37s"
        );

        // Now a reservation that actually held for an hour.
        let good = ListenerId::next();
        mgr.note_listener(good, peer(1), now);
        let circuit: Multiaddr = format!("/ip4/198.51.100.1/tcp/8080/p2p/{}/p2p-circuit", peer(1))
            .parse()
            .unwrap();
        mgr.on_new_listen_addr(good, &circuit);

        let later = now + Duration::from_secs(3600);
        mgr.on_listener_closed(good, Ok(()), later);
        // One failure's worth of backoff, not three: retried within ~37s rather
        // than ~2 minutes.
        assert!(mgr
            .on_tick(later + Duration::from_secs(37))
            .iter()
            .any(|a| matches!(a, RelayAction::Dial { relay, .. } if *relay == peer(1))));
    }

    #[test]
    fn going_public_releases_every_reservation() {
        let (mut mgr, now) = enabled(&[relay_entry(1), relay_entry(2)], 2);
        let a = listener(1);
        let b = listener(2);
        mgr.note_listener(a, peer(1), now);
        mgr.note_listener(b, peer(2), now);

        let actions = mgr.set_enabled(false, now);
        assert_eq!(actions.len(), 2, "both listeners torn down: {actions:?}");
        assert!(actions
            .iter()
            .all(|a| matches!(a, RelayAction::StopListening(_))));
        assert!(!mgr.has_circuit());
        // And nothing is sought while public.
        assert!(mgr.on_tick(now).is_empty());
    }

    // -- identify discovery ---------------------------------------------

    #[test]
    fn identify_supplies_candidates_when_nothing_is_configured() {
        // The real supply on the live network: `trusted_relays` defaults empty,
        // and the bootstraps advertise hop.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        assert!(mgr.set_enabled(true, now).is_empty(), "no candidates yet");

        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.7/tcp/8080".parse().unwrap()],
            now,
        );
        assert_eq!(actions.len(), 1);
        assert_eq!(dial_target(&actions[0]), peer(7));

        // …and connecting to it is what produces the reservation request.
        let listen = mgr.on_relay_ready(peer(7), now);
        match &listen[0] {
            RelayAction::Listen { circuit_addr, .. } => {
                assert!(circuit_addr.to_string().ends_with("/p2p-circuit"))
            }
            other => panic!("expected a Listen, got {other:?}"),
        }
    }

    #[test]
    fn a_discovered_relay_that_moved_is_redialled_at_its_new_address() {
        // Regression: the discovered list froze a candidate's address at first
        // sighting. A relay that came back somewhere else — a reborn bootstrap
        // after a fleet migration — was re-dialled at the stale address on
        // every backoff expiry, forever.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.7/tcp/8080".parse().unwrap()],
            now,
        );
        assert_eq!(dial_target(&actions[0]), peer(7));

        // The dial fails (the relay is restarting elsewhere)…
        mgr.on_relay_dial_failed(peer(7), now);
        // …and a later identify shows the same peer at a new address.
        let refresh = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.8/tcp/9090".parse().unwrap()],
            now,
        );
        assert!(refresh.is_empty(), "not pending, so nothing to do yet");

        // Once the backoff lapses, the retry must go where the relay is now.
        let actions = mgr.on_tick(now + Duration::from_secs(120));
        match &actions[..] {
            [RelayAction::Dial { relay, relay_addr }] => {
                assert_eq!(*relay, peer(7));
                assert_eq!(relay_addr.to_string(), "/ip4/198.51.100.8/tcp/9090");
            }
            other => panic!("expected one Dial, got {other:?}"),
        }
    }

    #[test]
    fn a_discovered_relay_that_only_gained_an_address_is_not_moved() {
        // The other half of the refresh: `with_push_listen_addr_updates` means
        // a relay pushes identify whenever its listen set changes, so a relay
        // we are reaching perfectly well announces itself again the moment it
        // adds an address. Taking the first candidate unconditionally would
        // walk us off the address that works onto whichever one sorts first.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.7/tcp/8080".parse().unwrap()],
            now,
        );
        assert_eq!(dial_target(&actions[0]), peer(7));
        mgr.on_relay_dial_failed(peer(7), now);

        // Still listening where we know it, plus a new address listed first.
        mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &[
                "/ip4/198.51.100.9/tcp/9999".parse().unwrap(),
                "/ip4/198.51.100.7/tcp/8080".parse().unwrap(),
            ],
            now,
        );

        let actions = mgr.on_tick(now + Duration::from_secs(120));
        match &actions[..] {
            [RelayAction::Dial { relay, relay_addr }] => {
                assert_eq!(*relay, peer(7));
                assert_eq!(relay_addr.to_string(), "/ip4/198.51.100.7/tcp/8080");
            }
            other => panic!("expected one Dial, got {other:?}"),
        }
    }

    #[test]
    fn a_peer_that_does_not_offer_hop_is_not_a_candidate() {
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &["/ipfs/kad/1.0.0".to_string()],
            &["/ip4/198.51.100.7/tcp/8080".parse().unwrap()],
            now,
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn a_relay_that_is_itself_relay_only_is_not_a_candidate() {
        // Regression: metro-win, 2026-08-11. `is_announceable` passes any
        // circuit address unconditionally — right for advertising our own
        // reserved address, wrong for choosing a relay. Accepting one built
        // `<their-circuit>/p2p/<them>/p2p-circuit`, which `listen_on` rejects on
        // every retry, so the node never regained a circuit after its one good
        // reservation lapsed.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &[
                format!("/ip4/18.219.43.67/tcp/8000/p2p-circuit/p2p/{}", peer(7))
                    .parse()
                    .unwrap(),
            ],
            now,
        );
        assert!(
            actions.is_empty(),
            "a peer reachable only through another relay cannot host our reservation"
        );
    }

    #[test]
    fn a_directly_dialable_relay_is_still_a_candidate() {
        // The guard above must not cost us ordinary relays.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.7/tcp/8080".parse().unwrap()],
            now,
        );
        assert!(
            !actions.is_empty(),
            "a directly-dialable relay must still be picked up"
        );
    }

    #[test]
    fn a_relay_reachable_only_on_a_lan_is_not_a_candidate() {
        // No use to peers who are not on that LAN.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[], 1, false);
        mgr.set_enabled(true, now);
        let actions = mgr.note_identify(
            peer(7),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &[
                "/ip4/192.168.1.7/tcp/8080".parse().unwrap(),
                "/ip4/127.0.0.1/tcp/8080".parse().unwrap(),
            ],
            now,
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn a_configured_relay_is_not_re_added_by_identify() {
        let (mut mgr, now) = enabled(&[relay_entry(1)], 2);
        // The identify is not a *discovery* — the relay is already configured —
        // but it is the signal that the pending reservation can now be
        // requested, so a Listen is exactly right.
        let actions = mgr.note_identify(
            peer(1),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.1/tcp/8080".parse().unwrap()],
            now,
        );
        assert!(
            matches!(actions.as_slice(), [RelayAction::Listen { relay, .. }] if *relay == peer(1)),
            "{actions:?}"
        );
        assert_eq!(mgr.discovered.len(), 0, "not added a second time");

        // A second identify from the same relay adds nothing: the slot is taken.
        let again = mgr.note_identify(
            peer(1),
            &[RELAY_HOP_PROTOCOL.to_string()],
            &["/ip4/198.51.100.1/tcp/8080".parse().unwrap()],
            now,
        );
        assert!(again.is_empty(), "{again:?}");
    }

    #[test]
    fn the_discovered_list_is_bounded() {
        let now = Instant::now();
        // max_slots 1 so discovery does not immediately consume candidates.
        let mut mgr = RelayManager::new(&[], 1, false);
        for n in 1..=(MAX_DISCOVERED as u8 + 5) {
            mgr.note_identify(
                peer(n),
                &[RELAY_HOP_PROTOCOL.to_string()],
                &[format!("/ip4/198.51.100.{n}/tcp/8080").parse().unwrap()],
                now,
            );
        }
        assert_eq!(
            mgr.discovered.len(),
            MAX_DISCOVERED,
            "an unbounded candidate list is a slow leak on a busy node"
        );
    }

    // -- config hygiene ---------------------------------------------------

    #[test]
    fn unusable_configured_relays_are_dropped_not_fatal() {
        // A typo in one relay must not stop the other from working.
        let mgr = RelayManager::new(
            &[
                "not-a-multiaddr".to_string(),
                "/ip4/198.51.100.9/tcp/8080".to_string(), // no /p2p component
                relay_entry(1),
            ],
            2,
            false,
        );
        assert_eq!(mgr.configured.len(), 1);
        assert_eq!(mgr.configured[0].0, peer(1));
        // …and the peer id is stripped from the stored address, so building the
        // circuit address cannot double it.
        assert_eq!(
            mgr.configured[0].1.to_string(),
            "/ip4/198.51.100.1/tcp/8080"
        );
    }

    #[test]
    fn zero_max_slots_is_read_as_one() {
        // Silently disabling relaying is never what a config meant.
        let now = Instant::now();
        let mut mgr = RelayManager::new(&[relay_entry(1)], 0, false);
        assert_eq!(mgr.set_enabled(true, now).len(), 1);
    }
}
