//! Configuration for P2P networking

use libp2p::StreamProtocol;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;
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

    /// Whether to open IPv6 listeners, and how hard to insist.
    ///
    /// `auto` binds `/ip6/::` and falls back to IPv4-only if the OS refuses;
    /// `true` makes that refusal a startup error; `false` opens no v6 listener
    /// and drops v6 addresses from the dial and announce sets.
    #[serde(default)]
    pub ipv6: Ipv6Mode,

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

    /// Kad protocol names this swarm serves, in outbound preference order.
    ///
    /// **Not a user-facing setting** — `#[serde(skip)]`, no `config.yaml`
    /// key, and production always takes the compiled default. It exists as a
    /// field only so tests can build swarms with differing protocol sets and
    /// exercise the migration topology in-process; there is no way for an
    /// operator to reach it. See [`kad_protocols()`] for why the real choice
    /// is a build flag rather than configuration.
    #[serde(skip, default = "default_kad_protocol_field")]
    pub kad_protocols: Vec<StreamProtocol>,

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

/// Default for [`NetworkConfig::kad_protocols`] — see [`kad_protocols()`].
fn default_kad_protocol_field() -> Vec<StreamProtocol> {
    kad_protocols()
}

/// Whether this build can serve more than one kad protocol name.
pub const KAD_MULTI_PROTOCOL_BUILD: bool = cfg!(feature = "kad-multi-protocol");

/// Whether this build has IPv6 support compiled in.
pub const IPV6_BUILD: bool = cfg!(feature = "ipv6");

/// How hard a node insists on IPv6.
///
/// Three states rather than a bool because "bind v6 if you can" and "v6 is
/// required" are different operational promises, and a host that silently ran
/// v4-only is exactly the failure a v6 deployment needs to see.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ipv6Mode {
    /// Bind `/ip6/::`; log once and carry on v4-only if the OS refuses.
    #[default]
    Auto,
    /// Bind `/ip6/::`; a refusal is a startup error.
    On,
    /// No v6 listeners, and v6 addresses are dropped from the dial and
    /// announce sets.
    Off,
}

impl Ipv6Mode {
    /// The mode this build can actually honour: always `Off` without the
    /// `ipv6` feature, so the config key stays loadable either way.
    pub fn effective(self) -> Self {
        if IPV6_BUILD {
            self
        } else {
            Self::Off
        }
    }

    /// Whether v6 is disabled once the build is taken into account.
    pub fn is_off(self) -> bool {
        self.effective() == Self::Off
    }
}

impl fmt::Display for Ipv6Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Auto => "auto",
            Self::On => "true",
            Self::Off => "false",
        })
    }
}

impl FromStr for Ipv6Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "true" | "on" | "yes" | "1" => Ok(Self::On),
            "false" | "off" | "no" | "0" => Ok(Self::Off),
            other => Err(format!("expected auto, true or false, got `{other}`")),
        }
    }
}

// Serialized as YAML's own `auto` / `true` / `false` rather than as an enum
// variant, so the key reads the way an operator writes it.
impl Serialize for Ipv6Mode {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => s.serialize_str("auto"),
            Self::On => s.serialize_bool(true),
            Self::Off => s.serialize_bool(false),
        }
    }
}

impl<'de> Deserialize<'de> for Ipv6Mode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Bool(bool),
            Str(String),
        }
        match Raw::deserialize(d)? {
            Raw::Bool(true) => Ok(Self::On),
            Raw::Bool(false) => Ok(Self::Off),
            Raw::Str(s) => s.parse().map_err(de::Error::custom),
        }
    }
}

/// What IPv6 actually ended up doing, once the listeners were opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv6Status {
    /// Disabled by config or by the build.
    Off,
    /// At least one v6 listener is bound.
    Active,
    /// Wanted, but the host refused every v6 bind.
    Unavailable,
}

impl Ipv6Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Active => "active",
            Self::Unavailable => "unavailable",
        }
    }
}

