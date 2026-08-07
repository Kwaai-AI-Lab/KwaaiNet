//! Node-local ledger wiring — the integration layer between this node's
//! identity, the `kwaai-ledger` artifact types, and the on-disk receipt store.
//!
//! Everything here is **best-effort by construction**. A node with an RSA
//! bootstrap key, an unwritable `~/.kwaainet/`, or a corrupt `ledger.db` must
//! still serve and consume inference exactly as it did before this feature
//! existed. So `LedgerNode::load()` returns `Option`, and every method that
//! could fail logs and returns `None`/`false` rather than propagating an error
//! into a dispatch path. Accounting is allowed to be unavailable; inference is
//! not allowed to break because accounting was unavailable.
//!
//! ## Who calls what
//!
//! * **Provider** (`handle_mux_stream_server`): [`sign_quote`] at lease grant,
//!   then [`claim_for_response`] after serving, then [`record_unsigned_claim`]
//!   immediately (so a consumer that never counter-signs is still visible), and
//!   finally [`record_receipt`] when the `ReceiptAck` comes back.
//! * **Consumer** (`InferenceMuxClient`): [`counter_sign`] on each claim it
//!   receives, which both verifies and persists.
//!
//! [`sign_quote`]: LedgerNode::sign_quote
//! [`claim_for_response`]: LedgerNode::claim_for_response
//! [`record_unsigned_claim`]: LedgerNode::record_unsigned_claim
//! [`record_receipt`]: LedgerNode::record_receipt
//! [`counter_sign`]: LedgerNode::counter_sign

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use ed25519_dalek::SigningKey;
use libp2p::PeerId;
use tracing::{debug, warn};

use kwaai_ledger::{
    economy_by_id, ClaimExt, Economy, LeaseQuote, LedgerStore, MicroCredits, ObservedWork,
    PeerBalance, Receipt, Settlement, SettlementContext, SignedLeaseGrant, WorkClaim,
    WorkClaimPayload, WORK_KIND_CHAT, WORK_KIND_EMBEDDINGS,
};

use crate::config::kwaainet_dir;
use crate::identity::{NodeIdentity, KEY_EPOCH};

/// Which currency experiment this node participates in, when nothing is
/// configured.
///
/// The community is A/B testing currency models a month at a time, so this is
/// expected to change between releases. It is safe to default to a live economy
/// because nothing is gated on balances yet — settlement is computed from
/// receipts the node already holds and is reported, not enforced.
pub const DEFAULT_ECONOMY: &str = "miles";

/// Fallback price when neither the economy's rate card nor
/// `KWAAINET_LEDGER_PRICE_MICRO_PER_1K` applies.
pub const DEFAULT_PRICE_MICRO_PER_1K_TOKENS: MicroCredits = 1_000;

/// How long after issue a claim may still be counter-signed. Generous relative
/// to the 30s lease TTL: the lease governs *capacity*, this governs how long a
/// provider can sit on an unsigned claim before it becomes worthless.
const CLAIM_VALIDITY_MS: u64 = 300_000;

/// A peer whose unsigned-claim ratio exceeds this has been served work it
/// declined to acknowledge more often than not. Advisory only in Phase 1 —
/// surfaced by `kwaainet ledger show`, not yet an admission gate, because
/// denying leases on ledger grounds would let a bug in *this* code partition
/// the network. See the deferred list in the Ledger plan.
pub const UNSIGNED_RATIO_WARN: f64 = 0.5;

/// `did:peer:` DID for an authenticated libp2p peer.
///
/// This is the whole point of the authenticated-caller plumbing added in Phase
/// 0: `did:peer:` is self-certifying, so a DID derived from the *transport's*
/// PeerId is a verified counterparty identity, not a self-declared one.
pub fn did_for_peer(peer: &PeerId) -> String {
    kwaai_trust::did::peer_id_to_did(peer)
}

pub struct LedgerNode {
    did: String,
    signing: SigningKey,
    /// `rusqlite::Connection` is `!Sync`, and receipt writes are sub-millisecond
    /// single-row statements, so a blocking mutex is the right tool. Every
    /// critical section below is await-free — do not introduce an `.await`
    /// while this is held.
    store: Mutex<LedgerStore>,
    price_micro_per_1k_tokens: MicroCredits,
    /// The currency model this node is running. Pricing comes from its rate
    /// card; settlement from its issuance rules.
    economy: Box<dyn Economy>,
    /// Nonces make otherwise-identical exchanges produce distinct
    /// `receipt_id`s. Seeded from the wall clock so a restarted node doesn't
    /// re-mint the nonce sequence it already used; monotonic within a process.
    nonces: AtomicU64,
}

