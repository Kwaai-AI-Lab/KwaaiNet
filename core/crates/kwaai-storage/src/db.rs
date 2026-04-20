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
pub(crate) const TENANTS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("tenants");

/// Vector data for all tenants.
/// key = tenant_id(16 bytes) ++ doc_id(8 bytes big-endian) = 24 bytes.
/// value = f32 embedding as little-endian bytes.
pub(crate) const VECTORS_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("vectors");

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

/// In-memory HNSW index for one tenant, with a tombstone layer for deletes.
pub(crate) struct TenantIndex {
    pub hnsw: Hnsw<'static, f32, DistCosine>,
    /// hnsw_internal_id → external doc id. `None` = tombstoned.
    pub id_map: Vec<Option<i64>>,
    /// external doc id → hnsw_internal_id (for upsert / delete).
    pub rev_map: HashMap<i64, usize>,
    pub next_id: usize,
    pub dimension: usize,
}

impl TenantIndex {
    pub fn new(dimension: usize) -> Self {
        // m=16, initial_capacity=65536, ef_construction=64, max_layer=16
        let hnsw: Hnsw<'static, f32, DistCosine> = Hnsw::new(16, 65_536, 16, 64, DistCosine);
        Self {
            hnsw,
            id_map: Vec::new(),
            rev_map: HashMap::new(),
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
        self.hnsw.insert((embedding, hnsw_id));
    }

    /// Tombstone a doc_id so it no longer appears in search results.
    pub fn tombstone(&mut self, doc_id: i64) -> bool {
        if let Some(&hnsw_id) = self.rev_map.get(&doc_id) {
            self.id_map[hnsw_id] = None;
            self.rev_map.remove(&doc_id);
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
    pub fn search(&self, query: &[f32], top_k: usize) -> Vec<(i64, f64)> {
        if self.rev_map.is_empty() || top_k == 0 {
            return vec![];
        }
        // Fetch extra candidates to compensate for tombstoned slots.
        let fetch_k = (top_k * 4).max(top_k + 16);
        let neighbours = self.hnsw.search(query, fetch_k, 64);
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

        tracing::info!(
            "storage: loaded {} active tenant(s)",
            indices.len()
        );
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
