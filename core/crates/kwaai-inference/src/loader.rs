//! Model loading from GGUF and SafeTensors formats.
//!
//! This module replaces the previous stub that stored `Vec::new()` for weights.
//! It performs real disk I/O and populates actual model weight structs via
//! `candle_transformers`.

use crate::{
    error::{InferenceError, InferenceResult},
    tokenizer::BpeTokenizer,
    ModelConfig,
};
use candle_core::{quantized::gguf_file, Device, Tensor};
use std::path::Path;
use tracing::info;

// ── GGUF ─────────────────────────────────────────────────────────────────────

/// Quantized weights — one variant per supported GGUF architecture.
pub enum GgufWeights {
    Llama(candle_transformers::models::quantized_llama::ModelWeights),
    Qwen2(candle_transformers::models::quantized_qwen2::ModelWeights),
}

impl GgufWeights {
    /// Dispatch the forward pass to the underlying architecture.
    ///
    /// `x`         — token-ID tensor of shape `[1, seq_len]`
    /// `index_pos` — position of the first token in `x` within the full sequence;
    ///               pass `0` to reset (or cold-start) the internal KV-cache.
    pub fn forward(&mut self, x: &Tensor, index_pos: usize) -> candle_core::Result<Tensor> {
        match self {
            GgufWeights::Llama(w) => w.forward(x, index_pos),
            GgufWeights::Qwen2(w) => w.forward(x, index_pos),
        }
    }
}

/// Quantized model loaded from a GGUF file.
pub struct GgufModel {
    /// Quantized weights for the detected architecture.
    pub weights: GgufWeights,
    /// Architecture config extracted from GGUF metadata.
    pub config: ModelConfig,
    /// Vocabulary size.
    pub vocab_size: usize,
    /// Number of transformer layers.
    pub num_layers: usize,
    /// BPE tokenizer built from the GGUF vocabulary and merge rules.
    pub tokenizer: BpeTokenizer,
}

/// Load a GGUF model file into memory.
///
/// Reads `general.architecture` from the GGUF metadata to pick the right
/// weight loader, then extracts architecture config and loads real weights.
pub fn load_gguf(path: &Path, device: &Device) -> InferenceResult<GgufModel> {
    use candle_transformers::models::{quantized_llama, quantized_qwen2};

    let mut file = std::fs::File::open(path).map_err(|e| {
        InferenceError::ModelLoadError(format!("Cannot open {}: {e}", path.display()))
    })?;

    let gguf = gguf_file::Content::read(&mut file).map_err(|e| {
        InferenceError::ModelLoadError(format!("Cannot parse GGUF header in {}: {e}", path.display()))
    })?;

    // Detect architecture from the GGUF general metadata.
    let arch = meta_str(&gguf, "general.architecture")
        .unwrap_or_else(|| "llama".to_string());

    // Architecture-specific metadata key prefix.
    let pfx = arch.as_str();

    let vocab_size   = meta_usize(&gguf, "llama.vocab_size")
        .or_else(|| meta_usize(&gguf, &format!("{pfx}.vocab_size")))
        .unwrap_or(32_000);
    let num_layers   = meta_usize(&gguf, &format!("{pfx}.block_count")).unwrap_or(32);
    let num_heads    = meta_usize(&gguf, &format!("{pfx}.attention.head_count")).unwrap_or(32);
    let num_kv_heads = meta_usize(&gguf, &format!("{pfx}.attention.head_count_kv")).unwrap_or(num_heads);
    let hidden_dim   = meta_usize(&gguf, &format!("{pfx}.embedding_length")).unwrap_or(4_096);
    let inter_dim    = meta_usize(&gguf, &format!("{pfx}.feed_forward_length")).unwrap_or(11_008);
    let rope_theta   = meta_f32(&gguf, &format!("{pfx}.rope.freq_base")).unwrap_or(10_000.0);
    let max_seq_len  = meta_usize(&gguf, &format!("{pfx}.context_length")).unwrap_or(4_096);

    info!(
        "GGUF arch={arch}: {num_layers} layers, {num_heads} heads ({num_kv_heads} kv), \
         hidden={hidden_dim}, vocab={vocab_size}"
    );

    // Build the BPE tokenizer from GGUF metadata BEFORE consuming `gguf`
    // in the weight loader below (which moves it by value).
    let tokenizer = BpeTokenizer::from_gguf(&gguf)?;

    let config = ModelConfig {
        architecture: arch.clone(),
        max_seq_len,
        num_heads,
        num_kv_heads,
        hidden_dim,
        intermediate_dim: inter_dim,
        rope_theta,
        layer_norm_eps: 1e-5,
    };

    // Dispatch to the architecture-specific quantized weight loader.
    let weights = match arch.as_str() {
        "llama" | "mistral" | "llama3" => {
            let w = quantized_llama::ModelWeights::from_gguf(gguf, &mut file, device)
                .map_err(|e| InferenceError::ModelLoadError(
                    format!("Cannot build {arch} weights: {e}")
                ))?;
            GgufWeights::Llama(w)
        }
        "qwen2" => {
            let w = quantized_qwen2::ModelWeights::from_gguf(gguf, &mut file, device)
                .map_err(|e| InferenceError::ModelLoadError(
                    format!("Cannot build qwen2 weights: {e}")
                ))?;
            GgufWeights::Qwen2(w)
        }
        other => {
            return Err(InferenceError::InvalidFormat(format!(
                "GGUF architecture '{other}' is not yet supported. \
                 Supported: llama, mistral, qwen2. \
                 See CONTRIBUTORS.md to add support."
            )));
        }
    };

    Ok(GgufModel { weights, config, vocab_size, num_layers, tokenizer })
}

