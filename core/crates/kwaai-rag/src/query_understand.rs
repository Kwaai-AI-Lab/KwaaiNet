//! Step 2.2 — Understand Query: classify a natural-language query into a typed
//! `QueryStructure` (intent + target entity + relation type).
//!
//! Step 2.3 — Resolve Entity: map the raw entity name strings in `QueryStructure`
//! to graph entity IDs.
//!
//! Three classification methods:
//!   `Rule`   — keyword patterns, zero latency, deterministic.
//!   `Llm`    — single LLM JSON call, flexible, adds ~2 s latency.
//!   `Hybrid` — Rule first; LLM fallback when intent is `Unknown`.

use crate::graph::GraphStore;

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ClassifyMethod {
    #[default]
    Rule,
    Llm,
    Hybrid,
}

/// Controls how graph query results are integrated into the LLM context.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GraphMode {
    /// Existing behaviour: inject one entity description as a synthetic chunk.
    #[default]
    Inject,
    /// Insert a structured graph-facts block at score 3.0 before Top-K chunks.
    Prepend,
    /// For FamilyRelation queries with a resolved entity, replace all chunks
    /// with a single authoritative graph-facts chunk (zero retrieval noise).
    Replace,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelDirection {
    Outgoing,
    Incoming,
    Both,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryIntent {
    FamilyRelation {
        relation: String,
        direction: RelDirection,
    },
    EntityDescription,
    OrgMembership,
    TemporalEvent,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct QueryStructure {
    pub intent: QueryIntent,
    /// Raw entity name strings extracted from the query (lowercased, before graph resolution).
    pub target_entities: Vec<String>,
    /// True when the query references "the author" / "the narrator" rather than a named entity.
    pub anchor_is_author: bool,
}

impl QueryStructure {
    pub fn unknown() -> Self {
        QueryStructure {
            intent: QueryIntent::Unknown,
            target_entities: vec![],
            anchor_is_author: false,
        }
    }
}

// ── Rule-based classifier ─────────────────────────────────────────────────────

/// "keyword of X" patterns → (keyword, relation_type, direction_from_X)
const OF_PATTERNS: &[(&str, &str, RelDirection)] = &[
    ("children of ", "parent_of", RelDirection::Outgoing),
    ("sons of ", "parent_of", RelDirection::Outgoing),
    ("daughters of ", "parent_of", RelDirection::Outgoing),
    (
        "grandchildren of ",
        "grandparent_of",
        RelDirection::Outgoing,
    ),
    ("wife of ", "spouse_of", RelDirection::Both),
    ("husband of ", "spouse_of", RelDirection::Both),
    ("spouse of ", "spouse_of", RelDirection::Both),
    ("father of ", "child_of", RelDirection::Outgoing),
    ("mother of ", "child_of", RelDirection::Outgoing),
    ("parent of ", "child_of", RelDirection::Outgoing),
    ("siblings of ", "sibling_of", RelDirection::Both),
    ("brothers of ", "sibling_of", RelDirection::Both),
    ("sisters of ", "sibling_of", RelDirection::Both),
    ("grandfather of ", "grandchild_of", RelDirection::Outgoing),
    ("grandmother of ", "grandchild_of", RelDirection::Outgoing),
];

/// "X's keyword" possessive patterns → (keyword_suffix, relation_type, direction_from_X)
const POSS_PATTERNS: &[(&str, &str, RelDirection)] = &[
    ("'s children", "parent_of", RelDirection::Outgoing),
    ("'s sons", "parent_of", RelDirection::Outgoing),
    ("'s daughters", "parent_of", RelDirection::Outgoing),
    ("'s grandchildren", "grandparent_of", RelDirection::Outgoing),
    ("'s wife", "spouse_of", RelDirection::Both),
    ("'s husband", "spouse_of", RelDirection::Both),
    ("'s spouse", "spouse_of", RelDirection::Both),
    ("'s father", "child_of", RelDirection::Outgoing),
    ("'s mother", "child_of", RelDirection::Outgoing),
    ("'s parents", "child_of", RelDirection::Outgoing),
    ("'s parent", "child_of", RelDirection::Outgoing),
    ("'s siblings", "sibling_of", RelDirection::Both),
    ("'s brothers", "sibling_of", RelDirection::Both),
    ("'s sisters", "sibling_of", RelDirection::Both),
    ("'s grandfather", "grandchild_of", RelDirection::Outgoing),
    ("'s grandmother", "grandchild_of", RelDirection::Outgoing),
    ("'s grandpa", "grandchild_of", RelDirection::Outgoing),
    ("'s granddad", "grandchild_of", RelDirection::Outgoing),
];

const AUTHOR_ANCHORS: &[&str] = &[
    "author",
    "author's",
    "narrator",
    "narrator's",
    "the author",
    "the narrator",
];

fn is_author_anchor(s: &str) -> bool {
    AUTHOR_ANCHORS.contains(&s.trim())
}

fn strip_question_words(s: &str) -> &str {
    const PREFIXES: &[&str] = &[
        "who were the ",
        "who was the ",
        "who is the ",
        "who are the ",
        "what were the ",
        "what was the ",
        "tell me more about the ",
        "tell me more about ",
        "tell me about the ",
        "tell me about ",
        "describe the ",
        "who were ",
        "who was ",
        "who is ",
        "who are ",
    ];
    let s = s.trim();
    for prefix in PREFIXES {
        if let Some(rest) = s.strip_prefix(prefix) {
            return rest;
        }
    }
    s
}

fn trim_tail_punct(s: &str) -> String {
    s.trim_end_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

pub fn understand_query_rule(query: &str) -> QueryStructure {
    let q = query.to_lowercase();

    // "keyword of X" patterns
    for (pattern, relation, direction) in OF_PATTERNS {
        if let Some(pos) = q.find(pattern) {
            let after = &q[pos + pattern.len()..];
            let entity_raw = trim_tail_punct(after);
            if entity_raw.is_empty() {
                continue;
            }
            let anchor_is_author = is_author_anchor(&entity_raw);
            return QueryStructure {
                intent: QueryIntent::FamilyRelation {
                    relation: relation.to_string(),
                    direction: direction.clone(),
                },
                target_entities: if anchor_is_author {
                    vec![]
                } else {
                    vec![entity_raw]
                },
                anchor_is_author,
            };
        }
    }

    // "X's keyword" possessive patterns
    for (pattern, relation, direction) in POSS_PATTERNS {
        if let Some(pos) = q.find(pattern) {
            let before = &q[..pos];
            let entity_raw = strip_question_words(before).trim().to_string();
            if entity_raw.is_empty() {
                continue;
            }
            let anchor_is_author = is_author_anchor(&entity_raw);
            return QueryStructure {
                intent: QueryIntent::FamilyRelation {
                    relation: relation.to_string(),
                    direction: direction.clone(),
                },
                target_entities: if anchor_is_author {
                    vec![]
                } else {
                    vec![entity_raw]
                },
                anchor_is_author,
            };
        }
    }

    // Entity description
    if q.starts_with("who was ")
        || q.starts_with("who is ")
        || q.starts_with("tell me about ")
        || q.starts_with("who were ")
        || q.starts_with("describe ")
    {
        return QueryStructure {
            intent: QueryIntent::EntityDescription,
            target_entities: vec![],
            anchor_is_author: false,
        };
    }

    // Org membership
    if q.contains("organisation")
        || q.contains("organization")
        || q.contains("movement")
        || q.contains(" league")
        || q.contains("convention")
        || q.contains("fellowship")
        || (q.starts_with("what was ") && !q.contains("district six") && !q.contains("kloof"))
    {
        return QueryStructure {
            intent: QueryIntent::OrgMembership,
            target_entities: vec![],
            anchor_is_author: false,
        };
    }

    // Temporal event — only match queries that are genuinely asking WHEN something happened,
    // not queries that merely mention a place or use "tell me about".
    // Spatial descriptors ("where was X", "what kind of place") and "tell me about X"
    // are entity-description queries, not temporal ones, and should not trigger sequence
    // diagram injection even when the entity has timeline events.
    // "before the forced removal(s)" is a temporal qualifier, not a question ABOUT the removals.
    // Only classify as TemporalEvent when "forced removal" is the subject of the question.
    let forced_removal_trigger = q.contains("forced removal")
        && !q.contains("before the forced")
        && !q.contains("prior to the forced");

    if q.starts_with("when did ")
        || q.starts_with("what happened ")
        || (q.contains("group areas")
            && (q.contains("when")
                || q.contains("how")
                || q.contains("affect")
                || q.contains("removal")))
        || forced_removal_trigger
    {
        return QueryStructure {
            intent: QueryIntent::TemporalEvent,
            target_entities: vec![],
            anchor_is_author: false,
        };
    }

    QueryStructure::unknown()
}

// ── LLM-based classifier ──────────────────────────────────────────────────────

pub async fn understand_query_llm(query: &str, url: &str, model: &str) -> QueryStructure {
    understand_query_llm_inner(query, url, model)
        .await
        .unwrap_or_else(QueryStructure::unknown)
}

async fn understand_query_llm_inner(query: &str, url: &str, model: &str) -> Option<QueryStructure> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .ok()?;

    let prompt = format!(
        "Classify this query about a historical memoir. Return ONLY a JSON object, no markdown.\n\
         Fields:\n\
           \"intent\": \"family_relation\" | \"entity_description\" | \"org_membership\" | \"temporal_event\" | \"unknown\"\n\
           \"relation\": for family_relation only — \"parent_of\"|\"child_of\"|\"spouse_of\"|\"sibling_of\"|\"grandparent_of\"|\"grandchild_of\" or null\n\
           \"direction\": for family_relation only — \"outgoing\"|\"incoming\"|\"both\" or null\n\
           \"target_entity\": named entity the query is about (full name as written in query), or null\n\
           \"anchor_is_author\": true if query is about \"the author\" or \"the narrator\", else false\n\n\
         Query: {query}"
    );

    let full_url = format!("{}/v1/chat/completions", url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.0,
        "max_tokens": 200,
        "format": "json",
    });

    let resp = client.post(&full_url).json(&body).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let v: serde_json::Value = resp.json().await.ok()?;
    let text = v["choices"][0]["message"]["content"]
        .as_str()?
        .trim()
        .to_string();
    parse_llm_query_structure(&text)
}

fn parse_llm_query_structure(text: &str) -> Option<QueryStructure> {
    let cleaned = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let start = cleaned.find('{')?;
    let end = cleaned.rfind('}').map(|i| i + 1)?;
    let v: serde_json::Value = serde_json::from_str(&cleaned[start..end]).ok()?;

    let intent_str = v["intent"].as_str().unwrap_or("unknown");
    let anchor_is_author = v["anchor_is_author"].as_bool().unwrap_or(false);
    let target_entity = v["target_entity"]
        .as_str()
        .filter(|s| !s.is_empty() && *s != "null")
        .map(|s| s.to_lowercase());

    let intent = match intent_str {
        "family_relation" => {
            let relation = v["relation"]
                .as_str()
                .filter(|s| !s.is_empty() && *s != "null")
                .unwrap_or("")
                .to_string();
            let direction = match v["direction"].as_str().unwrap_or("") {
                "outgoing" => RelDirection::Outgoing,
                "incoming" => RelDirection::Incoming,
                _ => RelDirection::Both,
            };
            if relation.is_empty() {
                QueryIntent::Unknown
            } else {
                QueryIntent::FamilyRelation {
                    relation,
                    direction,
                }
            }
        }
        "entity_description" => QueryIntent::EntityDescription,
        "org_membership" => QueryIntent::OrgMembership,
        "temporal_event" => QueryIntent::TemporalEvent,
        _ => QueryIntent::Unknown,
    };

    Some(QueryStructure {
        intent,
        target_entities: target_entity.into_iter().collect(),
        anchor_is_author,
    })
}

// ── Main entry point ──────────────────────────────────────────────────────────

pub async fn understand_query(
    query: &str,
    method: ClassifyMethod,
    url: Option<&str>,
    model: Option<&str>,
) -> QueryStructure {
    match method {
        ClassifyMethod::Rule => understand_query_rule(query),
        ClassifyMethod::Llm => {
            if let (Some(u), Some(m)) = (url, model) {
                understand_query_llm(query, u, m).await
            } else {
                understand_query_rule(query)
            }
        }
        ClassifyMethod::Hybrid => {
            let qs = understand_query_rule(query);
            if qs.intent == QueryIntent::Unknown {
                if let (Some(u), Some(m)) = (url, model) {
                    understand_query_llm(query, u, m).await
                } else {
                    qs
                }
            } else {
                qs
            }
        }
    }
}

// ── Entity resolution ─────────────────────────────────────────────────────────

/// Resolve the primary target entity from `QueryStructure` to a graph entity ID.
///
/// For `anchor_is_author` queries, returns the entity with an "author"/"narrator" alias.
/// Otherwise resolves the first `target_entities` string via multi-tier lookup:
///   1. Case-insensitive exact name match
///   2. Token intersection across both `alias_token_index` and `name_token_index`
pub fn resolve_target_entity(qs: &QueryStructure, graph: &GraphStore) -> Option<i64> {
    if qs.anchor_is_author {
        return graph
            .all_entities()
            .find(|e| {
                e.aliases.iter().any(|a| {
                    matches!(
                        a.to_lowercase().as_str(),
                        "author" | "the author" | "narrator" | "the narrator" | "the writer"
                    )
                })
            })
            .map(|e| e.id);
    }
    qs.target_entities
        .first()
        .and_then(|name| resolve_entity_by_name(name, graph))
}

fn resolve_entity_by_name(name_raw: &str, graph: &GraphStore) -> Option<i64> {
    // 1. Case-insensitive exact name match
    if let Some(e) = graph.find_by_name(name_raw) {
        return Some(e.id);
    }

    // 2. Token intersection: score each candidate by how many query tokens it matches.
    //    Include both raw ("j.m.h.") and trimmed ("jmh") forms so abbreviations resolve.
    let tokens: Vec<String> = name_raw
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

    scores
        .into_iter()
        .max_by_key(|(_, score)| *score)
        .map(|(id, _)| id)
}

/// Count outgoing graph relations that match the query intent's relation type.
/// Used by `retrieve_graph_anchored` to decide whether `Replace` mode should fire.
pub fn count_intent_facts(qs: &QueryStructure, entity_id: i64, graph: &GraphStore) -> usize {
    let rel = match &qs.intent {
        QueryIntent::FamilyRelation { relation, .. } => relation.as_str(),
        _ => return 0,
    };
    graph
        .outgoing_relations(entity_id)
        .unwrap_or_default()
        .into_iter()
        .filter(|(_, r, _, _)| r == rel)
        .count()
}
