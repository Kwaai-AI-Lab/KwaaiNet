//! Embedded storage backend — replaces the PostgreSQL connection pool.
//!
//! `StorageDb` wraps a `redb` database (tenant metadata) and a map of
//! in-memory HNSW indices (one per tenant). Both persist across restarts:
//! redb holds the raw vector bytes; on `open()` the HNSW indices are
//! rebuilt from stored vectors so no separate index file is needed.

use anyhow::{Context, Result};
use hnsw_rs::anndists::dist::distances::DistCosine;
use hnsw_rs::hnsw::Hnsw;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// redb table definitions
// ---------------------------------------------------------------------------

/// Tenant metadata.  key = UUID bytes (16), value = JSON TenantRecord.
pub(crate) const TENANTS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("tenants");

/// Vector data for all tenants.
/// key = tenant_id(16 bytes) ++ doc_id(8 bytes big-endian) = 24 bytes.
/// value = f32 embedding as little-endian bytes.
pub(crate) const VECTORS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("vectors");

// ---------------------------------------------------------------------------
// TenantRecord — persisted in redb
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TenantRecord {
    pub peer_id: String,
    pub display_name: Option<String>,
    pub capacity_limit_mb: i64,
    pub status: String,
    pub created_at: String,
    pub vector_dimension: usize,
}

// ---------------------------------------------------------------------------
// TenantIndex — in-memory HNSW per tenant
// ---------------------------------------------------------------------------

/// Corpora below this size use exact brute-force cosine search.
/// HNSW graph connectivity degrades badly at low vector counts.
const BRUTE_FORCE_THRESHOLD: usize = 2_000;

/// In-memory HNSW index for one tenant, with a tombstone layer for deletes.
pub struct TenantIndex {
    pub hnsw: Hnsw<'static, f32, DistCosine>,
    /// hnsw_internal_id → external doc id. `None` = tombstoned.
    pub id_map: Vec<Option<i64>>,
    /// external doc id → hnsw_internal_id (for upsert / delete).
    pub rev_map: HashMap<i64, usize>,
    /// Raw vectors for exact search on small corpora (mirrors redb but in-memory).
    pub stored_vecs: HashMap<i64, Vec<f32>>,
    pub next_id: usize,
    pub dimension: usize,
}

impl TenantIndex {
    pub fn new(dimension: usize) -> Self {
        Self::new_with_params(dimension, 16, 200)
    }

    /// Build with explicit HNSW graph parameters.
    /// m: bidirectional links per node (16 = sweet spot for dim=768).
    /// ef_construction: beam width during graph build; minimum 40, recommended ≥200.
    /// Higher ef_construction improves recall quality at index-build time (paid once);
    /// it cannot be compensated at query time by raising ef_search.
    pub fn new_with_params(dimension: usize, m: usize, ef_construction: usize) -> Self {
        let hnsw: Hnsw<'static, f32, DistCosine> =
            Hnsw::new(m, 65_536, 16, ef_construction, DistCosine);
        Self {
            hnsw,
            id_map: Vec::new(),
            rev_map: HashMap::new(),
            stored_vecs: HashMap::new(),
            next_id: 0,
            dimension,
        }
    }

    /// Insert or update a vector. Re-inserts tombstone the old slot.
    pub fn insert(&mut self, doc_id: i64, embedding: &[f32]) {
        if let Some(&old_id) = self.rev_map.get(&doc_id) {
            self.id_map[old_id] = None; // tombstone old entry
        }
        let hnsw_id = self.next_id;
        self.next_id += 1;
        if hnsw_id >= self.id_map.len() {
            self.id_map.resize(hnsw_id + 1, None);
        }
        self.id_map[hnsw_id] = Some(doc_id);
        self.rev_map.insert(doc_id, hnsw_id);
        self.stored_vecs.insert(doc_id, embedding.to_vec());
        self.hnsw.insert((embedding, hnsw_id));
    }

