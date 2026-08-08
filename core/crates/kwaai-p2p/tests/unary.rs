//! In-process tests for the hivemind unary RPC behaviour.
//!
//! Two real swarms over TCP+noise+yamux on 127.0.0.1 — the production
//! transport — with `unary::Behaviour` standalone (no kad), so every dial goes
//! through `Swarm::add_peer_address` + the behaviour's dial-on-demand path,
//! exactly the sequence a `call_unary_handler` to a not-yet-connected peer
//! takes. Cross-implementation coverage against a real go-libp2p daemon lives
//! in `kwaai-network-tests` (`07_wire_interop.rs` for the raw wire; the swarm
//! interop tier arrives with the service integration).

use std::time::Duration;

use futures::StreamExt;
use kwaai_p2p::unary::{self, UnaryError, UnaryProtocol, UnaryResult};
use libp2p::{swarm::SwarmEvent, Multiaddr, PeerId, Swarm, SwarmBuilder};
use tokio::sync::{mpsc, oneshot};

const PROTO: &str = "DHTProtocol.rpc_ping";

/// A per-test cap so a lost reply oneshot fails the test rather than hanging it.
const TEST_TIMEOUT: Duration = Duration::from_secs(10);

// ============================================================================
// Harness
// ============================================================================

fn new_swarm(config: unary::Config) -> Swarm<unary::Behaviour> {
    SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default().nodelay(true),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )
        .expect("tcp transport")
        .with_behaviour(|_| unary::Behaviour::new(config))
        .expect("behaviour")
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build()
}

/// Start listening and return the first bound address.
async fn listen(swarm: &mut Swarm<unary::Behaviour>) -> Multiaddr {
    swarm
        .listen_on("/ip4/127.0.0.1/tcp/0".parse().expect("valid multiaddr"))
        .expect("listen");
    loop {
        if let SwarmEvent::NewListenAddr { address, .. } = swarm.select_next_some().await {
            return address;
        }
    }
}

/// A responder node: serves `PROTO` with `handler` on its own task.
///
/// `handler` maps the request payload to the responder outcome; returning
/// `None` *drops* the responder oneshot instead of answering, exercising the
/// handler-dropped path.
struct Responder {
    peer_id: PeerId,
    addr: Multiaddr,
}

impl Responder {
    async fn spawn(
        handler: impl Fn(Vec<u8>) -> Option<Result<Vec<u8>, String>> + Send + 'static,
    ) -> Self {
        let mut swarm = new_swarm(unary::Config::default());
        swarm
            .behaviour_mut()
            .register_protocol(UnaryProtocol::new(PROTO));
        let peer_id = *swarm.local_peer_id();
        let addr = listen(&mut swarm).await;

        tokio::spawn(async move {
            loop {
                if let SwarmEvent::Behaviour(unary::Event::InboundRequest { request, .. }) =
                    swarm.select_next_some().await
                {
                    match handler(request.data) {
                        Some(result) => {
                            let _ = request.responder.send(result);
                        }
                        None => drop(request.responder),
                    }
                }
            }
        });

        Self { peer_id, addr }
    }
}

/// A caller node: drives its swarm on its own task, taking calls via a channel.
struct Caller {
    requests: mpsc::UnboundedSender<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>,
}

impl Caller {
    /// Spawn a caller that knows `responder`'s address (so dial-on-demand can
    /// resolve it, standing in for the Kademlia routing table).
    fn spawn(responder: &Responder) -> Self {
        let mut swarm = new_swarm(unary::Config::default());
        swarm.add_peer_address(responder.peer_id, responder.addr.clone());

        let (tx, mut rx) =
            mpsc::unbounded_channel::<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    request = rx.recv() => match request {
                        Some((peer, proto, data, reply)) => {
                            swarm.behaviour_mut().send_request(
                                peer,
                                UnaryProtocol::new(proto),
                                data,
                                reply,
                            );
                        }
                        None => break,
                    },
                    _event = swarm.select_next_some() => {}
                }
            }
        });

        Self { requests: tx }
    }

    async fn call(&self, peer: PeerId, proto: &str, data: &[u8]) -> UnaryResult {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.requests
            .send((peer, proto.to_string(), data.to_vec(), reply_tx))
            .expect("caller task alive");
        tokio::time::timeout(TEST_TIMEOUT, reply_rx)
            .await
            .expect("call must resolve within the test timeout")
            .expect("reply oneshot must not be dropped")
    }
}

