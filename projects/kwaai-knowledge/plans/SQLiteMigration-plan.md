# Replace redb with SQLite (WAL) for Multi-Process DB Access

## Context

Throughout the multi-corpus eval pipeline we repeatedly hit:

```
Error: opening meta store at ~/.kwaainet/rag/MobyDick/xxx.redb
Caused by: Database already open. Cannot acquire lock.
```

**Root cause**: redb uses OS-level file locking (one exclusive lock per `.redb` file per OS process). It is an embedded single-process database. Any two `kwaainet` invocations touching the same KB simultaneously — an eval still running while a new pipeline starts, or stale processes left by a killed script — produce an immediate hard failure with no retry or wait.

This caused cascading failures: killed pipelines left orphaned eval/ingest children holding locks; the next pipeline launched and failed on every KB those children had open. Circuit breaker state from the 503 storm also contributed to the DeepSea graph build producing 0 entities.

The fix is to replace redb with **SQLite in WAL mode**. SQLite WAL allows multiple concurrent readers and one writer across OS processes simultaneously — exactly the pattern we need (eval reads while graph build writes).

---

## Root Cause Detail

| Store | File | Lock behaviour |
|-------|------|----------------|
| `MetaStore` | `{tenant_id}.redb` | Opened once per `cmd_eval` call (whole session); per ingest |
| `GraphStore` | `graph-{tenant_id}.redb` | Opened per-question in eval; held for **hours** during graph build |
| `QueryCache` | `query_cache.redb` | Opened per retrieval |

Graph build wraps `GraphStore` in `Arc<Mutex<GraphStore>>` and holds it for the entire build (hours). Any concurrent eval of the same KB fails immediately at `Database::create()`. There is no retry, no wait, no graceful degradation.

redb v2 exposes no timeout or retry API at the file-lock level. The lock is all-or-nothing.

---

## Alternatives Considered

| Option | Multi-process | Pure Rust | Maturity | Notes |
|--------|--------------|-----------|----------|-------|
| **SQLite WAL (rusqlite)** | ✅ readers + writer | ✅ bindings | ★★★★★ | Recommended |
| LMDB (heed) | ✅ MVCC | ✅ bindings | ★★★★ | Max-size must be pre-set; directory not single file |
| fjall | ✅ LSM multi-writer | ✅ native | ★★★ | Newer, less battle-tested |
| RocksDB | ✅ | ❌ C++ dep | ★★★★ | Overkill; C++ build complexity |
| Sled | ❌ single-process | ✅ native | ★★★ | Same class of problem as redb |

**SQLite WAL** is the right choice:
- WAL mode: concurrent readers never block; one writer proceeds alongside N readers across processes
- All our values are already `Vec<u8>` / JSON — direct `BLOB` storage, no schema redesign
- Each redb `TableDefinition` maps directly to a `(key BLOB PRIMARY KEY, value BLOB)` SQL table
- `rusqlite` is the most-used embedded Rust DB crate, extremely stable
- SQLite files are inspectable with standard tooling (`sqlite3`, DB Browser)
- No pre-declared size limits (unlike LMDB)

---

## Implementation Plan

### 1. Cargo.toml (`core/crates/kwaai-rag/Cargo.toml`)

```toml
# Remove:
redb = "2"

# Add:
rusqlite = { version = "0.31", features = ["bundled"] }
```

`bundled` embeds SQLite so no system library is needed. This matches the redb approach of zero system dependencies.

### 2. Schema (same for all three stores)

Each redb `TableDefinition<&[u8], &[u8]>` becomes one SQL table:

```sql
CREATE TABLE IF NOT EXISTS {table_name} (
    key   BLOB NOT NULL PRIMARY KEY,
    value BLOB NOT NULL
) WITHOUT ROWID;
```

`WITHOUT ROWID` is appropriate here: all tables are keyed by blob, no auto-increment needed, and it's faster for point lookups.

WAL + synchronous pragmas applied once at open time:

```rust
conn.execute_batch(
    "PRAGMA journal_mode=WAL;
     PRAGMA synchronous=NORMAL;
     PRAGMA cache_size=-65536;  -- 64 MB page cache
     PRAGMA temp_store=MEMORY;"
)?;
```

### 3. `cache.rs` — QueryCache (1 table, simplest)

Replace `redb::Database` with `rusqlite::Connection`.

**Before (redb)**:
```rust
struct QueryCache { db: Database, ... }
// open: Database::create(&path)
// read: db.begin_read()?.open_table(ENTRIES_TABLE)?
// write: db.begin_write()?.open_table(ENTRIES_TABLE)?.insert(k, v)?  wtxn.commit()?
```

