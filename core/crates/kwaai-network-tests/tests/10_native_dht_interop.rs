//! A real Go daemon storing to, and finding from, our native DHT node.
//!
//! `kwaai-p2p/tests/dht_service.rs` proves the DHT service works between two
//! rust swarms. That cannot catch a foreign-stack disagreement, and this exact
//! direction is the one the `stream.rs` response-wrapper bug used to break:
//! every inbound `rpc_ping`/`rpc_store`/`rpc_find` a Rust node served to a Go
//! caller failed to unmarshal, so no Go peer could ever store into us.
//!
//! ```text
//!   [p2pd] ──DHTProtocol.rpc_store──▶ [NetworkService + DHTStorage]
//!   [p2pd] ──DHTProtocol.rpc_find───▶ [NetworkService + DHTStorage]
//! ```
//!
//! | test | proves |
//! | --- | --- |
//! | `daemon_stores_and_finds_a_subkeyed_record` | the petals record layout survives a Go round trip |
//! | `daemon_pings_the_native_dht_node` | the previously-broken inbound ping path |
//! | `daemon_gets_not_found_with_nearest_peers` | iterative lookups can route through us |
//!
//! The p2pd here is a plain unary caller — it never parses the DHT payloads, so
//! this is a wire-and-dispatch proof, not a claim that Go's own DHT client
//! agrees with our semantics. What it does establish is that a Go daemon can
//! negotiate the bare protocol names against a `NetworkService`, deliver a
//! hivemind DHT request, and get a decodable response back.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like the other integration tiers.
//!
//! # Process hygiene
//!
//! The daemon is spawned via `TestNode::new_wire_peer`, whose `P2PDaemon` kills
//! **its own child by PID** on drop — nothing here kills by process name or
//! touches the default socket path. The `NetworkService` listens on an ephemeral
//! loopback port with a freshly generated key, so it cannot collide with a node
//! running on the same machine, and is shut down explicitly at the end of each
//! test.

use std::time::Duration;

use kwaai_hivemind_dht::protocol::{
    FindRequest, FindResponse, NodeInfo, PingRequest, PingResponse, RequestAuthInfo, ResultType,
    StoreRequest, StoreResponse,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{DHTStorage, PROTOCOL_FIND, PROTOCOL_PING, PROTOCOL_STORE};
use kwaai_network_tests::{harness::TestNode, metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{dht_service, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use libp2p::identity::Keypair;
use prost::Message;

/// Cap on any single daemon interaction, so a regression surfaces as a failure
/// rather than a hung suite.
const CALL_TIMEOUT: Duration = Duration::from_secs(20);

/// A `NetworkService` serving the hivemind DHT on loopback.
struct DhtNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// Listen address including `/p2p/<id>`, dialable by a p2pd.
    addr: String,
    storage: DHTStorage,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl DhtNode {
    async fn spawn() -> Self {
        let keypair = Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, task) = NetworkService::spawn(NetworkConfig::for_tests(), keypair)
            .expect("network service should start");

        let storage = DHTStorage::new(peer_id);
        let maintenance = dht_service::spawn_dht_service(handle.clone(), storage.clone())
            .await
            .expect("the DHT service should register");

        let listen_addr = tokio::time::timeout(CALL_TIMEOUT, async {
            loop {
                if let Some(addr) = handle
                    .listen_addrs()
                    .await
                    .ok()
                    .and_then(|a| a.into_iter().next())
                {
                    return addr;
                }
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        })
        .await
        .expect("the service must report a listen address");

        Self {
            handle,
            peer_id,
            addr: format!("{listen_addr}/p2p/{peer_id}"),
            storage,
            tasks: vec![task, maintenance],
        }
    }

    /// Stop the event loop and wait for the task to exit, so a test never
    /// leaves a listener behind.
    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
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

/// The load-bearing one: a Go daemon stores a **subkeyed** record into our
/// native node and finds it back as `FOUND_DICTIONARY` with the subkey intact.
///
/// This is the exact shape every petals block announcement uses, and the
/// direction the `stream.rs` bug broke — a Go caller could not decode our
/// reply at all, so nothing a Go peer stored ever landed.
#[tokio::test]
async fn daemon_stores_and_finds_a_subkeyed_record() {
    require_integration!();
    let mut rec = MetricsRecorder::start(
        "integration::native_dht::daemon_stores_and_finds",
        "integration",
    );

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let node = DhtNode::spawn().await;

    daemon
        .client
        .connect_peer(&node.addr)
        .await
        .expect("the daemon must dial the native DHT node");

    let key = b"model.block.0".to_vec();
    let subkey = rmp_serde::to_vec("QmGoDaemonPeer").expect("msgpack subkey");
    let value = b"server-info-from-go".to_vec();

    let store = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key.clone()],
        subkeys: vec![subkey.clone()],
        values: vec![value.clone()],
        expiration_time: vec![future()],
        in_cache: vec![false],
        peer: None,
    };

    let bytes = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon.client.call_unary_handler(
            &node.peer_id.to_bytes(),
            PROTOCOL_STORE,
            &store.encode_to_vec(),
        ),
    )
    .await
    .expect("the store must resolve within the timeout")
    .expect("a Go daemon must be able to store into our native DHT node");

    let response = StoreResponse::decode(&bytes[..]).expect("decodable StoreResponse");
    assert_eq!(
        response.store_ok,
        vec![true],
        "the record must be accepted, not merely acknowledged"
    );
    assert_eq!(
        response.peer.expect("responder identity").node_id,
        NodeInfo::from_peer_id(node.peer_id).node_id,
        "the response must carry our own 20-byte DHTID"
    );

    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: None,
    };
    let bytes = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon.client.call_unary_handler(
            &node.peer_id.to_bytes(),
            PROTOCOL_FIND,
            &find.encode_to_vec(),
        ),
    )
    .await
    .expect("the find must resolve within the timeout")
    .expect("a Go daemon must be able to read back from our native DHT node");

    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");
    assert_eq!(
        response.results[0].result_type,
        ResultType::FoundDictionary as i32,
        "a subkeyed record must serve as rt=2 FOUND_DICTIONARY"
    );

    let dict =
        kwaai_hivemind_dht::parse_dictionary(&response.results[0].value).expect("a dictionary");
    assert_eq!(
        dict.entries[&subkey].0, value,
        "the subkey and value must survive the Go round trip byte for byte"
    );

    rec.metric("record_bytes", response.results[0].value.len());
    node.shutdown().await;
    rec.finish(true);
}

