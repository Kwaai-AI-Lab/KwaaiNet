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