impl LedgerNode {
    /// Load this node's ledger, or `None` if it can't participate.
    ///
    /// Never returns an error: the caller is a dispatch path that must keep
    /// working regardless.
    pub fn load() -> Option<Arc<Self>> {
        let identity = match NodeIdentity::load_or_create() {
            Ok(i) => i,
            Err(e) => {
                warn!("ledger disabled: cannot load node identity: {e:#}");
                return None;
            }
        };
        let did = identity.did();

        let secret = match identity.ed25519_secret_bytes() {
            Ok(s) => s,
            Err(e) => {
                // Expected on RSA bootstrap identities — informational, not a
                // problem to be fixed on this code path.
                debug!("ledger disabled: this node has no Ed25519 identity key ({e:#})");
                return None;
            }
        };

        let path = LedgerStore::default_path(&kwaainet_dir());
        let store = match LedgerStore::open(&path, did.clone()) {
            Ok(s) => s,
            Err(e) => {
                warn!("ledger disabled: cannot open {}: {e}", path.display());
                return None;
            }
        };

        // Env beats config so an operator can switch experiment for a single
        // run without editing YAML; config beats the built-in default.
        let configured = std::env::var("KWAAINET_ECONOMY").ok().or_else(|| {
            crate::config::KwaaiNetConfig::load_or_create()
                .ok()
                .and_then(|c| c.economy)
        });
        let economy = economy_by_id(configured.as_deref().unwrap_or(DEFAULT_ECONOMY));

        // An explicit override still wins, so an operator can price against the
        // network without switching currency model.
        let price = std::env::var("KWAAINET_LEDGER_PRICE_MICRO_PER_1K")
            .ok()
            .and_then(|v| v.parse::<MicroCredits>().ok())
            .unwrap_or_else(|| match economy.rate_card().micro_per_1k_chat_tokens {
                0 => DEFAULT_PRICE_MICRO_PER_1K_TOKENS,
                rate => rate,
            });

        debug!(
            "ledger enabled: {} at {} — economy {}, {} micro/1k tokens",
            did,
            path.display(),
            economy.id(),
            price
        );

        Some(Arc::new(Self {
            did,
            signing: SigningKey::from_bytes(&secret),
            store: Mutex::new(store),
            price_micro_per_1k_tokens: price,
            economy,
            nonces: AtomicU64::new(now_unix_ms()),
        }))
    }

    /// The process-wide ledger, loaded once.
    ///
    /// A single handle is the right shape: there is one identity key and one
    /// `ledger.db` per process, and `load()` does file I/O plus a schema
    /// migration that has no business running per stream. Callers that already
    /// have a natural place to pass it (the mux server, from `node.rs`) should
    /// still take it as a parameter; this exists for the client paths that are
    /// constructed too deep to thread a parameter through.
    pub fn shared() -> Option<Arc<Self>> {
        static SHARED: std::sync::OnceLock<Option<Arc<LedgerNode>>> = std::sync::OnceLock::new();
        SHARED.get_or_init(Self::load).clone()
    }

    /// In-memory ledger for tests — same behavior, nothing touches disk.
    #[cfg(test)]
    pub fn in_memory(signing: SigningKey, did: String) -> Arc<Self> {
        Arc::new(Self {
            store: Mutex::new(LedgerStore::open_in_memory(did.clone()).expect("in-memory ledger")),
            did,
            signing,
            price_micro_per_1k_tokens: DEFAULT_PRICE_MICRO_PER_1K_TOKENS,
            economy: economy_by_id(DEFAULT_ECONOMY),
            nonces: AtomicU64::new(1),
        })
    }

    pub fn did(&self) -> &str {
        &self.did
    }

    pub fn price_micro_per_1k_tokens(&self) -> MicroCredits {
        self.price_micro_per_1k_tokens
    }

    pub fn economy(&self) -> &dyn Economy {
        self.economy.as_ref()
    }

    /// Settle this node's own receipts under the configured economy.
    ///
    /// Local and advisory: it reports what this node believes it has earned, from
    /// evidence it holds. Making that authoritative needs a clearing house, which
    /// is deliberately not in the month-1 experiment — the point is to compare
    /// how the models *feel*, not to resist fraud yet.
    pub fn settle(&self, ctx: &SettlementContext) -> anyhow::Result<Settlement> {
        let ledger = {
            let store = self
                .store
                .lock()
                .map_err(|e| anyhow::anyhow!("ledger store lock poisoned: {e}"))?;
            store.work_ledger()?
        };
        Ok(self.economy.settle(&ledger, ctx))
    }

    fn next_nonce(&self) -> u64 {
        self.nonces.fetch_add(1, Ordering::Relaxed)
    }

