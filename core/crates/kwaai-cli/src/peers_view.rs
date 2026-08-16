//! Shared classification for peer listings.
//!
//! Two surfaces render the same connection table: `kwaainet p2p peers list`
//! (via the p2pd control socket) and the gRPC Network op the GUI subscribes to
//! (via `NetworkHandle`). They read the peer set from different places, but
//! "is this connection relayed?" and "is this peer one of ours?" must mean the
//! same thing in both — otherwise the CLI and the GUI quietly disagree about
//! the same network, which is worse than either being wrong on its own.
//!
//! None of this is derivable from a connection alone. Relay-vs-direct comes
//! from the multiaddr; bootstrap and trusted-relay membership come from local
//! configuration. That is why it lives here rather than in `kwaai-p2p`: the
//! swarm has no opinion about which peers the operator chose to trust.

use std::collections::HashSet;

use kwaai_p2p::{is_circuit, NetworkConfig};
use libp2p::{Multiaddr, PeerId};

use crate::config::KwaaiNetConfig;

/// How a connection reaches the peer.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnKind {
    /// Plain transport address — directly dialable.
    Direct,
    /// Path runs through a circuit relay.
    Relay,
}

/// Classify a connection from its multiaddr.
///
/// Neither the p2pd control protocol nor the swarm reports "relayed" as a fact
/// of its own, so this reads it off the address. Two shapes mean relayed:
///
/// * an explicit `/p2p-circuit` component;
/// * an address with **no transport at all** — a bare `/p2p/<id>`. That is what
///   an inbound connection's `send_back_addr` looks like when it arrived over a
///   relay's already-open circuit: the relay strips the circuit components
///   before handing us the stream, leaving only the peer id.
///
/// The second case is why `Direct` is not the fallback. Calling an unclassifi-
/// able address direct claims the stronger and more surprising thing — that a
/// stranger reached us unsolicited — on the least evidence, and on a NATed node
/// with no port forward that claim is simply wrong.
pub fn classify_addr(m: &Multiaddr) -> ConnKind {
    if is_circuit(m) || !has_transport(m) {
        ConnKind::Relay
    } else {
        ConnKind::Direct
    }
}

/// Whether the address names a way to actually reach the peer, rather than only
/// naming the peer.
fn has_transport(m: &Multiaddr) -> bool {
    m.iter()
        .any(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
}

/// Sort group for a connection, lowest first: bootstrap, trusted relay, plain
/// direct, then via-relay.
///
/// Bootstrap and trusted-relay rank above the direct/relay split because they
/// reflect configuration the operator explicitly chose — when scanning a peer
/// list you are usually asking "did my bootstraps connect?" before anything
/// else. Callers pair this with a peer-id tiebreak so the order is stable
/// across polls; an unstable sort makes a live view flicker as peers reorder
/// under it.
pub fn group_index(is_bootstrap: bool, is_trusted_relay: bool, kind: ConnKind) -> u8 {
    if is_bootstrap {
        return 0;
    }
    if is_trusted_relay {
        return 1;
    }
    match kind {
        ConnKind::Direct => 2,
        ConnKind::Relay => 3,
    }
}

/// Whether a peer serves the DHT, as far as identify has told us.
///
/// This is a *third* state deliberately: identify completes shortly after the
/// connection establishes, so `protocols` is empty on a freshly-connected peer
/// and stays empty for a beat. Collapsing that into a bool would make every
/// peer briefly look like a non-server, and anything filtering on it would
/// blink rows in and out as connections settle.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DhtRole {
    /// Advertises the kad protocol — a routing hop that can store records and
    /// be returned as a lookup result.
    Server,
    /// Identify completed and kad was absent. A query-only participant: it
    /// reads the DHT but never serves it, so it can be a peer's *target* but
    /// never a step on the way to one.
    Client,
    /// Identify has not reported yet. Not a claim either way.
    Unknown,
}

/// The kad protocol identifier, as advertised over identify.
///
/// Matched with a prefix rather than equality: libp2p appends a network
/// suffix when one is configured (`/ipfs/kad/1.0.0` vs `/kwaai/kad/1.0.0`),
/// and a peer on a named network still serves the DHT.
const KAD_PROTOCOL_INFIX: &str = "/kad/";

