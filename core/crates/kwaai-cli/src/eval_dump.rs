//! Machine-readable per-question records for `rag eval --dump-jsonl`.
//!
//! The markdown report keeps only document names and keyword scores, which is too
//! little to re-score a run offline (graded passage relevance, NLI nugget coverage).
//! One JSON line per question records what was retrieved, in which order, which
//! chunks the generator actually saw, and the answer — enough to score the run again
//! without re-running inference.

use std::io::Write;

use kwaai_rag::prompt::{context_plan, context_text, ChatMessage};
use kwaai_rag::retriever::RetrievedChunk;
use serde::Serialize;
use sha1::{Digest, Sha1};

/// Bump when a field changes meaning; readers check it before parsing.
pub const SCHEMA_VERSION: u32 = 1;

/// Hash of a passage's text, stable across snapshots of the same KB.
///
/// Whitespace runs collapse to one space and the ends are trimmed, so a chunk
/// re-rendered with different line breaks keys to the same cache entry. The Python
/// scorer (`tests/kwaai-knowledge/eval2/common.py`) must produce the same digest;
/// `tests/kwaai-knowledge/eval2/fixtures/hash_vectors.json` pins both sides.
pub fn text_hash(text: &str) -> String {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    hex::encode(Sha1::digest(normalized.as_bytes()))
}

#[derive(Debug, Serialize)]
pub struct DumpChunk {
    /// Position in the retriever's ranking, from 0.
    pub rank: usize,
    /// Storage row id; `None` for synthetic chunks (graph fact cards, summaries).
    pub chunk_id: Option<i64>,
    pub synthetic: bool,
    pub doc_name: String,
    pub chunk_index: u32,
    pub section_name: Option<String>,
    /// The chunk's own text: what the hash and gold anchors refer to.
    pub text: String,
    pub text_hash: String,
    /// What the generator saw for this chunk (the wider window when there is one).
    pub context_text: String,
    pub score: f64,
    pub rerank_score: Option<f64>,
    /// Whether the chunk survived the prompt's character budget.
    pub in_prompt: bool,
    /// Citation number `[n]` the generator saw it under, when it was in the prompt.
    pub prompt_slot: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct EvalDumpRecord<'a> {
    pub schema_version: u32,
    pub run_tag: Option<&'a str>,
    pub kb: &'a str,
    pub model: &'a str,
    pub mode: &'a str,
    pub top_k: usize,
    pub inference_url: &'a str,
    /// Graph size when the run started: evidence of which snapshot was loaded.
    pub graph_entities: usize,
    pub graph_relations: usize,
    pub qid: &'a str,
    pub question: &'a str,
    /// The question as sent to the generator (after any expansion).
    pub question_sent: &'a str,
    pub answer: &'a str,
    pub latency_ms: u128,
    pub retrieved: Vec<DumpChunk>,
    pub messages: &'a [ChatMessage],
    pub keyword_hits: f32,
    pub retrieval_hits: f32,
    pub total_keywords: f32,
    pub judge_score: Option<u8>,
}

/// Describe `chunks` in retrieval order, marking which ones the prompt kept.
///
/// `max_context_chars` must be the budget the prompt was built with, so the
/// manifest is the same plan `build_chat_messages` rendered.
pub fn dump_chunks(chunks: &[RetrievedChunk], max_context_chars: usize) -> Vec<DumpChunk> {
    let plan = context_plan(chunks, max_context_chars);
    let mut slot_of = vec![None; chunks.len()];
    for (slot, &idx) in plan.iter().enumerate() {
        slot_of[idx] = Some(slot + 1);
    }
    chunks
        .iter()
        .enumerate()
        .map(|(rank, c)| DumpChunk {
            rank,
            chunk_id: c.chunk_id,
            synthetic: c.chunk_id.is_none(),
            doc_name: c.chunk_meta.doc_name.clone(),
            chunk_index: c.chunk_meta.chunk_index,
            section_name: c.chunk_meta.section_name.clone(),
            text: c.chunk_meta.text.clone(),
            text_hash: text_hash(&c.chunk_meta.text),
            context_text: context_text(c).to_string(),
            score: c.score,
            rerank_score: c.rerank_score,
            in_prompt: slot_of[rank].is_some(),
            prompt_slot: slot_of[rank],
        })
        .collect()
}

