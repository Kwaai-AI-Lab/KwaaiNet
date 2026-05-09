use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct ChunkConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 800,
            chunk_overlap: 200,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: i64,
    pub text: String,
    pub surrounding: String,
    pub doc_name: String,
    pub chunk_index: u32,
    pub page_num: Option<u32>,
}

/// Deterministic stable ID: truncate(sha256(doc_name + "::" + chunk_index), 8 bytes) → i64
pub fn chunk_id(doc_name: &str, chunk_index: u32) -> i64 {
    let mut h = Sha256::new();
    h.update(doc_name.as_bytes());
    h.update(b"::");
    h.update(chunk_index.to_le_bytes());
    let bytes = h.finalize();
    i64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// Split text into overlapping chunks using a sliding window over Unicode scalar values.
/// Each chunk also stores a `surrounding` field (up to 1.5× chunk_size) for sentence-window retrieval.
pub fn split_text(text: &str, doc_name: &str, cfg: &ChunkConfig) -> Vec<Chunk> {
    let chars: Vec<char> = text.chars().collect();
    let total = chars.len();
    if total == 0 {
        return vec![];
    }

    let step = cfg.chunk_size.saturating_sub(cfg.chunk_overlap).max(1);
    let mut chunks = Vec::new();
    let mut pos = 0usize;
    let mut index = 0u32;

    while pos < total {
        let end = (pos + cfg.chunk_size).min(total);
        let text_str: String = chars[pos..end].iter().collect();

        // Surrounding: expand by half chunk_size on each side for context injection
        let surr_half = cfg.chunk_size / 4;
        let surr_start = pos.saturating_sub(surr_half);
        let surr_end = (end + surr_half).min(total);
        let surrounding: String = chars[surr_start..surr_end].iter().collect();

        chunks.push(Chunk {
            id: chunk_id(doc_name, index),
            text: text_str,
            surrounding,
            doc_name: doc_name.to_string(),
            chunk_index: index,
            page_num: None,
        });

        index += 1;
        pos += step;
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_are_stable() {
        let id1 = chunk_id("doc.txt", 0);
        let id2 = chunk_id("doc.txt", 0);
        assert_eq!(id1, id2);
        assert_ne!(chunk_id("doc.txt", 0), chunk_id("doc.txt", 1));
    }

    #[test]
    fn split_basic() {
        let cfg = ChunkConfig {
            chunk_size: 10,
            chunk_overlap: 2,
        };
        let chunks = split_text("Hello world foo bar baz", "test.txt", &cfg);
        assert!(!chunks.is_empty());
        assert!(chunks[0].text.len() <= 10);
    }
}