// ============================================================================
// Tests
// ============================================================================

/// The core path: dial-on-demand to a never-connected peer, slash-less
/// negotiation, one frame each way, payload intact.
#[tokio::test]
async fn round_trip_with_dial_on_demand() {
    let responder = Responder::spawn(|data| {
        let mut out = b"echo:".to_vec();
        out.extend_from_slice(&data);
        Some(Ok(out))
    })
    .await;
    let caller = Caller::spawn(&responder);

    let response = caller
        .call(responder.peer_id, PROTO, b"ping-payload")
        .await
        .expect("call should succeed");
    assert_eq!(response, b"echo:ping-payload");
}

/// The error arm must arrive as `UnaryError::Remote` with the handler's text.
#[tokio::test]
async fn error_arm_propagates() {
    let responder = Responder::spawn(|_| Some(Err("deliberate handler failure".to_string()))).await;
    let caller = Caller::spawn(&responder);

    let error = caller
        .call(responder.peer_id, PROTO, b"x")
        .await
        .expect_err("handler error must surface");
    match error {
        UnaryError::Remote(text) => assert_eq!(text, "deliberate handler failure"),
        other => panic!("expected Remote, got {other:?}"),
    }
}

/// Calling a protocol the peer does not serve must fail with
/// `UnsupportedProtocols` — the clean refusal the health probe depends on —
/// and must NOT tear down the connection: a subsequent call on a served
/// protocol succeeds.
#[tokio::test]
async fn unsupported_protocol_is_a_clean_refusal() {
    let responder = Responder::spawn(|data| Some(Ok(data))).await;
    let caller = Caller::spawn(&responder);

    let error = caller
        .call(responder.peer_id, "DHTProtocol.rpc_nonexistent", b"x")
        .await
        .expect_err("unserved protocol must be refused");
    assert!(
        matches!(error, UnaryError::UnsupportedProtocol(ref p) if p == "DHTProtocol.rpc_nonexistent"),
        "expected UnsupportedProtocol, got {error:?}"
    );

    let response = caller
        .call(responder.peer_id, PROTO, b"still-alive")
        .await
        .expect("the connection must survive a refused protocol");
    assert_eq!(response, b"still-alive");
}

/// A handler that drops its responder (crash-analog) must produce the error
/// arm on the wire, not a hang.
#[tokio::test]
async fn dropped_responder_becomes_an_error_reply() {
    let responder = Responder::spawn(|_| None).await;
    let caller = Caller::spawn(&responder);

    let error = caller
        .call(responder.peer_id, PROTO, b"x")
        .await
        .expect_err("dropped responder must surface as an error");
    match error {
        UnaryError::Remote(text) => assert!(
            text.contains("dropped"),
            "expected the dropped-handler message, got: {text}"
        ),
        other => panic!("expected Remote, got {other:?}"),
    }
}

/// Dialing a peer with no known addresses fails the parked request with
/// `DialFailure` instead of leaving it queued forever.
#[tokio::test]
async fn unreachable_peer_fails_the_request() {
    let responder = Responder::spawn(|data| Some(Ok(data))).await;
    let caller = Caller::spawn(&responder);

    // A peer ID nobody has an address for.
    let stranger = PeerId::random();
    let error = caller
        .call(stranger, PROTO, b"x")
        .await
        .expect_err("no route to the peer");
    assert!(
        matches!(error, UnaryError::DialFailure(_)),
        "expected DialFailure, got {error:?}"
    );
}

/// Concurrent calls on one connection must all resolve with their own payloads
/// (per-stream isolation, no callId cross-talk).
#[tokio::test]
async fn concurrent_calls_resolve_independently() {
    let responder = Responder::spawn(|data| {
        let mut out = b"echo:".to_vec();
        out.extend_from_slice(&data);
        Some(Ok(out))
    })
    .await;
    let caller = Caller::spawn(&responder);

    let calls = (0u8..8).map(|i| {
        let caller = &caller;
        let peer = responder.peer_id;
        async move { caller.call(peer, PROTO, &[i; 16]).await }
    });
    let results = futures::future::join_all(calls).await;

    for (i, result) in results.into_iter().enumerate() {
        let response = result.expect("concurrent call should succeed");
        let mut expected = b"echo:".to_vec();
        expected.extend_from_slice(&[i as u8; 16]);
        assert_eq!(response, expected, "call {i} got the wrong payload back");
    }
}

