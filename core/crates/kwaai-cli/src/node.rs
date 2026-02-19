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
use crate::daemon::DaemonManager;

type SharedStorage = Arc<RwLock<DHTStorage>>;

// ---------------------------------------------------------------------------
// DHT value types (Hivemind wire format)
// ---------------------------------------------------------------------------

/// Server info serialised as ExtType(64, [state, throughput, {fields}])
/// — the exact format Python Hivemind / map.kwaai.ai expects.
struct DHTServerInfo {
    state: i32,
    throughput: f64,
    start_block: i32,
    end_block: i32,
    public_name: String,
    version: String,
    torch_dtype: String,
    using_relay: bool,
    cache_tokens_left: i64,
    next_pings: HashMap<String, f64>,
    adapters: Vec<String>,
}

impl DHTServerInfo {
    fn new(start: i32, end: i32, name: &str, relay: bool) -> Self {
        Self {
            state: 2, // ONLINE
            throughput: 100.0,
            start_block: start,
            end_block: end,
            public_name: name.to_string(),
            version: concat!("kwaai-", env!("CARGO_PKG_VERSION")).to_string(),
            torch_dtype: "float16".to_string(),
            using_relay: relay,
            cache_tokens_left: 100_000,
            next_pings: HashMap::new(),
            adapters: vec![],
        }
    }