    // ── Provider side ─────────────────────────────────────────────────────────

    /// Sign the quote that accompanies a granted lease. The quote fixes the
    /// price *before* any work happens, which is what leaves only quantity to
    /// agree on afterwards.
    pub fn sign_quote(
        &self,
        lease_id: u64,
        consumer_did: String,
        model: String,
        ttl_secs: u32,
    ) -> Option<SignedLeaseGrant> {
        let quote = LeaseQuote {
            version: kwaai_ledger::PAYLOAD_VERSION,
            lease_id,
            provider_did: self.did.clone(),
            consumer_did,
            model,
            price_micro_per_1k_tokens: self.price_micro_per_1k_tokens,
            ttl_secs,
            granted_at_unix_ms: now_unix_ms(),
            nonce: self.next_nonce(),
            key_epoch: KEY_EPOCH,
            // Commit to the embedding price up front too, so neither price can
            // be chosen after the work is known.
            ext: match (kwaai_ledger::QuoteExt {
                price_micro_per_1k_embedding_tokens: self
                    .economy
                    .rate_card()
                    .micro_per_1k_embedding_tokens,
            })
            .encode()
            {
                Ok(b) => b,
                Err(e) => {
                    warn!("ledger: failed to encode quote extension: {e}");
                    return None;
                }
            },
        };
        match quote.sign(&self.signing) {
            Ok(g) => Some(g),
            Err(e) => {
                warn!("ledger: failed to sign lease quote: {e}");
                None
            }
        }
    }

    /// Build and sign a claim for one served response.
    ///
    /// Returns `None` when the response carries no measurable work — a non-200,
    /// a ragged embedding response, or any body with no usage block. That is the
    /// correct answer, not a failure: a claim the consumer cannot independently
    /// re-derive would just be refused.
    pub fn claim_for_response(
        &self,
        quote: &LeaseQuote,
        request_id: u64,
        response_body: &[u8],
    ) -> Option<WorkClaim> {
        let observed = parse_work(response_body)?;
        let total = observed
            .prompt_tokens
            .checked_add(observed.completion_tokens)?;
        // Embedding work is prefill only, so it is priced off its own rate rather
        // than the quoted chat price.
        // Always priced off the signed quote, whichever kind of work it was —
        // the consumer verifies against the same commitment.
        let credits_owed = match quote.credits_for(observed.kind, total) {
            Ok(c) => c,
            Err(e) => {
                warn!("ledger: credit arithmetic failed for request {request_id}: {e}");
                return None;
            }
        };

        let ext = if observed.kind == WORK_KIND_EMBEDDINGS {
            ClaimExt::embeddings(observed.vectors, observed.dimension)
        } else {
            ClaimExt::chat()
        };
        let payload = WorkClaimPayload {
            version: kwaai_ledger::PAYLOAD_VERSION,
            lease_id: quote.lease_id,
            request_id,
            provider_did: self.did.clone(),
            consumer_did: quote.consumer_did.clone(),
            prompt_tokens: observed.prompt_tokens,
            completion_tokens: observed.completion_tokens,
            response_digest: kwaai_ledger::response_digest(response_body),
            credits_owed,
            valid_until_unix_ms: now_unix_ms() + CLAIM_VALIDITY_MS,
            nonce: self.next_nonce(),
            key_epoch: KEY_EPOCH,
            ext: match ext.encode() {
                Ok(b) => b,
                Err(e) => {
                    warn!("ledger: failed to encode claim extension: {e}");
                    return None;
                }
            },
        };
        match payload.sign(&self.signing) {
            Ok(c) => Some(c),
            Err(e) => {
                warn!("ledger: failed to sign work claim: {e}");
                None
            }
        }
    }

    /// Record a claim we have sent but not yet had counter-signed. Called
    /// immediately on issue, *not* after a timeout: a consumer that vanishes
    /// mid-exchange should look the same as one that refuses, and both should be
    /// visible. `record_receipt` retires the matching row if the counter-signature
    /// does arrive.
    pub fn record_unsigned_claim(&self, claim: &WorkClaim) {
        let store = match self.store.lock() {
            Ok(s) => s,
            Err(e) => {
                warn!("ledger: store lock poisoned: {e}");
                return;
            }
        };
        if let Err(e) = store.record_unsigned_claim(claim) {
            warn!("ledger: failed to record unsigned claim: {e}");
        }
    }

    // ── Consumer side ─────────────────────────────────────────────────────────

