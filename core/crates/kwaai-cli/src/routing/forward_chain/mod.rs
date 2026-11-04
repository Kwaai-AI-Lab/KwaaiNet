//! Forward-chain routing, originator side: the step is sent to the first hop
//! with the rest of the route, each server forwards to the next, and the last
//! delivers the logits back here — N+1 network legs instead of star's 2N.
//!
//! Phase 1 is optimistic and originator-planned. Prefill stays star; any
//! failure finishes the step in star where provably safe, and the rest of
//! the run uses star.

pub mod serve;
pub mod wire;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use kwaai_p2p_daemon::P2PClient;
use libp2p::PeerId;
use tokio::sync::oneshot;

use self::serve::send_chain;
use self::wire::*;
use super::hop::ChainPositionError;
use super::star::forward_from;
use crate::block_rpc::{InferenceRequest, InferenceResponse, PayloadType, ResponseType};
use crate::display::print_warning;
use crate::reputation::ReputationStore;
use crate::shard_cmd::{
    daemon_socket, emit_progress, BlockServerEntry, InferenceProgress, ProgressPhase, ProgressSink,
    TokenContext,
};

/// How long one hop of a decode step may take before the originator stops
/// waiting for the result. Decode only, so generous.
const HOP_BUDGET: Duration = Duration::from_secs(30);

/// Waiting steps, each with the session it was sent under: a retry restarts
/// the session mid-run.
type Pending = Arc<Mutex<HashMap<u64, (u64, oneshot::Sender<ChainResult>)>>>;

/// One run's forward-chain state. Its own daemon connection owns the return
/// handler, so dropping this releases it.
pub struct ChainOriginator {
    _returns: P2PClient,
    return_proto: String,
    origin: Vec<u8>,
    pending: Pending,
    next_step: u64,
    /// Steps that completed as a forward chain.
    pub steps: usize,
    /// Why the run went back to star, once it has.
    pub fallback: Option<String>,
}

impl ChainOriginator {
    pub async fn open(session_id: u64, origin: &PeerId) -> Result<Self> {
        let client = P2PClient::connect(&daemon_socket())
            .await
            .context("connect forward-chain return client")?;
        let pending = Pending::default();
        let return_proto = result_proto(session_id);
        let p = pending.clone();
        client
            .add_unary_handler(
                &return_proto,
                move |data: Vec<u8>| {
                    let ack = accept_result(&p, &data);
                    async move {
                        rmp_serde::to_vec_named(&ack).map_err(|e| {
                            kwaai_p2p_daemon::error::Error::Protocol(format!(
                                "serialise ChainAck: {e}"
                            ))
                        })
                    }
                },
                false,
            )
            .await
            .context("register forward-chain return handler")?;
        Ok(Self {
            _returns: client,
            return_proto,
            origin: origin.to_bytes(),
            pending,
            next_step: 0,
            steps: 0,
            fallback: None,
        })
    }

    fn fall_back(&mut self, why: String) {
        if self.fallback.is_none() {
            print_warning(&format!("forward chain: {why} — rest of the run uses star"));
            self.fallback = Some(why);
        }
    }

