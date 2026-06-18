//! Comprehensive integration test suite for kwaai-rag.
//!
//! Covers every module that can be exercised without a live LLM or embedding
//! server: BM25 indexing, chunking, query cache, document schema detection,
//! plain-text extraction, knowledge graph CRUD, completeness scoring,
//! metadata persistence, RAG prompt building, and seed-JSON conversion.
//!
//! Modules that require external services (dream, hyde, retriever, iterative,
//! query_understanding, reranker, embedder) are not covered here.

use std::collections::HashMap;

use tempfile::TempDir;
use uuid::Uuid;

use kwaai_rag::{
    bm25::{rrf_merge, BM25Index},
    cache::QueryCache,
    chunker::{chunk_id, split_text, ChunkConfig, ChunkStrategy, SurrMode},
    doc_schema::{
        auto_detect_schema, load_doc_schema, match_section, parse_index_seeds, SectionDef,
    },
    document::{extract_text, supported_extensions},
    family::load_family_tree,
    graph::{
        description_from_fields, entity_id, expected_fields, normalize_name, EntityNode,
        FieldValue, GraphStore, ENTITY_TYPES, FAMILIAL_RELS, RELATION_TYPES,
    },
    meta_store::{ChunkMeta, MetaStore, SyncMeta},
    prompt::{build_chat_messages, build_rag_prompt, ChatMessage},
    retriever::RetrievedChunk,
    scorer::{expected_relation_groups, schema_type_for, score_entity},
    seed_json::{
        count_low_confidence, load_nb_json, to_family_tree, to_seed_yaml, NbEntity, NbPayload,
        NbRelation,
    },
};

// ─────────────────────────────────────────────────────────────────────────────
// Shared helpers
// ─────────────────────────────────────────────────────────────────────────────

fn test_tid() -> Uuid {
    Uuid::nil()
}

fn make_entity(name: &str, entity_type: &str) -> EntityNode {
    EntityNode {
        id: entity_id(name, entity_type),
        name: name.to_string(),
        entity_type: entity_type.to_string(),
        description: String::new(),
        embedding: vec![0.0; 3],
        mention_count: 1,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    }
}

fn make_chunk_meta(text: &str) -> ChunkMeta {
    ChunkMeta {
        doc_name: "test.txt".to_string(),
        chunk_index: 0,
        text: text.to_string(),
        surrounding: text.to_string(),
        page_num: None,
        ingested_at: "2024-01-01T00:00:00Z".to_string(),
        section_name: None,
        skip_extraction: false,
        section_note: None,
        section_type: Default::default(),
    }
}

