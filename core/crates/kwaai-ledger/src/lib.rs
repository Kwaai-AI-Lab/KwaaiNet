//! KwaaiNet internal work credits — co-signed work receipts.
//!
//! Nodes serve each other GPU inference with no accounting today. This crate
//! provides the artifacts that let two peers agree, in writing, on what was
//! delivered: a provider-signed **quote**, a provider-signed **claim** about
//! work actually done, and a consumer-counter-signed **receipt**.
//!
//! Netting receipts pairwise *is* a bilateral mutual-credit system. There is
//! deliberately no mint, no balance, no network-wide fungibility, and no
//! issuance — total supply is exactly zero, so a receipt is a record of mutual
//! obligation rather than of money. See the Ledger plan for why that scope was
//! chosen (it needs no consensus, and this codebase has no agreed-between-peers
//! state to build on).
//!
//! ## The exchange
//!
//! 1. **Quote** — at lease negotiation the provider signs
//!    [`LeaseQuote`] (price, model, both DIDs, a nonce), yielding
//!    [`SignedLeaseGrant`]. Price is fixed *before* work, so the only thing
//!    left to agree afterwards is quantity.
//! 2. **Claim** — after serving, the provider signs [`WorkClaimPayload`]
//!    (token counts + a digest of the exact response bytes), yielding
//!    [`WorkClaim`].
//! 3. **Receipt** — the consumer recomputes the digest and the token counts
//!    from bytes it already holds, checks the arithmetic against the quote, and
//!    counter-signs, yielding [`Receipt`].
//!
//! Because every dispatch path in KwaaiNet is non-streaming and the response
//! body already carries `eval_count` / `prompt_eval_count`, the consumer can
//! independently re-derive the quantity it is about to sign. A provider that
//! inflates the count breaks [`WorkClaimPayload::response_digest`]. No trusted
//! meter is required.
//!
//! ## Canonical encoding — why these rules exist
//!
//! Anything carrying value must have a *deterministic* signing preimage.
//! `kwaai-trust`'s [`VerifiableCredential`](kwaai_trust::VerifiableCredential)
//! does not: its `credentialSubject.claims` is a `HashMap` with
//! `#[serde(flatten)]`, and `RandomState` is seeded per instance, so signing an
//! in-memory value works while verifying the *same* credential after a JSON
//! round-trip can fail at random. Rather than inherit that, this crate enforces
//! three rules by construction:
//!
//! * **No maps.** Every signed payload is a struct of fixed, ordered fields.
//!   No `HashMap`, no `BTreeMap`, no `#[serde(flatten)]`.
//! * **No floats.** Integers only — tokens, bytes, milliseconds, micro-credits.
//!   Floats are fine to *announce* (`throughput: f64` in the DHT) but must never
//!   be *signed*.
//! * **Positional encoding.** Signing preimages use [`rmp_serde::to_vec`]
//!   (msgpack arrays, positional) rather than `to_vec_named` (msgpack maps).
//!   Field order is fixed at compile time by declaration order, so the preimage
//!   is byte-identical across processes and machines. This is also ~40% smaller
//!   on a per-request path.
//!
//! The signature is never part of its own preimage: each signed type wraps a
//! separate payload struct, so `to_signing_bytes()` is simply the encoding of
//! that payload. This avoids the error-prone "clone, null the signature field,
//! re-serialize" pattern entirely.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod store;
pub use store::{LedgerStore, PeerBalance, MIN_CLAIMS_FOR_RATIO};

/// Server-minted lease identifier, scoped to the granting peer's process.
/// Mirrors `capacity_lease::LeaseId`; never persisted by the lease table.
pub type LeaseId = u64;

/// A credit amount in its smallest unit. Always an integer — never a float.
/// One credit is 1_000_000 micro-credits.
pub type MicroCredits = u64;

/// Micro-credits per whole credit.
pub const MICRO_PER_CREDIT: MicroCredits = 1_000_000;

/// Ed25519 signature length; enforced on verify since signatures travel as
/// `Vec<u8>` (serde has no built-in impl for `[u8; 64]`).
const SIG_LEN: usize = 64;

/// Which identity key signed an artifact. Recorded so a future key rotation
/// does not silently invalidate historical receipts — cheap to carry now,
/// a migration to retrofit later.
pub type KeyEpoch = u32;

