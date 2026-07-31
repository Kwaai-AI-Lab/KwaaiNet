//! [`NetworkService`] — the swarm owner.
//!
//! The swarm is not `Sync` and must be polled from exactly one place, so it
//! lives on a dedicated tokio task and is reached only through
//! [`NetworkHandle`]. The task's `select!` loop has three arms:
//!
//! 1. **commands** from handles,
//! 2. **swarm events**,
//! 3. **maintenance** — a periodic Kademlia bootstrap to refresh buckets.
//!
//! Requests that cannot be answered from swarm state immediately are parked:
//! dials in `pending_dials` (keyed by `ConnectionId`) and DHT lookups in
//! `pending_kad` (keyed by `QueryId`). The invariant that keeps callers from
//! hanging forever is that **every pending entry is removed on both the success
//! and the failure event**.
//!
//! Unary needs no such bookkeeping: the behaviour owns each outbound `oneshot`,
//! and inbound dispatch is an unbounded send, which never blocks the loop.

use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Duration;

use anyhow::{Context, Result};
use futures::StreamExt;
use libp2p::{
    core::ConnectedPoint,
    identify, identity, kad, noise, ping,
    swarm::{ConnectionId, DialError, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm, SwarmBuilder,
};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, info, trace, warn};

use crate::behaviour::{KwaaiBehaviour, KwaaiBehaviourEvent};
use crate::config::NetworkConfig;
use crate::error::{P2PError, P2PResult};
use crate::handle::{
    parse_protocols, Command, Direction, InboundStreamSender, InboundUnaryCall, InboundUnarySender,
    NetworkHandle, PeerInfo,
};
use crate::raw_stream;
use crate::unary::{self, UnaryProtocol};

/// How often the maintenance arm refreshes the Kademlia routing table.
const KAD_REFRESH_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Depth of the command channel. Deep enough that bursts of handle calls do not
/// serialize on the event loop, shallow enough to apply backpressure.
const COMMAND_CHANNEL_SIZE: usize = 64;

/// A connection we are tracking for `list_peers`.
#[derive(Debug, Clone)]
struct Connection {
    addr: Multiaddr,
    direction: Direction,
}

/// Owns the swarm and drives it. Construct with [`NetworkService::spawn`].
pub struct NetworkService {
    swarm: Swarm<KwaaiBehaviour>,
    commands: mpsc::Receiver<Command>,

    /// Dials awaiting a `ConnectionEstablished` / `OutgoingConnectionError`.
    pending_dials: HashMap<ConnectionId, oneshot::Sender<P2PResult<PeerId>>>,
    /// DHT lookups awaiting query completion.
    pending_kad: HashMap<kad::QueryId, PendingKad>,
    /// Live connections, per peer, keyed by connection so multiple connections
    /// to one peer are tracked independently.
    connections: HashMap<PeerId, HashMap<ConnectionId, Connection>>,
    /// Addresses peers reported observing us at → the set of peers that said so.
    /// A set (not a counter) so repeated identifies from one peer count once.
    observed_addrs: HashMap<Multiaddr, HashSet<PeerId>>,
    /// The protocol list each connected peer advertised over identify. This is
    /// the capability feed: relay-hop support, AutoNAT and dcutr all show up
    /// here. Dropped when the last connection to a peer closes, so an entry
    /// always describes a peer we can act on.
    peer_protocols: HashMap<PeerId, Vec<String>>,
    /// Registered unary handlers: negotiated protocol → the handler task's
    /// channel. Kept in lockstep with the behaviour's inbound protocol set, so
    /// an entry here means the protocol is advertised and vice versa.
    unary_handlers: HashMap<String, InboundUnarySender>,
    /// Registered **raw-stream** handlers: negotiated protocol → the accepting
    /// task's channel. Same lockstep invariant as `unary_handlers`, against a
    /// separate protocol set — the two namespaces are independent, so a name
    /// registered here is not callable as a unary handler and vice versa.
    stream_handlers: HashMap<String, InboundStreamSender>,
}

/// A parked DHT lookup.
#[derive(Debug)]
enum PendingKad {
    /// `dht_find_peer`: on completion, read the target's addresses out of the
    /// routing table the query just populated.
    FindPeer {
        target: PeerId,
        reply: oneshot::Sender<P2PResult<Vec<Multiaddr>>>,
    },
    /// `bootstrap`'s kad walk — nothing to report, the dials already decided
    /// success. Tracked only so completion can be logged.
    Bootstrap,
    /// The periodic maintenance refresh.
    Maintenance,
}