/// Classify a peer's DHT participation from its advertised protocols.
///
/// Keyed off the protocol list rather than `agent_version` on purpose:
/// advertising a protocol is a commitment the peer's stack actually honours,
/// whereas the version string is free text a peer can set to anything.
///
/// Client-mode is not a hivemind artifact and does not go away with the p2pd
/// migration — rust-libp2p has the same mode, and our own nodes sit in it
/// until reachability resolves. It is a permanent property of the network.
pub fn dht_role(protocols: &[String]) -> DhtRole {
    if protocols.is_empty() {
        return DhtRole::Unknown;
    }
    if protocols.iter().any(|p| p.contains(KAD_PROTOCOL_INFIX)) {
        DhtRole::Server
    } else {
        DhtRole::Client
    }
}

/// The bootstrap peer IDs this node was configured to use.
///
/// Prefers the user's `initial_peers` override and falls back to the built-in
/// KwaaiNet/Petals defaults — the same precedence as `vpk discover` and
/// `node.rs`, so all three agree on what counts as a bootstrap.
pub fn bootstrap_peer_ids() -> HashSet<PeerId> {
    let bootstraps: Vec<String> = match KwaaiNetConfig::load_or_create() {
        Ok(cfg) if !cfg.initial_peers.is_empty() => cfg.initial_peers,
        _ => NetworkConfig::with_petals_bootstrap().bootstrap_peers,
    };

    peer_ids_from_multiaddrs(&bootstraps)
}

/// The trusted-relay peer IDs this node was configured with. Empty when none
/// are configured, which is the production default.
pub fn trusted_relay_peer_ids() -> HashSet<PeerId> {
    let relays = KwaaiNetConfig::load_or_create()
        .map(|cfg| cfg.trusted_relays)
        .unwrap_or_default();

    peer_ids_from_multiaddrs(&relays)
}

