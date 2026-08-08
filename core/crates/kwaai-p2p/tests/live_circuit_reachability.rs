//! Read-only probe: can an external peer reach a NATed native node through its
//! relay circuit?
//!
//! Dials the circuit address a node holds on a production bootstrap relay and
//! issues `DHTProtocol.rpc_ping`. This is the reachability the health-map
//! crawler depends on when it walks the DHT: if a node in the routing table
//! cannot be reached, every crawl pays a dial timeout for it.
//!
//! ```sh
//! KWAAI_CIRCUIT_ADDR="/ip4/…/tcp/8000/p2p/<relay>/p2p-circuit/p2p/<target>" \
//!   cargo test -p kwaai-p2p --test live_circuit_reachability -- --ignored --nocapture
//! ```

use std::time::{Duration, Instant};

use kwaai_hivemind_dht::protocol::{NodeInfo, PingRequest, PingResponse, RequestAuthInfo};
use kwaai_hivemind_dht::PROTOCOL_PING;
use kwaai_p2p::{NetworkConfig, NetworkService};
use libp2p::identity::Keypair;
use prost::Message;

#[tokio::test]
#[ignore = "read-only probe of a live relay circuit"]
async fn a_relayed_native_node_answers_rpc_ping_from_outside() {
    let Ok(circuit_addr) = std::env::var("KWAAI_CIRCUIT_ADDR") else {
        eprintln!("SKIPPED: set KWAAI_CIRCUIT_ADDR to the target's /p2p-circuit address");
        return;
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter("kwaai_p2p=debug")
        .try_init();

    let keypair = Keypair::generate_ed25519();
    let local = keypair.public().to_peer_id();
    println!("local (ephemeral) peer id: {local}");

    let config = NetworkConfig {
        listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
        request_timeout: Duration::from_secs(30),
        ..NetworkConfig::default()
    };
    let (handle, _task) = NetworkService::spawn(config, keypair).expect("swarm should start");

    println!("dialing {circuit_addr}");
    let t0 = Instant::now();
    let peer = tokio::time::timeout(Duration::from_secs(45), handle.connect_peer(&circuit_addr))
        .await
        .expect("dial through the relay circuit timed out")
        .expect("dial through the relay circuit failed");
    println!("connected to {peer} in {:?}", t0.elapsed());

    let ping = PingRequest {
        auth: Some(RequestAuthInfo::new()),
        peer: Some(NodeInfo::from_peer_id(local)),
        validate: false,
    };
    let t0 = Instant::now();
    let bytes = tokio::time::timeout(
        Duration::from_secs(30),
        handle.call_unary_handler(peer, PROTOCOL_PING, &ping.encode_to_vec()),
    )
    .await
    .expect("rpc_ping timed out")
    .expect("rpc_ping failed");
    let response = PingResponse::decode(&bytes[..]).expect("decodable PingResponse");
    println!(
        "rpc_ping answered in {:?}: dht_time={:.3} available={}",
        t0.elapsed(),
        response.dht_time,
        response.available
    );
}
