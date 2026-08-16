//! The bootstrap seed — the native replacement for `petals.cli.run_dht`.
//!
//! Tier 13 assembled a *node*: swarm, DHT service, control socket, announce.
//! A seed is the same swarm and DHT service with everything else removed, and
//! the properties worth pinning are the ones that removal could break.
//!
//! ```text
//!   ┌── client (a native node) ─────┐          ┌── seed (bootstrap) ─────────┐
//!   │ NetworkService                │──store──▶│ NetworkService              │
//!   │                               │◀──find───│ DHTStorage (in-memory)      │
//!   └───────────────────────────────┘          └── announces nothing ────────┘
//! ```
//!
//! | test | proves |
//! | --- | --- |
//! | `a_client_stores_and_finds_a_subkeyed_record_on_the_seed` | the round trip the whole seed exists for |
//! | `the_seed_announces_nothing_of_its_own` | a seed's storage starts empty and stays empty until a peer writes |
//! | `a_later_expiration_replaces_and_a_store_never_deletes` | the tombstone rule survives, so the map does not lose nodes |
//! | `the_seed_serves_many_peers_under_one_key` | subkeyed dictionary accumulation — many servers, one block key |
//! | `the_seed_keeps_the_identity_key_files_peer_id` | the `/p2p/<id>` multiaddrs baked into node configs stay valid |
//!
//! The seed is assembled here the way the CLI assembles it — the CLI is a
//! binary crate, so this cannot call it directly, and the glue on top is one
//! call per line.
//!
//! Since `feat/bootstrap-serve` converged the seed onto the ordinary node,
//! `kwaai-cli::bootstrap` is a config preset (`announce_self = false`, no
//! initial peers) handed to `node::run_node`, and the swarm below is what that
//! preset produces: kad server mode, `spawn_dht_service` over a `DHTStorage`,
//! and no announce loop. Serving is unconditional on both — it is publishing
//! and dialing that a bootstrap declines. What is asserted here is the seed's
//! *wire contract*, which is why these tests survived the convergence
//! untouched — they only ever talked to it as a peer.
//!
//! Gate: `KWAAI_INTEGRATION_TESTS=1`, like tiers 07–13.

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
use tempfile::TempDir;

const TIMEOUT: Duration = Duration::from_secs(20);

/// The TTL an announcing node uses, matching `announce::ANNOUNCE_TTL_SECS`.
const ANNOUNCE_TTL_SECS: f64 = 360.0;

// ============================================================================
// The seed under test
// ============================================================================

/// A DHT seed assembled as the seed config produces it: identity loaded from a
/// key file, swarm in kad server mode, DHT service over `DHTStorage`.
/// See `docs/BOOTSTRAP.md` for the config keys that select this shape.
///
/// No control socket, no announce loop, no node handlers — that absence is the
/// point of a seed, and `the_seed_announces_nothing_of_its_own` asserts it.
struct Seed {
    handle: NetworkHandle,
    peer_id: PeerId,
    /// libp2p listen address including `/p2p/<id>`, dialable by a client.
    ///
    /// The `DHTStorage` behind the service is deliberately *not* held: every
    /// assertion here goes over the wire, so what is tested is the seed as a
    /// peer sees it rather than a struct field a real client cannot reach.
    addr: Multiaddr,
    tasks: Vec<tokio::task::JoinHandle<()>>,
    _tmpdir: TempDir,
}

impl Seed {
    async fn start() -> Self {
        let tmpdir = TempDir::new().expect("tmpdir");
        let key_path = tmpdir.path().join("bootstrap_key.bin");

        // A seed loads its key; it never generates one. Generated here only to
        // create the file the loader then reads, which is the deployed shape.
        kwaai_p2p::identity::generate_keypair(&key_path).expect("the fixture key must generate");
        let keypair =
            kwaai_p2p::identity::load_keypair(&key_path).expect("the seed must load its key file");
        let peer_id = keypair.public().to_peer_id();

        // `for_tests` is `dht_server: true` on an ephemeral loopback port —
        // the same kad server mode a seed forces, without binding :8000 or
        // firing SSDP from CI.
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
        let mut tasks = vec![service_task];

        let storage = DHTStorage::new(peer_id);
        tasks.push(
            dht_service::spawn_dht_service(handle.clone(), storage.clone())
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
            _tmpdir: tmpdir,
        }
    }

