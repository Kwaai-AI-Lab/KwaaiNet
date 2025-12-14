//! Petals-Visible Node Example with Hivemind DHT via p2p-daemon
//!
//! This example uses go-libp2p-daemon with stream handlers to communicate
//! using Hivemind DHT protocols. The daemon handles all networking while
//! we handle Hivemind protocol messages.
//!
//! Architecture:
//! 1. Daemon handles: connectivity, DHT, NAT traversal, relay
//! 2. Stream handlers: receive and respond to Hivemind protocol requests
//! 3. Outbound requests: open streams to send STORE/FIND requests
//!
//! Run with: cargo run --release --example petals_visible -- --name "My-Node"
//!
//! After running, check map.petals.dev to see if your node appears.

use kwaai_hivemind_dht::{
    codec::{DHTRequest, DHTResponse},
    protocol::{FindResponse, FindResult, NodeInfo, PingRequest, PingResponse, RequestAuthInfo, ResponseAuthInfo, ResultType, StoreRequest, StoreResponse},
    value::get_dht_time,
    DHTStorage,
};
use prost::Message;
use kwaai_p2p::{hivemind::ServerInfo, NetworkConfig};
use kwaai_p2p_daemon::{stream, P2PDaemon};
use libp2p::PeerId;
use sha1::{Sha1, Digest};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::RwLock,
};
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Shared DHT storage for this node
type SharedStorage = Arc<RwLock<DHTStorage>>;

// =============================================================================
// DHT ServerInfo Structure (for DHT announcements)
// =============================================================================

/// ServerInfo structure for DHT STORE announcements
/// This matches the exact structure used by Petals servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DHTServerInfo {
    /// Server state: 0=OFFLINE, 1=JOINING, 2=ONLINE
    pub state: i32,

    /// Throughput in tokens/second
    pub throughput: f64,

    /// First block index this server hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_block: Option<i32>,

    /// Last block index + 1 (exclusive end)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_block: Option<i32>,

    /// Human-readable server name (displayed on map.petals.dev)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,

    /// Software version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Network requests per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_rps: Option<f64>,

    /// Forward pass requests per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_rps: Option<f64>,

    /// Inference requests per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_rps: Option<f64>,

    /// PyTorch dtype: "float16", "bfloat16", "float32"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub torch_dtype: Option<String>,

    /// Quantization type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quant_type: Option<String>,

    /// List of adapter names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters: Option<Vec<String>>,

    /// Whether server uses relay
    #[serde(skip_serializing_if = "Option::is_none")]
    pub using_relay: Option<bool>,

    /// Available cache capacity in tokens
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_tokens_left: Option<i64>,

    /// Ping latencies to downstream servers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_pings: Option<HashMap<String, f64>>,
}

impl DHTServerInfo {
    /// Create new DHT ServerInfo with required fields
    pub fn new(
        state: i32,
        throughput: f64,
        start_block: i32,
        end_block: i32,
        public_name: String,
        version: String,
    ) -> Self {
        Self {
            state,
            throughput,
            start_block: Some(start_block),
            end_block: Some(end_block),
            public_name: Some(public_name),
            version: Some(version),
            network_rps: Some(10.0),      // Placeholder
            forward_rps: Some(5.0),       // Placeholder
            inference_rps: Some(5.0),     // Placeholder
            torch_dtype: Some("float16".to_string()),
            quant_type: None,
            adapters: Some(vec![]),       // Empty list for no adapters
            using_relay: Some(false),     // Will be set based on NAT config
            cache_tokens_left: Some(100000),
            next_pings: Some(HashMap::new()),
        }
    }

    /// Serialize to msgpack bytes for DHT storage
    /// Uses Petals ExtType-wrapped format: ExtType(1, [state, throughput, {field_map}])
    pub fn to_msgpack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        // Build the field map
        let mut field_map: HashMap<String, rmpv::Value> = HashMap::new();