    /// Verify a provider's claim against the quote and the bytes we actually
    /// received, counter-sign it, and persist the resulting receipt.
    ///
    /// The token counts are parsed here from `response_body` — the consumer's
    /// *own* reading of the response, never the provider's assertion. That
    /// independent parse is what closes the inflated-count hole; see
    /// `WorkClaim::verify_against`.
    pub fn counter_sign(
        &self,
        claim: WorkClaim,
        grant: &SignedLeaseGrant,
        response_body: &[u8],
    ) -> Option<Receipt> {
        let observed = match parse_work(response_body) {
            Some(w) => w,
            None => {
                // The provider claimed work on a body we can't meter. Refusing
                // is correct: signing a quantity we cannot verify is exactly
                // what this design exists to avoid.
                warn!("ledger: refusing to counter-sign — no measurable work in the response body");
                return None;
            }
        };

        if let Err(e) = claim.verify_against(grant, response_body, &observed) {
            warn!(
                "ledger: refusing to counter-sign claim from {}: {e}",
                claim.payload.provider_did
            );
            return None;
        }

        let receipt = match claim.counter_sign(&self.signing, KEY_EPOCH) {
            Ok(r) => r,
            Err(e) => {
                warn!("ledger: failed to counter-sign: {e}");
                return None;
            }
        };
        self.record_receipt(&receipt);
        Some(receipt)
    }

    // ── Both sides ────────────────────────────────────────────────────────────

    /// Persist a fully co-signed receipt. Verification happens inside the store,
    /// so an unverifiable receipt (a forged `ReceiptAck`, say) is never counted.
    pub fn record_receipt(&self, receipt: &Receipt) {
        let store = match self.store.lock() {
            Ok(s) => s,
            Err(e) => {
                warn!("ledger: store lock poisoned: {e}");
                return;
            }
        };
        match store.record_receipt(receipt) {
            Ok(true) => debug!(
                "ledger: recorded receipt {} ({} micro)",
                receipt.receipt_id_hex().unwrap_or_default(),
                receipt.credits()
            ),
            // Idempotent replay — a duplicate ack is not an error.
            Ok(false) => debug!("ledger: receipt already recorded, ignoring replay"),
            Err(e) => warn!("ledger: failed to record receipt: {e}"),
        }
    }

    /// Whether we already hold a receipt with this content address. Used to
    /// confirm two parties really converged on the same artifact rather than on
    /// two receipts that merely agree numerically.
    ///
    /// Test-only for now — the replay path doesn't need it, because
    /// counter-signing is deterministic and `record_receipt` already rejects a
    /// duplicate `receipt_id` on the primary key.
    #[cfg(test)]
    pub fn has_receipt(&self, receipt_id: &[u8; 32]) -> anyhow::Result<bool> {
        let store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("ledger store lock poisoned: {e}"))?;
        Ok(store.has_receipt(receipt_id)?)
    }

    pub fn balances(&self) -> anyhow::Result<Vec<PeerBalance>> {
        let store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("ledger store lock poisoned: {e}"))?;
        Ok(store.balances()?)
    }

    pub fn totals(&self) -> anyhow::Result<(MicroCredits, MicroCredits, u64)> {
        let store = self
            .store
            .lock()
            .map_err(|e| anyhow::anyhow!("ledger store lock poisoned: {e}"))?;
        Ok(store.totals()?)
    }
}

/// Everything a party can independently derive from a delivered response body.
///
/// Tries embeddings first because their shape is unambiguous — an `embeddings`
/// array — whereas chat is identified by its usage counters. Both parties run
/// this same function over the same bytes, so the interpretation is agreed, not
/// asserted; the derived `kind` is then signed into the claim and re-checked by
/// `WorkClaim::verify_against`.
pub fn parse_work(body: &[u8]) -> Option<ObservedWork> {
    if let Some(w) = parse_embedding_work(body) {
        return Some(w);
    }
    let (prompt_tokens, completion_tokens) = parse_token_counts(body)?;
    Some(ObservedWork {
        kind: WORK_KIND_CHAT,
        prompt_tokens,
        completion_tokens,
        vectors: 0,
        dimension: 0,
    })
}

