//! Persistent node identity and `kwaainet identity` CLI commands
//!
//! Each KwaaiNet node has a persistent Ed25519 keypair stored at
//! `~/.kwaainet/identity.key` (raw protobuf-encoded bytes, compatible with
//! go-libp2p-daemon's `-id` flag). The keypair is the source of:
//!
//! - The node's libp2p `PeerId` (stable across restarts)
//! - The node's `did:peer:` DID (Layer 1 identity anchor)
//! - The verification key for VC proofs issued to/by this node
//!
//! ## Why persistence matters
//! Without a persistent keypair, each `kwaainet start` generates a fresh
//! `PeerId`. Any Verifiable Credentials issued to the previous PeerId become
//! orphaned — their subject DID no longer matches the node's current identity.

use anyhow::{Context, Result};
use libp2p::{identity::Keypair, PeerId};
use std::path::{Path, PathBuf};
use tracing::info;

use crate::cli::{IdentityAction, IdentityArgs};
use crate::display::*;
use kwaai_trust::{verify, CredentialStore, TrustScore, VerifiableCredential};

// ---------------------------------------------------------------------------
// NodeIdentity — the persistent cryptographic identity
// ---------------------------------------------------------------------------

/// The node's persistent Ed25519 identity
pub struct NodeIdentity {
    /// The full keypair — retained for Phase 4 peer endorsement signing
    #[allow(dead_code)]
    pub keypair: Keypair,
    pub peer_id: PeerId,
}

impl NodeIdentity {
    /// Load the node identity from `~/.kwaainet/identity.key`.
    /// Generates and saves a new keypair if the file does not exist.
    pub fn load_or_create() -> Result<Self> {
        let path = Self::key_file_path();
        if path.exists() {
            let bytes = std::fs::read(&path)
                .with_context(|| format!("reading identity key: {}", path.display()))?;
            let keypair = Keypair::from_protobuf_encoding(&bytes)
                .context("decoding identity key — file may be corrupted")?;
            let peer_id = keypair.public().to_peer_id();
            info!("Loaded persistent identity: {}", peer_id.to_base58());
            Ok(Self { keypair, peer_id })
        } else {
            Self::generate_and_save()
        }
    }

    /// Generate a fresh Ed25519 keypair, save it, and return the identity
    pub fn generate_and_save() -> Result<Self> {
        let path = Self::key_file_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating identity directory: {}", parent.display()))?;
        }
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let bytes = keypair
            .to_protobuf_encoding()
            .context("encoding identity key")?;
        std::fs::write(&path, &bytes)
            .with_context(|| format!("writing identity key: {}", path.display()))?;
        info!(
            "Generated new persistent identity: {} ({})",
            peer_id.to_base58(),
            path.display()
        );
        Ok(Self { keypair, peer_id })
    }

    /// The node's `did:peer:` DID derived from its PeerId
    pub fn did(&self) -> String {
        kwaai_trust::peer_id_to_did(&self.peer_id)
    }

    /// Path to the identity key file (`~/.kwaainet/identity.key`)
    pub fn key_file_path() -> PathBuf {
        kwaainet_home().join("identity.key")
    }
}

// ---------------------------------------------------------------------------
// CLI command handler
// ---------------------------------------------------------------------------

pub async fn run_identity_command(args: IdentityArgs) -> Result<()> {
    match args.action {
        IdentityAction::Show => show_identity().await,
        IdentityAction::ImportVc { path } => import_vc(&path).await,
        IdentityAction::ListVcs => list_vcs().await,
        IdentityAction::VerifyVc { path } => verify_vc_cmd(&path).await,
    }
}

// ---------------------------------------------------------------------------
// show
// ---------------------------------------------------------------------------