        if let Some(start_block) = self.start_block {
            field_map.insert("start_block".to_string(), rmpv::Value::from(start_block));
        }
        if let Some(end_block) = self.end_block {
            field_map.insert("end_block".to_string(), rmpv::Value::from(end_block));
        }
        if let Some(ref public_name) = self.public_name {
            field_map.insert("public_name".to_string(), rmpv::Value::from(public_name.as_str()));
        }
        if let Some(ref version) = self.version {
            field_map.insert("version".to_string(), rmpv::Value::from(version.as_str()));
        }
        if let Some(network_rps) = self.network_rps {
            field_map.insert("network_rps".to_string(), rmpv::Value::from(network_rps));
        }
        if let Some(forward_rps) = self.forward_rps {
            field_map.insert("forward_rps".to_string(), rmpv::Value::from(forward_rps));
        }
        if let Some(inference_rps) = self.inference_rps {
            field_map.insert("inference_rps".to_string(), rmpv::Value::from(inference_rps));
        }
        if let Some(ref torch_dtype) = self.torch_dtype {
            field_map.insert("torch_dtype".to_string(), rmpv::Value::from(torch_dtype.as_str()));
        }
        if let Some(ref quant_type) = self.quant_type {
            field_map.insert("quant_type".to_string(), rmpv::Value::from(quant_type.as_str()));
        }
        if let Some(ref adapters) = self.adapters {
            let adapter_values: Vec<rmpv::Value> = adapters.iter()
                .map(|s| rmpv::Value::from(s.as_str()))
                .collect();
            field_map.insert("adapters".to_string(), rmpv::Value::Array(adapter_values));
        }
        if let Some(using_relay) = self.using_relay {
            field_map.insert("using_relay".to_string(), rmpv::Value::from(using_relay));
        }
        if let Some(cache_tokens) = self.cache_tokens_left {
            field_map.insert("cache_tokens_left".to_string(), rmpv::Value::from(cache_tokens));
        }
        if let Some(ref next_pings) = self.next_pings {
            let ping_map: Vec<(rmpv::Value, rmpv::Value)> = next_pings.iter()
                .map(|(k, v)| (rmpv::Value::from(k.as_str()), rmpv::Value::from(*v)))
                .collect();
            field_map.insert("next_pings".to_string(), rmpv::Value::Map(ping_map));
        }

        // Build the 3-element array: [state, throughput, {field_map}]
        let map_pairs: Vec<(rmpv::Value, rmpv::Value)> = field_map.into_iter()
            .map(|(k, v)| (rmpv::Value::from(k), v))
            .collect();

        let inner_array = vec![
            rmpv::Value::from(self.state),
            rmpv::Value::from(self.throughput),
            rmpv::Value::Map(map_pairs),
        ];

        // Serialize the inner array
        let mut inner_bytes = Vec::new();
        rmpv::encode::write_value(&mut inner_bytes, &rmpv::Value::Array(inner_array))
            .map_err(|_| rmp_serde::encode::Error::Syntax("Failed to encode inner array".to_string()))?;

        // Wrap in ExtType(64, ...) - Python Hivemind uses 0x40 (64) for tuples
        let ext_value = rmpv::Value::Ext(64, inner_bytes);
        let mut result = Vec::new();
        rmpv::encode::write_value(&mut result, &ext_value)
            .map_err(|_| rmp_serde::encode::Error::Syntax("Failed to encode ExtType".to_string()))?;

        Ok(result)
    }
}

// =============================================================================
// Model Info Structure (for _petals.models registry)
// =============================================================================

/// Model information for registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Total number of transformer blocks in the model
    pub num_blocks: i32,

    /// HuggingFace repository identifier
    pub repository: String,
}

impl ModelInfo {
    /// Serialize to msgpack bytes for DHT storage as a dictionary/map
    /// Python health monitor expects: {"repository": "...", "num_blocks": 32}
    pub fn to_msgpack(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Build explicit dictionary for Python compatibility
        let mut map = Vec::new();
        map.push((
            rmpv::Value::String("repository".into()),
            rmpv::Value::String(self.repository.clone().into()),
        ));
        map.push((
            rmpv::Value::String("num_blocks".into()),
            rmpv::Value::from(self.num_blocks),
        ));

        let value = rmpv::Value::Map(map);

        // Encode to bytes using rmpv
        let mut buf = Vec::new();
        rmpv::encode::write_value(&mut buf, &value)?;
        Ok(buf)
    }
}

