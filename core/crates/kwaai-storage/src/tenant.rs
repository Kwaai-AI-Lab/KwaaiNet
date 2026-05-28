//! Tenant management backed by redb.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use redb::ReadableTable;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::db::{DbInner, StorageDb, TenantIndex, TenantRecord, TENANTS_TABLE, VECTORS_TABLE};

/// Public tenant info returned by the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantInfo {
    pub tenant_id: Uuid,
    pub peer_id: String,
    pub display_name: Option<String>,
    pub capacity_limit_mb: i64,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub vector_dimension: u32,
}

/// Per-tenant storage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantStats {
    pub tenant_id: Uuid,
    pub vector_count: i64,
    pub storage_bytes: i64,
}

/// Tenant lifecycle manager (backed by the embedded store).
#[derive(Clone)]
pub struct TenantManager {
    store: StorageDb,
}

impl TenantManager {
    pub fn new(store: StorageDb) -> Self {
        Self { store }
    }

    fn inner(&self) -> &DbInner {
        &self.store.inner
    }

    /// Create a new tenant and its in-memory HNSW index.
    pub async fn create(
        &self,
        peer_id: &str,
        capacity_limit_mb: i64,
        display_name: Option<&str>,
        vector_dimension: usize,
    ) -> Result<TenantInfo> {
        let tenant_id = Uuid::new_v4();
        let created_at = Utc::now().to_rfc3339();

        let record = TenantRecord {
            peer_id: peer_id.to_string(),
            display_name: display_name.map(str::to_string),
            capacity_limit_mb,
            status: "Active".to_string(),
            created_at: created_at.clone(),
            vector_dimension,
        };

        // Persist to redb.
        let wtxn = self.inner().db.begin_write()?;
        {
            let mut table = wtxn.open_table(TENANTS_TABLE)?;
            table.insert(
                tenant_id.as_bytes().as_ref(),
                serde_json::to_vec(&record)?.as_slice(),
            )?;
        }
        wtxn.commit()?;

        // Create in-memory index.
        self.inner().indices.write().unwrap().insert(
            tenant_id,
            Arc::new(Mutex::new(TenantIndex::new(vector_dimension))),
        );

        Ok(TenantInfo {
            tenant_id,
            peer_id: record.peer_id,
            display_name: record.display_name,
            capacity_limit_mb: record.capacity_limit_mb,
            status: record.status,
            created_at,
            vector_dimension: record.vector_dimension as u32,
        })
    }

    /// List all active tenants.
    pub async fn list(&self) -> Result<Vec<TenantInfo>> {
        let rtxn = self.inner().db.begin_read()?;
        let table = rtxn.open_table(TENANTS_TABLE)?;

        let mut out = Vec::new();
        for entry in table.iter()? {
            let (k, v) = entry?;
            let tid = Uuid::from_slice(k.value()).context("corrupt tenant key")?;
            let rec: TenantRecord = serde_json::from_slice(v.value())?;
            if rec.status != "Deleted" {
                out.push(record_to_info(tid, &rec));
            }
        }
        out.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        Ok(out)
    }

    /// Get a single active tenant by ID.
    pub async fn get(&self, tenant_id: Uuid) -> Result<Option<TenantInfo>> {
        let rtxn = self.inner().db.begin_read()?;
        let table = rtxn.open_table(TENANTS_TABLE)?;

        match table.get(tenant_id.as_bytes().as_ref())? {
            None => Ok(None),
            Some(v) => {
                let rec: TenantRecord = serde_json::from_slice(v.value())?;
                if rec.status == "Deleted" {
                    Ok(None)
                } else {
                    Ok(Some(record_to_info(tenant_id, &rec)))
                }
            }
        }
    }