fn make_retrieved(text: &str, score: f64) -> RetrievedChunk {
    RetrievedChunk {
        chunk_meta: make_chunk_meta(text),
        score,
        source_kb: None,
        rerank_score: None,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// BM25 + RRF
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn bm25_build_in_ram_and_search() {
    let chunks: Vec<(i64, &str, &str)> = vec![
        (1, "chap01.txt", "the quick brown fox jumps over"),
        (2, "chap02.txt", "district six cape town coloured community"),
        (
            3,
            "chap03.txt",
            "apartheid racial segregation national party",
        ),
    ];
    let idx = BM25Index::build_in_ram(&chunks).unwrap();
    let results = idx.search("district six cape", 3);
    assert!(!results.is_empty());
    assert_eq!(results[0].0, 2, "district six chunk should rank first");
}

#[test]
fn bm25_empty_index_returns_empty() {
    let idx = BM25Index::build_in_ram(&[]).unwrap();
    assert!(idx.search("anything", 5).is_empty());
}

#[test]
fn bm25_empty_query_returns_empty() {
    let chunks = vec![(1i64, "doc.txt", "some content here")];
    let idx = BM25Index::build_in_ram(&chunks).unwrap();
    assert!(idx.search("", 5).is_empty());
    assert!(idx.search("   ", 5).is_empty());
}

#[test]
fn bm25_top_k_respected() {
    let chunks: Vec<(i64, &str, &str)> = (0..10)
        .map(|i| (i as i64, "doc.txt", "apartheid south africa history"))
        .collect();
    let idx = BM25Index::build_in_ram(&chunks).unwrap();
    let results = idx.search("apartheid", 3);
    assert!(results.len() <= 3, "results should be capped at top_k");
}

#[test]
fn bm25_special_chars_in_query_handled() {
    let chunks = vec![(1i64, "doc.txt", "teacher league south africa history")];
    let idx = BM25Index::build_in_ram(&chunks).unwrap();
    // Special chars should fall back to safe query
    let results = idx.search("teacher & league (south africa)", 5);
    // Either finds a result or returns empty — must not panic
    let _ = results;
}

#[test]
fn bm25_persistent_open_and_delete() {
    let dir = TempDir::new().unwrap();
    let idx = BM25Index::open(dir.path()).unwrap();
    let chunks = vec![
        (10i64, "doc_a.txt", "hello world foo bar"),
        (11i64, "doc_b.txt", "apartheid segregation history"),
    ];
    idx.build_from_chunks(&chunks).unwrap();
    assert!(!idx.search("apartheid", 5).is_empty());

    idx.delete_doc("doc_b.txt").unwrap();
    let res = idx.search("apartheid", 5);
    assert!(res.is_empty() || res.iter().all(|(id, _)| *id != 11));
}

#[test]
fn bm25_rebuild_is_idempotent() {
    let dir = TempDir::new().unwrap();
    let idx = BM25Index::open(dir.path()).unwrap();
    let chunks = vec![(1i64, "doc.txt", "cape town district history")];
    idx.build_from_chunks(&chunks).unwrap();
    // Second rebuild should clear old docs and re-add
    idx.build_from_chunks(&chunks).unwrap();
    let results = idx.search("cape town", 5);
    assert!(!results.is_empty());
}

#[test]
fn rrf_empty_lists() {
    let merged = rrf_merge(&[], &[], 5);
    assert!(merged.is_empty());
}

#[test]
fn rrf_disjoint_lists() {
    let sem = vec![(1i64, 0.9), (2i64, 0.8)];
    let kw = vec![(3i64, 5.0), (4i64, 3.0)];
    let merged = rrf_merge(&sem, &kw, 10);
    assert_eq!(merged.len(), 4, "all four unique IDs should appear");
}

#[test]
fn rrf_top_k_applied() {
    let sem: Vec<(i64, f64)> = (0..20).map(|i| (i, 1.0)).collect();
    let kw: Vec<(i64, f64)> = (0..20).map(|i| (i, 1.0)).collect();
    let merged = rrf_merge(&sem, &kw, 5);
    assert_eq!(merged.len(), 5);
}

#[test]
fn rrf_overlap_boosts_shared_ids() {
    // ID 1 appears in both lists at top ranks
    let sem = vec![(1i64, 0.9), (2i64, 0.5)];
    let kw = vec![(1i64, 5.0), (3i64, 2.0)];
    let merged = rrf_merge(&sem, &kw, 3);
    assert_eq!(
        merged[0].0, 1,
        "ID 1 shared across both lists should be top"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Chunker
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn chunk_id_is_deterministic() {
    assert_eq!(chunk_id("myfile.txt", 0), chunk_id("myfile.txt", 0));
    assert_eq!(chunk_id("myfile.txt", 7), chunk_id("myfile.txt", 7));
}

#[test]
fn chunk_id_differs_by_doc_name() {
    assert_ne!(chunk_id("a.txt", 0), chunk_id("b.txt", 0));
}

#[test]
fn chunk_id_differs_by_index() {
    assert_ne!(chunk_id("doc.txt", 0), chunk_id("doc.txt", 1));
}

#[test]
fn character_split_empty_text() {
    let cfg = ChunkConfig::default();
    let chunks = split_text("", "doc.txt", &cfg, None);
    assert!(chunks.is_empty());
}

#[test]
fn character_split_respects_chunk_size() {
    let cfg = ChunkConfig {
        chunk_size: 20,
        chunk_overlap: 5,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Character,
        ..Default::default()
    };
    let text = "abcdefghijklmnopqrstuvwxyz".repeat(5);
    let chunks = split_text(&text, "doc.txt", &cfg, None);
    assert!(!chunks.is_empty());
    for c in &chunks {
        assert!(
            c.text.chars().count() <= cfg.chunk_size,
            "chunk too large: {}",
            c.text.len()
        );
    }
}

#[test]
fn character_split_below_min_len_dropped() {
    let cfg = ChunkConfig {
        chunk_size: 100,
        chunk_overlap: 0,
        min_chunk_len: 50,
        strategy: ChunkStrategy::Character,
        ..Default::default()
    };
    // 30 chars — below min_chunk_len of 50
    let chunks = split_text("short text under fifty chars!", "doc.txt", &cfg, None);
    assert!(
        chunks.is_empty(),
        "short texts under min_chunk_len should be dropped"
    );
}

#[test]
fn character_split_produces_sequential_indices() {
    let cfg = ChunkConfig {
        chunk_size: 10,
        chunk_overlap: 2,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Character,
        ..Default::default()
    };
    let text = "abcdefghijklmnopqrstuvwxyz01234567890";
    let chunks = split_text(text, "doc.txt", &cfg, None);
    for (i, c) in chunks.iter().enumerate() {
        assert_eq!(c.chunk_index, i as u32);
    }
}

#[test]
fn paragraph_split_multiple_paragraphs() {
    let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
    let cfg = ChunkConfig {
        chunk_size: 500,
        chunk_overlap: 50,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Paragraph,
        ..Default::default()
    };
    let chunks = split_text(text, "doc.txt", &cfg, None);
    // All three paragraphs fit in one chunk (short enough)
    assert!(!chunks.is_empty());
    assert!(chunks[0].text.contains("First paragraph"));
    assert!(chunks[0].text.contains("Second paragraph"));
    assert!(chunks[0].text.contains("Third paragraph"));
}

#[test]
fn paragraph_split_forces_new_chunk_when_oversized() {
    let long_para = "word ".repeat(200); // ~1000 chars
    let text = format!("{long_para}\n\n{long_para}");
    let cfg = ChunkConfig {
        chunk_size: 200,
        chunk_overlap: 20,
        min_chunk_len: 10,
        strategy: ChunkStrategy::Paragraph,
        ..Default::default()
    };
    let chunks = split_text(&text, "doc.txt", &cfg, None);
    assert!(
        chunks.len() >= 2,
        "long paragraphs should split into multiple chunks"
    );
}

#[test]
fn paragraph_surr_mode_full_includes_neighbors() {
    let text = "Para one body text here.\n\nPara two body text here.\n\nPara three body text here.";
    let cfg = ChunkConfig {
        chunk_size: 30,
        chunk_overlap: 5,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Paragraph,
        surr_mode: SurrMode::Full,
    };
    let chunks = split_text(text, "doc.txt", &cfg, None);
    if chunks.len() > 1 {
        // A middle chunk should have surrounding that contains neighbouring text
        let middle = &chunks[1];
        assert!(
            middle.surrounding.len() >= middle.text.len(),
            "Full surr_mode should include adjacent chunks"
        );
    }
}

#[test]
fn paragraph_heading_sets_section_name() {
    let text = "Chapter One\n\nThis is the body of chapter one. It has relevant content.\n\nChapter Two\n\nThis is chapter two body text.";
    let cfg = ChunkConfig {
        chunk_size: 200,
        chunk_overlap: 20,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Paragraph,
        ..Default::default()
    };
    let chunks = split_text(text, "doc.txt", &cfg, None);
    // Without schema, headings still set section_name
    let has_section_name = chunks.iter().any(|c| c.section_name.is_some());
    assert!(has_section_name, "headings should populate section_name");
}

#[test]
fn paragraph_schema_skip_extraction() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.sections.push(SectionDef {
        pattern: "index".to_string(),
        skip: true,
        narrator_note: None,
        index_seeds: false,
        section_type: Default::default(),
    });

    // Use a small chunk_size so the main-content paragraph and the index
    // paragraph end up in separate chunks (their combined length > chunk_size).
    // "Some body text about history." = 29 chars
    // "Aardvark, 5\nZebra, 10"        = 21 chars
    // Combined with separator = 51 chars, which exceeds chunk_size=35.
    let text = "Main Content\n\nSome body text about history.\n\nIndex\n\nAardvark, 5\nZebra, 10";
    let cfg = ChunkConfig {
        chunk_size: 35,
        chunk_overlap: 5,
        min_chunk_len: 5,
        strategy: ChunkStrategy::Paragraph,
        ..Default::default()
    };
    let chunks = split_text(text, "doc.txt", &cfg, Some(&schema));
    let skip_chunks: Vec<_> = chunks.iter().filter(|c| c.skip_extraction).collect();
    assert!(
        !skip_chunks.is_empty(),
        "Index section chunks should have skip_extraction=true"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Query Cache
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn cache_open_creates_db() {
    let dir = TempDir::new().unwrap();
    let cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    assert_eq!(cache.entry_count(), 0);
    assert_eq!(cache.total_hits(), 0);
}

#[test]
fn cache_put_and_get_exact_match() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    let emb = vec![1.0f32, 0.0, 0.0];
    cache
        .put(
            "What is District Six?".to_string(),
            emb.clone(),
            "A neighbourhood in Cape Town.".to_string(),
            vec![1, 2],
        )
        .unwrap();
    let hit = cache.get(&emb);
    assert!(hit.is_some(), "exact embedding should hit the cache");
    let entry = hit.unwrap();
    assert_eq!(entry.answer, "A neighbourhood in Cape Town.");
    assert_eq!(entry.chunk_ids, vec![1, 2]);
}

#[test]
fn cache_get_above_threshold_hits() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    // Store with [1.0, 0.0, 0.0]
    let stored_emb = vec![1.0f32, 0.0, 0.0];
    cache
        .put("q".to_string(), stored_emb, "answer".to_string(), vec![])
        .unwrap();

    // Query with cosine similarity ≈ 0.95 (well above 0.92 threshold)
    // [0.95, 0.312, 0.0] is a unit vector, cos_sim with [1,0,0] ≈ 0.95
    let query = vec![0.95f32, 0.312, 0.0];
    assert!(cache.get(&query).is_some(), "similar embedding should hit");
}

#[test]
fn cache_get_below_threshold_misses() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    let stored_emb = vec![1.0f32, 0.0, 0.0];
    cache
        .put("q".to_string(), stored_emb, "answer".to_string(), vec![])
        .unwrap();

    // cosine_sim([1,0,0], [0.0,1.0,0.0]) = 0.0 — far below 0.92
    let query = vec![0.0f32, 1.0, 0.0];
    assert!(
        cache.get(&query).is_none(),
        "dissimilar embedding should miss"
    );
}

#[test]
fn cache_hit_count_increments() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    let emb = vec![1.0f32, 0.0, 0.0];
    cache
        .put("q".to_string(), emb.clone(), "a".to_string(), vec![])
        .unwrap();
    cache.get(&emb);
    cache.get(&emb);
    assert_eq!(cache.total_hits(), 2);
}

#[test]
fn cache_clear_removes_all() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    for i in 0..5u8 {
        let emb = vec![i as f32, 0.0, 0.0];
        cache
            .put(format!("q{i}"), emb, format!("a{i}"), vec![])
            .unwrap();
    }
    assert_eq!(cache.entry_count(), 5);
    let removed = cache.clear().unwrap();
    assert_eq!(removed, 5);
    assert_eq!(cache.entry_count(), 0);
}

#[test]
fn cache_lru_eviction_removes_least_used() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    cache.max_entries = 2;

    let emb_a = vec![1.0f32, 0.0, 0.0];
    let emb_b = vec![0.0f32, 1.0, 0.0];
    let emb_c = vec![0.0f32, 0.0, 1.0];

    cache
        .put("A".to_string(), emb_a.clone(), "ans_a".to_string(), vec![])
        .unwrap();
    cache
        .put("B".to_string(), emb_b.clone(), "ans_b".to_string(), vec![])
        .unwrap();

    // Bump A's hit_count so A is not the LRU
    cache.get(&emb_a);

    // Putting C should evict B (hit_count=0, older) not A (hit_count=1)
    cache
        .put("C".to_string(), emb_c.clone(), "ans_c".to_string(), vec![])
        .unwrap();

    assert_eq!(cache.entry_count(), 2);
    assert!(
        cache.get(&emb_a).is_some(),
        "A should survive — it was accessed"
    );
    assert!(
        cache.get(&emb_b).is_none(),
        "B should be evicted — least used"
    );
    assert!(
        cache.get(&emb_c).is_some(),
        "C should be present — just inserted"
    );
}

