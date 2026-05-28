//! Test harness for spinning up one or more p2pd daemon instances.
//!
//! Each `TestNode` gets its own tmpdir (unique Unix socket path) so tests can
//! run in parallel without conflicts. The daemon process is killed when the
//! `TestNode` drops.
//!
//! # Topologies
//!
//! - `TestNode::new_relay_server()` — dht_server + relay, listens on a known
//!   TCP port so other nodes can bootstrap from it.
//! - `TestNode::new_dht_client(bootstrap)` — dht, auto_relay, bootstraps from
//!   the provided multiaddr.
//! - `TestNode::new_nat_client(bootstrap, relay_addr)` — force_reachability_private
//!   + auto_relay, bootstraps from `bootstrap`, uses `relay_addr` as trusted relay.
//!   This forces the node to obtain a /p2p-circuit address, giving us a relay path
//!   to test against.
//! - `RelayHarness::new()` — relay + two clients (one "NAT", one direct). Used by
//!   the relay connectivity tests.

use anyhow::{Context, Result};
use kwaai_p2p_daemon::{DaemonBuilder, P2PClient, P2PDaemon};
use std::net::TcpListener;
use tempfile::TempDir;

/// Find a free TCP port on localhost.
pub fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind for free port")
        .local_addr()
        .unwrap()
        .port()
}

/// A single test p2pd instance with its own socket and tmpdir.
pub struct TestNode {
    pub daemon: P2PDaemon,
    pub client: P2PClient,
    /// Peer ID (hex-encoded bytes, same format as hex::encode(peer_id.to_bytes()))
    pub peer_id_hex: String,
    /// IPC socket multiaddr used by the daemon, e.g. `/unix//tmp/.../p2pd.sock`
    pub socket_addr: String,
    /// TCP P2P listen port (if a specific port was requested via host_addrs)
    pub p2p_port: Option<u16>,
    // Keeps the tempdir alive for the lifetime of the node.
    _tmpdir: TempDir,
}

impl TestNode {
    /// Start a relay + DHT server node on a deterministic TCP port.
    ///
    /// Other nodes bootstrap from `/ip4/127.0.0.1/tcp/{port}/p2p/{peer_id}`.
    pub async fn new_relay_server() -> Result<Self> {
        let p2p_port = free_port();
        let tmpdir = TempDir::new()?;
        let socket_path = tmpdir.path().join("p2pd.sock");
        let socket_addr = format!("/unix/{}", socket_path.display());

        let mut daemon = DaemonBuilder::new()
            .with_listen_addr(&socket_addr)
            .dht_server(true)
            .relay(true)
            .bootstrap(false) // no external bootstrap for local test
            .host_addrs([format!("/ip4/127.0.0.1/tcp/{p2p_port}")])
            .spawn()
            .await
            .context("spawn relay server")?;

        let mut client = daemon.client().await.context("relay client")?;
        let peer_id_hex = client.identify().await.context("relay identify")?;

        Ok(Self {
            daemon,
            client,
            peer_id_hex,
            socket_addr,
            p2p_port: Some(p2p_port),
            _tmpdir: tmpdir,
        })
    }

    /// Bootstrap multiaddr for this node (only valid when p2p_port is set).
    ///
    /// Format: `/ip4/127.0.0.1/tcp/{port}/p2p/{peer_id_multihash}`
    /// NOTE: peer_id_hex here is raw bytes hex; for the multiaddr /p2p/ component
    /// we need the base58-encoded multihash form. We keep it hex internally and
    /// convert at use-site via libp2p::PeerId.
    pub fn bootstrap_multiaddr(&self) -> Option<String> {
        let port = self.p2p_port?;
        // peer_id_hex is hex(peer_id.to_bytes()); decode → PeerId → Display (base58)
        let peer_id = peer_id_from_hex(&self.peer_id_hex)?;
        Some(format!("/ip4/127.0.0.1/tcp/{port}/p2p/{peer_id}"))
    }

