//! Query DHT and build aggregated state like map.kwaai.ai/api/v1/state
//!
//! This tool queries all blocks for a model and builds a complete network topology view.

use kwaai_hivemind_dht::protocol::{FindRequest, FindResponse, RequestAuthInfo};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PDaemon;
use libp2p::PeerId;
use prost::Message;
use rmpv::Value;
use serde::{Deserialize, Serialize};
use sha1::{Sha1, Digest};
use std::collections::HashMap;
use std::error::Error;
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Generate a DHT ID from a raw key
fn generate_dht_id(raw_key: &str) -> Vec<u8> {
    let msgpack_bytes = rmp_serde::to_vec(raw_key).expect("Failed to serialize key");
    let mut hasher = Sha1::new();
    hasher.update(&msgpack_bytes);
    hasher.finalize().to_vec()
}

/// Server information from DHT
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServerInfo {
    state: String,
    throughput: f64,
    start_block: i64,
    end_block: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_rps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_rps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inference_rps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    torch_dtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quant_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    using_relay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_tokens_left: Option<i64>,
}

/// Peer span information
#[derive(Debug, Clone, Serialize)]
struct PeerSpan {
    peer_id: String,
    start: i64,
    end: i64,
    server_info: ServerInfo,
}

/// Peer row for output
#[derive(Debug, Serialize)]
struct ServerRow {
    short_peer_id: String,
    peer_id: String,
    peer_ip_info: String,
    show_public_name: bool,
    state: String,
    span: PeerSpan,
}

/// Model report
#[derive(Debug, Serialize)]
struct ModelReport {
    name: String,
    short_name: String,
    state: String,
    server_rows: Vec<ServerRow>,
    num_blocks: i64,
    dht_prefix: String,
}

/// Complete state output
#[derive(Debug, Serialize)]
struct StateOutput {
    model_reports: Vec<ModelReport>,
    num_peers: usize,
    num_blocks_covered: usize,
}

/// Parse ServerInfo from Petals ExtType format
fn parse_server_info(value_bytes: &[u8]) -> Option<ServerInfo> {
    match rmpv::decode::read_value(&mut &value_bytes[..]) {
        Ok(Value::Ext(_, ref ext_data)) => {
            // Petals format: ExtType wrapping [state, throughput, {field_map}]
            match rmpv::decode::read_value(&mut &ext_data[..]) {
                Ok(Value::Array(ref arr)) if arr.len() >= 3 => {
                    let state_val = arr.get(0)
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    let state = match state_val {
                        0 => "offline",
                        1 => "joining",
                        2 => "online",
                        _ => "unknown",
                    }.to_string();

                    let throughput = arr.get(1)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);

                    // Extract fields from the map at index 2
                    if let Some(Value::Map(ref field_map)) = arr.get(2) {
                        let mut info = ServerInfo {
                            state,
                            throughput,
                            start_block: 0,
                            end_block: 0,
                            public_name: None,
                            version: None,
                            network_rps: None,
                            forward_rps: None,
                            inference_rps: None,
                            torch_dtype: None,
                            quant_type: None,
                            using_relay: None,
                            cache_tokens_left: None,
                        };

                        for (k, v) in field_map {
                            if let Value::String(ref key) = k {
                                let key_str = key.as_str().unwrap_or("");
                                match key_str {
                                    "start_block" => info.start_block = v.as_i64().unwrap_or(0),
                                    "end_block" => info.end_block = v.as_i64().unwrap_or(0),
                                    "public_name" => {
                                        if let Value::String(ref s) = v {
                                            info.public_name = Some(s.as_str().unwrap_or("").to_string());
                                        }
                                    }
                                    "version" => {
                                        if let Value::String(ref s) = v {
                                            info.version = Some(s.as_str().unwrap_or("").to_string());
                                        }
                                    }
                                    "network_rps" => info.network_rps = v.as_f64(),
                                    "forward_rps" => info.forward_rps = v.as_f64(),
                                    "inference_rps" => info.inference_rps = v.as_f64(),
                                    "torch_dtype" => {
                                        if let Value::String(ref s) = v {
                                            info.torch_dtype = Some(s.as_str().unwrap_or("").to_string());
                                        }
                                    }
                                    "quant_type" => {
                                        if let Value::String(ref s) = v {
                                            info.quant_type = Some(s.as_str().unwrap_or("").to_string());
                                        }
                                    }
                                    "using_relay" => info.using_relay = v.as_bool(),
                                    "cache_tokens_left" => info.cache_tokens_left = v.as_i64(),
                                    _ => {}
                                }
                            }
                        }

                        return Some(info);
                    }
                }
                _ => {}
            }
        }
        Ok(Value::Array(ref arr)) if arr.len() >= 10 => {
            // KwaaiNet array format: [state, throughput, start_block, end_block, ...]
            let state_val = arr.get(0).and_then(|v| v.as_i64()).unwrap_or(0);
            let state = match state_val {
                0 => "offline",
                1 => "joining",
                2 => "online",
                _ => "unknown",
            }.to_string();

            return Some(ServerInfo {
                state,
                throughput: arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0),
                start_block: arr.get(2).and_then(|v| v.as_i64()).unwrap_or(0),
                end_block: arr.get(3).and_then(|v| v.as_i64()).unwrap_or(0),
                public_name: arr.get(4).and_then(|v| {
                    if let Value::String(ref s) = v {
                        Some(s.as_str().unwrap_or("").to_string())
                    } else {
                        None
                    }
                }),
                version: arr.get(5).and_then(|v| {
                    if let Value::String(ref s) = v {
                        Some(s.as_str().unwrap_or("").to_string())
                    } else {
                        None
                    }
                }),
                network_rps: arr.get(6).and_then(|v| v.as_f64()),
                forward_rps: arr.get(7).and_then(|v| v.as_f64()),
                inference_rps: arr.get(8).and_then(|v| v.as_f64()),
                torch_dtype: arr.get(9).and_then(|v| {
                    if let Value::String(ref s) = v {
                        Some(s.as_str().unwrap_or("").to_string())
                    } else {
                        None
                    }
                }),
                quant_type: None,
                using_relay: arr.get(11).and_then(|v| v.as_bool()),
                cache_tokens_left: arr.get(12).and_then(|v| v.as_i64()),
            });
        }
        _ => {}
    }
    None
}

