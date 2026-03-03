//! VC issuance helpers — signs VCs with the summit server's Ed25519 key

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signer, SigningKey};
use kwaai_trust::{binding_vc, summit_attendee_vc, VerifiableCredential};

/// Issue a signed `SummitAttendeeVC` to a passkey `did:key:` subject.
pub fn issue_summit_attendee_vc(
    signing_key: &SigningKey,
    issuer_did: &str,
    subject_did: &str,
) -> Result<VerifiableCredential> {
    let mut vc = summit_attendee_vc(
        issuer_did,
        subject_did,
        "Kwaai Personal AI Summit 2026",
        "2026-03-15",
    );
    sign_vc(&mut vc, signing_key, issuer_did)?;
    Ok(vc)
}

/// Issue a signed `BindingVC` linking a node `did:peer:` to a passkey `did:key:`.
pub fn issue_binding_vc(
    signing_key: &SigningKey,
    issuer_did: &str,
    node_did: &str,
    passkey_did: &str,
) -> Result<VerifiableCredential> {
    let mut vc = binding_vc(issuer_did, node_did, passkey_did);
    sign_vc(&mut vc, signing_key, issuer_did)?;
    Ok(vc)
}

/// Attach an `Ed25519Signature2020` proof to a VC.
fn sign_vc(vc: &mut VerifiableCredential, key: &SigningKey, issuer_did: &str) -> Result<()> {
    let signing_bytes = vc.to_signing_bytes()?;
    let signature = key.sign(&signing_bytes);
    let proof_value = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    vc.proof = Some(kwaai_trust::CredentialProof {
        proof_type: "Ed25519Signature2020".to_string(),
        created: chrono::Utc::now(),
        verification_method: format!("{}#key-1", issuer_did),
        proof_purpose: "assertionMethod".to_string(),
        proof_value,
    });
    Ok(())
}
