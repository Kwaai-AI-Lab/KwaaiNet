//! Dream RAG Phase 2 — autonomous knowledge graph completion cycle.
//!
//! One dream cycle:
//!   1. Score graph → identify incomplete entities
//!   2. Complete unknown-type entities (type reclassification)
//!   3. Complete thin-summary entities (description enrichment)
//!   4. Complete entities missing expected relations
//!   5. Auto-merge near-duplicates
//!   6. Prune zombie entities
//!   7. Re-score → write DreamReport

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::{mpsc, Semaphore};
use uuid::Uuid;

use crate::embedder::EmbedClient;
use crate::graph::{GraphStore, RELATION_TYPES};
use crate::meta_store::MetaStore;
use crate::scorer::{score_entity, score_graph};

// ── Config & report ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamConfig {
    /// Seconds between automatic cycles when running in daemon mode.
    pub interval_secs: u64,
    /// Entities with overall score below this are candidates for completion.
    pub completeness_threshold: f32,
    /// Embedding similarity at or above this triggers auto-merge.
    pub dedup_threshold: f32,
    /// Entities with overall score below this AND no chunks AND degree 0 are pruned.
    pub prune_threshold: f32,
    /// Maximum LLM completion calls per cycle (type + summary + relation steps share this budget).
    pub max_completions_per_cycle: usize,
    /// Concurrent completion workers.
    pub workers: usize,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            interval_secs: 3600,
            completeness_threshold: 0.6,
            dedup_threshold: 0.92,
            prune_threshold: 0.3,
            max_completions_per_cycle: 50,
            workers: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamReport {
    pub timestamp: String,
    pub duration_secs: f64,
    pub entities_type_completed: usize,
    pub entities_summary_completed: usize,
    pub entities_relations_added: usize,
    pub entities_merged: usize,
    pub entities_pruned: usize,
    pub score_before: f32,
    pub score_after: f32,
    /// Non-fatal errors encountered during the cycle.
    pub cycle_errors: Vec<String>,
}

// ── Entity completion ─────────────────────────────────────────────────────────

/// Structured result from one LLM completion call.
#[derive(Debug)]
pub struct EntityCompletion {
    pub entity_id: i64,
    pub schema_type: Option<String>,
    pub description: Option<String>,      // None = no improvement
    pub relations: Vec<(String, String)>, // (relation_type, target_name)
}

#[derive(Debug, Deserialize)]
struct CompletionPayload {
    #[serde(default)]
    schema_type: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    relations: Vec<CompletionRelation>,
}

#[derive(Debug, Deserialize)]
struct CompletionRelation {
    #[serde(rename = "type")]
    relation_type: String,
    target: String,
}

static VALID_SCHEMA_TYPES: OnceLock<Vec<&'static str>> = OnceLock::new();

fn valid_schema_types() -> &'static [&'static str] {
    VALID_SCHEMA_TYPES.get_or_init(|| {
        vec![
            "schema:Person",
            "schema:Organization",
            "schema:Place",
            "schema:Event",
            "schema:Product",
            "schema:CreativeWork",
            "schema:SoftwareApplication",
            "schema:DefinedTerm",
            "schema:HowTo",
            "schema:Role",
            "schema:QuantitativeValue",
            "schema:Statement",
            "schema:Date",
            "schema:Thing",
        ]
    })
}