/// A protocol registered *after* a connection is already established must be
/// callable on that same connection — the dynamic-registration property
/// Phase 3's IPC `add_unary_handler` depends on.
#[tokio::test]
async fn late_registration_reaches_existing_connections() {
    // Responder with full swarm access kept in the test body: register PROTO,
    // serve one call, then register LATE and keep serving.
    const LATE: &str = "hello";

    let mut swarm = new_swarm(unary::Config::default());
    swarm
        .behaviour_mut()
        .register_protocol(UnaryProtocol::new(PROTO));
    let peer_id = *swarm.local_peer_id();
    let addr = listen(&mut swarm).await;

    let (registered_tx, registered_rx) = oneshot::channel::<()>();
    let (register_late_tx, mut register_late_rx) = mpsc::unbounded_channel::<()>();
    tokio::spawn(async move {
        let mut registered_tx = Some(registered_tx);
        loop {
            tokio::select! {
                Some(()) = register_late_rx.recv() => {
                    swarm.behaviour_mut().register_protocol(UnaryProtocol::new(LATE));
                    if let Some(tx) = registered_tx.take() {
                        let _ = tx.send(());
                    }
                }
                event = swarm.select_next_some() => {
                    if let SwarmEvent::Behaviour(unary::Event::InboundRequest { request, .. }) = event {
                        let _ = request.responder.send(Ok(request.proto.to_string().into_bytes()));
                    }
                }
            }
        }
    });

    let responder = Responder { peer_id, addr };
    let caller = Caller::spawn(&responder);

    // Establish the connection via a call on the initially-registered protocol.
    let response = caller
        .call(peer_id, PROTO, b"")
        .await
        .expect("initial protocol should work");
    assert_eq!(response, PROTO.as_bytes());

    // LATE is not yet registered: refused on the live connection.
    let error = caller
        .call(peer_id, LATE, b"")
        .await
        .expect_err("not yet registered");
    assert!(matches!(error, UnaryError::UnsupportedProtocol(_)));

    // Register it and call again — same connection, no reconnect.
    register_late_tx.send(()).expect("responder task alive");
    tokio::time::timeout(TEST_TIMEOUT, registered_rx)
        .await
        .expect("registration must be acknowledged")
        .expect("registration ack");

    let response = caller
        .call(peer_id, LATE, b"")
        .await
        .expect("late-registered protocol must be reachable on the existing connection");
    assert_eq!(response, LATE.as_bytes());
}

/// With `max_concurrent_streams: 1` on the caller, a burst of calls must be
/// throttled through the single slot and still ALL complete — emission
/// back-pressure, not failure.
#[tokio::test]
async fn outbound_burst_is_throttled_not_failed() {
    let responder = Responder::spawn(|data| Some(Ok(data))).await;

    // Bespoke caller with a cap of one in-flight outbound stream.
    let mut swarm = new_swarm(unary::Config {
        max_concurrent_streams: 1,
        ..unary::Config::default()
    });
    swarm.add_peer_address(responder.peer_id, responder.addr.clone());
    let (tx, mut rx) =
        mpsc::unbounded_channel::<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                request = rx.recv() => match request {
                    Some((peer, proto, data, reply)) => swarm.behaviour_mut().send_request(
                        peer,
                        UnaryProtocol::new(proto),
                        data,
                        reply,
                    ),
                    None => break,
                },
                _event = swarm.select_next_some() => {}
            }
        }
    });

    let mut replies = Vec::new();
    for i in 0u8..4 {
        let (reply_tx, reply_rx) = oneshot::channel();
        tx.send((responder.peer_id, PROTO.to_string(), vec![i; 8], reply_tx))
            .expect("caller task alive");
        replies.push(reply_rx);
    }
    for (i, reply) in replies.into_iter().enumerate() {
        let response = tokio::time::timeout(TEST_TIMEOUT, reply)
            .await
            .expect("throttled call must still resolve")
            .expect("reply channel intact")
            .expect("call should succeed");
        assert_eq!(response, vec![i as u8; 8], "call {i} payload survived");
    }
}

