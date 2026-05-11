//! Native Rust node runner
//!
//! Uses go-libp2p-daemon (p2pd) with Hivemind DHT protocol handlers to make
//! this node visible on map.kwaai.ai — the same approach as the
//! `petals_visible` example, integrated into the kwaainet CLI lifecycle.

use anyhow::{Context, Result};
use kwaai_hivemind_dht::{
    codec::DHTRequest,
    protocol::{NodeInfo, RequestAuthInfo, StoreRequest},
    value::get_dht_time,
    DHTStorage,
};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::{stream, P2PDaemon};
use libp2p::PeerId;
use sha1::{Digest, Sha1};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{io::AsyncWriteExt, net::TcpListener, signal, sync::RwLock};
use tracing::{info, warn};

use crate::config::KwaaiNetConfig;
use crate::daemon::{DaemonManager, ShardManager};
use crate::identity::NodeIdentity;

type SharedStorage = Arc<RwLock<DHTStorage>>;

// ---------------------------------------------------------------------------
// VPK capability info
// ---------------------------------------------------------------------------

/// VPK (Virtual Private Knowledge) capability snapshot used in DHT records.
///
/// Populated by polling `GET http://localhost:{vpk_local_port}/api/health`
/// immediately before each DHT announcement. When VPK is unreachable the
/// field is absent from both the per-block record and the nodes registry.
///
/// Nodes are identified solely by PeerId — no IP addresses are advertised.
/// Remote Bobs connect via `/kwaai/storage/1.0.0` over the libp2p relay.
struct VpkInfo {
    mode: String,
    capacity_gb: f64,
    tenant_count: u32,
    vpk_version: String,
    public_name: String,
}

impl VpkInfo {
    /// Build the rmpv Map that appears as the `"vpk"` value in DHT field maps.
    fn to_msgpack_value(&self) -> rmpv::Value {
        rmpv::Value::Map(vec![
            (
                rmpv::Value::from("mode"),
                rmpv::Value::from(self.mode.as_str()),
            ),
            (
                rmpv::Value::from("capacity_gb"),
                rmpv::Value::from(self.capacity_gb),
            ),
            (
                rmpv::Value::from("tenant_count"),
                rmpv::Value::from(i64::from(self.tenant_count)),
            ),
            (
                rmpv::Value::from("vpk_version"),
                rmpv::Value::from(self.vpk_version.as_str()),
            ),
            (
                rmpv::Value::from("public_name"),
                rmpv::Value::from(self.public_name.as_str()),
            ),
        ])
    }

    /// Standalone msgpack bytes for the `_kwaai.vpk.nodes` DHT record value.
    fn to_msgpack_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &self.to_msgpack_value())?;
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// DHT value types (Hivemind wire format)
// ---------------------------------------------------------------------------

/// Server info serialised as ExtType(64, [state, throughput, {fields}])
/// — the exact format Python Hivemind / map.kwaai.ai expects.
///
/// The optional `trust_attestations` field carries the node's Verifiable
/// Credentials as compact JSON strings. Clients that understand the KwaaiNet
/// trust model (e.g., map.kwaai.ai v2) display trust badges; legacy clients
/// ignore the field.
struct DHTServerInfo {
    pub state: i32,
    throughput: f64,
    start_block: i32,
    end_block: i32,
    public_name: String,
    version: String,
    torch_dtype: String,
    using_relay: bool,
    cache_tokens_left: i64,
    #[allow(dead_code)]
    next_pings: HashMap<String, f64>,
    #[allow(dead_code)]
    adapters: Vec<String>,
    /// Compact JSON representations of the node's valid Verifiable Credentials.
    /// Empty when no credentials are stored; included in the DHT fields map
    /// only when non-empty to keep announcement payloads minimal.
    trust_attestations: Vec<String>,

    /// VPK capability snapshot. None when VPK is disabled or unreachable.
    /// Included in the DHT fields map only when Some.
    vpk_info: Option<VpkInfo>,

    /// Peer ID in base58 encoding. Included in the value map so that chain
    /// discovery can identify the serving peer even from FoundRegular responses
    /// (which do not carry the DHT subkey). Unknown fields are silently ignored
    /// by legacy Python Hivemind clients.
    peer_id_b58: String,
}

impl DHTServerInfo {
    #[allow(clippy::too_many_arguments)]
    fn new(
        start: i32,
        end: i32,
        name: &str,
        relay: bool,
        throughput: f64,
        trust_attestations: Vec<String>,
        vpk_info: Option<VpkInfo>,
        peer_id_b58: String,
    ) -> Self {
        Self {
            state: 2, // ONLINE
            throughput,
            start_block: start,
            end_block: end,
            public_name: name.to_string(),
            version: concat!("kwaai-", env!("CARGO_PKG_VERSION")).to_string(),
            torch_dtype: "float16".to_string(),
            using_relay: relay,
            cache_tokens_left: 100_000,
            next_pings: HashMap::new(),
            adapters: vec![],
            trust_attestations,
            vpk_info,
            peer_id_b58,
        }
    }

    fn to_msgpack(&self) -> Result<Vec<u8>> {
        let mut fields: Vec<(rmpv::Value, rmpv::Value)> = vec![
            (
                rmpv::Value::from("start_block"),
                rmpv::Value::from(self.start_block),
            ),
            (
                rmpv::Value::from("end_block"),
                rmpv::Value::from(self.end_block),
            ),
            (
                rmpv::Value::from("public_name"),
                rmpv::Value::from(self.public_name.as_str()),
            ),
            (
                rmpv::Value::from("version"),
                rmpv::Value::from(self.version.as_str()),
            ),
            (
                rmpv::Value::from("torch_dtype"),
                rmpv::Value::from(self.torch_dtype.as_str()),
            ),
            (
                rmpv::Value::from("using_relay"),
                rmpv::Value::from(self.using_relay),
            ),
            (
                rmpv::Value::from("cache_tokens_left"),
                rmpv::Value::from(self.cache_tokens_left),
            ),
            (rmpv::Value::from("adapters"), rmpv::Value::Array(vec![])),
            (rmpv::Value::from("next_pings"), rmpv::Value::Map(vec![])),
            (
                rmpv::Value::from("peer_id"),
                rmpv::Value::from(self.peer_id_b58.as_str()),
            ),
        ];

        // Include trust attestations when present — zero-cost for nodes without VCs.
        // Legacy clients (Python Hivemind, old map viewers) ignore unknown fields.
        if !self.trust_attestations.is_empty() {
            let ta_values: Vec<rmpv::Value> = self
                .trust_attestations
                .iter()
                .map(|s| rmpv::Value::String(rmpv::Utf8String::from(s.as_str())))
                .collect();
            fields.push((
                rmpv::Value::from("trust_attestations"),
                rmpv::Value::Array(ta_values),
            ));
        }

        // Include VPK capability when enabled and reachable.
        // Unknown map keys are silently ignored by legacy Hivemind clients
        // and old map viewers — no backward-compatibility risk.
        if let Some(ref vpk) = self.vpk_info {
            fields.push((rmpv::Value::from("vpk"), vpk.to_msgpack_value()));
        }

        let inner = rmpv::Value::Array(vec![
            rmpv::Value::from(self.state),
            rmpv::Value::from(self.throughput),
            rmpv::Value::Map(fields),
        ]);

        let mut inner_bytes = Vec::new();
        rmpv::encode::write_value(&mut inner_bytes, &inner)?;

        // Wrap in ExtType(64 = 0x40) — Python Hivemind tuple marker
        let ext = rmpv::Value::Ext(64, inner_bytes);
        let mut out = Vec::new();
        rmpv::encode::write_value(&mut out, &ext)?;
        Ok(out)
    }
}

