//! Read-only diagnostic probe against the production bootstraps.
//!
//! Fetches the real model-block and model-registry keys the health-map crawler
//! reads, and dumps every dictionary subkey verbatim. Exists to answer one
//! question empirically: when a native node's announce reports `store_ok` yet
//! the node does not appear on the map, is the record (a) absent under the key
//! the crawler asks for, (b) present under a subkey encoded differently from
//! the p2pd-era nodes, or (c) present and identical — pointing the finger at
//! the crawler side?
//!
//! Read-only: `rpc_find` only, ephemeral identity, no stores. Per the
//! `live_bootstrap.rs` precedent, `--ignored` alone is the live-read gate.
//!
//! ```sh
//! KWAAI_MAP_PROBE_KEY=Llama-3-1-8B-Instruct.0 \
//!   cargo test -p kwaai-p2p --test live_map_probe -- --ignored --nocapture
//! ```

use std::time::{Duration, Instant};

use kwaai_hivemind_dht::protocol::{FindRequest, NodeInfo, RequestAuthInfo, ResultType};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::PROTOCOL_FIND;
use kwaai_p2p::{NetworkConfig, NetworkService, KWAAI_BOOTSTRAP_SERVERS_DNS};
use libp2p::identity::Keypair;
use prost::Message;
use sha1::{Digest, Sha1};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Render a subkey the way a human debugging an encoding mismatch needs it:
/// raw hex plus, when it is a msgpack string, the decoded string.
fn describe_subkey(subkey: &[u8]) -> String {
    let as_str: Option<String> = rmp_serde::from_slice(subkey).ok();
    match as_str {
        Some(s) => format!("msgpack-str \"{s}\" ({} bytes)", subkey.len()),
        None => format!("raw {} ({} bytes)", hex(subkey), subkey.len()),
    }
}

#[tokio::test]
#[ignore = "read-only probe of the production KwaaiNet DHT"]
async fn dump_the_keys_the_map_crawler_reads() {
    let default_keys = "Llama-3-1-8B-Instruct.0,_petals.models".to_string();
    let keys_env = std::env::var("KWAAI_MAP_PROBE_KEY").unwrap_or(default_keys);
    let raw_keys: Vec<&str> = keys_env.split(',').collect();

    let keypair = Keypair::generate_ed25519();
    let local = keypair.public().to_peer_id();
    println!("local (ephemeral) peer id: {local}");
    println!("dht time: {:.3}", get_dht_time());

    let config = NetworkConfig {
        listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
        request_timeout: Duration::from_secs(30),
        ..NetworkConfig::default()
    };
    let (handle, _task) = NetworkService::spawn(config, keypair).expect("swarm should start");

    for addr in KWAAI_BOOTSTRAP_SERVERS_DNS {
        println!("\n════ bootstrap {addr}");
        let t0 = Instant::now();
        let peer = match tokio::time::timeout(CONNECT_TIMEOUT, handle.connect_peer(addr)).await {
            Ok(Ok(peer)) => {
                println!("  connected in {:?}", t0.elapsed());
                peer
            }
            Ok(Err(e)) => panic!("dial failed for {addr}: {e}"),
            Err(_) => panic!("dial to {addr} timed out"),
        };

        for raw_key in &raw_keys {
            let key = dht_id(raw_key);
            println!("\n  ── key \"{raw_key}\" (dht id {})", hex(&key));
            let find = FindRequest {
                auth: Some(RequestAuthInfo::new()),
                keys: vec![key],
                peer: Some(NodeInfo::from_peer_id(local)),
            };
            let bytes = tokio::time::timeout(
                Duration::from_secs(30),
                handle.call_unary_handler(peer, PROTOCOL_FIND, &find.encode_to_vec()),
            )
            .await
            .expect("rpc_find timed out")
            .expect("rpc_find failed");

            let response = kwaai_hivemind_dht::protocol::FindResponse::decode(&bytes[..])
                .expect("decodable FindResponse");
            let result = &response.results[0];
            println!(
                "  result_type: {} ({})",
                result.result_type,
                match result.result_type {
                    x if x == ResultType::NotFound as i32 => "NOT_FOUND",
                    x if x == ResultType::FoundRegular as i32 => "FOUND_REGULAR",
                    x if x == ResultType::FoundDictionary as i32 => "FOUND_DICTIONARY",
                    _ => "?",
                }
            );
            if result.result_type == ResultType::FoundDictionary as i32 {
                let dict = kwaai_hivemind_dht::parse_dictionary(&result.value)
                    .expect("a FOUND_DICTIONARY value must parse");
                let now = get_dht_time();
                println!("  {} subkey(s):", dict.entries.len());
                for (subkey, (value, expiration)) in &dict.entries {
                    println!(
                        "    - {}  value {} bytes, first byte {:#04x}, expires in {:+.0}s",
                        describe_subkey(subkey),
                        value.len(),
                        value.first().copied().unwrap_or(0),
                        expiration - now,
                    );
                    // Structural decode: hivemind wraps records in a msgpack
                    // Ext whose payload is itself msgpack. Print both layers so
                    // an encoding diff between two nodes' records is visible.
                    match rmpv::decode::read_value(&mut &value[..]) {
                        Ok(rmpv::Value::Ext(type_code, payload)) => {
                            match rmpv::decode::read_value(&mut &payload[..]) {
                                Ok(inner) => {
                                    println!("      ext type {type_code}, inner: {inner:?}")
                                }
                                Err(e) => println!(
                                    "      ext type {type_code}, inner NOT msgpack ({e}); raw: {}",
                                    hex(&payload)
                                ),
                            }
                        }
                        Ok(other) => println!("      non-ext msgpack: {other:?}"),
                        Err(e) => println!("      NOT msgpack ({e}); raw: {}", hex(value)),
                    }
                }
            } else if result.result_type == ResultType::FoundRegular as i32 {
                println!(
                    "  regular value: {} bytes, first byte {:#04x}",
                    result.value.len(),
                    result.value.first().copied().unwrap_or(0)
                );
            }
        }
    }
}