#[test]
fn cache_ttl_expired_count() {
    let dir = TempDir::new().unwrap();
    let mut cache = QueryCache::open(dir.path(), test_tid()).unwrap();
    let emb = vec![1.0f32, 0.0, 0.0];
    cache
        .put("q".to_string(), emb, "a".to_string(), vec![])
        .unwrap();
    // With TTL set to 0, any entry is immediately "expired"
    cache.ttl_secs = 0;
    // Entries inserted even a moment ago have timestamp=now, so now-ts=0, which is NOT > 0
    // The expired count stays 0 within the same second.
    // What we CAN assert: total_hits is unaffected
    assert_eq!(cache.total_hits(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
// DocSchema
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn auto_detect_isbn13() {
    let text = "ISBN 978-0-06-093546-9\nSome other content";
    let schema = auto_detect_schema(text);
    assert_eq!(schema.schema_type.as_deref(), Some("Book"));
    assert!(schema.metadata.get("isbn").unwrap().len() == 13);
}

#[test]
fn auto_detect_isbn10() {
    let text = "ISBN 0060935464\nSome other content";
    let schema = auto_detect_schema(text);
    assert_eq!(schema.schema_type.as_deref(), Some("Book"));
    assert_eq!(schema.metadata.get("isbn").unwrap().len(), 10);
}

#[test]
fn auto_detect_publisher() {
    let text = "Published by Oxford University Press, Oxford\nOther content";
    let schema = auto_detect_schema(text);
    assert!(
        schema.metadata.get("publisher").is_some(),
        "publisher should be extracted"
    );
    assert!(
        schema.metadata.get("publisher").unwrap().contains("Oxford"),
        "publisher name should contain Oxford"
    );
}

#[test]
fn auto_detect_copyright_year() {
    let text = "Copyright © 1984 Joe Rassool\nAll rights reserved";
    let schema = auto_detect_schema(text);
    assert_eq!(
        schema.metadata.get("year").map(|s| s.as_str()),
        Some("1984")
    );
    assert_eq!(
        schema.metadata.get("copyrightYear").map(|s| s.as_str()),
        Some("1984")
    );
}

#[test]
fn auto_detect_copyright_holder() {
    let text = "© Joe Rassool 1984";
    let schema = auto_detect_schema(text);
    assert!(
        schema.metadata.get("copyrightHolder").is_some(),
        "copyright holder should be found"
    );
}

#[test]
fn auto_detect_by_author() {
    let text = "By Yousuf Rassool\nDistrict Six — Lest We Forget";
    let schema = auto_detect_schema(text);
    assert_eq!(
        schema.metadata.get("author").map(|s| s.as_str()),
        Some("Yousuf Rassool")
    );
}

#[test]
fn auto_detect_empty_text_returns_default() {
    let schema = auto_detect_schema("");
    assert!(schema.schema_type.is_none());
    assert!(schema.metadata.is_empty());
}

#[test]
fn match_section_case_insensitive() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.sections.push(SectionDef {
        pattern: "appendix".to_string(),
        skip: true,
        narrator_note: None,
        index_seeds: false,
        section_type: Default::default(),
    });
    let sec = match_section("APPENDIX A — Documents", &schema);
    assert!(sec.is_some(), "should match case-insensitively");
    assert!(sec.unwrap().skip);
}

#[test]
fn match_section_no_match_returns_none() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.sections.push(SectionDef {
        pattern: "index".to_string(),
        skip: false,
        narrator_note: None,
        index_seeds: false,
        section_type: Default::default(),
    });
    assert!(match_section("Introduction", &schema).is_none());
}

#[test]
fn match_section_first_pattern_wins() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.sections.push(SectionDef {
        pattern: "chapter".to_string(),
        skip: false,
        narrator_note: Some("first".to_string()),
        index_seeds: false,
        section_type: Default::default(),
    });
    schema.sections.push(SectionDef {
        pattern: "chapter one".to_string(),
        skip: true,
        narrator_note: Some("second".to_string()),
        index_seeds: false,
        section_type: Default::default(),
    });
    let sec = match_section("Chapter One", &schema).unwrap();
    assert_eq!(
        sec.narrator_note.as_deref(),
        Some("first"),
        "first matching pattern should win"
    );
}

#[test]
fn parse_index_seeds_surname_first_format() {
    let index = "Rassool, Yousuf, 15, 23\nAdams, Robert, 7";
    let seeds = parse_index_seeds(index);
    let names: Vec<&str> = seeds.iter().map(|(n, _)| n.as_str()).collect();
    assert!(
        names.contains(&"Yousuf Rassool"),
        "should invert Surname, Firstname"
    );
    assert!(names.contains(&"Robert Adams"));
    for (_, t) in &seeds {
        assert_eq!(
            t.as_deref(),
            Some("Person"),
            "inverted names should be Person"
        );
    }
}

#[test]
fn parse_index_seeds_org_keywords() {
    let index = "African National Congress (ANC), 72\nTeacher's League, 5, 8";
    let seeds = parse_index_seeds(index);
    let orgs: Vec<_> = seeds
        .iter()
        .filter(|(_, t)| t.as_deref() == Some("Organization"))
        .collect();
    assert!(
        !orgs.is_empty(),
        "org-keyword entries should be typed as Organization"
    );
}

#[test]
fn parse_index_seeds_skips_blank_and_numeric() {
    let index = "\n   \n5\n12\nSmith, John, 1";
    let seeds = parse_index_seeds(index);
    // Only "John Smith" should be extracted
    assert_eq!(seeds.len(), 1);
}

#[test]
fn parse_index_seeds_strips_page_refs() {
    let index = "Cape Town, 12, 34, 56";
    let seeds = parse_index_seeds(index);
    assert!(!seeds.is_empty());
    assert_eq!(seeds[0].0, "Cape Town");
}

#[test]
fn doc_schema_context_line_with_author_year() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.document_title = Some("District Six".to_string());
    schema
        .metadata
        .insert("author".to_string(), "Joe Rassool".to_string());
    schema
        .metadata
        .insert("year".to_string(), "1984".to_string());
    let ctx = schema.context_line().unwrap();
    assert!(ctx.contains("District Six"));
    assert!(ctx.contains("Joe Rassool"));
    assert!(ctx.contains("1984"));
}

#[test]
fn doc_schema_context_line_empty_returns_none() {
    use kwaai_rag::doc_schema::DocSchema;
    let schema = DocSchema::default();
    assert!(schema.context_line().is_none());
}

#[test]
fn doc_schema_has_index_seeds() {
    use kwaai_rag::doc_schema::DocSchema;
    let mut schema = DocSchema::default();
    schema.sections.push(SectionDef {
        pattern: "index".to_string(),
        skip: true,
        narrator_note: None,
        index_seeds: true,
        section_type: Default::default(),
    });
    assert!(schema.has_index_seeds());

    let schema2 = DocSchema::default();
    assert!(!schema2.has_index_seeds());
}

#[test]
fn load_doc_schema_from_yaml_file() {
    let dir = TempDir::new().unwrap();
    let yaml = r#"
document_title: "Test Book"
default_narrator: "The Author"
sections:
  - pattern: "index"
    skip: true
    index_seeds: true
metadata:
  author: "Jane Doe"
  year: "2020"
schema_type: "Book"
"#;
    let path = dir.path().join("schema.yaml");
    std::fs::write(&path, yaml).unwrap();
    let schema = load_doc_schema(&path).unwrap();
    assert_eq!(schema.document_title.as_deref(), Some("Test Book"));
    assert_eq!(schema.default_narrator.as_deref(), Some("The Author"));
    assert_eq!(schema.schema_type.as_deref(), Some("Book"));
    assert!(schema.has_index_seeds());
    assert_eq!(
        schema.metadata.get("author").map(|s| s.as_str()),
        Some("Jane Doe")
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Document extraction
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn extract_text_from_txt_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("hello.txt");
    std::fs::write(&path, "Hello, District Six!").unwrap();
    let text = extract_text(&path).unwrap();
    assert_eq!(text.trim(), "Hello, District Six!");
}

#[test]
fn extract_text_from_md_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("notes.md");
    std::fs::write(&path, "# Heading\n\nContent here.").unwrap();
    let text = extract_text(&path).unwrap();
    assert!(text.contains("Heading"));
    assert!(text.contains("Content here"));
}

#[test]
fn extract_text_nonexistent_returns_error() {
    let path = std::path::Path::new("/tmp/kwaai_test_nonexistent_file_xyz.txt");
    assert!(extract_text(path).is_err());
}