    /// Tombstone a doc_id so it no longer appears in search results.
    pub fn tombstone(&mut self, doc_id: i64) -> bool {
        if let Some(&hnsw_id) = self.rev_map.get(&doc_id) {
            self.id_map[hnsw_id] = None;
            self.rev_map.remove(&doc_id);
            self.stored_vecs.remove(&doc_id);
            true
        } else {
            false
        }
    }

    /// Number of live (non-tombstoned) vectors.
    pub fn live_count(&self) -> usize {
        self.rev_map.len()
    }

    /// ANN search: returns top-k (doc_id, cosine_similarity) pairs.
    ///
    /// Falls back to exact brute-force cosine for corpora smaller than BRUTE_FORCE_THRESHOLD
    /// because HNSW loses recall significantly on small graphs.
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<(i64, f64)> {
        if self.rev_map.is_empty() || top_k == 0 {
            return vec![];
        }
        if self.rev_map.len() < BRUTE_FORCE_THRESHOLD {
            self.search_exact(query, top_k)
        } else {
            self.search_hnsw(query, top_k)
        }
    }

    /// Exact cosine search over all live vectors — O(n) but precise.
    pub fn search_exact(&self, query: &[f32], top_k: usize) -> Vec<(i64, f64)> {
        let qnorm: f64 = query
            .iter()
            .map(|&x| (x as f64) * (x as f64))
            .sum::<f64>()
            .sqrt();
        if qnorm == 0.0 {
            return vec![];
        }
        let mut scored: Vec<(i64, f64)> = self
            .stored_vecs
            .iter()
            .map(|(&doc_id, emb)| {
                let dot: f64 = query
                    .iter()
                    .zip(emb.iter())
                    .map(|(&q, &d)| (q as f64) * (d as f64))
                    .sum();
                let dnorm: f64 = emb
                    .iter()
                    .map(|&x| (x as f64) * (x as f64))
                    .sum::<f64>()
                    .sqrt();
                let sim = if dnorm > 0.0 {
                    (dot / (qnorm * dnorm)).clamp(-1.0, 1.0)
                } else {
                    0.0
                };
                (doc_id, sim)
            })
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }

    pub fn search_hnsw(&self, query: &[f32], top_k: usize) -> Vec<(i64, f64)> {
        self.search_hnsw_ef(query, top_k, 64)
    }

    pub fn search_hnsw_ef(&self, query: &[f32], top_k: usize, ef: usize) -> Vec<(i64, f64)> {
        // Fetch extra candidates to compensate for tombstoned slots.
        let fetch_k = (top_k * 4).max(top_k + 16);
        let neighbours = self.hnsw.search(query, fetch_k, ef);
        let mut results = Vec::with_capacity(top_k);
        for nb in neighbours {
            if let Some(Some(doc_id)) = self.id_map.get(nb.d_id) {
                // DistCosine returns cosine distance; similarity = 1 - distance.
                let score = (1.0 - nb.distance as f64).clamp(0.0, 1.0);
                results.push((*doc_id, score));
                if results.len() >= top_k {
                    break;
                }
            }
        }
        results
    }
}

// ---------------------------------------------------------------------------
// StorageDb — the public handle (Clone is cheap: inner is Arc)
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct StorageDb {
    pub(crate) inner: Arc<DbInner>,
}

pub(crate) struct DbInner {
    pub db: Database,
    pub indices: RwLock<HashMap<Uuid, Arc<Mutex<TenantIndex>>>>,
    #[allow(dead_code)]
    pub data_dir: PathBuf,
}

impl StorageDb {
    /// Open (or create) the embedded store at `data_dir`.
    ///
    /// All tenant indices are rebuilt from stored vector data so the
    /// process is ready to serve immediately after returning.
    pub fn open(data_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(data_dir)
            .with_context(|| format!("creating storage dir {}", data_dir.display()))?;

        let db_path = data_dir.join("metadata.redb");
        let db = Database::create(&db_path)
            .with_context(|| format!("opening embedded db at {}", db_path.display()))?;

        // Ensure both tables exist before any reads.
        {
            let wtxn = db.begin_write()?;
            wtxn.open_table(TENANTS_TABLE)?;
            wtxn.open_table(VECTORS_TABLE)?;
            wtxn.commit()?;
        }

        let indices = Self::rebuild_indices(&db)?;

        Ok(Self {
            inner: Arc::new(DbInner {
                db,
                indices: RwLock::new(indices),
                data_dir: data_dir.to_path_buf(),
            }),
        })
    }

