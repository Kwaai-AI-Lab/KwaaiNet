//! Error types for the P2P networking layer

use thiserror::Error;

/// Result type for P2P operations
pub type P2PResult<T> = Result<T, P2PError>;

/// Errors that can occur in P2P networking
///
/// `Clone` is required because the swarm event loop fans a single failure out
/// to every parked request when it shuts down.
#[derive(Error, Debug, Clone)]
pub enum P2PError {
    /// Failed to connect to peer
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Failed to dial peer
    #[error("Dial failed: {0}")]
    DialFailed(String),

    /// A dial was skipped because a connection to the peer already exists (or
    /// one is already being established).
    ///
    /// Not a failure — the caller's goal is already met. Distinct from
    /// [`P2PError::DialFailed`] so a caller that only wants *a* connection can
    /// treat it as success, while one that wanted a *new* connection can see
    /// that it did not get one.
    #[error("Already connected")]
    AlreadyConnected,

    /// DHT operation failed
    #[error("DHT operation failed: {0}")]
    DhtError(String),

    /// Request timed out
    #[error("Request timed out after {0}ms")]
    Timeout(u64),

    /// Peer not found
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Invalid address
    #[error("Invalid multiaddress: {0}")]
    InvalidAddress(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Transport error
    #[error("Transport error: {0}")]
    Transport(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Network not initialized
    #[error("Network not initialized")]
    NotInitialized,

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for P2PError {
    fn from(err: std::io::Error) -> Self {
        P2PError::Internal(err.to_string())
    }
}

impl From<libp2p::multiaddr::Error> for P2PError {
    fn from(err: libp2p::multiaddr::Error) -> Self {
        P2PError::InvalidAddress(err.to_string())
    }
}