/// Append one record as a JSON line and flush, so a killed run keeps what it did.
pub fn append(file: &mut std::fs::File, record: &EvalDumpRecord<'_>) -> anyhow::Result<()> {
    let line = serde_json::to_string(record)?;
    writeln!(file, "{line}")?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kwaai_rag::meta_store::ChunkMeta;

    fn chunk(id: Option<i64>, text: &str) -> RetrievedChunk {
        RetrievedChunk {
            chunk_id: id,
            chunk_meta: ChunkMeta {
                doc_name: "doc".into(),
                chunk_index: 3,
                text: text.into(),
                surrounding: String::new(),
                page_num: None,
                ingested_at: String::new(),
                section_name: None,
                skip_extraction: false,
                section_note: None,
                section_type: kwaai_rag::doc_schema::SectionType::Main,
            },
            score: 0.5,
            source_kb: None,
            rerank_score: None,
        }
    }

    /// The Python scorer keys its caches on the same digest; if these drift, every
    /// cached relevance grade silently misses.
    #[test]
    fn text_hash_matches_the_shared_vectors() {
        let raw =
            include_str!("../../../../tests/kwaai-knowledge/eval2/fixtures/hash_vectors.json");
        let vectors: Vec<(String, String)> = serde_json::from_str(raw).unwrap();
        assert!(!vectors.is_empty());
        for (input, expected) in vectors {
            assert_eq!(text_hash(&input), expected, "hash of {input:?}");
        }
    }

    #[test]
    fn text_hash_ignores_whitespace_layout() {
        assert_eq!(text_hash("a  b\n c "), text_hash("a b c"));
        assert_ne!(text_hash("a b c"), text_hash("a b d"));
    }

    /// A graph fact card has no storage row; it must be recorded as synthetic with a
    /// null id, not dropped or given a made-up id.
    #[test]
    fn synthetic_chunk_serializes_with_null_id() {
        let dumped = dump_chunks(&[chunk(None, "fact card")], 24_000);
        let v = serde_json::to_value(&dumped[0]).unwrap();
        assert_eq!(v["chunk_id"], serde_json::Value::Null);
        assert_eq!(v["synthetic"], true);
    }

    /// The manifest must agree with what the prompt rendered: chunks past the budget
    /// are marked out of the prompt, and in-prompt slots are exactly 1..=n.
    #[test]
    fn manifest_marks_chunks_the_budget_dropped() {
        let big = "z".repeat(400);
        let chunks: Vec<RetrievedChunk> = (0..10).map(|i| chunk(Some(i), &big)).collect();
        let budget = 1000;
        let dumped = dump_chunks(&chunks, budget);
        let kept = context_plan(&chunks, budget);
        assert!(kept.len() < chunks.len(), "budget must drop something");

        let in_prompt: Vec<usize> = dumped
            .iter()
            .filter(|d| d.in_prompt)
            .map(|d| d.rank)
            .collect();
        let mut expected = kept.clone();
        expected.sort_unstable();
        assert_eq!(in_prompt, expected);

        let mut slots: Vec<usize> = dumped.iter().filter_map(|d| d.prompt_slot).collect();
        slots.sort_unstable();
        assert_eq!(slots, (1..=kept.len()).collect::<Vec<_>>());

        // And the rendered prompt carries exactly that many excerpts.
        let msgs = kwaai_rag::prompt::build_chat_messages("q", &chunks, &[], budget, None);
        assert!(msgs[0]
            .content
            .contains(&format!("The following {} source excerpt(s)", kept.len())));
    }
}
