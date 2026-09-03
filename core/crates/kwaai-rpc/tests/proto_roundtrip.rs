//! Proto round-trip tests for the kwaai.v1 wire types.
//!
//! These exist to catch accidental field renumbering in `proto/kwaai.proto`.
//! Once the GUI and any third-party clients are pinned to a particular
//! ChatMessage / ChatToken encoding, renumbering a field (or repurposing a
//! tag) would silently break the wire. Round-tripping every documented field
//! — including the `optional` ones in both their Some and None states — is
//! the cheapest way to make that breakage loud at CI time.
//!
//! NB: prost emits `Option<T>` for proto3 `optional` fields. The contract we
//! lock down here is:
//!   * a `Some(value)` encodes to a non-empty body and decodes back to the
//!     same `Some(value)` (no value loss);
//!   * a `None` produces a body that omits the tag entirely and decodes back
//!     to `None` (we are NOT silently widening absent-vs-empty).

use kwaai_rpc::v1::{
    client_frame, server_frame, ChatMessage, ChatToken, ClientFrame, ConnectedPeer, DhtKnownKey,
    DhtLookupOutcome, DhtLookupPeer, DhtLookupReply, DhtLookupRequest, DhtRecord, DhtRecordKind,
    DhtRecordsReply, DhtRecordsRequest, DhtRole, DhtSubkey, DhtValueView, KadStatus,
    NetworkRequest, NetworkUpdate, PeerConnKind, RoutingPeer, SelfStatus, ServerFrame,
    UpdateReason,
};
use prost::Message;

/// `ChatMessage` with every field set, including `conversation_id`.
///
/// Guards against:
///   - `content` / `role` losing data (tags 1, 2)
///   - `optional string conversation_id` (tag 3) being dropped or remapped.
#[test]
fn chat_message_roundtrip_all_fields_set() {
    let original = ChatMessage {
        content: "Hello, daemon. How many transformer blocks do you serve?".to_string(),
        role: "user".to_string(),
        conversation_id: Some("conv-abc-123".to_string()),
    };

    let bytes = original.encode_to_vec();
    assert!(!bytes.is_empty(), "encoded ChatMessage must not be empty");

    let decoded = ChatMessage::decode(bytes.as_slice()).expect("decode ChatMessage");
    assert_eq!(
        decoded.content, original.content,
        "content survives round-trip"
    );
    assert_eq!(decoded.role, original.role, "role survives round-trip");
    assert_eq!(
        decoded.conversation_id, original.conversation_id,
        "optional conversation_id (tag 3) survives as Some(..)"
    );
    assert_eq!(
        decoded, original,
        "full ChatMessage equality after round-trip"
    );
}

/// `ChatMessage` with `conversation_id = None`. Guards the absent-vs-empty
/// distinction: an unset optional must decode back as `None`, not
/// `Some("".to_string())`.
#[test]
fn chat_message_roundtrip_optional_unset() {
    let original = ChatMessage {
        content: "ping".to_string(),
        role: "system".to_string(),
        conversation_id: None,
    };

    let bytes = original.encode_to_vec();
    let decoded = ChatMessage::decode(bytes.as_slice()).expect("decode ChatMessage");
    assert_eq!(
        decoded.conversation_id, None,
        "absent optional stays absent"
    );
    assert_eq!(decoded, original);
}

/// `ChatToken` mid-stream (typical streaming chunk): non-terminal token, no
/// finish_reason. Covers tags 1, 2 and the `None` state of tag 3.
#[test]
fn chat_token_roundtrip_streaming_chunk() {
    let original = ChatToken {
        text: "Sure, ".to_string(),
        done: false,
        finish_reason: None,
    };

    let bytes = original.encode_to_vec();
    let decoded = ChatToken::decode(bytes.as_slice()).expect("decode ChatToken");
    assert_eq!(decoded.text, original.text);
    assert!(!decoded.done, "done bool stays false");
    assert_eq!(decoded.finish_reason, None, "absent optional stays absent");
    assert_eq!(decoded, original);
}

