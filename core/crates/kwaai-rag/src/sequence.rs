//! Sequence diagram layer: per-entity lifelines (TimelineEvent) and
//! cross-entity interactions (SequenceInteraction).
//!
//! Temporal data is extracted from ingested chunks via an LLM call, stored in
//! two redb tables, and retrieved as a Mermaid sequence diagram for TemporalEvent queries.
//!
//! Build the timeline with:
//!   `kwaainet rag graph timeline build --kb D6 --model llama3.1:8b`
//!
//! Retrieval is a graceful no-op when the timeline tables are empty — TemporalEvent
//! queries fall back to iterative retrieval automatically.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::graph::GraphStore;

// ── Data types ────────────────────────────────────────────────────────────────

/// A dated event attached to exactly one entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub entity_id: i64,
    pub entity_name: String,
    /// Raw date string from source text: "1884", "February 1914", "1970s"
    pub date_raw: Option<String>,
    /// ISO-padded for chronological sort: "1884-01-01", "1970-01-01"
    pub date_sort: String,
    pub description: String,
    /// "arrival" | "founding" | "death" | "meeting" | "declaration" | "removal" | "other"
    pub event_class: String,
    pub evidence_chunk_id: i64,
}

/// A dated interaction (arrow) between exactly two entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceInteraction {
    pub from_entity_id: i64,
    pub from_entity_name: String,
    pub to_entity_id: i64,
    pub to_entity_name: String,
    /// Raw date string from source text
    pub date_raw: Option<String>,
    /// ISO-padded for chronological sort
    pub date_sort: String,
    /// Verb phrase: "visits", "meets", "marries", "addresses", "farewells"
    pub label: String,
    pub evidence_chunk_id: i64,
}

// ── LLM extraction payload ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RawEvent {
    pub entity: String,
    pub date: Option<String>,
    pub description: String,
    pub class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawInteraction {
    pub from: String,
    pub to: String,
    pub date: Option<String>,
    pub label: String,
}

#[derive(Debug, Deserialize)]
struct TemporalPayload {
    #[serde(default)]
    events: Vec<RawEvent>,
    #[serde(default)]
    interactions: Vec<RawInteraction>,
}

// ── Date normalization ────────────────────────────────────────────────────────

/// Convert fuzzy date strings to ISO "YYYY-MM-DD" for sort-stable comparison.
pub fn normalize_date(raw: &str) -> String {
    let s = raw.trim();
    // 4-digit year only
    if let Some(y) = parse_4digit_year(s) {
        return format!("{y:04}-01-01");
    }
    // "Month YYYY"
    if let Some((m, y)) = parse_month_year(s) {
        return format!("{y:04}-{m:02}-01");
    }
    // "YYYY-MM" or "YYYY-MM-DD"
    if s.len() >= 7
        && s.chars().take(4).all(|c| c.is_ascii_digit())
        && s.chars().nth(4) == Some('-')
    {
        return format!("{}-01", &s[..7]);
    }
    // Decade: "1970s"
    if s.len() >= 5 && s.ends_with('s') {
        if let Ok(d) = s[..s.len() - 1].parse::<u32>() {
            return format!("{d:04}-01-01");
        }
    }
    // Fallback: keep raw, sort last
    "9999-12-31".to_string()
}

fn parse_4digit_year(s: &str) -> Option<u32> {
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 4 {
        digits.parse().ok()
    } else {
        None
    }
}

fn parse_month_year(s: &str) -> Option<(u32, u32)> {
    const MONTHS: &[(&str, u32)] = &[
        ("january", 1),
        ("february", 2),
        ("march", 3),
        ("april", 4),
        ("may", 5),
        ("june", 6),
        ("july", 7),
        ("august", 8),
        ("september", 9),
        ("october", 10),
        ("november", 11),
        ("december", 12),
        ("jan", 1),
        ("feb", 2),
        ("mar", 3),
        ("apr", 4),
        ("jun", 6),
        ("jul", 7),
        ("aug", 8),
        ("sep", 9),
        ("oct", 10),
        ("nov", 11),
        ("dec", 12),
    ];
    let lower = s.to_lowercase();
    for (name, m) in MONTHS {
        if lower.contains(name) {
            // Extract 4-digit year from same string
            let year_str: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            if year_str.len() == 4 {
                if let Ok(y) = year_str.parse::<u32>() {
                    return Some((*m, y));
                }
            }
        }
    }
    None
}

