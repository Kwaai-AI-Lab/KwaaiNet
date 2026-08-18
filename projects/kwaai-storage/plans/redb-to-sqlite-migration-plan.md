# Migrate kwaai-storage's vector index from redb to SQLite (WAL)

## Context

`kwaai-storage::StorageDb` is the last remaining redb-backed store in the codebase. Its sibling stores in `kwaai-rag` (`MetaStore` for chunk/doc metadata, `GraphStore` for the entity graph) were already migrated to SQLite+WAL in v0.5.1 (commit `d651483`) specifically to fix multi-process lock contention — redb takes an exclusive, all-or-nothing OS file lock with no timeout/retry, so any two processes touching the same KB directory at once (e.g. two `rag eval` runs, or a `dream` background job running alongside a `chat` session) would fail with `Database already open. Cannot acquire lock.`

We hit exactly that error today: two concurrent `kwaainet rag eval` runs against the same KB failed at `kwaai_storage::StorageDb::open()` (`core/crates/kwaai-storage/src/db.rs:221-223`), which still creates/opens a `redb::Database` at a fixed `metadata.redb` file. The migration to SQLite covered chunk metadata and the graph, but this vector-index store was out of scope (confirmed: no mention of `kwaai-storage` anywhere in `projects/kwaai-knowledge/plans/SQLiteMigration-plan.md`). This plan closes that gap using the exact same proven pattern.

Data-continuity note: per user confirmation, a "rebuild-only" strategy (same as the original migration — old `.redb` left inert, data re-provisioned rather than byte-converted) is acceptable here, since no production Eve storage-fabric node currently holds real tenant data (VPK is still Phase 1 MVP, per project memory).

## Scope

Confined to one crate plus a 2-line filename fix in one caller:
- `core/crates/kwaai-storage/src/db.rs`, `tenant.rs`, `vectors.rs`, `Cargo.toml`
- `core/crates/kwaai-cli/src/storage.rs` (lines 137, 164 — hardcoded `metadata.redb` size lookups)

**No changes needed** to `kwaai-storage/src/api.rs` (Axum HTTP layer — no direct redb calls), `kwaai-cli/src/storage_rpc.rs` (P2P relay — goes through `TenantManager`/`VectorStore` public API only), `rag_cmd.rs`, `rag_api.rs`, `vpk_bench.rs` — all consume only `StorageDb::open`, `TenantManager`, `VectorStore`, `SearchResult`, `TenantInfo`/`TenantStats`, which keep their signatures. `kwaai-storage` is only depended on by `kwaai-cli` (optional `storage` feature) — no other crate in the workspace touches it.

Key fact that keeps this migration low-risk: vector **search** never touches the database today — `VectorStore::search()` (`vectors.rs:80-99`) and `TenantIndex`'s HNSW/brute-force logic (`db.rs:53-194`) are pure in-memory operations. Only *persistence* (tenant CRUD, vector upload/delete, and the full-table rebuild-on-open) touches the DB, so the swap is purely a persistence-layer change — the search algorithm is untouched.

## Implementation

**1. `Cargo.toml`** — remove `redb = "2"`, add `rusqlite = { version = "0.31", features = ["bundled"] }` (exact version/features already used by `kwaai-rag/Cargo.toml:20`, for workspace consistency).

