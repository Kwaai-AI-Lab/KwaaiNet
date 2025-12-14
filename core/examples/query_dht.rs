//! Query DHT entries to verify what's stored

use kwaai_hivemind_dht::protocol::{FindRequest, FindResponse, RequestAuthInfo};
use kwaai_p2p::NetworkConfig;
use kwaai_p2p_daemon::P2PDaemon;
use libp2p::PeerId;
use prost::Message;
use rmpv::Value;
use sha1::{Sha1, Digest};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let args: Vec<String> = std::env::args().collect();
    let query_key = args.get(1).map(|s| s.to_string()).unwrap_or_else(|| "_petals.models".to_string());

    println!("Querying DHT key: {}\n", query_key);

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
                let hashed_key = generate_dht_id(&query_key);

                println!("DHT Key: {}", query_key);
                println!("SHA1: {}\n", hex::encode(&hashed_key));

                let find_request = FindRequest {
                    auth: Some(RequestAuthInfo::new()),
                    keys: vec![hashed_key.clone()],
                    peer: None,  // Don't send peer info in FIND request
                };

                let mut request_bytes = Vec::new();
                find_request.encode(&mut request_bytes)?;

                info!("Sending FIND request ({} bytes)", request_bytes.len());

                let response_bytes = client.call_unary_handler(
                    &bootstrap_peer_id_bytes,
                    "DHTProtocol.rpc_find",
                    &request_bytes,
                ).await?;

                info!("Received response ({} bytes)", response_bytes.len());

                match FindResponse::decode(&response_bytes[..]) {
                    Ok(find_response) => {
                        println!("\n=== FIND RESPONSE ===");
                        println!("Results: {}", find_response.results.len());

                        for (i, result) in find_response.results.iter().enumerate() {
                            println!("\n--- Result {} ---", i);

                            let result_type_name = match result.result_type {
                                0 => "NOT_FOUND",
                                1 => "FOUND_REGULAR",
                                2 => "FOUND_DICTIONARY",
                                _ => "UNKNOWN",
                            };
                            println!("Type: {} ({})", result.result_type, result_type_name);

                            if result.result_type == 2 && !result.value.is_empty() {
                                // FOUND_DICTIONARY - decode the DictionaryDHTValue
                                println!("Dictionary value ({} bytes)", result.value.len());

                                // Try to decode as msgpack list: [max_expiration, latest_update, [[subkey, value, timestamp], ...]]
                                match rmpv::decode::read_value(&mut &result.value[..]) {
                                    Ok(Value::Ext(code, data)) => {
                                        println!("  ExtType code: {}", code);

                                        // Decode the inner data
                                        match rmpv::decode::read_value(&mut &data[..]) {
                                            Ok(Value::Array(inner)) if inner.len() >= 3 => {
                                                if let Some(Value::Array(entries_arr)) = inner.get(2) {
                                                    println!("\n  Dictionary entries found: {}", entries_arr.len());

                                                    for entry in entries_arr {
                                                        if let Value::Array(entry_data) = entry {
                                                            if entry_data.len() >= 2 {
                                                                // Subkey
                                                                if let Some(Value::String(ref subkey)) = entry_data.get(0) {
                                                                    println!("\n  📦 Subkey: {}", subkey.as_str().unwrap_or("(invalid)"));
                                                                }

                                                                // Value (msgpack-encoded)
                                                                if let Some(Value::Binary(ref value_bytes)) = entry_data.get(1) {
                                                                    // Try to decode the value
                                                                    match rmpv::decode::read_value(&mut &value_bytes[..]) {
                                                                        Ok(Value::Ext(_, ref ext_data)) => {
                                                                            // ExtType-wrapped value (Petals format)
                                                                            match rmpv::decode::read_value(&mut &ext_data[..]) {
                                                                                Ok(Value::Array(ref arr)) if arr.len() >= 3 => {
                                                                                    // Petals format: [state, throughput, {field_map}]
                                                                                    println!("     Server Info (Petals format):");

                                                                                    if let Some(Value::Integer(state)) = arr.get(0) {
                                                                                        let state_val = state.as_i64().unwrap_or(0);
                                                                                        let state_name = match state_val {
                                                                                            0 => "OFFLINE",
                                                                                            1 => "JOINING",
                                                                                            2 => "ONLINE",
                                                                                            _ => "UNKNOWN",
                                                                                        };
                                                                                        println!("       state: {} ({})", state_val, state_name);
                                                                                    }

                                                                                    if let Some(Value::F64(throughput)) = arr.get(1) {
                                                                                        println!("       throughput: {}", throughput);
                                                                                    }

                                                                                    // Element [2] contains the field map
                                                                                    if let Some(Value::Map(ref field_map)) = arr.get(2) {
                                                                                        for (k, v) in field_map {
                                                                                            if let Value::String(ref key) = k {
                                                                                                let key_str = key.as_str().unwrap_or("(invalid)");
                                                                                                match v {
                                                                                                    Value::Integer(i) => println!("       {}: {}", key_str, i.as_i64().unwrap_or(0)),
                                                                                                    Value::F64(f) => println!("       {}: {}", key_str, f),
                                                                                                    Value::String(s) => println!("       {}: {}", key_str, s.as_str().unwrap_or("(invalid)")),
                                                                                                    Value::Boolean(b) => println!("       {}: {}", key_str, b),
                                                                                                    Value::Nil => println!("       {}: null", key_str),
                                                                                                    Value::Array(arr) => {
                                                                                                        if arr.is_empty() {
                                                                                                            println!("       {}: []", key_str);
                                                                                                        } else {
                                                                                                            println!("       {}: [array with {} items]", key_str, arr.len());
                                                                                                        }
                                                                                                    }
                                                                                                    Value::Map(m) => {
                                                                                                        if m.is_empty() {
                                                                                                            println!("       {}: {{}}", key_str);
                                                                                                        } else {
                                                                                                            println!("       {}: {{map with {} items}}", key_str, m.len());
                                                                                                        }
                                                                                                    }
                                                                                                    _ => println!("       {}: (complex value)", key_str),
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                Ok(Value::Map(ref map)) => {
                                                                                    // Fallback for direct map format (KwaaiNet nodes)
                                                                                    println!("     Server Info (Map format):");
                                                                                    for (k, v) in map {
                                                                                        if let Value::String(ref key) = k {
                                                                                            let key_str = key.as_str().unwrap_or("(invalid)");
                                                                                            match v {
                                                                                                Value::Integer(i) => {
                                                                                                    let val = i.as_i64().unwrap_or(0);
                                                                                                    if key_str == "state" {
                                                                                                        let state_name = match val {
                                                                                                            0 => "OFFLINE",
                                                                                                            1 => "JOINING",
                                                                                                            2 => "ONLINE",
                                                                                                            _ => "UNKNOWN",
                                                                                                        };
                                                                                                        println!("       {}: {} ({})", key_str, val, state_name);
                                                                                                    } else {
                                                                                                        println!("       {}: {}", key_str, val);
                                                                                                    }
                                                                                                }
                                                                                                Value::F64(f) => println!("       {}: {}", key_str, f),
                                                                                                Value::String(s) => println!("       {}: {}", key_str, s.as_str().unwrap_or("(invalid)")),
                                                                                                Value::Boolean(b) => println!("       {}: {}", key_str, b),
                                                                                                Value::Nil => println!("       {}: null", key_str),
                                                                                                Value::Array(arr) => {
                                                                                                    if arr.is_empty() {
                                                                                                        println!("       {}: []", key_str);
                                                                                                    } else {
                                                                                                        println!("       {}: [array with {} items]", key_str, arr.len());
                                                                                                    }
                                                                                                }
                                                                                                Value::Map(m) => {
                                                                                                    if m.is_empty() {
                                                                                                        println!("       {}: {{}}", key_str);
                                                                                                    } else {
                                                                                                        println!("       {}: {{map with {} items}}", key_str, m.len());
                                                                                                    }
                                                                                                }
                                                                                                _ => println!("       {}: (complex value)", key_str),
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                _ => {
                                                                                    println!("     Value (ExtType data hex): {}", hex::encode(ext_data));
                                                                                }
                                                                            }
                                                                        }
                                                                        Ok(Value::Array(ref arr)) if arr.len() == 2 => {
                                                                            // This is model registry info: [num_blocks, repository]
                                                                            if let Some(Value::Integer(num_blocks)) = arr.get(0) {
                                                                                println!("     Blocks: {}", num_blocks.as_i64().unwrap_or(0));
                                                                            }
                                                                            if let Some(Value::String(repo)) = arr.get(1) {
                                                                                println!("     Repository: {}", repo.as_str().unwrap_or("(invalid)"));
                                                                            }
                                                                        }
                                                                        Ok(Value::Array(ref arr)) if arr.len() >= 10 => {
                                                                            // This is ServerInfo (array format)
                                                                            // [state, throughput, start_block, end_block, public_name, version,
                                                                            //  network_rps, forward_rps, inference_rps, torch_dtype, adapters,
                                                                            //  using_relay, cache_tokens_left, next_pings]
                                                                            println!("     Server Info:");

                                                                            if let Some(Value::Integer(state)) = arr.get(0) {
                                                                                let state_name = match state.as_i64().unwrap_or(0) {
                                                                                    0 => "OFFLINE",
                                                                                    1 => "JOINING",
                                                                                    2 => "ONLINE",
                                                                                    _ => "UNKNOWN",
                                                                                };
                                                                                println!("       state: {} ({})", state.as_i64().unwrap_or(0), state_name);
                                                                            }
                                                                            if let Some(Value::F64(throughput)) = arr.get(1) {
                                                                                println!("       throughput: {}", throughput);
                                                                            }
                                                                            if let Some(Value::Integer(start_block)) = arr.get(2) {
                                                                                println!("       start_block: {}", start_block.as_i64().unwrap_or(0));
                                                                            }
                                                                            if let Some(Value::Integer(end_block)) = arr.get(3) {
                                                                                println!("       end_block: {}", end_block.as_i64().unwrap_or(0));
                                                                            }
                                                                            if let Some(Value::String(public_name)) = arr.get(4) {
                                                                                println!("       public_name: {}", public_name.as_str().unwrap_or("(invalid)"));
                                                                            }
                                                                            if let Some(Value::String(version)) = arr.get(5) {
                                                                                println!("       version: {}", version.as_str().unwrap_or("(invalid)"));
                                                                            }
                                                                            if let Some(Value::F64(network_rps)) = arr.get(6) {
                                                                                println!("       network_rps: {}", network_rps);
                                                                            }
                                                                            if let Some(Value::F64(forward_rps)) = arr.get(7) {
                                                                                println!("       forward_rps: {}", forward_rps);
                                                                            }
                                                                            if let Some(Value::F64(inference_rps)) = arr.get(8) {
                                                                                println!("       inference_rps: {}", inference_rps);
                                                                            }
                                                                            if let Some(Value::String(dtype)) = arr.get(9) {
                                                                                println!("       torch_dtype: {}", dtype.as_str().unwrap_or("(invalid)"));
                                                                            }
                                                                            if let Some(Value::Array(adapters)) = arr.get(10) {
                                                                                println!("       adapters: [{}]", adapters.len());
                                                                            }
                                                                            if let Some(Value::Boolean(using_relay)) = arr.get(11) {
                                                                                println!("       using_relay: {}", using_relay);
                                                                            }
                                                                            if let Some(Value::Integer(cache_tokens)) = arr.get(12) {
                                                                                println!("       cache_tokens_left: {}", cache_tokens.as_i64().unwrap_or(0));
                                                                            }
                                                                            if let Some(Value::Map(pings)) = arr.get(13) {
                                                                                println!("       next_pings: {{{}}} entries", pings.len());
                                                                            }
                                                                        }
                                                                        Ok(Value::Map(ref map)) => {
                                                                            // ServerInfo in map format (alternative encoding)
                                                                            println!("     Server Info:");
                                                                            for (k, v) in map {
                                                                                if let Value::String(ref key) = k {
                                                                                    let key_str = key.as_str().unwrap_or("(invalid)");
                                                                                    match v {
                                                                                        Value::Integer(i) => println!("       {}: {}", key_str, i.as_i64().unwrap_or(0)),
                                                                                        Value::F64(f) => println!("       {}: {}", key_str, f),
                                                                                        Value::String(s) => println!("       {}: {}", key_str, s.as_str().unwrap_or("(invalid)")),
                                                                                        Value::Boolean(b) => println!("       {}: {}", key_str, b),
                                                                                        Value::Nil => println!("       {}: null", key_str),
                                                                                        Value::Array(arr) => println!("       {}: [array with {} items]", key_str, arr.len()),
                                                                                        Value::Map(m) => println!("       {}: {{map with {} items}}", key_str, m.len()),
                                                                                        _ => println!("       {}: (complex value)", key_str),
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            println!("     Value (hex): {}", hex::encode(value_bytes));
                                                                        }
                                                                    }
                                                                }

                                                                // Timestamp
                                                                if let Some(Value::F64(timestamp)) = entry_data.get(2) {
                                                                    println!("     Updated: {}", timestamp);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => println!("  Could not decode inner structure"),
                                        }
                                    }
                                    _ => {
                                        println!("  Raw hex: {}", hex::encode(&result.value));
                                    }
                                }
                            } else if result.result_type == 1 && !result.value.is_empty() {
                                // FOUND_REGULAR - single value entry
                                println!("Regular value ({} bytes)", result.value.len());

                                // Try to decode as ServerInfo
                                match rmpv::decode::read_value(&mut &result.value[..]) {
                                    Ok(Value::Map(ref map)) => {
                                        println!("\n  Server Info:");
                                        for (k, v) in map {
                                            if let Value::String(ref key) = k {
                                                let key_str = key.as_str().unwrap_or("(invalid)");
                                                match v {
                                                    Value::Integer(i) => println!("    {}: {}", key_str, i.as_i64().unwrap_or(0)),
                                                    Value::F64(f) => println!("    {}: {}", key_str, f),
                                                    Value::String(s) => println!("    {}: {}", key_str, s.as_str().unwrap_or("(invalid)")),
                                                    Value::Boolean(b) => println!("    {}: {}", key_str, b),
                                                    Value::Nil => println!("    {}: null", key_str),
                                                    Value::Array(arr) => println!("    {}: [array with {} items]", key_str, arr.len()),
                                                    Value::Map(m) => println!("    {}: {{map with {} items}}", key_str, m.len()),
                                                    _ => println!("    {}: (complex value)", key_str),
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("  Value (hex): {}", hex::encode(&result.value));
                                    }
                                }
                            } else if !result.value.is_empty() {
                                println!("Value size: {} bytes", result.value.len());
                                println!("Hex: {}", hex::encode(&result.value));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decode response: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
