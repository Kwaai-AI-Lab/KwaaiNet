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

/// Build a prompt rule string that instructs the LLM not to use source document
/// titles as the target of spatial/employment relations. Returns an empty string
/// when no titles are stored (making the call a no-op in the prompt).
pub fn doc_exclusion_rule(document_titles: &[String]) -> String {
    if document_titles.is_empty() {
        return String::new();
    }
    let titles = document_titles
        .iter()
        .map(|t| format!("\"{}\"", t))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "SOURCE DOCUMENT TITLES: {titles} — these are the titles of the works being analysed. \
         They are CreativeWork entities. Do NOT use them as the target of located_in, works_at, \
         part_of, or contains relations. When describing where events took place, use the actual \
         place name (e.g. 'District Six', not 'District Six - Lest We Forget')."
    )
}

/// Dispatch to the right task. General delegates to dream::complete_entity.
#[allow(clippy::too_many_arguments)]
pub async fn run_task(
    kind: DreamTaskKind,
    eid: i64,
    name: &str,
    entity_type: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
    mention_count: u32,
    chunk_count: usize,
    document_titles: &[String],
) -> EntityCompletion {
    match kind {
        DreamTaskKind::Biography => {
            run_biography_task(eid, name, current_description, evidence_text, url, model, mention_count, chunk_count, document_titles).await
        }
        DreamTaskKind::Geography => {
            run_geography_task(eid, name, current_description, evidence_text, url, model, document_titles).await
        }
        DreamTaskKind::OrgProfile => {
            run_org_task(eid, name, current_description, evidence_text, url, model, mention_count, chunk_count, document_titles).await
        }
        DreamTaskKind::EventProfile => {
            run_event_task(eid, name, current_description, evidence_text, url, model, document_titles).await
        }
        DreamTaskKind::ConceptDef => {
            run_concept_task(eid, name, current_description, evidence_text, url, model, document_titles).await
        }
        DreamTaskKind::WorkProfile => {
            run_work_task(eid, name, current_description, evidence_text, url, model, document_titles).await
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
        "temperature": 0.25,
        "max_tokens": 700,
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

/// Quick summary score matching scorer.rs tiers (no import needed).
pub fn summary_tier(desc: &str) -> u8 {
    if desc.is_empty() {
        0
    } else if desc.len() < 50 {
        1
    } else if desc.len() < 150 {
        2
    } else {
        let sentences = desc
            .chars()
            .filter(|c| matches!(c, '.' | '?' | '!'))
            .count();
        if sentences >= 2 {
            4 // 1.0
        } else {
            3 // 0.8
        }
    }
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

    // Accept the new description if it improves the summary score tier OR is
    // meaningfully longer (same tier but more content).  The old length+20
    // gate rejected tier-jumping rewrites where the new text was shorter but
    // far richer (e.g. a model that writes a 120-char description over an
    // existing 105-char one would be blocked; now it isn't).
    let description = {
        let new_tier = summary_tier(&payload.description);
        let old_tier = summary_tier(current_desc);
        if new_tier > old_tier
            || (new_tier == old_tier && payload.description.len() > current_desc.len() + 20)
        {
            Some(payload.description)
        } else {
            None
        }
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
    mention_count: u32,
    chunk_count: usize,
    document_titles: &[String],
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);

    // For very thin evidence (short passing mention), allow the LLM to use
    // widely-known facts to fill in basic biographical context (nationality,
    // primary role, major association).  Private individuals should stay text-only.
    let thin = text.len() < 600;
    let knowledge_rule = if thin {
        "If this is a well-known public figure (politician, general, author, etc.) you may \
         supplement sparse text with widely-known facts. For private individuals, use only \
         what the text provides."
    } else {
        "Include relations clearly supported by the text — explicit statements AND strong \
         contextual associations (e.g. a person living in a city, working for an organisation, \
         opposing a movement) that are clearly evidenced in the text. Do not invent facts."
    };

    let is_major = mention_count > 5 || chunk_count > 5;
    let description_rule = if is_major {
        format!(
            "DESCRIPTION: Write a full biographical paragraph of at least 4 sentences and \
             at least 300 characters. Cover: who they are, their role, key events they were \
             involved in, and their significance to the D6 community. Do NOT start with '{name}'."
        )
    } else {
        format!(
            "DESCRIPTION: MUST be at least 2 full sentences and at least 150 characters. \
             Do NOT start with '{name}'."
        )
    };

    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };

    let prompt = format!(
        "You are building a biography for a person named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text (all passages mentioning this person):\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: who this person is — nationality, era, and primary role or profession> <sentence 2: their key contribution, association, or significance in the context of the text>\",\
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
         - {description_rule}\n\
         - RELATION DIRECTION: 'parent_of' means the source IS THE PARENT. If {name} is a child of someone, write: that_person parent_of {name}. NEVER write {name} parent_of their own parent.\n\
         - spouse_of means LEGALLY MARRIED. ONLY include this relation if the text EXPLICITLY states marriage (e.g. 'married', 'wife', 'husband'). Do NOT infer marriage from association, friendship, or working together.\n\
         - If in doubt about a familial relation, use 'associated_with' instead.\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
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
    document_titles: &[String],
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    // Raise threshold: evidence is concatenated chunks so even well-known places
    // can have 1000+ chars of text that is all brief mentions ("met in Bloemfontein").
    let thin = text.len() < 800;
    let knowledge_rule = if thin {
        "You MAY supplement sparse source text with widely-known geographic facts \
         (country, continent, administrative region, key historical or physical feature). \
         For obscure, fictional, or highly local places, use only what the text provides."
    } else {
        "Use the source text as your primary reference for geographic details."
    };
    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };
    let prompt = format!(
        "You are describing a place named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what type of place it is and where located> <sentence 2: its historical, cultural, or political significance>\",\
           \"relations\":[\
             {{\"type\":\"located_in\",\"target\":\"<city, region, or country>\"}},\
             {{\"type\":\"part_of\",\"target\":\"<larger area or district>\"}},\
             {{\"type\":\"contains\",\"target\":\"<named sub-area or landmark>\"}}\
           ]}}\n\n\
         Rules:\n\
         - Write BOTH sentences; description must be at least 150 characters total\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
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
    mention_count: u32,
    chunk_count: usize,
    document_titles: &[String],
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let thin = text.len() < 600;
    let knowledge_rule = if thin {
        "If this is a well-known public organisation (government body, political party, \
         major institution) you may supplement sparse text with widely-known facts about \
         its purpose and location. For obscure or private organisations, use only what the \
         text provides."
    } else {
        "Only include relations where the target is explicitly named in the text."
    };

    let is_major = mention_count > 5 || chunk_count > 5;
    let description_rule = if is_major {
        format!(
            "DESCRIPTION: Write a full organisational profile of at least 4 sentences and \
             at least 300 characters. Cover: what it is, where it operates, its key activities \
             or purpose, and its significance in the context of the text. Do NOT start with '{name}'."
        )
    } else {
        format!(
            "DESCRIPTION: MUST be at least 2 full sentences and at least 150 characters. \
             Do NOT start with '{name}'."
        )
    };

    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };

    let prompt = format!(
        "You are profiling an organisation named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what type of organisation this is and where it operates> <sentence 2: its purpose, key activities, or role in the context of the text>\",\
           \"relations\":[\
             {{\"type\":\"associated_with\",\"target\":\"<key person associated with it>\"}},\
             {{\"type\":\"founded\",\"target\":\"<entity or institution this organisation founded>\"}},\
             {{\"type\":\"located_in\",\"target\":\"<headquarters location>\"}},\
             {{\"type\":\"part_of\",\"target\":\"<parent organisation>\"}},\
             {{\"type\":\"contains\",\"target\":\"<named subsidiary or branch>\"}},\
             {{\"type\":\"belongs_to\",\"target\":\"<federation or body it belongs to>\"}}\
           ]}}\n\n\
         Rules:\n\
         - {description_rule}\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
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
    document_titles: &[String],
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let thin = text.len() < 600;
    let knowledge_rule = if thin {
        "If this is a well-known historical event (war, political act, battle, legislation) \
         you may supplement sparse text with widely-known facts about its participants, \
         location, or broader context. For obscure or local events, use only what the text provides."
    } else {
        "Include relations clearly supported by the text — explicit or strongly implied by context."
    };
    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };
    let prompt = format!(
        "You are describing an event named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what happened and when or where this event occurred> <sentence 2: its significance, outcome, or key participants>\",\
           \"relations\":[\
             {{\"type\":\"located_in\",\"target\":\"<location where event took place>\"}},\
             {{\"type\":\"associated_with\",\"target\":\"<key participant or related event>\"}},\
             {{\"type\":\"related_to\",\"target\":\"<related organisation or event>\"}},\
             {{\"type\":\"occurred_on\",\"target\":\"<date or period>\"}}\
           ]}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
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
    document_titles: &[String],
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
    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };
    let prompt = format!(
        "You are describing the historical or social concept \"{name}\" as used in source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what this concept means or refers to in general terms> <sentence 2: how it is used or significant in the context of the source text>\",\
           \"relations\":[\
             {{\"type\":\"related_to\",\"target\":\"<related person, organisation, event, concept, law, or policy>\"}},\
             {{\"type\":\"defined_by\",\"target\":\"<organisation or document that defines or governs it>\"}},\
             {{\"type\":\"subtype_of\",\"target\":\"<broader concept or category>\"}}\
           ]}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
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
    document_titles: &[String],
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };
    let prompt = format!(
        "You are describing \"{name}\" — a creative work, publication, or physical object — from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what this work or object is — its type and creator or origin> <sentence 2: its significance or how it appears in the source text>\",\
           \"relations\":[\
             {{\"type\":\"associated_with\",\"target\":\"<person or organisation associated with it>\"}},\
             {{\"type\":\"related_to\",\"target\":\"<related item or event>\"}},\
             {{\"type\":\"described_in\",\"target\":\"<document or source that describes it>\"}},\
             {{\"type\":\"cites\",\"target\":\"<another work or entity it references>\"}},\
             {{\"type\":\"located_in\",\"target\":\"<place where it is found or used>\"}}\
           ]}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Only include relations where the target is explicitly named in the text{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description),
        None => empty(eid),
    }
}
