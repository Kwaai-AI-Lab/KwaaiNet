//! The assembled native node — the Phase 4 `run_node` gate.
//!
//! Tiers 09–12 each proved one component in isolation: a `NetworkService`
//! interoperates over the libp2p wire (09), a `DHTStorage` behind unary handlers
//! serves a Go caller (10), a `ControlServer` is indistinguishable from a p2pd
//! to a `P2PClient` (11), and pipe mode relays bytes across both (12). None of
//! them assembles those parts into a *node*, which is what `native_p2p = true`
//! now makes `run_node` do.
//!
//! This tier assembles them the way `node_native::NativeNode::start` does and
//! asserts the two properties a node has that its parts do not:
//!
//! ```text
//!   ┌── node B (announcer) ─────────┐          ┌── node A (bootstrap) ───────┐
//!   │ NetworkService + DHT service  │──store──▶│ NetworkService + DHT service│
//!   │ ControlServer ◀── P2PClient   │◀──find───│ ControlServer               │
//!   └───────────────────────────────┘          └─────────────────────────────┘
//! ```
//!
//! | test | proves |
//! | --- | --- |
//! | `announced_records_are_findable_on_the_bootstrap` | B bootstraps to A, announces, and A serves the records back |
//! | `an_external_client_drives_the_assembled_node` | a `P2PClient` on the node's own control socket does identify + a unary round trip |
//! | `the_control_socket_reaches_the_swarm_the_node_is_running` | the socket and the swarm are the *same* node, not two that happen to coexist |
//! | `a_node_serves_its_own_announced_records` | the local-store half of announce, which is what makes a node a DHT replica of itself |
//! | `the_peer_id_is_the_identity_key_file_not_a_fresh_one` | the migrated node keeps its DID, credentials and map entry |
//!
//! # Why no `kwaainet run-node` spawn test
//!
//! Deliberately omitted. A spawned binary writes the real `~/.kwaainet` PID
//! file, binds the real control socket, starts the gRPC surface on a fixed
//! port, and probes Ollama and the update endpoint — all process-global state
//! that collides with a node running on the developer's machine and with a
//! second copy of the test. The properties a spawn test would add over this one
//! are argument parsing and the config round trip, both already unit-tested,
//! against a large increase in flakiness. The components are assembled here
//! exactly as `NativeNode::start` assembles them, in the same order.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like tiers 07–12.
//!
//! # Process hygiene
//!
//! No p2pd is spawned and none is required — the point of the native path is
//! that the binary need not exist. Every swarm listens on an ephemeral loopback
//! port with a freshly generated key, and every control socket lives in its own
//! tmpdir, so nothing here can collide with a node running on the same machine.

use std::time::Duration;

