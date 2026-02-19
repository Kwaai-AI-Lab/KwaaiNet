//! BPE tokenizer backed by the HuggingFace `tokenizers` library.
//!
//! Two loading paths:
//!   - [`BpeTokenizer::from_file`] — loads a `tokenizer.json` from a
//!     HuggingFace model snapshot (used with SafeTensors models)
//!   - [`BpeTokenizer::from_gguf`] — builds BPE from the vocabulary and
//!     merge rules embedded in a GGUF file (used with Ollama GGUF models)

use crate::error::{InferenceError, InferenceResult};
use candle_core::quantized::gguf_file;
use std::path::Path;
use tokenizers::Tokenizer as HfTokenizer;
use tracing::info;

// ── Tokenizer trait ───────────────────────────────────────────────────────────

/// Common interface for all tokenizers.
pub trait Tokenizer: Send + Sync {
    /// Encode a text string to a sequence of token IDs.
    fn encode(&self, text: &str) -> InferenceResult<Vec<u32>>;

    /// Decode a sequence of token IDs back to text.
    /// Special tokens are skipped.
    fn decode(&self, tokens: &[u32]) -> InferenceResult<String>;

    /// Total vocabulary size (including added special tokens).
    fn vocab_size(&self) -> usize;

    /// Beginning-of-sequence token ID, if the model has one.
    fn bos_token_id(&self) -> Option<u32>;

    /// End-of-sequence token ID, if the model has one.
    fn eos_token_id(&self) -> Option<u32>;

    /// Padding token ID, if the model has one.
    fn pad_token_id(&self) -> Option<u32>;
}

// ── BpeTokenizer ──────────────────────────────────────────────────────────────

/// Real BPE tokenizer.  Wraps the HuggingFace `tokenizers` crate.
pub struct BpeTokenizer {
    inner: HfTokenizer,
    bos_id: Option<u32>,
    eos_id: Option<u32>,
    pad_id: Option<u32>,
}

impl BpeTokenizer {
    // ── Constructors ─────────────────────────────────────────────────────────

    /// Load from a HuggingFace `tokenizer.json` file.
    ///
    /// This is the path used for full-precision SafeTensors model snapshots.
    /// The file must live in the same directory as `config.json`.
    pub fn from_file(path: &Path) -> InferenceResult<Self> {
        let inner = HfTokenizer::from_file(path).map_err(|e| {
            InferenceError::ModelLoadError(format!(
                "Cannot load tokenizer from {}: {e}",
                path.display()
            ))
        })?;

        // Special token IDs — try the most common names for each role.
        let bos_id = inner
            .token_to_id("<|begin_of_text|>") // Llama 3
            .or_else(|| inner.token_to_id("<s>")); // Llama 2, Mistral

        let eos_id = inner
            .token_to_id("<|eot_id|>") // Llama 3 instruct
            .or_else(|| inner.token_to_id("<|endoftext|>")) // GPT-2 / Qwen
            .or_else(|| inner.token_to_id("</s>")) // Llama 2, Mistral
            .or_else(|| inner.token_to_id("<|im_end|>")); // ChatML

        let pad_id = inner
            .token_to_id("<|finetune_right_pad_id|>") // Llama 3 fine-tunes
            .or_else(|| inner.token_to_id("<pad>"))
            .or_else(|| inner.token_to_id("[PAD]"));

        info!(
            "Loaded tokenizer.json: vocab={}, bos={:?}, eos={:?}",
            inner.get_vocab_size(true),
            bos_id,
            eos_id,
        );

        Ok(Self { inner, bos_id, eos_id, pad_id })
    }

