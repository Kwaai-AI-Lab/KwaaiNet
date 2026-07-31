//! Serving the hivemind DHT natively: `rpc_ping` / `rpc_store` / `rpc_find`.
//!
//! [`kwaai_hivemind_dht::DHTStorage`] is deliberately network-free — it takes
//! requests and routing snapshots and returns responses. This module is the
//! only thing that connects it to a live swarm:
//!
//! * it registers the three bare hivemind protocols as unary handlers on a
//!   [`NetworkHandle`], each decoding a protobuf request, calling the matching
//!   `DHTStorage` method, and encoding the response;
//! * it runs a maintenance task that keeps the storage's routing table in step
//!   with Kademlia's k-buckets and sweeps expired records.
//!
//! Without the second half nothing would ever call `update_peers` or
//! `cleanup_expired`: the storage would answer every `rpc_find` with an empty
//! neighbour list (breaking iterative lookups against us) and would pin every
//! record nobody asks about forever.
//!
//! ```text
//!   remote peer ──DHTProtocol.rpc_store──▶ [unary handler] ──▶ DHTStorage
//!                                                                  ▲
//!   kad k-buckets ──routing_peers()──▶ [maintenance task] ─────────┘
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use kwaai_hivemind_dht::DHTStorage;
//! use kwaai_p2p::{dht_service, NetworkConfig, NetworkService};
//! use libp2p::identity::Keypair;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let keypair = Keypair::generate_ed25519();
//! let storage = DHTStorage::new(keypair.public().to_peer_id());
//! let (handle, _task) = NetworkService::spawn(NetworkConfig::default(), keypair)?;
//!
//! let dht = dht_service::spawn_dht_service(handle.clone(), storage).await?;
//! # let _ = dht;
//! # Ok(())
//! # }
//! ```

use std::time::Duration;

