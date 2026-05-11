use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChunkStrategy {
    #[default]
    Character, // sliding-window over Unicode scalars (original behaviour)
    Paragraph, // paragraph → sentence → character cascade (semantic)
}

#[derive(Debug, Clone)]
pub struct ChunkConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    /// Chunks shorter than this (in chars) are dropped.
    pub min_chunk_len: usize,
    pub strategy: ChunkStrategy,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 800,
            chunk_overlap: 200,
            min_chunk_len: 20,
            strategy: ChunkStrategy::Character,
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

/// Deterministic stable chunk ID.
pub fn chunk_id(doc_name: &str, chunk_index: u32) -> i64 {
    let mut h = Sha256::new();
    h.update(doc_name.as_bytes());
    h.update(b"::");
    h.update(chunk_index.to_le_bytes());
    let bytes = h.finalize();
    i64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// Split text into chunks using the configured strategy.
pub fn split_text(text: &str, doc_name: &str, cfg: &ChunkConfig) -> Vec<Chunk> {
    match cfg.strategy {
        ChunkStrategy::Character => split_character(text, doc_name, cfg),
        ChunkStrategy::Paragraph => split_paragraph(text, doc_name, cfg),
    }
}

// ── Character strategy (original) ────────────────────────────────────────────

fn split_character(text: &str, doc_name: &str, cfg: &ChunkConfig) -> Vec<Chunk> {
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

        let surr_half = cfg.chunk_size / 4;
        let surr_start = pos.saturating_sub(surr_half);
        let surr_end = (end + surr_half).min(total);
        let surrounding: String = chars[surr_start..surr_end].iter().collect();

        if text_str.chars().count() >= cfg.min_chunk_len {
            chunks.push(Chunk {
                id: chunk_id(doc_name, index),
                text: text_str,
                surrounding,
                doc_name: doc_name.to_string(),
                chunk_index: index,
                page_num: None,
            });
            index += 1;
        }

        pos += step;
    }
    chunks
}

// ── Paragraph strategy ────────────────────────────────────────────────────────

fn split_paragraph(text: &str, doc_name: &str, cfg: &ChunkConfig) -> Vec<Chunk> {
    let units = collect_units(text, cfg);
    if units.is_empty() {
        return vec![];
    }

    let chunk_texts = pack_chunks(&units, cfg);
    let surr_half = cfg.chunk_size / 4;
    let mut result = Vec::new();
    let mut index = 0u32;

    for (i, text_str) in chunk_texts.iter().enumerate() {
        if text_str.chars().count() < cfg.min_chunk_len {
            continue;
        }

        // Surrounding: tail of prev chunk + this chunk + head of next chunk.
        let mut surrounding = String::new();
        if i > 0 {
            let prev: Vec<char> = chunk_texts[i - 1].chars().collect();
            let tail_start = prev.len().saturating_sub(surr_half);
            surrounding.push_str(&prev[tail_start..].iter().collect::<String>());
            surrounding.push(' ');
        }
        surrounding.push_str(text_str);
        if i + 1 < chunk_texts.len() {
            let next: Vec<char> = chunk_texts[i + 1].chars().collect();
            let head_end = next.len().min(surr_half);
            surrounding.push(' ');
            surrounding.push_str(&next[..head_end].iter().collect::<String>());
        }

        result.push(Chunk {
            id: chunk_id(doc_name, index),
            text: text_str.clone(),
            surrounding,
            doc_name: doc_name.to_string(),
            chunk_index: index,
            page_num: None,
        });
        index += 1;
    }
    result
}

/// Collect atomic units from text: paragraphs split on `\n\n`, with oversized
/// paragraphs further split at sentence boundaries and then characters.
/// Short paragraphs (< min_chunk_len) are merged into their neighbour.
fn collect_units(text: &str, cfg: &ChunkConfig) -> Vec<String> {
    let mut units: Vec<String> = Vec::new();
    let mut acc = String::new();

    for para in text.split("\n\n") {
        let para = para.trim();
        if para.is_empty() {
            continue;
        }

        if para.chars().count() <= cfg.chunk_size {
            if para.chars().count() < cfg.min_chunk_len {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(para);
            } else {
                flush_acc(&mut acc, &mut units, cfg);
                units.push(para.to_string());
            }
        } else {
            flush_acc(&mut acc, &mut units, cfg);
            split_sentences(para, cfg, &mut units);
        }
    }

    flush_acc(&mut acc, &mut units, cfg);
    units
}

fn flush_acc(acc: &mut String, units: &mut Vec<String>, cfg: &ChunkConfig) {
    if acc.is_empty() {
        return;
    }
    if acc.chars().count() >= cfg.min_chunk_len {
        units.push(std::mem::take(acc));
    } else if let Some(last) = units.last_mut() {
        last.push('\n');
        last.push_str(acc);
        acc.clear();
    } else {
        // Nothing to merge into — keep until there is something
        // (will be re-visited on the next paragraph)
    }
}