/// Ollama `/api/embed`: `{"embeddings": [[...], ...], "prompt_eval_count": N}`.
///
/// Billed on `prompt_eval_count` so embeddings share one unit with chat and need
/// no exchange rate between them — an embedding pass is pure prefill, which is
/// why it gets its own (cheaper) line on the rate card rather than its own unit.
/// Vector count and dimension are carried as corroborating evidence: both are
/// countable straight from the delivered bytes, which makes them the most
/// directly verifiable quantity in the system.
///
/// Never streamed — `/api/embed` returns a single document — so there is no
/// NDJSON tail to scan here.
fn parse_embedding_work(body: &[u8]) -> Option<ObservedWork> {
    let v: serde_json::Value = serde_json::from_slice(body).ok()?;
    let rows = v.get("embeddings")?.as_array()?;
    if rows.is_empty() {
        return None;
    }
    let dimension = rows[0].as_array()?.len() as u32;
    // A ragged response is not meterable: the two parties could disagree on what
    // "dimension" means, and a claim neither can reproduce is worthless.
    if rows
        .iter()
        .any(|r| r.as_array().map(|a| a.len() as u32) != Some(dimension))
    {
        return None;
    }
    Some(ObservedWork {
        kind: WORK_KIND_EMBEDDINGS,
        // Absent on some builds; the vector count still bounds the work.
        prompt_tokens: v
            .get("prompt_eval_count")
            .and_then(|x| x.as_u64())
            .unwrap_or(0),
        completion_tokens: 0,
        vectors: rows.len() as u64,
        dimension,
    })
}

/// How many trailing chunks of a streamed body to examine when looking for the
/// usage block.
///
/// The block is always in the final chunk (Ollama's `"done":true` line) or the
/// one before it (an SSE usage frame ahead of `data: [DONE]`), so a small window
/// suffices and bounds the work on a large or malformed body. Both parties apply
/// the same constant, so they always reach the same answer.
const STREAM_TAIL_CHUNKS: usize = 64;

/// Prompt and completion token counts from an inference response body.
///
/// Handles every shape this node actually receives, because `resolve_inference_urls`
/// fronts a proxy that callers hit with any of:
///
/// * Ollama native, non-streaming: top-level `prompt_eval_count` / `eval_count`.
/// * Ollama native, **streaming** (`"stream": true`): NDJSON, where only the final
///   `"done":true` line carries the counts.
/// * OpenAI-compatible: `usage.prompt_tokens` / `usage.completion_tokens`, either
///   in a single document or in a trailing SSE `data:` frame.
///
/// The streaming case is not hypothetical — it is the dominant one. The heaviest
/// remote-GPU consumers in this repo all set `"stream": true` (`kwaai-rag`'s
/// `graph.rs` entity extraction and `sequence.rs` temporal extraction), because
/// streaming makes Ollama send headers immediately and avoids a relay send
/// timeout. Parsing only the whole body as one JSON document therefore meant
/// every graph build and dream cycle went completely unbilled.
///
/// `None` when the body carries no countable work — embeddings, upstream errors,
/// or a streamed OpenAI-compatible response, since Ollama's `/v1` endpoint emits
/// no usage block at all even with `stream_options.include_usage`. Declining is
/// the right answer there: a guessed count would be signed.
///
/// Both parties run this same function over the same bytes, so agreement is
/// automatic — which is why it lives here rather than in `kwaai-ledger`, whose
/// job is to stay agnostic of any one provider's response schema.
pub fn parse_token_counts(body: &[u8]) -> Option<(u64, u64)> {
    // Fast path: a single non-streaming JSON document.
    if let Ok(v) = serde_json::from_slice::<serde_json::Value>(body) {
        if let Some(counts) = counts_from_value(&v) {
            return Some(counts);
        }
    }

    // Streaming: NDJSON or SSE. Scan backwards — the usage block is always at
    // the end, and stopping at the first chunk that has one avoids walking the
    // whole transcript.
    body.split(|b| *b == b'\n')
        .rev()
        .filter_map(|line| {
            // Tolerate an SSE `data:` prefix so an OpenAI-compatible server that
            // does emit a usage frame is metered correctly. `data: [DONE]` is not
            // valid JSON and falls out here.
            let line = line.trim_ascii();
            let line = line.strip_prefix(b"data:").unwrap_or(line).trim_ascii();
            (!line.is_empty()).then_some(line)
        })
        .take(STREAM_TAIL_CHUNKS)
        .filter_map(|line| serde_json::from_slice::<serde_json::Value>(line).ok())
        .find_map(|v| counts_from_value(&v))
}

/// Pull counts out of one decoded chunk, in either vendor's spelling.
fn counts_from_value(v: &serde_json::Value) -> Option<(u64, u64)> {
    if let (Some(p), Some(c)) = (
        v.get("prompt_eval_count").and_then(|x| x.as_u64()),
        v.get("eval_count").and_then(|x| x.as_u64()),
    ) {
        return Some((p, c));
    }

    let usage = v.get("usage")?;
    let p = usage.get("prompt_tokens").and_then(|x| x.as_u64())?;
    let c = usage.get("completion_tokens").and_then(|x| x.as_u64())?;
    Some((p, c))
}

