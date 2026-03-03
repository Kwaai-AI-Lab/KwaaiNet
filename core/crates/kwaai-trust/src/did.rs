//! DID utilities for KwaaiNet
//!
//! KwaaiNet uses `did:peer` for node identities. A node's DID is derived
//! directly from its libp2p PeerId, which is itself derived from an Ed25519
//! keypair. This makes it a **self-certifying identifier** — no external
//! registry is needed to bind a DID to a cryptographic key.
//!
//! ## Format
//! ```text
//! did:peer:<base58-encoded-peer-id>
//! ```
//!
//! ## Key insight
//! The libp2p PeerId is functionally equivalent to a `did:key`. It is already
//! used as the Layer 1 identity anchor throughout the KwaaiNet DHT. This module
//! provides the glue between the libp2p world and the W3C DID world.

use libp2p::PeerId;

/// Multicodec varint prefix for P-256 (secp256r1) public keys: 0x1200
/// Encoded as a two-byte varint: 0x80 0x24
const P256_MULTICODEC: &[u8] = &[0x80, 0x24];

/// Convert a libp2p `PeerId` to a `did:peer:` DID string
///
/// # Example
/// ```ignore
/// // did:peer:QmYyQSo1c1Ym7orWxLYvCuxRjeczyuq4GNGbMaFfkMhp4
/// let did = peer_id_to_did(&peer_id);
/// ```
pub fn peer_id_to_did(peer_id: &PeerId) -> String {
    format!("did:peer:{}", peer_id.to_base58())
}

/// Extract a `PeerId` from a `did:peer:` DID string
///
/// Returns `None` if the DID is not in `did:peer:` format or the base58 is invalid.
pub fn did_to_peer_id(did: &str) -> Option<PeerId> {
    did.strip_prefix("did:peer:")
        .and_then(|base58| base58.parse().ok())
}

/// Returns `true` if the given DID string corresponds to the given `PeerId`
pub fn did_matches_peer(did: &str, peer_id: &PeerId) -> bool {
    did_to_peer_id(did).map(|p| p == *peer_id).unwrap_or(false)
}

/// Construct the W3C verification method URI for a node's primary key
///
/// Format: `did:peer:<base58>#key-1`
///
/// This is used in the `verificationMethod` field of a `CredentialProof`.
pub fn verification_method(peer_id: &PeerId) -> String {
    format!("{}#key-1", peer_id_to_did(peer_id))
}

/// Extract the raw 32-byte Ed25519 public key from a libp2p PeerId
///
/// A libp2p PeerId for an Ed25519 key is a multihash of the protobuf-encoded
/// public key:
/// ```text
/// identity_multihash( protobuf{ key_type=Ed25519, data=<32 bytes> } )
/// ```
///
/// The protobuf pattern is: `\x08\x01\x12\x20` + 32 key bytes.
/// The multihash wrapper prepends `\x00` (identity code) + varint(length).
///
/// Returns `None` for PeerIds that do not encode an Ed25519 public key
/// (e.g., SHA256-hashed RSA keys that predate the identity-multihash scheme).
pub fn extract_ed25519_bytes(peer_id: &PeerId) -> Option<[u8; 32]> {
    let bytes = peer_id.to_bytes();
    // Scan for the protobuf field header:
    //   field 1 (key_type), wire type 0, value 1 (Ed25519) → 0x08 0x01
    //   field 2 (data),     wire type 2, length 32         → 0x12 0x20
    for i in 0..bytes.len().saturating_sub(35) {
        if bytes[i] == 0x08 && bytes[i + 1] == 0x01 && bytes[i + 2] == 0x12 && bytes[i + 3] == 0x20
        {
            return bytes[i + 4..i + 36].try_into().ok();
        }
    }
    None
}

/// Derive a `did:key:` DID from a P-256 public key in DER/SPKI format.
///
/// The browser's `credential.response.getPublicKey()` returns the public key
/// in SubjectPublicKeyInfo (SPKI / DER) format. This function:
///
/// 1. Extracts the 65-byte uncompressed P-256 point (prefix `0x04` + x + y)
///    from the end of the SPKI envelope
/// 2. Compresses it to 33 bytes (prefix `0x02`/`0x03` + x)
/// 3. Prepends the P-256 multicodec varint (`0x80 0x24`)
/// 4. Base58btc-encodes the result with a leading `z` (multibase prefix)
///
/// # Errors
/// Returns an error if the bytes are too short or the uncompressed-point
/// prefix (`0x04`) is missing.
pub fn p256_spki_to_did(spki_bytes: &[u8]) -> Result<String, anyhow::Error> {
    // The last 65 bytes of a P-256 SPKI are the uncompressed public key:
    //   0x04 || x (32 bytes) || y (32 bytes)
    if spki_bytes.len() < 65 {
        return Err(anyhow::anyhow!(
            "SPKI too short: expected ≥65 bytes, got {}",
            spki_bytes.len()
        ));
    }
    let uncompressed = &spki_bytes[spki_bytes.len() - 65..];
    if uncompressed[0] != 0x04 {
        return Err(anyhow::anyhow!(
            "Expected uncompressed point prefix 0x04, got 0x{:02x}",
            uncompressed[0]
        ));
    }
    let x = &uncompressed[1..33];
    let y_lsb = uncompressed[64];

    // Compress: 0x02 if y is even, 0x03 if y is odd
    let prefix = if y_lsb & 1 == 0 { 0x02u8 } else { 0x03u8 };
    let mut compressed = Vec::with_capacity(33);
    compressed.push(prefix);
    compressed.extend_from_slice(x);

    // Prepend multicodec varint for P-256
    let mut multikey = Vec::with_capacity(2 + 33);
    multikey.extend_from_slice(P256_MULTICODEC);
    multikey.extend_from_slice(&compressed);

    // Base58btc encode and prepend multibase 'z' prefix
    Ok(format!(
        "did:key:z{}",
        bs58::encode(&multikey).into_string()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_peer_id_did() {
        // Generate a fresh keypair and PeerId
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();

        let did = peer_id_to_did(&peer_id);
        assert!(did.starts_with("did:peer:"));

        let recovered = did_to_peer_id(&did).expect("should round-trip");
        assert_eq!(recovered, peer_id);
    }

    #[test]
    fn did_matches_peer_positive() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let did = peer_id_to_did(&peer_id);
        assert!(did_matches_peer(&did, &peer_id));
    }

    #[test]
    fn extract_ed25519_roundtrip() {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let key_bytes = extract_ed25519_bytes(&peer_id).expect("should extract Ed25519 bytes");
        assert_eq!(key_bytes.len(), 32);
    }
}
