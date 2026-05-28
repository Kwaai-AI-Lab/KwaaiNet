//! Integration tests: persistence, multi-tenant isolation, capacity stats.

use kwaai_storage::{StorageDb, TenantManager, VectorStore};
use tempfile::TempDir;

fn unit(dim: usize, pos: usize) -> Vec<f32> {
    let mut v = vec![0.0f32; dim];
    v[pos] = 1.0;
    v
}

// ---------------------------------------------------------------------------
// Persistence
// ---------------------------------------------------------------------------

#[tokio::test]
async fn vectors_survive_store_reopen() {
    let tmp = TempDir::new().unwrap();
    let tid;

    // Write phase.
    {
        let db = StorageDb::open(tmp.path()).unwrap();
        let tm = TenantManager::new(db.clone());
        let vs = VectorStore::new(db);
        let t = tm
            .create("persist-peer", 100, Some("Persistence Test"), 4)
            .await
            .unwrap();
        tid = t.tenant_id;
        vs.upload(tid, &[(42, unit(4, 0)), (43, unit(4, 1))])
            .await
            .unwrap();
    }

    // Read phase after reopen.
    let db2 = StorageDb::open(tmp.path()).unwrap();
    let vs2 = VectorStore::new(db2.clone());
    let tm2 = TenantManager::new(db2);

    // Tenant still visible.
    let info = tm2.get(tid).await.unwrap();
    assert!(info.is_some(), "tenant should persist across reopen");
    assert_eq!(info.unwrap().display_name.as_deref(), Some("Persistence Test"));

    // Vectors are searchable.
    let r = vs2.search(tid, &unit(4, 0), 1).await.unwrap();
    assert_eq!(r.len(), 1);
    assert_eq!(r[0].id, 42);
    assert!((r[0].score - 1.0).abs() < 1e-6);

    // Count matches.
    assert_eq!(vs2.count(tid).await.unwrap(), 2);
}

#[tokio::test]
async fn tenant_deletion_persists_across_reopen() {
    let tmp = TempDir::new().unwrap();
    let tid;

    {
        let db = StorageDb::open(tmp.path()).unwrap();
        let tm = TenantManager::new(db.clone());
        let vs = VectorStore::new(db);
        let t = tm.create("doomed", 100, None, 4).await.unwrap();
        tid = t.tenant_id;
        vs.upload(tid, &[(1, unit(4, 0))]).await.unwrap();
        tm.delete(tid).await.unwrap();
    }

    let db2 = StorageDb::open(tmp.path()).unwrap();
    let tm2 = TenantManager::new(db2);

    // Deleted tenant not found.
    assert!(tm2.get(tid).await.unwrap().is_none());
    // Not in list.
    assert!(tm2.list().await.unwrap().is_empty());
}

// ---------------------------------------------------------------------------
// Multi-tenant isolation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn tenants_do_not_see_each_others_vectors() {
    let tmp = TempDir::new().unwrap();
    let db = StorageDb::open(tmp.path()).unwrap();
    let tm = TenantManager::new(db.clone());
    let vs = VectorStore::new(db);

    let t1 = tm.create("alice", 100, None, 4).await.unwrap();
    let t2 = tm.create("bob", 100, None, 4).await.unwrap();

    // t1 stores vector at doc 1 in dim 0; t2 stores doc 1 in dim 1.
    vs.upload(t1.tenant_id, &[(1, unit(4, 0))]).await.unwrap();
    vs.upload(t2.tenant_id, &[(1, unit(4, 1))]).await.unwrap();

    // Searching t1 with query aligned to dim 0 should return doc 1 (score 1.0).
    let r1 = vs.search(t1.tenant_id, &unit(4, 0), 5).await.unwrap();
    assert_eq!(r1.len(), 1);
    assert!((r1[0].score - 1.0).abs() < 1e-6);

    // Searching t2 with query aligned to dim 1 should return doc 1 (score 1.0).
    let r2 = vs.search(t2.tenant_id, &unit(4, 1), 5).await.unwrap();
    assert_eq!(r2.len(), 1);
    assert!((r2[0].score - 1.0).abs() < 1e-6);

    // Each tenant's count is independent.
    assert_eq!(vs.count(t1.tenant_id).await.unwrap(), 1);
    assert_eq!(vs.count(t2.tenant_id).await.unwrap(), 1);
}

#[tokio::test]
async fn deleting_one_tenant_leaves_other_intact() {
    let tmp = TempDir::new().unwrap();
    let db = StorageDb::open(tmp.path()).unwrap();
    let tm = TenantManager::new(db.clone());
    let vs = VectorStore::new(db);

    let t1 = tm.create("alice", 100, None, 4).await.unwrap();
    let t2 = tm.create("bob", 100, None, 4).await.unwrap();

    vs.upload(t1.tenant_id, &[(1, unit(4, 0))]).await.unwrap();
    vs.upload(t2.tenant_id, &[(2, unit(4, 1))]).await.unwrap();

    tm.delete(t1.tenant_id).await.unwrap();

    // t2 still works.
    let r = vs.search(t2.tenant_id, &unit(4, 1), 5).await.unwrap();
    assert_eq!(r[0].id, 2);

    // total vectors reflects only t2.
    assert_eq!(tm.total_vectors().await.unwrap(), 1);
}

// ---------------------------------------------------------------------------
// Capacity statistics
// ---------------------------------------------------------------------------

#[tokio::test]
async fn total_storage_bytes_matches_expected_formula() {
    let tmp = TempDir::new().unwrap();
    let db = StorageDb::open(tmp.path()).unwrap();
    let tm = TenantManager::new(db.clone());
    let vs = VectorStore::new(db);

    // dim=4: storage_bytes per vector = 4*4 + 24 = 40
    let t = tm.create("p", 100, None, 4).await.unwrap();
    vs.upload(t.tenant_id, &[(1, unit(4, 0)), (2, unit(4, 1)), (3, unit(4, 2))])
        .await
        .unwrap();

    let total = tm.total_storage_bytes().await.unwrap();
    assert_eq!(total, 3 * 40);
}
