//! Test Petals DHT Integration
//!
//! Uses the new NetworkConfig::with_petals_bootstrap() to join
//! the Petals network and discover peers.
//!
//! Run with: cargo run --example petals_dht

use futures::StreamExt;
use kwaai_p2p::{NetworkConfig, PETALS_BOOTSTRAP_SERVERS};
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode, QueryResult},
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId,
};
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[derive(NetworkBehaviour)]
struct DhtBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    println!("KwaaiNet + Petals DHT Test\n");
    println!("==========================\n");

    // Use the new with_petals_bootstrap config
    let config = NetworkConfig::with_petals_bootstrap();

    println!("Using Petals bootstrap servers:");
    for server in &config.bootstrap_peers {
        println!("  {}", server);
    }
    println!();

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local Peer ID: {}\n", local_peer_id);

    // Setup Kademlia with IPFS-compatible protocol
    let kademlia = {
        let store = MemoryStore::new(local_peer_id);
        let mut kad_config = kad::Config::default();
        kad_config.set_protocol_names(vec![
            libp2p::StreamProtocol::new("/ipfs/kad/1.0.0"),
        ]);
        let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, kad_config);
        behaviour.set_mode(Some(Mode::Client));
        behaviour
    };

    let identify = identify::Behaviour::new(identify::Config::new(
        "/kwaai/1.0.0".to_string(),
        local_key.public(),
    ));

    let behaviour = DhtBehaviour { kademlia, identify };

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
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Connect to Petals bootstrap
    println!("Connecting to Petals network...\n");
    for addr_str in &config.bootstrap_peers {
        let addr: Multiaddr = addr_str.parse()?;
        if let Some(peer_id) = extract_peer_id(&addr) {
            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr.clone());
            swarm.dial(addr)?;
        }
    }

    let mut connected = false;
    let mut bootstrap_done = false;
    let mut peers_found = 0;

    let timeout = tokio::time::sleep(Duration::from_secs(45));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                println!("\n--- Test complete ---\n");
                break;
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("Listening on: {}", address);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        if !connected {
                            connected = true;
                            println!("[CONNECTED] to Petals network via {}", peer_id);

                            // Start DHT bootstrap
                            println!("[DHT] Starting bootstrap...");
                            if let Err(e) = swarm.behaviour_mut().kademlia.bootstrap() {
                                warn!("Bootstrap error: {}", e);
                            }
                        }
                    }
                    SwarmEvent::Behaviour(DhtBehaviourEvent::Identify(
                        identify::Event::Received { peer_id, info },
                    )) => {
                        println!("[IDENTIFIED] {} - {}", peer_id, info.agent_version);
                        // Add discovered addresses to Kademlia
                        for addr in info.listen_addrs {
                            swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
                        }
                    }
                    SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed {
                            result: QueryResult::Bootstrap(Ok(stats)),
                            ..
                        },
                    )) => {
                        if !bootstrap_done {
                            bootstrap_done = true;
                            println!("[DHT] Bootstrap complete! {} peers in routing table",
                                     stats.num_remaining + 1);

                            // Now find random peers to discover more nodes
                            println!("[DHT] Searching for random peers...");
                            let random_peer = PeerId::random();
                            swarm.behaviour_mut().kademlia.get_closest_peers(random_peer);
                        }
                    }
                    SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(
                        kad::Event::OutboundQueryProgressed {
                            result: QueryResult::GetClosestPeers(Ok(result)),
                            ..
                        },
                    )) => {
                        let new_peers = result.peers.len();
                        peers_found += new_peers;
                        println!("[DHT] Found {} peers (total: {})", new_peers, peers_found);

                        for peer in &result.peers {
                            println!("      {}", peer);
                        }
                    }
                    SwarmEvent::Behaviour(DhtBehaviourEvent::Kademlia(
                        kad::Event::RoutingUpdated { peer, .. },
                    )) => {
                        println!("[DHT] Routing table updated: {}", peer);
                    }
                    _ => {}
                }
            }
        }
    }

    println!("==========================");
    println!("Results:");
    println!("  Connected to Petals: {}", if connected { "YES" } else { "NO" });
    println!("  DHT bootstrap: {}", if bootstrap_done { "COMPLETE" } else { "INCOMPLETE" });
    println!("  Peers discovered: {}", peers_found);

    if connected && bootstrap_done {
        println!("\n  KwaaiNet <-> Petals DHT integration: WORKING!");
    }

    Ok(())
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