/// Pull the `/p2p/<peer-id>` component out of each configured multiaddr.
///
/// Entries without one, or with an unparseable id, are skipped rather than
/// failing the lot: a single malformed line in config should not blank out the
/// bootstrap labels on an otherwise healthy peer list.
fn peer_ids_from_multiaddrs(addrs: &[String]) -> HashSet<PeerId> {
    addrs
        .iter()
        .filter_map(|addr| addr.split("/p2p/").nth(1))
        .filter_map(|id| id.parse::<PeerId>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(s: &str) -> Multiaddr {
        s.parse().expect("test multiaddr must parse")
    }

    #[test]
    fn classifies_plain_transport_as_direct() {
        assert_eq!(
            classify_addr(&addr("/ip4/198.18.0.10/tcp/8000")),
            ConnKind::Direct
        );
        assert_eq!(
            classify_addr(&addr("/ip4/198.18.0.10/tcp/8000/p2p/12D3KooWA9DGWLoTPRZaZhBnjaGoQoDeGjKZKuadyeR6mUwXNBTa")),
            ConnKind::Direct
        );
    }

    #[test]
    fn classifies_a_transportless_address_as_relayed() {
        // What an inbound connection over an already-open circuit looks like:
        // the relay strips the circuit components, leaving only the peer id.
        // Calling this "direct" told a NATed node with no port forward that a
        // stranger had reached it unsolicited, which cannot happen.
        assert_eq!(
            classify_addr(&addr(
                "/p2p/12D3KooWA9DGWLoTPRZaZhBnjaGoQoDeGjKZKuadyeR6mUwXNBTa"
            )),
            ConnKind::Relay
        );
    }

    #[test]
    fn classifies_circuit_paths_as_relayed() {
        // The relay component can sit at the end or be followed by the target
        // peer; both are relayed.
        assert_eq!(
            classify_addr(&addr("/ip4/198.18.0.50/tcp/4001/p2p-circuit")),
            ConnKind::Relay
        );
        assert_eq!(
            classify_addr(&addr("/ip4/198.18.0.50/tcp/4001/p2p/12D3KooWA9DGWLoTPRZaZhBnjaGoQoDeGjKZKuadyeR6mUwXNBTa/p2p-circuit/p2p/12D3KooWH3uVF6wv47WnArKHk5p6cvgCJEb74UTmxztmQDc298L3")),
            ConnKind::Relay
        );
    }

    #[test]
    fn bootstrap_outranks_every_other_group() {
        // A bootstrap reached over a relay still sorts first: the question
        // "did my bootstraps connect?" outranks how they connected.
        assert_eq!(group_index(true, false, ConnKind::Relay), 0);
        assert_eq!(group_index(true, true, ConnKind::Direct), 0);
        assert_eq!(group_index(false, true, ConnKind::Relay), 1);
        assert_eq!(group_index(false, false, ConnKind::Direct), 2);
        assert_eq!(group_index(false, false, ConnKind::Relay), 3);
    }

    #[test]
    fn extracts_peer_ids_and_skips_malformed_entries() {
        let ids = peer_ids_from_multiaddrs(&[
            "/ip4/198.18.0.10/tcp/8000/p2p/12D3KooWA9DGWLoTPRZaZhBnjaGoQoDeGjKZKuadyeR6mUwXNBTa"
                .to_string(),
            // No /p2p/ component — skipped, not fatal.
            "/ip4/198.18.0.11/tcp/8000".to_string(),
            // Present but not a valid peer id — also skipped.
            "/ip4/198.18.0.12/tcp/8000/p2p/not-a-peer-id".to_string(),
        ]);

        assert_eq!(ids.len(), 1);
        assert!(ids.contains(
            &"12D3KooWA9DGWLoTPRZaZhBnjaGoQoDeGjKZKuadyeR6mUwXNBTa"
                .parse::<PeerId>()
                .unwrap()
        ));
    }

    /// The protocol list a kwaainet node advertises once identify settles.
    /// Captured from a live nat-test node.
    fn server_protocols() -> Vec<String> {
        [
            "/ipfs/id/1.0.0",
            "/ipfs/id/push/1.0.0",
            "/ipfs/kad/1.0.0",
            "/ipfs/ping/1.0.0",
            "/libp2p/autonat/1.0.0",
            "/libp2p/circuit/relay/0.2.0/hop",
            "/libp2p/circuit/relay/0.2.0/stop",
            "/libp2p/dcutr",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    /// A hivemind `-dhtClient=1` sidecar, captured from the same network.
    /// Note it still advertises circuit relay hop.
    fn client_protocols() -> Vec<String> {
        [
            "/ipfs/id/1.0.0",
            "/ipfs/id/push/1.0.0",
            "/ipfs/ping/1.0.0",
            "/libp2p/autonat/1.0.0",
            "/libp2p/circuit/relay/0.2.0/hop",
            "/libp2p/circuit/relay/0.2.0/stop",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn kad_speaker_is_a_server() {
        assert_eq!(dht_role(&server_protocols()), DhtRole::Server);
    }

    #[test]
    fn query_only_peer_is_a_client() {
        assert_eq!(dht_role(&client_protocols()), DhtRole::Client);
    }

    /// The case that makes this an enum rather than a bool. identify lands
    /// *after* the connection establishes, so a just-connected peer reports
    /// nothing. Calling that "client" would misclassify every peer for the
    /// first moments of its life, and blink rows in and out of a filtered view.
    #[test]
    fn no_protocols_yet_is_unknown() {
        assert_eq!(dht_role(&[]), DhtRole::Unknown);
    }

    /// A named network suffixes the kad protocol id. Such a peer still serves
    /// the DHT, so the match is on the infix rather than the full string.
    #[test]
    fn named_network_kad_still_serves() {
        let protocols = vec![
            "/ipfs/id/1.0.0".to_string(),
            "/kwaai/kad/1.0.0".to_string(),
        ];
        assert_eq!(dht_role(&protocols), DhtRole::Server);
    }

    /// Relay hop must not be read as DHT participation: the hivemind clients
    /// advertise it, and conflating the two would classify every one of them
    /// as a server.
    #[test]
    fn relay_hop_alone_is_not_a_server() {
        let protocols = vec!["/libp2p/circuit/relay/0.2.0/hop".to_string()];
        assert_eq!(dht_role(&protocols), DhtRole::Client);
    }
}
