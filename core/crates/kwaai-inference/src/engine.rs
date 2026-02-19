//! Inference engine — real model loading via `candle_transformers`.

use crate::{
    config::EngineConfig,
    error::{InferenceError, InferenceResult},
    loader::{self, GgufModel, SafeTensorsModel},
    model::{ModelFormat, ModelHandle, ModelInfo},
    tokenizer::Tokenizer,
    InferenceProvider, ModelConfig,
};
use async_trait::async_trait;
use candle_core::{DType, Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::llama::Cache;
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

struct LoadedModelEntry {
    info: ModelInfo,
    weights: LoadedWeights,
    /// Architecture config kept for callers that need it without locking weights.
    #[allow(dead_code)]
    config: ModelConfig,
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct InferenceEngine {
    config: EngineConfig,
    device: Device,
    models: HashMap<u64, LoadedModelEntry>,
    next_model_id: AtomicU64,
    current_memory: usize,
    /// Decode throughput in tok/s from the most recent `generate()` call.
    /// Stored as the raw bits of an `f64` so it can live in an `AtomicU64`.
    last_decode_tps: AtomicU64,
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
            last_decode_tps: AtomicU64::new(0),
        })
    }

    /// Tokens/second measured during the decode phase of the last `generate()` call.
    /// Returns `0.0` if no generation has been run yet.
    pub fn last_throughput_tps(&self) -> f64 {
        let bits = self.last_decode_tps.load(Ordering::Relaxed);
        if bits == 0 { 0.0 } else { f64::from_bits(bits) }
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
            hidden_dim: config.hidden_dim,
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

    fn generate(&self, handle: &ModelHandle, prompt: &str) -> InferenceResult<String> {
        /// Maximum new tokens to generate per call.
        const MAX_NEW_TOKENS: usize = 256;
        /// Sampling temperature (0 → greedy, higher → more random).
        const TEMPERATURE: f64 = 0.8;

        let entry = self
            .models
            .get(&handle.id())
            .ok_or(InferenceError::InvalidHandle(handle.id()))?;

        let mut logits_processor = LogitsProcessor::new(42, Some(TEMPERATURE), None);

        let text = match &entry.weights {
            // ── Quantized GGUF path ───────────────────────────────────────────
            LoadedWeights::Gguf(m) => {
                let mut guard = m.lock().unwrap();

                // Encode prompt.
                let mut prompt_tokens = guard.tokenizer.encode(prompt)?;
                let eos_id = guard.tokenizer.eos_token_id();
                let bos_id = guard.tokenizer.bos_token_id();

                // Prepend BOS only when it is a distinct token from EOS.
                // Models like Qwen2 set BOS == EOS (both are <|endoftext|>=151643);
                // prepending EOS as BOS causes the model to immediately terminate.
                if let Some(bos) = bos_id {
                    if Some(bos) != eos_id {
                        prompt_tokens.insert(0, bos);
                    }
                }
                let prompt_len = prompt_tokens.len();

                // Build the full stop-token set: the registered EOS plus common
                // ChatML/instruct stop tokens that the vocab may contain.
                let mut stop_ids: Vec<u32> = eos_id.into_iter().collect();
                for candidate in &["<|im_end|>", "<|eot_id|>", "<|end_of_text|>"] {
                    if let Some(id) = guard.tokenizer.token_to_id(candidate) {
                        if !stop_ids.contains(&id) {
                            stop_ids.push(id);
                        }
                    }
                }

                info!(
                    "generate() GGUF handle {}: {} prompt tokens, stop={:?}",
                    handle.id(),
                    prompt_len,
                    stop_ids,
                );

                // Prefill: process the entire prompt in one forward pass.
                // index_pos=0 resets the model's internal KV-cache.
                let prompt_tensor =
                    Tensor::new(prompt_tokens.as_slice(), &self.device)
                        .map_err(InferenceError::from)?
                        .unsqueeze(0)
                        .map_err(InferenceError::from)?; // [1, prompt_len]

                let logits = guard
                    .weights
                    .forward(&prompt_tensor, 0)
                    .map_err(InferenceError::from)?; // [1, vocab_size]
                let logits = logits.squeeze(0).map_err(InferenceError::from)?; // [vocab_size]

                let mut next_token =
                    logits_processor.sample(&logits).map_err(InferenceError::from)?;

                let mut generated: Vec<u32> = Vec::new();
                let mut pos = prompt_len;

                // Decode loop: feed one token at a time, sample the next.
                let decode_start = std::time::Instant::now();
                loop {
                    if stop_ids.contains(&next_token) || generated.len() >= MAX_NEW_TOKENS {
                        break;
                    }
                    generated.push(next_token);

                    let token_tensor = Tensor::new(&[next_token], &self.device)
                        .map_err(InferenceError::from)?
                        .unsqueeze(0)
                        .map_err(InferenceError::from)?; // [1, 1]

                    let logits = guard
                        .weights
                        .forward(&token_tensor, pos)
                        .map_err(InferenceError::from)?;
                    let logits = logits.squeeze(0).map_err(InferenceError::from)?;

                    next_token =
                        logits_processor.sample(&logits).map_err(InferenceError::from)?;
                    pos += 1;
                }
                let decode_secs = decode_start.elapsed().as_secs_f64();

                if !generated.is_empty() && decode_secs > 0.0 {
                    let tps = generated.len() as f64 / decode_secs;
                    self.last_decode_tps.store(tps.to_bits(), Ordering::Relaxed);
                }

                debug!(
                    "generate() GGUF handle {}: {} tokens in {:.2}s ({:.1} tok/s)",
                    handle.id(),
                    generated.len(),
                    decode_secs,
                    if decode_secs > 0.0 { generated.len() as f64 / decode_secs } else { 0.0 },
                );

                guard.tokenizer.decode(&generated)?
            }

            // ── Full-precision SafeTensors path ───────────────────────────────
            LoadedWeights::SafeTensors(m) => {
                let guard = m.lock().unwrap();

                // Encode prompt.
                let mut prompt_tokens = guard.tokenizer.encode(prompt)?;
                let eos_id = guard.tokenizer.eos_token_id();
                let bos_id = guard.tokenizer.bos_token_id();

                // Only add BOS when it differs from EOS (same guard as GGUF path).
                if let Some(bos) = bos_id {
                    if Some(bos) != eos_id {
                        prompt_tokens.insert(0, bos);
                    }
                }
                let prompt_len = prompt_tokens.len();

                // Build the full stop-token set.
                let mut stop_ids: Vec<u32> = eos_id.into_iter().collect();
                for candidate in &["<|im_end|>", "<|eot_id|>", "<|end_of_text|>"] {
                    if let Some(id) = guard.tokenizer.token_to_id(candidate) {
                        if !stop_ids.contains(&id) {
                            stop_ids.push(id);
                        }
                    }
                }

                info!(
                    "generate() SafeTensors handle {}: {} prompt tokens, stop={:?}",
                    handle.id(),
                    prompt_len,
                    stop_ids,
                );

                // Create a fresh KV-cache for this generation session.
                let mut cache = Cache::new(true, DType::F16, &guard.llama_config, &self.device)
                    .map_err(InferenceError::from)?;

                // Prefill.
                let prompt_tensor =
                    Tensor::new(prompt_tokens.as_slice(), &self.device)
                        .map_err(InferenceError::from)?
                        .unsqueeze(0)
                        .map_err(InferenceError::from)?; // [1, prompt_len]

                let logits = guard
                    .model
                    .forward(&prompt_tensor, 0, &mut cache)
                    .map_err(InferenceError::from)?; // [1, vocab_size]
                let logits = logits.squeeze(0).map_err(InferenceError::from)?; // [vocab_size]

                let mut next_token =
                    logits_processor.sample(&logits).map_err(InferenceError::from)?;

                let mut generated: Vec<u32> = Vec::new();
                let mut pos = prompt_len;

                // Decode loop.
                let decode_start = std::time::Instant::now();
                loop {
                    if stop_ids.contains(&next_token) || generated.len() >= MAX_NEW_TOKENS {
                        break;
                    }
                    generated.push(next_token);

                    let token_tensor = Tensor::new(&[next_token], &self.device)
                        .map_err(InferenceError::from)?
                        .unsqueeze(0)
                        .map_err(InferenceError::from)?; // [1, 1]

                    let logits = guard
                        .model
                        .forward(&token_tensor, pos, &mut cache)
                        .map_err(InferenceError::from)?;
                    let logits = logits.squeeze(0).map_err(InferenceError::from)?;

                    next_token =
                        logits_processor.sample(&logits).map_err(InferenceError::from)?;
                    pos += 1;
                }
                let decode_secs = decode_start.elapsed().as_secs_f64();

                if !generated.is_empty() && decode_secs > 0.0 {
                    let tps = generated.len() as f64 / decode_secs;
                    self.last_decode_tps.store(tps.to_bits(), Ordering::Relaxed);
                }

                debug!(
                    "generate() SafeTensors handle {}: {} tokens in {:.2}s ({:.1} tok/s)",
                    handle.id(),
                    generated.len(),
                    decode_secs,
                    if decode_secs > 0.0 { generated.len() as f64 / decode_secs } else { 0.0 },
                );

                guard.tokenizer.decode(&generated)?
            }
        };

        Ok(text)
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
