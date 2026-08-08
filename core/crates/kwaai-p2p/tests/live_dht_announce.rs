//! Live DHT write test against the production KwaaiNet bootstraps.
//!
//! **This test writes to the real DHT.** It is therefore gated twice:
//! `#[ignore]` *and* `KWAAI_LIVE_DHT_WRITE=1`. The `live_bootstrap.rs`
//! precedent is that `--ignored` alone means read-only, so a second gate is
//! what separates "hit the network" from "modify the network".
//!
//! ```sh
//! KWAAI_LIVE_DHT_WRITE=1 \
//!   cargo test -p kwaai-p2p --test live_dht_announce -- --ignored --nocapture
//! ```
//!
//! What it proves that no in-process or p2pd-in-the-loop test can: the Python
//! Hivemind bootstraps — RSA-2048 `Qm…` identities running upstream petals
//! `run_dht` — accept a store from our native rust stack, serve it back as a
//! dictionary, and honour an update under the same subkey. That is the full
//! announce lifecycle minus the record payloads.
//!
//! # `rpc_store` is single-hop
//!
//! Verified live: a record stored on bootstrap #1 is **not** visible on
//! bootstrap #2. `rpc_store` writes to the receiving node and replicates
//! nothing; hivemind's `DHTNode.store` gets redundancy by calling `rpc_store`
//! on each of the `k` nearest nodes from the client side. That is exactly why
//! `announce::send_records_via_handle` fans out to every bootstrap rather than
//! storing once and trusting propagation. Step 2b asserts the negative so this
//! stays documented rather than being rediscovered as a bug.
//!
//! # Why this is safe to run against production
//!
//! * **Unreachable namespace.** The key is
//!   `_kwaai.test.ephemeral.{our_peer_b58}`, unique per run. The health-map
//!   crawler cannot reach it: it is not in `_petals.models`, not in the
//!   crawler's `FALLBACK_PREFIXES`, and not `_kwaai.vpk.nodes`. Nothing walks
//!   it, so nothing can display it.
//! * **Wrong shape for a crawler anyway.** The value is a plain msgpack map,
//!   deliberately *not* `Ext(64)`, so even a hypothetical crawl would reject it
//!   rather than render a phantom node.
//! * **Self-cleaning.** The TTL is 60 s, not the 360 s a real announce uses, so
//!   the records age out within a minute of the test finishing.
//! * **Ephemeral identity.** A freshly generated Ed25519 key, never touching
//!   `~/.kwaainet`, so the peer ID in the key is meaningless after the run.
//!
//! # The expiration rule this test exists to pin
//!
//! Hivemind storage rejects a store whose expiration is **not strictly greater**
//! than the record it replaces, and rejects an already-expired store outright.
//! So the "unannounce" step here writes a tombstone with an expiration *later*
//! than the original — a shorter or past one is silently refused, which is the
//! thing implementers get backwards. And a store can never delete: the assertion
//! is that the value **changed**, never that the key vanished.

use std::time::{Duration, Instant};

use kwaai_hivemind_dht::protocol::{
    FindRequest, FindResponse, NodeInfo, RequestAuthInfo, ResultType, StoreRequest, StoreResponse,
};
use kwaai_hivemind_dht::value::get_dht_time;
use kwaai_hivemind_dht::{PROTOCOL_FIND, PROTOCOL_STORE};
use kwaai_p2p::{
    NetworkConfig, NetworkHandle, NetworkService, PeerId, KWAAI_BOOTSTRAP_SERVERS_DNS,
};
use libp2p::identity::Keypair;
use prost::Message;
use sha1::{Digest, Sha1};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(30);
const CALL_TIMEOUT: Duration = Duration::from_secs(30);

/// Deliberately short: these records self-clean about a minute after the run.
/// A real announce uses 360 s.
const TEST_TTL_SECS: f64 = 60.0;

/// SHA1(msgpack(key)) — hivemind's `DHTID.generate()`. Duplicated from
/// `kwaainet`'s `announce::dht_id` rather than depended on, because this crate
/// must not take a dependency on the CLI binary.
fn dht_id(raw_key: &str) -> Vec<u8> {
    let packed = rmp_serde::to_vec(raw_key).expect("msgpack key");
    Sha1::new().chain_update(&packed).finalize().to_vec()
}