/// Terminal `ChatToken` with `done = true` and `finish_reason = Some("stop")`.
/// Pins down the EOS framing the server emits at the end of every Chat reply.
#[test]
fn chat_token_roundtrip_terminal_with_finish_reason() {
    let original = ChatToken {
        text: String::new(),
        done: true,
        finish_reason: Some("stop".to_string()),
    };

    let bytes = original.encode_to_vec();
    let decoded = ChatToken::decode(bytes.as_slice()).expect("decode ChatToken");
    assert!(decoded.done, "done bool stays true");
    assert_eq!(
        decoded.finish_reason.as_deref(),
        Some("stop"),
        "finish_reason value preserved"
    );
    assert_eq!(decoded, original);
}

/// Spot-check that we are encoding a *known wire tag* for `conversation_id`.
/// Proto3 tag 3 + LEN wire type (2) yields a key byte of `(3 << 3) | 2 = 0x1a`.
/// If somebody renumbers `conversation_id` to a different tag, this byte
/// disappears from the encoding and the assertion trips — flagging the
/// breakage even if every other test still passes.
///
/// We only check the prefix: the rest of the encoded message depends on the
/// other fields we set.
#[test]
fn chat_message_conversation_id_wire_tag_is_stable() {
    // Minimal payload: only conversation_id present, so the encoded bytes
    // contain exactly the tag-3 key, the length, and the UTF-8 bytes.
    let msg = ChatMessage {
        content: String::new(),
        role: String::new(),
        conversation_id: Some("x".to_string()),
    };
    let bytes = msg.encode_to_vec();

    // Expected: [0x1a, 0x01, b'x']
    //   0x1a = (tag=3 << 3) | wire_type=2 (LEN)
    //   0x01 = length-prefix of the embedded string ("x")
    //   b'x' = the payload
    assert_eq!(
        bytes.as_slice(),
        &[0x1a, 0x01, b'x'],
        "conversation_id must serialise at proto tag 3 (key byte 0x1a)"
    );
}

/// Same idea for `ChatToken.finish_reason` (proto tag 3) — guards against a
/// silent renumber of the terminal-token reason field.
#[test]
fn chat_token_finish_reason_wire_tag_is_stable() {
    let token = ChatToken {
        text: String::new(),
        done: false,
        finish_reason: Some("y".to_string()),
    };
    let bytes = token.encode_to_vec();

    assert_eq!(
        bytes.as_slice(),
        &[0x1a, 0x01, b'y'],
        "finish_reason must serialise at proto tag 3 (key byte 0x1a)"
    );
}

/// `ConnectedPeer` round-trip with every field populated.
///
/// The three enrichment fields (`protocols`, `rtt_ms`, `agent_version`) fill
/// in from identify and ping *after* a connection establishes, so the empty
/// state is meaningful and is covered separately below.
#[test]
fn connected_peer_roundtrip_all_fields_set() {
    let original = ConnectedPeer {
        peer_id: "12D3KooWExampleBootstrapPeerIdBase58".to_string(),
        addr: "/ip4/198.18.0.10/tcp/8000".to_string(),
        kind: PeerConnKind::Direct as i32,
        direction: "outbound".to_string(),
        is_bootstrap: true,
        is_trusted_relay: false,
        protocols: vec![
            "/ipfs/kad/1.0.0".to_string(),
            "/libp2p/circuit/relay/0.2.0/hop".to_string(),
        ],
        rtt_ms: 42,
        agent_version: "kwaainet/0.5.4".to_string(),
        via: String::new(),
        dcutr: false,
        // Consistent with the kad protocol advertised above: a peer that
        // speaks kad is a DHT server.
        dht_role: DhtRole::Server as i32,
    };

    let decoded = ConnectedPeer::decode(original.encode_to_vec().as_slice())
        .expect("ConnectedPeer must decode from its own encoding");
    assert_eq!(decoded, original);
}

/// A freshly established connection: identify and ping have not completed, so
/// the enrichment fields are empty/zero.
///
/// This pins the "not yet known" encoding. `rtt_ms == 0` must mean "no ping has
/// completed", never "zero latency", and an empty `protocols` must not be
/// confused with "speaks nothing" — a client rendering these needs the
/// distinction to survive the wire.
#[test]
fn connected_peer_roundtrip_before_identify() {
    let original = ConnectedPeer {
        peer_id: "12D3KooWExampleFreshlyConnectedPeer".to_string(),
        addr: "/ip4/192.168.1.10/tcp/4001".to_string(),
        kind: PeerConnKind::Relay as i32,
        direction: "inbound".to_string(),
        is_bootstrap: false,
        is_trusted_relay: false,
        protocols: vec![],
        rtt_ms: 0,
        agent_version: String::new(),
        via: String::new(),
        dcutr: false,
        // Identify has not run, so the role is genuinely not known yet —
        // the same "not yet known" state this test exists to pin.
        dht_role: DhtRole::Unknown as i32,
    };

    let decoded = ConnectedPeer::decode(original.encode_to_vec().as_slice())
        .expect("ConnectedPeer must decode from its own encoding");
    assert_eq!(decoded, original);
    assert!(decoded.protocols.is_empty());
    assert_eq!(decoded.rtt_ms, 0);
}

