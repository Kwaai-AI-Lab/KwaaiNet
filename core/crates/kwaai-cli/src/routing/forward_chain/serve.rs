//! The server half: take a chain step, ack at once, compute, then pass it on
//! — or, at the end of the route or on failure, report to the originator.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use libp2p::PeerId;
use tokio::sync::Semaphore;

use anyhow::{bail, Context, Result};
use candle_core::Device;
use futures::future::BoxFuture;
use kwaai_p2p_daemon::P2PClient;
use tracing::{debug, info, warn};

use super::wire::*;
use crate::block_rpc::{
    run_inference_request, InferenceRequest, InferenceResponse, LoadedShard, PayloadType,
    ResponseType, ShardCell,
};

/// Steps computing at once. Past this a step is refused, which its
/// originator handles by finishing in star.
const MAX_IN_FLIGHT: usize = 8;

/// How long passing a step on, or delivering a result, may take. The slot is
/// already free by then; this bounds the task.
const HAND_OFF_TIMEOUT: Duration = Duration::from_secs(30);

/// A unary call to a peer — the daemon in production, a fake in tests.
pub trait Unary: Send + Sync + 'static {
    fn call<'a>(
        &'a self,
        peer: &'a [u8],
        proto: &'a str,
        data: Vec<u8>,
    ) -> BoxFuture<'a, Result<Vec<u8>>>;
}

impl Unary for P2PClient {
    fn call<'a>(
        &'a self,
        peer: &'a [u8],
        proto: &'a str,
        data: Vec<u8>,
    ) -> BoxFuture<'a, Result<Vec<u8>>> {
        Box::pin(async move {
            self.call_unary_handler(peer, proto, &data)
                .await
                .map_err(|e| anyhow::anyhow!("{e:#}"))
        })
    }
}

/// Send a chain step to `peer`. `Ok` means it accepted; the result travels
/// on without us.
pub async fn send_chain<U: Unary + ?Sized>(u: &U, peer: &[u8], req: &ChainRequest) -> Result<()> {
    let bytes = rmp_serde::to_vec_named(req).context("serialise ChainRequest")?;
    let resp = u.call(peer, CHAIN_PROTO, bytes).await?;
    let ack: ChainAck = rmp_serde::from_slice(&resp).context("deserialise ChainAck")?;
    if !ack.accepted {
        bail!("refused: {}", ack.error.unwrap_or_default());
    }
    Ok(())
}

async fn deliver<U: Unary + ?Sized>(u: &U, origin: &[u8], proto: &str, r: &ChainResult) {
    let sent = tokio::time::timeout(HAND_OFF_TIMEOUT, async {
        let bytes = rmp_serde::to_vec_named(r)?;
        let ack: ChainAck = rmp_serde::from_slice(&u.call(origin, proto, bytes).await?)?;
        if !ack.accepted {
            bail!("originator refused: {}", ack.error.unwrap_or_default());
        }
        anyhow::Ok(())
    });
    if let Err(e) = sent
        .await
        .unwrap_or_else(|_| Err(anyhow::anyhow!("timed out")))
    {
        warn!(
            step = r.step,
            "forward chain: result not delivered to originator: {e:#}"
        );
    }
}