/// Call the LLM to complete one entity: resolve its schema.org type, generate
/// a substantive description, and surface relations present in the source text.
pub async fn complete_entity(
    eid: i64,
    name: &str,
    current_type: &str,
    current_description: &str,
    chunk_text: &str,
    inference_url: &str,
    model: &str,
) -> EntityCompletion {
    let schema_types = valid_schema_types().join(", ");
    let relation_types = RELATION_TYPES.join(", ");

    let prompt = format!(
        "You are completing a knowledge graph entity. \
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Entity name: \"{name}\"\n\
         Current type: {current_type}\n\
         Current description: \"{current_description}\"\n\n\
         Source text:\n---\n{chunk_text}\n---\n\n\
         JSON schema:\n\
         {{\"schema_type\":\"<type>\",\
           \"description\":\"<EXACTLY 2-3 sentences (minimum 150 characters) from the text>\",\
           \"relations\":[{{\"type\":\"<rel>\",\"target\":\"<entity name>\"}}]}}\n\n\
         Valid schema_type values: {schema_types}\n\
         Valid relation types: {relation_types}\n\n\
         Rules:\n\
         - schema_type must be one of the listed values; use schema:Thing if unsure\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - description must be derived from the source text, not invented\n\
         - only include relations clearly stated in the text\n\
         - only include target entities that appear by name in the text"
    );

    let url = format!(
        "{}/v1/chat/completions",
        inference_url.trim_end_matches('/')
    );
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(_) => {
            return EntityCompletion {
                entity_id: eid,
                schema_type: None,
                description: None,
                relations: vec![],
            }
        }
    };

    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.1,
        "max_tokens": 512,
    });

    let resp = match tokio::time::timeout(
        std::time::Duration::from_secs(30),
        client.post(&url).json(&body).send(),
    )
    .await
    {
        Ok(Ok(r)) if r.status().is_success() => r,
        _ => {
            return EntityCompletion {
                entity_id: eid,
                schema_type: None,
                description: None,
                relations: vec![],
            }
        }
    };

    let v: serde_json::Value =
        match tokio::time::timeout(std::time::Duration::from_secs(120), resp.json()).await {
            Ok(Ok(v)) => v,
            _ => {
                return EntityCompletion {
                    entity_id: eid,
                    schema_type: None,
                    description: None,
                    relations: vec![],
                }
            }
        };

    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("{}");
    let cleaned = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let payload: CompletionPayload = match serde_json::from_str(cleaned) {
        Ok(p) => p,
        Err(_) => {
            return EntityCompletion {
                entity_id: eid,
                schema_type: None,
                description: None,
                relations: vec![],
            }
        }
    };

    // Validate schema_type against known list.
    let schema_type = if valid_schema_types().contains(&payload.schema_type.as_str()) {
        Some(payload.schema_type)
    } else {
        None
    };

    // Only improve description if it's substantively longer.
    let description = if payload.description.len() > current_description.len() + 20 {
        Some(payload.description)
    } else {
        None
    };

    // Filter relations: type must be valid, target must be non-empty.
    let relations: Vec<(String, String)> = payload
        .relations
        .into_iter()
        .filter(|r| {
            !r.target.is_empty()
                && !r.relation_type.is_empty()
                && RELATION_TYPES.contains(&r.relation_type.as_str())
        })
        .map(|r| (r.relation_type, r.target))
        .collect();

    EntityCompletion {
        entity_id: eid,
        schema_type,
        description,
        relations,
    }
}

// ── Dream cycle ───────────────────────────────────────────────────────────────

