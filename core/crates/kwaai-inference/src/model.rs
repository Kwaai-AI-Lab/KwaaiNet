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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_handle_equality_and_id() {
        let h1 = ModelHandle::new(42);
        let h2 = ModelHandle::new(42);
        let h3 = ModelHandle::new(99);
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_eq!(h1.id(), 42);
    }

    #[test]
    fn model_handle_hashable() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ModelHandle::new(1));
        set.insert(ModelHandle::new(2));
        set.insert(ModelHandle::new(1));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn format_from_extension_known() {
        assert_eq!(ModelFormat::from_extension("gguf"), Some(ModelFormat::Gguf));
        assert_eq!(
            ModelFormat::from_extension("safetensors"),
            Some(ModelFormat::SafeTensors)
        );
        assert_eq!(ModelFormat::from_extension("ggml"), Some(ModelFormat::Ggml));
        assert_eq!(ModelFormat::from_extension("bin"), Some(ModelFormat::Ggml));
        assert_eq!(
            ModelFormat::from_extension("pt"),
            Some(ModelFormat::PyTorch)
        );
        assert_eq!(
            ModelFormat::from_extension("pth"),
            Some(ModelFormat::PyTorch)
        );
    }

    #[test]
    fn format_from_extension_case_insensitive() {
        assert_eq!(
            ModelFormat::from_extension("GGUF"),
            Some(ModelFormat::Gguf)
        );
        assert_eq!(
            ModelFormat::from_extension("SafeTensors"),
            Some(ModelFormat::SafeTensors)
        );
    }

    #[test]
    fn format_from_extension_unknown_returns_none() {
        assert_eq!(ModelFormat::from_extension("pkl"), None);
        assert_eq!(ModelFormat::from_extension(""), None);
        assert_eq!(ModelFormat::from_extension("json"), None);
    }

    #[test]
    fn model_info_default_fields() {
        let info = ModelInfo::default();
        assert_eq!(info.architecture, "unknown");
        assert_eq!(info.format, ModelFormat::Gguf);
        assert!(!info.is_quantized);
        assert_eq!(info.num_parameters, 0);
        assert_eq!(info.hidden_dim, 0);
    }
}
