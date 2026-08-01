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
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use futures::StreamExt;
use libp2p::{
    autonat,
    core::ConnectedPoint,
    dcutr, identify, identity, kad, noise, ping, relay,
    swarm::{ConnectionId, DialError, SwarmEvent},
    tcp, upnp, yamux, Multiaddr, PeerId, Swarm, SwarmBuilder,
};
use tokio::sync::{mpsc, oneshot, watch};
use tracing::{debug, info, trace, warn};

use crate::addresses::{
    dest_peer_id, is_announceable_with, peer_id_from_multiaddr, strip_dest_p2p, strip_p2p,
};
use crate::behaviour::{KwaaiBehaviour, KwaaiBehaviourEvent};
use crate::config::NetworkConfig;
use crate::error::{P2PError, P2PResult};
use crate::handle::{
    parse_protocols, Command, Direction, InboundStreamSender, InboundUnaryCall, InboundUnarySender,
    NetworkHandle, NetworkSnapshot, PeerInfo,
};
use crate::raw_stream;
use crate::reachability::{AnnounceState, Effect, Reachability, ReachabilityState, IDENTIFY_GRACE};
use crate::relay_manager::{RelayAction, RelayManager};
use crate::unary::{self, UnaryProtocol};

/// How often the maintenance arm refreshes the Kademlia routing table.
const KAD_REFRESH_INTERVAL: Duration = Duration::from_secs(5 * 60);

/// Depth of the command channel. Deep enough that bursts of handle calls do not
/// serialize on the event loop, shallow enough to apply backpressure.
const COMMAND_CHANNEL_SIZE: usize = 64;

/// How often the relay manager retries candidates whose backoff has expired.
const RELAY_TICK_INTERVAL: Duration = Duration::from_secs(15);

/// Cap on routing-table addresses per peer.
///
/// kad's own `Addresses` is an unbounded `SmallVec`: `insert` appends whatever
/// it is given, and the only removal is reactive — `address_failed` drops one
/// address per failed dial. Feeding it faster than dials fail makes the list
/// grow without limit, and every entry is then dialed on the next attempt.
///
/// A peer behind a symmetric NAT supplies exactly that. Each of its outbound
/// flows gets a fresh public port, it reports those ports as listen addresses
/// over identify, and none of them is dialable once the flow it belonged to
/// has closed. Measured in the NAT test bed: node-a accumulated 20+ addresses
/// for node-h and re-added them ~7x faster than kad evicted them, fanning
/// every dial out across the whole stale set.
///
/// With port reuse on, those dials all share one local port, so repeats
/// against an address already being attempted collide on the 4-tuple and fail
/// `AddrNotAvailable`. The cap is what keeps that from happening; it is not a
/// port-exhaustion guard (a reused port exhausts nothing).
///
/// Six is `Addresses`' own `SmallVec` inline size — the width kad is built
/// around, so staying at or under it also keeps the list from spilling to the
/// heap.
///
/// go-libp2p bounds the same growth by TTL rather than count, ageing observed
/// addresses out faster than listen addresses. rust-libp2p's kad has no TTL
/// layer, so a count cap is the closest available approximation.
const MAX_ADDRESSES_PER_PEER: usize = 6;

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
    /// Stream requests parked behind a `RoutedDial` lookup, flushed when the
    /// lookup completes or a connection to the peer establishes first.
    pending_routed: HashMap<PeerId, Vec<RoutedRequest>>,
    /// Live connections, per peer, keyed by connection so multiple connections
    /// to one peer are tracked independently.
    connections: HashMap<PeerId, HashMap<ConnectionId, Connection>>,
    /// Addresses peers reported observing us at → the set of peers that said so.
    /// A set (not a counter) so repeated identifies from one peer count once.
    observed_addrs: HashMap<Multiaddr, HashSet<PeerId>>,
    /// Whether the reserved documentation/benchmarking ranges count as
    /// unroutable. Mirrors `NetworkConfig::require_global_ips`; held here
    /// because identify-learned addresses are filtered before they reach kad,
    /// and that decision has to match the one the reachability state makes.
    require_global_ips: bool,
    /// The protocol list each connected peer advertised over identify. This is
    /// the capability feed: relay-hop support, AutoNAT and dcutr all show up
    /// here. Dropped when the last connection to a peer closes, so an entry
    /// always describes a peer we can act on.
    peer_protocols: HashMap<PeerId, Vec<String>>,
    /// Most recent ping round-trip time per peer. Sampled: overwritten on every
    /// ping event rather than accumulated, so this is "latency now", not an
    /// average. Same lifetime as `peer_protocols` — dropped with the last
    /// connection, so an entry always describes a peer we can act on.
    peer_rtt: HashMap<PeerId, Duration>,
    /// The agent version each connected peer advertised over identify (the
    /// remote's software build, e.g. `kwaainet/0.5.4`). Same lifetime as
    /// `peer_protocols`.
    peer_agent: HashMap<PeerId, String>,
    /// Registered unary handlers: negotiated protocol → the handler task's
    /// channel. Kept in lockstep with the behaviour's inbound protocol set, so
    /// an entry here means the protocol is advertised and vice versa.
    unary_handlers: HashMap<String, InboundUnarySender>,
    /// Registered **raw-stream** handlers: negotiated protocol → the accepting
    /// task's channel. Same lockstep invariant as `unary_handlers`, against a
    /// separate protocol set — the two namespaces are independent, so a name
    /// registered here is not callable as a unary handler and vice versa.
    stream_handlers: HashMap<String, InboundStreamSender>,
    /// Are we reachable from the outside, and on what evidence. Owns no I/O:
    /// it returns [`Effect`]s that this loop applies.
    reachability: ReachabilityState,
    /// Circuit reservations held while we are unreachable. A plain state
    /// machine rather than a task or a behaviour, because it needs `&mut Swarm`
    /// and swarm-level `ListenerClosed` events — neither of which a behaviour
    /// or a separate task can see.
    relays: RelayManager,
    /// Publishes [`AnnounceState`] to the announce loop. Sends are gated on a
    /// real change, so a consumer that only ever reacts to `changed()` does not
    /// re-announce on address churn.
    announce_tx: watch::Sender<AnnounceState>,
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
    /// A peer lookup running on behalf of parked stream requests (see
    /// `pending_routed`): on completion, forward them — the walk has populated
    /// the routing table with whatever addresses exist.
    RoutedDial { target: PeerId },
}