/// Query a single block and extract peer information
async fn query_block(
    client: &mut kwaai_p2p_daemon::P2PClient,
    bootstrap_peer_id_bytes: &[u8],
    dht_prefix: &str,
    block_num: i64,
) -> Result<Vec<(String, ServerInfo)>, Box<dyn Error>> {
    let block_key = format!("{}.{}", dht_prefix, block_num);
    let hashed_key = generate_dht_id(&block_key);

    let find_request = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![hashed_key],
        peer: None,
    };

    let mut request_bytes = Vec::new();
    find_request.encode(&mut request_bytes)?;

    let response_bytes = client.call_unary_handler(
        bootstrap_peer_id_bytes,
        "DHTProtocol.rpc_find",
        &request_bytes,
    ).await?;

    let mut peers = Vec::new();

    if let Ok(find_response) = FindResponse::decode(&response_bytes[..]) {
        for result in &find_response.results {
            if result.result_type == 2 && !result.value.is_empty() {
                // FOUND_DICTIONARY
                if let Ok(Value::Ext(_, data)) = rmpv::decode::read_value(&mut &result.value[..]) {
                    if let Ok(Value::Array(inner)) = rmpv::decode::read_value(&mut &data[..]) {
                        if let Some(Value::Array(entries_arr)) = inner.get(2) {
                            for entry in entries_arr {
                                if let Value::Array(entry_data) = entry {
                                    if entry_data.len() >= 2 {
                                        // Get peer ID (subkey)
                                        let peer_id = entry_data.get(0)
                                            .and_then(|v| {
                                                if let Value::String(ref s) = v {
                                                    s.as_str().map(|s| s.to_string())
                                                } else {
                                                    None
                                                }
                                            });

                                        // Get server info (value)
                                        let server_info = entry_data.get(1)
                                            .and_then(|v| {
                                                if let Value::Binary(ref bytes) = v {
                                                    parse_server_info(bytes)
                                                } else {
                                                    None
                                                }
                                            });

                                        if let (Some(pid), Some(info)) = (peer_id, server_info) {
                                            peers.push((pid, info));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(peers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let args: Vec<String> = std::env::args().collect();
    let model_name = args.get(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Llama-3.1-8B-Instruct".to_string());

    // Map display name to DHT prefix
    let (dht_prefix, num_blocks) = match model_name.as_str() {
        "Llama-3.1-8B-Instruct" => ("Llama-3-1-8B-Instruct-hf", 32),
        "Llama-3.1-70B-Instruct" => ("Llama-3-1-70B-Instruct-hf", 80),
        "Llama-3.3-70B-Instruct" => ("Llama-3-3-70B-Instruct-hf", 80),
        _ => {
            eprintln!("Unknown model. Supported: Llama-3.1-8B-Instruct, Llama-3.1-70B-Instruct, Llama-3.3-70B-Instruct");
            return Ok(());
        }
    };

    println!("Querying model: {}", model_name);
    println!("DHT prefix: {}", dht_prefix);
    println!("Total blocks: {}\n", num_blocks);

    let config = NetworkConfig::with_petals_bootstrap();

    if let Some(bootstrap_addr) = config.bootstrap_peers.first() {
        println!("Bootstrap peer: {}\n", bootstrap_addr);
    }

    let daemon = P2PDaemon::builder()
        .dht(true)
        .nat_portmap(false)
        .bootstrap_peers(config.bootstrap_peers.clone())
        .spawn()
        .await?;

    let mut client = daemon.client().await?;
    let peer_id_hex = client.identify().await?;
    let peer_id = PeerId::from_bytes(&hex::decode(&peer_id_hex)?)?;
    println!("[PEER ID] {}\n", peer_id.to_base58());

    info!("Waiting for DHT bootstrap...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    if let Some(bootstrap_addr) = config.bootstrap_peers.first() {
        client.connect_peer(bootstrap_addr).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        if let Some(peer_id_str) = bootstrap_addr.split("/p2p/").nth(1) {
            if let Ok(bootstrap_peer_id) = peer_id_str.parse::<PeerId>() {
                let bootstrap_peer_id_bytes = bootstrap_peer_id.to_bytes();

                println!("Querying {} blocks...\n", num_blocks);

                // Collect all peer information across all blocks
                let mut all_peers: HashMap<String, ServerInfo> = HashMap::new();
                let mut blocks_with_peers = 0;

                for block_num in 0..num_blocks {
                    info!("Querying block {}...", block_num);

                    match query_block(&mut client, &bootstrap_peer_id_bytes, dht_prefix, block_num).await {
                        Ok(peers) => {
                            if !peers.is_empty() {
                                blocks_with_peers += 1;
                                for (peer_id, server_info) in peers {
                                    // Keep the widest span for each peer
                                    all_peers.entry(peer_id.clone())
                                        .and_modify(|existing| {
                                            if server_info.start_block < existing.start_block {
                                                existing.start_block = server_info.start_block;
                                            }
                                            if server_info.end_block > existing.end_block {
                                                existing.end_block = server_info.end_block;
                                            }
                                        })
                                        .or_insert(server_info);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error querying block {}: {}", block_num, e);
                        }
                    }
                }

                println!("\n=== AGGREGATED STATE ===\n");

                // Build server rows
                let mut server_rows = Vec::new();
                for (peer_id, server_info) in &all_peers {
                    let short_peer_id = if peer_id.len() > 10 {
                        format!("...{}", &peer_id[peer_id.len()-6..])
                    } else {
                        peer_id.clone()
                    };

                    let show_public_name = server_info.public_name.is_some();

                    server_rows.push(ServerRow {
                        short_peer_id,
                        peer_id: peer_id.clone(),
                        peer_ip_info: "unknown".to_string(),
                        show_public_name,
                        state: server_info.state.clone(),
                        span: PeerSpan {
                            peer_id: peer_id.clone(),
                            start: server_info.start_block,
                            end: server_info.end_block,
                            server_info: server_info.clone(),
                        },
                    });
                }

                let model_state = if all_peers.is_empty() {
                    "offline"
                } else {
                    "healthy"
                };

                let model_report = ModelReport {
                    name: format!("unsloth/{}", model_name),
                    short_name: model_name.clone(),
                    state: model_state.to_string(),
                    server_rows,
                    num_blocks,
                    dht_prefix: dht_prefix.to_string(),
                };

                let state_output = StateOutput {
                    model_reports: vec![model_report],
                    num_peers: all_peers.len(),
                    num_blocks_covered: blocks_with_peers,
                };

                // Output as JSON
                let json = serde_json::to_string_pretty(&state_output)?;
                println!("{}", json);

                println!("\n=== SUMMARY ===");
                println!("Total peers found: {}", all_peers.len());
                println!("Blocks with peers: {}/{}", blocks_with_peers, num_blocks);
            }
        }
    }

    Ok(())
}
