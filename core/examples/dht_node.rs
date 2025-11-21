//! Day 2: DHT Put/Get Example
//!
//! Demonstrates DHT operations between two nodes:
//! - Store key-value pairs in DHT
//! - Retrieve values from DHT
//! - Bootstrap and peer discovery
//!
//! ## Usage
//!
//! Terminal 1 (first node - stores a value):
//! ```bash
//! cargo run --example dht_node -- --listen 4001 --put hello world
//! ```
//!
//! Terminal 2 (second node - retrieves the value):
//! ```bash
//! cargo run --example dht_node -- --bootstrap /ip4/127.0.0.1/tcp/4001/p2p/<PEER_ID> --get hello
//! ```

use futures::StreamExt;
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode, QueryResult, Record, RecordKey},
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn, error};
use tracing_subscriber::EnvFilter;

/// Combined network behaviour
#[derive(NetworkBehaviour)]
struct DhtBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

#[derive(Debug)]
enum Command {
    Put { key: String, value: String },
    Get { key: String },
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
            "--put" => {
                if i + 2 < args.len() {
                    command = Command::Put {
                        key: args[i + 1].clone(),
                        value: args[i + 2].clone(),
                    };
                    i += 3;
                } else {
                    i += 1;
                }
            }
            "--get" => {
                if i + 1 < args.len() {
                    command = Command::Get {
                        key: args[i + 1].clone(),
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
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let (listen_port, bootstrap_addr, command) = parse_args();

    info!("Starting KwaaiNet DHT Node...");

    // Generate keypair
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

    // Create Identify
    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let behaviour = DhtBehaviour { kademlia, identify };

    // Build swarm
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
    if let Some(addr) = bootstrap_addr {
        info!("Bootstrapping to: {}", addr);

        // Extract peer ID from multiaddr
        if let Some(peer_id) = extract_peer_id(&addr) {
            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
            swarm.dial(addr)?;
        } else {
            error!("Bootstrap address must include peer ID (/p2p/<peer_id>)");
        }
    }

    // Track if we've executed the command
    let mut command_executed = false;
    let mut connected = false;

    // Event loop
    loop {
        // Execute command once connected (or immediately if no bootstrap)
        if !command_executed && (connected || bootstrap_addr.is_none()) {
            match &command {
                Command::Put { key, value } => {
                    info!("Storing: {} = {}", key, value);
                    let record = Record {
                        key: RecordKey::new(&key),
                        value: value.as_bytes().to_vec(),
                        publisher: Some(local_peer_id),
                        expires: None,
                    };
                    swarm.behaviour_mut().kademlia.put_record(record, kad::Quorum::One)?;
                    command_executed = true;
                }
                Command::Get { key } => {
                    info!("Requesting: {}", key);
                    swarm.behaviour_mut().kademlia.get_record(RecordKey::new(&key));
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

                // Bootstrap Kademlia
                if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
                    warn!("Bootstrap error: {}", e);
                }
            }
            SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(kad::Event::OutboundQueryProgressed {
                result: QueryResult::PutRecord(Ok(put_ok)),
                ..
            })) => {
                info!("Successfully stored record: {:?}", put_ok.key);
                println!("\n  SUCCESS: Stored value in DHT\n");
            }
            SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(kad::Event::OutboundQueryProgressed {
                result: QueryResult::GetRecord(Ok(get_ok)),
                ..
            })) => {
                for peer_record in get_ok.records {
                    let value = String::from_utf8_lossy(&peer_record.record.value);
                    info!("Retrieved record: {:?} = {}", peer_record.record.key, value);
                    println!("\n  SUCCESS: Retrieved '{}'\n", value);
                }
            }
            SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(kad::Event::OutboundQueryProgressed {
                result: QueryResult::GetRecord(Err(e)),
                ..
            })) => {
                warn!("Get record failed: {:?}", e);
                println!("\n  FAILED: Could not retrieve value\n");
            }
            SwarmEvent::Behaviour(DhtBehaviourEvent::Identify(identify::Event::Received {
                peer_id,
                info,
            })) => {
                info!("Identified: {} ({})", peer_id, info.protocol_version);
                for addr in info.listen_addrs {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                }
            }
            SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(kad::Event::RoutingUpdated {
                peer, ..
            })) => {
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
