//! Entity metadata enrichment: build descriptions and extract structured metadata
//! (gender, etc.) from all evidence chunks.
//!
//! Each qualifying entity gets all its linked chunk text gathered, then a single
//! LLM call produces a JSON payload with `description` and (for Person entities)
//! `gender`. Existing non-empty values are preserved unless `force` is true.

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::{mpsc, Semaphore};
use uuid::Uuid;

use crate::embedder::EmbedClient;
use crate::graph::GraphStore;
use crate::meta_store::MetaStore;

// ── Config ────────────────────────────────────────────────────────────────────

pub struct EnrichConfig {
    /// Only enrich entities of these types (default: Person, Place, Organization).
    pub entity_types: Vec<String>,
    /// Skip entities with fewer mentions than this (default: 2).
    pub min_mentions: u32,
    /// Maximum entities to enrich in one run (default: all).
    pub limit: usize,
    /// Parallel LLM workers (default: 4).
    pub workers: usize,
    /// Max chunks to include as evidence per entity (default: 20).
    pub fetch_limit: usize,
    /// If true, overwrite fields that already have a value (default: false).
    pub force: bool,
    /// If true (default), extract gender for Person entities from text evidence.
    pub extract_gender: bool,
}

impl Default for EnrichConfig {
    fn default() -> Self {
        Self {
            entity_types: vec![
                "Person".into(),
                "Place".into(),
                "Organization".into(),
            ],
            min_mentions: 2,
            limit: usize::MAX,
            workers: 4,
            fetch_limit: 20,
            force: false,
            extract_gender: true,
        }
    }
}

#[derive(Debug, Default)]
pub struct EnrichReport {
    pub entities_processed: usize,
    pub entities_updated: usize,
    pub entities_skipped_no_evidence: usize,
    /// Number of Person entities that had gender set or updated.
    pub genders_set: usize,
    pub errors: Vec<String>,
}

// ── Internal types ─────────────────────────────────────────────────────────────

struct WorkItem {
    id: i64,
    name: String,
    entity_type: String,
    current_desc: String,
    current_gender: Option<String>,
    /// Whether the entity needs description enrichment in this run.
    need_desc: bool,
    /// Whether the entity needs gender extraction in this run.
    need_gender: bool,
    evidence_text: String,
}

struct EnrichResult {
    description: Option<String>,
    gender: Option<String>,
}

// ── Public API ─────────────────────────────────────────────────────────────────

