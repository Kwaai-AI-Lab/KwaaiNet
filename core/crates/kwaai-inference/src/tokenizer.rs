//! Tokenizer abstraction

use crate::error::{InferenceError, InferenceResult};

/// Trait for tokenizers
pub trait Tokenizer: Send + Sync {
    /// Encode text to token IDs
    fn encode(&self, text: &str) -> InferenceResult<Vec<u32>>;

    /// Decode token IDs to text
    fn decode(&self, tokens: &[u32]) -> InferenceResult<String>;

    /// Get vocabulary size
    fn vocab_size(&self) -> usize;

    /// Get the BOS (beginning of sequence) token ID
    fn bos_token_id(&self) -> Option<u32>;

    /// Get the EOS (end of sequence) token ID
    fn eos_token_id(&self) -> Option<u32>;

    /// Get the PAD token ID
    fn pad_token_id(&self) -> Option<u32>;
}

/// Simple placeholder tokenizer
pub struct SimpleTokenizer {
    vocab_size: usize,
}

impl SimpleTokenizer {
    /// Create a new simple tokenizer
    pub fn new(vocab_size: usize) -> Self {
        Self { vocab_size }
    }
}

impl Tokenizer for SimpleTokenizer {
    fn encode(&self, text: &str) -> InferenceResult<Vec<u32>> {
        // Very simple byte-level tokenization for testing
        Ok(text.bytes().map(|b| b as u32).collect())
    }

    fn decode(&self, tokens: &[u32]) -> InferenceResult<String> {
        let bytes: Vec<u8> = tokens
            .iter()
            .filter_map(|&t| if t < 256 { Some(t as u8) } else { None })
            .collect();
        String::from_utf8(bytes).map_err(|e| InferenceError::TokenizationError(e.to_string()))
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }

    fn bos_token_id(&self) -> Option<u32> {
        Some(1)
    }

    fn eos_token_id(&self) -> Option<u32> {
        Some(2)
    }

    fn pad_token_id(&self) -> Option<u32> {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenizer() {
        let tokenizer = SimpleTokenizer::new(256);
        let text = "Hello";
        let tokens = tokenizer.encode(text).unwrap();
        let decoded = tokenizer.decode(&tokens).unwrap();
        assert_eq!(decoded, text);
    }
}
