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
/// A `/p2p-circuit` component is the only signal available: neither the p2pd
/// control protocol nor the swarm reports "relayed" as a fact of its own.
pub fn classify_addr(m: &Multiaddr) -> ConnKind {
    if is_circuit(m) {
        ConnKind::Relay
    } else {
        ConnKind::Direct
    }
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
}
