use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

use anyhow::Result;

use crate::bm25::{rrf_merge, BM25Index};
use crate::embedder::EmbedClient;
use crate::graph::GraphStore;
use crate::meta_store::MetaStore;
use crate::retriever::{assemble_results, RetrieveConfig, RetrievedChunk};

const COVERAGE_R2: f32 = 0.70;
const COVERAGE_R3: f32 = 0.75;

static STOP_WORDS: &[&str] = &[
    "the", "and", "was", "were", "had", "has", "have", "been", "what", "who",
    "which", "when", "where", "tell", "about", "describe", "explain", "from",
    "with", "that", "this", "they", "their", "his", "her", "its", "our", "your",
    "for", "how", "did", "does", "also", "book", "author", "kind", "more",
];

fn coverage_terms(query: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    query
        .split_whitespace()
        .filter_map(|w| {
            // Strip non-alphanumeric edges, then drop possessive 's.
            let w = w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
            let w = w.strip_suffix("'s").or_else(|| w.strip_suffix("s'")).unwrap_or(&w).to_string();
            if w.len() >= 4 && !STOP_WORDS.contains(&w.as_str()) && seen.insert(w.clone()) {
                Some(w)
            } else {
                None
            }
        })
        .collect()
}

fn compute_coverage(terms: &[String], chunks: &[RetrievedChunk]) -> (f32, Vec<String>) {
    if terms.is_empty() {
        return (1.0, vec![]);
    }
    let text = chunks
        .iter()
        .map(|c| c.chunk_meta.text.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    let missing: Vec<String> = terms
        .iter()
        .filter(|t| !text.contains(t.as_str()))
        .cloned()
        .collect();
    let found = terms.len() - missing.len();
    (found as f32 / terms.len() as f32, missing)
}

fn chunk_key(c: &RetrievedChunk) -> (String, u32) {
    (c.chunk_meta.doc_name.clone(), c.chunk_meta.chunk_index)
}

async fn reformulate_query(
    original: &str,
    missing_terms: &[String],
    inference_url: &str,
    model: &str,
) -> Result<String> {
    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let system = "You are a search query expert. Given a question and missing search terms, \
                  write a short targeted search query (under 15 words) to find information \
                  about those terms. Return ONLY the query text, nothing else.";
    let user = format!(
        "Original question: {original}\nMissing terms: {}\nTargeted search query:",
        missing_terms.join(", ")
    );
    let payload = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user}
        ],
        "stream": false,
    });
    let resp = http
        .post(format!("{inference_url}/v1/chat/completions"))
        .json(&payload)
        .send()
        .await?;
    let body: serde_json::Value = resp.json().await?;
    Ok(body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or(original)
        .trim()
        .to_string())
}