/// The previously-broken inbound path in isolation: a Go daemon's `rpc_ping`
/// against a native responder. Before the wire fix this failed to unmarshal on
/// the Go side every time.
#[tokio::test]
async fn daemon_pings_the_native_dht_node() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::native_dht::daemon_pings", "integration");

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let node = DhtNode::spawn().await;

    daemon
        .client
        .connect_peer(&node.addr)
        .await
        .expect("the daemon must dial the native DHT node");

    let ping = PingRequest::new(NodeInfo::from_peer_id(PeerId::random()), true);
    let before = get_dht_time();

    let bytes = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon.client.call_unary_handler(
            &node.peer_id.to_bytes(),
            PROTOCOL_PING,
            &ping.encode_to_vec(),
        ),
    )
    .await
    .expect("the ping must resolve within the timeout")
    .expect("a Go daemon must be able to ping our native DHT node");

    let response = PingResponse::decode(&bytes[..]).expect("decodable PingResponse");
    assert_eq!(
        response.peer.expect("peer").node_id,
        NodeInfo::from_peer_id(node.peer_id).node_id
    );
    assert!(
        response.dht_time >= before,
        "dht_time must be a live clock reading, got {}",
        response.dht_time
    );
    // available=false is deliberate — see `dht_service::handle_ping`. Asserted
    // so a future change to the reachability story has to update this tier too.
    assert!(
        !response.available,
        "validation is deferred until the Phase 4 reachability work"
    );

    rec.metric("dht_time", response.dht_time);
    node.shutdown().await;
    rec.finish(true);
}

/// A key we do not hold answers `NOT_FOUND` **with** nearest peers, so a Go
/// caller's iterative lookup can keep walking through us rather than dead-ending.
#[tokio::test]
async fn daemon_gets_not_found_with_nearest_peers() {
    require_integration!();
    let mut rec = MetricsRecorder::start("integration::native_dht::not_found", "integration");

    let mut daemon = TestNode::new_wire_peer().await.expect("wire peer daemon");
    let node = DhtNode::spawn().await;

    daemon
        .client
        .connect_peer(&node.addr)
        .await
        .expect("the daemon must dial the native DHT node");

    // Seed the routing snapshot directly rather than waiting out the 60 s
    // maintenance tick; this test is about the response shape, not the cadence.
    let neighbours: Vec<PeerId> = (0..5).map(|_| PeerId::random()).collect();
    node.storage.update_peer_ids(neighbours.clone());

    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"a-key-nobody-stored".to_vec()],
        peer: None,
    };
    let bytes = tokio::time::timeout(
        CALL_TIMEOUT,
        daemon.client.call_unary_handler(
            &node.peer_id.to_bytes(),
            PROTOCOL_FIND,
            &find.encode_to_vec(),
        ),
    )
    .await
    .expect("the find must resolve within the timeout")
    .expect("a missing key must still produce a response, not an error");

    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");
    let result = &response.results[0];
    assert_eq!(result.result_type, ResultType::NotFound as i32);
    assert!(result.value.is_empty());
    assert_eq!(
        result.nearest_node_ids.len(),
        neighbours.len(),
        "NOT_FOUND must carry neighbours or an iterative lookup stalls here"
    );
    assert_eq!(
        result.nearest_peer_ids.len(),
        result.nearest_node_ids.len(),
        "the two neighbour arrays are index-aligned"
    );
    for peer_bytes in &result.nearest_peer_ids {
        PeerId::from_bytes(peer_bytes).expect("every returned peer id must parse");
    }

    rec.metric("nearest_peers", result.nearest_node_ids.len());
    node.shutdown().await;
    rec.finish(true);
}
