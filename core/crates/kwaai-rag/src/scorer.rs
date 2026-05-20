//! Dream RAG — Phase 1: Knowledge graph completeness scoring.
//!
//! Three independent pillars per entity:
//!   1. Type   — is the entity mapped to a specific schema.org type?
//!   2. Summary — is the description substantive (derived from source text)?
//!   3. Relationship — are the expected relations for this type present?
//!
//! The overall score is the unweighted average of the three pillars (0.0–1.0).

use crate::graph::{EntityNode, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Schema.org type mapping ───────────────────────────────────────────────────

/// Maps our 15 extraction types to canonical schema.org types.
/// "Unknown" maps to None — entity must be reclassified by the dream loop.
pub fn schema_type_for(entity_type: &str) -> Option<&'static str> {
    match entity_type {
        "Person" => Some("schema:Person"),
        "Organization" => Some("schema:Organization"),
        "Location" => Some("schema:Place"),
        "Event" => Some("schema:Event"),
        "Product" => Some("schema:Product"),
        "Document" => Some("schema:CreativeWork"),
        "Technology" => Some("schema:SoftwareApplication"),
        "Concept" => Some("schema:DefinedTerm"),
        "Method" => Some("schema:HowTo"),
        "Claim" => Some("schema:Statement"),
        "Role" => Some("schema:Role"),
        "Quantity" => Some("schema:QuantitativeValue"),
        "Date" => Some("schema:Date"),
        // Vague types — valid but uninformative
        "Topic" => Some("schema:Thing"),
        _ => None, // Unknown + anything unrecognised
    }
}

/// Expected relation categories for each schema.org type.
/// At least one relation from each inner slice should be present for full score.
/// Returns empty slice for types with no defined expectations (neutral — not penalised).
pub fn expected_relation_groups(schema_type: &str) -> &'static [&'static [&'static str]] {
    match schema_type {
        "schema:Person" => &[
            &[
                "parent_of",
                "child_of",
                "spouse_of",
                "sibling_of",
                "half_sibling_of",
                "grandparent_of",
                "grandchild_of",
                "uncle_of",
                "aunt_of",
                "niece_of",
                "nephew_of",
                "cousin_of",
                "foster_parent_of",
                "foster_child_of",
            ],
            &["works_at", "belongs_to", "founded", "manages"],
            &["located_in", "associated_with"],
        ],
        "schema:Organization" => &[&["located_in"], &["founded", "part_of", "contains"]],
        "schema:Place" => &[&["located_in", "contains", "part_of"]],
        "schema:Event" => &[
            &["occurred_on", "started", "ended"],
            &["located_in", "associated_with", "related_to"],
        ],
        "schema:Product" => &[&["instance_of", "associated_with", "related_to"]],
        "schema:CreativeWork" => &[&["described_in", "cites", "related_to", "defined_by"]],
        "schema:SoftwareApplication" => &[&["implements", "instance_of", "associated_with"]],
        "schema:DefinedTerm" => &[&["defined_by", "subtype_of", "related_to"]],
        "schema:HowTo" => &[&["related_to", "implements", "associated_with"]],
        _ => &[], // schema:Thing, schema:Role, schema:Statement, schema:QuantitativeValue, schema:Date
    }
}

// ── Score structs ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityScore {
    pub entity_id: i64,
    pub name: String,
    pub entity_type: String,
    pub schema_type: Option<String>,

    /// Pillar 1 — 0.0 (Unknown/no schema type) to 1.0 (specific schema.org type)
    pub type_score: f32,
    /// Pillar 2 — 0.0 (empty) to 1.0 (≥ 2 sentences and ≥ 150 chars)
    pub summary_score: f32,
    /// Pillar 3 — fraction of expected relation groups that have ≥ 1 match
    pub relation_score: f32,

    pub overall: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHealthReport {
    pub overall: f32,
    pub entity_count: usize,
    pub relation_count: usize,
    /// Distribution of resolved schema.org types across all entities.
    pub by_schema_type: HashMap<String, usize>,
    /// Entities still typed as Unknown with no schema_type set.
    pub unknown_count: usize,
    /// Per-entity scores, sorted worst-first.
    pub entity_scores: Vec<EntityScore>,
    /// Human-readable warnings for the worst offenders.
    pub top_issues: Vec<String>,
    pub generated_at: String,
}

// ── Scoring functions ─────────────────────────────────────────────────────────

