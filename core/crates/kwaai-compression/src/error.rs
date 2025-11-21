//! Error types for compression operations

use thiserror::Error;

/// Result type for compression operations
pub type CompressionResult<T> = Result<T, CompressionError>;

/// Errors that can occur during compression
#[derive(Error, Debug)]
pub enum CompressionError {
    /// Compression failed
    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    /// Decompression failed
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    /// Invalid data
    #[error("Invalid compressed data: {0}")]
    InvalidData(String),

    /// Shape mismatch
    #[error("Shape mismatch: expected {expected:?}, got {actual:?}")]
    ShapeMismatch {
        expected: Vec<usize>,
        actual: Vec<usize>,
    },

    /// Tensor error
    #[error("Tensor error: {0}")]
    TensorError(String),
}

impl From<candle_core::Error> for CompressionError {
    fn from(err: candle_core::Error) -> Self {
        CompressionError::TensorError(err.to_string())
    }
}
