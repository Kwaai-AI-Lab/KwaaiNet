//! The composed libp2p `NetworkBehaviour` for a KwaaiNet node.
//!
//! The minimum set needed for a node to join the network and be observed by it:
//!
//! - [`ping`] — liveness / RTT, and it keeps otherwise-idle connections honest.
//! - [`identify`] — protocol/agent advertisement plus the **observed address**
//!   feed that later phases use for reachability detection.
//! - [`kad`] — Kademlia peer routing on the *default* `/ipfs/kad/1.0.0`
//!   protocol. This is deliberate: the Python bootstraps run hivemind's
//!   go-libp2p daemon with no `ProtocolPrefix`, so any custom protocol name
//!   here silently partitions us from the live network.
//!
//! - [`unary`] — hivemind unary RPC. Inbound handler protocols register at
//!   runtime, so the behaviour starts with an empty protocol set and the
//!   service loop drives `register_protocol`/`unregister_protocol`.

use std::time::Duration;

use libp2p::{
    identify, kad,
    kad::store::MemoryStore,
    ping,
    swarm::NetworkBehaviour,
    {identity, PeerId},
};

use crate::config::NetworkConfig;
use crate::unary;

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
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
    pub kad: kad::Behaviour<MemoryStore>,
    pub unary: unary::Behaviour,
}

impl KwaaiBehaviour {
    /// Build the behaviour for `keypair` using `config`.
    pub fn new(keypair: &identity::Keypair, config: &NetworkConfig) -> Self {
        let local_peer_id = PeerId::from(keypair.public());

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

        // NOTE: `kad::Config::default()` uses `/ipfs/kad/1.0.0`. Do not call
        // `set_protocol_names` — see the module docs.
        let mut kad_config = kad::Config::default();
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

        Self {
            ping,
            identify,
            kad,
            unary,
        }
    }
}
