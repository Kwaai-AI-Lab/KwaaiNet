//! The composed libp2p `NetworkBehaviour` for a KwaaiNet node.
//!
//! The minimum set needed for a node to join the network and be observed by it:
//!
//! - [`ping`] — liveness / RTT, and it keeps otherwise-idle connections honest.
//! - [`identify`] — protocol/agent advertisement plus the **observed address**
//!   feed that later phases use for reachability detection.
//! - [`kad`] — Kademlia peer routing. **The build decides the protocol set**
//!   (see [`crate::config::kad_protocols()`]): a stock build serves
//!   `/kwaai/kad/1.0.0` alone, and the `kad-multi-protocol` build — the
//!   bootstrap-grade one for the migration window — also answers the legacy
//!   `/ipfs/kad/1.0.0`, so peers that predate the kwaai name still match.
//!   Serving the legacy name is what lets the public IPFS DHT recruit a
//!   publicly reachable node as one of its *DHT servers* — the absorption
//!   mode that fills routing tables and OOM-killed the old bootstraps — so
//!   the stock default is the name that cannot be recruited, and the dual
//!   build is reserved for the one host that must bridge. Narrower claim
//!   than it sounds: foreign peers still *connect* (TCP/noise/identify all
//!   negotiate below kad, and their addresses arrive from already-absorbed
//!   peers' tables), so the connection table fills either way — measured
//!   live on both builds. Connection-level gating is #174's job, not this
//!   protocol set's. There is no runtime knob; the rollout
//!   order (dual bootstraps deployed before any node upgrades) is what
//!   keeps upgraded and legacy nodes routable to each other.
//!
//! - [`unary`] — hivemind unary RPC. Inbound handler protocols register at
//!   runtime, so the behaviour starts with an empty protocol set and the
//!   service loop drives `register_protocol`/`unregister_protocol`.
//! - [`raw_stream`] — raw (unframed) libp2p streams on arbitrary, possibly
//!   slash-less protocols, which the control socket's pipe mode relays bytes
//!   over. Its protocol set is separate from `unary`'s, so a name can be served
//!   as a unary handler *or* as a raw stream but never ambiguously as both.
//!
//! The NAT-traversal set:
//!
//! - [`autonat`] — reachability probing. Always client **and** server in 0.12
//!   (`ProtocolSupport::Full`), which is what we want: a directly-reachable node
//!   answers dialbacks for the NATed ones. Its `only_global_ips` knob is the
//!   single address-class check in all of rust-libp2p 0.53, so it is driven
//!   from [`NetworkConfig::require_global_ips`] and cannot disagree with
//!   `crate::addresses`.
//! - `relay::client` — circuit reservations when we are unreachable. It cannot
//!   be constructed here: only `SwarmBuilder::with_relay_client` can build the
//!   transport and the behaviour as a matched pair, so it is *passed in*.
//! - `relay::Behaviour` behind a `Toggle` — the hop side, letting this node
//!   relay for others. On by default, matching the p2pd path's `-relay`.
//! - [`dcutr`] — direct-connection upgrade through relay. Fully automatic: it
//!   attempts a hole punch on every inbound relayed connection, so there is
//!   nothing to drive from the service loop beyond logging the outcome.
//! - `upnp` behind a `Toggle` — port mapping via the local gateway. Off in
//!   tests, where an SSDP broadcast would be noise at best.

use std::time::Duration;

use libp2p::{
    autonat, connection_limits, dcutr, identify, kad,
    kad::store::MemoryStore,
    ping, relay,
    swarm::{behaviour::toggle::Toggle, NetworkBehaviour},
    upnp, {identity, PeerId},
};

use crate::config::{NetworkConfig, KAD_MULTI_PROTOCOL_BUILD};
use crate::{raw_stream, unary};

/// The `protocol_version` advertised over identify.
///
/// go-libp2p (and therefore the hivemind daemon the Python bootstraps run)
/// sends `ipfs/0.1.0`. Some peers gate on this string, so we match go rather
/// than inventing a kwaai-specific value. The kwaai-specific information rides
/// in `agent_version` instead.
pub const DEFAULT_PROTOCOL_VERSION: &str = "ipfs/0.1.0";

