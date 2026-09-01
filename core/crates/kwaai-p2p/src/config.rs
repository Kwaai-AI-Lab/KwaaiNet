//! Configuration for P2P networking

use libp2p::StreamProtocol;
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

/// KwaaiNet's own Kademlia protocol ID.
pub const KWAAI_KAD_PROTOCOL: &str = "/kwaai/kad/1.0.0";

/// The default libp2p Kademlia protocol ID — shared with the public IPFS DHT,
/// which is exactly the problem: any node serving it on a public address gets
/// absorbed into IPFS's routing tables and crawled indefinitely. Kept only for
/// wire compatibility with peers that predate [`KWAAI_KAD_PROTOCOL`].
pub const LEGACY_KAD_PROTOCOL: &str = "/ipfs/kad/1.0.0";

/// KwaaiNet bootstrap servers addressed by `/dnsaddr/`.
///
/// TXT records at `_dnsaddr.bootstrap.kwaai.ai` name each bootstrap's current
/// transport, so re-addressing a host is a DNS edit rather than a release. The
/// `/p2p/<id>` suffix pins the identity and selects that peer's record; adding
/// a port or transport here would filter out any record that later differs.
pub const KWAAI_BOOTSTRAP_SERVERS_DNS: &[&str] = &[
    "/dnsaddr/bootstrap.kwaai.ai/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc",
    "/dnsaddr/bootstrap.kwaai.ai/p2p/Qmd3A8N5aQBATe2SYvNikaeCS9CAKN4E86jdCPacZ6RZJY",
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

    /// How long a connection may sit idle before the swarm closes it.
    #[serde(alias = "connection_timeout")]
    pub idle_connection_timeout: Duration,

    /// Request timeout
    pub request_timeout: Duration,

    /// Interval of the kad maintenance tick: bucket refresh plus re-dialing
    /// configured bootstraps with no recent connection. Defaulted so a config
    /// predating the field still loads; tests shorten it. Clamped to at least
    /// a second when the service starts — `serde(default)` covers a missing
    /// field, not an explicit `0s`, which `tokio::time::interval` panics on.
    #[serde(default = "default_kad_maintenance_interval")]
    pub kad_maintenance_interval: Duration,

    /// Maximum concurrent connections, inbound and outbound.
    pub max_connections: usize,

    /// Listen on and dial QUIC as well as TCP. Off by default: some networks
    /// block or throttle UDP. Defaulted so a config predating the field still loads.
    #[serde(default)]
    pub enable_quic: bool,

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
    /// that include a `/p2p/<peer-id>` component. Takes precedence over
    /// `bootstrap_peers`.
    #[serde(default)]
    pub initial_peers: Vec<String>,

    /// Force Kademlia into server mode instead of letting it auto-detect from
    /// confirmed external addresses. Needed for bootstrap-grade nodes and for
    /// in-process tests where nothing confirms an external address.
    #[serde(default)]
    pub dht_server: bool,

    // ---------------------------------------------------------------
    // NAT traversal
    // ---------------------------------------------------------------
    /// Relays this node will try to hold a circuit reservation on, as
    /// multiaddrs carrying `/p2p/<peer-id>`.
    ///
    /// A pure **operator override**. The normal supply of relay candidates is
    /// identify: any peer that advertises `/libp2p/circuit/relay/0.2.0/hop` is
    /// a candidate, which on the live network includes both bootstraps. Set
    /// this only to pin specific relays — an isolated test topology, or a
    /// deployment where a known-good relay must be preferred. Configured
    /// relays are tried first and dialed alongside the bootstrap peers so the
    /// reservation does not wait on a lazy dial.
    #[serde(default)]
    pub trusted_relays: Vec<String>,

    /// Serve as a circuit relay for other peers (the `hop` side).
    ///
    /// On by default, matching the p2pd path's `-relay`. Rate limits stay at
    /// libp2p's defaults; see [`crate::behaviour`].
    #[serde(default = "default_true")]
    pub relay_server: bool,

    /// Bytes one relayed circuit may carry before this node tears it down; `0` is uncapped.
    /// Defaults to the p2pd relay's `-relayDataLimit`, not libp2p's 128 KiB.
    #[serde(default = "default_relay_max_circuit_bytes")]
    pub relay_max_circuit_bytes: u64,

    /// How long one relayed circuit may stay open.
    /// Defaults to the p2pd relay's `-relayTimeLimit`, not libp2p's 2 minutes.
    #[serde(default = "default_relay_max_circuit_duration")]
    pub relay_max_circuit_duration: Duration,

    /// Ask the local gateway to map our listen port via UPnP/IGD.
    ///
    /// On by default (parity with p2pd's `-natPortMap`), off in
    /// [`NetworkConfig::for_tests`] — SSDP is a LAN broadcast and CI should not
    /// be talking to a gateway.
    #[serde(default = "default_true")]
    pub enable_upnp: bool,

    /// Declare this node unreachable without waiting for AutoNAT to say so.
    ///
    /// Parity with p2pd's `-forceReachabilityPrivate`: relay reservations start
    /// immediately instead of after a probe round, and AutoNAT can never
    /// *promote* the node to public afterwards. That one-way property is the
    /// point — AutoNAT can read a NAT-PMP mapping as public reachability and
    /// thereby stop circuits from ever forming.
    #[serde(default)]
    pub force_private: bool,

    /// An externally-reachable address to declare unconditionally.
    ///
    /// Set from `public_ip`/`announce_addr`. Confirmed into the swarm at
    /// startup and pins reachability to Public — it outranks `force_private`
    /// (with a warning, since setting both is contradictory) and AutoNAT can
    /// never demote it. The operator knows their port forward exists; a failed
    /// dialback probe does not disprove it.
    #[serde(default)]
    pub external_addr: Option<String>,

    /// Require addresses to be globally routable, rejecting the IANA-reserved
    /// documentation and benchmarking ranges as well as the private ones.
    ///
    /// **Default false.** This drives `autonat::Config::only_global_ips`, whose
    /// own default (`true`) is the single place in rust-libp2p 0.53 that would
    /// classify the docker nat-test bed's `198.18/15` addresses unreachable.
    /// Turn it on for a node on the real internet, where a documentation-range
    /// address could only ever be a misconfiguration.
    #[serde(default)]
    pub require_global_ips: bool,

    /// How many circuit reservations to hold at once.
    ///
    /// Two: one relay is a single point of failure, and each extra reservation
    /// costs a held connection plus a keep-alive on the relay's side for a
    /// benefit that falls off fast.
    #[serde(default = "default_max_relay_reservations")]
    pub max_relay_reservations: usize,

    /// Distinct peers that must report the same observed address before the
    /// identify-consensus fallback will promote it.
    ///
    /// One peer's opinion is not evidence: it may be reporting a NAT mapping
    /// only it can use. Two independent observers of the same address is the
    /// weakest claim worth acting on.
    #[serde(default = "default_identify_min_confirmations")]
    pub identify_min_confirmations: usize,

    /// Kademlia protocol IDs, in outbound preference order.
    ///
    /// Defaults to `[KWAAI_KAD_PROTOCOL, LEGACY_KAD_PROTOCOL]`: kwaai↔kwaai
    /// pairs negotiate the kwaai name, while peers that predate it fall back
    /// to `/ipfs/kad/1.0.0`. A bootstrap-grade node on a public address should
    /// set this to `[KWAAI_KAD_PROTOCOL]` alone — serving the legacy name is
    /// what lets the public IPFS DHT absorb it (see [`LEGACY_KAD_PROTOCOL`]).
    ///
    /// An empty list means the default, not "no kad": disabling the DHT is
    /// `enable_dht`'s job, and treating `[]` as default keeps a config
    /// template that omits the key from silently changing the protocol set.
    ///
    /// **Carrying two names costs a round trip.** The `V1Lazy` substream
    /// override takes the 0-RTT shortcut only on the last protocol offered,
    /// so with two entries the preferred one negotiates eagerly: +1 RTT per
    /// kad substream against an upgraded peer, and +2 against a legacy-only
    /// peer, which refuses the kwaai name before the lazy fallback. kad opens
    /// a substream per request, so that is per DHT hop. A single-entry list —
    /// what a bootstrap wants anyway — is back on the 0-RTT path, which makes
    /// this a cost of the migration window rather than of the feature.
    #[serde(default = "default_kad_protocols")]
    pub kad_protocols: Vec<String>,
}

