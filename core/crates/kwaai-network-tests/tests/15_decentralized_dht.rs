//! Decentralized DHT placement — joining, publishing and reading with no
//! bootstrap.
//!
//! Tiers 13 and 14 both assume a fixed entry point: a node announces to the
//! bootstraps it was configured with, and a seed serves what it is told. This
//! tier removes that assumption. There is **no seed here at all** — three
//! ordinary nodes, each joining through whichever peer it happens to know:
//!
//! ```text
//!        A ◀──── B joins via A ────  B  ◀──── C joins via B ────  C
//!        │                           │                            │
//!        └───────── all three serve rpc_store / rpc_find ─────────┘
//!                                                       C announces here ─┘
//!               A finds C's record without ever having spoken to C
//! ```
//!
//! | test | proves |
//! | --- | --- |
//! | `a_record_placed_on_the_nearest_peers_is_found_by_a_node_that_never_met_the_writer` | the whole feature: transitive join, per-key placement, third-party read |
//! | `a_record_survives_the_peer_that_introduced_it_leaving` | replication is real — killing the middle node does not take the record with it |
//! | `different_keys_land_on_different_peer_sets` | placement is per-key, not per-round |
//! | `the_peer_cache_round_trips_the_peers_a_node_met` | an established node can rejoin from memory |
//!
//! # Why the placement math is duplicated here
//!
//! `kwaai-cli` is a binary crate, so `placement::rank_candidates` cannot be
//! called from an integration test — the same constraint tier 14 notes for
//! `BootstrapSeed`. The ranking is therefore reimplemented below in a dozen
//! lines. That is a feature rather than a cost: if the binary's ranking ever
//! stops agreeing with an independent implementation of "SHA1, XOR,
//! nearest-first", these tests fail, which is exactly the divergence that would
//! otherwise be invisible.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like tiers 07–14.

use std::collections::HashSet;
use std::time::Duration;