/// A stream request parked while a `RoutedDial` lookup finds addresses for its
/// peer.
///
/// Go's p2pd wraps its host in a *routed host*: `NewStream` to a peer with no
/// known addresses transparently runs a DHT `FindPeer` first. rust-libp2p has
/// no equivalent — Kademlia only serves addresses it already has — so the
/// service replicates that here for the commands that dial by bare peer ID.
enum RoutedRequest {
    Unary {
        proto: String,
        data: Vec<u8>,
        reply: oneshot::Sender<unary::UnaryResult>,
    },
    RawStream {
        protos: Vec<String>,
        reply: oneshot::Sender<raw_stream::OpenResult>,
    },
    /// `ConnectPeer` with a bare `/p2p/<id>` address — no way to reach the
    /// peer was supplied, so the connect *is* the lookup. Go's daemon accepts
    /// exactly this from `shard run`'s pre-connect pass.
    Connect {
        reply: oneshot::Sender<P2PResult<PeerId>>,
    },
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
                // No `.port_reuse(true)` here — since libp2p 0.54 the option is
                // deprecated and does nothing, because reuse is decided per
                // connection by the behaviour that asks for the dial.
                //
                // That per-connection policy is what DCUtR needs and what a
                // global flag got wrong. A hole punch is a simultaneous open:
                // each peer's outbound SYN opens the pinhole that admits the
                // other's, which only works if the port each side dials *from*
                // is the port the other side is dialing *to* — the listen port.
                // `libp2p-dcutr` therefore requests `PortUse::Reuse` on its
                // punch dials, and gets it, without every ordinary dial having
                // to share one local port.
                //
                // Forcing reuse globally (which is what the old flag did) made
                // every dial bind the listen port, so a second dial to an
                // endpoint already connected collided on the 4-tuple and failed
                // `AddrNotAvailable`. Upstream hit the same wall and added a
                // fallback in libp2p-tcp 0.44: on that error it re-dials from a
                // fresh port rather than failing the connection.
                //
                // Neither mechanism rescues a symmetric NAT, which re-maps the
                // port per destination — that is what the relay fallback is for.
                tcp::Config::default().nodelay(true),
                noise::Config::new,
                yamux::Config::default,
            )
            .context("configuring TCP transport")?
            // DNS resolution is required for `/dns/bootstrap-N.kwaai.ai/...`
            // addresses; without it those dials fail at the transport layer.
            .with_dns()
            .context("configuring DNS resolution")?
            // Wraps the transport so `/…/p2p-circuit` addresses dial through a
            // relay, and hands back the matching client behaviour. It must come
            // after `with_dns` (the circuit transport wraps the resolved one),
            // and it is why the `with_behaviour` closure below takes two
            // arguments rather than one.
            .with_relay_client(noise::Config::new, yamux::Config::default)
            .context("configuring the relay client transport")?
            .with_behaviour(|kp, relay_client| {
                KwaaiBehaviour::new(kp, &behaviour_config, relay_client)
            })
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

        // A declared external address is an instruction, not a guess, so a
        // malformed one is a hard error — silently ignoring it would leave the
        // node quietly unreachable in exactly the deployment that took the
        // trouble to configure it.
        let declared = config
            .external_addr
            .as_deref()
            .map(|addr| {
                addr.parse::<Multiaddr>()
                    .with_context(|| format!("parsing external_addr {addr}"))
            })
            .transpose()?;

        let (reachability, startup_effects) = ReachabilityState::new(
            config.force_private,
            declared,
            config.identify_min_confirmations,
            config.require_global_ips,
        );

        let (tx, rx) = mpsc::channel(COMMAND_CHANNEL_SIZE);
        let (announce_tx, announce_rx) = watch::channel(AnnounceState::initial());
        let mut service = Self {
            swarm,
            commands: rx,
            pending_dials: HashMap::new(),
            pending_kad: HashMap::new(),
            pending_routed: HashMap::new(),
            connections: HashMap::new(),
            observed_addrs: HashMap::new(),
            require_global_ips: config.require_global_ips,
            peer_protocols: HashMap::new(),
            peer_rtt: HashMap::new(),
            peer_agent: HashMap::new(),
            unary_handlers: HashMap::new(),
            stream_handlers: HashMap::new(),
            reachability,
            relays: RelayManager::new(&config.trusted_relays, config.max_relay_reservations),
            announce_tx,
        };
        service.apply_reachability_effects(startup_effects);
        // `force_private` starts Private, so this is what makes reservations
        // begin at t=0 rather than after the grace period.
        service.sync_relay_enablement();
        // A declared external address or `force_private` means the node already
        // knows where it stands, so the announce loop can start immediately
        // instead of waiting out the grace period.
        service.publish_announce_state();

        let task = tokio::spawn(service.run());
        info!(peer_id = %local_peer_id, "network service started");
        Ok((
            NetworkHandle::new(local_peer_id, tx, config.request_timeout, announce_rx),
            task,
        ))
    }

    /// The event loop. Exits on `Command::Shutdown` or when all handles drop.
    async fn run(mut self) {
        let mut maintenance = tokio::time::interval(KAD_REFRESH_INTERVAL);
        // The first tick fires immediately; skip it so startup does not race
        // the initial bootstrap dial.
        maintenance.tick().await;

        // Fires once. Until it does, "no evidence" means "not yet"; after it,
        // the identify-consensus fallback decides and a node that has heard
        // nothing settles on Private rather than staying Unknown — and
        // therefore silent — indefinitely.
        let identify_grace = tokio::time::sleep(IDENTIFY_GRACE);
        tokio::pin!(identify_grace);
        let mut grace_fired = false;

        // Retries relays whose backoff has expired. Short, because it is the
        // only thing that recovers a node whose every candidate was in backoff
        // the last time it tried.
        let mut relay_tick = tokio::time::interval(RELAY_TICK_INTERVAL);
        relay_tick.tick().await;

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
                _ = &mut identify_grace, if !grace_fired => {
                    grace_fired = true;
                    let effects = self.reachability.on_grace_elapsed(&self.observed_addrs);
                    self.apply_reachability_effects(effects);
                    self.sync_relay_enablement();
                }
                _ = relay_tick.tick() => {
                    let actions = self.relays.on_tick(Instant::now());
                    self.apply_relay_actions(actions);
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
            Command::ConnectPeer { addr, reply } => {
                match peer_id_from_multiaddr(&addr) {
                    // A bare `/p2p/<id>` carries no way to reach the peer:
                    // resolve it through the DHT like Go's routed host instead
                    // of dialing an address that has no transport.
                    Some(peer) if strip_p2p(&addr).is_empty() => {
                        self.dispatch_routed(peer, RoutedRequest::Connect { reply });
                    }
                    _ => match self.dial(addr) {
                        Ok(connection_id) => {
                            self.pending_dials.insert(connection_id, reply);
                        }
                        Err(e) => {
                            let _ = reply.send(Err(e));
                        }
                    },
                }
            }

            Command::DisconnectPeer { peer, reply } => {
                let result = match self.swarm.disconnect_peer_id(peer) {
                    Ok(()) => Ok(()),
                    Err(()) => Err(P2PError::PeerNotFound(peer.to_base58())),
                };
                self.connections.remove(&peer);
                let _ = reply.send(result);
            }

            Command::ListPeers { reply } => {
                let _ = reply.send(self.collect_peers());
            }

            Command::NetworkSnapshot { reply } => {
                let routing = self.collect_routing_peers();
                let mut observed: Vec<(Multiaddr, usize)> = self
                    .observed_addrs
                    .iter()
                    .map(|(addr, observers)| (addr.clone(), observers.len()))
                    .collect();
                observed.sort_by(|a, b| b.1.cmp(&a.1));

                let _ = reply.send(NetworkSnapshot {
                    local_peer_id: *self.swarm.local_peer_id(),
                    peers: self.collect_peers(),
                    routing,
                    reachability: self.reachability.current().clone(),
                    relay_addrs: self.relays.confirmed_addrs(),
                    observed_addrs: observed,
                    listen_addrs: self.swarm.listeners().cloned().collect(),
                });
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

            Command::Reachability { reply } => {
                let _ = reply.send(self.reachability.current().clone());
            }

            // A walk over in-memory k-buckets — bounded by the routing table
            // size (k=20 per bucket, 256 buckets) and never touching the
            // network, so it is safe in the event loop.
            Command::RoutingPeers { reply } => {
                let _ = reply.send(self.collect_routing_peers());
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
                // Deliberately not capped (unlike the identify path): this is
                // the operator naming an address, not a peer claiming one. A
                // symmetric-NAT peer flooding identify must never be able to
                // evict a bootstrap address someone configured by hand.
                self.swarm.behaviour_mut().kad.add_address(&peer, addr);
                let _ = reply.send(());
            }

            Command::Bootstrap { peers, reply } => {
                let _ = reply.send(self.start_bootstrap(peers));
            }

            // While parked in `pending_routed` the service owns `reply`; see
            // `fail_all_pending`.
            Command::CallUnary {
                peer,
                proto,
                data,
                reply,
            } => {
                self.dispatch_routed(peer, RoutedRequest::Unary { proto, data, reply });
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

            // Same ownership story as `CallUnary`.
            Command::OpenRawStream {
                peer,
                protos,
                reply,
            } => {
                self.dispatch_routed(peer, RoutedRequest::RawStream { protos, reply });
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

    /// One [`PeerInfo`] per live connection, enriched with whatever identify
    /// and ping have learned so far.
    ///
    /// Per-connection, not per-peer: a peer with both a relay path and a direct
    /// path appears twice, deliberately — that duplication is how a hole-punch
    /// upgrade becomes visible. The enrichment fields are keyed by peer, so the
    /// two rows share them.
    fn collect_peers(&self) -> Vec<PeerInfo> {
        self.connections
            .iter()
            .flat_map(|(peer_id, conns)| {
                let rtt = self.peer_rtt.get(peer_id).copied();
                let agent_version = self.peer_agent.get(peer_id).cloned();
                let protocols = self
                    .peer_protocols
                    .get(peer_id)
                    .cloned()
                    .unwrap_or_default();
                conns.values().map(move |c| PeerInfo {
                    peer_id: *peer_id,
                    addr: c.addr.clone(),
                    direction: c.direction,
                    rtt,
                    agent_version: agent_version.clone(),
                    protocols: protocols.clone(),
                })
            })
            .collect()
    }

    /// Every peer in the Kademlia routing table.
    ///
    /// A walk over in-memory k-buckets — bounded by the routing table size
    /// (k=20 per bucket, 256 buckets) and never touching the network, so it is
    /// safe in the event loop.
    fn collect_routing_peers(&mut self) -> Vec<PeerId> {
        self.swarm
            .behaviour_mut()
            .kad
            .kbuckets()
            .flat_map(|bucket| {
                bucket
                    .iter()
                    .map(|entry| *entry.node.key.preimage())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Dial `addr`, seeding the routing table with it when it carries a
    /// `/p2p/<peer-id>` component.
    fn dial(&mut self, addr: Multiaddr) -> P2PResult<ConnectionId> {
        use libp2p::swarm::dial_opts::DialOpts;

        // `dest_peer_id`, not `peer_id_from_multiaddr`: the latter returns the
        // *first* `/p2p`, which on a circuit address is the relay. Filing the
        // circuit under the relay's key would both pollute the relay's entry
        // and leave the destination with no route.
        if let Some(peer) = dest_peer_id(&addr) {
            // Kad wants the address without the trailing /p2p component; it
            // re-attaches it itself via `with_p2p`. A bare `/p2p/<id>` strips
            // to an *empty* multiaddr — seeding that would poison the routing
            // table with an undialable entry that `known_addresses` then
            // mistakes for a way to reach the peer.
            //
            // `strip_dest_p2p` keeps a circuit's relay hop: without it the
            // stored entry reads `/…/p2p-circuit` with no relay named, and
            // every later dial of it fails with "Missing relay peer id".
            let stripped = strip_dest_p2p(&addr);
            if !stripped.is_empty() {
                // Uncapped for the same reason as `AddKadAddress`: an address
                // we are actively dialing is our own intent, not a remote
                // claim, and is the one entry least worth evicting.
                self.swarm.behaviour_mut().kad.add_address(&peer, stripped);
            }
        }

        // `allocate_new_port()`, overriding the `DialOpts` default of
        // best-effort port reuse.
        //
        // Reuse asks the transport to bind our listen port as the dial's source
        // port. That is right for a DCUtR hole punch — `libp2p-dcutr` requests
        // it on its own dials and still gets it — but wrong for an ordinary
        // dial, because the bind fails outright: `SO_REUSEPORT` lets several
        // *listeners* share a port, not a listener plus an outbound connect.
        // Measured on macOS (errno 48) and Linux (errno 98) alike, with no
        // prior connection to the target.
        //
        // libp2p-tcp 0.44 does retry on a fresh port, but only for
        // `AddrNotAvailable` raised by `connect`. Our failure is `EADDRINUSE`
        // from `bind`, one step earlier, where the `?` propagates before the
        // fallback can run — so the dial simply fails.
        // Built through `unknown_peer_id().address(..)` rather than
        // `DialOpts::from(addr)`, because only the builder exposes
        // `allocate_new_port()`; the two are otherwise the same dial.
        let opts = DialOpts::unknown_peer_id()
            .address(addr.clone())
            .allocate_new_port()
            .build();
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
        let strict = self.require_global_ips;
        let mut addrs: Vec<Multiaddr> = Vec::new();

        if let Some(bucket) = self.swarm.behaviour_mut().kad.kbucket(*peer) {
            for entry in bucket.iter() {
                if entry.node.key.preimage() == peer {
                    // Skip empty entries: an address with no transport cannot
                    // be dialed, only mistaken for reachability.
                    //
                    // Filter here, on the way *out*, because kad has more ways
                    // in than we control. Identify is filtered at the handler,
                    // but `kad::Behaviour::discovered` also inserts whatever
                    // `multiaddrs` a remote peer reports during a query walk,
                    // verbatim and inside libp2p — so a peer's loopback and LAN
                    // addresses arrive without passing anything of ours. This
                    // is the one place every consumer (connect, routed dial,
                    // the GUI's peer table) reads them back, so one check here
                    // covers paths that would otherwise need finding one at a
                    // time.
                    addrs.extend(
                        entry
                            .node
                            .value
                            .iter()
                            .filter(|a| !a.is_empty() && is_announceable_with(a, strict))
                            .cloned(),
                    );
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

        // Hand back addresses *without* the destination `/p2p/<peer-id>`: kad
        // stores fully-qualified ones (since libp2p 0.54) while connection
        // addresses never carry one, and callers append the id themselves — so
        // a mixed bag yields `/p2p/<id>/p2p/<id>`, which parses and then fails
        // to dial.
        //
        // Only the *destination* id goes. A circuit address also carries the
        // relay's id before `/p2p-circuit`, and that one has to survive: this
        // is the single place `connect`, the routed dial and the GUI peer table
        // all read addresses back, so stripping it here made every relay-only
        // peer undialable from the native path while p2pd reached them fine.
        for addr in &mut addrs {
            *addr = strip_dest_p2p(addr);
        }
        addrs.retain(|a| !a.is_empty());
        addrs.dedup();

        addrs
    }

    /// Route a stream request to `peer`, looking its addresses up in the DHT
    /// first when we have none — Go's routed-host semantics (see
    /// [`RoutedRequest`]). With a connection or known addresses, dispatch is
    /// immediate.
    fn dispatch_routed(&mut self, peer: PeerId, request: RoutedRequest) {
        if self.swarm.is_connected(&peer) || !self.known_addresses(&peer).is_empty() {
            self.forward_routed(peer, request);
            return;
        }
        let parked = self.pending_routed.entry(peer).or_default();
        // One lookup per burst: requests parked while a lookup is in flight
        // ride along on its completion.
        if parked.is_empty() {
            let query_id = self.swarm.behaviour_mut().kad.get_closest_peers(peer);
            self.pending_kad
                .insert(query_id, PendingKad::RoutedDial { target: peer });
            debug!(%peer, "no addresses for stream request — running find_peer first");
        }
        parked.push(request);
    }

    /// Hand a routed request to its behaviour, which owns `reply` from here on.
    fn forward_routed(&mut self, peer: PeerId, request: RoutedRequest) {
        match request {
            RoutedRequest::Unary { proto, data, reply } => {
                self.swarm.behaviour_mut().unary.send_request(
                    peer,
                    UnaryProtocol::new(proto),
                    data,
                    reply,
                );
            }
            RoutedRequest::RawStream { protos, reply } => {
                self.swarm.behaviour_mut().raw_stream.open_stream(
                    peer,
                    parse_protocols(&protos),
                    reply,
                );
            }
            RoutedRequest::Connect { reply } => {
                use libp2p::swarm::dial_opts::DialOpts;
                if self.swarm.is_connected(&peer) {
                    let _ = reply.send(Ok(peer));
                    return;
                }
                // A new port for the same reason as `dial()`: reuse
                // asks the transport to bind our listen port, which
                // `EADDRINUSE`s against our own listener.
                let opts = DialOpts::peer_id(peer).allocate_new_port().build();
                let connection_id = opts.connection_id();
                match self.swarm.dial(opts) {
                    Ok(()) => {
                        self.pending_dials.insert(connection_id, reply);
                    }
                    Err(e) => {
                        let _ = reply.send(Err(dial_error(&e, Some(peer))));
                    }
                }
            }
        }
    }

    /// Flush every request parked for `peer`: forward them when the lookup (or
    /// an incidental connection) produced a way to reach the peer, fail them
    /// with a clear error when it did not.
    fn flush_routed(&mut self, peer: PeerId) {
        let Some(parked) = self.pending_routed.remove(&peer) else {
            return;
        };
        if self.swarm.is_connected(&peer) || !self.known_addresses(&peer).is_empty() {
            for request in parked {
                self.forward_routed(peer, request);
            }
            return;
        }
        let text = format!("{peer}: peer not found in DHT (no addresses)");
        debug!(%peer, "find_peer produced no addresses — failing parked requests");
        for request in parked {
            match request {
                RoutedRequest::Unary { reply, .. } => {
                    let _ = reply.send(Err(unary::UnaryError::DialFailure(text.clone())));
                }
                RoutedRequest::RawStream { reply, .. } => {
                    let _ = reply.send(Err(raw_stream::RawStreamError::DialFailure(text.clone())));
                }
                RoutedRequest::Connect { reply } => {
                    let _ = reply.send(Err(P2PError::DialFailed(text.clone())));
                }
            }
        }
    }

    /// Seed one routing-table address for `peer`, holding the per-peer list at
    /// `MAX_ADDRESSES_PER_PEER`.
    ///
    /// Every path that learns an address from a *remote claim* goes through
    /// here. kad applies no bound of its own (see `MAX_ADDRESSES_PER_PEER`), so
    /// without this a peer that reports a fresh address on every identify grows
    /// its entry without limit and every stale entry is redialed forever.
    ///
    /// At the cap the oldest address is evicted to make room. Refusing the new
    /// one instead would be simpler but wrong: a peer that legitimately moves
    /// would be frozen at the six addresses it happened to report first, and
    /// could never become reachable again. Oldest-out keeps the list tracking
    /// where the peer is *now*, which is also the order kad's own
    /// `Addresses::insert` appends in.
    ///
    /// Returns whether the address was seeded.
    fn add_routing_address(&mut self, peer: &PeerId, addr: Multiaddr) -> bool {
        let existing: Vec<Multiaddr> = self
            .swarm
            .behaviour_mut()
            .kad
            .kbucket(*peer)
            .into_iter()
            .flat_map(|bucket| {
                bucket
                    .iter()
                    .filter(|entry| entry.node.key.preimage() == peer)
                    .flat_map(|entry| entry.node.value.iter().cloned())
                    .collect::<Vec<_>>()
            })
            .collect();

        // Already known. kad would dedupe this itself, but returning early
        // keeps a repeat identify from counting as churn.
        //
        // Compare with `/p2p/<peer-id>` stripped: kad stores fully-qualified
        // addresses, identify reports bare ones, and treating those as distinct
        // would let the same address occupy several of the six slots.
        let bare = strip_p2p(&addr);
        if existing.iter().any(|a| strip_p2p(a) == bare) {
            return false;
        }

        if existing.len() >= MAX_ADDRESSES_PER_PEER {
            // `remove` refuses to drop the last address, which is what we want:
            // kad keeps a peer in the table with one address rather than
            // flushing it on a transient failure.
            if let Some(oldest) = existing.first() {
                let removed = self
                    .swarm
                    .behaviour_mut()
                    .kad
                    .remove_address(peer, oldest)
                    .is_none();
                trace!(
                    %peer,
                    %oldest,
                    %addr,
                    peer_removed = removed,
                    "routing address cap reached; evicting the oldest"
                );
            }
        }

        self.swarm.behaviour_mut().kad.add_address(peer, addr);
        true
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
        for (_, parked) in self.pending_routed.drain() {
            for request in parked {
                match request {
                    RoutedRequest::Unary { reply, .. } => {
                        let _ = reply.send(Err(unary::UnaryError::DialFailure(error.to_string())));
                    }
                    RoutedRequest::RawStream { reply, .. } => {
                        let _ = reply.send(Err(raw_stream::RawStreamError::DialFailure(
                            error.to_string(),
                        )));
                    }
                    RoutedRequest::Connect { reply } => {
                        let _ = reply.send(Err(error.clone()));
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Swarm events
    // ------------------------------------------------------------------

    fn handle_swarm_event(&mut self, event: SwarmEvent<KwaaiBehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr {
                listener_id,
                address,
            } => {
                info!(%address, "listening on new address");
                // A circuit address on a listener we opened means the relay
                // accepted our reservation. relay-client confirms the address
                // as external itself, so there is nothing to add here.
                if self.relays.on_new_listen_addr(listener_id, &address) {
                    // `using_relay` just became true.
                    self.publish_announce_state();
                }
            }

            SwarmEvent::ListenerClosed {
                listener_id,
                reason,
                ..
            } => {
                // The authoritative signal that a reservation is gone.
                // `relay::client::Event` has no failure variant: a refusal or
                // timeout arrives here as Err, and a relay whose connection
                // died arrives as Ok.
                let reason_str;
                let reason = match &reason {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        reason_str = e.to_string();
                        Err(reason_str.as_str())
                    }
                };
                let (actions, lost) =
                    self.relays
                        .on_listener_closed(listener_id, reason, Instant::now());
                self.apply_relay_actions(actions);
                if lost {
                    // A confirmed reservation went away, so `using_relay` may
                    // have dropped. An unconfirmed one never reached the
                    // announce state in the first place.
                    self.publish_announce_state();
                }
            }

            SwarmEvent::ListenerError { listener_id, error } => {
                debug!(?listener_id, error = %error, "listener error");
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
                // A connection beat a pending routed lookup (the peer dialed
                // us, or another dial landed): flush now rather than making
                // the parked requests wait out the DHT walk. The lookup's
                // completion then finds nothing parked and is a no-op.
                if self.pending_routed.contains_key(&peer_id) {
                    self.flush_routed(peer_id);
                }
                // Note: a relay reservation is *not* requested here. It waits
                // for identify — see `RelayManager::on_relay_ready`.
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
                    // Latency and build version are properties of a live
                    // connection too. Keeping them would let a reconnected peer
                    // briefly report the *previous* session's RTT.
                    self.peer_rtt.remove(&peer_id);
                    self.peer_agent.remove(&peer_id);
                    // Same for its opinion about where it saw us. Without this
                    // the map only ever grows, and the identify-consensus
                    // fallback would keep counting observers that left —
                    // latching an address from before a network move and never
                    // letting go of it.
                    self.observed_addrs.retain(|_, observers| {
                        observers.remove(&peer_id);
                        !observers.is_empty()
                    });
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

                // `WrongPeerId` is *proof* this address does not belong to the
                // peer we filed it under — somebody else answered at it. Evict
                // it, or kad hands the same wrong address back on every
                // subsequent dial and the peer stays unreachable for as long as
                // the entry lives.
                //
                // Why eviction has to be evidence-based rather than a filter:
                // kad learns addresses we never see (`Behaviour::discovered`
                // inserts what remote peers report during a query walk, verbatim
                // and inside libp2p), and a dial *by PeerId* — which is how
                // every routed unary and stream request reaches the swarm —
                // asks the behaviours for addresses directly, so it never passes
                // through `known_addresses`' filter. But we cannot pre-emptively
                // drop everything that fails `is_announceable`: that answers
                // "worth telling the world about", not "dialable by me", and
                // loopback and LAN addresses are exactly how two nodes on one
                // machine or one subnet reach each other. Only a failed dial
                // distinguishes the two, so only a failed dial evicts.
                //
                // Observed on 2026-08-10: `/ip4/127.0.0.1/tcp/8080` filed under
                // metro-win's peer id resolved to a *different local node*, so
                // every call to metro-win hit it, failed, and never tried the
                // circuit address that would have worked. Reachability flapped
                // 100%/0% as the entry aged in and out of the table.
                if let (Some(peer), libp2p::swarm::DialError::WrongPeerId { obtained, address }) =
                    (peer_id, &error)
                {
                    warn!(
                        %peer, addr = %address, %obtained,
                        "address answered with a different peer id; evicting from the routing table"
                    );
                    let addr = address.clone();
                    self.swarm.behaviour_mut().kad.remove_address(&peer, &addr);
                }

                // A relay we cannot reach will never give us a reservation.
                if let Some(peer) = peer_id {
                    let actions = self.relays.on_relay_dial_failed(peer, Instant::now());
                    self.apply_relay_actions(actions);
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
                Ok(rtt) => {
                    trace!(peer = %peer, ?rtt, "ping");
                    // Last-write-wins: the peer table reports current latency,
                    // not a smoothed average. A failed ping deliberately leaves
                    // the previous value rather than clearing it — one lost
                    // probe is not evidence the peer is gone, and libp2p will
                    // close the connection (clearing this) if it really is.
                    self.peer_rtt.insert(peer, rtt);
                }
                Err(e) => debug!(peer = %peer, error = %e, "ping failed"),
            },
            KwaaiBehaviourEvent::Unary(unary::Event::InboundRequest { peer, request }) => {
                self.dispatch_unary(peer, request)
            }
            KwaaiBehaviourEvent::RawStream(raw_stream::Event::InboundStream(inbound)) => {
                self.dispatch_stream(inbound)
            }

            // ---- NAT traversal -------------------------------------
            KwaaiBehaviourEvent::Autonat(event) => self.handle_autonat_event(event),

            KwaaiBehaviourEvent::RelayClient(event) => match event {
                relay::client::Event::ReservationReqAccepted {
                    relay_peer_id,
                    renewal,
                    ..
                } => info!(relay = %relay_peer_id, renewal, "relay reservation accepted"),
                relay::client::Event::OutboundCircuitEstablished { relay_peer_id, .. } => {
                    debug!(relay = %relay_peer_id, "outbound circuit established")
                }
                relay::client::Event::InboundCircuitEstablished { src_peer_id, .. } => {
                    debug!(peer = %src_peer_id, "inbound circuit established")
                }
            },

            KwaaiBehaviourEvent::RelayServer(event) => {
                trace!(?event, "relay hop server event")
            }

            KwaaiBehaviourEvent::Dcutr(dcutr::Event {
                remote_peer_id,
                result,
            }) => match result {
                // A success here means the relayed connection was replaced by a
                // direct one — the whole point of holding the circuit.
                Ok(connection_id) => {
                    info!(peer = %remote_peer_id, ?connection_id, "dcutr hole punch succeeded")
                }
                Err(e) => debug!(peer = %remote_peer_id, error = %e, "dcutr hole punch failed"),
            },

            KwaaiBehaviourEvent::Upnp(event) => self.handle_upnp_event(event),
        }
    }

    /// Apply what the reachability machine asked for.
    ///
    /// This is the *only* place external addresses are added or removed, which
    /// is what keeps the swarm's address set and the machine's verdict from
    /// drifting apart. AutoNAT 0.12 never emits `ExternalAddrConfirmed` itself,
    /// so without this a `Public` verdict would be a log line and nothing more:
    /// identify would keep advertising nothing and kad would stay in client
    /// mode.
    fn apply_reachability_effects(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::ConfirmExternal(addr) => {
                    info!(%addr, "confirming external address");
                    self.swarm.add_external_address(addr);
                }
                Effect::RetractExternal(addr) => {
                    info!(%addr, "retracting external address");
                    self.swarm.remove_external_address(&addr);
                }
            }
        }
    }

    /// Recompute the announce state and publish it if anything a consumer acts
    /// on changed.
    ///
    /// The equality gate is the whole point. A node's addresses churn
    /// constantly — identify pushes, a reservation moving between relays, a
    /// re-NAT — and none of that belongs in a DHT record that carries no
    /// addresses (see [`AnnounceState`]). Only a change in *how reachable the
    /// node is* wakes the announce loop.
    fn publish_announce_state(&mut self) {
        let current = self.reachability.current().clone();
        let has_circuit = self.relays.has_circuit();
        self.announce_tx.send_if_modified(|state| {
            let next = AnnounceState::derive(&current, has_circuit, state.epoch);
            if !next.differs(state) {
                return false;
            }
            *state = AnnounceState {
                epoch: state.epoch + 1,
                ..next
            };
            info!(
                reachability = ?state.reachability,
                using_relay = state.using_relay,
                announceable = state.announceable,
                epoch = state.epoch,
                "announce state changed"
            );
            true
        });
    }

    /// Turn relay-seeking on or off to match the current reachability verdict.
    ///
    /// Private wants circuits; Public does not (holding one costs a relay real
    /// resources and routes peers to us the slow way). Unknown deliberately
    /// leaves the manager as it is: during the grace period we do not yet know
    /// enough to start, and a `force_private` node was switched on at startup
    /// and must not be switched off by a transient Unknown.
    fn sync_relay_enablement(&mut self) {
        let actions = match self.reachability.current() {
            Reachability::Private => self.relays.set_enabled(true, Instant::now()),
            Reachability::Public { .. } => self.relays.set_enabled(false, Instant::now()),
            Reachability::Unknown => {
                self.publish_announce_state();
                return;
            }
        };
        self.apply_relay_actions(actions);
        // Reachability moved, and dropping reservations may have moved
        // `using_relay` with it.
        self.publish_announce_state();
    }

    /// Carry out what the relay manager asked for.
    fn apply_relay_actions(&mut self, actions: Vec<RelayAction>) {
        for action in actions {
            match action {
                RelayAction::Dial { relay, relay_addr } => {
                    // Connected *and* identified — the reservation can be
                    // negotiated right now. Having the connection is not
                    // enough: until identify lands, hop is not in the
                    // connection's supported protocol set.
                    if self.connections.contains_key(&relay)
                        && self.peer_protocols.contains_key(&relay)
                    {
                        let next = self.relays.on_relay_ready(relay, Instant::now());
                        self.apply_relay_actions(next);
                        continue;
                    }
                    let dialable = relay_addr.with(libp2p::multiaddr::Protocol::P2p(relay));
                    if let Err(e) = self.dial(dialable) {
                        debug!(%relay, error = %e, "dialing a relay candidate");
                        let next = self.relays.on_relay_dial_failed(relay, Instant::now());
                        self.apply_relay_actions(next);
                    }
                }

                RelayAction::Listen {
                    relay,
                    circuit_addr,
                } => {
                    match self.swarm.listen_on(circuit_addr.clone()) {
                        Ok(id) => {
                            debug!(%relay, %circuit_addr, ?id, "circuit listener opened");
                            self.relays.note_listener(id, relay, Instant::now());
                        }
                        Err(e) => {
                            // No ListenerId exists, so there is no slot to
                            // close — the manager has to be told separately or
                            // this relay would never be marked failed.
                            warn!(%relay, %circuit_addr, error = ?e, "circuit listen failed");
                            let actions = self.relays.note_listen_failed(relay, Instant::now());
                            self.apply_relay_actions(actions);
                        }
                    }
                }
                RelayAction::StopListening(id) => {
                    debug!(?id, "closing a circuit listener");
                    self.swarm.remove_listener(id);
                }
            }
        }
    }

    /// AutoNAT status and probe outcomes.
    fn handle_autonat_event(&mut self, event: autonat::Event) {
        match event {
            autonat::Event::StatusChanged { old, new } => {
                info!(?old, ?new, "autonat reachability status changed");
                let effects = match new {
                    autonat::NatStatus::Public(addr) => self.reachability.on_autonat_public(addr),
                    autonat::NatStatus::Private => self.reachability.on_autonat_private(),
                    // Unknown is not a verdict — it is autonat saying it has
                    // stopped being sure. Whatever we last concluded stands
                    // until something positive replaces it.
                    autonat::NatStatus::Unknown => Vec::new(),
                };
                self.apply_reachability_effects(effects);
                self.sync_relay_enablement();
            }
            autonat::Event::OutboundProbe(probe) => {
                trace!(?probe, "autonat outbound probe");
            }
            autonat::Event::InboundProbe(probe) => {
                trace!(?probe, "autonat inbound probe");
            }
        }
    }

    /// UPnP port-mapping outcomes.
    ///
    /// Every failure arm is informational, never an error: a node with no IGD
    /// gateway, or one behind a gateway that refuses to map, is the *expected*
    /// case on most networks and is exactly what relay reservations are for.
    fn handle_upnp_event(&mut self, event: upnp::Event) {
        match event {
            upnp::Event::NewExternalAddr(addr) => {
                info!(%addr, "upnp mapped an external address");
                let effects = self.reachability.on_upnp_external(addr);
                self.apply_reachability_effects(effects);
                self.sync_relay_enablement();
            }
            upnp::Event::ExpiredExternalAddr(addr) => {
                info!(%addr, "upnp mapping expired");
                let effects = self.reachability.on_upnp_expired(&addr);
                self.apply_reachability_effects(effects);
                self.sync_relay_enablement();
            }
            upnp::Event::GatewayNotFound => {
                debug!("no upnp gateway found; relying on autonat and relays");
            }
            upnp::Event::NonRoutableGateway => {
                debug!("upnp gateway is not on a routable network; ignoring it");
            }
        }
    }

    fn handle_identify_event(&mut self, event: identify::Event) {
        match event {
            identify::Event::Received { peer_id, info, .. } => {
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
                // is useless to anyone else — and only those a *third* party
                // could dial.
                //
                // identify advertises every address the peer is listening on,
                // loopback and LAN-private included, and rust-libp2p filters
                // none of it (`Behaviour::all_addresses` chains listen and
                // external addresses verbatim; the receive side drops only
                // peer-id mismatches). go-libp2p strips these before sending,
                // so this only bites on the native path.
                //
                // Storing them is worse than useless. A peer's `127.0.0.1`
                // resolves to *us*, so dialing it opens a connection to
                // ourselves that fails `WrongPeerId` — and it does so
                // repeatedly, because kad keeps handing back the entry. Its
                // RFC1918 address is unroutable from any other subnet. Both
                // crowd out the circuit address that would actually have
                // worked, which is how a NATed peer ends up unreachable to
                // another NATed peer that can see it perfectly well in the DHT.
                //
                // `is_announceable_with` is the same test the reachability
                // state and relay manager already apply, so a node cannot
                // conclude an address is worth advertising while its peers
                // conclude the reverse. Circuit addresses pass regardless —
                // reaching the relay is what makes them dialable, and that is
                // exactly the address a NATed peer needs.
                let speaks_kad = info
                    .protocols
                    .iter()
                    .any(|p| self.swarm.behaviour().kad.protocol_names().contains(p));
                if speaks_kad {
                    for addr in &info.listen_addrs {
                        if !is_announceable_with(addr, self.require_global_ips) {
                            trace!(
                                peer = %peer_id,
                                %addr,
                                "skipping undialable listen addr from identify"
                            );
                            continue;
                        }
                        self.add_routing_address(&peer_id, addr.clone());
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
                // dcutr capability are all read off this list. The agent
                // version rides along: purely descriptive (nothing branches on
                // it), but it is what makes a peer table diagnosable when one
                // build in the mesh misbehaves.
                self.peer_agent.insert(peer_id, info.agent_version.clone());
                let protocols: Vec<String> = info.protocols.iter().map(|p| p.to_string()).collect();
                // Recorded *before* the relay manager runs: `apply_relay_actions`
                // reads this map to decide whether a reservation can be
                // negotiated immediately, and this identify is precisely what
                // makes that true.
                self.peer_protocols.insert(peer_id, protocols.clone());

                // (d) A peer advertising relay hop is a relay candidate — and
                // an identify from a relay we are already dialing is the signal
                // that its reservation can now be requested at all.
                let actions = self.relays.note_identify(
                    peer_id,
                    &protocols,
                    &info.listen_addrs,
                    Instant::now(),
                );
                self.apply_relay_actions(actions);
            }
            identify::Event::Error { peer_id, error, .. } => {
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
                        // Since libp2p 0.54 a closest-peers result carries
                        // `PeerInfo { peer_id, addrs }` rather than a bare
                        // `PeerId`, so match on the id field.
                        let found = match &result {
                            Ok(ok) => ok.peers.iter().any(|p| p.peer_id == target),
                            Err(kad::GetClosestPeersError::Timeout { peers, .. }) => {
                                peers.iter().any(|p| p.peer_id == target)
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
                    // Whatever the walk's own outcome, it has populated the
                    // routing table with everything it found — flush decides
                    // between forwarding and failing from there.
                    (Some(PendingKad::RoutedDial { target }), _) => {
                        self.flush_routed(target);
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