/// Split text at sentence boundaries (`.`, `!`, `?` followed by whitespace + letter).
/// Falls back to character split for sentences longer than chunk_size.
fn split_sentences(text: &str, cfg: &ChunkConfig, out: &mut Vec<String>) {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut start = 0;
    let mut i = 0;

    while i < n {
        if matches!(chars[i], '.' | '!' | '?')
            && i + 2 < n
            && chars[i + 1].is_whitespace()
            && chars[i + 2].is_alphabetic()
        {
            let seg: String = chars[start..=i].iter().collect();
            let seg = seg.trim().to_string();
            if seg.chars().count() > cfg.chunk_size {
                split_chars(&seg, cfg, out);
            } else if seg.chars().count() >= cfg.min_chunk_len {
                out.push(seg);
            }
            start = i + 2; // skip whitespace
            i = start;
            continue;
        }
        i += 1;
    }

    if start < n {
        let tail: String = chars[start..].iter().collect();
        let tail = tail.trim().to_string();
        if tail.chars().count() > cfg.chunk_size {
            split_chars(&tail, cfg, out);
        } else if tail.chars().count() >= cfg.min_chunk_len {
            out.push(tail);
        }
    }
}

/// Character-level fallback split (for very long sentences).
fn split_chars(text: &str, cfg: &ChunkConfig, out: &mut Vec<String>) {
    let chars: Vec<char> = text.chars().collect();
    let step = cfg.chunk_size.saturating_sub(cfg.chunk_overlap).max(1);
    let mut pos = 0;
    while pos < chars.len() {
        let end = (pos + cfg.chunk_size).min(chars.len());
        let s: String = chars[pos..end].iter().collect();
        if s.chars().count() >= cfg.min_chunk_len {
            out.push(s);
        }
        pos += step;
    }
}

/// Pack collected units into chunks up to chunk_size, prepending an overlap
/// tail from the previous chunk at each boundary.
fn pack_chunks(units: &[String], cfg: &ChunkConfig) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    let mut cur_len = 0usize;

    for unit in units {
        let unit_len = unit.chars().count();
        let sep = if parts.is_empty() { 0 } else { 1 };

        if parts.is_empty() || cur_len + sep + unit_len <= cfg.chunk_size {
            cur_len += sep + unit_len;
            parts.push(unit.clone());
        } else {
            // Emit current chunk.
            chunks.push(parts.join("\n"));

            // Overlap prefix from tail of emitted chunk.
            let prev = chunks.last().unwrap();
            let prev_chars: Vec<char> = prev.chars().collect();
            let ol_start = prev_chars.len().saturating_sub(cfg.chunk_overlap);
            let overlap: String = prev_chars[ol_start..].iter().collect();

            parts.clear();
            cur_len = 0;

            if !overlap.is_empty() {
                cur_len += overlap.chars().count() + 1;
                parts.push(overlap);
            }
            cur_len += unit_len;
            parts.push(unit.clone());
        }
    }

    if !parts.is_empty() {
        chunks.push(parts.join("\n"));
    }

    chunks
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_id_is_stable() {
        assert_eq!(chunk_id("doc.txt", 0), chunk_id("doc.txt", 0));
        assert_ne!(chunk_id("doc.txt", 0), chunk_id("doc.txt", 1));
    }

    #[test]
    fn character_split_basic() {
        let cfg = ChunkConfig {
            chunk_size: 10,
            chunk_overlap: 2,
            min_chunk_len: 5,
            ..Default::default()
        };
        let chunks = split_text("Hello world foo bar baz", "test.txt", &cfg);
        assert!(!chunks.is_empty());
        assert!(chunks[0].text.chars().count() <= 10);
    }

    #[test]
    fn paragraph_split_respects_boundaries() {
        let text = "First paragraph about topic A.\n\nSecond paragraph about topic B.\n\nThird paragraph about topic C.";
        let cfg = ChunkConfig {
            chunk_size: 200,
            chunk_overlap: 20,
            min_chunk_len: 10,
            strategy: ChunkStrategy::Paragraph,
        };
        let chunks = split_text(text, "test.txt", &cfg);
        // All three short paragraphs fit in one 200-char chunk.
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("First paragraph"));
    }

    #[test]
    fn paragraph_split_large_input() {
        let para_a = "The first paragraph contains information about the history of District Six in Cape Town, South Africa. It was a vibrant multicultural community.";
        let para_b = "The second paragraph describes the forced removals that took place under the Group Areas Act. Thousands of residents were displaced.";
        let para_c = "The third paragraph talks about the organisations that resisted the apartheid government policies. Many groups were involved.";
        let text = format!("{para_a}\n\n{para_b}\n\n{para_c}");
        let cfg = ChunkConfig {
            chunk_size: 200,
            chunk_overlap: 30,
            min_chunk_len: 20,
            strategy: ChunkStrategy::Paragraph,
        };
        let chunks = split_text(&text, "test.txt", &cfg);
        assert!(!chunks.is_empty());
        // Chunks should not cut mid-paragraph if they fit
        for chunk in &chunks {
            assert!(chunk.text.chars().count() <= cfg.chunk_size + cfg.chunk_overlap + 10);
        }
    }

    #[test]
    fn paragraph_surrounding_populated() {
        let text = "Para one.\n\nPara two which is a bit longer and has more content.\n\nPara three at the end.";
        let cfg = ChunkConfig {
            chunk_size: 30,
            chunk_overlap: 5,
            min_chunk_len: 5,
            strategy: ChunkStrategy::Paragraph,
        };
        let chunks = split_text(text, "test.txt", &cfg);
        // Middle chunks should have surrounding that's longer than just the text
        if chunks.len() > 1 {
            let middle = &chunks[1];
            assert!(middle.surrounding.len() >= middle.text.len());
        }
    }
}