// ── LLM extraction ────────────────────────────────────────────────────────────

/// Call the LLM to extract dated events and interactions from a chunk of text.
///
/// `entity_names` — entity names already linked to this chunk, used as the
/// "known entity" hint list so the LLM stays within the established entity set.
/// Returns empty vecs (not an error) when the LLM returns no temporal data.
pub async fn extract_temporal_events(
    text: &str,
    entity_names: &[String],
    pronoun_map: &[(String, String)],
    inference_url: &str,
    model: &str,
) -> Result<(Vec<RawEvent>, Vec<RawInteraction>)> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let entity_list = if entity_names.is_empty() {
        "(none identified — use any entity names present in the text)".to_string()
    } else {
        entity_names.join(", ")
    };

    let coref_context = if pronoun_map.is_empty() {
        String::new()
    } else {
        let pairs = pronoun_map
            .iter()
            .map(|(surface, entity)| format!("'{surface}' → {entity}"))
            .collect::<Vec<_>>()
            .join("; ");
        format!("Coreference resolutions for this passage: {pairs}\n\n")
    };

    let prompt = format!(
        "Extract dated events from this historical text.\n\
         Known entities in this passage: {entity_list}\n\n\
         {coref_context}\
         Return ONLY valid JSON — no markdown, no explanation:\n\
         {{\n\
           \"events\": [\n\
             {{\"entity\": \"<name>\", \"date\": \"<year or month-year>\", \"description\": \"<what happened>\", \"class\": \"<arrival|founding|death|meeting|declaration|removal|other>\"}}\n\
           ],\n\
           \"interactions\": [\n\
             {{\"from\": \"<name>\", \"to\": \"<name>\", \"date\": \"<year or month-year>\", \"label\": \"<verb phrase>\"}}\n\
           ]\n\
         }}\n\n\
         Rules:\n\
         - Only extract events that have a clear temporal anchor (year, decade, or relative order).\n\
         - Only use entity names from the known list above.\n\
         - Use the coreference resolutions to determine WHO an event belongs to. If the text says \"I arrived\" and 'I' → Yousuf Rassool, the event entity is \"Yousuf Rassool\". If the text says \"my grandfather came from Mauritius\" and 'my grandfather' → J.M.H. Gool, the event entity is \"J.M.H. Gool\" — not the narrator.\n\
         - \"interactions\" are between exactly two different entities; \"events\" attach to exactly one entity.\n\
         - If no temporal events are present, return {{\"events\": [], \"interactions\": []}}.\n\n\
         Text:\n{text}"
    );

    let url = format!("{}/api/chat", inference_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": true,
        "options": {"temperature": 0.0, "num_predict": 1024, "num_ctx": 8192},
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .context("temporal extraction request failed")?;

    let raw_text = resp
        .text()
        .await
        .context("temporal extraction body read failed")?;

    let mut content_buf = String::new();
    for line in raw_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(c) = v["message"]["content"].as_str() {
                content_buf.push_str(c);
            }
        }
    }

    let cleaned = content_buf
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    match serde_json::from_str::<TemporalPayload>(cleaned) {
        Ok(p) => Ok((p.events, p.interactions)),
        Err(e) => {
            tracing::debug!("temporal extraction JSON parse failed: {e}; raw: {cleaned:.200}");
            Ok((vec![], vec![]))
        }
    }
}

// ── Build helpers ─────────────────────────────────────────────────────────────