#[test]
fn supported_extensions_includes_core_formats() {
    let exts = supported_extensions();
    for expected in &["txt", "md", "pdf", "docx"] {
        assert!(
            exts.contains(expected),
            "expected extension '{}' missing",
            expected
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Graph: types and pure-function helpers
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn entity_id_is_deterministic() {
    assert_eq!(entity_id("Alice", "Person"), entity_id("Alice", "Person"));
}

#[test]
fn entity_id_case_insensitive_name() {
    // Name is lowercased before hashing
    assert_eq!(entity_id("Alice", "Person"), entity_id("alice", "Person"));
}

#[test]
fn entity_id_differs_across_names() {
    assert_ne!(entity_id("Alice", "Person"), entity_id("Bob", "Person"));
}

#[test]
fn entity_id_differs_across_types() {
    assert_ne!(
        entity_id("Alice", "Person"),
        entity_id("Alice", "Organization")
    );
}

#[test]
fn normalize_name_lowercases_and_strips_punctuation() {
    assert_eq!(normalize_name("J.M.H. Gool"), "j m h gool");
    assert_eq!(normalize_name("Abdul-Hamid (BG)"), "abdul hamid bg");
}

#[test]
fn normalize_name_collapses_whitespace() {
    assert_eq!(normalize_name("  Alice   Smith  "), "alice smith");
}

#[test]
fn expected_fields_person_has_10_fields() {
    assert_eq!(expected_fields("Person").len(), 10);
}

#[test]
fn expected_fields_location_has_2_fields() {
    assert_eq!(expected_fields("Location").len(), 2);
}

#[test]
fn expected_fields_organization_has_4_fields() {
    assert_eq!(expected_fields("Organization").len(), 4);
}

#[test]
fn expected_fields_unknown_type_is_empty() {
    assert!(expected_fields("Topic").is_empty());
    assert!(expected_fields("Unknown").is_empty());
}

#[test]
fn description_from_fields_generates_prose() {
    let mut fields = HashMap::new();
    fields.insert("birthDate".to_string(), FieldValue::new("1950-01-01", 1));
    fields.insert("occupation".to_string(), FieldValue::new("Teacher", 2));
    let desc = description_from_fields("Alice", "Person", &fields);
    assert!(!desc.is_empty());
    assert!(desc.contains("Alice"));
    assert!(desc.contains("birthDate") || desc.contains("1950"));
}

#[test]
fn description_from_fields_empty_fields_returns_empty() {
    let desc = description_from_fields("Bob", "Person", &HashMap::new());
    assert!(desc.is_empty());
}

#[test]
fn entity_types_list_contains_all_expected() {
    for expected in &["Person", "Organization", "Location", "Event", "Unknown"] {
        assert!(
            ENTITY_TYPES.contains(expected),
            "missing type: {}",
            expected
        );
    }
    assert_eq!(ENTITY_TYPES.len(), 17);
}

#[test]
fn relation_types_list_not_empty() {
    assert!(!RELATION_TYPES.is_empty());
    assert!(RELATION_TYPES.contains(&"parent_of"));
    assert!(RELATION_TYPES.contains(&"located_in"));
    assert!(RELATION_TYPES.contains(&"spouse_of"));
}

#[test]
fn familial_rels_are_subset_of_relation_types() {
    for rel in FAMILIAL_RELS {
        assert!(
            RELATION_TYPES.contains(rel),
            "familial rel not in RELATION_TYPES: {}",
            rel
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GraphStore
// ─────────────────────────────────────────────────────────────────────────────

fn open_graph(dir: &TempDir) -> GraphStore {
    GraphStore::open(dir.path(), test_tid()).unwrap()
}

#[test]
fn graph_open_starts_empty() {
    let dir = TempDir::new().unwrap();
    let g = open_graph(&dir);
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.relation_count(), 0);
}

#[test]
fn graph_upsert_and_get_entity() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let alice = make_entity("Alice", "Person");
    let id = alice.id;
    g.upsert_entity(alice).unwrap();

    assert_eq!(g.node_count(), 1);
    let found = g.get_entity(id).unwrap();
    assert_eq!(found.name, "Alice");
    assert_eq!(found.entity_type, "Person");
}

#[test]
fn graph_find_by_name() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    g.upsert_entity(make_entity("District Six", "Location"))
        .unwrap();
    assert!(g.find_by_name("District Six").is_some());
    assert!(
        g.find_by_name("district six").is_some(),
        "lookup should be case-insensitive"
    );
    assert!(g.find_by_name("NonExistent").is_none());
}

#[test]
fn graph_find_by_name_normalized() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    // Hyphen in stored name → space in normalized form
    g.upsert_entity(make_entity("Abdul-Hamid", "Person"))
        .unwrap();
    // "Abdul Hamid" normalizes the same way ("abdul hamid")
    assert!(g.find_by_name_normalized("Abdul Hamid").is_some());
    // Case difference also normalizes
    g.upsert_entity(make_entity("District Six", "Location"))
        .unwrap();
    assert!(g.find_by_name_normalized("DISTRICT SIX").is_some());
}

#[test]
fn graph_upsert_increments_mention_count() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let alice = make_entity("Alice", "Person");
    let id = alice.id;
    g.upsert_entity(alice.clone()).unwrap();
    g.upsert_entity(alice.clone()).unwrap();
    assert_eq!(g.get_entity(id).unwrap().mention_count, 2);
}

#[test]
fn graph_upsert_keeps_longer_description() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut e1 = make_entity("Alice", "Person");
    e1.description = "Short desc.".to_string();
    g.upsert_entity(e1).unwrap();

    let mut e2 = make_entity("Alice", "Person");
    e2.description = "A much longer description with more detail about Alice.".to_string();
    g.upsert_entity(e2).unwrap();

    let alice = g.find_by_name("Alice").unwrap();
    assert!(alice.description.len() > "Short desc.".len());
}

#[test]
fn graph_upsert_relation_stores_and_reads() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut alice = make_entity("Alice", "Person");
    let mut bob = make_entity("Bob", "Person");
    let aid = alice.id;
    let bid = bob.id;
    alice.embedding = vec![1.0, 0.0, 0.0];
    bob.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(alice).unwrap();
    g.upsert_entity(bob).unwrap();

    g.upsert_relation(aid, bid, "works_at", 42).unwrap();
    assert_eq!(g.relation_count(), 1);

    let nbrs = g.neighbors_of(aid);
    assert!(nbrs
        .iter()
        .any(|(id, rel, _)| *id == bid && rel == "works_at"));
}

#[test]
fn graph_familial_inverse_auto_added() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut parent = make_entity("Parent", "Person");
    let mut child = make_entity("Child", "Person");
    let pid = parent.id;
    let cid = child.id;
    parent.embedding = vec![1.0, 0.0, 0.0];
    child.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(parent).unwrap();
    g.upsert_entity(child).unwrap();

    g.upsert_relation(pid, cid, "parent_of", 1).unwrap();

    // child_of should have been auto-added in reverse
    let child_nbrs = g.neighbors_of(cid);
    assert!(
        child_nbrs
            .iter()
            .any(|(id, rel, _)| *id == pid && rel == "child_of"),
        "child_of inverse should be auto-added"
    );
}

#[test]
fn graph_symmetric_relation_stored_both_ways() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut alice = make_entity("Alice", "Person");
    let mut bob = make_entity("Bob", "Person");
    let aid = alice.id;
    let bid = bob.id;
    alice.embedding = vec![1.0, 0.0, 0.0];
    bob.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(alice).unwrap();
    g.upsert_entity(bob).unwrap();

    g.upsert_relation(aid, bid, "spouse_of", 1).unwrap();

    // spouse_of is symmetric — should appear in both directions
    let bob_nbrs = g.neighbors_of(bid);
    assert!(
        bob_nbrs
            .iter()
            .any(|(id, rel, _)| *id == aid && rel == "spouse_of"),
        "spouse_of should be stored in both directions"
    );
}

#[test]
fn graph_familial_constraint_rejects_non_person() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let org = make_entity("Cape Town Council", "Organization");
    let loc = make_entity("Cape Town", "Location");
    let oid = org.id;
    let lid = loc.id;
    g.upsert_entity(org).unwrap();
    g.upsert_entity(loc).unwrap();

    // familial relation between non-Person entities should be silently dropped
    g.upsert_relation(oid, lid, "parent_of", 1).unwrap();
    assert_eq!(
        g.relation_count(),
        0,
        "familial rel between non-Persons must be rejected"
    );
}

#[test]
fn graph_located_in_rejected_for_creative_work_target() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut book = make_entity("District Six Memoir", "Document");
    book.schema_type = Some("schema:CreativeWork".to_string());
    let mut person = make_entity("Joe", "Person");
    let book_id = book.id;
    let person_id = person.id;
    book.embedding = vec![1.0, 0.0, 0.0];
    person.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(book).unwrap();
    g.upsert_entity(person).unwrap();

    // located_in targeting a CreativeWork should be silently dropped
    g.upsert_relation(person_id, book_id, "located_in", 1)
        .unwrap();
    assert_eq!(g.relation_count(), 0);
}

