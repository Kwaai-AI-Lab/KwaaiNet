//! `kwaainet bootstrap serve` — the native DHT seed.
//!
//! Replaces the Python bootstrap (`python -m petals.cli.run_dht`) with an
//! in-process rust-libp2p swarm. A seed is a *configuration* of the ordinary
//! node, not a second implementation of one: this module maps four flags onto a
//! [`KwaaiNetConfig`] preset and hands it to the same runner `kwaainet start`
//! and the internal `run-node` use.
//!
//! ```text
//!   bootstrap serve --identity-key … --listen … --announce … [--no-relay]
//!            │
//!            ▼
//!   seed_config()  ── a KwaaiNetConfig with announce_self=false, dht_server=true
//!            │
//!            ▼
//!   node::run_node()  ── the SAME entry `Command::RunNode` calls
//!            │
//!            ▼
//!   node_native::run_native_node()  ── swarm, DHT service, control socket
//! ```
//!
//! # Why a preset rather than its own assembly
//!
//! An earlier version of this file built its own `NetworkService::spawn` +
//! `spawn_dht_service` pair — a second copy of what `node_native` assembles.
//! Two copies of swarm assembly drift: a fix to the node's dial behaviour, a
//! new handler, or a change in how reachability is decided would land on one
//! path and quietly miss the other, and the seed is the path with no operator
//! watching. Routing through the node runner means a seed is exactly a node
//! with a particular config, and `kwaainet start` with the same keys in
//! `config.yaml` behaves identically.
//!
//! # What the preset turns off, and what it deliberately leaves on
//!
//! Off: `announce_self` (the seed publishes no records of its own — no blocks,
//! no `_petals.models`, no `_kwaai.inference.nodes`, no VPK registry, and no
//! tombstone at shutdown, because nothing was announced), UPnP (a seed is
//! deployed at a known address; SSDP has no business firing from a datacentre),
//! health monitoring, VPK, Ollama supervision, and the bootstrap peer list (a
//! seed dials nobody — peers come to it).
//!
//! On: `dht_server`, unconditionally. A seed exists to answer Kademlia queries,
//! and auto-detection would leave it in client mode until an external address
//! was confirmed — refusing the very lookups it was deployed to serve. The
//! relay hop server stays on too unless `--no-relay`: NATed nodes reserve
//! circuits through the bootstraps.
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
use libp2p::Multiaddr;
use std::path::Path;
use tracing::{info, warn};

use crate::cli::{BootstrapAction, BootstrapArgs, BootstrapServeArgs};
use crate::config::KwaaiNetConfig;

/// The default listen multiaddr — the port every existing bootstrap uses.
pub const DEFAULT_LISTEN: &str = "/ip4/0.0.0.0/tcp/8000";

pub async fn run_bootstrap_command(args: BootstrapArgs) -> Result<()> {
    match args.action {
        BootstrapAction::Serve(serve) => serve_bootstrap(serve).await,
    }
}

/// Build the seed preset: an ordinary node config with everything a seed does
/// not do turned off.
///
/// Every field set here is load-bearing, so each is asserted by
/// `the_preset_is_exactly_the_documented_config`. The rest of the struct is
/// `KwaaiNetConfig::default()` — notably `model` and `blocks`, which keep their
/// defaults and are *never read* on this path: `run_native_node` only consults
/// them to build an announce record, and `announce_self = false` means it never
/// builds one. Nothing on the node runner downloads a model or spawns a shard;
/// that is `shard serve`'s job and the seed never invokes it.
fn seed_config(
    key_path: &Path,
    listen: &str,
    announce: &[String],
    relay_server: bool,
) -> Result<KwaaiNetConfig> {
    // Validate every announce address up front. A malformed one is an operator
    // error worth failing on: the whole point of the flag is that peers learn a
    // reachable address, and a silently dropped one leaves the seed advertising
    // only its container-internal address.
    let announce_addrs = parse_announce_addrs(announce)?;

    let mut config = KwaaiNetConfig {
        // The native stack — the whole point of the rework.
        native_p2p: true,
        // A seed serves the DHT and joins nothing.
        announce_self: false,
        // Server mode from t=0, before any external address is confirmed.
        dht_server: true,
        // Deployed at a known address; no gateway mapping to negotiate.
        enable_upnp: false,
        // The hop server NATed nodes make circuit reservations through. This is
        // what `node-a`/`node-relay` in the nat-test topology need from a
        // bootstrap.
        no_relay: !relay_server,
        // A seed dials nobody. `run_native_node` tolerates an empty list: it
        // logs that a seed dials nobody and never calls `handle.bootstrap`, so
        // there is no error and no retry loop.
        initial_peers: Vec::new(),
        // Nothing to monitor, poll, or supervise.
        vpk_enabled: false,
        ollama_manage: false,
        // The key file is loaded, never generated: a seed whose peer ID changed
        // on restart would invalidate every `/p2p/<id>` multiaddr baked into
        // the nodes' configs. Setting `identity_key` (rather than leaving it
        // None) is what selects the load-don't-generate path in `run_node`.
        identity_key: Some(key_path.to_path_buf()),
        ..KwaaiNetConfig::default()
    };
    config.health_monitoring.enabled = false;

    // `--listen` is a full multiaddr while the config carries a port, so the
    // port is lifted out of it. The listen address `node_native` builds is
    // `/ip4/0.0.0.0/tcp/<port>`, which is what every deployed seed passes
    // anyway — including the entrypoint's `/ip4/0.0.0.0/tcp/8000` default.
    config.port = listen_port(listen)?;

    // What identify reports to peers — the `ANNOUNCE_MADDRS` analogue.
    // Declared, so it outranks AutoNAT and is confirmed at t=0 rather than
    // after a probe round.
    config.announce_addr = announce_addrs.first().map(|a| a.to_string());

    Ok(config)
}

