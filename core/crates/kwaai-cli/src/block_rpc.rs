//! Inter-node activation tensor protocol for distributed block sharding.
//!
//! Protocol ID: `/kwaai/inference/1.0.0`
//!
//! Message flow:
//! ```text
//! Coordinator                          Block Server
//!   │── InferenceRequest (msgpack) ──────────────▶│
//!   │   {session_id, seq_pos,                      │
//!   │    payload_type, shape, data}                 │
//!   │                                               │  runs local blocks
//!   │◀── InferenceResponse (msgpack) ──────────────│
//!   │   {session_id, response_type,                 │
//!   │    shape, data}                               │
//! ```
//!
//! Tensor data is serialised as raw little-endian bytes:
//! - Token IDs: `u32-LE` each
//! - Hidden states / logits: `f16-LE` each (native half precision)

use anyhow::{bail, Context, Result};
use candle_core::{DType, Device, Tensor};
use kwaai_inference::TransformerShard;
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error};

// ── Protocol constant ─────────────────────────────────────────────────────────

/// libp2p protocol string registered with the p2p daemon.
pub const INFERENCE_PROTO: &str = "/kwaai/inference/1.0.0";

// ── Wire types ────────────────────────────────────────────────────────────────

/// What kind of data the [`InferenceRequest`] payload carries.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PayloadType {
    /// Raw token IDs (first node only).  Each element is a `u32` LE.
    TokenIds,
    /// Floating-point hidden states.  Each element is an `f16` LE.
    HiddenStates,
}

/// What kind of data the [`InferenceResponse`] payload carries.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    /// Intermediate hidden states — pass to the next node.
    HiddenStates,
    /// Final logits from the last node — use for sampling.
    Logits,
}

/// Sent by the coordinator to a block server.
#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceRequest {
    /// Unique identifier for this inference session (links to KV-cache on each server).
    pub session_id: u64,
    /// Global sequence position of the first token in this call (0 for prefill).
    pub seq_pos: u32,
    /// Whether `data` contains token IDs or hidden states.
    pub payload_type: PayloadType,
    /// Tensor shape: `[seq_len]` for token IDs, `[1, seq_len, hidden_dim]` for hidden states.
    pub shape: Vec<u32>,
    /// Raw tensor bytes (u32-LE for token IDs, f16-LE for hidden states).
    pub data: Vec<u8>,
}

/// Sent by a block server back to the coordinator.
#[derive(Debug, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub session_id: u64,
    /// Whether `data` contains hidden states or final logits.
    pub response_type: ResponseType,
    pub shape: Vec<u32>,
    /// Raw f16-LE tensor bytes.
    pub data: Vec<u8>,
    /// Set when the server encountered an error.
    pub error: Option<String>,
}

// ── Tensor ↔ bytes helpers ────────────────────────────────────────────────────

/// Serialise a `Tensor` to raw `f16-LE` bytes.
pub fn tensor_to_f16_bytes(tensor: &Tensor) -> Result<(Vec<u32>, Vec<u8>)> {
    // Normalise to F16 on CPU for serialisation
    let t = tensor
        .to_dtype(DType::F16)
        .context("to_dtype F16")?
        .flatten_all()
        .context("flatten")?;

    let f16_vec: Vec<half::f16> = t.to_vec1().context("to_vec1 f16")?;
    let shape: Vec<u32> = tensor.dims().iter().map(|&d| d as u32).collect();

    // SAFETY: f16 is repr(transparent) over u16; we transmute to raw bytes
    let bytes: Vec<u8> = f16_vec.iter().flat_map(|v| v.to_le_bytes()).collect();

    Ok((shape, bytes))
}

