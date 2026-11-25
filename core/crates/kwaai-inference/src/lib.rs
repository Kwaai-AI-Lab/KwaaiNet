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

/// Re-exported so downstream crates can handle MLX arrays without taking a
/// direct mlx-rs dependency (and without it drifting from the version we link).
#[cfg(feature = "mlx")]
pub use mlx_rs;

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

/// Parse HuggingFace `config.json` text into candle's `LlamaConfig`.
///
/// Llama-family configs keep every field at the top level; Gemma-2/3/4 (and
/// friends) nest the whole text model under `text_config`. Flatten first so
/// one struct serves both. Without this, a Gemma shard dies with
/// `missing field hidden_size` — the exact error `hivv.org-gemma4` hit on
/// Linux (2026-11-23). Nested values win over top-level duplicates; top-level
/// keys only fill gaps (architectures/model_type are ignored by the struct).
/// A `config.json` flattened to one level, with Gemma's rotary base mapped
/// onto the field candle reads.
///
/// Llama-family configs keep every field at the top level; Gemma-2/3/4 (and
/// friends) nest the whole text model under `text_config`. Flatten first so
/// one reader serves both. Nested values win over top-level duplicates;
/// top-level keys only fill gaps (`architectures`/`model_type` are ignored by
/// the structs that consume this).
///
/// Every config field is read through here — `head_dim` was first read
/// straight from the top level, which is absent on a Gemma, so the loader fell
/// back to `hidden/heads` (192) while the weights were built for 256 and
/// `q_proj` refused to load.
pub fn hf_flat_config(config_str: &str) -> Result<serde_json::Value, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(config_str)?;
    let mut merged = match value.get("hidden_size") {
        Some(_) => value,
        None => match value.get("text_config").and_then(|t| t.as_object()) {
            Some(text) => {
                let mut flat: serde_json::Map<String, serde_json::Value> = text.clone();
                if let Some(top) = value.as_object() {
                    for (k, v) in top {
                        flat.entry(k.clone()).or_insert_with(|| v.clone());
                    }
                }
                serde_json::Value::Object(flat)
            }
            None => value,
        },
    };
    // Gemma names the rotary base rope_parameters.{theta,base}; candle's
    // LlamaConfig reads rope_theta (default 10000 — wrong for Gemma, whose
    // real base is 1000000). Copy it across when present so long-context
    // rotation stays honest.
    if merged.get("rope_theta").is_none() {
        let theta = merged
            .get("rope_parameters")
            .and_then(|rp| rp.get("theta").or_else(|| rp.get("base")))
            .cloned();
        if let Some(t) = theta {
            if let Some(obj) = merged.as_object_mut() {
                obj.insert("rope_theta".to_string(), t);
            }
        }
    }
    Ok(merged)
}

pub fn hf_config_from_json(
    config_str: &str,
) -> Result<candle_transformers::models::llama::LlamaConfig, serde_json::Error> {
    serde_json::from_value(hf_flat_config(config_str)?)
}

/// The prefix a model's transformer weights live under.
///
/// Llama-family configs keep weights at `model.*`. Gemma-2/3/4 (and other
/// multimodal wrappers) nest the whole text model under
/// `model.language_model.*` — the same nesting [`hf_config_from_json`]
/// flattens on the config side. Without this a Gemma shard dies with
/// `embedding: cannot find tensor model.embed_tokens.weight`, the error
/// `hivv.org-gemma4` hit on Linux (2026-11-23): the config parsed (that is
/// what the previous fix did) but the tensors were still one level down.
///
/// Detection mirrors [`hf_config_from_json`]: a top-level `hidden_size`
/// means flat, a `text_config` means nested. Anything unparseable falls back
/// to the flat prefix, which is what every Llama-family model needs.
pub fn hf_tensor_prefix(config_str: &str) -> &'static str {
    match serde_json::from_str::<serde_json::Value>(config_str) {
        Ok(v) if v.get("hidden_size").is_some() => "model.",
        Ok(v) if v.get("text_config").is_some() => "model.language_model.",
        _ => "model.",
    }
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
                "curl -fsSL https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh"
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
                            "curl -fsSL https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh"
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
                jetson_default_sync_alloc();
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

/// Env var the patched cudarc honours (core/patches/README.md): any value
/// makes CUDA allocation synchronous instead of stream-ordered.
pub const CUDARC_DISABLE_ASYNC_ALLOC: &str = "CUDARC_DISABLE_ASYNC_ALLOC";

/// L4T's stream-ordered allocator caps at ~1/3 of memory, so a Jetson with the
/// variable unset gets it set before the first CUDA call. Fires only on Linux
/// aarch64 with the Tegra release file present.
#[cfg(feature = "cuda")]
fn jetson_default_sync_alloc() {
    if jetson_needs_sync_alloc(
        cfg!(all(target_os = "linux", target_arch = "aarch64")),
        std::path::Path::new("/etc/nv_tegra_release"),
        std::env::var_os(CUDARC_DISABLE_ASYNC_ALLOC).is_some(),
    ) {
        std::env::set_var(CUDARC_DISABLE_ASYNC_ALLOC, "1");
        tracing::info!(
            "Jetson detected: set {CUDARC_DISABLE_ASYNC_ALLOC}=1 (L4T's async allocator caps at ~1/3 of memory)"
        );
    }
}

/// The decision behind `jetson_default_sync_alloc`, kept pure so it can be
/// tested without CUDA or a Jetson.
#[cfg(any(feature = "cuda", test))]
fn jetson_needs_sync_alloc(
    linux_aarch64: bool,
    tegra_release: &std::path::Path,
    already_set: bool,
) -> bool {
    linux_aarch64 && !already_set && tegra_release.exists()
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

#[cfg(test)]
mod jetson_tests {
    use super::jetson_needs_sync_alloc;
    use std::path::Path;

    #[test]
    fn sets_only_on_tegra_linux_aarch64_when_unset() {
        let dir = std::env::temp_dir().join(format!("kwaai-tegra-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let tegra = dir.join("nv_tegra_release");
        std::fs::write(&tegra, "# R36 (release)\n").unwrap();
        let absent = dir.join("missing");

        assert!(jetson_needs_sync_alloc(true, &tegra, false));
        assert!(
            !jetson_needs_sync_alloc(true, &tegra, true),
            "operator's value wins"
        );
        assert!(
            !jetson_needs_sync_alloc(true, &absent, false),
            "not a Jetson"
        );
        assert!(
            !jetson_needs_sync_alloc(false, &tegra, false),
            "wrong platform"
        );
        assert!(!jetson_needs_sync_alloc(
            false,
            Path::new("/etc/nv_tegra_release"),
            false
        ));

        std::fs::remove_dir_all(&dir).ok();
    }
}
