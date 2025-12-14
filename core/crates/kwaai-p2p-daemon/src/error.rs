//! Error types for kwaai-p2p-daemon

use thiserror::Error;

/// Result type for daemon operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with the p2p daemon
#[derive(Debug, Error)]
pub enum Error {
    /// IO error (file, socket, etc.)
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Daemon process error
    #[error("Daemon process error: {0}")]
    Process(String),

    /// Protobuf encoding/decoding error
    #[error("Protobuf error: {0}")]
    Protobuf(#[from] prost::DecodeError),

    /// Protocol error from daemon
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Timeout error
    #[error("Operation timed out")]
    Timeout,

    /// Daemon not running
    #[error("Daemon is not running")]
    NotRunning,

    /// Invalid response from daemon
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Stream error
    #[error("Stream error: {0}")]
    Stream(String),

    /// DHT operation error
    #[error("DHT error: {0}")]
    Dht(String),
}
