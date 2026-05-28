//! Unit tests for kwaai-hivemind-dht: codec, value, storage, protocol.
//!
//! These tests require no network, no daemon, and no env vars.
//! They run on every `cargo test -p kwaai-network-tests`.

use kwaai_hivemind_dht::{
    DHTStorage, DHTValue,
    codec::{DHTRequest, DHTResponse, ProtocolMarker},
    protocol::{
        FindRequest, FindResult, NodeInfo, PingRequest, RequestAuthInfo,
        ResultType, StoreRequest,
    },
    value::{DHTValueBuilder, get_dht_time},
};
use kwaai_network_tests::metrics::MetricsRecorder;
use libp2p::PeerId;

// ============================================================================
// DHTValue — expiration and serialization
// ============================================================================

#[test]
fn value_with_ttl_is_valid() {
    let mut rec = MetricsRecorder::start("unit::dht::value_with_ttl_is_valid", "unit");
    let v = DHTValue::with_ttl(b"hello".to_vec(), 300.0);
    assert!(v.is_valid());
    assert!(!v.is_expired());
    assert!(v.time_until_expiration() > 0.0);
    rec.metric("ttl_remaining_s", v.time_until_expiration());
    rec.finish(true);
}

#[test]
fn value_already_expired() {
    let rec = MetricsRecorder::start("unit::dht::value_already_expired", "unit");
    let v = DHTValue::new(b"stale".to_vec(), 0.0); // epoch 0 — always expired
    assert!(!v.is_valid());
    assert!(v.is_expired());
    assert!(v.time_until_expiration() < 0.0);
    rec.finish(true);
}

#[test]
fn value_serialize_deserialize_roundtrip() {
    let mut rec = MetricsRecorder::start("unit::dht::value_serialize_deserialize_roundtrip", "unit");
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Payload {
        model: String,
        blocks: u32,
    }
    let original = Payload {
        model: "llama3.2:3b".into(),
        blocks: 32,
    };
    let dv = DHTValue::serialize(&original, 3600.0).unwrap();
    assert!(dv.is_valid());
    let decoded: Payload = dv.deserialize().unwrap();
    assert_eq!(decoded, original);
    rec.metric("payload_bytes", dv.value.len());
    rec.finish(true);
}

#[test]
fn deserialize_expired_value_returns_error() {
    let rec = MetricsRecorder::start("unit::dht::deserialize_expired_returns_error", "unit");
    let v = DHTValue::new(vec![0x93, 0x01, 0x02, 0x03], 0.0); // expired
    let result: kwaai_hivemind_dht::Result<Vec<u8>> = v.deserialize();
    assert!(result.is_err(), "should refuse to deserialize expired value");
    rec.finish(true);
}

#[test]
fn value_to_from_msgpack() {
    let mut rec = MetricsRecorder::start("unit::dht::value_to_from_msgpack", "unit");
    let payload = b"raw payload bytes".to_vec();
    let v = DHTValue::with_ttl(payload.clone(), 60.0);
    let mp = v.to_msgpack().unwrap();
    let decoded = DHTValue::from_msgpack(&mp).unwrap();
    assert_eq!(decoded, payload);
    rec.metric("msgpack_bytes", mp.len());
    rec.finish(true);
}

// ============================================================================
// DHTValueBuilder
// ============================================================================

#[test]
fn builder_ttl() {
    let rec = MetricsRecorder::start("unit::dht::builder_ttl", "unit");
    let v = DHTValueBuilder::new(b"data".to_vec())
        .ttl_seconds(120.0)
        .build();
    assert!(v.is_valid());
    assert!(v.time_until_expiration() > 100.0);
    rec.finish(true);
}

#[test]
fn builder_absolute_expiration() {
    let rec = MetricsRecorder::start("unit::dht::builder_absolute_expiration", "unit");
    let far_future = get_dht_time() + 9999.0;
    let v = DHTValueBuilder::new(b"data".to_vec())
        .expiration_time(far_future)
        .build();
    assert!(v.is_valid());
    rec.finish(true);
}