/// Run the seed in the foreground until SIGTERM/SIGINT.
///
/// Foreground is the contract: this is a container entrypoint, so the process
/// *is* the service — no daemonising, no PID file dance, and logs to
/// stdout/stderr where `docker logs` finds them. [`crate::node::run_node`] is
/// already foreground (it is what the hidden `run-node` subcommand calls, with
/// `kwaainet start --daemon` being the thing that forks *around* it), so
/// SIGTERM reaches the same shutdown path a node uses.
async fn serve_bootstrap(args: BootstrapServeArgs) -> Result<()> {
    let config = seed_config(
        &args.identity_key,
        &args.listen,
        &args.announce,
        !args.no_relay,
    )?;

    // Load the identity here, before the runner does, purely to log the peer ID
    // first: phase-1 verification and operator muscle-memory both grep for this
    // line, and `run_node` has a DID line and a config summary ahead of its own
    // peer-ID log. The runner loads the same file again through the same
    // protobuf decoding, so this cannot disagree with what it binds.
    let keypair = kwaai_p2p::identity::load_keypair(&args.identity_key).with_context(|| {
        format!(
            "loading the bootstrap identity from {} — a seed must have a stable identity, \
             so this file is never generated for you",
            args.identity_key.display()
        )
    })?;
    let peer_id = keypair.public().to_peer_id();

    info!("Bootstrap identity: {}", peer_id.to_base58());
    info!("Listening on: {}", args.listen);
    for addr in &args.announce {
        info!("Announcing: {addr}/p2p/{}", peer_id.to_base58());
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

    // The same runner `Command::RunNode` calls, which dispatches to
    // `node_native::run_native_node` because the preset sets `native_p2p`.
    crate::node::run_node(&config).await?;

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

/// The TCP port out of a `--listen` multiaddr.
///
/// The config carries a port and `node_native` rebuilds
/// `/ip4/0.0.0.0/tcp/<port>` from it, so the address is reduced to the one
/// component that differs between deployments. A multiaddr with no TCP
/// component is rejected rather than silently defaulted: binding a port the
/// operator did not ask for is how a seed ends up unreachable at the address
/// every node has pinned.
fn listen_port(listen: &str) -> Result<u16> {
    let addr: Multiaddr = listen
        .parse()
        .with_context(|| format!("parsing --listen {listen}"))?;
    addr.iter()
        .find_map(|p| match p {
            libp2p::multiaddr::Protocol::Tcp(port) => Some(port),
            _ => None,
        })
        .with_context(|| format!("--listen {listen} has no TCP port component"))
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
    ///
    /// The preset itself does not touch the file (it only records the path), so
    /// this asserts the loader `serve_bootstrap` and `run_node` both go
    /// through, and that nothing is generated as a side effect.
    #[test]
    fn a_missing_identity_key_is_an_error_not_a_new_identity() {
        let dir = tempfile::tempdir().expect("tmpdir");
        let missing = dir.path().join("does-not-exist.bin");

        let err = kwaai_p2p::identity::load_keypair(&missing)
            .expect_err("a missing identity key must be an error, not a fresh identity");
        assert!(
            !missing.exists(),
            "a failed load must not leave a generated key behind: {err:#}"
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

    // ── The preset ─────────────────────────────────────────────────────────

    /// Every field the preset sets, asserted one by one.
    ///
    /// This is the whole contract of the rework: a seed is *this config* handed
    /// to the ordinary node runner, so a field silently changing meaning is the
    /// failure mode worth pinning. The flags used are the ones the deployed
    /// entrypoint passes.
    #[test]
    fn the_preset_is_exactly_the_documented_config() {
        let config = seed_config(
            Path::new("/keys/identity.bin"),
            "/ip4/0.0.0.0/tcp/8000",
            &["/dns4/bootstrap1/tcp/8000".to_string()],
            true,
        )
        .expect("the deployed flags must produce a config");

        assert!(config.native_p2p, "a seed runs on the native stack");
        assert!(
            !config.announce_self,
            "a seed publishes no records of its own"
        );
        assert!(
            config.dht_server,
            "a seed answers Kademlia queries from t=0, before AutoNAT confirms an address"
        );
        assert!(
            !config.enable_upnp,
            "a seed is deployed at a known address and must not fire SSDP"
        );
        assert!(
            !config.no_relay,
            "the hop server stays on so NATed nodes can reserve circuits"
        );
        assert!(
            !config.health_monitoring.enabled,
            "a seed polls no health endpoint"
        );
        assert!(!config.vpk_enabled, "a seed hosts no VPK service");
        assert!(!config.ollama_manage, "a seed supervises no Ollama");
        assert!(
            config.initial_peers.is_empty(),
            "a seed dials nobody — peers come to it"
        );
        assert_eq!(
            config.identity_key.as_deref(),
            Some(Path::new("/keys/identity.bin")),
            "the identity is the file the operator named, and is loaded not generated"
        );
        assert_eq!(config.port, 8000, "the port comes out of --listen");
        assert_eq!(
            config.announce_addr.as_deref(),
            Some("/dns4/bootstrap1/tcp/8000"),
            "the first --announce is declared as the external address"
        );
    }

    /// `--no-relay` is the one flag that flips a preset field, so it gets its
    /// own case rather than riding on the default.
    #[test]
    fn no_relay_turns_off_the_hop_server() {
        let config = seed_config(Path::new("/keys/k.bin"), DEFAULT_LISTEN, &[], false)
            .expect("a config without announce addresses is still valid");
        assert!(config.no_relay, "--no-relay must reach the config");
        assert_eq!(
            config.announce_addr, None,
            "no --announce means no declared external address"
        );
    }

    /// A non-default `--listen` port reaches the config, which is what
    /// `node_native` rebuilds its listen multiaddr from.
    #[test]
    fn the_listen_port_is_lifted_out_of_the_multiaddr() {
        let config = seed_config(Path::new("/k.bin"), "/ip4/0.0.0.0/tcp/34567", &[], true)
            .expect("an ephemeral-range port is a valid listen address");
        assert_eq!(config.port, 34567);
    }

    /// A `--listen` with no TCP port is rejected rather than defaulted: binding
    /// a port nobody asked for is how a seed goes missing at the address every
    /// node has pinned.
    #[test]
    fn a_listen_address_without_a_tcp_port_is_rejected() {
        let err = seed_config(Path::new("/k.bin"), "/ip4/0.0.0.0", &[], true)
            .expect_err("a listen address with no TCP port must be rejected");
        assert!(
            format!("{err:#}").contains("no TCP port"),
            "the error must name the problem: {err:#}"
        );
    }

    /// The preset leaves `model` and `blocks` at their defaults, and that is
    /// safe because nothing on the seed path reads them.
    ///
    /// Documented as a test so the reasoning survives: `run_native_node` uses
    /// them only to build an announce record, and `announce_self = false` short
    /// -circuits before any record is built. No model download and no shard
    /// spawn is reachable from the node runner at all — both belong to
    /// `shard serve`.
    #[test]
    fn the_preset_leaves_the_model_fields_alone_because_nothing_reads_them() {
        let config = seed_config(Path::new("/k.bin"), DEFAULT_LISTEN, &[], true).expect("config");
        let default = KwaaiNetConfig::default();
        assert_eq!(config.model, default.model);
        assert_eq!(config.blocks, default.blocks);
        assert!(
            !config.announce_self,
            "which is only safe because no announce record is ever built"
        );
    }
}
