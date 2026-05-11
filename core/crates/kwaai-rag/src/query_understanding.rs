//! Query understanding: sub-query decomposition + multi-retrieval merge.
//!
//! Implements the MultiQuery pattern: an LLM breaks a complex question into N
//! atomic sub-queries, each is retrieved independently, results are deduplicated
//! by chunk ID, and the best score per chunk is kept before final re-ranking.
//!
//! This improves recall on multi-hop and ambiguous questions where a single
//! embedding vector misses complementary aspects of the question.

use std::future::Future;
use std::pin::Pin;

use anyhow::{Context, Result};
use serde_json::json;

use crate::embedder::EmbedClient;
use crate::meta_store::MetaStore;
use crate::retriever::{retrieve_hybrid, RetrieveConfig, RetrievedChunk};

/// Ask the LLM to decompose `query` into up to `n_variants` focused sub-queries.
///
/// Returns the original query plus the generated variants (deduplicated).
/// Falls back to `[query]` if the LLM call fails, so callers are always safe.
pub async fn decompose_query(
    query: &str,
    n_variants: usize,
    inference_url: &str,
    model: &str,
) -> Vec<String> {
    match decompose_inner(query, n_variants, inference_url, model).await {
        Ok(mut qs) => {
            // Always include the original in case decomposition is lossy.
            if !qs.contains(&query.to_string()) {
                qs.insert(0, query.to_string());
            }
            qs
        }
        Err(e) => {
            tracing::warn!("query decomposition failed, using raw query: {e}");
            vec![query.to_string()]
        }
    }
}

async fn decompose_inner(
    query: &str,
    n_variants: usize,
    inference_url: &str,
    model: &str,
) -> Result<Vec<String>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let prompt = format!(
        "You are an expert at query decomposition for retrieval-augmented generation.\n\
         Break the following question into {n_variants} distinct, focused sub-questions that \
         together cover all aspects of the original question. Each sub-question should be \
         self-contained and optimised for keyword and semantic search.\n\
         Return ONLY a JSON array of strings. No explanation, no markdown fences.\n\n\
         Question: {query}"
    );

    let url = format!(
        "{}/v1/chat/completions",
        inference_url.trim_end_matches('/')
    );
    let body = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.3,
        "max_tokens": 512,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .context("calling inference API for query decomposition")?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        anyhow::bail!("inference API returned {status}: {text}");
    }

    let v: serde_json::Value = serde_json::from_str(&text).context("parsing inference response")?;
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .context("missing content in response")?;

    // Parse the JSON array from the LLM output.
    // The LLM may sometimes wrap it in markdown fences; strip them first.
    let cleaned = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let sub_queries: Vec<String> =
        serde_json::from_str(cleaned).context("LLM did not return a valid JSON array")?;

    Ok(sub_queries
        .into_iter()
        .filter(|q| !q.trim().is_empty())
        .take(n_variants)
        .collect())
}

/// Retrieve with query understanding: decompose → multi-retrieve → deduplicate → re-rank.
///
/// Each sub-query runs a full hybrid retrieval. Results are merged by keeping the
/// highest score seen for each chunk ID across all sub-queries, then re-sorted.
pub async fn retrieve_with_understanding<F>(
    query: &str,
    cfg: &RetrieveConfig,
    embed: &EmbedClient,
    meta: &MetaStore,
    inference_url: &str,
    model: &str,
    search_fn: F,
) -> Result<Vec<RetrievedChunk>>
where
    F: Fn(Vec<f32>, usize) -> Pin<Box<dyn Future<Output = Result<Vec<(i64, f64)>>> + Send>>
        + Clone
        + Send,
{
    let sub_queries = decompose_query(query, 3, inference_url, model).await;

    tracing::debug!(
        "query understanding: {} sub-queries for {:?}",
        sub_queries.len(),
        query
    );

    // Retrieve for each sub-query, collect all chunks.
    let mut best_by_id: std::collections::HashMap<i64, RetrievedChunk> =
        std::collections::HashMap::new();

    for sq in &sub_queries {
        let results = retrieve_hybrid(sq, cfg, embed, meta, search_fn.clone()).await?;
        for chunk in results {
            let stable_key =
                stable_chunk_key(&chunk.chunk_meta.doc_name, chunk.chunk_meta.chunk_index);
            best_by_id
                .entry(stable_key)
                .and_modify(|existing| {
                    if chunk.score > existing.score {
                        *existing = chunk.clone();
                    }
                })
                .or_insert(chunk);
        }
    }

    let mut merged: Vec<RetrievedChunk> = best_by_id.into_values().collect();
    merged.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    merged.truncate(cfg.top_k);
    Ok(merged)
}

fn stable_chunk_key(doc_name: &str, chunk_index: u32) -> i64 {
    use std::hash::Hash;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    doc_name.hash(&mut h);
    chunk_index.hash(&mut h);
    std::hash::Hasher::finish(&h) as i64
}
