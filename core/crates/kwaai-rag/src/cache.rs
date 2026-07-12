//! Semantic query cache — Phase 6.
//!
//! LRU + TTL caching keyed by query embedding cosine similarity (threshold 0.92).
//! Persists to `<rag_dir>/query_cache.db` (SQLite WAL). Lookup is brute-force cosine scan
//! (adequate for ≤2000 entries; add HNSW later if needed).

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub const DEFAULT_TTL_SECS: u64 = 86_400; // 24 h
pub const DEFAULT_MAX_ENTRIES: usize = 2_000;
pub const DEFAULT_SIMILARITY_THRESHOLD: f64 = 0.92;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub id: i64,
    pub query_text: String,
    pub query_embedding: Vec<f32>,
    pub answer: String,
    /// Chunk IDs returned by retrieval for this query.
    pub chunk_ids: Vec<i64>,
    /// Unix epoch seconds.
    pub timestamp: i64,
    pub hit_count: u32,
}

pub struct QueryCache {
    conn: Connection,
    entries: HashMap<i64, CacheEntry>,
    pub ttl_secs: u64,
    pub max_entries: usize,
    pub similarity_threshold: f64,
}

// rusqlite::Connection is !Send; safe to mark Send because QueryCache is always accessed
// from a single async task context (never shared across threads simultaneously).
unsafe impl Send for QueryCache {}

impl QueryCache {
    /// Open (or create) the query cache for a KB tenant.
    pub fn open(data_dir: &Path, _tenant_id: Uuid) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join("query_cache.db");
        let conn = Connection::open(&path)
            .with_context(|| format!("opening query cache at {}", path.display()))?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=-65536;
             PRAGMA temp_store=MEMORY;",
        )?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entries (
                key   BLOB NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
             ) WITHOUT ROWID;",
        )?;

        let mut cache = Self {
            conn,
            entries: HashMap::new(),
            ttl_secs: DEFAULT_TTL_SECS,
            max_entries: DEFAULT_MAX_ENTRIES,
            similarity_threshold: DEFAULT_SIMILARITY_THRESHOLD,
        };
        cache.rebuild()?;
        Ok(cache)
    }

    fn rebuild(&mut self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT value FROM entries")?;
        let rows: Vec<Vec<u8>> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        for v in rows {
            if let Ok(e) = serde_json::from_slice::<CacheEntry>(&v) {
                self.entries.insert(e.id, e);
            }
        }
        tracing::debug!(entries = self.entries.len(), "query cache loaded");
        Ok(())
    }

    // ── Reads ─────────────────────────────────────────────────────────────────

    /// Look up by embedding similarity. Returns a hit entry if cosine ≥ threshold
    /// and timestamp is within TTL; increments hit_count and persists.
    pub fn get(&mut self, embedding: &[f32]) -> Option<CacheEntry> {
        let now = unix_now();
        let threshold = self.similarity_threshold;
        let ttl = self.ttl_secs;

        let best_id = best_match(&self.entries, embedding, threshold, ttl, now)?;
        let entry = self.entries.get_mut(&best_id)?;
        entry.hit_count += 1;
        let cloned = entry.clone();
        let _ = self.persist(&cloned);
        Some(cloned)
    }

    // ── Writes ────────────────────────────────────────────────────────────────

    /// Store a new (query, answer, chunk_ids) entry. Evicts expired + LRU first.
    pub fn put(
        &mut self,
        query_text: String,
        embedding: Vec<f32>,
        answer: String,
        chunk_ids: Vec<i64>,
    ) -> Result<()> {
        self.evict()?;
        let id = cache_id(&query_text);
        let entry = CacheEntry {
            id,
            query_text,
            query_embedding: embedding,
            answer,
            chunk_ids,
            timestamp: unix_now(),
            hit_count: 0,
        };
        self.persist(&entry)?;
        self.entries.insert(id, entry);
        Ok(())
    }

    /// Remove all entries; returns the count removed.
    pub fn clear(&mut self) -> Result<usize> {
        let ids: Vec<i64> = self.entries.keys().copied().collect();
        let count = ids.len();
        for id in ids {
            self.remove(id)?;
        }
        Ok(count)
    }

    // ── Stats ─────────────────────────────────────────────────────────────────

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn total_hits(&self) -> u64 {
        self.entries.values().map(|e| e.hit_count as u64).sum()
    }

    pub fn expired_count(&self) -> usize {
        let now = unix_now();
        let ttl = self.ttl_secs;
        self.entries
            .values()
            .filter(|e| now - e.timestamp > ttl as i64)
            .count()
    }

    // ── Internals ─────────────────────────────────────────────────────────────

    fn evict(&mut self) -> Result<()> {
        let now = unix_now();
        let ttl = self.ttl_secs;

        let expired: Vec<i64> = self
            .entries
            .values()
            .filter(|e| now - e.timestamp > ttl as i64)
            .map(|e| e.id)
            .collect();
        for id in expired {
            self.remove(id)?;
        }

        while self.entries.len() >= self.max_entries {
            let lru = self
                .entries
                .values()
                .min_by_key(|e| (e.hit_count, e.timestamp))
                .map(|e| e.id);
            match lru {
                Some(id) => self.remove(id)?,
                None => break,
            }
        }
        Ok(())
    }

    fn persist(&self, entry: &CacheEntry) -> Result<()> {
        let key = entry.id.to_le_bytes();
        let val = serde_json::to_vec(entry)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO entries (key, value) VALUES (?1, ?2)",
            params![key.as_ref(), val.as_slice()],
        )?;
        Ok(())
    }

    fn remove(&mut self, id: i64) -> Result<()> {
        self.entries.remove(&id);
        let key = id.to_le_bytes();
        self.conn
            .execute("DELETE FROM entries WHERE key = ?1", params![key.as_ref()])?;
        Ok(())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn best_match(
    entries: &HashMap<i64, CacheEntry>,
    embedding: &[f32],
    threshold: f64,
    ttl: u64,
    now: i64,
) -> Option<i64> {
    let qnorm: f64 = embedding
        .iter()
        .map(|&x| (x as f64).powi(2))
        .sum::<f64>()
        .sqrt();
    if qnorm == 0.0 {
        return None;
    }

    let mut best_id = None;
    let mut best_sim = threshold - 1e-9;

    for entry in entries.values() {
        if now - entry.timestamp > ttl as i64 {
            continue;
        }
        if entry.query_embedding.is_empty() {
            continue;
        }
        let dot: f64 = embedding
            .iter()
            .zip(entry.query_embedding.iter())
            .map(|(&q, &d)| (q as f64) * (d as f64))
            .sum();
        let dnorm: f64 = entry
            .query_embedding
            .iter()
            .map(|&x| (x as f64).powi(2))
            .sum::<f64>()
            .sqrt();
        if dnorm == 0.0 {
            continue;
        }
        let sim = (dot / (qnorm * dnorm)).clamp(-1.0, 1.0);
        if sim > best_sim {
            best_sim = sim;
            best_id = Some(entry.id);
        }
    }
    best_id
}

fn cache_id(query: &str) -> i64 {
    let mut h = Sha256::new();
    h.update(query.to_lowercase().as_bytes());
    let d = h.finalize();
    i64::from_le_bytes(d[..8].try_into().unwrap())
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