/// Model info stored in the `_petals.models` DHT registry.
struct ModelInfo {
    num_blocks: i32,
    repository: String,
}

impl ModelInfo {
    fn to_msgpack(&self) -> Result<Vec<u8>> {
        let map = vec![
            (
                rmpv::Value::from("repository"),
                rmpv::Value::from(self.repository.as_str()),
            ),
            (
                rmpv::Value::from("num_blocks"),
                rmpv::Value::from(self.num_blocks),
            ),
        ];
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &rmpv::Value::Map(map))?;
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// DHT key helpers
// ---------------------------------------------------------------------------

/// SHA1(msgpack(raw_key)) — Hivemind's DHTID.generate() equivalent.
fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub async fn run_node(config: &KwaaiNetConfig) -> Result<()> {
    // PID tracking
    let daemon_mgr = DaemonManager::new();
    daemon_mgr
        .write_pid(std::process::id())
        .context("writing PID")?;
    info!("KwaaiNet node starting (PID {})", std::process::id());

    // -----------------------------------------------------------------------
    // Persistent identity — load or generate the Ed25519 keypair so the
    // PeerId is stable across restarts. Credentials are bound to this DID.
    // -----------------------------------------------------------------------
    let node_identity = NodeIdentity::load_or_create().context("loading node identity")?;
    let node_did = node_identity.did();
    info!("Node DID: {}", node_did);

    // Load valid VCs for this node's DID to include in DHT announcements
    let trust_attestations = match kwaai_trust::CredentialStore::open_default() {
        Ok(store) => {
            let vcs = store.load_valid_for_subject(&node_did);
            if vcs.is_empty() {
                info!("Trust attestations: none (run `kwaainet identity import-vc` to add)");
            } else {
                info!("Trust attestations: {} valid VC(s)", vcs.len());
                for vc in &vcs {
                    info!(
                        "  [{}] issued by {}",
                        vc.kwaai_type().map(|t| t.as_str()).unwrap_or("Unknown"),
                        &vc.issuer_did()[..vc.issuer_did().len().min(32)]
                    );
                }
            }
            vcs.iter()
                .filter_map(|vc| vc.to_compact_json().ok())
                .collect::<Vec<_>>()
        }
        Err(e) => {
            warn!(
                "Could not open credential store: {} — proceeding without VCs",
                e
            );
            vec![]
        }
    };

    let public_name = format!(
        "{}/v{}",
        config
            .public_name
            .clone()
            .unwrap_or_else(|| "kwaainet-node".to_string()),
        env!("CARGO_PKG_VERSION"),
    );

    info!(
        model = %config.model,
        blocks = config.blocks,
        port = config.port,
        name = %public_name,
        "Configuring KwaaiNet node"
    );

    // Bootstrap peers — prefer config, fall back to Petals defaults
    let net_cfg = NetworkConfig::with_petals_bootstrap();
    let bootstrap_peers: Vec<String> = if config.initial_peers.is_empty() {
        net_cfg.bootstrap_peers.clone()
    } else {
        config.initial_peers.clone()
    };

    // -----------------------------------------------------------------------
    // Step 1: Start p2pd
    // -----------------------------------------------------------------------
    info!("[1/5] Starting p2p daemon...");
    let p2pd_path = find_p2pd_binary();
    if p2pd_path.is_none() {
        eprintln!("  ⚠️  p2pd not found — run `kwaainet setup --get-deps` to install it");
    }

    // p2pd listens for P2P traffic on the configured port
    let host_addr = format!("/ip4/0.0.0.0/tcp/{}", config.port);

    // Announce address: prefer explicit announce_addr, fall back to public_ip.
    // announce_addr is a raw multiaddr (e.g. /dns/kwaainet/tcp/8080).
    // public_ip is an IP address formatted as /ip4/<ip>/tcp/<port>.
    // An empty string public_ip is treated as "no public IP".
    let announce_addr = config.announce_addr.clone().or_else(|| {
        config
            .public_ip
            .as_deref()
            .filter(|ip| !ip.is_empty())
            .map(|ip| format!("/ip4/{}/tcp/{}", ip, config.port))
    });

    let identity_key_path = NodeIdentity::key_file_path();

    // Trusted relays: empty means "let AutoRelay discover via DHT". When the
    // user configures explicit trusted_relays we pass them through; otherwise
    // we leave the list empty rather than auto-promoting bootstraps to relays
    // (bootstraps may not run a hop relay service, which causes reservations
    // to silently fail).
    let trusted_relays = config.trusted_relays.clone();

    let builder = P2PDaemon::builder()
        .dht(true)
        .bootstrap(!bootstrap_peers.is_empty())
        .relay(!config.no_relay)
        .auto_relay(true)
        .auto_nat(true)
        // Pre-declaring private blocks AutoNAT from ever promoting the node
        // to public, even if it actually is. Only honour an explicit opt-in.
        .force_reachability_private(config.force_private)
        .nat_portmap(true)
        .host_addrs([host_addr])
        .bootstrap_peers(bootstrap_peers.clone())
        .trusted_relays(trusted_relays.clone())
        .with_identity_key(&identity_key_path);

    let builder = if let Some(ref addr) = announce_addr {
        builder.announce_addrs([addr.as_str()])
    } else {
        builder
    };

    let builder = if let Some(ref path) = p2pd_path {
        builder.with_binary_path(path)
    } else {
        builder
    };

    // Allow a custom socket path so multiple nodes can run on the same machine.
    // Usage: KWAAINET_SOCKET=/tmp/kwaai-p2pd-b.sock kwaainet run-node
    let builder = if let Ok(sock) = std::env::var("KWAAINET_SOCKET") {
        // with_listen_addr expects full multiaddr format, e.g. /unix//tmp/kwaai-p2pd-b.sock
        #[cfg(unix)]
        let addr = format!("/unix/{}", sock);
        #[cfg(not(unix))]
        let addr = sock;
        builder.with_listen_addr(addr)
    } else {
        builder
    };

    let mut daemon = builder.spawn().await.context("starting p2pd")?;
    let mut client = daemon.client().await.context("p2pd client")?;

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?).context("parse peer ID")?;
    info!("Peer ID: {}", peer_id.to_base58());

    // -----------------------------------------------------------------------
    // Step 2: DHT storage
    // -----------------------------------------------------------------------
    info!("[2/5] Initialising DHT storage...");
    let storage: SharedStorage = Arc::new(RwLock::new(DHTStorage::new(peer_id)));

    // -----------------------------------------------------------------------
    // Step 3: Register Hivemind RPC stream handlers with p2pd
    // -----------------------------------------------------------------------
    info!("[3/5] Registering Hivemind RPC handlers...");
    let handler_listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("binding RPC handler listener")?;
    let handler_addr = handler_listener.local_addr()?;