use kwaai_hivemind_dht::protocol::{
    FindRequest, FindResponse, NodeInfo, RequestAuthInfo, ResultType, StoreRequest, StoreResponse,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{DHTStorage, PROTOCOL_FIND, PROTOCOL_STORE};
use kwaai_network_tests::{metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{dht_service, Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use kwaai_p2p_daemon::{ControlServer, P2PClient};
use prost::Message;
use sha1::{Digest, Sha1};
use tempfile::TempDir;

/// Cap on any single interaction, so a regression fails rather than hanging.
const TIMEOUT: Duration = Duration::from_secs(20);

/// The TTL every announced record carries, matching `announce::ANNOUNCE_TTL_SECS`.
const ANNOUNCE_TTL_SECS: f64 = 360.0;

// ============================================================================
// The node under test
// ============================================================================

/// A native node assembled exactly as `node_native::NativeNode::start` does:
/// identity from a key file, swarm, DHT service, control socket.
///
/// The CLI is a binary crate, so this cannot call `NativeNode::start` itself.
/// What it can do is compose the same public API in the same order, which is
/// the part that could break — the CLI glue on top is one call per line.
struct AssembledNode {
    handle: NetworkHandle,
    peer_id: PeerId,
    storage: DHTStorage,
    /// Control-socket multiaddr, for `P2PClient::connect`.
    socket: String,
    /// libp2p listen address including `/p2p/<id>`, dialable by another node.
    addr: Multiaddr,
    tasks: Vec<tokio::task::JoinHandle<()>>,
    _tmpdir: TempDir,
}

impl AssembledNode {
    /// Start a node whose identity is read from a key file in its own tmpdir,
    /// the way the CLI reads `~/.kwaainet/identity.key`.
    async fn start() -> Self {
        let tmpdir = TempDir::new().expect("tmpdir");
        let key_path = tmpdir.path().join("identity.key");
        let keypair = kwaai_p2p::identity::load_or_generate(&key_path)
            .expect("the identity key file must load or generate");
        let peer_id = keypair.public().to_peer_id();

        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
        let mut tasks = vec![service_task];

        // DHT serving — bootstrap-grade, as `-b` gave the p2pd path.
        let storage = DHTStorage::new(peer_id);
        tasks.push(
            dht_service::spawn_dht_service(handle.clone(), storage.clone())
                .await
                .expect("the DHT service must register"),
        );

        let addr = wait_for_listen_addr(&handle).await;

        // Control socket last, so an external client never sees a node whose
        // handlers are only half-registered.
        let socket = format!("/unix/{}", tmpdir.path().join("kwaai.sock").display());
        let server = ControlServer::bind(&socket, handle.clone())
            .await
            .expect("the control socket must bind");
        tasks.push(tokio::spawn(server.run()));

        Self {
            handle,
            peer_id,
            storage,
            socket,
            addr: format!("{addr}/p2p/{peer_id}")
                .parse()
                .expect("a dialable listen address"),
            tasks,
            _tmpdir: tmpdir,
        }
    }

    /// Dial `other` and seed Kademlia from it, as `NativeNode::start` does with
    /// the configured bootstrap peers.
    async fn bootstrap_to(&self, other: &AssembledNode) {
        tokio::time::timeout(TIMEOUT, self.handle.bootstrap(vec![other.addr.clone()]))
            .await
            .expect("bootstrap must not hang")
            .expect("bootstrap must succeed against a live peer");
    }

    /// An external process's view of this node.
    async fn external_client(&self) -> P2PClient {
        P2PClient::connect(&self.socket)
            .await
            .expect("a P2PClient must connect to the assembled node's control socket")
    }

    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        for task in self.tasks {
            task.abort();
        }
    }
}

async fn wait_for_listen_addr(handle: &NetworkHandle) -> Multiaddr {
    tokio::time::timeout(TIMEOUT, async {
        loop {
            if let Some(a) = handle
                .listen_addrs()
                .await
                .ok()
                .and_then(|a| a.into_iter().next())
            {
                return a;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("the swarm must report a listen address")
}

// ============================================================================
// Announce records
// ============================================================================

/// `SHA1(msgpack(raw_key))` — hivemind's `DHTID.generate()`, as `announce::dht_id`.
fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// One block record in the shape `announce::build_announce_records` produces:
/// a real subkey (`msgpack(peer_b58)`, never a sentinel), a 360 s TTL, and
/// `in_cache = false`.
fn block_record(peer_id: PeerId, prefix: &str, block: i32, value: &[u8]) -> StoreRequest {
    StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id(&format!("{prefix}.{block}"))],
        subkeys: vec![rmp_serde::to_vec(&peer_id.to_base58()).expect("msgpack subkey")],
        values: vec![value.to_vec()],
        expiration_time: vec![get_dht_time() + ANNOUNCE_TTL_SECS],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(peer_id)),
    }
}

/// Push a record to `peer` over the handle, as `send_records_via_handle` does.
async fn store_via_handle(
    handle: &NetworkHandle,
    peer: PeerId,
    record: &StoreRequest,
) -> StoreResponse {
    let bytes = record.encode_to_vec();
    let resp = tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(peer, PROTOCOL_STORE, &bytes),
    )
    .await
    .expect("rpc_store must not hang")
    .expect("rpc_store must reach the peer");
    StoreResponse::decode(&resp[..]).expect("a decodable StoreResponse")
}

/// Look a key up on `peer` over the handle.
async fn find_via_handle(handle: &NetworkHandle, peer: PeerId, key: Vec<u8>) -> FindResponse {
    let request = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: None,
    };
    let resp = tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(peer, PROTOCOL_FIND, &request.encode_to_vec()),
    )
    .await
    .expect("rpc_find must not hang")
    .expect("rpc_find must reach the peer");
    FindResponse::decode(&resp[..]).expect("a decodable FindResponse")
}

// ============================================================================
// Two nodes: announce and find
// ============================================================================