struct WorkItem {
    entity_id: i64,
    name: String,
    entity_type: String,
    schema_type: Option<String>,
    description: String,
    evidence_text: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn run_dream_cycle(
    data_dir: &Path,
    tenant_id: Uuid,
    embed: &EmbedClient,
    cfg: &DreamConfig,
    inference_urls: &[String],
    model: &str,
    progress: Option<impl Fn(usize, usize, &str)>,
) -> Result<DreamReport> {
    let started = std::time::Instant::now();
    let mut cycle_errors: Vec<String> = Vec::new();
    let mut report = DreamReport {
        timestamp: chrono::Utc::now().to_rfc3339(),
        duration_secs: 0.0,
        entities_type_completed: 0,
        entities_summary_completed: 0,
        entities_relations_added: 0,
        entities_merged: 0,
        entities_pruned: 0,
        score_before: 0.0,
        score_after: 0.0,
        cycle_errors: vec![],
    };

    // ── Step 1: Score, collect work items ────────────────────────────────────
    let work_items = {
        // Open MetaStore inside this block so it is dropped before the LLM
        // fan-out — prevents "Database already open" errors if another command
        // (e.g. alias-scan) runs concurrently against the same KB.
        let meta = MetaStore::open(data_dir, tenant_id).context("open meta store for scoring")?;
        let store = GraphStore::open(data_dir, tenant_id).context("open graph for scoring")?;
        let health = score_graph(&store);
        report.score_before = health.overall;
        let mut items: Vec<WorkItem> = Vec::new();
        let budget = cfg.max_completions_per_cycle;

        // Priority order: Unknown type first, then thin summary, then missing relations.
        let mut candidates: Vec<_> = health
            .entity_scores
            .iter()
            .filter(|s| s.overall < cfg.completeness_threshold)
            .collect();
        candidates.sort_by(|a, b| a.overall.partial_cmp(&b.overall).unwrap());

        for score in candidates.iter().take(budget) {
            let node = match store.get_entity(score.entity_id) {
                Some(n) => n,
                None => continue,
            };

            // Use evidence field (all known chunks) when populated; fall back to index lookup.
            let chunk_ids: Vec<i64> = if !node.evidence.is_empty() {
                node.evidence.clone()
            } else {
                store.chunks_for_entity(score.entity_id).to_vec()
            };
            if chunk_ids.is_empty() {
                continue; // zombie — handled in prune step
            }

            // Fetch up to 20 chunks and concatenate for richer evidence.
            let fetch_limit = chunk_ids.len().min(20);
            let chunks = meta.get_chunks(&chunk_ids[..fetch_limit])?;
            let evidence_text: String = chunks
                .iter()
                .flatten()
                .map(|c| c.text.as_str())
                .collect::<Vec<_>>()
                .join("\n---\n");
            if evidence_text.is_empty() {
                continue;
            }

            items.push(WorkItem {
                entity_id: score.entity_id,
                name: node.name.clone(),
                entity_type: node.entity_type.clone(),
                schema_type: node.schema_type.clone(),
                description: node.description.clone(),
                evidence_text,
            });
        }
        items
    }; // GraphStore dropped here

    let total = work_items.len();
    if total == 0 {
        // Nothing to do — still score and return.
        let store = GraphStore::open(data_dir, tenant_id).context("open graph for final score")?;
        report.score_after = score_graph(&store).overall;
        report.duration_secs = started.elapsed().as_secs_f64();
        return Ok(report);
    }

    // ── Step 2–4: Fan-out LLM completions ────────────────────────────────────
    let urls = Arc::new(inference_urls.to_vec());
    let url_counter = Arc::new(AtomicUsize::new(0));
    let sem = Arc::new(Semaphore::new(cfg.workers.max(1)));
    // Channel capacity = total work items so tasks never block on send before the
    // receiver loop starts (bounded smaller capacity → deadlock when channel fills
    // while the spawning loop still holds all semaphore permits).
    let (tx, mut rx) = mpsc::channel::<Result<EntityCompletion>>(total.max(1));
    let model_str = model.to_string();

    for item in work_items {
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        let urls = urls.clone();
        let url_counter = url_counter.clone();
        let model = model_str.clone();

        tokio::spawn(async move {
            let _permit = permit;
            let idx = url_counter.fetch_add(1, Ordering::Relaxed) % urls.len().max(1);
            let url = &urls[idx % urls.len()];
            // Prefer the stored schema_type; fall back to the static map so
            // "Person" entities without a schema_type field still get the
            // Biography task rather than the General task.
            let resolved_st = item
                .schema_type
                .as_deref()
                .or_else(|| crate::scorer::schema_type_for(&item.entity_type));
            let kind = crate::dream_tasks::task_for_schema_type(resolved_st);
            let result = crate::dream_tasks::run_task(
                kind,
                item.entity_id,
                &item.name,
                &item.entity_type,
                &item.description,
                &item.evidence_text,
                url,
                &model,
            )
            .await;
            let _ = tx.send(Ok(result)).await;
        });
    }
    drop(tx);

    let mut completions: Vec<EntityCompletion> = Vec::new();
    let mut done = 0usize;
    while let Some(res) = rx.recv().await {
        done += 1;
        if let Some(ref cb) = progress {
            cb(done, total, "completing");
        }
        match res {
            Ok(c) => completions.push(c),
            Err(e) => cycle_errors.push(e.to_string()),
        }
    }

    // ── Step 5: Write completions to GraphStore ───────────────────────────────
    {
        let mut store = GraphStore::open(data_dir, tenant_id).context("open graph for writes")?;

        for completion in &completions {
            let eid = completion.entity_id;

            // Type completion
            if let Some(ref st) = completion.schema_type {
                if store
                    .get_entity(eid)
                    .and_then(|n| n.schema_type.as_ref())
                    .is_none()
                {
                    if let Err(e) = store.set_schema_type(eid, st) {
                        cycle_errors.push(format!("set_schema_type {eid}: {e}"));
                    } else {
                        report.entities_type_completed += 1;
                    }
                }
            }

            // Summary completion — upsert with longer description + re-embed
            if let Some(ref new_desc) = completion.description {
                if let Some(node) = store.get_entity(eid).cloned() {
                    let embed_text = crate::graph::GraphStore::entity_embed_text(
                        &node.name,
                        &node.aliases,
                        new_desc,
                    );
                    match embed.embed_batch(&[embed_text.as_str()]).await {
                        Ok(embs) if !embs.is_empty() => {
                            let updated = crate::graph::EntityNode {
                                description: new_desc.clone(),
                                embedding: embs.into_iter().next().unwrap(),
                                schema_type: completion
                                    .schema_type
                                    .clone()
                                    .or(node.schema_type.clone()),
                                ..node
                            };
                            if let Err(e) = store.upsert_entity(updated) {
                                cycle_errors.push(format!("upsert description {eid}: {e}"));
                            } else {
                                report.entities_summary_completed += 1;
                            }
                        }
                        Err(e) => cycle_errors.push(format!("embed {eid}: {e}")),
                        _ => {}
                    }
                }
            }

            // Relation completion — only add if both endpoints exist in graph
            for (rel_type, target_name) in &completion.relations {
                let dst_id = match store.find_by_name(target_name) {
                    Some(n) => n.id,
                    None => continue, // reject hallucinated targets
                };
                let src_chunk = store.chunks_for_entity(eid).first().copied().unwrap_or(0);
                if let Err(e) = store.upsert_relation(eid, dst_id, rel_type, src_chunk) {
                    cycle_errors.push(format!("upsert_relation {eid}→{dst_id}: {e}"));
                } else {
                    report.entities_relations_added += 1;
                }
            }
        }

        // ── Step 6: Auto-merge near-duplicates ────────────────────────────────
        if let Some(ref cb) = progress {
            cb(0, 0, "deduplicating");
        }
        let exact = store.find_dedup_candidates_exact();
        let fuzzy = store.find_dedup_candidates(cfg.dedup_threshold);
        // Tier 3: structural name patterns — auto-merge honorific and subset only;
        // fuzzy (edit-distance) candidates are shown in `graph dedup` for human review.
        // Tier 4 (neighbour containment) is display-only in `graph dedup` — too noisy
        // for unattended auto-merge in memoir-style texts where co-occurrence is high.
        let name_struct = store.find_dedup_candidates_name_structure();
        // Only auto-merge "honorific" candidates from tier 3.
        // "subset" and "fuzzy" require human review in `graph dedup` because subset
        // matches like "Cape Town" → "University of Cape Town" can destroy real entities.
        let merge_pairs: Vec<(i64, i64)> = exact
            .into_iter()
            .chain(fuzzy.into_iter().map(|(a, b, _)| (a, b)))
            .chain(
                name_struct
                    .into_iter()
                    .filter(|(_, _, reason)| *reason == "honorific")
                    .map(|(a, b, _)| (a, b)),
            )
            .collect();

        for (alias_id, canonical_id) in &merge_pairs {
            match store.merge_entity_into(*alias_id, *canonical_id) {
                Ok(_) => report.entities_merged += 1,
                Err(e) => cycle_errors.push(format!("merge {alias_id}→{canonical_id}: {e}")),
            }
        }
        if !merge_pairs.is_empty() {
            store.rebuild_in_memory().ok();
        }

        // ── Step 7: Prune zombies ─────────────────────────────────────────────
        if let Some(ref cb) = progress {
            cb(0, 0, "pruning");
        }
        let zombie_ids: Vec<i64> = store
            .all_entities()
            .filter(|n| {
                n.mention_count <= 1
                    && store.neighbors_of(n.id).is_empty()
                    && store.chunks_for_entity(n.id).is_empty()
                    && {
                        let neighbor_rels: Vec<String> = vec![];
                        score_entity(n, &neighbor_rels).overall < cfg.prune_threshold
                    }
            })
            .map(|n| n.id)
            .collect();

        for eid in &zombie_ids {
            match store.delete_entity(*eid) {
                Ok(_) => report.entities_pruned += 1,
                Err(e) => cycle_errors.push(format!("delete_entity {eid}: {e}")),
            }
        }
        if !zombie_ids.is_empty() {
            store.rebuild_in_memory().ok();
        }

        // ── Step 8: Final score ───────────────────────────────────────────────
        report.score_after = score_graph(&store).overall;
    }

    report.duration_secs = started.elapsed().as_secs_f64();
    report.cycle_errors = cycle_errors;
    Ok(report)
}
