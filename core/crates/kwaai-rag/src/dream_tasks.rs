//! Task-specific entity enrichment for Dream RAG.
//!
//! Each task targets a schema.org type and mines the entity's full evidence
//! (all chunk IDs stored in EntityNode.evidence) to extract type-appropriate
//! metadata via LLM, writing back richer descriptions and relations.
//!
//! All tasks share the EntityCompletion output so they slot directly into the
//! existing write-back path in dream.rs.

use serde::Deserialize;

use crate::dream::EntityCompletion;
use crate::graph::RELATION_TYPES;

// ── Task dispatch ─────────────────────────────────────────────────────────────

/// Which dream task to run for an entity, selected by schema.org type.
#[derive(Debug, Clone, PartialEq)]
pub enum DreamTaskKind {
    Biography,    // schema:Person
    Geography,    // schema:Place
    OrgProfile,   // schema:Organization
    EventProfile, // schema:Event
    ConceptDef,   // schema:DefinedTerm — historical/social concepts
    WorkProfile,  // schema:CreativeWork / schema:Product — books, films, objects
    General,      // Unknown / Thing — falls back to general completion
}

pub fn task_for_schema_type(schema_type: Option<&str>) -> DreamTaskKind {
    match schema_type {
        Some("schema:Person") => DreamTaskKind::Biography,
        Some("schema:Place") => DreamTaskKind::Geography,
        Some("schema:Organization") => DreamTaskKind::OrgProfile,
        Some("schema:Event") => DreamTaskKind::EventProfile,
        Some("schema:DefinedTerm") => DreamTaskKind::ConceptDef,
        Some("schema:CreativeWork") | Some("schema:Product") => DreamTaskKind::WorkProfile,
        _ => DreamTaskKind::General,
    }
}

