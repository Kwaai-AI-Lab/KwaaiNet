//! Serving the hivemind DHT through a real swarm, in process.
//!
//! `dht_service`'s unit tests cover the decode/apply/encode arms directly.
//! This tier puts the same handlers behind two live [`NetworkService`]s so the
//! path under test is the production one end to end: a remote peer negotiates
//! the bare protocol name, the frame crosses a libp2p stream, the service
//! dispatch map routes it, and the response comes back decodable.
//!
//! What it proves that the unit tests cannot:
//!
//! - the three bare protocols (`DHTProtocol.rpc_{ping,store,find}`) actually
//!   negotiate through `unary::Behaviour` when registered by `spawn_dht_service`,
//! - a subkeyed store survives the wire and comes back as `FOUND_DICTIONARY`,
//!   which is the record layout every petals block key depends on,
//! - the maintenance task really does feed the routing table, so our `rpc_find`
//!   answers carry nearest peers rather than an empty list.
//!
//! Both swarms are loopback-only with freshly generated keys, so nothing here
//! can collide with a node running on the same machine.

use std::time::Duration;

use kwaai_hivemind_dht::protocol::{
    FindRequest, FindResponse, NodeInfo, PingRequest, PingResponse, RequestAuthInfo, ResultType,
    StoreRequest, StoreResponse,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{DHTStorage, PROTOCOL_FIND, PROTOCOL_PING, PROTOCOL_STORE};
use kwaai_p2p::{dht_service, NetworkConfig, NetworkHandle, NetworkService, P2PError, PeerId};
use libp2p::identity::Keypair;
use prost::Message;

/// A per-call cap so a lost reply fails the test rather than hanging it.
const TEST_TIMEOUT: Duration = Duration::from_secs(20);

/// Spawn a loopback service, keeping the task handle alive for the test.
fn spawn_service() -> (NetworkHandle, tokio::task::JoinHandle<()>, PeerId) {
    let keypair = Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    let (handle, task) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service should start");
    (handle, task, peer_id)
}

/// The first listen address of `handle`, with `/p2p/<peer-id>` appended so it is
/// directly dialable.
async fn dialable_addr(handle: &NetworkHandle, peer_id: PeerId) -> String {
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    loop {
        if let Some(addr) = handle
            .listen_addrs()
            .await
            .ok()
            .and_then(|a| a.into_iter().next())
        {
            return format!("{addr}/p2p/{peer_id}");
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for a listen address"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// A client service connected to a second service that is serving the DHT.
struct Fixture {
    client: NetworkHandle,
    server: NetworkHandle,
    server_id: PeerId,
    storage: DHTStorage,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Fixture {
    async fn new() -> Self {
        let (client, client_task, _) = spawn_service();
        let (server, server_task, server_id) = spawn_service();

        let storage = DHTStorage::new(server_id);
        let maintenance = dht_service::spawn_dht_service(server.clone(), storage.clone())
            .await
            .expect("the DHT service should register");

        let addr = dialable_addr(&server, server_id).await;
        let connected = client
            .connect_peer(&addr)
            .await
            .expect("loopback dial should succeed");
        assert_eq!(connected, server_id);

        Self {
            client,
            server,
            server_id,
            storage,
            tasks: vec![client_task, server_task, maintenance],
        }
    }

    /// Call a DHT protocol on the serving node, failing rather than hanging.
    async fn call(&self, proto: &str, data: Vec<u8>) -> Result<Vec<u8>, P2PError> {
        tokio::time::timeout(
            TEST_TIMEOUT,
            self.client.call_unary_handler(self.server_id, proto, &data),
        )
        .await
        .expect("the call must resolve within the timeout")
    }

    async fn shutdown(self) {
        let _ = self.client.shutdown().await;
        let _ = self.server.shutdown().await;
        for task in self.tasks {
            task.abort();
        }
    }
}

fn future() -> f64 {
    get_dht_time() + 3600.0
}

// ============================================================================
// Tests
// ============================================================================

/// The petals-critical round trip: store a subkeyed record over the wire, then
/// find it back as `FOUND_DICTIONARY` (rt=2) with our subkey intact.
///
/// A regular (sentinel-subkey) store would come back as `FOUND_REGULAR` and the
/// second server to announce a block would erase the first — which is exactly
/// the bug this record layout exists to prevent.
#[tokio::test]
async fn subkeyed_store_is_found_back_as_a_dictionary() {
    let fx = Fixture::new().await;

    let key = b"model.block.0".to_vec();
    let subkey = rmp_serde::to_vec("QmClientPeer").expect("msgpack subkey");
    let value = b"server-info-bytes".to_vec();

    let store = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key.clone()],
        subkeys: vec![subkey.clone()],
        values: vec![value.clone()],
        expiration_time: vec![future()],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(fx.client.peer_id())),
    };

    let bytes = fx
        .call(PROTOCOL_STORE, store.encode_to_vec())
        .await
        .expect("a native DHT node must accept an rpc_store");
    let response = StoreResponse::decode(&bytes[..]).expect("decodable StoreResponse");
    assert_eq!(response.store_ok, vec![true], "the record must be accepted");
    assert_eq!(
        response.peer.expect("responder identity").node_id,
        NodeInfo::from_peer_id(fx.server_id).node_id,
        "the response must identify the serving node"
    );

    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: Some(NodeInfo::from_peer_id(fx.client.peer_id())),
    };
    let bytes = fx
        .call(PROTOCOL_FIND, find.encode_to_vec())
        .await
        .expect("a native DHT node must answer an rpc_find");
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");

    assert_eq!(
        response.results[0].result_type,
        ResultType::FoundDictionary as i32,
        "rt=2 FOUND_DICTIONARY is what a subkeyed record must serve as"
    );
    let dict =
        kwaai_hivemind_dht::parse_dictionary(&response.results[0].value).expect("a dictionary");
    assert_eq!(
        dict.entries[&subkey].0, value,
        "our subkey and value must survive the wire round trip"
    );

    fx.shutdown().await;
}

/// A key nobody stored is `NOT_FOUND` — with nearest peers still attached, which
/// is what lets a caller's iterative lookup make progress through us.
#[tokio::test]
async fn missing_keys_report_not_found_with_nearest_peers() {
    let fx = Fixture::new().await;

    // Seed the routing snapshot directly: the maintenance task's first real
    // tick is 60 s out, and this test is about the find response shape, not
    // about the refresh cadence (which `routing_peers_reach_the_storage` covers).
    fx.storage.update_peer_ids((0..3).map(|_| PeerId::random()));

    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"nobody-stored-this".to_vec()],
        peer: Some(NodeInfo::from_peer_id(fx.client.peer_id())),
    };
    let bytes = fx
        .call(PROTOCOL_FIND, find.encode_to_vec())
        .await
        .expect("rpc_find must answer even for a missing key");
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");

    assert_eq!(response.results[0].result_type, ResultType::NotFound as i32);
    assert!(response.results[0].value.is_empty());
    assert_eq!(
        response.results[0].nearest_node_ids.len(),
        3,
        "NOT_FOUND must still carry neighbours or iterative lookups stall"
    );

    fx.shutdown().await;
}