/// Generate a DHT ID from a raw key using Hivemind's DHTID.generate() logic
/// 1. Serialize key with msgpack
/// 2. Hash with SHA1
/// 3. Return 20-byte hash
fn generate_dht_id(raw_key: &str) -> Vec<u8> {
    // Serialize with msgpack (same as Python's MSGPackSerializer.dumps())
    let msgpack_bytes = rmp_serde::to_vec(raw_key).expect("Failed to serialize key");

    // Hash with SHA1
    let mut hasher = Sha1::new();
    hasher.update(&msgpack_bytes);
    let hash = hasher.finalize();

    // Return 20-byte hash
    hash.to_vec()
}

/// Announce server blocks and model info to the DHT
async fn announce_to_dht(
    client: &mut kwaai_p2p_daemon::P2PClient,
    peer_id: PeerId,
    storage: &SharedStorage,
    config: &NetworkConfig,
    dht_server_info: &DHTServerInfo,
    model_name: &str,
    start_block: i32,
    end_block: i32,
) -> Result<(), Box<dyn Error>> {
    // Build DHT prefix from model name
    // Convert model name to DHT prefix format:
    // "Llama-3.1-8B-Instruct" -> "Meta-Llama-3-1-8B-Instruct-hf"
    let dht_prefix = if !model_name.contains("/") {
        // Add Meta- prefix if not present for Llama models
        //let prefix = if model_name.starts_with("Llama") && !model_name.starts_with("Meta-") {
        //    format!("Meta-{}", model_name)
        //} else {
        //    model_name.to_string()
        //};

        let prefix = model_name.to_string();

        // Replace dots with hyphens
        let prefix = prefix.replace(".", "-");

        // Add -hf suffix if not present
        if prefix.ends_with("-hf") {
            prefix
        } else {
            format!("{}-hf", prefix)
        }
    } else {
        // If it contains /, assume it's already in org/model format
        model_name.replace(".", "-").replace("/", "-")
    };

    info!("📋 DHT Prefix: {}", dht_prefix);
    info!("📋 Will announce blocks: {}.0 through {}.{}", dht_prefix, dht_prefix, end_block - 1);

    // Serialize DHT server info
    let info_bytes = dht_server_info.to_msgpack()?;

    // Build STORE request for block announcements
    let mut keys = Vec::new();
    let mut subkeys = Vec::new();
    let mut values = Vec::new();
    let mut expiration_times = Vec::new();
    let mut in_cache_flags = Vec::new();

    let peer_id_base58 = peer_id.to_base58();
    let subkey = rmp_serde::to_vec(&peer_id_base58)?;

    for block_num in start_block..end_block {
        let block_uid = format!("{}.{}", dht_prefix, block_num);
        let hashed_key = generate_dht_id(&block_uid);

        keys.push(hashed_key);
        subkeys.push(subkey.clone());
        values.push(info_bytes.clone());
        expiration_times.push(get_dht_time() + 360.0);
        in_cache_flags.push(false);
    }

    let num_blocks = keys.len();
    let node_info = NodeInfo::from_peer_id(peer_id);
    let store_request = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys,
        subkeys,
        values,
        expiration_time: expiration_times,
        in_cache: in_cache_flags,
        peer: Some(node_info.clone()),
    };

    // Store in local DHT
    {
        let storage_guard = storage.read().await;
        let _response = storage_guard.handle_store(store_request.clone());
    }

    // Connect to bootstrap peer and send STORE request
    if let Some(bootstrap_addr) = config.bootstrap_peers.first() {
        // First, explicitly connect to the bootstrap peer
        match client.connect_peer(bootstrap_addr).await {
            Ok(_) => {
                info!("Connected to bootstrap peer");

                // Give connection a moment to stabilize
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;

                // Now send STORE request
                if let Some(peer_id_str) = bootstrap_addr.split("/p2p/").nth(1) {
                    if let Ok(bootstrap_peer_id) = peer_id_str.parse::<PeerId>() {
                        let bootstrap_peer_id_bytes = bootstrap_peer_id.to_bytes();
                        match send_store_to_peer(client, &bootstrap_peer_id_bytes, store_request.clone()).await {
                            Ok(_) => {
                                info!("✅ Announced {} blocks to bootstrap peer", num_blocks);
                            }
                            Err(e) => {
                                warn!("Failed to announce blocks: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                warn!("Failed to connect to bootstrap peer: {}", e);
            }
        }
    }

    // Announce model info to _petals.models registry
    // NOTE: This should be the TOTAL blocks in the complete model, not what this node serves
    // Individual ServerInfo entries contain start_block/end_block for what this node serves
    let total_model_blocks = match model_name {
        "Llama-3.3-70B-Instruct" => 80,
        "Llama-3.1-8B-Instruct" => 32,
        "Llama-3.1-70B-Instruct" => 80,
        "Llama-2-70B-hf" => 80,
        _ => 80,
    };

    let model_info = ModelInfo {
        num_blocks: total_model_blocks,
        repository: format!("https://huggingface.co/meta-llama/{}", model_name),
    };

    let registry_key = generate_dht_id("_petals.models");
    let registry_subkey = rmp_serde::to_vec(&dht_prefix)?;
    let registry_value = model_info.to_msgpack()?;

    let model_store_request = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![registry_key],
        subkeys: vec![registry_subkey],
        values: vec![registry_value],
        expiration_time: vec![get_dht_time() + 360.0],
        in_cache: vec![false],
        peer: Some(node_info.clone()),
    };

    // Store in local DHT
    {
        let storage_guard = storage.read().await;
        let _response = storage_guard.handle_store(model_store_request.clone());
    }

    // Send to bootstrap peer (already connected from block announcement)
    if let Some(bootstrap_addr) = config.bootstrap_peers.first() {
        if let Some(peer_id_str) = bootstrap_addr.split("/p2p/").nth(1) {
            if let Ok(bootstrap_peer_id) = peer_id_str.parse::<PeerId>() {
                let bootstrap_peer_id_bytes = bootstrap_peer_id.to_bytes();
                match send_store_to_peer(client, &bootstrap_peer_id_bytes, model_store_request).await {
                    Ok(_) => {
                        info!("✅ Announced model info to _petals.models registry");
                    }
                    Err(e) => {
                        warn!("Failed to announce model info: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();

    let public_name = args
        .iter()
        .position(|a| a == "--name")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("kwaai-node-{}", &uuid()[..8]));

    let model_name = args
        .iter()
        .position(|a| a == "--model")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Llama-3.3-70B-Instruct".to_string());

    let version = args
        .iter()
        .position(|a| a == "--version")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "kwaai-0.1.0".to_string());

    let torch_dtype = args
        .iter()
        .position(|a| a == "--dtype")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "float16".to_string());

    let start_block: i32 = args
        .iter()
        .position(|a| a == "--start-block")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let end_block: i32 = args
        .iter()
        .position(|a| a == "--end-block")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    println!("KwaaiNet Petals-Visible Node");
    println!("============================\n");

    // Create DHT server info with all Petals-compatible fields
    // For custom bootstrap peers (not public Petals swarm), start directly in ONLINE state
    let dht_server_info = DHTServerInfo::new(
        2,  // state: ONLINE (for private swarms, skip JOINING state)
        100.0,  // throughput
        start_block,
        end_block,
        public_name.clone(),
        version.clone(),
    );

    // Also create RPC ServerInfo for Hivemind RPC protocol (future use)
    let _rpc_server_info = ServerInfo::new(&public_name)
        .with_span(start_block as u32, end_block as u32)
        .with_cache_tokens(100000)
        .with_throughput(100.0)
        .with_dtype(&torch_dtype);

    println!("Node Configuration:");
    println!("  Public Name: {}", public_name);
    println!("  Model: {}", model_name);
    println!("  Version: {}", version);
    println!("  Block Range: [{}, {})", start_block, end_block);
    println!("  Torch dtype: {}", torch_dtype);
    println!("  Throughput: {} tokens/sec", dht_server_info.throughput);
    println!();

    // Use Petals bootstrap servers
    let config = NetworkConfig::with_petals_bootstrap();
    println!("Bootstrap servers:");
    for server in &config.bootstrap_peers {
        println!("  {}", server);
    }
    println!();

    // =========================================================================
    // STEP 1: Start p2p-daemon
    // =========================================================================
    info!("[1/5] Starting p2p daemon with Petals bootstrap...");
    let mut daemon = P2PDaemon::builder()
        .dht(true) // Enable full DHT mode
        .relay(true) // Enable relay for NAT traversal
        .nat_portmap(true) // Try NAT port mapping
        .bootstrap_peers(config.bootstrap_peers.clone())
        .spawn()
        .await?;

    println!("[DAEMON] Spawned at: {}", daemon.listen_addr());

    let mut client = daemon.client().await?;
    let peer_id_hex = client.identify().await?;

    // Convert to PeerId and print in base58 format (human-readable)
    let peer_id_temp = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)?;
    println!("[PEER ID] {}", peer_id_temp.to_base58());

    // =========================================================================
    // STEP 2: Initialize DHT storage
    // =========================================================================
    info!("[2/5] Initializing local DHT storage...");

    // Convert hex peer_id to PeerId
    let peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)?;
    let storage: SharedStorage = Arc::new(RwLock::new(DHTStorage::new(peer_id)));
    println!("[STORAGE] Ready");

    // =========================================================================
    // STEP 3: Register stream handlers for incoming Hivemind requests
    // =========================================================================
    info!("[3/5] Setting up Hivemind protocol handlers...");

    // Create local listener for incoming streams from daemon
    let handler_listener = TcpListener::bind("127.0.0.1:0").await?;
    let handler_addr = handler_listener.local_addr()?;
    println!("[HANDLER] Listening on: {}", handler_addr);

    // Register stream handlers with daemon
    let handler_multiaddr = format!("/ip4/127.0.0.1/tcp/{}", handler_addr.port());
    let protocols = vec![
        "DHTProtocol.rpc_ping".to_string(),
        "DHTProtocol.rpc_store".to_string(),
        "DHTProtocol.rpc_find".to_string(),
    ];

    client
        .register_stream_handler(&handler_multiaddr, protocols.clone())
        .await?;
    println!("[HANDLER] Registered protocols: {:?}", protocols);

    // =========================================================================
    // STEP 4: Wait for DHT bootstrap
    // =========================================================================
    info!("[4/5] Waiting for DHT bootstrap (30 seconds)...");
    info!("         (giving daemon time to connect to bootstrap peers)");
    tokio::time::sleep(Duration::from_secs(30)).await;

    // =========================================================================
    // STEP 5: Initial announcement to DHT
    // =========================================================================
    info!("[5/5] Announcing server blocks and model info to DHT...");

    // Make initial announcement
    announce_to_dht(
        &mut client,
        peer_id,
        &storage,
        &config,
        &dht_server_info,
        &model_name,
        start_block,
        end_block,
    ).await?;

    // =========================================================================
    // Keep running and monitor incoming requests
    // =========================================================================
    println!("\n[STATUS] Node is running!");
    println!("         - Daemon handles: connectivity, DHT, relay");
    println!("         - Handlers ready for: PING, STORE, FIND");
    println!("         - Local DHT storage active");
    println!("         Press Ctrl+C to shutdown.\n");

    info!("========================================");
    info!("Monitoring incoming DHT requests...");
    info!("========================================");

    let mut stats_interval = tokio::time::interval(Duration::from_secs(30)); // Show stats every 30 seconds

    // Accept incoming streams from daemon
    let storage_clone = storage.clone();

    // Setup periodic re-announcement interval
    let mut announce_interval = tokio::time::interval(Duration::from_secs(120)); // Re-announce every 120 seconds
    announce_interval.tick().await; // Skip first tick (already announced above)

    loop {
        tokio::select! {
            // Handle incoming streams
            result = handler_listener.accept() => {
                match result {
                    Ok((mut stream, addr)) => {
                        info!("📥 INCOMING REQUEST from daemon: {}", addr);

                        // Clone storage for this handler
                        let storage = storage_clone.clone();

                        // Spawn handler for this stream
                        tokio::spawn(async move {
                            if let Err(e) = handle_hivemind_stream(&mut stream, storage).await {
                                warn!("Error handling stream: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                        warn!("Error accepting stream: {}", e);
                    }
                }
            }

            // Show DHT storage stats periodically
            _ = stats_interval.tick() => {
                let storage_guard = storage_clone.read().await;
                let (total, valid) = storage_guard.stats();
                info!("📊 DHT Storage: {} total entries, {} valid", total, valid);
            }

            // Re-announce to DHT periodically (every 120 seconds)
            _ = announce_interval.tick() => {
                info!("⏰ Re-announcing to DHT network...");
                match announce_to_dht(
                    &mut client,
                    peer_id,
                    &storage,
                    &config,
                    &dht_server_info,
                    &model_name,
                    start_block,
                    end_block,
                ).await {
                    Ok(_) => {
                        info!("✅ Re-announcement successful");
                    }
                    Err(e) => {
                        warn!("Failed to re-announce: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Handle an incoming Hivemind protocol stream
async fn handle_hivemind_stream(
    stream: &mut tokio::net::TcpStream,
    storage: SharedStorage,
) -> Result<(), Box<dyn Error>> {
    // Parse StreamInfo to get peer_id and protocol
    let stream_info = stream::parse_stream_info(stream).await?;

    info!("Handling {} from peer (len={})",
        stream_info.proto, stream_info.peer.len());

    // Read the Hivemind request
    let request_bytes = stream::read_varint_framed(stream).await?;

    // Log first 64 bytes of received data
    let hex_preview: String = request_bytes.iter()
        .take(64)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ");
    info!("Received {} bytes (first 64 bytes): {}", request_bytes.len(), hex_preview);

    // Decode based on protocol
    let response_bytes = match stream_info.proto.as_str() {
        "DHTProtocol.rpc_ping" => {
            handle_ping(&request_bytes, storage).await?
        }
        "DHTProtocol.rpc_store" => {
            handle_store(&request_bytes, storage).await?
        }
        "DHTProtocol.rpc_find" => {
            handle_find(&request_bytes, storage).await?
        }
        proto => {
            warn!("Unknown protocol: {}", proto);
            return Ok(());
        }
    };

    // Send response
    stream::write_varint_framed(stream, &response_bytes).await?;
    stream.flush().await?;

    Ok(())
}

/// Handle PING request
async fn handle_ping(_request_bytes: &[u8], storage: SharedStorage) -> Result<Vec<u8>, Box<dyn Error>> {
    debug!("Processing PING request ({} bytes)", _request_bytes.len());

    // Decode and use DHTStorage to handle it
    let request = DHTRequest::decode(_request_bytes)?;

    let storage_guard = storage.read().await;
    let response = storage_guard.handle_request(request)?;
    let response_bytes = response.encode()?;

    debug!("Sending PING response ({} bytes)", response_bytes.len());
    Ok(response_bytes)
}

/// Handle STORE request - store blocks in local DHT
async fn handle_store(
    request_bytes: &[u8],
    storage: SharedStorage,
) -> Result<Vec<u8>, Box<dyn Error>> {
    debug!("Processing STORE request ({} bytes)", request_bytes.len());

    // Decode and use DHTStorage to handle it
    let request = DHTRequest::decode(request_bytes)?;

    let storage_guard = storage.read().await;
    let response = storage_guard.handle_request(request)?;
    let response_bytes = response.encode()?;

    debug!("Sending STORE response ({} bytes)", response_bytes.len());
    Ok(response_bytes)
}

/// Handle FIND request - query local DHT storage
async fn handle_find(
    request_bytes: &[u8],
    storage: SharedStorage,
) -> Result<Vec<u8>, Box<dyn Error>> {
    debug!("Processing FIND request ({} bytes)", request_bytes.len());

    // Decode and use DHTStorage to handle it
    let request = DHTRequest::decode(request_bytes)?;

    let storage_guard = storage.read().await;
    let response = storage_guard.handle_request(request)?;
    let response_bytes = response.encode()?;

    debug!("Sending FIND response ({} bytes)", response_bytes.len());
    Ok(response_bytes)
}

/// Send a STORE request to a peer via Hivemind protocol
async fn send_store_to_peer(
    client: &mut kwaai_p2p_daemon::P2PClient,
    peer_id_bytes: &[u8],
    store_request: StoreRequest,
) -> Result<(), Box<dyn Error>> {
    info!("Calling unary handler DHTProtocol.rpc_store...");

    // For RPC calls via unary handlers, we send RAW PROTOBUF
    // The Hivemind [8-byte len][marker][protobuf] format is only for DHT storage values!
    use prost::Message;
    let mut request_bytes = Vec::new();
    store_request.encode(&mut request_bytes)?;

    info!("Sending STORE request ({} bytes raw protobuf)", request_bytes.len());

    // // Log the FULL message for debugging (commented - too verbose)
    // let hex_full: String = request_bytes.iter()
    //     .map(|b| format!("{:02x}", b))
    //     .collect::<Vec<_>>()
    //     .join(" ");
    // info!("FULL REQUEST HEX: {}", hex_full);

    // // Also log what we're trying to send in human-readable form (commented - too verbose)
    // info!("Request structure:");
    // info!("  - auth present: {}", store_request.auth.is_some());
    // info!("  - keys.len(): {}", store_request.keys.len());
    // info!("  - subkeys.len(): {}", store_request.subkeys.len());
    // info!("  - values.len(): {}", store_request.values.len());
    // if !store_request.keys.is_empty() {
    //     info!("  - First key: {:?}", String::from_utf8_lossy(&store_request.keys[0]));
    // }
    // if !store_request.subkeys.is_empty() {
    //     info!("  - First subkey: {:?}", String::from_utf8_lossy(&store_request.subkeys[0]));
    // }
    // if !store_request.values.is_empty() {
    //     info!("  - First value size: {} bytes", store_request.values[0].len());
    // }

    // Call unary handler - the daemon handles ALL protocol negotiation!
    // No multistream negotiation needed in our code - that's the beauty of unary handlers!
    let response_bytes = client.call_unary_handler(
        peer_id_bytes,
        "DHTProtocol.rpc_store",  // Protocol name (no leading slash needed - daemon adds it)
        &request_bytes,
    ).await?;

    info!("Received response ({} bytes)", response_bytes.len());

    // Debug: log response bytes
    let hex_preview: String = response_bytes.iter()
        .take(64)
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ");
    info!("Response (first 64 bytes): {}", hex_preview);

    // Decode response as raw protobuf
    use kwaai_hivemind_dht::protocol::StoreResponse;
    let store_response = StoreResponse::decode(&response_bytes[..])?;

    let stored_count = store_response.store_ok.iter().filter(|&&s| s).count();
    info!("STORE response: {}/{} blocks stored", stored_count, store_response.store_ok.len());

    if stored_count == 0 {
        warn!("Bootstrap peer did not store any blocks!");
    } else {
        info!("Successfully announced blocks to bootstrap peer!");
    }

    Ok(())
}

/// Read a multistream-select message (varint-length-prefixed string)
async fn read_multistream_msg(stream: &mut tokio::net::TcpStream) -> Result<String, Box<dyn Error>> {
    // Use the existing varint framing from kwaai_p2p_daemon::stream
    let msg_bytes = stream::read_varint_framed(stream).await?;
    Ok(String::from_utf8_lossy(&msg_bytes).to_string())
}

/// Write a multistream-select message (varint-length-prefixed string)
async fn write_multistream_msg(stream: &mut tokio::net::TcpStream, msg: &str) -> Result<(), Box<dyn Error>> {
    // Use the existing varint framing from kwaai_p2p_daemon::stream
    stream::write_varint_framed(stream, msg.as_bytes()).await?;
    Ok(())
}

fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now)
}

/// Decode protobuf-encoded peer ID bytes to base58 format (Qm... or 12D3...)
fn decode_peer_id_from_protobuf(bytes: &[u8]) -> Option<String> {
    // Try to parse as libp2p PeerId
    match PeerId::from_bytes(bytes) {
        Ok(peer_id) => Some(peer_id.to_base58()),
        Err(_) => None,
    }
}
