//! `kwaainet bootstrap serve` — the native DHT seed.
//!
//! Replaces the Python bootstrap (`python -m petals.cli.run_dht`) with an
//! in-process rust-libp2p swarm. A seed is a *stripped* node: it serves the
//! hivemind DHT for everyone else and announces nothing of its own.
//!
//! ```text
//!   ┌──────────── kwaainet bootstrap serve ─────────────┐
//!   │  identity (RSA-2048 or Ed25519, from --identity-key)
//!   │      │                                            │
//!   │      ▼                                            │
//!   │  NetworkService ──▶ swarm ─── kad server mode ────│──▶ /ipfs/kad/1.0.0
//!   │      │                    └── relay hop server ───│──▶ NATed peers reserve
//!   │      ▼                    └── identify ───────────│──▶ announces --announce
//!   │  DHTStorage ◀── rpc_ping / rpc_store / rpc_find   │
//!   └───────────────────────────────────────────────────┘
//! ```
//!
//! # What a seed deliberately does *not* have
//!
//! Everything `node_native::NativeNode::start` adds on top of the swarm and the
//! DHT service: no announce records (a seed stores other peers' records and
//! publishes none of its own — matching `run_dht`, which has no `--announce`
//! anything), no control socket, no health-monitor client, no auto-update, no
//! VPK polling, no shard manager, no Ollama proxy. A seed that announced itself
//! would appear on the map as an inference node offering zero blocks.
//!
//! # Storage is in-memory on purpose
//!
//! `DHTStorage` keeps records in memory and `run_dht` did the same. A restarted
//! seed comes back empty and refills within one re-announce round (nodes
//! re-announce every 300 s against a 360 s TTL), so persistence would buy a
//! window of staleness rather than availability — the restarted seed would
//! serve records for nodes that died while it was down.
//!
//! # Interop
//!
//! Callers are both native Rust nodes and Go-p2pd nodes; the protocol IDs and
//! the `DHTStorage` behind them are identical on both paths, so a p2pd caller
//! cannot tell this from the Python seed it replaces.

use anyhow::{Context, Result};
use kwaai_hivemind_dht::DHTStorage;
use kwaai_p2p::{NetworkConfig, NetworkHandle, NetworkService};
use libp2p::{Multiaddr, PeerId};
use std::path::Path;
use std::time::Duration;
use tracing::{info, warn};

use crate::cli::{BootstrapAction, BootstrapArgs, BootstrapServeArgs};

/// Per-request budget on unary calls, matching the native node path.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// The default listen multiaddr — the port every existing bootstrap uses.
pub const DEFAULT_LISTEN: &str = "/ip4/0.0.0.0/tcp/8000";

pub async fn run_bootstrap_command(args: BootstrapArgs) -> Result<()> {
    match args.action {
        BootstrapAction::Serve(serve) => serve_bootstrap(serve).await,
    }
}

