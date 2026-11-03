//! Forward-chain messages. msgpack with named fields, like `block_rpc`.

use serde::{Deserialize, Serialize};

use crate::block_rpc::InferenceRequest;

/// Servers take a chain step here. The reply is a [`ChainAck`], sent before
/// any compute — never the result.
pub const CHAIN_PROTO: &str = "/kwaai/inference-chain/1.0.0";

/// The originator's per-run return protocol is this plus the session id in
/// hex. Per run because handler registration is last-writer-wins: a shared
/// name would let two runs on one node take each other's results.
pub const RESULT_PROTO_PREFIX: &str = "/kwaai/inference-result/1.0.0/";

pub fn result_proto(session_id: u64) -> String {
    format!("{RESULT_PROTO_PREFIX}{session_id:016x}")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteHop {
    pub peer_id: Vec<u8>,
    pub start_block: u32,
    pub end_block: u32,
}

/// One server's part in a step, appended as the step passes through.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HopTrace {
    pub peer_id: Vec<u8>,
    pub start_block: u32,
    pub end_block: u32,
    pub fwd_ms: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainRequest {
    pub inner: InferenceRequest,
    /// The hops after the receiving server, in order.
    pub route: Vec<RouteHop>,
    /// Where the result goes, and on which protocol.
    pub origin: Vec<u8>,
    pub return_proto: String,
    /// Matches the result to the originator's waiting step.
    pub step: u64,
    pub trace: Vec<HopTrace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainAck {
    pub accepted: bool,
    pub error: Option<String>,
}

/// Why an onward hop did not happen. `Refused` and `Unreached` mean the next
/// server provably ran nothing, so the originator can resume star from here;
/// `Failed` means it may have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForwardErrorKind {
    Refused,
    Unreached,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForwardError {
    pub peer_id: Vec<u8>,
    pub block: u32,
    pub kind: ForwardErrorKind,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChainOutcome {
    /// The last block ran: the step is complete.
    Logits { shape: Vec<u32>, data: Vec<u8> },
    /// The sender ran its blocks but could not pass them on.
    Partial {
        end_block: u32,
        shape: Vec<u32>,
        data: Vec<u8>,
        error: ForwardError,
    },
    /// The sender's own compute failed after it acked.
    HopFailed {
        peer_id: Vec<u8>,
        block: u32,
        msg: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainResult {
    pub session_id: u64,
    pub step: u64,
    pub outcome: ChainOutcome,
    pub trace: Vec<HopTrace>,
}

/// Which [`ForwardErrorKind`] an onward call's error is. Only what proves the
/// next server ran nothing may say so.
pub fn classify_forward_error(err: &str) -> ForwardErrorKind {
    if err.contains("protocols not supported") || err.contains("refused:") {
        ForwardErrorKind::Refused
    } else if err.contains("Dial failed")
        || err.contains("Connection failed")
        || err.contains("Peer not found")
    {
        ForwardErrorKind::Unreached
    } else {
        ForwardErrorKind::Failed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block_rpc::PayloadType;

    #[test]
    fn a_chain_request_round_trips() {
        let req = ChainRequest {
            inner: InferenceRequest {
                session_id: 7,
                seq_pos: 3,
                payload_type: PayloadType::HiddenStates,
                shape: vec![1, 1, 4],
                data: vec![1, 2, 3, 4, 5, 6, 7, 8],
            },
            route: vec![RouteHop {
                peer_id: vec![9; 38],
                start_block: 8,
                end_block: 16,
            }],
            origin: vec![1; 38],
            return_proto: result_proto(7),
            step: 2,
            trace: vec![],
        };
        let bytes = rmp_serde::to_vec_named(&req).unwrap();
        let back: ChainRequest = rmp_serde::from_slice(&bytes).unwrap();
        assert_eq!(back.route, req.route);
        assert_eq!(back.inner.data, req.inner.data);
        assert_eq!(
            back.return_proto,
            "/kwaai/inference-result/1.0.0/0000000000000007"
        );
    }

    #[test]
    fn a_result_round_trips_every_outcome() {
        for outcome in [
            ChainOutcome::Logits {
                shape: vec![1, 1, 2],
                data: vec![0; 4],
            },
            ChainOutcome::Partial {
                end_block: 8,
                shape: vec![1, 1, 2],
                data: vec![0; 4],
                error: ForwardError {
                    peer_id: vec![2],
                    block: 8,
                    kind: ForwardErrorKind::Unreached,
                    msg: "x".into(),
                },
            },
            ChainOutcome::HopFailed {
                peer_id: vec![3],
                block: 16,
                msg: "y".into(),
            },
        ] {
            let r = ChainResult {
                session_id: 1,
                step: 2,
                outcome,
                trace: vec![],
            };
            let back: ChainResult =
                rmp_serde::from_slice(&rmp_serde::to_vec_named(&r).unwrap()).unwrap();
            assert_eq!(format!("{:?}", back.outcome), format!("{:?}", r.outcome));
        }
    }

    #[test]
    fn only_provable_non_runs_avoid_failed() {
        use ForwardErrorKind::*;
        assert_eq!(classify_forward_error("protocols not supported"), Refused);
        assert_eq!(classify_forward_error("refused: node warming up"), Refused);
        assert_eq!(
            classify_forward_error("Dial failed: no addresses"),
            Unreached
        );
        assert_eq!(classify_forward_error("Peer not found: 12D3"), Unreached);
        assert_eq!(classify_forward_error("stream reset"), Failed);
        assert_eq!(
            classify_forward_error("Request timed out after 30000ms"),
            Failed
        );
    }
}