    /// Soft-delete a tenant and remove its vector data.
    pub async fn delete(&self, tenant_id: Uuid) -> Result<()> {
        // Mark deleted in redb.
        let wtxn = self.inner().db.begin_write()?;
        {
            let mut tenants = wtxn.open_table(TENANTS_TABLE)?;
            // Clone the bytes out before dropping the immutable AccessGuard.
            let rec_bytes: Vec<u8> = tenants
                .get(tenant_id.as_bytes().as_ref())?
                .context("tenant not found")?
                .value()
                .to_vec();
            let mut rec: TenantRecord = serde_json::from_slice(&rec_bytes)?;
            if rec.status == "Deleted" {
                bail!("tenant not found: {}", tenant_id);
            }
            rec.status = "Deleted".to_string();
            tenants.insert(
                tenant_id.as_bytes().as_ref(),
                serde_json::to_vec(&rec)?.as_slice(),
            )?;

            // Delete all vectors for this tenant from redb.
            let mut vectors = wtxn.open_table(VECTORS_TABLE)?;
            let prefix = *tenant_id.as_bytes();
            // Collect keys to delete (can't mutate while iterating range).
            let keys_to_delete: Vec<[u8; 24]> = {
                let start: [u8; 24] = {
                    let mut k = [0u8; 24];
                    k[..16].copy_from_slice(&prefix);
                    k
                };
                let mut collected = Vec::new();
                for entry in vectors.range(start.as_ref()..)? {
                    let (k, _) = entry?;
                    let kb = k.value();
                    if kb.len() < 16 || kb[..16] != prefix {
                        break;
                    }
                    let mut arr = [0u8; 24];
                    arr.copy_from_slice(kb);
                    collected.push(arr);
                }
                collected
            };
            for k in keys_to_delete {
                vectors.remove(k.as_ref())?;
            }
        }
        wtxn.commit()?;

        // Remove in-memory index.
        self.inner().indices.write().unwrap().remove(&tenant_id);

        Ok(())
    }

    /// Storage stats for a tenant.
    pub async fn stats(&self, tenant_id: Uuid) -> Result<TenantStats> {
        let indices = self.inner().indices.read().unwrap();
        let vector_count = indices
            .get(&tenant_id)
            .map(|arc| arc.lock().unwrap().live_count() as i64)
            .unwrap_or(0);

        // Estimate storage: 4 bytes per float * dimension * vector_count + 24-byte redb key overhead.
        let dim = indices
            .get(&tenant_id)
            .map(|arc| arc.lock().unwrap().dimension)
            .unwrap_or(384);
        let storage_bytes = vector_count * (4 * dim as i64 + 24);

        Ok(TenantStats {
            tenant_id,
            vector_count,
            storage_bytes,
        })
    }

    /// Count of active tenants.
    pub async fn count(&self) -> Result<i64> {
        Ok(self.inner().indices.read().unwrap().len() as i64)
    }

    /// Total live vectors across all tenants.
    pub async fn total_vectors(&self) -> Result<i64> {
        let n = self
            .inner()
            .indices
            .read()
            .unwrap()
            .values()
            .map(|arc| arc.lock().unwrap().live_count() as i64)
            .sum();
        Ok(n)
    }

    /// Total storage bytes across all active tenants (used for Eve-level capacity check).
    /// Uses the same formula as per-tenant stats: live_count * (4 * dim + 24).
    pub async fn total_storage_bytes(&self) -> Result<i64> {
        let bytes = self
            .inner()
            .indices
            .read()
            .unwrap()
            .values()
            .map(|arc| {
                let idx = arc.lock().unwrap();
                idx.live_count() as i64 * (4 * idx.dimension as i64 + 24)
            })
            .sum();
        Ok(bytes)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::StorageDb;
    use crate::vectors::VectorStore;
    use tempfile::TempDir;

    fn setup() -> (TempDir, TenantManager) {
        let tmp = TempDir::new().unwrap();
        let db = StorageDb::open(tmp.path()).unwrap();
        (tmp, TenantManager::new(db))
    }

    fn setup_with_vectors() -> (TempDir, TenantManager, VectorStore) {
        let tmp = TempDir::new().unwrap();
        let db = StorageDb::open(tmp.path()).unwrap();
        let tm = TenantManager::new(db.clone());
        let vs = VectorStore::new(db);
        (tmp, tm, vs)
    }

    #[tokio::test]
    async fn create_returns_info_with_correct_fields() {
        let (_tmp, tm) = setup();
        let info = tm
            .create("peer-abc", 512, Some("My Tenant"), 128)
            .await
            .unwrap();
        assert_eq!(info.peer_id, "peer-abc");
        assert_eq!(info.capacity_limit_mb, 512);
        assert_eq!(info.display_name.as_deref(), Some("My Tenant"));
        assert_eq!(info.vector_dimension, 128);
        assert_eq!(info.status, "Active");
        assert!(!info.tenant_id.is_nil());
    }

    #[tokio::test]
    async fn list_returns_all_active_tenants() {
        let (_tmp, tm) = setup();
        tm.create("p1", 100, None, 4).await.unwrap();
        tm.create("p2", 200, None, 4).await.unwrap();
        let list = tm.list().await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn list_excludes_deleted_tenants() {
        let (_tmp, tm) = setup();
        let t = tm.create("p1", 100, None, 4).await.unwrap();
        tm.create("p2", 200, None, 4).await.unwrap();
        tm.delete(t.tenant_id).await.unwrap();
        let list = tm.list().await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].peer_id, "p2");
    }

