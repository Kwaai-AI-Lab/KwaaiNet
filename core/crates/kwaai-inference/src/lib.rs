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
//! use std::path::Path;
//! use kwaai_inference::{EngineConfig, InferenceEngine, InferenceProvider, ModelFormat};
//!
//! fn main() -> anyhow::Result<()> {
//!     let config = EngineConfig::default();
//!     let mut engine = InferenceEngine::new(config)?;
//!
//!     // Load a GGUF model (real weights, no stub)
//!     let handle = engine.load_model(Path::new("model.gguf"), ModelFormat::Gguf)?;
//!
//!     // generate() returns Err until the tokenizer is wired up (next step)
//!     let info = engine.model_info(&handle)?;
//!     println!("Loaded: {} ({} vocab, {} ctx)", info.name, info.vocab_size, info.context_length);
//!
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod engine;
pub mod error;
pub mod loader;
pub mod model;
pub mod shard;
pub mod tokenizer;

#[cfg(feature = "mlx")]
pub mod mlx_shard;

pub use config::EngineConfig;
pub use engine::InferenceEngine;
pub use error::{InferenceError, InferenceResult};
pub use model::{ModelFormat, ModelHandle, ModelInfo};
pub use shard::{ShardConfig, TransformerShard};

use async_trait::async_trait;
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    /// CPU inference
    #[default]
    Cpu,
    /// CUDA GPU inference
    Cuda(usize),
    /// Metal GPU inference (Apple Silicon) — candle backend, slow for decode
    Metal(usize),
    /// Apple MLX inference — unified memory, fast prefill + decode
    Mlx,
}

impl DeviceType {
    /// Detect the best available device.
    ///
    /// Priority: CUDA > MLX > CPU.
    /// Metal is intentionally excluded — candle's Metal backend is 10x slower
    /// than CPU for single-token decode. Use `DeviceType::Metal(0)` directly
    /// if you need to force it.
    pub fn detect_best() -> Self {
        #[cfg(feature = "cuda")]
        if candle_core::utils::cuda_is_available() {
            tracing::info!("CUDA device detected");
            return Self::Cuda(0);
        }

        #[cfg(feature = "mlx")]
        if crate::mlx_shard::mlx_available() {
            tracing::info!("MLX device detected");
            return Self::Mlx;
        }

        // Metal is skipped: candle's Metal backend has ~3% GPU utilization for
        // single-token decode due to per-op kernel launch overhead.
        #[cfg(feature = "metal")]
        if candle_core::utils::metal_is_available() {
            tracing::warn!(
                "Metal GPU available but skipped (decode is 10x slower than CPU). \
                 Use --use-gpu to force Metal."
            );
        }

        tracing::info!("Using CPU for inference");
        Self::Cpu
    }

    /// Require a GPU device, erroring if none is available.
    ///
    /// Use this when the user explicitly asked for GPU (`--gpu`, `--use-gpu`).
    /// Unlike `detect_best()`, this never silently falls back to CPU.
    pub fn require_gpu() -> InferenceResult<Self> {
        #[cfg(feature = "cuda")]
        {
            // cuda_is_available() is cfg!(feature = "cuda") — always true here.
            // The real runtime check happens in to_candle_device() which calls
            // cuInit(). But we know the binary was compiled with CUDA support.
            tracing::info!("CUDA feature compiled in — selecting CUDA device");
            return Ok(Self::Cuda(0));
        }

        #[cfg(feature = "mlx")]
        if crate::mlx_shard::mlx_available() {
            tracing::info!("MLX device detected");
            return Ok(Self::Mlx);
        }

        // Build a diagnostic message
        #[cfg(not(feature = "cuda"))]
        {
            let has_nvidia = std::process::Command::new("nvidia-smi")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .is_ok();

            let reinstall = if cfg!(target_os = "windows") {
                "irm https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1 | iex"
            } else {
                "curl -fsSL https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | bash"
            };
            let msg = if has_nvidia {
                format!(
                    "NVIDIA GPU detected but this binary was compiled without CUDA support.\n  \
                     Reinstall with: {reinstall}"
                )
            } else {
                "No GPU found. This binary was compiled without CUDA support and no NVIDIA GPU was detected.".to_string()
            };
            return Err(InferenceError::DeviceNotAvailable(msg));
        }

        #[allow(unreachable_code)]
        Err(InferenceError::DeviceNotAvailable(
            "No GPU available".to_string(),
        ))
    }

    /// Detect the best device with a warning if GPU hardware is present but
    /// the binary lacks GPU support.
    ///
    /// Use this for auto-detect paths where the user didn't explicitly request
    /// GPU (config defaults, `shard run --local`, `shard serve`).
    pub fn detect_best_logged() -> Self {
        let device = Self::detect_best();
        #[cfg(not(feature = "cuda"))]
        if matches!(device, DeviceType::Cpu) {
            if let Ok(output) = std::process::Command::new("nvidia-smi")
                .args(["--query-gpu=name", "--format=csv,noheader"])
                .output()
            {
                if output.status.success() {
                    let gpu = String::from_utf8_lossy(&output.stdout);
                    let gpu = gpu.trim();
                    if !gpu.is_empty() {
                        let reinstall = if cfg!(target_os = "windows") {
                            "irm https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1 | iex"
                        } else {
                            "curl -fsSL https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | bash"
                        };
                        eprintln!(
                            "\n  ⚠ NVIDIA GPU detected ({gpu}) but this binary lacks CUDA support.\n    \
                             Reinstall with: {reinstall}\n"
                        );
                    }
                }
            }
        }
        device
    }

    /// Returns true if this device type uses a GPU.
    pub fn is_gpu(&self) -> bool {
        !matches!(self, DeviceType::Cpu)
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
            DeviceType::Mlx => {
                // MLX doesn't use candle devices — return CPU as a fallback
                // (callers should check for Mlx and use MlxTransformerShard directly)
                Ok(candle_core::Device::Cpu)
            }
        }
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Cpu => write!(f, "CPU"),
            DeviceType::Cuda(ord) => write!(f, "CUDA (GPU {ord})"),
            DeviceType::Metal(ord) => write!(f, "Metal (GPU {ord})"),
            DeviceType::Mlx => write!(f, "MLX (Apple Silicon)"),
        }
    }
}