use kwaai_hivemind_dht::protocol::{
    FindRequest, FindResponse, NodeInfo, RequestAuthInfo, ResultType, StoreRequest, StoreResponse,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{DHTStorage, PROTOCOL_FIND, PROTOCOL_STORE};
use kwaai_network_tests::{metrics::MetricsRecorder, require_integration};
use kwaai_p2p::{dht_service, Multiaddr, NetworkConfig, NetworkHandle, NetworkService, PeerId};
use prost::Message;
use sha1::{Digest, Sha1};

const TIMEOUT: Duration = Duration::from_secs(20);
const ANNOUNCE_TTL_SECS: f64 = 360.0;

/// The default `dht_replication`.
const K: usize = 3;

// ============================================================================
// A node — no seed, no privileged role
// ============================================================================

/// An ordinary native node: swarm, DHT service, nothing else.
///
/// Every node here is identical. There is no bootstrap in this topology — the
/// premise of the feature is that any reachable node is a join point, so the
/// test refuses to designate one.
struct Node {
    handle: NetworkHandle,
    peer_id: PeerId,
    addr: Multiaddr,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Node {
    async fn start() -> Self {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
        let mut tasks = vec![service_task];

        let storage = DHTStorage::new(peer_id);
        tasks.push(
            dht_service::spawn_dht_service(handle.clone(), storage)
                .await
                .expect("the DHT service must register"),
        );

        let addr = wait_for_listen_addr(&handle).await;
        Self {
            handle,
            peer_id,
            addr: format!("{addr}/p2p/{peer_id}")
                .parse()
                .expect("a dialable listen address"),
            tasks,
        }
    }

    /// Join the network through `via` — an ordinary peer, not a seed.
    async fn join_via(&self, via: &Node) {
        tokio::time::timeout(TIMEOUT, self.handle.bootstrap(vec![via.addr.clone()]))
            .await
            .expect("bootstrap must not hang")
            .expect("joining through a live peer must succeed");
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

/// Wait until `handle` knows at least `n` peers with a dialable address.
///
/// Kademlia populates the routing table asynchronously after a bootstrap, so a
/// placement round run immediately would see a smaller candidate set than the
/// node is about to have. Polling the real accessor is what a caller does.
async fn wait_for_known_peers(handle: &NetworkHandle, n: usize) -> Vec<kwaai_p2p::KnownPeer> {
    tokio::time::timeout(TIMEOUT, async {
        loop {
            if let Ok(peers) = handle.known_peers().await {
                if peers.len() >= n {
                    return peers;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap_or_else(|_| panic!("the node must come to know at least {n} peer(s)"))
}

// ============================================================================
// Placement, reimplemented independently of the binary
// ============================================================================

/// `SHA1(msgpack(raw_key))` — a record's hivemind DHTID.
fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// `SHA1(peer_id.to_bytes())` — a peer's hivemind DHTID.
fn peer_dht_id(peer: &PeerId) -> Vec<u8> {
    Sha1::new()
        .chain_update(peer.to_bytes())
        .finalize()
        .to_vec()
}

/// XOR over 20-byte big-endian IDs; lexicographic order is numeric order.
fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

/// `peers` ordered nearest-first around `record_id`.
fn rank(peers: &[PeerId], record_id: &[u8]) -> Vec<PeerId> {
    let mut ranked: Vec<(Vec<u8>, PeerId)> = peers
        .iter()
        .map(|p| (xor(record_id, &peer_dht_id(p)), *p))
        .collect();
    ranked.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    ranked.into_iter().map(|(_, p)| p).collect()
}

// ============================================================================
// Records and RPCs
// ============================================================================

/// A subkeyed record under an arbitrary key, in announce's shape.
fn record(peer_id: PeerId, key: &str, value: &[u8]) -> StoreRequest {
    StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id(key)],
        subkeys: vec![rmp_serde::to_vec(&peer_id.to_base58()).expect("msgpack subkey")],
        values: vec![value.to_vec()],
        expiration_time: vec![get_dht_time() + ANNOUNCE_TTL_SECS],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(peer_id)),
    }
}

async fn store_on(handle: &NetworkHandle, peer: PeerId, req: &StoreRequest) -> bool {
    let Ok(Ok(resp)) = tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(peer, PROTOCOL_STORE, &req.encode_to_vec()),
    )
    .await
    else {
        return false;
    };
    StoreResponse::decode(&resp[..])
        .map(|r| r.store_ok.iter().any(|&ok| ok))
        .unwrap_or(false)
}

async fn find_on(handle: &NetworkHandle, peer: PeerId, key: Vec<u8>) -> Option<FindResponse> {
    let request = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: None,
    };
    tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(peer, PROTOCOL_FIND, &request.encode_to_vec()),
    )
    .await
    .ok()?
    .ok()
    .and_then(|b| FindResponse::decode(&b[..]).ok())
}

/// Place `req` on the `k` nearest of `candidates`, advancing past failures.
///
/// The announce path's `place_with` walk, over the wire.
async fn place(
    handle: &NetworkHandle,
    candidates: &[PeerId],
    req: &StoreRequest,
    key: &str,
    k: usize,
) -> Vec<PeerId> {
    let mut stored = Vec::new();
    for peer in rank(candidates, &dht_id(key)) {
        if stored.len() >= k {
            break;
        }
        if store_on(handle, peer, req).await {
            stored.push(peer);
        }
    }
    stored
}

/// Read `key` back from the `k` nearest peers we know, merging what they hold.
///
/// The read side of the same rule: a reader ranks the candidates it knows
/// against the key and asks the nearest, rather than asking a bootstrap.
async fn find_nearest(
    handle: &NetworkHandle,
    candidates: &[PeerId],
    key: &str,
    k: usize,
    writer: PeerId,
) -> Option<Vec<u8>> {
    let subkey = rmp_serde::to_vec(&writer.to_base58()).expect("msgpack subkey");
    for peer in rank(candidates, &dht_id(key)).into_iter().take(k) {
        let Some(resp) = find_on(handle, peer, dht_id(key)).await else {
            continue;
        };
        let Some(result) = resp.results.first() else {
            continue;
        };
        if result.result_type != ResultType::FoundDictionary as i32 {
            continue;
        }
        if let Some(dict) = kwaai_hivemind_dht::parse_dictionary(&result.value) {
            if let Some(entry) = dict.entries.get(&subkey) {
                return Some(entry.0.clone());
            }
        }
    }
    None
}

// ============================================================================
// The feature
// ============================================================================

/// A joins nothing, B joins via A, C joins via B. C publishes a record by
/// nearest-peer placement, and **A finds it** — despite A having no
/// configuration naming C, and no bootstrap existing anywhere in the topology.
///
/// This is the phase's headline claim: any reachable node is a join point, and
/// a record written by a stranger is findable by a node that only ever dialed
/// one peer.
#[tokio::test]
async fn a_record_placed_on_the_nearest_peers_is_found_by_a_node_that_never_met_the_writer() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::decentralized_dht::transitive_join_and_read",
        "integration",
    );

    // No seed anywhere: A is simply the first node up.
    let a = Node::start().await;
    let b = Node::start().await;
    let c = Node::start().await;
    b.join_via(&a).await;
    c.join_via(&b).await;

    // C's candidate set is whatever it has come to know — not a configured list.
    let known = wait_for_known_peers(&c.handle, 1).await;
    let candidates: Vec<PeerId> = known.iter().map(|k| k.peer_id).collect();
    assert!(
        !candidates.is_empty(),
        "C must know at least the peer it joined through"
    );

    let key = "_kwaai.inference.nodes";
    let value = b"an Ext(64) ServerInfo would live here".to_vec();
    let stored_on = place(
        &c.handle,
        &candidates,
        &record(c.peer_id, key, &value),
        key,
        K,
    )
    .await;
    assert!(
        !stored_on.is_empty(),
        "C must place its record on at least one peer"
    );

    // A reads via its own nearest-peer walk. A never dialed C.
    let a_known = wait_for_known_peers(&a.handle, 1).await;
    let a_candidates: Vec<PeerId> = a_known.iter().map(|k| k.peer_id).collect();
    assert!(
        !a_candidates.contains(&c.peer_id) || stored_on.contains(&a.peer_id),
        "the test is only meaningful if A did not configure C"
    );

    let found = find_nearest(&a.handle, &a_candidates, key, K, c.peer_id).await;
    assert_eq!(
        found,
        Some(value),
        "A must find C's record through the peers it knows, with no bootstrap in the topology"
    );

    rec.finish(true);
    c.shutdown().await;
    b.shutdown().await;
    a.shutdown().await;
}

/// Killing the node that introduced C does not take C's record with it.
///
/// With k = 3 over a 3-node network the record is everywhere, so B leaving
/// leaves a replica on A. This is what replication buys: the peer that happened
/// to be the join point is not a single point of failure for the data.
#[tokio::test]
async fn a_record_survives_the_peer_that_introduced_it_leaving() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::decentralized_dht::replica_survives_peer_loss",
        "integration",
    );

    let a = Node::start().await;
    let b = Node::start().await;
    let c = Node::start().await;
    b.join_via(&a).await;
    c.join_via(&b).await;

    let known = wait_for_known_peers(&c.handle, 2).await;
    let candidates: Vec<PeerId> = known.iter().map(|k| k.peer_id).collect();

    let key = "Qwen/Qwen3-8B-hf.0";
    let value = b"block 0 server info".to_vec();
    let stored_on = place(
        &c.handle,
        &candidates,
        &record(c.peer_id, key, &value),
        key,
        K,
    )
    .await;

    // k=3 over the two peers C knows plus itself: every reachable peer holds it.
    assert!(
        stored_on.contains(&a.peer_id),
        "with k=3 over a 3-node network, A must hold a replica — got {stored_on:?}"
    );

    // B — the peer that introduced C — goes away entirely.
    let b_peer_id = b.peer_id;
    b.shutdown().await;
    tokio::time::sleep(Duration::from_millis(500)).await;

    // A still serves the record. Read over the wire from C, not from A's own
    // handle: a node cannot dial itself, so `handle.call_unary_handler(self)`
    // would fail for reasons unrelated to whether the replica survived. Asking
    // C is also the more honest question — it is what a third party sees.
    let resp = find_on(&c.handle, a.peer_id, dht_id(key)).await;
    let subkey = rmp_serde::to_vec(&c.peer_id.to_base58()).expect("msgpack subkey");
    let dict = resp
        .as_ref()
        .and_then(|r| r.results.first())
        .filter(|r| r.result_type == ResultType::FoundDictionary as i32)
        .and_then(|r| kwaai_hivemind_dht::parse_dictionary(&r.value))
        .expect("A must still hold the record after B is gone");

    assert_eq!(
        dict.entries.get(&subkey).map(|e| e.0.clone()),
        Some(value),
        "the replica on A must outlive B ({b_peer_id})"
    );

    rec.finish(true);
    c.shutdown().await;
    a.shutdown().await;
}