/// `NetworkUpdate` carrying all three sections.
///
/// Guards the section tags against renumbering: `self_status` (3),
/// `connected` (4) and `routing` (5) are what the GUI's Network page binds to.
#[test]
fn network_update_roundtrip_all_sections() {
    let original = NetworkUpdate {
        server_time: "2026-08-01T12:00:00Z".to_string(),
        reason: UpdateReason::Reachability as i32,
        self_status: Some(SelfStatus {
            peer_id: "12D3KooWExampleSelfPeerId".to_string(),
            reachability: "private".to_string(),
            reachability_source: "autonat".to_string(),
            using_relay: true,
            announceable: true,
            listen_addrs: vec!["/ip4/0.0.0.0/tcp/4001".to_string()],
            observed_addrs: vec!["/ip4/203.0.113.7/tcp/4001".to_string()],
            relay_addrs: vec!["/ip4/198.18.0.50/tcp/4001/p2p-circuit".to_string()],
            local_protocols: vec![
                "/ipfs/kad/1.0.0".to_string(),
                "/kwaai/inference/1.0.0".to_string(),
            ],
            // A node can be private on v4 and listening on v6 at once.
            ipv6: "active".to_string(),
        }),
        connected: vec![ConnectedPeer {
            peer_id: "12D3KooWExamplePeerA".to_string(),
            addr: "/ip4/198.18.0.10/tcp/8000".to_string(),
            kind: PeerConnKind::Direct as i32,
            direction: "outbound".to_string(),
            is_bootstrap: true,
            is_trusted_relay: false,
            protocols: vec!["/ipfs/kad/1.0.0".to_string()],
            rtt_ms: 7,
            agent_version: "kwaainet/0.5.4".to_string(),
            via: String::new(),
            dcutr: false,
            dht_role: DhtRole::Server as i32,
        }],
        routing: vec![
            RoutingPeer {
                peer_id: "12D3KooWExamplePeerA".to_string(),
                connected: true,
                is_bootstrap: false,
                addrs: vec!["/ip4/198.18.0.10/tcp/8000".to_string()],
                ..Default::default()
            },
            RoutingPeer {
                peer_id: "12D3KooWExampleKnownButOffline".to_string(),
                connected: false,
                is_bootstrap: false,
                // A routing entry keeps its addresses after the connection
                // drops — that is what makes it dialable again later.
                addrs: vec!["/ip4/198.18.0.99/tcp/8000".to_string()],
                ..Default::default()
            },
        ],
        bootstrap_total: 2,
        bootstrap_reachable: 1,
    };

    let decoded = NetworkUpdate::decode(original.encode_to_vec().as_slice())
        .expect("NetworkUpdate must decode from its own encoding");
    assert_eq!(decoded, original);
}

/// The routing table legitimately holds peers we are not connected to, and a
/// young node has connections but an empty table. Neither set contains the
/// other, so both edges must survive the wire.
#[test]
fn network_update_roundtrip_disjoint_peer_sets() {
    // Connections established, kad still in client mode: empty routing table.
    let young = NetworkUpdate {
        server_time: "2026-08-01T12:00:00Z".to_string(),
        reason: UpdateReason::Tick as i32,
        self_status: Some(SelfStatus {
            reachability: "unknown".to_string(),
            announceable: false,
            ..Default::default()
        }),
        connected: vec![ConnectedPeer {
            peer_id: "12D3KooWExamplePeerA".to_string(),
            ..Default::default()
        }],
        routing: vec![],
        ..Default::default()
    };
    let decoded = NetworkUpdate::decode(young.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, young);
    assert!(decoded.routing.is_empty());
    assert_eq!(decoded.connected.len(), 1);

    // The converse: a routing entry for a peer we hold no connection to.
    let known_not_connected = NetworkUpdate {
        server_time: "2026-08-01T12:00:00Z".to_string(),
        reason: UpdateReason::Peers as i32,
        self_status: None,
        connected: vec![],
        routing: vec![RoutingPeer {
            peer_id: "12D3KooWExampleKnownButOffline".to_string(),
            connected: false,
            is_bootstrap: false,
            // Deliberately empty: a routing entry carrying no address is
            // the degenerate case a client has to be able to distinguish,
            // and an empty list must survive the wire as one.
            addrs: vec![],
            ..Default::default()
        }],
        ..Default::default()
    };
    let decoded =
        NetworkUpdate::decode(known_not_connected.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, known_not_connected);
}

