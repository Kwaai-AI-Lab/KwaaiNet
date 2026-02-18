//! Hivemind/Petals Protocol Compatibility
//!
//! This module provides compatibility with the Petals/Hivemind distributed ML network,
//! enabling KwaaiNet nodes to participate in both networks.
//!
//! ## Protocol Format
//!
//! Hivemind uses a simple framing protocol over libp2p streams:
//! ```text
//! ┌─────────────────────────────────────┐
//! │ Length (8 bytes, big-endian)        │
//! ├─────────────────────────────────────┤
//! │ Marker (1 byte): 0x00=msg, 0x01=err │
//! ├─────────────────────────────────────┤
//! │ Protobuf payload                    │
//! └─────────────────────────────────────┘
//! ```

use prost::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, warn};

/// Protocol marker bytes
pub const MARKER_MESSAGE: u8 = 0x00;
pub const MARKER_ERROR: u8 = 0x01;

/// Hivemind protocol stream protocol ID
pub const HIVEMIND_PROTOCOL: &str = "/hivemind/0.0.0/rpc";

// =============================================================================
// Protobuf Messages (manually defined to avoid build.rs complexity)
// =============================================================================

/// ExpertUID - request message for rpc_info
#[derive(Clone, PartialEq, Message)]
pub struct ExpertUID {
    /// Expert/block identifier (optional for general server info)
    #[prost(string, tag = "1")]
    pub uid: String,
}

/// ExpertInfo - response message for rpc_info
#[derive(Clone, PartialEq, Message)]
pub struct ExpertInfo {
    /// MessagePack-serialized server info
    #[prost(bytes = "vec", tag = "1")]
    pub serialized_info: Vec<u8>,
}

// =============================================================================
// Server Info Structure (MessagePack serialized)
// =============================================================================

/// Server information returned by rpc_info
///
/// This is serialized as MessagePack inside ExpertInfo.serialized_info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server state: "online", "offline", "joining"
    pub state: String,

    /// Server version (e.g., "2.3.0.dev2")
    pub version: String,

    /// Public display name for the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_name: Option<String>,

    /// Start of block range
    pub start_block: u32,

    /// End of block range
    pub end_block: u32,

    /// Current throughput (tokens per second)
    pub throughput: f32,

    /// Network RPS (requests per second)
    pub network_rps: f32,

    /// Forward pass RPS
    pub forward_rps: f32,

    /// Inference RPS
    pub inference_rps: f32,

    /// PyTorch dtype equivalent (e.g., "float16", "bfloat16")
    pub torch_dtype: String,

    /// Quantization type (e.g., "none", "nf4")
    pub quant_type: String,

    /// Available cache tokens for inference
    pub cache_tokens_left: u64,

    /// Whether this node is using libp2p relay
    pub using_relay: bool,

    /// Whether this node is in DHT client mode only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dht_client_mode: Option<bool>,

    /// LoRA adapters list
    pub adapters: Vec<String>,

    /// Next ping targets (peer_id -> optional latency)
    pub next_pings: HashMap<String, Option<f64>>,

    /// Block span this server provides [start, end) - legacy field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<(u32, u32)>>,

    /// Additional metadata
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl Default for ServerInfo {
    fn default() -> Self {
        Self {
            state: "online".to_string(),
            version: "2.3.0.dev2".to_string(), // Match Petals version
            public_name: None,
            start_block: 0,
            end_block: 1,
            throughput: 10.0,
            network_rps: 100.0,
            forward_rps: 50.0,
            inference_rps: 10.0,
            torch_dtype: "float16".to_string(),
            quant_type: "none".to_string(),
            cache_tokens_left: 32768,
            using_relay: false,
            dht_client_mode: Some(false),
            adapters: Vec::new(),
            next_pings: HashMap::new(),
            spans: Some(vec![(0, 1)]),
            extra: HashMap::new(),
        }
    }
}

impl ServerInfo {
    /// Create a new ServerInfo with the given public name
    pub fn new(public_name: impl Into<String>) -> Self {
        Self {
            public_name: Some(public_name.into()),
            ..Default::default()
        }
    }

    /// Set the block span this server provides
    pub fn with_span(mut self, start: u32, end: u32) -> Self {
        self.start_block = start;
        self.end_block = end;
        self.spans = Some(vec![(start, end)]);
        self
    }

    /// Set available cache tokens
    pub fn with_cache_tokens(mut self, tokens: u64) -> Self {
        self.cache_tokens_left = tokens;
        self
    }