    #[tokio::test]
    async fn get_returns_tenant_by_id() {
        let (_tmp, tm) = setup();
        let t = tm.create("peer-x", 100, None, 4).await.unwrap();
        let found = tm.get(t.tenant_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().peer_id, "peer-x");
    }

    #[tokio::test]
    async fn get_returns_none_for_unknown_id() {
        let (_tmp, tm) = setup();
        assert!(tm.get(Uuid::new_v4()).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn get_returns_none_after_delete() {
        let (_tmp, tm) = setup();
        let t = tm.create("peer", 100, None, 4).await.unwrap();
        tm.delete(t.tenant_id).await.unwrap();
        assert!(tm.get(t.tenant_id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn delete_unknown_tenant_returns_error() {
        let (_tmp, tm) = setup();
        assert!(tm.delete(Uuid::new_v4()).await.is_err());
    }

    #[tokio::test]
    async fn count_tracks_active_tenants() {
        let (_tmp, tm) = setup();
        assert_eq!(tm.count().await.unwrap(), 0);
        let t = tm.create("p1", 100, None, 4).await.unwrap();
        assert_eq!(tm.count().await.unwrap(), 1);
        tm.delete(t.tenant_id).await.unwrap();
        assert_eq!(tm.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn stats_zero_for_new_tenant() {
        let (_tmp, tm) = setup();
        let t = tm.create("p", 100, None, 4).await.unwrap();
        let s = tm.stats(t.tenant_id).await.unwrap();
        assert_eq!(s.vector_count, 0);
        assert_eq!(s.storage_bytes, 0);
    }

    #[tokio::test]
    async fn stats_updates_after_upload() {
        let (_tmp, tm, vs) = setup_with_vectors();
        let t = tm.create("p", 100, None, 4).await.unwrap();
        vs.upload(t.tenant_id, &[(1, vec![1.0, 0.0, 0.0, 0.0])])
            .await
            .unwrap();
        let s = tm.stats(t.tenant_id).await.unwrap();
        assert_eq!(s.vector_count, 1);
        // formula: count * (4 * dim + 24) = 1 * (16 + 24) = 40
        assert_eq!(s.storage_bytes, 40);
    }

    #[tokio::test]
    async fn total_vectors_sums_across_tenants() {
        let (_tmp, tm, vs) = setup_with_vectors();
        let t1 = tm.create("p1", 100, None, 4).await.unwrap();
        let t2 = tm.create("p2", 100, None, 4).await.unwrap();
        vs.upload(t1.tenant_id, &[(1, vec![1.0, 0.0, 0.0, 0.0])])
            .await
            .unwrap();
        vs.upload(
            t2.tenant_id,
            &[
                (1, vec![0.0, 1.0, 0.0, 0.0]),
                (2, vec![0.0, 0.0, 1.0, 0.0]),
            ],
        )
        .await
        .unwrap();
        assert_eq!(tm.total_vectors().await.unwrap(), 3);
    }
}

fn record_to_info(tenant_id: Uuid, rec: &TenantRecord) -> TenantInfo {
    TenantInfo {
        tenant_id,
        peer_id: rec.peer_id.clone(),
        display_name: rec.display_name.clone(),
        capacity_limit_mb: rec.capacity_limit_mb,
        status: rec.status.clone(),
        created_at: rec.created_at.clone(),
        vector_dimension: rec.vector_dimension as u32,
    }
}
