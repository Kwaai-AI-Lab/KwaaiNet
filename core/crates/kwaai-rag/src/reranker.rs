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
/// A user's relevance judgements, keyed by [`crate::iterative::chunk_key`].
/// `+1` = relevant, `-1` = irrelevant.
pub type Marks = std::collections::HashMap<(String, u32), i8>;

/// Apply relevance marks deterministically: drop `-1`, hoist `+1` to the front in
/// their existing relative order, leave everything else untouched.
///
/// Deliberately not a hint to the LLM. A human-in-the-loop control should do exactly
/// and visibly what the user asked, not nudge a sampler.
pub fn apply_marks(chunks: Vec<RetrievedChunk>, marks: &Marks) -> Vec<RetrievedChunk> {
    if marks.is_empty() {
        return chunks;
    }
    let mut pinned = Vec::new();
    let mut rest = Vec::new();
    for c in chunks {
        match marks.get(&crate::iterative::chunk_key(&c)).copied() {
            Some(m) if m < 0 => continue,
            Some(m) if m > 0 => pinned.push(c),
            _ => rest.push(c),
        }
    }
    pinned.append(&mut rest);
    pinned
}

pub async fn rerank_chunks(
    query: &str,
    candidates: Vec<RetrievedChunk>,
    inference_url: &str,
    model: &str,
    top_k: usize,
) -> Vec<RetrievedChunk> {
    rerank_chunks_with_marks(
        query,
        candidates,
        inference_url,
        model,
        top_k,
        &Marks::new(),
    )
    .await
}

/// Listwise rerank, honouring the user's relevance marks.
///
/// Marked-irrelevant chunks are dropped *before* the LLM call, so they neither reach
/// the prompt nor cost tokens; marked-relevant chunks are pinned to the front and the
/// model reranks only what is left.
pub async fn rerank_chunks_with_marks(
    query: &str,
    candidates: Vec<RetrievedChunk>,
    inference_url: &str,
    model: &str,
    top_k: usize,
    marks: &Marks,
) -> Vec<RetrievedChunk> {
    let candidates = apply_marks(candidates, marks);
    let n_pinned = candidates
        .iter()
        .take_while(|c| {
            marks
                .get(&crate::iterative::chunk_key(c))
                .copied()
                .unwrap_or(0)
                > 0
        })
        .count();

    if candidates.len() <= 1 || n_pinned >= top_k {
        let mut out = candidates;
        out.truncate(top_k);
        return out;
    }

    let mut out: Vec<RetrievedChunk> = candidates[..n_pinned].to_vec();
    for c in out.iter_mut() {
        c.rerank_score = Some(1.0);
    }
    let rest = candidates[n_pinned..].to_vec();
    let want = top_k - n_pinned;
    out.extend(rerank_listwise(query, rest, inference_url, model, want).await);
    out
}

async fn rerank_listwise(
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
            // Track what we have taken by chunk identity. The previous implementation
            // used pointer identity over freshly cloned values, which could never
            // match, so the back-fill below silently appended duplicates.
            let mut seen: std::collections::HashSet<(String, u32)> =
                std::collections::HashSet::new();
            let n = candidates.len();
            for (rank, idx) in order.into_iter().enumerate() {
                if idx == 0 || idx > n {
                    continue; // 1-based; 0 or out-of-range = ignore
                }
                let mut chunk = candidates[idx - 1].clone();
                let key = crate::iterative::chunk_key(&chunk);
                if !seen.insert(key) {
                    continue; // the model repeated an index
                }
                // Mark rerank_score: 1.0 for rank 1, decaying by 0.05 per position.
                chunk.rerank_score = Some((1.0 - rank as f64 * 0.05).max(0.0));
                reranked.push(chunk);
                if reranked.len() == top_k {
                    break;
                }
            }
            // If the LLM returned fewer indices than top_k, fill from the original order.
            if reranked.len() < top_k {
                for chunk in candidates.iter() {
                    if reranked.len() == top_k {
                        break;
                    }
                    if seen.insert(crate::iterative::chunk_key(chunk)) {
                        reranked.push(chunk.clone());
                    }
                }
            }
            reranked
        }
        Err(e) => {
            tracing::debug!("reranker failed, using original order: {e}");
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
    // Each index needs ~4 chars + separators; give a comfortable margin.
    let max_tokens = (n * 6).clamp(256, 512) as u64;
    let body = json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": max_tokens,
    });

    let resp = client.post(&url).json(&body).send().await?;

    if !resp.status().is_success() {
        anyhow::bail!("reranker: inference API returned {}", resp.status());
    }

    let v: serde_json::Value = resp.json().await?;
    let raw = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string();

    // Extract the JSON array even if the model wraps it in fences or adds preamble.
    let content = if let (Some(start), Some(end)) = (raw.find('['), raw.rfind(']')) {
        &raw[start..=end]
    } else {
        raw.trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    };

    let indices: Vec<usize> = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("could not parse reranker response as index array: {e}\nraw: {raw:?}")
    })?;
    Ok(indices)
}

