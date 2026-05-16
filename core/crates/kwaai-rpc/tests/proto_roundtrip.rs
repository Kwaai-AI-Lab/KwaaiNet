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

use kwaai_rpc::v1::{ChatMessage, ChatToken};
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
    assert_eq!(decoded.content, original.content, "content survives round-trip");
    assert_eq!(decoded.role, original.role, "role survives round-trip");
    assert_eq!(
        decoded.conversation_id, original.conversation_id,
        "optional conversation_id (tag 3) survives as Some(..)"
    );
    assert_eq!(decoded, original, "full ChatMessage equality after round-trip");
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
    assert_eq!(decoded.conversation_id, None, "absent optional stays absent");
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