/// The k-bucket detail the DHT-entry dialog renders.
///
/// `last_contact_secs` is `optional`, and absent is a real state — a routing
/// entry seeded from a configured address before any dial succeeded. Proto3
/// erases a zero-valued scalar, so without the `optional` keyword "never
/// connected" and "connected this very second" would arrive identical.
#[test]
fn routing_peer_roundtrip_kad_detail() {
    let never_dialed = RoutingPeer {
        peer_id: "12D3KooWExampleSeeded".to_string(),
        connected: false,
        kad_status: KadStatus::Disconnected as i32,
        bucket: 255,
        bucket_entries: 20,
        bucket_capacity: 20,
        bucket_pending: true,
        distance: "f".repeat(64),
        last_contact_secs: None,
        ..Default::default()
    };
    let decoded = RoutingPeer::decode(never_dialed.encode_to_vec().as_slice())
        .expect("RoutingPeer must decode from its own encoding");
    assert_eq!(decoded, never_dialed);
    assert_eq!(decoded.last_contact_secs, None);

    let just_now = RoutingPeer {
        last_contact_secs: Some(0),
        ..never_dialed.clone()
    };
    let decoded = RoutingPeer::decode(just_now.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded.last_contact_secs, Some(0));
}

/// `UpdateReason::Tick` is the proto3 zero value, so it occupies no bytes on
/// the wire. That is deliberate — but it means a client cannot distinguish
/// "reason was TICK" from "sender predates the field". Pin the two facts that
/// matter: TICK is the default, and every other reason survives the round trip.
#[test]
fn update_reason_tick_is_the_zero_value() {
    let tick = NetworkUpdate {
        reason: UpdateReason::Tick as i32,
        ..Default::default()
    };
    assert!(
        tick.encode_to_vec().is_empty(),
        "an otherwise-empty NetworkUpdate with reason=TICK must encode to zero bytes"
    );

    for reason in [
        UpdateReason::Reachability,
        UpdateReason::Peers,
        UpdateReason::Heartbeat,
    ] {
        let msg = NetworkUpdate {
            reason: reason as i32,
            ..Default::default()
        };
        let decoded = NetworkUpdate::decode(msg.encode_to_vec().as_slice()).expect("must decode");
        assert_eq!(
            decoded.reason, reason as i32,
            "{reason:?} must survive the round trip"
        );
    }
}

/// The Session envelope slot. `NetworkRequest` rides at oneof tag 17 on
/// ClientFrame and `NetworkUpdate` at tag 17 on ServerFrame; the GUI dispatches
/// on exactly those, and tags 10-16 are already spoken for by other operations.
#[test]
fn network_frames_occupy_oneof_tag_17() {
    let req = ClientFrame {
        id: 1,
        body: Some(client_frame::Body::Network(NetworkRequest {
            subscribe: true,
            interval_secs: 5,
        })),
    };
    let decoded =
        ClientFrame::decode(req.encode_to_vec().as_slice()).expect("ClientFrame must decode");
    assert!(
        matches!(decoded.body, Some(client_frame::Body::Network(_))),
        "NetworkRequest must ride in the ClientFrame.network oneof arm"
    );

    // Tag 17, wire type 2 (LEN) => (17 << 3) | 2 = 138 => varint [0x8a, 0x01].
    let body_only = ClientFrame {
        id: 0,
        body: Some(client_frame::Body::Network(NetworkRequest::default())),
    };
    assert_eq!(
        body_only.encode_to_vec().as_slice(),
        &[0x8a, 0x01, 0x00],
        "ClientFrame.network must serialise at proto tag 17"
    );

    let update = ServerFrame {
        id: 1,
        body: Some(server_frame::Body::Network(NetworkUpdate::default())),
    };
    assert_eq!(
        update.encode_to_vec().as_slice(),
        &[0x08, 0x01, 0x8a, 0x01, 0x00],
        "ServerFrame.network must serialise at proto tag 17"
    );
}