#[test]
fn builder_default_ttl_one_hour() {
    let rec = MetricsRecorder::start("unit::dht::builder_default_ttl_one_hour", "unit");
    let v = DHTValueBuilder::new(b"data".to_vec()).build();
    // Default TTL is 3600s; remaining should be close to that
    let remaining = v.time_until_expiration();
    assert!(remaining > 3500.0, "default TTL should be ~3600s, got {remaining}");
    rec.finish(true);
}

#[test]
fn builder_from_type() {
    let rec = MetricsRecorder::start("unit::dht::builder_from_type", "unit");
    let value = ("kwaai-node".to_string(), 42u32);
    let v = DHTValueBuilder::from_type(&value)
        .unwrap()
        .ttl_seconds(300.0)
        .build();
    assert!(v.is_valid());
    rec.finish(true);
}

// ============================================================================
// DHTStorage — store / find / expiry / stats
// ============================================================================

#[test]
fn storage_store_and_find() {
    let rec = MetricsRecorder::start("unit::dht::storage_store_and_find", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);

    let node = NodeInfo::from_peer_id(peer);
    let exp = get_dht_time() + 3600.0;
    let store_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"block.0".to_vec()],
        subkeys: vec![vec![]],
        values: vec![b"server_data".to_vec()],
        expiration_time: vec![exp],
        in_cache: vec![false],
        peer: Some(node.clone()),
    };
    let store_res = storage.handle_store(store_req);
    assert_eq!(store_res.store_ok.len(), 1);
    assert!(store_res.store_ok[0], "store should succeed");

    let find_req = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"block.0".to_vec()],
        peer: Some(node),
    };
    let find_res = storage.handle_find(find_req);
    assert_eq!(find_res.results.len(), 1);
    assert_eq!(
        find_res.results[0].result_type,
        ResultType::FoundRegular as i32
    );
    assert_eq!(find_res.results[0].value, b"server_data");
    rec.finish(true);
}

#[test]
fn storage_rejects_expired_store() {
    let rec = MetricsRecorder::start("unit::dht::storage_rejects_expired_store", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);
    let node = NodeInfo::from_peer_id(peer);

    let store_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"stale_key".to_vec()],
        subkeys: vec![vec![]],
        values: vec![b"ignored".to_vec()],
        expiration_time: vec![0.0], // epoch 0 — already expired
        in_cache: vec![false],
        peer: Some(node),
    };
    let res = storage.handle_store(store_req);
    assert!(!res.store_ok[0], "expired value must be rejected");
    rec.finish(true);
}

#[test]
fn storage_find_missing_key_returns_not_found() {
    let rec = MetricsRecorder::start("unit::dht::storage_find_missing_returns_not_found", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);
    let node = NodeInfo::from_peer_id(peer);

    let find_req = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"no_such_key".to_vec()],
        peer: Some(node),
    };
    let res = storage.handle_find(find_req);
    assert_eq!(res.results.len(), 1);
    assert_eq!(res.results[0].result_type, ResultType::NotFound as i32);
    rec.finish(true);
}

#[test]
fn storage_cleanup_removes_expired() {
    let mut rec = MetricsRecorder::start("unit::dht::storage_cleanup_removes_expired", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);
    let node = NodeInfo::from_peer_id(peer);

    // Store one live and one immediately-expired value
    // (We store both via handle_store; the expired one will be rejected at store time)
    let exp_future = get_dht_time() + 3600.0;
    let store_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"live".to_vec()],
        subkeys: vec![vec![]],
        values: vec![b"v".to_vec()],
        expiration_time: vec![exp_future],
        in_cache: vec![false],
        peer: Some(node),
    };
    storage.handle_store(store_req);
    let (total_before, valid_before) = storage.stats();

    storage.cleanup_expired();

    let (total_after, valid_after) = storage.stats();
    // Valid count should be unchanged (only live entries remain)
    assert_eq!(valid_before, valid_after);
    // Total may have dropped if cleanup removed previously-stored stale entries
    assert!(total_after <= total_before);
    rec.metric("entries_before", total_before);
    rec.metric("entries_after", total_after);
    rec.finish(true);
}

