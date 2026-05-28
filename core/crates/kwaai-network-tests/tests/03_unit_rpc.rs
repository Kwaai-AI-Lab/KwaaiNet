//! Unit tests for kwaai-rpc: proto frame roundtrips, error codes, field presence.
//! No network required.

use kwaai_network_tests::metrics::MetricsRecorder;
use kwaai_rpc::v1::{
    Cancel, ChatMessage, ChatToken, ClientFrame, Done, GenerateRequest, PingReply, PingRequest,
    ServerFrame, ShardRunRequest, StatusReply, StatusRequest,
    client_frame, server_frame,
    error::Code as ErrorCode,
};
use prost013::Message as _;

// ============================================================================
// Helpers
// ============================================================================

fn encode_decode_client_frame(body: client_frame::Body) -> ClientFrame {
    let frame = ClientFrame { id: 42, body: Some(body) };
    let bytes = frame.encode_to_vec();
    ClientFrame::decode(bytes.as_slice()).unwrap()
}

fn encode_decode_server_frame(body: server_frame::Body) -> ServerFrame {
    let frame = ServerFrame { id: 42, body: Some(body) };
    let bytes = frame.encode_to_vec();
    ServerFrame::decode(bytes.as_slice()).unwrap()
}

// ============================================================================
// ClientFrame variants
// ============================================================================

#[test]
fn client_frame_ping_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::client_frame_ping_roundtrip", "unit");
    let decoded = encode_decode_client_frame(client_frame::Body::Ping(PingRequest {}));
    assert_eq!(decoded.id, 42);
    assert!(matches!(decoded.body, Some(client_frame::Body::Ping(_))));
    rec.finish(true);
}