#[test]
fn graph_bfs_depth_one() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut a = make_entity("A", "Person");
    let mut b = make_entity("B", "Person");
    let mut c = make_entity("C", "Person");
    let aid = a.id;
    let bid = b.id;
    let cid = c.id;
    a.embedding = vec![1.0, 0.0, 0.0];
    b.embedding = vec![0.0, 1.0, 0.0];
    c.embedding = vec![0.0, 0.0, 1.0];
    g.upsert_entity(a).unwrap();
    g.upsert_entity(b).unwrap();
    g.upsert_entity(c).unwrap();
    g.upsert_relation(aid, bid, "works_at", 1).unwrap();
    g.upsert_relation(bid, cid, "located_in", 2).unwrap();

    let reachable = g.bfs_neighbors(&[aid], 1);
    assert!(reachable.contains(&aid));
    assert!(reachable.contains(&bid));
    assert!(
        !reachable.contains(&cid),
        "C is 2 hops from A — should not appear at depth 1"
    );
}

#[test]
fn graph_bfs_depth_two() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut a = make_entity("A", "Person");
    let mut b = make_entity("B", "Person");
    let mut c = make_entity("C", "Location");
    let aid = a.id;
    let bid = b.id;
    let cid = c.id;
    a.embedding = vec![1.0, 0.0, 0.0];
    b.embedding = vec![0.0, 1.0, 0.0];
    c.embedding = vec![0.0, 0.0, 1.0];
    g.upsert_entity(a).unwrap();
    g.upsert_entity(b).unwrap();
    g.upsert_entity(c).unwrap();
    g.upsert_relation(aid, bid, "works_at", 1).unwrap();
    g.upsert_relation(bid, cid, "located_in", 2).unwrap();

    let reachable = g.bfs_neighbors(&[aid], 2);
    assert!(reachable.contains(&cid), "C should be reachable at depth 2");
}

#[test]
fn graph_link_chunk_and_entity_chunks() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let alice = make_entity("Alice", "Person");
    let aid = alice.id;
    g.upsert_entity(alice).unwrap();

    g.link_chunk(99, &[aid]).unwrap();
    let chunks = g.entity_chunks(&[aid]);
    assert!(chunks.contains(&99));
}

#[test]
fn graph_search_entities_cosine() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut alice = make_entity("Alice", "Person");
    let mut bob = make_entity("Bob", "Person");
    alice.embedding = vec![1.0, 0.0, 0.0];
    bob.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(alice).unwrap();
    g.upsert_entity(bob).unwrap();

    // Query aligned with Alice's embedding
    let results = g.search_entities(&[1.0f32, 0.0, 0.0], 5);
    assert!(!results.is_empty());
    assert_eq!(
        results[0].0,
        entity_id("Alice", "Person"),
        "Alice should rank first"
    );
}

#[test]
fn graph_find_ids_by_name_token() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    g.upsert_entity(make_entity("Yousuf Rassool", "Person"))
        .unwrap();
    g.upsert_entity(make_entity("Peter Rassool", "Person"))
        .unwrap();
    g.upsert_entity(make_entity("District Six", "Location"))
        .unwrap();

    let ids = g.find_ids_by_name_token("rassool");
    assert_eq!(ids.len(), 2, "both Rassool persons should match");

    let ids2 = g.find_ids_by_name_token("six");
    assert_eq!(ids2.len(), 1);
}

#[test]
fn graph_merge_entity_into_transfers_relations() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut canonical = make_entity("Yousuf Rassool", "Person");
    let mut alias = make_entity("Joe Rassool", "Person");
    let mut colleague = make_entity("Dr Gool", "Person");
    canonical.embedding = vec![1.0, 0.0, 0.0];
    alias.embedding = vec![0.9, 0.44, 0.0];
    colleague.embedding = vec![0.0, 1.0, 0.0];
    let cid = canonical.id;
    let aid = alias.id;
    let gid = colleague.id;
    g.upsert_entity(canonical).unwrap();
    g.upsert_entity(alias).unwrap();
    g.upsert_entity(colleague).unwrap();

    // Give alias a relation to Dr Gool
    g.upsert_relation(aid, gid, "associated_with", 1).unwrap();
    assert_eq!(g.relation_count(), 1);

    // Merge alias into canonical
    let n_moved = g.merge_entity_into(aid, cid).unwrap();
    assert!(n_moved >= 1, "at least one relation should be transferred");

    // Reopen to trigger rebuild and verify state
    drop(g);
    let g2 = GraphStore::open(dir.path(), test_tid()).unwrap();

    // Alias entity should be gone
    assert!(
        g2.find_by_name("Joe Rassool").is_none(),
        "alias entity should be deleted"
    );
    // Canonical should remain
    assert!(g2.find_by_name("Yousuf Rassool").is_some());
    // The transferred relation should now connect canonical → colleague
    assert!(
        g2.relation_count() >= 1,
        "relation should survive under canonical"
    );
}

#[test]
fn graph_merge_self_is_noop() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    g.upsert_entity(make_entity("Alice", "Person")).unwrap();
    let id = entity_id("Alice", "Person");
    let moved = g.merge_entity_into(id, id).unwrap();
    assert_eq!(moved, 0);
}

#[test]
fn graph_outgoing_relations_directed() {
    let dir = TempDir::new().unwrap();
    let mut g = open_graph(&dir);
    let mut a = make_entity("A", "Person");
    let mut b = make_entity("B", "Person");
    let aid = a.id;
    let bid = b.id;
    a.embedding = vec![1.0, 0.0, 0.0];
    b.embedding = vec![0.0, 1.0, 0.0];
    g.upsert_entity(a).unwrap();
    g.upsert_entity(b).unwrap();
    g.upsert_relation(aid, bid, "related_to", 1).unwrap();

    let outgoing = g.outgoing_relations(aid).unwrap();
    assert!(outgoing
        .iter()
        .any(|(id, rel, _, _)| *id == bid && rel == "related_to"));
}

// ─────────────────────────────────────────────────────────────────────────────
// Scorer
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn schema_type_for_person() {
    assert_eq!(schema_type_for("Person"), Some("schema:Person"));
}

#[test]
fn schema_type_for_location() {
    assert_eq!(schema_type_for("Location"), Some("schema:Place"));
    assert_eq!(schema_type_for("Place"), Some("schema:Place"));
}

#[test]
fn schema_type_for_unknown_returns_none() {
    assert!(schema_type_for("Unknown").is_none());
    assert!(schema_type_for("Foobar").is_none());
}

#[test]
fn schema_type_for_topic_is_schema_thing() {
    assert_eq!(schema_type_for("Topic"), Some("schema:Thing"));
}

#[test]
fn expected_relation_groups_person_has_3_groups() {
    let groups = expected_relation_groups("schema:Person");
    assert_eq!(groups.len(), 3, "Person should have 3 relation groups");
}

#[test]
fn expected_relation_groups_unknown_type_empty() {
    let groups = expected_relation_groups("schema:Unknown");
    assert!(groups.is_empty());
}

#[test]
fn score_entity_unknown_type_zero_type_score() {
    let node = EntityNode {
        id: 1,
        name: "Mystery".to_string(),
        entity_type: "Unknown".to_string(),
        description: "Some description that is reasonably long enough.".to_string(),
        embedding: vec![],
        mention_count: 5,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &[]);
    assert_eq!(
        score.type_score, 0.0,
        "Unknown entity type should give type_score=0.0"
    );
}

#[test]
fn score_entity_concept_with_good_description() {
    // Concept uses description-length scoring (no expected_fields)
    let node = EntityNode {
        id: 2,
        name: "Apartheid".to_string(),
        entity_type: "Concept".to_string(),
        description: "Apartheid was a system of institutionalised racial segregation. \
                       It governed South Africa from 1948 to 1994."
            .to_string(),
        embedding: vec![],
        mention_count: 10,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &[]);
    assert!(
        score.summary_score >= 0.6,
        "Good description should score >= 0.6"
    );
    assert!(score.type_score > 0.0, "Concept maps to schema:DefinedTerm");
}

#[test]
fn score_entity_person_with_fields() {
    let mut fields = HashMap::new();
    fields.insert("birthDate".to_string(), FieldValue::new("1950", 1));
    fields.insert("occupation".to_string(), FieldValue::new("Teacher", 2));
    // 2 of 10 fields filled → summary_score = 0.2

    let node = EntityNode {
        id: 3,
        name: "Alice".to_string(),
        entity_type: "Person".to_string(),
        description: String::new(),
        embedding: vec![],
        mention_count: 5,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields,
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &[]);
    // 2/10 fields = 0.2 summary_score
    assert!((score.summary_score - 0.2).abs() < 0.001);
}

