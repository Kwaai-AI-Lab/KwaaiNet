//! Error types for the inference engine

use thiserror::Error;

/// Result type for inference operations
pub type InferenceResult<T> = Result<T, InferenceError>;

/// Errors that can occur during inference
#[derive(Error, Debug)]
pub enum InferenceError {
    /// Model loading failed
    #[error("Failed to load model: {0}")]
    ModelLoadError(String),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Invalid model format
    #[error("Invalid model format: {0}")]
    InvalidFormat(String),

    /// Inference failed
    #[error("Inference failed: {0}")]
    InferenceFailed(String),

    /// Tensor operation failed
    #[error("Tensor operation failed: {0}")]
    TensorError(String),

    /// Tokenization failed
    #[error("Tokenization failed: {0}")]
    TokenizationError(String),

    /// Out of memory
    #[error("Out of memory: required {required} bytes, available {available} bytes")]
    OutOfMemory { required: usize, available: usize },

    /// Device not available
    #[error("Device not available: {0}")]
    DeviceNotAvailable(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Model handle invalid
    #[error("Invalid model handle: {0}")]
    InvalidHandle(u64),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<candle_core::Error> for InferenceError {
    fn from(err: candle_core::Error) -> Self {
        InferenceError::TensorError(err.to_string())
    }
}
