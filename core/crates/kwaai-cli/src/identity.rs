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

/// Current identity key epoch.
///
/// Recorded alongside signatures (see `kwaai_ledger`) so that rotating the node
/// key does not silently invalidate history. Bump this when a rotation
/// mechanism lands; carrying it now is free, retrofitting it later is a
/// migration.
pub const KEY_EPOCH: u32 = 1;

/// The node's persistent Ed25519 identity
pub struct NodeIdentity {
    /// The full keypair — the signing key behind `sign()`.
    pub keypair: Keypair,
    pub peer_id: PeerId,
}

impl NodeIdentity {
    /// Load the node identity from `~/.kwaainet/identity.key`.
    /// Generates and saves a new keypair if the file does not exist.
    pub fn load_or_create() -> Result<Self> {
        let path = Self::key_file_path();
        if path.exists() {
            Self::load_from(&path)
        } else {
            Self::generate_and_save()
        }
    }

    /// Load an identity from an explicit libp2p-protobuf-encoded key file.
    /// Unlike `load_or_create`, this does not fall back to generating a new
    /// key — used for bootstrap deployments that mount a pre-existing key
    /// (e.g. an RSA `bootstrap_keyN.bin`).
    pub fn load_from(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("reading identity key: {}", path.display()))?;
        let keypair = Keypair::from_protobuf_encoding(&bytes).context(
            "decoding identity key — file may be corrupted or use an unsupported key type",
        )?;
        let peer_id = keypair.public().to_peer_id();
        // Repair permissions on keys written before they were created 0600.
        // Existing nodes have world-readable identity keys on disk; loading is
        // the only moment we reliably get to fix that in place.
        tighten_key_permissions(path);
        info!(
            "Loaded identity from {}: {}",
            path.display(),
            peer_id.to_base58()
        );
        Ok(Self { keypair, peer_id })
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
        write_key_file(&path, &bytes)?;
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

    /// The raw 32-byte Ed25519 secret, for building an `ed25519_dalek::SigningKey`.
    ///
    /// This is the signing path behind work receipts (`kwaai_ledger`).
    /// Deliberately the *only* signing accessor: libp2p's `Keypair::sign` would
    /// be a second, redundant way to produce the same Ed25519 signatures, and
    /// having two invites them to drift.
    ///
    /// Verification needs no key exchange — the signer's `did:peer:` *is* its
    /// public key, so any holder of the DID can verify.
    ///
    /// Returns an error for non-Ed25519 identities — bootstrap nodes may mount
    /// RSA keys, which cannot produce receipts (and whose PeerIds the
    /// `did:peer:` resolver cannot extract a verifying key from either).
    pub fn ed25519_secret_bytes(&self) -> Result<[u8; 32]> {
        let ed = self
            .keypair
            .clone()
            .try_into_ed25519()
            .map_err(|_| anyhow::anyhow!("node identity is not an Ed25519 key"))?;
        ed.secret()
            .as_ref()
            .try_into()
            .map_err(|_| anyhow::anyhow!("unexpected Ed25519 secret length"))
    }

    /// Path to the identity key file (`~/.kwaainet/identity.key`, or `$KWAAINET_HOME/identity.key`)
    pub fn key_file_path() -> PathBuf {
        crate::config::kwaainet_dir().join("identity.key")
    }
}

/// Narrow an existing key file to `0600` if it is currently more permissive.
///
/// Best-effort and deliberately non-fatal: a node that cannot chmod its key
/// (unusual ownership, exotic filesystem) should still start rather than refuse
/// to run, but it gets a warning so the exposure is visible.
fn tighten_key_permissions(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let Ok(meta) = std::fs::metadata(path) else {
            return;
        };
        let mode = meta.permissions().mode() & 0o777;
        if mode & 0o077 != 0 {
            match std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)) {
                Ok(()) => info!(
                    "Tightened identity key permissions from {:o} to 600: {}",
                    mode,
                    path.display()
                ),
                Err(e) => tracing::warn!(
                    "Identity key {} is mode {:o} (group/world readable) and could not be \
                     tightened to 600: {e}. This key authorises every signature this node \
                     makes — restrict it manually.",
                    path.display(),
                    mode
                ),
            }
        }
    }
    #[cfg(not(unix))]
    let _ = path;
}

/// Write the identity key atomically and with owner-only permissions.
///
/// Two problems with the previous bare `std::fs::write`:
///
/// * **Permissions.** The file inherited the process umask — typically `0644`,
///   i.e. world-readable. This key authorises every signature the node makes,
///   including work receipts, so it must be `0600`. Note that receipts are
///   durable and retroactively valuable: a key stolen while receipts are
///   "worth nothing" can forge history that acquires value later, so this
///   matters from the first signature rather than from the first balance.
/// * **Atomicity.** A crash mid-write left a truncated key and a node that
///   could no longer prove its identity. Write to a temp file in the same
///   directory, then rename — `rename(2)` is atomic within a filesystem.
fn write_key_file(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let tmp = dir.join(format!(
        ".{}.tmp",
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("identity.key")
    ));

    // Create with 0600 from the outset on Unix, so the secret is never briefly
    // world-readable between creation and a later chmod.
    #[cfg(unix)]
    {
        use std::io::Write as _;
        use std::os::unix::fs::OpenOptionsExt as _;
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&tmp)
            .with_context(|| format!("creating temp identity key: {}", tmp.display()))?;
        f.write_all(bytes)
            .with_context(|| format!("writing temp identity key: {}", tmp.display()))?;
        f.sync_all().ok();
    }
    #[cfg(not(unix))]
    {
        std::fs::write(&tmp, bytes)
            .with_context(|| format!("writing temp identity key: {}", tmp.display()))?;
    }

    std::fs::rename(&tmp, path).with_context(|| {
        format!(
            "atomically replacing identity key {} with {}",
            path.display(),
            tmp.display()
        )
    })?;
    Ok(())
}

// ---------------------------------------------------------------------------
// CLI command handler
// ---------------------------------------------------------------------------

pub async fn run_identity_command(args: IdentityArgs) -> Result<()> {
    match args.action {
        IdentityAction::Show => show_identity(args.json).await,
        IdentityAction::ImportVc { path } => import_vc(&path).await,
        IdentityAction::ListVcs => list_vcs().await,
        IdentityAction::VerifyVc { path } => verify_vc_cmd(&path).await,
    }
}

// ---------------------------------------------------------------------------
// show
// ---------------------------------------------------------------------------

async fn show_identity(json: bool) -> Result<()> {
    let identity = NodeIdentity::load_or_create()?;
    let store = CredentialStore::open_default()?;
    let vcs = store.load_valid_for_subject(&identity.did());
    let score = TrustScore::from_credentials(&vcs);

    if json {
        #[derive(serde::Serialize)]
        struct IdentityJson<'a> {
            did: &'a str,
            peer_id: String,
            trust_tier: String,
            score: f64,
            credential_count: usize,
        }
        let did = identity.did();
        let out = IdentityJson {
            did: did.as_str(),
            peer_id: identity.peer_id.to_base58(),
            trust_tier: score.tier_label().to_string(),
            score: score.score,
            credential_count: vcs.len(),
        };
        println!("{}", serde_json::to_string(&out).unwrap_or_default());
        return Ok(());
    }

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