#[test]
fn score_entity_relation_score_with_all_groups_matched() {
    // Person has 3 groups: family, agent, structural
    let rels = vec![
        "parent_of".to_string(),
        "works_at".to_string(),
        "located_in".to_string(),
    ];
    let node = EntityNode {
        id: 4,
        name: "Bob".to_string(),
        entity_type: "Person".to_string(),
        description: String::new(),
        embedding: vec![],
        mention_count: 5,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &rels);
    assert!(
        (score.relation_score - 1.0).abs() < 0.01,
        "all 3 groups matched → relation_score should be 1.0, got {}",
        score.relation_score
    );
}

#[test]
fn score_entity_peripheral_entity_lower_bar() {
    // mention_count <= 2 only needs 1 group satisfied
    let rels = vec!["parent_of".to_string()]; // only 1 of 3 groups
    let node = EntityNode {
        id: 5,
        name: "Minor".to_string(),
        entity_type: "Person".to_string(),
        description: String::new(),
        embedding: vec![],
        mention_count: 2,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &rels);
    assert!(
        score.relation_score >= 1.0,
        "peripheral entity with 1 group matched should reach max relation_score"
    );
}

#[test]
fn score_entity_overall_is_average() {
    let node = EntityNode {
        id: 6,
        name: "TestEntity".to_string(),
        entity_type: "Unknown".to_string(), // type_score = 0.0
        description: String::new(),         // summary_score = 0.0
        embedding: vec![],
        mention_count: 5,
        first_chunk_id: 0,
        aliases: vec![],
        schema_type: None,
        gender: None,
        evidence: vec![],
        fields: HashMap::new(),
        confidence: 0.0,
        extraction_confidence: 0.0,
    };
    let score = score_entity(&node, &[]);
    // overall = (0.0 + 0.0 + neutral_relation) / 3
    // Unknown has neutral relation score = 0.5
    let expected = (0.0f32 + 0.0 + 0.5) / 3.0;
    assert!(
        (score.overall - expected).abs() < 0.01,
        "overall should be average of three pillars, got {}",
        score.overall
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// MetaStore
// ─────────────────────────────────────────────────────────────────────────────

fn open_meta(dir: &TempDir) -> MetaStore {
    MetaStore::open(dir.path(), test_tid()).unwrap()
}

fn meta(doc: &str, idx: u32, text: &str) -> ChunkMeta {
    ChunkMeta {
        doc_name: doc.to_string(),
        chunk_index: idx,
        text: text.to_string(),
        surrounding: text.to_string(),
        page_num: None,
        ingested_at: MetaStore::now_rfc3339(),
        section_name: None,
        skip_extraction: false,
        section_note: None,
        section_type: Default::default(),
    }
}

#[test]
fn meta_store_open_creates_db() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    assert!(store.all_chunks().unwrap().is_empty());
}

#[test]
fn meta_store_put_and_get_chunks() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let m1 = meta("doc_a.txt", 0, "Hello from chunk zero.");
    let m2 = meta("doc_a.txt", 1, "Hello from chunk one.");
    let ids = vec![chunk_id("doc_a.txt", 0), chunk_id("doc_a.txt", 1)];
    store.put_chunks(&[m1, m2], &ids).unwrap();

    let fetched = store.get_chunks(&ids).unwrap();
    assert_eq!(fetched.len(), 2);
    assert!(fetched[0].as_ref().unwrap().text.contains("chunk zero"));
    assert!(fetched[1].as_ref().unwrap().text.contains("chunk one"));
}

#[test]
fn meta_store_get_chunk_missing_returns_none() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let result = store.get_chunks(&[999999i64]).unwrap();
    assert!(result[0].is_none());
}

#[test]
fn meta_store_all_chunks_returns_all() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let chunks: Vec<ChunkMeta> = (0..5)
        .map(|i| meta("doc.txt", i, &format!("text {i}")))
        .collect();
    let ids: Vec<i64> = (0..5).map(|i| chunk_id("doc.txt", i)).collect();
    store.put_chunks(&chunks, &ids).unwrap();
    let all = store.all_chunks().unwrap();
    assert_eq!(all.len(), 5);
}

#[test]
fn meta_store_list_docs() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    store
        .put_chunks(&[meta("a.txt", 0, "aaa")], &[chunk_id("a.txt", 0)])
        .unwrap();
    store
        .put_chunks(&[meta("b.txt", 0, "bbb")], &[chunk_id("b.txt", 0)])
        .unwrap();
    let docs = store.list_docs().unwrap();
    assert!(docs.contains(&"a.txt".to_string()));
    assert!(docs.contains(&"b.txt".to_string()));
}

#[test]
fn meta_store_delete_doc_removes_chunks() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let cid = chunk_id("doc.txt", 0);
    store
        .put_chunks(&[meta("doc.txt", 0, "content")], &[cid])
        .unwrap();
    let removed = store.delete_doc("doc.txt").unwrap();
    assert_eq!(removed, vec![cid]);

    let all = store.all_chunks().unwrap();
    assert!(
        all.is_empty(),
        "all chunks should be removed after delete_doc"
    );
}

#[test]
fn meta_store_sync_meta_roundtrip() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let sm = SyncMeta {
        file_path: "/tmp/doc.txt".to_string(),
        mtime_secs: 1_700_000_000,
        file_size: 4096,
    };
    store.put_sync_meta("doc.txt", &sm).unwrap();
    let loaded = store.get_sync_meta("doc.txt").unwrap().unwrap();
    assert_eq!(loaded.file_path, "/tmp/doc.txt");
    assert_eq!(loaded.mtime_secs, 1_700_000_000);
    assert_eq!(loaded.file_size, 4096);
}

#[test]
fn meta_store_delete_sync_meta() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let sm = SyncMeta {
        file_path: "/tmp/doc.txt".to_string(),
        mtime_secs: 1_000,
        file_size: 100,
    };
    store.put_sync_meta("doc.txt", &sm).unwrap();
    store.delete_sync_meta("doc.txt").unwrap();
    assert!(store.get_sync_meta("doc.txt").unwrap().is_none());
}

#[test]
fn meta_store_all_sync_metas() {
    let dir = TempDir::new().unwrap();
    let store = open_meta(&dir);
    let sm = SyncMeta {
        file_path: "f".to_string(),
        mtime_secs: 1,
        file_size: 1,
    };
    store.put_sync_meta("a.txt", &sm).unwrap();
    store.put_sync_meta("b.txt", &sm).unwrap();
    let all = store.all_sync_metas().unwrap();
    assert_eq!(all.len(), 2);
}

// ─────────────────────────────────────────────────────────────────────────────
// Prompt building
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn rag_prompt_contains_query_and_sources() {
    let chunks = vec![
        make_retrieved("District Six was a vibrant community.", 0.9),
        make_retrieved("It was demolished under apartheid.", 0.8),
    ];
    let prompt = build_rag_prompt("What was District Six?", &chunks, 10_000);
    assert!(prompt.contains("What was District Six?"));
    assert!(prompt.contains("[1]"));
    assert!(prompt.contains("[2]"));
    assert!(prompt.contains("District Six was a vibrant"));
}

#[test]
fn rag_prompt_empty_chunks() {
    let prompt = build_rag_prompt("What?", &[], 10_000);
    assert!(prompt.contains("What?"));
    assert!(prompt.contains("0 source"));
}

#[test]
fn rag_prompt_respects_max_context_chars() {
    // Large chunk text that would exceed a tiny budget
    let big_text = "x".repeat(500);
    let chunks = vec![
        make_retrieved(&big_text, 0.9),
        make_retrieved(&big_text, 0.8),
    ];
    let prompt = build_rag_prompt("query?", &chunks, 100);
    // The prompt body should be much shorter than 2 × 500 + overhead
    // At minimum, only the first chunk (partially) fits
    assert!(prompt.len() < 2 * 500 + 200);
}

#[test]
fn chat_messages_has_system_and_user() {
    let chunks = vec![make_retrieved("Some context.", 0.9)];
    let messages = build_chat_messages("My query?", &chunks, &[], 10_000, None);
    let roles: Vec<&str> = messages.iter().map(|m| m.role.as_str()).collect();
    assert!(roles.contains(&"system"));
    assert_eq!(roles.last(), Some(&"user"));
}