/// A responder at its inbound cap must answer the overflow call with the
/// "at capacity" error arm — a clean refusal the caller can act on — rather
/// than resetting the stream or timing it out.
#[tokio::test]
async fn inbound_overflow_gets_a_capacity_refusal() {
    use std::sync::{Arc, Mutex};

    // Responder with a single inbound slot that parks requests instead of
    // answering, so the test controls when the slot frees up.
    let mut swarm = new_swarm(unary::Config {
        max_concurrent_streams: 1,
        ..unary::Config::default()
    });
    swarm
        .behaviour_mut()
        .register_protocol(UnaryProtocol::new(PROTO));
    let peer_id = *swarm.local_peer_id();
    let addr = listen(&mut swarm).await;

    let parked: Arc<Mutex<Vec<tokio::sync::oneshot::Sender<Result<Vec<u8>, String>>>>> =
        Arc::new(Mutex::new(Vec::new()));
    let parked_in_loop = Arc::clone(&parked);
    tokio::spawn(async move {
        loop {
            if let SwarmEvent::Behaviour(unary::Event::InboundRequest { request, .. }) =
                swarm.select_next_some().await
            {
                parked_in_loop
                    .lock()
                    .expect("parked lock")
                    .push(request.responder);
            }
        }
    });

    let responder = Responder { peer_id, addr };
    let caller = Caller::spawn(&responder);

    // Occupy the only slot, waiting until the request has actually reached
    // the app layer (the worker is now parked awaiting `parked[0]`).
    let (first_tx, first_rx) = oneshot::channel();
    caller
        .requests
        .send((peer_id, PROTO.to_string(), b"first".to_vec(), first_tx))
        .expect("caller task alive");
    tokio::time::timeout(TEST_TIMEOUT, async {
        loop {
            if !parked.lock().expect("parked lock").is_empty() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("first call must reach the responder's app layer");

    // The overflow call gets the refusal error arm.
    let error = caller
        .call(peer_id, PROTO, b"second")
        .await
        .expect_err("overflow must be refused");
    match error {
        UnaryError::Remote(text) => assert!(
            text.contains("capacity"),
            "expected the at-capacity refusal, got: {text}"
        ),
        other => panic!("expected Remote refusal, got {other:?}"),
    }

    // Free the slot; the parked first call completes normally.
    parked
        .lock()
        .expect("parked lock")
        .pop()
        .expect("first responder parked")
        .send(Ok(b"released".to_vec()))
        .expect("worker still awaiting");
    let response = tokio::time::timeout(TEST_TIMEOUT, first_rx)
        .await
        .expect("first call resolves after release")
        .expect("reply channel intact")
        .expect("first call succeeds");
    assert_eq!(response, b"released");
}

/// Liveness: with a single outbound slot, a call queued behind one that FAILS
/// negotiation must still get its turn — the slot is freed by
/// `DialUpgradeError`, not worker completion, so it exercises the explicit
/// wake path rather than `FuturesUnordered`'s own wakes.
#[tokio::test]
async fn queued_call_survives_a_negotiation_failure_ahead_of_it() {
    let responder = Responder::spawn(|data| Some(Ok(data))).await;

    let mut swarm = new_swarm(unary::Config {
        max_concurrent_streams: 1,
        ..unary::Config::default()
    });
    swarm.add_peer_address(responder.peer_id, responder.addr.clone());
    let (tx, mut rx) =
        mpsc::unbounded_channel::<(PeerId, String, Vec<u8>, oneshot::Sender<UnaryResult>)>();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                request = rx.recv() => match request {
                    Some((peer, proto, data, reply)) => swarm.behaviour_mut().send_request(
                        peer,
                        UnaryProtocol::new(proto),
                        data,
                        reply,
                    ),
                    None => break,
                },
                _event = swarm.select_next_some() => {}
            }
        }
    });

    // A occupies the only slot and will be refused at negotiation.
    let (a_tx, a_rx) = oneshot::channel();
    tx.send((
        responder.peer_id,
        "DHTProtocol.rpc_unserved".to_string(),
        b"a".to_vec(),
        a_tx,
    ))
    .expect("caller task alive");
    // B queues behind A on a served protocol.
    let (b_tx, b_rx) = oneshot::channel();
    tx.send((responder.peer_id, PROTO.to_string(), b"b".to_vec(), b_tx))
        .expect("caller task alive");

    let a = tokio::time::timeout(TEST_TIMEOUT, a_rx)
        .await
        .expect("A resolves")
        .expect("A channel intact");
    assert!(
        matches!(a, Err(UnaryError::UnsupportedProtocol(_))),
        "A must be refused, got {a:?}"
    );
    let b = tokio::time::timeout(TEST_TIMEOUT, b_rx)
        .await
        .expect("B must not hang behind A's freed slot")
        .expect("B channel intact")
        .expect("B should succeed");
    assert_eq!(b, b"b");
}