/// A running DHT seed: the swarm handle, its identity, and the storage it
/// serves. Kept together so shutdown is one call.
pub struct BootstrapSeed {
    pub handle: NetworkHandle,
    pub peer_id: PeerId,
    /// The records this seed serves on behalf of other peers. Never populated
    /// by the seed itself.
    ///
    /// Held rather than dropped: `spawn_dht_service` owns a clone, and keeping
    /// one here lets a caller (and the tests) inspect what the seed is serving.
    #[allow(dead_code)]
    pub storage: DHTStorage,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl BootstrapSeed {
    /// Build the swarm from a *required* identity key and register the three
    /// hivemind DHT handlers on it.
    ///
    /// # Identity
    ///
    /// The key file is loaded, never generated: a seed whose peer ID changed on
    /// restart would invalidate every `/p2p/<id>` multiaddr baked into the
    /// nodes' configs. `load_keypair` goes through libp2p's protobuf encoding,
    /// so the RSA-2048 keys minted in the Python era decode as-is and reproduce
    /// their original `Qm…` peer IDs.
    pub async fn start(
        key_path: &Path,
        listen: &str,
        announce: &[String],
        relay_server: bool,
    ) -> Result<Self> {
        let keypair = kwaai_p2p::identity::load_keypair(key_path).with_context(|| {
            format!(
                "loading the bootstrap identity from {} — a seed must have a stable identity, \
                 so this file is never generated for you",
                key_path.display()
            )
        })?;
        let peer_id = keypair.public().to_peer_id();

        // Validate every announce address up front. A malformed one is an
        // operator error worth failing on: the whole point of the flag is that
        // peers learn a reachable address, and a silently dropped one leaves
        // the seed advertising only its container-internal address.
        let announce_addrs = parse_announce_addrs(announce)?;

        let net_config = NetworkConfig {
            listen_addrs: vec![listen.to_string()],
            request_timeout: REQUEST_TIMEOUT,
            // Server mode unconditionally: a seed exists to answer Kademlia
            // queries, and auto-detection would leave it in client mode until
            // an external address is confirmed.
            dht_server: true,
            // The hop server NATed nodes make circuit reservations through.
            // This is what `node-a`/`node-relay` in the nat-test topology need
            // from a bootstrap.
            relay_server,
            // A seed is deployed at a known address; it never needs a gateway
            // mapping, and SSDP has no business firing from a datacentre.
            enable_upnp: false,
            force_private: false,
            // What identify reports to peers — the `ANNOUNCE_MADDRS` analogue.
            // Declared, so it outranks AutoNAT and is confirmed at t=0 rather
            // than after a probe round.
            external_addr: announce_addrs.first().map(|a| a.to_string()),
            // The nat-test topology addresses the simulated internet as
            // `198.18/15`, which is IANA-reserved for benchmarking; requiring
            // globally-routable addresses would classify the entire test
            // network unreachable.
            require_global_ips: false,
            // A seed dials nobody: peers come to it.
            bootstrap_peers: Vec::new(),
            initial_peers: Vec::new(),
            ..NetworkConfig::default()
        };

        let (handle, swarm_task) =
            NetworkService::spawn(net_config, keypair).context("starting the libp2p swarm")?;
        let mut tasks = vec![swarm_task];

        // ── DHT serving — the entire job of a seed ─────────────────────────
        // Registers rpc_ping / rpc_store / rpc_find as unary handlers backed by
        // `DHTStorage`, which owns the record semantics (subkeyed dictionary
        // stores, the tombstone rule where a strictly-greater expiration
        // replaces and a store never deletes, and TTL expiry). Going through
        // `DHTStorage` rather than reimplementing is what keeps a native seed
        // and the Python one indistinguishable to a caller.
        let storage = DHTStorage::new(peer_id);
        tasks.push(
            kwaai_p2p::spawn_dht_service(handle.clone(), storage.clone())
                .await
                .context("registering the hivemind DHT service")?,
        );

        Ok(Self {
            handle,
            peer_id,
            storage,
            tasks,
        })
    }

    /// Stop the swarm and every task hanging off it.
    pub async fn shutdown(self) {
        if let Err(e) = self.handle.shutdown().await {
            warn!("Network service shutdown: {e}");
        }
        for task in self.tasks {
            task.abort();
        }
    }
}

/// Run the seed in the foreground until SIGTERM/SIGINT.
///
/// Foreground is the contract: this is a container entrypoint, so the process
/// *is* the service — no daemonising, no PID file, and logs to stdout/stderr
/// where `docker logs` finds them.
async fn serve_bootstrap(args: BootstrapServeArgs) -> Result<()> {
    let seed = BootstrapSeed::start(
        &args.identity_key,
        &args.listen,
        &args.announce,
        !args.no_relay,
    )
    .await?;

    // Prominent, and first: an operator comparing this against the Python seed
    // checks the peer ID before anything else, and the nodes' configs pin it.
    info!("Bootstrap identity: {}", seed.peer_id.to_base58());
    info!("Listening on: {}", args.listen);
    for addr in &args.announce {
        info!("Announcing: {addr}/p2p/{}", seed.peer_id.to_base58());
    }
    if args.announce.is_empty() {
        warn!(
            "No --announce address given — peers will only learn the addresses this seed \
             observes for itself, which inside Docker is a container-internal address"
        );
    }
    if args.announce.len() > 1 {
        warn!(
            "Only the first --announce address is declared as the external address; \
             the remaining {} will be learned from identify if they are reachable",
            args.announce.len() - 1
        );
    }
    log_listen_addrs(&seed.handle, seed.peer_id).await;
    info!("Hivemind DHT service ready (rpc_ping/rpc_store/rpc_find)");
    info!("✅ KwaaiNet bootstrap seed running — serving the DHT, announcing nothing");

    crate::node::shutdown_signal().await;

    info!("Shutting down the bootstrap seed");
    seed.shutdown().await;
    info!("KwaaiNet bootstrap seed stopped");
    Ok(())
}

/// Parse and validate the announce multiaddrs.
fn parse_announce_addrs(announce: &[String]) -> Result<Vec<Multiaddr>> {
    announce
        .iter()
        .map(|a| {
            a.parse::<Multiaddr>()
                .with_context(|| format!("parsing --announce {a}"))
        })
        .collect()
}

/// Log the addresses the swarm actually bound, so an operator can see the seed
/// is reachable without attaching a debugger.
///
/// Bound addresses arrive asynchronously from the swarm, so this polls briefly
/// rather than reading once and reporting an empty list.
async fn log_listen_addrs(handle: &NetworkHandle, peer_id: PeerId) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        match handle.listen_addrs().await {
            Ok(addrs) if !addrs.is_empty() => {
                for addr in addrs {
                    info!("Bound: {addr}/p2p/{}", peer_id.to_base58());
                }
                return;
            }
            Ok(_) => {}
            Err(e) => {
                warn!("Could not read the swarm's listen addresses: {e}");
                return;
            }
        }
        if tokio::time::Instant::now() >= deadline {
            warn!("The swarm reported no listen addresses within 5s");
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    /// A test-only RSA-2048 key in libp2p protobuf encoding, and the peer ID it
    /// must produce.
    ///
    /// Generated once with `openssl genrsa 2048 | openssl rsa -outform DER`,
    /// wrapped as `0x08 0x00` (KeyType=RSA) + `0x12` + varint(len) + PKCS#1
    /// DER. This is *not* a production key — it exists so the RSA decode path
    /// the real `bootstrap_keyN.bin` files depend on is covered by a test that
    /// carries no secret.
    const RSA_KEY_FIXTURE: &[u8] = include_bytes!("../tests/fixtures/test_rsa_bootstrap_key.bin");

    /// The peer ID `RSA_KEY_FIXTURE` produces. RSA keys hash to `Qm…` (the
    /// public key exceeds 42 bytes, so it is not inlined into the multihash) —
    /// the same shape as the production bootstrap identities.
    const RSA_KEY_FIXTURE_PEER_ID: &str = "QmVfKgg4WGThYinMEramvptGJdaaXVHVhfmzwkxLmnBNAb";

    #[test]
    fn an_rsa_protobuf_key_loads_and_yields_its_peer_id() {
        let keypair = Keypair::from_protobuf_encoding(RSA_KEY_FIXTURE)
            .expect("the RSA fixture must decode under libp2p-identity");
        assert_eq!(
            keypair.public().to_peer_id().to_base58(),
            RSA_KEY_FIXTURE_PEER_ID,
            "the peer ID derived from an RSA key must be stable — the production \
             bootstrap multiaddrs pin it"
        );
    }

    /// The seed loads the same key from a file, which is how it is actually
    /// deployed (`--identity-key /config/bootstrap_key1.bin`).
    #[test]
    fn the_seed_loads_an_rsa_key_from_a_file() {
        let dir = tempfile::tempdir().expect("tmpdir");
        let path = dir.path().join("bootstrap_key1.bin");
        std::fs::write(&path, RSA_KEY_FIXTURE).expect("write the fixture");

        let keypair = kwaai_p2p::identity::load_keypair(&path)
            .expect("the seed's loader must accept an RSA key file");
        assert_eq!(
            keypair.public().to_peer_id().to_base58(),
            RSA_KEY_FIXTURE_PEER_ID
        );
    }

    /// A missing key file fails loudly rather than minting a fresh identity —
    /// the property that stops a typo'd path from silently orphaning every
    /// `/p2p/<id>` multiaddr in the nodes' configs.
    #[tokio::test]
    async fn a_missing_identity_key_is_an_error_not_a_new_identity() {
        let dir = tempfile::tempdir().expect("tmpdir");
        let missing = dir.path().join("does-not-exist.bin");

        // Matched rather than `expect_err`, which would need `Debug` on a
        // struct holding live swarm task handles.
        let err = match BootstrapSeed::start(&missing, "/ip4/127.0.0.1/tcp/0", &[], false).await {
            Ok(_) => panic!("a missing identity key must be an error, not a fresh identity"),
            Err(e) => e,
        };
        assert!(
            format!("{err:#}").contains("never generated for you"),
            "the error must explain why it did not generate a key: {err:#}"
        );
        assert!(
            !missing.exists(),
            "a failed load must not leave a generated key behind"
        );
    }

    #[test]
    fn a_malformed_announce_address_is_rejected() {
        let err = parse_announce_addrs(&["not-a-multiaddr".to_string()])
            .expect_err("a malformed announce address must be rejected");
        assert!(format!("{err:#}").contains("not-a-multiaddr"));
    }

    #[test]
    fn announce_addresses_parse() {
        let addrs = parse_announce_addrs(&[
            "/dns4/bootstrap1/tcp/8000".to_string(),
            "/ip4/198.18.0.10/tcp/8000".to_string(),
        ])
        .expect("well-formed announce addresses must parse");
        assert_eq!(addrs.len(), 2);
    }
}
