//! The native (in-process rust-libp2p) node runner.
//!
//! Selected by `native_p2p = true` in the node config. Assembles the pieces
//! Phases 1–3 built into the same node `run_node` produces with the Go daemon:
//!
//! ```text
//!                       ┌──────────────── kwaainet run-node ────────────────┐
//!   external clients ──▶│ ControlServer ──┐                                 │
//!   (GUI, shard serve,  │                 ├──▶ NetworkHandle ──▶ swarm ──── │──▶ libp2p
//!    p2p subcommands)   │ node handlers ──┘         ▲                       │
//!                       │ (hello, proxies, mux)     │                       │
//!                       │ DHT service ──────────────┘                       │
//!                       └───────────────────────────────────────────────────┘
//! ```
//!
//! # What differs from the p2pd path
//!
//! * **No child process**, so no watchdog, no crash restart, no
//!   `restart_p2pd*`, and no `find_p2pd_binary` — a native node runs with no
//!   `p2pd` binary installed at all.
//! * **The RPC listener is gone.** The p2pd path binds a loopback TCP listener
//!   and registers it as a *stream handler* for the three `DHTProtocol.rpc_*`
//!   names, so p2pd forwards each inbound DHT request over TCP with a
//!   `StreamInfo` prologue and a `PersistentConnectionRequest` wrapper around
//!   the payload. Natively, `spawn_dht_service` registers them as unary
//!   handlers directly on the swarm — same protocol IDs, same `DHTStorage`,
//!   two fewer hops and no wrapper to unwrap.
//! * **No IDENTIFY-driven restart cycle.** The p2pd path re-spawns the daemon
//!   to change its announce addresses. Address discovery and re-announce on
//!   change belong to the NAT slice; until then a native node announces what
//!   `announce_addr`/`public_ip` say, or nothing.
//!
//! # What is deliberately identical
//!
//! The **peer ID** (same key file, same libp2p protobuf encoding), the
//! **control socket** (`ControlServer` on the path `KWAAINET_SOCKET` or
//! `DEFAULT_SOCKET_NAME` names, which every external client already dials), the
//! **announce records** (`announce.rs`, byte-for-byte), the **re-announce
//! cadence** (300 s ± 30 s jitter, 360 s TTL), the **tombstone on shutdown**,
//! and the **protocol IDs** of every handler.
//!
//! # Not yet here — the NAT slice
//!
//! AutoNAT, circuit relay, DCUtR and UPnP are p2pd-only. A native node is
//! therefore reachable only if it is directly dialable: it can always *call*
//! out (so it announces, stores and finds fine from behind a NAT), but inbound
//! connections need a public address. `no_relay`, `force_private` and
//! `trusted_relays` have no effect on this path.

use anyhow::{Context, Result};
use kwaai_hivemind_dht::DHTStorage;
use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use kwaai_p2p_daemon::ControlServer;
use libp2p::PeerId;
use std::time::Duration;
use tracing::{info, warn};

use crate::announce::{
    build_announce_records, build_unannounce_records, send_records_via_handle, AnnounceContext,
    DHTServerInfo, StoreTiming,
};
use crate::config::KwaaiNetConfig;
use crate::daemon::ShardManager;
use crate::node::SigHup;

