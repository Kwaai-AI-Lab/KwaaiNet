//! Configuration for P2P networking

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// KwaaiNet bootstrap servers for DHT discovery.
/// These are the official KwaaiNet/Petals DHT entry points.
pub const KWAAI_BOOTSTRAP_SERVERS: &[&str] = &[
    // bootstrap-1.kwaai.ai (18.219.43.67) - Primary KwaaiNet bootstrap
    "/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
    // bootstrap-2.kwaai.ai (52.23.252.2) - Secondary KwaaiNet bootstrap
    "/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY",
];

/// Legacy Petals/Hivemind bootstrap servers (kept for reference).
pub const PETALS_BOOTSTRAP_SERVERS: &[&str] = &[
    // bootstrap-1.kwaai.ai (18.219.43.67) - Primary Kwaai bootstrap
    "/ip4/18.219.43.67/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
    // bootstrap-2.kwaai.ai (52.23.252.2) - Secondary Kwaai bootstrap
    "/ip4/52.23.252.2/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY",
    // uncomment for local development bootstrap server
    //"/ip4/127.0.0.1/tcp/8000/p2p/QmXwErKD4k7aLzgDWGuNj5yjEtiMuicGp72juNB3Yyqtt9"
];

/// KwaaiNet bootstrap servers addressed by DNS name rather than by a pinned IP.
///
/// Prefer these for the native swarm: the transport is built `.with_dns()`, and
/// a DNS name survives the bootstrap hosts being re-addressed. The `/ip4/`
/// constants above are kept because existing CLI call sites assert on them.
pub const KWAAI_BOOTSTRAP_SERVERS_DNS: &[&str] = &[
    "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
    "/dns/bootstrap-2.kwaai.ai/tcp/8000/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY",
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

    /// TCP port the swarm listens on (both IPv4 and IPv6). `0` = ephemeral,
    /// which is what tests want. Used by [`NetworkConfig::swarm_listen_addrs`].
    #[serde(default)]
    pub port: u16,

    /// Peers dialed at startup by `NetworkHandle::bootstrap`, as multiaddrs
    /// that include a `/p2p/<peer-id>` component. Distinct from
    /// `bootstrap_peers` (a `String` list kept for the existing CLI call sites)
    /// so callers can supply already-parsed addresses.
    #[serde(default)]
    pub initial_peers: Vec<String>,

    /// Force Kademlia into server mode instead of letting it auto-detect from
    /// confirmed external addresses. Needed for bootstrap-grade nodes and for
    /// in-process tests where nothing confirms an external address.
    #[serde(default)]
    pub dht_server: bool,
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
            protocol_version: crate::behaviour::DEFAULT_PROTOCOL_VERSION.to_string(),
            agent_version: crate::behaviour::default_agent_version(),
            port: 0,
            initial_peers: Vec::new(),
            dht_server: false,
        }
    }
}

impl NetworkConfig {
    /// Create a new configuration builder
    pub fn builder() -> NetworkConfigBuilder {
        NetworkConfigBuilder::default()
    }

    /// Create config with KwaaiNet bootstrap servers included.
    /// This enables DHT discovery via the KwaaiNet/Hivemind network.
    pub fn with_kwaai_bootstrap() -> Self {
        Self {
            bootstrap_peers: KWAAI_BOOTSTRAP_SERVERS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ..Self::default()
        }
    }

    /// Config for an in-process test swarm: loopback-only, ephemeral port, kad
    /// forced into server mode (nothing will confirm an external address on
    /// 127.0.0.1, and a client-mode kad answers no queries).
    pub fn for_tests() -> Self {
        Self {
            listen_addrs: vec!["/ip4/127.0.0.1/tcp/0".to_string()],
            port: 0,
            dht_server: true,
            ..Self::default()
        }
    }

    /// Multiaddrs the swarm should listen on.
    ///
    /// If `listen_addrs` was set explicitly it wins; otherwise both the IPv4
    /// and IPv6 wildcards on [`NetworkConfig::port`] are used.
    pub fn swarm_listen_addrs(&self) -> Vec<String> {
        if !self.listen_addrs.is_empty() {
            return self.listen_addrs.clone();
        }
        vec![
            format!("/ip4/0.0.0.0/tcp/{}", self.port),
            format!("/ip6/::/tcp/{}", self.port),
        ]
    }

    /// The peers to dial at startup: `initial_peers` if set, else
    /// `bootstrap_peers`.
    pub fn effective_initial_peers(&self) -> Vec<String> {
        if self.initial_peers.is_empty() {
            self.bootstrap_peers.clone()
        } else {
            self.initial_peers.clone()
        }
    }

    /// Create config with Petals bootstrap servers included (legacy).
    /// This enables DHT discovery via the Petals/Hivemind network.
    pub fn with_petals_bootstrap() -> Self {
        Self {
            bootstrap_peers: PETALS_BOOTSTRAP_SERVERS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ..Self::default()
        }
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

    /// Set the TCP listen port (0 = ephemeral)
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set the peers dialed at startup
    pub fn initial_peers(mut self, peers: Vec<String>) -> Self {
        self.config.initial_peers = peers;
        self
    }

    /// Force Kademlia server mode
    pub fn dht_server(mut self, server: bool) -> Self {
        self.config.dht_server = server;
        self
    }

    /// Include Petals bootstrap servers for DHT discovery
    pub fn with_petals_bootstrap(mut self) -> Self {
        self.config
            .bootstrap_peers
            .extend(PETALS_BOOTSTRAP_SERVERS.iter().map(|s| s.to_string()));
        self
    }

    /// Build the configuration
    pub fn build(self) -> NetworkConfig {
        self.config
    }
}