/// Knowledge axiom: an entity is "present" in a chunk of text if its canonical
/// name or any alias appears in the text.
///
/// Matching rules:
/// - Name / alias tokens of length ≥ 4 are matched case-insensitively.
/// - The special alias "I" triggers a first-person check ("I " or "I'" anywhere
///   in the text) so the narrator entity is always considered present in
///   first-person passages without requiring the narrator's full name to appear.
pub fn entity_present_in_text(name: &str, aliases: &[String], text: &str) -> bool {
    let text_lower = text.to_lowercase();
    // Canonical name — match any token with len ≥ 4
    if name
        .split_whitespace()
        .any(|t| t.len() >= 4 && text_lower.contains(&t.to_lowercase()))
    {
        return true;
    }
    for alias in aliases {
        if alias.eq_ignore_ascii_case("I") {
            // First-person narrator: treat as present when text is first-person.
            if text_lower.contains(" i ")
                || text_lower.contains(" i'")
                || text_lower.starts_with("i ")
            {
                return true;
            }
        } else if alias.len() >= 3 && text_lower.contains(&alias.to_lowercase()) {
            return true;
        }
    }
    false
}

/// Resolve a raw entity name string to a graph entity ID using the same
/// multi-tier lookup used elsewhere in the retriever.
fn resolve_name(name: &str, graph: &GraphStore) -> Option<i64> {
    // Exact case-insensitive match
    if let Some(e) = graph.find_by_name(name) {
        return Some(e.id);
    }
    // Token-intersection fallback
    let tokens: Vec<String> = name
        .split_whitespace()
        .flat_map(|t| {
            let raw = t.to_lowercase();
            let trimmed: String = t
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if raw == trimmed {
                vec![raw]
            } else {
                vec![raw, trimmed]
            }
        })
        .filter(|t| t.len() >= 2)
        .collect();

    if tokens.is_empty() {
        return None;
    }
    let mut scores: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
    for token in &tokens {
        for &id in graph.find_ids_by_alias_token(token) {
            *scores.entry(id).or_default() += 1;
        }
        for id in graph.find_ids_by_name_token(token) {
            *scores.entry(id).or_default() += 1;
        }
    }
    scores.into_iter().max_by_key(|(_, s)| *s).map(|(id, _)| id)
}

