use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use uuid::Uuid;

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
    /// Section heading active when this chunk was produced (from DocSchema).
    #[serde(default)]
    pub section_name: Option<String>,
    /// True if this chunk's section is marked skip=true in the DocSchema.
    #[serde(default)]
    pub skip_extraction: bool,
    /// Narrator note for this chunk's section — injected into extraction prompts.
    #[serde(default)]
    pub section_note: Option<String>,
    /// Semantic section type — used for context-window boundary enforcement.
    #[serde(default)]
    pub section_type: crate::doc_schema::SectionType,
}

// Newtype wrapper so we can implement Send on rusqlite::Connection.
// Safe because SQLite in serialized mode (the default) is thread-safe; the Mutex
// in MetaStore ensures only one thread accesses the connection at a time.
struct SafeConn(Connection);
unsafe impl Send for SafeConn {}

pub struct MetaStore {
    conn: Mutex<SafeConn>,
    tenant_id: Uuid,
}

// MetaStore is Send + Sync via Mutex<SafeConn>: no additional unsafe needed.

impl MetaStore {
    fn db(&self) -> MutexGuard<'_, SafeConn> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn open(data_dir: &Path, tenant_id: Uuid) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join(format!("{}.db", tenant_id));
        if data_dir.join(format!("{}.redb", tenant_id)).exists() {
            eprintln!(
                "⚠  Legacy redb store detected for {}. Run `kwaainet rag rebuild` to migrate.",
                tenant_id
            );
        }
        let conn = Connection::open(&path)
            .with_context(|| format!("opening meta store at {}", path.display()))?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=-65536;
             PRAGMA temp_store=MEMORY;",
        )?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS chunks (
                key BLOB NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
             ) WITHOUT ROWID;
             CREATE TABLE IF NOT EXISTS docs (
                key BLOB NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
             ) WITHOUT ROWID;
             CREATE TABLE IF NOT EXISTS sync (
                key BLOB NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
             ) WITHOUT ROWID;
             CREATE TABLE IF NOT EXISTS summary_nodes (
                key BLOB NOT NULL PRIMARY KEY,
                value BLOB NOT NULL
             ) WITHOUT ROWID;",
        )?;
        Ok(Self {
            conn: Mutex::new(SafeConn(conn)),
            tenant_id,
        })
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

        let mut doc_ids: std::collections::HashMap<&str, Vec<i64>> = Default::default();
        for (meta, &id) in chunks.iter().zip(ids.iter()) {
            doc_ids.entry(&meta.doc_name).or_default().push(id);
        }

        let db = self.db();
        let txn = db.0.unchecked_transaction()?;

        for (meta, &id) in chunks.iter().zip(ids.iter()) {
            let key = Self::chunk_key(self.tenant_id, id);
            let val = serde_json::to_vec(meta)?;
            txn.execute(
                "INSERT OR REPLACE INTO chunks (key, value) VALUES (?1, ?2)",
                params![key.as_ref(), val.as_slice()],
            )?;
        }

        for (doc_name, new_ids) in &doc_ids {
            let key = Self::doc_key(self.tenant_id, doc_name);
            let mut existing: Vec<i64> = txn
                .query_row(
                    "SELECT value FROM docs WHERE key = ?1",
                    params![key.as_slice()],
                    |r| r.get::<_, Vec<u8>>(0),
                )
                .optional()?
                .and_then(|v| serde_json::from_slice(&v).ok())
                .unwrap_or_default();
            existing.extend_from_slice(new_ids);
            existing.sort_unstable();
            existing.dedup();
            let val = serde_json::to_vec(&existing)?;
            txn.execute(
                "INSERT OR REPLACE INTO docs (key, value) VALUES (?1, ?2)",
                params![key.as_slice(), val.as_slice()],
            )?;
        }

        txn.commit()?;
        Ok(())
    }

    pub fn get_chunks(&self, ids: &[i64]) -> Result<Vec<Option<ChunkMeta>>> {
        let db = self.db();
        let mut out = Vec::with_capacity(ids.len());
        for &id in ids {
            let key = Self::chunk_key(self.tenant_id, id);
            let meta: Option<ChunkMeta> =
                db.0.query_row(
                    "SELECT value FROM chunks WHERE key = ?1",
                    params![key.as_ref()],
                    |r| r.get::<_, Vec<u8>>(0),
                )
                .optional()?
                .and_then(|v| serde_json::from_slice(&v).ok());
            out.push(meta);
        }
        Ok(out)
    }

    pub fn all_chunks(&self) -> Result<Vec<(i64, ChunkMeta)>> {
        let prefix = self.tenant_id.as_bytes();
        let start: [u8; 24] = {
            let mut k = [0u8; 24];
            k[..16].copy_from_slice(prefix);
            k
        };
        let db = self.db();
        let mut stmt =
            db.0.prepare("SELECT key, value FROM chunks WHERE key >= ?1 ORDER BY key")?;
        let rows: Vec<(Vec<u8>, Vec<u8>)> = stmt
            .query_map(params![start.as_ref()], |r| {
                Ok((r.get::<_, Vec<u8>>(0)?, r.get::<_, Vec<u8>>(1)?))
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = Vec::new();
        for (kb, v) in rows {
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let id = i64::from_le_bytes(kb[16..24].try_into().unwrap());
            let meta: ChunkMeta = serde_json::from_slice(&v)?;
            out.push((id, meta));
        }
        Ok(out)
    }

    pub fn list_docs(&self) -> Result<Vec<String>> {
        let prefix = self.tenant_id.as_bytes();
        let start: Vec<u8> = prefix.to_vec();
        let db = self.db();
        let mut stmt =
            db.0.prepare("SELECT key FROM docs WHERE key >= ?1 ORDER BY key")?;
        let rows: Vec<Vec<u8>> = stmt
            .query_map(params![start.as_slice()], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        let mut docs = Vec::new();
        for kb in rows {
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
        let db = self.db();
        let ids: Vec<i64> =
            db.0.query_row(
                "SELECT value FROM docs WHERE key = ?1",
                params![doc_key.as_slice()],
                |r| r.get::<_, Vec<u8>>(0),
            )
            .optional()?
            .and_then(|v| serde_json::from_slice(&v).ok())
            .unwrap_or_default();

        if ids.is_empty() {
            return Ok(vec![]);
        }

        let txn = db.0.unchecked_transaction()?;
        for &id in &ids {
            let key = Self::chunk_key(self.tenant_id, id);
            txn.execute("DELETE FROM chunks WHERE key = ?1", params![key.as_ref()])?;
        }
        txn.execute(
            "DELETE FROM docs WHERE key = ?1",
            params![doc_key.as_slice()],
        )?;
        txn.commit()?;
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
        self.db().0.execute(
            "INSERT OR REPLACE INTO sync (key, value) VALUES (?1, ?2)",
            params![key.as_slice(), val.as_slice()],
        )?;
        Ok(())
    }

    pub fn get_sync_meta(&self, doc_name: &str) -> Result<Option<SyncMeta>> {
        let key = Self::sync_key(self.tenant_id, doc_name);
        let meta: Option<SyncMeta> = self
            .db()
            .0
            .query_row(
                "SELECT value FROM sync WHERE key = ?1",
                params![key.as_slice()],
                |r| r.get::<_, Vec<u8>>(0),
            )
            .optional()?
            .and_then(|v| serde_json::from_slice(&v).ok());
        Ok(meta)
    }

    pub fn delete_sync_meta(&self, doc_name: &str) -> Result<()> {
        let key = Self::sync_key(self.tenant_id, doc_name);
        self.db()
            .0
            .execute("DELETE FROM sync WHERE key = ?1", params![key.as_slice()])?;
        Ok(())
    }

    // ── Summary nodes ────────────────────────────────────────────────────────

    fn summary_key(tenant_id: uuid::Uuid, summary_id: i64) -> [u8; 24] {
        let mut k = [0u8; 24];
        k[..16].copy_from_slice(tenant_id.as_bytes());
        k[16..].copy_from_slice(&summary_id.to_le_bytes());
        k
    }

    pub fn put_summary_nodes(&self, nodes: &[crate::summary::SummaryNode]) -> Result<()> {
        let db = self.db();
        let txn = db.0.unchecked_transaction()?;
        for node in nodes {
            let key = Self::summary_key(self.tenant_id, node.id);
            let val = serde_json::to_vec(node)?;
            txn.execute(
                "INSERT OR REPLACE INTO summary_nodes (key, value) VALUES (?1, ?2)",
                params![key.as_ref(), val.as_slice()],
            )?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn all_summary_nodes(&self) -> Result<Vec<crate::summary::SummaryNode>> {
        let prefix = self.tenant_id.as_bytes();
        let start: [u8; 24] = {
            let mut k = [0u8; 24];
            k[..16].copy_from_slice(prefix);
            k
        };
        let db = self.db();
        let mut stmt =
            db.0.prepare("SELECT key, value FROM summary_nodes WHERE key >= ?1 ORDER BY key")?;
        let rows: Vec<(Vec<u8>, Vec<u8>)> = stmt
            .query_map(params![start.as_ref()], |r| {
                Ok((r.get::<_, Vec<u8>>(0)?, r.get::<_, Vec<u8>>(1)?))
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = Vec::new();
        for (kb, v) in rows {
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let node: crate::summary::SummaryNode = serde_json::from_slice(&v)?;
            out.push(node);
        }
        Ok(out)
    }

    pub fn clear_summary_nodes(&self) -> Result<()> {
        let prefix = self.tenant_id.as_bytes();
        let start: [u8; 24] = {
            let mut k = [0u8; 24];
            k[..16].copy_from_slice(prefix);
            k
        };
        let db = self.db();
        let mut stmt =
            db.0.prepare("SELECT key FROM summary_nodes WHERE key >= ?1 ORDER BY key")?;
        let keys: Vec<Vec<u8>> = stmt
            .query_map(params![start.as_ref()], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;

        let txn = db.0.unchecked_transaction()?;
        for kb in &keys {
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            txn.execute(
                "DELETE FROM summary_nodes WHERE key = ?1",
                params![kb.as_slice()],
            )?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Return all (doc_name, SyncMeta) pairs for this tenant.
    pub fn all_sync_metas(&self) -> Result<Vec<(String, SyncMeta)>> {
        let prefix = self.tenant_id.as_bytes();
        let start: Vec<u8> = prefix.to_vec();
        let db = self.db();
        let mut stmt =
            db.0.prepare("SELECT key, value FROM sync WHERE key >= ?1 ORDER BY key")?;
        let rows: Vec<(Vec<u8>, Vec<u8>)> = stmt
            .query_map(params![start.as_slice()], |r| {
                Ok((r.get::<_, Vec<u8>>(0)?, r.get::<_, Vec<u8>>(1)?))
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut out = Vec::new();
        for (kb, v) in rows {
            if kb.len() < 16 || &kb[..16] != prefix.as_ref() {
                break;
            }
            let name = String::from_utf8_lossy(&kb[16..]).into_owned();
            let meta: SyncMeta = serde_json::from_slice(&v)?;
            out.push((name, meta));
        }
        Ok(out)
    }
}
