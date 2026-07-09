//! Integration tests for the chunk → MetaStore → BM25 storage pipeline.
//!
//! Validates that document text flows correctly through the storage layer and
//! is retrievable. No LLM, embedder, or network required.

use kwaai_rag::{
    bm25::BM25Index,
    chunker::{chunk_id, split_text, ChunkConfig, ChunkStrategy, SurrMode},
    meta_store::{ChunkMeta, MetaStore},
};
use tempfile::TempDir;
use uuid::Uuid;

// ── helpers ───────────────────────────────────────────────────────────────────

fn default_cfg() -> ChunkConfig {
    ChunkConfig {
        chunk_size: 100,
        chunk_overlap: 10,
        min_chunk_len: 20,
        strategy: ChunkStrategy::Paragraph,
        surr_mode: SurrMode::Truncated,
    }
}

fn tenant() -> Uuid {
    Uuid::nil()
}

/// Convert a Chunk from the chunker into a ChunkMeta for MetaStore.
fn to_chunk_meta(c: &kwaai_rag::chunker::Chunk) -> ChunkMeta {
    ChunkMeta {
        doc_name: c.doc_name.clone(),
        chunk_index: c.chunk_index,
        text: c.text.clone(),
        surrounding: c.surrounding.clone(),
        page_num: c.page_num,
        ingested_at: "2024-01-01T00:00:00Z".to_string(),
        section_name: c.section_name.clone(),
        skip_extraction: c.skip_extraction,
        section_note: c.section_note.clone(),
        section_type: c.section_type.clone(),
    }
}

// ── chunk → MetaStore ─────────────────────────────────────────────────────────

#[test]
fn pipeline_chunks_stored_in_meta_store() {
    let dir = TempDir::new().unwrap();
    let meta = MetaStore::open(dir.path(), tenant()).unwrap();

    let text = "Alice went to the market. Bob followed her down the lane. \
                Carol met them at the corner of Elm Street and Oak Avenue.";
    let chunks = split_text(text, "doc.txt", &default_cfg(), None);
    assert!(!chunks.is_empty(), "expected at least one chunk");

    let metas: Vec<ChunkMeta> = chunks.iter().map(to_chunk_meta).collect();
    let ids: Vec<i64> = chunks.iter().map(|c| c.id).collect();
    meta.put_chunks(&metas, &ids).unwrap();

    let retrieved = meta.get_chunks(&ids).unwrap();
    for (i, r) in retrieved.iter().enumerate() {
        let m = r.as_ref().expect("chunk should be stored");
        assert_eq!(m.doc_name, "doc.txt");
        assert!(!m.text.is_empty(), "chunk {i} text should not be empty");
    }
}

#[test]
fn pipeline_list_docs_after_ingest() {
    let dir = TempDir::new().unwrap();
    let meta = MetaStore::open(dir.path(), tenant()).unwrap();

    let text = "The quick brown fox jumps over the lazy dog near the riverside.";
    let chunks = split_text(text, "mybook.txt", &default_cfg(), None);
    let metas: Vec<ChunkMeta> = chunks.iter().map(to_chunk_meta).collect();
    let ids: Vec<i64> = chunks.iter().map(|c| c.id).collect();
    meta.put_chunks(&metas, &ids).unwrap();

    let docs = meta.list_docs().unwrap();
    assert!(docs.contains(&"mybook.txt".to_string()), "got: {docs:?}");
}

#[test]
fn pipeline_delete_doc_removes_from_meta_store() {
    let dir = TempDir::new().unwrap();
    let meta = MetaStore::open(dir.path(), tenant()).unwrap();

    let text = "This is a sample document about ancient history and its lasting effects.";
    let chunks = split_text(text, "ancient.txt", &default_cfg(), None);
    let metas: Vec<ChunkMeta> = chunks.iter().map(to_chunk_meta).collect();
    let ids: Vec<i64> = chunks.iter().map(|c| c.id).collect();
    meta.put_chunks(&metas, &ids).unwrap();

    assert!(meta.list_docs().unwrap().contains(&"ancient.txt".to_string()));

    meta.delete_doc("ancient.txt").unwrap();
    assert!(!meta.list_docs().unwrap().contains(&"ancient.txt".to_string()));

    // The chunk records should also be gone
    let retrieved = meta.get_chunks(&ids).unwrap();
    assert!(
        retrieved.iter().all(|r| r.is_none()),
        "deleted chunks should return None"
    );
}

