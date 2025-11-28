//! # kwaai-p2p
//!
//! P2P networking layer for KwaaiNet using libp2p with Kademlia DHT.
//!
//! This crate provides the foundational networking infrastructure for
//! decentralized AI inference and training, including:
//!
//! - **Peer Discovery**: Kademlia DHT for finding nodes by capability
//! - **Message Routing**: Request/response protocols for inference
//! - **NAT Traversal**: Hole punching and relay circuits
//!
//! ## Example
//!
//! ```rust,no_run
//! use kwaai_p2p::{KwaaiNetwork, NetworkConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = NetworkConfig::default();
//!     let mut network = KwaaiNetwork::new(config).await?;
//!
//!     // Join the network
//!     network.bootstrap(vec!["/ip4/127.0.0.1/tcp/4001".parse()?]).await?;
//!
//!     // Find peers with inference capability
//!     let peers = network.find_peers("inference:llama2-7b").await?;
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod dht;
pub mod error;
pub mod hivemind;
pub mod network;
pub mod protocol;
pub mod rpc;
pub mod transport;

pub use config::{NetworkConfig, PETALS_BOOTSTRAP_SERVERS};
pub use error::{P2PError, P2PResult};
pub use network::KwaaiNetwork;

use async_trait::async_trait;
use libp2p::{Multiaddr, PeerId};
use serde::{Deserialize, Serialize};

/// Core trait for P2P network operations
///
/// Implementors provide the fundamental networking capabilities
/// required for distributed AI inference.
#[async_trait]
pub trait NetworkBehaviour: Send + Sync {
    /// Join the network via bootstrap peers
    async fn bootstrap(&mut self, peers: Vec<Multiaddr>) -> P2PResult<()>;

    /// Find peers with specific capabilities
    async fn find_peers(&self, capability: &str) -> P2PResult<Vec<PeerId>>;

    /// Send a request to a specific peer
    async fn send_request(&self, peer: PeerId, request: Request) -> P2PResult<Response>;

    /// Get the local peer ID
    fn local_peer_id(&self) -> PeerId;

    /// Check if connected to the network
    fn is_connected(&self) -> bool;
}

/// Core trait for DHT operations
///
/// Provides distributed hash table functionality for peer discovery
/// and capability registration.
#[async_trait]
pub trait DhtOperations: Send + Sync {
    /// Store a value in the DHT
    async fn put(&mut self, key: &str, value: Vec<u8>) -> P2PResult<()>;

    /// Retrieve a value from the DHT
    async fn get(&self, key: &str) -> P2PResult<Option<Vec<u8>>>;

    /// Announce this node as a provider for a key
    async fn provide(&mut self, key: &str) -> P2PResult<()>;

    /// Find providers for a key
    async fn get_providers(&self, key: &str) -> P2PResult<Vec<PeerId>>;
}

/// Request message for P2P communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// Unique request identifier
    pub id: u64,
    /// Request type
    pub request_type: RequestType,
    /// Request payload
    pub payload: Vec<u8>,
}

/// Response message for P2P communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Request ID this responds to
    pub request_id: u64,
    /// Response status
    pub status: ResponseStatus,
    /// Response payload
    pub payload: Vec<u8>,
}

/// Types of requests supported by the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestType {
    /// Request inference from a remote expert
    InferenceRequest,
    /// Request parameter exchange for averaging
    ParameterExchange,
    /// Health check / ping
    Ping,
    /// Capability query
    CapabilityQuery,
}

/// Response status codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    /// Request successful
    Ok,
    /// Request failed with error
    Error(String),
    /// Peer is busy, try again later
    Busy,
    /// Capability not available
    NotAvailable,
}

/// Node capabilities advertised in DHT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// Peer ID
    pub peer_id: String,
    /// Can perform inference
    pub can_inference: bool,
    /// Can participate in training
    pub can_train: bool,
    /// Available model IDs
    pub model_ids: Vec<String>,
    /// Available expert IDs (for MoE)
    pub expert_ids: Vec<String>,
    /// Estimated compute power (TFLOPS)
    pub compute_power: f32,
    /// Available memory (MB)
    pub available_memory: u64,
}

impl NodeCapabilities {
    /// Create new capabilities with defaults
    pub fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            can_inference: false,
            can_train: false,
            model_ids: Vec::new(),
            expert_ids: Vec::new(),
            compute_power: 0.0,
            available_memory: 0,
        }
    }

    /// Encode capabilities for DHT storage
    pub fn encode(&self) -> P2PResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| P2PError::Serialization(e.to_string()))
    }

    /// Decode capabilities from DHT
    pub fn decode(data: &[u8]) -> P2PResult<Self> {
        bincode::deserialize(data).map_err(|e| P2PError::Serialization(e.to_string()))
    }
}
