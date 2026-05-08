use anyhow::{Context, Result};
use chrono::Utc;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// key = tenant_uuid[16] + chunk_id_i64_le[8] = 24 bytes → ChunkMeta JSON
const CHUNKS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("chunks");

/// key = tenant_uuid[16] + doc_name_bytes → chunk_id list JSON
const DOCS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("docs");

/// key = tenant_uuid[16] + doc_name_bytes → SyncMeta JSON
/// Tracks the last-seen mtime/size for folder-sync change detection.
const SYNC_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("sync");

/// Metadata stored per-doc by `rag sync` for change detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMeta {
    /// Absolute path to the source file on disk.
    pub file_path: String,
    /// Seconds since Unix epoch of the file's last modification time.
    pub mtime_secs: u64,
    /// File size in bytes.
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkMeta {
    pub doc_name: String,
    pub chunk_index: u32,
    pub text: String,
    pub surrounding: String,
    pub page_num: Option<u32>,
    pub ingested_at: String,
}

pub struct MetaStore {
    db: Database,
    tenant_id: Uuid,
}

impl MetaStore {
    pub fn open(data_dir: &Path, tenant_id: Uuid) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join(format!("{}.redb", tenant_id));
        let db = Database::create(&path)
            .with_context(|| format!("opening meta store at {}", path.display()))?;

        // Ensure tables exist.
        let wtxn = db.begin_write()?;
        wtxn.open_table(CHUNKS_TABLE)?;
        wtxn.open_table(DOCS_TABLE)?;
        wtxn.open_table(SYNC_TABLE)?;
        wtxn.commit()?;