    fn to_msgpack(&self) -> Result<Vec<u8>> {
        let mut fields: Vec<(rmpv::Value, rmpv::Value)> = vec![
            (rmpv::Value::from("start_block"),       rmpv::Value::from(self.start_block)),
            (rmpv::Value::from("end_block"),         rmpv::Value::from(self.end_block)),
            (rmpv::Value::from("public_name"),       rmpv::Value::from(self.public_name.as_str())),
            (rmpv::Value::from("version"),           rmpv::Value::from(self.version.as_str())),
            (rmpv::Value::from("torch_dtype"),       rmpv::Value::from(self.torch_dtype.as_str())),
            (rmpv::Value::from("using_relay"),       rmpv::Value::from(self.using_relay)),
            (rmpv::Value::from("cache_tokens_left"), rmpv::Value::from(self.cache_tokens_left)),
            (rmpv::Value::from("adapters"),          rmpv::Value::Array(vec![])),
            (rmpv::Value::from("next_pings"),        rmpv::Value::Map(vec![])),
        ];

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
            (rmpv::Value::from("repository"), rmpv::Value::from(self.repository.as_str())),
            (rmpv::Value::from("num_blocks"),  rmpv::Value::from(self.num_blocks)),
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

/// Convert a HuggingFace model name to the Hivemind DHT prefix.
/// "unsloth/Llama-3.1-8B-Instruct" → "unsloth-Llama-3-1-8B-Instruct"
fn dht_prefix(model: &str) -> String {
    model.replace('.', "-").replace('/', "-")
}

fn model_total_blocks(model: &str) -> i32 {
    let m = model.to_lowercase();
    if m.contains("70b") { 80 } else if m.contains("13b") { 40 } else { 32 }
}

fn hf_url(model: &str) -> String {
    if model.contains('/') {
        format!("https://huggingface.co/{}", model)
    } else {
        format!("https://huggingface.co/meta-llama/{}", model)
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

pub async fn run_node(config: &KwaaiNetConfig) -> Result<()> {
    // PID tracking
    let daemon_mgr = DaemonManager::new();
    daemon_mgr.write_pid(std::process::id()).context("writing PID")?;
    info!("KwaaiNet node starting (PID {})", std::process::id());

    let public_name = config
        .public_name
        .clone()
        .unwrap_or_else(|| "kwaainet-node".to_string());

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

    // p2pd listens for P2P traffic on the configured port
    let host_addr = format!("/ip4/0.0.0.0/tcp/{}", config.port);

    // Announce the public IP so the health monitor can reach us
    let announce_addr = config.public_ip.as_deref().map(|ip| {
        format!("/ip4/{}/tcp/{}", ip, config.port)
    });

    let builder = P2PDaemon::builder()
        .dht(true)
        .relay(!config.no_relay)
        .auto_relay(true)
        .auto_nat(true)
        .nat_portmap(true)
        .host_addrs([host_addr])
        .bootstrap_peers(bootstrap_peers.clone());

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

    let mut daemon = builder.spawn().await.context("starting p2pd")?;
    let mut client = daemon.client().await.context("p2pd client")?;

    let peer_id_hex = client.identify().await.context("identify peer")?;
    let peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)
        .context("parse peer ID")?;
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

    // -----------------------------------------------------------------------
    // Step 4: Wait for DHT bootstrap
    // -----------------------------------------------------------------------
    info!("[4/5] Bootstrapping (30 seconds)...");
    tokio::time::sleep(Duration::from_secs(30)).await;

    // -----------------------------------------------------------------------
    // Step 5: Initial DHT announcement
    // -----------------------------------------------------------------------
    info!("[5/5] Announcing to DHT...");
    let server_info = DHTServerInfo::new(
        0,
        config.blocks as i32,
        &public_name,
        !config.no_relay,
    );
    announce(
        &mut client,
        peer_id,
        &storage,
        &bootstrap_peers,
        &config.model,
        0,
        config.blocks as i32,
        &server_info,
    )
    .await
    .context("initial DHT announcement")?;

    info!("✅ KwaaiNet node running");
    info!("   Peer ID : {}", peer_id.to_base58());
    info!("   Name    : {}", public_name);
    info!("   Model   : {}", config.model);
    info!("   Blocks  : 0–{}", config.blocks);
    info!("   Map     : https://map.kwaai.ai");

    // -----------------------------------------------------------------------
    // Event loop: handle incoming RPC + periodic re-announce
    // -----------------------------------------------------------------------
    let storage_clone = storage.clone();
    let mut reannounce = tokio::time::interval(Duration::from_secs(120));
    reannounce.tick().await; // skip the immediate first tick

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

            // Periodic re-announcement
            _ = reannounce.tick() => {
                info!("Re-announcing to DHT...");
                if let Err(e) = announce(
                    &mut client, peer_id, &storage, &bootstrap_peers,
                    &config.model, 0, config.blocks as i32, &server_info,
                ).await {
                    warn!("Re-announce failed: {}", e);
                }
            }

            // Shutdown signal
            _ = shutdown_signal() => {
                info!("Shutdown signal received");
                break;
            }
        }
    }

    let _ = daemon.shutdown().await;
    daemon_mgr.remove_pid();
    info!("KwaaiNet node stopped");
    Ok(())
}

// ---------------------------------------------------------------------------
// DHT announcement
// ---------------------------------------------------------------------------

async fn announce(
    client: &mut kwaai_p2p_daemon::P2PClient,
    peer_id: PeerId,
    storage: &SharedStorage,
    bootstrap_peers: &[String],
    model: &str,
    start_block: i32,
    end_block: i32,
    server_info: &DHTServerInfo,
) -> Result<()> {
    let prefix = dht_prefix(model);
    info!("DHT prefix: {} (blocks .{} – .{})", prefix, start_block, end_block - 1);

    let info_bytes = server_info.to_msgpack()?;
    let subkey = rmp_serde::to_vec(&peer_id.to_base58())?;
    let node_info = NodeInfo::from_peer_id(peer_id);

    // Build block STORE request
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
    { let g = storage.read().await; let _ = g.handle_store(block_req.clone()); }

    // Push to bootstrap peer
    send_to_bootstrap(client, bootstrap_peers, block_req).await;
    info!("✅ Announced {} blocks", end_block - start_block);

    // Model registry entry
    let model_info = ModelInfo {
        num_blocks: model_total_blocks(model),
        repository: hf_url(model),
    };
    let registry_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id("_petals.models")],
        subkeys: vec![rmp_serde::to_vec(&prefix)?],
        values: vec![model_info.to_msgpack()?],
        expiration_time: vec![get_dht_time() + 360.0],
        in_cache: vec![false],
        peer: Some(node_info),
    };

    { let g = storage.read().await; let _ = g.handle_store(registry_req.clone()); }
    send_to_bootstrap(client, bootstrap_peers, registry_req).await;
    info!("✅ Announced model to _petals.models registry");

    Ok(())
}