/// A plain msgpack map — **not** `Ext(64)`, so no crawler can mistake it for a
/// ServerInfo record.
fn test_value(note: &str) -> Vec<u8> {
    let map = rmpv::Value::Map(vec![
        (rmpv::Value::from("test"), rmpv::Value::Boolean(true)),
        (rmpv::Value::from("note"), rmpv::Value::from(note)),
    ]);
    let mut buf = Vec::new();
    rmpv::encode::write_value(&mut buf, &map).expect("encode test value");
    buf
}

/// The peer ID out of a `/dns/…/p2p/<id>` bootstrap address.
fn bootstrap_peer_id(addr: &str) -> PeerId {
    addr.split("/p2p/")
        .nth(1)
        .expect("a bootstrap address carries a /p2p/ component")
        .parse()
        .expect("a valid bootstrap peer id")
}

/// Issue one unary call, returning its bytes and how long it took.
async fn timed_call(
    handle: &NetworkHandle,
    peer: PeerId,
    proto: &str,
    payload: Vec<u8>,
    what: &str,
) -> (Vec<u8>, Duration) {
    let t0 = Instant::now();
    let result = tokio::time::timeout(
        CALL_TIMEOUT,
        handle.call_unary_handler(peer, proto, &payload),
    )
    .await;
    let elapsed = t0.elapsed();

    match result {
        Ok(Ok(bytes)) => {
            println!("  {what}: {} bytes in {elapsed:?}", bytes.len());
            (bytes, elapsed)
        }
        // Surface the exact error — a rejection here is the critical migration
        // finding, not a test bug, so it must not be summarized away.
        Ok(Err(e)) => panic!("{what} FAILED against {peer}\n  error: {e}"),
        Err(_) => panic!("{what} timed out after {CALL_TIMEOUT:?} against {peer}"),
    }
}

/// Read a key back and return the dictionary entry stored under `subkey`, if
/// any, alongside the raw result type.
fn dictionary_entry(response: &FindResponse, subkey: &[u8]) -> (i32, Option<Vec<u8>>) {
    let result = &response.results[0];
    if result.result_type != ResultType::FoundDictionary as i32 {
        return (result.result_type, None);
    }
    let dict = kwaai_hivemind_dht::parse_dictionary(&result.value)
        .expect("a FOUND_DICTIONARY value must parse as a dictionary");
    (
        result.result_type,
        dict.entries.get(subkey).map(|(v, _)| v.clone()),
    )
}

