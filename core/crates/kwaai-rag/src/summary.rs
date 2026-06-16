use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::embedder::EmbedClient;
use crate::meta_store::MetaStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryNode {
    pub id: i64,
    pub doc_name: String,
    /// 1 = window summary (N contiguous chunks), 2 = section summary.
    pub level: u8,
    pub chunk_ids: Vec<i64>,
    pub text: String,
    pub embedding: Vec<f32>,
    #[serde(default)]
    pub section_name: Option<String>,
}

pub struct SummaryStore {
    pub nodes: Vec<SummaryNode>,
}

impl SummaryStore {
    pub fn load(nodes: Vec<SummaryNode>) -> Self {
        Self { nodes }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn search(&self, query_emb: &[f32], top_k: usize, threshold: f64) -> Vec<(usize, f64)> {
        let qnorm: f64 = query_emb
            .iter()
            .map(|&x| (x as f64) * (x as f64))
            .sum::<f64>()
            .sqrt();
        if qnorm == 0.0 || self.nodes.is_empty() {
            return vec![];
        }
        let mut scored: Vec<(usize, f64)> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| !n.embedding.is_empty())
            .filter_map(|(i, n)| {
                let dot: f64 = query_emb
                    .iter()
                    .zip(n.embedding.iter())
                    .map(|(&q, &d)| (q as f64) * (d as f64))
                    .sum();
                let dnorm: f64 = n
                    .embedding
                    .iter()
                    .map(|&x| (x as f64) * (x as f64))
                    .sum::<f64>()
                    .sqrt();
                if dnorm == 0.0 {
                    return None;
                }
                let sim = (dot / (qnorm * dnorm)).clamp(-1.0, 1.0);
                if sim >= threshold {
                    Some((i, sim))
                } else {
                    None
                }
            })
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }
}

async fn call_summary_llm(prompt: &str, url: &str, model: &str) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(180))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;
    let full_url = format!("{}/v1/chat/completions", url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": 400,
    });
    let resp = tokio::time::timeout(
        std::time::Duration::from_secs(120),
        client.post(&full_url).json(&body).send(),
    )
    .await
    .ok()?
    .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let v: serde_json::Value =
        tokio::time::timeout(std::time::Duration::from_secs(120), resp.json())
            .await
            .ok()?
            .ok()?;
    Some(
        v["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string(),
    )
}

struct WindowInfo {
    chunk_ids: Vec<i64>,
    text: String,
}

pub async fn generate_summaries(
    meta: &MetaStore,
    embed: &EmbedClient,
    inference_url: &str,
    model: &str,
    window_size: usize,
    on_status: impl Fn(&str),
) -> Result<Vec<SummaryNode>> {
    let all_chunks = meta.all_chunks()?;

    let mut by_doc: std::collections::BTreeMap<String, Vec<(i64, crate::meta_store::ChunkMeta)>> =
        Default::default();
    for (id, cm) in all_chunks {
        by_doc.entry(cm.doc_name.clone()).or_default().push((id, cm));
    }
    for chunks in by_doc.values_mut() {
        chunks.sort_by_key(|(_, cm)| cm.chunk_index);
    }

    let mut nodes: Vec<SummaryNode> = Vec::new();
    let mut next_id: i64 = 1;

    for (doc_name, chunks) in &by_doc {
        let skip_count = chunks.iter().filter(|(_, cm)| cm.skip_extraction).count();
        let active: Vec<_> = chunks
            .iter()
            .filter(|(_, cm)| !cm.skip_extraction)
            .collect();
        on_status(&format!(
            "  ○ Summarizing '{doc_name}' ({} active chunks, {skip_count} skipped, window={window_size})",
            active.len()
        ));

        // Track window infos per section for level-2 aggregation.
        let mut section_windows: std::collections::HashMap<String, Vec<WindowInfo>> =
            Default::default();

        for window in active.chunks(window_size) {
            let chunk_ids: Vec<i64> = window.iter().map(|(id, _)| *id).collect();
            let combined: String = window
                .iter()
                .map(|(_, cm)| cm.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n");
            let trimmed_end = if combined.len() > 6_000 {
                let mut end = 6_000;
                while !combined.is_char_boundary(end) { end -= 1; }
                end
            } else {
                combined.len()
            };
            let trimmed = &combined[..trimmed_end];
            let prompt = format!(
                "Summarize the following passage in 2-4 sentences, \
                 preserving key facts, names, dates, and places. Be concise.\n\n\
                 {trimmed}\n\nSummary:"
            );
            let text = match call_summary_llm(&prompt, inference_url, model).await {
                Some(t) if !t.trim().is_empty() => t.trim().to_string(),
                _ => window[0].1.text.chars().take(200).collect(),
            };
            let embedding = embed.embed_one(&text).await?;
            let section_key = window[0].1.section_name.clone().unwrap_or_default();

            section_windows
                .entry(section_key.clone())
                .or_default()
                .push(WindowInfo {
                    chunk_ids: chunk_ids.clone(),
                    text: text.clone(),
                });

            nodes.push(SummaryNode {
                id: next_id,
                doc_name: doc_name.clone(),
                level: 1,
                chunk_ids,
                text,
                embedding,
                section_name: window[0].1.section_name.clone(),
            });
            next_id += 1;
        }

        // Level 2: one section summary per section that spans ≥2 windows.
        for (section_key, windows) in &section_windows {
            if windows.len() < 2 {
                continue;
            }
            let combined: String = windows
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<_>>()
                .join("\n\n");
            let trimmed_end = if combined.len() > 6_000 {
                let mut end = 6_000;
                while !combined.is_char_boundary(end) { end -= 1; }
                end
            } else {
                combined.len()
            };
            let trimmed = &combined[..trimmed_end];
            let label = if section_key.is_empty() {
                "section"
            } else {
                section_key.as_str()
            };
            let prompt = format!(
                "Summarize the following section titled \"{label}\" in 3-5 sentences, \
                 preserving key facts, names, dates, and places.\n\n\
                 {trimmed}\n\nSummary:"
            );
            let text = match call_summary_llm(&prompt, inference_url, model).await {
                Some(t) if !t.trim().is_empty() => t.trim().to_string(),
                _ => windows[0].text.chars().take(300).collect(),
            };
            let embedding = embed.embed_one(&text).await?;
            let all_chunk_ids: Vec<i64> = windows
                .iter()
                .flat_map(|w| w.chunk_ids.iter().copied())
                .collect();
            nodes.push(SummaryNode {
                id: next_id,
                doc_name: doc_name.clone(),
                level: 2,
                chunk_ids: all_chunk_ids,
                text,
                embedding,
                section_name: if section_key.is_empty() {
                    None
                } else {
                    Some(section_key.clone())
                },
            });
            next_id += 1;
        }
    }

    on_status(&format!("  ○ Generated {} summary nodes total", nodes.len()));
    Ok(nodes)
}