fn now_unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kwaai_ledger::MilesEconomy;

    fn keypair() -> (SigningKey, String) {
        // A deterministic key is fine: nothing here depends on secrecy, and a
        // fixed key keeps failures reproducible.
        let signing = SigningKey::from_bytes(&[7u8; 32]);
        let pk = libp2p::identity::ed25519::PublicKey::try_from_bytes(
            signing.verifying_key().as_bytes(),
        )
        .expect("valid ed25519 public key");
        let peer = PeerId::from_public_key(&libp2p::identity::PublicKey::from(pk));
        (signing, did_for_peer(&peer))
    }

    fn other_keypair() -> (SigningKey, String) {
        let signing = SigningKey::from_bytes(&[9u8; 32]);
        let pk = libp2p::identity::ed25519::PublicKey::try_from_bytes(
            signing.verifying_key().as_bytes(),
        )
        .expect("valid ed25519 public key");
        let peer = PeerId::from_public_key(&libp2p::identity::PublicKey::from(pk));
        (signing, did_for_peer(&peer))
    }

    const OLLAMA_BODY: &[u8] =
        br#"{"model":"m","message":{"content":"hi"},"prompt_eval_count":120,"eval_count":40}"#;

    #[test]
    fn parses_ollama_native_counts() {
        assert_eq!(parse_token_counts(OLLAMA_BODY), Some((120, 40)));
    }

    #[test]
    fn parses_openai_compatible_usage() {
        let body = br#"{"choices":[],"usage":{"prompt_tokens":11,"completion_tokens":22,"total_tokens":33}}"#;
        assert_eq!(parse_token_counts(body), Some((11, 22)));
    }

    // Captured verbatim from a live `POST /api/chat` with `"stream": true` —
    // the exact shape kwaai-rag's graph and sequence extraction produce, and the
    // one that used to bill nothing at all.
    const OLLAMA_STREAMED: &[u8] = br#"{"model":"llama3.2:3b","created_at":"2026-08-07T02:10:28.8642Z","message":{"role":"assistant","content":"OK"},"done":false}
{"model":"llama3.2:3b","created_at":"2026-08-07T02:10:28.874443Z","message":{"role":"assistant","content":""},"done":true,"done_reason":"stop","total_duration":21803544250,"load_duration":21730667875,"prompt_eval_count":27,"prompt_eval_duration":49601000,"eval_count":2,"eval_duration":11555000}
"#;

    #[test]
    fn parses_a_streamed_ollama_response() {
        // The regression that matters: this is the dominant remote-GPU path in
        // the repo and it produced no claim whatsoever before the fix.
        assert_eq!(parse_token_counts(OLLAMA_STREAMED), Some((27, 2)));
    }

    #[test]
    fn a_streamed_response_bills_the_same_as_its_non_streamed_twin() {
        // Both parties must agree regardless of transport framing, or a provider
        // could pick the encoding that pays better.
        let non_streamed =
            br#"{"message":{"content":"OK"},"done":true,"prompt_eval_count":27,"eval_count":2}"#;
        assert_eq!(
            parse_token_counts(OLLAMA_STREAMED),
            parse_token_counts(non_streamed)
        );
    }

    #[test]
    fn streamed_parsing_tolerates_ragged_line_endings() {
        // Trailing newlines, blank lines and \r\n all occur in practice; none of
        // them may change the amount billed.
        let base = std::str::from_utf8(OLLAMA_STREAMED).unwrap().trim_end();
        for variant in [
            base.to_string(),
            format!("{base}\n"),
            format!("{base}\n\n\n"),
            base.replace('\n', "\r\n"),
        ] {
            assert_eq!(
                parse_token_counts(variant.as_bytes()),
                Some((27, 2)),
                "framing changed the bill: {variant:?}"
            );
        }
    }

    #[test]
    fn parses_a_trailing_sse_usage_frame() {
        // An OpenAI-compatible server that does emit usage before [DONE].
        let body = b"data: {\"choices\":[{\"delta\":{\"content\":\"ok\"}}]}\n\ndata: {\"choices\":[],\"usage\":{\"prompt_tokens\":11,\"completion_tokens\":22}}\n\ndata: [DONE]\n\n";
        assert_eq!(parse_token_counts(body), Some((11, 22)));
    }

    #[test]
    fn a_stream_with_no_usage_anywhere_is_declined() {
        // Ollama's own /v1 endpoint emits no usage even with
        // stream_options.include_usage, so this is the real behaviour, not a
        // hypothetical. Declining is correct \u2014 a guessed count would be signed.
        let body = b"data: {\"choices\":[{\"delta\":{\"content\":\"ok\"}}]}\n\ndata: [DONE]\n\n";
        assert_eq!(parse_token_counts(body), None);
    }

    #[test]
    fn the_tail_window_is_bounded() {
        // A usage block buried further back than STREAM_TAIL_CHUNKS is not found.
        // Asserting it keeps the bound honest: both parties apply the same cap, so
        // they still agree, but the cost of a huge body stays bounded.
        let mut body = String::from("{\"prompt_eval_count\":1,\"eval_count\":1}\n");
        for _ in 0..STREAM_TAIL_CHUNKS + 5 {
            body.push_str("{\"done\":false}\n");
        }
        assert_eq!(parse_token_counts(body.as_bytes()), None);
    }

    // Captured verbatim from a live POST /api/embed with two inputs.
    const EMBED_BODY: &[u8] = br#"{"model":"all-minilm:latest","embeddings":[[0.1,0.2,0.3],[0.4,0.5,0.6]],"total_duration":1,"load_duration":1,"prompt_eval_count":8}"#;

    #[test]
    fn parses_an_embedding_response() {
        let w = parse_work(EMBED_BODY).expect("embeddings must be meterable");
        assert_eq!(w.kind, WORK_KIND_EMBEDDINGS);
        assert_eq!(w.prompt_tokens, 8);
        assert_eq!(w.completion_tokens, 0);
        assert_eq!(w.vectors, 2);
        assert_eq!(w.dimension, 3);
    }

    #[test]
    fn chat_is_still_classified_as_chat() {
        let w = parse_work(OLLAMA_STREAMED).expect("chat must still parse");
        assert_eq!(w.kind, WORK_KIND_CHAT);
        assert_eq!((w.prompt_tokens, w.completion_tokens), (27, 2));
        assert_eq!(w.vectors, 0);
    }

    #[test]
    fn a_ragged_embedding_response_is_declined() {
        // Two parties could disagree about what "dimension" means, and a claim
        // neither can reproduce is worthless.
        let body = br#"{"embeddings":[[0.1,0.2],[0.3]],"prompt_eval_count":4}"#;
        assert_eq!(parse_work(body), None);
        assert_eq!(parse_work(br#"{"embeddings":[]}"#), None);
    }

    #[test]
    fn an_embedding_exchange_settles_end_to_end() {
        let (provider_key, provider_did) = keypair();
        let (consumer_key, consumer_did) = other_keypair();
        let provider = LedgerNode::in_memory(provider_key, provider_did);
        let consumer = LedgerNode::in_memory(consumer_key, consumer_did.clone());

        let grant = provider
            .sign_quote(1, consumer_did, "all-minilm".into(), 30)
            .expect("quote");
        let claim = provider
            .claim_for_response(&grant.quote, 1, EMBED_BODY)
            .expect("embedding work must produce a claim");

        // Priced off the embedding line, not the chat price in the quote.
        let expected = MilesEconomy::default()
            .rate_card()
            .micro_per_1k_embedding_tokens
            * 8_u64
            / 1000;
        assert_eq!(claim.payload.credits_owed, expected.max(1));
        assert_eq!(claim.payload.ext_or_default().kind, WORK_KIND_EMBEDDINGS);
        assert_eq!(claim.payload.ext_or_default().vectors, 2);

        let receipt = consumer
            .counter_sign(claim, &grant, EMBED_BODY)
            .expect("the consumer must be able to re-derive and counter-sign");
        provider.record_receipt(&receipt);
        assert_eq!(provider.balances().unwrap()[0].receipts, 1);
    }

    #[test]
    fn embedding_work_cannot_be_billed_as_chat() {
        // The reason `kind` is signed rather than re-derived independently: chat
        // is four times the rate here, so mislabelling is a direct overcharge.
        let (provider_key, provider_did) = keypair();
        let (consumer_key, consumer_did) = other_keypair();
        let provider = LedgerNode::in_memory(provider_key, provider_did);
        let consumer = LedgerNode::in_memory(consumer_key, consumer_did.clone());

        let grant = provider
            .sign_quote(1, consumer_did, "m".into(), 30)
            .expect("quote");
        let honest = provider
            .claim_for_response(&grant.quote, 1, EMBED_BODY)
            .expect("claim");

        // Re-sign the same work as chat, at the chat rate.
        let mut payload = honest.payload.clone();
        payload.ext = kwaai_ledger::ClaimExt::chat().encode().unwrap();
        payload.credits_owed = grant.quote.credits_for_tokens(8).unwrap();
        let forged = payload
            .sign(&ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]))
            .unwrap();

        assert!(
            consumer.counter_sign(forged, &grant, EMBED_BODY).is_none(),
            "a claim that mislabels embedding work as chat must be refused"
        );
    }

    #[test]
    fn a_v2_claim_with_no_extension_still_reads_as_chat() {
        // A peer one release behind cannot express a work kind at all. Its claims
        // must keep working and be understood as chat, which is the only thing v2
        // could describe — this is the compatibility window in use.
        let (provider_key, provider_did) = keypair();
        let (_, consumer_did) = other_keypair();
        let provider = LedgerNode::in_memory(provider_key, provider_did);
        let grant = provider
            .sign_quote(1, consumer_did, "m".into(), 30)
            .expect("quote");

        let mut payload = provider
            .claim_for_response(&grant.quote, 1, OLLAMA_STREAMED)
            .expect("claim")
            .payload;
        payload.version = 2;
        payload.ext = Vec::new();
        let older = payload
            .sign(&ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]))
            .unwrap();

        assert!(older.verify().is_ok(), "v2 must still verify");
        assert_eq!(older.payload.ext_or_default().kind, WORK_KIND_CHAT);
    }

    #[test]
    fn unmeterable_bodies_yield_no_counts() {
        // Embeddings, plain-text upstream errors, and truncated JSON must all
        // decline rather than guess — a guessed count would be signed.
        assert_eq!(parse_token_counts(br#"{"embedding":[0.1,0.2]}"#), None);
        assert_eq!(parse_token_counts(b"upstream: connection refused"), None);
        assert_eq!(parse_token_counts(b"{"), None);
        assert_eq!(parse_token_counts(b""), None);
    }

    #[test]
    fn a_full_exchange_nets_symmetrically_through_two_ledgers() {
        let (provider_key, provider_did) = keypair();
        let (consumer_key, consumer_did) = other_keypair();

        let provider = LedgerNode::in_memory(provider_key, provider_did.clone());
        let consumer = LedgerNode::in_memory(consumer_key, consumer_did.clone());

        let grant = provider
            .sign_quote(1, consumer_did.clone(), "m".into(), 30)
            .expect("quote");
        let claim = provider
            .claim_for_response(&grant.quote, 42, OLLAMA_BODY)
            .expect("claim");
        provider.record_unsigned_claim(&claim);

        // Before the counter-signature the provider has earned nothing.
        let before = provider.balances().unwrap();
        assert_eq!(before.len(), 1);
        assert_eq!(before[0].earned, 0);
        assert_eq!(before[0].unsigned_claims, 1);

        let receipt = consumer
            .counter_sign(claim, &grant, OLLAMA_BODY)
            .expect("counter-signed");
        provider.record_receipt(&receipt);

        let p = &provider.balances().unwrap()[0];
        let c = &consumer.balances().unwrap()[0];
        // 160 tokens at 1000 micro/1k, rounded up.
        assert_eq!(p.earned, 160);
        assert_eq!(p.spent, 0);
        assert_eq!(p.unsigned_claims, 0, "the receipt should retire the claim");
        assert_eq!(c.spent, 160);
        assert_eq!(c.earned, 0);
        assert_eq!(p.net(), -c.net(), "both sides must agree on the net");
    }

    #[test]
    fn a_consumer_refuses_a_claim_whose_counts_do_not_match_the_body() {
        let (provider_key, provider_did) = keypair();
        let (consumer_key, consumer_did) = other_keypair();
        let provider = LedgerNode::in_memory(provider_key, provider_did);
        let consumer = LedgerNode::in_memory(consumer_key, consumer_did.clone());

        let grant = provider
            .sign_quote(1, consumer_did, "m".into(), 30)
            .expect("quote");
        let honest = provider
            .claim_for_response(&grant.quote, 1, OLLAMA_BODY)
            .expect("claim");

        // Inflate the counts and re-sign, so the signature is valid but the
        // quantity is a lie the consumer can catch from the body alone.
        let mut payload = honest.payload.clone();
        payload.completion_tokens = 100_000;
        payload.credits_owed = grant
            .quote
            .credits_for_tokens(payload.prompt_tokens + payload.completion_tokens)
            .unwrap();
        let forged = payload.sign(&SigningKey::from_bytes(&[7u8; 32])).unwrap();

        assert!(consumer.counter_sign(forged, &grant, OLLAMA_BODY).is_none());
        assert!(
            consumer.balances().unwrap().is_empty(),
            "a refused claim must leave no trace of debt"
        );
    }

    #[test]
    fn an_unmeterable_response_produces_no_claim_at_all() {
        let (provider_key, provider_did) = keypair();
        let (_, consumer_did) = other_keypair();
        let provider = LedgerNode::in_memory(provider_key, provider_did);
        let grant = provider
            .sign_quote(1, consumer_did, "m".into(), 30)
            .expect("quote");

        assert!(provider
            .claim_for_response(&grant.quote, 1, br#"{"embedding":[0.1]}"#)
            .is_none());
        assert!(provider
            .claim_for_response(&grant.quote, 2, b"upstream: 503")
            .is_none());
    }
}
