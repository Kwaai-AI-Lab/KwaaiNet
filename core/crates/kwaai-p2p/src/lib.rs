//! # kwaai-p2p
//!
//! The in-process rust-libp2p swarm for a KwaaiNet node.
//!
//! A [`NetworkService`] owns the `Swarm` on its own tokio task; everything else
//! talks to it through a clonable [`NetworkHandle`]. That split exists because
//! the swarm must be polled from exactly one place, while many call sites need
//! to dial peers, list connections and run DHT lookups concurrently.
//!
//! Phase 1 covers ping, identify and Kademlia — enough for a node to join the
//! network, learn its observed addresses, and resolve peers. Phase 2 adds
//! hivemind unary RPC: [`NetworkHandle::call_unary_handler`] to call a remote
//! handler and [`NetworkHandle::add_unary_handler`] to serve one, both named
//! after their `kwaai_p2p_daemon::P2PClient` counterparts. Relay, dcutr,
//! AutoNAT and UPnP arrive in later phases; [`behaviour::KwaaiBehaviour`] is
//! structured so each is an additive field.
//!
//! ## Example
//!
//! ```rust,no_run
//! use kwaai_p2p::{NetworkConfig, NetworkService};
//! use libp2p::identity::Keypair;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = NetworkConfig::default();
//!     let (handle, _task) = NetworkService::spawn(config, Keypair::generate_ed25519())?;
//!
//!     let addr = "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc";
//!     handle.connect_peer(addr).await?;
//!
//!     for peer in handle.list_peers().await? {
//!         println!("{} via {} ({})", peer.peer_id, peer.addr, peer.direction.as_str());
//!     }
//!
//!     handle.shutdown().await?;
//!     Ok(())
//! }
//! ```

pub mod addresses;
pub mod behaviour;
pub mod config;
pub mod dht_service;
pub mod error;
pub mod handle;
pub mod identity;
pub mod raw_stream;
pub mod reachability;
pub mod relay_manager;
pub mod service;
pub mod transport;
pub mod unary;

pub use addresses::{is_announceable, is_circuit, is_routable_v4};
pub use behaviour::{KwaaiBehaviour, KwaaiBehaviourEvent};
pub use config::{
    NetworkConfig, KWAAI_BOOTSTRAP_SERVERS, KWAAI_BOOTSTRAP_SERVERS_DNS, PETALS_BOOTSTRAP_SERVERS,
};
pub use dht_service::{remove_dht_service, spawn_dht_service};
pub use error::{P2PError, P2PResult};
pub use handle::{Direction, InboundUnaryCall, NetworkHandle, PeerInfo, UnaryHandler};
pub use raw_stream::{InboundStream, RawStream, RawStreamError};
pub use reachability::{
    AnnounceState, Reachability, ReachabilityKind, Source as ReachabilitySource,
};
pub use service::NetworkService;

// Re-exported so downstream crates can name peers and addresses without taking
// their own libp2p dependency (and version-skewing against ours).
pub use libp2p::{Multiaddr, PeerId};

use serde::{Deserialize, Serialize};

/// Node capabilities advertised in the DHT.
///
/// Retained from the pre-rewrite crate because `kwaai-network-tests` asserts on
/// its encoding. The announce path that will actually publish this lands in
/// Phase 2 (hivemind `ServerInfo` records), at which point this type is either
/// wired up or removed.
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
