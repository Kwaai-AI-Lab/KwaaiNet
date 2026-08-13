//! Multiaddr classification: which addresses are worth telling the world about.
//!
//! One question runs through the whole NAT slice — *is this address any use to
//! a peer that is not us?* The reachability state machine asks it of every
//! identify-observed address before treating it as evidence; the relay manager
//! asks it of a candidate relay's listen addresses before dialing one.
//!
//! The rules are a port of the p2pd path's `is_announceable_addr` /
//! `is_globally_routable_v4` (`kwaai-cli::node`), kept deliberately faithful
//! rather than "improved", because they encode two decisions that the obvious
//! implementation gets wrong:
//!
//! 1. **A circuit address is always announceable.** `/…/p2p-circuit` says
//!    "reach me through this relay", and the relay's own address is the part
//!    that has to be routable. Classifying the circuit by the IP in front of it
//!    would reject every reservation held on a relay we reached over a LAN.
//! 2. **The reserved-for-documentation and benchmarking ranges are accepted.**
//!    RFC5737 (`192.0.2/24`, `198.51.100/24`, `203.0.113/24`) and RFC2544
//!    (`198.18/15`) are reserved by IANA but they are not LAN-private, and the
//!    docker nat-test topology is built on them — `198.18.0.20` is a relay node
//!    there. Treating them as unroutable would make the entire test bed
//!    classify itself unreachable. `is_routable_v4` therefore permits them, and
//!    the golden test at the bottom of this file pins that.
//!
//! rust-libp2p 0.53 does its own filtering in exactly one place that matters:
//! `autonat::Config::only_global_ips` (default `true`) rejects RFC2544 via its
//! `is_benchmarking` check. That is why [`crate::config::NetworkConfig`] carries
//! `require_global_ips` and defaults it to **false** — kad, identify, dcutr and
//! the swarm itself do no address-class filtering whatsoever, so AutoNAT's knob
//! is the only lever, and it has to be off for the test bed to work.

use std::net::Ipv4Addr;

use libp2p::{multiaddr::Protocol, Multiaddr, PeerId};

/// Whether `addr` is worth advertising to other peers.
///
/// True for any address carrying a `/p2p-circuit` segment (rule 1 above), and
/// for direct addresses whose every IP component is routable. An address that
/// mixes a routable and an unroutable IP is rejected: the unroutable one is
/// almost always a local interface that leaked into the list.
pub fn is_announceable(addr: &Multiaddr) -> bool {
    let mut has_circuit = false;
    let mut routable_ip = false;
    let mut bad_ip = false;

    for proto in addr.iter() {
        match proto {
            Protocol::P2pCircuit => has_circuit = true,
            Protocol::Ip4(a) => {
                if is_routable_v4(a) {
                    routable_ip = true;
                } else {
                    bad_ip = true;
                }
            }
            Protocol::Ip6(a) => {
                if a.is_unspecified() || a.is_loopback() {
                    bad_ip = true;
                } else {
                    routable_ip = true;
                }
            }
            _ => {}
        }
    }

    if has_circuit {
        return true;
    }
    routable_ip && !bad_ip
}

/// Whether `a` is plausibly reachable from outside the local network.
///
/// Rejects unspecified, loopback, link-local, broadcast, multicast, the RFC1918
/// private ranges and RFC6598 carrier-grade NAT (`100.64/10`).
///
/// **Accepts** the RFC5737 documentation ranges and RFC2544 benchmarking range
/// — see the module docs for why that is deliberate and not an oversight.
pub fn is_routable_v4(a: Ipv4Addr) -> bool {
    if a.is_unspecified()
        || a.is_loopback()
        || a.is_link_local()
        || a.is_broadcast()
        || a.is_multicast()
        || a.is_private()
    {
        return false;
    }
    // RFC6598 carrier-grade NAT: 100.64.0.0/10. A node behind CGNAT cannot be
    // dialed at this address any more than it can at an RFC1918 one.
    let [b0, b1, ..] = a.octets();
    if b0 == 100 && (64..=127).contains(&b1) {
        return false;
    }
    true
}