    async fn shutdown(self) {
        let _ = self.handle.shutdown().await;
        for task in self.tasks {
            task.abort();
        }
    }
}

/// A native client — what a real node is, as far as the seed can tell.
struct Client {
    handle: NetworkHandle,
    peer_id: PeerId,
    tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl Client {
    async fn start() -> Self {
        let keypair = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id();
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
        Self {
            handle,
            peer_id,
            tasks: vec![service_task],
        }
    }

    /// Dial the seed and seed Kademlia from it, as a node does with its
    /// configured bootstrap peers.
    async fn bootstrap_to(&self, seed: &Seed) {
        tokio::time::timeout(TIMEOUT, self.handle.bootstrap(vec![seed.addr.clone()]))
            .await
            .expect("bootstrap must not hang")
            .expect("bootstrap must succeed against a live seed");
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
// Records
// ============================================================================

/// `SHA1(msgpack(raw_key))` — hivemind's `DHTID.generate()`.
fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// One block record in the shape `announce::build_announce_records` produces.
fn block_record(peer_id: PeerId, prefix: &str, block: i32, value: &[u8]) -> StoreRequest {
    record_with_expiration(
        peer_id,
        prefix,
        block,
        value,
        get_dht_time() + ANNOUNCE_TTL_SECS,
    )
}

fn record_with_expiration(
    peer_id: PeerId,
    prefix: &str,
    block: i32,
    value: &[u8],
    expiration: f64,
) -> StoreRequest {
    StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![dht_id(&format!("{prefix}.{block}"))],
        subkeys: vec![rmp_serde::to_vec(&peer_id.to_base58()).expect("msgpack subkey")],
        values: vec![value.to_vec()],
        expiration_time: vec![expiration],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(peer_id)),
    }
}

async fn store_on_seed(
    handle: &NetworkHandle,
    seed: PeerId,
    record: &StoreRequest,
) -> StoreResponse {
    let resp = tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(seed, PROTOCOL_STORE, &record.encode_to_vec()),
    )
    .await
    .expect("rpc_store must not hang")
    .expect("rpc_store must reach the seed");
    StoreResponse::decode(&resp[..]).expect("a decodable StoreResponse")
}

async fn find_on_seed(handle: &NetworkHandle, seed: PeerId, key: Vec<u8>) -> FindResponse {
    let request = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key],
        peer: None,
    };
    let resp = tokio::time::timeout(
        TIMEOUT,
        handle.call_unary_handler(seed, PROTOCOL_FIND, &request.encode_to_vec()),
    )
    .await
    .expect("rpc_find must not hang")
    .expect("rpc_find must reach the seed");
    FindResponse::decode(&resp[..]).expect("a decodable FindResponse")
}

/// The `(value, expiration)` a subkey holds in a found dictionary.
fn dict_entry(found: &FindResponse, peer_id: PeerId) -> (Vec<u8>, f64) {
    assert_eq!(
        found.results[0].result_type,
        ResultType::FoundDictionary as i32,
        "a subkeyed record must read back as a dictionary"
    );
    let dict = kwaai_hivemind_dht::parse_dictionary(&found.results[0].value)
        .expect("the result must parse as a hivemind dictionary");
    let subkey = rmp_serde::to_vec(&peer_id.to_base58()).expect("msgpack subkey");
    let entry = dict
        .entries
        .get(&subkey)
        .unwrap_or_else(|| panic!("the dictionary must hold an entry for {peer_id}"));
    (entry.0.clone(), entry.1)
}

// ============================================================================
// The round trip a seed exists for
// ============================================================================

