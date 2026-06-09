use sha2::{Digest, Sha256};

use crate::doc_schema::{match_section, DocSchema};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ChunkStrategy {
    #[default]
    Character, // sliding-window over Unicode scalars (original behaviour)
    Paragraph, // paragraph → sentence → character cascade (semantic)
}

/// Controls how much surrounding context is stored alongside each chunk.
///
/// `Truncated` (default): ±(chunk_size/4) chars from adjacent chunks.
/// `Full`: for paragraph strategy, the complete adjacent chunks are included,
///   giving the LLM full enclosing paragraphs without altering the retrieval
///   embedding (which is always computed on the narrow chunk text only).
#[derive(Debug, Clone, PartialEq, Default)]
pub enum SurrMode {
    #[default]
    Truncated,
    Full,
}

#[derive(Debug, Clone)]
pub struct ChunkConfig {
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    /// Chunks shorter than this (in chars) are dropped.
    pub min_chunk_len: usize,
    pub strategy: ChunkStrategy,
    pub surr_mode: SurrMode,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            chunk_size: 800,
            chunk_overlap: 200,
            min_chunk_len: 20,
            strategy: ChunkStrategy::Character,
            surr_mode: SurrMode::Truncated,
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
    /// Section heading that was active when this chunk was produced (from DocSchema).
    pub section_name: Option<String>,
    /// When true, this chunk should be skipped during graph/entity extraction.
    pub skip_extraction: bool,
    /// Narrator note from DocSchema — injected into extraction prompts for this chunk.
    pub section_note: Option<String>,
    /// Semantic section type — used to enforce context-window boundaries.
    pub section_type: crate::doc_schema::SectionType,
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
///
/// If `schema` is provided, paragraph-level headings are matched against the
/// schema sections to tag each chunk with `section_name`, `skip_extraction`,
/// and `section_note`. Has no effect on the Character strategy.
pub fn split_text(
    text: &str,
    doc_name: &str,
    cfg: &ChunkConfig,
    schema: Option<&DocSchema>,
) -> Vec<Chunk> {
    match cfg.strategy {
        ChunkStrategy::Character => split_character(text, doc_name, cfg),
        ChunkStrategy::Paragraph => split_paragraph(text, doc_name, cfg, schema),
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
                section_name: None,
                skip_extraction: false,
                section_note: None,
                section_type: crate::doc_schema::SectionType::Main,
            });
            index += 1;
        }

        pos += step;
    }
    chunks
}

// ── Paragraph strategy ────────────────────────────────────────────────────────

