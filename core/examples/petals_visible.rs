//! Petals-Visible Node Example
//!
//! This example creates a KwaaiNet node that:
//! 1. Connects to the Petals DHT via bootstrap servers
//! 2. Announces itself in the DHT
//! 3. Makes the node discoverable on map.kwaai.ai
//!
//! Run with: cargo run --release --example petals_visible -- --name "My-Node"
//!
//! After running, check map.kwaai.ai to see if your node appears.
//! Note: The health monitor needs to discover and query your node.

use futures::StreamExt;
use kwaai_p2p::{
    hivemind::ServerInfo,
    rpc::{create_hivemind_protocol, RpcHandler},
    NetworkConfig,
};
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode, Record, RecordKey},
    noise, relay, request_response,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol,
};
use std::{error::Error, time::Duration};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

// =============================================================================
// Network Behaviour
// =============================================================================

#[derive(NetworkBehaviour)]
struct PetalsBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
    rpc: request_response::Behaviour<kwaai_p2p::rpc::HivemindCodec>,
    relay_client: relay::client::Behaviour,
}

// =============================================================================
// Main
// =============================================================================

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

    let listen_port: u16 = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    // Model to announce (must match a model the health monitor tracks)
    let model_name = args
        .iter()
        .position(|a| a == "--model")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Llama-3.3-70B-Instruct".to_string());

    println!("KwaaiNet Petals-Visible Node");
    println!("============================\n");

    // Create server info
    let server_info = ServerInfo::new(&public_name)
        .with_span(0, 8) // Blocks we "serve"
        .with_cache_tokens(10000)
        .with_throughput(10.0)
        .with_dtype("float16")
        .with_relay(true); // Using relay for NAT traversal

    println!("Node Configuration:");
    println!("  Name: {}", public_name);
    println!("  Port: {}", listen_port);
    println!("  Model: {}", model_name);
    println!("  Version: {}", server_info.version);
    println!("  Blocks: {:?}", server_info.spans);
    println!();

    // Use KwaaiNet bootstrap servers
    let config = NetworkConfig::with_kwaai_bootstrap();
    println!("Bootstrap servers:");
    for server in &config.bootstrap_peers {
        println!("  {}", server);
    }
    println!();

    // Generate identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local Peer ID: {}\n", local_peer_id);

    // Build swarm with relay support
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key, relay_client| {
            // Setup Kademlia with IPFS-compatible protocol
            let kademlia = {
                let store = MemoryStore::new(local_peer_id);
                let mut kad_config = kad::Config::default();
                kad_config.set_protocol_names(vec![StreamProtocol::new("/ipfs/kad/1.0.0")]);
                let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, kad_config);
                behaviour.set_mode(Some(Mode::Server)); // Be a DHT server
                behaviour
            };

            // Setup Identify with Petals-like agent version
            let identify = identify::Behaviour::new(
                identify::Config::new("/hivemind/0.0.0".to_string(), key.public())
                    .with_agent_version(format!("kwaai/{}", env!("CARGO_PKG_VERSION"))),
            );

            // Setup RPC handler for responding to health monitor queries
            let (rpc, _protocol) = create_hivemind_protocol();

            Ok(PetalsBehaviour {
                kademlia,
                identify,
                rpc,
                relay_client,
            })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(300)))
        .build();

    // Listen on both IPv4 and IPv6
    let listen_addr_v4: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", listen_port).parse()?;
    let listen_addr_v6: Multiaddr = format!("/ip6/::/tcp/{}", listen_port).parse()?;

    swarm.listen_on(listen_addr_v4)?;
    swarm.listen_on(listen_addr_v6)?;

    // Add external address for NAT traversal (health monitor needs to reach us)
    // This allows peers to discover our public address
    if let Ok(external_ip) = std::env::var("EXTERNAL_IP") {
        let external_addr: Multiaddr = format!("/ip4/{}/tcp/{}", external_ip, listen_port).parse()?;
        swarm.add_external_address(external_addr);
        println!("Added external address: /ip4/{}/tcp/{}\n", external_ip, listen_port);
    }

    // Connect to Petals bootstrap
    println!("Connecting to Petals network...\n");
    for addr_str in &config.bootstrap_peers {
        let addr: Multiaddr = addr_str.parse()?;
        if let Some(peer_id) = extract_peer_id(&addr) {
            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
            if let Err(e) = swarm.dial(addr) {
                warn!("Failed to dial bootstrap: {}", e);
            }
        }
    }

    let mut connected = false;
    let mut bootstrap_done = false;
    let mut announced = false;

    // Create RPC handler for responding to health monitor queries
    let rpc_handler = RpcHandler::new(server_info.clone());

    // DHT heartbeat timer - re-announce every 4 minutes to prevent expiration
    let mut heartbeat_interval = tokio::time::interval(Duration::from_secs(240));
    heartbeat_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    println!("Waiting for connections...");
    println!("Once connected, node info will be announced to DHT.\n");

    loop {
        tokio::select! {
            _ = heartbeat_interval.tick() => {
                // Periodic DHT re-announcement to keep records alive
                if bootstrap_done {
                    println!("[HEARTBEAT] Re-announcing to DHT...");
                    announce_to_dht(
                        &mut swarm,
                        &model_name,
                        &local_peer_id,
                        &server_info,
                    );
                }
            }

            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("[LISTEN] {}/p2p/{}", address, local_peer_id);
                    }

                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        if !connected {
                            connected = true;
                            println!("[CONNECTED] to Petals network via {}", peer_id);

                            // Bootstrap DHT
                            println!("[DHT] Starting bootstrap...");
                            if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
                                warn!("Bootstrap error: {}", e);
                            }
                        }
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Identify(
                        identify::Event::Received { peer_id, info },
                    )) => {
                        info!("Identified: {} ({})", peer_id, info.agent_version);
                        for addr in info.listen_addrs {
                            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                        }
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed {
                            result: kad::QueryResult::Bootstrap(Ok(stats)),
                            ..
                        },
                    )) => {
                        if !bootstrap_done {
                            bootstrap_done = true;
                            println!("[DHT] Bootstrap complete! {} peers in routing table",
                                     stats.num_remaining + 1);

                            // Announce ourselves in the DHT
                            if !announced {
                                announced = true;
                                announce_to_dht(
                                    &mut swarm,
                                    &model_name,
                                    &local_peer_id,
                                    &server_info,
                                );
                            }
                        }
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed {
                            result: kad::QueryResult::PutRecord(Ok(kad::PutRecordOk { key })),
                            ..
                        },
                    )) => {
                        println!("[DHT] Record stored: {:?}", String::from_utf8_lossy(key.as_ref()));
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed {
                            result: kad::QueryResult::StartProviding(Ok(kad::AddProviderOk { key })),
                            ..
                        },
                    )) => {
                        println!("[DHT] Now providing: {:?}", String::from_utf8_lossy(key.as_ref()));
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Kademlia(
                        kad::Event::RoutingUpdated { peer, .. },
                    )) => {
                        info!("DHT routing updated: {}", peer);
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Kademlia(
                        kad::Event::InboundRequest { request },
                    )) => {
                        println!("[DHT] Inbound request: {:?}", request);
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Rpc(
                        request_response::Event::Message { peer, message },
                    )) => {
                        match message {
                            request_response::Message::Request {
                                request,
                                channel,
                                ..
                            } => {
                                println!("[RPC] Received request from {}", peer);
                                let response = rpc_handler.handle_request(request);
                                if let Err(e) = swarm.behaviour_mut().rpc.send_response(channel, response) {
                                    warn!("Failed to send RPC response: {:?}", e);
                                }
                            }
                            request_response::Message::Response { .. } => {
                                // We don't send requests, so we shouldn't get responses
                                info!("[RPC] Unexpected response from {}", peer);
                            }
                        }
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Rpc(
                        request_response::Event::InboundFailure { peer, error, .. },
                    )) => {
                        warn!("[RPC] Inbound failure from {}: {:?}", peer, error);
                    }

                    SwarmEvent::Behaviour(PetalsBehaviourEvent::Rpc(
                        request_response::Event::OutboundFailure { peer, error, .. },
                    )) => {
                        warn!("[RPC] Outbound failure to {}: {:?}", peer, error);
                    }

                    SwarmEvent::IncomingConnection { local_addr, send_back_addr, .. } => {
                        println!("[CONNECTION] Incoming from {} to {}", send_back_addr, local_addr);
                    }

                    SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                        info!("[CONNECTION] Closed with {}: {:?}", peer_id, cause);
                    }

                    _ => {}
                }
            }
        }
    }
}

