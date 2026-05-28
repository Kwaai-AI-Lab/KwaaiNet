//! Task-specific entity enrichment for Dream RAG.
//!
//! Each task targets a schema.org type and mines the entity's full evidence
//! (all chunk IDs stored in EntityNode.evidence) to extract type-appropriate
//! metadata via LLM, writing back richer descriptions and relations.
//!
//! All tasks share the EntityCompletion output so they slot directly into the
//! existing write-back path in dream.rs.

use std::collections::HashMap;

use serde::Deserialize;

use crate::dream::EntityCompletion;
use crate::graph::{FieldValue, RELATION_TYPES};

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
    evidence_chunks: &[i64],
    no_relations: bool,
) -> EntityCompletion {
    match kind {
        DreamTaskKind::Biography => {
            run_biography_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                mention_count,
                chunk_count,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
        }
        DreamTaskKind::Geography => {
            run_geography_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
        }
        DreamTaskKind::OrgProfile => {
            run_org_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                mention_count,
                chunk_count,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
        }
        DreamTaskKind::EventProfile => {
            run_event_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
        }
        DreamTaskKind::ConceptDef => {
            run_concept_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
        }
        DreamTaskKind::WorkProfile => {
            run_work_task(
                eid,
                name,
                current_description,
                evidence_text,
                url,
                model,
                document_titles,
                evidence_chunks,
                no_relations,
            )
            .await
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
                no_relations,
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
    /// Legacy prose description returned by General task and fallback paths.
    #[serde(default)]
    description: String,
    /// Structured fields returned by Biography/Geography/OrgProfile tasks.
    #[serde(default)]
    fields: HashMap<String, String>,
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

/// Parse an LLM task response into an `EntityCompletion`.
/// `evidence_chunks` are the entity's evidence chunk IDs — attached to every
/// new field value so provenance is tracked from the dream cycle.
fn parse_result(
    raw: &str,
    eid: i64,
    current_desc: &str,
    evidence_chunks: &[i64],
) -> EntityCompletion {
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
                fields: HashMap::new(),
            }
        }
    };

    // Convert string fields to FieldValues, seeding with all evidence chunks.
    let fields: HashMap<String, FieldValue> = payload
        .fields
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| {
            let mut fv = FieldValue::new(v, *evidence_chunks.first().unwrap_or(&0));
            for &cid in evidence_chunks.iter().skip(1) {
                fv.add_evidence(cid);
            }
            (k, fv)
        })
        .collect();

    // Keep legacy prose description for fallback / General task path.
    let description = if !payload.description.is_empty() {
        let new_tier = summary_tier(&payload.description);
        let old_tier = summary_tier(current_desc);
        if new_tier > old_tier
            || (new_tier == old_tier && payload.description.len() > current_desc.len() + 20)
        {
            Some(payload.description)
        } else {
            None
        }
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
        schema_type: None,
        description,
        relations,
        fields,
    }
}

fn empty(eid: i64) -> EntityCompletion {
    EntityCompletion {
        entity_id: eid,
        schema_type: None,
        description: None,
        relations: vec![],
        fields: HashMap::new(),
    }
}