    client
        .register_stream_handler(
            &format!("/ip4/127.0.0.1/tcp/{}", handler_addr.port()),
            vec![
                "DHTProtocol.rpc_ping".to_string(),
                "DHTProtocol.rpc_store".to_string(),
                "DHTProtocol.rpc_find".to_string(),
            ],
        )
        .await
        .context("registering stream handlers")?;
    info!("RPC handlers ready on {}", handler_addr);

    // Register the /kwaai/p2p/hello/1.0.0 handler so any peer can DM us.
    // Lives alongside the Hivemind RPC handlers because both belong to the
    // node's "while we're alive, please answer these" surface area.
    client
        .add_unary_handler(
            kwaai_p2p_daemon::hello::HELLO_PROTO,
            kwaai_p2p_daemon::hello::make_handler(),
            false,
        )
        .await
        .context("registering p2p hello handler")?;

    // -----------------------------------------------------------------------
    // Step 4: Wait for DHT bootstrap (intelligent polling)
    // -----------------------------------------------------------------------
    info!("[4/5] Bootstrapping...");
    dial_and_wait_for_bootstrap(&mut client, &bootstrap_peers).await?;

    // If p2pd crashed during bootstrap (Kademlia walk goroutine panic in
    // go-libp2p-kad-dht), restart it now before proceeding to announce.
    // This avoids a wasted announce attempt and surfaces the crash output.
    if !daemon.is_running() {
        let stderr = daemon.captured_stderr().await;
        if !stderr.is_empty() {
            warn!("p2pd crash output:\n{}", stderr.trim());
        }
        warn!("⚠️  p2pd crashed during bootstrap — restarting immediately…");
        restart_p2pd(
            &mut daemon,
            &mut client,
            &p2pd_path,
            &config,
            &bootstrap_peers,
            &announce_addr,
            handler_addr,
        )
        .await
        .context("p2pd restart after bootstrap crash")?;
        info!("✅ p2pd restarted after bootstrap crash — continuing to announce");
    }

    // -----------------------------------------------------------------------
    // Step 5: Initial DHT announcement
    // -----------------------------------------------------------------------
    info!("[5/5] Announcing to DHT...");

    // Install the SIGHUP handler here — before announce() — so that if
    // `shard serve` calls signal_reannounce() while announce() is still
    // running (a ~5 s window), tokio captures the signal instead of
    // applying the default Unix action (terminate process).
    #[cfg(unix)]
    let mut sighup = {
        use tokio::signal::unix::{signal, SignalKind};
        signal(SignalKind::hangup()).expect("SIGHUP handler")
    };

    // Determine effective throughput using the Petals formula:
    //   effective_tps = min(compute_tps, network_rps × relay_penalty)
    //   network_rps   = download_bps / (hidden_size × 16)
    // using_relay: true only if we have no reachable address (behind NAT) and relay is allowed.
    // announce_addr already encodes both explicit announce_addr and non-empty public_ip.
    let using_relay = announce_addr.is_none() && !config.no_relay;

    // Measure network bandwidth once at startup (1 MiB Cloudflare probe).
    // Stored so re-announcements can recompute effective_tps without re-probing.
    let dl_bps: f64 = if crate::throughput::load(&config.model).is_some() {
        info!("  Measuring network bandwidth (1 MiB probe)...");
        let bps = crate::throughput::measure_download_bps().await;
        if bps > 0.0 {
            info!("  Network:  {:.1} Mbps download", bps / 1_000_000.0);
        } else {
            info!("  Network:  measurement failed — using compute limit only");
        }
        bps
    } else {
        0.0
    };

    let throughput = compute_effective_tps(&config.model, dl_bps, using_relay);
    if let Some(ref entry) = crate::throughput::load(&config.model) {
        info!(
            "  Compute:  {:.1} tok/s (measured, hidden_dim={})",
            entry.compute_tps, entry.hidden_size
        );
        info!(
            "  Effective: {:.1} tok/s  connection={} (min({:.1}, {:.1}×{}))",
            throughput,
            if using_relay { "relay" } else { "direct" },
            entry.compute_tps,
            if dl_bps > 0.0 {
                dl_bps / (entry.hidden_size as f64 * 16.0)
            } else {
                f64::INFINITY
            },
            if using_relay { "0.2" } else { "1.0" },
        );
    } else {
        info!(
            "  Throughput: {:.1} tok/s (default — run `kwaainet benchmark` to measure)",
            throughput
        );
    }

    // Use the canonical DHT prefix from the map (set during startup model selection).
    // Falls back to a computed prefix if the map wasn't consulted (e.g. --model override).
    let prefix = config.effective_dht_prefix();
    let repository = config.model_repository.clone().unwrap_or_else(|| {
        if config.model.contains('/') {
            format!("https://huggingface.co/{}", config.model)
        } else {
            format!("https://huggingface.co/meta-llama/{}", config.model)
        }
    });

    info!("  DHT prefix:  {}", prefix);
    info!("  Repository:  {}", repository);
    info!("  Using relay: {}", using_relay);