    /// One decode step as a forward chain along `chain`, the pinned path.
    #[allow(clippy::too_many_arguments)]
    pub async fn step(
        &mut self,
        client: &mut P2PClient,
        chain: &[BlockServerEntry],
        total_blocks: usize,
        session_id: u64,
        seq_pos: u32,
        request: InferenceRequest,
        our_peer_id: Option<&PeerId>,
        failed_peers: &mut HashSet<PeerId>,
        unreached: &HashSet<PeerId>,
        sink: Option<&dyn ProgressSink>,
        reputation: Option<Arc<Mutex<ReputationStore>>>,
        token_ctx: TokenContext,
    ) -> Result<InferenceResponse> {
        let step = self.next_step;
        self.next_step += 1;
        let route: Vec<RouteHop> = chain[1..]
            .iter()
            .map(|e| RouteHop {
                peer_id: e.peer_id.to_bytes(),
                start_block: e.start_block as u32,
                end_block: e.end_block as u32,
            })
            .collect();
        let (tx, rx) = oneshot::channel();
        lock(&self.pending).insert(step, (session_id, tx));
        let req = ChainRequest {
            inner: request,
            route,
            origin: self.origin.clone(),
            return_proto: self.return_proto.clone(),
            step,
            trace: Vec::new(),
        };

        let started = Instant::now();
        if let Err(e) = send_chain(&*client, &chain[0].peer_id.to_bytes(), &req).await {
            lock(&self.pending).remove(&step);
            let msg = format!("{e:#}");
            self.fall_back(format!("first hop {}: {msg}", chain[0].public_name));
            if classify_forward_error(&msg) == ForwardErrorKind::Failed {
                return Err(anyhow!(
                    "forward chain: first hop {} failed: {msg}",
                    chain[0].public_name
                ));
            }
            return forward_from(
                client,
                chain,
                total_blocks,
                session_id,
                seq_pos,
                req.inner,
                0,
                our_peer_id,
                failed_peers,
                unreached,
                sink,
                reputation,
                token_ctx,
            )
            .await;
        }

        let deadline = HOP_BUDGET * chain.len() as u32;
        let result = match tokio::time::timeout(deadline, rx).await {
            Ok(Ok(r)) => r,
            _ => {
                lock(&self.pending).remove(&step);
                self.fall_back(format!("no result within {}s", deadline.as_secs()));
                return Err(anyhow!(
                    "forward chain: no result within {}s (a hop may have run — KV state unknown)",
                    deadline.as_secs()
                ));
            }
        };
        emit_trace(sink, chain, our_peer_id, &result.trace, token_ctx);

        match result.outcome {
            ChainOutcome::Logits { shape, data } => {
                self.steps += 1;
                tracing::debug!(
                    step,
                    total_ms = started.elapsed().as_secs_f64() * 1000.0,
                    "forward chain step"
                );
                Ok(InferenceResponse {
                    session_id,
                    response_type: ResponseType::Logits,
                    shape,
                    data,
                    error: None,
                })
            }
            ChainOutcome::Partial {
                end_block,
                shape,
                data,
                error,
            } if error.kind != ForwardErrorKind::Failed => {
                self.fall_back(format!(
                    "{:?} at block {} ({}): {}",
                    error.kind,
                    error.block,
                    name_of(chain, &error.peer_id),
                    error.msg
                ));
                let resume = InferenceRequest {
                    session_id,
                    seq_pos,
                    payload_type: PayloadType::HiddenStates,
                    shape,
                    data,
                };
                forward_from(
                    client,
                    chain,
                    total_blocks,
                    session_id,
                    seq_pos,
                    resume,
                    end_block as usize,
                    our_peer_id,
                    failed_peers,
                    unreached,
                    sink,
                    reputation,
                    token_ctx,
                )
                .await
            }
            ChainOutcome::Partial { error, .. } => {
                Err(self.hop_failed(chain, failed_peers, &error.peer_id, error.block, &error.msg))
            }
            ChainOutcome::HopFailed {
                peer_id,
                block,
                msg,
            } => Err(self.hop_failed(chain, failed_peers, &peer_id, block, &msg)),
        }
    }