/// `rpc_ping` round trip: our node ID and a live DHT clock come back, and
/// `available` is deliberately false because the dial-back validation that
/// would justify a true is deferred to Phase 4.
#[tokio::test]
async fn ping_round_trips_over_the_wire() {
    let fx = Fixture::new().await;

    let ping = PingRequest::new(NodeInfo::from_peer_id(fx.client.peer_id()), true);
    let before = get_dht_time();
    let bytes = fx
        .call(PROTOCOL_PING, ping.encode_to_vec())
        .await
        .expect("a native DHT node must answer rpc_ping");
    let response = PingResponse::decode(&bytes[..]).expect("decodable PingResponse");

    assert_eq!(
        response.peer.expect("peer").node_id,
        NodeInfo::from_peer_id(fx.server_id).node_id,
        "ping must report the serving node's own DHTID"
    );
    assert!(
        response.dht_time >= before,
        "dht_time must be a live clock reading"
    );
    assert!(
        !response.available,
        "validation is deferred; claiming availability would be unverified"
    );

    fx.shutdown().await;
}

/// The routing snapshot really flows from kad's k-buckets into the storage.
///
/// Without this wiring `rpc_find` would answer with an empty neighbour list
/// forever — the storage has no other source of peers. Asserted through
/// `routing_peers()` plus an explicit `update_peer_ids`, rather than by waiting
/// out the 60 s maintenance interval.
#[tokio::test]
async fn routing_peers_reach_the_storage() {
    let fx = Fixture::new().await;

    // The client dialed us, but kad only admits a peer to a bucket once
    // identify has confirmed it speaks the kad protocol — so this is a poll,
    // not an immediate assertion.
    let deadline = tokio::time::Instant::now() + TEST_TIMEOUT;
    let peers = loop {
        let peers = fx
            .server
            .routing_peers()
            .await
            .expect("routing_peers must reach the service");
        if peers.contains(&fx.client.peer_id()) {
            break peers;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "a connected peer must appear in the kad routing table, got {peers:?}"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    };

    // The maintenance task performs exactly this step on each tick.
    fx.storage.update_peer_ids(peers);

    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"any-key".to_vec()],
        // No `peer`, so nothing is excluded from the neighbour list.
        peer: None,
    };
    let bytes = fx
        .call(PROTOCOL_FIND, find.encode_to_vec())
        .await
        .expect("rpc_find should answer");
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");

    let returned: Vec<PeerId> = response.results[0]
        .nearest_peer_ids
        .iter()
        .map(|b| PeerId::from_bytes(b).expect("a valid peer id"))
        .collect();
    assert!(
        returned.contains(&fx.client.peer_id()),
        "the routing snapshot must show up in served neighbour lists, got {returned:?}"
    );

    fx.shutdown().await;
}