fn default_true() -> bool {
    true
}

fn default_max_relay_reservations() -> usize {
    2
}

fn default_identify_min_confirmations() -> usize {
    2
}

fn default_kad_protocols() -> Vec<String> {
    vec![
        KWAAI_KAD_PROTOCOL.to_string(),
        LEGACY_KAD_PROTOCOL.to_string(),
    ]
}

fn default_kad_stream_protocols() -> Vec<StreamProtocol> {
    default_kad_protocols()
        .into_iter()
        .map(|name| {
            StreamProtocol::try_from_owned(name).expect("default kad protocol ids are valid")
        })
        .collect()
}

/// Every entry of a non-empty `kad_protocols` failed to parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidKadProtocols {
    pub entries: Vec<String>,
}

impl std::fmt::Display for InvalidKadProtocols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "kad_protocols has no usable entry ({:?}); protocol ids must start with '/', \
             e.g. \"{KWAAI_KAD_PROTOCOL}\". Remove the key to get the default set.",
            self.entries,
        )
    }
}

impl std::error::Error for InvalidKadProtocols {}

/// 4 GiB, the p2pd relay's `-relayDataLimit`.
fn default_relay_max_circuit_bytes() -> u64 {
    1 << 32
}

/// 30 minutes, the p2pd relay's `-relayTimeLimit`.
fn default_relay_max_circuit_duration() -> Duration {
    Duration::from_secs(30 * 60)
}