#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    #[error("signature is {0} bytes, expected {SIG_LEN}")]
    BadSignatureLength(usize),
    #[error("signature verification failed for {role} ({did})")]
    BadSignature { role: &'static str, did: String },
    #[error("could not resolve a verifying key from DID: {0}")]
    UnresolvableDid(String),
    #[error("encoding failed: {0}")]
    Encode(String),
    #[error("credits overflowed while computing {0}")]
    Overflow(&'static str),
    #[error("claim does not match its quote: {0}")]
    QuoteMismatch(String),
    #[error("response digest mismatch — the claim refers to different bytes than were delivered")]
    DigestMismatch,
    #[error("ledger store: {0}")]
    Store(String),
    #[error(
        "token count mismatch — provider claimed {claimed_prompt}+{claimed_completion} but the \
         delivered response reports {observed_prompt}+{observed_completion}"
    )]
    TokenCountMismatch {
        claimed_prompt: u64,
        claimed_completion: u64,
        observed_prompt: u64,
        observed_completion: u64,
    },
}

type Result<T> = std::result::Result<T, LedgerError>;

// ── Quote ─────────────────────────────────────────────────────────────────────

/// The provider's offer, signed at lease-negotiation time. Fixing the price
/// before any work happens is what reduces the post-hoc argument to quantity
/// alone.
///
/// Field order is the signing preimage — do not reorder without bumping a
/// protocol version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaseQuote {
    pub lease_id: LeaseId,
    pub provider_did: String,
    /// Binding the quote to one consumer is what stops a grant being replayed
    /// against a different peer. Only meaningful because the transport now
    /// exposes the authenticated caller.
    pub consumer_did: String,
    pub model: String,
    pub price_micro_per_1k_tokens: MicroCredits,
    pub ttl_secs: u32,
    pub granted_at_unix_ms: u64,
    pub nonce: u64,
    pub key_epoch: KeyEpoch,
}

impl LeaseQuote {
    /// Deterministic signing preimage. See the canonical-encoding rules in the
    /// module docs.
    pub fn to_signing_bytes(&self) -> Result<Vec<u8>> {
        to_canonical(self)
    }

    /// Credits owed for `total_tokens` at this quote's price, rounding up so a
    /// provider is never underpaid for a partial 1k block.
    pub fn credits_for_tokens(&self, total_tokens: u64) -> Result<MicroCredits> {
        self.price_micro_per_1k_tokens
            .checked_mul(total_tokens)
            .ok_or(LedgerError::Overflow("price × tokens"))
            .map(|n| n.div_ceil(1000))
    }

    pub fn sign(self, key: &SigningKey) -> Result<SignedLeaseGrant> {
        let sig = key.sign(&self.to_signing_bytes()?);
        Ok(SignedLeaseGrant {
            quote: self,
            provider_sig: sig.to_bytes().to_vec(),
        })
    }
}

/// A [`LeaseQuote`] plus the provider's signature over it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedLeaseGrant {
    pub quote: LeaseQuote,
    pub provider_sig: Vec<u8>,
}

impl SignedLeaseGrant {
    /// Verify the provider actually issued this quote.
    pub fn verify(&self) -> Result<()> {
        verify_sig(
            &self.quote.provider_did,
            &self.quote.to_signing_bytes()?,
            &self.provider_sig,
            "provider",
        )
    }
}

// ── Claim ─────────────────────────────────────────────────────────────────────

/// The provider's assertion about work actually delivered.
///
/// `response_digest` is what makes this self-agreeing: it is SHA-256 over the
/// exact response bytes the consumer received, and the token counts are parsed
/// from those same bytes. Inflating a count without breaking the digest is not
/// possible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkClaimPayload {
    pub lease_id: LeaseId,
    pub request_id: u64,
    pub provider_did: String,
    pub consumer_did: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub response_digest: [u8; 32],
    pub credits_owed: MicroCredits,
    /// Unix ms after which this claim may no longer be counter-signed, bounding
    /// how long a provider can sit on an unsigned claim.
    pub valid_until_unix_ms: u64,
    pub nonce: u64,
    pub key_epoch: KeyEpoch,
}

impl WorkClaimPayload {
    pub fn to_signing_bytes(&self) -> Result<Vec<u8>> {
        to_canonical(self)
    }

    pub fn total_tokens(&self) -> Result<u64> {
        self.prompt_tokens
            .checked_add(self.completion_tokens)
            .ok_or(LedgerError::Overflow("prompt + completion tokens"))
    }

