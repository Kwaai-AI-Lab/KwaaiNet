//! Database connection pool and migrations.

use anyhow::{Context, Result};
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use tracing::info;

/// Shared database connection pool and migration runner.
#[derive(Clone)]
pub struct StorageDb {
    pool: Pool,
}

impl StorageDb {
    /// Connect to PostgreSQL and return a new `StorageDb`.
    pub async fn connect(pg_url: &str) -> Result<Self> {
        let mut cfg = Config::new();
        cfg.url = Some(pg_url.to_string());
        let pool = cfg
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .context("failed to create connection pool")?;

        // Verify the connection works
        let client = pool.get().await.context("cannot connect to PostgreSQL")?;
        client
            .simple_query("SELECT 1")
            .await
            .context("PostgreSQL health check failed")?;

        Ok(Self { pool })
    }

    /// Run all schema migrations (idempotent).
    pub async fn migrate(&self) -> Result<()> {
        let client = self.pool.get().await?;

        client
            .batch_execute(
                r#"
-- pgvector extension (idempotent; requires superuser or CREATE privilege)
CREATE EXTENSION IF NOT EXISTS vector;

-- Tenants table
CREATE TABLE IF NOT EXISTS tenants (
    tenant_id   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    peer_id     VARCHAR(100) NOT NULL,
    display_name VARCHAR(200),
    capacity_limit_mb BIGINT NOT NULL DEFAULT 1024,
    status      VARCHAR(20) NOT NULL DEFAULT 'Active',
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_tenant_status CHECK (status IN ('Active', 'Suspended', 'Deleted'))
);
CREATE INDEX IF NOT EXISTS idx_tenants_peer_id ON tenants(peer_id);
CREATE INDEX IF NOT EXISTS idx_tenants_status ON tenants(status);

-- Audit log (firewall-style: events only, no content)
CREATE TABLE IF NOT EXISTS storage_audit_log (
    audit_id    BIGSERIAL PRIMARY KEY,
    tenant_id   UUID REFERENCES tenants(tenant_id),
    event_type  VARCHAR(50) NOT NULL,
    details     JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_audit_tenant ON storage_audit_log(tenant_id);
CREATE INDEX IF NOT EXISTS idx_audit_created ON storage_audit_log(created_at);
"#,
            )
            .await
            .context("migration failed")?;

        info!("storage migrations applied");
        Ok(())
    }

    /// Get a client from the pool.
    pub async fn client(
        &self,
    ) -> Result<deadpool_postgres::Object> {
        self.pool.get().await.context("pool exhausted")
    }

    /// Return the raw pool for direct use.
    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}