/// Per-request budget on outbound unary calls.
///
/// Matches the `tokio::time::timeout(30s, …)` the p2pd path wraps every
/// `call_unary_handler` in. The native handle enforces it itself
/// (`NetworkConfig::request_timeout`), so `send_records_via_handle` needs no
/// timeout of its own.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Everything the native path starts, kept together so shutdown is one call and
/// nothing can be forgotten.
///
/// Field order matters on drop only for the control socket, which is removed by
/// `ControlServer`'s own `Drop`; the rest are tasks that end when the service
/// drops their channels.
pub struct NativeNode {
    /// The swarm handle — the outbound half of everything.
    pub handle: NetworkHandle,
    /// This node's peer ID, identical to what the p2pd path would report.
    pub peer_id: PeerId,
    /// The DHT records this node serves to other peers.
    pub storage: DHTStorage,
    /// Background tasks: swarm loop, DHT maintenance, control socket, mux.
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl NativeNode {
    /// Build the libp2p identity, start the swarm, serve the DHT, bind the
    /// control socket, and register this node's own protocol handlers.
    ///
    /// Does **not** announce — the caller owns announce timing because it needs
    /// throughput measurement, VPK health and block range first.
    ///
    /// # Identity
    ///
    /// The keypair comes from the same file the p2pd path passes to `-id`
    /// (`config.identity_key`, else `~/.kwaainet/identity.key`) through the same
    /// libp2p protobuf encoding, so the peer ID is identical on both paths. A
    /// missing file is generated exactly as `NodeIdentity::load_or_create`
    /// would; an explicitly configured path is *not* generated, so a typo fails
    /// loudly instead of silently minting a new identity.
    pub async fn start(config: &KwaaiNetConfig, bootstrap_peers: &[String]) -> Result<Self> {
        let key_path = config
            .identity_key
            .clone()
            .unwrap_or_else(crate::identity::NodeIdentity::key_file_path);
        let keypair = if config.identity_key.is_some() {
            kwaai_p2p::identity::load_keypair(&key_path)
                .with_context(|| format!("loading node identity from {}", key_path.display()))?
        } else {
            kwaai_p2p::identity::load_or_generate(&key_path).context("loading node identity")?
        };
        let peer_id = keypair.public().to_peer_id();

        // The same listen address the p2pd path passes as `-hostAddrs`.
        let net_config = NetworkConfig {
            listen_addrs: vec![format!("/ip4/0.0.0.0/tcp/{}", config.port)],
            bootstrap_peers: bootstrap_peers.to_vec(),
            request_timeout: REQUEST_TIMEOUT,
            port: config.port,
            initial_peers: bootstrap_peers.to_vec(),
            // p2pd runs with `-b` (Kademlia bootstrap) on every node, which is
            // what makes a node answer DHT queries rather than only issuing
            // them. `dht_server` is the native equivalent.
            dht_server: true,
            ..NetworkConfig::default()
        };

        let (handle, swarm_task) =
            NetworkService::spawn(net_config, keypair).context("starting the libp2p swarm")?;
        info!("Peer ID: {}", peer_id.to_base58());

        let mut tasks = vec![swarm_task];

        // ── DHT serving ────────────────────────────────────────────────────
        // Bootstrap-grade: this node answers rpc_store/rpc_find/rpc_ping for
        // other peers, which is what p2pd's `-b` DHT serving provided.
        let storage = DHTStorage::new(peer_id);
        tasks.push(
            kwaai_p2p::spawn_dht_service(handle.clone(), storage.clone())
                .await
                .context("starting the native DHT service")?,
        );
        info!("Hivemind DHT service registered (rpc_ping/rpc_store/rpc_find)");

        // ── Bootstrap ──────────────────────────────────────────────────────
        // Dials the configured peers and seeds Kademlia. Non-fatal: a node with
        // no reachable bootstrap still serves its own DHT and control socket,
        // and the announce loop retries every 300 s.
        if bootstrap_peers.is_empty() {
            warn!("No bootstrap peers configured — this node will not join the network");
        } else {
            let addrs = bootstrap_peers
                .iter()
                .filter_map(|a| match a.parse() {
                    Ok(ma) => Some(ma),
                    Err(e) => {
                        warn!("Skipping unparseable bootstrap address {a}: {e}");
                        None
                    }
                })
                .collect::<Vec<_>>();
            match handle.bootstrap(addrs).await {
                Ok(()) => info!("Bootstrapped to {} peer(s)", bootstrap_peers.len()),
                Err(e) => warn!("Bootstrap failed: {e} — will retry via the announce loop"),
            }
        }

        // ── Node protocol handlers ─────────────────────────────────────────
        register_node_handlers(&handle).await;
        match crate::inference_mux::start_native_inference_mux_server(&handle).await {
            Ok(task) => tasks.push(task),
            Err(e) => warn!("inference-mux server registration failed: {e:#}"),
        }

        // ── Control socket ─────────────────────────────────────────────────
        // Last, so external clients never observe a node whose handlers are
        // only half-registered. Bound at the same address p2pd would have used,
        // so the GUI, `kwaainet p2p …`, `shard serve` and the map crawler are
        // unchanged.
        let socket_addr = control_socket_addr();
        let server = ControlServer::bind(&socket_addr, handle.clone())
            .await
            .with_context(|| format!("binding the control socket at {socket_addr}"))?;
        info!("Control socket listening at {socket_addr}");
        tasks.push(tokio::spawn(server.run()));

        Ok(Self {
            handle,
            peer_id,
            storage,
            tasks,
        })
    }

    /// Push one announcement round to every bootstrap peer, and into our own
    /// storage so we serve what we publish.
    ///
    /// Returns the per-bootstrap timings `node.rs` feeds to the reputation
    /// store — the announce doubles as the reputation probe on both paths.
    pub async fn announce(
        &self,
        ctx: &AnnounceContext<'_>,
        server_info: &DHTServerInfo,
        bootstrap_peers: &[String],
    ) -> Result<Vec<crate::announce::StoreTiming>> {
        let records = build_announce_records(ctx, server_info)?;
        for record in &records {
            self.storage.handle_store(record.clone());
        }
        let (ok, timings) = send_records_via_handle(&self.handle, bootstrap_peers, &records).await;
        if ok {
            info!(
                "✅ Announced {} blocks",
                server_info.end_block - server_info.start_block
            );
        } else {
            warn!("❌ Announcement failed — node will not appear on map");
        }
        Ok(timings)
    }

    /// Write the `state = -1` tombstone so the map drops this node immediately
    /// rather than waiting out the 360 s TTL.
    pub async fn unannounce(
        &self,
        ctx: &AnnounceContext<'_>,
        server_info: &DHTServerInfo,
        bootstrap_peers: &[String],
    ) {
        let records = match build_unannounce_records(ctx, server_info) {
            Ok(r) => r,
            Err(e) => {
                warn!("Unannounce: failed to build records: {e}");
                return;
            }
        };
        for record in &records {
            self.storage.handle_store(record.clone());
        }
        send_records_via_handle(&self.handle, bootstrap_peers, &records).await;
        info!("Unannounced from DHT — node removed from map");
    }

    /// Stop the swarm and every task hanging off it.
    ///
    /// The control socket's listener is closed and its socket file removed by
    /// `ControlServer`'s `Drop`; the DHT maintenance loop and the mux accept
    /// loop end on their own once the service drops their channels.
    pub async fn shutdown(self) {
        if let Err(e) = self.handle.shutdown().await {
            warn!("Network service shutdown: {e}");
        }
        for task in self.tasks {
            task.abort();
        }
    }
}

/// Run the node on the native stack: start everything, announce, then serve
/// until a shutdown signal.
///
/// Called from `run_node` after its shared prologue (PID file, SIGHUP
/// registration, gRPC surface, identity, credentials, bootstrap-peer
/// resolution), so this owns only what is genuinely path-specific. Returns when
/// the node has unannounced and stopped; `run_node` does the PID cleanup and
/// the deferred auto-update respawn exactly as it does for the p2pd path.
///
/// # Loop arms vs the p2pd path
///
/// | arm | native |
/// | --- | --- |
/// | inbound DHT RPC | **gone** — served in-swarm by `spawn_dht_service`, not over a forwarded TCP stream |
/// | SIGHUP re-announce | same |
/// | 300 s ± 30 s re-announce | same, minus the daemon watchdog that gated it |
/// | periodic IDENTIFY + restart | **deferred to the NAT slice** |
/// | 10 s p2pd heartbeat | **gone** — no child process to outlive us |
/// | 60 s relay keepalive | **gone** — no unix socket to a child, no relay circuit yet |
/// | Ollama recovery | same |
/// | shutdown | same |
pub async fn run_native_node(
    config: &KwaaiNetConfig,
    bootstrap_peers: &[String],
    public_name: &str,
    trust_attestations: Vec<String>,
    sighup: &mut SigHup,
) -> Result<Option<String>> {
    info!("[1/4] Starting the native p2p stack...");
    let node = NativeNode::start(config, bootstrap_peers).await?;
    let peer_id = node.peer_id;

    // ── Announce inputs ────────────────────────────────────────────────────
    info!("[2/4] Preparing the DHT announcement...");

    // Without NAT traversal a native node is "direct" when it has an address to
    // advertise and unreachable otherwise; there is no relay circuit that could
    // make the middle case ("reachable, but only via a relay") true yet. The
    // p2pd path's `all_addrs_are_relay` check over IDENTIFY-discovered addresses
    // has no native counterpart until the NAT slice lands, so this reports
    // relay=false whenever an address is configured and relay=true when none is
    // — the same "no usable address means don't claim Direct" rule.
    let announce_addr = configured_announce_addr(config);
    let using_relay = announce_addr.is_none();
    if let Some(ref addr) = announce_addr {
        info!("  Announce addr: {addr}");
    } else {
        warn!(
            "No announce_addr/public_ip configured — a native node has no NAT traversal yet, so \
             it will only be reachable if its listen address is directly dialable"
        );
    }

    let dl_bps = crate::node::measure_download_bps_for(&config.model).await;
    let throughput = crate::node::report_effective_tps(&config.model, dl_bps, using_relay);
    let prefix = config.effective_dht_prefix();
    let repository = crate::node::effective_repository(config);
    info!("  DHT prefix:  {}", prefix);
    info!("  Repository:  {}", repository);
    info!("  Using relay: {}", using_relay);

    let vpk_info = crate::node::initial_vpk_info(config, public_name).await;

    let ctx = AnnounceContext {
        peer_id,
        prefix: &prefix,
        repository: &repository,
        total_blocks: config.model_total_blocks(),
    };

    let mut config = config.clone();
    let mut server_info = DHTServerInfo::new(
        config.start_block as i32,
        config.effective_end_block() as i32,
        public_name,
        using_relay,
        throughput,
        trust_attestations,
        vpk_info,
        peer_id.to_base58(),
    );

    // ── Initial announcement ───────────────────────────────────────────────
    info!("[3/4] Announcing to DHT...");
    if let Err(e) = node.announce(&ctx, &server_info, bootstrap_peers).await {
        warn!("Initial announce failed: {e:#} — will retry at the 300 s tick");
    }

    info!("[4/4] ✅ KwaaiNet node running (native p2p)");
    info!("   Peer ID : {}", peer_id.to_base58());
    info!("   Name    : {}", public_name);
    info!("   Model   : {}", config.model);
    info!(
        "   Blocks  : {}–{}",
        config.start_block,
        config.effective_end_block()
    );
    info!("   Map     : https://map.kwaai.ai");

    // ── Event loop ─────────────────────────────────────────────────────────
    // Same 300 s ± 30 s jittered cadence as the p2pd path: the DHT TTL is 360 s,
    // so every record keeps at least 30 s of headroom, and the jitter stops a
    // mass restart from thundering-herding the bootstraps.
    let mut rep_store = crate::reputation::ReputationStore::load();
    let mut next_announce = Box::pin(tokio::time::sleep(Duration::from_secs(
        crate::node::jitter_secs(300, 30),
    )));
    let mut ollama_recovery_rx = crate::node::spawn_ollama_watcher(&config);
    let mut pending_update_version: Option<String> = None;

    loop {
        tokio::select! {
            // SIGHUP (Unix) — re-read config and re-announce. `shard serve`
            // signals a block-range change this way.
            _ = sighup.recv() => {
                info!("SIGHUP received — re-reading config and re-announcing");
                reload_block_range(&mut config);
                refresh_server_info(&mut server_info, &config);
                if let Err(e) = node.announce(&ctx, &server_info, bootstrap_peers).await {
                    warn!("Re-announce after SIGHUP failed: {e:#}");
                }
            }

            // Periodic re-announcement.
            _ = &mut next_announce => {
                reload_block_range(&mut config);
                #[cfg(not(unix))]
                {
                    let flag = crate::config::run_dir().join("reannounce.flag");
                    if flag.exists() {
                        let _ = std::fs::remove_file(&flag);
                    }
                }

                crate::node::refresh_throughput(&mut server_info, &config.model, dl_bps, using_relay);
                crate::node::refresh_vpk_info(&mut server_info, &config, public_name).await;

                // Auto-update — installs a new binary when available (pre-v1.0)
                // and breaks the loop so the respawn happens after our own
                // cleanup. Identical to the p2pd path.
                let auto_update = KwaaiNetConfig::load_or_create()
                    .map(|c| c.contribute_policy(false).auto_update)
                    .unwrap_or(false);
                if auto_update {
                    if let Some(version) = crate::node::maybe_auto_update().await {
                        pending_update_version = Some(version);
                        break;
                    }
                }

                refresh_server_info(&mut server_info, &config);
                info!(
                    "Re-announcing to DHT (shard_ready={})...",
                    ShardManager::shard_is_ready()
                );
                match node.announce(&ctx, &server_info, bootstrap_peers).await {
                    Ok(timings) => record_reputation(&mut rep_store, timings),
                    Err(e) => warn!("Re-announce failed: {e:#}"),
                }

                next_announce
                    .as_mut()
                    .reset(tokio::time::Instant::now()
                        + Duration::from_secs(crate::node::jitter_secs(300, 30)));
            }

            // Ollama came back up — re-announce immediately so clients learn the
            // host is usable again without waiting out the 300 s tick.
            Some(()) = ollama_recovery_rx.recv() => {
                info!("Ollama recovered — triggering immediate re-announce");
                refresh_server_info(&mut server_info, &config);
                if let Err(e) = node.announce(&ctx, &server_info, bootstrap_peers).await {
                    warn!("Re-announce after Ollama recovery failed: {e:#}");
                }
            }

            _ = crate::node::shutdown_signal() => {
                info!("Shutdown signal received");
                break;
            }
        }
    }

    // Tombstone before tearing down the swarm, so the map drops us immediately
    // rather than waiting out the TTL.
    info!("Unannouncing from DHT...");
    node.unannounce(&ctx, &server_info, bootstrap_peers).await;
    node.shutdown().await;

    Ok(pending_update_version)
}

/// The address this node advertises, or `None` when it has none configured.
///
/// Same precedence as the p2pd path: an explicit `announce_addr` multiaddr
/// wins, else `public_ip` formatted with `public_port` (for port-forwarded
/// deployments) or the listen `port`. An empty `public_ip` string means "no
/// public IP", not "the empty address".
fn configured_announce_addr(config: &KwaaiNetConfig) -> Option<String> {
    config.announce_addr.clone().or_else(|| {
        let port = config.public_port.unwrap_or(config.port);
        config
            .public_ip
            .as_deref()
            .filter(|ip| !ip.is_empty())
            .map(|ip| format!("/ip4/{ip}/tcp/{port}"))
    })
}

/// Re-read the on-disk config for a block-range change written by
/// `shard serve` (via `signal_reannounce`) or `kwaainet config set`.
fn reload_block_range(config: &mut KwaaiNetConfig) {
    let Ok(fresh) = KwaaiNetConfig::load_or_create() else {
        return;
    };
    if fresh.start_block == config.start_block && fresh.blocks == config.blocks {
        return;
    }
    info!(
        "Block range updated: [{}–{}) → [{}–{})",
        config.start_block,
        config.effective_end_block(),
        fresh.start_block,
        fresh.start_block + fresh.blocks,
    );
    config.start_block = fresh.start_block;
    config.blocks = fresh.blocks;
}

/// Sync the announced block range and readiness state from the live config.
fn refresh_server_info(server_info: &mut DHTServerInfo, config: &KwaaiNetConfig) {
    server_info.start_block = config.start_block as i32;
    server_info.end_block = config.effective_end_block() as i32;
    server_info.state = if ShardManager::shard_is_ready() { 2 } else { 0 };
}

/// Fold one announce round's per-bootstrap timings into the reputation store.
///
/// The announce doubles as the reputation probe on both paths — no extra RPCs.
/// The display name is the `/dns/` hostname from the bootstrap multiaddr when
/// there is one, else a truncated peer ID.
fn record_reputation(rep: &mut crate::reputation::ReputationStore, timings: Vec<StoreTiming>) {
    use crate::reputation::{now_secs, PeerObservation};
    for (peer_id_str, addr, latency_ms, success) in timings {
        let name = addr
            .split('/')
            .collect::<Vec<_>>()
            .windows(2)
            .find(|w| w[0] == "dns" || w[0] == "dns4" || w[0] == "dns6")
            .map(|w| w[1].to_string())
            .unwrap_or_else(|| peer_id_str[..peer_id_str.len().min(12)].to_string());
        rep.record(
            &peer_id_str,
            &name,
            PeerObservation {
                timestamp_secs: now_secs(),
                latency_ms,
                success,
                observed_tps: None,
                claimed_tps: None,
            },
        );
    }
}

/// The control-socket multiaddr this node binds.
///
/// Resolved exactly as every client resolves it — `KWAAINET_SOCKET` if set,
/// else the platform default — so `KWAAINET_SOCKET=… kwaainet run-node` and
/// `KWAAINET_SOCKET=… kwaainet p2p peers list` still meet on the same path, and
/// two nodes on one machine still get separate sockets.
fn control_socket_addr() -> String {
    match std::env::var("KWAAINET_SOCKET") {
        Ok(sock) => {
            #[cfg(unix)]
            {
                format!("/unix/{sock}")
            }
            #[cfg(not(unix))]
            {
                sock
            }
        }
        Err(_) => kwaai_p2p_daemon::default_socket_addr(),
    }
}

/// Register the node's own "while we're alive, please answer these" protocols.
///
/// The same three the p2pd path registers over the control socket, with the
/// same protocol IDs and the same handler bodies — only the registration call
/// differs. Failures are warnings, not errors, matching the p2pd path: a node
/// that cannot serve its Ollama proxy is still a useful DHT peer.
async fn register_node_handlers(handle: &NetworkHandle) {
    let hello = kwaai_p2p_daemon::hello::make_handler();
    if let Err(e) = handle
        .add_unary_handler(kwaai_p2p_daemon::hello::HELLO_PROTO, move |data| {
            let fut = hello(data);
            async move { fut.await.map_err(|e| e.to_string()) }
        })
        .await
    {
        warn!("registering p2p hello handler failed: {e}");
    }

    let ollama = crate::ollama_proxy::make_ollama_proxy_handler();
    if let Err(e) = handle
        .add_unary_handler(crate::ollama_proxy::OLLAMA_PROXY_PROTO, move |data| {
            let fut = ollama(data);
            async move { fut.await.map_err(|e| e.to_string()) }
        })
        .await
    {
        warn!("registering ollama-proxy handler failed: {e}");
    }

    let shard = crate::ollama_proxy::make_shard_proxy_handler();
    if let Err(e) = handle
        .add_unary_handler(crate::ollama_proxy::SHARD_PROXY_PROTO, move |data| {
            let fut = shard(data);
            async move { fut.await.map_err(|e| e.to_string()) }
        })
        .await
    {
        warn!("registering shard-proxy handler failed: {e}");
    }
}