impl NetworkService {
    /// Build the swarm, start listening, and spawn the event loop.
    ///
    /// Returns a [`NetworkHandle`] and the task's `JoinHandle`. Dropping every
    /// clone of the handle closes the command channel and ends the loop, so a
    /// caller that wants the service to outlive its own scope must keep a
    /// handle alive.
    pub fn spawn(
        config: NetworkConfig,
        keypair: identity::Keypair,
    ) -> Result<(NetworkHandle, tokio::task::JoinHandle<()>)> {
        let local_peer_id = keypair.public().to_peer_id();
        let behaviour_config = config.clone();

        let mut swarm = SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_tcp(
                tcp::Config::default().nodelay(true),
                noise::Config::new,
                yamux::Config::default,
            )
            .context("configuring TCP transport")?
            // DNS resolution is required for `/dns/bootstrap-N.kwaai.ai/...`
            // addresses; without it those dials fail at the transport layer.
            .with_dns()
            .context("configuring DNS resolution")?
            .with_behaviour(|kp| KwaaiBehaviour::new(kp, &behaviour_config))
            .map_err(|e| anyhow::anyhow!("configuring behaviour: {e}"))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(config.connection_timeout))
            .build();

        for addr in config.swarm_listen_addrs() {
            let addr: Multiaddr = addr
                .parse()
                .with_context(|| format!("parsing listen address {addr}"))?;
            match swarm.listen_on(addr.clone()) {
                Ok(_) => debug!(%addr, "listening"),
                // One failed listener (commonly IPv6 on a v4-only host) must
                // not sink the whole node.
                Err(e) => warn!(%addr, error = %e, "failed to listen on address"),
            }
        }

        let (tx, rx) = mpsc::channel(COMMAND_CHANNEL_SIZE);
        let service = Self {
            swarm,
            commands: rx,
            pending_dials: HashMap::new(),
            pending_kad: HashMap::new(),
            connections: HashMap::new(),
            observed_addrs: HashMap::new(),
            peer_protocols: HashMap::new(),
            unary_handlers: HashMap::new(),
            stream_handlers: HashMap::new(),
        };

