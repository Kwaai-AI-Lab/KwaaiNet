use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use anyhow::Result;
use tracing::debug;

use crate::bm25::{rrf_merge, BM25Index};
use crate::embedder::EmbedClient;
use crate::graph::GraphStore;
use crate::hyde::{embed_with_hyde, embed_with_hyde_blend};
use crate::meta_store::{ChunkMeta, MetaStore};

/// Replace entity alias mentions in query with canonical names before embedding.
///
/// Builds an alias→canonical map from the graph (sorted longest-alias-first to
/// prevent partial matches), then does a case-insensitive whole-word substitution.
/// Only the first match per alias is replaced to prevent cascading rewrites.
/// The original query is preserved for BM25 (which benefits from matching the
/// alias form present in the source text).
///
/// Example: "Who was J.M.H. Gool?" → "Who was Haji Joosub Maulvi Hamid Gool?"
pub fn canonicalize_query(query: &str, graph: &GraphStore) -> String {
    let mut pairs: Vec<(String, String)> = graph
        .all_entities()
        .flat_map(|e| {
            e.aliases
                .iter()
                .filter(|a| a.len() >= 3)
                .map(|a| (a.to_lowercase(), e.name.clone()))
                .collect::<Vec<_>>()
        })
        .collect();
    // Longest alias first — prevents "Gool" matching before "J.M.H. Gool"
    pairs.sort_by_key(|b| std::cmp::Reverse(b.0.len()));

    let mut result = query.to_string();
    let mut result_lower = result.to_lowercase();

    for (alias_lower, canonical) in &pairs {
        let Some(idx) = result_lower.find(alias_lower.as_str()) else {
            continue;
        };
        // Whole-word boundary check
        let before_ok = idx == 0
            || !result_lower
                .chars()
                .nth(idx - 1)
                .is_some_and(|c| c.is_alphanumeric());
        let end = idx + alias_lower.len();
        let after_ok = end >= result_lower.len()
            || !result_lower
                .chars()
                .nth(end)
                .is_some_and(|c| c.is_alphanumeric());
        if before_ok && after_ok {
            debug!(
                alias = alias_lower.as_str(),
                canonical, "query alias → canonical"
            );
            result = format!("{}{}{}", &result[..idx], canonical, &result[end..]);
            result_lower = result.to_lowercase();
        }
    }
    result
}

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

    // Substitute entity alias forms with canonical names before embedding so the
    // query vector clusters near the correctly-normalised entity descriptions.
    // BM25 still runs against the original query (alias forms match source text better).
    let canonical_query = canonicalize_query(query, graph);
    let embed_query = if canonical_query != query {
        canonical_query.as_str()
    } else {
        query
    };

    // Dense embedding — use HyDE (optionally blended) if configured, else plain query embedding.
    let embedding = match (&cfg.hyde_inference_url, &cfg.hyde_model) {
        (Some(url), Some(model)) => match cfg.hyde_alpha {
            Some(alpha) => embed_with_hyde_blend(embed_query, embed, url, model, alpha).await,
            None => embed_with_hyde(embed_query, embed, url, model).await,
        },
        _ => embed.embed_one(embed_query).await?,
    };

    // 1. Find seed entities: embedding similarity + name-token matching.
    //    Embedding search alone fails for abbreviations/acronyms (e.g. "J.M.H. Gool"
    //    doesn't match the description embedding of the canonical entity). Name-token
    //    matching catches those cases by finding entities whose name contains any
    //    significant query word as a whole token.
    let mut seed_hits = graph.search_entities(&embedding, 5);
    let name_stop: &[&str] = &[
        "who", "what", "was", "were", "the", "tell", "about", "and", "for", "did", "how", "where",
        "when", "describe", "more", "kind", "place",
    ];
    let emb_seed_ids: std::collections::HashSet<i64> =
        seed_hits.iter().map(|(id, _)| *id).collect();
    let mut name_matched_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();
    for word in query.split_whitespace() {
        let w = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if w.len() >= 3 && !name_stop.contains(&w.as_str()) {
            for id in graph.find_ids_by_name_token(&w) {
                if !emb_seed_ids.contains(&id) {
                    seed_hits.push((id, 0.85));
                }
                name_matched_ids.insert(id);
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
    let mut results = assemble_results(merged, cfg, meta)?;
    inject_entity_descriptions(query, &seed_hits, &name_matched_ids, graph, &mut results);
    Ok(results)
}

/// Returns true when `entity_id` is the KB narrator / author.
///
/// Detection is purely alias-based so it generalises across KBs: any entity
/// with an alias of "author", "narrator", "the author", etc. qualifies.
/// Seed files should declare the narrator entity with one of these aliases
/// (e.g. `aliases: [Author]` in the family-tree YAML).
fn is_author_entity(entity_id: i64, graph: &GraphStore) -> bool {
    let Some(entity) = graph.get_entity(entity_id) else {
        return false;
    };
    entity.aliases.iter().any(|a| {
        matches!(
            a.to_lowercase().as_str(),
            "author" | "the author" | "narrator" | "the narrator" | "the writer"
        )
    })
}

/// Resolve a query about the author to the specific relative being asked about.
///
/// Uses the adjacency list relation types seeded by the family tree to walk
/// spouse_of / child_of / sibling_of edges from Yousuf Rassool's node.
fn resolve_author_relative(query: &str, anchor_id: i64, graph: &GraphStore) -> Option<i64> {
    let q = query.to_lowercase();
    let neighbors = graph.neighbors_of(anchor_id);

    // Wife / spouse
    if q.contains("wife") || q.contains("spouse") {
        return neighbors
            .iter()
            .find(|(_, rel, _)| rel == "spouse_of")
            .map(|(id, _, _)| *id);
    }

    // Mother
    if q.contains("mother") || q.contains(" mom") || q.contains("mama") {
        return neighbors
            .iter()
            .filter(|(_, rel, _)| rel == "child_of")
            .find(|(id, _, _)| {
                graph
                    .get_entity(*id)
                    .and_then(|e| e.gender.clone())
                    .as_deref()
                    == Some("Female")
            })
            .map(|(id, _, _)| *id);
    }

    // Father
    if q.contains("father") || q.contains(" dad") || q.contains("papa") {
        return neighbors
            .iter()
            .filter(|(_, rel, _)| rel == "child_of")
            .find(|(id, _, _)| {
                graph
                    .get_entity(*id)
                    .and_then(|e| e.gender.clone())
                    .as_deref()
                    == Some("Male")
            })
            .map(|(id, _, _)| *id);
    }

    // Grandfather / grandpa — prefer entities with an explicit "grandfather" alias seeded by
    // the family tree (reliable) over graph-traversal through noisy LLM-extracted edges.
    if q.contains("grandfather") || q.contains("grandpa") || q.contains("grandfath") {
        // Pass 1: alias-based lookup across all graph entities
        for e in graph.all_entities() {
            if e.aliases.iter().any(|a| {
                let lc = a.to_lowercase();
                lc == "grandfather"
                    || lc == "my grandfather"
                    || lc == "grandpa"
                    || lc == "my grandpa"
            }) {
                return Some(e.id);
            }
        }
        // Pass 2: graph traversal fallback (noisy but best-effort)
        let parents: Vec<i64> = neighbors
            .iter()
            .filter(|(_, rel, _)| rel == "child_of")
            .map(|(id, _, _)| *id)
            .collect();
        for parent_id in &parents {
            for (gp_id, rel, _) in graph.neighbors_of(*parent_id) {
                if rel == "child_of"
                    && graph
                        .get_entity(gp_id)
                        .and_then(|e| e.gender.clone())
                        .as_deref()
                        == Some("Male")
                {
                    return Some(gp_id);
                }
            }
        }
        return None;
    }

    // Siblings
    if q.contains("sibling") || q.contains("brother") || q.contains("sister") {
        let want_gender = if q.contains("sister") {
            Some("Female")
        } else if q.contains("brother") {
            Some("Male")
        } else {
            None
        };
        return neighbors
            .iter()
            .filter(|(_, rel, _)| rel == "sibling_of")
            .find(|(id, _, _)| {
                want_gender.is_none_or(|g| {
                    graph
                        .get_entity(*id)
                        .and_then(|e| e.gender.clone())
                        .as_deref()
                        == Some(g)
                })
            })
            .map(|(id, _, _)| *id);
    }

    None
}

/// Prepend a synthetic chunk for the most relevant graph entity.
///
/// When the top matched entity is the memoir author (Yousuf Rassool), resolves
/// the query to the specific relative being asked about (wife, grandfather, mother…)
/// and injects that relative's description instead.  Otherwise injects the top
/// embedding-matched entity's description.  Exactly one synthetic chunk is added
/// so the context window is not crowded.
pub(crate) fn inject_entity_descriptions(
    query: &str,
    seed_hits: &[(i64, f64)],
    name_matched: &std::collections::HashSet<i64>,
    graph: &GraphStore,
    pool: &mut Vec<RetrievedChunk>,
) {
    let q_lower = query.to_lowercase();
    let is_relative_query = [
        "wife",
        "spouse",
        "husband",
        "mother",
        "father",
        "grandfather",
        "grandmother",
        "grandchild",
        "grandchildren",
        "sibling",
        "brother",
        "sister",
    ]
    .iter()
    .any(|kw| q_lower.contains(kw));

    // For personal-relative queries (wife, grandfather, etc.) we MUST land on the
    // author entity so resolve_author_relative() can walk family graph edges.
    // If the author isn't in the seed hits, skip injection rather than injecting
    // a spurious entity (e.g. a venue whose description happens to contain "wife").
    let (anchor_id, inject_id): (i64, i64) = if is_relative_query {
        // Try embedding seed hits first, then fall back to a direct name-token lookup.
        // The author entity (Joe Rassool / Yousuf Rassool) has alias "author" but that
        // alias won't appear in entity names, so embedding hits may miss it entirely.
        let author_id = seed_hits
            .iter()
            .find(|(id, _)| is_author_entity(*id, graph))
            .map(|(id, _)| *id)
            .or_else(|| {
                graph
                    .find_ids_by_name_token("rassool")
                    .into_iter()
                    .find(|id| is_author_entity(*id, graph))
            });
        let Some(aid) = author_id else { return };
        let iid = resolve_author_relative(query, aid, graph).unwrap_or(aid);
        (aid, iid)
    } else {
        // For non-relative queries: try candidates in descending score order and pick the
        // first one whose description passes the quality gate. This avoids skipping injection
        // entirely when the highest-scoring entity happens to have a thin description.
        let desc_ok = |id: i64| {
            let Some(e) = graph.get_entity(id) else {
                return false;
            };
            let desc = e.description.trim();
            let sents = desc
                .chars()
                .filter(|c| matches!(c, '.' | '?' | '!'))
                .count();
            if name_matched.contains(&id) {
                desc.len() >= 40 && sents >= 1
            } else {
                desc.len() >= 100 && sents >= 2
            }
        };
        let candidate = seed_hits
            .iter()
            .filter(|(_, s)| *s > 0.85)
            .chain(seed_hits.iter().filter(|(_, s)| *s > 0.7 && *s <= 0.85))
            .find(|(id, _)| desc_ok(*id));
        let Some((id, _)) = candidate else { return };
        (*id, *id)
    };

    let Some(entity) = graph.get_entity(inject_id) else {
        return;
    };
    // Description quality gate (already evaluated above for non-relative; re-checked here
    // for relative queries where inject_id may differ from anchor_id).
    let desc = entity.description.trim();
    let sentences = desc
        .chars()
        .filter(|c| matches!(c, '.' | '?' | '!'))
        .count();
    let is_name_matched = name_matched.contains(&inject_id);
    let is_resolved_relative = is_relative_query && inject_id != anchor_id;
    let use_lenient = is_name_matched || is_resolved_relative;
    if use_lenient {
        if desc.len() < 40 || sentences < 1 {
            return;
        }
    } else if desc.len() < 100 || sentences < 2 {
        return;
    }

    let synthetic = RetrievedChunk {
        chunk_meta: ChunkMeta {
            doc_name: format!("[Graph: {}]", entity.name),
            chunk_index: 0,
            text: format!("{}: {}", entity.name, entity.description),
            surrounding: String::new(),
            page_num: None,
            ingested_at: String::new(),
            section_name: None,
            skip_extraction: false,
            section_note: None,
        },
        score: 2.0,
        source_kb: None,
        rerank_score: None,
    };

    pool.insert(0, synthetic);
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
        .zip(metas)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{EntityNode, GraphStore};
    use tempfile::tempdir;

    fn make_store_with_alias(name: &str, alias: &str) -> (GraphStore, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let mut store = GraphStore::open(dir.path(), uuid::Uuid::new_v4()).unwrap();
        let id = crate::graph::entity_id(name, "Person");
        store
            .upsert_entity(EntityNode {
                id,
                name: name.to_string(),
                entity_type: "Person".to_string(),
                description: String::new(),
                embedding: vec![],
                mention_count: 1,
                first_chunk_id: 0,
                aliases: vec![alias.to_string()],
                schema_type: None,
                evidence: Vec::new(),
                gender: None,
                fields: Default::default(),
            })
            .unwrap();
        (store, dir)
    }

    #[test]
    fn canonicalize_substitutes_alias() {
        let (store, _dir) = make_store_with_alias("Canonical Full Name", "CFN");
        let result = canonicalize_query("Who was CFN?", &store);
        assert_eq!(result, "Who was Canonical Full Name?");
    }

    #[test]
    fn canonicalize_no_match_unchanged() {
        let (store, _dir) = make_store_with_alias("Canonical Full Name", "CFN");
        let result = canonicalize_query("Tell me about something else.", &store);
        assert_eq!(result, "Tell me about something else.");
    }

    #[test]
    fn canonicalize_case_insensitive() {
        let (store, _dir) = make_store_with_alias("Global Standards Body", "GSB");
        let result = canonicalize_query("What was the gsb?", &store);
        assert_eq!(result, "What was the Global Standards Body?");
    }

    #[test]
    fn canonicalize_word_boundary_only() {
        let (store, _dir) = make_store_with_alias("Target Entity", "TGT");
        // "TGT" should not match inside "XTGTX"
        let result = canonicalize_query("Tell me about XTGTX.", &store);
        assert_eq!(result, "Tell me about XTGTX.");
    }
}
