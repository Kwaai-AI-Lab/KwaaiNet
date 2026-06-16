//! Seed a knowledge graph from a ground-truth family tree YAML file.
//!
//! This module is used by `kwaainet rag graph seed` to:
//!   1. Upsert canonical Person entities (with authoritative descriptions + embeddings).
//!   2. Merge known alias entities into each canonical (re-pointing all their relations).
//!   3. Upsert ground-truth family relations (parent_of, spouse_of, sibling_of, etc.).
//!
//! The YAML format is documented in `tests/kwaai-knowledge/d6_family_tree.yaml`.

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

use crate::embedder::EmbedClient;
use crate::graph::{entity_id, EntityNode, GraphStore};

// ── YAML types ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct FamilyTree {
    pub persons: Vec<PersonSeed>,
    #[serde(default)]
    pub relations: Vec<RelationSeed>,
}

#[derive(Deserialize)]
pub struct PersonSeed {
    pub canonical: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub description: String,
    /// Entity type for this seed entry. Defaults to "Person" for backward compatibility.
    /// Set to "Organization" or "Place" to seed non-person entities.
    #[serde(default = "default_entity_type")]
    pub entity_type: String,
    /// Explicit gender override ("Male" / "Female"). When set, takes precedence over any
    /// gender inferred from the entity description, ensuring reliable father/mother resolution.
    pub gender: Option<String>,
}

fn default_entity_type() -> String {
    "Person".to_string()
}

#[derive(Deserialize)]
pub struct RelationSeed {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub relation_type: String,
}

// ── Stats ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct SeedStats {
    pub entities_upserted: usize,
    pub aliases_merged: usize,
    pub relations_merged: usize,
    pub relations_upserted: usize,
    pub aliases_not_found: usize,
    /// Non-seeded parent_of/child_of edges purged because ground-truth parentage was seeded.
    pub relations_purged: usize,
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn load_family_tree(path: &Path) -> Result<FamilyTree> {
    let raw = std::fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&raw)?)
}

/// Seed the graph with canonical Person entities, merge aliases, and plant family relations.
///
/// The progress callback receives short status strings suitable for printing to a terminal.
pub async fn seed_family_tree(
    graph: &mut GraphStore,
    tree: &FamilyTree,
    embed: &EmbedClient,
    mut progress: impl FnMut(&str),
) -> Result<SeedStats> {
    let mut stats = SeedStats::default();

    // ── Pass 1: upsert canonical entities and merge aliases ───────────────────
    for person in &tree.persons {
        progress(&format!("seeding {}", person.canonical));

        let desc = if !person.description.is_empty() {
            person.description.trim().to_string()
        } else {
            String::new()
        };
        let embed_text = if desc.is_empty() {
            person.canonical.clone()
        } else {
            format!("{}: {}", person.canonical, desc)
        };
        let embedding = embed.embed_one(&embed_text).await.unwrap_or_default();

        let eid = entity_id(&person.canonical, &person.entity_type);
        let existing = graph.get_entity(eid).cloned();

        // Merge existing stored aliases with all YAML-declared aliases so that even
        // previously-merged aliases (no longer in the graph as separate entities) remain
        // queryable via find_ids_by_name_token.
        let mut merged_aliases: Vec<String> = existing
            .as_ref()
            .map(|e| e.aliases.clone())
            .unwrap_or_default();
        for a in &person.aliases {
            if !merged_aliases.contains(a) {
                merged_aliases.push(a.clone());
            }
        }

        let node = EntityNode {
            id: eid,
            name: person.canonical.clone(),
            entity_type: person.entity_type.clone(),
            description: if !desc.is_empty() {
                desc
            } else {
                existing
                    .as_ref()
                    .map(|e| e.description.clone())
                    .unwrap_or_default()
            },
            embedding,
            mention_count: existing
                .as_ref()
                .map(|e| e.mention_count)
                .unwrap_or(1)
                .max(1),
            first_chunk_id: existing.as_ref().map(|e| e.first_chunk_id).unwrap_or(0),
            aliases: merged_aliases,
            schema_type: existing.as_ref().and_then(|e| e.schema_type.clone()),
            evidence: Vec::new(),
            gender: person
                .gender
                .clone()
                .or_else(|| existing.as_ref().and_then(|e| e.gender.clone())),
            fields: existing
                .as_ref()
                .map(|e| e.fields.clone())
                .unwrap_or_default(),
            confidence: existing.as_ref().map(|e| e.confidence).unwrap_or(0.0),
            // Seeded entities are ground-truth — always high confidence.
            extraction_confidence: 1.0,
        };

        graph.upsert_entity(node)?;
        stats.entities_upserted += 1;

        // Merge each alias into the canonical entity
        for alias in &person.aliases {
            let alias_node = find_alias(graph, alias);
            match alias_node {
                Some(alias_id) if alias_id != eid => {
                    progress(&format!("  merged '{}' → '{}'", alias, person.canonical));
                    let moved = graph.merge_entity_into(alias_id, eid)?;
                    stats.aliases_merged += 1;
                    stats.relations_merged += moved;
                }
                None => {
                    tracing::debug!("alias '{}' not found in graph — skipping", alias);
                    stats.aliases_not_found += 1;
                }
                _ => {}
            }
        }
    }

    // Rebuild adjacency after all merges before upsetting relations
    graph.rebuild_in_memory()?;

    // ── Pass 2: upsert ground-truth family relations ──────────────────────────
    for rel in &tree.relations {
        let from_id = find_alias(graph, &rel.from);
        let to_id = find_alias(graph, &rel.to);

        match (from_id, to_id) {
            (Some(fid), Some(tid)) => {
                graph.upsert_relation(fid, tid, &rel.relation_type, 0)?;
                stats.relations_upserted += 1;
            }
            _ => {
                tracing::warn!(
                    "relation '{}' → '{}' [{}]: one or both endpoints not in graph",
                    rel.from,
                    rel.to,
                    rel.relation_type
                );
            }
        }
    }

    // ── Pass 3: purge LLM-hallucinated parent/child edges ────────────────────
    // For any entity whose parents are now established by seed, remove any
    // competing non-seeded child_of/parent_of edges (hallucinations from LLM
    // relation extraction runs).
    let purged = graph.purge_unseeded_parent_relations()?;
    if purged > 0 {
        progress(&format!(
            "purged {purged} unseeded parent/child edges that conflict with seed"
        ));
    }
    stats.relations_purged = purged;

    Ok(stats)
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Try exact name match first, then normalized (strips punctuation).
fn find_alias(graph: &GraphStore, name: &str) -> Option<i64> {
    graph
        .find_by_name(name)
        .or_else(|| graph.find_by_name_normalized(name))
        .map(|n| n.id)
}
