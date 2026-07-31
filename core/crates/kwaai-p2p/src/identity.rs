//! Persistent node identity: load-or-generate a libp2p keypair from disk.
//!
//! **File format** — raw libp2p protobuf-encoded private key bytes
//! (`Keypair::to_protobuf_encoding` / `from_protobuf_encoding`), byte-for-byte
//! the same format `kwaai-cli`'s `NodeIdentity` writes to
//! `~/.kwaainet/identity.key` and the same format go-libp2p-daemon accepts via
//! `-id`. New keys are Ed25519; existing RSA keys (the `bootstrap_keyN.bin`
//! files the Python bootstraps use, and any `Qm…` identity) load fine because
//! the workspace enables libp2p's `rsa` feature.
//!
//! This module is the reusable core for map-server and the native node.
//! `kwaai-cli::identity::NodeIdentity` keeps its own copy of this logic and its
//! own path resolution / DID / credential-store concerns; nothing in this
//! phase changes CLI behavior. The two are compatible on-disk by construction.

use std::path::Path;

use anyhow::{Context, Result};
use libp2p::{identity::Keypair, PeerId};
use tracing::info;

/// Load a libp2p keypair from a protobuf-encoded key file.
///
/// Does **not** generate on a missing file — use [`load_or_generate`] for that.
/// Callers that must not create keys (e.g. a bootstrap mounting a pre-existing
/// RSA key) want this one, so a typo in the path fails loudly instead of
/// silently minting a new peer ID.
pub fn load_keypair(path: &Path) -> Result<Keypair> {
    let bytes = std::fs::read(path)
        .with_context(|| format!("reading identity key: {}", path.display()))?;
    let keypair = Keypair::from_protobuf_encoding(&bytes).with_context(|| {
        format!(
            "decoding identity key {} — file may be corrupted or use an unsupported key type",
            path.display()
        )
    })?;
    info!(
        peer_id = %keypair.public().to_peer_id(),
        path = %path.display(),
        "loaded node identity"
    );
    Ok(keypair)
}

/// Generate a fresh Ed25519 keypair and write it to `path` in libp2p protobuf
/// encoding, creating parent directories as needed.
pub fn generate_keypair(path: &Path) -> Result<Keypair> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating identity directory: {}", parent.display()))?;
        }
    }
    let keypair = Keypair::generate_ed25519();
    let bytes = keypair
        .to_protobuf_encoding()
        .context("encoding identity key")?;
    std::fs::write(path, &bytes)
        .with_context(|| format!("writing identity key: {}", path.display()))?;
    info!(
        peer_id = %keypair.public().to_peer_id(),
        path = %path.display(),
        "generated new node identity"
    );
    Ok(keypair)
}

/// Load the keypair at `path`, generating and persisting a new Ed25519 keypair
/// if the file does not exist.
pub fn load_or_generate(path: &Path) -> Result<Keypair> {
    if path.exists() {
        load_keypair(path)
    } else {
        generate_keypair(path)
    }
}

/// The `PeerId` for a keypair — a one-liner, but it keeps call sites from
/// having to import `libp2p::identity` themselves.
pub fn peer_id_of(keypair: &Keypair) -> PeerId {
    keypair.public().to_peer_id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_then_load_roundtrips_the_peer_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("identity.key");

        let generated = generate_keypair(&path).unwrap();
        assert!(path.exists(), "key file should have been created");

        let loaded = load_keypair(&path).unwrap();
        assert_eq!(peer_id_of(&generated), peer_id_of(&loaded));
    }

    #[test]
    fn load_or_generate_is_stable_across_calls() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("identity.key");

        let first = load_or_generate(&path).unwrap();
        let second = load_or_generate(&path).unwrap();
        assert_eq!(peer_id_of(&first), peer_id_of(&second));
    }

    #[test]
    fn generated_keys_are_ed25519_and_encode_as_12d3() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("identity.key");
        let kp = generate_keypair(&path).unwrap();
        assert_eq!(kp.key_type(), libp2p::identity::KeyType::Ed25519);
        // Ed25519 peer IDs use an identity multihash → base58 starts "12D3Koo"
        assert!(peer_id_of(&kp).to_base58().starts_with("12D3Koo"));
    }

    #[test]
    fn load_missing_file_errors_rather_than_generating() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("does-not-exist.key");
        assert!(load_keypair(&path).is_err());
        assert!(!path.exists());
    }

    #[test]
    fn load_corrupt_file_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("identity.key");
        std::fs::write(&path, b"not a protobuf key").unwrap();
        assert!(load_keypair(&path).is_err());
    }

    /// The file format must stay compatible with `kwaai-cli`'s NodeIdentity,
    /// which reads whatever `Keypair::to_protobuf_encoding` produced.
    #[test]
    fn on_disk_format_is_bare_libp2p_protobuf() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("identity.key");
        let kp = generate_keypair(&path).unwrap();

        let bytes = std::fs::read(&path).unwrap();
        let decoded = Keypair::from_protobuf_encoding(&bytes).unwrap();
        assert_eq!(peer_id_of(&kp), peer_id_of(&decoded));
    }
}
