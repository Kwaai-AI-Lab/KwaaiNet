//! Configuration for P2P networking

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Petals/Hivemind bootstrap servers for DHT discovery.
/// These enable KwaaiNet nodes to join the broader distributed ML network.
pub const PETALS_BOOTSTRAP_SERVERS: &[&str] = &[
    // bootstrap1.petals.dev - Primary Petals bootstrap
    "/ip4/159.89.214.152/tcp/31337/p2p/QmedTaZXmULqwspJXz44SsPZyTNKxhnnFvYRajfH7MGhCY",
    // Note: bootstrap2 (159.203.156.48:31338) was offline as of Nov 2024
];

/// Configuration for the P2P network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen addresses for incoming connections
    pub listen_addrs: Vec<String>,

    /// Bootstrap peers to connect to on startup
    pub bootstrap_peers: Vec<String>,

    /// Enable Kademlia DHT
    pub enable_dht: bool,

    /// DHT replication factor
    pub dht_replication: usize,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// Request timeout
    pub request_timeout: Duration,

    /// Maximum concurrent connections
    pub max_connections: usize,

    /// Enable NAT traversal
    pub enable_nat_traversal: bool,

    /// Enable relay client (for nodes behind NAT)
    pub enable_relay_client: bool,

    /// Protocol version string
    pub protocol_version: String,

    /// Agent version string
    pub agent_version: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
            bootstrap_peers: Vec::new(),
            enable_dht: true,
            dht_replication: 20,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_connections: 100,
            enable_nat_traversal: true,
            enable_relay_client: true,
            protocol_version: "kwaai/1.0.0".to_string(),
            agent_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

impl NetworkConfig {
    /// Create a new configuration builder
    pub fn builder() -> NetworkConfigBuilder {
        NetworkConfigBuilder::default()
    }

    /// Create config with Petals bootstrap servers included.
    /// This enables DHT discovery via the Petals/Hivemind network.
    pub fn with_petals_bootstrap() -> Self {
        let mut config = Self::default();
        config.bootstrap_peers = PETALS_BOOTSTRAP_SERVERS
            .iter()
            .map(|s| s.to_string())
            .collect();
        config
    }
}

/// Builder for NetworkConfig
#[derive(Default)]
pub struct NetworkConfigBuilder {
    config: NetworkConfig,
}

impl NetworkConfigBuilder {
    /// Set listen addresses
    pub fn listen_addrs(mut self, addrs: Vec<String>) -> Self {
        self.config.listen_addrs = addrs;
        self
    }

    /// Add bootstrap peers
    pub fn bootstrap_peers(mut self, peers: Vec<String>) -> Self {
        self.config.bootstrap_peers = peers;
        self
    }

    /// Set connection timeout
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.connection_timeout = timeout;
        self
    }

    /// Set request timeout
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.config.request_timeout = timeout;
        self
    }

    /// Set maximum connections
    pub fn max_connections(mut self, max: usize) -> Self {
        self.config.max_connections = max;
        self
    }

    /// Include Petals bootstrap servers for DHT discovery
    pub fn with_petals_bootstrap(mut self) -> Self {
        self.config.bootstrap_peers.extend(
            PETALS_BOOTSTRAP_SERVERS
                .iter()
                .map(|s| s.to_string())
        );
        self
    }

    /// Build the configuration
    pub fn build(self) -> NetworkConfig {
        self.config
    }
}