    // Check local VPK health when integration is enabled.
    // Retries up to 5 times with 1 s gaps to avoid a race with the storage
    // child process (spawned by `kwaainet start --daemon` just before us).
    let vpk_info = if config.vpk_enabled {
        let port = config.vpk_local_port.unwrap_or(7432);
        info!("VPK enabled — checking local service on port {}", port);
        let mut health_result = None;
        for attempt in 0..5u32 {
            if let Some(h) = check_vpk_health(port).await {
                health_result = Some(h);
                break;
            }
            if attempt < 4 {
                info!("VPK not ready yet, retrying in 1 s… ({}/5)", attempt + 1);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
        match health_result {
            Some(health) => {
                let mode = config
                    .vpk_mode
                    .clone()
                    .unwrap_or_else(|| "both".to_string());
                let capacity_gb = health["capacity_gb_available"].as_f64().unwrap_or(0.0);
                let tenant_count = health["tenant_count"].as_u64().unwrap_or(0) as u32;
                let vpk_version = health["version"].as_str().unwrap_or("unknown").to_string();
                info!(
                    "VPK healthy: mode={} tenants={} capacity={:.1}GB v={}",
                    mode, tenant_count, capacity_gb, vpk_version
                );
                Some(VpkInfo {
                    mode,
                    capacity_gb,
                    tenant_count,
                    vpk_version,
                    public_name: public_name.clone(),
                })
            }
            None => {
                warn!(
                    "VPK health check failed on port {} after 5 attempts — skipping DHT advertisement",
                    port
                );
                None
            }
        }
    } else {
        None
    };

    // Always announce the configured block range so the node appears on the map.
    let announce_start = config.start_block as i32;
    let announce_end = config.effective_end_block() as i32;

    let mut server_info = DHTServerInfo::new(
        announce_start,
        announce_end,
        &public_name,
        using_relay,
        throughput,
        trust_attestations,
        vpk_info,
        peer_id.to_base58(),
    );
    announce(
        &mut client,
        peer_id,
        &storage,
        &bootstrap_peers,
        &prefix,
        &repository,
        config.model_total_blocks(),
        announce_start,
        announce_end,
        &server_info,
        None,
    )
    .await
    .context("initial DHT announcement")?;

    // If p2pd crashed during announce (Kademlia race despite the sleep above),
    // restart it immediately rather than waiting 120 s for the watchdog tick.
    if !daemon.is_running() {
        let stderr = daemon.captured_stderr().await;
        if !stderr.is_empty() {
            warn!("p2pd crash output:\n{}", stderr.trim());
        }
        warn!("⚠️  p2pd crashed during initial announce — restarting immediately…");
        match restart_p2pd(
            &mut daemon,
            &mut client,
            &p2pd_path,
            &config,
            &bootstrap_peers,
            &announce_addr,
            handler_addr,
        )
        .await
        {
            Ok(()) => {
                info!("✅ p2pd restarted — retrying initial announce…");
                if let Err(e) = announce(
                    &mut client,
                    peer_id,
                    &storage,
                    &bootstrap_peers,
                    &prefix,
                    &repository,
                    config.model_total_blocks(),
                    announce_start,
                    announce_end,
                    &server_info,
                    None,
                )
                .await
                {
                    warn!(
                        "Initial announce retry failed: {} — will retry at 300s tick",
                        e
                    );
                }
            }
            Err(e) => warn!("p2pd restart failed: {} — will retry at 120s tick", e),
        }
    }

    info!("✅ KwaaiNet node running");
    info!("   Peer ID : {}", peer_id.to_base58());
    info!("   Name    : {}", public_name);
    info!("   Model   : {}", config.model);
    info!(
        "   Blocks  : {}–{}",
        config.start_block,
        config.effective_end_block()
    );
    info!("   Map     : https://map.kwaai.ai");

    // -----------------------------------------------------------------------
    // Event loop: handle incoming RPC + periodic re-announce
    // -----------------------------------------------------------------------
    // Shadow config with a local mutable copy so the event loop can update
    // start_block/blocks when SIGHUP triggers a config re-read.
    let mut config = config.clone();
    let storage_clone = storage.clone();

    // Re-announce every 300 s ± 30 s (jittered so nodes don't thundering-herd
    // the bootstrap peers after a network partition or mass restart).
    // DHT TTL is 360 s, so 270–330 s keeps every record refreshed with
    // at least 30 s headroom.  One observation per peer per cycle is recorded
    // in the reputation store, piggybacked on the STORE RPC latency.
    let mut rep_store = crate::reputation::ReputationStore::load();
    let mut next_announce = Box::pin(tokio::time::sleep(Duration::from_secs(jitter_secs(
        300, 30,
    ))));

    // (SIGHUP handler installed above before announce() — see Step 5 setup)

    loop {
        tokio::select! {
            // Incoming RPC stream from p2pd
            result = handler_listener.accept() => {
                match result {
                    Ok((mut stream, addr)) => {
                        info!("Incoming RPC from {}", addr);
                        let s = storage_clone.clone();
                        tokio::spawn(async move {
                            if let Err(e) = handle_rpc_stream(&mut stream, s).await {
                                warn!("RPC handler error: {}", e);
                            }
                        });
                    }
                    Err(e) => warn!("Accept error: {}", e),
                }
            }

            // SIGHUP (Unix) / never (Windows) — re-read config and re-announce.
            // Uses #[cfg] inside the arm expression to avoid a conditional arm,
            // which is unsupported by tokio::select!.
            _ = async {
                #[cfg(unix)] { sighup.recv().await; }
                #[cfg(not(unix))] { std::future::pending::<Option<()>>().await; }
            } => {
                info!("SIGHUP received — re-reading config and re-announcing");
                if let Ok(fresh) = KwaaiNetConfig::load_or_create() {
                    if fresh.start_block != config.start_block || fresh.blocks != config.blocks {
                        info!(
                            "Block range updated: [{}–{}) → [{}–{})",
                            config.start_block, config.effective_end_block(),
                            fresh.start_block, fresh.start_block + fresh.blocks,
                        );
                        config.start_block = fresh.start_block;
                        config.blocks = fresh.blocks;
                    }
                }
                let sb = config.start_block as i32;
                let eb = config.effective_end_block() as i32;
                server_info.start_block = sb;
                server_info.end_block = eb;
                if let Err(e) = announce(
                    &mut client, peer_id, &storage, &bootstrap_peers,
                    &prefix, &repository, config.model_total_blocks(),
                    sb, eb, &server_info, None,
                ).await {
                    warn!("Re-announce after SIGHUP failed: {}", e);
                }
            }

            // Periodic re-announcement (300 s ± 30 s jitter)
            _ = &mut next_announce => {
                // p2pd watchdog: restart if the child process died unexpectedly.
                if !daemon.is_running() {
                    let stderr = daemon.captured_stderr().await;
                    if !stderr.is_empty() {
                        warn!("p2pd crash output:\n{}", stderr.trim());
                    }
                    warn!("⚠️  p2pd process died — attempting restart…");
                    match restart_p2pd(
                        &mut daemon, &mut client, &p2pd_path, &config,
                        &bootstrap_peers, &announce_addr, handler_addr,
                    ).await {
                        Ok(()) => info!("✅ p2pd restarted and handlers re-registered"),
                        Err(e) => warn!("p2pd restart failed: {} — will retry in 120s", e),
                    }
                }

                // Re-read config to pick up start_block changes written by
                // `shard serve` (via signal_reannounce) or `kwaainet config set`.
                // On Windows this also drains the reannounce.flag file.
                if let Ok(fresh) = KwaaiNetConfig::load_or_create() {
                    if fresh.start_block != config.start_block || fresh.blocks != config.blocks {
                        info!(
                            "Block range updated: [{}–{}) → [{}–{})",
                            config.start_block, config.effective_end_block(),
                            fresh.start_block, fresh.start_block + fresh.blocks,
                        );
                        config.start_block = fresh.start_block;
                        config.blocks = fresh.blocks;
                    }
                }
                #[cfg(not(unix))]
                {
                    let flag = crate::config::run_dir().join("reannounce.flag");
                    if flag.exists() {
                        let _ = std::fs::remove_file(&flag);
                    }
                }

                // Refresh throughput from cache.
                let fresh_tps = compute_effective_tps(&config.model, dl_bps, using_relay);
                if (fresh_tps - server_info.throughput).abs() > 0.05 {
                    info!(
                        "Throughput updated: {:.1} → {:.1} tok/s",
                        server_info.throughput, fresh_tps
                    );
                    server_info.throughput = fresh_tps;
                }

                // Re-check VPK health so that storage serve started after the
                // daemon comes online without requiring a full daemon restart.
                if config.vpk_enabled {
                    let port = config.vpk_local_port.unwrap_or(7432);
                    let fresh_vpk = match check_vpk_health(port).await {
                        Some(health) => {
                            let mode = config.vpk_mode.clone().unwrap_or_else(|| "both".to_string());
                            Some(VpkInfo {
                                mode,
                                capacity_gb: health["capacity_gb_available"].as_f64().unwrap_or(0.0),
                                tenant_count: health["tenant_count"].as_u64().unwrap_or(0) as u32,
                                vpk_version: health["version"].as_str().unwrap_or("unknown").to_string(),
                                public_name: public_name.clone(),
                            })
                        }
                        None => None,
                    };
                    if fresh_vpk.is_some() != server_info.vpk_info.is_some() {
                        info!(
                            "VPK state changed: {} → {}",
                            if server_info.vpk_info.is_some() { "enabled" } else { "disabled" },
                            if fresh_vpk.is_some() { "enabled" } else { "disabled" },
                        );
                    }
                    server_info.vpk_info = fresh_vpk;
                }

                // Auto-update — installs new binary when available (pre-v1.0).
                // If installed, break the event loop so the daemon exits cleanly
                // and can be restarted (by systemd/launchd or manually) with the
                // new binary.
                let auto_update = KwaaiNetConfig::load_or_create()
                    .map(|c| c.contribute_policy(false).auto_update)
                    .unwrap_or(false);
                if auto_update && maybe_auto_update().await {
                    break;
                }

                let sb = config.start_block as i32;
                let eb = config.effective_end_block() as i32;
                server_info.start_block = sb;
                server_info.end_block = eb;
                info!("Re-announcing to DHT (shard_ready={})...", ShardManager::shard_is_ready());
                if let Err(e) = announce(
                    &mut client, peer_id, &storage, &bootstrap_peers,
                    &prefix, &repository, config.model_total_blocks(),
                    sb, eb, &server_info, Some(&mut rep_store),
                ).await {
                    warn!("Re-announce failed: {}", e);
                }

                // Schedule the next tick with fresh jitter.
                next_announce
                    .as_mut()
                    .reset(tokio::time::Instant::now() + Duration::from_secs(jitter_secs(300, 30)));
            }

            // Shutdown signal
            _ = shutdown_signal() => {
                info!("Shutdown signal received");
                break;
            }
        }
    }

    // Unannounce before shutting down p2pd so the map reflects the node as
    // offline immediately rather than waiting up to 360 s for TTL expiry.
    info!("Unannouncing from DHT...");
    unannounce(
        &mut client,
        peer_id,
        &storage,
        &bootstrap_peers,
        &prefix,
        &server_info,
    )
    .await;

    let _ = daemon.shutdown().await;
    daemon_mgr.remove_pid();
    info!("KwaaiNet node stopped");
    Ok(())
}

// ---------------------------------------------------------------------------
// DHT announcement / unannouncement
// ---------------------------------------------------------------------------

/// Remove this node's DHT records immediately on clean shutdown.
///
/// Sends STORE requests with already-expired timestamps and state=-1 (offline)
/// to all bootstrap peers. Bootstrap peers drop expired records immediately
/// instead of waiting for the 360 s TTL to elapse naturally.
async fn unannounce(
    client: &mut kwaai_p2p_daemon::P2PClient,
    peer_id: PeerId,
    storage: &SharedStorage,
    bootstrap_peers: &[String],
    prefix: &str,
    server_info: &DHTServerInfo,
) {
    let offline_info = DHTServerInfo {
        state: -1, // OFFLINE — tells map.kwaai.ai to remove the node immediately
        throughput: 0.0,
        start_block: server_info.start_block,
        end_block: server_info.end_block,
        public_name: server_info.public_name.clone(),
        version: server_info.version.clone(),
        torch_dtype: server_info.torch_dtype.clone(),
        using_relay: server_info.using_relay,
        cache_tokens_left: 0,
        next_pings: HashMap::new(),
        adapters: vec![],
        trust_attestations: vec![],
        vpk_info: None,
        peer_id_b58: server_info.peer_id_b58.clone(),
    };
    // Use the same 360 s TTL as a regular announcement — Hivemind bootstrap
    // peers reject updates with a shorter TTL than the existing record.
    // State=-1 tells map.kwaai.ai the node is offline immediately; the record
    // then expires naturally after 360 s (same as a missed re-announcement).
    let expired = get_dht_time() + 360.0;

    let info_bytes = match offline_info.to_msgpack() {
        Ok(b) => b,
        Err(e) => {
            warn!("Unannounce: failed to serialise server info: {}", e);
            return;
        }
    };
    let subkey = match rmp_serde::to_vec(&peer_id.to_base58()) {
        Ok(b) => b,
        Err(e) => {
            warn!("Unannounce: failed to serialise subkey: {}", e);
            return;
        }
    };
    let node_info = NodeInfo::from_peer_id(peer_id);

    // Block records — one per announced block
    let mut keys = Vec::new();
    let mut subkeys = Vec::new();
    let mut values = Vec::new();
    let mut expirations = Vec::new();
    let mut in_cache = Vec::new();
    for block in server_info.start_block..server_info.end_block {
        keys.push(dht_id(&format!("{}.{}", prefix, block)));
        subkeys.push(subkey.clone());
        values.push(info_bytes.clone());
        expirations.push(expired);
        in_cache.push(false);
    }
    let block_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys,
        subkeys,
        values,
        expiration_time: expirations,
        in_cache,
        peer: Some(node_info.clone()),
    };
    {
        let g = storage.read().await;
        let _ = g.handle_store(block_req.clone());
    }
    send_to_bootstrap(client, bootstrap_peers, block_req).await;

    // VPK record — only if this node had VPK enabled
    if let Some(ref vpk) = server_info.vpk_info {
        if let Ok(vpk_bytes) = vpk.to_msgpack_bytes() {
            let vpk_req = StoreRequest {
                auth: Some(RequestAuthInfo::new()),
                keys: vec![dht_id("_kwaai.vpk.nodes")],
                subkeys: vec![subkey.clone()],
                values: vec![vpk_bytes],
                expiration_time: vec![expired],
                in_cache: vec![false],
                peer: Some(node_info),
            };
            {
                let g = storage.read().await;
                let _ = g.handle_store(vpk_req.clone());
            }
            send_to_bootstrap(client, bootstrap_peers, vpk_req).await;
        }
    }

    info!("Unannounced from DHT — node removed from map");
}

#[allow(clippy::too_many_arguments)]
async fn announce(
    client: &mut kwaai_p2p_daemon::P2PClient,
    peer_id: PeerId,
    storage: &SharedStorage,
    bootstrap_peers: &[String],
    prefix: &str,
    repository: &str,
    total_blocks: i32,
    start_block: i32,
    end_block: i32,
    server_info: &DHTServerInfo,
    rep: Option<&mut crate::reputation::ReputationStore>,
) -> Result<()> {
    info!(
        "DHT prefix: {} (blocks .{} – .{})",
        prefix,
        start_block,
        end_block - 1
    );

    let info_bytes = server_info.to_msgpack()?;
    let subkey = rmp_serde::to_vec(&peer_id.to_base58())?;
    let node_info = NodeInfo::from_peer_id(peer_id);

    // Build block STORE request — always announce configured blocks so the node
    // appears on the map. State=0 (joining) when shard is not yet loaded.
    {
        let mut keys = Vec::new();
        let mut subkeys = Vec::new();
        let mut values = Vec::new();
        let mut expirations = Vec::new();
        let mut in_cache = Vec::new();

        for block in start_block..end_block {
            keys.push(dht_id(&format!("{}.{}", prefix, block)));
            subkeys.push(subkey.clone());
            values.push(info_bytes.clone());
            expirations.push(get_dht_time() + 360.0);
            in_cache.push(false);
        }

        let block_req = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys,
            subkeys,
            values,
            expiration_time: expirations,
            in_cache,
            peer: Some(node_info.clone()),
        };

        // Store locally
        {
            let g = storage.read().await;
            let _ = g.handle_store(block_req.clone());
        }

        // Push to bootstrap peers; piggyback reputation observations on the
        // STORE latency — no extra RPCs needed.
        let (ok, timings) = send_to_bootstrap(client, bootstrap_peers, block_req).await;
        if ok {
            info!("✅ Announced {} blocks", end_block - start_block);
        } else {
            warn!("❌ Block announcement failed — node will not appear on map");
        }
        if let Some(rep) = rep {
            use crate::reputation::{now_secs, PeerObservation};
            for (peer_id_str, addr, latency_ms, success) in timings {
                // Use the DNS hostname from the multiaddr as the display name
                // (e.g. "bootstrap-1.kwaai.ai").  Fall back to the first 12
                // chars of the peer ID when no /dns/ component is present.
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
    }

    // Model registry entry
    let model_info = ModelInfo {
        num_blocks: total_blocks,
        repository: repository.to_string(),
    };
    let registry_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id("_petals.models")],
        subkeys: vec![rmp_serde::to_vec(&prefix)?],
        values: vec![model_info.to_msgpack()?],
        expiration_time: vec![get_dht_time() + 360.0],
        in_cache: vec![false],
        peer: Some(node_info.clone()),
    };

