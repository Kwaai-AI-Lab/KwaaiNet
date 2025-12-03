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
    noise, request_response,
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
        .unwrap_or(31337);

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
        .with_dtype("float16");

    println!("Node Configuration:");
    println!("  Name: {}", public_name);
    println!("  Port: {}", listen_port);
    println!("  Model: {}", model_name);
    println!("  Version: {}", server_info.version);
    println!("  Blocks: {:?}", server_info.spans);
    println!();

    // Use Petals bootstrap
    let config = NetworkConfig::with_petals_bootstrap();
    println!("Bootstrap servers:");
    for server in &config.bootstrap_peers {
        println!("  {}", server);
    }
    println!();

    // Generate identity
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local Peer ID: {}\n", local_peer_id);

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
        identify::Config::new("/hivemind/0.0.0".to_string(), local_key.public())
            .with_agent_version(format!("kwaai/{}", env!("CARGO_PKG_VERSION"))),
    );

    // Setup RPC handler for responding to health monitor queries
    let (rpc, _protocol) = create_hivemind_protocol();

    let behaviour = PetalsBehaviour {
        kademlia,
        identify,
        rpc,
    };

    // Build swarm
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(300)))
        .build();

    // Listen on specified port
    let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", listen_port).parse()?;
    swarm.listen_on(listen_addr)?;

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

    println!("Waiting for connections...");
    println!("Once connected, node info will be announced to DHT.\n");

    loop {
        tokio::select! {
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

    // Petals uses specific DHT key patterns for server discovery
    // Format: {model_name}.{peer_id}
    let server_key = format!("{}.{}", model_name, peer_id);

    // Serialize server info
    let info_bytes = match server_info.to_msgpack() {
        Ok(bytes) => bytes,
        Err(e) => {
            warn!("Failed to serialize server info: {}", e);
            return;
        }
    };

    println!("  Key: {}", server_key);
    println!("  Info size: {} bytes", info_bytes.len());

    // Put the record in DHT
    let record = Record {
        key: RecordKey::new(&server_key),
        value: info_bytes,
        publisher: Some(*peer_id),
        expires: None,
    };

    if let Err(e) = swarm.behaviour_mut().kademlia.put_record(record, kad::Quorum::One) {
        warn!("Failed to put record: {}", e);
    }

    // Also start providing the model key so we're discoverable
    let model_key = RecordKey::new(&model_name);
    if let Err(e) = swarm.behaviour_mut().kademlia.start_providing(model_key) {
        warn!("Failed to start providing: {}", e);
    }

    println!("  Announcement sent!");
    println!("\n[STATUS] Node is now announcing itself to the Petals DHT.");
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
