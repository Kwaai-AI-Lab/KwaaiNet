//! [`NetworkHandle`] — the clonable facade over the swarm task.
//!
//! Every method sends a [`Command`] carrying a `oneshot::Sender` down an
//! `mpsc` channel to [`crate::service::NetworkService`]'s event loop and awaits
//! the reply. The handle never touches the `Swarm` itself, which is what makes
//! it `Clone + Send + Sync` and safe to hand to unrelated tasks.
//!
//! Method names deliberately mirror `kwaai_p2p_daemon::client::P2PClient`
//! (`connect_peer`, `disconnect_peer`, `list_peers`, …) so that later phases
//! can swap the p2pd client for this handle with minimal churn at call sites.
//!
//! **Event-loop discipline:** no command handler may block. Commands that
//! cannot be answered synchronously from swarm state (dials, DHT queries) are
//! parked in a pending map keyed by `ConnectionId`/`QueryId` and resolved when
//! the corresponding swarm event arrives. Every pending entry must be removed
//! on *both* the success and failure event, or the caller waits forever — see
//! the error arms in `service.rs`.

use libp2p::{Multiaddr, PeerId};
use tokio::sync::{mpsc, oneshot};

use crate::error::{P2PError, P2PResult};

/// A connected peer, as reported by [`NetworkHandle::list_peers`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerInfo {
    /// The remote peer's ID.
    pub peer_id: PeerId,
    /// The multiaddr of the connection (remote address for outbound, the
    /// observed send-back address for inbound).
    pub addr: Multiaddr,
    /// Which side opened the connection.
    pub direction: Direction,
}

/// Which end dialed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// We dialed them.
    Outbound,
    /// They dialed us.
    Inbound,
}

impl Direction {
    /// Lowercase label, matching the p2pd control-protocol vocabulary.
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Outbound => "outbound",
            Direction::Inbound => "inbound",
        }
    }
}

/// Commands accepted by the swarm event loop.
///
/// Every variant carries a `oneshot::Sender` so that a caller always learns the
/// outcome — including "the service is shutting down", which surfaces as a
/// dropped sender and therefore a `RecvError` on the handle side.
#[derive(Debug)]
pub enum Command {
    /// Dial a multiaddr (which must carry `/p2p/<peer-id>`), resolving when the
    /// connection is established or the dial fails.
    ConnectPeer {
        addr: Multiaddr,
        reply: oneshot::Sender<P2PResult<PeerId>>,
    },
    /// Close all connections to a peer.
    DisconnectPeer {
        peer: PeerId,
        reply: oneshot::Sender<P2PResult<()>>,
    },
    /// Snapshot of currently connected peers.
    ListPeers {
        reply: oneshot::Sender<Vec<PeerInfo>>,
    },
    /// Addresses other peers reported observing us at, with a count of how many
    /// distinct peers reported each.
    ObservedAddrs {
        reply: oneshot::Sender<Vec<(Multiaddr, usize)>>,
    },
    /// Addresses the swarm is actually listening on.
    ListenAddrs {
        reply: oneshot::Sender<Vec<Multiaddr>>,
    },
    /// Kademlia lookup for a peer's addresses.
    DhtFindPeer {
        peer: PeerId,
        reply: oneshot::Sender<P2PResult<Vec<Multiaddr>>>,
    },
    /// Insert an address into the Kademlia routing table.
    AddKadAddress {
        peer: PeerId,
        addr: Multiaddr,
        reply: oneshot::Sender<()>,
    },
    /// Dial every initial peer, then run `kad.bootstrap()`.
    Bootstrap {
        peers: Vec<Multiaddr>,
        reply: oneshot::Sender<P2PResult<()>>,
    },
    /// Stop the event loop.
    Shutdown { reply: oneshot::Sender<()> },
}

/// Clonable control handle for a running [`crate::service::NetworkService`].
#[derive(Debug, Clone)]
pub struct NetworkHandle {
    local_peer_id: PeerId,
    commands: mpsc::Sender<Command>,
}