/// Connect to the first bootstrap peer and send a STORE request.
async fn send_to_bootstrap(
    client: &mut kwaai_p2p_daemon::P2PClient,
    bootstrap_peers: &[String],
    req: StoreRequest,
) {
    let Some(addr) = bootstrap_peers.first() else { return };

    match client.connect_peer(addr).await {
        Err(e) => { warn!("Bootstrap connect failed: {}", e); return; }
        Ok(_) => {}
    }
    tokio::time::sleep(Duration::from_secs(2)).await;

    let Some(peer_id_str) = addr.split("/p2p/").nth(1) else { return };
    let Ok(bp) = peer_id_str.parse::<PeerId>() else { return };

    use prost::Message;
    let mut bytes = Vec::new();
    if let Err(e) = req.encode(&mut bytes) {
        warn!("Encode STORE request failed: {}", e);
        return;
    }

    match client.call_unary_handler(&bp.to_bytes(), "DHTProtocol.rpc_store", &bytes).await {
        Ok(resp_bytes) => {
            use kwaai_hivemind_dht::protocol::StoreResponse;
            if let Ok(resp) = StoreResponse::decode(&resp_bytes[..]) {
                let ok = resp.store_ok.iter().filter(|&&s| s).count();
                info!("STORE response: {}/{} stored", ok, resp.store_ok.len());
            }
        }
        Err(e) => warn!("STORE RPC failed: {}", e),
    }
}

// ---------------------------------------------------------------------------
// Incoming RPC stream handler
// ---------------------------------------------------------------------------

async fn handle_rpc_stream(
    tcp: &mut tokio::net::TcpStream,
    storage: SharedStorage,
) -> Result<()> {
    let info = stream::parse_stream_info(tcp)
        .await
        .map_err(|e| anyhow::anyhow!("parse stream info: {}", e))?;
    info!("RPC {}", info.proto);

    let bytes = stream::read_varint_framed(tcp)
        .await
        .map_err(|e| anyhow::anyhow!("read frame: {}", e))?;

    let req = DHTRequest::decode(&bytes)
        .map_err(|e| anyhow::anyhow!("decode DHTRequest: {}", e))?;

    let response_bytes = {
        let g = storage.read().await;
        let resp = g
            .handle_request(req)
            .map_err(|e| anyhow::anyhow!("handle_request: {}", e))?;
        resp.encode()
            .map_err(|e| anyhow::anyhow!("encode DHTResponse: {}", e))?
    };

    stream::write_varint_framed(tcp, &response_bytes)
        .await
        .map_err(|e| anyhow::anyhow!("write frame: {}", e))?;
    tcp.flush().await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Signal handling
// ---------------------------------------------------------------------------

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("SIGTERM handler");
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

fn find_free_port(preferred: u16) -> Option<u16> {
    if port_is_free(preferred) { return Some(preferred); }
    for p in (preferred + 1)..=(preferred + 100) {
        if port_is_free(p) { return Some(p); }
    }
    None
}

fn port_is_free(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}

fn find_p2pd_binary() -> Option<std::path::PathBuf> {
    // Next to our own binary
    if let Ok(exe) = std::env::current_exe() {
        let c = exe.parent()?.join("p2pd");
        if c.exists() { return Some(c); }
    }
    // Cargo target dir (dev builds)
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let c = std::path::PathBuf::from(manifest).join("../../../target/debug/p2pd");
        if c.exists() { return Some(c); }
    }
    // PATH
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let c = dir.join("p2pd");
        if c.exists() { return Some(c); }
    }
    None
}
