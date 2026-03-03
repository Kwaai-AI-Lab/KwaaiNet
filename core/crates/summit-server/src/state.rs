//! Shared application state

use anyhow::{Context, Result};
use deadpool_postgres::Pool;
use ed25519_dalek::SigningKey;
use std::sync::Arc;
use webauthn_rs::prelude::*;

use crate::config::Config;

pub struct AppState {
    pub webauthn: Arc<Webauthn>,
    pub db: Pool,
    pub signing_key: SigningKey,
    /// DID of the summit server's own issuer identity
    pub issuer_did: String,
}

impl AppState {
    pub fn new(webauthn: Arc<Webauthn>, db: Pool, signing_key: SigningKey) -> Self {
        // The issuer DID is a did:web pointing to the RP origin, or a fixed identifier.
        // For simplicity we encode the public key as a did:key.
        let verifying_key = signing_key.verifying_key();
        let pubkey_bytes = verifying_key.to_bytes();

        // Ed25519 multicodec prefix: 0xed 0x01
        let mut multikey = vec![0xed_u8, 0x01_u8];
        multikey.extend_from_slice(&pubkey_bytes);
        let issuer_did = format!("did:key:z{}", bs58::encode(&multikey).into_string());

        Self {
            webauthn,
            db,
            signing_key,
            issuer_did,
        }
    }
}

pub type SharedState = Arc<AppState>;

pub fn build_webauthn(config: &Config) -> Result<Webauthn> {
    let origin = url::Url::parse(&config.rp_origin).context("Invalid RP_ORIGIN URL")?;
    WebauthnBuilder::new(&config.rp_id, &origin)
        .context("Failed to create WebauthnBuilder")?
        .rp_name("Kwaai Summit 2026")
        .build()
        .context("Failed to build Webauthn")
}

pub fn build_signing_key(hex: &str) -> Result<SigningKey> {
    let bytes = hex::decode(hex).context("SUMMIT_SIGNING_KEY_HEX is not valid hex")?;
    let arr: [u8; 32] = bytes.try_into().map_err(|_| {
        anyhow::anyhow!("SUMMIT_SIGNING_KEY_HEX must be exactly 32 bytes (64 hex chars)")
    })?;
    Ok(SigningKey::from_bytes(&arr))
}
