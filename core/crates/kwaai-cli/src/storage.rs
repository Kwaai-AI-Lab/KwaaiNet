//! `kwaainet storage` — manage the local storage fabric (Eve role).
//!
//! Validates an operator-supplied PostgreSQL DSN, enables pgvector, runs schema
//! migrations, and configures the node to offer encrypted vector storage to the network.

use anyhow::{Context, Result};

use crate::cli::{StorageAction, StorageArgs};
use crate::config::{KwaaiNetConfig, StorageConfig};
use crate::display::*;

pub async fn run(args: StorageArgs) -> Result<()> {
    match args.action {
        StorageAction::Init {
            pg_url,
            capacity_gb,
            port,
            endpoint,
        } => init(pg_url, capacity_gb, port, endpoint).await,
        StorageAction::Status => status().await,
        StorageAction::Serve => serve().await,
        StorageAction::Start => start(),
        StorageAction::Stop => stop(),
        StorageAction::Destroy { yes } => destroy(yes),
    }
}

// ---------------------------------------------------------------------------
// init
// ---------------------------------------------------------------------------

async fn init(
    pg_url: String,
    capacity_gb: f64,
    vpk_port: u16,
    endpoint: Option<String>,
) -> Result<()> {
    print_box_header("Storage Fabric — Init");

    // 1. Validate connection --------------------------------------------------
    println!("  [1/3] Validating PostgreSQL connection…");
    let db = kwaai_storage::StorageDb::connect(&pg_url)
        .await
        .context("Cannot connect to PostgreSQL — check the DSN and ensure the server is running")?;
    println!("         Connected");

    // 2. Enable pgvector + run migrations ------------------------------------
    println!("  [2/3] Enabling pgvector and applying schema…");
    db.migrate()
        .await
        .context("Schema migration failed — ensure the connecting user has CREATE EXTENSION privilege")?;
    println!("         pgvector enabled, schema ready");

    // 3. Save config ----------------------------------------------------------
    println!("  [3/3] Saving configuration…");
    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.vpk_enabled = true;
    cfg.vpk_mode = Some("eve".to_string());
    cfg.vpk_local_port = Some(vpk_port);
    if let Some(ep) = endpoint.clone() {
        cfg.vpk_endpoint = Some(ep);
    }
    cfg.storage = Some(StorageConfig {
        pg_url: pg_url.clone(),
        capacity_gb,
        _legacy_data_path: None,
        _legacy_pg_port: None,
    });
    cfg.save()?;

    // Summary -----------------------------------------------------------------
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │  Storage Fabric Initialized              │");
    println!("  ├─────────────────────────────────────────┤");
    println!("  │  DSN:        {}", truncate_path(&pg_url, 27));
    println!("  │  Capacity:   {:.1} GB                    │", capacity_gb);
    println!("  │  VPK port:   {}                      │", vpk_port);
    println!("  │  Mode:       Eve (storage provider)     │");
    if let Some(ref ep) = endpoint {
        println!("  │  Endpoint:   {}", truncate_path(ep, 28));
    }
    println!("  └─────────────────────────────────────────┘");
    println!();
    print_success("Storage fabric initialized");
    print_info("Start the node: kwaainet start --daemon");
    print_info("Check status:   kwaainet storage status");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------

async fn status() -> Result<()> {
    print_box_header("Storage Fabric — Status");

    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(ref storage) = cfg.storage else {
        print_warning("Storage not initialized. Run: kwaainet storage init --pg-url <DSN>");
        print_separator();
        return Ok(());
    };

    println!("  PG URL:      {}", storage.pg_url);
    println!("  Capacity:    {:.1} GB", storage.capacity_gb);
    println!();

    // DB connectivity + schema check -----------------------------------------
    match kwaai_storage::StorageDb::connect(&storage.pg_url).await {
        Ok(db) => {
            print_success("PostgreSQL: reachable");

            let client = db.client().await?;

            if let Ok(rows) = client
                .query(
                    "SELECT pg_size_pretty(pg_database_size(current_database()))",
                    &[],
                )
                .await
            {
                if let Some(row) = rows.first() {
                    let size: &str = row.get(0);
                    println!("  DB size:     {}", size);
                }
            }

            if let Ok(rows) = client
                .query(
                    "SELECT count(*) FROM information_schema.tables \
                     WHERE table_name LIKE 'eve_vectors_%'",
                    &[],
                )
                .await
            {
                if let Some(row) = rows.first() {
                    let count: i64 = row.get(0);
                    println!("  Tenants:     {} (vector tables)", count);
                }
            }

            if let Ok(rows) = client
                .query(
                    "SELECT count(*) FROM information_schema.tables \
                     WHERE table_name = 'tenants'",
                    &[],
                )
                .await
            {
                let has_tenants = rows
                    .first()
                    .map(|r| {
                        let n: i64 = r.get(0);
                        n == 1
                    })
                    .unwrap_or(false);
                if !has_tenants {
                    println!();
                    print_warning("Storage schema not yet applied — start VPK to run migrations automatically");
                }
            }
        }
        Err(e) => {
            print_warning(&format!("PostgreSQL: not reachable — {}", e));
        }
    }

    // VPK health (if port configured)
    if let Some(vpk_port) = cfg.vpk_local_port {
        println!();
        let url = format!("http://localhost:{}/api/health", vpk_port);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()?;
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let status = json["status"].as_str().unwrap_or("ok");
                    let tenants = json["tenant_count"].as_u64().unwrap_or(0);
                    let vectors = json["total_vectors"].as_u64().unwrap_or(0);
                    let cap = json["capacity_gb_available"].as_f64().unwrap_or(0.0);
                    println!("  VPK status:  {} (port {})", status, vpk_port);
                    println!("  Tenants:     {}", tenants);
                    println!("  Vectors:     {}", vectors);
                    println!("  Available:   {:.1} GB", cap);
                }
            }
            _ => {
                println!("  VPK:         not reachable (port {})", vpk_port);
                print_info("VPK starts automatically with: kwaainet start --daemon");
            }
        }
    }

    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// serve — run the storage API in the foreground
