//! Tantivy-backed BM25 full-text index for hybrid RAG retrieval.
//!
//! Complements dense vector search: handles acronym and exact-name queries
//! that semantic embeddings handle poorly on narrow-domain corpora.
//! Results are merged with dense results via Reciprocal Rank Fusion (RRF).

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{
    Field, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing, TextOptions, Value, FAST,
    STORED, STRING,
};
use tantivy::{Index, IndexWriter, ReloadPolicy, TantivyDocument};

// ---------------------------------------------------------------------------
// Schema field names
// ---------------------------------------------------------------------------

const F_CHUNK_ID: &str = "chunk_id";
const F_DOC_NAME: &str = "doc_name";
const F_TITLE: &str = "title";
const F_BODY: &str = "body";

pub struct BM25Index {
    index: Index,
    chunk_id_field: Field,
    doc_name_field: Field,
    title_field: Field,
    body_field: Field,
}

impl BM25Index {
    /// Build a transient in-RAM index from chunks. No disk persistence.
    /// Used for single-shot CLI queries where per-query rebuild is acceptable.
    pub fn build_in_ram(chunks: &[(i64, &str, &str)]) -> Result<Self> {
        let schema = Self::build_schema();
        let chunk_id_field = schema.get_field(F_CHUNK_ID).unwrap();
        let doc_name_field = schema.get_field(F_DOC_NAME).unwrap();
        let title_field = schema.get_field(F_TITLE).unwrap();
        let body_field = schema.get_field(F_BODY).unwrap();
        let index = Index::create_in_ram(schema);
        let this = Self {
            index,
            chunk_id_field,
            doc_name_field,
            title_field,
            body_field,
        };
        this.build_from_chunks(chunks)?;
        Ok(this)
    }

    /// Open (or create) the tantivy index at `data_dir/tantivy/`.
    pub fn open(data_dir: &Path) -> Result<Self> {
        let index_dir = data_dir.join("tantivy");
        std::fs::create_dir_all(&index_dir)?;

        let schema = Self::build_schema();
        let chunk_id_field = schema.get_field(F_CHUNK_ID).unwrap();
        let doc_name_field = schema.get_field(F_DOC_NAME).unwrap();
        let title_field = schema.get_field(F_TITLE).unwrap();
        let body_field = schema.get_field(F_BODY).unwrap();

        let index = if Index::exists(&tantivy::directory::MmapDirectory::open(&index_dir)?)? {
            Index::open_in_dir(&index_dir)
                .with_context(|| format!("opening tantivy index at {}", index_dir.display()))?
        } else {
            Index::create_in_dir(&index_dir, schema)
                .with_context(|| format!("creating tantivy index at {}", index_dir.display()))?
        };

        Ok(Self {
            index,
            chunk_id_field,
            doc_name_field,
            title_field,
            body_field,
        })
    }

    fn build_schema() -> Schema {
        let mut builder = Schema::builder();

        // chunk_id: stored as u64 FAST for retrieval, not indexed for text search
        builder.add_u64_field(F_CHUNK_ID, NumericOptions::default() | STORED | FAST);

        // doc_name: indexed as exact keyword (STRING) so delete_term works; also stored.
        builder.add_text_field(F_DOC_NAME, STRING | STORED);

        // title: doc-name tokens, searched with 3× boost via field weights in query
        let title_opts = TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("en_stem")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        );
        builder.add_text_field(F_TITLE, title_opts);