    /// Rebuild all in-memory HNSW indices from redb on startup.
    fn rebuild_indices(db: &Database) -> Result<HashMap<Uuid, Arc<Mutex<TenantIndex>>>> {
        let rtxn = db.begin_read()?;
        let tenants = rtxn.open_table(TENANTS_TABLE)?;
        let vectors = rtxn.open_table(VECTORS_TABLE)?;

        // First pass: collect active tenant dimensions.
        let mut dims: HashMap<Uuid, usize> = HashMap::new();
        for entry in tenants.iter()? {
            let (k, v) = entry?;
            let tid = Uuid::from_slice(k.value()).context("corrupt tenant key")?;
            let rec: TenantRecord =
                serde_json::from_slice(v.value()).context("corrupt tenant record")?;
            if rec.status != "Deleted" {
                dims.insert(tid, rec.vector_dimension);
            }
        }

        // Create empty indices.
        let indices: HashMap<Uuid, Arc<Mutex<TenantIndex>>> = dims
            .iter()
            .map(|(&tid, &dim)| (tid, Arc::new(Mutex::new(TenantIndex::new(dim)))))
            .collect();

        // Second pass: load vectors and populate indices.
        for entry in vectors.iter()? {
            let (k, v) = entry?;
            let key = k.value();
            if key.len() < 24 {
                continue;
            }
            let tid = Uuid::from_slice(&key[..16]).unwrap_or(Uuid::nil());
            if let Some(arc) = indices.get(&tid) {
                let doc_id = i64::from_be_bytes(key[16..24].try_into().unwrap());
                let embedding = bytes_to_f32s(v.value());
                arc.lock().unwrap().insert(doc_id, &embedding);
            }
        }

        tracing::info!("storage: loaded {} active tenant(s)", indices.len());
        Ok(indices)
    }
}

// ---------------------------------------------------------------------------
// Key / value encoding helpers
// ---------------------------------------------------------------------------

/// Composite redb key for a vector: tenant_id(16) ++ doc_id_be(8) = 24 bytes.
pub(crate) fn vector_key(tenant_id: Uuid, doc_id: i64) -> [u8; 24] {
    let mut k = [0u8; 24];
    k[..16].copy_from_slice(tenant_id.as_bytes());
    k[16..].copy_from_slice(&doc_id.to_be_bytes());
    k
}

pub(crate) fn f32s_to_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|f| f.to_le_bytes()).collect()
}