// ── Task implementations ──────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
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
    evidence_chunks: &[i64],
    no_relations: bool,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);

    let thin = text.len() < 600;
    let knowledge_rule = if thin {
        "If this is a well-known public figure (politician, general, author, etc.) you may \
         supplement sparse text with widely-known facts. For private individuals, use only \
         what the text provides."
    } else {
        "Use only facts clearly supported by the source text. Do not invent facts."
    };
    let _ = mention_count;
    let _ = chunk_count;

    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };

    let relations_section = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"located_in\",\"target\":\"<birth place, home city, or country>\"},\
           {\"type\":\"belongs_to\",\"target\":\"<organisation, political party, or group>\"},\
           {\"type\":\"works_at\",\"target\":\"<employer or institution>\"},\
           {\"type\":\"associated_with\",\"target\":\"<key person, event, or movement>\"}\
         ]"
    };
    let relation_rules = if no_relations {
        "- relations must be an empty array []"
    } else {
        "- Omit relations whose target is empty or vague\n\
         - Only include relations clearly stated in the text"
    };

    let prompt = format!(
        "You are building a biography for a person named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema (omit any field whose value is not in the text):\n\
         {{\"fields\":{{\
           \"birthDate\":\"date of birth\",\
           \"birthPlace\":\"place of birth\",\
           \"deathDate\":\"date of death if deceased\",\
           \"nationality\":\"nationality or cultural identity\",\
           \"occupation\":\"profession or main occupation\",\
           \"affiliation\":\"organization they belong or belonged to\"\
         }},\
         {relations_section}}}\n\n\
         Rules:\n\
         - Only include fields whose values are determinable from the text\n\
         - {relation_rules}\n\
         - {knowledge_rule}{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run_geography_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
    document_titles: &[String],
    evidence_chunks: &[i64],
    no_relations: bool,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
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
    let geo_relations = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"located_in\",\"target\":\"<city, region, or country>\"},\
           {\"type\":\"part_of\",\"target\":\"<larger area or district>\"},\
           {\"type\":\"contains\",\"target\":\"<named sub-area or landmark>\"}\
         ]"
    };
    let prompt = format!(
        "You are describing a place named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema (omit any field whose value is not in the text):\n\
         {{\"fields\":{{\
           \"addressLocality\":\"city, district or suburb\",\
           \"addressRegion\":\"province or region\",\
           \"addressCountry\":\"country\",\
           \"locationType\":\"type of place (district, city, country, neighbourhood, etc.)\",\
           \"historicalNote\":\"historical or cultural significance\"\
         }},\
         {geo_relations}}}\n\n\
         Rules:\n\
         - Only include fields whose values are determinable from the text\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}

#[allow(clippy::too_many_arguments)]
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
    evidence_chunks: &[i64],
    no_relations: bool,
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
    let _ = mention_count;
    let _ = chunk_count;

    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };

    let org_relations = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"associated_with\",\"target\":\"<key person associated with it>\"},\
           {\"type\":\"founded\",\"target\":\"<entity this organisation founded>\"},\
           {\"type\":\"located_in\",\"target\":\"<headquarters location>\"},\
           {\"type\":\"part_of\",\"target\":\"<parent organisation>\"},\
           {\"type\":\"contains\",\"target\":\"<named subsidiary or branch>\"},\
           {\"type\":\"belongs_to\",\"target\":\"<federation or body it belongs to>\"}\
         ]"
    };
    let prompt = format!(
        "You are profiling an organisation named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema (omit any field whose value is not in the text):\n\
         {{\"fields\":{{\
           \"foundingDate\":\"year or period when founded\",\
           \"dissolutionDate\":\"year or period when dissolved, if applicable\",\
           \"location\":\"city or country of headquarters or main office\",\
           \"founder\":\"founder name\",\
           \"orgType\":\"type of organization (school, mosque, political party, government body, etc.)\"\
         }},\
         {org_relations}}}\n\n\
         Rules:\n\
         - Only include fields whose values are determinable from the text\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run_event_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
    document_titles: &[String],
    evidence_chunks: &[i64],
    no_relations: bool,
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
    let event_relations = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"located_in\",\"target\":\"<location where event took place>\"},\
           {\"type\":\"associated_with\",\"target\":\"<key participant or related event>\"},\
           {\"type\":\"related_to\",\"target\":\"<related organisation or event>\"},\
           {\"type\":\"occurred_on\",\"target\":\"<date or period>\"}\
         ]"
    };
    let prompt = format!(
        "You are describing an event named \"{name}\" from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what happened and when or where this event occurred> <sentence 2: its significance, outcome, or key participants>\",\
           {event_relations}}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run_concept_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
    document_titles: &[String],
    evidence_chunks: &[i64],
    no_relations: bool,
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
    let concept_relations = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"related_to\",\"target\":\"<related person, organisation, event, concept, law, or policy>\"},\
           {\"type\":\"defined_by\",\"target\":\"<organisation or document that defines or governs it>\"},\
           {\"type\":\"subtype_of\",\"target\":\"<broader concept or category>\"}\
         ]"
    };
    let prompt = format!(
        "You are describing the historical or social concept \"{name}\" as used in source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what this concept means or refers to in general terms> <sentence 2: how it is used or significant in the context of the source text>\",\
           {concept_relations}}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Omit any relation whose target is empty or vague\n\
         - {knowledge_rule}{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run_work_task(
    eid: i64,
    name: &str,
    current_description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
    document_titles: &[String],
    evidence_chunks: &[i64],
    no_relations: bool,
) -> EntityCompletion {
    let text = trim_evidence(evidence_text);
    let doc_rule = doc_exclusion_rule(document_titles);
    let doc_rule_line = if doc_rule.is_empty() {
        String::new()
    } else {
        format!("\n         - {doc_rule}")
    };
    let work_relations = if no_relations {
        "\"relations\":[]"
    } else {
        "\"relations\":[\
           {\"type\":\"associated_with\",\"target\":\"<person or organisation associated with it>\"},\
           {\"type\":\"related_to\",\"target\":\"<related item or event>\"},\
           {\"type\":\"described_in\",\"target\":\"<document or source that describes it>\"},\
           {\"type\":\"cites\",\"target\":\"<another work or entity it references>\"},\
           {\"type\":\"located_in\",\"target\":\"<place where it is found or used>\"}\
         ]"
    };
    let prompt = format!(
        "You are describing \"{name}\" — a creative work, publication, or physical object — from source text.\n\
         Return ONLY valid JSON — no markdown, no explanation.\n\n\
         Source text:\n---\n{text}\n---\n\n\
         JSON schema:\n\
         {{\"description\":\"<sentence 1: what this work or object is — its type and creator or origin> <sentence 2: its significance or how it appears in the source text>\",\
           {work_relations}}}\n\n\
         Rules:\n\
         - description MUST be at least 2 full sentences and at least 150 characters\n\
         - Only include relations where the target is explicitly named in the text{doc_rule_line}"
    );

    match call_llm(&prompt, url, model).await {
        Some(raw) => parse_result(&raw, eid, current_description, evidence_chunks),
        None => empty(eid),
    }
}