/// A dictionary record with its subkey breakdown — the shape every per-block
/// key really has. The `optional` fields would lose meaning if widened to
/// 0/empty: an unbounded dictionary would read as capped at zero.
#[test]
fn dht_record_roundtrip_dictionary_with_subkeys() {
    let original = DhtRecord {
        key_id: "5b7a1c0f".repeat(5),
        key_name: "Qwen3-8B.0".to_string(),
        kind: DhtRecordKind::Dictionary as i32,
        in_cache: false,
        subkey_count: 2,
        value_bytes: 412,
        expiration: 1_788_000_000.5,
        expires_in_secs: 317.25,
        latest_expiration: Some(1_788_000_090.0),
        dict_maxsize: Some(1_048_576),
        value: None,
        subkeys: vec![
            DhtSubkey {
                subkey: "\"12D3KooWExampleA\"".to_string(),
                expiration: 1_788_000_000.5,
                expires_in_secs: 317.25,
                value_bytes: 206,
                value: Some(DhtValueView {
                    msgpack_type: "ext".to_string(),
                    rendered: "ext(64)[2, 1.5, {\"start_block\": 0}]".to_string(),
                    raw_bytes: 206,
                    ext_code: Some(64),
                    decoded: true,
                }),
            },
            DhtSubkey {
                // Independent expiration: a subkey outliving its record is
                // normal, and the countdown must be able to go negative.
                subkey: "\"12D3KooWExampleB\"".to_string(),
                expiration: 1_787_999_900.0,
                expires_in_secs: -82.75,
                value_bytes: 206,
                value: Some(DhtValueView {
                    msgpack_type: "undecodable".to_string(),
                    rendered: "c1".to_string(),
                    raw_bytes: 1,
                    ext_code: None,
                    decoded: false,
                }),
            },
        ],
    };

    let decoded = DhtRecord::decode(original.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, original);
    assert_eq!(decoded.subkeys[1].expires_in_secs, -82.75);
    assert_eq!(
        decoded.subkeys[0].value.as_ref().unwrap().ext_code,
        Some(64)
    );
}

/// A regular record must be distinguishable from a one-entry dictionary: the
/// value lives on the record and the dictionary-only fields are absent.
#[test]
fn dht_record_roundtrip_regular_value_absent_optionals() {
    let original = DhtRecord {
        key_id: "00".repeat(20),
        // Unnamed is the normal case for a record replicated onto us: the id
        // is a hash and there is nothing to invert.
        key_name: String::new(),
        kind: DhtRecordKind::Regular as i32,
        in_cache: true,
        subkey_count: 0,
        value_bytes: 9,
        expiration: 1_788_000_000.0,
        expires_in_secs: -5.0,
        latest_expiration: None,
        dict_maxsize: None,
        value: Some(DhtValueView {
            msgpack_type: "bin".to_string(),
            rendered: "hex:fffe".to_string(),
            raw_bytes: 9,
            ext_code: None,
            decoded: true,
        }),
        subkeys: vec![],
    };

    let decoded = DhtRecord::decode(original.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, original);
    assert!(decoded.latest_expiration.is_none());
    assert!(decoded.dict_maxsize.is_none());
    assert!(decoded.subkeys.is_empty());
}