pub(crate) fn bytes_to_f32s(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn unit(dim: usize, pos: usize) -> Vec<f32> {
        let mut v = vec![0.0f32; dim];
        v[pos] = 1.0;
        v
    }

    // ── TenantIndex ──────────────────────────────────────────────────────────

    #[test]
    fn index_new_is_empty() {
        let idx = TenantIndex::new(4);
        assert_eq!(idx.live_count(), 0);
        assert_eq!(idx.dimension, 4);
    }

    #[test]
    fn index_insert_increments_live_count() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        assert_eq!(idx.live_count(), 1);
        idx.insert(2, &unit(4, 1));
        assert_eq!(idx.live_count(), 2);
    }

    #[test]
    fn index_tombstone_decrements_live_count() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        assert!(idx.tombstone(1));
        assert_eq!(idx.live_count(), 0);
    }

    #[test]
    fn index_tombstone_unknown_returns_false() {
        let mut idx = TenantIndex::new(4);
        assert!(!idx.tombstone(999));
    }

    #[test]
    fn index_search_on_empty_returns_empty() {
        let idx = TenantIndex::new(4);
        assert!(idx.search(&unit(4, 0), 5).is_empty());
    }

    #[test]
    fn index_search_top_k_zero_returns_empty() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        assert!(idx.search(&unit(4, 0), 0).is_empty());
    }

    #[test]
    fn index_search_exact_identical_score_one() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        let r = idx.search_exact(&unit(4, 0), 1);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].0, 1);
        assert!((r[0].1 - 1.0).abs() < 1e-6, "score={}", r[0].1);
    }

    #[test]
    fn index_search_exact_orthogonal_score_zero() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        let r = idx.search_exact(&unit(4, 1), 1);
        assert_eq!(r.len(), 1);
        assert!((r[0].1 - 0.0).abs() < 1e-6, "score={}", r[0].1);
    }

    #[test]
    fn index_search_exact_sorted_descending() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0)); // score 1.0 against query [1,0,0,0]
        idx.insert(2, &unit(4, 1)); // score 0.0
        let r = idx.search_exact(&unit(4, 0), 2);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].0, 1);
        assert!(r[0].1 > r[1].1);
    }

    #[test]
    fn index_search_exact_respects_top_k() {
        let mut idx = TenantIndex::new(4);
        for i in 0..8i64 {
            idx.insert(i, &unit(4, (i as usize) % 4));
        }
        let r = idx.search_exact(&unit(4, 0), 3);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn index_search_exact_zero_query_returns_empty() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        assert!(idx.search_exact(&[0.0, 0.0, 0.0, 0.0], 5).is_empty());
    }

    #[test]
    fn index_upsert_keeps_live_count_at_one() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        idx.insert(1, &unit(4, 1)); // re-insert same doc_id
        assert_eq!(idx.live_count(), 1);
        // updated vector is now at dim 1
        let r = idx.search_exact(&unit(4, 1), 1);
        assert!((r[0].1 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn index_tombstoned_doc_absent_from_exact_search() {
        let mut idx = TenantIndex::new(4);
        idx.insert(1, &unit(4, 0));
        idx.insert(2, &unit(4, 0));
        idx.tombstone(1);
        let r = idx.search_exact(&unit(4, 0), 5);
        assert!(!r.iter().any(|(id, _)| *id == 1));
    }

    #[test]
    fn index_search_hnsw_ef_basic() {
        // Uses HNSW path directly (small corpus is fine — just verifying no panics + shape).
        let mut idx = TenantIndex::new(4);
        for i in 0..30i64 {
            let mut v = vec![0.0f32; 4];
            v[(i as usize) % 4] = 1.0;
            idx.insert(i, &v);
        }
        let r = idx.search_hnsw_ef(&unit(4, 0), 5, 32);
        assert!(!r.is_empty());
        assert!(r.len() <= 5);
        for (_, score) in &r {
            assert!(*score >= 0.0 && *score <= 1.0);
        }
    }

    // ── Encoding helpers ─────────────────────────────────────────────────────

    #[test]
    fn f32_bytes_roundtrip() {
        let original = vec![1.0f32, -2.5, 0.0, 1e10];
        let bytes = f32s_to_bytes(&original);
        let restored = bytes_to_f32s(&bytes);
        assert_eq!(restored.len(), original.len());
        for (a, b) in original.iter().zip(restored.iter()) {
            assert!((a - b).abs() < 1e-9, "mismatch: {a} vs {b}");
        }
    }

    #[test]
    fn f32_bytes_empty_roundtrip() {
        assert!(f32s_to_bytes(&[]).is_empty());
        assert!(bytes_to_f32s(&[]).is_empty());
    }

    #[test]
    fn vector_key_length_and_layout() {
        let tid = Uuid::new_v4();
        let doc_id: i64 = 0x0102030405060708;
        let k = vector_key(tid, doc_id);
        assert_eq!(k.len(), 24);
        assert_eq!(&k[..16], tid.as_bytes());
        assert_eq!(&k[16..], &doc_id.to_be_bytes());
    }

    // ── StorageDb ────────────────────────────────────────────────────────────

    #[test]
    fn storage_db_open_creates_directory() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("nested/store");
        StorageDb::open(&sub).unwrap();
        assert!(sub.exists());
    }

    #[test]
    fn storage_db_open_starts_with_empty_indices() {
        let tmp = TempDir::new().unwrap();
        let db = StorageDb::open(tmp.path()).unwrap();
        assert!(db.inner.indices.read().unwrap().is_empty());
    }
}
