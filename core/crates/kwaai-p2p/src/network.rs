//! Main network implementation

use crate::{
    config::NetworkConfig,
    dht::{DhtCommand, DhtManager},
    error::{P2PError, P2PResult},
    protocol::KwaaiProtocol,
    rpc::HivemindCodec,
    DhtOperations, NetworkBehaviour, NodeCapabilities, Request, Response,
};
use async_trait::async_trait;
use libp2p::{
    identify, identity,
    kad::{self, store::MemoryStore, Mode, Quorum, Record, RecordKey},
    noise, request_response,
    swarm::{NetworkBehaviour as SwarmBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{mpsc, Mutex, RwLock};
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

    /// DHT command receiver
    dht_command_rx: Arc<Mutex<mpsc::UnboundedReceiver<DhtCommand>>>,
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
    /// Hivemind RPC protocol for health monitor queries
    pub rpc: request_response::Behaviour<HivemindCodec>,
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
    /// RPC event
    Rpc(request_response::Event<crate::rpc::RpcRequest, crate::rpc::RpcResponse>),
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

impl From<request_response::Event<crate::rpc::RpcRequest, crate::rpc::RpcResponse>>
    for KwaaiBehaviourEvent
{
    fn from(
        event: request_response::Event<crate::rpc::RpcRequest, crate::rpc::RpcResponse>,
    ) -> Self {
        KwaaiBehaviourEvent::Rpc(event)
    }
}

impl KwaaiNetwork {
    /// Create a new network instance
    pub async fn new(config: NetworkConfig) -> P2PResult<Self> {
        // Generate identity keypair
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("Local peer ID: {}", local_peer_id);

        // Create DHT command channel
        let (dht_command_tx, dht_command_rx) = mpsc::unbounded_channel();

        // Create the swarm
        let swarm = Self::create_swarm(local_key.clone(), &config)?;

        Ok(Self {
            local_peer_id,
            config,
            swarm: Arc::new(Mutex::new(Some(swarm))),
            dht: Arc::new(RwLock::new(DhtManager::with_channel(dht_command_tx))),
            connected_peers: Arc::new(RwLock::new(HashMap::new())),
            is_running: AtomicBool::new(false),
            dht_command_rx: Arc::new(Mutex::new(dht_command_rx)),
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

        // Create RPC protocol for Hivemind compatibility
        let (rpc, _protocol) = crate::rpc::create_hivemind_protocol();

        let behaviour = KwaaiBehaviour {
            kademlia,
            identify,
            kwaai,
            rpc,
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

    /// Announce blocks to the DHT (Petals-compatible)
    ///
    /// This announces the node's availability to serve specific model blocks,
    /// making it discoverable on map.kwaai.ai
    pub async fn announce_blocks(
        &self,
        model_name: &str,
        server_info: &crate::hivemind::ServerInfo,
    ) -> P2PResult<()> {
        use crate::hivemind::ServerInfo;

        info!("Announcing blocks to DHT for model: {}", model_name);

        // Serialize server info to MessagePack
        let info_bytes = server_info
            .to_msgpack()
            .map_err(|e| P2PError::Serialization(e.to_string()))?;

        info!(
            "Announcing {} blocks ({}..{}), info size: {} bytes",
            server_info.end_block - server_info.start_block,
            server_info.start_block,
            server_info.end_block,
            info_bytes.len()
        );

        // Get DHT manager with write lock
        let mut dht = self.dht.write().await;

        // Announce each block: {model_name}.{block_index}
        for block_idx in server_info.start_block..server_info.end_block {
            let module_uid = format!("{}.{}", model_name, block_idx);

            // Put DHT record
            dht.put(&module_uid, info_bytes.clone()).await?;

            // Start providing
            dht.provide(&module_uid).await?;

            debug!("Announced module: {}", module_uid);
        }

        // Also announce model metadata key
        let model_metadata_key = format!("_petals.models.{}", model_name);
        dht.put(&model_metadata_key, info_bytes).await?;

        info!("Block announcement complete for {}", model_name);

        Ok(())
    }

    /// Process a single DHT command from the channel
    pub async fn process_dht_command(&self) -> P2PResult<bool> {
        let mut rx = self.dht_command_rx.lock().await;

        match rx.try_recv() {
            Ok(command) => {
                let mut swarm_guard = self.swarm.lock().await;
                let swarm = swarm_guard.as_mut().ok_or(P2PError::NotInitialized)?;

                match command {
                    DhtCommand::PutRecord { key, value, publisher } => {
                        info!("Processing DHT PutRecord: {}", key);

                        let record = Record {
                            key: RecordKey::new(&key),
                            value,
                            publisher: publisher.or(Some(self.local_peer_id)),
                            expires: None,
                        };

                        swarm
                            .behaviour_mut()
                            .kademlia
                            .put_record(record, Quorum::One)
                            .map_err(|e| P2PError::DhtError(e.to_string()))?;

                        debug!("DHT record stored: {}", key);
                    }

                    DhtCommand::StartProviding { key } => {
                        info!("Processing DHT StartProviding: {}", key);

                        let record_key = RecordKey::new(&key);
                        swarm
                            .behaviour_mut()
                            .kademlia
                            .start_providing(record_key)
                            .map_err(|e| P2PError::DhtError(e.to_string()))?;

                        debug!("Started providing: {}", key);
                    }

                    DhtCommand::GetRecord { key } => {
                        info!("Processing DHT GetRecord: {}", key);

                        let record_key = RecordKey::new(&key);
                        swarm
                            .behaviour_mut()
                            .kademlia
                            .get_record(record_key);

                        debug!("DHT get record query sent: {}", key);
                    }

                    DhtCommand::GetProviders { key } => {
                        info!("Processing DHT GetProviders: {}", key);

                        let record_key = RecordKey::new(&key);
                        swarm
                            .behaviour_mut()
                            .kademlia
                            .get_providers(record_key);

                        debug!("DHT get providers query sent: {}", key);
                    }
                }

                Ok(true) // Command processed
            }
            Err(mpsc::error::TryRecvError::Empty) => {
                Ok(false) // No commands available
            }
            Err(mpsc::error::TryRecvError::Disconnected) => {
                Err(P2PError::Internal("DHT command channel disconnected".to_string()))
            }
        }
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
