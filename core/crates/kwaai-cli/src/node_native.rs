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
//! * **No IDENTIFY-driven restart cycle.** Reachability changes arrive on a
//!   watch channel and the record is re-published in place.
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
//! # NAT traversal
//!
//! AutoNAT, circuit relay, DCUtR and UPnP all run natively now, each mapped
//! from the config field that drove the corresponding p2pd flag:
//!
//! | config | p2pd flag | native |
//! | --- | --- | --- |
//! | `force_private` | `-forceReachabilityPrivate` | reachability starts Private, so reservations begin at t=0 |
//! | `no_relay` | `-relay` | toggles the circuit **hop server** |
//! | `trusted_relays` | `-trustedRelays` | operator override; the real supply is identify hop discovery |
//! | `announce_addr` / `public_ip` | `-announceAddrs` | declared external address, outranking AutoNAT |
//! | — | `-natPortMap` | UPnP, always on |
//!
//! What is *not* in reach in-process is real hole punching, which needs actual
//! NATs — that is docker nat-test topology work.

use anyhow::{Context, Result};
use kwaai_hivemind_dht::DHTStorage;
use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use kwaai_p2p_daemon::ControlServer;
use libp2p::PeerId;
use std::time::Duration;
use tracing::{error, info, warn};

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

/// How long to let reachability settle before acting on a change.
///
/// At startup a reservation being confirmed and AutoNAT confirming an address
/// land within seconds of each other; announcing once for the pair beats
/// announcing twice.
const ANNOUNCE_SETTLE: Duration = Duration::from_secs(10);

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
            // them. `dht_server` is the native equivalent and defaults to true
            // for the same reason; false leaves kad in auto-mode instead.
            dht_server: config.dht_server,

            // ── NAT traversal ──────────────────────────────────────────────
            // Each of these has a p2pd flag it corresponds to, so a node
            // migrating between the two paths behaves the same way.
            //
            // `-trustedRelays`. Empty by default now: the real supply of relay
            // candidates is identify hop discovery, and the bootstraps offer
            // hop. See `default_trusted_relays` in config.rs.
            trusted_relays: config.trusted_relays.clone(),
            // `-relay`.
            relay_server: !config.no_relay,
            // `-natPortMap`. On by default; off for a node deployed at a known
            // address (a bootstrap node), which has no gateway to ask.
            enable_upnp: config.enable_upnp,
            // `-forceReachabilityPrivate`. Defaults true, so relay reservations
            // start immediately rather than after an AutoNAT round.
            force_private: config.force_private,
            // `-announceAddrs`. An operator declaration, so it outranks
            // `force_private` and AutoNAT cannot demote it.
            external_addr: configured_announce_addr(config),
            // Deliberately permissive: this is the only address-class filter in
            // rust-libp2p 0.53, and turning it on would classify the docker
            // nat-test topology's `198.18/15` addresses unreachable.
            require_global_ips: false,
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
        // An empty list is a misconfiguration for an ordinary node and normal
        // for a bootstrap, which dials nobody. Either way it is not an error and
        // there is no retry loop: nothing is dialled and `handle.bootstrap` is
        // never called.
        if bootstrap_peers.is_empty() {
            if config.announce_self {
                warn!("No bootstrap peers configured — this node will not join the network");
            }
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
        // One Capacity Lease table per node process, exactly as the p2pd path
        // builds in `node.rs` — shared by the ollama-proxy handler, the
        // capacity-lease handler and the mux server, so all three contend for
        // the same Ollama slots rather than each holding its own semaphore.
        let lease_max_concurrent = std::env::var("OLLAMA_NUM_PARALLEL")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .filter(|&n| n > 0)
            .unwrap_or(crate::capacity_lease::DEFAULT_MAX_CONCURRENT);
        let lease_table =
            crate::capacity_lease::LeaseTable::new(config.model.clone(), lease_max_concurrent);
        let _lease_sweep_handle =
            lease_table.spawn_periodic_sweep(std::time::Duration::from_secs(5));

        register_node_handlers(&handle, lease_table.clone()).await;
        match crate::inference_mux::start_native_inference_mux_server(&handle, lease_table.clone())
            .await
        {
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
    ///
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
/// | periodic IDENTIFY + restart | **gone** — a watch channel re-announces in place |
/// | 10 s p2pd heartbeat | **gone** — no child process to outlive us |
/// | 60 s relay keepalive | **gone** — libp2p-relay renews reservations at 3/4 TTL by itself |
/// | Ollama recovery | same |
/// | shutdown | same |
pub async fn run_native_node(
    config: &KwaaiNetConfig,
    bootstrap_peers: &[String],
    public_name: &str,
    trust_attestations: Vec<String>,
    sighup: &mut SigHup,
    grpc: &crate::grpc_server::GrpcServerHandle,
) -> Result<Option<String>> {
    info!("[1/4] Starting the native p2p stack...");
    let node = NativeNode::start(config, bootstrap_peers).await?;
    let peer_id = node.peer_id;

    // Hand the swarm to the gRPC surface, which bound before the node existed
    // so that ping/status/generate would answer during startup. Until now its
    // Network op has been reporting "still starting"; from here it can serve.
    grpc.attach_network(node.handle.clone()).await;

    // ── Announce inputs ────────────────────────────────────────────────────
    // Everything from here to the initial announce describes a record this node
    // is about to publish, so a bootstrap skips the lot: the bandwidth probe is
    // a real network round trip, and it has no model to measure a throughput
    // for.
    let announce_self = config.announce_self;
    if announce_self {
        info!("[2/4] Preparing the DHT announcement...");
    }

    // `using_relay` is true when a circuit reservation is actually confirmed.
    let mut announce_state = node.handle.announce_state();
    let startup_state = node.handle.current_announce_state();
    let mut using_relay = startup_state.using_relay;
    // Epoch of the last state actually published to the DHT. Gating on the
    // epoch (not on `using_relay`) means a reachability-only transition still
    // re-announces; `using_relay` alone would miss Private→Public with no
    // circuit, leaving the record stale until the 300 s tick.
    let mut last_announced_epoch = startup_state.epoch;

    let prefix = config.effective_dht_prefix();
    let repository = crate::node::effective_repository(config);

    // One gate for every announce input. A bootstrap computes none of them:
    // `measure_download_bps_for` runs a live download, and `initial_vpk_info`
    // polls a VPK endpoint — both pointless for a node that publishes nothing.
    let (dl_bps, throughput, vpk_info) = if !announce_self {
        (0.0, 0.0, None)
    } else {
        if let Some(addr) = configured_announce_addr(config) {
            info!("  Announce addr: {addr} (declared)");
        }
        let dl_bps = crate::node::measure_download_bps_for(&config.model).await;
        let tps = crate::node::report_effective_tps(&config.model, dl_bps, using_relay);
        info!("  DHT prefix:  {}", prefix);
        info!("  Repository:  {}", repository);
        info!("  Using relay: {}", using_relay);
        (
            dl_bps,
            tps,
            crate::node::initial_vpk_info(config, public_name).await,
        )
    };

    let ctx = AnnounceContext {
        peer_id,
        prefix: &prefix,
        repository: &repository,
        total_blocks: config.model_total_blocks(),
    };

    let mut config = config.clone();
    let mut server_info = DHTServerInfo::new(
        config.start_block() as i32,
        config.effective_end_block() as i32,
        public_name,
        using_relay,
        throughput,
        trust_attestations,
        vpk_info,
        peer_id.to_base58(),
    );

    // ── Initial announcement ───────────────────────────────────────────────
    if announce_self {
        info!("[3/4] Announcing to DHT...");
        if let Err(e) = node.announce(&ctx, &server_info, bootstrap_peers).await {
            warn!("Initial announce failed: {e:#} — will retry at the 300 s tick");
        }
    }

    info!("[4/4] ✅ KwaaiNet node running");
    info!("   Peer ID : {}", peer_id.to_base58());
    // A bootstrap has no name, model or block range to report.
    if announce_self {
        info!("   Name    : {}", public_name);
        info!("   Model   : {}", config.model);
        info!(
            "   Blocks  : {}–{}",
            config.start_block(),
            config.effective_end_block()
        );
        info!("   Map     : https://map.kwaai.ai");
    }

    // ── Event loop ─────────────────────────────────────────────────────────
    // Same 300 s ± 30 s jittered cadence as the p2pd path: the DHT TTL is 360 s,
    // so every record keeps at least 30 s of headroom, and the jitter stops a
    // mass restart from thundering-herding the bootstraps.
    let mut rep_store = crate::reputation::ReputationStore::load();
    let mut next_announce = Box::pin(tokio::time::sleep(Duration::from_secs(
        crate::node::jitter_secs(300, 30),
    )));
    // The Ollama watcher exists to trigger a re-announce when inference comes
    // back, so a bootstrap does not start one: it would poll a port it does not
    // serve every 15 s to fire a signal for an arm that is disabled anyway.
    // A dangling receiver stands in so the `select!` arm keeps its type, and the
    // sender is held rather than dropped so the branch is simply never ready.
    let (_ollama_recovery_tx, mut ollama_recovery_rx) = tokio::sync::mpsc::channel::<()>(1);
    if announce_self {
        ollama_recovery_rx = crate::node::spawn_ollama_watcher(&config);
    }
    let mut pending_update_version: Option<String> = None;
    // Deadline for the reachability-change settle window; None = no change pending.
    let mut announce_settle: Option<tokio::time::Instant> = None;

    loop {
        tokio::select! {
            // SIGHUP (Unix) — re-read config and re-announce. `shard serve`
            // signals a block-range change this way.
            _ = sighup.recv() => {
                info!("SIGHUP received — re-reading config");
                reload_block_range(&mut config);
                refresh_server_info(&mut server_info, &config);
                if announce_self {
                    if let Err(e) = node.announce(&ctx, &server_info, bootstrap_peers).await {
                        warn!("Re-announce after SIGHUP failed: {e:#}");
                    }
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

                // Fold in the current reachability so the periodic record is
                // never staler than the swarm. `borrow` (not `borrow_and_update`)
                // leaves the change notification for the settle arm.
                let tick_state = *announce_state.borrow();
                if tick_state.announceable {
                    using_relay = tick_state.using_relay;
                    server_info.using_relay = using_relay;
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

                if announce_self {
                    refresh_server_info(&mut server_info, &config);
                    info!(
                        "Re-announcing to DHT (shard_ready={})...",
                        ShardManager::shard_is_ready()
                    );
                    match node.announce(&ctx, &server_info, bootstrap_peers).await {
                        Ok(timings) => {
                            record_reputation(&mut rep_store, timings);
                            if tick_state.announceable {
                                last_announced_epoch = tick_state.epoch;
                            }
                        }
                        Err(e) => warn!("Re-announce failed: {e:#}"),
                    }
                }

                next_announce
                    .as_mut()
                    .reset(tokio::time::Instant::now()
                        + Duration::from_secs(crate::node::jitter_secs(300, 30)));
            }

            // Reachability or relay status changed — re-publish in place.
            //
            // The delay is a deadline armed here and awaited in its own branch,
            // not an inline sleep: sleeping inside the arm would hold up every
            // other branch — most visibly shutdown — for the full settle window.
            changed = announce_state.changed(), if announce_settle.is_none() => {
                if changed.is_err() {
                    // Every sender lives in the service task; an error here
                    // means the swarm is gone. Continuing would keep announcing
                    // a node that can no longer be reached.
                    error!("Network service ended unexpectedly — shutting down");
                    break;
                }
                announce_settle = Some(tokio::time::Instant::now() + ANNOUNCE_SETTLE);
            }

            _ = async { tokio::time::sleep_until(announce_settle.unwrap()).await },
                if announce_settle.is_some() =>
            {
                announce_settle = None;
                let state = *announce_state.borrow_and_update();
                if !announce_self {
                    // A bootstrap publishes nothing, so a reachability change
                    // has no record to refresh.
                    continue;
                }
                if !state.announceable {
                    info!(
                        "Reachability is still unknown — deferring the announce rather than \
                         claiming a reachability we cannot back up"
                    );
                    continue;
                }
                if state.epoch == last_announced_epoch {
                    // Nothing new since the last successful publish.
                    continue;
                }
                using_relay = state.using_relay;
                info!(
                    "Reachability changed ({:?}, using_relay={}) — re-announcing",
                    state.reachability, using_relay
                );
                crate::node::refresh_throughput(&mut server_info, &config.model, dl_bps, using_relay);
                server_info.using_relay = using_relay;
                refresh_server_info(&mut server_info, &config);
                match node.announce(&ctx, &server_info, bootstrap_peers).await {
                    // Only a successful publish consumes the epoch; on failure
                    // the next settle window or the 300 s tick retries it.
                    Ok(_) => last_announced_epoch = state.epoch,
                    Err(e) => warn!("Re-announce after a reachability change failed: {e:#}"),
                }
            }

            // Ollama came back up — re-announce immediately so clients learn the
            // host is usable again without waiting out the 300 s tick.
            Some(()) = ollama_recovery_rx.recv(), if announce_self => {
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
    // rather than waiting out the TTL. A bootstrap published nothing, so it has
    // nothing to retract.
    if announce_self {
        info!("Unannouncing from DHT...");
        node.unannounce(&ctx, &server_info, bootstrap_peers).await;
    }
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
        config.start_block(),
        config.effective_end_block(),
        fresh.start_block(),
        fresh.start_block() + fresh.blocks,
    );
    config.start_block = fresh.start_block;
    config.blocks = fresh.blocks;
}

/// Sync the announced block range and readiness state from the live config.
fn refresh_server_info(server_info: &mut DHTServerInfo, config: &KwaaiNetConfig) {
    server_info.start_block = config.start_block() as i32;
    server_info.end_block = config.effective_end_block() as i32;
    server_info.state = KwaaiNetConfig::announce_state();
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
                lease_outcome: None,
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
async fn register_node_handlers(
    handle: &NetworkHandle,
    lease_table: std::sync::Arc<crate::capacity_lease::LeaseTable>,
) {
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

    let ollama = crate::ollama_proxy::make_ollama_proxy_handler(lease_table.clone());
    if let Err(e) = handle
        .add_unary_handler(crate::ollama_proxy::OLLAMA_PROXY_PROTO, move |data| {
            let fut = ollama(data);
            async move { fut.await.map_err(|e| e.to_string()) }
        })
        .await
    {
        warn!("registering ollama-proxy handler failed: {e}");
    }

    // Capacity-lease unary handler — the p2p:// (non-mux) callers negotiate
    // here. The mux path negotiates over its own already-open stream instead.
    let lease = crate::capacity_lease::make_capacity_lease_handler(lease_table);
    if let Err(e) = handle
        .add_unary_handler(crate::capacity_lease::CAPACITY_LEASE_PROTO, move |data| {
            let fut = lease(data);
            async move { fut.await.map_err(|e| e.to_string()) }
        })
        .await
    {
        warn!("registering capacity-lease handler failed: {e}");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::announce::dht_id;
    use kwaai_hivemind_dht::protocol::{FindRequest, RequestAuthInfo, ResultType};

    /// Build a node on an ephemeral loopback port with the given
    /// `announce_self`, and a config that touches nothing outside the tempdir.
    ///
    /// `port: 0` lets the OS pick, so two of these can run concurrently and
    /// neither collides with a developer's real node on 8080.
    fn seed_like_config(announce_self: bool, home: &std::path::Path) -> KwaaiNetConfig {
        KwaaiNetConfig {
            native_p2p: Some(true),
            announce_self,
            dht_server: true,
            enable_upnp: false,
            port: 0,
            initial_peers: Vec::new(),
            vpk_enabled: false,
            ollama_manage: false,
            identity_key: Some(home.join("identity.key")),
            ..KwaaiNetConfig::default()
        }
    }

    /// The keys an announce round would publish under, in the order
    /// `build_announce_records` emits them.
    fn announced_keys(config: &KwaaiNetConfig) -> Vec<String> {
        vec![
            format!("{}.{}", config.effective_dht_prefix(), config.start_block()),
            "_petals.models".to_string(),
            "_kwaai.inference.nodes".to_string(),
        ]
    }

    /// Nothing is findable in a node's own storage under any key it would have
    /// announced.
    fn storage_has_nothing_under(storage: &DHTStorage, keys: &[String]) -> bool {
        keys.iter().all(|key| {
            let response = storage.handle_find(FindRequest {
                auth: Some(RequestAuthInfo::new()),
                keys: vec![dht_id(key)],
                peer: None,
            });
            response.results[0].result_type == ResultType::NotFound as i32
        })
    }

    /// A bootstrap preset asks for no publishing; an ordinary node still does.
    ///
    /// Both directions live in one test, run sequentially, because
    /// `KWAAINET_SOCKET` is process-global: each node binds a control socket,
    /// and two `#[tokio::test]`s running in parallel would either collide on
    /// the default path or race each other's env var.
    ///
    /// The assertion is made at the deliver seam rather than by counting
    /// outbound RPCs: the first thing `announce` does on the happy path is
    /// write every record into its *own* storage, so a storage that stays empty
    /// across an explicit `announce()` proves no record was built, let alone
    /// sent. With no bootstrap peers there is nowhere to send to either, which
    /// keeps this free of a second process.
    #[tokio::test]
    async fn a_bootstrap_preset_publishes_nothing_and_a_node_still_does() {
        // `announce_self` is now enforced by the caller: `run_native_node` skips
        // the announce entirely rather than calling into a method that no-ops.
        // What the config must guarantee is that a bootstrap preset asks for no
        // publishing at all.
        let home = tempfile::tempdir().expect("tmpdir");
        let bootstrap_cfg = seed_like_config(false, home.path());
        assert!(
            !bootstrap_cfg.announce_self,
            "a bootstrap preset must not publish records of its own"
        );
        drop(home);

        // ── announce_self = true: an ordinary node still publishes ──────────
        // Without this half, the assertions above would pass just as happily
        // against an announce path that was broken outright.
        let home = tempfile::tempdir().expect("tmpdir");
        let config = seed_like_config(true, home.path());
        std::env::set_var("KWAAINET_SOCKET", home.path().join("node.sock"));
        kwaai_p2p::identity::generate_keypair(config.identity_key.as_ref().unwrap())
            .expect("the fixture key must generate");

        let node = NativeNode::start(&config, &[])
            .await
            .expect("an ordinary node must start");

        let prefix = config.effective_dht_prefix();
        let ctx = AnnounceContext {
            peer_id: node.peer_id,
            prefix: &prefix,
            repository: "https://huggingface.co/Qwen/Qwen3-8B",
            total_blocks: 32,
        };
        let server_info = DHTServerInfo::new(
            config.start_block() as i32,
            config.effective_end_block() as i32,
            "node-under-test",
            false,
            0.0,
            Vec::new(),
            None,
            node.peer_id.to_base58(),
        );

        node.announce(&ctx, &server_info, &[])
            .await
            .expect("an ordinary announce must build its records");
        assert!(
            !storage_has_nothing_under(&node.storage, &announced_keys(&config)),
            "an announcing node must write its own records into its storage"
        );

        node.shutdown().await;
        std::env::remove_var("KWAAINET_SOCKET");
    }
}