/// Iterative retrieval: parallel vector+graph fusion, followed by up to two
/// gap-filling rounds (graph entity search, then LLM query reformulation).
///
/// `on_status` receives narration lines as retrieval progresses — print them
/// directly to give the user a live view of what the pipeline is doing.
pub async fn retrieve_iterative<S>(
    query: &str,
    cfg: &RetrieveConfig,
    embed: &EmbedClient,
    meta: &MetaStore,
    graph: &GraphStore,
    search_fn: S,
    inference_url: &str,
    model: &str,
    on_status: impl Fn(&str),
) -> Result<Vec<RetrievedChunk>>
where
    S: Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output = Result<Vec<(i64, f64)>>> + Send>>,
{
    let candidate_k = cfg.top_k * 4;

    let all = meta.all_chunks()?;
    let triples: Vec<(i64, &str, &str)> = all
        .iter()
        .map(|(id, cm)| (*id, cm.doc_name.as_str(), cm.text.as_str()))
        .collect();
    let bm25 = BM25Index::build_in_ram(&triples)?;

    // ── Round 1: vector+graph fusion ──────────────────────────────────────────

    let embedding = embed.embed_one(query).await?;
    let semantic_raw = search_fn(embedding.clone(), candidate_k).await?;
    let keyword_raw = bm25.search(query, candidate_k);
    let vector_raw = rrf_merge(&semantic_raw, &keyword_raw, candidate_k);

    let seed_hits = graph.search_entities(&embedding, 3);
    let graph_raw: Vec<(i64, f64)> = if seed_hits.is_empty() {
        vec![]
    } else {
        let seed_ids: Vec<i64> = seed_hits.iter().map(|(id, _)| *id).collect();
        let neighbors = graph.bfs_neighbors(&seed_ids, 2);
        let chunk_ids = graph.entity_chunks(&neighbors);
        let seed_chunk_set: HashSet<i64> = seed_hits
            .iter()
            .flat_map(|(eid, _)| graph.chunks_for_entity(*eid).iter().copied())
            .collect();
        chunk_ids
            .into_iter()
            .map(|cid| (cid, if seed_chunk_set.contains(&cid) { 1.0 } else { 0.6 }))
            .collect()
    };

    let fused_raw = rrf_merge(&graph_raw, &vector_raw, candidate_k);
    let over_cfg = RetrieveConfig { top_k: candidate_k, ..cfg.clone() };
    let mut pool = assemble_results(fused_raw, &over_cfg, meta)?;

    let doc_count = pool
        .iter()
        .map(|c| &c.chunk_meta.doc_name)
        .collect::<HashSet<_>>()
        .len();
    on_status(&format!(
        "\n  ○ Round 1   vector+graph fusion → {} chunks from {} documents",
        pool.len(),
        doc_count
    ));

    // ── Coverage check ────────────────────────────────────────────────────────

    let terms = coverage_terms(query);
    let (coverage, missing) = compute_coverage(&terms, &pool);
    on_status(&format!(
        "  ○ Coverage  {}/{} query terms found  ({:.0}%)",
        terms.len() - missing.len(),
        terms.len(),
        coverage * 100.0
    ));

    if coverage >= COVERAGE_R2 || terms.is_empty() {
        on_status("  ○ Coverage  satisfied — skipping gap-fill rounds");
    } else {
        // ── Round 2: graph entity gap-fill ────────────────────────────────────

        on_status(&format!(
            "  ○ Round 2   gap-filling for [{}]",
            missing
                .iter()
                .take(5)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ")
        ));

        let gap_emb = embed.embed_one(&missing.join(" ")).await?;
        let gap_hits = graph.search_entities(&gap_emb, 6);
        let existing_keys: HashSet<_> = pool.iter().map(chunk_key).collect();

        let gap_added = if !gap_hits.is_empty() {
            let gap_ids: Vec<i64> = gap_hits.iter().map(|(id, _)| *id).collect();
            let gap_neighbors = graph.bfs_neighbors(&gap_ids, 2);
            let gap_chunk_ids = graph.entity_chunks(&gap_neighbors);
            let new_metas = meta.get_chunks(&gap_chunk_ids)?;
            let new_chunks: Vec<RetrievedChunk> = gap_chunk_ids
                .into_iter()
                .zip(new_metas)
                .filter_map(|(_, meta_opt)| {
                    let cm = meta_opt?;
                    let key = (cm.doc_name.clone(), cm.chunk_index);
                    if existing_keys.contains(&key) {
                        return None;
                    }
                    Some(RetrievedChunk {
                        chunk_meta: cm,
                        score: 0.45,
                        source_kb: None,
                        rerank_score: None,
                    })
                })
                .collect();
            let added = new_chunks.len();
            pool.extend(new_chunks);
            added
        } else {
            0
        };

        on_status(&format!(
            "  ○ Round 2   added {gap_added} chunks via graph gap-fill"
        ));

        let (coverage2, missing2) = compute_coverage(&terms, &pool);

        // ── Round 3: LLM query reformulation ──────────────────────────────────

        if coverage2 < COVERAGE_R3 && !missing2.is_empty() && !inference_url.is_empty() {
            on_status(&format!(
                "  ○ Round 3   LLM reformulation for [{}]",
                missing2
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));

            match reformulate_query(query, &missing2, inference_url, model).await {
                Ok(ref_query) => {
                    let preview: String = ref_query.chars().take(70).collect();
                    on_status(&format!("  ○ Round 3   → \"{preview}\""));

                    if let Ok(ref_emb) = embed.embed_one(&ref_query).await {
                        let ref_sem = search_fn(ref_emb, candidate_k / 2).await?;
                        let ref_kw = bm25.search(&ref_query, candidate_k / 2);
                        let ref_raw = rrf_merge(&ref_sem, &ref_kw, candidate_k / 2);
                        let ref_cfg =
                            RetrieveConfig { top_k: candidate_k / 2, ..cfg.clone() };
                        let ref_chunks = assemble_results(ref_raw, &ref_cfg, meta)?;

                        let existing_keys2: HashSet<_> = pool.iter().map(chunk_key).collect();
                        let new: Vec<_> = ref_chunks
                            .into_iter()
                            .filter(|c| !existing_keys2.contains(&chunk_key(c)))
                            .collect();
                        on_status(&format!(
                            "  ○ Round 3   added {} chunks from reformulated query",
                            new.len()
                        ));
                        pool.extend(new);
                    }
                }
                Err(e) => {
                    on_status(&format!("  ○ Round 3   reformulation failed ({e})"));
                }
            }
        }
    }

    // ── Final: boost by term coverage, dedup, select top-k ───────────────────

    for chunk in &mut pool {
        let text = chunk.chunk_meta.text.to_lowercase();
        let hits = terms
            .iter()
            .filter(|t| text.contains(t.as_str()))
            .count();
        chunk.score += hits as f64 * 0.05;
    }
    pool.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut seen = HashSet::new();
    pool.retain(|c| seen.insert(chunk_key(c)));
    pool.truncate(cfg.top_k);

    let final_docs = pool
        .iter()
        .map(|c| &c.chunk_meta.doc_name)
        .collect::<HashSet<_>>()
        .len();
    on_status(&format!(
        "  ○ Final     {} chunks from {} documents — passing to LLM\n",
        pool.len(),
        final_docs
    ));

    Ok(pool)
}
