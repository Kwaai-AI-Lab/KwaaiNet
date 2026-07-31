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
    DHTServerInfo,
};
use crate::config::KwaaiNetConfig;

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