    /// Start a standard DHT client node, bootstrapping from `bootstrap_addr`.
    pub async fn new_dht_client(bootstrap_addr: &str) -> Result<Self> {
        let tmpdir = TempDir::new()?;
        let socket_path = tmpdir.path().join("p2pd.sock");
        let socket_addr = format!("/unix/{}", socket_path.display());

        let mut daemon = DaemonBuilder::new()
            .with_listen_addr(&socket_addr)
            .dht(true)
            .bootstrap(true)
            .bootstrap_peer(bootstrap_addr)
            .spawn()
            .await
            .context("spawn dht client")?;

        let mut client = daemon.client().await.context("dht client connect")?;
        let peer_id_hex = client.identify().await.context("dht client identify")?;

        Ok(Self {
            daemon,
            client,
            peer_id_hex,
            socket_addr,
            p2p_port: None,
            _tmpdir: tmpdir,
        })
    }

    /// Start a "NATed" node: force_reachability_private + auto_relay.
    ///
    /// Bootstraps from `bootstrap_addr` and names `relay_addr` as a trusted relay.
    /// p2pd will skip AutoNAT probing and immediately try to obtain a circuit relay
    /// address via the trusted relay, making the relay path available quickly in tests.
    pub async fn new_nat_client(bootstrap_addr: &str, relay_addr: &str) -> Result<Self> {
        let tmpdir = TempDir::new()?;
        let socket_path = tmpdir.path().join("p2pd.sock");
        let socket_addr = format!("/unix/{}", socket_path.display());

        let mut daemon = DaemonBuilder::new()
            .with_listen_addr(&socket_addr)
            .dht(true)
            .auto_relay(true)
            .auto_nat(true)
            .force_reachability_private(true)
            .trusted_relays([relay_addr])
            .bootstrap(true)
            .bootstrap_peer(bootstrap_addr)
            .spawn()
            .await
            .context("spawn nat client")?;

        let mut client = daemon.client().await.context("nat client connect")?;
        let peer_id_hex = client.identify().await.context("nat client identify")?;

        Ok(Self {
            daemon,
            client,
            peer_id_hex,
            socket_addr,
            p2p_port: None,
            _tmpdir: tmpdir,
        })
    }

    /// Return the observed multiaddrs this node has been told about.
    pub async fn observed_addrs(&mut self) -> Result<Vec<Vec<u8>>> {
        let (_, addrs) = self
            .client
            .identify_with_addrs()
            .await
            .context("identify_with_addrs")?;
        Ok(addrs)
    }

    /// True when at least one of the observed multiaddrs contains the p2p-circuit protocol.
    pub async fn has_relay_addr(&mut self) -> Result<bool> {
        let addrs = self.observed_addrs().await?;
        for addr_bytes in &addrs {
            if let Ok(s) = std::str::from_utf8(addr_bytes) {
                if s.contains("p2p-circuit") || s.contains("circuit") {
                    return Ok(true);
                }
            }
            // Also try parsing as libp2p Multiaddr
            if let Ok(ma) = libp2p::Multiaddr::try_from(addr_bytes.clone()) {
                let s = ma.to_string();
                if s.contains("p2p-circuit") {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

/// Three-node topology used for relay tests:
/// - `relay`: DHT server + relay server, known port
/// - `nat_node`: behind NAT, must use relay to be reachable
/// - `observer`: standard DHT client, will connect to nat_node via relay
pub struct RelayHarness {
    pub relay: TestNode,
    pub nat_node: TestNode,
    pub observer: TestNode,
}

impl RelayHarness {
    /// Spin up the three-node topology.
    ///
    /// Startup order: relay first (so bootstrap addr is known), then nat_node
    /// and observer in parallel.
    pub async fn new() -> Result<Self> {
        let relay = TestNode::new_relay_server()
            .await
            .context("relay node startup")?;

        let bootstrap = relay
            .bootstrap_multiaddr()
            .context("relay has no p2p_port")?;

        let relay_multiaddr = bootstrap.clone();

        let (nat_node, observer) = tokio::try_join!(
            TestNode::new_nat_client(&bootstrap, &relay_multiaddr),
            TestNode::new_dht_client(&bootstrap),
        )?;

        Ok(Self {
            relay,
            nat_node,
            observer,
        })
    }
}

/// Decode a hex peer_id back to a libp2p PeerId (for multiaddr construction).
pub fn peer_id_from_hex(hex_str: &str) -> Option<libp2p::PeerId> {
    let bytes = hex::decode(hex_str).ok()?;
    libp2p::PeerId::from_bytes(&bytes).ok()
}