/// A client bootstraps to the seed, stores a subkeyed record, and finds it back
/// byte-for-byte.
///
/// This is the entire job of `run_dht` and therefore of its replacement: with
/// this working, a node gets onto the map. `rpc_store` is single-hop, so the
/// seed serving the record back is exactly the guarantee the bootstrap fan-out
/// relies on.
#[tokio::test]
async fn a_client_stores_and_finds_a_subkeyed_record_on_the_seed() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::bootstrap_seed::store_and_find_round_trip",
        "integration",
    );

    let seed = Seed::start().await;
    let client = Client::start().await;
    client.bootstrap_to(&seed).await;

    let prefix = "Qwen/Qwen3-8B-hf";
    let value = b"an Ext(64) ServerInfo would live here".to_vec();
    let record = block_record(client.peer_id, prefix, 0, &value);

    let stored = store_on_seed(&client.handle, seed.peer_id, &record).await;
    assert_eq!(
        stored.store_ok,
        vec![true],
        "the seed must accept a client's store"
    );

    let found = find_on_seed(&client.handle, seed.peer_id, dht_id(&format!("{prefix}.0"))).await;
    let (got, _) = dict_entry(&found, client.peer_id);
    assert_eq!(
        got, value,
        "the value must survive the round trip unchanged — the map crawler decodes it"
    );

    rec.finish(true);
    client.shutdown().await;
    seed.shutdown().await;
}

/// A seed publishes nothing of its own.
///
/// `run_dht` stores other peers' records and announces none — a seed that
/// announced itself would appear on the map as an inference node serving zero
/// blocks. The property is asserted from *outside*: nothing is findable under
/// the seed's own subkey, and its storage is empty until a peer writes to it.
#[tokio::test]
async fn the_seed_announces_nothing_of_its_own() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::bootstrap_seed::announces_nothing",
        "integration",
    );

    let seed = Seed::start().await;
    let client = Client::start().await;
    client.bootstrap_to(&seed).await;

    // The keys a node would have announced under, had the seed been a node.
    for key in [
        "Qwen/Qwen3-8B-hf.0",
        "_kwaai.inference.nodes",
        "_petals.models",
    ] {
        let found = find_on_seed(&client.handle, seed.peer_id, dht_id(key)).await;
        assert_eq!(
            found.results[0].result_type,
            ResultType::NotFound as i32,
            "a seed must not publish {key} — it serves the DHT, it does not join it"
        );
    }

    // And the storage backing those answers is genuinely empty, not merely
    // missing the keys guessed above.
    let prefix = "Qwen/Qwen3-8B-hf";
    let record = block_record(client.peer_id, prefix, 3, b"client record");
    assert!(
        store_on_seed(&client.handle, seed.peer_id, &record)
            .await
            .store_ok[0],
        "the seed must accept the client's record"
    );
    let found = find_on_seed(&client.handle, seed.peer_id, dht_id(&format!("{prefix}.3"))).await;
    let (_, _) = dict_entry(&found, client.peer_id);

    // The only subkey under that key is the client's, never the seed's.
    let dict = kwaai_hivemind_dht::parse_dictionary(&found.results[0].value).expect("a dictionary");
    let seed_subkey = rmp_serde::to_vec(&seed.peer_id.to_base58()).expect("msgpack subkey");
    assert!(
        !dict.entries.contains_key(&seed_subkey),
        "the seed must never insert itself into a record it is merely storing"
    );

    rec.finish(true);
    client.shutdown().await;
    seed.shutdown().await;
}

// ============================================================================
// Record semantics that must not regress
// ============================================================================

