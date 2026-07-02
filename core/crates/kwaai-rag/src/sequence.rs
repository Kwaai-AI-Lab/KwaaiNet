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

// ── Deterministic date scanning types ────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum DateMentionType {
    ExplicitYear,
    MonthYear,
    Decade,
    RelativePhrase,
}

/// A date reference found deterministically in chunk text (no LLM).
#[derive(Debug, Clone)]
pub struct DateMention {
    pub chunk_id: i64,
    pub date_raw: String,
    pub date_sort: String,
    pub date_confidence: f32,
    pub char_offset: usize,
    /// Full sentence containing the date (≤300 chars).
    pub sentence: String,
    pub mention_type: DateMentionType,
}

#[derive(Debug, Clone)]
pub enum AttributionMethod {
    ProximityHigh,
    ProximitySentence,
    SoleEntity,
    KinshipMap,
    LlmFallback,
}

/// A date mention attributed to a specific entity, with a composite confidence score.
#[derive(Debug, Clone)]
pub struct AttributedEvent {
    pub date_mention: DateMention,
    pub entity_id: i64,
    pub entity_name: String,
    pub description: String,
    pub event_class: String,
    /// date_confidence × attribution_confidence
    pub confidence: f32,
    pub attribution_method: AttributionMethod,
}

// ── Date normalization ────────────────────────────────────────────────────────

