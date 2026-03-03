//! Postgres queries

use anyhow::{Context, Result};
use deadpool_postgres::{Config as PoolConfig, Pool, Runtime};
use tokio_postgres::NoTls;
use uuid::Uuid;

pub async fn build_pool(database_url: &str) -> Result<Pool> {
    let mut cfg = PoolConfig::new();
    cfg.url = Some(database_url.to_string());
    cfg.pool = Some(deadpool_postgres::PoolConfig::new(10));
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("Failed to create Postgres pool")?;
    Ok(pool)
}

pub async fn migrate(pool: &Pool) -> Result<()> {
    let sql = include_str!("../migrations/001_initial.sql");
    let client = pool
        .get()
        .await
        .context("DB pool exhausted during migration")?;
    client
        .batch_execute(sql)
        .await
        .context("Migration failed")?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Passkey credentials
// ---------------------------------------------------------------------------

pub async fn insert_passkey_credential(
    pool: &Pool,
    user_id: Uuid,
    credential_id: &[u8],
    did_key: &str,
    display_name: &str,
    passkey_json: &str,
) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO passkey_credentials \
             (user_id, credential_id, did_key, display_name, passkey_json) \
             VALUES ($1, $2, $3, $4, $5) \
             ON CONFLICT (credential_id) DO NOTHING",
            &[
                &user_id,
                &credential_id,
                &did_key,
                &display_name,
                &passkey_json,
            ],
        )
        .await
        .context("insert_passkey_credential")?;
    Ok(())
}

/// Returns `(user_id, did_key, passkey_json)` for all credentials matching the given ids.
#[allow(dead_code)]
pub async fn get_passkeys_by_credential_ids(
    pool: &Pool,
    cred_ids: &[Vec<u8>],
) -> Result<Vec<(Uuid, String, String)>> {
    if cred_ids.is_empty() {
        return Ok(vec![]);
    }
    let client = pool.get().await?;
    // Use ANY($1) with a bytea array
    let rows = client
        .query(
            "SELECT user_id, did_key, passkey_json \
             FROM passkey_credentials WHERE credential_id = ANY($1)",
            &[&cred_ids],
        )
        .await
        .context("get_passkeys_by_credential_ids")?;
    Ok(rows
        .iter()
        .map(|r| (r.get(0), r.get(1), r.get(2)))
        .collect())
}

pub async fn get_passkeys_for_did(pool: &Pool, did_key: &str) -> Result<Vec<(Uuid, String)>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT user_id, passkey_json FROM passkey_credentials WHERE did_key = $1",
            &[&did_key],
        )
        .await?;
    Ok(rows.iter().map(|r| (r.get(0), r.get(1))).collect())
}

// ---------------------------------------------------------------------------
// Pending challenges
// ---------------------------------------------------------------------------

pub async fn insert_pending_registration(
    pool: &Pool,
    challenge_id: Uuid,
    user_id: Uuid,
    display_name: &str,
    state_json: &str,
) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO pending_registrations \
             (challenge_id, user_id, display_name, state_json) VALUES ($1, $2, $3, $4)",
            &[&challenge_id, &user_id, &display_name, &state_json],
        )
        .await?;
    Ok(())
}

pub async fn take_pending_registration(
    pool: &Pool,
    challenge_id: Uuid,
) -> Result<Option<(Uuid, String, String)>> {
    let client = pool.get().await?;
    let opt = client
        .query_opt(
            "DELETE FROM pending_registrations \
             WHERE challenge_id = $1 AND expires_at > NOW() \
             RETURNING user_id, display_name, state_json",
            &[&challenge_id],
        )
        .await?;
    Ok(opt.map(|r| (r.get(0), r.get(1), r.get(2))))
}

pub async fn insert_pending_authentication(
    pool: &Pool,
    challenge_id: Uuid,
    state_json: &str,
) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO pending_authentications (challenge_id, state_json) VALUES ($1, $2)",
            &[&challenge_id, &state_json],
        )
        .await?;
    Ok(())
}

pub async fn take_pending_authentication(
    pool: &Pool,
    challenge_id: Uuid,
) -> Result<Option<String>> {
    let client = pool.get().await?;
    let opt = client
        .query_opt(
            "DELETE FROM pending_authentications \
             WHERE challenge_id = $1 AND expires_at > NOW() \
             RETURNING state_json",
            &[&challenge_id],
        )
        .await?;
    Ok(opt.map(|r| r.get(0)))
}

// ---------------------------------------------------------------------------
// Issued VCs
// ---------------------------------------------------------------------------

pub async fn insert_vc(pool: &Pool, subject_did: &str, vc_type: &str, vc_json: &str) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO issued_vcs (subject_did, vc_type, vc_json) VALUES ($1, $2, $3)",
            &[&subject_did, &vc_type, &vc_json],
        )
        .await?;
    Ok(())
}

pub async fn get_vcs_for_subject(pool: &Pool, subject_did: &str) -> Result<Vec<String>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT vc_json FROM issued_vcs WHERE subject_did = $1 ORDER BY issued_at DESC",
            &[&subject_did],
        )
        .await?;
    Ok(rows.iter().map(|r| r.get::<_, String>(0)).collect())
}

// ---------------------------------------------------------------------------
// Node bindings
// ---------------------------------------------------------------------------

pub async fn insert_node_binding(
    pool: &Pool,
    passkey_did: &str,
    node_did: &str,
    binding_vc_json: &str,
) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO node_bindings (passkey_did, node_did, binding_vc_json) \
             VALUES ($1, $2, $3) ON CONFLICT (passkey_did, node_did) DO NOTHING",
            &[&passkey_did, &node_did, &binding_vc_json],
        )
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Sessions
// ---------------------------------------------------------------------------

pub async fn insert_session(
    pool: &Pool,
    token: Uuid,
    user_id: Uuid,
    passkey_did: &str,
) -> Result<()> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO sessions (token, user_id, passkey_did) VALUES ($1, $2, $3)",
            &[&token, &user_id, &passkey_did],
        )
        .await?;
    Ok(())
}

/// Returns `(user_id, passkey_did)` if the session token is valid and not expired.
pub async fn get_session(pool: &Pool, token: Uuid) -> Result<Option<(Uuid, String)>> {
    let client = pool.get().await?;
    let opt = client
        .query_opt(
            "SELECT user_id, passkey_did FROM sessions \
             WHERE token = $1 AND expires_at > NOW()",
            &[&token],
        )
        .await?;
    Ok(opt.map(|r| (r.get(0), r.get(1))))
}
