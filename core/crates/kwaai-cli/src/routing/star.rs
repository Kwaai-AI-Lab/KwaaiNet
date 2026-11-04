//! Star routing: the originator calls every hop itself, and each activation
//! comes back to it before going out to the next server.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;

use super::hop::{classify_hop_failure, hop_candidates, ChainPositionError, HOP_TIMEOUT_MARKER};
use crate::block_rpc::{call_block_forward, InferenceRequest, PayloadType};
use crate::display::print_warning;
use crate::reputation::{now_secs, PeerObservation, ReputationStore};
use crate::shard_cmd::{
    emit_progress, BlockServerEntry, HopFailureKind, InferenceProgress, ProgressPhase,
    ProgressSink, TokenContext,
};

/// Send an `InferenceRequest` to the first peer in `chain`, routing the
/// activation tensor through each subsequent hop until the last returns
/// logits. `chain` is normally a pinned path, which is followed as planned;
/// see [`hop_candidates`] for how a hop is chosen and why a failed hop makes
/// the caller rebuild the path rather than this loop skipping ahead.
#[allow(clippy::too_many_arguments)]
pub async fn forward_through_chain(
    client: &mut P2PClient,
    chain: &[BlockServerEntry],
    total_blocks: usize,
    session_id: u64,
    seq_pos: u32,
    first_request: InferenceRequest,
    our_peer_id: Option<&PeerId>,
    failed_peers: &mut std::collections::HashSet<PeerId>,
    unreached: &std::collections::HashSet<PeerId>,
    sink: Option<&dyn ProgressSink>,
    reputation: Option<Arc<std::sync::Mutex<ReputationStore>>>,
    token_ctx: TokenContext,
) -> Result<crate::block_rpc::InferenceResponse> {
    forward_from(
        client,
        chain,
        total_blocks,
        session_id,
        seq_pos,
        first_request,
        0,
        our_peer_id,
        failed_peers,
        unreached,
        sink,
        reputation,
        token_ctx,
    )
    .await
}