        Ok(Self { db, tenant_id })
    }

    fn chunk_key(tenant_id: Uuid, chunk_id: i64) -> [u8; 24] {
        let mut k = [0u8; 24];
        k[..16].copy_from_slice(tenant_id.as_bytes());
        k[16..].copy_from_slice(&chunk_id.to_le_bytes());
        k
    }

    fn doc_key(tenant_id: Uuid, doc_name: &str) -> Vec<u8> {
        let mut k = tenant_id.as_bytes().to_vec();
        k.extend_from_slice(doc_name.as_bytes());
        k
    }

    pub fn put_chunks(&self, chunks: &[ChunkMeta], ids: &[i64]) -> Result<()> {
        assert_eq!(chunks.len(), ids.len());
        let wtxn = self.db.begin_write()?;
        {
            let mut chunk_table = wtxn.open_table(CHUNKS_TABLE)?;
            let mut doc_table = wtxn.open_table(DOCS_TABLE)?;

            // Group chunk IDs by doc_name for the docs index.
            let mut doc_ids: std::collections::HashMap<&str, Vec<i64>> = Default::default();
            for (meta, &id) in chunks.iter().zip(ids.iter()) {
                let key = Self::chunk_key(self.tenant_id, id);
                let val = serde_json::to_vec(meta)?;
                chunk_table.insert(key.as_ref(), val.as_slice())?;
                doc_ids.entry(&meta.doc_name).or_default().push(id);
            }

            for (doc_name, new_ids) in doc_ids {
                let key = Self::doc_key(self.tenant_id, doc_name);
                // Merge with existing IDs.
                let mut existing: Vec<i64> = match doc_table.get(key.as_slice())? {
                    Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                    None => vec![],
                };
                existing.extend_from_slice(&new_ids);
                existing.sort_unstable();
                existing.dedup();
                let val = serde_json::to_vec(&existing)?;
                doc_table.insert(key.as_slice(), val.as_slice())?;
            }
        }
        wtxn.commit()?;
        Ok(())
    }

    pub fn get_chunks(&self, ids: &[i64]) -> Result<Vec<Option<ChunkMeta>>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(CHUNKS_TABLE)?;
        let mut out = Vec::with_capacity(ids.len());
        for &id in ids {
            let key = Self::chunk_key(self.tenant_id, id);
            let meta = match table.get(key.as_ref())? {
                Some(v) => Some(serde_json::from_slice(v.value())?),
                None => None,
            };
            out.push(meta);
        }
        Ok(out)
    }

    pub fn all_chunks(&self) -> Result<Vec<(i64, ChunkMeta)>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(CHUNKS_TABLE)?;
        let prefix = self.tenant_id.as_bytes();
        let start: [u8; 24] = {
            let mut k = [0u8; 24];
            k[..16].copy_from_slice(prefix);
            k
        };
        let mut out = Vec::new();
        for entry in table.range(start.as_ref()..)? {
            let (k, v) = entry?;
            let kb = k.value();
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let id = i64::from_le_bytes(kb[16..24].try_into().unwrap());
            let meta: ChunkMeta = serde_json::from_slice(v.value())?;
            out.push((id, meta));
        }
        Ok(out)
    }

    pub fn list_docs(&self) -> Result<Vec<String>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(DOCS_TABLE)?;
        let prefix = self.tenant_id.as_bytes();
        let mut docs = Vec::new();
        for entry in table.range(prefix.as_ref()..)? {
            let (k, _) = entry?;
            let kb = k.value();
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let name = String::from_utf8_lossy(&kb[16..]).into_owned();
            docs.push(name);
        }
        Ok(docs)
    }

    /// Delete all chunks for a document. Returns the chunk IDs removed.
    pub fn delete_doc(&self, doc_name: &str) -> Result<Vec<i64>> {
        let doc_key = Self::doc_key(self.tenant_id, doc_name);
        let ids: Vec<i64> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(DOCS_TABLE)?;
            match table.get(doc_key.as_slice())? {
                Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                None => return Ok(vec![]),
            }
        };

        let wtxn = self.db.begin_write()?;
        {
            let mut chunk_table = wtxn.open_table(CHUNKS_TABLE)?;
            let mut doc_table = wtxn.open_table(DOCS_TABLE)?;
            for &id in &ids {
                let key = Self::chunk_key(self.tenant_id, id);
                chunk_table.remove(key.as_ref())?;
            }
            doc_table.remove(doc_key.as_slice())?;
        }
        wtxn.commit()?;
        Ok(ids)
    }

    pub fn now_rfc3339() -> String {
        Utc::now().to_rfc3339()
    }

    // ── Sync metadata ────────────────────────────────────────────────────────

    fn sync_key(tenant_id: Uuid, doc_name: &str) -> Vec<u8> {
        let mut k = tenant_id.as_bytes().to_vec();
        k.extend_from_slice(doc_name.as_bytes());
        k
    }

    pub fn put_sync_meta(&self, doc_name: &str, meta: &SyncMeta) -> Result<()> {
        let key = Self::sync_key(self.tenant_id, doc_name);
        let val = serde_json::to_vec(meta)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut table = wtxn.open_table(SYNC_TABLE)?;
            table.insert(key.as_slice(), val.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    pub fn get_sync_meta(&self, doc_name: &str) -> Result<Option<SyncMeta>> {
        let key = Self::sync_key(self.tenant_id, doc_name);
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(SYNC_TABLE)?;
        match table.get(key.as_slice())? {
            Some(v) => Ok(Some(serde_json::from_slice(v.value())?)),
            None => Ok(None),
        }
    }

    pub fn delete_sync_meta(&self, doc_name: &str) -> Result<()> {
        let key = Self::sync_key(self.tenant_id, doc_name);
        let wtxn = self.db.begin_write()?;
        {
            let mut table = wtxn.open_table(SYNC_TABLE)?;
            table.remove(key.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Return all (doc_name, SyncMeta) pairs for this tenant.
    pub fn all_sync_metas(&self) -> Result<Vec<(String, SyncMeta)>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(SYNC_TABLE)?;
        let prefix = self.tenant_id.as_bytes();
        let start: Vec<u8> = prefix.to_vec();
        let mut out = Vec::new();
        for entry in table.range(start.as_slice()..)? {
            let (k, v) = entry?;
            let kb = k.value();
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let name = String::from_utf8_lossy(&kb[16..]).into_owned();
            let meta: SyncMeta = serde_json::from_slice(v.value())?;
            out.push((name, meta));
        }
        Ok(out)
    }
}
