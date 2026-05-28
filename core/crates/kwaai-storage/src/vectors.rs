//! Per-tenant vector storage backed by hnsw_rs (in-memory) + redb (persistence).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::{f32s_to_bytes, vector_key, DbInner, StorageDb, VECTORS_TABLE};

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
    pub async fn upload(&self, tenant_id: Uuid, vectors: &[(i64, Vec<f32>)]) -> Result<usize> {
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
    pub async fn delete(&self, tenant_id: Uuid, ids: &[i64]) -> Result<usize> {
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::StorageDb;
    use crate::tenant::TenantManager;
    use tempfile::TempDir;

    fn unit(dim: usize, pos: usize) -> Vec<f32> {
        let mut v = vec![0.0f32; dim];
        v[pos] = 1.0;
        v
    }

    async fn setup() -> (TempDir, TenantManager, VectorStore) {
        let tmp = TempDir::new().unwrap();
        let db = StorageDb::open(tmp.path()).unwrap();
        let tm = TenantManager::new(db.clone());
        let vs = VectorStore::new(db);
        (tmp, tm, vs)
    }

    #[tokio::test]
    async fn upload_returns_vector_count() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        let n = vs
            .upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1))])
            .await
            .unwrap();
        assert_eq!(n, 2);
    }

    #[tokio::test]
    async fn upload_empty_slice_returns_zero() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        assert_eq!(vs.upload(t.tenant_id, &[]).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn upload_unknown_tenant_returns_error() {
        let (_tmp, _tm, vs) = setup().await;
        assert!(vs
            .upload(Uuid::new_v4(), &[(1, unit(4, 0))])
            .await
            .is_err());
    }

    #[tokio::test]
    async fn search_returns_nearest_first() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1))])
            .await
            .unwrap();
        let r = vs.search(t.tenant_id, &unit(4, 0), 1).await.unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].id, 1);
        assert!((r[0].score - 1.0).abs() < 1e-6, "score={}", r[0].score);
    }

    #[tokio::test]
    async fn search_scores_in_descending_order() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1))])
            .await
            .unwrap();
        let r = vs.search(t.tenant_id, &unit(4, 0), 2).await.unwrap();
        assert_eq!(r.len(), 2);
        assert!(r[0].score >= r[1].score);
    }

    #[tokio::test]
    async fn search_unknown_tenant_returns_error() {
        let (_tmp, _tm, vs) = setup().await;
        assert!(vs
            .search(Uuid::new_v4(), &unit(4, 0), 5)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn delete_reduces_live_count() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1))])
            .await
            .unwrap();
        let deleted = vs.delete(t.tenant_id, &[1]).await.unwrap();
        assert_eq!(deleted, 1);
        assert_eq!(vs.count(t.tenant_id).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn delete_removes_vector_from_search() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1))])
            .await
            .unwrap();
        vs.delete(t.tenant_id, &[1]).await.unwrap();
        let r = vs.search(t.tenant_id, &unit(4, 0), 10).await.unwrap();
        assert!(!r.iter().any(|x| x.id == 1));
    }

    #[tokio::test]
    async fn delete_nonexistent_ids_returns_zero() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        assert_eq!(vs.delete(t.tenant_id, &[999, 1000]).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn delete_empty_ids_returns_zero() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        assert_eq!(vs.delete(t.tenant_id, &[]).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn count_tracks_upload_and_delete() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        assert_eq!(vs.count(t.tenant_id).await.unwrap(), 0);
        vs.upload(t.tenant_id, &[(1, unit(4, 0))]).await.unwrap();
        assert_eq!(vs.count(t.tenant_id).await.unwrap(), 1);
        vs.delete(t.tenant_id, &[1]).await.unwrap();
        assert_eq!(vs.count(t.tenant_id).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn ensure_table_is_noop() {
        let (_tmp, tm, vs) = setup().await;
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.ensure_table(t.tenant_id, 4).await.unwrap();
        assert_eq!(vs.count(t.tenant_id).await.unwrap(), 0);
    }
}