#[test]
fn client_frame_generate_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::client_frame_generate_roundtrip", "unit");
    let req = GenerateRequest {
        role: "user".to_string(),
        content: "Explain distributed inference.".to_string(),
        conversation_id: Some("conv-1".to_string()),
    };
    let decoded = encode_decode_client_frame(client_frame::Body::Generate(req));
    if let Some(client_frame::Body::Generate(g)) = decoded.body {
        assert_eq!(g.role, "user");
        assert_eq!(g.content, "Explain distributed inference.");
        assert_eq!(g.conversation_id.as_deref(), Some("conv-1"));
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn client_frame_shard_run_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::client_frame_shard_run_roundtrip", "unit");
    let req = ShardRunRequest {
        role: "user".to_string(),
        content: "What is KwaaiNet?".to_string(),
        model: Some("llama3.2:3b".to_string()),
        conversation_id: None,
    };
    let decoded = encode_decode_client_frame(client_frame::Body::ShardRun(req));
    if let Some(client_frame::Body::ShardRun(s)) = decoded.body {
        assert_eq!(s.role, "user");
        assert_eq!(s.model.as_deref(), Some("llama3.2:3b"));
        assert!(s.conversation_id.is_none());
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn client_frame_status_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::client_frame_status_roundtrip", "unit");
    let decoded = encode_decode_client_frame(client_frame::Body::Status(StatusRequest {}));
    assert!(matches!(decoded.body, Some(client_frame::Body::Status(_))));
    rec.finish(true);
}

#[test]
fn client_frame_cancel_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::client_frame_cancel_roundtrip", "unit");
    let decoded = encode_decode_client_frame(client_frame::Body::Cancel(Cancel { target_id: 17 }));
    if let Some(client_frame::Body::Cancel(c)) = decoded.body {
        assert_eq!(c.target_id, 17);
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

// ============================================================================
// ServerFrame variants
// ============================================================================

#[test]
fn server_frame_pong_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::server_frame_pong_roundtrip", "unit");
    let decoded = encode_decode_server_frame(server_frame::Body::Pong(PingReply {
        server_time: "2026-05-23T18:32:00Z".to_string(),
    }));
    if let Some(server_frame::Body::Pong(p)) = decoded.body {
        assert_eq!(p.server_time, "2026-05-23T18:32:00Z");
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn server_frame_token_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::server_frame_token_roundtrip", "unit");
    let decoded = encode_decode_server_frame(server_frame::Body::Token(ChatToken {
        text: " distributed".to_string(),
        done: false,
        finish_reason: None,
    }));
    if let Some(server_frame::Body::Token(t)) = decoded.body {
        assert_eq!(t.text, " distributed");
        assert!(!t.done);
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn server_frame_done_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::server_frame_done_roundtrip", "unit");
    let decoded = encode_decode_server_frame(server_frame::Body::Done(Done {}));
    assert!(matches!(decoded.body, Some(server_frame::Body::Done(_))));
    rec.finish(true);
}

#[test]
fn server_frame_error_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::server_frame_error_roundtrip", "unit");
    let decoded = encode_decode_server_frame(server_frame::Body::Error(kwaai_rpc::v1::Error {
        code: ErrorCode::NoPeersForModel as i32,
        message: "no peers serving llama3.2:3b".to_string(),
    }));
    if let Some(server_frame::Body::Error(e)) = decoded.body {
        assert_eq!(e.code, ErrorCode::NoPeersForModel as i32);
        assert!(e.message.contains("llama3.2:3b"));
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

#[test]
fn server_frame_status_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::server_frame_status_roundtrip", "unit");
    let reply = StatusReply {
        server_time: "2026-05-23T18:32:00Z".to_string(),
        model: "llama3.2:3b".to_string(),
        shard_ready: true,
        peer_count: 12,
        uptime_secs: 3600,
    };
    let decoded = encode_decode_server_frame(server_frame::Body::Status(reply));
    if let Some(server_frame::Body::Status(s)) = decoded.body {
        assert_eq!(s.model, "llama3.2:3b");
        assert!(s.shard_ready);
        assert_eq!(s.peer_count, 12);
        assert_eq!(s.uptime_secs, 3600);
    } else {
        panic!("wrong variant");
    }
    rec.finish(true);
}

// ============================================================================
// Error codes
// ============================================================================

#[test]
fn error_codes_expected_values() {
    let rec = MetricsRecorder::start("unit::rpc::error_codes_expected_values", "unit");
    // These values are part of the wire contract — never renumber
    assert_eq!(ErrorCode::Unknown as i32, 0);
    assert_eq!(ErrorCode::InvalidArgument as i32, 1);
    assert_eq!(ErrorCode::NotFound as i32, 2);
    assert_eq!(ErrorCode::Unavailable as i32, 3);
    assert_eq!(ErrorCode::Cancelled as i32, 4);
    assert_eq!(ErrorCode::Internal as i32, 5);
    assert_eq!(ErrorCode::Unimplemented as i32, 6);
    assert_eq!(ErrorCode::NoPeersForModel as i32, 7);
    assert_eq!(ErrorCode::InsufficientCoverage as i32, 8);
    assert_eq!(ErrorCode::AllCandidatesFailed as i32, 9);
    assert_eq!(ErrorCode::ModelLoadFailed as i32, 10);
    rec.finish(true);
}

// ============================================================================
// Legacy types
// ============================================================================

#[test]
fn chat_message_roundtrip() {
    let rec = MetricsRecorder::start("unit::rpc::chat_message_roundtrip", "unit");
    let msg = ChatMessage {
        content: "Hello from kwaai".to_string(),
        role: "user".to_string(),
        conversation_id: Some("conv-abc".to_string()),
    };
    let bytes = msg.encode_to_vec();
    let decoded = ChatMessage::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded.content, msg.content);
    assert_eq!(decoded.role, msg.role);
    assert_eq!(decoded.conversation_id, msg.conversation_id);
    rec.finish(true);
}

#[test]
fn chat_token_done_flag() {
    let rec = MetricsRecorder::start("unit::rpc::chat_token_done_flag", "unit");
    let done_token = ChatToken {
        text: String::new(),
        done: true,
        finish_reason: Some("stop".to_string()),
    };
    let bytes = done_token.encode_to_vec();
    let decoded = ChatToken::decode(bytes.as_slice()).unwrap();
    assert!(decoded.done);
    assert_eq!(decoded.finish_reason.as_deref(), Some("stop"));
    rec.finish(true);
}

// ============================================================================
// Frame id correlation
// ============================================================================

#[test]
fn frame_id_preserved_through_encode_decode() {
    let rec = MetricsRecorder::start("unit::rpc::frame_id_preserved", "unit");
    for id in [1u64, u64::MAX, 12345, 0xDEADBEEF] {
        let frame = ClientFrame {
            id,
            body: Some(client_frame::Body::Ping(PingRequest {})),
        };
        let bytes = frame.encode_to_vec();
        let decoded = ClientFrame::decode(bytes.as_slice()).unwrap();
        assert_eq!(decoded.id, id, "id {id} not preserved");
    }
    rec.finish(true);
}

// ============================================================================
// Empty frame (no body) is valid protobuf
// ============================================================================

#[test]
fn empty_frame_is_valid_proto() {
    let rec = MetricsRecorder::start("unit::rpc::empty_frame_is_valid_proto", "unit");
    let frame = ClientFrame { id: 99, body: None };
    let bytes = frame.encode_to_vec();
    let decoded = ClientFrame::decode(bytes.as_slice()).unwrap();
    assert_eq!(decoded.id, 99);
    assert!(decoded.body.is_none());
    rec.finish(true);
}