/// Enrich entity metadata for all qualifying entities in the knowledge base.
///
/// Progress callback receives `(done, total, label)`.
pub async fn enrich_entity_descriptions(
    cfg: &EnrichConfig,
    model: &str,
    inference_url: &str,
    embed: &EmbedClient,
    data_dir: &Path,
    tenant_id: Uuid,
    progress: impl Fn(usize, usize, &str) + Send + Sync,
) -> Result<EnrichReport> {
    let mut report = EnrichReport::default();

    // ── Phase 1: build work items ─────────────────────────────────────────────
    let work_items: Vec<WorkItem> = {
        let store = GraphStore::open(data_dir, tenant_id)?;
        let meta = MetaStore::open(data_dir, tenant_id)?;
        let mut items: Vec<WorkItem> = Vec::new();

        for node in store.all_entities() {
            if !cfg
                .entity_types
                .iter()
                .any(|t| t.eq_ignore_ascii_case(&node.entity_type))
            {
                continue;
            }
            if node.mention_count < cfg.min_mentions {
                continue;
            }

            let is_person = node.entity_type.eq_ignore_ascii_case("person");
            let need_desc = cfg.force || node.description.is_empty();
            let need_gender = cfg.extract_gender
                && is_person
                && (cfg.force || node.gender.is_none());

            // Skip if nothing to do for this entity
            if !need_desc && !need_gender {
                continue;
            }

            let chunk_ids = store.chunks_for_entity(node.id).to_vec();
            if chunk_ids.is_empty() {
                report.entities_skipped_no_evidence += 1;
                continue;
            }

            let fetch_limit = chunk_ids.len().min(cfg.fetch_limit);
            let chunks = match meta.get_chunks(&chunk_ids[..fetch_limit]) {
                Ok(c) => c,
                Err(_) => {
                    report.entities_skipped_no_evidence += 1;
                    continue;
                }
            };

            let evidence_text: String = chunks
                .iter()
                .flatten()
                .map(|c| {
                    let mut s = String::new();
                    if let Some(ref sec) = c.section_name {
                        s.push_str(&format!("[Section: {sec}]\n"));
                    }
                    s.push_str(&c.text);
                    s
                })
                .collect::<Vec<_>>()
                .join("\n---\n");

            if evidence_text.is_empty() {
                report.entities_skipped_no_evidence += 1;
                continue;
            }

            items.push(WorkItem {
                id: node.id,
                name: node.name.clone(),
                entity_type: node.entity_type.clone(),
                current_desc: node.description.clone(),
                current_gender: node.gender.clone(),
                need_desc,
                need_gender,
                evidence_text,
            });

            if items.len() >= cfg.limit {
                break;
            }
        }
        items
    }; // store + meta dropped here

    let total = work_items.len();
    if total == 0 {
        return Ok(report);
    }

    // ── Phase 2: fan-out LLM calls ────────────────────────────────────────────
    let sem = Arc::new(Semaphore::new(cfg.workers.max(1)));
    let url_arc = Arc::new(inference_url.to_string());
    let model_arc = Arc::new(model.to_string());
    let counter = Arc::new(AtomicUsize::new(0));

    let (tx, mut rx) = mpsc::channel::<(i64, EnrichResult)>(total.max(1));

    for item in work_items {
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        let url = url_arc.clone();
        let model_clone = model_arc.clone();
        let counter = counter.clone();

        tokio::spawn(async move {
            let _permit = permit;
            let result = call_enrich(
                &item.name,
                &item.entity_type,
                &item.current_desc,
                item.current_gender.as_deref(),
                &item.evidence_text,
                &url,
                &model_clone,
                item.need_desc,
                item.need_gender,
            )
            .await;
            counter.fetch_add(1, Ordering::Relaxed);
            let _ = tx.send((item.id, result)).await;
        });
    }
    drop(tx);

    let mut updates: Vec<(i64, EnrichResult)> = Vec::new();
    let mut done = 0usize;
    while let Some((eid, result)) = rx.recv().await {
        done += 1;
        progress(done, total, "enriching");
        if result.description.is_some() || result.gender.is_some() {
            updates.push((eid, result));
        }
    }

    report.entities_processed = done;

    // ── Phase 3: write updates + re-embed ─────────────────────────────────────
    let mut store = GraphStore::open(data_dir, tenant_id).context("open graph for writes")?;

    for (eid, result) in &updates {
        let node = match store.get_entity(*eid).cloned() {
            Some(n) => n,
            None => continue,
        };

        let new_desc = result
            .description
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(&node.description);

        let gender_updated = result.gender.is_some();
        let new_gender = result.gender.clone().or_else(|| node.gender.clone());

        let embed_text = GraphStore::entity_embed_text(&node.name, &node.aliases, new_desc);
        match embed.embed_batch(&[embed_text.as_str()]).await {
            Ok(embs) if !embs.is_empty() => {
                let updated = crate::graph::EntityNode {
                    description: new_desc.to_string(),
                    embedding: embs.into_iter().next().unwrap(),
                    gender: new_gender,
                    ..node
                };
                if let Err(e) = store.upsert_entity(updated) {
                    report.errors.push(format!("upsert entity {eid}: {e}"));
                } else {
                    report.entities_updated += 1;
                    if gender_updated {
                        report.genders_set += 1;
                    }
                }
            }
            _ => {
                report.errors.push(format!("embed failed for entity {eid}"));
            }
        }
    }

    Ok(report)
}

// ── LLM call ──────────────────────────────────────────────────────────────────