/// The end-to-end announce path between two native nodes.
///
/// Node B bootstraps to node A, announces a block record over its
/// `NetworkHandle`, and A — running nothing but the same assembled stack —
/// stores it and serves it back on `rpc_find` with the value byte-identical.
///
/// This is the property the whole migration exists to preserve: with the flag
/// on, a node still gets onto the map. `rpc_store` is single-hop (verified live
/// in Phase 2), so A serving the record back is exactly the guarantee the
/// bootstrap fan-out relies on.
#[tokio::test]
async fn announced_records_are_findable_on_the_bootstrap() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::native_node::announce_is_findable_on_the_bootstrap",
        "integration",
    );

    let bootstrap = AssembledNode::start().await;
    let announcer = AssembledNode::start().await;
    announcer.bootstrap_to(&bootstrap).await;

    let prefix = "Qwen/Qwen3-8B-hf";
    let value = b"an Ext(64) ServerInfo would live here".to_vec();
    let record = block_record(announcer.peer_id, prefix, 0, &value);

    let stored = store_via_handle(&announcer.handle, bootstrap.peer_id, &record).await;
    assert_eq!(
        stored.store_ok,
        vec![true],
        "the bootstrap must accept the announce"
    );

    let found = find_via_handle(
        &announcer.handle,
        bootstrap.peer_id,
        dht_id(&format!("{prefix}.0")),
    )
    .await;
    assert_eq!(found.results.len(), 1);
    assert_eq!(
        found.results[0].result_type,
        ResultType::FoundDictionary as i32,
        "a subkeyed record reads back as a dictionary, so many servers accumulate under one key"
    );

    // The value must survive the round trip byte-for-byte — the map crawler
    // decodes it, so any reshaping here takes nodes off the map.
    let dict = kwaai_hivemind_dht::parse_dictionary(&found.results[0].value)
        .expect("the result must parse as a hivemind dictionary");
    let subkey = rmp_serde::to_vec(&announcer.peer_id.to_base58()).unwrap();
    assert_eq!(
        dict.entries[&subkey].0, value,
        "the announced value must come back unchanged"
    );

    rec.finish(true);
    announcer.shutdown().await;
    bootstrap.shutdown().await;
}

/// A node stores what it announces into its own storage, so it serves as a
/// replica of its own records rather than depending entirely on the bootstraps.
///
/// `NativeNode::announce` does this before the network round trip, mirroring the
/// p2pd path's local `handle_store`. Asserted through a *remote* `rpc_find` from
/// the other node, so this tests the serving path and not just the storage API.
#[tokio::test]
async fn a_node_serves_its_own_announced_records() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::native_node::serves_its_own_records",
        "integration",
    );

    let announcer = AssembledNode::start().await;
    let reader = AssembledNode::start().await;
    reader.bootstrap_to(&announcer).await;

    let prefix = "Qwen/Qwen3-8B-hf";
    let value = b"locally stored ServerInfo".to_vec();
    let record = block_record(announcer.peer_id, prefix, 7, &value);

    // The local half of announce: store into our own storage.
    assert!(
        announcer.storage.handle_store(record.clone()).store_ok[0],
        "a node must accept its own announcement"
    );

    let found = find_via_handle(
        &reader.handle,
        announcer.peer_id,
        dht_id(&format!("{prefix}.7")),
    )
    .await;
    assert_eq!(
        found.results[0].result_type,
        ResultType::FoundDictionary as i32,
        "the announcing node must serve its own record to a remote peer"
    );

    rec.finish(true);
    reader.shutdown().await;
    announcer.shutdown().await;
}

// ============================================================================
// The control socket on the assembled node
// ============================================================================