/// Deserialise `f16-LE` bytes back to a `Tensor` on the given device.
pub fn f16_bytes_to_tensor(bytes: &[u8], shape: &[u32], device: &Device) -> Result<Tensor> {
    if !bytes.len().is_multiple_of(2) {
        bail!(
            "f16 byte buffer length {} is not a multiple of 2",
            bytes.len()
        );
    }
    let f16_vec: Vec<half::f16> = bytes
        .chunks_exact(2)
        .map(|c| half::f16::from_le_bytes([c[0], c[1]]))
        .collect();
    let shape_usize: Vec<usize> = shape.iter().map(|&d| d as usize).collect();
    Tensor::from_vec(f16_vec, shape_usize.as_slice(), device).context("Tensor::from_vec f16")
}

/// Serialise token IDs to raw `u32-LE` bytes.
pub fn token_ids_to_bytes(ids: &[u32]) -> (Vec<u32>, Vec<u8>) {
    let shape = vec![ids.len() as u32];
    let bytes = ids.iter().flat_map(|id| id.to_le_bytes()).collect();
    (shape, bytes)
}

/// Deserialise `u32-LE` bytes to a token ID slice.
pub fn bytes_to_token_ids(bytes: &[u8]) -> Result<Vec<u32>> {
    if !bytes.len().is_multiple_of(4) {
        bail!(
            "token_id byte buffer length {} is not a multiple of 4",
            bytes.len()
        );
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect())
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Call a block server's inference handler and return the decoded response.
///
/// `peer_id_bytes` should be obtained via `peer.to_bytes()` from a `libp2p::PeerId`.
pub async fn call_block_forward(
    client: &P2PClient,
    peer_id: &PeerId,
    request: &InferenceRequest,
) -> Result<InferenceResponse> {
    let peer_bytes = peer_id.to_bytes();
    let req_bytes = rmp_serde::to_vec_named(request).context("serialise InferenceRequest")?;

    debug!(
        session = request.session_id,
        seq_pos = request.seq_pos,
        "Calling inference on peer {}",
        peer_id
    );

    let resp_bytes = client
        .call_unary_handler(&peer_bytes, INFERENCE_PROTO, &req_bytes)
        .await
        .context("call_unary_handler")?;

    let response: InferenceResponse =
        rmp_serde::from_slice(&resp_bytes).context("deserialise InferenceResponse")?;

    if let Some(ref err) = response.error {
        bail!("Remote inference error: {err}");
    }

    Ok(response)
}

// ── Server handler factory ────────────────────────────────────────────────────

/// Build a unary handler function suitable for
/// [`P2PClient::add_unary_handler`] that dispatches incoming activation
/// tensors through the local shard.
///
/// The returned closure is `'static + Send + Sync` so it can be registered
/// with the p2p daemon.
#[allow(clippy::type_complexity)]
pub fn make_block_rpc_handler(
    shard: Arc<TransformerShard>,
    device: Device,
) -> impl Fn(
    Vec<u8>,
) -> std::pin::Pin<
    Box<dyn std::future::Future<Output = kwaai_p2p_daemon::error::Result<Vec<u8>>> + Send>,
> + Send
       + Sync
       + 'static {
    move |data: Vec<u8>| {
        let shard = shard.clone();
        let device = device.clone();
        Box::pin(async move {
            match handle_inference_request(&shard, &device, &data).await {
                Ok(resp) => rmp_serde::to_vec_named(&resp).map_err(|e| {
                    kwaai_p2p_daemon::error::Error::Protocol(format!(
                        "Failed to serialise response: {e}"
                    ))
                }),
                Err(e) => {
                    error!("Inference request failed: {e:#}");
                    // Return an error response rather than dropping the connection
                    let resp = InferenceResponse {
                        session_id: 0,
                        response_type: ResponseType::HiddenStates,
                        shape: vec![],
                        data: vec![],
                        error: Some(e.to_string()),
                    };
                    rmp_serde::to_vec_named(&resp).map_err(|e| {
                        kwaai_p2p_daemon::error::Error::Protocol(format!(
                            "Failed to serialise error response: {e}"
                        ))
                    })
                }
            }
        })
    }
}

