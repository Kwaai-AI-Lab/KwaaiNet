//! Verifiable Credential signature verification
//!
//! Verifies `Ed25519Signature2020` proofs on KwaaiNet VCs. The proof
//! covers the canonical JSON of the VC with the `proof` field absent.
//!
//! ## Verification algorithm
//! 1. Remove `proof` from the VC and serialise to JSON bytes.
//! 2. Decode the `proofValue` (base64url → 64 bytes).
//! 3. Extract the issuer's Ed25519 public key from the `verificationMethod` DID.
//! 4. Verify the Ed25519 signature.
//!
//! ## Signing (for issuers — summit server, GliaNet Foundation, etc.)
//! See `sign_credential_bytes`. Node-to-node signing (peer endorsements)
//! will be wired up in Phase 4.

use crate::credential::VerifiableCredential;
use crate::did::{did_to_peer_id, extract_ed25519_bytes};
use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

// ---------------------------------------------------------------------------
// Verification
// ---------------------------------------------------------------------------

/// Outcome of credential verification
#[derive(Debug)]
pub struct VerificationResult {
    /// `true` if all required fields are present and the credential is not expired
    pub structure_valid: bool,
    /// `Some(true)` if the signature verified, `Some(false)` if it failed,
    /// `None` if no proof was present or the proof type is unsupported
    pub signature_valid: Option<bool>,
    /// Human-readable status message
    pub message: String,
}

impl VerificationResult {
    /// Returns `true` only if both structure and signature are valid
    pub fn is_valid(&self) -> bool {
        self.structure_valid && self.signature_valid == Some(true)
    }
}

/// Verify a Verifiable Credential
///
/// Checks structure (required fields, expiry) and, if a proof is present,
/// the Ed25519 signature using the key encoded in the issuer's `did:peer:` DID.
pub fn verify(vc: &VerifiableCredential) -> VerificationResult {
    // --- Structure checks ---

    if vc.credential_type.is_empty() {
        return invalid_structure("Missing credential type");
    }
    if vc.issuer.is_empty() {
        return invalid_structure("Missing issuer DID");
    }
    if vc.subject.id.is_empty() {
        return invalid_structure("Missing subject DID");
    }
    if vc.is_expired() {
        return invalid_structure("Credential has expired");
    }

    // --- Proof check ---

    let Some(proof) = &vc.proof else {
        return VerificationResult {
            structure_valid: true,
            signature_valid: None,
            message: "Credential has no proof (unverified)".to_string(),
        };
    };

    if proof.proof_type != "Ed25519Signature2020" {
        return VerificationResult {
            structure_valid: true,
            signature_valid: None,
            message: format!("Unsupported proof type: {}", proof.proof_type),
        };
    }

    match verify_ed25519_proof(vc) {
        Ok(true) => VerificationResult {
            structure_valid: true,
            signature_valid: Some(true),
            message: "Signature verified ✓".to_string(),
        },
        Ok(false) => VerificationResult {
            structure_valid: true,
            signature_valid: Some(false),
            message: "Signature verification failed".to_string(),
        },
        Err(e) => VerificationResult {
            structure_valid: true,
            signature_valid: Some(false),
            message: format!("Verification error: {}", e),
        },
    }
}

fn invalid_structure(msg: &str) -> VerificationResult {
    VerificationResult {
        structure_valid: false,
        signature_valid: None,
        message: msg.to_string(),
    }
}

fn verify_ed25519_proof(vc: &VerifiableCredential) -> Result<bool> {
    let proof = vc.proof.as_ref().context("no proof")?;

    // Decode base64url signature → 64 bytes
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(&proof.proof_value)
        .context("decoding proofValue base64url")?;
    if sig_bytes.len() != 64 {
        bail!(
            "Expected 64-byte Ed25519 signature, got {}",
            sig_bytes.len()
        );
    }
    let sig_array: [u8; 64] = sig_bytes.try_into().unwrap();
    let signature = Signature::from_bytes(&sig_array);

    // Extract verifying key from the verificationMethod DID
    // Format: "did:peer:<base58>#key-1"
    let vm = &proof.verification_method;
    let did = vm.split('#').next().context("invalid verificationMethod")?;
    let peer_id =
        did_to_peer_id(did).with_context(|| format!("could not parse DID as PeerId: {did}"))?;

    let key_bytes = extract_ed25519_bytes(&peer_id)
        .with_context(|| format!("could not extract Ed25519 key from PeerId: {peer_id}"))?;

    let verifying_key =
        VerifyingKey::from_bytes(&key_bytes).context("invalid Ed25519 public key")?;

    // Verify over the canonical VC bytes (without proof field)
    let payload = vc.to_signing_bytes()?;

    Ok(verifying_key.verify(&payload, &signature).is_ok())
}

// ---------------------------------------------------------------------------
// Signing (for issuers — not used by nodes in Phase 1)
// ---------------------------------------------------------------------------

/// Sign a VC using raw Ed25519 secret key bytes (32 bytes)
///
/// Attaches an `Ed25519Signature2020` proof to the VC in place.
/// Used by VC issuers (summit server, GliaNet Foundation, bootstrap servers).
/// Node-to-node signing for peer endorsements is wired up in Phase 4.
pub fn sign_credential_bytes(
    vc: &mut VerifiableCredential,
    secret_key_bytes: &[u8; 32],
    issuer_peer_id: &libp2p::PeerId,
) -> Result<()> {
    use ed25519_dalek::{Signer, SigningKey};

    let signing_key = SigningKey::from_bytes(secret_key_bytes);
    let payload = vc.to_signing_bytes()?;
    let signature = signing_key.sign(&payload);
    let proof_value = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    vc.proof = Some(crate::credential::CredentialProof {
        proof_type: "Ed25519Signature2020".to_string(),
        created: chrono::Utc::now(),
        verification_method: crate::did::verification_method(issuer_peer_id),
        proof_purpose: "assertionMethod".to_string(),
        proof_value,
    });

    Ok(())
}
