//! HyDE — Hypothetical Document Embeddings.
//!
//! Instead of embedding the raw query, ask the LLM to write a short hypothetical
//! answer and embed that. Document-to-document similarity is much tighter than
//! query-to-document, so retrieved chunks are more relevant.
//!
//! Falls back to plain query embedding on any LLM or network failure, so the
//! pipeline is never blocked.

use anyhow::{Context, Result};
use serde_json::json;

use crate::embedder::EmbedClient;

/// Ask the LLM to write a short hypothetical answer paragraph for `query`.
/// The result is used for embedding only — not shown to the user.
pub async fn generate_hypothetical_answer(
    query: &str,
    inference_url: &str,
    model: &str,
) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()?;

    let prompt = format!(
        "Write a concise, factual paragraph (2–4 sentences) that directly answers \
         the following question, as if it were an excerpt from a relevant document. \
         Use specific details. Do not mention the question itself.\n\nQuestion: {query}"
    );

    let url = format!("{}/v1/chat/completions", inference_url.trim_end_matches('/'));
    let body = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.1,
        "max_tokens": 256,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .context("HyDE: calling inference API")?;

    if !resp.status().is_success() {
        anyhow::bail!("HyDE: inference API returned {}", resp.status());
    }

    let v: serde_json::Value = resp.json().await.context("HyDE: parsing response")?;
    let answer = v["choices"][0]["message"]["content"]
        .as_str()
        .context("HyDE: missing content in response")?
        .trim()
        .to_string();

    tracing::debug!(len = answer.len(), "HyDE hypothetical generated");
    Ok(answer)
}

/// Embed the query using HyDE. Falls back to plain query embedding on failure.
pub async fn embed_with_hyde(
    query: &str,
    embed: &EmbedClient,
    inference_url: &str,
    model: &str,
) -> Vec<f32> {
    match generate_hypothetical_answer(query, inference_url, model).await {
        Ok(hypothetical) => match embed.embed_one(&hypothetical).await {
            Ok(emb) => {
                tracing::debug!("HyDE embedding succeeded");
                emb
            }
            Err(e) => {
                tracing::warn!("HyDE embed failed, falling back to query embedding: {e}");
                embed.embed_one(query).await.unwrap_or_default()
            }
        },
        Err(e) => {
            tracing::warn!("HyDE generation failed, falling back to query embedding: {e}");
            embed.embed_one(query).await.unwrap_or_default()
        }
    }
}

/// Blend original query embedding with a HyDE embedding.
///
/// `alpha = 0.0` returns the plain query embedding (no HyDE effect).
/// `alpha = 1.0` returns the pure HyDE embedding.
/// `alpha = 0.5` (default) is an equal blend — prevents factoid regressions while
/// still benefiting from document-to-document similarity for descriptive queries.
///
/// Falls back to plain query embedding if HyDE generation or embedding fails.
pub async fn embed_with_hyde_blend(
    query: &str,
    embed: &EmbedClient,
    inference_url: &str,
    model: &str,
    alpha: f32,
) -> Vec<f32> {
    let query_emb = match embed.embed_one(query).await {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!("query embedding failed: {e}");
            return vec![];
        }
    };

    if alpha <= 0.0 {
        return query_emb;
    }

    let hyde_emb = match generate_hypothetical_answer(query, inference_url, model).await {
        Ok(hypothetical) => match embed.embed_one(&hypothetical).await {
            Ok(emb) => {
                tracing::debug!("HyDE blend embedding succeeded");
                emb
            }
            Err(e) => {
                tracing::warn!("HyDE blend embed failed, using pure query embedding: {e}");
                return query_emb;
            }
        },
        Err(e) => {
            tracing::warn!("HyDE blend generation failed, using pure query embedding: {e}");
            return query_emb;
        }
    };

    if alpha >= 1.0 {
        return hyde_emb;
    }

    // Linear blend then L2-normalise.
    let blended: Vec<f32> = query_emb
        .iter()
        .zip(hyde_emb.iter())
        .map(|(q, h)| (1.0 - alpha) * q + alpha * h)
        .collect();

    let norm = blended.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        blended.into_iter().map(|x| x / norm).collect()
    } else {
        blended
    }
}