async fn show_identity() -> Result<()> {
    let identity = NodeIdentity::load_or_create()?;
    let store = CredentialStore::open_default()?;
    let vcs = store.load_valid_for_subject(&identity.did());
    let score = TrustScore::from_credentials(&vcs);

    print_box_header("KwaaiNet Node Identity");
    println!("  DID:        {}", identity.did());
    println!("  Peer ID:    {}", identity.peer_id.to_base58());
    println!("  Key file:   {}", NodeIdentity::key_file_path().display());
    println!("  Cred store: {}", CredentialStore::default_dir().display());
    println!();
    println!(
        "  Trust tier: {}  (score: {:.0}%)",
        score.tier_label(),
        score.score * 100.0
    );
    println!("  Valid credentials: {}", vcs.len());

    if !vcs.is_empty() {
        println!();
        for vc in &vcs {
            let vc_type = vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown");
            let expiry = vc
                .expiration_date
                .map(|e| e.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "no expiry".to_string());
            let issuer_short = abbreviate_did(vc.issuer_did(), 20);
            println!("    [{vc_type:<22}]  expires: {expiry}  issuer: {issuer_short}");
        }
    } else {
        println!();
        print_info("No credentials yet. Attend a Kwaai summit to receive your first VC.");
        print_info("Import a VC with: kwaainet identity import-vc <file.json>");
    }

    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// import-vc
// ---------------------------------------------------------------------------

async fn import_vc(path: &Path) -> Result<()> {
    let store = CredentialStore::open_default()?;
    let vc = store.import_file(path)?;

    let result = verify(&vc);
    let vc_type = vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown");

    print_box_header("Import Verifiable Credential");
    println!("  Type:    {}", vc_type);
    println!("  Subject: {}", vc.subject_did());
    println!("  Issuer:  {}", vc.issuer_did());
    println!(
        "  Issued:  {}",
        vc.issuance_date.format("%Y-%m-%d %H:%M UTC")
    );
    if let Some(exp) = vc.expiration_date {
        println!("  Expires: {}", exp.format("%Y-%m-%d"));
    }
    println!();

    match (result.structure_valid, result.signature_valid) {
        (true, Some(true)) => print_success(&format!("Signature verified: {}", result.message)),
        (true, None) => print_warning(&format!("No proof to verify: {}", result.message)),
        (true, Some(false)) => print_warning(&format!("Signature check: {}", result.message)),
        (false, _) => print_error(&format!("Invalid credential: {}", result.message)),
    }

    print_success(&format!(
        "Saved to: {}",
        CredentialStore::default_dir().display()
    ));
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// list-vcs
// ---------------------------------------------------------------------------

async fn list_vcs() -> Result<()> {
    let identity = NodeIdentity::load_or_create()?;
    let store = CredentialStore::open_default()?;
    let all_vcs = store.load_all();

    let (mine, others): (Vec<_>, Vec<_>) = all_vcs
        .into_iter()
        .partition(|vc| vc.subject_did() == identity.did());

    print_box_header("Verifiable Credentials");
    println!("  Node DID:   {}", identity.did());
    println!("  Store:      {}", store.dir().display());
    println!();

    if mine.is_empty() && others.is_empty() {
        println!("  No credentials stored.");
        print_info("Import a credential with: kwaainet identity import-vc <file.json>");
    } else {
        if !mine.is_empty() {
            println!("  This node ({} credential(s)):", mine.len());
            print_vc_table(&mine);
        }
        if !others.is_empty() {
            println!();
            println!("  Other subjects ({} credential(s)):", others.len());
            print_vc_table(&others);
        }
    }

    print_separator();
    Ok(())
}

fn print_vc_table(vcs: &[VerifiableCredential]) {
    println!(
        "    {:<24} {:<12} {:<10}  Issuer",
        "Type", "Issued", "Status"
    );
    println!("    {}", "-".repeat(72));
    for vc in vcs {
        let vc_type = vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown");
        let issued = vc.issuance_date.format("%Y-%m-%d").to_string();
        let status = if vc.is_expired() { "Expired" } else { "Valid" };
        let issuer = abbreviate_did(vc.issuer_did(), 22);
        println!("    {vc_type:<24} {issued:<12} {status:<10}  {issuer}");
    }
}

// ---------------------------------------------------------------------------
// verify-vc
// ---------------------------------------------------------------------------

async fn verify_vc_cmd(path: &Path) -> Result<()> {
    let json = std::fs::read_to_string(path)
        .with_context(|| format!("reading VC file: {}", path.display()))?;
    let vc: VerifiableCredential =
        serde_json::from_str(&json).context("parsing credential JSON")?;

    let result = verify(&vc);

    print_box_header("Verify Verifiable Credential");
    println!("  File:    {}", path.display());
    println!(
        "  Type:    {}",
        vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown")
    );
    println!("  Subject: {}", vc.subject_did());
    println!("  Issuer:  {}", vc.issuer_did());
    println!();
    println!(
        "  Structure: {}",
        if result.structure_valid {
            "valid"
        } else {
            "INVALID"
        }
    );
    match result.signature_valid {
        Some(true) => println!("  Signature: verified"),
        Some(false) => println!("  Signature: FAILED"),
        None => println!("  Signature: not checked"),
    }
    println!("  Detail:  {}", result.message);
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Shorten a DID or raw string to at most `max_len` chars, appending `…`
fn abbreviate_did(did: &str, max_len: usize) -> String {
    if did.len() <= max_len {
        did.to_string()
    } else {
        format!("{}…", &did[..max_len])
    }
}

fn kwaainet_home() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".kwaainet")
}
