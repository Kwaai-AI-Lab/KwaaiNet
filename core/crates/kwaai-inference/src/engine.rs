//! Inference engine — real model loading via `candle_transformers`.

use crate::{
    config::EngineConfig,
    error::{InferenceError, InferenceResult},
    loader::{self, GgufModel, SafeTensorsModel},
    model::{ModelFormat, ModelHandle, ModelInfo},
    InferenceProvider, ModelConfig,
};
use async_trait::async_trait;
use candle_core::{Device, Tensor};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tracing::{debug, info};

// ── Loaded weights ────────────────────────────────────────────────────────────

/// Real model weights stored in a loaded entry.
///
/// `Mutex` gives us interior mutability so `InferenceEngine` can remain `Sync`
/// even though the forward-pass methods require `&mut` access to KV-cache state.
///
/// Fields are read in the upcoming forward-pass / generation step.
#[allow(dead_code)]
enum LoadedWeights {
    /// Quantized model from a GGUF file (Q4_K_M, Q5_K_M, …)
    Gguf(Mutex<GgufModel>),
    /// Full-precision model from SafeTensors shards (F16 / F32)
    SafeTensors(Mutex<SafeTensorsModel>),
}

/// Fields `weights` and `config` are used in the upcoming forward-pass step.
#[allow(dead_code)]
struct LoadedModelEntry {
    info: ModelInfo,
    weights: LoadedWeights,
    /// Architecture config kept for callers that need it without locking weights.
    config: ModelConfig,
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct InferenceEngine {
    config: EngineConfig,
    device: Device,
    models: HashMap<u64, LoadedModelEntry>,
    next_model_id: AtomicU64,
    current_memory: usize,
}

impl InferenceEngine {
    pub fn new(config: EngineConfig) -> InferenceResult<Self> {
        let device = config.device.to_candle_device()?;
        info!("Inference engine initialised on {:?}", config.device);
        Ok(Self {
            config,
            device,
            models: HashMap::new(),
            next_model_id: AtomicU64::new(1),
            current_memory: 0,
        })
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn memory_usage(&self) -> usize {
        self.current_memory
    }

    pub fn loaded_model_count(&self) -> usize {
        self.models.len()
    }

    pub fn list_models(&self) -> Vec<ModelInfo> {
        self.models.values().map(|e| e.info.clone()).collect()
    }

    fn check_memory(&self, required: usize) -> InferenceResult<()> {
        let available = self.config.max_memory.saturating_sub(self.current_memory);
        if required > available {
            return Err(InferenceError::OutOfMemory { required, available });
        }
        Ok(())
    }

    fn next_id(&self) -> u64 {
        self.next_model_id.fetch_add(1, Ordering::Relaxed)
    }
}

// ── InferenceProvider impl ────────────────────────────────────────────────────

#[async_trait]
impl InferenceProvider for InferenceEngine {
    fn load_model(&mut self, path: &Path, format: ModelFormat) -> InferenceResult<ModelHandle> {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        info!("Loading model: {} ({:?})", file_name, format);

        // Reject unsupported formats before touching the filesystem.
        if format == ModelFormat::PyTorch {
            return Err(InferenceError::InvalidFormat(
                "PyTorch .bin/.pt format is not supported. \
                 Convert the model to SafeTensors or GGUF first."
                    .to_string(),
            ));
        }

        if !path.exists() {
            return Err(InferenceError::ModelNotFound(path.display().to_string()));
        }

        // For directories (sharded SafeTensors), sum all shard sizes.
        // For single files, just use the file size.
        // Use std::fs::metadata (not DirEntry::metadata) to follow symlinks.
        let file_size: usize = if path.is_dir() {
            std::fs::read_dir(path)
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .map(|e| e.path())
                        .filter(|p| {
                            p.extension().and_then(|x| x.to_str()) == Some("safetensors")
                        })
                        .filter_map(|p| std::fs::metadata(&p).ok())
                        .map(|m| m.len())
                        .sum::<u64>() as usize
                })
                .unwrap_or(0)
        } else {
            std::fs::metadata(path)
                .map_err(InferenceError::from)?
                .len() as usize
        };

        // Memory estimate: both GGUF and SafeTensors are memory-mapped, so the
        // working-set size is approximately the file size plus a small overhead.
        let estimated_memory = (file_size as f64 * 1.1) as usize;

        self.check_memory(estimated_memory)?;