/// [`forward_through_chain`] from `start_block`, where `first_request`
/// carries the activations that reached it — how a forward chain that stopped
/// part-way resumes in star.
#[allow(clippy::too_many_arguments)]
pub(super) async fn forward_from(
    client: &mut P2PClient,
    chain: &[BlockServerEntry],
    total_blocks: usize,
    session_id: u64,
    seq_pos: u32,
    first_request: InferenceRequest,
    start_block: usize,
    our_peer_id: Option<&PeerId>,
    failed_peers: &mut std::collections::HashSet<PeerId>,
    unreached: &std::collections::HashSet<PeerId>,
    sink: Option<&dyn ProgressSink>,
    reputation: Option<Arc<std::sync::Mutex<ReputationStore>>>,
    token_ctx: TokenContext,
) -> Result<crate::block_rpc::InferenceResponse> {
    use crate::block_rpc::InferenceResponse;

    let mut request = first_request;
    let mut response: Option<InferenceResponse> = None;
    let mut pos = start_block;

    while pos < total_blocks {
        let candidates = hop_candidates(chain, pos, failed_peers, unreached, our_peer_id);

        if candidates.is_empty() {
            return Err(ChainPositionError {
                block: pos,
                message: format!("No server covers block {pos} — chain has a gap (all candidates failed or blacklisted)"),
            }
            .into());
        }

        // Per-hop deadline: 5 minutes is generous enough for slow CPU prefill of
        // large models while still eventually unblocking a hung/crashed peer.
        const HOP_TIMEOUT: Duration = Duration::from_secs(300);

        let mut succeeded = false;
        for (cand_idx, candidate) in candidates.iter().enumerate() {
            // Our own shard is called like any other: the daemon loops a
            // call to its own peer id back to the local handler.
            let is_self = our_peer_id == Some(&candidate.peer_id);
            emit_progress(sink, || {
                InferenceProgress::new(token_ctx.started, ProgressPhase::HopStart)
                    .peer(candidate, is_self)
                    .blocks(candidate.start_block, candidate.end_block)
                    .token(token_ctx)
                    .candidate(cand_idx)
            });
            let hop_start = std::time::Instant::now();
            let result = tokio::time::timeout(HOP_TIMEOUT, async {
                call_block_forward(client, &candidate.peer_id, &request).await
            })
            .await
            .unwrap_or_else(|_| {
                Err(anyhow::anyhow!(
                    "{HOP_TIMEOUT_MARKER} {}s — peer unresponsive or compute too slow",
                    HOP_TIMEOUT.as_secs()
                ))
            });
            let hop_ms = hop_start.elapsed().as_secs_f64() * 1000.0;

            match result {
                Ok(resp) => {
                    // Record successful hop in local reputation store.
                    if let Some(ref rep) = reputation {
                        if let Ok(mut store) = rep.lock() {
                            store.record(
                                &candidate.peer_id.to_base58(),
                                &candidate.public_name,
                                PeerObservation {
                                    timestamp_secs: now_secs(),
                                    latency_ms: hop_ms,
                                    success: true,
                                    observed_tps: None,
                                    claimed_tps: None,
                                    lease_outcome: None,
                                },
                            );
                        }
                    }
                    emit_progress(sink, || {
                        InferenceProgress::new(token_ctx.started, ProgressPhase::HopOk)
                            .peer(candidate, is_self)
                            .blocks(candidate.start_block, candidate.end_block)
                            .token(token_ctx)
                            .candidate(cand_idx)
                            .ms(hop_ms)
                            .ok(true)
                    });
                    pos = candidate.end_block;
                    if pos < total_blocks {
                        request = InferenceRequest {
                            session_id,
                            seq_pos,
                            payload_type: PayloadType::HiddenStates,
                            shape: resp.shape.clone(),
                            data: resp.data.clone(),
                        };
                    }
                    response = Some(resp);
                    succeeded = true;
                    break;
                }
                Err(e) => {
                    // Record failed hop in local reputation store.
                    if let Some(ref rep) = reputation {
                        if let Ok(mut store) = rep.lock() {
                            store.record(
                                &candidate.peer_id.to_base58(),
                                &candidate.public_name,
                                PeerObservation {
                                    timestamp_secs: now_secs(),
                                    latency_ms: hop_ms,
                                    success: false,
                                    observed_tps: None,
                                    claimed_tps: None,
                                    lease_outcome: None,
                                },
                            );
                        }
                    }
                    let err_str = format!("{e:#}");
                    let kind = classify_hop_failure(&err_str);
                    emit_progress(sink, || {
                        InferenceProgress::new(token_ctx.started, ProgressPhase::HopFailed)
                            .peer(candidate, is_self)
                            .blocks(candidate.start_block, candidate.end_block)
                            .token(token_ctx)
                            .candidate(cand_idx)
                            .ms(hop_ms)
                            .failure(kind)
                            .msg(err_str.clone())
                    });
                    // Protocol negotiation failures mean the peer has no inference
                    // handler registered — blacklist for the session.
                    // Transient stream errors are NOT blacklisted: the peer may recover.
                    match kind {
                        HopFailureKind::NoHandler => {
                            failed_peers.insert(candidate.peer_id);
                            print_warning(&format!(
                                "Peer {} ({}) has no inference handler — skipping for this session",
                                candidate
                                    .peer_id
                                    .to_base58()
                                    .chars()
                                    .take(12)
                                    .collect::<String>(),
                                candidate.public_name,
                            ));
                        }
                        HopFailureKind::Transient => {
                            // Transient: warn but keep peer eligible for future tokens.
                            print_warning(&format!(
                                "Peer {} ({}) transient error (not blacklisted): {e:#}",
                                candidate
                                    .peer_id
                                    .to_base58()
                                    .chars()
                                    .take(12)
                                    .collect::<String>(),
                                candidate.public_name,
                            ));
                        }
                        // A timeout is a peer that never answered, so it is
                        // blacklisted like any other hard failure — it is split
                        // out only so telemetry can name it.
                        HopFailureKind::Timeout | HopFailureKind::Other => {
                            failed_peers.insert(candidate.peer_id);
                            print_warning(&format!(
                                "Peer {} ({}) failed: {e:#}",
                                candidate
                                    .peer_id
                                    .to_base58()
                                    .chars()
                                    .take(12)
                                    .collect::<String>(),
                                candidate.public_name,
                            ));
                        }
                    }
                }
            }
        }

        if !succeeded {
            return Err(ChainPositionError {
                block: pos,
                message: format!(
                    "All {} candidate(s) for block {pos} failed",
                    candidates.len()
                ),
            }
            .into());
        }
    }

    response.ok_or_else(|| anyhow::anyhow!("Empty chain — no peers to forward through"))
}