/// Refuse a step that cannot be served as routed, before acking it.
pub fn check_route(
    blocks: (u32, u32),
    is_last: bool,
    me: &[u8],
    caller: &[u8],
    req: &ChainRequest,
) -> Result<(), String> {
    if req.start_block != blocks.0 {
        return Err(format!(
            "planned to start at block {}, this shard starts at {}",
            req.start_block, blocks.0
        ));
    }
    if !req.return_proto.starts_with(RESULT_PROTO_PREFIX)
        || PeerId::from_bytes(&req.origin).is_err()
    {
        return Err("result address is not an originator's".into());
    }
    // A first hop takes steps only from their originator, a later one only
    // from the hop before, and token ids only at the first. Both are fields
    // the sender writes, so this names the sender; it does not authenticate it.
    match req.trace.last() {
        None if caller != req.origin.as_slice() => {
            return Err("a first hop takes steps only from their originator".into())
        }
        Some(prev) if caller != prev.peer_id.as_slice() || prev.end_block != blocks.0 => {
            return Err("step does not come from the hop before".into())
        }
        Some(_) if req.inner.payload_type == PayloadType::TokenIds => {
            return Err("token ids past the first hop".into())
        }
        _ => {}
    }
    // Only single-token decode steps travel by chain, so no request can make a
    // hop send on more than one token's hidden state.
    let one_token = match req.inner.payload_type {
        PayloadType::TokenIds => req.inner.data.len() == 4,
        PayloadType::HiddenStates => req.inner.shape.get(..2) == Some(&[1, 1][..]),
    };
    if req.inner.seq_pos == 0 || !one_token {
        return Err("a chain step must be a single-token decode step".into());
    }
    if req.inner.payload_type == PayloadType::TokenIds && blocks.0 != 0 {
        return Err(format!(
            "token ids sent to a shard starting at block {}",
            blocks.0
        ));
    }
    match (is_last, req.route.first()) {
        (true, Some(_)) => return Err("route continues past the last block".into()),
        (false, None) => return Err(format!("route ends at block {}", blocks.1)),
        (false, Some(next)) if next.start_block != blocks.1 => {
            return Err(format!(
                "next hop starts at block {}, this shard ends at {}",
                next.start_block, blocks.1
            ))
        }
        _ => {}
    }
    let mut seen: HashSet<&[u8]> = [me].into_iter().collect();
    let mut end = blocks.1;
    for h in &req.route {
        if !seen.insert(&h.peer_id) {
            return Err("route visits a peer twice".into());
        }
        if h.start_block != end {
            return Err(format!("route gap at block {end}"));
        }
        end = h.end_block;
    }
    Ok(())
}

/// Everything after the ack: given this hop's compute result, pass the step
/// on or report it home.
pub async fn finish_hop<U: Unary + ?Sized>(
    u: &U,
    me: &[u8],
    blocks: (u32, u32),
    req: ChainRequest,
    computed: Result<InferenceResponse>,
    fwd_ms: f32,
) {
    let ChainRequest {
        inner,
        route,
        origin,
        return_proto,
        step,
        nonce,
        mut trace,
        start_block: _,
    } = req;
    let (session_id, seq_pos) = (inner.session_id, inner.seq_pos);

    let outcome = match computed {
        Err(e) => ChainOutcome::HopFailed {
            peer_id: me.to_vec(),
            block: blocks.0,
            msg: format!("{e:#}"),
        },
        Ok(resp) => {
            trace.push(HopTrace {
                peer_id: me.to_vec(),
                start_block: blocks.0,
                end_block: blocks.1,
                fwd_ms,
            });
            match (resp.response_type, route.first()) {
                (ResponseType::Logits, _) => ChainOutcome::Logits {
                    shape: resp.shape,
                    data: resp.data,
                },
                (ResponseType::HiddenStates, Some(next)) => {
                    let onward = ChainRequest {
                        start_block: next.start_block,
                        inner: InferenceRequest {
                            session_id,
                            seq_pos,
                            payload_type: PayloadType::HiddenStates,
                            shape: resp.shape,
                            data: resp.data,
                        },
                        route: route[1..].to_vec(),
                        origin: origin.clone(),
                        return_proto: return_proto.clone(),
                        step,
                        nonce,
                        trace: trace.clone(),
                    };
                    let sent = tokio::time::timeout(
                        HAND_OFF_TIMEOUT,
                        send_chain(u, &next.peer_id, &onward),
                    )
                    .await
                    .unwrap_or_else(|_| Err(anyhow::anyhow!("hand-off timed out")));
                    match sent {
                        Ok(()) => return,
                        Err(e) => {
                            let msg = format!("{e:#}");
                            ChainOutcome::Partial {
                                end_block: blocks.1,
                                shape: onward.inner.shape,
                                data: onward.inner.data,
                                error: ForwardError {
                                    peer_id: next.peer_id.clone(),
                                    block: next.start_block,
                                    kind: classify_forward_error(&msg),
                                    msg,
                                },
                            }
                        }
                    }
                }
                (ResponseType::HiddenStates, None) => ChainOutcome::HopFailed {
                    peer_id: me.to_vec(),
                    block: blocks.0,
                    msg: "hidden states with no onward route".into(),
                },
            }
        }
    };
    let result = ChainResult {
        session_id,
        step,
        nonce,
        outcome,
        trace,
    };
    deliver(u, &origin, &return_proto, &result).await;
}

