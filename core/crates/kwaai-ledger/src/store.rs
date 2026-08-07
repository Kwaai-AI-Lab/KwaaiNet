//! Local persistence for work receipts.
//!
//! Each node keeps its own copy of every receipt it is a party to. There is no
//! shared ledger and no replication: a receipt is agreed by exactly the two
//! peers who signed it, so two honest nodes independently arrive at the same
//! pairwise net without ever exchanging balances.
//!
//! ## What is and is not counted
//!
//! * A fully co-signed [`Receipt`] counts toward earned (we were provider) or
//!   spent (we were consumer).
//! * A provider-only [`WorkClaim`] is **recorded but never counted**. It means a
//!   consumer took delivery and declined to counter-sign. Tracking the ratio per
//!   peer is the only enforcement available without an arbiter — a peer that
//!   habitually refuses can be denied future leases.
//!
//! ## Idempotency
//!
//! `receipt_id` (a content address) is the primary key, so replaying a receipt
//! is rejected by the uniqueness constraint rather than by application logic.
//! `record_receipt` returns `false` for a duplicate rather than erroring: a
//! retried delivery is normal, not exceptional.

use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension};

use crate::{LedgerError, MicroCredits, Receipt, WorkClaim};

type Result<T> = std::result::Result<T, LedgerError>;

impl From<rusqlite::Error> for LedgerError {
    fn from(e: rusqlite::Error) -> Self {
        LedgerError::Store(e.to_string())
    }
}

/// Per-peer netting, from this node's point of view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerBalance {
    pub peer_did: String,
    /// Credits this peer owes us for compute we served.
    pub earned: MicroCredits,
    /// Credits we owe this peer for compute we consumed.
    pub spent: MicroCredits,
    pub receipts: u64,
    /// Claims we issued to this peer that it never counter-signed.
    pub unsigned_claims: u64,
    /// Receipts where *we* were the provider. Tracked separately so
    /// [`unsigned_ratio`](Self::unsigned_ratio)'s denominator is "claims we
    /// issued" rather than "all receipts with this peer".
    pub receipts_as_provider: u64,
}

impl PeerBalance {
    /// Net position: positive means the peer owes us.
    pub fn net(&self) -> i128 {
        self.earned as i128 - self.spent as i128
    }

    /// Claims we issued to this peer, settled or not.
    pub fn claims_issued(&self) -> u64 {
        self.receipts_as_provider + self.unsigned_claims
    }

    /// Share of our claims against this peer that it declined to sign.
    /// `None` when we have never issued it a claim.
    pub fn unsigned_ratio(&self) -> Option<f64> {
        let issued = self.claims_issued();
        (issued > 0).then(|| self.unsigned_claims as f64 / issued as f64)
    }
}

/// Below this many issued claims, an unsigned ratio says nothing useful about a
/// peer's behaviour.
///
/// An ack can be lost for reasons that are entirely our own — most of all a
/// short-lived consumer process exiting before its ack reaches the socket, which
/// is a bug class we have already hit. One lost ack out of one claim is a 100%
/// ratio and would brand a peer that did nothing wrong, so callers must require
/// a real sample before reading anything into it.
pub const MIN_CLAIMS_FOR_RATIO: u64 = 5;

pub struct LedgerStore {
    conn: Connection,
    /// This node's DID — decides whether a receipt is earned or spent.
    our_did: String,
}

impl LedgerStore {
    /// Default location: `~/.kwaainet/ledger.db` (honours `$KWAAINET_HOME` via
    /// the caller, which passes the resolved directory).
    pub fn default_path(kwaainet_dir: &Path) -> PathBuf {
        kwaainet_dir.join("ledger.db")
    }