    /// A hop that may have touched its KV: blacklist it and let the caller
    /// rebuild the path, exactly as a failed star hop does.
    fn hop_failed(
        &mut self,
        chain: &[BlockServerEntry],
        failed_peers: &mut HashSet<PeerId>,
        peer: &[u8],
        block: u32,
        msg: &str,
    ) -> anyhow::Error {
        let name = name_of(chain, peer);
        self.fall_back(format!("hop {name} at block {block} failed: {msg}"));
        if let Ok(id) = PeerId::from_bytes(peer) {
            failed_peers.insert(id);
        }
        ChainPositionError {
            block: block as usize,
            message: format!("forward chain: hop {name} at block {block} failed: {msg}"),
        }
        .into()
    }
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

/// Hand a delivered result to the step waiting for it.
fn accept_result(pending: &Pending, data: &[u8]) -> ChainAck {
    let nack = |e: String| ChainAck {
        accepted: false,
        error: Some(e),
    };
    let r: ChainResult = match rmp_serde::from_slice(data) {
        Ok(r) => r,
        Err(e) => return nack(format!("deserialise ChainResult: {e}")),
    };
    let mut waiting = lock(pending);
    match waiting.get(&r.step) {
        None => return nack(format!("step {} is not waiting", r.step)),
        Some((sid, _)) if *sid != r.session_id => {
            return nack(format!("session {} is not step {}'s", r.session_id, r.step))
        }
        Some(_) => {}
    }
    let (_, tx) = waiting
        .remove(&r.step)
        .expect("checked above under this lock");
    let step = r.step;
    match tx.send(r) {
        Ok(()) => ChainAck {
            accepted: true,
            error: None,
        },
        Err(_) => nack(format!("step {step} no longer waiting")),
    }
}

fn name_of(chain: &[BlockServerEntry], peer: &[u8]) -> String {
    chain
        .iter()
        .find(|e| e.peer_id.to_bytes() == peer)
        .map(|e| e.public_name.clone())
        .unwrap_or_else(|| {
            PeerId::from_bytes(peer)
                .map(|p| p.to_base58())
                .unwrap_or_else(|_| "?".into())
        })
}

/// One `HopOk` per hop the step passed through. `ms` is that hop's compute:
/// in a forward chain the originator times only the whole step.
fn emit_trace(
    sink: Option<&dyn ProgressSink>,
    chain: &[BlockServerEntry],
    our_peer_id: Option<&PeerId>,
    trace: &[HopTrace],
    token_ctx: TokenContext,
) {
    for (i, h) in trace.iter().enumerate() {
        let Some(entry) = chain.iter().find(|e| e.peer_id.to_bytes() == h.peer_id) else {
            continue;
        };
        emit_progress(sink, || {
            InferenceProgress::new(token_ctx.started, ProgressPhase::HopOk)
                .peer(entry, our_peer_id == Some(&entry.peer_id))
                .blocks(h.start_block as usize, h.end_block as usize)
                .token(token_ctx)
                .candidate(i)
                .ms(h.fwd_ms as f64)
                .ok(true)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(step: u64, session_id: u64) -> Vec<u8> {
        rmp_serde::to_vec_named(&ChainResult {
            session_id,
            step,
            outcome: ChainOutcome::Logits {
                shape: vec![1, 1, 1],
                data: vec![0, 0],
            },
            trace: vec![],
        })
        .unwrap()
    }

    #[test]
    fn a_result_reaches_only_the_step_waiting_for_it() {
        let pending = Pending::default();
        let (tx, mut rx) = oneshot::channel();
        lock(&pending).insert(4, (7, tx));

        assert!(
            !accept_result(&pending, &result(4, 8)).accepted,
            "another session's"
        );
        assert!(
            !accept_result(&pending, &result(5, 7)).accepted,
            "unknown step"
        );
        assert!(rx.try_recv().is_err());

        assert!(accept_result(&pending, &result(4, 7)).accepted);
        assert_eq!(rx.try_recv().unwrap().step, 4);
        assert!(!accept_result(&pending, &result(4, 7)).accepted, "a replay");

        let (tx, mut rx) = oneshot::channel();
        lock(&pending).insert(5, (9, tx));
        assert!(
            accept_result(&pending, &result(5, 9)).accepted,
            "a retry's fresh session"
        );
        assert_eq!(rx.try_recv().unwrap().step, 5);
    }
}