/// An unchanged external client drives the node `run_node` assembled.
///
/// Tier 11 proved a standalone `ControlServer` is indistinguishable from a
/// p2pd. This proves the one `run_node` binds is too — that adding the DHT
/// service and the node's own handlers underneath it did not disturb the socket
/// the GUI, `kwaainet p2p …` and `shard serve` all dial.
///
/// Identify plus a unary round trip is the minimum every client does at
/// startup, and the round trip crosses both boundaries: registered over one
/// node's socket, called over the other's, answered across the libp2p wire.
#[tokio::test]
async fn an_external_client_drives_the_assembled_node() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::native_node::external_client_round_trip",
        "integration",
    );

    let responder = AssembledNode::start().await;
    let caller = AssembledNode::start().await;
    caller.bootstrap_to(&responder).await;

    // ── identify ────────────────────────────────────────────────────────────
    let mut responder_client = responder.external_client().await;
    let reported = responder_client
        .identify()
        .await
        .expect("identify must succeed against the assembled node");
    let reported =
        PeerId::from_bytes(&hex::decode(&reported).expect("hex peer id")).expect("a valid peer id");
    assert_eq!(
        reported, responder.peer_id,
        "the control socket must report the swarm's own peer ID, not a second identity"
    );

    // ── unary round trip across both boundaries ─────────────────────────────
    const PROTO: &str = "/kwaai/p2p/hello/1.0.0";
    responder_client
        .add_unary_handler(
            PROTO,
            |req: Vec<u8>| async move {
                let mut out = b"assembled:".to_vec();
                out.extend_from_slice(&req);
                Ok(out)
            },
            false,
        )
        .await
        .expect("an external client must be able to register a handler");

    let caller_client = caller.external_client().await;
    let reply = tokio::time::timeout(
        TIMEOUT,
        caller_client.call_unary_handler(&responder.peer_id.to_bytes(), PROTO, b"ping"),
    )
    .await
    .expect("the call must not hang")
    .expect("the call must reach the handler");
    assert_eq!(
        reply, b"assembled:ping",
        "the handler an external client registered must answer a remote caller"
    );

    rec.finish(true);
    caller.shutdown().await;
    responder.shutdown().await;
}

/// The control socket and the swarm are one node, not two that coexist.
///
/// The failure this guards against is real and silent: a `ControlServer` bound
/// to a *different* `NetworkHandle` than the one serving the DHT would pass
/// every tier-11 test and still be a broken node — `kwaainet p2p peers list`
/// would show peers the DHT service could not reach. Asserted by dialing
/// through the socket and observing the result on the swarm's own peer list.
#[tokio::test]
async fn the_control_socket_reaches_the_swarm_the_node_is_running() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::native_node::socket_and_swarm_are_one_node",
        "integration",
    );

    let node = AssembledNode::start().await;
    let other = AssembledNode::start().await;

    let mut client = node.external_client().await;
    tokio::time::timeout(TIMEOUT, client.connect_peer(&other.addr.to_string()))
        .await
        .expect("connect must not hang")
        .expect("the control socket must dial a reachable peer");

    let peers = node
        .handle
        .list_peers()
        .await
        .expect("the swarm must list its peers");
    assert!(
        peers.iter().any(|p| p.peer_id == other.peer_id),
        "a dial made through the control socket must appear on the swarm the node is running; \
         got {peers:?}"
    );

    rec.finish(true);
    other.shutdown().await;
    node.shutdown().await;
}

/// The node's peer ID comes from its identity key file, so a node that migrates
/// to the native path keeps its DID, its credentials and its map entry.
///
/// The on-disk format is the bare libp2p protobuf encoding that
/// `kwaai-cli::NodeIdentity` writes and that p2pd accepts via `-id`, so the
/// same file yields the same peer ID on either path. A regression here would
/// silently mint a new identity on upgrade — records under the old subkey would
/// age out and every issued credential would be orphaned.
#[tokio::test]
async fn the_peer_id_is_the_identity_key_file_not_a_fresh_one() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::native_node::peer_id_comes_from_the_key_file",
        "integration",
    );

    let tmpdir = TempDir::new().expect("tmpdir");
    let key_path = tmpdir.path().join("identity.key");

    // What the p2pd path would have had: the peer ID of the key on disk.
    let expected = kwaai_p2p::identity::load_or_generate(&key_path)
        .expect("key generates")
        .public()
        .to_peer_id();

    // Starting a node from the same file must reproduce it, and the control
    // socket must report it — the peer ID external clients see.
    let keypair = kwaai_p2p::identity::load_keypair(&key_path).expect("key loads");
    let (handle, service_task) =
        NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
    assert_eq!(
        handle.peer_id(),
        expected,
        "the swarm must adopt the key file's identity, not generate a fresh one"
    );

    let socket = format!("/unix/{}", tmpdir.path().join("kwaai.sock").display());
    let server = ControlServer::bind(&socket, handle.clone())
        .await
        .expect("the control socket must bind");
    let server_task = tokio::spawn(server.run());

    let mut client = P2PClient::connect(&socket).await.expect("client connects");
    let reported = PeerId::from_bytes(
        &hex::decode(client.identify().await.expect("identify")).expect("hex peer id"),
    )
    .expect("a valid peer id");
    assert_eq!(
        reported, expected,
        "external clients must see the key file's peer ID"
    );

    rec.finish(true);
    let _ = handle.shutdown().await;
    server_task.abort();
    let _ = tokio::time::timeout(Duration::from_secs(5), service_task).await;
}