/// Convert raw LLM events+interactions for a specific chunk into typed structs,
/// resolving entity names to graph IDs. Returns (events, interactions).
///
/// `chunk_text` is used to apply the **co-presence axiom**: an interaction
/// between A and B is only stored when both A and B are explicitly present in
/// the chunk text (by canonical name, alias, or first-person pronoun for the
/// narrator entity). This prevents spurious interactions caused by the LLM
/// pairing entities that appear in the same chunk for unrelated reasons.
pub fn resolve_extracted(
    raw_events: Vec<RawEvent>,
    raw_interactions: Vec<RawInteraction>,
    chunk_id: i64,
    chunk_text: &str,
    graph: &GraphStore,
) -> (Vec<TimelineEvent>, Vec<SequenceInteraction>) {
    let mut events = Vec::new();
    for ev in raw_events {
        // Axiom: date must be present and not a placeholder. The LLM sometimes
        // returns "not specified", "none", "no date", or null when no temporal
        // anchor exists — those events are noise; drop them.
        let date_str = match &ev.date {
            None => continue,
            Some(d) => {
                let d = d.trim().to_lowercase();
                if d.is_empty()
                    || d == "none"
                    || d == "null"
                    || d.starts_with("not")
                    || d.starts_with("no ")
                    || d.starts_with("unknown")
                    || d.starts_with("unspecified")
                {
                    continue;
                }
                ev.date.as_deref().unwrap()
            }
        };
        let Some(eid) = resolve_name(&ev.entity, graph) else {
            continue;
        };
        let entity_name = graph
            .get_entity(eid)
            .map(|e| e.name.clone())
            .unwrap_or_else(|| ev.entity.clone());
        // Note: no entity-presence check for events — attribution is done via the
        // coref pronoun_map passed to the LLM, which correctly attributes "my
        // grandfather came from India" to JMH Gool even when "Gool" doesn't appear
        // in the chunk text. The co-presence axiom applies to interactions only.
        let date_sort = normalize_date(date_str);
        events.push(TimelineEvent {
            entity_id: eid,
            entity_name,
            date_raw: ev.date,
            date_sort,
            description: ev.description,
            event_class: ev.class.unwrap_or_else(|| "other".to_string()),
            evidence_chunk_id: chunk_id,
        });
    }

    let mut interactions = Vec::new();
    for ia in raw_interactions {
        let Some(from_id) = resolve_name(&ia.from, graph) else {
            continue;
        };
        let Some(to_id) = resolve_name(&ia.to, graph) else {
            continue;
        };
        if from_id == to_id {
            continue;
        }
        let from_entity = graph.get_entity(from_id);
        let to_entity = graph.get_entity(to_id);
        // Co-presence axiom: both entities must be explicitly present in the chunk
        // text. Interactions fabricated by the LLM between entities that merely
        // appear in the same chunk for unrelated reasons are dropped here.
        if let (Some(fe), Some(te)) = (from_entity, to_entity) {
            if !entity_present_in_text(&fe.name, &fe.aliases, chunk_text)
                || !entity_present_in_text(&te.name, &te.aliases, chunk_text)
            {
                tracing::debug!(
                    "co-presence axiom: dropping interaction '{}' → '{}' (not both present in chunk {})",
                    fe.name, te.name, chunk_id
                );
                continue;
            }
        }
        let from_name = from_entity
            .map(|e| e.name.clone())
            .unwrap_or_else(|| ia.from.clone());
        let to_name = to_entity
            .map(|e| e.name.clone())
            .unwrap_or_else(|| ia.to.clone());
        let date_sort = ia
            .date
            .as_deref()
            .map(normalize_date)
            .unwrap_or_else(|| "9999-12-31".to_string());
        interactions.push(SequenceInteraction {
            from_entity_id: from_id,
            from_entity_name: from_name,
            to_entity_id: to_id,
            to_entity_name: to_name,
            date_raw: ia.date,
            date_sort,
            label: ia.label,
            evidence_chunk_id: chunk_id,
        });
    }

    (events, interactions)
}

// ── Mermaid rendering ─────────────────────────────────────────────────────────

/// Shorten a long entity name to a 12-char Mermaid participant alias.
fn mermaid_alias(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(4)
        .collect::<String>()
        .to_uppercase()
}

