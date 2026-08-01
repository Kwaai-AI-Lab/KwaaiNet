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
    client_frame, server_frame, ChatMessage, ChatToken, ClientFrame, ConnectedPeer, NetworkRequest,
    NetworkUpdate, PeerConnKind, RoutingPeer, SelfStatus, ServerFrame, UpdateReason,
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
        }],
        routing: vec![
            RoutingPeer {
                peer_id: "12D3KooWExamplePeerA".to_string(),
                connected: true,
            },
            RoutingPeer {
                peer_id: "12D3KooWExampleKnownButOffline".to_string(),
                connected: false,
            },
        ],
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
        }],
    };
    let decoded =
        NetworkUpdate::decode(known_not_connected.encode_to_vec().as_slice()).expect("must decode");
    assert_eq!(decoded, known_not_connected);
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
