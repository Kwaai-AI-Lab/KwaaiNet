//! # kwaai-inference
//!
//! Inference engine for KwaaiNet using Candle ML framework.
//!
//! This crate provides the core ML inference capabilities including:
//!
//! - **Model Loading**: Support for GGUF, SafeTensors formats
//! - **Inference**: Text generation, embeddings, and more
//! - **Resource Management**: Memory-aware model loading
//!
//! ## Example
//!
//! ```rust,no_run
//! use kwaai_inference::{InferenceEngine, EngineConfig, ModelFormat};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = EngineConfig::default();
//!     let mut engine = InferenceEngine::new(config)?;
//!
//!     // Load a model
//!     let handle = engine.load_model("model.gguf", ModelFormat::Gguf)?;
//!
//!     // Run inference
//!     let output = engine.generate(&handle, "Hello, world!")?;
//!     println!("{}", output);
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod engine;
pub mod error;
pub mod model;
pub mod tokenizer;

pub use config::EngineConfig;
pub use engine::InferenceEngine;
pub use error::{InferenceError, InferenceResult};
pub use model::{ModelFormat, ModelHandle, ModelInfo};

use async_trait::async_trait;
use candle_core::Tensor;
use std::path::Path;

/// Core trait for inference operations
///
/// Implementors provide the ability to load models and run inference.
#[async_trait]
pub trait InferenceProvider: Send + Sync {
    /// Load a model from the given path
    fn load_model(&mut self, path: &Path, format: ModelFormat) -> InferenceResult<ModelHandle>;

    /// Run inference on the given input tensor
    fn forward(&self, handle: &ModelHandle, input: &Tensor) -> InferenceResult<Tensor>;

    /// Generate text from a prompt
    fn generate(&self, handle: &ModelHandle, prompt: &str) -> InferenceResult<String>;

    /// Unload a model to free memory
    fn unload(&mut self, handle: ModelHandle) -> InferenceResult<()>;

    /// Get model information
    fn model_info(&self, handle: &ModelHandle) -> InferenceResult<ModelInfo>;
}

/// Trait for model loaders
///
/// Implementors can load models from various formats.
pub trait ModelLoader: Send + Sync {
    /// Load model from GGUF format
    fn load_gguf(&self, data: &[u8]) -> InferenceResult<LoadedModel>;

    /// Load model from SafeTensors format
    fn load_safetensors(&self, data: &[u8]) -> InferenceResult<LoadedModel>;
}

/// A loaded model ready for inference
pub struct LoadedModel {
    /// Model weights
    pub weights: Vec<Tensor>,
    /// Model configuration
    pub config: ModelConfig,
    /// Vocabulary size
    pub vocab_size: usize,
    /// Hidden size
    pub hidden_size: usize,
    /// Number of layers
    pub num_layers: usize,
}

/// Model configuration
#[derive(Debug, Clone)]
pub struct ModelConfig {
    /// Model architecture type
    pub architecture: String,
    /// Maximum sequence length
    pub max_seq_len: usize,
    /// Number of attention heads
    pub num_heads: usize,
    /// Number of key-value heads (for GQA)
    pub num_kv_heads: usize,
    /// Hidden dimension
    pub hidden_dim: usize,
    /// Intermediate dimension (for FFN)
    pub intermediate_dim: usize,
    /// RoPE theta (for positional encoding)
    pub rope_theta: f32,
    /// Layer norm epsilon
    pub layer_norm_eps: f32,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            architecture: "llama".to_string(),
            max_seq_len: 4096,
            num_heads: 32,
            num_kv_heads: 8,
            hidden_dim: 4096,
            intermediate_dim: 11008,
            rope_theta: 10000.0,
            layer_norm_eps: 1e-5,
        }
    }
}

/// Device type for inference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// CPU inference
    Cpu,
    /// CUDA GPU inference
    Cuda(usize),
    /// Metal GPU inference (Apple Silicon)
    Metal(usize),
}

impl Default for DeviceType {
    fn default() -> Self {
        Self::Cpu
    }
}

impl DeviceType {
    /// Detect the best available device
    pub fn detect_best() -> Self {
        #[cfg(feature = "cuda")]
        if candle_core::utils::cuda_is_available() {
            return Self::Cuda(0);
        }

        #[cfg(feature = "metal")]
        if candle_core::utils::metal_is_available() {
            return Self::Metal(0);
        }

        Self::Cpu
    }

    /// Convert to Candle device
    pub fn to_candle_device(&self) -> InferenceResult<candle_core::Device> {
        match self {
            DeviceType::Cpu => Ok(candle_core::Device::Cpu),
            #[cfg(feature = "cuda")]
            DeviceType::Cuda(ordinal) => {
                candle_core::Device::new_cuda(*ordinal).map_err(InferenceError::from)
            }
            #[cfg(feature = "metal")]
            DeviceType::Metal(ordinal) => {
                candle_core::Device::new_metal(*ordinal).map_err(InferenceError::from)
            }
            #[cfg(not(feature = "cuda"))]
            DeviceType::Cuda(_) => Err(InferenceError::DeviceNotAvailable("CUDA".to_string())),
            #[cfg(not(feature = "metal"))]
            DeviceType::Metal(_) => Err(InferenceError::DeviceNotAvailable("Metal".to_string())),
        }
    }
}