fn ack(accepted: bool, error: Option<String>) -> kwaai_p2p_daemon::error::Result<Vec<u8>> {
    rmp_serde::to_vec_named(&ChainAck { accepted, error })
        .map_err(|e| kwaai_p2p_daemon::error::Error::Protocol(format!("serialise ChainAck: {e}")))
}

/// Admit a step: decoded, served here, and routed sanely. Returns the shard
/// to compute on.
async fn admit(
    shard: &ShardCell,
    me: &[u8],
    caller: &[u8],
    data: &[u8],
) -> Result<(Arc<LoadedShard>, ChainRequest), String> {
    let req: ChainRequest =
        rmp_serde::from_slice(data).map_err(|e| format!("deserialise ChainRequest: {e}"))?;
    let s = shard
        .read()
        .await
        .clone()
        .ok_or("node warming up — model loading in background")?;
    let (a, b) = s.blocks();
    check_route((a as u32, b as u32), s.is_last(), me, caller, &req)?;
    Ok((s, req))
}

/// Register [`CHAIN_PROTO`] on `client`. Onward and return calls go out on a
/// client of their own, never the connection dispatching the inbound step.
pub async fn register_chain_handler(
    client: &P2PClient,
    daemon_addr: &str,
    shard: ShardCell,
    device: Device,
) -> Result<()> {
    let mut onward = P2PClient::connect(daemon_addr)
        .await
        .context("connect forward-chain client")?;
    let me = hex::decode(onward.identify().await.context("identify")?)?;
    let onward: Arc<P2PClient> = Arc::new(onward);
    let slots = Arc::new(Semaphore::new(MAX_IN_FLIGHT));
    let handler = move |caller: Vec<u8>, data: Vec<u8>| {
        let (shard, device, onward, me, slots) = (
            shard.clone(),
            device.clone(),
            onward.clone(),
            me.clone(),
            slots.clone(),
        );
        async move {
            let (s, req) = match admit(&shard, &me, &caller, &data).await {
                Ok(v) => v,
                Err(e) => {
                    debug!("forward chain: refused step: {e}");
                    return ack(false, Some(e));
                }
            };
            let Ok(permit) = slots.try_acquire_owned() else {
                return ack(false, Some("busy".into()));
            };
            tokio::spawn(async move {
                let (a, b) = s.blocks();
                let started = Instant::now();
                let (inner, req) = take_inner(req);
                let computed = run_inference_request(s, device, inner).await;
                let fwd_ms = started.elapsed().as_secs_f32() * 1000.0;
                drop(permit);
                finish_hop(&*onward, &me, (a as u32, b as u32), req, computed, fwd_ms).await;
            });
            ack(true, None)
        }
    };
    client
        .add_unary_handler_with_caller(CHAIN_PROTO, handler, false)
        .await
        .context("register forward-chain handler")?;
    info!("forward chain: serving {CHAIN_PROTO}");
    Ok(())
}