        let task = tokio::spawn(service.run());
        info!(peer_id = %local_peer_id, "network service started");
        Ok((
            NetworkHandle::new(local_peer_id, tx, config.request_timeout),
            task,
        ))
    }

    /// The event loop. Exits on `Command::Shutdown` or when all handles drop.
    async fn run(mut self) {
        let mut maintenance = tokio::time::interval(KAD_REFRESH_INTERVAL);
        // The first tick fires immediately; skip it so startup does not race
        // the initial bootstrap dial.
        maintenance.tick().await;

        loop {
            tokio::select! {
                command = self.commands.recv() => {
                    match command {
                        Some(Command::Shutdown { reply }) => {
                            debug!("shutdown requested");
                            self.fail_all_pending(P2PError::NotInitialized);
                            let _ = reply.send(());
                            break;
                        }
                        Some(command) => self.handle_command(command),
                        None => {
                            debug!("all handles dropped; stopping network service");
                            self.fail_all_pending(P2PError::NotInitialized);
                            break;
                        }
                    }
                }
                event = self.swarm.select_next_some() => {
                    self.handle_swarm_event(event);
                }
                _ = maintenance.tick() => {
                    self.refresh_routing_table();
                }
            }
        }
        info!("network service stopped");
    }

    // ------------------------------------------------------------------
    // Commands
    // ------------------------------------------------------------------

    /// Apply one command. Must never await or block — anything slow gets parked
    /// in a pending map and resolved from `handle_swarm_event`.
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::ConnectPeer { addr, reply } => match self.dial(addr) {
                Ok(connection_id) => {
                    self.pending_dials.insert(connection_id, reply);
                }
                Err(e) => {
                    let _ = reply.send(Err(e));
                }
            },

            Command::DisconnectPeer { peer, reply } => {
                let result = match self.swarm.disconnect_peer_id(peer) {
                    Ok(()) => Ok(()),
                    Err(()) => Err(P2PError::PeerNotFound(peer.to_base58())),
                };
                self.connections.remove(&peer);
                let _ = reply.send(result);
            }

            Command::ListPeers { reply } => {
                let peers = self
                    .connections
                    .iter()
                    .flat_map(|(peer_id, conns)| {
                        conns.values().map(move |c| PeerInfo {
                            peer_id: *peer_id,
                            addr: c.addr.clone(),
                            direction: c.direction,
                        })
                    })
                    .collect();
                let _ = reply.send(peers);
            }

            Command::ObservedAddrs { reply } => {
                let mut addrs: Vec<(Multiaddr, usize)> = self
                    .observed_addrs
                    .iter()
                    .map(|(addr, observers)| (addr.clone(), observers.len()))
                    .collect();
                // Most-confirmed first — callers generally want the best
                // external-address candidate.
                addrs.sort_by(|a, b| b.1.cmp(&a.1));
                let _ = reply.send(addrs);
            }

            Command::ListenAddrs { reply } => {
                let addrs = self.swarm.listeners().cloned().collect();
                let _ = reply.send(addrs);
            }

            Command::PeerProtocols { peer, reply } => {
                let _ = reply.send(self.peer_protocols.get(&peer).cloned());
            }

            // A walk over in-memory k-buckets — bounded by the routing table
            // size (k=20 per bucket, 256 buckets) and never touching the
            // network, so it is safe in the event loop.
            Command::RoutingPeers { reply } => {
                let peers = self
                    .swarm
                    .behaviour_mut()
                    .kad
                    .kbuckets()
                    .flat_map(|bucket| {
                        bucket
                            .iter()
                            .map(|entry| *entry.node.key.preimage())
                            .collect::<Vec<_>>()
                    })
                    .collect();
                let _ = reply.send(peers);
            }

            Command::DhtFindPeer { peer, reply } => {
                // Short-circuit: if we already have addresses (routing table or
                // a live connection) there is no need for a network walk.
                let known = self.known_addresses(&peer);
                if !known.is_empty() {
                    let _ = reply.send(Ok(known));
                    return;
                }
                let query_id = self.swarm.behaviour_mut().kad.get_closest_peers(peer);
                self.pending_kad.insert(
                    query_id,
                    PendingKad::FindPeer {
                        target: peer,
                        reply,
                    },
                );
            }

            Command::AddKadAddress { peer, addr, reply } => {
                self.swarm.behaviour_mut().kad.add_address(&peer, addr);
                let _ = reply.send(());
            }

            Command::Bootstrap { peers, reply } => {
                let _ = reply.send(self.start_bootstrap(peers));
            }

            Command::CallUnary {
                peer,
                proto,
                data,
                reply,
            } => {
                self.swarm.behaviour_mut().unary.send_request(
                    peer,
                    UnaryProtocol::new(proto),
                    data,
                    reply,
                );
            }

            Command::AddUnaryHandler {
                proto,
                sender,
                reply,
            } => {
                self.swarm
                    .behaviour_mut()
                    .unary
                    .register_protocol(UnaryProtocol::new(proto.clone()));
                // Re-registering replaces the sender; dropping the old one ends
                // the previous handler's dispatch task.
                self.unary_handlers.insert(proto.clone(), sender);
                debug!(%proto, "unary handler registered");
                let _ = reply.send(());
            }

            Command::RemoveUnaryHandler { proto, reply } => {
                let existed = self.unregister_unary(&proto);
                let _ = reply.send(existed);
            }

            // No pending-map entry, for the same reason as `CallUnary`: the
            // behaviour owns `reply` and resolves it on every outcome.
            Command::OpenRawStream {
                peer,
                protos,
                reply,
            } => {
                self.swarm.behaviour_mut().raw_stream.open_stream(
                    peer,
                    parse_protocols(&protos),
                    reply,
                );
            }

            Command::AddStreamHandler {
                protos,
                sender,
                reply,
            } => {
                let mut refused = Vec::new();
                for proto in protos {
                    if proto.is_empty() {
                        refused.push(proto);
                        continue;
                    }
                    // Refuse rather than replace: unlike a unary handler, a raw
                    // stream handler is an *accept loop* belonging to one
                    // process, and silently rebinding it would strand the first
                    // owner's receiver with no way to notice. This matches Go's
                    // `doStreamHandler` with `balanced` false.
                    if self.stream_handlers.contains_key(&proto) {
                        refused.push(proto);
                        continue;
                    }
                    self.swarm
                        .behaviour_mut()
                        .raw_stream
                        .register_protocol(UnaryProtocol::new(proto.as_str()));
                    self.stream_handlers.insert(proto.clone(), sender.clone());
                    debug!(%proto, "raw stream handler registered");
                }
                let _ = reply.send(refused);
            }

            Command::RemoveStreamHandler { protos, reply } => {
                let mut removed = Vec::new();
                for proto in protos {
                    if self.unregister_stream(&proto) {
                        removed.push(proto);
                    }
                }
                let _ = reply.send(removed);
            }

            // Handled in `run` so the loop can break.
            Command::Shutdown { reply } => {
                let _ = reply.send(());
            }
        }
    }

    /// Dial `addr`, seeding the routing table with it when it carries a
    /// `/p2p/<peer-id>` component.
    fn dial(&mut self, addr: Multiaddr) -> P2PResult<ConnectionId> {
        use libp2p::swarm::dial_opts::DialOpts;

        if let Some(peer) = peer_id_from_multiaddr(&addr) {
            // Kad wants the address without the trailing /p2p component; it
            // re-attaches it itself via `with_p2p`.
            self.swarm
                .behaviour_mut()
                .kad
                .add_address(&peer, strip_p2p(&addr));
        }

        let opts = DialOpts::from(addr.clone());
        let connection_id = opts.connection_id();
        self.swarm
            .dial(opts)
            .map_err(|e| P2PError::DialFailed(format!("{addr}: {e}")))?;
        debug!(%addr, ?connection_id, "dialing");
        Ok(connection_id)
    }

    /// Dial every bootstrap address and kick off a Kademlia bootstrap.
    ///
    /// Dials are fire-and-forget here (their outcomes surface as swarm events
    /// and connection state); the error case we care about is "nothing to dial
    /// and no peers known", which would make `kad.bootstrap()` fail anyway.
    fn start_bootstrap(&mut self, peers: Vec<Multiaddr>) -> P2PResult<()> {
        let mut dialed = 0usize;
        let mut last_error = None;
        for addr in peers {
            match self.dial(addr.clone()) {
                Ok(_) => dialed += 1,
                Err(e) => {
                    warn!(%addr, error = %e, "bootstrap dial failed to start");
                    last_error = Some(e);
                }
            }
        }

        match self.swarm.behaviour_mut().kad.bootstrap() {
            Ok(query_id) => {
                self.pending_kad.insert(query_id, PendingKad::Bootstrap);
                info!(dialed, "bootstrap started");
                Ok(())
            }
            Err(e) => {
                // NoKnownPeers: every address failed to even parse into a dial.
                if dialed == 0 {
                    Err(last_error.unwrap_or_else(|| P2PError::DhtError(format!("bootstrap: {e}"))))
                } else {
                    // Dials are in flight; kad will have peers shortly and the
                    // maintenance tick will retry.
                    debug!(error = %e, "kad bootstrap deferred until a peer connects");
                    Ok(())
                }
            }
        }
    }

    /// Periodic bucket refresh. A failure here is expected while isolated.
    fn refresh_routing_table(&mut self) {
        match self.swarm.behaviour_mut().kad.bootstrap() {
            Ok(query_id) => {
                trace!("kad routing table refresh started");
                self.pending_kad.insert(query_id, PendingKad::Maintenance);
            }
            Err(e) => trace!(error = %e, "kad refresh skipped"),
        }
    }

    /// Addresses we already know for `peer`: routing table entries first, then
    /// any live connection's address.
    fn known_addresses(&mut self, peer: &PeerId) -> Vec<Multiaddr> {
        let mut addrs: Vec<Multiaddr> = Vec::new();

        if let Some(bucket) = self.swarm.behaviour_mut().kad.kbucket(*peer) {
            for entry in bucket.iter() {
                if entry.node.key.preimage() == peer {
                    addrs.extend(entry.node.value.iter().cloned());
                }
            }
        }

        if let Some(conns) = self.connections.get(peer) {
            for conn in conns.values() {
                if !addrs.contains(&conn.addr) {
                    addrs.push(conn.addr.clone());
                }
            }
        }

        addrs
    }

    /// Stop serving `proto`: drop the dispatch channel and stop advertising it.
    /// Returns whether a handler was registered. Idempotent.
    fn unregister_unary(&mut self, proto: &str) -> bool {
        let existed = self.unary_handlers.remove(proto).is_some();
        self.swarm
            .behaviour_mut()
            .unary
            .unregister_protocol(&UnaryProtocol::new(proto));
        if existed {
            debug!(%proto, "unary handler removed");
        }
        existed
    }

    /// Stop accepting raw streams on `proto`: drop the accept channel and stop
    /// advertising it. Returns whether a handler was registered. Idempotent.
    fn unregister_stream(&mut self, proto: &str) -> bool {
        let existed = self.stream_handlers.remove(proto).is_some();
        if existed {
            self.swarm
                .behaviour_mut()
                .raw_stream
                .unregister_protocol(&UnaryProtocol::new(proto));
            debug!(%proto, "raw stream handler removed");
        }
        existed
    }

    /// Route one inbound raw stream to its handler.
    ///
    /// Dispatch is by **negotiated** protocol. A stream with no handler is
    /// dropped, which resets it — the same signal Go sends (`s.Reset()` in
    /// `handleStream` when the protocol is not in its handler map). That is the
    /// rare path: an unregistered protocol is refused during negotiation, so
    /// this only fires if the handler went away between negotiation and
    /// dispatch.
    fn dispatch_stream(&mut self, inbound: raw_stream::InboundStream) {
        let proto = inbound.proto.as_ref().to_string();

        let Some(sender) = self.stream_handlers.get(&proto) else {
            debug!(%proto, "inbound raw stream with no handler; resetting");
            return;
        };

        if let Err(e) = sender.send(inbound) {
            warn!(%proto, "raw stream handler is gone; deregistering the protocol");
            self.unregister_stream(&proto);
            // `e.0` is the stream; dropping it resets, which is what the remote
            // needs to see rather than a stream that is open but never read.
            drop(e.0);
        }
    }

    /// Route one inbound unary call to its handler.
    ///
    /// Dispatch is keyed on the **negotiated** protocol, never the frame's
    /// `proto` field. Three outcomes, none of which may block:
    ///
    /// - handler present and its task alive → hand the call over; the task owns
    ///   the responder from here,
    /// - handler present but its receiver dropped (the task died) → deregister
    ///   the protocol so subsequent calls get a clean refusal instead of a
    ///   silent black hole, and fail this call,
    /// - nothing registered → fail immediately. This is the rare path: the
    ///   protocol was deregistered between negotiation and decode, since
    ///   unregistered protocols are refused during negotiation.
    fn dispatch_unary(&mut self, peer: PeerId, request: unary::InboundRequest) {
        let unary::InboundRequest {
            proto,
            data,
            responder,
        } = request;
        let proto = proto.as_ref().to_string();

        let Some(sender) = self.unary_handlers.get(&proto) else {
            let _ = responder.send(Err(format!("no handler for {proto}")));
            return;
        };

        let call = InboundUnaryCall {
            peer,
            data,
            responder,
        };
        if let Err(e) = sender.send(call) {
            warn!(%proto, "unary handler task is gone; deregistering the protocol");
            self.unregister_unary(&proto);
            let _ = e.0.responder.send(Err(format!("no handler for {proto}")));
        }
    }

    /// Resolve every parked request with `error`. Called on shutdown so no
    /// caller is left awaiting a reply that will never come.
    fn fail_all_pending(&mut self, error: P2PError) {
        for (_, reply) in self.pending_dials.drain() {
            let _ = reply.send(Err(error.clone()));
        }
        for (_, pending) in self.pending_kad.drain() {
            if let PendingKad::FindPeer { reply, .. } = pending {
                let _ = reply.send(Err(error.clone()));
            }
        }
    }

    // ------------------------------------------------------------------
    // Swarm events
    // ------------------------------------------------------------------

    fn handle_swarm_event(&mut self, event: SwarmEvent<KwaaiBehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!(%address, "listening on new address");
            }

            SwarmEvent::ConnectionEstablished {
                peer_id,
                connection_id,
                endpoint,
                ..
            } => {
                let (addr, direction) = match &endpoint {
                    ConnectedPoint::Dialer { address, .. } => {
                        (address.clone(), Direction::Outbound)
                    }
                    ConnectedPoint::Listener { send_back_addr, .. } => {
                        (send_back_addr.clone(), Direction::Inbound)
                    }
                };
                debug!(peer = %peer_id, %addr, direction = direction.as_str(), "connection established");

                self.connections
                    .entry(peer_id)
                    .or_default()
                    .insert(connection_id, Connection { addr, direction });

                if let Some(reply) = self.pending_dials.remove(&connection_id) {
                    let _ = reply.send(Ok(peer_id));
                }
            }

            SwarmEvent::ConnectionClosed {
                peer_id,
                connection_id,
                cause,
                ..
            } => {
                debug!(peer = %peer_id, ?cause, "connection closed");
                let last = match self.connections.entry(peer_id) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().remove(&connection_id);
                        let empty = entry.get().is_empty();
                        if empty {
                            entry.remove();
                        }
                        empty
                    }
                    Entry::Vacant(_) => true,
                };
                if last {
                    // The capability list describes a peer we can act on; a
                    // disconnected peer's is stale by definition.
                    self.peer_protocols.remove(&peer_id);
                }
            }

            SwarmEvent::OutgoingConnectionError {
                connection_id,
                peer_id,
                error,
            } => {
                // Must remove the pending entry here or `connect_peer` hangs.
                if let Some(reply) = self.pending_dials.remove(&connection_id) {
                    let _ = reply.send(Err(dial_error(&error, peer_id)));
                } else {
                    debug!(?peer_id, error = %error, "outgoing connection error");
                }
            }

            SwarmEvent::IncomingConnectionError {
                send_back_addr,
                error,
                ..
            } => {
                debug!(%send_back_addr, error = %error, "incoming connection error");
            }

            SwarmEvent::Behaviour(event) => self.handle_behaviour_event(event),

            other => trace!(?other, "unhandled swarm event"),
        }
    }

    fn handle_behaviour_event(&mut self, event: KwaaiBehaviourEvent) {
        match event {
            KwaaiBehaviourEvent::Identify(event) => self.handle_identify_event(event),
            KwaaiBehaviourEvent::Kad(event) => self.handle_kad_event(event),
            KwaaiBehaviourEvent::Ping(ping::Event { peer, result, .. }) => match result {
                Ok(rtt) => trace!(peer = %peer, ?rtt, "ping"),
                Err(e) => debug!(peer = %peer, error = %e, "ping failed"),
            },
            KwaaiBehaviourEvent::Unary(unary::Event::InboundRequest { peer, request }) => {
                self.dispatch_unary(peer, request)
            }
            KwaaiBehaviourEvent::RawStream(raw_stream::Event::InboundStream(inbound)) => {
                self.dispatch_stream(inbound)
            }
        }
    }

    fn handle_identify_event(&mut self, event: identify::Event) {
        match event {
            identify::Event::Received { peer_id, info } => {
                debug!(
                    peer = %peer_id,
                    protocol_version = %info.protocol_version,
                    agent_version = %info.agent_version,
                    observed_addr = %info.observed_addr,
                    listen_addrs = info.listen_addrs.len(),
                    protocols = ?info.protocols,
                    "identify received"
                );

                // (a) Feed the routing table. Only addresses the peer actually
                // claims to listen on — the connection's ephemeral source port
                // is useless to anyone else.
                let speaks_kad = info
                    .protocols
                    .iter()
                    .any(|p| self.swarm.behaviour().kad.protocol_names().contains(p));
                if speaks_kad {
                    for addr in &info.listen_addrs {
                        self.swarm
                            .behaviour_mut()
                            .kad
                            .add_address(&peer_id, addr.clone());
                    }
                }

                // (b) Record what this peer observed our address to be. Counting
                // *distinct observers* is what makes this evidence rather than
                // one peer's opinion.
                self.observed_addrs
                    .entry(info.observed_addr)
                    .or_default()
                    .insert(peer_id);

                // (c) Remember what the peer can do. Relay-hop, AutoNAT and
                // dcutr capability are all read off this list.
                self.peer_protocols.insert(
                    peer_id,
                    info.protocols.iter().map(|p| p.to_string()).collect(),
                );
            }
            identify::Event::Error { peer_id, error } => {
                debug!(peer = %peer_id, error = %error, "identify failed");
            }
            other => trace!(?other, "identify event"),
        }
    }

    fn handle_kad_event(&mut self, event: kad::Event) {
        match event {
            kad::Event::OutboundQueryProgressed {
                id, result, step, ..
            } => {
                // Only act on the final step; intermediate steps would resolve
                // a `oneshot` early and leave the query untracked.
                if !step.last {
                    return;
                }
                match (self.pending_kad.remove(&id), result) {
                    (
                        Some(PendingKad::FindPeer { target, reply }),
                        kad::QueryResult::GetClosestPeers(result),
                    ) => {
                        let found = match &result {
                            Ok(ok) => ok.peers.contains(&target),
                            Err(kad::GetClosestPeersError::Timeout { peers, .. }) => {
                                peers.contains(&target)
                            }
                        };
                        let addrs = self.known_addresses(&target);
                        debug!(peer = %target, found, addrs = addrs.len(), "dht find_peer complete");
                        let _ = reply.send(Ok(addrs));
                    }
                    (Some(PendingKad::FindPeer { target, reply }), other) => {
                        warn!(peer = %target, ?other, "unexpected result for find_peer query");
                        let _ = reply.send(Err(P2PError::DhtError(
                            "unexpected query result for find_peer".to_string(),
                        )));
                    }
                    (Some(PendingKad::Bootstrap), kad::QueryResult::Bootstrap(result)) => {
                        match result {
                            Ok(ok) => info!(
                                peer = %ok.peer,
                                remaining = ok.num_remaining,
                                "kad bootstrap step complete"
                            ),
                            Err(e) => warn!(error = ?e, "kad bootstrap failed"),
                        }
                    }
                    (Some(PendingKad::Maintenance), _) => {
                        trace!("kad maintenance refresh complete");
                    }
                    (Some(PendingKad::Bootstrap), other) => {
                        trace!(?other, "unexpected result for bootstrap query");
                    }
                    // A query we did not park (e.g. kad's own automatic
                    // republishing) — nothing to resolve.
                    (None, other) => trace!(?other, "untracked kad query completed"),
                }
            }

            kad::Event::RoutingUpdated {
                peer, addresses, ..
            } => {
                trace!(%peer, addrs = addresses.len(), "kad routing table updated");
            }

            kad::Event::ModeChanged { new_mode } => {
                info!(?new_mode, "kad mode changed");
            }

            other => trace!(?other, "kad event"),
        }
    }
}