        // ── Dispatch to the real loader ──────────────────────────────────────
        let (weights, config, vocab_size, _num_layers, is_quantized) = match format {
            ModelFormat::Gguf | ModelFormat::Ggml => {
                let m = loader::load_gguf(path, &self.device)?;
                let c = m.config.clone();
                let v = m.vocab_size;
                let l = m.num_layers;
                (LoadedWeights::Gguf(Mutex::new(m)), c, v, l, true)
            }

            ModelFormat::SafeTensors => {
                if path.is_dir() {
                    // Sharded model directory (e.g. a HuggingFace snapshot).
                    // Collect all .safetensors shard files, sorted by name.
                    let mut shard_paths: Vec<std::path::PathBuf> =
                        std::fs::read_dir(path)
                            .map_err(InferenceError::from)?
                            .filter_map(|e| e.ok())
                            .map(|e| e.path())
                            .filter(|p| {
                                p.extension().and_then(|x| x.to_str())
                                    == Some("safetensors")
                            })
                            .collect();
                    shard_paths.sort();

                    if shard_paths.is_empty() {
                        return Err(InferenceError::ModelNotFound(format!(
                            "No .safetensors shards found in {}",
                            path.display()
                        )));
                    }

                    let config_path = path.join("config.json");
                    let path_refs: Vec<&Path> =
                        shard_paths.iter().map(|p| p.as_path()).collect();
                    let m = loader::load_safetensors(&path_refs, &config_path, &self.device)?;
                    let c = m.config.clone();
                    let v = m.vocab_size;
                    let l = m.num_layers;
                    (LoadedWeights::SafeTensors(Mutex::new(m)), c, v, l, false)
                } else {
                    // Single-shard: config.json must sit alongside the .safetensors file.
                    let config_path = path
                        .parent()
                        .unwrap_or(Path::new("."))
                        .join("config.json");
                    let path_slice = [path];
                    let m = loader::load_safetensors(&path_slice, &config_path, &self.device)?;
                    let c = m.config.clone();
                    let v = m.vocab_size;
                    let l = m.num_layers;
                    (LoadedWeights::SafeTensors(Mutex::new(m)), c, v, l, false)
                }
            }

            ModelFormat::PyTorch => {
                // Already rejected above; unreachable but keeps the match exhaustive.
                unreachable!("PyTorch format rejected before this point")
            }
        };

        let id = self.next_id();
        let info = ModelInfo {
            id: id.to_string(),
            name: file_name,
            architecture: config.architecture.clone(),
            format,
            memory_bytes: estimated_memory,
            vocab_size,
            context_length: config.max_seq_len,
            is_quantized,
            ..Default::default()
        };

        self.models.insert(id, LoadedModelEntry { info, weights, config });
        self.current_memory += estimated_memory;

        info!("Model loaded — handle {id}, ~{:.1} GB", estimated_memory as f64 / 1e9);
        Ok(ModelHandle::new(id))
    }

    fn forward(&self, handle: &ModelHandle, _input: &Tensor) -> InferenceResult<Tensor> {
        let _entry = self
            .models
            .get(&handle.id())
            .ok_or(InferenceError::InvalidHandle(handle.id()))?;

        // TODO (next step): implement the autoregressive forward pass.
        // Requires:
        //   • a real tokenizer so callers can pass token-ID tensors
        //   • a per-session KV cache (Mutex<Vec<(Tensor, Tensor)>> for GGUF,
        //     candle_transformers::models::llama::Cache for full-precision)
        //   • routing based on LoadedWeights variant
        // See CONTRIBUTORS.md — "Forward pass & generation".
        debug!("forward() called on handle {} — wiring pending", handle.id());
        Err(InferenceError::InferenceFailed(
            "forward() is not yet wired. \
             Implement in next step together with tokenizer and KV-cache."
                .to_string(),
        ))
    }

    fn generate(&self, handle: &ModelHandle, _prompt: &str) -> InferenceResult<String> {
        let _entry = self
            .models
            .get(&handle.id())
            .ok_or(InferenceError::InvalidHandle(handle.id()))?;

        // TODO (next step): implement text generation.
        // Requires:
        //   • real BPE tokenizer (encode prompt → token IDs, decode IDs → text)
        //   • autoregressive loop calling forward() with KV-cache
        //   • sampling (temperature / top-p / top-k)
        // See CONTRIBUTORS.md — "Forward pass & generation".
        debug!("generate() called on handle {} — tokenizer not yet wired", handle.id());
        Err(InferenceError::InferenceFailed(
            "generate() requires a tokenizer. \
             See CONTRIBUTORS.md: 'Replace byte-level placeholder tokenizer'."
                .to_string(),
        ))
    }

    fn unload(&mut self, handle: ModelHandle) -> InferenceResult<()> {
        let entry = self
            .models
            .remove(&handle.id())
            .ok_or(InferenceError::InvalidHandle(handle.id()))?;
        self.current_memory = self.current_memory.saturating_sub(entry.info.memory_bytes);
        info!("Unloaded model {}", handle.id());
        Ok(())
    }

    fn model_info(&self, handle: &ModelHandle) -> InferenceResult<ModelInfo> {
        let entry = self
            .models
            .get(&handle.id())
            .ok_or(InferenceError::InvalidHandle(handle.id()))?;
        Ok(entry.info.clone())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = EngineConfig::default();
        let engine = InferenceEngine::new(config).unwrap();
        assert_eq!(engine.loaded_model_count(), 0);
        assert_eq!(engine.memory_usage(), 0);
    }

    #[test]
    fn test_missing_model_returns_not_found() {
        let config = EngineConfig::default();
        let mut engine = InferenceEngine::new(config).unwrap();
        let result = engine.load_model(Path::new("/nonexistent/model.gguf"), ModelFormat::Gguf);
        assert!(matches!(result, Err(InferenceError::ModelNotFound(_))));
    }

    #[test]
    fn test_pytorch_format_rejected() {
        let config = EngineConfig::default();
        let mut engine = InferenceEngine::new(config).unwrap();
        // PyTorch format should be rejected without even checking if the file exists.
        // We create a temp file just to get past the existence check path, but
        // the format check fires first.
        let result =
            engine.load_model(Path::new("/tmp/model.pt"), ModelFormat::PyTorch);
        assert!(matches!(result, Err(InferenceError::InvalidFormat(_))));
    }
}