/// The kad protocol names this build serves, in outbound preference order.
///
/// Compiled in, deliberately not configurable. Which names a node serves
/// decides whether the public IPFS DHT can absorb it, and every attempt to
/// express that as configuration turned out to be a way of getting it
/// silently wrong: entries droppable one at a time, an invalid list falling
/// back to the default that restored the very name the operator was removing,
/// and validation split across two crates that could disagree with each
/// other. A build flag has none of those failure modes — a binary either can
/// serve both names or cannot — and it logs which at startup.
///
/// `kad-multi-protocol` is the bootstrap-grade build for the migration
/// window: it also answers the legacy `/ipfs/kad/1.0.0`, so peers predating
/// the kwaai name are not cut off while the fleet upgrades. Serving that name
/// on a public address is what lets the global IPFS DHT absorb a node, so it
/// is a deliberate, temporary trade on the one host that has to bridge.
/// Retire the feature, and the patched libp2p-kad with it, once the fleet has
/// moved.
///
/// **Two names cost a round trip.** `V1Lazy` takes the 0-RTT shortcut only on
/// the last protocol offered, so the preferred name negotiates eagerly: +1
/// RTT per kad substream against an upgraded peer, +2 against a legacy-only
/// one, which refuses the kwaai name before falling back. kad opens a
/// substream per request, so that is per DHT hop — another reason this
/// belongs to the migration window rather than to steady state.
pub fn kad_protocols() -> Vec<StreamProtocol> {
    let names: &[&str] = if KAD_MULTI_PROTOCOL_BUILD {
        &[KWAAI_KAD_PROTOCOL, LEGACY_KAD_PROTOCOL]
    } else {
        &[KWAAI_KAD_PROTOCOL]
    };
    names
        .iter()
        .map(|n| {
            StreamProtocol::try_from_owned((*n).to_string())
                .expect("compiled kad protocol ids are valid")
        })
        .collect()
}

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
            ipv6: Ipv6Mode::Auto,
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
            kad_protocols: default_kad_protocol_field(),
            enable_upnp: true,
            force_private: false,
            external_addr: None,
            require_global_ips: false,
            max_relay_reservations: default_max_relay_reservations(),
            identify_min_confirmations: default_identify_min_confirmations(),
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
            ipv6: Ipv6Mode::Auto,
            dht_server: true,
            relay_server: false,
            enable_upnp: false,
            ..Self::default()
        }
    }

    /// Multiaddrs the swarm should listen on.
    ///
    /// If `listen_addrs` was set explicitly it wins; otherwise the IPv4
    /// wildcard on [`NetworkConfig::port`] is used, plus the IPv6 one unless
    /// [`NetworkConfig::ipv6`] is off. Both transports listen on the same port
    /// number: libp2p sets `IPV6_V6ONLY` on the v6 socket, so they do not
    /// collide.
    pub fn swarm_listen_addrs(&self) -> Vec<String> {
        if !self.listen_addrs.is_empty() {
            return self.listen_addrs.clone();
        }
        let ipv6 = !self.ipv6.is_off();
        let mut addrs = vec![format!("/ip4/0.0.0.0/tcp/{}", self.port)];
        if ipv6 {
            addrs.push(format!("/ip6/::/tcp/{}", self.port));
        }
        if self.enable_quic {
            // Same port number: a different protocol, so it does not collide.
            addrs.push(format!("/ip4/0.0.0.0/udp/{}/quic-v1", self.port));
            if ipv6 {
                addrs.push(format!("/ip6/::/udp/{}/quic-v1", self.port));
            }
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

    /// Set the IPv6 mode
    pub fn ipv6(mut self, mode: Ipv6Mode) -> Self {
        self.config.ipv6 = mode;
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
    }
}

#[cfg(test)]
mod kad_protocols {
    use super::*;

    /// The kwaai name is always preferred, and is the only one an ordinary
    /// build serves. A node serving `/ipfs/kad/1.0.0` on a public address is
    /// one the global IPFS DHT can absorb, which is the whole point.
    #[test]
    fn the_build_decides_the_protocol_set() {
        let names: Vec<String> = kad_protocols().iter().map(|p| p.to_string()).collect();
        assert_eq!(names[0], KWAAI_KAD_PROTOCOL, "kwaai is always preferred");
        if KAD_MULTI_PROTOCOL_BUILD {
            assert_eq!(names, vec![KWAAI_KAD_PROTOCOL, LEGACY_KAD_PROTOCOL]);
        } else {
            assert_eq!(
                names,
                vec![KWAAI_KAD_PROTOCOL],
                "an ordinary build must not serve the legacy name"
            );
        }
    }
}

#[cfg(test)]
mod ipv6_mode {
    use super::*;

    fn parse(yaml: &str) -> Ipv6Mode {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default)]
            ipv6: Ipv6Mode,
        }
        serde_yaml::from_str::<Wrapper>(yaml)
            .expect("valid ipv6 key")
            .ipv6
    }

    /// `auto` is a string and `true`/`false` are booleans in the same key, so
    /// the deserializer has to accept both shapes.
    #[test]
    fn the_key_accepts_auto_and_the_booleans() {
        assert_eq!(parse("ipv6: auto"), Ipv6Mode::Auto);
        assert_eq!(parse("ipv6: true"), Ipv6Mode::On);
        assert_eq!(parse("ipv6: false"), Ipv6Mode::Off);
        assert_eq!(parse("other: 1"), Ipv6Mode::Auto, "absent means auto");
    }

    #[test]
    fn a_bogus_value_is_an_error() {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(default)]
            #[allow(dead_code)]
            ipv6: Ipv6Mode,
        }
        assert!(serde_yaml::from_str::<Wrapper>("ipv6: bogus").is_err());
    }

    /// Round-tripping must not turn `true` into the string `"true"`, which
    /// would still parse but no longer look like the boolean an operator wrote.
    #[test]
    fn on_serializes_as_a_yaml_boolean() {
        assert_eq!(serde_yaml::to_string(&Ipv6Mode::On).unwrap().trim(), "true");
        assert_eq!(
            serde_yaml::to_string(&Ipv6Mode::Off).unwrap().trim(),
            "false"
        );
        assert_eq!(
            serde_yaml::to_string(&Ipv6Mode::Auto).unwrap().trim(),
            "auto"
        );
    }

    #[test]
    fn strings_parse_case_insensitively() {
        for (s, want) in [
            ("AUTO", Ipv6Mode::Auto),
            ("On", Ipv6Mode::On),
            ("yes", Ipv6Mode::On),
            ("1", Ipv6Mode::On),
            ("NO", Ipv6Mode::Off),
            ("0", Ipv6Mode::Off),
        ] {
            assert_eq!(s.parse::<Ipv6Mode>().unwrap(), want, "{s}");
        }
        assert!("maybe".parse::<Ipv6Mode>().is_err());
    }

    /// Without the feature the key still loads; it just cannot mean anything.
    #[test]
    fn the_build_has_the_last_word() {
        assert_eq!(Ipv6Mode::On.effective() == Ipv6Mode::On, IPV6_BUILD);
        assert_eq!(Ipv6Mode::Auto.is_off(), !IPV6_BUILD);
        assert!(Ipv6Mode::Off.is_off());
    }

    /// A config written before the key existed must default to `auto`.
    #[test]
    fn a_legacy_config_defaults_to_auto() {
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
        assert_eq!(old.ipv6, Ipv6Mode::Auto);
    }
}
