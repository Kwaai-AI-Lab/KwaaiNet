//! Entity metadata enrichment: build descriptions and extract structured metadata
//! (gender, etc.) from all evidence chunks.
//!
//! Each qualifying entity gets all its linked chunk text gathered, then a single
//! LLM call produces a JSON payload with `description` and (for Person entities)
//! `gender`. Existing non-empty values are preserved unless `force` is true.

use std::cmp::Reverse;
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
            entity_types: vec!["Person".into(), "Place".into(), "Organization".into()],
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
    /// All alias forms merged into this entity (pronoun aliases excluded).
    /// Passed to the LLM prompt so it knows "he", "Dr. Gool", "J.M.H. Gool"
    /// in the evidence text all refer to the same entity.
    aliases: Vec<String>,
    current_desc: String,
    current_gender: Option<String>,
    /// Whether the entity needs description enrichment in this run.
    need_desc: bool,
    /// Whether the entity needs gender extraction in this run.
    need_gender: bool,
    evidence_text: String,
    /// Inference URL assigned to this item (round-robins across the pool).
    inference_url: String,
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
    inference_urls: &[String],
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
            let need_gender =
                cfg.extract_gender && is_person && (cfg.force || node.gender.is_none());

            // Skip if nothing to do for this entity
            if !need_desc && !need_gender {
                continue;
            }

            let chunk_ids = store.chunks_for_entity(node.id).to_vec();
            if chunk_ids.is_empty() {
                report.entities_skipped_no_evidence += 1;
                continue;
            }

            // Compute aliases early so we can use them for evidence filtering.
            // Filter out generic pronoun aliases so the LLM hint is signal, not noise.
            const PRONOUN_ALIASES: &[&str] = &[
                "i",
                "he",
                "she",
                "they",
                "him",
                "her",
                "his",
                "the author",
                "the narrator",
                "narrator",
                "author",
                "the writer",
            ];
            let meaningful_aliases: Vec<String> = node
                .aliases
                .iter()
                .filter(|a| !PRONOUN_ALIASES.contains(&a.to_lowercase().as_str()))
                .cloned()
                .collect();

            // Distinctive aliases: anything beyond just the entity's first name alone.
            // A bare first name (e.g. "Fatima" for "Fatima Gool") is too generic — it matches
            // unrelated people with the same first name appearing in other passages of the memoir.
            let canonical_lower = node.name.to_lowercase();
            let first_name_part: &str = canonical_lower
                .split_whitespace()
                .next()
                .unwrap_or(&canonical_lower);
            let distinctive_aliases: Vec<String> = meaningful_aliases
                .iter()
                .filter(|a| a.to_lowercase() != first_name_part)
                .cloned()
                .collect();

            // Fetch a wider pool first (up to 4× fetch_limit), filter by distinctive aliases,
            // then cap at fetch_limit. This ensures we don't miss valid chunks that happen to
            // fall beyond the first fetch_limit positions in the unfiltered set.
            let pre_filter_limit = chunk_ids.len().min(cfg.fetch_limit * 4);
            let chunks = match meta.get_chunks(&chunk_ids[..pre_filter_limit]) {
                Ok(c) => c,
                Err(_) => {
                    report.entities_skipped_no_evidence += 1;
                    continue;
                }
            };

            // Keep only chunks where the full entity name or a distinctive alias appears.
            // This prevents a bare first-name alias from pulling in chunks about unrelated
            // people who share the same first name, which causes the LLM to fabricate
            // relationships between the entity and those unrelated people.
            //
            // Sort by named-entity density (uppercase word count) so information-rich
            // chunks — those listing notable associates, dates, organisations — appear
            // early and are not crowded out by narrative padding.
            let mut filtered_chunks: Vec<_> = chunks
                .iter()
                .flatten()
                .filter(|c| {
                    let text_lower = c.text.to_lowercase();
                    text_lower.contains(&canonical_lower)
                        || distinctive_aliases
                            .iter()
                            .any(|a| text_lower.contains(a.to_lowercase().as_str()))
                })
                .collect();
            filtered_chunks.sort_by_key(|c| {
                // Higher score = more uppercase words = more named entities = rank first
                let density: usize = c
                    .text
                    .split_whitespace()
                    .filter(|w| {
                        w.chars()
                            .next()
                            .map(|ch| ch.is_uppercase())
                            .unwrap_or(false)
                    })
                    .count();
                std::cmp::Reverse(density)
            });
            let evidence_text: String = filtered_chunks
                .into_iter()
                .take(cfg.fetch_limit)
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

            // If distinctive-alias filtering removed everything, the entity's entity_to_chunks
            // links are all from generic-first-name occurrences (e.g. "Fatima Gool" whose
            // linked chunks come from a different Fatima in the text). Fall back to a full-corpus
            // scan that finds any chunk mentioning the canonical name or a distinctive alias.
            let evidence_text = if evidence_text.is_empty() && !distinctive_aliases.is_empty() {
                tracing::warn!(
                    entity = %node.name,
                    chunk_ids = chunk_ids.len(),
                    "no distinctive-alias evidence in entity_to_chunks; scanning full corpus"
                );
                match meta.all_chunks() {
                    Ok(all) => {
                        let mut fallback_chunks: Vec<_> = all
                            .into_iter()
                            .filter_map(|(_id, c)| {
                                let text_lower = c.text.to_lowercase();
                                if text_lower.contains(&canonical_lower)
                                    || distinctive_aliases
                                        .iter()
                                        .any(|a| text_lower.contains(a.to_lowercase().as_str()))
                                {
                                    Some(c)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        fallback_chunks.sort_by_key(|c| {
                            let density: usize = c
                                .text
                                .split_whitespace()
                                .filter(|w| {
                                    w.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false)
                                })
                                .count();
                            Reverse(density)
                        });
                        let fallback: Vec<String> = fallback_chunks
                            .into_iter()
                            .take(cfg.fetch_limit)
                            .map(|c| {
                                let mut s = String::new();
                                if let Some(ref sec) = c.section_name {
                                    s.push_str(&format!("[Section: {sec}]\n"));
                                }
                                s.push_str(&c.text);
                                s
                            })
                            .collect();
                        fallback.join("\n---\n")
                    }
                    Err(_) => evidence_text,
                }
            } else {
                evidence_text
            };
            if evidence_text.is_empty() {
                report.entities_skipped_no_evidence += 1;
                continue;
            }

            let url_idx = items.len() % inference_urls.len().max(1);
            let inference_url = inference_urls.get(url_idx).cloned().unwrap_or_default();
            items.push(WorkItem {
                id: node.id,
                name: node.name.clone(),
                entity_type: node.entity_type.clone(),
                aliases: meaningful_aliases,
                current_desc: node.description.clone(),
                current_gender: node.gender.clone(),
                need_desc,
                need_gender,
                evidence_text,
                inference_url,
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
    let model_arc = Arc::new(model.to_string());
    let counter = Arc::new(AtomicUsize::new(0));

    let (tx, mut rx) = mpsc::channel::<(i64, EnrichResult)>(total.max(1));

    for item in work_items {
        let permit = sem.clone().acquire_owned().await.expect("semaphore closed");
        let tx = tx.clone();
        let url = item.inference_url.clone();
        let model_clone = model_arc.clone();
        let counter = counter.clone();

        tokio::spawn(async move {
            let _permit = permit;
            let result = call_enrich(
                &item.name,
                &item.entity_type,
                &item.aliases,
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

#[allow(clippy::too_many_arguments)]
async fn call_enrich(
    name: &str,
    entity_type: &str,
    aliases: &[String],
    current_desc: &str,
    _current_gender: Option<&str>,
    evidence_text: &str,
    url: &str,
    model: &str,
    need_desc: bool,
    need_gender: bool,
) -> EnrichResult {
    let type_label = match entity_type.to_lowercase().as_str() {
        "person" => "person",
        "place" | "location" => "place or location",
        "organization" => "organization or group",
        _ => "entity",
    };

    // Provide existing description as context only when NOT forcing a fresh generation.
    // When need_desc is true AND a description already exists, we're re-generating from
    // scratch (--force was used) — don't let the old text anchor the LLM.
    // Also skip if the previous extraction failed ("there is no mention").
    let existing_hint = if !need_desc
        && !current_desc.is_empty()
        && current_desc.len() > 50
        && !current_desc
            .to_lowercase()
            .starts_with("there is no mention")
    {
        format!("\nExisting summary (may be refined): {current_desc}\n")
    } else {
        String::new()
    };

    // Tell the LLM about alias forms so it can connect pronoun/abbreviation references
    // in the evidence text back to the entity being described. Explicit wording prevents
    // 8B models from reading "(Timmie)" as a child/relative rather than a nickname.
    let alias_hint = if aliases.is_empty() {
        String::new()
    } else {
        format!(
            " (NOTE: the following are all alternative names for this same person — \
             not children, relatives, or other people: {})",
            aliases.join(", ")
        )
    };

    let prompt = if need_gender {
        // JSON-mode prompt for Person entities: description + gender
        format!(
            "You are a knowledge extraction assistant working with a historical memoir.{existing_hint}\n\
             Below are all excerpts from the document that mention \"{name}\"{alias_hint} (a {type_label}):\n\n\
             {evidence_text}\n\n\
             Based ONLY on the excerpts above, return ONLY a JSON object with these two fields \
             (no other text, no markdown fences):\n\
             {{\n  \"description\": \"<3-7 factual sentences about {name}. \
             You MUST include every specific fact found in the excerpts: \
             (1) who they are and their primary role or title, \
             (2) any years or dates mentioned (birth, arrival, marriage, death), \
             (3) places of origin, residence, or travel specifically named in the text, \
             (4) the full names of family members ONLY when the text EXPLICITLY states the \
             relationship using kinship terms directly attached to {name} — e.g. \
             '{name}\\'s daughter', 'son of {name}', '{name} married X'. \
             Do NOT name someone as a relative of {name} just because they appear in the same \
             passage — the evidence covers a broad memoir and other named people are usually \
             separate characters, not {name}\\'s relatives. \
             (5) organisations they founded, led, or belonged to. \
             (6) ALL notable persons explicitly mentioned in the excerpts as having visited, \
             met, corresponded with, or been personally connected to {name} — list every \
             full name; do not summarise or say 'and others'. \
             PRIORITISE specific named entities and dates over general characterisations. \
             Do NOT add information absent from the excerpts. \
             CRITICAL: Any aliases listed above are all names for {name} themselves — \
             do NOT treat them as names of their children, relatives, or other people.>\",\n\
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
        // Type-specific prose prompt (no JSON needed when only description is needed)
        let (sentence_count, type_guidance) = match entity_type.to_lowercase().as_str() {
            "place" | "location" => (
                "3–5",
                "PRIORITISE specific facts from the text: \
                 (1) founding year or key historical date, \
                 (2) exact street address or location within the city if mentioned, \
                 (3) physical description and geographic setting, \
                 (4) full names of specific persons associated with it, \
                 (5) notable events that occurred there. \
                 List all named persons and dates you find — do not omit them.",
            ),
            "organization" => (
                "3–5",
                "PRIORITISE specific facts from the text: \
                 (1) founding year and the full name of the founding person, \
                 (2) the organisation's full name, abbreviation, and stated purpose, \
                 (3) affiliated organisations named in the text, \
                 (4) full names of key members or leaders mentioned, \
                 (5) specific campaigns, events, or positions it is known for. \
                 List all named persons, dates, and affiliated bodies you find.",
            ),
            _ => (
                "2–3",
                "Include the specific facts from the excerpts: what it is, \
                 any founding year or key date, named persons associated with it, \
                 and its significance in the story.",
            ),
        };
        format!(
            "You are a knowledge extraction assistant working with a historical memoir.{existing_hint}\n\
             Below are all excerpts from the document that mention \"{name}\"{alias_hint} (a {type_label}):\n\n\
             {evidence_text}\n\n\
             Based ONLY on the excerpts above, write a concise {sentence_count} sentence description \
             of \"{name}\". {type_guidance} \
             Do NOT add information not present in the excerpts. \
             Output ONLY the description paragraph."
        )
    };

    let result = match call_llm(&prompt, url, model, need_gender).await {
        Some(text) if need_gender => parse_enrich_json(&text),
        Some(text) if text.len() >= 20 => EnrichResult {
            description: Some(text),
            gender: None,
        },
        _ => EnrichResult {
            description: None,
            gender: None,
        },
    };

    if let Some(ref desc) = result.description {
        let unsupported = unsupported_proper_nouns(desc, evidence_text, name, aliases);
        if !unsupported.is_empty() {
            tracing::warn!(
                "hallucination risk in \"{}\": claims {:?} not found in evidence",
                name,
                unsupported
            );
        }
    }

    result
}

/// Returns proper-noun tokens from `description` that do not appear in `evidence`.
/// Used to surface hallucinated names and places before they are written to the graph.
fn unsupported_proper_nouns(
    description: &str,
    evidence: &str,
    entity_name: &str,
    aliases: &[String],
) -> Vec<String> {
    const STOP_WORDS: &[&str] = &[
        "The",
        "A",
        "An",
        "In",
        "Of",
        "And",
        "For",
        "With",
        "By",
        "On",
        "At",
        "From",
        "To",
        "Is",
        "Was",
        "He",
        "She",
        "His",
        "Her",
        "Their",
        "This",
        "That",
        "These",
        "Those",
        "Based",
        "Only",
        "All",
        "Any",
        "Each",
        "Both",
        "Some",
        "Not",
        "No",
        "Born",
        "Known",
        "Also",
        "Early",
        "Later",
        "During",
        "After",
        "Before",
        "Member",
        "Leader",
        "Founder",
        "President",
        "Doctor",
        "Professor",
    ];

    let excluded: Vec<String> = std::iter::once(entity_name.to_lowercase())
        .chain(aliases.iter().map(|s| s.to_lowercase()))
        .collect();

    let evidence_lower = evidence.to_lowercase();
    let mut candidates: Vec<String> = Vec::new();

    for word in description.split_whitespace() {
        let word = word.trim_end_matches(|c: char| !c.is_alphanumeric());
        if word.len() < 3 {
            continue;
        }
        let first = word.chars().next().unwrap_or_default();
        if !first.is_uppercase() {
            continue;
        }
        if STOP_WORDS.contains(&word) {
            continue;
        }
        if word.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let lower = word.to_lowercase();
        if excluded
            .iter()
            .any(|ex| ex.contains(lower.as_str()) || lower.contains(ex.as_str()))
        {
            continue;
        }
        if !evidence_lower.contains(lower.as_str()) {
            candidates.push(word.to_string());
        }
    }

    candidates.sort();
    candidates.dedup();
    candidates
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
    let json_str = if end > start {
        &cleaned[start..end]
    } else {
        cleaned
    };

    let v: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => {
            return EnrichResult {
                description: None,
                gender: None,
            }
        }
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

    EnrichResult {
        description,
        gender,
    }
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

    // Use native /api/chat — supports "format":"json" and "stream":false properly.
    // /v1/chat/completions streams SSE by default and ignores "format":"json".
    let full_url = format!("{}/api/chat", url.trim_end_matches('/'));
    let mut body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": false,
        "options": {
            "temperature": 0.1,
            "num_predict": 1000,
        },
    });
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

    let v: serde_json::Value =
        tokio::time::timeout(std::time::Duration::from_secs(120), resp.json())
            .await
            .ok()?
            .ok()?;

    let text = v["message"]["content"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_string();

    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}