/// The stricter classification used when `require_global_ips` is set.
///
/// Adds the reserved ranges [`is_routable_v4`] deliberately permits back onto
/// the reject list, matching what `autonat::Config::only_global_ips` enforces
/// internally. An operator turns this on when the node is on the real internet
/// and a documentation-range address in the announce set would only ever be a
/// misconfiguration.
pub fn is_globally_routable_v4(a: Ipv4Addr) -> bool {
    if !is_routable_v4(a) {
        return false;
    }
    let [b0, b1, b2, _] = a.octets();
    // RFC2544 benchmarking: 198.18.0.0/15.
    if b0 == 198 && (b1 == 18 || b1 == 19) {
        return false;
    }
    // RFC5737 documentation: 192.0.2/24, 198.51.100/24, 203.0.113/24.
    if (b0 == 192 && b1 == 0 && b2 == 2)
        || (b0 == 198 && b1 == 51 && b2 == 100)
        || (b0 == 203 && b1 == 0 && b2 == 113)
    {
        return false;
    }
    true
}

/// [`is_announceable`], with the reserved ranges rejected when `strict`.
///
/// The one entry point callers should use, so the `require_global_ips` decision
/// lives in one place rather than being re-derived at each call site.
pub fn is_announceable_with(addr: &Multiaddr, strict: bool) -> bool {
    if !is_announceable(addr) {
        return false;
    }
    if !strict {
        return true;
    }
    // A circuit address is still announceable under `strict` — what has to be
    // routable is the relay's address, and we reached the relay to get here.
    if is_circuit(addr) {
        return true;
    }
    addr.iter().all(|p| match p {
        Protocol::Ip4(a) => is_globally_routable_v4(a),
        _ => true,
    })
}

/// Whether `addr` routes through a circuit relay.
pub fn is_circuit(addr: &Multiaddr) -> bool {
    addr.iter().any(|p| matches!(p, Protocol::P2pCircuit))
}

/// Whether a peer reachable at `addr` can serve as *our* relay.
///
/// Deliberately **not** [`is_announceable`], which answers a different question.
/// That one says "is this address worth telling the world about" and returns
/// true for any circuit address unconditionally (rule 1 in the module docs) —
/// correct for advertising our own reserved address, wrong for picking a relay.
///
/// A relay has to be **directly dialable**. A peer that is itself only reachable
/// through someone else's circuit cannot host our reservation: asking for one
/// builds `<their-circuit>/p2p/<them>/p2p-circuit`, a doubly-nested address that
/// `Swarm::listen_on` rejects outright. Every reservation attempt against such a
/// candidate fails, forever, on backoff.
///
/// Observed on metro-win 2026-08-11: its one good reservation lapsed, the relay
/// manager rotated onto identify-discovered candidates that were themselves
/// relay-only, and it never obtained another circuit — 2350 `listen_on refused`
/// failures across ~12 candidates, while the node still reported healthy because
/// DHT re-announce kept succeeding.
pub fn is_relay_candidate_addr(addr: &Multiaddr) -> bool {
    is_announceable(addr) && !is_circuit(addr)
}

/// Drop a trailing `/p2p/<peer-id>` component.
///
/// Kademlia stores addresses without it and re-attaches it itself, so anything
/// handed to `kad::Behaviour::add_address` has to come through here first.
pub fn strip_p2p(addr: &Multiaddr) -> Multiaddr {
    addr.iter()
        .filter(|p| !matches!(p, Protocol::P2p(_)))
        .collect()
}

/// Drop the *destination* `/p2p/<peer-id>` while keeping a circuit's relay hop.
///
/// [`strip_p2p`] removes every `/p2p` component. That is right for a direct
/// address, where the only one names the peer the caller re-attaches anyway,
/// and wrong for a circuit address, where there are two with different jobs:
///
/// ```text
/// /ip4/<relay-ip>/tcp/<port>/p2p/<relay>/p2p-circuit/p2p/<dest>
///                           ^^^^^^^^^^^^             ^^^^^^^^^^
///                           which relay to           who to reach
///                           dial *through*           through it
/// ```
///
/// Stripping the relay hop leaves `/ip4/…/tcp/…/p2p-circuit/p2p/<dest>`, which
/// rust-libp2p refuses to dial outright — `Missing relay peer id.` — because a
/// circuit dial has to name the relay to ask. go-libp2p accepts the shortened
/// form (it learned the address *from* that relay, so the identity is implicit),
/// which is why the p2pd path reached relay-only peers that the native path
/// could not. Observed live against metro-win, 2026-08-10.
///
/// Only the component *after* `/p2p-circuit` is the destination, so only that
/// one is safe to drop.
pub fn strip_dest_p2p(addr: &Multiaddr) -> Multiaddr {
    if !is_circuit(addr) {
        return strip_p2p(addr);
    }
    let mut past_circuit = false;
    addr.iter()
        .filter(|p| {
            if matches!(p, Protocol::P2pCircuit) {
                past_circuit = true;
                return true;
            }
            !(past_circuit && matches!(p, Protocol::P2p(_)))
        })
        .collect()
}