    pub fn sign(self, key: &SigningKey) -> Result<WorkClaim> {
        let sig = key.sign(&self.to_signing_bytes()?);
        Ok(WorkClaim {
            payload: self,
            provider_sig: sig.to_bytes().to_vec(),
        })
    }
}

/// A [`WorkClaimPayload`] plus the provider's signature. On its own this is
/// **unpayable**: a provider holding only a claim has no counter-party
/// agreement. Callers should record it (to track peers who refuse to sign) but
/// must never count it as earned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkClaim {
    pub payload: WorkClaimPayload,
    pub provider_sig: Vec<u8>,
}

impl WorkClaim {
    pub fn verify(&self) -> Result<()> {
        verify_sig(
            &self.payload.provider_did,
            &self.payload.to_signing_bytes()?,
            &self.provider_sig,
            "provider",
        )
    }

    /// Everything a consumer must check before counter-signing.
    ///
    /// Two *independent* checks are required, and both matter:
    ///
    /// * `response_body` — the raw bytes the consumer received. The digest binds
    ///   the claim to one specific delivery, so a provider cannot later re-point
    ///   the receipt at a different (larger) response.
    /// * `observed_*_tokens` — the counts the consumer parsed **itself** out of
    ///   that body. The digest alone does *not* constrain the claimed counts: a
    ///   provider can present an honest digest of the real bytes while claiming
    ///   any token count it likes. Only comparing against independently parsed
    ///   counts closes that hole.
    ///
    /// Parsing is the caller's job so this crate stays agnostic of any one
    /// inference provider's response schema (Ollama's `eval_count` /
    /// `prompt_eval_count`, today).
    pub fn verify_against(
        &self,
        grant: &SignedLeaseGrant,
        response_body: &[u8],
        observed_prompt_tokens: u64,
        observed_completion_tokens: u64,
    ) -> Result<()> {
        self.verify()?;
        grant.verify()?;

        let q = &grant.quote;
        let p = &self.payload;
        if p.lease_id != q.lease_id {
            return Err(LedgerError::QuoteMismatch(format!(
                "lease_id {} != quoted {}",
                p.lease_id, q.lease_id
            )));
        }
        if p.provider_did != q.provider_did || p.consumer_did != q.consumer_did {
            return Err(LedgerError::QuoteMismatch(
                "claim parties differ from the quoted parties".into(),
            ));
        }
        if p.response_digest != digest(response_body) {
            return Err(LedgerError::DigestMismatch);
        }
        if p.prompt_tokens != observed_prompt_tokens
            || p.completion_tokens != observed_completion_tokens
        {
            return Err(LedgerError::TokenCountMismatch {
                claimed_prompt: p.prompt_tokens,
                claimed_completion: p.completion_tokens,
                observed_prompt: observed_prompt_tokens,
                observed_completion: observed_completion_tokens,
            });
        }
        let expected = q.credits_for_tokens(p.total_tokens()?)?;
        if p.credits_owed != expected {
            return Err(LedgerError::QuoteMismatch(format!(
                "credits_owed {} != {expected} implied by the quoted price",
                p.credits_owed
            )));
        }
        Ok(())
    }

    /// Counter-sign, producing a payable [`Receipt`].
    pub fn counter_sign(self, key: &SigningKey, consumer_key_epoch: KeyEpoch) -> Result<Receipt> {
        let sig = key.sign(&self.to_receipt_signing_bytes()?);
        Ok(Receipt {
            claim: self,
            consumer_sig: sig.to_bytes().to_vec(),
            consumer_key_epoch,
        })
    }

    /// The consumer signs over the provider's *signed* claim, so its signature
    /// commits to the provider's signature too — neither side can later swap in
    /// a different claim body.
    fn to_receipt_signing_bytes(&self) -> Result<Vec<u8>> {
        to_canonical(self)
    }

    /// Content address of the claim itself, computable by **either** party from
    /// the claim alone.
    ///
    /// This is deliberately not [`Receipt::receipt_id`]: that one hashes the
    /// whole co-signed receipt, so it depends on `consumer_sig` and a provider
    /// holding an unsigned claim cannot compute it. The store keys outstanding
    /// claims on *this* id precisely so a later counter-signature can be matched
    /// back to the claim it settles.
    pub fn claim_id(&self) -> Result<[u8; 32]> {
        Ok(digest(&to_canonical(self)?))
    }
}

