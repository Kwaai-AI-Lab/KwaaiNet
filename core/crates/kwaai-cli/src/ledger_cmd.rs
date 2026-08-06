//! `kwaainet ledger` — inspect internal work credits.
//!
//! Phase 0 surface: verify a receipt file, and self-test that this node's
//! identity key can participate in the ledger at all. Earning and spending
//! arrive with the transport wiring in Phase 1.

use anyhow::{Context, Result};
use ed25519_dalek::SigningKey;
use kwaai_ledger::{LeaseQuote, Receipt, WorkClaimPayload};

use crate::cli::{LedgerAction, LedgerArgs};
use crate::display::*;
use crate::identity::{NodeIdentity, KEY_EPOCH};

pub async fn run(args: LedgerArgs) -> Result<()> {
    match args.action {
        LedgerAction::Verify { path } => verify_receipt(&path),
        LedgerAction::SelfTest => self_test(),
    }
}

fn verify_receipt(path: &std::path::Path) -> Result<()> {
    let bytes =
        std::fs::read(path).with_context(|| format!("reading receipt file: {}", path.display()))?;
    let receipt: Receipt = rmp_serde::from_slice(&bytes)
        .with_context(|| format!("decoding receipt (expected msgpack): {}", path.display()))?;

    print_box_header("Work Receipt");

    let claim = &receipt.claim.payload;
    println!("  Receipt ID:  {}", receipt.receipt_id_hex()?);
    println!("  Provider:    {}", receipt.provider_did());
    println!("  Consumer:    {}", receipt.consumer_did());
    println!("  Lease:       {}", claim.lease_id);
    println!("  Request:     {}", claim.request_id);
    println!(
        "  Tokens:      {} prompt + {} completion",
        claim.prompt_tokens, claim.completion_tokens
    );
    println!(
        "  Credits:     {} micro ({:.6})",
        claim.credits_owed,
        claim.credits_owed as f64 / kwaai_ledger::MICRO_PER_CREDIT as f64
    );
    println!("  Digest:      {}", hex::encode(claim.response_digest));
    println!();

    match receipt.verify() {
        Ok(()) => {
            print_success("Both signatures valid — receipt is payable.");
            print_separator();
            Ok(())
        }
        Err(e) => {
            print_error(&format!("Verification failed: {e}"));
            print_separator();
            // A receipt that does not verify is a hard failure, not a warning:
            // callers scripting this need a non-zero exit.
            anyhow::bail!("receipt verification failed")
        }
    }
}

/// Sign a sample receipt as both parties using this node's own key, then verify
/// it after a full wire round-trip.
///
/// The round-trip is the point: signing and verifying the same in-memory value
/// can succeed while a decoded copy fails, which is exactly the class of bug
/// that went unnoticed in `kwaai-trust`'s credential signing.
fn self_test() -> Result<()> {
    print_box_header("Ledger Self-Test");

    let identity = NodeIdentity::load_or_create()?;
    let did = identity.did();
    println!("  Identity:    {did}");

    let secret = identity.ed25519_secret_bytes().context(
        "this node cannot sign receipts — the ledger requires an Ed25519 identity \
         (RSA bootstrap keys are not supported)",
    )?;
    let signing = SigningKey::from_bytes(&secret);

    // Both parties are this node: a self-test proves the key works, not that a
    // real exchange happened.
    let body = br#"{"message":{"content":"self-test"},"prompt_eval_count":3,"eval_count":7}"#;
    let quote = LeaseQuote {
        lease_id: 1,
        provider_did: did.clone(),
        consumer_did: did.clone(),
        model: "self-test".to_string(),
        price_micro_per_1k_tokens: 1_000,
        ttl_secs: 30,
        granted_at_unix_ms: now_unix_ms(),
        nonce: 1,
        key_epoch: KEY_EPOCH,
    };
    let grant = quote.clone().sign(&signing)?;
    grant
        .verify()
        .context("provider signature on the quote did not verify")?;
    print_success("Signed and verified a lease quote.");

    let credits = quote.credits_for_tokens(3 + 7)?;
    let claim = WorkClaimPayload {
        lease_id: quote.lease_id,
        request_id: 1,
        provider_did: did.clone(),
        consumer_did: did.clone(),
        prompt_tokens: 3,
        completion_tokens: 7,
        response_digest: kwaai_ledger::response_digest(body),
        credits_owed: credits,
        valid_until_unix_ms: quote.granted_at_unix_ms + 60_000,
        nonce: 2,
        key_epoch: KEY_EPOCH,
    }
    .sign(&signing)?;

    claim
        .verify_against(&grant, body, 3, 7)
        .context("claim did not verify against its own quote")?;
    print_success("Signed a work claim and verified it against the quote.");

    let receipt = claim.counter_sign(&signing, KEY_EPOCH)?;

    // The load-bearing check: survive a real encode → decode boundary.
    let encoded = rmp_serde::to_vec(&receipt).context("encoding receipt")?;
    let decoded: Receipt =
        rmp_serde::from_slice(&encoded).context("decoding receipt after round-trip")?;
    decoded
        .verify()
        .context("receipt failed to verify after a wire round-trip")?;

    if decoded.receipt_id()? != receipt.receipt_id()? {
        anyhow::bail!("receipt id changed across the wire round-trip — encoding is not canonical");
    }

    print_success(&format!(
        "Receipt verified after a {} byte wire round-trip.",
        encoded.len()
    ));
    println!();
    println!("  Receipt ID:  {}", receipt.receipt_id_hex()?);
    println!("  Credits:     {credits} micro");
    println!();
    print_success("This node can produce and verify work receipts.");
    print_separator();
    Ok(())
}

fn now_unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