#[allow(clippy::type_complexity)]
fn split_paragraph(
    text: &str,
    doc_name: &str,
    cfg: &ChunkConfig,
    schema: Option<&DocSchema>,
) -> Vec<Chunk> {
    let units = collect_units_with_headings(text, cfg);
    if units.is_empty() {
        return vec![];
    }

    // Resolve section metadata for each content unit. Headings update state
    // but are never emitted as chunks. A schema match resets skip+note; an
    // unrecognised heading updates the name only (skip/note keep prior value
    // but are cleared when no schema is loaded).
    let mut cur_section_name: Option<String> = None;
    let mut cur_skip = false;
    let mut cur_note: Option<String> = None;
    let mut cur_section_type = crate::doc_schema::SectionType::Main;

    // (text, section_name, skip, note, section_type) — content units only, in order.
    let mut content_units: Vec<(
        String,
        Option<String>,
        bool,
        Option<String>,
        crate::doc_schema::SectionType,
    )> = Vec::new();

    for (is_heading, unit_text) in &units {
        if *is_heading {
            if let Some(schema) = schema {
                if let Some(sec) = match_section(unit_text, schema) {
                    cur_section_name = Some(unit_text.clone());
                    cur_skip = sec.skip;
                    cur_note = sec.narrator_note.clone();
                    cur_section_type = sec.section_type.clone();
                } else {
                    // Unrecognised heading: update name, reset skip/note to
                    // neutral so chapters after a skip section aren't tainted.
                    cur_section_name = Some(unit_text.clone());
                    cur_skip = false;
                    cur_note = None;
                    cur_section_type = crate::doc_schema::SectionType::Main;
                }
            } else {
                cur_section_name = Some(unit_text.clone());
            }
        } else {
            content_units.push((
                unit_text.clone(),
                cur_section_name.clone(),
                cur_skip,
                cur_note.clone(),
                cur_section_type.clone(),
            ));
        }
    }

    // Pack units into chunks, carrying the section metadata of the FIRST unit
    // that opens each packed chunk. This avoids any substring-matching
    // heuristic — metadata is assigned deterministically as packing proceeds.
    let packed = pack_chunks_with_meta(&content_units, cfg);

    let surr_half = cfg.chunk_size / 4;
    // Build plain chunk texts for surrounding computation.
    let chunk_texts: Vec<&str> = packed.iter().map(|(t, _, _, _, _)| t.as_str()).collect();

    let mut result = Vec::new();
    let mut index = 0u32;

    for (i, (text_str, sec_name, skip, note, sec_type)) in packed.iter().enumerate() {
        if text_str.chars().count() < cfg.min_chunk_len {
            continue;
        }

        let mut surrounding = String::new();
        match cfg.surr_mode {
            SurrMode::Full => {
                if i > 0 {
                    surrounding.push_str(chunk_texts[i - 1]);
                    surrounding.push(' ');
                }
                surrounding.push_str(text_str);
                if i + 1 < chunk_texts.len() {
                    surrounding.push(' ');
                    surrounding.push_str(chunk_texts[i + 1]);
                }
            }
            SurrMode::Truncated => {
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
            }
        }

        result.push(Chunk {
            id: chunk_id(doc_name, index),
            text: text_str.clone(),
            surrounding,
            doc_name: doc_name.to_string(),
            chunk_index: index,
            page_num: None,
            section_name: sec_name.clone(),
            skip_extraction: *skip,
            section_note: note.clone(),
            section_type: sec_type.clone(),
        });
        index += 1;
    }
    result
}

/// Pack content units into chunks up to `chunk_size`, carrying the section
/// metadata of the FIRST unit that opens each packed chunk.
#[allow(clippy::type_complexity)]
fn pack_chunks_with_meta(
    units: &[(
        String,
        Option<String>,
        bool,
        Option<String>,
        crate::doc_schema::SectionType,
    )],
    cfg: &ChunkConfig,
) -> Vec<(
    String,
    Option<String>,
    bool,
    Option<String>,
    crate::doc_schema::SectionType,
)> {
    type Meta = (
        Option<String>,
        bool,
        Option<String>,
        crate::doc_schema::SectionType,
    );
    let mut result: Vec<(
        String,
        Option<String>,
        bool,
        Option<String>,
        crate::doc_schema::SectionType,
    )> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    let mut cur_len = 0usize;
    let mut cur_meta: Meta = (None, false, None, crate::doc_schema::SectionType::Main);

    let emit = |parts: &mut Vec<String>,
                cur_meta: &Meta,
                result: &mut Vec<(
        String,
        Option<String>,
        bool,
        Option<String>,
        crate::doc_schema::SectionType,
    )>| {
        if !parts.is_empty() {
            result.push((
                parts.join("\n"),
                cur_meta.0.clone(),
                cur_meta.1,
                cur_meta.2.clone(),
                cur_meta.3.clone(),
            ));
        }
    };

    for (unit_text, sec_name, skip, note, sec_type) in units {
        let unit_len = unit_text.chars().count();
        let sep = if parts.is_empty() { 0 } else { 1 };

        // Section-boundary: never pack units from different section zones into the
        // same chunk. This prevents Acknowledgements text bleeding into Dedication.
        let same_zone = parts.is_empty() || cur_meta.3.same_window_zone(sec_type);

        if same_zone && (parts.is_empty() || cur_len + sep + unit_len <= cfg.chunk_size) {
            if parts.is_empty() {
                cur_meta = (sec_name.clone(), *skip, note.clone(), sec_type.clone());
            }
            cur_len += sep + unit_len;
            parts.push(unit_text.clone());
        } else {
            emit(&mut parts, &cur_meta, &mut result);

            // Overlap prefix from tail of emitted chunk — but only within the same zone.
            let prev_zone_same = result
                .last()
                .map(|(_, _, _, _, t)| t.same_window_zone(sec_type))
                .unwrap_or(false);
            let overlap = if prev_zone_same {
                let prev_text = result
                    .last()
                    .map(|(t, _, _, _, _)| t.as_str())
                    .unwrap_or("");
                let prev_chars: Vec<char> = prev_text.chars().collect();
                let ol_start = prev_chars.len().saturating_sub(cfg.chunk_overlap);
                prev_chars[ol_start..].iter().collect::<String>()
            } else {
                String::new()
            };

            parts.clear();
            cur_len = 0;
            cur_meta = (sec_name.clone(), *skip, note.clone(), sec_type.clone());

            if !overlap.is_empty() {
                cur_len += overlap.chars().count() + 1;
                parts.push(overlap);
            }
            cur_len += unit_len;
            parts.push(unit_text.clone());
        }
    }

    emit(&mut parts, &cur_meta, &mut result);
    result
}

