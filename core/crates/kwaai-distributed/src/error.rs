//! Error types for distributed operations

use thiserror::Error;

/// Result type for distributed operations
pub type DistributedResult<T> = Result<T, DistributedError>;

/// Errors that can occur in distributed operations
#[derive(Error, Debug)]
pub enum DistributedError {
    /// Expert not found
    #[error("Expert not found: {0}")]
    ExpertNotFound(String),

    /// Remote call failed
    #[error("Remote call failed: {0}")]
    RemoteCallFailed(String),

    /// Timeout
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// No peers available
    #[error("No peers available for {0}")]
    NoPeersAvailable(String),

    /// Averaging failed
    #[error("Averaging failed: {0}")]
    AveragingFailed(String),

    /// Routing failed
    #[error("Routing failed: {0}")]
    RoutingFailed(String),

    /// All retries exhausted
    #[error("All retries exhausted: {0}")]
    RetriesExhausted(String),

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Tensor error
    #[error("Tensor error: {0}")]
    TensorError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<kwaai_p2p::P2PError> for DistributedError {
    fn from(err: kwaai_p2p::P2PError) -> Self {
        DistributedError::NetworkError(err.to_string())
    }
}

impl From<kwaai_compression::CompressionError> for DistributedError {
    fn from(err: kwaai_compression::CompressionError) -> Self {
        DistributedError::CompressionError(err.to_string())
    }
}

impl From<candle_core::Error> for DistributedError {
    fn from(err: candle_core::Error) -> Self {
        DistributedError::TensorError(err.to_string())
    }
}
