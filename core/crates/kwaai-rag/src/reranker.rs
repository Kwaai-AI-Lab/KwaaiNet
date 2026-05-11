//! LLM listwise reranker — Phase 4.
//!
//! After hybrid retrieval collects a wide candidate pool (top_k * 4), this
//! makes one LLM call to reorder them by relevance, then truncates to top_k.
//! Uses a listwise prompt: send all passages at once, ask for a ranked index array.
//!
//! Falls back to the original order on any failure so the pipeline is never blocked.

use anyhow::Result;
use serde_json::json;

use crate::retriever::RetrievedChunk;

/// Rerank `candidates` against `query` using a single listwise LLM call.
/// Returns the top `top_k` chunks reordered by relevance.
/// On any failure (timeout, parse error, API error) returns the first `top_k` unchanged.
pub async fn rerank_chunks(
    query: &str,
    mut candidates: Vec<RetrievedChunk>,
    inference_url: &str,
    model: &str,
    top_k: usize,
) -> Vec<RetrievedChunk> {
    if candidates.len() <= 1 {
        candidates.truncate(top_k);
        return candidates;
    }

    match rerank_inner(query, &candidates, inference_url, model).await {
        Ok(order) => {
            let mut reranked: Vec<RetrievedChunk> = Vec::with_capacity(top_k);
            let n = candidates.len();
            for (rank, idx) in order.into_iter().enumerate() {
                if idx == 0 || idx > n {
                    continue; // 1-based; 0 or out-of-range = ignore
                }
                // Mark rerank_score: 1.0 for rank 1, decaying by 0.1 per position.
                let score = (1.0 - rank as f64 * 0.05).max(0.0);
                let mut chunk = candidates[idx - 1].clone();
                chunk.rerank_score = Some(score);
                reranked.push(chunk);
                if reranked.len() == top_k {
                    break;
                }
            }
            // If the LLM returned fewer indices than top_k, fill from the original order.
            if reranked.len() < top_k {
                let seen: std::collections::HashSet<*const RetrievedChunk> =
                    reranked.iter().map(|c| c as *const _).collect();
                for chunk in candidates.iter() {
                    if reranked.len() == top_k {
                        break;
                    }
                    if !seen.contains(&(chunk as *const _)) {
                        reranked.push(chunk.clone());
                    }
                }
            }
            reranked
        }
        Err(e) => {
            tracing::warn!("reranker failed, using original order: {e}");
            candidates.truncate(top_k);
            candidates
        }
    }
}

async fn rerank_inner(
    query: &str,
    candidates: &[RetrievedChunk],
    inference_url: &str,
    model: &str,
) -> Result<Vec<usize>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Build numbered passage list (truncate each to 300 chars to fit context).
    let passages: String = candidates
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let text: String = c.chunk_meta.text.chars().take(300).collect();
            format!("[{}] {}", i + 1, text)
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    let n = candidates.len();
    let prompt = format!(
        "Here are {n} passages. Return a JSON array of their numbers (1-based) \
         ordered from MOST to LEAST relevant to the question below. \
         Include all {n} numbers. Return ONLY the JSON array, no explanation.\n\n\
         Question: {query}\n\nPassages:\n{passages}"
    );

    let url = format!(
        "{}/v1/chat/completions",
        inference_url.trim_end_matches('/')
    );
    let body = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": 128,
    });

    let resp = client.post(&url).json(&body).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("reranker: inference API returned {}", resp.status());
    }

    let v: serde_json::Value = resp.json().await?;
    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim()
        .to_string();

    let indices: Vec<usize> = serde_json::from_str(&content)?;
    Ok(indices)
}