async fn call_enrich(
    name: &str,
    entity_type: &str,
    current_desc: &str,
    _current_gender: Option<&str>,
    evidence_text: &str,
    url: &str,
    model: &str,
    _need_desc: bool,
    need_gender: bool,
) -> EnrichResult {
    let type_label = match entity_type.to_lowercase().as_str() {
        "person" => "person",
        "place" | "location" => "place or location",
        "organization" => "organization or group",
        _ => "entity",
    };

    let existing_hint = if !current_desc.is_empty() {
        format!("\nExisting summary (may be refined): {current_desc}\n")
    } else {
        String::new()
    };

    let prompt = if need_gender {
        // JSON-mode prompt for Person entities: description + gender
        format!(
            "You are a knowledge extraction assistant working with a historical memoir.{existing_hint}\n\
             Below are all excerpts from the document that mention \"{name}\" (a {type_label}):\n\n\
             {evidence_text}\n\n\
             Based ONLY on the excerpts above, return ONLY a JSON object with these two fields \
             (no other text, no markdown fences):\n\
             {{\n  \"description\": \"<2-3 sentences: who {name} is, their significance, \
             key relationships — text-grounded only>\",\n\
               \"gender\": <\"Male\" | \"Female\" | null>\n\
             }}\n\n\
             For gender, look for DIRECT TEXTUAL EVIDENCE:\n\
             - Pronouns: he/him/his/himself → \"Male\"; she/her/hers/herself → \"Female\"\n\
             - Titles: Mr./Sir/Uncle → \"Male\"; Mrs./Miss/Madam/Aunt → \"Female\"\n\
             - Role words: son/brother/father/grandson → \"Male\"; daughter/sister/mother/granddaughter → \"Female\"\n\
             - Explicit statements: \"a man\", \"a woman\", \"the boy\", \"the girl\"\n\
             Return null if the evidence is absent or ambiguous."
        )
    } else {
        // Plain prose for non-Person entities or when only description is needed
        format!(
            "You are a knowledge extraction assistant working with a historical memoir.{existing_hint}\n\
             Below are all excerpts from the document that mention \"{name}\" (a {type_label}):\n\n\
             {evidence_text}\n\n\
             Based ONLY on the excerpts above, write a concise 2–3 sentence description of \"{name}\" \
             that captures: who or what they are, their significance in the story, and any key \
             relationships or roles. Do NOT add information not present in the excerpts. \
             Output ONLY the description paragraph."
        )
    };

    match call_llm(&prompt, url, model, need_gender).await {
        Some(text) if need_gender => parse_enrich_json(&text),
        Some(text) if text.len() >= 20 => EnrichResult {
            description: Some(text),
            gender: None,
        },
        _ => EnrichResult {
            description: None,
            gender: None,
        },
    }
}

fn parse_enrich_json(text: &str) -> EnrichResult {
    // Strip markdown code fences
    let cleaned = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    // Find JSON object bounds (model sometimes prepends/appends stray text)
    let start = cleaned.find('{').unwrap_or(0);
    let end = cleaned.rfind('}').map(|i| i + 1).unwrap_or(cleaned.len());
    let json_str = if end > start { &cleaned[start..end] } else { cleaned };

    let v: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return EnrichResult { description: None, gender: None },
    };

    let description = v["description"]
        .as_str()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() >= 20);

    let gender = v["gender"]
        .as_str()
        .and_then(|s| match s.trim().to_lowercase().as_str() {
            "male" => Some("Male".to_string()),
            "female" => Some("Female".to_string()),
            _ => None,
        });

    EnrichResult { description, gender }
}

async fn call_llm(prompt: &str, url: &str, model: &str, json_mode: bool) -> Option<String> {
    for attempt in 0u32..3 {
        if attempt > 0 {
            // Give the p2p proxy time to re-establish the stream after a reset.
            tokio::time::sleep(std::time::Duration::from_secs(3 * attempt as u64)).await;
        }
        if let Some(text) = call_llm_once(prompt, url, model, json_mode).await {
            return Some(text);
        }
    }
    None
}

async fn call_llm_once(prompt: &str, url: &str, model: &str, json_mode: bool) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let full_url = format!("{}/v1/chat/completions", url.trim_end_matches('/'));
    let mut body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.1,
        "max_tokens": 400,
    });
    // Ollama JSON mode — forces the response to be valid JSON
    if json_mode {
        body["format"] = serde_json::json!("json");
    }

    let resp = tokio::time::timeout(
        std::time::Duration::from_secs(90),
        client.post(&full_url).json(&body).send(),
    )
    .await
    .ok()?
    .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let v: serde_json::Value = tokio::time::timeout(
        std::time::Duration::from_secs(120),
        resp.json(),
    )
    .await
    .ok()?
    .ok()?;

    let text = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string();

    if text.is_empty() { None } else { Some(text) }
}