    /// Build a BPE tokenizer from the vocabulary and merge rules embedded
    /// inside a GGUF file (`tokenizer.ggml.tokens` / `tokenizer.ggml.merges`).
    ///
    /// This is the path used for quantized Ollama GGUF models.
    ///
    /// The implementation serialises the GGUF data into the canonical
    /// HuggingFace tokenizer JSON format and loads it via [`HfTokenizer::from_str`].
    /// This avoids any internal HashMap type compatibility issues between
    /// the `tokenizers` crate and the standard library.
    pub fn from_gguf(gguf: &gguf_file::Content) -> InferenceResult<Self> {
        use serde_json::{json, Map, Value};

        // ── Vocabulary ───────────────────────────────────────────────────────
        let token_strs = gguf_string_array(gguf, "tokenizer.ggml.tokens").map_err(|e| {
            InferenceError::ModelLoadError(format!("Missing GGUF tokenizer vocab: {e}"))
        })?;
        let n = token_strs.len();

        // ── Merge rules ──────────────────────────────────────────────────────
        // Each GGUF merge entry is the string "token_a token_b".
        let merges_raw = gguf_string_array(gguf, "tokenizer.ggml.merges")
            .unwrap_or_default();

        if merges_raw.is_empty() {
            return Err(InferenceError::ModelLoadError(
                "GGUF file contains no BPE merge rules. \
                 Provide a tokenizer.json file alongside the model."
                    .to_string(),
            ));
        }

        // ── Special token IDs ────────────────────────────────────────────────
        let bos_id = gguf_u32(gguf, "tokenizer.ggml.bos_token_id")
            .or_else(|| find_token_id(&token_strs, &["<|begin_of_text|>", "<s>"]));
        let eos_id = gguf_u32(gguf, "tokenizer.ggml.eos_token_id")
            .or_else(|| {
                find_token_id(
                    &token_strs,
                    &["<|eot_id|>", "<|endoftext|>", "</s>", "<|im_end|>"],
                )
            });
        let pad_id = gguf_u32(gguf, "tokenizer.ggml.padding_token_id")
            .or_else(|| find_token_id(&token_strs, &["<pad>", "[PAD]"]));

        // ── Construct tokenizer JSON ─────────────────────────────────────────
        // Serialise vocab and merges into the HuggingFace tokenizer JSON schema
        // and load via `HfTokenizer::from_str`.  This sidesteps internal hasher
        // type requirements (ahash vs std) in the `tokenizers` builder API.
        let vocab_map: Map<String, Value> = token_strs
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), Value::Number(i.into())))
            .collect();

        let tokenizer_json = json!({
            "version": "1.0",
            "truncation": null,
            "padding": null,
            "added_tokens": [],
            "normalizer": null,
            "pre_tokenizer": {
                "type": "ByteLevel",
                "add_prefix_space": false,
                "trim_offsets": true,
                "use_regex": true
            },
            "post_processor": null,
            "decoder": {
                "type": "ByteLevel",
                "add_prefix_space": true,
                "trim_offsets": true,
                "use_regex": true
            },
            "model": {
                "type": "BPE",
                "dropout": null,
                "unk_token": "<unk>",
                "continuing_subword_prefix": null,
                "end_of_word_suffix": null,
                "fuse_unk": false,
                "byte_fallback": false,
                "vocab": Value::Object(vocab_map),
                "merges": merges_raw
            }
        });

        let json_str = tokenizer_json.to_string();
        let inner: HfTokenizer = serde_json::from_str(&json_str).map_err(|e| {
            InferenceError::ModelLoadError(format!(
                "Cannot build tokenizer from GGUF vocab/merges: {e}"
            ))
        })?;

        info!(
            "Built GGUF tokenizer: vocab={n}, merges={}, bos={:?}, eos={:?}",
            merges_raw.len(),
            bos_id,
            eos_id,
        );

        Ok(Self { inner, bos_id, eos_id, pad_id })
    }
}

// ── Tokenizer trait impl ──────────────────────────────────────────────────────

impl Tokenizer for BpeTokenizer {
    fn encode(&self, text: &str) -> InferenceResult<Vec<u32>> {
        self.inner
            .encode(text, false)
            .map(|enc| enc.get_ids().to_vec())
            .map_err(|e| InferenceError::TokenizationError(e.to_string()))
    }

    fn decode(&self, tokens: &[u32]) -> InferenceResult<String> {
        self.inner
            .decode(tokens, true)
            .map_err(|e| InferenceError::TokenizationError(e.to_string()))
    }

    fn vocab_size(&self) -> usize {
        self.inner.get_vocab_size(true)
    }

    fn bos_token_id(&self) -> Option<u32> {
        self.bos_id
    }

    fn eos_token_id(&self) -> Option<u32> {
        self.eos_id
    }

    fn pad_token_id(&self) -> Option<u32> {
        self.pad_id
    }
}

// ── GGUF metadata helpers ─────────────────────────────────────────────────────

fn gguf_string_array(ct: &gguf_file::Content, key: &str) -> Result<Vec<String>, String> {
    use gguf_file::Value;
    match ct.metadata.get(key) {
        Some(Value::Array(arr)) => arr
            .iter()
            .map(|v| match v {
                Value::String(s) => Ok(s.clone()),
                _ => Err(format!("{key}: expected string elements")),
            })
            .collect(),
        None => Err(format!("key '{key}' not found")),
        _ => Err(format!("key '{key}' is not an array")),
    }
}

fn gguf_u32(ct: &gguf_file::Content, key: &str) -> Option<u32> {
    use gguf_file::Value;
    match ct.metadata.get(key)? {
        Value::U32(v) => Some(*v),
        Value::U64(v) => Some(*v as u32),
        Value::I32(v) if *v >= 0 => Some(*v as u32),
        Value::I64(v) if *v >= 0 => Some(*v as u32),
        _ => None,
    }
}

fn find_token_id(tokens: &[String], candidates: &[&str]) -> Option<u32> {
    for &name in candidates {
        if let Some(id) = tokens.iter().position(|t| t == name) {
            return Some(id as u32);
        }
    }
    None
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify the public API compiles and the struct is well-formed.
    /// Full tokenisation tests require a model file; run those as integration tests.
    #[test]
    fn test_bpe_tokenizer_types() {
        // BpeTokenizer must be Send + Sync (required by InferenceEngine).
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<BpeTokenizer>();
    }
}