    {
        let g = storage.read().await;
        let _ = g.handle_store(registry_req.clone());
    }
    if send_to_bootstrap(client, bootstrap_peers, registry_req)
        .await
        .0
    {
        info!("✅ Announced model to _petals.models registry");
    } else {
        warn!("❌ Model registry announcement failed");
    }

    // VPK nodes registry — advertise this node's VPK capability when enabled.
    // Key: _kwaai.vpk.nodes  subkey: msgpack(peer_id_base58)
    // Value: msgpack({ mode, endpoint, capacity_gb, tenant_count, vpk_version })
    // TTL: 360 s (refreshed every 120 s together with block records)
    if let Some(ref vpk) = server_info.vpk_info {
        let vpk_req = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![dht_id("_kwaai.vpk.nodes")],
            subkeys: vec![subkey.clone()],
            values: vec![vpk.to_msgpack_bytes()?],
            expiration_time: vec![get_dht_time() + 360.0],
            in_cache: vec![false],
            peer: Some(node_info),
        };

        {
            let g = storage.read().await;
            let _ = g.handle_store(vpk_req.clone());
        }
        if send_to_bootstrap(client, bootstrap_peers, vpk_req).await.0 {
            info!("✅ Announced VPK capability to _kwaai.vpk.nodes");
        } else {
            warn!("❌ VPK nodes announcement failed");
        }
    }

    Ok(())
}