#[tokio::test]
#[ignore = "writes to the production KwaaiNet DHT; also needs KWAAI_LIVE_DHT_WRITE=1"]
async fn stores_and_updates_a_record_on_the_live_bootstraps() {
    if std::env::var("KWAAI_LIVE_DHT_WRITE").as_deref() != Ok("1") {
        eprintln!(
            "SKIPPED: this test WRITES to the production DHT. \
             Set KWAAI_LIVE_DHT_WRITE=1 to run it."
        );
        return;
    }

    let _ = tracing_subscriber::fmt()
        .with_env_filter("kwaai_p2p=debug")
        .try_init();

    // ── Ephemeral identity ──────────────────────────────────────────────────
    let keypair = Keypair::generate_ed25519();
    let local = keypair.public().to_peer_id();
    let local_b58 = local.to_base58();
    println!("local (ephemeral) peer id: {local_b58}");

    // 30 s per call — today's `send_to_bootstrap` budget, applied by the handle
    // rather than by a manual `tokio::time::timeout` wrapper.
    let config = NetworkConfig {
        listen_addrs: vec!["/ip4/0.0.0.0/tcp/0".to_string()],
        request_timeout: Duration::from_secs(30),
        ..NetworkConfig::default()
    };
    let (handle, task) = NetworkService::spawn(config, keypair).expect("swarm should start");

    // ── Connect to both bootstraps ──────────────────────────────────────────
    let mut bootstraps = Vec::new();
    for addr in KWAAI_BOOTSTRAP_SERVERS_DNS {
        println!("dialing {addr}");
        let t0 = Instant::now();
        match tokio::time::timeout(CONNECT_TIMEOUT, handle.connect_peer(addr)).await {
            Ok(Ok(peer)) => {
                println!("  connected: {peer} in {:?}", t0.elapsed());
                assert_eq!(
                    peer,
                    bootstrap_peer_id(addr),
                    "the connected peer must be the one the address names"
                );
                bootstraps.push(peer);
            }
            // A handshake failure against the RSA bootstraps is the Phase 1 risk
            // the migration plan flags — report it verbatim.
            Ok(Err(e)) => panic!("DIAL/HANDSHAKE FAILED for {addr}\n  error: {e}"),
            Err(_) => panic!("dial to {addr} timed out after {CONNECT_TIMEOUT:?}"),
        }
    }
    assert_eq!(
        bootstraps.len(),
        2,
        "this test needs both bootstraps: it writes to one and reads from the other"
    );
    let (writer, reader) = (bootstraps[0], bootstraps[1]);

    // ── The record ──────────────────────────────────────────────────────────
    // Unique per run and unreachable by the crawler; see the module docs.
    let raw_key = format!("_kwaai.test.ephemeral.{local_b58}");
    let key = dht_id(&raw_key);
    // A REAL subkey, exactly as an announce uses — neither IS_REGULAR_VALUE nor
    // IS_DICTIONARY — so the record serves back as a dictionary.
    let subkey = rmp_serde::to_vec(&local_b58).expect("msgpack subkey");
    let value = test_value("kwaainet native-p2p interop test");
    let expiration = get_dht_time() + TEST_TTL_SECS;

    println!("\nkey: {raw_key}");
    println!("  dht id:     {}", hex(&key));
    println!("  subkey:     {} bytes", subkey.len());
    println!("  expiration: {expiration:.3} (now + {TEST_TTL_SECS}s)");

    let store = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key.clone()],
        subkeys: vec![subkey.clone()],
        values: vec![value.clone()],
        expiration_time: vec![expiration],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(local)),
    };

    // ── 1. STORE to bootstrap #1 ────────────────────────────────────────────
    println!("\n[1] storing to {writer}");
    let (bytes, store_ms) = timed_call(
        &handle,
        writer,
        PROTOCOL_STORE,
        store.encode_to_vec(),
        "rpc_store",
    )
    .await;
    let response = StoreResponse::decode(&bytes[..]).expect("decodable StoreResponse");
    assert_eq!(
        response.store_ok,
        vec![true],
        "a Python Hivemind bootstrap must accept our native store"
    );
    println!("  store_ok: {:?}", response.store_ok);

    // ── 2. FIND the record back ─────────────────────────────────────────────
    //
    // From the bootstrap we wrote to. `rpc_store` is a **single-hop RPC**, not a
    // DHT put: it stores on the receiving node and replicates nothing. Hivemind's
    // `DHTNode.store` gets redundancy by calling `rpc_store` on each of the `k`
    // nearest nodes itself, which is a client-side loop we deliberately do not
    // run here. So reading from the *other* bootstrap returns rt=0 NOT_FOUND —
    // verified live, and asserted below so the distinction stays documented
    // rather than being rediscovered as a bug.
    let find = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key.clone()],
        peer: Some(NodeInfo::from_peer_id(local)),
    };

    println!("\n[2] finding from {writer} (the bootstrap that took the write)");
    let (bytes, find_ms) = timed_call(
        &handle,
        writer,
        PROTOCOL_FIND,
        find.encode_to_vec(),
        "rpc_find",
    )
    .await;
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");
    let (result_type, found) = dictionary_entry(&response, &subkey);

    assert_eq!(
        result_type,
        ResultType::FoundDictionary as i32,
        "a subkeyed record must come back as rt=2 FOUND_DICTIONARY, got rt={result_type}"
    );
    assert_eq!(
        found.as_deref(),
        Some(value.as_slice()),
        "the value read back must be the one we stored"
    );
    println!("  rt=2 FOUND_DICTIONARY, our subkey present and byte-identical");

    // The other bootstrap has never heard of it — single-hop store, as above.
    // This is why `send_to_bootstrap` fans out to every bootstrap rather than
    // storing once and trusting replication.
    println!("\n[2b] confirming {reader} does NOT have it (single-hop store)");
    let (bytes, _) = timed_call(
        &handle,
        reader,
        PROTOCOL_FIND,
        find.encode_to_vec(),
        "rpc_find (other bootstrap)",
    )
    .await;
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");
    assert_eq!(
        response.results[0].result_type,
        ResultType::NotFound as i32,
        "rpc_store is single-hop: an unwritten bootstrap must not have the record"
    );
    // Observed live: the production bootstraps answer NOT_FOUND with an
    // **empty** neighbour list. Hivemind's `rpc_find` does attach nearest peers
    // (`protocol.py:362-364`) — and our `DHTStorage` does too — so the empty
    // list is a property of these deployments' routing tables (they are network
    // roots that appear to hold no other DHT nodes), not of the protocol.
    // Recorded rather than asserted either way: the count is informational, and
    // pinning it to zero would fail the moment the fleet grows.
    println!(
        "  rt=0 NOT_FOUND with {} nearest peers",
        response.results[0].nearest_peer_ids.len()
    );

    // ── 3. "Unannounce": overwrite with a tombstone ─────────────────────────
    // The expiration MUST be strictly greater than the original. A shorter or
    // past one is rejected outright — that is the rule this step exists to pin.
    let tombstone_expiration = expiration + 1.0;
    let tombstone = test_value("kwaainet native-p2p interop test — withdrawn");
    println!("\n[3] storing a tombstone under the same subkey");
    println!("  expiration: {tombstone_expiration:.3} (strictly greater than {expiration:.3})");

    let withdraw = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![key.clone()],
        subkeys: vec![subkey.clone()],
        values: vec![tombstone.clone()],
        expiration_time: vec![tombstone_expiration],
        in_cache: vec![false],
        peer: Some(NodeInfo::from_peer_id(local)),
    };
    let (bytes, _) = timed_call(
        &handle,
        writer,
        PROTOCOL_STORE,
        withdraw.encode_to_vec(),
        "rpc_store (tombstone)",
    )
    .await;
    let response = StoreResponse::decode(&bytes[..]).expect("decodable StoreResponse");
    assert_eq!(
        response.store_ok,
        vec![true],
        "a strictly-greater expiration must be accepted; if this is false the \
         update rule is not what we think it is"
    );

    // ── 4. FIND again — the VALUE changed, the key did not vanish ───────────
    // A hivemind store can never delete. There is no delete RPC: withdrawal is
    // always "write something else and let the TTL run out". So the assertion
    // is that the value CHANGED, and explicitly that the key is still FOUND.
    println!("\n[4] re-reading from {writer}");
    let (bytes, _) = timed_call(
        &handle,
        writer,
        PROTOCOL_FIND,
        find.encode_to_vec(),
        "rpc_find (after tombstone)",
    )
    .await;
    let response = FindResponse::decode(&bytes[..]).expect("decodable FindResponse");
    let (result_type, found) = dictionary_entry(&response, &subkey);

    assert_eq!(
        result_type,
        ResultType::FoundDictionary as i32,
        "the key must still be FOUND — a store cannot delete, only supersede"
    );
    assert_eq!(
        found.as_deref(),
        Some(tombstone.as_slice()),
        "the value must now be the tombstone"
    );
    assert_ne!(
        found.as_deref(),
        Some(value.as_slice()),
        "the original value must be gone"
    );
    println!("  value changed to the tombstone; key still present, as expected");

    println!("\n--- timings ---");
    println!("  rpc_store: {store_ms:?}");
    println!("  rpc_find:  {find_ms:?}");
    println!(
        "\nrecords expire naturally at ~{tombstone_expiration:.0} \
         (about {TEST_TTL_SECS}s from the store)"
    );

    handle.shutdown().await.expect("shutdown");
    let _ = tokio::time::timeout(Duration::from_secs(5), task).await;
}

/// Lowercase hex, for printing a DHT ID.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