/// Extract the `/p2p/<peer-id>` component from a multiaddr, if present.
///
/// Returns the **first** one. On a circuit address that is the *relay*, not the
/// destination — use [`dest_peer_id`] when you want "who does this address
/// reach".
pub fn peer_id_from_multiaddr(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| match p {
        Protocol::P2p(peer) => Some(peer),
        _ => None,
    })
}

/// The peer this address *reaches*, as opposed to the one it routes through.
///
/// For a direct address that is the only `/p2p` component. For a circuit it is
/// the one after `/p2p-circuit`; the one before names the relay. Returns `None`
/// for a circuit address that names a relay but no destination.
pub fn dest_peer_id(addr: &Multiaddr) -> Option<PeerId> {
    if !is_circuit(addr) {
        return peer_id_from_multiaddr(addr);
    }
    let mut past_circuit = false;
    addr.iter().find_map(|p| match p {
        Protocol::P2pCircuit => {
            past_circuit = true;
            None
        }
        Protocol::P2p(peer) if past_circuit => Some(peer),
        _ => None,
    })
}

/// The circuit-listen address for a reservation on `relay` at `relay_addr`:
/// `<relay_addr>/p2p/<relay>/p2p-circuit`.
///
/// This is what `Swarm::listen_on` is given to request a reservation. The relay
/// address must *not* already carry a `/p2p` component, or the peer id ends up
/// in the address twice and the dial resolves to nothing.
pub fn circuit_listen_addr(relay_addr: &Multiaddr, relay: PeerId) -> Option<Multiaddr> {
    // A relay reached through another relay cannot host a reservation, and
    // appending to its circuit address silently produces
    // `<their-circuit>/p2p/<them>/p2p-circuit` — nested, undialable, and
    // rejected by `listen_on` on every retry. Refuse to build it at all;
    // `is_relay_candidate_addr` should have filtered this out upstream, and a
    // `None` here means it did not.
    if is_circuit(relay_addr) {
        return None;
    }
    Some(
        strip_p2p(relay_addr)
            .with(Protocol::P2p(relay))
            .with(Protocol::P2pCircuit),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ma(s: &str) -> Multiaddr {
        s.parse().expect("test multiaddr should parse")
    }

    fn v4(s: &str) -> Ipv4Addr {
        s.parse().expect("test ipv4 should parse")
    }

    // -- the golden case ------------------------------------------------

    #[test]
    fn rfc2544_benchmarking_addresses_are_routable() {
        // 198.18.0.0/15 is the docker nat-test topology's "public" network;
        // 198.18.0.20 is node-a, the trusted relay every NATed node in that bed
        // reserves against. If this ever flips to false, every node in the test
        // bed classifies itself unreachable and the whole topology goes dark.
        assert!(is_routable_v4(v4("198.18.0.20")));
        assert!(is_routable_v4(v4("198.19.255.254")));
        assert!(is_announceable(&ma("/ip4/198.18.0.20/tcp/8080")));

        // …and `require_global_ips` is precisely the switch that rejects them.
        assert!(!is_globally_routable_v4(v4("198.18.0.20")));
        assert!(!is_announceable_with(
            &ma("/ip4/198.18.0.20/tcp/8080"),
            true
        ));
    }

    #[test]
    fn rfc5737_documentation_ranges_are_routable() {
        for addr in ["192.0.2.1", "198.51.100.7", "203.0.113.9"] {
            assert!(is_routable_v4(v4(addr)), "{addr} should be routable");
            assert!(
                !is_globally_routable_v4(v4(addr)),
                "{addr} should fail the strict check"
            );
        }
    }

    // -- the reject list ------------------------------------------------

    #[test]
    fn private_and_local_ranges_are_not_routable() {
        for addr in [
            "0.0.0.0",         // unspecified
            "127.0.0.1",       // loopback
            "169.254.1.1",     // link-local
            "255.255.255.255", // broadcast
            "224.0.0.1",       // multicast
            "10.0.0.1",        // RFC1918
            "172.16.0.1",      // RFC1918
            "192.168.1.10",    // RFC1918
            "100.64.0.1",      // RFC6598 CGNAT, low edge
            "100.127.255.254", // RFC6598 CGNAT, high edge
        ] {
            assert!(!is_routable_v4(v4(addr)), "{addr} should not be routable");
        }
    }

    #[test]
    fn cgnat_boundaries_are_exact() {
        // 100.64/10 is 100.64.0.0 – 100.127.255.255. One octet either side of
        // that window is ordinary public space.
        assert!(is_routable_v4(v4("100.63.255.255")));
        assert!(!is_routable_v4(v4("100.64.0.0")));
        assert!(!is_routable_v4(v4("100.127.255.255")));
        assert!(is_routable_v4(v4("100.128.0.0")));
    }

    #[test]
    fn ordinary_public_addresses_pass_both_checks() {
        for addr in ["18.219.43.67", "52.23.252.2", "8.8.8.8"] {
            assert!(is_routable_v4(v4(addr)));
            assert!(is_globally_routable_v4(v4(addr)));
        }
    }

    // -- multiaddr level ------------------------------------------------

    #[test]
    fn loopback_and_lan_addresses_are_not_announceable() {
        assert!(!is_announceable(&ma("/ip4/127.0.0.1/tcp/8080")));
        assert!(!is_announceable(&ma("/ip4/192.168.1.10/tcp/8080")));
        assert!(!is_announceable(&ma("/ip6/::1/tcp/8080")));
        assert!(!is_announceable(&ma("/ip4/0.0.0.0/tcp/8080")));
    }

    #[test]
    fn public_ipv6_is_announceable() {
        assert!(is_announceable(&ma("/ip6/2001:db8::1/tcp/8080")));
    }

    #[test]
    fn a_circuit_address_is_announceable_whatever_fronts_it() {
        // The relay was reached over a LAN; the reservation is still real and
        // the circuit address is still how peers get to us.
        let addr = ma(
            "/ip4/192.168.1.5/tcp/8080/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN/p2p-circuit",
        );
        assert!(is_announceable(&addr));
        assert!(
            is_announceable_with(&addr, true),
            "strict must not break it"
        );
        assert!(is_circuit(&addr));
    }

    // -- circuit addresses must keep the relay hop ------------------------
    //
    // Regression cover for the 2026-08-10 finding: a relay-only peer was
    // reachable from the Go p2pd path and undialable from the native one,
    // because `known_addresses` ran every address through `strip_p2p` and that
    // deleted the relay's peer id along with the destination's. rust-libp2p
    // then rejected the result with "Missing relay peer id."

    const RELAY: &str = "QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc";
    const DEST: &str = "12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE";

    fn full_circuit() -> Multiaddr {
        ma(&format!(
            "/ip4/18.219.43.67/tcp/8000/p2p/{RELAY}/p2p-circuit/p2p/{DEST}"
        ))
    }

    #[test]
    fn strip_dest_p2p_keeps_the_relay_and_drops_the_destination() {
        assert_eq!(
            strip_dest_p2p(&full_circuit()).to_string(),
            format!("/ip4/18.219.43.67/tcp/8000/p2p/{RELAY}/p2p-circuit"),
        );
    }

    #[test]
    fn a_stripped_circuit_readdressed_to_its_destination_is_dialable() {
        // Exactly the round trip `known_addresses` -> caller performs: strip on
        // the way out, re-attach the destination on the way in. The relay hop
        // has to survive it, or the dial fails before it leaves the process.
        let round_tripped = strip_dest_p2p(&full_circuit())
            .with(Protocol::P2p(DEST.parse().expect("dest peer id")));
        assert_eq!(round_tripped, full_circuit());
        assert!(
            round_tripped
                .iter()
                .any(|p| matches!(p, Protocol::P2p(id) if id.to_string() == RELAY)),
            "the relay hop is what makes a circuit address dialable"
        );
    }

    #[test]
    fn the_old_strip_p2p_would_have_broken_the_circuit() {
        // Pins *why* `strip_dest_p2p` exists. `strip_p2p` is still correct for
        // a plain relay address (see `circuit_listen_addr`), so it stays — this
        // asserts the two are genuinely different on a circuit.
        let broken = strip_p2p(&full_circuit());
        assert_eq!(broken.to_string(), "/ip4/18.219.43.67/tcp/8000/p2p-circuit");
        assert!(
            !broken.iter().any(|p| matches!(p, Protocol::P2p(_))),
            "no relay named — this is the address rust-libp2p refuses"
        );
        assert_ne!(broken, strip_dest_p2p(&full_circuit()));
    }

    #[test]
    fn strip_dest_p2p_matches_strip_p2p_on_direct_addresses() {
        let direct = ma(&format!("/ip4/75.141.127.202/tcp/8080/p2p/{DEST}"));
        assert_eq!(strip_dest_p2p(&direct), strip_p2p(&direct));
        assert_eq!(
            strip_dest_p2p(&direct).to_string(),
            "/ip4/75.141.127.202/tcp/8080"
        );
    }

    #[test]
    fn dest_peer_id_reads_through_the_relay_not_the_relay_itself() {
        // `peer_id_from_multiaddr` returns the *first* /p2p, which on a circuit
        // is the relay. Filing a circuit under that key would leave the
        // destination with no route at all.
        assert_eq!(
            dest_peer_id(&full_circuit()).map(|p| p.to_string()),
            Some(DEST.to_string())
        );
        assert_eq!(
            peer_id_from_multiaddr(&full_circuit()).map(|p| p.to_string()),
            Some(RELAY.to_string()),
            "documents the trap dest_peer_id exists to avoid"
        );
    }

    #[test]
    fn dest_peer_id_is_none_when_a_circuit_names_no_destination() {
        // Our own reservation-listen address: a relay and a circuit, nobody on
        // the far end yet. Nothing to file it under.
        let listen = ma(&format!(
            "/ip4/18.219.43.67/tcp/8000/p2p/{RELAY}/p2p-circuit"
        ));
        assert_eq!(dest_peer_id(&listen), None);
    }

    #[test]
    fn dest_peer_id_on_a_direct_address_is_the_peer() {
        let direct = ma(&format!("/ip4/75.141.127.202/tcp/8080/p2p/{DEST}"));
        assert_eq!(
            dest_peer_id(&direct).map(|p| p.to_string()),
            Some(DEST.to_string())
        );
    }

    #[test]
    fn a_circuit_address_is_never_a_relay_candidate() {
        // `is_announceable` says yes to every circuit address by design; the
        // relay-candidate question is a different one and must say no.
        let circuit = ma(&format!(
            "/ip4/18.219.43.67/tcp/8000/p2p/{RELAY}/p2p-circuit"
        ));
        assert!(is_announceable(&circuit), "still announceable as our own");
        assert!(
            !is_relay_candidate_addr(&circuit),
            "but useless as a relay we reserve on"
        );
        let direct = ma("/ip4/198.51.100.7/tcp/8080");
        assert!(is_relay_candidate_addr(&direct));
        assert!(
            !is_relay_candidate_addr(&ma("/ip4/192.168.1.7/tcp/8080")),
            "LAN-only relays are still rejected"
        );
    }

    #[test]
    fn circuit_listen_addr_refuses_to_nest_circuits() {
        // The doubly-nested address that listen_on rejected forever.
        let relay: PeerId = RELAY.parse().unwrap();
        let already_circuit = ma("/ip4/18.219.43.67/tcp/8000/p2p-circuit");
        assert_eq!(circuit_listen_addr(&already_circuit, relay), None);

        let direct = ma("/ip4/18.219.43.67/tcp/8000");
        assert_eq!(
            circuit_listen_addr(&direct, relay).map(|a| a.to_string()),
            Some(format!(
                "/ip4/18.219.43.67/tcp/8000/p2p/{RELAY}/p2p-circuit"
            ))
        );
    }

    #[test]
    fn a_mixed_address_is_rejected() {
        // Routable and unroutable IPs in one address: the unroutable one is a
        // local interface that leaked in, so the whole address is suspect.
        assert!(!is_announceable(&ma(
            "/ip4/1.2.3.4/tcp/8080/ip4/127.0.0.1/tcp/9"
        )));
    }

    #[test]
    fn an_address_with_no_ip_component_is_not_announceable() {
        // A `/dns/` address has no IP to classify. Nothing here promotes it —
        // the reachability machine only ever sees IP addresses from identify.
        assert!(!is_announceable(&ma("/dns/bootstrap-1.kwaai.ai/tcp/8000")));
    }

    // -- helpers --------------------------------------------------------

    #[test]
    fn strips_the_p2p_component() {
        let addr = ma("/ip4/1.2.3.4/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc");
        assert_eq!(strip_p2p(&addr).to_string(), "/ip4/1.2.3.4/tcp/8000");
    }

    #[test]
    fn extracts_peer_id_from_multiaddr() {
        let addr = ma(
            "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
        );
        let peer = peer_id_from_multiaddr(&addr).expect("peer id present");
        assert_eq!(
            peer.to_base58(),
            "QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
        );
        assert!(peer_id_from_multiaddr(&ma("/ip4/127.0.0.1/tcp/4001")).is_none());
    }

    #[test]
    fn builds_a_circuit_listen_address() {
        let relay: PeerId = "QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
            .parse()
            .unwrap();
        // The relay address arrives with a /p2p already attached; appending a
        // second one would produce an address that dials nowhere.
        let with_p2p =
            ma("/ip4/198.18.0.20/tcp/8080/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc");
        let listen = circuit_listen_addr(&with_p2p, relay).expect("a direct relay address builds");
        assert_eq!(
            listen.to_string(),
            "/ip4/198.18.0.20/tcp/8080/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit"
        );
        assert_eq!(
            listen
                .iter()
                .filter(|p| matches!(p, Protocol::P2p(_)))
                .count(),
            1,
            "exactly one /p2p component"
        );
        assert!(is_circuit(&listen));
    }

    // -- what the identify → kad filter admits ---------------------------

    /// The exact set `service.rs` applies to a peer's advertised listen
    /// addresses before storing them in the routing table.
    ///
    /// identify sends every address a peer is listening on and rust-libp2p
    /// filters none of it, so this is the only thing standing between the
    /// routing table and entries that cannot be dialed. Two of these are worse
    /// than merely useless: a peer's loopback resolves to *us*, so dialing it
    /// connects to ourselves and fails `WrongPeerId` — repeatedly, because kad
    /// keeps handing the entry back — and a LAN address from another subnet
    /// crowds out the circuit address that would have worked.
    #[test]
    fn identify_learned_addrs_admit_only_dialable_ones() {
        // Rejected: nothing outside the advertising peer's own host or LAN can
        // use these.
        assert!(!is_announceable_with(&ma("/ip4/127.0.0.1/tcp/8080"), false));
        assert!(!is_announceable_with(
            &ma("/ip4/192.168.1.10/tcp/8080"),
            false
        ));
        assert!(!is_announceable_with(&ma("/ip4/10.0.0.5/tcp/8080"), false));
        assert!(!is_announceable_with(
            &ma("/ip4/172.16.0.9/tcp/8080"),
            false
        ));
        assert!(!is_announceable_with(&ma("/ip6/::1/tcp/8080"), false));
        assert!(!is_announceable_with(
            &ma("/ip4/169.254.1.1/tcp/8080"),
            false
        ));
        assert!(!is_announceable_with(
            &ma("/ip4/100.64.0.1/tcp/8080"),
            false
        ));

        // Accepted: a real public address.
        assert!(is_announceable_with(
            &ma("/ip4/18.219.43.67/tcp/8080"),
            false
        ));

        // Accepted, and load-bearing: this is how a NATed peer says "reach me
        // through this relay". Filtering it out is what leaves two NATed peers
        // unable to connect while each sees the other in the DHT.
        let circuit = ma("/ip4/198.18.0.20/tcp/8080/p2p/\
             QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc/p2p-circuit");
        assert!(is_announceable_with(&circuit, false));
    }

    /// The nat-test topology runs on RFC2544 `198.18/15`, which is reserved but
    /// not LAN-private. With `require_global_ips` off — the default, and what
    /// the test bed needs — those addresses have to survive the filter, or
    /// every node there classifies its peers as undialable and the topology
    /// cannot form at all.
    #[test]
    fn the_test_bed_range_survives_the_identify_filter() {
        let nat_test = ma("/ip4/198.18.0.20/tcp/8080");
        assert!(
            is_announceable_with(&nat_test, false),
            "RFC2544 must pass with require_global_ips off"
        );
        assert!(
            !is_announceable_with(&nat_test, true),
            "and must not pass once the operator declares a real-internet node"
        );
    }
}