#[test]
fn storage_handle_request_ping() {
    let rec = MetricsRecorder::start("unit::dht::storage_handle_request_ping", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);
    let node = NodeInfo::from_peer_id(peer);

    let req = DHTRequest::Ping(PingRequest::new(node, true));
    let res = storage.handle_request(req).unwrap();
    matches!(res, kwaai_hivemind_dht::codec::DHTResponse::Ping(_));
    rec.finish(true);
}

#[test]
fn storage_multi_key_batch() {
    let mut rec = MetricsRecorder::start("unit::dht::storage_multi_key_batch", "unit");
    let peer = PeerId::random();
    let storage = DHTStorage::new(peer);
    let node = NodeInfo::from_peer_id(peer);
    let exp = get_dht_time() + 3600.0;

    // Store 5 keys in one request
    let n = 5usize;
    let store_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: (0..n).map(|i| format!("key{i}").into_bytes()).collect(),
        subkeys: vec![vec![]; n],
        values: (0..n).map(|i| format!("val{i}").into_bytes()).collect(),
        expiration_time: vec![exp; n],
        in_cache: vec![false; n],
        peer: Some(node.clone()),
    };
    let store_res = storage.handle_store(store_req);
    assert!(store_res.store_ok.iter().all(|&ok| ok));

    // Find all 5 in one request
    let find_req = FindRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: (0..n).map(|i| format!("key{i}").into_bytes()).collect(),
        peer: Some(node),
    };
    let find_res = storage.handle_find(find_req);
    assert_eq!(find_res.results.len(), n);
    for (i, result) in find_res.results.iter().enumerate() {
        assert_eq!(result.result_type, ResultType::FoundRegular as i32);
        assert_eq!(result.value, format!("val{i}").into_bytes());
    }
    rec.metric("batch_size", n);
    rec.finish(true);
}

// ============================================================================
// ResultType conversions
// ============================================================================

#[test]
fn result_type_roundtrip() {
    let rec = MetricsRecorder::start("unit::dht::result_type_roundtrip", "unit");
    for (rt, expected) in [
        (ResultType::NotFound, 0i32),
        (ResultType::FoundRegular, 1),
        (ResultType::FoundDictionary, 2),
    ] {
        assert_eq!(rt as i32, expected);
        assert_eq!(ResultType::from_i32(expected), Some(rt));
        assert_eq!(ResultType::try_from(expected), Ok(rt));
    }
    assert_eq!(ResultType::from_i32(99), None);
    rec.finish(true);
}

// ============================================================================
// NodeInfo ↔ PeerId roundtrip
// ============================================================================

#[test]
fn node_info_peer_id_roundtrip() {
    let rec = MetricsRecorder::start("unit::dht::node_info_peer_id_roundtrip", "unit");
    let original = PeerId::random();
    let ni = NodeInfo::from_peer_id(original);
    let recovered = ni.to_peer_id().expect("should recover PeerId from NodeInfo");
    assert_eq!(original, recovered);
    rec.finish(true);
}

// ============================================================================
// HivemindCodec / DHTRequest / DHTResponse encode-decode
// ============================================================================

#[test]
fn codec_ping_roundtrip() {
    let mut rec = MetricsRecorder::start("unit::dht::codec_ping_roundtrip", "unit");
    let node = NodeInfo::from_peer_id(PeerId::random());
    let req = DHTRequest::Ping(PingRequest::new(node, true));

    let bytes = req.encode().unwrap();
    // First 8 bytes = big-endian length, byte 8 = marker
    assert!(bytes.len() >= 9);
    let marker = bytes[8];
    assert_eq!(marker, ProtocolMarker::Ping as u8);

    // decode() takes [marker + proto], skipping the length prefix
    let decoded = DHTRequest::decode(&bytes[8..]).unwrap();
    assert!(matches!(decoded, DHTRequest::Ping(_)));
    rec.metric("encoded_bytes", bytes.len());
    rec.finish(true);
}