/// Render a Mermaid sequence diagram from a set of timeline events and interactions.
/// All items are sorted by date_sort before rendering.
pub fn render_mermaid(
    entity_label: &str,
    events: &[TimelineEvent],
    interactions: &[SequenceInteraction],
) -> String {
    if events.is_empty() && interactions.is_empty() {
        return format!(
            "[Sequence Diagram: {entity_label}]\n\nNo temporal events found in the knowledge base.\n"
        );
    }

    // Collect unique participants
    let mut participants: Vec<(i64, String)> = Vec::new();
    let mut seen_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();
    for ev in events {
        if seen_ids.insert(ev.entity_id) {
            participants.push((ev.entity_id, ev.entity_name.clone()));
        }
    }
    for ia in interactions {
        if seen_ids.insert(ia.from_entity_id) {
            participants.push((ia.from_entity_id, ia.from_entity_name.clone()));
        }
        if seen_ids.insert(ia.to_entity_id) {
            participants.push((ia.to_entity_id, ia.to_entity_name.clone()));
        }
    }

    // Build alias map: entity_id → short alias
    let alias_map: std::collections::HashMap<i64, String> = participants
        .iter()
        .enumerate()
        .map(|(i, (id, name))| {
            let base = mermaid_alias(name);
            // Ensure uniqueness by appending index if needed
            let alias = if i == 0 { base } else { format!("{base}{i}") };
            (*id, alias)
        })
        .collect();

    let mut lines = Vec::new();
    lines.push(format!("[Sequence Diagram: {entity_label}]"));
    lines.push(String::new());
    lines.push("sequenceDiagram".to_string());

    for (id, name) in &participants {
        let alias = alias_map.get(id).cloned().unwrap_or_default();
        // Escape name for Mermaid
        let safe_name = name.replace('"', "'");
        lines.push(format!("    participant {alias} as \"{safe_name}\""));
    }

    // Sort all items by date_sort
    let mut sorted_events: Vec<(&TimelineEvent, &str)> =
        events.iter().map(|e| (e, e.date_sort.as_str())).collect();
    let mut sorted_interactions: Vec<(&SequenceInteraction, &str)> = interactions
        .iter()
        .map(|i| (i, i.date_sort.as_str()))
        .collect();
    sorted_events.sort_by_key(|(_, d)| *d);
    sorted_interactions.sort_by_key(|(_, d)| *d);

    // Merge by date
    let mut ei = 0;
    let mut ii = 0;
    while ei < sorted_events.len() || ii < sorted_interactions.len() {
        let take_event = match (sorted_events.get(ei), sorted_interactions.get(ii)) {
            (Some((_, ed)), Some((_, id))) => ed <= id,
            (Some(_), None) => true,
            _ => false,
        };
        if take_event {
            let (ev, _) = sorted_events[ei];
            ei += 1;
            let alias = alias_map.get(&ev.entity_id).cloned().unwrap_or_default();
            let date_str = ev.date_raw.as_deref().unwrap_or("unknown date");
            lines.push(format!(
                "    Note over {alias}: {} — {}",
                date_str, ev.description
            ));
        } else {
            let (ia, _) = sorted_interactions[ii];
            ii += 1;
            let from_alias = alias_map
                .get(&ia.from_entity_id)
                .cloned()
                .unwrap_or_default();
            let to_alias = alias_map.get(&ia.to_entity_id).cloned().unwrap_or_default();
            let date_str = ia.date_raw.as_deref().unwrap_or("unknown date");
            lines.push(format!(
                "    {from_alias}->>{to_alias}: {} — {}",
                date_str, ia.label
            ));
        }
    }

    lines.join("\n")
}

// ── Retrieval ─────────────────────────────────────────────────────────────────