/// Dispatch to the right task. General delegates to dream::complete_entity.
pub async fn run_task(
    kind: DreamTaskKind,
    eid: i64,
    name: &str,
    entity_type: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    match kind {
        DreamTaskKind::Biography => {
            run_biography_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::Geography => {
            run_geography_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::OrgProfile => {
            run_org_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::EventProfile => {
            run_event_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::ConceptDef => {
            run_concept_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::WorkProfile => {
            run_work_task(eid, name, current_description, evidence_text, url, model).await
        }
        DreamTaskKind::General => {
            crate::dream::complete_entity(
                eid,
                name,
                entity_type,
                current_description,
                evidence_text,
                url,
                model,
            )
            .await
        }
    }
}

// ── Evidence trimming ─────────────────────────────────────────────────────────

/// Cap evidence at ~8 000 chars (~2 000 tokens), breaking at a sentence boundary.
pub fn trim_evidence(text: &str) -> &str {
    const LIMIT: usize = 8_000;
    if text.len() <= LIMIT {
        return text;
    }
    text[..LIMIT]
        .rfind(". ")
        .map(|p| &text[..p + 2])
        .unwrap_or(&text[..LIMIT])
}

// ── Shared LLM helpers ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TaskPayload {
    #[serde(default)]
    description: String,
    #[serde(default)]
    relations: Vec<TaskRelation>,
}

#[derive(Debug, Deserialize)]
struct TaskRelation {
    #[serde(rename = "type")]
    relation_type: String,
    target: String,
}

async fn call_llm(prompt: &str, url: &str, model: &str) -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let full_url = format!("{}/v1/chat/completions", url.trim_end_matches('/'));
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.1,
        "max_tokens": 600,
    });

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

    Some(
        v["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("{}")
            .to_string(),
    )
}

fn parse_result(raw: &str, eid: i64, current_desc: &str) -> EntityCompletion {
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let payload: TaskPayload = match serde_json::from_str(cleaned) {
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

    let description = if payload.description.len() > current_desc.len() + 20 {
        Some(payload.description)
    } else {
        None
    };

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
        schema_type: None, // type already resolved; don't overwrite
        description,
        relations,
    }
}

fn empty(eid: i64) -> EntityCompletion {
    EntityCompletion {
        entity_id: eid,
        schema_type: None,
        description: None,
        relations: vec![],
    }
}

// ── Task implementations ──────────────────────────────────────────────────────

pub async fn run_biography_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);

    // For very thin evidence (short passing mention), allow the LLM to use
    // widely-known facts to fill in basic biographical context (nationality,
    // primary role, major association).  Private individuals should stay text-only.
    let thin = text.len() < 300;
    let knowledge_rule = if thin {
        "If this is a well-known public figure (politician, general, author, etc.) you may \
         supplement sparse text with widely-known facts. For private individuals, use only \
         what the text provides."
    } else {
        "Only include relations that are explicitly stated in the text. Do not invent facts."
    };

    let prompt = format!(
        "You are building a biography for a person named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text (all passages mentioning this person):\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence biography>\",\
           \"relations\":[\
             {{\"type\":\"located_in\",\"target\":\"<birth place, home city, or country>\"}},\
             {{\"type\":\"spouse_of\",\"target\":\"<spouse name>\"}},\
             {{\"type\":\"child_of\",\"target\":\"<parent name>\"}},\
             {{\"type\":\"parent_of\",\"target\":\"<child name>\"}},\
             {{\"type\":\"sibling_of\",\"target\":\"<sibling name>\"}},\
             {{\"type\":\"belongs_to\",\"target\":\"<organisation, political party, or group>\"}},\
             {{\"type\":\"works_at\",\"target\":\"<employer, military branch, or institution>\"}},\
             {{\"type\":\"associated_with\",\"target\":\"<key person, event, or movement>\"}}\
           ]}}\n\n\
         Rules:\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}

pub async fn run_geography_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let thin = text.len() < 300;
    let knowledge_rule = if thin {
        "If this is a well-known city, country, region, or landmark you may supplement \
         sparse text with widely-known geographic facts (country, continent, key feature). \
         For obscure or fictional places, use only what the text provides."
    } else {
        "Only include relations where the target is explicitly named in the text."
    };
    let prompt = format!(
        "You are describing a place named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence description of this place>\",\
           \"relations\":[\
             {{\"type\":\"located_in\",\"target\":\"<city, region, or country>\"}},\
             {{\"type\":\"part_of\",\"target\":\"<larger area or district>\"}},\
             {{\"type\":\"contains\",\"target\":\"<named sub-area or landmark>\"}}\
           ]}}\n\n\
         Rules:\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}

pub async fn run_org_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let thin = text.len() < 300;
    let knowledge_rule = if thin {
        "If this is a well-known public organisation (government body, political party, \
         major institution) you may supplement sparse text with widely-known facts about \
         its purpose and location. For obscure or private organisations, use only what the \
         text provides."
    } else {
        "Only include relations where the target is explicitly named in the text."
    };
    let prompt = format!(
        "You are profiling an organisation named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence profile of this organisation>\",\
           \"relations\":[\
             {{\"type\":\"associated_with\",\"target\":\"<key person associated with it>\"}},\
             {{\"type\":\"founded\",\"target\":\"<entity or institution this organisation founded>\"}},\
             {{\"type\":\"located_in\",\"target\":\"<headquarters location>\"}},\
             {{\"type\":\"part_of\",\"target\":\"<parent organisation>\"}},\
             {{\"type\":\"contains\",\"target\":\"<named subsidiary or branch>\"}},\
             {{\"type\":\"belongs_to\",\"target\":\"<federation or body it belongs to>\"}}\
           ]}}\n\n\
         Rules:\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}

pub async fn run_event_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let prompt = format!(
        "You are describing an event named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence description of this event>\",\
           \"relations\":[\
             {{\"type\":\"located_in\",\"target\":\"<location where event took place>\"}},\
             {{\"type\":\"associated_with\",\"target\":\"<key participant name>\"}},\
             {{\"type\":\"related_to\",\"target\":\"<related organisation or event>\"}},\
             {{\"type\":\"occurred_on\",\"target\":\"<date or period>\"}}\
           ]}}\n\n\
         Only include relations where the target is explicitly named in the text."
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}

pub async fn run_concept_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let thin = text.len() < 400;
    let knowledge_rule = if thin {
        "If this is a well-known historical, legal, or social concept you may supplement \
         sparse text with widely-known facts about its meaning or origin. \
         For obscure or highly context-specific terms, use only what the text provides."
    } else {
        "Only include relations where the target is explicitly named in the text."
    };
    let prompt = format!(
        "You are describing the historical or social concept \"{name}\" as used in source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence explanation of what this concept means in the context of the text>\",\
           \"relations\":[\
             {{\"type\":\"related_to\",\"target\":\"<related concept, law, or policy>\"}},\
             {{\"type\":\"defined_by\",\"target\":\"<organisation or document that defines it>\"}},\
             {{\"type\":\"subtype_of\",\"target\":\"<broader concept>\"}}\
           ]}}\n\n\
         Rules:\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}

pub async fn run_work_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let prompt = format!(
        "You are describing \"{name}\" — a creative work, publication, or physical object — from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<2-3 sentence description of what this is and how it appears in the text>\",\
           \"relations\":[\
             {{\"type\":\"associated_with\",\"target\":\"<person or organisation associated with it>\"}},\
             {{\"type\":\"related_to\",\"target\":\"<related item or event>\"}},\
             {{\"type\":\"described_in\",\"target\":\"<document or source that describes it>\"}},\
             {{\"type\":\"cites\",\"target\":\"<another work or entity it references>\"}},\
             {{\"type\":\"located_in\",\"target\":\"<place where it is found or used>\"}}\
           ]}}\n\n\
         Only include relations where the target is explicitly named in the text."
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}
