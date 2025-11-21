//! Day 3: Peer Discovery Example
//!
//! Demonstrates capability-based peer discovery using DHT providers:
//! - Register as provider for a capability (e.g., "inference:llama2")
//! - Find other nodes providing the same capability
//!
//! ## Usage
//!
//! Terminal 1 (provider):
//! ```bash
//! cargo run --example peer_discovery -- --listen 4001 --provide inference:llama2
//! ```
//!
//! Terminal 2 (discoverer):
//! ```bash
//! cargo run --example peer_discovery -- --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID> --find inference:llama2
//! ```

use futures::StreamExt;
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode, QueryResult, RecordKey},
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(NetworkBehaviour)]
struct DiscoveryBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

#[derive(Debug)]
enum Command {
    Provide { capability: String },
    Find { capability: String },
    None,
}

fn parse_args() -> (Option<u16>, Option<Multiaddr>, Command) {
    let args: Vec<String> = std::env::args().collect();
    let mut listen_port: Option<u16> = None;
    let mut bootstrap: Option<Multiaddr> = None;
    let mut command = Command::None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--listen" => {
                if i + 1 < args.len() {
                    listen_port = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--bootstrap" => {
                if i + 1 < args.len() {
                    bootstrap = args[i + 1].parse().ok();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--provide" => {
                if i + 1 < args.len() {
                    command = Command::Provide {
                        capability: args[i + 1].clone(),
                    };
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--find" => {
                if i + 1 < args.len() {
                    command = Command::Find {
                        capability: args[i + 1].clone(),
                    };
                    i += 2;
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    (listen_port, bootstrap, command)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let (listen_port, bootstrap_addr, command) = parse_args();

    info!("Starting KwaaiNet Peer Discovery Node...");

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Node Peer ID: {}", local_peer_id);

    // Create Kademlia
    let kademlia = {
        let store = MemoryStore::new(local_peer_id);
        let mut config = kad::Config::default();
        config.set_replication_factor(std::num::NonZeroUsize::new(3).unwrap());
        let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, config);
        behaviour.set_mode(Some(Mode::Server));
        behaviour
    };

    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let behaviour = DiscoveryBehaviour { kademlia, identify };

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // Listen
    let listen_addr: Multiaddr = format!(
        "/ip4/0.0.0.0/tcp/{}",
        listen_port.unwrap_or(0)
    ).parse()?;
    swarm.listen_on(listen_addr)?;

    // Bootstrap if address provided
    if let Some(addr) = bootstrap_addr.clone() {
        info!("Bootstrapping to: {}", addr);
        if let Some(peer_id) = extract_peer_id(&addr) {
            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
            swarm.dial(addr)?;
        }
    }

    let mut command_executed = false;
    let mut connected = bootstrap_addr.is_none();

    loop {
        // Execute command once connected
        if !command_executed && connected {
            match &command {
                Command::Provide { capability } => {
                    info!("Registering as provider for: {}", capability);
                    let key = RecordKey::new(&capability);
                    swarm.behaviour_mut().kademlia.start_providing(key)?;
                    command_executed = true;
                    println!("\n  Registered as provider for '{}'\n", capability);
                    println!("  Other nodes can find this node with:");
                    println!("  --find {}\n", capability);
                }
                Command::Find { capability } => {
                    info!("Searching for providers of: {}", capability);
                    let key = RecordKey::new(&capability);
                    swarm.behaviour_mut().kademlia.get_providers(key);
                    command_executed = true;
                }
                Command::None => {
                    command_executed = true;
                }
            }
        }

        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                let full_addr = format!("{}/p2p/{}", address, local_peer_id);
                info!("Listening on: {}", full_addr);
                println!("\n  Bootstrap with: --bootstrap {}\n", full_addr);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("Connected to: {}", peer_id);
                connected = true;

                if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
                    warn!("Bootstrap error: {}", e);
                }
            }
            SwarmEvent::Behaviour(DiscoveryBehaviourEvent::Kademlia(
                kad::Event::OutboundQueryProgressed {
                    result: QueryResult::GetProviders(Ok(providers)),
                    ..
                },
            )) => {
                let provider_count = providers.providers.len();
                info!("Found {} provider(s)", provider_count);

                if provider_count > 0 {
                    println!("\n  SUCCESS: Found {} provider(s) for capability:\n", provider_count);
                    for provider in &providers.providers {
                        println!("    - {}", provider);
                    }
                    println!();
                } else {
                    println!("\n  No providers found yet (DHT may still be propagating)\n");
                }
            }
            SwarmEvent::Behaviour(DiscoveryBehaviourEvent::Kademlia(
                kad::Event::OutboundQueryProgressed {
                    result: QueryResult::StartProviding(Ok(add_provider)),
                    ..
                },
            )) => {
                info!("Successfully registered as provider: {:?}", add_provider.key);
            }
            SwarmEvent::Behaviour(DiscoveryBehaviourEvent::Identify(
                identify::Event::Received { peer_id, info },
            )) => {
                info!("Identified: {} ({})", peer_id, info.protocol_version);
                for addr in info.listen_addrs {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                }
            }
            SwarmEvent::Behaviour(DiscoveryBehaviourEvent::Kademlia(
                kad::Event::RoutingUpdated { peer, .. },
            )) => {
                info!("Routing updated: {}", peer);
            }
            _ => {}
        }
    }
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
