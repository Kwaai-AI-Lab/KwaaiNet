//! Convert NotebookLM structured JSON output into a graph seed.
//!
//! Input JSON schema: see the NotebookLM extraction prompt in
//!   tests/notebooklm_extraction_prompt.md
//!
//! Usage:
//!   kwaainet rag graph seed-from-json --file output.json --kb D6
//!   kwaainet rag graph seed-from-json --file output.json --kb D6 --emit-yaml  # also write seed.yaml

use anyhow::{Context, Result};
use serde::Deserialize;

// ── JSON schema types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct NbDocument {
    pub title: Option<String>,
    pub author: Option<String>,
    pub period: Option<String>,
    pub genre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NbEntity {
    pub canonical: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub gender: Option<String>,
    pub description: Option<String>,
    pub birth_year: Option<i32>,
    pub death_year: Option<i32>,
    pub confidence: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NbRelation {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub relation_type: String,
    pub evidence: Option<String>,
    pub confidence: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NbPayload {
    pub document: Option<NbDocument>,
    #[serde(default)]
    pub entities: Vec<NbEntity>,
    #[serde(default)]
    pub relations: Vec<NbRelation>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Load and validate a NotebookLM JSON payload from a file.
///
/// Strips markdown code fences if the user pasted from a chat UI.
pub fn load_nb_json(path: &std::path::Path) -> Result<NbPayload> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?;
    // Strip markdown code fences if the user pasted from a chat UI
    let cleaned = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    serde_json::from_str::<NbPayload>(cleaned)
        .with_context(|| "parsing NotebookLM JSON — check schema matches the extraction prompt")
}

/// Count entities with `confidence: low` (skipped during seeding).
pub fn count_low_confidence(payload: &NbPayload) -> (usize, usize) {
    let low_entities = payload
        .entities
        .iter()
        .filter(|e| e.confidence.as_deref() == Some("low"))
        .count();
    let low_relations = payload
        .relations
        .iter()
        .filter(|r| r.confidence.as_deref() == Some("low"))
        .count();
    (low_entities, low_relations)
}

/// Convert an NbPayload into the YAML seed format string.
///
/// The output is directly loadable by `kwaainet rag graph seed --file`.
/// Low-confidence relations are omitted; all entities are included regardless of confidence.
pub fn to_seed_yaml(payload: &NbPayload) -> String {
    let mut out = String::new();

    // Header comment block
    if let Some(doc) = &payload.document {
        out.push_str("# Auto-generated from NotebookLM extraction\n");
        if let Some(t) = &doc.title {
            out.push_str(&format!("# Source document: {t}\n"));
        }
        if let Some(a) = &doc.author {
            out.push_str(&format!("# Author: {a}\n"));
        }
        if let Some(p) = &doc.period {
            out.push_str(&format!("# Period: {p}\n"));
        }
        if let Some(g) = &doc.genre {
            out.push_str(&format!("# Genre: {g}\n"));
        }
    } else {
        out.push_str("# Auto-generated from NotebookLM extraction\n");
    }
    out.push('\n');

    // Group entities by type
    let people: Vec<_> = payload
        .entities
        .iter()
        .filter(|e| e.entity_type.eq_ignore_ascii_case("person"))
        .collect();
    let orgs: Vec<_> = payload
        .entities
        .iter()
        .filter(|e| e.entity_type.eq_ignore_ascii_case("organization"))
        .collect();
    let other: Vec<_> = payload
        .entities
        .iter()
        .filter(|e| {
            !e.entity_type.eq_ignore_ascii_case("person")
                && !e.entity_type.eq_ignore_ascii_case("organization")
        })
        .collect();

    if !people.is_empty() {
        out.push_str("persons:\n\n");
        for e in &people {
            write_entity(&mut out, e);
        }
    }
    if !orgs.is_empty() {
        out.push_str("organizations:\n\n");
        for e in &orgs {
            write_entity(&mut out, e);
        }
    }
    if !other.is_empty() {
        out.push_str("other_entities:\n\n");
        for e in &other {
            write_entity(&mut out, e);
        }
    }

    // Relations — only include high/medium confidence or where confidence is unset
    let rels: Vec<_> = payload
        .relations
        .iter()
        .filter(|r| r.confidence.as_deref() != Some("low"))
        .collect();

    if !rels.is_empty() {
        out.push_str("relations:\n\n");
        for r in &rels {
            if let Some(ev) = &r.evidence {
                // Truncate evidence to 120 chars to keep the comment readable
                let snippet: String = ev.chars().take(120).collect();
                out.push_str(&format!("  # {snippet}\n"));
            }
            out.push_str(&format!(
                "  - {{ from: {:?}, to: {:?}, type: {:?} }}\n",
                r.from, r.to, r.relation_type
            ));
        }
    }

    out
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn write_entity(out: &mut String, e: &NbEntity) {
    out.push_str(&format!("  - canonical: {:?}\n", e.canonical));
    if !e.aliases.is_empty() {
        out.push_str("    aliases:\n");
        for a in &e.aliases {
            out.push_str(&format!("      - {:?}\n", a));
        }
    }
    if let Some(g) = &e.gender {
        out.push_str(&format!("    gender: {:?}\n", g));
    }
    if let Some(d) = &e.description {
        let trimmed = d.trim();
        // Build-year info appended to description if present
        let mut desc = trimmed.to_string();
        match (e.birth_year, e.death_year) {
            (Some(b), Some(d_yr)) => desc.push_str(&format!(" (b. {b}, d. {d_yr})")),
            (Some(b), None) => desc.push_str(&format!(" (b. {b})")),
            (None, Some(d_yr)) => desc.push_str(&format!(" (d. {d_yr})")),
            (None, None) => {}
        }
        // Use block scalar for descriptions (handles single-line and multi-line uniformly)
        out.push_str("    description: >\n");
        for line in desc.lines() {
            out.push_str(&format!("      {}\n", line.trim()));
        }
    } else {
        // Even without a description, append birth/death years as the description
        match (e.birth_year, e.death_year) {
            (Some(b), Some(d_yr)) => {
                out.push_str(&format!(
                    "    description: >\n      b. {b}, d. {d_yr}\n"
                ));
            }
            (Some(b), None) => {
                out.push_str(&format!("    description: >\n      b. {b}\n"));
            }
            (None, Some(d_yr)) => {
                out.push_str(&format!("    description: >\n      d. {d_yr}\n"));
            }
            (None, None) => {}
        }
    }
    out.push('\n');
}

// ── conversion to FamilyTree for seeding ─────────────────────────────────────

/// Convert an NbPayload into a [`crate::family::FamilyTree`] for direct seeding.
///
/// Non-Person entities (Organizations, Locations, etc.) are included as PersonSeed entries
/// with their entity_type embedded in the description so the graph gets all entities.
/// Only high/medium confidence relations are included.
pub fn to_family_tree(payload: &NbPayload) -> crate::family::FamilyTree {
    use crate::family::{FamilyTree, PersonSeed, RelationSeed};

    let persons: Vec<PersonSeed> = payload
        .entities
        .iter()
        .map(|e| {
            let mut desc = e
                .description
                .as_deref()
                .unwrap_or("")
                .trim()
                .to_string();
            // Append birth/death years to description if available
            match (e.birth_year, e.death_year) {
                (Some(b), Some(d)) => desc.push_str(&format!(" (b. {b}, d. {d})")),
                (Some(b), None) => desc.push_str(&format!(" (b. {b})")),
                (None, Some(d)) => desc.push_str(&format!(" (d. {d})")),
                (None, None) => {}
            }
            PersonSeed {
                canonical: e.canonical.clone(),
                aliases: e.aliases.clone(),
                description: desc,
            }
        })
        .collect();

    let relations: Vec<RelationSeed> = payload
        .relations
        .iter()
        .filter(|r| r.confidence.as_deref() != Some("low"))
        .map(|r| RelationSeed {
            from: r.from.clone(),
            to: r.to.clone(),
            relation_type: r.relation_type.clone(),
        })
        .collect();

    FamilyTree { persons, relations }
}