/// Move the tensor out for compute, leaving the routing envelope behind.
/// `finish_hop` builds the onward tensor from the compute result.
fn take_inner(mut req: ChainRequest) -> (InferenceRequest, ChainRequest) {
    let husk = InferenceRequest {
        session_id: req.inner.session_id,
        seq_pos: req.inner.seq_pos,
        payload_type: req.inner.payload_type,
        shape: Vec::new(),
        data: Vec::new(),
    };
    let inner = std::mem::replace(&mut req.inner, husk);
    (inner, req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    type Reply = Box<dyn Fn(&[u8], &str) -> Result<Vec<u8>> + Send + Sync>;
    /// `(peer, proto, payload)` per call.
    type Call = (Vec<u8>, String, Vec<u8>);

    /// Records every call; answers with `reply`.
    struct Fake {
        calls: Mutex<Vec<Call>>,
        reply: Reply,
    }

    impl Fake {
        fn new(reply: impl Fn(&[u8], &str) -> Result<Vec<u8>> + Send + Sync + 'static) -> Self {
            Self {
                calls: Mutex::new(Vec::new()),
                reply: Box::new(reply),
            }
        }
        fn accepting() -> Self {
            Self::new(|_, _| {
                Ok(rmp_serde::to_vec_named(&ChainAck {
                    accepted: true,
                    error: None,
                })?)
            })
        }
        fn calls(&self) -> Vec<Call> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl Unary for Fake {
        fn call<'a>(
            &'a self,
            peer: &'a [u8],
            proto: &'a str,
            data: Vec<u8>,
        ) -> BoxFuture<'a, Result<Vec<u8>>> {
            self.calls
                .lock()
                .unwrap()
                .push((peer.to_vec(), proto.to_string(), data));
            let r = (self.reply)(peer, proto);
            Box::pin(async move { r })
        }
    }

    const ME: &[u8] = &[1];
    const NEXT: &[u8] = &[2];
    fn origin() -> Vec<u8> {
        "12D3KooWDwa7LY7AYiNWqAaSEj42ztFw6sULYqTLRNFATZezogpW"
            .parse::<PeerId>()
            .unwrap()
            .to_bytes()
    }

    fn hop(peer: &[u8], s: u32, e: u32) -> RouteHop {
        RouteHop {
            peer_id: peer.to_vec(),
            start_block: s,
            end_block: e,
        }
    }

    fn request(route: Vec<RouteHop>) -> ChainRequest {
        ChainRequest {
            inner: InferenceRequest {
                session_id: 7,
                seq_pos: 5,
                payload_type: PayloadType::HiddenStates,
                shape: vec![1, 1, 2],
                data: vec![0; 4],
            },
            start_block: 8,
            route,
            origin: origin(),
            return_proto: result_proto(7),
            step: 3,
            nonce: [5; 16],
            trace: vec![],
        }
    }

    fn computed(response_type: ResponseType) -> Result<InferenceResponse> {
        Ok(InferenceResponse {
            session_id: 7,
            response_type,
            shape: vec![1, 1, 2],
            data: vec![5; 4],
            error: None,
        })
    }

    fn result_sent(fake: &Fake) -> ChainResult {
        let calls = fake.calls();
        let (peer, proto, data) = calls.last().expect("a result was delivered");
        assert_eq!(
            (peer.as_slice(), proto.as_str()),
            (origin().as_slice(), result_proto(7).as_str())
        );
        let r: ChainResult = rmp_serde::from_slice(data).unwrap();
        assert_eq!(r.nonce, [5; 16], "the run's nonce comes back");
        r
    }

    #[tokio::test]
    async fn the_last_hop_delivers_logits_home() {
        let fake = Fake::accepting();
        finish_hop(
            &fake,
            ME,
            (8, 16),
            request(vec![]),
            computed(ResponseType::Logits),
            2.5,
        )
        .await;
        let r = result_sent(&fake);
        assert!(
            matches!(r.outcome, ChainOutcome::Logits { .. }),
            "{:?}",
            r.outcome
        );
        assert_eq!((r.step, r.trace.len()), (3, 1));
        assert_eq!(fake.calls().len(), 1);
    }

    #[tokio::test]
    async fn a_middle_hop_passes_the_step_on_and_reports_nothing() {
        let fake = Fake::accepting();
        let req = request(vec![hop(NEXT, 16, 24), hop(&[3], 24, 32)]);
        finish_hop(
            &fake,
            ME,
            (8, 16),
            req,
            computed(ResponseType::HiddenStates),
            1.0,
        )
        .await;
        let calls = fake.calls();
        assert_eq!(calls.len(), 1, "one onward call, no result");
        let (peer, proto, data) = &calls[0];
        assert_eq!((peer.as_slice(), proto.as_str()), (NEXT, CHAIN_PROTO));
        let onward: ChainRequest = rmp_serde::from_slice(data).unwrap();
        assert_eq!(onward.route, vec![hop(&[3], 24, 32)]);
        assert_eq!((onward.trace.len(), onward.nonce), (1, [5; 16]));
        assert_eq!(onward.inner.data, vec![5; 4], "our output, not our input");
    }

    #[tokio::test]
    async fn a_refusing_next_hop_returns_our_output_for_star_to_resume() {
        let fake = Fake::new(|peer, _| {
            if peer == NEXT {
                Ok(rmp_serde::to_vec_named(&ChainAck {
                    accepted: false,
                    error: Some("warming up".into()),
                })?)
            } else {
                Ok(rmp_serde::to_vec_named(&ChainAck {
                    accepted: true,
                    error: None,
                })?)
            }
        });
        let req = request(vec![hop(NEXT, 16, 32)]);
        finish_hop(
            &fake,
            ME,
            (8, 16),
            req,
            computed(ResponseType::HiddenStates),
            1.0,
        )
        .await;
        match result_sent(&fake).outcome {
            ChainOutcome::Partial {
                end_block,
                data,
                error,
                ..
            } => {
                assert_eq!((end_block, data), (16, vec![5; 4]));
                assert_eq!((error.kind, error.block), (ForwardErrorKind::Refused, 16));
            }
            o => panic!("{o:?}"),
        }
    }

    #[tokio::test]
    async fn an_unreachable_next_hop_is_unreached_and_anything_else_failed() {
        for (err, kind) in [
            ("Dial failed: no addresses", ForwardErrorKind::Unreached),
            ("stream reset", ForwardErrorKind::Failed),
        ] {
            let fake = Fake::new(move |peer, _| {
                if peer == NEXT {
                    bail!("{err}")
                }
                Ok(rmp_serde::to_vec_named(&ChainAck {
                    accepted: true,
                    error: None,
                })?)
            });
            let req = request(vec![hop(NEXT, 16, 32)]);
            finish_hop(
                &fake,
                ME,
                (8, 16),
                req,
                computed(ResponseType::HiddenStates),
                1.0,
            )
            .await;
            match result_sent(&fake).outcome {
                ChainOutcome::Partial { error, .. } => assert_eq!(error.kind, kind, "{err}"),
                o => panic!("{o:?}"),
            }
        }
    }

    #[tokio::test]
    async fn a_compute_failure_is_reported_as_this_hop() {
        let fake = Fake::accepting();
        let req = request(vec![hop(NEXT, 16, 32)]);
        finish_hop(
            &fake,
            ME,
            (8, 16),
            req,
            Err(anyhow::anyhow!("out-of-order step")),
            1.0,
        )
        .await;
        match result_sent(&fake).outcome {
            ChainOutcome::HopFailed {
                peer_id,
                block,
                msg,
            } => {
                assert_eq!((peer_id.as_slice(), block), (ME, 8));
                assert!(msg.contains("out-of-order"));
            }
            o => panic!("{o:?}"),
        }
        assert_eq!(fake.calls().len(), 1, "nothing goes onward");
    }

    #[test]
    fn check_route_rejects_what_cannot_be_served() {
        let ok = request(vec![hop(NEXT, 16, 24), hop(&[3], 24, 32)]);
        assert!(check_route((8, 16), false, ME, &origin(), &ok).is_ok());

        let looped = request(vec![hop(NEXT, 16, 24), hop(ME, 24, 32)]);
        assert!(check_route((8, 16), false, ME, &origin(), &looped)
            .unwrap_err()
            .contains("twice"));

        let mismatch = request(vec![hop(NEXT, 12, 32)]);
        assert!(check_route((8, 16), false, ME, &origin(), &mismatch)
            .unwrap_err()
            .contains("starts at"));

        let gap = request(vec![hop(NEXT, 16, 20), hop(&[3], 24, 32)]);
        assert!(check_route((8, 16), false, ME, &origin(), &gap)
            .unwrap_err()
            .contains("gap"));

        assert!(check_route(
            (8, 16),
            true,
            ME,
            &origin(),
            &request(vec![hop(NEXT, 16, 24)])
        )
        .is_err());
        assert!(check_route((8, 16), false, ME, &origin(), &request(vec![])).is_err());

        assert!(check_route((4, 16), false, ME, &origin(), &ok)
            .unwrap_err()
            .contains("planned"));

        let mut elsewhere = request(vec![hop(NEXT, 16, 24)]);
        elsewhere.return_proto = "/kwaai/storage/1.0.0".into();
        assert!(check_route((8, 16), false, ME, &origin(), &elsewhere)
            .unwrap_err()
            .contains("originator"));
        elsewhere.return_proto = result_proto(7);
        elsewhere.origin = vec![9];
        assert!(check_route((8, 16), false, ME, &origin(), &elsewhere)
            .unwrap_err()
            .contains("originator"));
    }

    #[test]
    fn check_route_takes_only_single_token_decode_steps() {
        let me = &origin();
        let mut prefill = request(vec![hop(NEXT, 16, 32)]);
        prefill.inner.seq_pos = 0;
        assert!(check_route((8, 16), false, ME, me, &prefill)
            .unwrap_err()
            .contains("single-token"));

        let mut wide = request(vec![hop(NEXT, 16, 32)]);
        wide.inner.shape = vec![1, 2, 2];
        assert!(check_route((8, 16), false, ME, me, &wide).is_err());

        let mut tokens = request(vec![hop(NEXT, 8, 32)]);
        tokens.start_block = 0;
        tokens.inner.payload_type = PayloadType::TokenIds;
        tokens.inner.data = vec![0; 8];
        assert!(
            check_route((0, 8), false, ME, me, &tokens).is_err(),
            "two ids"
        );
        tokens.inner.data = vec![0; 4];
        assert!(check_route((0, 8), false, ME, me, &tokens).is_ok());
    }

    /// These name the sender rather than authenticate it: `origin` and
    /// `trace` are the sender's own fields. The single-token rule is what
    /// bounds what a stranger can get sent on.
    #[test]
    fn check_route_takes_a_step_only_from_who_may_send_it() {
        let stranger: &[u8] = &[4];
        let ok = request(vec![hop(NEXT, 16, 32)]);
        assert!(check_route((8, 16), false, ME, stranger, &ok)
            .unwrap_err()
            .contains("originator"));
        assert!(
            check_route((8, 16), false, ME, &[], &ok).is_err(),
            "an unknown caller is refused"
        );

        let mut later = request(vec![hop(NEXT, 16, 32)]);
        later.trace.push(HopTrace {
            peer_id: stranger.to_vec(),
            start_block: 0,
            end_block: 8,
            fwd_ms: 1.0,
        });
        assert!(check_route((8, 16), false, ME, stranger, &later).is_ok());
        assert!(check_route((8, 16), false, ME, &origin(), &later)
            .unwrap_err()
            .contains("hop before"));

        // Naming itself as the hop before does not get token ids past block 0.
        later.inner.payload_type = PayloadType::TokenIds;
        later.start_block = 0;
        later.trace[0].end_block = 0;
        later.route = vec![hop(NEXT, 8, 32)];
        assert!(check_route((0, 8), false, ME, stranger, &later)
            .unwrap_err()
            .contains("token ids"));
    }
}