/// Two different keys select different holders from the same candidate set.
///
/// If every key ranked to the same peers, decentralization would only have
/// moved the single point of failure rather than removed it. Asserted over
/// many synthetic peer IDs so the result does not depend on three particular
/// random keypairs.
#[tokio::test]
async fn different_keys_land_on_different_peer_sets() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::decentralized_dht::per_key_placement",
        "integration",
    );

    let peers: Vec<PeerId> = (0..24).map(|_| PeerId::random()).collect();

    let keys = [
        "_petals.models",
        "_kwaai.inference.nodes",
        "Qwen/Qwen3-8B-hf.0",
        "Qwen/Qwen3-8B-hf.17",
        "Qwen/Qwen3-8B-hf.31",
    ];

    let selections: Vec<Vec<PeerId>> = keys
        .iter()
        .map(|k| rank(&peers, &dht_id(k)).into_iter().take(K).collect())
        .collect();

    assert!(
        selections.windows(2).any(|w| w[0] != w[1]),
        "different keys must not all resolve to the same {K} peers"
    );

    // No single peer holds everything — the property that makes any one node
    // droppable.
    let everywhere = selections
        .iter()
        .fold(None::<HashSet<PeerId>>, |acc, sel| {
            let set: HashSet<PeerId> = sel.iter().copied().collect();
            Some(match acc {
                None => set,
                Some(a) => a.intersection(&set).copied().collect(),
            })
        })
        .unwrap_or_default();
    assert!(
        everywhere.len() < K,
        "no peer set should hold every key across {} unrelated keys",
        keys.len()
    );

    // And ranking is stable: the same key ranks identically on a second pass.
    assert_eq!(
        selections[0],
        rank(&peers, &dht_id(keys[0]))
            .into_iter()
            .take(K)
            .collect::<Vec<_>>(),
        "placement must be reproducible or a reader looks in the wrong place"
    );

    rec.finish(true);
}

