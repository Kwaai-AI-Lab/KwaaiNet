//! Reading the signed address records peers publish about themselves.
//!
//! A KwaaiNet node announces the addresses it can be dialed on — chiefly the
//! relay circuits a NATed node holds, which no dialer can reconstruct — as an
//! RFC 0003 `PeerRecord` inside a protobuf `SignedEnvelope`, in the interop
//! signing domain so a Go or JS reader can verify the same bytes. This module
//! is the reading half. The writing half lives with the announcement
//! (`kwaai-cli`'s `announce`), because only the node itself can sign.
//!
//! It sits in this crate rather than beside the announcement decoder because
//! what it produces is a **dial instruction**, and everything downstream of
//! that — seeding the learned-address map, choosing dial candidates — is
//! network-layer work. The DHT store it arrives from
//! (`kwaai-hivemind-dht`) has no record validators, so the bytes under a
//! peer's subkey are whatever the last writer put there; verification is the
//! only thing that makes them safe to follow.

use libp2p::{Multiaddr, PeerId};

use crate::addresses::strip_dest_p2p;

/// How many addresses one record may contribute.
///
/// Matches the publisher's own cap (`announce::MAX_DIAL_ADDRS`), which is
/// sized for the shapes that actually occur: a direct address plus the two or
/// three circuits `relay_manager` holds reservations on at once, or one
/// address per transport on a public node. Enforced again here because the
/// record is a remote claim — a peer that publishes forty addresses (or a
/// signed record replayed from a node with a very different address set)
/// would otherwise cost every dialer forty attempts and forty slots in the
/// learned-address map.
pub const MAX_RECORD_ADDRS: usize = 4;