/// Returns `(is_heading, text)` pairs for all paragraphs.
/// A paragraph is treated as a heading if it is a single line ≤ 120 chars.
/// Heading entries update section state but are never emitted as retrieval chunks.
fn collect_units_with_headings(text: &str, cfg: &ChunkConfig) -> Vec<(bool, String)> {
    let mut result: Vec<(bool, String)> = Vec::new();
    let mut acc = String::new();

    let flush = |acc: &mut String, result: &mut Vec<(bool, String)>, cfg: &ChunkConfig| {
        if acc.is_empty() {
            return;
        }
        if acc.chars().count() >= cfg.min_chunk_len {
            result.push((false, std::mem::take(acc)));
        } else if let Some((_, last)) = result.last_mut() {
            last.push('\n');
            last.push_str(acc);
            acc.clear();
        }
    };

    for para in text.split("\n\n") {
        let para = para.trim();
        if para.is_empty() {
            continue;
        }

        // A heading is a single line, short (≤80 chars), and does not end with
        // sentence-terminating punctuation (.  !  ?).  This avoids treating short
        // content sentences as section boundaries.
        let trimmed_para = para.trim_end();
        let is_heading = para.lines().count() == 1
            && para.chars().count() <= 80
            && !trimmed_para.ends_with('.')
            && !trimmed_para.ends_with('!')
            && !trimmed_para.ends_with('?');

        if is_heading {
            flush(&mut acc, &mut result, cfg);
            result.push((true, para.to_string()));
        } else if para.chars().count() <= cfg.chunk_size {
            if para.chars().count() < cfg.min_chunk_len {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(para);
            } else {
                flush(&mut acc, &mut result, cfg);
                result.push((false, para.to_string()));
            }
        } else {
            flush(&mut acc, &mut result, cfg);
            let mut out: Vec<String> = Vec::new();
            split_sentences(para, cfg, &mut out);
            for s in out {
                result.push((false, s));
            }
        }
    }

    flush(&mut acc, &mut result, cfg);
    result
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
#[allow(dead_code)]
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
        let chunks = split_text("Hello world foo bar baz", "test.txt", &cfg, None);
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
            ..Default::default()
        };
        let chunks = split_text(text, "test.txt", &cfg, None);
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
            ..Default::default()
        };
        let chunks = split_text(&text, "test.txt", &cfg, None);
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
            ..Default::default()
        };
        let chunks = split_text(text, "test.txt", &cfg, None);
        // Middle chunks should have surrounding that's longer than just the text
        if chunks.len() > 1 {
            let middle = &chunks[1];
            assert!(middle.surrounding.len() >= middle.text.len());
        }
    }
}
