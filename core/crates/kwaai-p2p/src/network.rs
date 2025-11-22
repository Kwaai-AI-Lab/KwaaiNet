//! Main network implementation

use crate::{
    config::NetworkConfig,
    dht::DhtManager,
    error::{P2PError, P2PResult},
    protocol::KwaaiProtocol,
    DhtOperations, NetworkBehaviour, NodeCapabilities, Request, Response,
};
use async_trait::async_trait;
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode},
    noise,
    swarm::{NetworkBehaviour as SwarmBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info, warn};

/// The main KwaaiNet P2P network manager
pub struct KwaaiNetwork {
    /// Local peer ID
    local_peer_id: PeerId,

    /// Network configuration
    config: NetworkConfig,

    /// Swarm (wrapped in Arc<Mutex> for thread-safe async access)
    swarm: Arc<Mutex<Option<Swarm<KwaaiBehaviour>>>>,

    /// DHT manager
    dht: Arc<RwLock<DhtManager>>,

    /// Connected peers
    connected_peers: Arc<RwLock<HashMap<PeerId, PeerInfo>>>,

    /// Is network running (atomic for thread-safe access)
    is_running: AtomicBool,
}

/// Information about a connected peer
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// Peer's addresses
    pub addresses: Vec<Multiaddr>,
    /// Peer's capabilities
    pub capabilities: Option<NodeCapabilities>,
    /// Connection time
    pub connected_at: std::time::Instant,
}

/// Combined network behaviour for libp2p swarm
#[derive(SwarmBehaviour)]
#[behaviour(to_swarm = "KwaaiBehaviourEvent")]
pub struct KwaaiBehaviour {
    /// Kademlia DHT for peer discovery
    pub kademlia: kad::Behaviour<MemoryStore>,
    /// Identify protocol for peer info exchange
    pub identify: identify::Behaviour,
    /// Custom KwaaiNet protocol
    pub kwaai: KwaaiProtocol,
}

/// Events from the combined behaviour
#[derive(Debug)]
pub enum KwaaiBehaviourEvent {
    /// Kademlia event
    Kademlia(kad::Event),
    /// Identify event
    Identify(identify::Event),
    /// KwaaiNet protocol event
    Kwaai(()),
}

impl From<kad::Event> for KwaaiBehaviourEvent {
    fn from(event: kad::Event) -> Self {
        KwaaiBehaviourEvent::Kademlia(event)
    }
}

impl From<identify::Event> for KwaaiBehaviourEvent {
    fn from(event: identify::Event) -> Self {
        KwaaiBehaviourEvent::Identify(event)
    }
}

impl From<()> for KwaaiBehaviourEvent {
    fn from(_: ()) -> Self {
        KwaaiBehaviourEvent::Kwaai(())
    }
}

impl KwaaiNetwork {
    /// Create a new network instance
    pub async fn new(config: NetworkConfig) -> P2PResult<Self> {
        // Generate identity keypair
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("Local peer ID: {}", local_peer_id);

        // Create the swarm
        let swarm = Self::create_swarm(local_key.clone(), &config)?;

        Ok(Self {
            local_peer_id,
            config,
            swarm: Arc::new(Mutex::new(Some(swarm))),
            dht: Arc::new(RwLock::new(DhtManager::new())),
            connected_peers: Arc::new(RwLock::new(HashMap::new())),
            is_running: AtomicBool::new(false),
        })
    }

    /// Create the libp2p swarm with configured behaviours
    fn create_swarm(
        local_key: identity::Keypair,
        config: &NetworkConfig,
    ) -> P2PResult<Swarm<KwaaiBehaviour>> {
        let local_peer_id = PeerId::from(local_key.public());

        // Create Kademlia behaviour
        let kademlia = {
            let store = MemoryStore::new(local_peer_id);
            let mut kad_config = kad::Config::default();
            kad_config.set_replication_factor(
                std::num::NonZeroUsize::new(config.dht_replication).unwrap(),
            );
            let mut behaviour = kad::Behaviour::with_config(local_peer_id, store, kad_config);
            behaviour.set_mode(Some(Mode::Server));
            behaviour
        };

        // Create Identify behaviour
        let identify = identify::Behaviour::new(identify::Config::new(
            config.protocol_version.clone(),
            local_key.public(),
        ));

        // Create custom protocol
        let kwaai = KwaaiProtocol::new();

        let behaviour = KwaaiBehaviour {
            kademlia,
            identify,
            kwaai,
        };

        // Build the swarm
        let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| P2PError::Transport(e.to_string()))?
            .with_behaviour(|_| Ok(behaviour))
            .map_err(|e| P2PError::Internal(e.to_string()))?
            .build();

        Ok(swarm)
    }

    /// Start listening on configured addresses
    pub async fn start(&self) -> P2PResult<()> {
        let mut swarm_guard = self.swarm.lock().await;
        let swarm = swarm_guard.as_mut().ok_or(P2PError::NotInitialized)?;

        for addr_str in &self.config.listen_addrs {
            let addr: Multiaddr = addr_str.parse().map_err(|e: libp2p::multiaddr::Error| {
                P2PError::InvalidAddress(e.to_string())
            })?;
            swarm
                .listen_on(addr.clone())
                .map_err(|e| P2PError::Transport(e.to_string()))?;
            info!("Listening on {}", addr);
        }

        self.is_running.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Get the local peer ID
    pub fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// Check if network is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl NetworkBehaviour for KwaaiNetwork {
    async fn bootstrap(&mut self, peers: Vec<Multiaddr>) -> P2PResult<()> {
        let mut swarm_guard = self.swarm.lock().await;
        let swarm = swarm_guard.as_mut().ok_or(P2PError::NotInitialized)?;

        for addr in peers {
            info!("Dialing bootstrap peer: {}", addr);
            swarm
                .dial(addr.clone())
                .map_err(|e| P2PError::DialFailed(e.to_string()))?;

            // Add to Kademlia routing table if we can extract peer ID
            if let Some(peer_id) = extract_peer_id(&addr) {
                swarm
                    .behaviour_mut()
                    .kademlia
                    .add_address(&peer_id, addr.clone());
            }
        }

        // Bootstrap Kademlia
        swarm
            .behaviour_mut()
            .kademlia
            .bootstrap()
            .map_err(|e| P2PError::DhtError(e.to_string()))?;

        Ok(())
    }

    async fn find_peers(&self, capability: &str) -> P2PResult<Vec<PeerId>> {
        let dht = self.dht.read().await;
        dht.find_providers(capability).await
    }

    async fn send_request(&self, _peer: PeerId, _request: Request) -> P2PResult<Response> {
        // TODO: Implement request/response protocol
        Err(P2PError::Internal("Not yet implemented".to_string()))
    }

    fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    fn is_connected(&self) -> bool {
        self.is_running.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl DhtOperations for KwaaiNetwork {
    async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()> {
        let mut dht = self.dht.write().await;
        dht.put(key, value).await
    }

    async fn get(&self, key: &str) -> P2PResult<Option<Vec<u8>>> {
        let dht = self.dht.read().await;
        dht.get(key).await
    }

    async fn provide(&mut self, key: &str) -> P2PResult<()> {
        let mut dht = self.dht.write().await;
        dht.provide(key).await
    }

    async fn get_providers(&self, key: &str) -> P2PResult<Vec<PeerId>> {
        let dht = self.dht.read().await;
        dht.find_providers(key).await
    }
}

/// Extract peer ID from a multiaddress if present
fn extract_peer_id(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| {
        if let libp2p::multiaddr::Protocol::P2p(peer_id) = p {
            Some(peer_id)
        } else {
            None
        }
    })
}
