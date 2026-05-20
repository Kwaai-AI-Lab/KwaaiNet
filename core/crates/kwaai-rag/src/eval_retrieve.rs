//! Phase 4 — Retrieval evaluation for Dream RAG.
//!
//! Measures how well the vector store retrieves relevant chunks given
//! natural-language queries about entities in the knowledge graph.
//!
//! Intended use:
//!   1. Run `kwaainet rag dream embed-eval --kb <KB>` to get the current score.
//!   2. Re-run across different graph completeness checkpoints to produce
//!      the retrieve-score vs. graph-score correlation for the Dream RAG paper.

use crate::embedder::EmbedClient;
use crate::graph::GraphStore;
use crate::scorer::{schema_type_for, score_graph};
use anyhow::Result;
use serde::Serialize;
use std::collections::HashSet;
use std::future::Future;

// ── Query generation ─────────────────────────────────────────────────────────

pub struct EvalQuery {
    pub entity_id: i64,
    pub entity_name: String,
    pub schema_type: String,
    pub query_text: String,
    /// True when query_text is a description-content query (paper's primary signal).
    /// False when it is a name-lookup query (baseline).
    pub is_content_query: bool,
    pub gold_chunk_ids: HashSet<i64>,
}

/// Build a natural-language question for an entity based on its schema type.
fn query_for_name(name: &str, schema_type: &str) -> String {
    match schema_type {
        "schema:Person" => format!("Who is {}?", name),
        "schema:Place" => format!("Where is {} located?", name),
        "schema:Organization" => format!("What is {}?", name),
        "schema:Event" => format!("What happened at the {}?", name),
        "schema:DefinedTerm" => format!("What does {} mean?", name),
        "schema:CreativeWork" | "schema:Product" => format!("What is {}?", name),
        _ => format!("Tell me about {}.", name),
    }
}

/// Build a content query from the entity description (name excluded).
///
/// Strips the entity name from the description so the query tests semantic
/// content retrieval rather than name matching.  This is the primary signal
/// for the Dream RAG paper: richer descriptions → better content-based recall.
fn query_from_description(name: &str, description: &str) -> Option<String> {
    if description.len() < 40 {
        return None; // too thin to generate a useful content query
    }
    // Use at most the first 200 chars of the description, excluding the entity name.
    let snippet: String = description
        .chars()
        .take(200)
        .collect::<String>()
        .replace(name, "they"); // anonymise so the name isn't a free cue
    Some(snippet.trim().to_string())
}

/// Generate evaluation queries from the knowledge graph.
///
/// Filters out:
/// - Entities with no evidence chunks (seeded or orphaned)
/// - Entities with very short names (single letters, fragments)
///
/// Query selection: prefer description-content queries (the paper's primary signal — tests
/// whether richer descriptions improve content-based entity retrieval) and fall back to
/// name-lookup queries when descriptions are too thin.
pub fn generate_eval_queries(store: &GraphStore, max_queries: usize) -> Vec<EvalQuery> {
    let mut queries: Vec<EvalQuery> = store
        .all_entities()
        .filter(|e| {
            if e.name.len() < 3 {
                return false;
            }
            let chunks = store.chunks_for_entity(e.id);
            if chunks.is_empty() {
                return false;
            }
            true
        })
        .map(|e| {
            let st = e
                .schema_type
                .as_deref()
                .or_else(|| schema_type_for(&e.entity_type))
                .unwrap_or("schema:Thing");

            let (query_text, is_content_query) =
                match query_from_description(&e.name, &e.description) {
                    Some(content_q) => (content_q, true),
                    None => (query_for_name(&e.name, st), false),
                };

            let gold_chunk_ids = store.chunks_for_entity(e.id).iter().copied().collect();
            EvalQuery {
                entity_id: e.id,
                entity_name: e.name.clone(),
                schema_type: st.to_string(),
                query_text,
                is_content_query,
                gold_chunk_ids,
            }
        })
        .collect();

    // Sort by name for deterministic ordering
    queries.sort_by(|a, b| a.entity_name.cmp(&b.entity_name));
    queries.truncate(max_queries);
    queries
}

// ── Eval results ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct EvalEntityResult {
    pub entity_name: String,
    pub schema_type: String,
    pub gold_chunks: usize,
    pub query_text: String,
    pub is_content_query: bool,
    // Chunk-space: is an evidence chunk in top-k raw chunk results?
    pub chunk_hit_at_1: bool,
    pub chunk_hit_at_5: bool,
    pub chunk_hit_at_10: bool,
    pub chunk_reciprocal_rank: f32,
    // Entity-space: is the entity itself in top-k entity embedding results?
    pub entity_hit_at_1: bool,
    pub entity_hit_at_5: bool,
    pub entity_hit_at_10: bool,
    pub entity_reciprocal_rank: f32,
}

/// Retrieval evaluation report.
///
/// Two complementary measurements:
///
/// **Chunk retrieval** — query → raw chunk vector store. Measures whether
/// entity-name queries retrieve the source passage mentioning the entity.
/// Tends to be low for name-lookup queries; a useful lower bound showing
/// where plain vector search fails.
///
/// **Entity retrieval** — query → entity embedding space. Measures whether
/// the enriched entity description embedding correctly surfaces the entity
/// for a natural-language question about it. This is the primary signal for
/// the Dream RAG paper: richer descriptions → better entity embeddings →
/// higher entity retrieval recall. Expect this to rise as graph score rises.
#[derive(Debug, Serialize)]
pub struct EvalReport {
    /// Current graph completeness score (0–1).
    pub graph_score: f32,
    /// Total entities in graph.
    pub entity_count: usize,
    /// Number of queries run.
    pub query_count: usize,
    /// Fraction of queries that used description-content queries (vs name-lookup).
    pub content_query_fraction: f32,
    // ── Chunk-space metrics ──
    pub chunk_recall_at_1: f32,
    pub chunk_recall_at_5: f32,
    pub chunk_recall_at_10: f32,
    pub chunk_mrr: f32,
    // ── Entity-space metrics (primary for the paper) ──
    pub entity_recall_at_1: f32,
    pub entity_recall_at_3: f32,
    pub entity_recall_at_5: f32,
    pub entity_recall_at_10: f32,
    pub entity_mrr: f32,
    pub generated_at: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub per_entity: Vec<EvalEntityResult>,
}

