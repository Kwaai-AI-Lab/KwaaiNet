//! kwaainet – KwaaiNet node CLI

mod calibration;
mod cli;
mod config;
mod daemon;
mod display;
mod health;
mod monitor;
mod node;
mod ollama;
mod service;
mod updater;

use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Command, MonitorAction, ServiceAction};
use kwaai_inference::{EngineConfig, InferenceEngine, InferenceProvider, ModelFormat};
use config::KwaaiNetConfig;
use daemon::DaemonManager;
use display::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialise logging (RUST_LOG overrides, default info)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

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

            // Apply CLI overrides to config
            if let Some(m) = args.model { cfg.model = m; }
            if let Some(b) = args.blocks { cfg.blocks = b; }
            if let Some(p) = args.port { cfg.port = p; }
            if args.no_gpu { cfg.use_gpu = false; }
            if let Some(n) = args.public_name { cfg.public_name = Some(n); }
            if let Some(ip) = args.public_ip { cfg.public_ip = Some(ip); }
            if let Some(a) = args.announce_addr { cfg.announce_addr = Some(a); }
            if args.no_relay { cfg.no_relay = true; }

            // Auto-detect public IP if not set
            if cfg.public_ip.is_none() {
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
                // Build extra args from current config so the child knows them
                let child_pid = DaemonManager::spawn_daemon_child(&[])?;
                println!();
                print_success(&format!("KwaaiNet daemon started (PID {})", child_pid));
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

            if mgr.is_running() {
                info!("Stopping existing process…");
                mgr.stop_process()?;
            }

            let child_pid = DaemonManager::spawn_daemon_child(&[])?;
            print_success(&format!("KwaaiNet daemon restarted (PID {})", child_pid));
            print_separator();
        }

        // -------------------------------------------------------------------
        // status
        // -------------------------------------------------------------------
        Command::Status => {
            let mgr = DaemonManager::new();
            let status = mgr.get_status();

            print_box_header("📊 KwaaiNet Node Status");

            if status.running {
                let pid = status.pid.unwrap_or(0);
                let uptime = status.uptime_secs.map(format_uptime).unwrap_or_else(|| "unknown".to_string());
                let cpu = status.cpu_percent.map(|c| format!("{:.1}%", c)).unwrap_or_else(|| "n/a".to_string());
                let mem = status.memory_mb.map(|m| format!("{:.0} MB", m)).unwrap_or_else(|| "n/a".to_string());

                println!("  🟢 Status:  Running");
                println!("  🔢 PID:     {}", pid);
                println!("  ⏱️  Uptime:  {}", uptime);
                println!("  💻 CPU:     {}", cpu);
                println!("  🧠 Memory:  {}", mem);
            } else {
                println!("  🔴 Status:  Not running");
                print_info("Start with: kwaainet start --daemon");
            }

            print_separator();
        }

        // -------------------------------------------------------------------
        // logs
        // -------------------------------------------------------------------
        Command::Logs(args) => {
            let log_path = config::log_file();

            if !log_path.exists() {
                print_warning("No log file found yet. Start the node first.");
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
            let mut cfg = KwaaiNetConfig::load_or_create()?;

            if args.view || (!args.view && args.set.is_none()) {
                print_box_header("⚙️  KwaaiNet Configuration");
                println!("  🤖 model:        {}", cfg.model);
                println!("  🧱 blocks:       {}", cfg.blocks);
                println!("  🔌 port:         {}", cfg.port);
                println!("  🖥️  use_gpu:      {}", cfg.use_gpu);
                println!("  📋 log_level:    {}", cfg.log_level);
                if let Some(ref n) = cfg.public_name { println!("  📋 public_name:  {}", n); }
                if let Some(ref ip) = cfg.public_ip { println!("  📋 public_ip:    {}", ip); }
                print_separator();
            } else if let Some(kv) = args.set {
                let key = &kv[0];
                let value = &kv[1];
                cfg.set_key(key, value)?;
                print_box_header("⚙️  Configuration Updated");
                print_success(&format!("Set {} = {}", key, value));
                print_separator();
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
            print_success("Health monitoring disabled. Restart the node to apply: kwaainet restart");
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
                    println!("  Installed: {}", if st.installed { "✅ Yes" } else { "❌ No" });
                    println!("  Running:   {}", if st.running { "🟢 Yes" } else { "🔴 No" });
                    if let Some(pid) = st.pid { println!("  PID:       {}", pid); }
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
                print_success(&format!("Node restarted (PID {}). Reconnecting to P2P network.", pid));
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
        Command::Monitor(args) => {
            match args.action {
                MonitorAction::Stats => {
                    print_box_header("📈 P2P Connection Statistics");
                    match monitor::load_stats() {
                        Some(stats) => {
                            println!("  Samples:     {}", stats.samples);
                            println!("  Connections: {} current / {:.1} avg", stats.current_connections, stats.avg_connections);
                            println!("  Min/Max:     {} / {}", stats.min_connections, stats.max_connections);
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

                    if a.enable { cfg.enabled = true; }
                    if a.disable { cfg.enabled = false; }
                    if let Some(t) = a.threshold { cfg.disconnection_threshold_minutes = t; }
                    if let Some(url) = a.webhook { cfg.webhook_url = Some(url); }
                    if let Some(m) = a.min_connections { cfg.min_connections = m; }

                    if a.enable || a.disable || a.threshold.is_some() || a.min_connections.is_some() {
                        monitor::save_alert_config(&cfg)?;
                    }

                    println!("  Enabled:    {}", if cfg.enabled { "✅ Yes" } else { "❌ No" });
                    println!("  Threshold:  {} minutes", cfg.disconnection_threshold_minutes);
                    println!("  Min conns:  {}", cfg.min_connections);
                    println!("  Webhook:    {}", cfg.webhook_url.as_deref().unwrap_or("Not configured"));
                    print_separator();
                }
            }
        }

        // -------------------------------------------------------------------
        // update
        // -------------------------------------------------------------------
        Command::Update(args) => {
            print_box_header("🔄 KwaaiNet Update");
            let checker = updater::UpdateChecker::new();
            println!("  Current version: v{}", checker.current_version);
            println!("  Checking for updates…");
            println!();

            match checker.check(args.force).await? {
                None => {
                    print_success("You are running the latest version!");
                }
                Some(info) => {
                    println!("  🎉 New version available: v{}", info.version);
                    if let Some(ref name) = info.name { println!("  Release: {}", name); }
                    if let Some(ref url) = info.url { println!("  Details: {}", url); }
                    if let Some(ref body) = info.body {
                        println!("\n  Release notes:");
                        for line in body.lines().take(5) {
                            if !line.trim().is_empty() { println!("     {}", line); }
                        }
                    }
                    println!();
                    if args.check {
                        print_info("Run 'kwaainet update' (without --check) to install");
                    } else {
                        print_info("Download the latest release from the URL above to update");
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
            println!("    Memory: {} total / {} available",
                format_bytes(hw.total_memory), format_bytes(hw.available_memory));
            println!("    CPU cores: {}", hw.cpu_cores);
            println!();

            let profile = engine.calibrate(&model);
            println!("  Total model blocks: {}", profile.total_blocks);
            println!();
            println!("  Recommendations:");
            println!("    🔹 Minimum:     {} blocks", profile.min_blocks);
            println!("    ⭐ Recommended: {} blocks", profile.recommended_blocks);
            println!("    🔸 Maximum:     {} blocks", profile.max_blocks);
            print_separator();

            if let Some(ref apply) = args.apply {
                if let Some(new_blocks) = profile.get_blocks(apply) {
                    let mut cfg = KwaaiNetConfig::load_or_create()?;
                    cfg.blocks = new_blocks;
                    cfg.save()?;
                    print_success(&format!("Applied {} profile: blocks = {}", apply, new_blocks));
                    print_info("Restart the node to use the new setting: kwaainet restart");
                } else {
                    print_error(&format!("Unknown profile '{}'. Use: min, recommended, or max", apply));
                }
            } else {
                print_info(&format!("Apply recommended: kwaainet calibrate --apply recommended"));
            }
        }

        // -------------------------------------------------------------------
        // load-model
        // -------------------------------------------------------------------
        Command::LoadModel(args) => {
            print_box_header("📦 KwaaiNet Model Loader");
            println!("  Model ref: {}", args.model);
            println!();

            // Resolve Ollama model reference → GGUF blob path
            let blob_path = match ollama::resolve_model_blob(&args.model) {
                Ok(p) => p,
                Err(e) => {
                    print_error(&format!("{e}"));
                    return Ok(());
                }
            };

            let file_size = std::fs::metadata(&blob_path)
                .map(|m| m.len())
                .unwrap_or(0);

            println!("  Blob:   {}", blob_path.display());
            println!("  Size:   {}", format_bytes(file_size));
            println!();
            println!("  Loading weights into memory…");

            let start = std::time::Instant::now();

            let mut engine = match InferenceEngine::new(EngineConfig::default()) {
                Ok(e) => e,
                Err(e) => {
                    print_error(&format!("Failed to create inference engine: {e}"));
                    return Ok(());
                }
            };

            match engine.load_model(&blob_path, ModelFormat::Gguf) {
                Ok(handle) => {
                    let elapsed = start.elapsed();
                    let info = engine.model_info(&handle)
                        .expect("handle was just created");

                    print_success(&format!("Loaded in {:.1}s", elapsed.as_secs_f32()));
                    println!();
                    println!("  Architecture:  {}", info.architecture);
                    println!("  Vocab size:    {}", info.vocab_size);
                    println!("  Context:       {} tokens", info.context_length);
                    println!("  Memory usage:  {}", format_bytes(info.memory_bytes as u64));
                    println!("  Quantized:     {}", info.is_quantized);
                }
                Err(e) => {
                    print_error(&format!("Failed to load model: {e}"));
                }
            }
            print_separator();
        }

        // -------------------------------------------------------------------
        // setup
        // -------------------------------------------------------------------
        Command::Setup => {
            print_box_header("🔧 KwaaiNet Setup");
            let cfg = KwaaiNetConfig::load_or_create()?;

            // Create all required directories
            std::fs::create_dir_all(config::run_dir())?;
            std::fs::create_dir_all(config::log_dir())?;

            print_success("Directories created");
            print_success(&format!("Config written to {}", config::config_file().display()));
            println!();
            println!("  Model:  {}", cfg.model);
            println!("  Blocks: {}", cfg.blocks);
            println!("  Port:   {}", cfg.port);
            println!();
            print_info("Start the node with: kwaainet start --daemon");
            print_separator();
        }
    }

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