/// Retrieve a sequence diagram for a TemporalEvent query.
///
/// Collects TimelineEvents and SequenceInteractions for `entity_ids` and their
/// 1-hop neighbours. Returns a synthetic RetrievedChunk at score 3.0, or None
/// when the timeline tables are empty (timeline build hasn't been run yet).
pub fn retrieve_sequence(
    query: &str,
    entity_ids: &[i64],
    graph: &GraphStore,
) -> Option<crate::retriever::RetrievedChunk> {
    if entity_ids.is_empty() {
        return None;
    }

    // Collect subject entity IDs + 1-hop neighbours
    let mut all_ids: std::collections::HashSet<i64> = entity_ids.iter().copied().collect();
    for &eid in entity_ids {
        for (nid, _, _) in graph.neighbors_of(eid) {
            all_ids.insert(nid);
        }
    }

    let all_ids_vec: Vec<i64> = all_ids.into_iter().collect();

    let mut events: Vec<TimelineEvent> = graph.get_timeline_events(&all_ids_vec);
    let mut interactions: Vec<SequenceInteraction> = graph.get_interactions_for(&all_ids_vec);

    if events.is_empty() && interactions.is_empty() {
        return None;
    }

    // Quality gate: the PRIMARY entity (the first / highest-scoring match) must have at
    // least one event with a specific year. A 4-digit year not immediately followed by
    // 's' passes. "1884" → passes. "February 1914" → passes. "1920s" / "decades ago" → fail.
    //
    // We gate on entity_ids[0] only — not all matched entities and not neighbours — so
    // incidental place-name matches ("Cape Town" scoring 2 for "cape"+"town") don't
    // falsely open the gate for a query whose subject has no dated events.
    let primary_entity_id: Option<i64> = entity_ids.first().copied();
    let has_specific_year = events.iter().any(|e| {
        if Some(e.entity_id) != primary_entity_id {
            return false;
        }
        e.date_raw
            .as_deref()
            .map(|d| {
                let b = d.as_bytes();
                for i in 0..b.len().saturating_sub(3) {
                    if b[i..i + 4].iter().all(|c| c.is_ascii_digit()) {
                        let next = b.get(i + 4).copied();
                        if !matches!(next, Some(b's') | Some(b'S')) {
                            return true;
                        }
                    }
                }
                false
            })
            .unwrap_or(false)
    });
    if !has_specific_year {
        return None;
    }

    // Filter interactions to only those connecting entities we care about
    interactions.retain(|ia| {
        all_ids_vec.contains(&ia.from_entity_id) && all_ids_vec.contains(&ia.to_entity_id)
    });

    // Sort by date
    events.sort_by(|a, b| a.date_sort.cmp(&b.date_sort));
    interactions.sort_by(|a, b| a.date_sort.cmp(&b.date_sort));

    // Primary entity label: first entity's name
    let label = entity_ids
        .first()
        .and_then(|id| graph.get_entity(*id))
        .map(|e| e.name.clone())
        .unwrap_or_else(|| "Timeline".to_string());

    let mermaid = render_mermaid(&label, &events, &interactions);

    // Also build a prose summary for LLM readability
    let mut prose_lines = vec![format!("Timeline for {label}:")];
    for ev in &events {
        let date = ev.date_raw.as_deref().unwrap_or("(date unknown)");
        prose_lines.push(format!(
            "- {} — {} ({})",
            date, ev.description, ev.entity_name
        ));
    }
    for ia in &interactions {
        let date = ia.date_raw.as_deref().unwrap_or("(date unknown)");
        prose_lines.push(format!(
            "- {} — {} {} {}",
            date, ia.from_entity_name, ia.label, ia.to_entity_name
        ));
    }
    let prose = prose_lines.join("\n");

    // Prose first so the LLM sees facts before the diagram syntax.
    // The mermaid block is appended for completeness but may be truncated by max_chars.
    let combined = format!("{prose}\n\n---\n\n{mermaid}");

    // Synthetic ChunkMeta wrapping the diagram
    let chunk_meta = crate::meta_store::ChunkMeta {
        text: combined,
        doc_name: format!("sequence_diagram:{label}"),
        chunk_index: 0,
        surrounding: String::new(),
        page_num: None,
        ingested_at: String::new(),
        section_name: None,
        skip_extraction: false,
        section_note: Some(query.to_string()),
        section_type: crate::doc_schema::SectionType::default(),
    };

    // Score 1.9: below entity descriptions (2.05) so they appear first in context,
    // but above regular vector chunks (0.06–0.15) so timeline events are included.
    // The reorder_for_context pass places this chunk last due to odd-index reversal.
    Some(crate::retriever::RetrievedChunk {
        chunk_meta,
        score: 1.9,
        source_kb: None,
        rerank_score: None,
    })
}

// ── Text-based entity extraction for TemporalEvent queries ───────────────────

/// Extract entity IDs relevant to a temporal query using token matching.
///
/// Returns a list of entity IDs found in the query string, using the same
/// alias-token index already used for FamilyRelation queries.
pub fn extract_temporal_entity_ids(query: &str, graph: &GraphStore) -> Vec<i64> {
    let q = query.to_lowercase();
    let tokens: Vec<String> = q
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() >= 3)
        .map(|t| t.to_string())
        .collect();

    let mut scores: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
    for token in &tokens {
        for &id in graph.find_ids_by_alias_token(token) {
            *scores.entry(id).or_default() += 1;
        }
        for id in graph.find_ids_by_name_token(token) {
            *scores.entry(id).or_default() += 1;
        }
    }

    // Allow ≥1 hit — temporal queries name specific entities (places, historical figures)
    // whose tokens are rare enough that a single match is high-confidence. The ≥2 threshold
    // was silently dropping "JMH Gool" (only token: "gool") and short place names.
    let mut candidates: Vec<(i64, usize)> = scores.into_iter().filter(|(_, s)| *s >= 1).collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1));
    candidates.into_iter().map(|(id, _)| id).take(3).collect()
}