/// Map a `DialError` onto our error type, keeping the original text — the exact
/// wording matters when diagnosing handshake failures against foreign stacks.
fn dial_error(error: &DialError, peer_id: Option<PeerId>) -> P2PError {
    let who = peer_id
        .map(|p| p.to_base58())
        .unwrap_or_else(|| "unknown peer".to_string());
    match error {
        DialError::NoAddresses => P2PError::InvalidAddress(format!("{who}: no addresses to dial")),
        DialError::WrongPeerId { obtained, .. } => P2PError::ConnectionFailed(format!(
            "{who}: peer id mismatch, remote identified as {obtained}"
        )),
        other => P2PError::ConnectionFailed(format!("{who}: {other}")),
    }
}

/// Extract the `/p2p/<peer-id>` component from a multiaddr, if present.
fn peer_id_from_multiaddr(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| match p {
        libp2p::multiaddr::Protocol::P2p(peer) => Some(peer),
        _ => None,
    })
}

/// Drop a trailing `/p2p/<peer-id>` component.
fn strip_p2p(addr: &Multiaddr) -> Multiaddr {
    addr.iter()
        .filter(|p| !matches!(p, libp2p::multiaddr::Protocol::P2p(_)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_peer_id_from_multiaddr() {
        let addr: Multiaddr =
            "/dns/bootstrap-1.kwaai.ai/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
                .parse()
                .unwrap();
        let peer = peer_id_from_multiaddr(&addr).expect("peer id present");
        assert_eq!(
            peer.to_base58(),
            "QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
        );
    }

    #[test]
    fn no_peer_id_component_yields_none() {
        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/4001".parse().unwrap();
        assert!(peer_id_from_multiaddr(&addr).is_none());
    }

    #[test]
    fn strips_the_p2p_component() {
        let addr: Multiaddr =
            "/ip4/1.2.3.4/tcp/8000/p2p/QmQhRuheeCLEsVD3RsnknM75gPDDqxAb8DhnWgro7KhaJc"
                .parse()
                .unwrap();
        assert_eq!(strip_p2p(&addr).to_string(), "/ip4/1.2.3.4/tcp/8000");
    }
}