// ── Receipt ───────────────────────────────────────────────────────────────────

/// A fully co-signed record of delivered work: the only artifact that counts
/// toward earned or spent credits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub claim: WorkClaim,
    pub consumer_sig: Vec<u8>,
    pub consumer_key_epoch: KeyEpoch,
}

impl Receipt {
    /// Verify both signatures. Does not re-check the response digest — that
    /// requires the original bytes and is the consumer's job at counter-sign
    /// time via [`WorkClaim::verify_against`].
    pub fn verify(&self) -> Result<()> {
        self.claim.verify()?;
        verify_sig(
            &self.claim.payload.consumer_did,
            &self.claim.to_receipt_signing_bytes()?,
            &self.consumer_sig,
            "consumer",
        )
    }

    /// Stable content address: SHA-256 over the canonical encoding of the whole
    /// co-signed receipt. Used as the primary key for idempotent storage, so a
    /// replayed receipt is rejected by a uniqueness constraint rather than by
    /// application logic.
    pub fn receipt_id(&self) -> Result<[u8; 32]> {
        Ok(digest(&to_canonical(self)?))
    }

    pub fn receipt_id_hex(&self) -> Result<String> {
        Ok(hex::encode(self.receipt_id()?))
    }

    /// Content address of the underlying claim — the key under which the
    /// provider may already be holding this work as outstanding.
    pub fn claim_id(&self) -> Result<[u8; 32]> {
        self.claim.claim_id()
    }

    pub fn credits(&self) -> MicroCredits {
        self.claim.payload.credits_owed
    }

    pub fn provider_did(&self) -> &str {
        &self.claim.payload.provider_did
    }

    pub fn consumer_did(&self) -> &str {
        &self.claim.payload.consumer_did
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Canonical (positional, deterministic) msgpack encoding — the signing
/// preimage for every type in this crate.
fn to_canonical<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    rmp_serde::to_vec(v).map_err(|e| LedgerError::Encode(e.to_string()))
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().into()
}

/// Resolve a `did:peer:` to its Ed25519 verifying key. The DID *is* the key —
/// there is no registry and no network lookup.
fn verifying_key_for(did: &str) -> Result<VerifyingKey> {
    let peer_id =
        kwaai_trust::did_to_peer_id(did).ok_or_else(|| LedgerError::UnresolvableDid(did.into()))?;
    let raw = kwaai_trust::did::extract_ed25519_bytes(&peer_id)
        .ok_or_else(|| LedgerError::UnresolvableDid(did.into()))?;
    VerifyingKey::from_bytes(&raw).map_err(|_| LedgerError::UnresolvableDid(did.into()))
}

fn verify_sig(did: &str, preimage: &[u8], sig: &[u8], role: &'static str) -> Result<()> {
    if sig.len() != SIG_LEN {
        return Err(LedgerError::BadSignatureLength(sig.len()));
    }
    let mut buf = [0u8; SIG_LEN];
    buf.copy_from_slice(sig);
    verifying_key_for(did)?
        .verify(preimage, &Signature::from_bytes(&buf))
        .map_err(|_| LedgerError::BadSignature {
            role,
            did: did.to_string(),
        })
}

/// Compute the digest of a response body — exposed so callers on both sides use
/// exactly the same function rather than reimplementing SHA-256 framing.
pub fn response_digest(body: &[u8]) -> [u8; 32] {
    digest(body)
}

/// Encode any ledger artifact for the wire or for storage, using the same
/// canonical encoding as the signing preimage.
pub fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    to_canonical(v)
}

/// Decode a ledger artifact produced by [`encode`].
pub fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T> {
    rmp_serde::from_slice(bytes).map_err(|e| LedgerError::Encode(e.to_string()))
}