/// 5 minutes, matching kad's own periodic bootstrap.
fn default_kad_maintenance_interval() -> Duration {
    Duration::from_secs(5 * 60)
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
            bootstrap_peers: Vec::new(),
            enable_dht: true,
            dht_replication: 20,
            // Long enough that a punched connection survives to be seen and
            // used. `max_connections` is what keeps this bounded.
            idle_connection_timeout: Duration::from_secs(10 * 60),
            request_timeout: Duration::from_secs(60),
            kad_maintenance_interval: default_kad_maintenance_interval(),
            max_connections: 100,
            enable_quic: false,
            enable_nat_traversal: true,
            enable_relay_client: true,
            protocol_version: crate::behaviour::DEFAULT_PROTOCOL_VERSION.to_string(),
            agent_version: crate::behaviour::default_agent_version(),
            port: 0,
            initial_peers: Vec::new(),
            dht_server: false,
            trusted_relays: Vec::new(),
            relay_server: true,
            relay_max_circuit_bytes: default_relay_max_circuit_bytes(),
            relay_max_circuit_duration: default_relay_max_circuit_duration(),
            enable_upnp: true,
            force_private: false,
            external_addr: None,
            require_global_ips: false,
            max_relay_reservations: default_max_relay_reservations(),
            identify_min_confirmations: default_identify_min_confirmations(),
            kad_protocols: default_kad_protocols(),
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
    ///
    /// The hop server and UPnP are off. UPnP because SSDP is a LAN broadcast
    /// that has no business firing from CI; the hop server because most tests
    /// do not need it and the relay tests that do set `relay_server: true`
    /// explicitly, which reads better than every other test silently running a
    /// relay it never uses.
    pub fn for_tests() -> Self {
        Self {
            listen_addrs: vec!["/ip4/127.0.0.1/tcp/0".to_string()],
            port: 0,
            dht_server: true,
            relay_server: false,
            enable_upnp: false,
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
        let mut addrs = vec![
            format!("/ip4/0.0.0.0/tcp/{}", self.port),
            format!("/ip6/::/tcp/{}", self.port),
        ];
        if self.enable_quic {
            // Same port number: a different protocol, so it does not collide.
            addrs.push(format!("/ip4/0.0.0.0/udp/{}/quic-v1", self.port));
            addrs.push(format!("/ip6/::/udp/{}/quic-v1", self.port));
        }
        addrs
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

    /// [`Self::kad_protocols`] as validated [`StreamProtocol`]s, preserving
    /// order.
    ///
    /// An *empty* list means the default set — that is the "key absent from
    /// the template" contract, and a kad with no protocols could talk to
    /// nobody. Invalid entries (no leading `/`) alongside at least one valid
    /// one are dropped with a warning.
    ///
    /// A non-empty list with *no* valid entry is an error, not a fallback.
    /// The fallback would restore `/ipfs/kad/1.0.0` on the one node type that
    /// set this key to be rid of it, so a single missing slash in
    /// `kad_protocols: ["kwaai/kad/1.0.0"]` would quietly re-expose a
    /// bootstrap to the public IPFS DHT with only a `warn!` nobody is tailing.
    /// Refusing to start is the safe direction here.
    pub fn kad_stream_protocols(&self) -> Result<Vec<StreamProtocol>, InvalidKadProtocols> {
        if self.kad_protocols.is_empty() {
            return Ok(default_kad_stream_protocols());
        }
        let parsed: Vec<StreamProtocol> = self
            .kad_protocols
            .iter()
            .filter_map(|name| match StreamProtocol::try_from_owned(name.clone()) {
                Ok(protocol) => Some(protocol),
                Err(err) => {
                    tracing::warn!(%name, "ignoring invalid kad protocol id: {err}");
                    None
                }
            })
            .collect();
        if parsed.is_empty() {
            Err(InvalidKadProtocols {
                entries: self.kad_protocols.clone(),
            })
        } else {
            Ok(parsed)
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
    pub fn idle_connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.idle_connection_timeout = timeout;
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

#[cfg(test)]
mod relay_circuit_limits {
    use super::*;

    /// p2pd's `-relayDataLimit` / `-relayTimeLimit`, pinned against libp2p's
    /// 128 KiB / 2 min.
    #[test]
    fn the_defaults_match_the_p2pd_relay() {
        let cfg = NetworkConfig::default();
        assert_eq!(cfg.relay_max_circuit_bytes, 1 << 32);
        assert_eq!(cfg.relay_max_circuit_duration, Duration::from_secs(30 * 60));
        assert!(
            cfg.relay_max_circuit_bytes > 1 << 17
                && cfg.relay_max_circuit_duration > Duration::from_secs(120),
            "the point of the override is to exceed libp2p's 128 KiB / 2 min",
        );
    }

    /// `behaviour.rs` clamps an operator's value; the default must already fit.
    #[test]
    fn the_default_circuit_duration_fits_the_wire_format() {
        let secs = NetworkConfig::default()
            .relay_max_circuit_duration
            .as_secs();
        assert!(
            secs <= u32::MAX as u64,
            "{secs}s exceeds the u32 wire field"
        );
    }

    #[test]
    fn the_limits_round_trip_and_default_when_absent() {
        let cfg = NetworkConfig {
            relay_max_circuit_bytes: 4096,
            relay_max_circuit_duration: Duration::from_secs(30),
            ..NetworkConfig::default()
        };
        let json = serde_json::to_string(&cfg).expect("serialize");
        let back: NetworkConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.relay_max_circuit_bytes, 4096);
        assert_eq!(back.relay_max_circuit_duration, Duration::from_secs(30));

        // A config written before these fields existed.
        let legacy = r#"{
            "listen_addrs": [], "bootstrap_peers": [], "enable_dht": true,
            "dht_replication": 20,
            "idle_connection_timeout": {"secs": 600, "nanos": 0},
            "request_timeout": {"secs": 60, "nanos": 0},
            "max_connections": 100, "enable_nat_traversal": true,
            "enable_relay_client": true, "protocol_version": "x",
            "agent_version": "y"
        }"#;
        let old: NetworkConfig = serde_json::from_str(legacy).expect("legacy config");
        assert_eq!(old.relay_max_circuit_bytes, 1 << 32);
        assert_eq!(
            old.relay_max_circuit_duration,
            Duration::from_secs(30 * 60),
            "an old config must pick up the new default, not libp2p's",
        );
        assert_eq!(
            old.kad_protocols,
            vec![KWAAI_KAD_PROTOCOL, LEGACY_KAD_PROTOCOL],
            "a config predating kad_protocols must get the dual default",
        );
    }
}

#[cfg(test)]
mod kad_protocols {
    use super::*;

    /// Order is the outbound preference: the kwaai name must come first so
    /// kwaai↔kwaai pairs stop negotiating the protocol the public IPFS DHT
    /// matches on.
    #[test]
    fn default_kad_protocols_prefer_kwaai_over_legacy() {
        let names: Vec<String> = NetworkConfig::default()
            .kad_stream_protocols()
            .expect("the default set is valid")
            .iter()
            .map(|p| p.to_string())
            .collect();
        assert_eq!(names, vec![KWAAI_KAD_PROTOCOL, LEGACY_KAD_PROTOCOL]);
    }

    /// `kad_protocols: []` in a config template means "the default", the same
    /// contract as an absent key — not a kad with no protocols.
    #[test]
    fn empty_kad_protocols_fall_back_to_the_default() {
        let cfg = NetworkConfig {
            kad_protocols: Vec::new(),
            ..NetworkConfig::default()
        };
        assert_eq!(
            cfg.kad_stream_protocols(),
            NetworkConfig::default().kad_stream_protocols()
        );
    }

    /// A typo'd entry alongside a usable one is dropped: the operator still
    /// gets the protocol set they asked for, minus the typo.
    #[test]
    fn invalid_kad_protocol_entries_are_dropped_when_one_survives() {
        let cfg = NetworkConfig {
            kad_protocols: vec![
                "no-leading-slash".to_string(),
                KWAAI_KAD_PROTOCOL.to_string(),
            ],
            ..NetworkConfig::default()
        };
        let names: Vec<String> = cfg
            .kad_stream_protocols()
            .expect("one entry parses")
            .iter()
            .map(|p| p.to_string())
            .collect();
        assert_eq!(names, vec![KWAAI_KAD_PROTOCOL]);
    }

    /// The reachable typo: `kwaai/kad/1.0.0` without the leading slash, on a
    /// bootstrap whose whole reason for setting the key is to stop serving
    /// the legacy name. Falling back to the default here would put
    /// `/ipfs/kad/1.0.0` back on the wire, which is the incident. It must
    /// fail instead.
    #[test]
    fn an_entirely_invalid_kad_protocol_list_is_fatal() {
        let all_bad = NetworkConfig {
            kad_protocols: vec!["kwaai/kad/1.0.0".to_string()],
            ..NetworkConfig::default()
        };
        let err = all_bad
            .kad_stream_protocols()
            .expect_err("no entry parses, so there is nothing to serve");
        assert_eq!(err.entries, vec!["kwaai/kad/1.0.0".to_string()]);
        assert!(
            !err.to_string().is_empty(),
            "the error names the offending entries"
        );
    }
}
