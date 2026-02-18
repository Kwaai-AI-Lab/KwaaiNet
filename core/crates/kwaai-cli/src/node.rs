//! Native Rust node runner
//!
//! Starts a KwaaiNet / Petals-compatible node using:
//!   - kwaai-p2p-daemon  (go-libp2p-daemon for transport)
//!   - kwaai-p2p         (KwaaiNetwork with Kademlia DHT)
//!   - kwaai-hivemind-dht (Hivemind protocol server-info announcements)
//!
//! This replaces the Python `python -m petals.cli.run_server` call.

use anyhow::{Context, Result};
use std::time::Duration;
use tokio::signal;
use tracing::{info, warn};

use kwaai_p2p::{KwaaiNetwork, NetworkConfig, ServerInfo};

use crate::config::KwaaiNetConfig;
use crate::daemon::DaemonManager;

/// Run the node in the foreground until SIGTERM/SIGINT.
pub async fn run_node(config: &KwaaiNetConfig) -> Result<()> {
    let daemon_mgr = DaemonManager::new();
    let pid = std::process::id();
    daemon_mgr.write_pid(pid).context("writing PID file")?;
    info!("KwaaiNet node starting (PID {})", pid);

    let port = find_free_port(config.port).unwrap_or(config.port);
    if port != config.port {
        warn!("Port {} busy, using {}", config.port, port);
    }

    info!(
        model = %config.model,
        blocks = config.blocks,
        port,
        "Configuring KwaaiNet node"
    );

    // --- P2P Daemon (go-libp2p-daemon) ---
    let p2pd_path = find_p2pd_binary();
    let mut p2p_daemon_opt = None;

    if let Some(ref path) = p2pd_path {
        info!("Starting p2pd at {}", path.display());
        match kwaai_p2p_daemon::P2PDaemon::builder()
            .with_binary_path(path)
            .with_listen_addr(format!("/ip4/0.0.0.0/tcp/{}", port))
            .dht(true)
            .bootstrap_peers(config.initial_peers.clone())
            .spawn()
            .await
        {
            Ok(d) => {
                info!("p2pd daemon started, listening at {}", d.listen_addr());
                p2p_daemon_opt = Some(d);
            }
            Err(e) => {
                warn!("Could not start p2pd: {}. Continuing with libp2p-only mode.", e);
            }
        }
    } else {
        info!("p2pd binary not found; running in libp2p-only mode");
    }

    // --- KwaaiNetwork (libp2p swarm with Kademlia DHT) ---
    let mut net_cfg = NetworkConfig::with_petals_bootstrap();
    net_cfg.listen_addrs = vec![format!("/ip4/0.0.0.0/tcp/{}", port + 1)];
    if !config.initial_peers.is_empty() {
        net_cfg.bootstrap_peers = config.initial_peers.clone();
    }

    let mut network = KwaaiNetwork::new(net_cfg)
        .await
        .context("creating KwaaiNetwork")?;

    let local_peer_id = network.local_peer_id();
    info!("Local peer ID: {}", local_peer_id);

    // Build server info for Hivemind DHT announcement
    let server_info = ServerInfo::new(
        config.public_name.clone().unwrap_or_else(|| "kwaainet-node".to_string()),
    )
    .with_span(0, config.blocks)
    .with_cache_tokens(32768)
    .with_throughput(10.0)
    .with_relay(!config.no_relay);

    // Start the network
    network.start().await.context("starting KwaaiNetwork")?;

    // Announce blocks to DHT
    network
        .announce_blocks(&config.model, &server_info)
        .await
        .context("announcing blocks to DHT")?;

    info!("✅ KwaaiNet node running");
    info!("   Peer ID:  {}", local_peer_id);
    info!("   Model:    {}", config.model);
    info!("   Blocks:   0–{}", config.blocks);
    if let Some(ref name) = config.public_name {
        info!("   Name:     {}", name);
    }

    // Run DHT event loop in background
    let mut network_task = tokio::spawn(async move {
        loop {
            if let Err(e) = network.process_dht_command().await {
                warn!("DHT command error: {}", e);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });

    // Wait for shutdown signal
    wait_for_shutdown(&mut network_task).await;

    // Cleanup
    if let Some(mut d) = p2p_daemon_opt {
        info!("Stopping p2pd");
        let _ = d.shutdown().await;
    }
    daemon_mgr.remove_pid();
    info!("KwaaiNet node stopped");
    Ok(())
}

// ---------------------------------------------------------------------------
// Signal handling
// ---------------------------------------------------------------------------

async fn wait_for_shutdown(task: &mut tokio::task::JoinHandle<()>) {
    #[cfg(unix)]
    {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("SIGTERM handler");
        tokio::select! {
            _ = signal::ctrl_c() => { info!("Received Ctrl-C, shutting down"); }
            _ = sigterm.recv() => { info!("Received SIGTERM, shutting down"); }
            _ = &mut *task => { warn!("Network task ended unexpectedly"); }
        }
    }
    #[cfg(not(unix))]
    {
        tokio::select! {
            _ = signal::ctrl_c() => { info!("Received Ctrl-C, shutting down"); }
            _ = &mut *task => { warn!("Network task ended unexpectedly"); }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn find_free_port(preferred: u16) -> Option<u16> {
    if port_is_free(preferred) { return Some(preferred); }
    for port in (preferred + 1)..=(preferred + 100) {
        if port_is_free(port) { return Some(port); }
    }
    None
}

fn port_is_free(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}

fn find_p2pd_binary() -> Option<std::path::PathBuf> {
    // Next to our own binary (deployed alongside kwaainet)
    if let Ok(exe) = std::env::current_exe() {
        let candidate = exe.parent()?.join("p2pd");
        if candidate.exists() { return Some(candidate); }
    }
    // In Cargo target dir (dev builds)
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let candidate = std::path::PathBuf::from(manifest)
            .join("../../../target/debug/p2pd");
        if candidate.exists() { return Some(candidate); }
    }
    // In PATH
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join("p2pd");
        if candidate.exists() { return Some(candidate); }
    }
    None
}
