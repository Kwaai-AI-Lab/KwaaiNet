//! `kwaainet storage` — manage the local storage fabric (Eve role).
//!
//! Uses an embedded hnsw_rs + redb backend — no PostgreSQL or Docker required.

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::cli::{StorageAction, StorageArgs};
use crate::config::{KwaaiNetConfig, StorageConfig};
use crate::display::*;

pub async fn run(args: StorageArgs) -> Result<()> {
    match args.action {
        StorageAction::Init {
            capacity_gb,
            port,
            data_dir,
        } => init(capacity_gb, port, data_dir).await,
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
    capacity_gb: Option<f64>,
    vpk_port: u16,
    data_dir_override: Option<PathBuf>,
) -> Result<()> {
    print_box_header("Storage Fabric — Init");

    // 1. Open (or create) the embedded store -----------------------------------
    println!("  [1/2] Opening embedded vector store…");
    let data_dir = data_dir_override.unwrap_or_else(default_data_dir);

    // Resolve capacity: explicit arg, or 10% of available disk (min 1 GB).
    let auto_capacity = capacity_gb.is_none();
    let capacity_gb = capacity_gb.unwrap_or_else(|| available_disk_10pct(&data_dir));
    if auto_capacity {
        println!("         Capacity: {:.1} GB (10% of available disk — use --capacity-gb to override)", capacity_gb);
    }
    kwaai_storage::StorageDb::open(&data_dir)
        .with_context(|| format!("cannot open embedded store at {}", data_dir.display()))?;
    println!("         Store ready at {}", data_dir.display());

    // 2. Save config -----------------------------------------------------------
    println!("  [2/2] Saving configuration…");
    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.vpk_enabled = true;
    cfg.vpk_mode = Some("eve".to_string());
    cfg.vpk_local_port = Some(vpk_port);
    cfg.storage = Some(StorageConfig {
        data_dir: data_dir.to_string_lossy().into_owned(),
        capacity_gb,
        _legacy_pg_url: None,
        _legacy_data_path: None,
        _legacy_pg_port: None,
    });
    cfg.save()?;

    // Summary -----------------------------------------------------------------
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │  Storage Fabric Initialized              │");
    println!("  ├─────────────────────────────────────────┤");
    println!(
        "  │  Store:      {}│",
        truncate_path(&data_dir.to_string_lossy(), 27)
    );
    println!(
        "  │  Capacity:   {:.1} GB                    │",
        capacity_gb
    );
    println!("  │  Health port: {}                     │", vpk_port);
    println!("  │  Mode:        Eve (storage provider)    │");
    println!("  │  Access:      P2P relay (no port fwd)   │");
    println!("  └─────────────────────────────────────────┘");
    println!();
    print_success("Storage fabric initialized");
    print_info("Remote Bobs connect via /kwaai/storage/1.0.0 using your PeerId.");
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
        print_warning("Storage not initialized. Run: kwaainet storage init");
        print_separator();
        return Ok(());
    };

    let data_dir = PathBuf::from(&storage.data_dir);
    println!("  Store:       {}", data_dir.display());
    println!("  Capacity:    {:.1} GB", storage.capacity_gb);
    println!();

    // Check VPK health first — when storage serve is running it holds an
    // exclusive lock on the redb file, so we get stats from the HTTP endpoint
    // instead of trying to open the db directly (which would fail with a lock
    // error and produce a confusing "not reachable" warning).
    let vpk_reachable = if let Some(vpk_port) = cfg.vpk_local_port {
        let url = format!("http://localhost:{}/api/health", vpk_port);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()?;
        match client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    let api_status = json["status"].as_str().unwrap_or("ok");
                    let tenants = json["tenant_count"].as_u64().unwrap_or(0);
                    let vectors = json["total_vectors"].as_u64().unwrap_or(0);
                    let cap = json["capacity_gb_available"].as_f64().unwrap_or(0.0);
                    print_success(&format!("Storage API: {} (port {})", api_status, vpk_port));
                    println!("  Tenants:     {}", tenants);
                    println!("  Vectors:     {}", vectors);
                    println!("  Available:   {:.1} GB", cap);
                    // Disk usage (read-only metadata, no db lock needed)
                    if let Ok(db_size) =
                        std::fs::metadata(data_dir.join("metadata.redb")).map(|m| m.len())
                    {
                        println!("  DB size:     {:.1} KB", db_size as f64 / 1024.0);
                    }
                }
                true
            }
            _ => false,
        }
    } else {
        false
    };

    // Only try to open the db directly when the API is not running — the db
    // file is exclusively locked when the serve is active.
    if !vpk_reachable {
        match kwaai_storage::StorageDb::open(&data_dir) {
            Ok(db) => {
                use kwaai_storage::TenantManager;
                print_success("Embedded store: reachable (API not running)");

                let tm = TenantManager::new(db);
                let tenant_count = tm.count().await.unwrap_or(0);
                let total_vectors = tm.total_vectors().await.unwrap_or(0);
                println!("  Tenants:     {} (vector tables)", tenant_count);
                println!("  Vectors:     {}", total_vectors);
                if let Ok(db_size) =
                    std::fs::metadata(data_dir.join("metadata.redb")).map(|m| m.len())
                {
                    println!("  DB size:     {:.1} KB", db_size as f64 / 1024.0);
                }
            }
            Err(_) => {
                print_warning("Storage API not running.");
                print_info("Start it with: kwaainet start --daemon");
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
    // Bind to loopback only — remote access goes through /kwaai/storage/1.0.0 P2P relay.
    let bind_addr = format!("127.0.0.1:{}", vpk_port);
    let data_dir = PathBuf::from(&storage.data_dir);
    let capacity_gb = storage.capacity_gb;

    let mgr = crate::daemon::StorageApiManager::new();
    if mgr.is_running() || crate::daemon::port_in_use(vpk_port) {
        print_warning(&format!(
            "Storage API is already running on port {}.",
            vpk_port
        ));
        print_info("Check status: kwaainet storage status");
        print_info("Stop it:      kwaainet storage stop");
        print_separator();
        return Ok(());
    }
    mgr.write_pid(std::process::id());

    print_box_header("Storage Fabric — API Server");
    println!("  Store:    {}", data_dir.display());
    println!("  Bind:     {}", bind_addr);
    println!("  Capacity: {:.1} GB", capacity_gb);
    println!();

    let db = kwaai_storage::StorageDb::open(&data_dir)?;
    print_success("Embedded store opened, indices loaded");

    let peer_id = crate::identity::NodeIdentity::load_or_create()
        .map(|id| id.peer_id.to_base58())
        .unwrap_or_else(|_| "unknown".to_string());

    // Register the p2p relay handler so Eve can be reached without port forwarding.
    // The P2PClient MUST stay alive for the lifetime of this process — p2pd removes
    // the handler registration when the persistent connection closes.
    let handler =
        crate::storage_rpc::make_storage_rpc_handler(db.clone(), capacity_gb, peer_id.clone());
    let daemon_addr = crate::shard_cmd::daemon_socket();
    let _p2p_client = match kwaai_p2p_daemon::P2PClient::connect(&daemon_addr).await {
        Ok(p2p_client) => {
            match p2p_client
                .add_unary_handler(crate::storage_rpc::STORAGE_PROTO, handler, false)
                .await
            {
                Ok(()) => print_success(&format!(
                    "P2P relay handler registered ({})",
                    crate::storage_rpc::STORAGE_PROTO
                )),
                Err(e) => print_warning(&format!("P2P handler registration failed: {e}")),
            }
            Some(p2p_client)
        }
        Err(_) => {
            print_info("KwaaiNet node not running — P2P relay unavailable (HTTP-only mode)");
            print_info("Start the node first for relay access: kwaainet start --daemon");
            None
        }
    };

    print_success(&format!("Starting local health API on {} (operator use only)", bind_addr));
    print_separator();

    kwaai_storage::run_storage_api(db, &bind_addr, capacity_gb, peer_id).await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// start / stop
// ---------------------------------------------------------------------------

fn start() -> Result<()> {
    print_box_header("Storage Fabric — Start");
    let cfg = KwaaiNetConfig::load_or_create()?;
    if cfg.storage.is_none() {
        print_warning("Storage not initialized. Run: kwaainet storage init");
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

    let data_dir = cfg
        .storage
        .as_ref()
        .map(|s| s.data_dir.clone())
        .unwrap_or_default();

    println!("  This will permanently remove:");
    println!("    - Storage configuration from config.yaml");
    println!("    - Embedded store at: {}", data_dir);
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

    // Delete the embedded store directory
    if !data_dir.is_empty() {
        let path = PathBuf::from(&data_dir);
        if path.exists() {
            std::fs::remove_dir_all(&path)
                .with_context(|| format!("removing store at {}", path.display()))?;
            print_success(&format!("Store removed: {}", path.display()));
        }
    }

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

fn default_data_dir() -> PathBuf {
    crate::config::kwaainet_dir().join("storage")
}

/// Return 10% of the available space on the disk that contains `path`,
/// clamped to a minimum of 1 GB. Falls back to 5 GB if disk info is
/// unavailable.
fn available_disk_10pct(path: &std::path::Path) -> f64 {
    use sysinfo::Disks;

    // Ensure the target directory exists so we can match it against mount points.
    let _ = std::fs::create_dir_all(path);
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    let disks = Disks::new_with_refreshed_list();

    // Find the disk whose mount point is the longest prefix of our path
    // (most specific mount wins).
    let best = disks
        .iter()
        .filter(|d| canonical.starts_with(d.mount_point()))
        .max_by_key(|d| d.mount_point().as_os_str().len());

    let available_gb = match best {
        Some(disk) => disk.available_space() as f64 / 1_073_741_824.0,
        None => return 5.0,
    };

    (available_gb * 0.10).max(1.0)
}

fn truncate_path(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}", s, width = max)
    } else {
        format!("…{:<width$}", &s[s.len() - max + 1..], width = max - 1)
    }
}