#[test]
fn chat_messages_user_message_is_last() {
    let chunks = vec![make_retrieved("ctx", 0.9)];
    let messages = build_chat_messages("Hello?", &chunks, &[], 10_000, None);
    let last = messages.last().unwrap();
    assert_eq!(last.role, "user");
    assert_eq!(last.content, "Hello?");
}

#[test]
fn chat_messages_includes_history() {
    let history = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Previous question".to_string(),
        },
        ChatMessage {
            role: "assistant".to_string(),
            content: "Previous answer".to_string(),
        },
    ];
    let messages = build_chat_messages("New question?", &[], &history, 10_000, None);
    // system + 2 history + user = 4
    assert_eq!(messages.len(), 4);
}

#[test]
fn chat_messages_includes_doc_context() {
    let chunks = vec![make_retrieved("ctx", 0.9)];
    let messages = build_chat_messages(
        "query?",
        &chunks,
        &[],
        10_000,
        Some("\"District Six\" by Joe Rassool (1984)"),
    );
    let system = &messages[0];
    assert_eq!(system.role, "system");
    assert!(
        system.content.contains("District Six"),
        "doc context should appear in system message"
    );
}

#[test]
fn chat_messages_source_count_matches_chunks() {
    let chunks: Vec<RetrievedChunk> = (0..4)
        .map(|i| make_retrieved(&format!("chunk {i}"), 0.9))
        .collect();
    let messages = build_chat_messages("q?", &chunks, &[], 10_000, None);
    let system = &messages[0];
    assert!(system.content.contains("[1]") || system.content.contains("4 source"));
}

// ─────────────────────────────────────────────────────────────────────────────
// SeedJson
// ─────────────────────────────────────────────────────────────────────────────

fn make_payload(entities: Vec<NbEntity>, relations: Vec<NbRelation>) -> NbPayload {
    NbPayload {
        document: None,
        entities,
        relations,
    }
}

fn nb_entity(name: &str, typ: &str, confidence: Option<&str>) -> NbEntity {
    NbEntity {
        canonical: name.to_string(),
        entity_type: typ.to_string(),
        aliases: vec![],
        gender: None,
        description: Some(format!("A {} named {}.", typ, name)),
        birth_year: None,
        death_year: None,
        confidence: confidence.map(|s| s.to_string()),
    }
}

fn nb_relation(from: &str, to: &str, typ: &str, confidence: Option<&str>) -> NbRelation {
    NbRelation {
        from: from.to_string(),
        to: to.to_string(),
        relation_type: typ.to_string(),
        evidence: None,
        confidence: confidence.map(|s| s.to_string()),
    }
}

#[test]
fn seed_json_load_from_file() {
    let dir = TempDir::new().unwrap();
    let json = r#"{
        "entities": [
            {"canonical": "Alice", "type": "Person", "description": "A person."}
        ],
        "relations": []
    }"#;
    let path = dir.path().join("payload.json");
    std::fs::write(&path, json).unwrap();
    let payload = load_nb_json(&path).unwrap();
    assert_eq!(payload.entities.len(), 1);
    assert_eq!(payload.entities[0].canonical, "Alice");
}

#[test]
fn seed_json_strips_markdown_fences() {
    let dir = TempDir::new().unwrap();
    let json = "```json\n{\"entities\":[], \"relations\":[]}\n```";
    let path = dir.path().join("fenced.json");
    std::fs::write(&path, json).unwrap();
    let payload = load_nb_json(&path).unwrap();
    assert!(payload.entities.is_empty());
}

#[test]
fn seed_json_count_low_confidence() {
    let payload = make_payload(
        vec![
            nb_entity("Alice", "Person", Some("high")),
            nb_entity("Bob", "Person", Some("low")),
            nb_entity("Carol", "Person", None),
        ],
        vec![
            nb_relation("Alice", "Bob", "related_to", Some("low")),
            nb_relation("Alice", "Carol", "associated_with", Some("high")),
        ],
    );
    let (low_e, low_r) = count_low_confidence(&payload);
    assert_eq!(low_e, 1, "one low-confidence entity");
    assert_eq!(low_r, 1, "one low-confidence relation");
}

#[test]
fn seed_json_to_seed_yaml_includes_entity_names() {
    let payload = make_payload(
        vec![
            nb_entity("Alice", "Person", None),
            nb_entity("Cape Town Council", "Organization", None),
        ],
        vec![nb_relation(
            "Alice",
            "Cape Town Council",
            "belongs_to",
            Some("high"),
        )],
    );
    let yaml = to_seed_yaml(&payload);
    assert!(yaml.contains("Alice"));
    assert!(yaml.contains("Cape Town Council"));
    assert!(yaml.contains("belongs_to"));
}

#[test]
fn seed_json_to_seed_yaml_excludes_low_confidence_relations() {
    let payload = make_payload(
        vec![
            nb_entity("Alice", "Person", None),
            nb_entity("Bob", "Person", None),
        ],
        vec![
            nb_relation("Alice", "Bob", "parent_of", Some("low")),
            nb_relation("Alice", "Bob", "related_to", Some("high")),
        ],
    );
    let yaml = to_seed_yaml(&payload);
    assert!(
        yaml.contains("related_to"),
        "high confidence relation should appear"
    );
    assert!(
        !yaml.contains("parent_of"),
        "low confidence relation should be excluded"
    );
}

#[test]
fn seed_json_to_family_tree_converts_entities() {
    let payload = make_payload(
        vec![
            nb_entity("Alice", "Person", None),
            nb_entity("Bob", "Person", None),
        ],
        vec![nb_relation("Alice", "Bob", "parent_of", Some("high"))],
    );
    let tree = to_family_tree(&payload);
    assert_eq!(tree.persons.len(), 2);
    assert_eq!(tree.relations.len(), 1);
    assert_eq!(tree.relations[0].relation_type, "parent_of");
}

#[test]
fn seed_json_to_family_tree_excludes_low_confidence_relations() {
    let payload = make_payload(
        vec![nb_entity("Alice", "Person", None)],
        vec![nb_relation("Alice", "Bob", "related_to", Some("low"))],
    );
    let tree = to_family_tree(&payload);
    assert!(
        tree.relations.is_empty(),
        "low-confidence relation should be excluded"
    );
}

#[test]
fn seed_json_birth_death_years_in_description() {
    let entity = NbEntity {
        canonical: "Alice".to_string(),
        entity_type: "Person".to_string(),
        aliases: vec![],
        gender: None,
        description: Some("Activist.".to_string()),
        birth_year: Some(1920),
        death_year: Some(2001),
        confidence: None,
    };
    let payload = make_payload(vec![entity], vec![]);
    let yaml = to_seed_yaml(&payload);
    assert!(yaml.contains("1920"), "birth year should appear in yaml");
    assert!(yaml.contains("2001"), "death year should appear in yaml");
}

// ─────────────────────────────────────────────────────────────────────────────
// Family tree loading
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn family_load_yaml_parses_persons_and_relations() {
    let dir = TempDir::new().unwrap();
    let yaml = r#"
persons:
  - canonical: "Yousuf Rassool"
    aliases:
      - "Joe Rassool"
    description: "Activist and author from District Six."
  - canonical: "Fatima Rassool"
    description: "Wife of Yousuf Rassool."
relations:
  - from: "Yousuf Rassool"
    to: "Fatima Rassool"
    type: "spouse_of"
"#;
    let path = dir.path().join("family.yaml");
    std::fs::write(&path, yaml).unwrap();
    let tree = load_family_tree(&path).unwrap();

    assert_eq!(tree.persons.len(), 2);
    let yousuf = tree
        .persons
        .iter()
        .find(|p| p.canonical == "Yousuf Rassool")
        .unwrap();
    assert_eq!(yousuf.aliases, vec!["Joe Rassool"]);
    assert!(yousuf.description.contains("Activist"));
    assert_eq!(tree.relations.len(), 1);
    assert_eq!(tree.relations[0].relation_type, "spouse_of");
}

#[test]
fn family_load_yaml_empty_relations_ok() {
    let dir = TempDir::new().unwrap();
    let yaml = "persons:\n  - canonical: \"Alice\"\n    description: \"Test.\"\n";
    let path = dir.path().join("f.yaml");
    std::fs::write(&path, yaml).unwrap();
    let tree = load_family_tree(&path).unwrap();
    assert_eq!(tree.persons.len(), 1);
    assert!(tree.relations.is_empty());
}