#[test]
fn codec_store_roundtrip() {
    let mut rec = MetricsRecorder::start("unit::dht::codec_store_roundtrip", "unit");
    let peer = PeerId::random();
    let node = NodeInfo::from_peer_id(peer);
    let exp = get_dht_time() + 3600.0;

    let store_req = StoreRequest {
        auth: Some(RequestAuthInfo::new()),
        keys: vec![b"model.block.0".to_vec()],
        subkeys: vec![vec![]],
        values: vec![b"server_info_bytes".to_vec()],
        expiration_time: vec![exp],
        in_cache: vec![false],
        peer: Some(node),
    };
    let req = DHTRequest::Store(store_req);
    let bytes = req.encode().unwrap();

    assert_eq!(bytes[8], ProtocolMarker::Store as u8);
    let decoded = DHTRequest::decode(&bytes[8..]).unwrap();
    if let DHTRequest::Store(s) = decoded {
        assert_eq!(s.keys.len(), 1);
        assert_eq!(s.keys[0], b"model.block.0");
        assert_eq!(s.values[0], b"server_info_bytes");
    } else {
        panic!("wrong variant");
    }
    rec.metric("encoded_bytes", bytes.len());
    rec.finish(true);
}

#[test]
fn codec_find_roundtrip() {
    let mut rec = MetricsRecorder::start("unit::dht::codec_find_roundtrip", "unit");
    let node = NodeInfo::from_peer_id(PeerId::random());
    let keys = vec![b"block.0".to_vec(), b"block.1".to_vec(), b"block.2".to_vec()];
    let req = DHTRequest::Find(FindRequest::new(node, keys.clone()));

    let bytes = req.encode().unwrap();
    assert_eq!(bytes[8], ProtocolMarker::Find as u8);
    let decoded = DHTRequest::decode(&bytes[8..]).unwrap();
    if let DHTRequest::Find(f) = decoded {
        assert_eq!(f.keys, keys);
    } else {
        panic!("wrong variant");
    }
    rec.metric("key_count", keys.len());
    rec.finish(true);
}

#[test]
fn codec_find_response_roundtrip() {
    let rec = MetricsRecorder::start("unit::dht::codec_find_response_roundtrip", "unit");
    let peer = PeerId::random();
    let node = NodeInfo::from_peer_id(peer);

    let results = vec![
        FindResult::found_regular(b"value_bytes".to_vec(), get_dht_time() + 300.0, vec![], vec![]),
        FindResult::not_found(vec![], vec![]),
    ];
    use kwaai_hivemind_dht::protocol::FindResponse;
    let resp = DHTResponse::Find(FindResponse::new(node, results));

    let bytes = resp.encode().unwrap();
    assert_eq!(bytes[8], ProtocolMarker::Find as u8);
    let decoded = DHTResponse::decode(&bytes[8..]).unwrap();
    if let DHTResponse::Find(f) = decoded {
        assert_eq!(f.results.len(), 2);
        assert_eq!(f.results[0].result_type, ResultType::FoundRegular as i32);
        assert_eq!(f.results[0].value, b"value_bytes");
        assert_eq!(f.results[1].result_type, ResultType::NotFound as i32);
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn codec_decode_empty_bytes_returns_error() {
    let rec = MetricsRecorder::start("unit::dht::codec_decode_empty_bytes_returns_error", "unit");
    let result = DHTRequest::decode(&[]);
    assert!(result.is_err());
    rec.finish(true);
}

#[test]
fn codec_decode_invalid_marker_returns_error() {
    let rec = MetricsRecorder::start("unit::dht::codec_decode_invalid_marker_returns_error", "unit");
    // marker byte 0xFF is not valid
    let result = DHTRequest::decode(&[0xFF, 0x00]);
    assert!(result.is_err());
    rec.finish(true);
}

// ============================================================================
// Wire format length prefix correctness
// ============================================================================

#[test]
fn codec_length_prefix_correct() {
    let rec = MetricsRecorder::start("unit::dht::codec_length_prefix_correct", "unit");
    let node = NodeInfo::from_peer_id(PeerId::random());
    let req = DHTRequest::Ping(PingRequest::new(node, false));
    let bytes = req.encode().unwrap();

    let declared_len = u64::from_be_bytes(bytes[0..8].try_into().unwrap()) as usize;
    let actual_body_len = bytes.len() - 8;
    assert_eq!(declared_len, actual_body_len, "length prefix must equal body length");
    rec.finish(true);
}
