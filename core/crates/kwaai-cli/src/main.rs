//! kwaainet – KwaaiNet node CLI

mod api;
mod block_rpc;
mod calibration;
mod cli;
mod config;
mod daemon;
mod display;
mod health;
mod hf;
mod identity;
mod llama_local;
mod map;
mod monitor;
mod node;
mod ollama;
mod progress;
mod rebalancer;
mod reputation;
mod reputation_cmd;
mod service;
mod setup;
mod shard_api;
mod shard_cmd;
#[cfg(feature = "storage")]
mod storage;
#[cfg(feature = "storage")]
mod storage_rpc;
mod throughput;
mod uninstall;
mod updater;
mod vpk;
mod vpk_bench;

use anyhow::{Context as _, Result};
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Command, MonitorAction, ServeArgs, ServiceAction};
use config::KwaaiNetConfig;
use daemon::{DaemonManager, ShardManager, StorageApiManager};
use display::*;
use kwaai_inference::{EngineConfig, InferenceEngine, InferenceProvider, ModelFormat};

/// Add the binary's directory to the library search path so bundled CUDA
/// runtime libraries (.so/.dll) are found by cudarc's dynamic loading.
fn setup_cuda_library_path() {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return,
    };
    let Some(_dir) = exe.parent() else { return };

    #[cfg(target_os = "linux")]
    {
        let dir_str = _dir.to_string_lossy();
        let key = "LD_LIBRARY_PATH";
        match std::env::var(key) {
            Ok(existing) if existing.contains(&*dir_str) => {}
            Ok(existing) => {
                std::env::set_var(key, format!("{dir_str}:{existing}"));
            }
            Err(_) => {
                std::env::set_var(key, &*dir_str);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let dir_str = _dir.to_string_lossy();
        let key = "PATH";
        if let Ok(existing) = std::env::var(key) {
            if !existing.contains(&*dir_str) {
                std::env::set_var(key, format!("{dir_str};{existing}"));
            }
        }
    }
}

/// Returns true when the configured model weights are already on disk.
/// Checks both HuggingFace snapshot cache and Ollama blob store.
fn model_is_locally_available(model: &str) -> bool {
    crate::hf::resolve_snapshot(model).is_ok() || crate::ollama::resolve_model_blob(model).is_ok()
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_cuda_library_path();
    let cli = Cli::parse();

    // Initialise logging (RUST_LOG overrides, default info)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Spawn a background update check that runs concurrently with the command.
    // Uses a 24-hour on-disk cache so it only hits the network once per day.
    // Skipped for `update` (redundant) and `run-node` (internal daemon process).
    let skip_update_hint = matches!(cli.command, Command::Update(_) | Command::RunNode);
    let update_task = (!skip_update_hint)
        .then(|| tokio::spawn(async { updater::UpdateChecker::new().check(false).await }));

    match cli.command {
        // -------------------------------------------------------------------
        // Internal: run the native node (used in daemon mode)
        // -------------------------------------------------------------------
        Command::RunNode => {
            let cfg = KwaaiNetConfig::load_or_create()?;
            node::run_node(&cfg).await?;
        }

        // -------------------------------------------------------------------
        // start
        // -------------------------------------------------------------------
        Command::Start(args) => {
            let mut cfg = KwaaiNetConfig::load_or_create()?;

            // Track whether the user explicitly chose a model on the CLI.
            let explicit_model = args.model.is_some();

            // Apply CLI overrides to config
            if let Some(m) = args.model {
                cfg.model = m;
            }
            if let Some(b) = args.blocks {
                cfg.blocks = b;
            }
            if let Some(p) = args.port {
                cfg.port = p;
            }
            if args.no_gpu {
                cfg.use_gpu = false;
            }
            if let Some(n) = args.public_name {
                cfg.public_name = Some(n);
            }
            if let Some(ip) = args.public_ip {
                cfg.public_ip = Some(ip);
            }
            if let Some(a) = args.announce_addr {
                cfg.announce_addr = Some(a);
            }
            if args.no_relay {
                cfg.no_relay = true;
            }

            // ── Read the network map and select the best locally-available model ──
            if !explicit_model {
                print_box_header("🗺  Reading Network Map");
                let local_models = ollama::list_local_models();
                if local_models.is_empty() {
                    print_warning("No local Ollama models found — using configured model");
                } else {
                    println!("  Local models ({}):", local_models.len());
                    for m in &local_models {
                        println!("    • {}", m);
                    }
                    println!();

                    match map::fetch_map(&cfg.health_monitoring.api_endpoint).await {
                        Ok(map_state) => {
                            println!(
                                "  Network map ({} model(s)):",
                                map_state.model_reports.len()
                            );
                            for r in &map_state.model_reports {
                                let avail = local_models.iter().any(|l| map::match_score(l, r) > 0);
                                println!(
                                    "    {:42}  {:2} server(s)  {}",
                                    r.short_name,
                                    r.server_count(),
                                    if avail {
                                        "✓ have locally"
                                    } else {
                                        "✗ not installed"
                                    },
                                );
                            }
                            println!();

                            match map::pick_best_model(&local_models, &map_state, &cfg.model) {
                                Some(choice) => {
                                    // Only switch the model name when the current one is an
                                    // Ollama short ref (no '/').  If the user has a HuggingFace
                                    // repo path (contains '/') such as
                                    // `unsloth/Llama-3.1-8B-Instruct`, keep it — the Ollama map
                                    // selection must not clobber an explicit HF model.
                                    let is_hf_model = cfg.model.contains('/');
                                    if !is_hf_model && choice.ollama_ref != cfg.model {
                                        print_info(&format!(
                                            "Switching model  {}  →  {}",
                                            cfg.model, choice.ollama_ref
                                        ));
                                        cfg.model = choice.ollama_ref;
                                    } else {
                                        print_success(&format!("Confirmed model: {}", cfg.model));
                                    }
                                    if let Some(ref mn) = choice.map_name {
                                        println!(
                                            "    Map entry:  {}  ({} server(s))",
                                            mn, choice.server_count
                                        );
                                    }
                                    // Only adopt the map's DHT prefix and repository when the
                                    // map selection actually drove the model choice (Ollama ref).
                                    // For HF models the user configured explicitly, the fallback
                                    // derivation in node.rs is correct and we must not overwrite
                                    // it with metadata for a different (Ollama-matched) model.
                                    if !is_hf_model {
                                        if let Some(ref dp) = choice.dht_prefix {
                                            println!("    DHT prefix: {}", dp);
                                            cfg.model_dht_prefix = Some(dp.clone());
                                        }
                                        if let Some(ref repo) = choice.repository {
                                            cfg.model_repository = Some(repo.clone());
                                        }
                                    } else {
                                        // Clear any stale map-derived values from a previous model.
                                        cfg.model_dht_prefix = None;
                                        cfg.model_repository = None;
                                    }
                                    // Persist so the daemon child picks it up.
                                    let _ = cfg.save();
                                }
                                None => {
                                    print_info(&format!(
                                        "No local model matched the map — using: {}",
                                        cfg.model
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            print_warning(&format!(
                                "Could not reach network map ({e}) — using: {}",
                                cfg.model
                            ));
                        }
                    }
                }
                print_separator();
            }

            // Auto-detect public IP if neither announce_addr nor public_ip is set
            if cfg.announce_addr.is_none() && cfg.public_ip.is_none() {
                if let Some(ip) = config::detect_public_ip().await {
                    cfg.public_ip = Some(ip);
                }
            }

            let mgr = DaemonManager::new();

            if mgr.is_running() && !args.concurrent {
                print_warning("A KwaaiNet node is already running. Use --concurrent to allow multiple instances.");
                print_info("Stop the existing node with: kwaainet stop");
                std::process::exit(1);
            }

            if !mgr.try_acquire_lock()? {
                print_error("Another instance is starting. Try again shortly.");
                std::process::exit(1);
            }

            print_box_header("🚀 Starting KwaaiNet Node");
            println!("  Model:   {}", cfg.model);
            println!("  Blocks:  {}", cfg.blocks);
            println!("  Port:    {}", cfg.port);
            println!("  Peers:   {}", cfg.initial_peers.len());
            if let Some(ref name) = cfg.public_name {
                println!("  Name:    {}", name);
            }
            print_separator();

            if args.daemon {
                let child_pid = DaemonManager::spawn_daemon_child(&[])?;
                println!();
                print_success(&format!("KwaaiNet daemon started (PID {})", child_pid));

                // Wait for p2pd socket to be ready before spawning children.
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;

                let policy = cfg.contribute_policy(args.no_contribute);

                // --- Shard serving ---
                let shard_available = args.shard || model_is_locally_available(&cfg.model);
                if policy.shards && shard_available {
                    match ShardManager::spawn_shard_child() {
                        Ok(shard_pid) => {
                            ShardManager::new().write_pid(shard_pid);
                            print_success(&format!("Shard serving started  (PID {})", shard_pid));
                            print_info("Shard logs:   kwaainet logs --shard");
                        }
                        Err(e) => print_warning(&format!("Could not start shard serving: {e}")),
                    }
                } else if policy.shards && !shard_available {
                    print_info(&format!(
                        "No local model found for '{}' — skipping shard serving.",
                        cfg.model
                    ));
                    print_info("Download: kwaainet shard download");
                }

                // --- Storage serving ---
                #[cfg(feature = "storage")]
                if policy.storage && cfg.storage.is_some() {
                    match StorageApiManager::spawn_storage_child() {
                        Ok(storage_pid) => {
                            print_success(&format!(
                                "Storage serving started (PID {})",
                                storage_pid
                            ));
                            print_info("Storage logs: kwaainet logs --storage");
                        }
                        Err(e) => print_warning(&format!("Could not start storage serving: {e}")),
                    }
                } else if policy.storage {
                    print_info("Storage not initialised — skipping. Run: kwaainet storage init");
                }

                if args.no_contribute {
                    print_info("Contribution disabled (--no-contribute). Re-enable: kwaainet config set contribute.shards true");
                }

                print_info("Check status: kwaainet status");
                print_info("View logs:    kwaainet logs");
                print_info("Stop daemon:  kwaainet stop");
                print_separator();
            } else {
                // Foreground – run until Ctrl-C
                node::run_node(&cfg).await?;
            }
        }

        // -------------------------------------------------------------------
        // stop
        // -------------------------------------------------------------------
        Command::Stop => {
            let mgr = DaemonManager::new();
            print_box_header("🛑 Stopping KwaaiNet Node");
            // Stop dependents first before the main daemon
            let shard_mgr = ShardManager::new();
            if shard_mgr.is_running() {
                shard_mgr.stop_process();
                print_success("Shard server stopped");
            }
            let storage_mgr = StorageApiManager::new();
            if storage_mgr.is_running() {
                storage_mgr.stop_process();
                print_success("Storage API stopped");
            }
            mgr.stop_process()?;
            print_success("KwaaiNet daemon stopped");
            print_separator();
        }

        // -------------------------------------------------------------------
        // restart
        // -------------------------------------------------------------------
        Command::Restart => {
            let mgr = DaemonManager::new();
            print_box_header("🔄 Restarting KwaaiNet Node");

            // Stop dependents first (same order as Stop) so the new daemon
            // can spawn fresh children without hitting port-in-use guards.
            let shard_mgr = ShardManager::new();
            if shard_mgr.is_running() {
                shard_mgr.stop_process();
            }
            let storage_mgr = StorageApiManager::new();
            if storage_mgr.is_running() {
                storage_mgr.stop_process();
            }
            if mgr.is_running() {
                mgr.stop_process()?;
            }

            // Kill any orphaned p2pd processes unconditionally — even if the
            // kwaainet daemon was already dead, p2pd may still be running and
            // would prevent the new daemon from binding the socket.
            daemon::kill_orphaned_p2pd();

            let child_pid = DaemonManager::spawn_daemon_child(&[])?;
            print_success(&format!("KwaaiNet daemon restarted (PID {})", child_pid));

            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            let restart_cfg = KwaaiNetConfig::load_or_create().unwrap_or_default();
            let policy = restart_cfg.contribute_policy(false);

            if policy.shards && model_is_locally_available(&restart_cfg.model) {
                match ShardManager::spawn_shard_child() {
                    Ok(pid) => {
                        ShardManager::new().write_pid(pid);
                        print_success(&format!("Shard serving restarted (PID {})", pid));
                    }
                    Err(e) => print_warning(&format!("Could not restart shard serving: {e}")),
                }
            }

            #[cfg(feature = "storage")]
            if policy.storage && restart_cfg.storage.is_some() {
                match StorageApiManager::spawn_storage_child() {
                    Ok(pid) => print_success(&format!("Storage serving restarted (PID {})", pid)),
                    Err(e) => print_warning(&format!("Could not restart storage serving: {e}")),
                }
            }
            print_separator();
        }

        // -------------------------------------------------------------------
        // status
        // -------------------------------------------------------------------
        Command::Status(args) => {
            let mgr = DaemonManager::new();
            let status = mgr.get_status();
            let shard_mgr = ShardManager::new();
            let shard_running = shard_mgr.is_running();
            let shard_pid = shard_mgr.read_pid();

            if args.json {
                #[derive(serde::Serialize)]
                struct StatusJson {
                    running: bool,
                    pid: Option<u32>,
                    uptime_secs: Option<u64>,
                    cpu_percent: Option<f32>,
                    memory_mb: Option<f64>,
                    shard_running: bool,
                    shard_pid: Option<u32>,
                }
                let out = StatusJson {
                    running: status.running,
                    pid: status.pid,
                    uptime_secs: status.uptime_secs,
                    cpu_percent: status.cpu_percent,
                    memory_mb: status.memory_mb,
                    shard_running,
                    shard_pid,
                };
                println!("{}", serde_json::to_string(&out).unwrap_or_default());
            } else {
                print_box_header("📊 KwaaiNet Node Status");

                if status.running {
                    let pid = status.pid.unwrap_or(0);
                    let uptime = status
                        .uptime_secs
                        .map(format_uptime)
                        .unwrap_or_else(|| "unknown".to_string());
                    let cpu = status
                        .cpu_percent
                        .map(|c| format!("{:.1}%", c))
                        .unwrap_or_else(|| "n/a".to_string());
                    let mem = status
                        .memory_mb
                        .map(|m| format!("{:.0} MB", m))
                        .unwrap_or_else(|| "n/a".to_string());

                    println!("  🟢 Status:  Running");
                    println!("  🔢 PID:     {}", pid);
                    println!("  ⏱️  Uptime:  {}", uptime);
                    println!("  💻 CPU:     {}", cpu);
                    println!("  🧠 Memory:  {}", mem);
                } else {
                    println!("  🔴 Status:  Not running");
                    print_info("Start with: kwaainet start --daemon");
                }

                println!();
                if shard_running {
                    println!("  🟢 Shard:   Running (PID {})", shard_pid.unwrap_or(0));
                } else {
                    println!("  ⚫ Shard:   Not running");
                    print_info("Start: kwaainet start --daemon");
                }

                // Show storage API status (only when storage is configured)
                let storage_mgr = StorageApiManager::new();
                let cfg = KwaaiNetConfig::load_or_create().unwrap_or_default();
                if cfg.storage.is_some() {
                    println!();
                    if storage_mgr.is_running() {
                        let storage_pid = storage_mgr.read_pid().unwrap_or(0);
                        let port = cfg.vpk_local_port.unwrap_or(7432);
                        println!("  🟢 Storage: Running (PID {}, port {})", storage_pid, port);
                    } else {
                        println!("  🔴 Storage: Not running");
                        print_info("Restart node: kwaainet stop && kwaainet start --daemon");
                    }
                }

                print_separator();
            }
        }

        // -------------------------------------------------------------------
        // logs
        // -------------------------------------------------------------------
        Command::Logs(args) => {
            let log_path = if args.shard {
                config::log_dir().join("shard.log")
            } else if args.storage {
                config::log_dir().join("storage_serve.log")
            } else {
                config::log_file()
            };

            if !log_path.exists() {
                print_warning("No log file found. Start the node first: kwaainet start --daemon");
                return Ok(());
            }

            if args.follow {
                // Tail -f style
                let mut pos = {
                    let meta = std::fs::metadata(&log_path)?;
                    meta.len()
                };
                // Print last N lines first
                print_last_lines(&log_path, args.lines);
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                    let meta = std::fs::metadata(&log_path)?;
                    if meta.len() > pos {
                        let mut file = std::fs::File::open(&log_path)?;
                        use std::io::{Read, Seek, SeekFrom};
                        file.seek(SeekFrom::Start(pos))?;
                        let mut buf = String::new();
                        file.read_to_string(&mut buf)?;
                        print!("{}", buf);
                        pos = meta.len();
                    }
                }
            } else {
                print_last_lines(&log_path, args.lines);
            }
        }

        // -------------------------------------------------------------------
        // config
        // -------------------------------------------------------------------
        Command::Config(args) => {
            use cli::ConfigAction;
            let mut cfg = KwaaiNetConfig::load_or_create()?;

            match args.action {
                None | Some(ConfigAction::Show) => {
                    if args.json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&cfg).unwrap_or_else(|_| "{}".to_string())
                        );
                    } else {
                        print_box_header("⚙️  KwaaiNet Configuration");
                        println!("  🤖 model:        {}", cfg.model);
                        println!("  🧱 blocks:       {}", cfg.blocks);
                        println!("  🔌 port:         {}", cfg.port);
                        println!("  🖥️  use_gpu:      {}", cfg.use_gpu);
                        println!("  📋 log_level:    {}", cfg.log_level);
                        if let Some(ref n) = cfg.public_name {
                            println!("  📋 public_name:  {}", n);
                        }
                        if let Some(ref ip) = cfg.public_ip {
                            println!("  📋 public_ip:    {}", ip);
                        }
                        if cfg.vpk_enabled {
                            println!();
                            println!("  🔐 vpk_enabled:  true");
                            println!(
                                "  🔐 vpk_mode:     {}",
                                cfg.vpk_mode.as_deref().unwrap_or("(not set)")
                            );
                            println!(
                                "  🔐 vpk_port:     {} (local health check)",
                                cfg.vpk_local_port.unwrap_or(7432)
                            );
                            println!("  🔐 vpk_access:   P2P relay via /kwaai/storage/1.0.0");
                        }
                        print_separator();
                    }
                }
                Some(ConfigAction::Set { key, value }) => {
                    cfg.set_key(&key, &value)?;
                    print_box_header("⚙️  Configuration Updated");
                    print_success(&format!("Set {} = {}", key, value));
                    print_separator();
                }
            }
        }

        // -------------------------------------------------------------------
        // health-*
        // -------------------------------------------------------------------
        Command::HealthStatus => {
            let mgr = DaemonManager::new();
            let status = mgr.read_status().unwrap_or_default();
            print_box_header("📊 Health Monitoring Status");
            if let Some(h) = status.health_monitoring {
                println!("{}", serde_json::to_string_pretty(&h).unwrap_or_default());
            } else {
                println!("  Health monitoring data not available.");
                print_info("Start the node first: kwaainet start --daemon");
            }
            print_separator();
        }

        Command::HealthEnable => {
            let mut cfg = KwaaiNetConfig::load_or_create()?;
            cfg.health_monitoring.enabled = true;
            cfg.save()?;
            print_success("Health monitoring enabled. Restart the node to apply: kwaainet restart");
        }

        Command::HealthDisable => {
            let mut cfg = KwaaiNetConfig::load_or_create()?;
            cfg.health_monitoring.enabled = false;
            cfg.save()?;
            print_success(
                "Health monitoring disabled. Restart the node to apply: kwaainet restart",
            );
        }

        // -------------------------------------------------------------------
        // service
        // -------------------------------------------------------------------
        Command::Service(args) => {
            let svc = service::get_service_manager();
            match args.action {
                ServiceAction::Install => {
                    print_box_header("🔧 Installing Auto-Start Service");
                    svc.install()?;
                    print_success("Auto-start service installed. KwaaiNet will start on boot.");
                    print_separator();
                }
                ServiceAction::Uninstall => {
                    print_box_header("🔧 Uninstalling Auto-Start Service");
                    svc.uninstall()?;
                    print_success("Auto-start service uninstalled.");
                    print_separator();
                }
                ServiceAction::Status => {
                    let st = svc.status();
                    print_box_header("🔧 Auto-Start Service Status");
                    println!(
                        "  Installed: {}",
                        if st.installed { "✅ Yes" } else { "❌ No" }
                    );
                    println!(
                        "  Running:   {}",
                        if st.running { "🟢 Yes" } else { "🔴 No" }
                    );
                    if let Some(pid) = st.pid {
                        println!("  PID:       {}", pid);
                    }
                    print_separator();
                }
                ServiceAction::Restart => {
                    print_box_header("🔧 Restarting Auto-Start Service");
                    svc.restart()?;
                    print_success("Service restarted.");
                    print_separator();
                }
            }
        }

        // -------------------------------------------------------------------
        // reconnect
        // -------------------------------------------------------------------
        Command::Reconnect => {
            print_box_header("🔄 P2P Network Reconnection");
            let mgr = DaemonManager::new();
            if mgr.is_running() {
                mgr.stop_process()?;
                let pid = DaemonManager::spawn_daemon_child(&[])?;
                print_success(&format!(
                    "Node restarted (PID {}). Reconnecting to P2P network.",
                    pid
                ));
            } else {
                let svc = service::get_service_manager();
                if svc.status().running {
                    svc.restart()?;
                    print_success("Service restarted. Node will reconnect on startup.");
                } else {
                    print_error("No running node found. Start it first: kwaainet start --daemon");
                    std::process::exit(1);
                }
            }
            print_separator();
        }

        // -------------------------------------------------------------------
        // monitor
        // -------------------------------------------------------------------
        Command::Monitor(args) => match args.action {
            MonitorAction::Stats => {
                print_box_header("📈 P2P Connection Statistics");
                match monitor::load_stats() {
                    Some(stats) => {
                        println!("  Samples:     {}", stats.samples);
                        println!(
                            "  Connections: {} current / {:.1} avg",
                            stats.current_connections, stats.avg_connections
                        );
                        println!(
                            "  Min/Max:     {} / {}",
                            stats.min_connections, stats.max_connections
                        );
                        println!("  Uptime:      {:.1}%", stats.uptime_percent);
                    }
                    None => {
                        println!("  No monitoring data yet.");
                        print_info("Start the node and wait for data collection.");
                    }
                }
                print_separator();
            }
            MonitorAction::Alert(a) => {
                let mut cfg = monitor::load_alert_config();
                print_box_header("🚨 Alert Configuration");

                if a.enable {
                    cfg.enabled = true;
                }
                if a.disable {
                    cfg.enabled = false;
                }
                if let Some(t) = a.threshold {
                    cfg.disconnection_threshold_minutes = t;
                }
                if let Some(url) = a.webhook {
                    cfg.webhook_url = Some(url);
                }
                if let Some(m) = a.min_connections {
                    cfg.min_connections = m;
                }

                if a.enable || a.disable || a.threshold.is_some() || a.min_connections.is_some() {
                    monitor::save_alert_config(&cfg)?;
                }

                println!(
                    "  Enabled:    {}",
                    if cfg.enabled { "✅ Yes" } else { "❌ No" }
                );
                println!(
                    "  Threshold:  {} minutes",
                    cfg.disconnection_threshold_minutes
                );
                println!("  Min conns:  {}", cfg.min_connections);
                println!(
                    "  Webhook:    {}",
                    cfg.webhook_url.as_deref().unwrap_or("Not configured")
                );
                print_separator();
            }
        },

        // -------------------------------------------------------------------
        // update
        // -------------------------------------------------------------------
        Command::Update(args) => {
            print_box_header("🔄 KwaaiNet Update");
            let checker = updater::UpdateChecker::new();
            println!("  Current version: v{}", checker.current_version);
            println!("  Checking for updates…");
            println!();

            // Always do a live check when the user explicitly runs `kwaainet update`.
            // The 24-hour cache is only for the background ambient hint, not user-initiated checks.
            match checker.check(true).await? {
                None => {
                    print_success("You are running the latest version!");
                }
                Some(info) => {
                    println!("  🎉 New version available: v{}", info.version);
                    if let Some(ref name) = info.name {
                        println!("  Release: {}", name);
                    }
                    if let Some(ref url) = info.url {
                        println!("  Details: {}", url);
                    }
                    if let Some(ref body) = info.body {
                        println!("\n  Release notes:");
                        for line in body.lines().take(5) {
                            if !line.trim().is_empty() {
                                println!("     {}", line);
                            }
                        }
                    }
                    println!();
                    if args.check {
                        print_info("Run 'kwaainet update' (without --check) to install");
                    } else {
                        // Gracefully stop tracked services so the daemon can
                        // unannounce from DHT before the hard kill in the batch.
                        #[cfg(windows)]
                        {
                            let shard_mgr = ShardManager::new();
                            if shard_mgr.is_running() {
                                shard_mgr.stop_process();
                                print_info("Shard server stopping…");
                            }
                            let storage_mgr = StorageApiManager::new();
                            if storage_mgr.is_running() {
                                storage_mgr.stop_process();
                                print_info("Storage API stopping…");
                            }
                            let node_mgr = DaemonManager::new();
                            if node_mgr.is_running() {
                                let _ = node_mgr.stop_process();
                                print_info("Daemon stopping…");
                            }
                            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        }
                        println!("  Installing v{}…", info.version);
                        println!();
                        checker.install_update(&info.version).await?;
                        println!();
                        #[cfg(windows)]
                        print_success(&format!(
                            "Installer launched in background — all services will be stopped automatically.\n  Run 'kwaainet start --daemon' once done."
                        ));
                        #[cfg(not(windows))]
                        print_success(&format!(
                            "Updated to v{}! Restart: kwaainet start --daemon",
                            info.version
                        ));
                    }
                }
            }
            print_separator();
        }

        // -------------------------------------------------------------------
        // calibrate
        // -------------------------------------------------------------------
        Command::Calibrate(args) => {
            let cfg = KwaaiNetConfig::load_or_create()?;
            let model = args.model.unwrap_or_else(|| cfg.model.clone());

            print_box_header("🔧 KwaaiNet Block Calibration");
            println!("  Model: {}", model);
            println!();

            let engine = calibration::CalibrationEngine::new();
            let hw = &engine.hardware;
            println!("  Hardware:");
            println!(
                "    Memory: {} total / {} available",
                format_bytes(hw.total_memory),
                format_bytes(hw.available_memory)
            );
            println!("    CPU cores: {}", hw.cpu_cores);
            if let Some(ref gpu) = hw.gpu {
                println!(
                    "    GPU: {} ({} total / {} free)",
                    gpu.name,
                    format_bytes(gpu.total_vram),
                    format_bytes(gpu.free_vram)
                );
            } else {
                println!("    GPU: none detected");
            }
            println!();

            let profile = engine.calibrate(&model);
            println!("  Total model blocks: {}", profile.total_blocks);
            if profile.gpu_based {
                println!("  Based on: GPU VRAM");
            } else {
                println!("  Based on: System RAM");
            }
            println!();
            println!("  Recommendations:");
            println!("    🔹 Minimum:       {} blocks", profile.min_blocks);
            println!(
                "    ⭐ Recommended:   {} blocks",
                profile.recommended_blocks
            );
            println!("    🔸 Maximum:       {} blocks", profile.max_blocks);
            println!();
            println!(
                "    📊 Available now: {} blocks",
                profile.available_now_blocks
            );
            print_separator();

            if let Some(ref apply) = args.apply {
                if let Some(new_blocks) = profile.get_blocks(apply) {
                    let mut cfg = KwaaiNetConfig::load_or_create()?;
                    cfg.blocks = new_blocks;
                    cfg.save()?;
                    print_success(&format!(
                        "Applied {} profile: blocks = {}",
                        apply, new_blocks
                    ));
                    print_info("Restart the node to use the new setting: kwaainet restart");
                } else {
                    print_error(&format!(
                        "Unknown profile '{}'. Use: min, recommended, or max",
                        apply
                    ));
                }
            } else {
                print_info("Apply recommended: kwaainet calibrate --apply recommended");
            }
        }

        // -------------------------------------------------------------------
        // load-model
        // -------------------------------------------------------------------
        Command::LoadModel(args) => {
            print_box_header("📦 KwaaiNet Model Loader");
            println!("  Model ref: {}", args.model);
            println!();

            // Detect source: `owner/model` without `hf.co/` prefix → HF cache.
            // Everything else (e.g. `qwen3:0.6b`, `hf.co/org/model:tag`) → Ollama.
            let is_hf = args.model.contains('/') && !args.model.starts_with("hf.co/");

            // Use available system RAM (85%) as the memory budget.
            let system_ram = {
                use sysinfo::System;
                let mut sys = System::new();
                sys.refresh_memory();
                sys.total_memory() // bytes
            };
            let max_memory = ((system_ram as f64 * 0.85) as usize).max(4 * 1024 * 1024 * 1024); // at least 4 GB

            let engine_config = EngineConfig {
                max_memory,
                ..EngineConfig::default()
            };

            let mut engine = match InferenceEngine::new(engine_config) {
                Ok(e) => e,
                Err(e) => {
                    print_error(&format!("Failed to create inference engine: {e}"));
                    return Ok(());
                }
            };

            if is_hf {
                // ── HuggingFace SafeTensors ──────────────────────────────────
                let snapshot_dir = match hf::resolve_snapshot(&args.model) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                };

                // Sum shard sizes for display (follow symlinks via std::fs::metadata).
                let total_size: u64 = std::fs::read_dir(&snapshot_dir)
                    .ok()
                    .map(|rd| {
                        rd.filter_map(|e| e.ok())
                            .map(|e| e.path())
                            .filter(|p| {
                                p.extension().and_then(|x| x.to_str()) == Some("safetensors")
                            })
                            .filter_map(|p| std::fs::metadata(&p).ok())
                            .map(|m| m.len())
                            .sum()
                    })
                    .unwrap_or(0);

                println!("  Source:   HuggingFace cache");
                println!("  Path:     {}", snapshot_dir.display());
                println!("  Size:     {}", format_bytes(total_size));
                println!();
                println!("  Loading SafeTensors shards into memory…");

                let start = std::time::Instant::now();
                match engine.load_model(&snapshot_dir, ModelFormat::SafeTensors) {
                    Ok(handle) => {
                        let elapsed = start.elapsed();
                        let info = engine.model_info(&handle).expect("handle was just created");
                        print_success(&format!("Loaded in {:.1}s", elapsed.as_secs_f32()));
                        println!();
                        println!("  Architecture:  {}", info.architecture);
                        println!("  Vocab size:    {}", info.vocab_size);
                        println!("  Context:       {} tokens", info.context_length);
                        println!(
                            "  Memory usage:  {}",
                            format_bytes(info.memory_bytes as u64)
                        );
                        println!("  Quantized:     {}", info.is_quantized);
                    }
                    Err(e) => {
                        print_error(&format!("Failed to load model: {e}"));
                    }
                }
            } else {
                // ── Ollama GGUF ──────────────────────────────────────────────
                let blob_path = match ollama::resolve_model_blob(&args.model) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                };

                let file_size = std::fs::metadata(&blob_path).map(|m| m.len()).unwrap_or(0);

                println!("  Source:   Ollama");
                println!("  Blob:     {}", blob_path.display());
                println!("  Size:     {}", format_bytes(file_size));
                println!();
                println!("  Loading GGUF weights into memory…");

                let start = std::time::Instant::now();
                match engine.load_model(&blob_path, ModelFormat::Gguf) {
                    Ok(handle) => {
                        let elapsed = start.elapsed();
                        let info = engine.model_info(&handle).expect("handle was just created");
                        print_success(&format!("Loaded in {:.1}s", elapsed.as_secs_f32()));
                        println!();
                        println!("  Architecture:  {}", info.architecture);
                        println!("  Vocab size:    {}", info.vocab_size);
                        println!("  Context:       {} tokens", info.context_length);
                        println!(
                            "  Memory usage:  {}",
                            format_bytes(info.memory_bytes as u64)
                        );
                        println!("  Quantized:     {}", info.is_quantized);
                    }
                    Err(e) => {
                        print_error(&format!("Failed to load model: {e}"));
                    }
                }
            }

            print_separator();
        }

        // -------------------------------------------------------------------
        // generate  (tokenizer smoke-test; forward pass not yet implemented)
        // -------------------------------------------------------------------
        Command::Generate(args) => {
            print_box_header("🧠 KwaaiNet Generate");
            println!("  Model:  {}", args.model);
            println!("  Prompt: {:?}", args.prompt);
            println!();

            let is_hf = args.model.contains('/') && !args.model.starts_with("hf.co/");

            // Detect system RAM for engine config (same as load-model).
            let system_ram = {
                use sysinfo::System;
                let mut sys = System::new();
                sys.refresh_memory();
                sys.total_memory()
            };
            let engine_config = EngineConfig {
                max_memory: ((system_ram as f64 * 0.85) as usize).max(4 * 1024 * 1024 * 1024),
                ..EngineConfig::default()
            };

            let mut engine = match InferenceEngine::new(engine_config) {
                Ok(e) => e,
                Err(e) => {
                    print_error(&format!("Engine init failed: {e}"));
                    return Ok(());
                }
            };

            // Load model (reuse same logic as load-model).
            let handle = if is_hf {
                let snapshot = match hf::resolve_snapshot(&args.model) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                };
                println!("  Loading SafeTensors shards…");
                match engine.load_model(&snapshot, ModelFormat::SafeTensors) {
                    Ok(h) => h,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                }
            } else {
                let blob = match ollama::resolve_model_blob(&args.model) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                };
                println!("  Loading GGUF blob…");
                match engine.load_model(&blob, ModelFormat::Gguf) {
                    Ok(h) => h,
                    Err(e) => {
                        print_error(&format!("{e}"));
                        return Ok(());
                    }
                }
            };

            println!("  Model loaded.");
            println!();

            match engine.generate(&handle, &args.prompt) {
                Ok(text) => {
                    print_success("Generation complete");
                    println!("{text}");

                    // Report and persist throughput so `kwaainet start` can
                    // announce a real value to the network map.
                    let tps = engine.last_throughput_tps();
                    if tps > 0.0 {
                        let hidden_size = engine
                            .model_info(&handle)
                            .map(|i| i.hidden_dim)
                            .unwrap_or(4096);
                        println!();
                        println!(
                            "  Throughput: {:.1} tok/s  (hidden_dim={})",
                            tps, hidden_size
                        );
                        if let Err(e) = throughput::save(&args.model, tps, hidden_size) {
                            eprintln!("  Warning: could not save throughput cache: {e}");
                        }
                    }
                }
                Err(e) => {
                    println!("  {e}");
                }
            }

            print_separator();
        }

        // -------------------------------------------------------------------
        // benchmark
        // -------------------------------------------------------------------
        Command::Benchmark(args) => {
            let cfg = KwaaiNetConfig::load_or_create()?;
            let model = args.model.as_deref().unwrap_or(&cfg.model).to_string();

            // ── Try llama.cpp fast path first ────────────────────────────
            #[cfg(feature = "llama-cpp")]
            let ran_llama = {
                // Try to find a GGUF: explicit path > Ollama > ~/.kwaainet/models/
                let gguf_path: Option<std::path::PathBuf> = if let Some(ref p) = args.model_path {
                    if p.extension().and_then(|e| e.to_str()) == Some("gguf") && p.exists() {
                        Some(p.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
                .or_else(|| ollama::resolve_model_blob(&model).ok())
                .or_else(|| {
                    // Scan ~/.kwaainet/models/
                    let models_dir = dirs::home_dir()?.join(".kwaainet/models");
                    let base = model.split('/').last().unwrap_or(&model).to_lowercase();
                    std::fs::read_dir(&models_dir)
                        .ok()?
                        .flatten()
                        .find_map(|e| {
                            let name = e.file_name().to_string_lossy().to_lowercase();
                            if name.ends_with(".gguf") && name.contains(&base) {
                                Some(e.path())
                            } else {
                                None
                            }
                        })
                });

                if let Some(gguf_path) = gguf_path {
                    let n_steps = args.steps;

                    print_box_header("⚡ KwaaiNet Benchmark");
                    println!("  Model:  {}", model);
                    println!("  Device: llama.cpp (GPU)");
                    println!("  GGUF:   {}", gguf_path.display());
                    println!("  Steps:  {}", n_steps);
                    println!();

                    println!("  Loading model…");
                    let load_start = std::time::Instant::now();
                    let (backend, llama_model) =
                        llama_local::load_model(&gguf_path).context("Failed to load GGUF model")?;
                    let load_secs = load_start.elapsed().as_secs_f64();
                    print_success(&format!("Model loaded ({:.1}s)", load_secs));
                    println!();

                    // Use a prompt that elicits a long response to avoid early EOS.
                    let prompt = "<|start_header_id|>user<|end_header_id|>\n\nExplain the theory of general relativity in detail.<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n";

                    // Warm-up + measure in a single spawn_blocking to avoid
                    // Send issues with LlamaBackend.
                    println!("  Warming up (5 tokens)…");
                    println!("  Measuring ({n_steps} decode tokens)…");
                    let result = tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
                        // Warm-up (untimed, separate context)
                        let _ = llama_local::run_inference_streaming(
                            &backend,
                            &llama_model,
                            prompt,
                            5,
                            1.0,
                            0,
                            1.0,
                            |_| true,
                        );
                        // Measured run
                        llama_local::run_inference_streaming(
                            &backend,
                            &llama_model,
                            prompt,
                            n_steps,
                            1.0, // temperature=1.0 to avoid early EOS
                            0,
                            1.0,
                            |_| true,
                        )
                    })
                    .await
                    .map_err(|e| anyhow::anyhow!("benchmark join: {e}"))??;

                    let decode_tps = result.tokens_generated as f64 / (result.decode_ms / 1000.0);
                    let prefill_tps = 1.0 / (result.prefill_ms / 1000.0); // prompt is ~1 logical unit

                    // Use standard hidden_size for the model
                    let hidden_size = if model.to_lowercase().contains("70b") {
                        8192
                    } else {
                        4096
                    };

                    println!();
                    println!("  ── Results ───────────────────────────────────────────────");
                    println!("  Prefill:     {:>7.1}ms", result.prefill_ms);
                    println!(
                        "  Decode:      {:>7.1} tok/s  ({} tokens in {:.0}ms)",
                        decode_tps, result.tokens_generated, result.decode_ms
                    );
                    println!("  Load time:   {:>7.1}s", load_secs);
                    println!("  Device:      llama.cpp (GPU)");

                    if let Err(e) = throughput::save(&model, decode_tps, hidden_size) {
                        eprintln!("  Warning: could not save throughput cache: {e}");
                    } else {
                        println!("  Cached ✓     (~/.kwaainet/throughput_cache.json)");
                    }

                    print_separator();
                    true
                } else {
                    false
                }
            };
            #[cfg(not(feature = "llama-cpp"))]
            let ran_llama = false;

            // ── Fallback: candle TransformerShard benchmark ───────────────
            if !ran_llama {
                use kwaai_inference::tokenizer::Tokenizer as _;
                use kwaai_inference::TransformerShard;

                let device_type = if args.gpu {
                    kwaai_inference::DeviceType::require_gpu()
                        .context("--gpu was specified but no GPU is available")?
                } else {
                    kwaai_inference::DeviceType::Cpu
                };

                print_box_header("⚡ KwaaiNet Benchmark");
                println!("  Model:  {}", model);
                println!("  Device: {}", device_type);
                println!("  Steps:  {} (+ 5 warm-up)", args.steps);
                println!();

                // Resolve model directory
                let model_dir = if let Some(p) = &args.model_path {
                    p.clone()
                } else {
                    hf::resolve_snapshot(&model)?
                };
                let config_path = model_dir.join("config.json");
                let total_blocks = cfg.model_total_blocks() as usize;

                // Collect safetensors shards
                let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(&model_dir)
                    .context("read model dir")?
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("safetensors"))
                    .collect();
                paths.sort();
                if paths.is_empty() {
                    print_error(&format!(
                        "No .safetensors files found in {}",
                        model_dir.display()
                    ));
                    return Ok(());
                }

                let device = device_type
                    .to_candle_device()
                    .context("Failed to create compute device")?;

                // Load TransformerShard (same path as shard run --local)
                println!("  Loading TransformerShard ({} blocks)…", total_blocks);
                let load_start = std::time::Instant::now();
                let path_refs: Vec<&std::path::Path> = paths.iter().map(|p| p.as_path()).collect();
                let shard = std::sync::Arc::new(
                    TransformerShard::load(&path_refs, &config_path, &device, 0, total_blocks)
                        .context("Failed to load model")?,
                );
                let load_secs = load_start.elapsed().as_secs_f64();
                print_success(&format!(
                    "Model loaded ({} blocks, {:.1}s)",
                    total_blocks, load_secs
                ));
                println!();

                let hidden_size = shard.cfg.hidden_dim;

                // Tokenize a fixed benchmark prompt
                let prompt = "<|start_header_id|>user<|end_header_id|>\n\nThe capital of France is<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n";
                let mut token_ids: Vec<u32> = {
                    use kwaai_inference::tokenizer::Tokenizer as _;
                    shard.tokenizer.encode(prompt).context("encode prompt")?
                };
                if let Some(bos) = shard.tokenizer.bos_token_id() {
                    token_ids.insert(0, bos);
                }

                // Run warm-up, prefill, and decode in a single blocking thread
                // to avoid Metal command-queue contention across thread boundaries.
                let n_prompt = token_ids.len();
                let n_steps = args.steps;
                let bench_shard = shard.clone();
                let bench_ids = token_ids.clone();

                println!("  Warming up (5 steps)…");
                println!("  Measuring ({n_steps} decode steps)…");

                let (prefill_ms, decode_ms) =
                    tokio::task::spawn_blocking(move || -> anyhow::Result<(f64, f64)> {
                        // ── Warm-up (separate session, not timed) ────────────
                        {
                            let session = 0xDEAD_BEEF_u64;
                            let mut ids = bench_ids.clone();
                            let mut sp = 0usize;
                            for _ in 0..5 {
                                let logits = bench_shard
                                    .forward_full(session, &ids, sp)
                                    .map_err(|e| anyhow::anyhow!("{e}"))?;
                                let logits_cpu = logits.to_device(&candle_core::Device::Cpu)?;
                                let flat = logits_cpu.flatten_all()?;
                                let next = shard_cmd::sample_token(&flat, 1.0, 0, 1.0)? as u32;
                                sp += ids.len();
                                ids = vec![next];
                            }
                        }

                        // ── Prefill (timed) ──────────────────────────────────
                        let session = 0xBE_0001_u64;
                        let prefill_start = std::time::Instant::now();
                        let logits = bench_shard
                            .forward_full(session, &bench_ids, 0)
                            .map_err(|e| anyhow::anyhow!("{e}"))?;
                        let prefill_ms = prefill_start.elapsed().as_secs_f64() * 1000.0;

                        let logits_cpu = logits.to_device(&candle_core::Device::Cpu)?;
                        let flat = logits_cpu.flatten_all()?;
                        let mut id = shard_cmd::sample_token(&flat, 1.0, 0, 1.0)? as u32;

                        // ── Decode (timed) ───────────────────────────────────
                        let mut sp = n_prompt;
                        let decode_start = std::time::Instant::now();
                        for _ in 0..n_steps {
                            let logits = bench_shard
                                .forward_full(session, &[id], sp)
                                .map_err(|e| anyhow::anyhow!("{e}"))?;
                            let logits_cpu = logits.to_device(&candle_core::Device::Cpu)?;
                            let flat = logits_cpu.flatten_all()?;
                            id = shard_cmd::sample_token(&flat, 1.0, 0, 1.0)? as u32;
                            sp += 1;
                        }
                        let decode_ms = decode_start.elapsed().as_secs_f64() * 1000.0;

                        Ok((prefill_ms, decode_ms))
                    })
                    .await
                    .map_err(|e| anyhow::anyhow!("benchmark join: {e}"))??;

                let prefill_tps = n_prompt as f64 / (prefill_ms / 1000.0);
                let decode_tps = n_steps as f64 / (decode_ms / 1000.0);

                println!();
                println!("  ── Results ───────────────────────────────────────────────");
                println!(
                    "  Prefill:     {:>7.1} tok/s  ({} tokens in {:.0}ms)",
                    prefill_tps, n_prompt, prefill_ms
                );
                println!(
                    "  Decode:      {:>7.1} tok/s  ({} tokens in {:.0}ms)",
                    decode_tps, n_steps, decode_ms
                );
                println!("  Load time:   {:>7.1}s", load_secs);
                println!("  Device:      {}", device_type);

                if let Err(e) = throughput::save(&model, decode_tps, hidden_size) {
                    eprintln!("  Warning: could not save throughput cache: {e}");
                } else {
                    println!("  Cached ✓     (~/.kwaainet/throughput_cache.json)");
                }

                print_separator();
            }
        }

        // -------------------------------------------------------------------
        // serve  — OpenAI-compatible API server
        // -------------------------------------------------------------------
        Command::Serve(args) => {
            serve_command(args).await?;
        }

        // -------------------------------------------------------------------
        // identity
        // -------------------------------------------------------------------
        Command::Identity(args) => {
            identity::run_identity_command(args).await?;
        }

        // -------------------------------------------------------------------
        // vpk
        // -------------------------------------------------------------------
        Command::Vpk(args) => {
            vpk::run(args).await?;
        }

        // -------------------------------------------------------------------
        // reputation
        // -------------------------------------------------------------------
        Command::Reputation(args) => {
            reputation_cmd::run(args).await?;
        }

        // -------------------------------------------------------------------
        // storage (requires --features storage)
        // -------------------------------------------------------------------
        #[cfg(feature = "storage")]
        Command::Storage(args) => {
            storage::run(args).await?;
        }
        #[cfg(not(feature = "storage"))]
        Command::Storage(_) => {
            print_error(
                "Storage support not compiled. Rebuild with: cargo build --features storage",
            );
        }

        // -------------------------------------------------------------------
        // uninstall
        // -------------------------------------------------------------------
        Command::Uninstall(args) => {
            uninstall::run_uninstall(&args)?;
        }

        // -------------------------------------------------------------------
        // ui — launch Node Dashboard (web UI)
        // -------------------------------------------------------------------
        Command::Ui => {
            use std::env;
            use std::process::Command;
            use std::thread;
            use std::time::Duration;

            print_box_header("KwaaiNet Node Dashboard");

            let mut dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            let mut backend_dir = None;
            for _ in 0..20 {
                let dashboard = dir.join("systems").join("node-dashboard");
                let backend = dashboard.join("backend");
                let server_js = backend.join("server.js");
                if server_js.exists() {
                    backend_dir = Some(backend);
                    break;
                }
                if !dir.pop() {
                    break;
                }
            }

            let backend_dir = match backend_dir {
                Some(d) => d,
                None => {
                    print_error("Node Dashboard not found. Run from KwaaiNet repo root:");
                    println!("  ./start-ui.sh");
                    println!("  Or: cd systems/node-dashboard && npm run dev");
                    print_separator();
                    return Ok(());
                }
            };

            // Check for Node.js
            if Command::new("node").arg("--version").output().is_err() {
                print_error(
                    "Node.js not found. Install Node.js 18+ and ensure it is on your PATH.",
                );
                print_separator();
                return Ok(());
            }

            println!("  Starting dashboard backend at http://127.0.0.1:3456 ...");
            let child = Command::new("node")
                .arg("server.js")
                .current_dir(&backend_dir)
                .spawn();

            match child {
                Ok(_) => {
                    thread::sleep(Duration::from_secs(2));
                    let url = "http://127.0.0.1:3456";
                    #[cfg(target_os = "macos")]
                    let _ = Command::new("open").arg(url).spawn();
                    #[cfg(target_os = "windows")]
                    let _ = Command::new("cmd").args(["/C", "start", url]).spawn();
                    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
                    let _ = Command::new("xdg-open").arg(url).spawn();
                    print_success(
                        "Dashboard started. Open in browser if it did not open automatically.",
                    );
                    println!("  {}", url);
                }
                Err(e) => {
                    print_error(&format!("Failed to start dashboard: {}", e));
                }
            }
            print_separator();
        }

        // -------------------------------------------------------------------
        // shard
        // -------------------------------------------------------------------
        Command::Shard(args) => {
            shard_cmd::run(args).await?;
        }

        // -------------------------------------------------------------------
        // setup
        // -------------------------------------------------------------------
        Command::Setup(args) => {
            if args.get_deps {
                setup::get_dependencies().await?;
            } else {
                print_box_header("🔧 KwaaiNet Setup");
                let cfg = KwaaiNetConfig::load_or_create()?;

                // Create all required directories
                std::fs::create_dir_all(config::run_dir())?;
                std::fs::create_dir_all(config::log_dir())?;

                print_success("Directories created");
                print_success(&format!(
                    "Config written to {}",
                    config::config_file().display()
                ));
                println!();
                println!("  Model:  {}", cfg.model);
                println!("  Blocks: {}", cfg.blocks);
                println!("  Port:   {}", cfg.port);
                println!();
                print_info("Run `kwaainet setup --get-deps` to download p2pd if missing");
                print_info("Start the node with: kwaainet start --daemon");
                print_separator();
            }
        }
    }

    // Print a one-line update hint if a newer version was found.
    // Wait up to 2 s — for long-running commands the task finished long ago
    // (instant cache hit); for fast commands 2 s is a graceful upper bound.
    if let Some(task) = update_task {
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), task).await;
        if let Ok(Ok(Ok(Some(info)))) = result {
            // Only show the hint when the cached version is actually newer than what's running.
            if updater::is_newer(&info.version, updater::CURRENT_VERSION) {
                println!();
                print_info(&format!(
                    "kwaainet v{} is available — run 'kwaainet update' to upgrade",
                    info.version
                ));
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Serve helper
// ---------------------------------------------------------------------------

async fn serve_command(args: ServeArgs) -> Result<()> {
    let cfg = KwaaiNetConfig::load_or_create()?;
    let model = args.model.unwrap_or_else(|| cfg.model.clone());

    print_box_header("🌐 KwaaiNet OpenAI API Server");
    println!("  Model:  {}", model);
    println!("  Port:   {}", args.port);
    println!();

    if crate::daemon::port_in_use(args.port) {
        print_warning(&format!(
            "Port {} is already in use — API server may already be running.",
            args.port
        ));
        print_info(&format!(
            "Check with: curl http://localhost:{}/v1/models",
            args.port
        ));
        print_separator();
        return Ok(());
    }

    let system_ram = {
        use sysinfo::System;
        let mut sys = System::new();
        sys.refresh_memory();
        sys.total_memory()
    };
    let engine_config = EngineConfig {
        max_memory: ((system_ram as f64 * 0.85) as usize).max(4 * 1024 * 1024 * 1024),
        ..EngineConfig::default()
    };

    let mut engine = match InferenceEngine::new(engine_config) {
        Ok(e) => e,
        Err(e) => {
            print_error(&format!("Engine init failed: {e}"));
            return Ok(());
        }
    };

    let is_hf = model.contains('/') && !model.starts_with("hf.co/");
    let handle = if is_hf {
        let snapshot = match hf::resolve_snapshot(&model) {
            Ok(p) => p,
            Err(e) => {
                print_error(&format!("{e}"));
                return Ok(());
            }
        };
        println!("  Loading SafeTensors shards…");
        match engine.load_model(&snapshot, ModelFormat::SafeTensors) {
            Ok(h) => h,
            Err(e) => {
                print_error(&format!("{e}"));
                return Ok(());
            }
        }
    } else {
        let blob = match ollama::resolve_model_blob(&model) {
            Ok(p) => p,
            Err(e) => {
                print_error(&format!("{e}"));
                return Ok(());
            }
        };
        println!("  Loading GGUF blob…");
        match engine.load_model(&blob, ModelFormat::Gguf) {
            Ok(h) => h,
            Err(e) => {
                let msg = e.to_string();
                print_error(&format!("{msg}"));
                if msg.contains("unknown dtype") {
                    print_info("This model uses a quantization type not yet supported by the");
                    print_info("candle inference backend. Use llama.cpp instead:");
                    print_info(&format!(
                        "  kwaainet shard api --model-path {} --port {}",
                        blob.display(),
                        args.port
                    ));
                    print_info("llama.cpp supports all GGUF quantization types including IQ* and Q4_0_8_8.");
                }
                return Ok(());
            }
        }
    };

    print_success("Model loaded — starting API server");
    print_separator();

    api::run_api_server(args.port, engine, handle, model).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn print_last_lines(path: &std::path::Path, n: usize) {
    match std::fs::read_to_string(path) {
        Ok(text) => {
            let lines: Vec<&str> = text.lines().collect();
            let start = lines.len().saturating_sub(n);
            for line in &lines[start..] {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error reading log: {}", e),
    }
}