// ── SafeTensors ───────────────────────────────────────────────────────────────

/// Full-precision model loaded from SafeTensors shards.
pub struct SafeTensorsModel {
    /// Llama model (F16 or F32).
    pub model: candle_transformers::models::llama::Llama,
    /// Architecture config derived from HuggingFace `config.json`.
    pub config: ModelConfig,
    /// Candle runtime config (kept for the generation loop).
    pub llama_config: candle_transformers::models::llama::Config,
    /// Vocabulary size.
    pub vocab_size: usize,
    /// Number of transformer layers.
    pub num_layers: usize,
    /// BPE tokenizer loaded from `tokenizer.json` in the snapshot directory.
    pub tokenizer: BpeTokenizer,
}

/// Load a SafeTensors model from one or more shard files.
///
/// `safetensors_paths` — one or more `.safetensors` shard files.
/// `config_json_path`  — HuggingFace-style `config.json` in the same directory.
pub fn load_safetensors(
    safetensors_paths: &[&Path],
    config_json_path: &Path,
    device: &Device,
) -> InferenceResult<SafeTensorsModel> {
    use candle_core::DType;
    use candle_nn::VarBuilder;
    // LlamaConfig is the HuggingFace-compatible struct (implements Deserialize).
    // Config is the internal candle runtime struct produced by into_config().
    use candle_transformers::models::llama::{Llama, LlamaConfig};

    // Parse HuggingFace config.json using the Deserialize impl on LlamaConfig.
    let config_str = std::fs::read_to_string(config_json_path).map_err(|e| {
        InferenceError::ModelLoadError(format!(
            "Cannot read {}: {e}",
            config_json_path.display()
        ))
    })?;
    let hf_config: LlamaConfig = serde_json::from_str(&config_str).map_err(|e| {
        InferenceError::ModelLoadError(format!("Cannot parse config.json: {e}"))
    })?;

    info!(
        "SafeTensors: {} layers, {} heads ({} kv), hidden={}, vocab={}",
        hf_config.num_hidden_layers,
        hf_config.num_attention_heads,
        hf_config.num_key_value_heads(),
        hf_config.hidden_size,
        hf_config.vocab_size,
    );

    let vocab_size = hf_config.vocab_size;
    let num_layers = hf_config.num_hidden_layers;
    let num_heads = hf_config.num_attention_heads;
    let num_kv_heads = hf_config.num_key_value_heads();
    let hidden_dim = hf_config.hidden_size;
    let inter_dim = hf_config.intermediate_size;
    let rope_theta = hf_config.rope_theta;
    let max_seq_len = hf_config.max_position_embeddings;
    let rms_eps = hf_config.rms_norm_eps;

    // Convert to candle's runtime Config (use_flash_attn=false until we add
    // flash-attention support in a future step).
    let llama_config = hf_config.into_config(false);

    // Memory-map the weight shards (safe for read-only model files).
    // SAFETY: callers must ensure no other process writes to these files
    // while the model is loaded.
    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(safetensors_paths, DType::F16, device).map_err(
            |e| InferenceError::ModelLoadError(format!("Cannot mmap SafeTensors shards: {e}")),
        )?
    };

    let model = Llama::load(vb, &llama_config).map_err(|e| {
        InferenceError::ModelLoadError(format!("Cannot build Llama model: {e}"))
    })?;

    let config = ModelConfig {
        architecture: "llama".to_string(),
        max_seq_len,
        num_heads,
        num_kv_heads,
        hidden_dim,
        intermediate_dim: inter_dim,
        rope_theta,
        layer_norm_eps: rms_eps as f32,
    };

    // Load the BPE tokenizer from tokenizer.json in the snapshot directory.
    let tokenizer_path = config_json_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("tokenizer.json");
    let tokenizer = BpeTokenizer::from_file(&tokenizer_path)?;

    Ok(SafeTensorsModel { model, config, llama_config, vocab_size, num_layers, tokenizer })
}

// ── GGUF metadata helpers ─────────────────────────────────────────────────────

fn meta_str(ct: &gguf_file::Content, key: &str) -> Option<String> {
    use gguf_file::Value;
    match ct.metadata.get(key)? {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

fn meta_usize(ct: &gguf_file::Content, key: &str) -> Option<usize> {
    use gguf_file::Value;
    match ct.metadata.get(key)? {
        Value::U8(v)             => Some(*v as usize),
        Value::U16(v)            => Some(*v as usize),
        Value::U32(v)            => Some(*v as usize),
        Value::U64(v)            => Some(*v as usize),
        Value::I8(v) if *v > 0  => Some(*v as usize),
        Value::I16(v) if *v > 0 => Some(*v as usize),
        Value::I32(v) if *v > 0 => Some(*v as usize),
        Value::I64(v) if *v > 0 => Some(*v as usize),
        _ => None,
    }
}

fn meta_f32(ct: &gguf_file::Content, key: &str) -> Option<f32> {
    use gguf_file::Value;
    match ct.metadata.get(key)? {
        Value::F32(v) => Some(*v),
        Value::F64(v) => Some(*v as f32),
        _ => None,
    }
}