#[cfg(test)]
mod marks_tests {
    use super::*;
    use crate::meta_store::ChunkMeta;

    fn chunk(doc: &str, idx: u32) -> RetrievedChunk {
        RetrievedChunk {
            chunk_meta: ChunkMeta {
                doc_name: doc.into(),
                chunk_index: idx,
                text: format!("{doc}#{idx}"),
                surrounding: String::new(),
                page_num: None,
                ingested_at: String::new(),
                section_name: None,
                skip_extraction: false,
                section_note: None,
                section_type: crate::doc_schema::SectionType::Main,
            },
            score: 0.0,
            source_kb: None,
            rerank_score: None,
        }
    }

    fn keys(v: &[RetrievedChunk]) -> Vec<(String, u32)> {
        v.iter().map(crate::iterative::chunk_key).collect()
    }

    #[test]
    fn marks_drop_negative_and_hoist_positive_stably() {
        let chunks: Vec<RetrievedChunk> = (0..5).map(|i| chunk("d", i)).collect();
        let mut marks = Marks::new();
        marks.insert(("d".into(), 1), -1);
        marks.insert(("d".into(), 4), 1);
        marks.insert(("d".into(), 2), 1);

        let out = apply_marks(chunks, &marks);
        // Pinned keep their original relative order (2 before 4), then the unmarked.
        assert_eq!(
            keys(&out),
            vec![
                ("d".into(), 2),
                ("d".into(), 4),
                ("d".into(), 0),
                ("d".into(), 3)
            ]
        );
    }

    #[test]
    fn empty_marks_are_identity() {
        let chunks: Vec<RetrievedChunk> = (0..4).map(|i| chunk("d", i)).collect();
        let before = keys(&chunks);
        assert_eq!(keys(&apply_marks(chunks, &Marks::new())), before);
    }

    /// Regression: the back-fill used to compare `*const` pointers taken from freshly
    /// cloned values against the originals. They could never match, so the dedup was a
    /// no-op and already-selected chunks were appended a second time whenever the model
    /// returned fewer indices than `top_k`.
    #[test]
    fn backfill_does_not_duplicate() {
        let candidates: Vec<RetrievedChunk> = (0..5).map(|i| chunk("d", i)).collect();
        // Simulate the model naming only index 3, with top_k = 4.
        let mut reranked: Vec<RetrievedChunk> = Vec::new();
        let mut seen: std::collections::HashSet<(String, u32)> = std::collections::HashSet::new();
        let c = candidates[2].clone();
        seen.insert(crate::iterative::chunk_key(&c));
        reranked.push(c);
        for c in candidates.iter() {
            if reranked.len() == 4 {
                break;
            }
            if seen.insert(crate::iterative::chunk_key(c)) {
                reranked.push(c.clone());
            }
        }
        let k = keys(&reranked);
        let mut uniq = k.clone();
        uniq.sort();
        uniq.dedup();
        assert_eq!(
            k.len(),
            uniq.len(),
            "duplicate chunks in reranked output: {k:?}"
        );
        assert_eq!(k.len(), 4);
    }
}