/// Send a STORE request to each bootstrap peer.
///
/// Returns `(any_success, per_peer_timings)` where timings are
/// `(peer_id_str, addr, latency_ms, success)` — one entry per bootstrap peer.
/// Callers that record reputation use the timings; others discard them.
///
/// Does NOT call connect_peer() — p2pd already has bootstrap addresses from
/// its own DHT bootstrap (via -b flag) and dials internally when needed.
/// Calling connect_peer() to Hivemind bootstrap servers crashes p2pd's
/// background goroutines via the gRPC control path.
async fn send_to_bootstrap(
    client: &mut kwaai_p2p_daemon::P2PClient,
    bootstrap_peers: &[String],
    req: StoreRequest,
) -> (bool, Vec<(String, String, f64, bool)>) {
    if bootstrap_peers.is_empty() {
        return (false, vec![]);
    }

    use prost::Message;
    let mut bytes = Vec::new();
    if let Err(e) = req.encode(&mut bytes) {
        warn!("Encode STORE request failed: {}", e);
        return (false, vec![]);
    }

    let mut succeeded = 0usize;
    let mut timings: Vec<(String, String, f64, bool)> = Vec::with_capacity(bootstrap_peers.len());

    for addr in bootstrap_peers {
        let Some(peer_id_str) = addr.split("/p2p/").nth(1) else {
            warn!("Bootstrap peer has no /p2p/ component: {}", addr);
            continue;
        };
        let bp = match peer_id_str.parse::<PeerId>() {
            Ok(p) => p,
            Err(e) => {
                warn!("Invalid peer ID in {}: {}", addr, e);
                continue;
            }
        };

        let t0 = std::time::Instant::now();
        let result = tokio::time::timeout(
            Duration::from_secs(30),
            client.call_unary_handler(&bp.to_bytes(), "DHTProtocol.rpc_store", &bytes),
        )
        .await;
        let latency_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let rpc_ok = match &result {
            Ok(Ok(_)) => true,
            _ => false,
        };
        timings.push((peer_id_str.to_string(), addr.clone(), latency_ms, rpc_ok));

        match result {
            Ok(Ok(resp_bytes)) => {
                use kwaai_hivemind_dht::protocol::StoreResponse;
                if let Ok(resp) = StoreResponse::decode(&resp_bytes[..]) {
                    let ok = resp.store_ok.iter().filter(|&&s| s).count();
                    info!(
                        "STORE response from {}: {}/{} stored",
                        peer_id_str,
                        ok,
                        resp.store_ok.len()
                    );
                    if ok > 0 {
                        succeeded += 1;
                    }
                }
            }
            Ok(Err(e)) => warn!("STORE RPC failed ({}): {}", addr, e),
            Err(_) => {
                warn!("STORE RPC timeout ({}): exceeded 30s", addr);
            }
        }
    }

    if succeeded > 0 {
        info!(
            "✅ Announced to {} of {} bootstrap peers",
            succeeded,
            bootstrap_peers.len()
        );
    } else {
        warn!(
            "❌ Announcement failed on all {} bootstrap peers — see warnings above",
            bootstrap_peers.len()
        );
    }
    (succeeded > 0, timings)
}

