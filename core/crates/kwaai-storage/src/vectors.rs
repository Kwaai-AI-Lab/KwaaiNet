//! Per-tenant vector storage backed by hnsw_rs (in-memory) + redb (persistence).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{DbInner, StorageDb, VECTORS_TABLE, f32s_to_bytes, vector_key};

/// A single search result: opaque doc ID + cosine similarity score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: i64,
    pub score: f64,
}

/// Per-tenant vector store (hnsw_rs + redb).
#[derive(Clone)]
pub struct VectorStore {
    store: StorageDb,
}

impl VectorStore {
    pub fn new(store: StorageDb) -> Self {
        Self { store }
    }

    fn inner(&self) -> &DbInner {
        &self.store.inner
    }

    /// No-op: index is created by TenantManager::create. Kept for API compatibility.
    pub async fn ensure_table(&self, _tenant_id: Uuid, _dimension: usize) -> Result<()> {
        Ok(())
    }

    /// Upload a batch of vectors for a tenant.
    ///
    /// Each entry is `(doc_id, embedding)`. Doc IDs are opaque to Eve — they
    /// are the scrambled/plain document IDs from Bob's pipeline.
    pub async fn upload(
        &self,
        tenant_id: Uuid,
        vectors: &[(i64, Vec<f32>)],
    ) -> Result<usize> {
        if vectors.is_empty() {
            return Ok(0);
        }

        // Verify the tenant's index exists.
        let arc = {
            let indices = self.inner().indices.read().unwrap();
            indices
                .get(&tenant_id)
                .cloned()
                .context("tenant not found")?
        };

        // Persist to redb (one write transaction for the whole batch).
        {
            let wtxn = self.inner().db.begin_write()?;
            {
                let mut table = wtxn.open_table(VECTORS_TABLE)?;
                for (doc_id, embedding) in vectors {
                    let key = vector_key(tenant_id, *doc_id);
                    let bytes = f32s_to_bytes(embedding);
                    table.insert(key.as_ref(), bytes.as_slice())?;
                }
            }
            wtxn.commit()?;
        }

        // Update in-memory HNSW index.
        {
            let mut index = arc.lock().unwrap();
            for (doc_id, embedding) in vectors {
                index.insert(*doc_id, embedding);
            }
        }

        Ok(vectors.len())
    }

    /// ANN search for the top-K most similar vectors (cosine similarity).
    pub async fn search(
        &self,
        tenant_id: Uuid,
        query: &[f32],
        top_k: usize,
    ) -> Result<Vec<SearchResult>> {
        let arc = {
            let indices = self.inner().indices.read().unwrap();
            indices
                .get(&tenant_id)
                .cloned()
                .context("tenant not found")?
        };

        let results = arc.lock().unwrap().search(query, top_k);
        Ok(results
            .into_iter()
            .map(|(id, score)| SearchResult { id, score })
            .collect())
    }

    /// Delete vectors by doc ID.
    pub async fn delete(
        &self,
        tenant_id: Uuid,
        ids: &[i64],
    ) -> Result<usize> {
        if ids.is_empty() {
            return Ok(0);
        }

        let arc = {
            let indices = self.inner().indices.read().unwrap();
            indices
                .get(&tenant_id)
                .cloned()
                .context("tenant not found")?
        };

        // Remove from redb.
        {
            let wtxn = self.inner().db.begin_write()?;
            {
                let mut table = wtxn.open_table(VECTORS_TABLE)?;
                for &doc_id in ids {
                    let key = vector_key(tenant_id, doc_id);
                    table.remove(key.as_ref())?;
                }
            }
            wtxn.commit()?;
        }

        // Tombstone in-memory index.
        let mut deleted = 0;
        {
            let mut index = arc.lock().unwrap();
            for &doc_id in ids {
                if index.tombstone(doc_id) {
                    deleted += 1;
                }
            }
        }

        Ok(deleted)
    }

    /// Count live vectors for a tenant.
    pub async fn count(&self, tenant_id: Uuid) -> Result<i64> {
        let indices = self.inner().indices.read().unwrap();
        Ok(indices
            .get(&tenant_id)
            .map(|arc| arc.lock().unwrap().live_count() as i64)
            .unwrap_or(0))
    }
}
