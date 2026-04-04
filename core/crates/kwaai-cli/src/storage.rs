//! `kwaainet storage` — manage the local storage fabric (Eve role).
//!
//! Provisions a local PostgreSQL+pgvector instance, creates the VPK database,
//! and configures the node to offer encrypted vector storage to the network.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command as Cmd;

use crate::cli::{StorageAction, StorageArgs};
use crate::config::{KwaaiNetConfig, StorageConfig};
use crate::display::*;

pub async fn run(args: StorageArgs) -> Result<()> {
    match args.action {
        StorageAction::Init {
            capacity_gb,
            data_path,
            port,
            endpoint,
            pg_port,
        } => init(capacity_gb, data_path, port, endpoint, pg_port).await,
        StorageAction::Status => status().await,
        StorageAction::Start => start(),
        StorageAction::Stop => stop(),
        StorageAction::Destroy { yes } => destroy(yes),
    }
}

// ---------------------------------------------------------------------------
// init
// ---------------------------------------------------------------------------

async fn init(
    capacity_gb: f64,
    data_path: Option<String>,
    vpk_port: u16,
    endpoint: Option<String>,
    pg_port: u16,
) -> Result<()> {
    print_box_header("Storage Fabric — Init");

    let data_dir = match data_path {
        Some(p) => expand_tilde(&p),
        None => crate::config::storage_dir(),
    };
    let pgdata = data_dir.join("pgdata");

    // 1. Check for PostgreSQL -------------------------------------------------
    println!("  [1/6] Checking PostgreSQL…");
    let (initdb, pg_ctl, createdb, psql) = find_pg_binaries()?;
    let pg_version = pg_version_string(&initdb);
    println!("         {}", pg_version);

    // 2. Check for pgvector extension -----------------------------------------
    println!("  [2/6] Checking pgvector extension…");
    check_pgvector_available(&psql, pg_port)?;

    // 3. Init data directory --------------------------------------------------
    println!("  [3/6] Initializing data directory…");
    std::fs::create_dir_all(&data_dir)
        .with_context(|| format!("creating {}", data_dir.display()))?;

    if pgdata.join("PG_VERSION").exists() {
        println!("         Already initialized at {}", pgdata.display());
    } else {
        run_cmd(
            Cmd::new(&initdb)
                .arg("--pgdata")
                .arg(&pgdata)
                .arg("--auth=trust")
                .arg("--no-locale")
                .arg("--encoding=UTF8"),
            "initdb",
        )?;
        // Configure port + listen address
        let conf_path = pgdata.join("postgresql.conf");
        let mut conf = std::fs::read_to_string(&conf_path).unwrap_or_default();
        conf.push_str(&format!(
            "\n# KwaaiNet storage fabric\nport = {}\nlisten_addresses = 'localhost'\n",
            pg_port
        ));
        std::fs::write(&conf_path, conf)?;
        println!("         Created at {}", pgdata.display());
    }

    // 4. Start PostgreSQL -----------------------------------------------------
    println!("  [4/6] Starting PostgreSQL on port {}…", pg_port);
    let log_file = crate::config::log_dir().join("postgres.log");
    std::fs::create_dir_all(log_file.parent().unwrap())?;

    if pg_is_running(&pg_ctl, &pgdata) {
        println!("         Already running");
    } else {
        run_cmd(
            Cmd::new(&pg_ctl)
                .arg("start")
                .arg("-D")
                .arg(&pgdata)
                .arg("-l")
                .arg(&log_file)
                .arg("-w"), // wait for startup
            "pg_ctl start",
        )?;
        println!("         Started (log: {})", log_file.display());
    }

    // 5. Create database + enable pgvector ------------------------------------
    println!("  [5/6] Creating database…");
    let db_name = "kwaainet_vpk";
    // Check if DB already exists
    let db_exists = Cmd::new(&psql)
        .args([
            "-p",
            &pg_port.to_string(),
            "-h",
            "localhost",
            "-d",
            "postgres",
            "-tAc",
            &format!("SELECT 1 FROM pg_database WHERE datname='{}'", db_name),
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "1")
        .unwrap_or(false);

    if db_exists {
        println!("         Database '{}' already exists", db_name);
    } else {
        run_cmd(
            Cmd::new(&createdb)
                .args(["-p", &pg_port.to_string(), "-h", "localhost", db_name]),
            "createdb",
        )?;
        println!("         Created '{}'", db_name);
    }

    // Enable pgvector extension
    run_cmd(
        Cmd::new(&psql)
            .args([
                "-p",
                &pg_port.to_string(),
                "-h",
                "localhost",
                "-d",
                db_name,
                "-c",
                "CREATE EXTENSION IF NOT EXISTS vector",
            ]),
        "enable pgvector",
    )?;
    println!("         pgvector extension enabled");

    // 6. Save config ----------------------------------------------------------
    println!("  [6/6] Saving configuration…");
    let pg_url = format!(
        "postgresql://localhost:{}/{}",
        pg_port, db_name
    );
    let mut cfg = KwaaiNetConfig::load_or_create()?;
    cfg.vpk_enabled = true;
    cfg.vpk_mode = Some("eve".to_string());
    cfg.vpk_local_port = Some(vpk_port);
    if let Some(ep) = endpoint.clone() {
        cfg.vpk_endpoint = Some(ep);
    }
    cfg.storage = Some(StorageConfig {
        pg_url: pg_url.clone(),
        data_path: data_dir.to_string_lossy().to_string(),
        capacity_gb,
        pg_port,
    });
    cfg.save()?;

    // Summary -----------------------------------------------------------------
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │  Storage Fabric Initialized              │");
    println!("  ├─────────────────────────────────────────┤");
    println!("  │  PostgreSQL: localhost:{}             │", pg_port);
    println!("  │  Database:   {}              │", db_name);
    println!(
        "  │  Data path:  {}",
        truncate_path(&data_dir.to_string_lossy(), 30)
    );
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
        print_warning("Storage not initialized. Run: kwaainet storage init");
        print_separator();
        return Ok(());
    };

    let pgdata = PathBuf::from(&storage.data_path).join("pgdata");
    let (_, pg_ctl, _, psql) = find_pg_binaries()?;

    // PG running?
    let running = pg_is_running(&pg_ctl, &pgdata);
    println!(
        "  PostgreSQL:  {} (port {})",
        if running { "running" } else { "stopped" },
        storage.pg_port
    );
    println!("  Data path:   {}", storage.data_path);
    println!("  Capacity:    {:.1} GB", storage.capacity_gb);
    println!("  PG URL:      {}", storage.pg_url);
    println!();

    if running {
        // Query DB size
        let output = Cmd::new(&psql)
            .args([
                "-p",
                &storage.pg_port.to_string(),
                "-h",
                "localhost",
                "-d",
                "kwaainet_vpk",
                "-tAc",
                "SELECT pg_size_pretty(pg_database_size('kwaainet_vpk'))",
            ])
            .output();
        if let Ok(out) = output {
            let size = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !size.is_empty() {
                println!("  DB size:     {}", size);
            }
        }

        // Count tenant tables (eve_vectors_*)
        let output = Cmd::new(&psql)
            .args([
                "-p",
                &storage.pg_port.to_string(),
                "-h",
                "localhost",
                "-d",
                "kwaainet_vpk",
                "-tAc",
                "SELECT count(*) FROM information_schema.tables WHERE table_name LIKE 'eve_vectors_%'",
            ])
            .output();
        if let Ok(out) = output {
            let count = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !count.is_empty() {
                println!("  Tenants:     {} (vector tables)", count);
            }
        }

        // Check if 'tenants' table exists (PHE migration has run)
        let output = Cmd::new(&psql)
            .args([
                "-p",
                &storage.pg_port.to_string(),
                "-h",
                "localhost",
                "-d",
                "kwaainet_vpk",
                "-tAc",
                "SELECT count(*) FROM information_schema.tables WHERE table_name = 'tenants'",
            ])
            .output();
        if let Ok(out) = output {
            let has_tenants = String::from_utf8_lossy(&out.stdout).trim() == "1";
            if !has_tenants {
                println!();
                print_warning("PHE migrations not yet applied — start VPK to run them automatically");
            }
        }
    } else {
        print_warning("PostgreSQL is not running");
        print_info("Start it: kwaainet storage start");
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
// start / stop
// ---------------------------------------------------------------------------

fn start() -> Result<()> {
    print_box_header("Storage Fabric — Start PostgreSQL");

    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(ref storage) = cfg.storage else {
        print_warning("Storage not initialized. Run: kwaainet storage init");
        print_separator();
        return Ok(());
    };

    let pgdata = PathBuf::from(&storage.data_path).join("pgdata");
    let (_, pg_ctl, _, _) = find_pg_binaries()?;

    if pg_is_running(&pg_ctl, &pgdata) {
        print_success("PostgreSQL is already running");
    } else {
        let log_file = crate::config::log_dir().join("postgres.log");
        run_cmd(
            Cmd::new(&pg_ctl)
                .arg("start")
                .arg("-D")
                .arg(&pgdata)
                .arg("-l")
                .arg(&log_file)
                .arg("-w"),
            "pg_ctl start",
        )?;
        print_success(&format!("PostgreSQL started on port {}", storage.pg_port));
    }

    print_separator();
    Ok(())
}

fn stop() -> Result<()> {
    print_box_header("Storage Fabric — Stop PostgreSQL");

    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(ref storage) = cfg.storage else {
        print_warning("Storage not initialized. Run: kwaainet storage init");
        print_separator();
        return Ok(());
    };

    let pgdata = PathBuf::from(&storage.data_path).join("pgdata");
    let (_, pg_ctl, _, _) = find_pg_binaries()?;

    if !pg_is_running(&pg_ctl, &pgdata) {
        print_success("PostgreSQL is already stopped");
    } else {
        run_cmd(
            Cmd::new(&pg_ctl)
                .arg("stop")
                .arg("-D")
                .arg(&pgdata)
                .arg("-m")
                .arg("fast"),
            "pg_ctl stop",
        )?;
        print_success("PostgreSQL stopped");
    }

    print_separator();
    Ok(())
}

// ---------------------------------------------------------------------------
// destroy
// ---------------------------------------------------------------------------

fn destroy(skip_confirm: bool) -> Result<()> {
    print_box_header("Storage Fabric — Destroy");

    let cfg = KwaaiNetConfig::load_or_create()?;
    let Some(ref storage) = cfg.storage else {
        print_warning("Storage not initialized — nothing to destroy.");
        print_separator();
        return Ok(());
    };

    let data_dir = PathBuf::from(&storage.data_path);
    let pgdata = data_dir.join("pgdata");

    println!("  This will permanently delete:");
    println!("    - PostgreSQL data at {}", pgdata.display());
    println!("    - All tenant data and encrypted vectors");
    println!("    - Storage configuration from config.yaml");
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

    // Stop PG if running
    if let Ok((_, pg_ctl, _, _)) = find_pg_binaries() {
        if pg_is_running(&pg_ctl, &pgdata) {
            let _ = Cmd::new(&pg_ctl)
                .arg("stop")
                .arg("-D")
                .arg(&pgdata)
                .arg("-m")
                .arg("immediate")
                .status();
        }
    }

    // Remove data directory
    if data_dir.exists() {
        std::fs::remove_dir_all(&data_dir)
            .with_context(|| format!("removing {}", data_dir.display()))?;
        print_success(&format!("Removed {}", data_dir.display()));
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

/// Locate PostgreSQL binaries on PATH or common install locations.
fn find_pg_binaries() -> Result<(PathBuf, PathBuf, PathBuf, PathBuf)> {
    let bins = ["initdb", "pg_ctl", "createdb", "psql"];
    let mut found: Vec<PathBuf> = Vec::new();

    for name in &bins {
        if let Some(p) = find_in_path(name) {
            found.push(p);
        } else if let Some(p) = find_in_brew(name) {
            found.push(p);
        } else {
            bail!(
                "'{}' not found. Install PostgreSQL:\n  macOS:  brew install postgresql@17\n  Linux:  sudo apt install postgresql",
                name
            );
        }
    }

    Ok((
        found[0].clone(),
        found[1].clone(),
        found[2].clone(),
        found[3].clone(),
    ))
}

/// Check if the pgvector extension is available in the PostgreSQL installation.
fn check_pgvector_available(psql: &Path, pg_port: u16) -> Result<()> {
    // Strategy 1: Use pg_config --sharedir to find the extension file
    let pg_config = find_in_path("pg_config").or_else(|| find_in_brew("pg_config"));
    if let Some(pg_config_path) = pg_config {
        let ext_output = Cmd::new(&pg_config_path).arg("--sharedir").output();
        if let Ok(out) = ext_output {
            let sharedir = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let vector_control = PathBuf::from(&sharedir)
                .join("extension")
                .join("vector.control");
            if vector_control.exists() {
                println!("         pgvector extension found");
                return Ok(());
            }
        }
    }

    // Strategy 2: If PG is already running, check pg_available_extensions
    let ext_check = Cmd::new(psql)
        .args([
            "-p",
            &pg_port.to_string(),
            "-h",
            "localhost",
            "-d",
            "postgres",
            "-tAc",
            "SELECT 1 FROM pg_available_extensions WHERE name = 'vector'",
        ])
        .output();
    if let Ok(out) = ext_check {
        if String::from_utf8_lossy(&out.stdout).trim() == "1" {
            println!("         pgvector extension found");
            return Ok(());
        }
    }

    // Strategy 3: Scan common Homebrew share directories (pgvector may be
    // installed for a different PG major version than the one on PATH).
    let pg_version = get_pg_major_version(psql);
    let bases = ["/opt/homebrew/share", "/usr/local/share"];
    let pg_versions = ["14", "15", "16", "17", "18"];
    for base in &bases {
        for ver in &pg_versions {
            for fmt in &[format!("postgresql@{}", ver), format!("postgresql{}", ver)] {
                let candidate = PathBuf::from(base).join(fmt).join("extension").join("vector.control");
                if candidate.exists() {
                    // Found pgvector — check if it matches our PG version
                    if let Some(ref our_ver) = pg_version {
                        if our_ver != ver {
                            print_warning(&format!(
                                "pgvector is installed for PostgreSQL {} but you have PostgreSQL {}",
                                ver, our_ver
                            ));
                            print_info(&format!(
                                "Fix: brew install postgresql@{} (upgrade PG) or rebuild pgvector for PG {}",
                                ver, our_ver
                            ));
                            bail!("pgvector/PostgreSQL version mismatch");
                        }
                    }
                    println!("         pgvector extension found");
                    return Ok(());
                }
            }
        }
    }

    // Extension not found — give install instructions
    print_warning("pgvector extension not found");
    print_info("Install it:");
    #[cfg(target_os = "macos")]
    print_info("  brew install pgvector");
    #[cfg(target_os = "linux")]
    print_info("  sudo apt install postgresql-17-pgvector");
    bail!("pgvector extension is required for the storage fabric");
}

/// Get the major version of the PostgreSQL installation.
fn get_pg_major_version(psql: &Path) -> Option<String> {
    let output = Cmd::new(psql).arg("--version").output().ok()?;
    let version_str = String::from_utf8_lossy(&output.stdout);
    // "psql (PostgreSQL) 16.12 (Homebrew)" → "16"
    version_str
        .split_whitespace()
        .find(|s| s.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .and_then(|v| v.split('.').next())
        .map(|s| s.to_string())
}

/// Check if PostgreSQL is running for the given data directory.
fn pg_is_running(pg_ctl: &Path, pgdata: &Path) -> bool {
    Cmd::new(pg_ctl)
        .arg("status")
        .arg("-D")
        .arg(pgdata)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Get PostgreSQL version string.
fn pg_version_string(initdb: &Path) -> String {
    Cmd::new(initdb)
        .arg("--version")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "PostgreSQL (unknown version)".to_string())
}

/// Run a command, returning an error with context on failure.
fn run_cmd(cmd: &mut Cmd, label: &str) -> Result<()> {
    let output = cmd
        .output()
        .with_context(|| format!("failed to run {}", label))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("{} failed: {}", label, stderr.trim());
    }
    Ok(())
}

/// Search PATH for a binary.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

/// Search common Homebrew PostgreSQL locations.
fn find_in_brew(name: &str) -> Option<PathBuf> {
    // Homebrew on Apple Silicon
    for version in &["17", "16", "15", "14"] {
        let candidate = PathBuf::from(format!(
            "/opt/homebrew/opt/postgresql@{}/bin/{}",
            version, name
        ));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Homebrew on Intel Mac
    for version in &["17", "16", "15", "14"] {
        let candidate = PathBuf::from(format!(
            "/usr/local/opt/postgresql@{}/bin/{}",
            version, name
        ));
        if candidate.exists() {
            return Some(candidate);
        }
    }
    // Homebrew unversioned
    let candidate = PathBuf::from(format!("/opt/homebrew/bin/{}", name));
    if candidate.exists() {
        return Some(candidate);
    }
    None
}

/// Expand `~` at the start of a path to the user's home directory.
fn expand_tilde(p: &str) -> PathBuf {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest);
        }
    }
    PathBuf::from(p)
}

/// Truncate a path string for display in the summary box.
fn truncate_path(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}│", s, width = max)
    } else {
        format!("…{:<width$}│", &s[s.len() - max + 1..], width = max - 1)
    }
}