/// Convert fuzzy date strings to ISO "YYYY-MM-DD" for sort-stable comparison.
pub fn normalize_date(raw: &str) -> String {
    let s = raw.trim();
    // "Month YYYY" first — must be checked before bare-year so that
    // "February 1914" isn't collapsed to "1914" by parse_4digit_year.
    if let Some((m, y)) = parse_month_year(s) {
        return format!("{y:04}-{m:02}-01");
    }
    // Bare 4-digit year (digits-only filter; also catches "circa 1920", etc.)
    if let Some(y) = parse_4digit_year(s) {
        return format!("{y:04}-01-01");
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

/// Extract a 4-digit year from an entity field value like "1886", "1886-03-15",
/// "c.1884", or "April 1940". Returns None if no plausible year is found.
fn extract_year_from_field(value: &str) -> Option<u32> {
    let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
    // Look for the first run of exactly 4 consecutive digits representing a year.
    for i in 0..digits.len().saturating_sub(3) {
        if let Ok(y) = digits[i..i + 4].parse::<u32>() {
            if (1700..=2099).contains(&y) {
                return Some(y);
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
         1. Only extract events that have a clear temporal anchor (year, decade, or relative order).\n\
         2. Only use entity names from the known list above.\n\
         3. COREFERENCE: Use the coreference resolutions to determine WHO an event belongs to. If the text says \"I arrived\" and 'I' → Yousuf Rassool, the event entity is \"Yousuf Rassool\". If the text says \"my grandfather came from Mauritius\" and 'my grandfather' → J.M.H. Gool, the event entity is \"J.M.H. Gool\" — not the narrator.\n\
         4. NARRATOR RULE (applies only to the narrator entity): When a narrator describes events that clearly predate their birth — such as events from the 1700s–1800s or events where the narrator could not have been present — do NOT assign those events to the narrator entity. Example: if the text says \"In 1795 the Dutch settlers...\", do NOT create an event for \"Yousuf Rassool\". But DO create events for the actual subjects (\"Dutch settlers\", \"J.M.H. Gool\", etc.) — the narrator recounting a story does NOT suppress events for the story's subjects. A narrator saying \"my grandfather arrived in 1884\" → event for J.M.H. Gool (extract it). A narrator saying \"In 1900 the Congress declared\" → event for Congress/organisation (extract it, not for the narrator).\n\
         5. BIRTH/DEATH: Only use class \"birth\" when the text contains explicit birth language (\"was born\", \"born in\", \"birth of\", \"b.\"). Do not infer a birth year from a date that merely appears near an entity's name. Only use class \"death\" when the text contains explicit death language (\"died\", \"passed away\", \"death of\", \"d.\").\n\
         6. INTERACTION DIRECTION: \"from\" = the entity performing the action (the agent). \"to\" = the entity receiving or benefiting from it (the patient/destination). When a location or object is described possessively (\"his surgery\", \"her house\"), the possessor is the \"to\" entity — they receive the benefit. Example: \"Gandhi helped refurbish AH Gool's surgery\" → from=Gandhi, to=AH Gool.\n\
         7. LABEL: The \"label\" field must be a short verb phrase (3–6 words). Do not include entity names in the label. Wrong: \"Gandhi helped refurbish surgery\". Right: \"helped refurbish surgery\".\n\
         8. \"interactions\" are between exactly two different entities; \"events\" attach to exactly one entity.\n\
         9. If no temporal events are present, return {{\"events\": [], \"interactions\": []}}.\n\n\
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
/// Applied axioms (in order):
/// 1. **Date-presence**: events with null or placeholder dates are dropped.
/// 2. **Date-range**: events with unparseable dates or years outside [1700, 2099] are dropped.
/// 3. **Event dedup**: same (entity_id, event_class, year) pair within a chunk → keep first.
/// 4. **Co-presence**: interactions where either entity is absent from chunk text are dropped.
/// 5. **Interaction dedup**: same (from_id, to_id) pair within a chunk → keep first.
/// 6. **Temporal bounds**: events whose year falls outside an entity's known lifetime
///    (birthDate/deathDate fields, ±1 year margin) are dropped.
/// 7. **Cross-entity floor**: for entities with no seeded birthDate, the earliest known
///    presence year of any seeded parent (birthDate/arrived_cape_town/foundingDate) acts as
///    a temporal floor — events before it are impossible (child can't predate parent's arrival).
pub fn resolve_extracted(
    raw_events: Vec<RawEvent>,
    raw_interactions: Vec<RawInteraction>,
    chunk_id: i64,
    chunk_text: &str,
    graph: &GraphStore,
) -> (Vec<TimelineEvent>, Vec<SequenceInteraction>) {
    let mut events = Vec::new();
    // Dedup key: (entity_id, event_class, year_str)
    let mut seen_events: std::collections::HashSet<(i64, String, String)> =
        std::collections::HashSet::new();

    for ev in raw_events {
        // Axiom 1: date must be present and not a placeholder.
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
        let date_sort = normalize_date(date_str);
        // Axiom 2: unparseable dates normalise to "9999-12-31" (fallback); drop them.
        // Also drop any year outside the plausible historical range for any modern document.
        if date_sort == "9999-12-31" {
            tracing::debug!(
                "date-range axiom: dropping unparseable date {:?} in chunk {}",
                ev.date,
                chunk_id
            );
            continue;
        }
        let year: u32 = date_sort[..4].parse().unwrap_or(0);
        if !(1700..=2099).contains(&year) {
            tracing::debug!(
                "date-range axiom: dropping out-of-range year {year} in chunk {}",
                chunk_id
            );
            continue;
        }
        let Some(eid) = resolve_name(&ev.entity, graph) else {
            continue;
        };
        let entity_name = graph
            .get_entity(eid)
            .map(|e| e.name.clone())
            .unwrap_or_else(|| ev.entity.clone());
        // Axiom 3: deduplicate events with same (entity, class, year) in this chunk.
        // The LLM sometimes emits two slightly different descriptions of the same event.
        let event_class = ev.class.as_deref().unwrap_or("other").to_string();
        let dedup_key = (eid, event_class.clone(), date_sort[..4].to_string());
        if !seen_events.insert(dedup_key) {
            tracing::debug!(
                "event-dedup axiom: dropping duplicate ({entity_name}, {event_class}, {year}) in chunk {chunk_id}"
            );
            continue;
        }
        // Axiom 6: biographical temporal bounds — drop events whose year falls outside
        // the entity's known lifetime. Birth year comes from the "birthDate" or
        // "foundingDate" field; death/end year from "deathDate" or "dissolutionDate".
        // A ±1 margin is applied so announcements/obituaries in adjacent years survive.
        if let Some(entity) = graph.get_entity(eid) {
            let birth_year = entity
                .fields
                .get("birthDate")
                .or_else(|| entity.fields.get("foundingDate"))
                .and_then(|f| extract_year_from_field(&f.value));
            let death_year = entity
                .fields
                .get("deathDate")
                .or_else(|| entity.fields.get("dissolutionDate"))
                .and_then(|f| extract_year_from_field(&f.value));
            if let Some(by) = birth_year {
                if year + 1 < by {
                    tracing::debug!(
                        "temporal-bounds axiom: dropping event year {year} for {entity_name} (born ~{by}) in chunk {chunk_id}"
                    );
                    continue;
                }
            }
            if let Some(dy) = death_year {
                if year > dy + 1 {
                    tracing::debug!(
                        "temporal-bounds axiom: dropping event year {year} for {entity_name} (died ~{dy}) in chunk {chunk_id}"
                    );
                    continue;
                }
            }
            // Axiom 7: cross-entity temporal floor — if no birthDate is seeded for this
            // entity, use the earliest known presence year of any seeded parent.
            // A child entity cannot have events before their parent arrived/was born.
            if birth_year.is_none() {
                let parent_ids = graph.trusted_parent_ids(eid);
                let parent_floor: Option<u32> = parent_ids
                    .iter()
                    .filter_map(|pid| graph.get_entity(*pid))
                    .filter_map(|parent| {
                        parent
                            .fields
                            .get("birthDate")
                            .or_else(|| parent.fields.get("arrived_cape_town"))
                            .or_else(|| parent.fields.get("foundingDate"))
                            .and_then(|f| extract_year_from_field(&f.value))
                    })
                    .min();
                if let Some(floor) = parent_floor {
                    if year + 1 < floor {
                        tracing::debug!(
                            "cross-entity axiom: dropping event year {year} for {entity_name} \
                             (parent active from ~{floor}) in chunk {chunk_id}"
                        );
                        continue;
                    }
                }
            }
        }
        // Note: no entity-presence check for events — attribution is done via the
        // coref pronoun_map passed to the LLM. The co-presence axiom applies only
        // to interactions because coref-attributed events (e.g. "my grandfather
        // came from Mauritius" → JMH Gool) are valid even when the canonical name
        // doesn't appear in the chunk text.
        events.push(TimelineEvent {
            entity_id: eid,
            entity_name,
            date_raw: ev.date,
            date_sort,
            description: ev.description,
            event_class,
            evidence_chunk_id: chunk_id,
        });
    }

    let mut interactions = Vec::new();
    let mut seen_interactions: std::collections::HashSet<(i64, i64)> =
        std::collections::HashSet::new();

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
        // Axiom 4 (co-presence): both entities must be explicitly present in the chunk
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
        // Axiom 5 (interaction dedup): same (from_id, to_id) pair within a chunk → keep first.
        if !seen_interactions.insert((from_id, to_id)) {
            continue;
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

// ── Rule-based kinship extraction ─────────────────────────────────────────────

/// Kinship and membership keywords used for rule-based interaction extraction.
/// Each entry is `(keyword, relation_label)`. The entity whose name appears
/// BEFORE the keyword is the subject; the entity AFTER is the object.
///
/// "founded by" / "born to" are reversed: subject (founder/parent) comes after.
const KINSHIP_KEYWORDS: &[(&str, &str)] = &[
    ("son of", "child_of"),
    ("daughter of", "child_of"),
    ("wife of", "spouse_of"),
    ("husband of", "spouse_of"),
    ("married to", "spouse_of"),
    ("father of", "parent_of"),
    ("mother of", "parent_of"),
    ("brother of", "sibling_of"),
    ("sister of", "sibling_of"),
    ("grandfather of", "grandparent_of"),
    ("grandmother of", "grandparent_of"),
    ("grandson of", "grandchild_of"),
    ("granddaughter of", "grandchild_of"),
    ("foster son of", "foster_child_of"),
    ("foster daughter of", "foster_child_of"),
    ("foster father of", "foster_parent_of"),
    ("foster mother of", "foster_parent_of"),
    ("member of", "member_of"),
    ("founded by", "founded_by"),
    ("born to", "child_of"),
];

/// Keywords where the agent (subject) appears AFTER the keyword, not before.
/// E.g., "X founded by Y" → Y founds X; "X born to Y" → X is child of Y.
fn kinship_is_reversed(kw: &str) -> bool {
    matches!(kw, "founded by" | "born to")
}

/// Rule-based kinship/membership interaction extraction. **No LLM required.**
///
/// Scans each sentence in `text` for kinship keywords. When a keyword is found
/// and two distinct known entities flank it — one before, one after — an
/// Strip inline footnote text from a chunk before passing it to any LLM call.
///
/// PDF parsers interleave per-page footnote text with body text. The ENDNOTES
/// section skip in the doc schema handles the end-of-book section, but footnotes
/// at the bottom of each page appear inline in the body chunk stream. These are
/// stripped to prevent the LLM from treating footnote assertions as first-class
/// claims (e.g., "xiii J.M.H. Gool is not mentioned in Bunche's notes" becoming
/// a visited-by event, or "50 JMH Gool buried in Mowbray" polluting death extraction).
///
/// A footnote marker line starts with 1–3 ASCII digits OR lowercase Roman numerals
/// ([ivxlc]+), followed by a space, followed by an uppercase letter. Continuation
/// lines (non-blank lines immediately after the marker with no new marker) are also
/// stripped because footnotes wrap across lines in narrow page-bottom columns.
pub fn strip_inline_footnotes(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let n = lines.len();
    let mut keep = vec![true; n];
    let mut i = 0;
    while i < n {
        if is_footnote_marker_line(lines[i]) {
            keep[i] = false;
            // Strip continuation lines until blank line or next marker.
            let mut j = i + 1;
            while j < n {
                let next = lines[j].trim();
                if next.is_empty() || is_footnote_marker_line(lines[j]) {
                    break;
                }
                keep[j] = false;
                j += 1;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    lines
        .iter()
        .zip(keep.iter())
        .filter(|(_, &k)| k)
        .map(|(line, _)| *line)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns true when a line opens a footnote block.
///
/// Heuristics:
/// - 1–3 ASCII digits followed by a space and an uppercase letter.
///   (4-digit years like "1906" are NOT matched because we stop at j < 3.)
/// - Lowercase Roman-numeral sequence ([ivxlc]+, 1–8 chars) followed by a
///   space and an uppercase letter.
fn is_footnote_marker_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.is_empty() {
        return false;
    }
    let b = trimmed.as_bytes();

    // Numeric marker: 2–3 digits then space then uppercase.
    // Require ≥2 digits to avoid matching single-digit chapter/section numbers
    // like "3 The Group Areas Act" or "1 Introduction" which appear in chunked PDFs
    // when section headings lack a leading zero. Inline footnotes in D6 are all ≥10.
    let mut j = 0;
    while j < b.len() && j < 3 && b[j].is_ascii_digit() {
        j += 1;
    }
    if j >= 2 && j < b.len() && b[j] == b' ' {
        let rest = trimmed[j..].trim_start();
        if rest.starts_with(|c: char| c.is_uppercase()) {
            return true;
        }
    }

    // Roman numeral marker (lowercase only): [ivxlc]+ then space then uppercase
    let roman: &[u8] = b"ivxlc";
    let mut k = 0;
    while k < b.len() && k < 8 && roman.contains(&b[k]) {
        k += 1;
    }
    if k >= 2 && k < b.len() && b[k] == b' ' {
        // Require ≥2 chars to avoid matching a standalone "i" or "v" that could
        // be a sentence-initial word (rare in memoir body text, but possible).
        let rest = trimmed[k..].trim_start();
        if rest.starts_with(|c: char| c.is_uppercase()) {
            return true;
        }
    }

    false
}

/// interaction triple `(subject_id, object_id, label)` is emitted.
///
/// `known_entities` is `(entity_id, canonical_name, aliases)`. Tokens shorter
/// than 3 characters or equal to "I" are skipped to avoid spurious matches.
///
/// Deduplication: only the first occurrence of each `(subject_id, object_id)`
/// pair is kept (within the same chunk call).
pub fn extract_kinship_interactions(
    text: &str,
    known_entities: &[(i64, String, Vec<String>)],
) -> Vec<(i64, i64, String)> {
    let mut results: Vec<(i64, i64, String)> = Vec::new();
    let mut seen: std::collections::HashSet<(i64, i64)> = std::collections::HashSet::new();

    let sentences: Vec<&str> = text
        .split(['.', '?', '!', ';'])
        .map(str::trim)
        .filter(|s| s.len() >= 12)
        .collect();

    for sentence in sentences {
        let s_lower = sentence.to_lowercase();

        // Quick pre-check: does this sentence contain any kinship keyword?
        let Some((kw, label)) = KINSHIP_KEYWORDS.iter().find(|(kw, _)| s_lower.contains(kw)) else {
            continue;
        };
        let kw_pos = match s_lower.find(kw) {
            Some(p) => p,
            None => continue,
        };
        let kw_end = kw_pos + kw.len();
        let reversed = kinship_is_reversed(kw);

        // Map each known entity to its first occurrence position in this sentence.
        let mut positions: Vec<(usize, usize, i64)> = Vec::new(); // (start, end, id)
        for (id, name, aliases) in known_entities {
            let candidates =
                std::iter::once(name.as_str()).chain(aliases.iter().map(String::as_str));
            for tok in candidates {
                if tok.len() < 3 || tok.eq_ignore_ascii_case("I") {
                    continue;
                }
                if let Some(pos) = s_lower.find(&tok.to_lowercase()) {
                    positions.push((pos, pos + tok.len(), *id));
                    break;
                }
            }
        }

        if positions.len() < 2 {
            continue;
        }

        // Before keyword → subject; after keyword → object (unless reversed).
        let before = positions
            .iter()
            .filter(|(_, end, _)| *end <= kw_pos)
            .max_by_key(|(start, _, _)| *start);
        let after = positions
            .iter()
            .filter(|(start, _, _)| *start >= kw_end)
            .min_by_key(|(start, _, _)| *start);

        if let (Some((_, _, a_id)), Some((_, _, b_id))) = (before, after) {
            if a_id == b_id {
                continue;
            }
            let (sub_id, obj_id) = if reversed {
                (*b_id, *a_id)
            } else {
                (*a_id, *b_id)
            };
            if seen.insert((sub_id, obj_id)) {
                results.push((sub_id, obj_id, label.to_string()));
            }
        }
    }

    results
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

/// Build a map from first-person kinship role phrases (e.g. "my grandfather") to the
/// entity they resolve to, by walking the narrator's edges in the graph.
///
/// Returns HashMap<phrase, (entity_id, entity_name)>. Only `grandparent_of`, `parent_of`
/// (disambiguated via outgoing/incoming direction), and `spouse_of` are mapped — these are
/// the relation types seeded by the family-tree YAML and present in D6.
///
/// Used in timeline build to augment coreference candidates per chunk: when "my grandfather"
/// appears in a chunk, the narrator's grandparent entity is injected into the entity whitelist
/// even if its canonical name does not appear verbatim in the chunk text.
pub fn narrator_kinship_map(
    narrator_id: i64,
    graph: &GraphStore,
) -> std::collections::HashMap<String, (i64, String)> {
    use std::collections::{HashMap, HashSet};

    // Narrator's directly outgoing parent_of edges → these are the narrator's children.
    // All other "parent_of" entries in neighbors_of are incoming (entity → narrator).
    let narrator_children: HashSet<i64> = graph
        .outgoing_relations(narrator_id)
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, rel, _, _)| rel == "parent_of")
        .map(|(id, _, _, _)| id)
        .collect();

    let mut map: HashMap<String, (i64, String)> = HashMap::new();

    for (nbr_id, rel_type, _) in graph.neighbors_of(narrator_id) {
        let Some(entity) = graph.get_entity(nbr_id) else {
            continue;
        };
        let name = entity.name.clone();
        let gender = entity.gender.as_deref().unwrap_or("");

        match rel_type.as_str() {
            "grandparent_of" => {
                // Bidirectional, but "grandparent_of" on adj[narrator] always means
                // the neighbour IS a grandparent of the narrator (the YAML seeds it that way).
                let (singular, possessive) = match gender {
                    "Male" => ("grandfather", "grandpa"),
                    "Female" => ("grandmother", "grandma"),
                    _ => ("grandparent", "grandparent"),
                };
                for phrase in [
                    singular,
                    possessive,
                    &format!("my {singular}"),
                    &format!("my {possessive}"),
                ] {
                    map.entry(phrase.to_string())
                        .or_insert_with(|| (nbr_id, name.clone()));
                }
            }
            "parent_of" => {
                if narrator_children.contains(&nbr_id) {
                    // narrator → child edge; skip
                    continue;
                }
                // parent → narrator edge (narrator's parent)
                let role = match gender {
                    "Male" => "father",
                    "Female" => "mother",
                    _ => continue,
                };
                for phrase in [role, &format!("my {role}")] {
                    map.entry(phrase.to_string())
                        .or_insert_with(|| (nbr_id, name.clone()));
                }
            }
            "spouse_of" => {
                let role = match gender {
                    "Male" => "husband",
                    "Female" => "wife",
                    _ => continue,
                };
                for phrase in [role, &format!("my {role}")] {
                    map.entry(phrase.to_string())
                        .or_insert_with(|| (nbr_id, name.clone()));
                }
            }
            _ => {}
        }
    }

    map
}

// ── Deterministic date scanning ───────────────────────────────────────────────

/// Relative date phrases mapped to approximate years for D6 memoir context.
/// Year 0 = too vague to emit — these are recognised but skipped.
const RELATIVE_DATE_PHRASES: &[(&str, u32)] = &[
    ("at the turn of the century", 1900),
    ("turn of the century", 1900),
    ("the war years", 1941),
    ("during the war", 1941),
    ("war time", 1941),
    ("wartime", 1941),
    ("before the removals", 1960),
    ("during the removals", 1970),
    ("after the removals", 1985),
    ("after independence", 1962),
    ("since independence", 1962),
    ("before independence", 1959),
    // vague — skip
    ("in the early days", 0),
    ("at that time", 0),
    ("in those days", 0),
];

/// Extract the sentence (or ≤300-char window) that contains `char_offset`.
fn extract_sentence_at(text: &str, char_offset: usize) -> String {
    let safe_offset = char_offset.min(text.len());
    let before = &text[..safe_offset];
    let start = before
        .rfind(". ")
        .or_else(|| before.rfind("! "))
        .or_else(|| before.rfind("? "))
        .or_else(|| before.rfind('\n'))
        .map(|p| p + 1)
        .unwrap_or(0);
    let after = &text[safe_offset..];
    let end = after
        .find(". ")
        .or_else(|| after.find("! "))
        .or_else(|| after.find("? "))
        .or_else(|| after.find('\n'))
        .map(|p| p + safe_offset + 2)
        .unwrap_or(text.len());
    let sentence = text[start..end.min(text.len())].trim().to_string();
    if sentence.len() <= 300 {
        sentence
    } else {
        let center = safe_offset.saturating_sub(start);
        let lo = center.saturating_sub(150);
        let hi = (lo + 300).min(sentence.len());
        // snap to char boundary
        let lo = sentence
            .char_indices()
            .map(|(i, _)| i)
            .filter(|&i| i <= lo)
            .last()
            .unwrap_or(0);
        sentence[lo..hi].trim().to_string()
    }
}

/// Infer the event class from a sentence using keyword matching.
fn infer_event_class(sentence: &str) -> String {
    let s = sentence.to_lowercase();
    if s.contains("was born") || s.contains("born in") || s.contains("birth of") || s.contains(" b.") {
        return "birth".to_string();
    }
    if s.contains("died") || s.contains("passed away") || s.contains("death of") || s.contains(" d.") {
        return "death".to_string();
    }
    if s.contains("married") || s.contains("wedded") || s.contains("marriage") || s.contains("wed to") {
        return "marriage".to_string();
    }
    if s.contains("removed") || s.contains("evicted") || s.contains("demolished") || s.contains("forced out") || s.contains("bulldozed") || s.contains("displaced") {
        return "removal".to_string();
    }
    if s.contains("declared") || s.contains("declaration") || s.contains("announced") || s.contains("proclamation") || s.contains("enacted") || s.contains("passed into law") {
        return "declaration".to_string();
    }
    if s.contains("spoke at") || s.contains("addressed") || s.contains("conference") || s.contains("congress") || s.contains("convention") || s.contains("meeting of") || s.contains("gathering of") {
        return "meeting".to_string();
    }
    if s.contains("founded") || s.contains("established") || s.contains("built") || s.contains("opened") || s.contains("formed the") || s.contains("created the") {
        return "founding".to_string();
    }
    if s.contains("appointed") || s.contains("elected") || s.contains("became president") || s.contains("became chair") || s.contains("assumed the role") || s.contains("served as") {
        return "appointment".to_string();
    }
    if s.contains("attended") || s.contains("enrolled at") || s.contains("graduated") || s.contains("studied at") || s.contains("matriculated") || s.contains("training college") {
        return "education".to_string();
    }
    if s.contains("published") || s.contains("wrote the") || s.contains("appeared in") || s.contains("printed in") {
        return "publication".to_string();
    }
    if s.contains("arrived") || s.contains("came to") || s.contains("moved to") || s.contains("settled in") || s.contains("returned to") || s.contains("went to live") {
        return "arrival".to_string();
    }
    "other".to_string()
}

/// Scan a chunk of text for date references deterministically (no LLM).
///
/// Detects:
/// - Bare 4-digit years [1700, 2099] — confidence 0.90
/// - Month + year ("February 1914") — confidence 0.75
/// - Decades ("the 1880s") — confidence 0.50
/// - Relative phrases from RELATIVE_DATE_PHRASES — confidence 0.30
///
/// Each mention records the surrounding sentence for context and the byte
/// offset for proximity scoring during entity attribution.
pub fn scan_chunk_for_dates(text: &str, chunk_id: i64) -> Vec<DateMention> {
    let mut mentions: Vec<DateMention> = Vec::new();
    // Track (start_byte, end_byte) spans already claimed to avoid duplicate matches.
    let mut covered: Vec<(usize, usize)> = Vec::new();

    let is_covered = |covered: &[(usize, usize)], offset: usize| {
        covered.iter().any(|&(s, e)| offset >= s && offset < e)
    };

    // Pass 1 — relative phrases (longest first to prevent substring clobber).
    let text_lower = text.to_lowercase();
    let mut phrases: Vec<(&str, u32)> = RELATIVE_DATE_PHRASES.to_vec();
    phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    for (phrase, approx_year) in &phrases {
        if *approx_year == 0 {
            continue;
        }
        let mut search = 0usize;
        while let Some(rel) = text_lower[search..].find(phrase) {
            let off = search + rel;
            let end = off + phrase.len();
            if !is_covered(&covered, off) {
                covered.push((off, end));
                mentions.push(DateMention {
                    chunk_id,
                    date_raw: phrase.to_string(),
                    date_sort: format!("{:04}-01-01", approx_year),
                    date_confidence: 0.30,
                    char_offset: off,
                    sentence: extract_sentence_at(text, off),
                    mention_type: DateMentionType::RelativePhrase,
                });
            }
            search = end;
        }
    }

    // Pass 2 — scan for 4-digit years at word boundaries.
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i + 3 < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        // Measure the digit run starting at i
        let digit_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        let digit_end = i;
        if digit_end - digit_start != 4 {
            continue;
        }
        // Word-boundary check: not preceded or followed by a digit
        if digit_start > 0 && bytes[digit_start - 1].is_ascii_digit() {
            continue;
        }
        if i < bytes.len() && bytes[i].is_ascii_digit() {
            continue;
        }
        let year_str = &text[digit_start..digit_end];
        let Ok(year) = year_str.parse::<u32>() else {
            continue;
        };
        if !(1700..=2099).contains(&year) {
            continue;
        }
        if is_covered(&covered, digit_start) {
            continue;
        }

        // Check for decade: year divisible by 10 and followed by 's' or "'s"
        let is_decade = year % 10 == 0 && {
            let rest = &text[digit_end..];
            rest.starts_with('s') || rest.starts_with("'s")
        };

        // Check for month prefix: scan up to 25 bytes before the year for a month name
        let look_back_start = digit_start.saturating_sub(25);
        let preceding = &text[look_back_start..digit_start];
        let preceding_with_year = format!("{preceding}{year_str}");
        let month_prefix = parse_month_year(&preceding_with_year);

        let (date_raw, date_sort, date_confidence, mention_type, effective_offset) =
            if let Some((m, _y)) = month_prefix {
                // Find the month name in the preceding text to get its offset
                const MONTH_NAMES: &[&str] = &[
                    "january", "february", "march", "april", "may", "june", "july",
                    "august", "september", "october", "november", "december",
                    "jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
                ];
                let preceding_lower = preceding.to_lowercase();
                let month_word = MONTH_NAMES
                    .iter()
                    .filter_map(|mn| preceding_lower.rfind(mn).map(|p| (*mn, p)))
                    .max_by_key(|(_, p)| *p);
                if let Some((mn, rel_pos)) = month_word {
                    let abs_month_off = look_back_start + rel_pos;
                    // Recover the original casing from text
                    let raw_month = &text[abs_month_off..abs_month_off + mn.len()];
                    let raw = format!("{raw_month} {year_str}");
                    // Mark the month span as covered too
                    covered.push((abs_month_off, digit_end));
                    (
                        raw,
                        format!("{year:04}-{m:02}-01"),
                        0.75f32,
                        DateMentionType::MonthYear,
                        abs_month_off,
                    )
                } else {
                    // parse_month_year succeeded but we can't locate the word — fall back
                    (
                        year_str.to_string(),
                        format!("{year:04}-01-01"),
                        0.90f32,
                        DateMentionType::ExplicitYear,
                        digit_start,
                    )
                }
            } else if is_decade {
                covered.push((digit_start, digit_end + 1));
                (
                    format!("{year_str}s"),
                    format!("{year:04}-01-01"),
                    0.50f32,
                    DateMentionType::Decade,
                    digit_start,
                )
            } else {
                covered.push((digit_start, digit_end));
                (
                    year_str.to_string(),
                    format!("{year:04}-01-01"),
                    0.90f32,
                    DateMentionType::ExplicitYear,
                    digit_start,
                )
            };

        mentions.push(DateMention {
            chunk_id,
            date_raw,
            date_sort,
            date_confidence,
            char_offset: effective_offset,
            sentence: extract_sentence_at(text, effective_offset),
            mention_type,
        });
    }

    mentions.sort_by_key(|m| m.char_offset);
    mentions
}

/// Attribute each DateMention to chunk entities using heuristic confidence scoring.
///
/// A single date mention may be attributed to MULTIPLE entities when several meet the
/// confidence threshold. This matches the LLM-first behaviour which naturally emits multiple
/// events per date, and prevents "winner-takes-all" from silencing place entities (e.g.
/// District Six) whenever a person entity also appears near the same date mention.
///
/// Scoring tiers (attribution_confidence):
/// 1. Kinship phrase in sentence → 0.85 (KinshipMap)
/// 2. Sole entity in chunk → 0.85 (SoleEntity)
/// 3. Entity within ~120 chars of date in sentence → 0.85 (ProximityHigh)
/// 4. Entity elsewhere in same sentence → 0.60 (ProximitySentence)
/// 5. Narrator via first-person pronouns in sentence → 0.60 (ProximitySentence)
/// 6. Entity in ±400-char context window (adjacent sentence) → 0.55
/// 7. Entity in chunk but not context window → 0.35 → LLM queue
///
/// composite = date_confidence × attribution_confidence.
/// Only sends to LLM when NO entity meets `confidence_threshold`.
pub fn attribute_dates_to_entities(
    mentions: Vec<DateMention>,
    chunk_entities: &[(i64, String, Vec<String>)],
    narrator_kinship: &std::collections::HashMap<String, (i64, String)>,
    chunk_text: &str,
    confidence_threshold: f32,
) -> (Vec<AttributedEvent>, Vec<DateMention>) {
    let mut high_conf: Vec<AttributedEvent> = Vec::new();
    let mut low_conf: Vec<DateMention> = Vec::new();

    if chunk_entities.is_empty() {
        low_conf.extend(mentions);
        return (high_conf, low_conf);
    }

    let sole_entity = chunk_entities.len() == 1;

    for mention in mentions {
        let sentence = mention.sentence.clone();
        let sentence_lower = sentence.to_lowercase();
        let date_pos_in_sentence = sentence.find(&mention.date_raw).unwrap_or(0);

        // ±400-char context window for adjacent-sentence entity detection.
        let ctx_start = mention.char_offset.saturating_sub(400);
        let ctx_end = (mention.char_offset + 400).min(chunk_text.len());
        let ctx_window_lower = chunk_text[ctx_start..ctx_end].to_lowercase();

        // First-person narrator detection.
        let sentence_is_first_person = sentence_lower.contains(" i ")
            || sentence_lower.starts_with("i ")
            || sentence_lower.contains(" my ")
            || sentence_lower.contains(" me ")
            || sentence_lower.contains("'ve ")
            || sentence_lower.contains("'d ")
            || sentence_lower.contains("'m ");

        // Entities already attributed for this mention (by id) — deduplicate multi-path hits.
        let mut attributed_ids: std::collections::HashSet<i64> =
            std::collections::HashSet::new();
        let mut attributed: Vec<AttributedEvent> = Vec::new();

        // ── Step 1: kinship map (highest priority) ────────────────────────────────────
        for (phrase, (eid, ename)) in narrator_kinship {
            if !sentence_lower.contains(phrase.as_str()) {
                continue;
            }
            let (final_id, final_name) = chunk_entities
                .iter()
                .find(|(id, _, _)| id == eid)
                .map(|(id, name, _)| (*id, name.clone()))
                .unwrap_or((*eid, ename.clone()));
            let composite = mention.date_confidence * 0.85;
            if composite >= confidence_threshold && attributed_ids.insert(final_id) {
                attributed.push(AttributedEvent {
                    description: sentence.clone(),
                    event_class: infer_event_class(&sentence),
                    entity_id: final_id,
                    entity_name: final_name,
                    confidence: composite,
                    attribution_method: AttributionMethod::KinshipMap,
                    date_mention: mention.clone(),
                });
            }
        }

        // ── Step 2: sole entity (fast path) ──────────────────────────────────────────
        if sole_entity {
            let (id, name, aliases) = &chunk_entities[0];
            if entity_present_in_text(name, aliases, chunk_text) {
                let composite = mention.date_confidence * 0.85;
                if composite >= confidence_threshold && attributed_ids.insert(*id) {
                    attributed.push(AttributedEvent {
                        description: sentence.clone(),
                        event_class: infer_event_class(&sentence),
                        entity_id: *id,
                        entity_name: name.clone(),
                        confidence: composite,
                        attribution_method: AttributionMethod::SoleEntity,
                        date_mention: mention.clone(),
                    });
                }
            }
            // With exactly one entity, only that entity can receive this mention.
            if !attributed.is_empty() {
                high_conf.extend(attributed);
            } else {
                low_conf.push(mention);
            }
            continue;
        }

        // ── Step 3: proximity / context scoring (multiple entities) ───────────────────
        // Emit for ALL entities that meet the threshold, not just the single best.
        // A dated sentence can validly involve multiple entities (person + place, etc.).
        for (id, name, aliases) in chunk_entities {
            // Skip entities already emitted via kinship map.
            if attributed_ids.contains(id) {
                continue;
            }

            // Entity position in the sentence.
            let ent_pos_in_sentence = {
                let by_name = sentence_lower.find(&name.to_lowercase());
                let by_alias = aliases
                    .iter()
                    .filter(|a| a.len() >= 3 && !a.eq_ignore_ascii_case("I"))
                    .filter_map(|a| sentence_lower.find(&a.to_lowercase()))
                    .min();
                match (by_name, by_alias) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                }
            };

            // Entity in ±400-char context window (adjacent sentence check).
            let ent_in_context = name
                .split_whitespace()
                .filter(|t| t.len() >= 4)
                .any(|t| ctx_window_lower.contains(&t.to_lowercase()))
                || aliases
                    .iter()
                    .filter(|a| a.len() >= 3 && !a.eq_ignore_ascii_case("I"))
                    .any(|a| ctx_window_lower.contains(&a.to_lowercase()));

            let is_narrator = aliases.iter().any(|a| a.eq_ignore_ascii_case("I"));

            let (attr_conf, method) = if let Some(ep) = ent_pos_in_sentence {
                let dist = (ep as isize - date_pos_in_sentence as isize).unsigned_abs();
                if dist <= 120 {
                    (0.85f32, AttributionMethod::ProximityHigh)
                } else {
                    (0.60f32, AttributionMethod::ProximitySentence)
                }
            } else if is_narrator && sentence_is_first_person {
                // Narrator via first-person pronouns. Axiom 6 filters pre-birth events.
                // Skip if the sentence is clearly a historical reference the narrator is
                // recounting rather than a personal event (e.g. "the way the British had
                // done in 1795" — the year appears in a subordinate historical clause).
                let is_hypothetical = sentence_lower.contains("in my imagination")
                    || sentence_lower.contains("could see")
                    || sentence_lower.contains("could imagine")
                    || sentence_lower.contains("the way the")
                    || sentence_lower.contains("as they had")
                    || sentence_lower.contains("as had been done")
                    || sentence_lower.contains("had done in")
                    || sentence_lower.contains("had been in");
                if is_hypothetical {
                    continue;
                }
                (0.60f32, AttributionMethod::ProximitySentence)
            } else if ent_in_context {
                // Entity in adjacent sentence — common memoir prose pattern.
                (0.55f32, AttributionMethod::ProximitySentence)
            } else if entity_present_in_text(name, aliases, chunk_text) {
                (0.35f32, AttributionMethod::ProximitySentence)
            } else {
                continue;
            };

            let composite = mention.date_confidence * attr_conf;
            if composite >= confidence_threshold && attributed_ids.insert(*id) {
                attributed.push(AttributedEvent {
                    description: sentence.clone(),
                    event_class: infer_event_class(&sentence),
                    entity_id: *id,
                    entity_name: name.clone(),
                    confidence: composite,
                    attribution_method: method,
                    date_mention: mention.clone(),
                });
            }
        }

        if !attributed.is_empty() {
            high_conf.extend(attributed);
        } else {
            // No entity above threshold — send to LLM fallback.
            low_conf.push(mention);
        }
    }

    (high_conf, low_conf)
}

/// Selective LLM fallback for low-confidence DateMentions.
///
/// Batches all unresolved mentions for a single chunk into one LLM call using a
/// focused attribution prompt — no 9-rule machinery, just ask which entity and
/// what happened. Null-entity responses are discarded.
pub async fn extract_events_for_uncertain(
    mentions: Vec<DateMention>,
    chunk_text: &str,
    entity_names: &[String],
    pronoun_map: &[(String, String)],
    inference_url: &str,
    model: &str,
) -> Result<Vec<RawEvent>> {
    if mentions.is_empty() || entity_names.is_empty() {
        return Ok(vec![]);
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()?;

    let entity_list = entity_names.join(", ");
    let coref_context = if pronoun_map.is_empty() {
        String::new()
    } else {
        let pairs = pronoun_map
            .iter()
            .map(|(s, e)| format!("'{s}' → {e}"))
            .collect::<Vec<_>>()
            .join("; ");
        format!("Coreference: {pairs}\n")
    };
    let date_refs = mentions
        .iter()
        .map(|m| format!("- \"{}\": {}", m.date_raw, m.sentence))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "You are a historical event extractor.\n\
         PASSAGE:\n{chunk_text}\n\n\
         DATE REFERENCES (each line: date_raw: sentence containing it):\n{date_refs}\n\n\
         ENTITIES IN THIS PASSAGE: {entity_list}\n\
         {coref_context}\n\
         For each date reference output one JSON object per line:\n\
         {{\"date\": \"<date_raw>\", \"entity\": \"<entity name from list above>\", \
         \"description\": \"<what happened in one sentence>\", \
         \"class\": \"<birth|death|arrival|founding|removal|meeting|declaration|appointment|education|publication|other>\"}}\n\
         If no entity clearly matches, output {{\"date\": \"...\", \"entity\": null}}.\n\
         Do not invent events not supported by the passage.\n\
         Output only JSON objects, one per line — no explanation, no markdown."
    );

    let url = format!("{}/api/chat", inference_url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "stream": true,
        "options": {"temperature": 0.0, "num_predict": 512, "num_ctx": 8192},
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .context("uncertain event extraction request failed")?;
    let raw_text = resp
        .text()
        .await
        .context("uncertain event extraction body read failed")?;

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

    let mut events = Vec::new();
    for line in content_buf.lines() {
        let line = line
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if v["entity"].is_null() {
                continue;
            }
            let entity = v["entity"].as_str().unwrap_or("").to_string();
            if entity.is_empty() {
                continue;
            }
            events.push(RawEvent {
                entity,
                date: v["date"].as_str().map(|s| s.to_string()),
                description: v["description"].as_str().unwrap_or("").to_string(),
                class: v["class"].as_str().map(|s| s.to_string()),
            });
        }
    }

    tracing::debug!(
        "uncertain extraction: {} mentions → {} events",
        mentions.len(),
        events.len()
    );
    Ok(events)
}