impl NetworkHandle {
    pub(crate) fn new(local_peer_id: PeerId, commands: mpsc::Sender<Command>) -> Self {
        Self {
            local_peer_id,
            commands,
        }
    }

    /// This node's peer ID. Available without touching the event loop.
    pub fn peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// This node's peer ID in base58 (`12D3Koo…` / `Qm…`).
    pub fn local_peer_id(&self) -> String {
        self.local_peer_id.to_base58()
    }

    /// Send a command and await its reply, mapping a dead event loop onto
    /// [`P2PError::NotInitialized`].
    async fn call<T>(&self, make: impl FnOnce(oneshot::Sender<T>) -> Command) -> P2PResult<T> {
        let (tx, rx) = oneshot::channel();
        self.commands
            .send(make(tx))
            .await
            .map_err(|_| P2PError::NotInitialized)?;
        rx.await.map_err(|_| P2PError::NotInitialized)
    }

    /// Dial `multiaddr_str`, which must include a `/p2p/<peer-id>` component.
    /// Resolves once the connection is established.
    pub async fn connect_peer(&self, multiaddr_str: &str) -> P2PResult<PeerId> {
        let addr: Multiaddr = multiaddr_str
            .parse()
            .map_err(|e| P2PError::InvalidAddress(format!("{multiaddr_str}: {e}")))?;
        self.call(|reply| Command::ConnectPeer { addr, reply })
            .await?
    }

    /// Close all connections to `peer`.
    pub async fn disconnect_peer(&self, peer: PeerId) -> P2PResult<()> {
        self.call(|reply| Command::DisconnectPeer { peer, reply })
            .await?
    }

    /// Currently connected peers.
    pub async fn list_peers(&self) -> P2PResult<Vec<PeerInfo>> {
        self.call(|reply| Command::ListPeers { reply }).await
    }

    /// Addresses peers have observed us at, paired with the number of distinct
    /// peers that reported each. A high count is the signal later phases use to
    /// promote an address to a confirmed external address.
    pub async fn observed_addrs(&self) -> P2PResult<Vec<(Multiaddr, usize)>> {
        self.call(|reply| Command::ObservedAddrs { reply }).await
    }

    /// Addresses the swarm is listening on (post-resolution, so an ephemeral
    /// `/tcp/0` shows the real port).
    pub async fn listen_addrs(&self) -> P2PResult<Vec<Multiaddr>> {
        self.call(|reply| Command::ListenAddrs { reply }).await
    }

    /// Resolve a peer's addresses through Kademlia.
    ///
    /// Runs `get_closest_peers(peer)` and, when the query completes, reads the
    /// addresses that the walk deposited in the local routing table. Returns an
    /// empty vec if the peer was not found.
    pub async fn dht_find_peer(&self, peer: PeerId) -> P2PResult<Vec<Multiaddr>> {
        self.call(|reply| Command::DhtFindPeer { peer, reply })
            .await?
    }

    /// Add a known address for `peer` to the Kademlia routing table.
    pub async fn add_kad_address(&self, peer: PeerId, addr: Multiaddr) -> P2PResult<()> {
        self.call(|reply| Command::AddKadAddress { peer, addr, reply })
            .await
    }

    /// Dial each address in `initial_peers` and then run a Kademlia bootstrap.
    ///
    /// Succeeds if *at least one* peer was reachable; fails only when every
    /// dial failed (or the list was empty), because a partially-reachable
    /// bootstrap set is the normal case on a live network.
    pub async fn bootstrap(&self, initial_peers: Vec<Multiaddr>) -> P2PResult<()> {
        self.call(|reply| Command::Bootstrap {
            peers: initial_peers,
            reply,
        })
        .await?
    }

    /// Ask the event loop to stop. Returns once it has acknowledged; await the
    /// `JoinHandle` from `NetworkService::spawn` to know it has fully exited.
    pub async fn shutdown(&self) -> P2PResult<()> {
        self.call(|reply| Command::Shutdown { reply }).await
    }
}
