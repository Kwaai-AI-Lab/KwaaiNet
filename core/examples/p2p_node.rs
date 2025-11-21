//! Day 1: P2P Node Startup Example
//!
//! Demonstrates basic P2P node initialization:
//! - Generate identity keypair
//! - Create libp2p swarm
//! - Listen on TCP
//! - Log peer ID
//!
//! Run with: cargo run --example p2p_node

use futures::StreamExt;
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode},
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

/// Combined network behaviour
#[derive(NetworkBehaviour)]
struct NodeBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    info!("Starting KwaaiNet P2P Node...");

    // Generate a random keypair for this node
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Node Peer ID: {}", local_peer_id);

    // Create Kademlia behaviour
    let kademlia = {
        let store = MemoryStore::new(local_peer_id);
        let mut config = kad::Config::default();
        config.set_replication_factor(std::num::NonZeroUsize::new(3).unwrap());
        let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, config);
        behaviour.set_mode(Some(Mode::Server));
        behaviour
    };

    // Create Identify behaviour
    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let behaviour = NodeBehaviour { kademlia, identify };

    // Build the swarm
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

    // Parse listen address from args or use default
    let listen_addr: Multiaddr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/ip4/0.0.0.0/tcp/0".to_string())
        .parse()?;

    // Start listening
    swarm.listen_on(listen_addr.clone())?;
    info!("Listening on {}", listen_addr);

    // Event loop
    info!("Node is running. Press Ctrl+C to stop.");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                let full_addr = format!("{}/p2p/{}", address, local_peer_id);
                info!("Listening on: {}", full_addr);
                println!("\n  Connect with: cargo run --example p2p_node -- {}\n", full_addr);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                info!("Connected to peer: {}", peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                info!("Disconnected from peer: {}", peer_id);
            }
            SwarmEvent::IncomingConnection { local_addr, send_back_addr, .. } => {
                info!("Incoming connection from {} to {}", send_back_addr, local_addr);
            }
            SwarmEvent::Behaviour(NodeBehaviourEvent::Identify(identify::Event::Received {
                peer_id,
                info,
            })) => {
                info!(
                    "Identified peer {}: {} ({})",
                    peer_id,
                    info.protocol_version,
                    info.agent_version
                );
                // Add peer addresses to Kademlia
                for addr in info.listen_addrs {
                    swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                }
            }
            SwarmEvent::Behaviour(NodeBehaviourEvent::Kademlia(kad::Event::RoutingUpdated {
                peer,
                ..
            })) => {
                info!("Kademlia routing updated: {}", peer);
            }
            _ => {}
        }
    }
}
