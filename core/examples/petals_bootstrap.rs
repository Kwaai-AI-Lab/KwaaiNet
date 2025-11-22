//! Petals Bootstrap Connection Test
//!
//! Tests connectivity to Petals/Hivemind bootstrap servers.
//! This demonstrates libp2p transport-layer compatibility.
//!
//! Run with: cargo run --example petals_bootstrap

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

#[derive(NetworkBehaviour)]
struct BootstrapBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    println!("KwaaiNet <-> Petals Bootstrap Test\n");
    println!("===================================\n");

    // Petals bootstrap servers
    let bootstrap_addrs = vec![
        "/ip4/159.89.214.152/tcp/31337/p2p/QmedTaZXmULqwspJXz44SsPZyTNKxhnnFvYRajfH7MGhCY",
        "/ip4/159.203.156.48/tcp/31338/p2p/QmQGTqmM7NKjV6ggU1ZCap8zWiyKR89RViDXiqehSiCpY5",
    ];

    println!("Petals bootstrap servers:");
    for addr in &bootstrap_addrs {
        println!("  {}", addr);
    }
    println!();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("Local Peer ID: {}", local_peer_id);

    // Setup Kademlia with Hivemind-compatible protocol
    let kademlia = {
        let store = MemoryStore::new(local_peer_id);
        let mut config = kad::Config::default();
        // Try to use a protocol that might be compatible
        config.set_protocol_names(vec![
            libp2p::StreamProtocol::new("/ipfs/kad/1.0.0"),
        ]);
        let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, config);
        behaviour.set_mode(Some(Mode::Client));
        behaviour
    };

    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let behaviour = BootstrapBehaviour { kademlia, identify };

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(30)))
        .build();

    // Listen on random port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Try to connect to Petals bootstrap servers
    println!("Attempting to connect to Petals bootstrap servers...\n");

    for addr_str in &bootstrap_addrs {
        let addr: Multiaddr = addr_str.parse()?;

        // Extract peer ID from address
        let peer_id = addr.iter().find_map(|p| {
            if let libp2p::multiaddr::Protocol::P2p(id) = p {
                Some(id)
            } else {
                None
            }
        });

        if let Some(peer_id) = peer_id {
            info!("Dialing: {} (peer: {})", addr, peer_id);
            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());

            match swarm.dial(addr.clone()) {
                Ok(_) => println!("  Dial initiated: {}", addr),
                Err(e) => println!("  Dial failed: {} - {}", addr, e),
            }
        }
    }

    println!("\nWaiting for connection results (30 second timeout)...\n");

    let timeout = tokio::time::sleep(Duration::from_secs(30));
    tokio::pin!(timeout);

    let mut connections = 0;
    let mut identified = 0;

    loop {
        tokio::select! {
            _ = &mut timeout => {
                println!("\n--- Timeout reached ---");
                break;
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("Listening on: {}", address);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                        connections += 1;
                        println!("  [CONNECTED] {} via {:?}", peer_id, endpoint);
                    }
                    SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                        println!("  [DISCONNECTED] {} - {:?}", peer_id, cause);
                    }
                    SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                        println!("  [CONNECTION ERROR] {:?} - {}", peer_id, error);
                    }
                    SwarmEvent::Behaviour(BootstrapBehaviourEvent::Identify(
                        identify::Event::Received { peer_id, info },
                    )) => {
                        identified += 1;
                        println!("\n  [IDENTIFIED] {}", peer_id);
                        println!("    Protocol: {}", info.protocol_version);
                        println!("    Agent: {}", info.agent_version);
                        println!("    Protocols supported: {:?}", info.protocols);
                        println!("    Listen addresses: {:?}", info.listen_addrs);
                    }
                    SwarmEvent::Behaviour(BootstrapBehaviourEvent::Kademlia(
                        kad::Event::RoutingUpdated { peer, .. },
                    )) => {
                        println!("  [DHT] Routing updated: {}", peer);
                    }
                    SwarmEvent::Behaviour(BootstrapBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed { result, .. },
                    )) => {
                        println!("  [DHT] Query progress: {:?}", result);
                    }
                    _ => {}
                }
            }
        }
    }

    println!("\n===================================");
    println!("Results:");
    println!("  Connections established: {}", connections);
    println!("  Peers identified: {}", identified);

    if connections > 0 {
        println!("\n  Transport-layer connectivity to Petals: WORKING");
        if identified > 0 {
            println!("  Protocol identification: WORKING");
        }
    } else {
        println!("\n  Could not connect to Petals bootstrap servers.");
        println!("  Possible reasons:");
        println!("    - Servers may be down or changed");
        println!("    - Firewall blocking outbound connections");
        println!("    - Protocol incompatibility at transport layer");
    }

    Ok(())
}