/// `remove_dht_service` is visible on the wire: afterwards a store is refused
/// during negotiation rather than silently accepted or left hanging.
#[tokio::test]
async fn removing_the_service_makes_calls_fail_cleanly() {
    let fx = Fixture::new().await;

    let store = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"k".to_vec()],
        subkeys: vec![rmp_serde::to_vec("sk").unwrap()],
        values: vec![b"v".to_vec()],
        expiration_time: vec![future()],
        in_cache: vec![false],
        peer: None,
    };

    fx.call(PROTOCOL_STORE, store.encode_to_vec())
        .await
        .expect("the service is registered");

    assert!(
        dht_service::remove_dht_service(&fx.server)
            .await
            .expect("removal should reach the service"),
        "removing a registered DHT service reports true"
    );

    let error = fx
        .call(PROTOCOL_STORE, store.encode_to_vec())
        .await
        .expect_err("a removed handler must not answer");
    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("does not support"),
            "removal must surface as a negotiation refusal, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }

    fx.shutdown().await;
}

/// A malformed payload gets the remote handler's error arm, not a hang and not
/// a well-formed reply to something that was never parsed.
#[tokio::test]
async fn undecodable_requests_surface_as_remote_errors() {
    let fx = Fixture::new().await;

    let error = fx
        .call(PROTOCOL_STORE, vec![0x0a, 0xff, 0xff])
        .await
        .expect_err("garbage must be rejected");
    match &error {
        P2PError::Protocol(text) => assert!(
            text.contains("undecodable"),
            "the handler's error text must be preserved, got {text}"
        ),
        other => panic!("expected P2PError::Protocol, got {other:?}"),
    }

    fx.shutdown().await;
}