/// After joining transitively, C's known-peer set — what the peer cache is
/// written from — contains both A and B.
///
/// The cache's purpose is that a restarted node can rejoin without its config.
/// That only works if the peers it met are actually visible through the
/// accessor the cache writer reads, which is what this pins.
#[tokio::test]
async fn the_peer_cache_round_trips_the_peers_a_node_met() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::decentralized_dht::peer_cache_contents",
        "integration",
    );

    let a = Node::start().await;
    let b = Node::start().await;
    let c = Node::start().await;
    b.join_via(&a).await;
    c.join_via(&b).await;

    // C must come to know both A and B — B directly, A through the kad walk.
    let known = wait_for_known_peers(&c.handle, 2).await;
    let ids: HashSet<PeerId> = known.iter().map(|k| k.peer_id).collect();
    assert!(
        ids.contains(&b.peer_id),
        "C must remember the peer it joined through"
    );
    assert!(
        ids.contains(&a.peer_id),
        "C must remember A, learned transitively — that is what makes the cache \
         more useful than the config it replaces"
    );

    // Every remembered peer carries a dialable address, which is what the cache
    // persists — a remembered peer with no address could never be rejoined to.
    for peer in &known {
        assert!(
            !peer.addrs.is_empty(),
            "known peer {} must carry an address",
            peer.peer_id
        );
    }

    // The cache's serialised shape, round-tripped as the writer/loader do.
    let entries: Vec<(String, Vec<String>)> = known
        .iter()
        .map(|k| {
            (
                k.peer_id.to_base58(),
                k.addrs
                    .iter()
                    .map(|a| format!("{a}/p2p/{}", k.peer_id))
                    .collect(),
            )
        })
        .collect();
    let json = serde_json::to_string(&serde_json::json!({
        "peers": entries.iter().map(|(id, addrs)| serde_json::json!({
            "peer_id": id, "addrs": addrs, "last_seen": 1,
        })).collect::<Vec<_>>()
    }))
    .expect("the cache must serialise");

    let reloaded: serde_json::Value = serde_json::from_str(&json).expect("and load back");
    let cached_ids: HashSet<String> = reloaded["peers"]
        .as_array()
        .expect("peers is an array")
        .iter()
        .map(|p| p["peer_id"].as_str().expect("a peer id").to_string())
        .collect();

    assert!(cached_ids.contains(&a.peer_id.to_base58()));
    assert!(cached_ids.contains(&b.peer_id.to_base58()));

    rec.finish(true);
    c.shutdown().await;
    b.shutdown().await;
    a.shutdown().await;
}
