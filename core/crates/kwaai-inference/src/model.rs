//! Model management

use serde::{Deserialize, Serialize};
use tracing::warn;

/// Handle to a loaded model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModelHandle(pub(crate) u64);

impl ModelHandle {
    /// Create a new model handle
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the internal ID
    pub fn id(&self) -> u64 {
        self.0
    }
}

/// Model format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelFormat {
    /// GGUF format (llama.cpp compatible)
    Gguf,
    /// SafeTensors format
    SafeTensors,
    /// GGML format (legacy)
    Ggml,
    /// PyTorch format
    PyTorch,
}

impl ModelFormat {
    /// Detect format from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        let format = match ext.to_lowercase().as_str() {
            "gguf" => Some(Self::Gguf),
            "safetensors" => Some(Self::SafeTensors),
            "ggml" | "bin" => Some(Self::Ggml),
            "pt" | "pth" => Some(Self::PyTorch),
            _ => None,
        };
        if format.is_none() {
            warn!("Unrecognized model file extension: {}", ext);
        }
        format
    }
}

/// Information about a loaded model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model identifier
    pub id: String,

    /// Model name
    pub name: String,

    /// Model architecture
    pub architecture: String,

    /// Model format
    pub format: ModelFormat,

    /// Number of parameters
    pub num_parameters: u64,

    /// Memory usage in bytes
    pub memory_bytes: usize,

    /// Is quantized
    pub is_quantized: bool,

    /// Quantization type (if quantized)
    pub quantization: Option<String>,

    /// Vocabulary size
    pub vocab_size: usize,

    /// Context length
    pub context_length: usize,

    /// Hidden dimension (embedding size per token).
    /// Used by the Petals throughput formula: network_rps = bandwidth / (hidden_dim × 16 bits).
    pub hidden_dim: usize,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            architecture: "unknown".to_string(),
            format: ModelFormat::Gguf,
            num_parameters: 0,
            memory_bytes: 0,
            is_quantized: false,
            quantization: None,
            vocab_size: 0,
            context_length: 0,
            hidden_dim: 0,
        }
    }
}
