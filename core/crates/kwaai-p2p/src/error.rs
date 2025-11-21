//! Error types for the P2P networking layer

use thiserror::Error;

/// Result type for P2P operations
pub type P2PResult<T> = Result<T, P2PError>;

/// Errors that can occur in P2P networking
#[derive(Error, Debug)]
pub enum P2PError {
    /// Failed to connect to peer
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Failed to dial peer
    #[error("Dial failed: {0}")]
    DialFailed(String),

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