use kwaai_hivemind_dht::protocol::{
    FindRequest, NodeInfo, PingRequest, PingResponse, ResponseAuthInfo, StoreRequest,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{DHTStorage, PROTOCOL_FIND, PROTOCOL_PING, PROTOCOL_STORE};
use prost::Message;
use tracing::{debug, trace, warn};

use crate::error::P2PResult;
use crate::handle::NetworkHandle;

/// How often the maintenance task refreshes the routing snapshot and sweeps
/// expired records.
///
/// 60 s is well under the 360 s TTL every announced record carries, so a record
/// is swept within a minute of expiring, and it is frequent enough that the
/// neighbour lists we serve track a churning routing table. The cost is one
/// in-memory k-bucket walk per minute.
pub const MAINTENANCE_INTERVAL: Duration = Duration::from_secs(60);

/// Register the three hivemind DHT protocols and start the maintenance task.
///
/// Returns the maintenance task's `JoinHandle`. Dropping it does **not** stop
/// the task — it ends on its own when the [`NetworkHandle`] it holds can no
/// longer reach a running service, so a caller that wants an early stop should
/// abort the handle explicitly.
///
/// The handlers themselves are owned by the service (one dispatch task per
/// protocol, spawned by `add_unary_handler`) and outlive this call.
pub async fn spawn_dht_service(
    handle: NetworkHandle,
    storage: DHTStorage,
) -> P2PResult<tokio::task::JoinHandle<()>> {
    let local_peer = handle.peer_id();

    {
        let storage = storage.clone();
        handle
            .add_unary_handler(PROTOCOL_STORE, move |data: Vec<u8>| {
                let storage = storage.clone();
                async move { handle_store(&storage, &data) }
            })
            .await?;
    }

    {
        let storage = storage.clone();
        handle
            .add_unary_handler(PROTOCOL_FIND, move |data: Vec<u8>| {
                let storage = storage.clone();
                async move { handle_find(&storage, &data) }
            })
            .await?;
    }

    handle
        .add_unary_handler(PROTOCOL_PING, move |data: Vec<u8>| async move {
            handle_ping(local_peer, &data)
        })
        .await?;

    debug!(peer = %local_peer, "hivemind DHT service registered");

    Ok(tokio::spawn(maintenance_loop(handle, storage)))
}

/// Decode a `StoreRequest`, apply it, and encode the `StoreResponse`.
fn handle_store(storage: &DHTStorage, data: &[u8]) -> Result<Vec<u8>, String> {
    let request =
        StoreRequest::decode(data).map_err(|e| format!("undecodable rpc_store request: {e}"))?;
    trace!(keys = request.keys.len(), "serving rpc_store");
    Ok(storage.handle_store(request).encode_to_vec())
}

/// Decode a `FindRequest`, answer it, and encode the `FindResponse`.
fn handle_find(storage: &DHTStorage, data: &[u8]) -> Result<Vec<u8>, String> {
    let request =
        FindRequest::decode(data).map_err(|e| format!("undecodable rpc_find request: {e}"))?;
    trace!(keys = request.keys.len(), "serving rpc_find");
    Ok(storage.handle_find(request).encode_to_vec())
}

/// Answer `rpc_ping` with our own [`NodeInfo`] and the current DHT time.
///
/// # Why `available = false`
///
/// Hivemind's `rpc_ping` sets `available = True` only after the responder has
/// itself dialed the caller back and confirmed the caller is reachable
/// (`protocol.py`'s `validate` flow). We do not run that dial-back yet, so
/// claiming `available = true` would be an unverified assertion about a third
/// party. Reporting `false` is the safe direction: the field feeds the
/// **caller's** confidence in its own reachability, so a `false` costs the
/// caller nothing but a missed confirmation from one peer — it does not affect
/// whether our records are stored, found, or returned, and every other field in
/// the response (our node ID, the DHT time) is fully valid. Deferred until the
/// reachability state machine lands in Phase 4, which is where the dial-back
/// belongs.
///
/// `DHTStorage::handle_request` hardcodes `available = true`, which is why the
/// ping arm is served here rather than delegated to it.
fn handle_ping(local_peer: crate::PeerId, data: &[u8]) -> Result<Vec<u8>, String> {
    // Decoded for its side effect of rejecting garbage: a caller that sent us
    // something that is not a PingRequest gets a clean error arm rather than a
    // well-formed reply to a message we did not understand.
    let _request =
        PingRequest::decode(data).map_err(|e| format!("undecodable rpc_ping request: {e}"))?;

    let response = PingResponse {
        auth: Some(ResponseAuthInfo::new()),
        peer: Some(NodeInfo::from_peer_id(local_peer)),
        dht_time: get_dht_time(),
        available: false,
    };
    Ok(response.encode_to_vec())
}

/// Keep the storage's routing table and record set fresh.
///
/// Ends when the network service is gone: `routing_peers()` then fails with
/// [`crate::P2PError::NotInitialized`], which is the signal that there is
/// nothing left to maintain.
async fn maintenance_loop(handle: NetworkHandle, storage: DHTStorage) {
    let mut ticker = tokio::time::interval(MAINTENANCE_INTERVAL);
    // The first tick fires immediately; skip it so we do not snapshot an empty
    // routing table before the initial bootstrap has connected anything.
    ticker.tick().await;

    loop {
        ticker.tick().await;

        match handle.routing_peers().await {
            Ok(peers) => {
                trace!(count = peers.len(), "refreshing DHT routing snapshot");
                storage.update_peer_ids(peers);
            }
            Err(e) => {
                debug!(error = %e, "network service is gone; stopping DHT maintenance");
                return;
            }
        }

        storage.cleanup_expired();
        let (total, _) = storage.stats();
        trace!(records = total, "DHT maintenance sweep complete");
    }
}

/// Stop serving the three hivemind DHT protocols.
///
/// Idempotent; reports whether anything was actually registered. After this
/// resolves, a caller's `rpc_store` gets the same clean negotiation refusal a
/// never-registered protocol produces.
pub async fn remove_dht_service(handle: &NetworkHandle) -> P2PResult<bool> {
    let mut any = false;
    for proto in [PROTOCOL_STORE, PROTOCOL_FIND, PROTOCOL_PING] {
        match handle.remove_unary_handler(proto).await {
            Ok(existed) => any |= existed,
            Err(e) => {
                warn!(%proto, error = %e, "failed to remove DHT handler");
                return Err(e);
            }
        }
    }
    Ok(any)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kwaai_hivemind_dht::protocol::{FindResponse, RequestAuthInfo, ResultType, StoreResponse};
    use libp2p::PeerId;

    fn future() -> f64 {
        get_dht_time() + 3600.0
    }

    /// A malformed request produces an error arm, not a well-formed reply to
    /// something we never parsed.
    #[test]
    fn undecodable_requests_are_rejected() {
        let storage = DHTStorage::new(PeerId::random());
        // A protobuf field header for a varint field the messages do not have,
        // followed by a truncated body.
        let garbage = &[0x0a, 0xff, 0xff];

        assert!(handle_store(&storage, garbage).is_err());
        assert!(handle_find(&storage, garbage).is_err());
        assert!(handle_ping(PeerId::random(), garbage).is_err());
    }

    /// `rpc_ping` reports our own DHTID and a live clock, and deliberately does
    /// not claim to have validated the caller.
    #[test]
    fn ping_reports_our_node_id_and_defers_validation() {
        let local = PeerId::random();
        let request = PingRequest::new(NodeInfo::from_peer_id(PeerId::random()), true);

        let bytes = handle_ping(local, &request.encode_to_vec()).expect("ping should succeed");
        let response = PingResponse::decode(&bytes[..]).expect("decodable response");

        assert_eq!(
            response.peer.expect("peer").node_id,
            NodeInfo::from_peer_id(local).node_id
        );
        assert!(response.dht_time > 0.0);
        assert!(
            !response.available,
            "validation is deferred, so availability must not be asserted"
        );
    }

    /// The store and find arms are the storage's own semantics, reached through
    /// a protobuf round trip — including the subkeyed dictionary behaviour the
    /// petals record layout depends on.
    #[test]
    fn store_and_find_round_trip_through_the_wire_types() {
        let storage = DHTStorage::new(PeerId::random());
        let key = b"key".to_vec();
        let subkey = rmp_serde::to_vec("QmPeer").unwrap();

        let store = StoreRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![key.clone()],
            subkeys: vec![subkey.clone()],
            values: vec![b"value".to_vec()],
            expiration_time: vec![future()],
            in_cache: vec![false],
            peer: None,
        };
        let bytes = handle_store(&storage, &store.encode_to_vec()).expect("store should succeed");
        assert_eq!(
            StoreResponse::decode(&bytes[..]).unwrap().store_ok,
            vec![true]
        );

        let find = FindRequest {
            auth: Some(RequestAuthInfo::new()),
            keys: vec![key],
            peer: None,
        };
        let bytes = handle_find(&storage, &find.encode_to_vec()).expect("find should succeed");
        let response = FindResponse::decode(&bytes[..]).unwrap();

        assert_eq!(
            response.results[0].result_type,
            ResultType::FoundDictionary as i32,
            "a subkeyed record must serve as FOUND_DICTIONARY"
        );
        let dict = kwaai_hivemind_dht::parse_dictionary(&response.results[0].value).unwrap();
        assert_eq!(dict.entries[&subkey].0, b"value");
    }
}