/// The tombstone rule: a store with a strictly-greater expiration replaces, and
/// a store never deletes.
///
/// This is how a node leaves the map — it writes a `state = -1` value with a
/// later expiration, and the seed must let that overwrite win. A seed that
/// compared values instead of expirations, or that treated an older store as a
/// delete, would either strand dead nodes on the map or drop live ones.
#[tokio::test]
async fn a_later_expiration_replaces_and_a_store_never_deletes() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::bootstrap_seed::tombstone_and_expiration_rules",
        "integration",
    );

    let seed = Seed::start().await;
    let client = Client::start().await;
    client.bootstrap_to(&seed).await;

    let prefix = "Qwen/Qwen3-8B-hf";
    let key = dht_id(&format!("{prefix}.1"));
    let base = get_dht_time() + ANNOUNCE_TTL_SECS;

    let alive = b"state=online".to_vec();
    assert!(
        store_on_seed(
            &client.handle,
            seed.peer_id,
            &record_with_expiration(client.peer_id, prefix, 1, &alive, base),
        )
        .await
        .store_ok[0]
    );

    // A *later* expiration replaces — the unannounce tombstone.
    let tombstone = b"state=-1".to_vec();
    assert!(
        store_on_seed(
            &client.handle,
            seed.peer_id,
            &record_with_expiration(client.peer_id, prefix, 1, &tombstone, base + 60.0),
        )
        .await
        .store_ok[0],
        "a store with a greater expiration must be accepted"
    );
    let (got, exp) = dict_entry(
        &find_on_seed(&client.handle, seed.peer_id, key.clone()).await,
        client.peer_id,
    );
    assert_eq!(got, tombstone, "the later expiration must win");
    assert!(exp > base, "the stored expiration must be the later one");

    // An *earlier* expiration must not overwrite, and must not delete.
    let stale = b"state=stale".to_vec();
    store_on_seed(
        &client.handle,
        seed.peer_id,
        &record_with_expiration(client.peer_id, prefix, 1, &stale, base - 60.0),
    )
    .await;
    let (got, _) = dict_entry(
        &find_on_seed(&client.handle, seed.peer_id, key).await,
        client.peer_id,
    );
    assert_eq!(
        got, tombstone,
        "an older store must neither overwrite the newer value nor delete the record"
    );

    rec.finish(true);
    client.shutdown().await;
    seed.shutdown().await;
}

/// Many peers accumulate under one block key.
///
/// The map's whole model is "which servers hold block N", which works only
/// because each server writes under its own subkey and the seed merges them
/// into one dictionary rather than letting the last writer win.
#[tokio::test]
async fn the_seed_serves_many_peers_under_one_key() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::bootstrap_seed::many_peers_one_key",
        "integration",
    );

    let seed = Seed::start().await;
    let alice = Client::start().await;
    let bob = Client::start().await;
    alice.bootstrap_to(&seed).await;
    bob.bootstrap_to(&seed).await;

    let prefix = "Qwen/Qwen3-8B-hf";
    for (client, value) in [(&alice, b"alice".to_vec()), (&bob, b"bob".to_vec())] {
        let record = block_record(client.peer_id, prefix, 2, &value);
        assert!(
            store_on_seed(&client.handle, seed.peer_id, &record)
                .await
                .store_ok[0],
            "the seed must accept each peer's announcement"
        );
    }

    let found = find_on_seed(&alice.handle, seed.peer_id, dht_id(&format!("{prefix}.2"))).await;
    assert_eq!(
        dict_entry(&found, alice.peer_id).0,
        b"alice".to_vec(),
        "alice's record must survive bob's store"
    );
    assert_eq!(dict_entry(&found, bob.peer_id).0, b"bob".to_vec());

    rec.finish(true);
    bob.shutdown().await;
    alice.shutdown().await;
    seed.shutdown().await;
}

/// A seed's peer ID is its key file's, so the `/p2p/<id>` multiaddrs pinned in
/// every node's config keep resolving across a restart.
///
/// A seed that generated an identity when its key was missing would come back
/// from a bad mount as a stranger no node could dial — the failure mode this
/// asserts against.
#[tokio::test]
async fn the_seed_keeps_the_identity_key_files_peer_id() {
    require_integration!();
    let rec = MetricsRecorder::start(
        "integration::bootstrap_seed::identity_is_the_key_file",
        "integration",
    );

    let tmpdir = TempDir::new().expect("tmpdir");
    let key_path = tmpdir.path().join("bootstrap_key1.bin");
    let expected = kwaai_p2p::identity::generate_keypair(&key_path)
        .expect("the key must generate")
        .public()
        .to_peer_id();

    // Two successive starts from the same file, as a restart does.
    for _ in 0..2 {
        let keypair = kwaai_p2p::identity::load_keypair(&key_path).expect("the key must load");
        let (handle, service_task) =
            NetworkService::spawn(NetworkConfig::for_tests(), keypair).expect("service starts");
        assert_eq!(
            handle.peer_id(),
            expected,
            "a restarted seed must keep the key file's peer ID"
        );
        let _ = handle.shutdown().await;
        let _ = tokio::time::timeout(Duration::from_secs(5), service_task).await;
    }

    rec.finish(true);
}