// ── Core evaluation ───────────────────────────────────────────────────────────

/// Run retrieval evaluation.
///
/// `chunk_search_fn(embedding, top_k)` → `Vec<(chunk_id, score)>` (chunk space).
/// Entity-space search uses `GraphStore::search_entities` directly.
pub async fn evaluate_retrieval<F, Fut>(
    store: &GraphStore,
    embed: &EmbedClient,
    chunk_search_fn: F,
    max_queries: usize,
    verbose: bool,
    progress: Option<&dyn Fn(usize, usize)>,
) -> Result<EvalReport>
where
    F: Fn(Vec<f32>, usize) -> Fut,
    Fut: Future<Output = Result<Vec<(i64, f64)>>>,
{
    const TOP_K: usize = 10;

    let health = score_graph(store);
    let queries = generate_eval_queries(store, max_queries);
    let total = queries.len();

    let mut content_query_count = 0usize;
    // Chunk-space accumulators
    let mut c_hits_1 = 0usize;
    let mut c_hits_5 = 0usize;
    let mut c_hits_10 = 0usize;
    let mut c_rr_sum = 0f32;
    // Entity-space accumulators
    let mut e_hits_1 = 0usize;
    let mut e_hits_3 = 0usize;
    let mut e_hits_5 = 0usize;
    let mut e_hits_10 = 0usize;
    let mut e_rr_sum = 0f32;

    let mut per_entity: Vec<EvalEntityResult> = Vec::new();

    for (i, q) in queries.iter().enumerate() {
        if let Some(cb) = progress {
            cb(i + 1, total);
        }

        if q.is_content_query {
            content_query_count += 1;
        }

        // embed_one() applies the "search_query:" prefix for nomic-embed-text automatically.
        let emb = match embed.embed_one(&q.query_text).await {
            Ok(v) => v,
            Err(_) => continue,
        };

        // ── Chunk-space ──
        let chunk_results = chunk_search_fn(emb.clone(), TOP_K)
            .await
            .unwrap_or_default();
        let c_rank = chunk_results
            .iter()
            .enumerate()
            .find(|(_, (id, _))| q.gold_chunk_ids.contains(id))
            .map(|(r, _)| r + 1)
            .unwrap_or(0);

        let c_rr = if c_rank > 0 { 1.0 / c_rank as f32 } else { 0.0 };
        if c_rank == 1 {
            c_hits_1 += 1;
        }
        if c_rank > 0 && c_rank <= 5 {
            c_hits_5 += 1;
        }
        if c_rank > 0 && c_rank <= 10 {
            c_hits_10 += 1;
        }
        c_rr_sum += c_rr;

        // ── Entity-space (primary signal for the paper) ──
        let entity_results = store.search_entities(&emb, TOP_K);
        let e_rank = entity_results
            .iter()
            .enumerate()
            .find(|(_, (id, _))| *id == q.entity_id)
            .map(|(r, _)| r + 1)
            .unwrap_or(0);

        let e_rr = if e_rank > 0 { 1.0 / e_rank as f32 } else { 0.0 };
        if e_rank == 1 {
            e_hits_1 += 1;
        }
        if e_rank > 0 && e_rank <= 3 {
            e_hits_3 += 1;
        }
        if e_rank > 0 && e_rank <= 5 {
            e_hits_5 += 1;
        }
        if e_rank > 0 && e_rank <= 10 {
            e_hits_10 += 1;
        }
        e_rr_sum += e_rr;

        if verbose {
            per_entity.push(EvalEntityResult {
                entity_name: q.entity_name.clone(),
                schema_type: q.schema_type.clone(),
                gold_chunks: q.gold_chunk_ids.len(),
                query_text: q.query_text.clone(),
                is_content_query: q.is_content_query,
                chunk_hit_at_1: c_rank == 1,
                chunk_hit_at_5: c_rank > 0 && c_rank <= 5,
                chunk_hit_at_10: c_rank > 0 && c_rank <= 10,
                chunk_reciprocal_rank: c_rr,
                entity_hit_at_1: e_rank == 1,
                entity_hit_at_5: e_rank > 0 && e_rank <= 5,
                entity_hit_at_10: e_rank > 0 && e_rank <= 10,
                entity_reciprocal_rank: e_rr,
            });
        }
    }

    let n = total as f32;
    Ok(EvalReport {
        graph_score: health.overall,
        entity_count: store.node_count(),
        query_count: total,
        content_query_fraction: content_query_count as f32 / n,
        chunk_recall_at_1: c_hits_1 as f32 / n,
        chunk_recall_at_5: c_hits_5 as f32 / n,
        chunk_recall_at_10: c_hits_10 as f32 / n,
        chunk_mrr: c_rr_sum / n,
        entity_recall_at_1: e_hits_1 as f32 / n,
        entity_recall_at_3: e_hits_3 as f32 / n,
        entity_recall_at_5: e_hits_5 as f32 / n,
        entity_recall_at_10: e_hits_10 as f32 / n,
        entity_mrr: e_rr_sum / n,
        generated_at: chrono::Utc::now().to_rfc3339(),
        per_entity,
    })
}