/// Wait for p2pd's own DHT bootstrap to establish connections.
///
/// p2pd bootstraps independently using the -b addresses it was started with.
/// We must NOT call connect_peer() here — doing so via the gRPC control path
/// crashes p2pd's background goroutines when the target speaks Hivemind DHT
/// (not the standard go-libp2p-daemon control protocol).
///
/// Instead, we poll list_peers() until p2pd shows at least one bootstrap peer
/// connected (via its own bootstrap walk) or the 30 s timeout expires.
async fn dial_and_wait_for_bootstrap(
    client: &mut kwaai_p2p_daemon::P2PClient,
    bootstrap_peers: &[String],
) -> Result<()> {
    const MAX_WAIT_SECS: u64 = 30;
    const POLL_INTERVAL_MS: u64 = 500;
    // How many consecutive IO errors before we assume p2pd has crashed and
    // bail early (avoids waiting the full 30 s when the socket is gone).
    const IO_ERROR_CRASH_THRESHOLD: usize = 5;

    let start = tokio::time::Instant::now();
    let max_wait = Duration::from_secs(MAX_WAIT_SECS);

    // Extract bootstrap peer IDs as base58 strings for matching.
    // list_peers() returns raw protobuf bytes which don't match PeerId::to_bytes()
    // (multihash-prefixed encoding), so we compare via base58 strings instead.
    let bootstrap_peer_ids: Vec<String> = bootstrap_peers
        .iter()
        .filter_map(|addr| addr.split("/p2p/").nth(1))
        .map(|s| s.to_string())
        .collect();

    let mut consecutive_io_errors: usize = 0;

    loop {
        // Query connected peers from p2pd
        match client.list_peers().await {
            Ok(peers) => {
                consecutive_io_errors = 0;

                // Check if any connected peer matches bootstrap peers.
                // Decode raw bytes → PeerId → base58 for proper comparison.
                let connected_bootstrap_count = peers
                    .iter()
                    .filter(|peer_info| match PeerId::from_bytes(&peer_info.id) {
                        Ok(pid) => bootstrap_peer_ids.contains(&pid.to_base58()),
                        Err(_) => false,
                    })
                    .count();

                if connected_bootstrap_count > 0 {
                    let elapsed = start.elapsed();
                    info!(
                        "✅ Connected to {} bootstrap peer(s) in {:.1}s",
                        connected_bootstrap_count,
                        elapsed.as_secs_f64()
                    );
                    return Ok(());
                }

                // Log progress every 5 seconds
                let elapsed = start.elapsed();
                if elapsed.as_secs() % 5 == 0 && elapsed.as_millis() < POLL_INTERVAL_MS as u128 * 2
                {
                    info!("   Waiting for bootstrap peers... ({:.0}s elapsed, {} total peers connected)",
                          elapsed.as_secs_f64(), peers.len());
                }
            }
            Err(e) => {
                let err_str = format!("{}", e);
                // IO errors (EPIPE, ENOENT) mean p2pd has crashed or its
                // socket is gone — bail early instead of waiting the full 30 s.
                if err_str.contains("Broken pipe")
                    || err_str.contains("No such file")
                    || err_str.contains("Connection refused")
                    || err_str.contains("early eof")
                {
                    consecutive_io_errors += 1;
                    if consecutive_io_errors >= IO_ERROR_CRASH_THRESHOLD {
                        warn!(
                            "p2pd appears crashed ({} consecutive IO errors) — \
                             aborting bootstrap wait early",
                            consecutive_io_errors
                        );
                        return Ok(());
                    }
                } else {
                    consecutive_io_errors = 0;
                }
                warn!("Peer list query failed: {} — continuing to wait", e);
            }
        }

        // Check timeout
        if start.elapsed() >= max_wait {
            warn!(
                "⚠️  Bootstrap timeout after {}s — no bootstrap peers visible yet",
                MAX_WAIT_SECS
            );
            warn!("   Node will still announce, but may not be visible on map initially");
            return Ok(());
        }

        // Wait before next poll
        tokio::time::sleep(Duration::from_millis(POLL_INTERVAL_MS)).await;
    }
}

// ---------------------------------------------------------------------------
// Incoming RPC stream handler
// ---------------------------------------------------------------------------

/// Try to decode `bytes` as a protobuf `T`.  If that fails, try again after
/// stripping a leading unsigned-varint length prefix (Hivemind Python sends
/// `encode_uvarint(len) + protobuf_bytes`; our own nodes send raw protobuf).
/// Returns the decoded message and whether the prefix was present (so the
/// response can be framed consistently).
fn decode_with_varint_fallback<T: prost::Message + Default>(bytes: &[u8]) -> Result<(T, bool)> {
    // Try 1: raw protobuf (our own nodes)
    if let Ok(msg) = T::decode(bytes) {
        return Ok((msg, false));
    }
    // Try 2: varint-length-prefixed (Hivemind Python compat)
    if !bytes.is_empty() {
        if let Ok((len, rest)) = unsigned_varint::decode::usize(bytes) {
            if rest.len() >= len {
                if let Ok(msg) = T::decode(&rest[..len]) {
                    return Ok((msg, true));
                }
            }
        }
    }
    // Both failed — return the raw-decode error for the original bytes
    T::decode(bytes)
        .map(|m| (m, false))
        .map_err(|e| anyhow::anyhow!("{}", e))
}

async fn handle_rpc_stream(tcp: &mut tokio::net::TcpStream, storage: SharedStorage) -> Result<()> {
    let info = stream::parse_stream_info(tcp)
        .await
        .map_err(|e| anyhow::anyhow!("parse stream info: {}", e))?;
    info!("RPC {}", info.proto);

    use prost::Message as _;
    use tokio::io::AsyncReadExt as _;
    let mut bytes = Vec::new();
    tcp.read_to_end(&mut bytes)
        .await
        .map_err(|e| anyhow::anyhow!("read request: {}", e))?;

    let (req, varint_framed) = match info.proto.as_str() {
        "DHTProtocol.rpc_store" => {
            let (r, vf) =
                decode_with_varint_fallback::<kwaai_hivemind_dht::protocol::StoreRequest>(&bytes)
                    .map_err(|e| anyhow::anyhow!("decode StoreRequest: {}", e))?;
            (DHTRequest::Store(r), vf)
        }
        "DHTProtocol.rpc_find" => {
            let (r, vf) =
                decode_with_varint_fallback::<kwaai_hivemind_dht::protocol::FindRequest>(&bytes)
                    .map_err(|e| anyhow::anyhow!("decode FindRequest: {}", e))?;
            (DHTRequest::Find(r), vf)
        }
        _ => {
            let (r, vf) =
                decode_with_varint_fallback::<kwaai_hivemind_dht::protocol::PingRequest>(&bytes)
                    .map_err(|e| anyhow::anyhow!("decode PingRequest: {}", e))?;
            (DHTRequest::Ping(r), vf)
        }
    };

    let response_bytes = {
        let g = storage.read().await;
        let resp = g
            .handle_request(req)
            .map_err(|e| anyhow::anyhow!("handle_request: {}", e))?;
        use kwaai_hivemind_dht::codec::DHTResponse;
        let raw = match resp {
            DHTResponse::Store(r) => r.encode_to_vec(),
            DHTResponse::Find(r) => r.encode_to_vec(),
            DHTResponse::Ping(r) => r.encode_to_vec(),
        };
        // Mirror the request framing so the caller can parse the response.
        if varint_framed {
            let mut buf = unsigned_varint::encode::usize_buffer();
            let prefix = unsigned_varint::encode::usize(raw.len(), &mut buf);
            let mut framed = Vec::with_capacity(prefix.len() + raw.len());
            framed.extend_from_slice(prefix);
            framed.extend_from_slice(&raw);
            framed
        } else {
            raw
        }
    };

    tcp.write_all(&response_bytes).await?;
    tcp.flush().await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Passive reputation probing
// ---------------------------------------------------------------------------

/// Send a lightweight DHT find to each bootstrap peer and record latency +
/// connectivity in the reputation store. Called every 120 s from the event loop.
/// Return `base ± spread` seconds using a fast LCG over the current nanosecond
/// timestamp. No `rand` crate needed. Range: `[base - spread, base + spread]`.
fn jitter_secs(base: u64, spread: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0) as u64;
    let r = ns
        .wrapping_mul(6_364_136_223_846_793_005_u64)
        .wrapping_add(1_442_695_040_888_963_407_u64);
    let range = 2 * spread + 1;
    base - spread + (r >> 32) % range
}