pub fn score_entity(node: &EntityNode, neighbor_relation_types: &[String]) -> EntityScore {
    // Resolve schema.org type: prefer the stored schema_type, fall back to map.
    let resolved = node
        .schema_type
        .as_deref()
        .or_else(|| schema_type_for(&node.entity_type));

    // Pillar 1: type
    let type_score: f32 = match resolved {
        None => 0.0,                 // Unknown — must be completed
        Some("schema:Thing") => 0.4, // vague but valid
        Some(_) => 1.0,
    };

    // Pillar 2: summary
    let desc = node.description.trim();
    let summary_score: f32 = if desc.is_empty() {
        0.0
    } else if desc.len() < 50 {
        0.3
    } else if desc.len() < 150 {
        0.6
    } else {
        // count sentence-ending punctuation as a proxy for sentence count
        let sentences = desc
            .chars()
            .filter(|c| matches!(c, '.' | '?' | '!'))
            .count();
        if sentences >= 2 {
            1.0
        } else {
            0.8
        }
    };

    // Pillar 3: relationships
    // Peripheral entities (mention_count ≤ 2) are only expected to satisfy
    // 1 relation group instead of all groups; they're supporting cast and the
    // source text rarely gives enough context for full biographical data.
    let relation_score: f32 = match resolved {
        None | Some("schema:Thing") => {
            // No expectations defined — neutral, not penalised
            0.5
        }
        Some(st) => {
            let groups = expected_relation_groups(st);
            if groups.is_empty() {
                0.5
            } else {
                let matched = groups
                    .iter()
                    .filter(|group| {
                        group.iter().any(|expected| {
                            neighbor_relation_types
                                .iter()
                                .any(|r| r.as_str() == *expected)
                        })
                    })
                    .count();
                // Scale expectations down for peripheral entities so one
                // matched group is enough for a full score.
                let effective_required = if node.mention_count <= 2 {
                    1usize.min(groups.len())
                } else {
                    groups.len()
                };
                (matched as f32 / effective_required as f32).min(1.0)
            }
        }
    };

    let overall = (type_score + summary_score + relation_score) / 3.0;

    EntityScore {
        entity_id: node.id,
        name: node.name.clone(),
        entity_type: node.entity_type.clone(),
        schema_type: resolved.map(|s| s.to_string()),
        type_score,
        summary_score,
        relation_score,
        overall,
    }
}

pub fn score_graph(store: &GraphStore) -> GraphHealthReport {
    let mut entity_scores: Vec<EntityScore> = store
        .all_entities()
        .map(|node| {
            let neighbor_rels: Vec<String> = store
                .neighbors_of(node.id)
                .into_iter()
                .map(|(_, rel, _)| rel)
                .collect();
            score_entity(node, &neighbor_rels)
        })
        .collect();

    // Sort worst-first for the report.
    entity_scores.sort_by(|a, b| a.overall.partial_cmp(&b.overall).unwrap());

    let overall = if entity_scores.is_empty() {
        0.0
    } else {
        entity_scores.iter().map(|s| s.overall).sum::<f32>() / entity_scores.len() as f32
    };

    let unknown_count = entity_scores
        .iter()
        .filter(|s| s.schema_type.is_none())
        .count();

    let mut by_schema_type: HashMap<String, usize> = HashMap::new();
    for s in &entity_scores {
        let key = s
            .schema_type
            .clone()
            .unwrap_or_else(|| "Unknown".to_string());
        *by_schema_type.entry(key).or_default() += 1;
    }

    let mut top_issues: Vec<String> = Vec::new();
    for s in entity_scores.iter().take(10) {
        if s.type_score == 0.0 {
            top_issues.push(format!(
                "'{}' — type Unknown, needs reclassification",
                s.name
            ));
        } else if s.summary_score < 0.3 {
            top_issues.push(format!(
                "'{}' [{}] — missing or very thin description",
                s.name, s.entity_type
            ));
        } else if s.relation_score < 0.34 {
            top_issues.push(format!(
                "'{}' [{}] — no expected relations found",
                s.name, s.entity_type
            ));
        } else {
            top_issues.push(format!(
                "'{}' [{}] — overall {:.0}% (type={:.0}% summary={:.0}% relations={:.0}%)",
                s.name,
                s.entity_type,
                s.overall * 100.0,
                s.type_score * 100.0,
                s.summary_score * 100.0,
                s.relation_score * 100.0
            ));
        }
    }

    let relation_count = store.relation_count();

    GraphHealthReport {
        overall,
        entity_count: entity_scores.len(),
        relation_count,
        by_schema_type,
        unknown_count,
        entity_scores,
        top_issues,
        generated_at: chrono::Utc::now().to_rfc3339(),
    }
}

// ── Public re-exports used by dream.rs ───────────────────────────────────────

/// Returns the resolved schema.org type for an entity, preferring the stored
/// schema_type field over the static map.
pub fn resolved_schema_type(node: &EntityNode) -> Option<&str> {
    node.schema_type
        .as_deref()
        .or_else(|| schema_type_for(&node.entity_type))
}