/// Process one incoming inference request against the local shard.
pub async fn handle_inference_request(
    shard: &TransformerShard,
    device: &Device,
    raw: &[u8],
) -> Result<InferenceResponse> {
    let req: InferenceRequest =
        rmp_serde::from_slice(raw).context("deserialise InferenceRequest")?;

    let session_id = req.session_id;
    let seq_pos = req.seq_pos as usize;

    debug!(
        session = session_id,
        seq_pos,
        is_first = shard.is_first(),
        is_last = shard.is_last(),
        "Handling inference request"
    );

    // Dispatch based on payload type and node role
    let (output, is_logits) = match req.payload_type {
        PayloadType::TokenIds => {
            // Only the first node should receive token IDs
            if !shard.is_first() {
                bail!(
                    "Received TokenIds payload but this shard starts at block {} (not 0)",
                    shard.start_block
                );
            }
            let token_ids = bytes_to_token_ids(&req.data).context("decode token IDs")?;
            if shard.is_last() {
                // Single-node shard covers the whole model — embed + blocks + head in one pass
                let logits = shard.forward_full(session_id, &token_ids, seq_pos)?;
                (logits, true)
            } else {
                let hidden = shard.forward_first(session_id, &token_ids, seq_pos)?;
                (hidden, false)
            }
        }

        PayloadType::HiddenStates => {
            let hidden = f16_bytes_to_tensor(&req.data, &req.shape, device)
                .context("decode hidden states")?;
            if shard.is_last() {
                let logits = shard.forward_last(session_id, hidden, seq_pos)?;
                (logits, true)
            } else {
                let out = shard.forward_middle(session_id, hidden, seq_pos)?;
                (out, false)
            }
        }
    };

    // Serialise output tensor to f16 bytes
    let (shape, data) = tensor_to_f16_bytes(&output).context("serialise output tensor")?;

    Ok(InferenceResponse {
        session_id,
        response_type: if is_logits {
            ResponseType::Logits
        } else {
            ResponseType::HiddenStates
        },
        shape,
        data,
        error: None,
    })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_ids_round_trip() {
        let ids = vec![1u32, 42, 999, 32000];
        let (shape, bytes) = token_ids_to_bytes(&ids);
        assert_eq!(shape, vec![4]);
        let decoded = bytes_to_token_ids(&bytes).unwrap();
        assert_eq!(decoded, ids);
    }

    #[test]
    fn f16_bytes_round_trip() {
        use candle_core::{DType, Device, Tensor};
        let device = Device::Cpu;
        let data = vec![1.0f32, 2.0, 3.0, 4.0];
        let tensor = Tensor::from_vec(data.clone(), (1usize, 1usize, 4usize), &device)
            .unwrap()
            .to_dtype(DType::F16)
            .unwrap();
        let (shape, bytes) = tensor_to_f16_bytes(&tensor).unwrap();
        let recovered = f16_bytes_to_tensor(&bytes, &shape, &device).unwrap();
        assert_eq!(recovered.dims(), tensor.dims());
        // Check values approximately (f16 has limited precision)
        let vals: Vec<half::f16> = recovered.flatten_all().unwrap().to_vec1().unwrap();
        for (orig, got) in data.iter().zip(vals.iter()) {
            assert!((orig - got.to_f32()).abs() < 0.01);
        }
    }

    #[test]
    fn inference_request_msgpack_round_trip() {
        let req = InferenceRequest {
            session_id: 12345,
            seq_pos: 7,
            payload_type: PayloadType::HiddenStates,
            shape: vec![1, 1, 4096],
            data: vec![0u8; 8192],
        };
        let bytes = rmp_serde::to_vec_named(&req).unwrap();
        let decoded: InferenceRequest = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(decoded.session_id, req.session_id);
        assert_eq!(decoded.seq_pos, req.seq_pos);
        assert_eq!(decoded.shape, req.shape);
        assert_eq!(decoded.payload_type, req.payload_type);
    }
}