**2. `db.rs` — schema + `StorageDb`**
- Replace the two redb `TableDefinition<&[u8],&[u8]>` constants with SQLite DDL: `tenants (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID` and `vectors (key BLOB PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID` — byte-for-byte identical key/value layouts to today (16-byte UUID; 24-byte `tenant_id(16) ++ doc_id_be(8)`), so `vector_key()`, `f32s_to_bytes`/`bytes_to_f32s`, and `TenantRecord` JSON encoding (`db.rs:34-42, 293-308`) need **zero changes**.
- New file `metadata.db` (was `metadata.redb`), opened via `Connection::open()` with the standard pragma block copied verbatim from `meta_store.rs:71-76`: `PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA cache_size=-65536; PRAGMA temp_store=MEMORY;`
- `DbInner`: replace the bare `db: Database` field with `conn: Mutex<SafeConn>` (new `struct SafeConn(Connection); unsafe impl Send for SafeConn {}`, same justification comment as `meta_store.rs:39-46`). Use the `MetaStore` pattern here (not `GraphStore`'s bare-`unsafe impl Send`) because `StorageDb` is `Arc`-cloned and called concurrently from many call sites (`api.rs` handlers, `storage_rpc.rs::dispatch`, `rag_cmd.rs`, `rag_api.rs`) with no external mutex — it needs to be genuinely `Sync`, which `Mutex<SafeConn>` provides.
- `open()`: add the legacy-detection warning matching `meta_store.rs:63-68`'s style: `if data_dir.join("metadata.redb").exists() { eprintln!("⚠  Legacy redb store detected. Run \`kwaainet rag rebuild\` to migrate."); }` — existence check only, never reads the old file (per the confirmed rebuild-only precedent).
- `rebuild_indices()` logic (`db.rs:245-285`) is unchanged in structure — same two-pass scan (tenants → build empty `TenantIndex` per active tenant, then vectors → `insert()` each into its tenant's index) — just backed by `SELECT key, value FROM tenants` / `SELECT key, value FROM vectors` instead of redb table iteration.
- Add a small new public accessor (e.g. `StorageDb::db_size_bytes(&self) -> Result<u64>`) so `kwaai-cli` stops hardcoding the filename — cleaner than just search-replacing `metadata.redb` → `metadata.db` in a sibling crate.

**3. `tenant.rs` — `TenantManager`**: swap each redb `begin_write/open_table/insert/commit` and `begin_read/open_table/get/iter/range` call for the rusqlite equivalent (`INSERT OR REPLACE INTO tenants (key,value) VALUES (?,?)`, `SELECT value FROM tenants WHERE key=?`, `SELECT key,value FROM tenants`). The cascade-delete range-scan (`tenant.rs:158-179`, currently a manual iterate-collect-then-remove dance because redb can't mutate a table under a live range iterator) becomes a single `DELETE FROM vectors WHERE key >= ?1 AND key < ?2` using the same 24-byte prefix-bound trick already proven in `meta_store.rs`'s range-scan code — simpler than today's redb version.

**4. `vectors.rs` — `VectorStore`**: swap `upload()`/`delete()`'s redb write-txn blocks (`vectors.rs:56-66, 117-126`) for `INSERT OR REPLACE`/`DELETE` statements, batched inside a single `rusqlite` transaction for multi-row calls (same atomicity as today's one redb write txn). `search()` and `count()` are untouched — they don't touch the DB at all.

**5. `kwaai-cli/src/storage.rs`**: replace the two hardcoded `data_dir.join("metadata.redb")` size lookups (lines 137, 164) with the new `StorageDb::db_size_bytes()` accessor.

**6. Tests**: the existing 63 tests across `db.rs`, `vectors.rs`, `tenant.rs` (inline `#[cfg(test)]`/`#[tokio::test]`), `tests/storage.rs`, and `tests/api.rs` are all written against the public API (no test touches redb tables or the `metadata.redb` file format directly) — they should compile and pass unmodified. `tests/storage.rs::vectors_survive_store_reopen` is the most important one to watch since it exercises `rebuild_indices()` end-to-end.
   - **New regression test** (per this project's bug-driven-tests convention): add a test in `tests/storage.rs` that opens two `StorageDb` handles concurrently against the same `data_dir` (or spawns two child processes hitting the same dir) and asserts no lock error — this directly encodes the bug being fixed.

## Verification

1. `cd core && cargo test -p kwaai-storage` — all existing + new tests green.
2. `cd core && cargo build -p kwaainet --release && cp core/target/release/kwaainet ~/.cargo/bin/kwaainet && codesign -s - --force ~/.cargo/bin/kwaainet` (standing project convention).
3. Repro the original bug and confirm it's fixed: run two concurrent `kwaainet rag eval` (or `rag query`) processes against the **same** KB simultaneously (this is exactly what failed earlier in this session with `MobyDick`) — confirm neither errors with "Database already open."
4. `kwaainet storage init` / `kwaainet storage status` smoke test to confirm the Eve-node path still opens/reports correctly against the new `metadata.db`.
5. Re-run `rag eval` on 1-2 already-migrated KBs (e.g. MobyDick, Legal) to confirm retrieval scores are unchanged from their last recorded eval (score parity check, same spirit as the original migration's verification) — a formality since the search/HNSW logic itself is untouched, but worth confirming once.