        // body: chunk text, standard BM25 with stemming
        let body_opts = TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("en_stem")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        );
        builder.add_text_field(F_BODY, body_opts);

        builder.build()
    }

    fn writer(&self) -> Result<IndexWriter> {
        // 16 MB buffer is fine for personal-scale KBs
        self.index
            .writer(16_000_000)
            .context("creating tantivy writer")
    }

    /// Build (or rebuild) the index from a flat list of (chunk_id, doc_name, text) triples.
    /// Called once on startup if the index is empty, or after `rag destroy` + `rag init`.
    pub fn build_from_chunks(&self, chunks: &[(i64, &str, &str)]) -> Result<()> {
        let mut writer = self.writer()?;
        // Delete everything first (idempotent rebuild).
        writer.delete_all_documents()?;
        for &(chunk_id, doc_name, text) in chunks {
            writer.add_document(self.make_doc(chunk_id, doc_name, text))?;
        }
        writer.commit()?;
        Ok(())
    }

    /// Add new chunks incrementally (called during ingest).
    pub fn add_chunks(&self, chunks: &[(i64, &str, &str)]) -> Result<()> {
        if chunks.is_empty() {
            return Ok(());
        }
        let mut writer = self.writer()?;
        for &(chunk_id, doc_name, text) in chunks {
            writer.add_document(self.make_doc(chunk_id, doc_name, text))?;
        }
        writer.commit()?;
        Ok(())
    }

    /// Delete all chunks belonging to a document (called on `rag delete-doc`).
    pub fn delete_doc(&self, doc_name: &str) -> Result<()> {
        let mut writer = self.writer()?;
        let term = tantivy::Term::from_field_text(self.doc_name_field, doc_name);
        writer.delete_term(term);
        writer.commit()?;
        Ok(())
    }

    /// BM25 search. Returns (chunk_id, score) sorted descending.
    pub fn search(&self, query_text: &str, top_k: usize) -> Vec<(i64, f64)> {
        if top_k == 0 || query_text.trim().is_empty() {
            return vec![];
        }
        let reader = match self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
        {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        let searcher = reader.searcher();
        if searcher.num_docs() == 0 {
            return vec![];
        }

        // Query over both title (3× boost) and body
        let mut parser =
            QueryParser::for_index(&self.index, vec![self.title_field, self.body_field]);
        parser.set_field_boost(self.title_field, 3.0);
        parser.set_field_boost(self.body_field, 1.0);

        let query = match parser.parse_query(query_text) {
            Ok(q) => q,
            Err(_) => {
                // Fall back to individual term union if parse fails (e.g. special chars)
                let safe: String = query_text
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == ' ' {
                            c
                        } else {
                            ' '
                        }
                    })
                    .collect();
                match parser.parse_query(&safe) {
                    Ok(q) => q,
                    Err(_) => return vec![],
                }
            }
        };

        let top_docs = match searcher.search(&query, &TopDocs::with_limit(top_k)) {
            Ok(d) => d,
            Err(_) => return vec![],
        };

        top_docs
            .into_iter()
            .filter_map(|(score, addr)| {
                let doc: TantivyDocument = searcher.doc(addr).ok()?;
                let id_val = doc.get_first(self.chunk_id_field)?;
                let chunk_id = id_val.as_u64()? as i64;
                Some((chunk_id, score as f64))
            })
            .collect()
    }

    fn make_doc(&self, chunk_id: i64, doc_name: &str, text: &str) -> TantivyDocument {
        let title = doc_name
            .trim_end_matches(".docx")
            .trim_end_matches(".pdf")
            .trim_end_matches(".txt")
            .replace(['_', '-'], " ");

        let mut doc = TantivyDocument::default();
        doc.add_u64(self.chunk_id_field, chunk_id as u64);
        doc.add_text(self.doc_name_field, doc_name);
        doc.add_text(self.title_field, &title);
        doc.add_text(self.body_field, text);
        doc
    }
}

/// Reciprocal Rank Fusion: merge two ranked lists into one combined ranking.
/// k=60 is the standard value (Cormack et al. 2009).
pub fn rrf_merge(semantic: &[(i64, f64)], keyword: &[(i64, f64)], top_k: usize) -> Vec<(i64, f64)> {
    const K: f64 = 60.0;
    let mut rrf: HashMap<i64, f64> = HashMap::new();

    for (rank, &(id, _)) in semantic.iter().enumerate() {
        *rrf.entry(id).or_default() += 1.0 / (K + rank as f64 + 1.0);
    }
    for (rank, &(id, _)) in keyword.iter().enumerate() {
        *rrf.entry(id).or_default() += 1.0 / (K + rank as f64 + 1.0);
    }

    let mut merged: Vec<(i64, f64)> = rrf.into_iter().collect();
    merged.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    merged.truncate(top_k);
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn make_index() -> (TempDir, BM25Index) {
        let dir = TempDir::new().unwrap();
        let idx = BM25Index::open(dir.path()).unwrap();
        (dir, idx)
    }

    #[test]
    fn bm25_finds_relevant() {
        let (_dir, idx) = make_index();
        let chunks = vec![
            (
                1i64,
                "Part_2_Chapter_14_Teacher_League_South_Africa.docx",
                "the teacher's league of south africa tlsa was founded in 1913",
            ),
            (
                2i64,
                "Part_2_Chapter_5_District_Six.docx",
                "district six characters caledon street cape town coloured community",
            ),
            (
                3i64,
                "Part_2_Chapter_15_Apartheid.docx",
                "apartheid laws racial segregation south africa national party act",
            ),
        ];
        idx.build_from_chunks(&chunks).unwrap();
        let results = idx.search("TLSA teacher league south africa", 3);
        assert!(!results.is_empty(), "should find at least one result");
        assert_eq!(
            results[0].0, 1,
            "Chapter 14 should rank first for TLSA query"
        );
    }

    #[test]
    fn bm25_incremental_add_and_delete() {
        let (_dir, idx) = make_index();
        let chunks = vec![
            (10i64, "doc_a.docx", "hello world foo bar"),
            (11i64, "doc_b.docx", "apartheid segregation south africa"),
        ];
        idx.build_from_chunks(&chunks).unwrap();
        assert!(!idx.search("apartheid", 5).is_empty());

        // Delete doc_b
        idx.delete_doc("doc_b.docx").unwrap();
        // Add a new chunk to doc_b replacement
        idx.add_chunks(&[(12i64, "doc_c.docx", "cricket sport bat wicket")])
            .unwrap();

        let res = idx.search("apartheid", 5);
        // doc_b deleted — should no longer appear
        assert!(res.is_empty() || res.iter().all(|(id, _)| *id != 11));
    }

    #[test]
    fn rrf_combines_lists() {
        let semantic = vec![(2i64, 0.9), (1i64, 0.8), (3i64, 0.7)];
        let keyword = vec![(1i64, 5.0), (3i64, 3.0), (2i64, 1.0)];
        let merged = rrf_merge(&semantic, &keyword, 3);
        assert_eq!(merged.len(), 3);
        // chunk 1 appears at rank-2 semantic + rank-1 keyword → should be top-ranked
        assert_eq!(merged[0].0, 1, "chunk 1 should win RRF");
    }
}