/// The `agent_version` advertised over identify, e.g. `kwaainet/0.5.4`.
pub fn default_agent_version() -> String {
    format!("kwaainet/{}", env!("CARGO_PKG_VERSION"))
}

/// The composed behaviour driven by [`crate::service::NetworkService`].
#[derive(NetworkBehaviour)]
pub struct KwaaiBehaviour {
    /// First so a refusal happens before anything else allocates for the
    /// connection.
    pub connection_limits: connection_limits::Behaviour,
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
    pub kad: kad::Behaviour<MemoryStore>,
    pub unary: unary::Behaviour,
    pub raw_stream: raw_stream::Behaviour,
    pub autonat: autonat::Behaviour,
    pub relay_client: relay::client::Behaviour,
    pub relay_server: Toggle<relay::Behaviour>,
    pub dcutr: dcutr::Behaviour,
    pub upnp: Toggle<upnp::tokio::Behaviour>,
}

impl KwaaiBehaviour {
    /// Build the behaviour for `keypair` using `config`.
    ///
    /// `relay_client` is not constructed here because it cannot be: the client
    /// behaviour and the circuit transport are one object split in two, and
    /// only `SwarmBuilder::with_relay_client` can produce the pair. It arrives
    /// already built from `NetworkService::spawn`'s builder chain — which is
    /// also why the `with_behaviour` closure there takes two arguments.
    pub fn new(
        keypair: &identity::Keypair,
        config: &NetworkConfig,
        relay_client: relay::client::Behaviour,
    ) -> Self {
        let local_peer_id = PeerId::from(keypair.public());

        // `max_connections` has been in the config all along, enforced nowhere.
        // It matters now: idle connections are kept for ten minutes rather than
        // thirty seconds, so this is the only thing bounding growth, and a
        // bootstrap is where that bites first.
        //
        // Incoming is capped below the total so a node can always dial out. If
        // inbound filled every slot the node would go deaf — unable to re-dial
        // the bootstraps and relays that keep it reachable.
        //
        // Deliberately no per-peer cap: DCUtR holds the relayed and the punched
        // connection at once, and refusing the second is refusing the punch.
        let max = u32::try_from(config.max_connections).unwrap_or(u32::MAX);
        let connection_limits = connection_limits::Behaviour::new(
            connection_limits::ConnectionLimits::default()
                .with_max_established(Some(max))
                .with_max_established_incoming(Some((max / 5 * 4).max(1)))
                .with_max_pending_incoming(Some((max / 4).max(4))),
        );

        let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(30)));

        let protocol_version = if config.protocol_version.is_empty() {
            DEFAULT_PROTOCOL_VERSION.to_string()
        } else {
            config.protocol_version.clone()
        };
        let agent_version = if config.agent_version.is_empty() {
            default_agent_version()
        } else {
            config.agent_version.clone()
        };

        let identify = identify::Behaviour::new(
            identify::Config::new(protocol_version, keypair.public())
                .with_agent_version(agent_version)
                // Push our listen-addr changes so peers (and their kad tables)
                // learn about newly-discovered external addresses without
                // waiting for the next identify interval.
                .with_push_listen_addr_updates(true)
                .with_interval(Duration::from_secs(5 * 60)),
        );

        // Compiled in, so nothing here can disagree with the binary. Logged
        // because a bootstrap built from the wrong artifact is otherwise
        // silent about it: the build flag shows up nowhere else at runtime.
        let protocols = config.kad_protocols.clone();
        debug_assert!(
            protocols.len() <= 1 || KAD_MULTI_PROTOCOL_BUILD,
            "more than one kad protocol on a build that cannot serve them"
        );
        tracing::info!(
            protocols = ?protocols.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
            multi_protocol_build = KAD_MULTI_PROTOCOL_BUILD,
            "kad protocol set"
        );
        // `first()` with a fallback rather than an index: the compiled sets
        // are never empty, but the field is a pub Vec that tests and helpers
        // may hand an empty list — which must not panic from inside the
        // swarm builder. The patched `set_protocol_names` already ignores
        // an empty list, so the fallback keeps the two behaviours aligned.
        let mut kad_config = kad::Config::new(
            protocols
                .first()
                .cloned()
                .unwrap_or_else(|| crate::config::kad_protocols()[0].clone()),
        );
        #[cfg(feature = "kad-multi-protocol")]
        kad_config.set_protocol_names(protocols);
        kad_config.set_query_timeout(config.request_timeout);
        kad_config.set_replication_factor(
            std::num::NonZeroUsize::new(config.dht_replication)
                .unwrap_or(std::num::NonZeroUsize::new(20).expect("20 != 0")),
        );

        let store = MemoryStore::new(local_peer_id);
        let mut kad = kad::Behaviour::with_config(local_peer_id, store, kad_config);

        if config.dht_server {
            // Force server mode: answer queries and be inserted into other
            // peers' routing tables even before our reachability is confirmed.
            kad.set_mode(Some(kad::Mode::Server));
        }
        // Otherwise leave `auto_mode` on: kad flips to Server once an external
        // address is confirmed, Client until then.

        // The inbound protocol set starts empty — handlers register at runtime
        // through `NetworkHandle::add_unary_handler`. `max_concurrent_streams`
        // keeps its default: it is a per-connection resource guard, unrelated to
        // anything `NetworkConfig` currently expresses.
        let unary = unary::Behaviour::new(unary::Config {
            request_timeout: config.request_timeout,
            ..unary::Config::default()
        });

        // Raw-stream registrations are likewise runtime-only (pipe mode's
        // `STREAM_HANDLER`), so this too starts with an empty protocol set.
        let raw_stream = raw_stream::Behaviour::new();

        // Four AutoNAT knobs are moved off their defaults; everything else is
        // left alone deliberately, `confidence_max` most of all — needing three
        // consistent probes to flip status is the flapping damper, and lowering
        // it would make a single unlucky dialback re-announce the node.
        let autonat = autonat::Behaviour::new(
            local_peer_id,
            autonat::Config {
                // The one address-class lever in rust-libp2p 0.53; kept in
                // step with `crate::addresses` through the same key.
                only_global_ips: config.require_global_ips,
                // Defaults (15s / 90s / 15min) are tuned for a long-lived node
                // on a stable network. A node that has just started wants to
                // know where it stands before its first announce lands, so
                // probe sooner and re-check more often.
                boot_delay: Duration::from_secs(5),
                retry_interval: Duration::from_secs(30),
                refresh_interval: Duration::from_secs(5 * 60),
                ..autonat::Config::default()
            },
        );

        // Hop server: libp2p's rate limiters and concurrency caps stay; only the
        // per-circuit volume and duration are raised, to match the p2pd relay.
        let max_circuit_duration = config
            .relay_max_circuit_duration
            // libp2p converts this to u32 seconds and panics otherwise.
            .min(Duration::from_secs(u64::from(u32::MAX)));
        let relay_server = Toggle::from(config.relay_server.then(|| {
            relay::Behaviour::new(
                local_peer_id,
                relay::Config {
                    max_circuit_bytes: config.relay_max_circuit_bytes,
                    max_circuit_duration,
                    ..relay::Config::default()
                },
            )
        }));

        // dcutr has no Config in 0.11 — it acts on relayed connections by
        // itself, so there is nothing to tune and nothing to drive.
        let dcutr = dcutr::Behaviour::new(local_peer_id);

        // upnp is `Default`-only. Off in tests: SSDP is a LAN broadcast, and CI
        // has no business talking to whatever gateway happens to answer.
        let upnp = Toggle::from(config.enable_upnp.then(upnp::tokio::Behaviour::default));

        Self {
            connection_limits,
            ping,
            identify,
            kad,
            unary,
            raw_stream,
            autonat,
            relay_client,
            relay_server,
            dcutr,
            upnp,
        }
    }
}