/// Shared fixtures for this crate's tests. Lives outside `mod tests` so the
/// `store` module's tests can use the same identities and receipt builders
/// rather than duplicating them.
#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use libp2p::identity::Keypair;

    /// A test identity: its `did:peer:` form and a dalek signing key derived
    /// from the same secret bytes.
    pub struct Id {
        pub did: String,
        pub signing: SigningKey,
    }

    pub fn make_id() -> Id {
        let kp = Keypair::generate_ed25519();
        let did = kwaai_trust::peer_id_to_did(&kp.public().to_peer_id());
        // libp2p's `secret()` yields its own SecretKey wrapper; dalek wants the
        // raw 32 bytes. This is the same conversion a caller in kwaai-cli makes
        // from NodeIdentity.
        let ed = kp.try_into_ed25519().unwrap();
        let raw: [u8; 32] = ed.secret().as_ref().try_into().unwrap();
        Id {
            did,
            signing: SigningKey::from_bytes(&raw),
        }
    }

    pub fn quote(provider: &Id, consumer: &Id) -> LeaseQuote {
        LeaseQuote {
            lease_id: 7,
            provider_did: provider.did.clone(),
            consumer_did: consumer.did.clone(),
            model: "llama3.1:8b".into(),
            price_micro_per_1k_tokens: 2_000,
            ttl_secs: 30,
            granted_at_unix_ms: 1_770_000_000_000,
            nonce: 42,
            key_epoch: 1,
        }
    }

    pub fn claim_for(
        grant: &SignedLeaseGrant,
        provider: &Id,
        body: &[u8],
        prompt: u64,
        completion: u64,
    ) -> WorkClaim {
        let q = &grant.quote;
        let credits = q.credits_for_tokens(prompt + completion).unwrap();
        WorkClaimPayload {
            lease_id: q.lease_id,
            request_id: 1,
            provider_did: q.provider_did.clone(),
            consumer_did: q.consumer_did.clone(),
            prompt_tokens: prompt,
            completion_tokens: completion,
            response_digest: response_digest(body),
            credits_owed: credits,
            valid_until_unix_ms: q.granted_at_unix_ms + 60_000,
            nonce: 99,
            key_epoch: 1,
        }
        .sign(&provider.signing)
        .unwrap()
    }

    /// A complete, co-signed receipt for the given parties.
    pub fn full_receipt(
        provider: &Id,
        consumer: &Id,
        body: &[u8],
        prompt: u64,
        completion: u64,
    ) -> Receipt {
        let grant = quote(provider, consumer).sign(&provider.signing).unwrap();
        claim_for(&grant, provider, body, prompt, completion)
            .counter_sign(&consumer.signing, 1)
            .unwrap()
    }

    /// A provider-only claim, as a provider would actually hold it: signed by
    /// the provider and nothing else. Deliberately does *not* reach for the
    /// consumer's key — an earlier version of this fixture counter-signed just
    /// to obtain a receipt id, which hid the fact that a real provider cannot
    /// compute one.
    pub fn unsigned_claim(
        provider: &Id,
        consumer: &Id,
        body: &[u8],
        prompt: u64,
        completion: u64,
    ) -> WorkClaim {
        let grant = quote(provider, consumer).sign(&provider.signing).unwrap();
        claim_for(&grant, provider, body, prompt, completion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::*;

    // ── The load-bearing test ────────────────────────────────────────────────

    /// The property whose absence hid the nondeterministic-signature bug in
    /// `kwaai-trust`: a receipt must still verify after a full encode → bytes →
    /// decode round-trip, i.e. across a process boundary, not merely in memory.
    #[test]
    fn receipt_verifies_after_wire_roundtrip() {
        let (p, c) = (make_id(), make_id());
        let body = br#"{"message":{"content":"hi"},"eval_count":12}"#;
        let grant = quote(&p, &c).sign(&p.signing).unwrap();
        let receipt = claim_for(&grant, &p, body, 5, 12)
            .counter_sign(&c.signing, 1)
            .unwrap();

        let bytes = rmp_serde::to_vec(&receipt).unwrap();
        let decoded: Receipt = rmp_serde::from_slice(&bytes).unwrap();

        assert!(
            decoded.verify().is_ok(),
            "receipt must verify after a wire round-trip"
        );
        assert_eq!(
            decoded.receipt_id().unwrap(),
            receipt.receipt_id().unwrap(),
            "receipt_id must be stable across the round-trip"
        );
        assert_eq!(decoded, receipt);
    }

    /// Encoding must be byte-identical every time, from independently
    /// constructed values — the actual determinism guarantee.
    #[test]
    fn signing_preimage_is_byte_stable_across_instances() {
        let (p, c) = (make_id(), make_id());
        let a = quote(&p, &c).to_signing_bytes().unwrap();
        for _ in 0..50 {
            assert_eq!(
                a,
                quote(&p, &c).to_signing_bytes().unwrap(),
                "signing preimage must be deterministic"
            );
        }
    }

    /// Positional encoding should be materially smaller than named, since this
    /// rides a per-request path.
    #[test]
    fn positional_encoding_is_smaller_than_named() {
        let (p, c) = (make_id(), make_id());
        let q = quote(&p, &c);
        let positional = rmp_serde::to_vec(&q).unwrap().len();
        let named = rmp_serde::to_vec_named(&q).unwrap().len();
        assert!(
            positional < named,
            "positional {positional} should be smaller than named {named}"
        );
    }

    // ── Adversarial ─────────────────────────────────────────────────────────

    /// Regression: an earlier version of `verify_against` checked only the
    /// digest, which does **not** constrain the claimed token counts — a
    /// provider could present an honest digest of the real bytes while claiming
    /// any count. Independently observed counts are what close that hole.
    #[test]
    fn inflated_token_count_is_rejected_even_with_an_honest_digest() {
        let (p, c) = (make_id(), make_id());
        let body = br#"{"eval_count":12}"#;
        let grant = quote(&p, &c).sign(&p.signing).unwrap();

        // Provider inflates completion_tokens, recomputes credits to stay
        // internally consistent, and re-signs — so its own signature is valid
        // and the digest genuinely matches the bytes it delivered.
        let mut payload = claim_for(&grant, &p, body, 5, 12).payload;
        payload.completion_tokens = 100_000;
        payload.credits_owed = grant
            .quote
            .credits_for_tokens(payload.total_tokens().unwrap())
            .unwrap();
        let inflated = payload.sign(&p.signing).unwrap();

        assert!(
            inflated.verify().is_ok(),
            "provider's own signature is valid — that is the point"
        );
        assert_eq!(
            inflated.payload.response_digest,
            response_digest(body),
            "and the digest is honest — so the digest alone cannot catch this"
        );

        // The consumer parsed 5 + 12 out of the body itself.
        let err = inflated.verify_against(&grant, body, 5, 12).unwrap_err();
        assert!(
            matches!(
                err,
                LedgerError::TokenCountMismatch {
                    claimed_completion: 100_000,
                    observed_completion: 12,
                    ..
                }
            ),
            "expected TokenCountMismatch, got {err:?}"
        );
    }

    #[test]
    fn claim_pointing_at_different_bytes_is_rejected_by_the_digest() {
        let (p, c) = (make_id(), make_id());
        let delivered = br#"{"eval_count":12}"#;
        let grant = quote(&p, &c).sign(&p.signing).unwrap();

        // Claim was built over different bytes than the consumer received.
        let claim = claim_for(&grant, &p, b"some other response", 5, 12);
        assert!(matches!(
            claim.verify_against(&grant, delivered, 5, 12).unwrap_err(),
            LedgerError::DigestMismatch
        ));
    }

    #[test]
    fn quote_replayed_against_a_different_consumer_is_rejected() {
        let (p, c, other) = (make_id(), make_id(), make_id());
        let grant = quote(&p, &c).sign(&p.signing).unwrap();

        // `other` receives a grant that names `c` as the consumer.
        assert_ne!(grant.quote.consumer_did, other.did);
        assert!(
            grant.verify().is_ok(),
            "signature is genuine; binding is what rejects it"
        );
        assert!(
            grant.quote.consumer_did != other.did,
            "a consumer must reject a grant naming someone else"
        );

        // A claim asserting `other` as consumer does not match the quote.
        let mut payload = claim_for(&grant, &p, b"x", 1, 1).payload;
        payload.consumer_did = other.did.clone();
        let mismatched = payload.sign(&p.signing).unwrap();
        assert!(matches!(
            mismatched.verify_against(&grant, b"x", 1, 1).unwrap_err(),
            LedgerError::QuoteMismatch(_)
        ));
    }

    #[test]
    fn receipt_replayed_under_a_different_lease_has_a_different_id() {
        let (p, c) = (make_id(), make_id());
        let body = b"same bytes";
        let g1 = quote(&p, &c).sign(&p.signing).unwrap();
        let mut q2 = quote(&p, &c);
        q2.lease_id = 8;
        let g2 = q2.sign(&p.signing).unwrap();

        let r1 = claim_for(&g1, &p, body, 5, 5)
            .counter_sign(&c.signing, 1)
            .unwrap();
        let r2 = claim_for(&g2, &p, body, 5, 5)
            .counter_sign(&c.signing, 1)
            .unwrap();

        assert_ne!(
            r1.receipt_id().unwrap(),
            r2.receipt_id().unwrap(),
            "identical work under a different lease must not collide"
        );
    }

    #[test]
    fn forged_consumer_signature_is_rejected() {
        let (p, c, attacker) = (make_id(), make_id(), make_id());
        let grant = quote(&p, &c).sign(&p.signing).unwrap();
        // Attacker counter-signs a claim addressed to `c`.
        let forged = claim_for(&grant, &p, b"x", 1, 1)
            .counter_sign(&attacker.signing, 1)
            .unwrap();

        assert!(matches!(
            forged.verify().unwrap_err(),
            LedgerError::BadSignature {
                role: "consumer",
                ..
            }
        ));
    }

    #[test]
    fn tampering_with_the_claim_body_invalidates_the_consumer_signature() {
        let (p, c) = (make_id(), make_id());
        let grant = quote(&p, &c).sign(&p.signing).unwrap();
        let mut receipt = claim_for(&grant, &p, b"x", 1, 1)
            .counter_sign(&c.signing, 1)
            .unwrap();
        assert!(receipt.verify().is_ok());

        // Because the consumer signs over the provider's *signed* claim,
        // swapping the body breaks the consumer signature too.
        receipt.claim.payload.credits_owed += 1;
        assert!(
            receipt.verify().is_err(),
            "mutating the claim must invalidate the receipt"
        );
    }

    #[test]
    fn wrong_length_signature_is_rejected_without_panicking() {
        let (p, c) = (make_id(), make_id());
        let mut grant = quote(&p, &c).sign(&p.signing).unwrap();
        grant.provider_sig.truncate(10);
        assert!(matches!(
            grant.verify().unwrap_err(),
            LedgerError::BadSignatureLength(10)
        ));
    }

    #[test]
    fn unresolvable_did_is_an_error_not_a_panic() {
        let (p, c) = (make_id(), make_id());
        let mut grant = quote(&p, &c).sign(&p.signing).unwrap();
        grant.quote.provider_did = "did:key:zNotAPeerDid".into();
        assert!(matches!(
            grant.verify().unwrap_err(),
            LedgerError::UnresolvableDid(_)
        ));
    }

    // ── Arithmetic ──────────────────────────────────────────────────────────

    #[test]
    fn credits_round_up_so_a_provider_is_never_underpaid() {
        let (p, c) = (make_id(), make_id());
        let q = quote(&p, &c); // 2_000 micro per 1k tokens
        assert_eq!(q.credits_for_tokens(1000).unwrap(), 2_000);
        assert_eq!(q.credits_for_tokens(0).unwrap(), 0);
        // A single token still costs something rather than rounding to zero.
        assert_eq!(q.credits_for_tokens(1).unwrap(), 2);
        assert_eq!(q.credits_for_tokens(1500).unwrap(), 3_000);
    }

    #[test]
    fn credit_arithmetic_overflow_is_an_error_not_a_wrap() {
        let (p, c) = (make_id(), make_id());
        let mut q = quote(&p, &c);
        q.price_micro_per_1k_tokens = u64::MAX;
        assert!(matches!(
            q.credits_for_tokens(u64::MAX).unwrap_err(),
            LedgerError::Overflow(_)
        ));
    }

    #[test]
    fn a_valid_exchange_verifies_end_to_end() {
        let (p, c) = (make_id(), make_id());
        let body = br#"{"message":{"content":"answer"},"eval_count":40}"#;
        let grant = quote(&p, &c).sign(&p.signing).unwrap();
        let claim = claim_for(&grant, &p, body, 10, 40);

        // Consumer independently parsed 10 prompt + 40 completion from `body`.
        claim
            .verify_against(&grant, body, 10, 40)
            .expect("claim is honest");
        let receipt = claim.counter_sign(&c.signing, 1).unwrap();
        receipt.verify().expect("co-signed receipt verifies");

        assert_eq!(receipt.credits(), 100); // 50 tokens @ 2_000/1k, rounded up
        assert_eq!(receipt.provider_did(), p.did);
        assert_eq!(receipt.consumer_did(), c.did);
        assert_eq!(receipt.receipt_id_hex().unwrap().len(), 64);
    }
}