/// The addresses `claimed` signed for itself, or nothing.
///
/// # What the check closes
///
/// Anyone may write under any subkey in the store, and an address list is a
/// dialing instruction: without verification an attacker steers a peer's
/// traffic at an address that peer never signed — black-holing it by naming a dead
/// host, or aiming it at a third party. Impersonation itself is still
/// caught at the Noise handshake, but neither of those attacks needs the
/// attacker to complete one.
///
/// # Both checks are load-bearing
///
/// `PeerRecord::from_signed_envelope_interop` proves only that the record is
/// **self-consistent** — that whoever signed it named themselves. It says
/// nothing about *which* peer that is. The `record.peer_id() == claimed`
/// comparison below is what ties the record to the peer being dialed; without
/// it an attacker signs a record with their own key, stores it under the
/// victim's subkey, and every library check passes.
///
/// Any failure — an unparseable envelope, a bad signature, the legacy
/// (non-interop) signing domain, a record naming someone else — yields an
/// empty vec. That costs the peer nothing worse than the bare-PeerId dial it
/// would have got before the field existed.
///
/// # Shape of what comes back
///
/// Addresses are returned **bare**, each run through
/// [`strip_dest_p2p`]: the destination `/p2p/<peer>` is dropped, a circuit's
/// relay hop kept. That is the same convention `NetworkService`'s
/// `candidate_addresses` hands out, so a record that carries the destination
/// on its addresses and one that does not yield identical results, and a
/// caller that re-attaches the id can never produce `/p2p/<id>/p2p/<id>`.
pub fn verified_addrs(envelope: &[u8], claimed: PeerId) -> Vec<Multiaddr> {
    let Ok(envelope) = libp2p::core::SignedEnvelope::from_protobuf_encoding(envelope) else {
        return Vec::new();
    };
    let Ok(record) = libp2p::core::PeerRecord::from_signed_envelope_interop(envelope) else {
        return Vec::new();
    };
    if record.peer_id() != claimed {
        return Vec::new();
    }
    record
        .addresses()
        .iter()
        .take(MAX_RECORD_ADDRS)
        .map(strip_dest_p2p)
        .filter(|a| !a.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    /// A signed envelope for `key` naming `addrs`, exactly as the publisher
    /// emits one.
    fn envelope(key: &Keypair, addrs: &[&str]) -> Vec<u8> {
        let addrs = addrs
            .iter()
            .map(|a| a.parse().expect("a valid addr"))
            .collect();
        libp2p::core::PeerRecord::new_interop(key, addrs)
            .expect("signs")
            .into_signed_envelope()
            .into_protobuf_encoding()
    }

    #[test]
    fn round_trips_a_direct_address() {
        let key = Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let bytes = envelope(&key, &["/ip4/203.0.113.7/tcp/4001"]);

        assert_eq!(
            verified_addrs(&bytes, peer),
            vec!["/ip4/203.0.113.7/tcp/4001".parse::<Multiaddr>().unwrap()]
        );
    }

    /// The attack the signature exists to stop: anyone may write under any
    /// subkey, so a record whose addresses were signed by a different key must
    /// yield nothing — the peer falls back to a bare-PeerId dial rather than
    /// being steered at an address its own key never vouched for.
    #[test]
    fn rejects_a_record_signed_by_another_key() {
        let victim = Keypair::generate_ed25519().public().to_peer_id();
        let attacker = Keypair::generate_ed25519();
        let bytes = envelope(&attacker, &["/ip4/198.51.100.9/tcp/4001"]);

        assert!(
            verified_addrs(&bytes, victim).is_empty(),
            "an address list the announced peer did not sign must not be dialed"
        );
    }

    /// The publisher signs in the interop domain deliberately, so that a Go or
    /// JS reader can verify it. A record in rust-libp2p's legacy domain is not
    /// that, and must not be accepted by accident.
    #[test]
    fn rejects_the_legacy_signing_domain() {
        let key = Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let addr = "/ip4/203.0.113.7/tcp/4001".parse().expect("a valid addr");
        let legacy = libp2p::core::PeerRecord::new(&key, vec![addr])
            .expect("signs")
            .into_signed_envelope()
            .into_protobuf_encoding();

        assert!(verified_addrs(&legacy, peer).is_empty());
    }

    /// Circuits are the whole motivation, and their shape is the one a dialer
    /// cannot reconstruct. The relay hop must survive; only the destination is
    /// stripped, so a publisher that leaves the destination on and one that
    /// does not agree.
    #[test]
    fn strips_only_the_destination_from_a_circuit() {
        let key = Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let relay = PeerId::random();
        let bare = format!("/ip4/76.13.5.74/tcp/4001/p2p/{relay}/p2p-circuit");
        let qualified = format!("{bare}/p2p/{peer}");

        let expected: Vec<Multiaddr> = vec![bare.parse().unwrap()];
        assert_eq!(verified_addrs(&envelope(&key, &[&bare]), peer), expected);
        assert_eq!(
            verified_addrs(&envelope(&key, &[&qualified]), peer),
            expected,
            "the two publishing conventions must decode identically"
        );
    }

    #[test]
    fn garbage_yields_nothing() {
        assert!(verified_addrs(&[0xde, 0xad, 0xbe, 0xef], PeerId::random()).is_empty());
        assert!(verified_addrs(&[], PeerId::random()).is_empty());
    }

    /// A remote claim is bounded here as well as at the publisher: the cap is
    /// what keeps a signed record from filling a dialer's candidate list.
    #[test]
    fn caps_the_address_count() {
        let key = Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let many: Vec<String> = (0..10)
            .map(|i| format!("/ip4/203.0.113.{i}/tcp/4001"))
            .collect();
        let refs: Vec<&str> = many.iter().map(String::as_str).collect();

        assert_eq!(
            verified_addrs(&envelope(&key, &refs), peer).len(),
            MAX_RECORD_ADDRS
        );
    }

    /// A record carrying nothing but `/p2p/<self>` decodes to no addresses at
    /// all, rather than to an empty multiaddr that cannot be dialed and would
    /// occupy a slot in the learned map.
    #[test]
    fn drops_an_address_that_was_only_a_peer_id() {
        let key = Keypair::generate_ed25519();
        let peer = key.public().to_peer_id();
        let bytes = envelope(&key, &[&format!("/p2p/{peer}")]);

        assert!(verified_addrs(&bytes, peer).is_empty());
    }
}