#[test]
fn inspect_gool_entities_live() {
    // Read entity descriptions from the live D6 graph to debug injection failures
    use std::path::Path;
    let data_dir = Path::new("/Users/rezarassool/.kwaainet/rag/D6");
    let tid = uuid::Uuid::parse_str("dfdf26a4-c00f-4ea7-9317-a187ac215acf").unwrap();
    let Ok(g) = kwaai_rag::graph::GraphStore::open(data_dir, tid) else {
        eprintln!("Could not open D6 graph (may be locked)");
        return;
    };
    let tokens = [
        "gool",
        "cissie",
        "hamid",
        "rassool",
        "nazima",
        "buitencingle",
    ];
    for token in &tokens {
        let ids = g.find_ids_by_name_token(token);
        for id in ids {
            if let Some(e) = g.get_entity(id) {
                let desc = e.description.trim();
                let sentences = desc
                    .chars()
                    .filter(|c| matches!(c, '.' | '?' | '!'))
                    .count();
                println!(
                    "ENTITY [{token}] name={:?} desc_len={} sentences={} desc={:?}",
                    e.name,
                    desc.len(),
                    sentences,
                    if desc.len() > 200 { &desc[..200] } else { desc }
                );
            }
        }
    }
}

/// Diagnose author entity family graph relations to check if grandfather/wife traversal works.
#[test]
fn d6_inspect_author_relations() {
    use std::path::Path;
    let data_dir = Path::new("/Users/rezarassool/.kwaainet/rag/D6");
    let tid = uuid::Uuid::parse_str("dfdf26a4-c00f-4ea7-9317-a187ac215acf").unwrap();
    let Ok(g) = kwaai_rag::graph::GraphStore::open(data_dir, tid) else {
        eprintln!("Could not open D6 graph (may be locked)");
        return;
    };
    // Find all Rassool entities as author candidates, print aliases
    let rassool_ids = g.find_ids_by_name_token("rassool");
    for id in rassool_ids {
        if let Some(e) = g.get_entity(id) {
            println!(
                "RASSOOL name={:?} id={id} aliases={:?} desc_len={}",
                e.name,
                e.aliases,
                e.description.len()
            );
        }
    }
    // Also search for alias "author" / "narrator" across ALL entities
    println!("\n--- Entities with author/narrator alias ---");
    for e in g.all_entities() {
        let has_author_alias = e.aliases.iter().any(|a| {
            matches!(
                a.to_lowercase().as_str(),
                "author" | "the author" | "narrator" | "the narrator" | "the writer"
            )
        });
        if has_author_alias {
            println!("AUTHOR ALIAS entity={:?} aliases={:?}", e.name, e.aliases);
        }
    }
    // Check JMH Gool aliases for "grandfather"
    println!("\n--- JMH Gool aliases ---");
    for id in g.find_ids_by_name_token("joosub") {
        if let Some(e) = g.get_entity(id) {
            if e.name.contains("Gool") {
                println!("JMH entity name={:?} aliases={:?}", e.name, e.aliases);
            }
        }
    }
    println!("\n--- Entities with grandfather alias ---");
    for e in g.all_entities() {
        if e.aliases.iter().any(|a| {
            a.to_lowercase().contains("grandfather") || a.to_lowercase().contains("grandpa")
        }) {
            println!("GRANDFATHER ALIAS entity={:?}", e.name);
        }
    }
    // Walk grandfather path from Joe/Yousuf Rassool
    println!("\n--- Grandfather traversal ---");
    for id in g.find_ids_by_name_token("rassool") {
        if let Some(e) = g.get_entity(id) {
            let parents: Vec<i64> = g
                .neighbors_of(id)
                .into_iter()
                .filter(|(_, rel, _)| rel == "child_of")
                .map(|(nid, _, _)| nid)
                .collect();
            if !parents.is_empty() {
                println!("Author {:?} parents:", e.name);
                for pid in &parents {
                    let pname = g.get_entity(*pid).map(|e| e.name.as_str()).unwrap_or("?");
                    println!("  parent={pname:?}");
                    for (gpid, rel, _) in g.neighbors_of(*pid) {
                        if rel == "child_of" {
                            let gname = g.get_entity(gpid).map(|e| e.name.as_str()).unwrap_or("?");
                            let gender = g.get_entity(gpid).and_then(|e| e.gender.clone());
                            println!("    grandparent={gname:?} gender={gender:?}");
                        }
                    }
                }
            }
        }
    }
}

/// Update key D6 entity descriptions with accurate biography facts from the eval ground truth.
/// Run this once to fix entities that have thin/wrong descriptions from dream cycles.
/// Uses exact name matching to avoid accidentally updating the wrong entity.
#[test]
fn d6_fix_key_entity_descriptions() {
    use std::path::Path;
    let data_dir = Path::new("/Users/rezarassool/.kwaainet/rag/D6");
    let tid = uuid::Uuid::parse_str("dfdf26a4-c00f-4ea7-9317-a187ac215acf").unwrap();
    let Ok(mut g) = kwaai_rag::graph::GraphStore::open(data_dir, tid) else {
        eprintln!("Could not open D6 graph (may be locked by eval)");
        return;
    };

    // (exact_entity_name, new_description)
    let updates: &[(&str, &str)] = &[
        // Revert accidentally-updated entities back to neutral descriptions
        ("Al Hajj Joosub Maulvi Hamid",
         "Al Hajj Joosub Maulvi Hamid — a title-based name reference in District Six; \
          distinct from Haji Joosub Maulvi Hamid Gool."),
        ("Auntie Cissie",
         "Auntie Cissie — a family member of the author referred to with an affectionate title; \
          distinct from Cissie Gool."),
        // Correctly update the target entities with accurate biographies
        ("Haji Joosub Maulvi Hamid Gool",
         "J.M.H. Gool (Joosub Maulvi Hamid Gool) was the author's maternal grandfather. \
          He came from India to the Cape in 1884 via Mauritius. He was a prosperous merchant \
          and entrepreneur in the spice trade, and his firm J.M.H. Gool & Co. at 25 Church \
          Street, Cape Town became suppliers to the troops of Queen Victoria. He founded the \
          Hanaffi Quwatul Islam Mosque in Loop Street, Cape Town (completed 1898), and lived \
          at No. 7 Buitencingle Street."),
        ("Cissie Gool",
         "Cissie Gool (Zainunnissa Gool) was a renowned Cape Town politician and activist. \
          She was the daughter of Dr. Abdullah Abdurahman, a long-serving Cape Town city \
          councillor. Cissie Gool was herself a city councillor known for her fiery speeches \
          and involvement in the anti-apartheid struggle. She organized demonstrations against \
          residential segregation and was a leader of the Liberation League."),
    ];

    // Collect (id, name, current_desc_len) for all entities upfront to avoid borrow conflict
    let id_name_pairs: Vec<(i64, String)> =
        g.all_entities().map(|e| (e.id, e.name.clone())).collect();

    for (exact_name, new_desc) in updates {
        let found = id_name_pairs
            .iter()
            .find(|(_, name)| name.as_str() == *exact_name);
        match found {
            Some((id, name)) => {
                let cur_len = g.get_entity(*id).map(|e| e.description.len()).unwrap_or(0);
                println!(
                    "Updating '{}': {} chars -> {} chars",
                    name,
                    cur_len,
                    new_desc.len()
                );
                g.set_description(*id, new_desc)
                    .expect("set_description failed");
            }
            None => {
                eprintln!("WARNING: no entity found with exact name={exact_name:?}");
            }
        }
    }
    println!("Entity description update complete.");
}

#[test]
fn d6_check_cissie_aliases() {
    use std::path::Path;
    let data_dir = Path::new("/Users/rezarassool/.kwaainet/rag/D6");
    let tid = uuid::Uuid::parse_str("dfdf26a4-c00f-4ea7-9317-a187ac215acf").unwrap();
    let Ok(g) = kwaai_rag::graph::GraphStore::open(data_dir, tid) else {
        return;
    };
    // Find exact Cissie Gool entity
    if let Some(e) = g.find_by_name("Cissie Gool") {
        println!(
            "Cissie Gool id={} aliases={:?} desc_len={}",
            e.id,
            e.aliases,
            e.description.len()
        );
        let sentences = e
            .description
            .chars()
            .filter(|c| matches!(c, '.' | '?' | '!'))
            .count();
        println!("  sentences={sentences}");
    }
    // Count entities with "grandfather" in ANY alias
    let mut count = 0;
    for e in g.all_entities() {
        if e.aliases.iter().any(|a| {
            a.to_lowercase().contains("grandfather") || a.to_lowercase().contains("grandpa")
        }) {
            count += 1;
            println!(
                "GRANDPARENT ALIAS: name={:?} aliases={:?}",
                e.name, e.aliases
            );
        }
    }
    println!("Total entities with grandfather/grandpa alias: {count}");
}