/// Announce this node in the DHT so the health monitor can discover it
fn announce_to_dht(
    swarm: &mut libp2p::Swarm<PetalsBehaviour>,
    model_name: &str,
    peer_id: &PeerId,
    server_info: &ServerInfo,
) {
    println!("\n[ANNOUNCE] Announcing node to DHT...");

    // Serialize server info
    let info_bytes = match server_info.to_msgpack() {
        Ok(bytes) => bytes,
        Err(e) => {
            warn!("Failed to serialize server info: {}", e);
            return;
        }
    };

    println!("  Model: {}", model_name);
    println!("  Blocks: {} to {}", server_info.start_block, server_info.end_block);
    println!("  Info size: {} bytes", info_bytes.len());

    // Petals DHT key format: announce each block as {model_name}.{block_index}
    // This matches Petals' declare_active_modules pattern
    for block_idx in server_info.start_block..server_info.end_block {
        let module_uid = format!("{}.{}", model_name, block_idx);

        let record = Record {
            key: RecordKey::new(&module_uid),
            value: info_bytes.clone(),
            publisher: Some(*peer_id),
            expires: None,
        };

        if let Err(e) = swarm.behaviour_mut().kademlia.put_record(record, kad::Quorum::One) {
            warn!("Failed to put record for {}: {}", module_uid, e);
        } else {
            println!("  [DHT] Announced module: {}", module_uid);
        }

        // Also announce as a content provider (for Petals compatibility)
        let key = RecordKey::new(&module_uid);
        if let Err(e) = swarm.behaviour_mut().kademlia.start_providing(key) {
            warn!("Failed to start providing {}: {}", module_uid, e);
        }
    }

    // Also announce model metadata key (Petals uses this for discovery)
    let model_metadata_key = format!("_petals.models.{}", model_name);
    let model_record = Record {
        key: RecordKey::new(&model_metadata_key),
        value: info_bytes,
        publisher: Some(*peer_id),
        expires: None,
    };

    if let Err(e) = swarm.behaviour_mut().kademlia.put_record(model_record, kad::Quorum::One) {
        warn!("Failed to put model metadata record: {}", e);
    } else {
        println!("  [DHT] Announced model metadata: {}", model_metadata_key);
    }

    println!("  Announcement complete!");
    println!("\n[STATUS] Node is now announcing itself to the Petals DHT.");
    println!("         DHT records will be refreshed every 4 minutes (heartbeat).");
    println!("         Check map.kwaai.ai in a few minutes to see if it appears.");
    println!("         RPC handler is active and ready to respond to health monitor queries.");
    println!("         Keep this node running for discovery.\n");
}

fn extract_peer_id(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| {
        if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
            Some(peer_id)
        } else {
            None
        }
    })
}

fn uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", now)
}