/// A lookup reply. NOT_FOUND and ERROR both carry no record, so only `outcome`
/// separates "nobody has this" from "nobody was reachable".
#[test]
fn dht_lookup_reply_roundtrip_distinguishes_not_found_from_error() {
    let original = DhtLookupReply {
        server_time: "2026-09-03T12:00:00.000Z".to_string(),
        key: "_petals.models".to_string(),
        key_id: "ab".repeat(20),
        peers: vec![
            DhtLookupPeer {
                peer_id: "12D3KooWBootstrapOne".to_string(),
                outcome: DhtLookupOutcome::Found as i32,
                error: String::new(),
                elapsed_ms: 42,
                record: Some(DhtRecord {
                    key_id: "ab".repeat(20),
                    key_name: "_petals.models".to_string(),
                    kind: DhtRecordKind::Dictionary as i32,
                    subkey_count: 1,
                    ..Default::default()
                }),
            },
            DhtLookupPeer {
                peer_id: "12D3KooWBootstrapTwo".to_string(),
                outcome: DhtLookupOutcome::NotFound as i32,
                error: String::new(),
                elapsed_ms: 17,
                record: None,
            },
            DhtLookupPeer {
                peer_id: "12D3KooWUnreachable".to_string(),
                outcome: DhtLookupOutcome::Error as i32,
                error: "timed out after 10s".to_string(),
                elapsed_ms: 10_000,
                record: None,
            },
        ],
        record: None,
    };

    let decoded = DhtLookupReply::decode(original.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, original);
    assert_ne!(decoded.peers[1].outcome, decoded.peers[2].outcome);
    assert!(
        decoded.record.is_none(),
        "no merged record when none was best"
    );
}

/// Both new bodies must serialise at tags 26 and 27, asserted as wire bytes:
/// a decode-back-into-the-same-variant check passes for any tag protoc
/// accepts, so only the bytes catch a renumbering.
#[test]
fn dht_frames_occupy_oneof_tags_26_and_27() {
    // Tag 26, wire type 2 (LEN) => (26 << 3) | 2 = 210 => varint [0xd2, 0x01].
    // Tag 27 => (27 << 3) | 2 = 218 => varint [0xda, 0x01].
    let records_req = ClientFrame {
        id: 0,
        body: Some(client_frame::Body::DhtRecords(DhtRecordsRequest::default())),
    };
    assert_eq!(
        records_req.encode_to_vec().as_slice(),
        &[0xd2, 0x01, 0x00],
        "ClientFrame.dht_records must serialise at proto tag 26"
    );

    let lookup_req = ClientFrame {
        id: 0,
        body: Some(client_frame::Body::DhtLookup(DhtLookupRequest::default())),
    };
    assert_eq!(
        lookup_req.encode_to_vec().as_slice(),
        &[0xda, 0x01, 0x00],
        "ClientFrame.dht_lookup must serialise at proto tag 27"
    );

    let records_reply = ServerFrame {
        id: 1,
        body: Some(server_frame::Body::DhtRecords(DhtRecordsReply::default())),
    };
    assert_eq!(
        records_reply.encode_to_vec().as_slice(),
        &[0x08, 0x01, 0xd2, 0x01, 0x00],
        "ServerFrame.dht_records must serialise at proto tag 26"
    );

    let lookup_reply = ServerFrame {
        id: 1,
        body: Some(server_frame::Body::DhtLookup(DhtLookupReply::default())),
    };
    assert_eq!(
        lookup_reply.encode_to_vec().as_slice(),
        &[0x08, 0x01, 0xda, 0x01, 0x00],
        "ServerFrame.dht_lookup must serialise at proto tag 27"
    );
}

/// `limit` bounds a listing and `total_records` says when it was cut short.
#[test]
fn dht_records_roundtrip_carries_the_listing_bounds() {
    let req = ClientFrame {
        id: 26,
        body: Some(client_frame::Body::DhtRecords(DhtRecordsRequest {
            include_values: true,
            key_ids: vec!["ab".repeat(20)],
            limit: 250,
        })),
    };
    let decoded = ClientFrame::decode(req.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, req);

    let reply = ServerFrame {
        id: 26,
        body: Some(server_frame::Body::DhtRecords(DhtRecordsReply {
            server_time: "2026-09-03T12:00:00.000Z".to_string(),
            records: vec![DhtRecord {
                key_id: "cd".repeat(20),
                ..Default::default()
            }],
            known_keys: vec![DhtKnownKey {
                name: "_petals.models".to_string(),
                key_id: "ab".repeat(20),
            }],
            // One row returned out of many matched: the truncation signal.
            total_records: 12_043,
        })),
    };
    let decoded = ServerFrame::decode(reply.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, reply);
    let Some(server_frame::Body::DhtRecords(r)) = decoded.body else {
        panic!("wrong arm");
    };
    assert!(r.records.len() < r.total_records as usize);
}