// ---------------------------------------------------------------------------

async fn serve() -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(ref storage) = cfg.storage else {
        print_warning("Storage not initialized. Run: kwaainet storage init");
        return Ok(());
    };

    let vpk_port = cfg.vpk_local_port.unwrap_or(7432);
    let bind_addr = format!("0.0.0.0:{}", vpk_port);

    print_box_header("Storage Fabric — API Server");
    println!("  PG URL:   {}", storage.pg_url);
    println!("  Bind:     {}", bind_addr);
    println!("  Capacity: {:.1} GB", storage.capacity_gb);
    println!();

    // Connect to PG and run migrations
    let db = kwaai_storage::StorageDb::connect(&storage.pg_url).await?;
    db.migrate().await?;
    print_success("Database connected, migrations applied");

    // Get peer ID for health endpoint
    let peer_id = crate::identity::NodeIdentity::load_or_create()
        .map(|id| id.peer_id.to_base58())
        .unwrap_or_else(|_| "unknown".to_string());

    print_success(&format!("Starting API on {}", bind_addr));
    print_separator();

    kwaai_storage::run_storage_api(db, &bind_addr, storage.capacity_gb, peer_id).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// start / stop
// ---------------------------------------------------------------------------

fn start() -> Result<()> {
    print_box_header("Storage Fabric — Start");

    let cfg = KwaaiNetConfig::load_or_create()?;
    if cfg.storage.is_none() {
        print_warning("Storage not initialized. Run: kwaainet storage init --pg-url <DSN>");
        print_separator();
        return Ok(());
    }

    print_info("Use 'kwaainet storage serve' to run the API server in the foreground.");
    print_info("Use 'kwaainet start --daemon' to start all services including the storage API.");
    print_separator();
    Ok(())
}

fn stop() -> Result<()> {
    print_box_header("Storage Fabric — Stop");

    let cfg = KwaaiNetConfig::load_or_create()?;
    if cfg.storage.is_none() {
        print_warning("Storage not initialized — nothing to stop.");
        print_separator();
        return Ok(());
    }

    print_info("Use 'kwaainet stop' to stop all services including the storage API.");
    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// destroy
// ---------------------------------------------------------------------------

fn destroy(skip_confirm: bool) -> Result<()> {
    print_box_header("Storage Fabric — Destroy");

    let cfg = KwaaiNetConfig::load_or_create()?;
    if cfg.storage.is_none() {
        print_warning("Storage not initialized — nothing to destroy.");
        print_separator();
        return Ok(());
    }

    println!("  This will permanently remove:");
    println!("    - Storage configuration from config.yaml");
    println!("  (Your PostgreSQL data is untouched)");
    println!();

    if !skip_confirm {
        print!("  Type 'yes' to confirm: ");
        use std::io::Write;
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() != "yes" {
            print_info("Aborted.");
            print_separator();
            return Ok(());
        }
    }

    // Clear storage config
    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.storage = None;
    cfg.vpk_enabled = false;
    cfg.vpk_mode = None;
    cfg.save()?;
    print_success("Storage configuration cleared");

    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Truncate a path string for display in the summary box.
fn truncate_path(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}│", s, width = max)
    } else {
        format!("…{:<width$}│", &s[s.len() - max + 1..], width = max - 1)
    }
}
