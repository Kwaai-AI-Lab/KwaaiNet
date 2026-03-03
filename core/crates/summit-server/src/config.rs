//! Configuration loaded from environment variables

use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    /// PostgreSQL connection string
    pub database_url: String,
    /// WebAuthn relying-party ID (domain, e.g. "summit.kwaai.ai" or "localhost")
    pub rp_id: String,
    /// WebAuthn relying-party origin (full URL, e.g. "https://summit.kwaai.ai")
    pub rp_origin: String,
    /// Hex-encoded 32-byte Ed25519 private key seed used to sign issued VCs
    pub signing_key_hex: String,
    /// TCP bind address (default "0.0.0.0:3000")
    pub bind_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            rp_id: std::env::var("RP_ID").unwrap_or_else(|_| "localhost".to_string()),
            rp_origin: std::env::var("RP_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            signing_key_hex: std::env::var("SUMMIT_SIGNING_KEY_HEX")
                .context("SUMMIT_SIGNING_KEY_HEX must be set (32 hex-encoded bytes)")?,
            bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
        })
    }
}