    /// Set throughput
    pub fn with_throughput(mut self, tps: f32) -> Self {
        self.throughput = tps;
        self
    }

    /// Set torch dtype
    pub fn with_dtype(mut self, dtype: impl Into<String>) -> Self {
        self.torch_dtype = dtype.into();
        self
    }

    /// Set relay usage flag
    pub fn with_relay(mut self, using_relay: bool) -> Self {
        self.using_relay = using_relay;
        self
    }

    /// Serialize to MessagePack bytes
    pub fn to_msgpack(&self) -> Result<Vec<u8>, rmp_serde::encode::Error> {
        rmp_serde::to_vec_named(self)
    }

    /// Deserialize from MessagePack bytes
    pub fn from_msgpack(data: &[u8]) -> Result<Self, rmp_serde::decode::Error> {
        rmp_serde::from_slice(data)
    }

    /// Create ExpertInfo protobuf response
    pub fn to_expert_info(&self) -> Result<ExpertInfo, rmp_serde::encode::Error> {
        Ok(ExpertInfo {
            serialized_info: self.to_msgpack()?,
        })
    }
}

// =============================================================================
// Message Framing
// =============================================================================

/// Encode a protobuf message with Hivemind framing
pub fn encode_message<M: Message>(msg: &M) -> Vec<u8> {
    let payload = msg.encode_to_vec();
    let len = payload.len() as u64 + 1; // +1 for marker byte

    debug!("Encoding Hivemind message: {} bytes payload", payload.len());
    let mut result = Vec::with_capacity(8 + 1 + payload.len());
    result.extend_from_slice(&len.to_be_bytes());
    result.push(MARKER_MESSAGE);
    result.extend_from_slice(&payload);
    result
}

/// Encode an error response with Hivemind framing
pub fn encode_error(error_msg: &str) -> Vec<u8> {
    warn!("Encoding Hivemind error response: {}", error_msg);
    let payload = error_msg.as_bytes();
    let len = payload.len() as u64 + 1;

    let mut result = Vec::with_capacity(8 + 1 + payload.len());
    result.extend_from_slice(&len.to_be_bytes());
    result.push(MARKER_ERROR);
    result.extend_from_slice(payload);
    result
}

/// Decode the length prefix from a Hivemind message
pub fn decode_length(data: &[u8]) -> Option<u64> {
    if data.len() < 8 {
        warn!("decode_length: buffer too short ({} bytes)", data.len());
        return None;
    }
    Some(u64::from_be_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]))
}

/// Decode a Hivemind message, returning (is_error, payload)
pub fn decode_message(data: &[u8]) -> Option<(bool, &[u8])> {
    if data.len() < 9 {
        warn!("decode_message: buffer too short ({} bytes)", data.len());
        return None;
    }

    let len = decode_length(data)? as usize;
    if data.len() < 8 + len {
        warn!("decode_message: incomplete message (have {}, need {})", data.len(), 8 + len);
        return None;
    }

    let marker = data[8];
    let payload = &data[9..8 + len];

    let is_error = marker == MARKER_ERROR;
    debug!("Decoded Hivemind message: is_error={}, payload={} bytes", is_error, payload.len());
    Some((is_error, payload))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_info_msgpack() {
        let info = ServerInfo::new("test-node")
            .with_span(0, 8)
            .with_cache_tokens(1000)
            .with_throughput(15.5);

        let encoded = info.to_msgpack().unwrap();
        let decoded = ServerInfo::from_msgpack(&encoded).unwrap();

        assert_eq!(decoded.public_name, Some("test-node".to_string()));
        assert_eq!(decoded.spans, Some(vec![(0, 8)]));
        assert_eq!(decoded.start_block, 0);
        assert_eq!(decoded.end_block, 8);
        assert_eq!(decoded.cache_tokens_left, 1000);
        assert_eq!(decoded.throughput, 15.5);
    }

    #[test]
    fn test_message_framing() {
        let uid = ExpertUID {
            uid: "test".to_string(),
        };

        let encoded = encode_message(&uid);
        let (is_error, payload) = decode_message(&encoded).unwrap();

        assert!(!is_error);
        let decoded = ExpertUID::decode(payload).unwrap();
        assert_eq!(decoded.uid, "test");
    }

    #[test]
    fn test_error_framing() {
        let error = encode_error("Something went wrong");
        let (is_error, payload) = decode_message(&error).unwrap();

        assert!(is_error);
        assert_eq!(std::str::from_utf8(payload).unwrap(), "Something went wrong");
    }
}
