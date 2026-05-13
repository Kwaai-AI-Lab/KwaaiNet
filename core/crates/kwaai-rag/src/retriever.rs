use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use anyhow::Result;

use crate::bm25::{rrf_merge, BM25Index};
use crate::embedder::EmbedClient;
use crate::graph::GraphStore;
use crate::hyde::{embed_with_hyde, embed_with_hyde_blend};
use crate::meta_store::{ChunkMeta, MetaStore};

#[derive(Debug, Clone)]
pub struct RetrievedChunk {
    pub chunk_meta: ChunkMeta,
    pub score: f64,
    pub source_kb: Option<String>,
    pub rerank_score: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RetrieveConfig {
    pub top_k: usize,
    pub min_score: f64,
    pub use_sentence_window: bool,
    /// When set, uses HyDE: embeds a LLM-generated hypothetical answer instead of the raw query.
    pub hyde_inference_url: Option<String>,
    pub hyde_model: Option<String>,
    /// Blend factor for HyDE (0.0 = pure query, 1.0 = pure HyDE, 0.5 = equal blend).
    /// Only applies when `hyde_inference_url` and `hyde_model` are set.
    /// When `None`, defaults to 1.0 (pure HyDE, original behaviour).
    pub hyde_alpha: Option<f32>,
}

impl Default for RetrieveConfig {
    fn default() -> Self {
        Self {
            top_k: 5,
            min_score: 0.0,
            use_sentence_window: false,
            hyde_inference_url: None,
            hyde_model: None,
            hyde_alpha: None,
        }
    }
}

/// Dense-only retrieval: embed query → vector search → fetch chunk metadata.
pub async fn retrieve(
    query: &str,
    cfg: &RetrieveConfig,
    embed: &EmbedClient,
    meta: &MetaStore,
    search_fn: impl Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output = Result<Vec<(i64, f64)>>> + Send>>,
) -> Result<Vec<RetrievedChunk>> {
    let embedding = embed.embed_one(query).await?;
    let raw = search_fn(embedding, cfg.top_k * 4).await?;
    assemble_results(raw, cfg, meta)
}

/// Hybrid retrieval: dense vector search + BM25, merged via RRF.
///
/// Use this instead of `retrieve` for production queries — it handles
/// acronym and exact-name queries that pure semantic search misses.
pub async fn retrieve_hybrid(
    query: &str,
    cfg: &RetrieveConfig,
    embed: &EmbedClient,
    meta: &MetaStore,
    search_fn: impl Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output = Result<Vec<(i64, f64)>>> + Send>>,
) -> Result<Vec<RetrievedChunk>> {
    // Build BM25 index from all stored chunks (including doc name for title-word discrimination).
    let all = meta.all_chunks()?;
    let triples: Vec<(i64, &str, &str)> = all
        .iter()
        .map(|(id, cm)| (*id, cm.doc_name.as_str(), cm.text.as_str()))
        .collect();
    let bm25 = BM25Index::build_in_ram(&triples)?;

    let candidate_k = cfg.top_k * 4;

    // Dense embedding — use HyDE (optionally blended) if configured, else plain query embedding.
    let embedding = match (&cfg.hyde_inference_url, &cfg.hyde_model) {
        (Some(url), Some(model)) => match cfg.hyde_alpha {
            Some(alpha) => embed_with_hyde_blend(query, embed, url, model, alpha).await,
            None => embed_with_hyde(query, embed, url, model).await,
        },
        _ => embed.embed_one(query).await?,
    };

    let semantic_raw = search_fn(embedding, candidate_k).await?;
    let keyword_raw = bm25.search(query, candidate_k);

    // Merge with RRF.
    let merged = rrf_merge(&semantic_raw, &keyword_raw, cfg.top_k * 2);
    assemble_results(merged, cfg, meta)
}

/// Graph-anchored retrieval: entity similarity search → BFS traversal → chunk lookup,
/// fused with hybrid vector+BM25 results via RRF.
///
/// Falls back gracefully to `retrieve_hybrid` if the graph has no entities.
pub async fn retrieve_graph_anchored(
    query: &str,
    cfg: &RetrieveConfig,
    embed: &EmbedClient,
    meta: &MetaStore,
    graph: &GraphStore,
    search_fn: impl Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output = Result<Vec<(i64, f64)>>> + Send>>,
) -> Result<Vec<RetrievedChunk>> {
    let candidate_k = cfg.top_k * 4;

    // Dense embedding — use HyDE (optionally blended) if configured, else plain query embedding.
    let embedding = match (&cfg.hyde_inference_url, &cfg.hyde_model) {
        (Some(url), Some(model)) => match cfg.hyde_alpha {
            Some(alpha) => embed_with_hyde_blend(query, embed, url, model, alpha).await,
            None => embed_with_hyde(query, embed, url, model).await,
        },
        _ => embed.embed_one(query).await?,
    };

    // 1. Find seed entities: embedding similarity + name-token matching.
    //    Embedding search alone fails for abbreviations/acronyms (e.g. "J.M.H. Gool"
    //    doesn't match the description embedding of the canonical entity). Name-token
    //    matching catches those cases by finding entities whose name contains any
    //    significant query word as a whole token.
    let mut seed_hits = graph.search_entities(&embedding, 5);
    let name_stop: &[&str] = &[
        "who", "what", "was", "were", "the", "tell", "about", "and", "for",
        "did", "how", "where", "when", "describe", "more", "kind", "place",
    ];
    let name_seed_ids: std::collections::HashSet<i64> =
        seed_hits.iter().map(|(id, _)| *id).collect();
    for word in query.split_whitespace() {
        let w = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if w.len() >= 3 && !name_stop.contains(&w.as_str()) {
            for id in graph.find_ids_by_name_token(&w) {
                if !name_seed_ids.contains(&id) {
                    seed_hits.push((id, 0.85));
                }
            }
        }
    }

    let graph_chunks: Vec<(i64, f64)> = if seed_hits.is_empty() {
        vec![]
    } else {
        // 2. BFS: collect all entity IDs within 2 hops.
        let seed_ids: Vec<i64> = seed_hits.iter().map(|(id, _)| *id).collect();
        let neighbor_ids = graph.bfs_neighbors(&seed_ids, 2);

        // 3. Collect all chunk IDs that mention any of these entities.
        let chunk_ids = graph.entity_chunks(&neighbor_ids);

        // 4. Score each chunk: base = 1.0 (presence), boost seed entity hits.
        let seed_set: HashSet<i64> = seed_ids.into_iter().collect();
        let seed_entity_chunks: HashSet<i64> = seed_hits
            .iter()
            .flat_map(|(eid, _)| graph.chunks_for_entity(*eid).iter().copied())
            .collect();

        chunk_ids
            .into_iter()
            .map(|cid| {
                let score = if seed_entity_chunks.contains(&cid) {
                    1.0
                } else {
                    0.6
                };
                let _ = &seed_set;
                (cid, score)
            })
            .collect()
    };

    // 5. Hybrid vector+BM25 retrieval.
    let all = meta.all_chunks()?;
    let triples: Vec<(i64, &str, &str)> = all
        .iter()
        .map(|(id, cm)| (*id, cm.doc_name.as_str(), cm.text.as_str()))
        .collect();
    let bm25 = BM25Index::build_in_ram(&triples)?;
    let semantic_raw = search_fn(embedding, candidate_k).await?;
    let keyword_raw = bm25.search(query, candidate_k);
    let vector_chunks = rrf_merge(&semantic_raw, &keyword_raw, candidate_k);

    // 6. RRF fusion: graph chunks + vector chunks.
    let merged = rrf_merge(&graph_chunks, &vector_chunks, cfg.top_k * 2);
    assemble_results(merged, cfg, meta)
}

pub(crate) fn assemble_results(
    raw: Vec<(i64, f64)>,
    cfg: &RetrieveConfig,
    meta: &MetaStore,
) -> Result<Vec<RetrievedChunk>> {
    let ids: Vec<i64> = raw.iter().map(|(id, _)| *id).collect();
    let metas = meta.get_chunks(&ids)?;

    let mut results: Vec<RetrievedChunk> = raw
        .into_iter()
        .zip(metas.into_iter())
        .filter_map(|((id, score), meta_opt)| {
            let _ = id;
            let chunk_meta = meta_opt?;
            if score < cfg.min_score {
                return None;
            }
            Some(RetrievedChunk {
                chunk_meta,
                score,
                source_kb: None,
                rerank_score: None,
            })
        })
        .collect();

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(cfg.top_k);
    Ok(results)
}