// ---------------------------------------------------------------------------
// Auto-update
// ---------------------------------------------------------------------------

/// Check for a newer release and, if found, install it automatically.
/// After a successful install the daemon exits cleanly so the OS service
/// manager (systemd, launchd) or the user can restart it with the new binary.
/// On Windows the installer batch runs detached and kills the process itself.
///
/// Returns `true` when an update was installed and the caller should break
/// the event loop to let the daemon exit.
async fn maybe_auto_update() -> bool {
    let checker = crate::updater::UpdateChecker::new();
    let update = match checker.check(false).await {
        Ok(Some(u)) => u,
        _ => return false,
    };

    info!(
        "Auto-update: new version {} available — installing…",
        update.version
    );

    if let Err(e) = checker.install_update(&update.version).await {
        warn!("Auto-update install failed: {e}");
        return false;
    }

    #[cfg(unix)]
    {
        info!(
            "Auto-update: v{} installed — daemon exiting for restart with new binary.",
            update.version
        );
        return true;
    }

    #[cfg(not(unix))]
    {
        // Windows: the installer batch kills and replaces the process.
        info!("Auto-update: installer launched — daemon will be replaced when batch completes.");
        return false;
    }
}

// ---------------------------------------------------------------------------
// Signal handling
// ---------------------------------------------------------------------------

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut sigterm =
            signal::unix::signal(signal::unix::SignalKind::terminate()).expect("SIGTERM handler");
        tokio::select! {
            _ = signal::ctrl_c() => { info!("Received Ctrl-C"); }
            _ = sigterm.recv()   => { info!("Received SIGTERM"); }
        }
    }
    #[cfg(not(unix))]
    {
        signal::ctrl_c().await.expect("Ctrl-C handler");
        info!("Received Ctrl-C");
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Poll the local VPK health endpoint (non-blocking, 3 s timeout).
/// Returns the parsed JSON body on a 2xx response, None otherwise.
async fn check_vpk_health(port: u16) -> Option<serde_json::Value> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok()?;
    let url = format!("http://localhost:{}/api/health", port);
    let resp = client.get(&url).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.json::<serde_json::Value>().await.ok()
}

#[allow(dead_code)]
fn find_free_port(preferred: u16) -> Option<u16> {
    if port_is_free(preferred) {
        return Some(preferred);
    }
    ((preferred + 1)..=(preferred + 100)).find(|&p| port_is_free(p))
}

#[allow(dead_code)]
fn port_is_free(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}

/// Compute effective throughput from the cached benchmark result.
///
/// Re-reads `~/.kwaainet/throughput_cache.json` on every call so that a
/// `kwaainet benchmark` run after the daemon started is reflected within
/// the next re-announcement cycle (120 s).
///
/// `dl_bps` is the download bandwidth measured at startup and reused here
/// to avoid a slow network probe on every re-announce.
fn compute_effective_tps(model: &str, dl_bps: f64, using_relay: bool) -> f64 {
    match crate::throughput::load(model) {
        Some(entry) => crate::throughput::effective_tps(&entry, dl_bps, using_relay),
        None => 10.0, // fallback until benchmark is run
    }
}

/// Spawn a fresh p2pd and reconnect the P2PClient.
/// Called by the watchdog when the daemon detects p2pd has exited.
async fn restart_p2pd(
    daemon: &mut P2PDaemon,
    client: &mut kwaai_p2p_daemon::P2PClient,
    p2pd_path: &Option<std::path::PathBuf>,
    config: &crate::config::KwaaiNetConfig,
    bootstrap_peers: &[String],
    announce_addr: &Option<String>,
    handler_addr: std::net::SocketAddr,
) -> Result<()> {
    // Drop the dead daemon handle (reaps the zombie if still pending).
    let _ = daemon.shutdown().await;

    let host_addr = format!("/ip4/0.0.0.0/tcp/{}", config.port);
    let identity_key_path = crate::identity::NodeIdentity::key_file_path();

    let builder = P2PDaemon::builder()
        .dht(true)
        .bootstrap(!bootstrap_peers.is_empty())
        .relay(!config.no_relay)
        .auto_relay(true)
        .auto_nat(true)
        .force_reachability_private(config.force_private)
        .nat_portmap(true)
        .host_addrs([host_addr])
        .bootstrap_peers(bootstrap_peers.to_vec())
        .trusted_relays(config.trusted_relays.clone())
        .with_identity_key(&identity_key_path);

    let builder = match announce_addr {
        Some(addr) => builder.announce_addrs([addr.as_str()]),
        None => builder,
    };
    let builder = match p2pd_path {
        Some(path) => builder.with_binary_path(path),
        None => builder,
    };
    let builder = if let Ok(sock) = std::env::var("KWAAINET_SOCKET") {
        #[cfg(unix)]
        let addr = format!("/unix/{}", sock);
        #[cfg(not(unix))]
        let addr = sock;
        builder.with_listen_addr(addr)
    } else {
        builder
    };

    let mut new_daemon = builder.spawn().await.context("restarting p2pd")?;
    let mut new_client = new_daemon.client().await.context("p2pd client (restart)")?;

    new_client
        .register_stream_handler(
            &format!("/ip4/127.0.0.1/tcp/{}", handler_addr.port()),
            vec![
                "DHTProtocol.rpc_ping".to_string(),
                "DHTProtocol.rpc_store".to_string(),
                "DHTProtocol.rpc_find".to_string(),
            ],
        )
        .await
        .context("re-registering stream handlers")?;

    dial_and_wait_for_bootstrap(&mut new_client, bootstrap_peers).await?;

    *daemon = new_daemon;
    *client = new_client;

    // The storage API holds a persistent connection to p2pd and registers
    // /kwaai/storage/1.0.0 once at startup. Since p2pd just restarted, that
    // connection is dead and the handler is gone. Restart the storage API so
    // it reconnects and re-registers with the fresh p2pd.
    #[cfg(feature = "storage")]
    if config.storage.is_some() {
        let storage_mgr = crate::daemon::StorageApiManager::new();
        if storage_mgr.is_running() {
            storage_mgr.stop_process();
        }
        match crate::daemon::StorageApiManager::spawn_storage_child() {
            Ok(pid) => {
                storage_mgr.write_pid(pid);
                info!("✅ storage API restarted after p2pd restart (PID {})", pid);
            }
            Err(e) => warn!("failed to restart storage API after p2pd restart: {}", e),
        }
    }

    Ok(())
}

fn find_p2pd_binary() -> Option<std::path::PathBuf> {
    #[cfg(windows)]
    let name = "p2pd.exe";
    #[cfg(not(windows))]
    let name = "p2pd";

    // Next to our own binary
    if let Ok(exe) = std::env::current_exe() {
        let c = exe.parent()?.join(name);
        if c.exists() {
            return Some(c);
        }
    }
    // Cargo target dir (dev builds)
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        #[cfg(windows)]
        let c = std::path::PathBuf::from(manifest).join("../../../target/debug/p2pd.exe");
        #[cfg(not(windows))]
        let c = std::path::PathBuf::from(manifest).join("../../../target/debug/p2pd");
        if c.exists() {
            return Some(c);
        }
    }
    // PATH
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let c = dir.join(name);
        if c.exists() {
            return Some(c);
        }
    }
    None
}