// ── chunk → BM25 ─────────────────────────────────────────────────────────────

#[test]
fn pipeline_bm25_indexes_chunk_text_and_returns_hit() {
    let dir = TempDir::new().unwrap();
    let index = BM25Index::open(dir.path()).unwrap();

    let text = "District Six was a vibrant community in Cape Town before forced removals.";
    let chunks = split_text(text, "d6.txt", &default_cfg(), None);
    let bm25_chunks: Vec<(i64, &str, &str)> =
        chunks.iter().map(|c| (c.id, c.doc_name.as_str(), c.text.as_str())).collect();
    index.build_from_chunks(&bm25_chunks).unwrap();

    let results = index.search("District Six", 5);
    assert!(!results.is_empty(), "BM25 search should return at least one result");
    let found_ids: Vec<i64> = results.iter().map(|(id, _)| *id).collect();
    let chunk_ids: Vec<i64> = chunks.iter().map(|c| c.id).collect();
    assert!(
        found_ids.iter().any(|id| chunk_ids.contains(id)),
        "at least one chunk id should appear in BM25 results"
    );
}

#[test]
fn pipeline_bm25_search_miss_for_absent_term() {
    let dir = TempDir::new().unwrap();
    let index = BM25Index::open(dir.path()).unwrap();

    let text = "Alice went to the park and fed the ducks by the pond.";
    let chunks = split_text(text, "park.txt", &default_cfg(), None);
    let bm25_chunks: Vec<(i64, &str, &str)> =
        chunks.iter().map(|c| (c.id, c.doc_name.as_str(), c.text.as_str())).collect();
    index.build_from_chunks(&bm25_chunks).unwrap();

    // Search for a term that doesn't appear in the text
    let results = index.search("quantum physics superposition", 5);
    assert!(results.is_empty(), "BM25 should return empty for absent term");
}

#[test]
fn pipeline_bm25_delete_doc_removes_from_index() {
    let dir = TempDir::new().unwrap();
    let index = BM25Index::open(dir.path()).unwrap();

    let text = "Walied Rassool was born in Cape Town and attended local schools.";
    let chunks = split_text(text, "walied.txt", &default_cfg(), None);
    let bm25_chunks: Vec<(i64, &str, &str)> =
        chunks.iter().map(|c| (c.id, c.doc_name.as_str(), c.text.as_str())).collect();
    index.build_from_chunks(&bm25_chunks).unwrap();

    // Confirm it's found before deletion
    assert!(!index.search("Walied", 5).is_empty(), "should be found before delete");

    index.delete_doc("walied.txt").unwrap();

    // After deletion the term should no longer appear
    let after = index.search("Walied", 5);
    assert!(after.is_empty(), "should not be found after delete_doc");
}

// ── chunk ID determinism ──────────────────────────────────────────────────────

#[test]
fn pipeline_chunk_ids_are_deterministic() {
    let text = "Roses are red, violets are blue. This is a test of deterministic chunking.";
    let chunks1 = split_text(text, "poem.txt", &default_cfg(), None);
    let chunks2 = split_text(text, "poem.txt", &default_cfg(), None);

    let ids1: Vec<i64> = chunks1.iter().map(|c| c.id).collect();
    let ids2: Vec<i64> = chunks2.iter().map(|c| c.id).collect();
    assert_eq!(ids1, ids2, "chunk IDs must be deterministic across runs");
}

#[test]
fn pipeline_chunk_id_helper_matches_split_output() {
    let text = "A short sentence about nothing in particular for testing purposes.";
    let chunks = split_text(text, "test.txt", &default_cfg(), None);
    for c in &chunks {
        let expected = chunk_id("test.txt", c.chunk_index);
        assert_eq!(c.id, expected, "chunk_id() must match id on Chunk struct for index {}", c.chunk_index);
    }
}

// ── chunk count ───────────────────────────────────────────────────────────────

#[test]
fn pipeline_large_doc_produces_multiple_chunks() {
    // ~500 words → at chunk_size=100 should produce multiple chunks
    let sentence = "The history of District Six is long and complex. ";
    let text: String = sentence.repeat(20);
    let chunks = split_text(&text, "long.txt", &default_cfg(), None);
    assert!(chunks.len() >= 3, "expected multiple chunks for long text, got {}", chunks.len());
}