**After (rusqlite)**:
```rust
struct QueryCache { conn: Mutex<Connection>, ... }
// open: Connection::open(&path)? then pragma setup + CREATE TABLE IF NOT EXISTS
// read: conn.lock().query_row("SELECT value FROM entries WHERE key=?", [key], ...)
// write: conn.lock().execute("INSERT OR REPLACE INTO entries VALUES (?,?)", [key, val])?
```

`Mutex<Connection>` because `rusqlite::Connection` is not `Send` by default (though `rusqlite` does expose a `send_feature`). Alternatively use a single-connection pool via `r2d2-sqlite`.

Since `QueryCache` is low-contention (one process uses it at a time), a `Mutex<Connection>` is sufficient.

### 4. `meta_store.rs` — MetaStore (4 tables)

Same pattern. The 4 tables become:

```sql
CREATE TABLE IF NOT EXISTS chunks     (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS docs       (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS sync       (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS summary_nodes (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
```

Key encoding is unchanged — tenant UUID prefix + entity/doc ID bytes as before.

**Iteration** (currently `rtxn.open_table(T)?.iter()?`):

```rust
let conn = self.conn.lock().unwrap();
let mut stmt = conn.prepare("SELECT key, value FROM chunks WHERE key >= ? AND key < ?")?;
// range scan by tenant_id prefix
```

All 4 tables use the same 16-byte tenant UUID prefix, so range scans by prefix replace the current full-table iteration with a `WHERE key >= prefix AND key < prefix+1` range.

### 5. `graph.rs` — GraphStore (7 tables)

Largest change. The 7 tables:

```sql
CREATE TABLE IF NOT EXISTS entities        (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS relations       (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS chunk_entity    (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS entity_chunk    (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS timeline        (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS interactions    (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS metadata        (key TEXT PRIMARY KEY, value TEXT NOT NULL);
```

`metadata` uses `TEXT` keys since it's already a `&str` table.

**Connection sharing during graph build**: `GraphStore` holds `Arc<Mutex<Connection>>`. The graph build keeps `Arc<Mutex<GraphStore>>` alive for hours — `Mutex<Connection>` inside serialises writes, while **other processes** (eval) can open their own `Connection` to the same SQLite WAL file and read concurrently without blocking or erroring.

This is the key improvement: with redb one process, one lock. With SQLite WAL, the eval process opens its own connection and reads freely while the build process writes.

### 6. Data migration

On first open of a KB after upgrade, there will be no `.db` file yet (redb files have `.redb` extension; SQLite files will use `.db`). Old `.redb` files will simply be ignored (not deleted automatically — leave for the user to clean up).

If a KB was previously built with redb, it needs to be rebuilt (`kwaainet rag rebuild`). This is acceptable — it's the same requirement as any schema migration. Add a one-time notice in the `open()` functions:

```rust
if data_dir.join(format!("{}.redb", tenant_id)).exists() {
    eprintln!("⚠  Legacy redb store detected. Run `kwaainet rag rebuild` to migrate.");
}
```

### 7. Connection pool option (future)

If read concurrency within a single process becomes a bottleneck, `r2d2-sqlite` or `deadpool-sqlite` provides a connection pool. For now, `Mutex<Connection>` is sufficient — our bottleneck is GPU inference throughput, not DB reads.

---

## Files Changed

| File | Change |
|------|--------|
| `core/crates/kwaai-rag/Cargo.toml` | `redb = "2"` → `rusqlite = { version = "0.31", features = ["bundled"] }` |
| `core/crates/kwaai-rag/src/cache.rs` | Replace redb with rusqlite; `Mutex<Connection>` |
| `core/crates/kwaai-rag/src/meta_store.rs` | Replace redb with rusqlite; 4 tables |
| `core/crates/kwaai-rag/src/graph.rs` | Replace redb with rusqlite; 7 tables; `Arc<Mutex<Connection>>` |

No changes to `rag_cmd.rs`, `retriever.rs`, `ingestion.rs`, or any other file — the store APIs are unchanged; only the internal implementation is swapped.

---

## Verification

```bash
# Build
cd core && cargo build -p kwaainet

# Full pipeline on a small KB — confirm no lock errors with concurrent access
kwaainet rag rebuild docs/test.pdf --kb LockTest --model llama3.1:8b \
  --inference-urls p2p://METRO_LINUX --workers 2 --entity-types Person &

# While rebuild is running, query the same KB (should succeed, not lock-fail)
kwaainet rag query --kb LockTest "test query"

# Confirm WAL files created
ls ~/.kwaainet/rag/LockTest/*.db-wal   # should exist while write is in progress

# Run D6 eval to confirm correctness (score should match pre-migration baseline of 88.9%)
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json
```

Expected: concurrent query succeeds without "Database already open" error. D6 eval score within 1% of 88.9% baseline.