    pub fn open(path: &Path, our_did: impl Into<String>) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| LedgerError::Store(format!("creating {}: {e}", parent.display())))?;
        }
        let conn = Connection::open(path)
            .map_err(|e| LedgerError::Store(format!("opening {}: {e}", path.display())))?;
        // WAL so a reader (`ledger show`) never blocks the dispatch path writing
        // receipts, matching how kwaai-storage configures its embedded db.
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA foreign_keys=ON;",
        )?;
        let store = Self {
            conn,
            our_did: our_did.into(),
        };
        store.migrate()?;
        Ok(store)
    }

    /// In-memory store, for tests.
    pub fn open_in_memory(our_did: impl Into<String>) -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self {
            conn,
            our_did: our_did.into(),
        };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS receipts (
                 receipt_id        BLOB    NOT NULL PRIMARY KEY,
                 provider_did      TEXT    NOT NULL,
                 consumer_did      TEXT    NOT NULL,
                 lease_id          INTEGER NOT NULL,
                 request_id        INTEGER NOT NULL,
                 prompt_tokens     INTEGER NOT NULL,
                 completion_tokens INTEGER NOT NULL,
                 credits           INTEGER NOT NULL,
                 recorded_at_ms    INTEGER NOT NULL,
                 payload           BLOB    NOT NULL,
                 -- Added after the first receipts existed; defaults to chat,
                 -- which is the only kind those rows could have been.
                 work_kind         INTEGER NOT NULL DEFAULT 0
             ) WITHOUT ROWID;

             CREATE INDEX IF NOT EXISTS receipts_provider ON receipts(provider_did);
             CREATE INDEX IF NOT EXISTS receipts_consumer ON receipts(consumer_did);

             -- Claims a consumer declined to counter-sign. Keyed on the claim's
             -- own content address (`WorkClaim::claim_id`) — computable by the
             -- provider alone — so a later counter-signature is recognisably
             -- the same work.
             CREATE TABLE IF NOT EXISTS unsigned_claims (
                 claim_id       BLOB    NOT NULL PRIMARY KEY,
                 provider_did   TEXT    NOT NULL,
                 consumer_did   TEXT    NOT NULL,
                 lease_id       INTEGER NOT NULL,
                 request_id     INTEGER NOT NULL,
                 credits        INTEGER NOT NULL,
                 recorded_at_ms INTEGER NOT NULL
             ) WITHOUT ROWID;

             CREATE INDEX IF NOT EXISTS unsigned_consumer ON unsigned_claims(consumer_did);",
        )?;

        // Migration for stores created before work kinds existed. SQLite has no
        // ADD COLUMN IF NOT EXISTS, and re-adding is a plain error rather than
        // anything destructive, so it is safe to attempt and ignore.
        let _ = self.conn.execute(
            "ALTER TABLE receipts ADD COLUMN work_kind INTEGER NOT NULL DEFAULT 0",
            [],
        );
        Ok(())
    }

    pub fn our_did(&self) -> &str {
        &self.our_did
    }

    /// Record a co-signed receipt. Returns `false` if we already had it.
    ///
    /// Verifies before storing — a store full of unverifiable receipts would be
    /// worse than an empty one, and this is the last point at which we hold the
    /// signatures.
    pub fn record_receipt(&self, receipt: &Receipt) -> Result<bool> {
        receipt.verify()?;
        let id = receipt.receipt_id()?;
        let p = &receipt.claim.payload;
        let payload = crate::encode(receipt)?;

        let changed = self.conn.execute(
            "INSERT OR IGNORE INTO receipts (
                 receipt_id, provider_did, consumer_did, lease_id, request_id,
                 prompt_tokens, completion_tokens, credits, recorded_at_ms, payload,
                 work_kind
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![
                &id[..],
                &p.provider_did,
                &p.consumer_did,
                p.lease_id as i64,
                p.request_id as i64,
                p.prompt_tokens as i64,
                p.completion_tokens as i64,
                p.credits_owed as i64,
                now_ms() as i64,
                payload,
                p.ext_or_default().kind as i64,
            ],
        )?;

        // A claim that has now been settled is no longer outstanding. Matched on
        // the *claim* id, not the receipt id: the receipt id hashes
        // `consumer_sig`, which the provider did not have when it recorded the
        // claim, so it could never have used that as the key.
        if changed > 0 {
            self.conn.execute(
                "DELETE FROM unsigned_claims WHERE claim_id = ?1",
                rusqlite::params![&receipt.claim_id()?[..]],
            )?;
        }
        Ok(changed > 0)
    }

    /// Record that we served work but hold no counter-signature for it.
    ///
    /// Keyed on [`WorkClaim::claim_id`], which both parties can compute from the
    /// claim alone, so [`record_receipt`](Self::record_receipt) can retire it
    /// later if the consumer eventually signs.
    pub fn record_unsigned_claim(&self, claim: &WorkClaim) -> Result<bool> {
        claim.verify()?;
        let claim_id = claim.claim_id()?;
        let p = &claim.payload;
        let changed = self.conn.execute(
            "INSERT OR IGNORE INTO unsigned_claims (
                 claim_id, provider_did, consumer_did, lease_id, request_id,
                 credits, recorded_at_ms
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                &claim_id[..],
                &p.provider_did,
                &p.consumer_did,
                p.lease_id as i64,
                p.request_id as i64,
                p.credits_owed as i64,
                now_ms() as i64,
            ],
        )?;
        Ok(changed > 0)
    }

    pub fn has_receipt(&self, receipt_id: &[u8; 32]) -> Result<bool> {
        Ok(self
            .conn
            .query_row(
                "SELECT 1 FROM receipts WHERE receipt_id = ?1",
                rusqlite::params![&receipt_id[..]],
                |_| Ok(()),
            )
            .optional()?
            .is_some())
    }

    /// Per-peer netting, ordered by largest absolute net first so the most
    /// significant relationships surface at the top of `ledger show`.
    /// Raw two-way work volume per counterparty, for [`crate::Economy::settle`].
    ///
    /// Deliberately returns **tokens, not credits**. `credits` records what the
    /// two parties agreed bilaterally under whatever rate card the provider was
    /// running; an economy has to re-price the same work in its own terms, or the
    /// A/B comparison between currency models would be contaminated by whichever
    /// card happened to be in force.
    pub fn work_ledger(&self) -> Result<crate::WorkLedger> {
        let mut stmt = self.conn.prepare(
            "SELECT provider_did, consumer_did, prompt_tokens + completion_tokens, work_kind
             FROM receipts WHERE provider_did = ?1 OR consumer_did = ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![&self.our_did], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)? as u64,
                r.get::<_, i64>(3)? as u8,
            ))
        })?;

        let mut folded = Vec::new();
        for row in rows {
            let (provider, consumer, tokens, kind) = row?;
            // Same exclusion as `balances`: self-dealing nets to zero and would
            // otherwise be counted on both sides of the same row.
            if provider == consumer {
                continue;
            }
            if provider == self.our_did {
                folded.push((consumer, kind, tokens, 0));
            } else {
                folded.push((provider, kind, 0, tokens));
            }
        }
        Ok(crate::economy::work_ledger_from(folded))
    }

    pub fn balances(&self) -> Result<Vec<PeerBalance>> {
        use std::collections::BTreeMap;
        let mut acc: BTreeMap<String, PeerBalance> = BTreeMap::new();

        let mut stmt = self.conn.prepare(
            "SELECT provider_did, consumer_did, credits FROM receipts
             WHERE provider_did = ?1 OR consumer_did = ?1",
        )?;
        let rows = stmt.query_map(rusqlite::params![&self.our_did], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, i64>(2)? as MicroCredits,
            ))
        })?;

        for row in rows {
            let (provider, consumer, credits) = row?;
            // Self-dealing (both sides us, e.g. `ledger self-test`) nets to zero
            // and would otherwise double-count; skip it.
            if provider == consumer {
                continue;
            }
            let we_provided = provider == self.our_did;
            let peer = if we_provided { consumer } else { provider };
            let entry = acc.entry(peer.clone()).or_insert_with(|| PeerBalance {
                peer_did: peer,
                earned: 0,
                spent: 0,
                receipts: 0,
                unsigned_claims: 0,
                receipts_as_provider: 0,
            });
            entry.receipts += 1;
            if we_provided {
                entry.earned = entry.earned.saturating_add(credits);
                entry.receipts_as_provider += 1;
            } else {
                entry.spent = entry.spent.saturating_add(credits);
            }
        }

        let mut stmt = self.conn.prepare(
            "SELECT consumer_did, COUNT(*) FROM unsigned_claims
             WHERE provider_did = ?1 GROUP BY consumer_did",
        )?;
        let rows = stmt.query_map(rusqlite::params![&self.our_did], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as u64))
        })?;
        for row in rows {
            let (peer, n) = row?;
            let entry = acc.entry(peer.clone()).or_insert_with(|| PeerBalance {
                peer_did: peer,
                earned: 0,
                spent: 0,
                receipts: 0,
                unsigned_claims: 0,
                receipts_as_provider: 0,
            });
            entry.unsigned_claims = n;
        }

        let mut out: Vec<PeerBalance> = acc.into_values().collect();
        out.sort_by_key(|b| std::cmp::Reverse(b.net().abs()));
        Ok(out)
    }

    /// `(earned, spent, receipt_count)` across all peers.
    pub fn totals(&self) -> Result<(MicroCredits, MicroCredits, u64)> {
        let balances = self.balances()?;
        let earned = balances.iter().map(|b| b.earned).sum();
        let spent = balances.iter().map(|b| b.spent).sum();
        let count = balances.iter().map(|b| b.receipts).sum();
        Ok((earned, spent, count))
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::*;

    fn store_for(did: &str) -> LedgerStore {
        LedgerStore::open_in_memory(did).unwrap()
    }

    #[test]
    fn records_a_receipt_and_nets_it_as_earned_for_the_provider() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let r = full_receipt(&p, &c, b"body", 10, 40);

        assert!(store.record_receipt(&r).unwrap());
        let balances = store.balances().unwrap();
        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].peer_did, c.did);
        assert_eq!(balances[0].earned, r.credits());
        assert_eq!(balances[0].spent, 0);
        assert!(balances[0].net() > 0, "provider is owed");
    }

    #[test]
    fn the_same_receipt_nets_as_spent_for_the_consumer() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&c.did); // same receipt, consumer's copy
        let r = full_receipt(&p, &c, b"body", 10, 40);

        assert!(store.record_receipt(&r).unwrap());
        let b = &store.balances().unwrap()[0];
        assert_eq!(b.peer_did, p.did);
        assert_eq!(b.spent, r.credits());
        assert_eq!(b.earned, 0);
        assert!(b.net() < 0, "consumer owes");
    }

    /// Both honest nodes must independently arrive at mirror-image nets — the
    /// whole premise of bilateral accounting without a shared ledger.
    #[test]
    fn both_parties_independently_agree_on_the_net() {
        let (p, c) = (make_id(), make_id());
        let r = full_receipt(&p, &c, b"body", 10, 40);
        let provider_store = store_for(&p.did);
        let consumer_store = store_for(&c.did);
        provider_store.record_receipt(&r).unwrap();
        consumer_store.record_receipt(&r).unwrap();

        let pn = provider_store.balances().unwrap()[0].net();
        let cn = consumer_store.balances().unwrap()[0].net();
        assert_eq!(pn, -cn, "nets must mirror: {pn} vs {cn}");
    }

    #[test]
    fn replayed_receipt_is_idempotent_not_double_counted() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let r = full_receipt(&p, &c, b"body", 10, 40);

        assert!(store.record_receipt(&r).unwrap(), "first insert is new");
        assert!(!store.record_receipt(&r).unwrap(), "replay is a no-op");
        assert!(!store.record_receipt(&r).unwrap());

        let b = &store.balances().unwrap()[0];
        assert_eq!(b.receipts, 1, "replay must not inflate the count");
        assert_eq!(b.earned, r.credits(), "nor the credits");
    }

    #[test]
    fn an_unverifiable_receipt_is_never_stored() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let mut r = full_receipt(&p, &c, b"body", 10, 40);
        r.claim.payload.credits_owed += 1_000_000; // breaks both signatures

        assert!(store.record_receipt(&r).is_err());
        assert!(
            store.balances().unwrap().is_empty(),
            "nothing should have been written"
        );
    }

    #[test]
    fn unsigned_claims_are_tracked_but_never_counted_as_earned() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let claim = unsigned_claim(&p, &c, b"body", 10, 40);

        assert!(store.record_unsigned_claim(&claim).unwrap());
        let b = &store.balances().unwrap()[0];
        assert_eq!(b.unsigned_claims, 1);
        assert_eq!(b.earned, 0, "an unsigned claim is not income");
        assert_eq!(b.receipts, 0);
        assert_eq!(b.unsigned_ratio(), Some(1.0), "100% refused so far");
    }

    #[test]
    fn a_late_counter_signature_retires_the_unsigned_claim() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let body = b"body";
        let claim = unsigned_claim(&p, &c, body, 10, 40);
        store.record_unsigned_claim(&claim).unwrap();
        assert_eq!(store.balances().unwrap()[0].unsigned_claims, 1);

        // Consumer signs after all.
        let claim_id = claim.claim_id().unwrap();
        let receipt = claim.counter_sign(&c.signing, 1).unwrap();
        assert_eq!(
            receipt.claim_id().unwrap(),
            claim_id,
            "counter-signing must not change the claim id, or retirement cannot work"
        );
        assert_ne!(
            receipt.receipt_id().unwrap(),
            claim_id,
            "receipt id and claim id are distinct addresses — the receipt id \
             covers the consumer signature, which the provider never had"
        );
        store.record_receipt(&receipt).unwrap();

        let b = &store.balances().unwrap()[0];
        assert_eq!(b.unsigned_claims, 0, "claim should be retired");
        assert_eq!(b.receipts, 1);
        assert_eq!(b.earned, receipt.credits());
        assert_eq!(b.unsigned_ratio(), Some(0.0));
    }

    #[test]
    fn self_dealing_is_excluded_from_netting() {
        let p = make_id();
        let store = store_for(&p.did);
        // Both parties are us — what `ledger self-test` produces.
        let r = full_receipt(&p, &p, b"body", 1, 1);
        store.record_receipt(&r).unwrap();
        assert!(
            store.balances().unwrap().is_empty(),
            "self-dealing must not appear as a counterparty"
        );
    }

    #[test]
    fn totals_aggregate_across_peers() {
        let me = make_id();
        let (a, b) = (make_id(), make_id());
        let store = store_for(&me.did);
        // We serve `a`, and consume from `b`.
        store
            .record_receipt(&full_receipt(&me, &a, b"x", 10, 40))
            .unwrap();
        store
            .record_receipt(&full_receipt(&b, &me, b"y", 5, 5))
            .unwrap();

        let (earned, spent, count) = store.totals().unwrap();
        assert!(earned > 0 && spent > 0);
        assert_eq!(count, 2);
        assert_eq!(store.balances().unwrap().len(), 2);
    }

    #[test]
    fn has_receipt_reflects_what_was_stored() {
        let (p, c) = (make_id(), make_id());
        let store = store_for(&p.did);
        let r = full_receipt(&p, &c, b"body", 1, 1);
        let id = r.receipt_id().unwrap();

        assert!(!store.has_receipt(&id).unwrap());
        store.record_receipt(&r).unwrap();
        assert!(store.has_receipt(&id).unwrap());
    }

    #[test]
    fn survives_reopen_on_disk() {
        let (p, c) = (make_id(), make_id());
        let dir = tempfile::tempdir().unwrap();
        let path = LedgerStore::default_path(dir.path());
        let r = full_receipt(&p, &c, b"body", 10, 40);

        {
            let store = LedgerStore::open(&path, &p.did).unwrap();
            store.record_receipt(&r).unwrap();
        }
        let reopened = LedgerStore::open(&path, &p.did).unwrap();
        assert_eq!(reopened.balances().unwrap()[0].earned, r.credits());
        assert!(reopened.has_receipt(&r.receipt_id().unwrap()).unwrap());
    }
}
