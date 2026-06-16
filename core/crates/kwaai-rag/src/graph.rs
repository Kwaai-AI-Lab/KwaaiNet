//! Knowledge graph: entity nodes, directed relations, BFS traversal, and
//! LLM-based entity extraction.
//!
//! Persists to a per-tenant `graph-<uuid>.redb` file alongside the chunk store.
//! In-memory adjacency list and entity index are rebuilt on open — same pattern
//! as kwaai-storage's HNSW index.

use anyhow::{Context, Result};
use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use uuid::Uuid;

// ── redb table definitions ────────────────────────────────────────────────────

/// key = entity_id i64 LE (8 bytes) → EntityNode JSON
const ENTITIES_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("entities");

/// key = src_id_le(8) ++ dst_id_le(8) ++ relation_type_bytes → RelationRecord JSON
const RELATIONS_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("relations");

/// key = chunk_id i64 LE (8 bytes) → [entity_id] JSON array
const CHUNK_ENTITY_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("chunk_entities");

/// key = entity_id i64 LE (8 bytes) → [chunk_id] JSON array
const ENTITY_CHUNK_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("entity_chunks");

/// key = entity_id i64 LE (8 bytes) → JSON Vec<TimelineEvent>
const TIMELINE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("entity_timeline_v1");

/// key = src_id_le(8) ++ dst_id_le(8) → JSON Vec<SequenceInteraction>
const INTERACTION_TABLE: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("seq_interactions_v1");

/// key = string → JSON-encoded value (KB-level metadata)
const METADATA_TABLE: TableDefinition<&str, &str> = TableDefinition::new("metadata");

// ── Ontology constants ────────────────────────────────────────────────────────

/// Supported entity types for LLM extraction prompts.
pub const ENTITY_TYPES: &[&str] = &[
    "Person",
    "Organization",
    "Location",
    "Event",
    "Concept",
    "Method",
    "Claim",
    "Quantity",
    "Date",
    "Document",
    "Product",
    "Technology",
    "Role",
    "Topic",
    "Unknown",
];

/// Supported relation types for LLM extraction prompts.
pub const RELATION_TYPES: &[&str] = &[
    // Family — explicit so LLM uses precise terms instead of "related_to"
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
    // Agent
    "works_at",
    "founded",
    "manages",
    "belongs_to",
    "member_of",
    "led",
    "endorses",
    // Spatial / biographical
    "lived_in",
    "visited",
    "built",
    // Structural
    "part_of",
    "contains",
    "located_in",
    "instance_of",
    "subtype_of",
    // Temporal
    "occurred_on",
    "started",
    "ended",
    "followed_by",
    "precedes",
    // Semantic
    "related_to",
    "contradicts",
    "supports",
    "cites",
    "implements",
    // Informational
    "defined_by",
    "described_in",
    "measured_by",
    "associated_with",
    "caused_by",
];

/// Relation types used when entity_types is restricted to Person only.
/// Drops structural/temporal/informational types that are meaningless between two people.
pub const PERSON_RELATION_TYPES: &[&str] = &[
    // Family
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
    // Social / agent (Person–Person only variants)
    "works_at",
    "belongs_to",
    "endorses",
    "associated_with",
    "related_to",
    "supported",
];

/// Familial relation types — only valid between two Person entities.
pub const FAMILIAL_RELS: &[&str] = &[
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
];

/// Asymmetric familial relation → its logical inverse.
/// When A parent_of B is stored, B child_of A is automatically stored too.
/// Symmetric relations (spouse_of, sibling_of, cousin_of, half_sibling_of) are not listed —
/// they are stored in both directions by the caller.
const FAMILIAL_INVERSE: &[(&str, &str)] = &[
    ("parent_of", "child_of"),
    ("child_of", "parent_of"),
    ("grandparent_of", "grandchild_of"),
    ("grandchild_of", "grandparent_of"),
    ("uncle_of", "nephew_of"), // approximate — gender of nephew/niece unknown here
    ("aunt_of", "niece_of"),
    ("nephew_of", "uncle_of"),
    ("niece_of", "aunt_of"),
    ("foster_parent_of", "foster_child_of"),
    ("foster_child_of", "foster_parent_of"),
];

// ── Core types ────────────────────────────────────────────────────────────────

/// A single structured metadata field on an entity, with provenance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValue {
    pub value: String,
    /// Chunk IDs whose text provided evidence for this field value.
    #[serde(default)]
    pub evidence_chunk_ids: Vec<i64>,
    /// 0.0–1.0; grows with evidence count: `min(1.0, count / 3.0)`.
    pub confidence: f32,
}

impl FieldValue {
    pub fn new(value: impl Into<String>, chunk_id: i64) -> Self {
        Self {
            value: value.into(),
            evidence_chunk_ids: vec![chunk_id],
            confidence: 1.0_f32 / 3.0,
        }
    }

    /// Add a supporting chunk ID and recompute confidence.
    pub fn add_evidence(&mut self, chunk_id: i64) {
        if !self.evidence_chunk_ids.contains(&chunk_id) {
            self.evidence_chunk_ids.push(chunk_id);
        }
        self.confidence = (self.evidence_chunk_ids.len() as f32 / 3.0).min(1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityNode {
    /// Deterministic: sha256(name.lower() + "::" + entity_type)[..8] as i64 LE.
    pub id: i64,
    pub name: String,
    pub entity_type: String,
    /// Prose summary derived from `fields`; updated whenever fields change.
    /// Falls back to raw LLM description for entity types without expected fields.
    pub description: String,
    /// Embedding of "{name}: {description}" for similarity search.
    /// Includes the name so abbreviations/acronyms find the right entity.
    pub embedding: Vec<f32>,
    pub mention_count: u32,
    pub first_chunk_id: i64,
    /// Names of entities that were merged into this one (alias names preserved for lookup).
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Canonical schema.org type resolved by the dream completion step.
    /// Falls back to SCHEMA_TYPE_MAP[entity_type] in the scorer when None.
    /// Never changes entity_id — that remains hash(name + entity_type).
    #[serde(default)]
    pub schema_type: Option<String>,
    /// Inferred from pronouns in description: "Male", "Female", or None if unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// All chunk IDs that mention this entity. Populated at load time from the
    /// chunk index — not persisted in the entity record itself. Always current
    /// after rebuild(). Use this in dream tasks instead of a separate lookup.
    #[serde(default, skip_serializing)]
    pub evidence: Vec<i64>,
    /// Structured schema.org-aligned metadata fields with evidence provenance.
    /// Keys are schema.org property names (e.g. "birthDate", "addressLocality").
    /// Empty for entity types without a defined field schema.
    #[serde(default)]
    pub fields: HashMap<String, FieldValue>,
    /// Structural completeness score in [0, 1] computed by score_entity() after each
    /// CC build. 0.0 means not yet scored (backwards-compatible default).
    /// Low scores drive EC refinement when --ec-refine-threshold is set.
    #[serde(default)]
    pub confidence: f32,
    /// LLM-assigned confidence at extraction time: 1.0 = clearly stated in text,
    /// 0.5 = inferred from context, 0.2 = uncertain/possible hallucination.
    /// Persisted across dream cycles so low-confidence entities can be targeted for
    /// review by a larger model (e.g. llama3.3:70b via --review-model).
    #[serde(default = "default_extraction_confidence")]
    pub extraction_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationRecord {
    pub src_id: i64,
    pub dst_id: i64,
    pub relation_type: String,
    /// 0.0–1.0; grows with evidence count: min(1.0, len(evidence) / 10).
    pub strength: f32,
    pub evidence_chunk_ids: Vec<i64>,
}

/// Raw extraction output from the LLM before embedding / storing.
#[derive(Debug, Deserialize)]
pub struct ExtractedEntity {
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    /// Legacy prose description; used when `fields` is empty (full 15-type extraction).
    #[serde(default)]
    pub description: String,
    /// Structured fields returned by the 3-type (no_relations) extraction prompt.
    #[serde(default)]
    pub fields: HashMap<String, String>,
    /// LLM-assigned extraction confidence: 1.0 = clearly stated in text, 0.5 = inferred
    /// from context, 0.2 = uncertain or possible hallucination. Defaults to 1.0 when
    /// the LLM omits the field (older prompts, backwards compatibility).
    #[serde(default = "default_extraction_confidence")]
    pub extraction_confidence: f32,
}

fn default_extraction_confidence() -> f32 {
    1.0
}

#[derive(Debug, Deserialize)]
pub struct ExtractedRelation {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Deserialize)]
struct ExtractionPayload {
    #[serde(default)]
    entities: Vec<ExtractedEntity>,
    #[serde(default)]
    relations: Vec<ExtractedRelation>,
}

// ── Deterministic entity ID ───────────────────────────────────────────────────

pub fn entity_id(name: &str, entity_type: &str) -> i64 {
    let mut h = Sha256::new();
    h.update(name.to_lowercase().as_bytes());
    h.update(b"::");
    h.update(entity_type.as_bytes());
    let d = h.finalize();
    i64::from_le_bytes(d[..8].try_into().unwrap())
}

// ── Schema field registry ─────────────────────────────────────────────────────

/// Schema.org-aligned metadata fields expected for each entity type.
/// Returns `(field_key, human_description)` pairs. Empty for types without a
/// defined structured schema (fall back to prose description).
pub fn expected_fields(entity_type: &str) -> &'static [(&'static str, &'static str)] {
    match entity_type {
        "Person" => &[
            ("birthDate", "date of birth"),
            ("birthPlace", "place of birth"),
            ("deathDate", "date of death (if deceased)"),
            ("nationality", "nationality or cultural identity"),
            ("occupation", "profession or main occupation"),
            ("affiliation", "organization they belong or belonged to"),
            ("spouse", "spouse or partner name"),
            ("parent", "parent names"),
            ("sibling", "sibling names"),
            ("child", "child names"),
        ],
        "Place" | "Location" => &[
            (
                "locationType",
                "type of place (district, city, country, neighbourhood, building, street)",
            ),
            ("historicalNote", "what happened here — verbatim or close paraphrase from the text"),
        ],
        "Organization" => &[
            ("foundingDate", "year or period when founded"),
            (
                "dissolutionDate",
                "year or period when dissolved, if applicable",
            ),
            ("founder", "founder name"),
            (
                "orgType",
                "type of organization (school, mosque, newspaper, political party, etc.)",
            ),
        ],
        _ => &[],
    }
}

/// Build a prose description from an entity's structured fields.
/// Produces "Name — key: value; key: value; ..." ordered by `expected_fields`.
/// Returns empty string when no fields are filled.
pub fn description_from_fields(
    name: &str,
    entity_type: &str,
    fields: &HashMap<String, FieldValue>,
) -> String {
    let schema = expected_fields(entity_type);
    if schema.is_empty() || fields.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = schema
        .iter()
        .filter_map(|(key, _)| {
            fields
                .get(*key)
                .filter(|fv| {
                    if fv.value.is_empty() {
                        return false;
                    }
                    // Drop placeholder values the LLM emits when it has no evidence.
                    !matches!(
                        fv.value.to_lowercase().trim(),
                        "unknown"
                            | "undefined"
                            | "n/a"
                            | "none"
                            | "not stated"
                            | "not specified"
                            | "not mentioned"
                            | "not known"
                            | "not applicable"
                            | "unspecified"
                    )
                })
                .map(|fv| format!("{}: {}", key, fv.value))
        })
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("{} — {}", name, parts.join("; "))
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Strip punctuation, collapse whitespace, lowercase — used for fuzzy name matching.
pub fn normalize_name(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Canonical ordered pair — always (smaller, larger) — for dedup seen-sets.
#[inline]
/// Returns true when two family relation types are structurally contradictory when
/// applied from the same entity to the same target (e.g., cannot be both `spouse_of`
/// and `sibling_of` the same person simultaneously).
///
/// Same role to the same target is NOT contradictory — it is a positive dedup signal.
/// Inverse pairs (parent_of ↔ child_of) applied to the same target ARE contradictory
/// because they would create a self-referential loop.
fn family_role_contradicts(r1: &str, r2: &str) -> bool {
    if r1 == r2 {
        return false; // same role — positive signal, not a contradiction
    }
    // Pairs that would create circular or impossible relationships:
    const CONTRADICTION_PAIRS: &[(&str, &str)] = &[
        ("spouse_of", "sibling_of"),
        ("spouse_of", "half_sibling_of"),
        ("spouse_of", "child_of"),
        ("spouse_of", "parent_of"),
        ("spouse_of", "grandparent_of"),
        ("spouse_of", "grandchild_of"),
        ("parent_of", "child_of"),
        ("grandparent_of", "grandchild_of"),
        ("sibling_of", "parent_of"),
        ("sibling_of", "child_of"),
        ("half_sibling_of", "parent_of"),
        ("half_sibling_of", "child_of"),
    ];
    for &(a, b) in CONTRADICTION_PAIRS {
        if (r1 == a && r2 == b) || (r1 == b && r2 == a) {
            return true;
        }
    }
    false
}

pub fn ord_pair(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

const HONORIFICS: &[&str] = &[
    "dr",
    "mr",
    "mrs",
    "ms",
    "miss",
    "prof",
    "professor",
    "rev",
    "reverend",
    "sir",
    "haji",
    "hajj",
    "maulvi",
    "maulana",
    "imam",
    "sheikh",
    "shaykh",
    "auntie",
    "aunt",
    "uncle",
    "oom",
    "tannie",
    "oupa",
    "my",
];

/// Strip trailing academic/professional qualification tokens from a name.
/// Returns the normalized stripped form if at least one token was removed; None otherwise.
///
/// Works on the original (pre-normalize) tokens so that dotted forms like "M.A.",
/// "Ph.D.", "LL.B." are recognised — normalize_name() maps dots to spaces, which
/// would split "M.A." into ["m", "a"] and break the match.
fn strip_qualifications(name: &str) -> Option<String> {
    fn is_qual(tok: &str) -> bool {
        const QUALS: &[&str] = &[
            "ma", "ba", "bsc", "msc", "phd", "llb", "llm", "bed", "bcom", "mba", "mpa", "hons",
            "dip", "jp", "obe", "mbe", "mbbs",
        ];
        // Strip all dots then lowercase — recognises "M.A.", "Ph.D.", "LL.B." as well as
        // undotted "MA", "PhD", "LLB".
        let undotted: String = tok
            .chars()
            .filter(|&c| c != '.')
            .collect::<String>()
            .to_lowercase();
        !undotted.is_empty() && QUALS.contains(&undotted.as_str())
    }

    let tokens: Vec<&str> = name.split_whitespace().collect();
    let orig_len = tokens.len();
    let mut end = orig_len;
    while end > 0 && is_qual(tokens[end - 1]) {
        end -= 1;
    }
    if end == orig_len {
        return None; // nothing stripped
    }
    Some(normalize_name(&tokens[..end].join(" ")))
}

/// Normalize then strip all leading and trailing honorific tokens.
fn stripped_key(name: &str) -> String {
    let norm = normalize_name(name);
    norm.split_whitespace()
        .filter(|w| !HONORIFICS.contains(w))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Levenshtein edit distance (character-level).
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (la, lb) = (a.len(), b.len());
    if la == 0 {
        return lb;
    }
    if lb == 0 {
        return la;
    }
    let mut prev: Vec<usize> = (0..=lb).collect();
    let mut curr = vec![0usize; lb + 1];
    #[allow(clippy::needless_range_loop)]
    // `i` used both as index into `a` and as counter for curr[0]
    for i in 0..la {
        curr[0] = i + 1;
        for j in 0..lb {
            let cost = if a[i] == b[j] { 0 } else { 1 };
            curr[j + 1] = (curr[j] + 1).min(prev[j + 1] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[lb]
}

// ── GraphStore ────────────────────────────────────────────────────────────────

pub struct GraphStore {
    db: Database,
    /// entity_id → EntityNode
    nodes: HashMap<i64, EntityNode>,
    /// src_id → [(dst_id, relation_type, strength)] — bidirectional for traversal
    adj: HashMap<i64, Vec<(i64, String, f32)>>,
    /// chunk_id → [entity_id]
    chunk_to_entities: HashMap<i64, Vec<i64>>,
    /// entity_id → [chunk_id]
    entity_to_chunks: HashMap<i64, Vec<i64>>,
    /// Exhaustive alias token index: raw-lowercased token → [entity_id].
    /// Built from every whitespace-split token of canonical name + all aliases.
    /// Stores both the raw form ("j.m.h.") and trimmed form ("j.m.h") so query
    /// tokenizers that strip trailing punctuation still get a hit.
    pub alias_token_index: HashMap<String, Vec<i64>>,
}

impl GraphStore {
    /// Open (or create) the graph store for a tenant.
    pub fn open(data_dir: &Path, tenant_id: Uuid) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join(format!("graph-{}.redb", tenant_id));
        let db = Database::create(&path)
            .with_context(|| format!("opening graph store at {}", path.display()))?;

        {
            let wtxn = db.begin_write()?;
            wtxn.open_table(ENTITIES_TABLE)?;
            wtxn.open_table(RELATIONS_TABLE)?;
            wtxn.open_table(CHUNK_ENTITY_TABLE)?;
            wtxn.open_table(ENTITY_CHUNK_TABLE)?;
            wtxn.open_table(METADATA_TABLE)?;
            wtxn.commit()?;
        }

        let mut store = Self {
            db,
            nodes: HashMap::new(),
            adj: HashMap::new(),
            chunk_to_entities: HashMap::new(),
            entity_to_chunks: HashMap::new(),
            alias_token_index: HashMap::new(),
        };
        store.rebuild()?;
        Ok(store)
    }

    fn rebuild(&mut self) -> Result<()> {
        let rtxn = self.db.begin_read()?;

        {
            let table = rtxn.open_table(ENTITIES_TABLE)?;
            for entry in table.iter()? {
                let (_, v) = entry?;
                let node: EntityNode = serde_json::from_slice(v.value())?;
                self.nodes.insert(node.id, node);
            }
        }

        {
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            for entry in table.iter()? {
                let (_, v) = entry?;
                let rel: RelationRecord = serde_json::from_slice(v.value())?;
                self.adj.entry(rel.src_id).or_default().push((
                    rel.dst_id,
                    rel.relation_type.clone(),
                    rel.strength,
                ));
                self.adj.entry(rel.dst_id).or_default().push((
                    rel.src_id,
                    rel.relation_type.clone(),
                    rel.strength,
                ));
            }
        }

        {
            let table = rtxn.open_table(CHUNK_ENTITY_TABLE)?;
            for entry in table.iter()? {
                let (k, v) = entry?;
                if k.value().len() != 8 {
                    continue;
                }
                let cid = i64::from_le_bytes(k.value().try_into().unwrap());
                let eids: Vec<i64> = serde_json::from_slice(v.value()).unwrap_or_default();
                self.chunk_to_entities.insert(cid, eids);
            }
        }

        {
            let table = rtxn.open_table(ENTITY_CHUNK_TABLE)?;
            for entry in table.iter()? {
                let (k, v) = entry?;
                if k.value().len() != 8 {
                    continue;
                }
                let eid = i64::from_le_bytes(k.value().try_into().unwrap());
                let cids: Vec<i64> = serde_json::from_slice(v.value()).unwrap_or_default();
                self.entity_to_chunks.insert(eid, cids);
            }
        }

        // Populate entity.evidence from the index (only for live entities).
        for (&eid, cids) in &self.entity_to_chunks {
            if let Some(node) = self.nodes.get_mut(&eid) {
                node.evidence = cids.clone();
            }
        }

        // Build exhaustive alias token index: raw lowercased tokens (no normalization).
        // Covers "j.m.h." from alias "J.M.H. Gool" so the retriever finds the canonical
        // entity even when the query word hasn't been normalized.
        self.alias_token_index.clear();
        for (&id, node) in &self.nodes {
            let name_forms =
                std::iter::once(node.name.as_str()).chain(node.aliases.iter().map(|a| a.as_str()));
            for form in name_forms {
                for token in form.split_whitespace() {
                    let raw = token.to_lowercase();
                    let trimmed: String = token
                        .trim_matches(|c: char| !c.is_alphanumeric())
                        .to_lowercase();
                    for tok in [raw.as_str(), trimmed.as_str()] {
                        if tok.len() >= 2 {
                            self.alias_token_index
                                .entry(tok.to_string())
                                .or_default()
                                .push(id);
                        }
                    }
                }
            }
        }

        tracing::info!(
            entities = self.nodes.len(),
            relations = self.adj.values().map(|v| v.len()).sum::<usize>() / 2,
            "graph store loaded"
        );
        Ok(())
    }

    // ── Writes ────────────────────────────────────────────────────────────────

    /// Insert or merge an entity. Increments mention_count; keeps longer description.
    pub fn upsert_entity(&mut self, node: EntityNode) -> Result<()> {
        let merged = match self.nodes.get(&node.id) {
            Some(existing) => {
                // Merge structured fields: existing evidence is preserved; new chunk IDs added.
                let mut merged_fields = existing.fields.clone();
                for (key, new_fv) in &node.fields {
                    merged_fields
                        .entry(key.clone())
                        .and_modify(|efv| {
                            for &cid in &new_fv.evidence_chunk_ids {
                                efv.add_evidence(cid);
                            }
                            if efv.value.is_empty() && !new_fv.value.is_empty() {
                                efv.value.clone_from(&new_fv.value);
                            }
                        })
                        .or_insert_with(|| new_fv.clone());
                }
                // Recompute description from merged fields, then pick the richest candidate.
                // Computed-from-fields was previously always preferred, but that caused short
                // "Name — occupation: X" strings to overwrite rich YAML-seeded prose descriptions.
                // Rule: pick the longest non-empty candidate across computed, incoming, existing.
                let computed =
                    description_from_fields(&existing.name, &existing.entity_type, &merged_fields);
                let best_desc = [
                    computed,
                    node.description.clone(),
                    existing.description.clone(),
                ]
                .into_iter()
                .filter(|d| !d.is_empty())
                .max_by_key(|d| d.len())
                .unwrap_or_default();
                let best_emb = if best_desc == existing.description {
                    existing.embedding.clone()
                } else {
                    node.embedding.clone()
                };
                let mut merged_aliases = existing.aliases.clone();
                for a in &node.aliases {
                    if !merged_aliases.contains(a) {
                        merged_aliases.push(a.clone());
                    }
                }
                EntityNode {
                    id: node.id,
                    name: existing.name.clone(),
                    entity_type: existing.entity_type.clone(),
                    description: best_desc,
                    embedding: best_emb,
                    mention_count: existing.mention_count + 1,
                    first_chunk_id: existing.first_chunk_id,
                    aliases: merged_aliases,
                    schema_type: existing.schema_type.clone().or(node.schema_type.clone()),
                    gender: existing.gender.clone().or(node.gender.clone()),
                    evidence: existing.evidence.clone(),
                    fields: merged_fields,
                    confidence: 0.0,
                    // Multiple sightings raise extraction confidence — if it appears in
                    // several chunks it is likely real even if one extraction was uncertain.
                    extraction_confidence: f32::max(
                        existing.extraction_confidence,
                        node.extraction_confidence,
                    ),
                }
            }
            None => node,
        };

        let key = merged.id.to_le_bytes();
        let val = serde_json::to_vec(&merged)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            t.insert(key.as_ref(), val.as_slice())?;
        }
        wtxn.commit()?;
        self.nodes.insert(merged.id, merged);
        Ok(())
    }

    /// Remove `alias_name` from `canonical_name`'s alias list and re-insert it as its own
    /// stub entity. Returns the number of aliases removed (0 = alias not found, 1 = removed).
    /// The restored entity has mention_count=1 and a zero embedding — run `graph reembed`
    /// afterwards to restore a proper embedding for it.
    pub fn unmerge_alias(
        &mut self,
        canonical_name: &str,
        entity_type: &str,
        alias_name: &str,
    ) -> Result<usize> {
        let canonical_id = entity_id(canonical_name, entity_type);
        let canonical = match self.nodes.get(&canonical_id).cloned() {
            Some(n) => n,
            None => anyhow::bail!("entity '{}' not found", canonical_name),
        };
        let pos = canonical.aliases.iter().position(|a| a == alias_name);
        if pos.is_none() {
            return Ok(0);
        }
        let mut new_aliases = canonical.aliases.clone();
        new_aliases.remove(pos.unwrap());
        let new_mention_count = canonical.mention_count.saturating_sub(1);
        let updated_canonical = EntityNode {
            aliases: new_aliases,
            mention_count: new_mention_count,
            ..canonical.clone()
        };

        let dim = canonical.embedding.len();
        let alias_id = entity_id(alias_name, entity_type);
        let alias_node = EntityNode {
            id: alias_id,
            name: alias_name.to_string(),
            entity_type: entity_type.to_string(),
            description: String::new(),
            embedding: vec![0.0_f32; dim],
            mention_count: 1,
            first_chunk_id: canonical.first_chunk_id,
            aliases: vec![],
            schema_type: None,
            gender: None,
            evidence: vec![],
            fields: Default::default(),
            confidence: 0.0,
            extraction_confidence: 1.0,
        };

        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            t.insert(
                &canonical_id.to_le_bytes()[..],
                serde_json::to_vec(&updated_canonical)?.as_slice(),
            )?;
            t.insert(
                &alias_id.to_le_bytes()[..],
                serde_json::to_vec(&alias_node)?.as_slice(),
            )?;
        }
        wtxn.commit()?;

        if let Some(n) = self.nodes.get_mut(&canonical_id) {
            n.aliases = updated_canonical.aliases;
            n.mention_count = updated_canonical.mention_count;
        }
        self.nodes.insert(alias_id, alias_node);
        Ok(1)
    }

    /// Insert or strengthen a directed relation, adding the evidence chunk.
    pub fn upsert_relation(
        &mut self,
        src_id: i64,
        dst_id: i64,
        relation_type: &str,
        evidence_chunk_id: i64,
    ) -> Result<()> {
        // ── Constraint: located_in / works_at must not target a CreativeWork entity ──
        // This prevents book/document titles from being incorrectly used as place or
        // employer targets when the LLM confuses the source document title with a location.
        if matches!(relation_type, "located_in" | "works_at") {
            if let Some(dst_node) = self.nodes.get(&dst_id) {
                if dst_node.schema_type.as_deref() == Some("schema:CreativeWork") {
                    return Ok(()); // silently drop — destination is a work, not a place/org
                }
            }
        }

        // ── Constraint: familial relations require both endpoints to be Person entities ──
        // If an entity is absent from the graph (e.g., a seed target not yet extracted),
        // treat it as potentially valid — absent entities can't be "wrong type".
        if FAMILIAL_RELS.contains(&relation_type) {
            let src_is_person = self
                .nodes
                .get(&src_id)
                .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                .unwrap_or(true); // absent → assume OK; only reject known non-Person entities
            let dst_is_person = self
                .nodes
                .get(&dst_id)
                .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                .unwrap_or(true);
            if !src_is_person || !dst_is_person {
                return Ok(()); // silently skip — non-person familial relations are always errors
            }
        }

        self.upsert_relation_unchecked(src_id, dst_id, relation_type, evidence_chunk_id)?;

        // ── Auto-add logical inverse for asymmetric familial relations ──
        if let Some(&inverse) = FAMILIAL_INVERSE
            .iter()
            .find(|(r, _)| *r == relation_type)
            .map(|(_, inv)| inv)
        {
            self.upsert_relation_unchecked(dst_id, src_id, inverse, evidence_chunk_id)?;
        }

        // ── Symmetric familial relations: store both directions ──
        if matches!(
            relation_type,
            "spouse_of" | "sibling_of" | "half_sibling_of" | "cousin_of"
        ) {
            self.upsert_relation_unchecked(dst_id, src_id, relation_type, evidence_chunk_id)?;
        }

        Ok(())
    }

    fn upsert_relation_unchecked(
        &mut self,
        src_id: i64,
        dst_id: i64,
        relation_type: &str,
        evidence_chunk_id: i64,
    ) -> Result<()> {
        let key = relation_key(src_id, dst_id, relation_type);

        let merged = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            match table.get(key.as_slice())? {
                Some(v) => {
                    let mut r: RelationRecord = serde_json::from_slice(v.value())?;
                    if !r.evidence_chunk_ids.contains(&evidence_chunk_id) {
                        r.evidence_chunk_ids.push(evidence_chunk_id);
                        r.strength = (r.evidence_chunk_ids.len() as f32 / 10.0).min(1.0);
                    }
                    r
                }
                None => RelationRecord {
                    src_id,
                    dst_id,
                    relation_type: relation_type.to_string(),
                    strength: 0.1,
                    evidence_chunk_ids: vec![evidence_chunk_id],
                },
            }
        };

        let val = serde_json::to_vec(&merged)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(RELATIONS_TABLE)?;
            t.insert(key.as_slice(), val.as_slice())?;
        }
        wtxn.commit()?;

        update_adj(
            &mut self.adj,
            src_id,
            dst_id,
            relation_type,
            merged.strength,
        );
        update_adj(
            &mut self.adj,
            dst_id,
            src_id,
            relation_type,
            merged.strength,
        );
        Ok(())
    }

    /// Record which entities are mentioned in a chunk.
    pub fn link_chunk(&mut self, chunk_id: i64, entity_ids: &[i64]) -> Result<()> {
        if entity_ids.is_empty() {
            return Ok(());
        }

        let wtxn = self.db.begin_write()?;
        {
            let mut ce = wtxn.open_table(CHUNK_ENTITY_TABLE)?;
            let mut ec = wtxn.open_table(ENTITY_CHUNK_TABLE)?;

            let ck = chunk_id.to_le_bytes();
            let mut eids: Vec<i64> = match ce.get(ck.as_ref())? {
                Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                None => vec![],
            };
            for &eid in entity_ids {
                if !eids.contains(&eid) {
                    eids.push(eid);
                }
            }
            ce.insert(ck.as_ref(), serde_json::to_vec(&eids)?.as_slice())?;

            for &eid in entity_ids {
                let ek = eid.to_le_bytes();
                let mut cids: Vec<i64> = match ec.get(ek.as_ref())? {
                    Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                    None => vec![],
                };
                if !cids.contains(&chunk_id) {
                    cids.push(chunk_id);
                }
                ec.insert(ek.as_ref(), serde_json::to_vec(&cids)?.as_slice())?;
            }
        }
        wtxn.commit()?;

        let ce = self.chunk_to_entities.entry(chunk_id).or_default();
        for &eid in entity_ids {
            if !ce.contains(&eid) {
                ce.push(eid);
            }
            let ec = self.entity_to_chunks.entry(eid).or_default();
            if !ec.contains(&chunk_id) {
                ec.push(chunk_id);
            }
        }
        Ok(())
    }

    /// Increment mention_count for query enrichment (called after a successful query).
    pub fn increment_mention(&mut self, entity_id: i64) -> Result<()> {
        if let Some(node) = self.nodes.get_mut(&entity_id) {
            node.mention_count += 1;
            let key = entity_id.to_le_bytes();
            let val = serde_json::to_vec(node)?;
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(ENTITIES_TABLE)?;
                t.insert(key.as_ref(), val.as_slice())?;
            }
            wtxn.commit()?;
        }
        Ok(())
    }

    // ── Reads ─────────────────────────────────────────────────────────────────

    /// BFS from seed entity IDs, up to `hops` hops. Returns all reachable entity IDs
    /// including the seeds.
    pub fn bfs_neighbors(&self, seeds: &[i64], hops: usize) -> Vec<i64> {
        let mut visited: HashSet<i64> = seeds.iter().copied().collect();
        let mut frontier: Vec<i64> = seeds.to_vec();
        for _ in 0..hops {
            let mut next = Vec::new();
            for &id in &frontier {
                if let Some(neighbors) = self.adj.get(&id) {
                    for &(nbr, _, _) in neighbors {
                        if visited.insert(nbr) {
                            next.push(nbr);
                        }
                    }
                }
            }
            if next.is_empty() {
                break;
            }
            frontier = next;
        }
        visited.into_iter().collect()
    }

    /// All chunk IDs that mention any of the given entity IDs.
    pub fn entity_chunks(&self, entity_ids: &[i64]) -> Vec<i64> {
        let mut out: HashSet<i64> = HashSet::new();
        for &eid in entity_ids {
            if let Some(cids) = self.entity_to_chunks.get(&eid) {
                out.extend(cids.iter().copied());
            }
        }
        out.into_iter().collect()
    }

    /// Brute-force cosine search over entity description embeddings.
    /// Adequate for <10K entities; add HNSW later if needed.
    pub fn search_entities(&self, query_emb: &[f32], top_k: usize) -> Vec<(i64, f64)> {
        let qnorm: f64 = query_emb
            .iter()
            .map(|&x| (x as f64) * (x as f64))
            .sum::<f64>()
            .sqrt();
        if qnorm == 0.0 || self.nodes.is_empty() {
            return vec![];
        }

        let mut scored: Vec<(i64, f64)> = self
            .nodes
            .values()
            .filter(|n| !n.embedding.is_empty())
            .map(|n| {
                let dot: f64 = query_emb
                    .iter()
                    .zip(n.embedding.iter())
                    .map(|(&q, &d)| (q as f64) * (d as f64))
                    .sum();
                let dnorm: f64 = n
                    .embedding
                    .iter()
                    .map(|&x| (x as f64) * (x as f64))
                    .sum::<f64>()
                    .sqrt();
                let sim = if dnorm > 0.0 {
                    (dot / (qnorm * dnorm)).clamp(-1.0, 1.0)
                } else {
                    0.0
                };
                (n.id, sim)
            })
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        scored
    }

    /// Recompute and persist the confidence score for one entity. Returns the new score.
    pub fn rescore_entity(&mut self, id: i64) -> f32 {
        let rels: Vec<String> = self
            .adj
            .get(&id)
            .map(|v| v.iter().map(|(_, r, _)| r.clone()).collect())
            .unwrap_or_default();
        let score = if let Some(node) = self.nodes.get(&id) {
            crate::scorer::score_entity(node, &rels).overall
        } else {
            return 0.0;
        };
        if let Some(node) = self.nodes.get_mut(&id) {
            node.confidence = score;
        }
        score
    }

    pub fn get_entity(&self, id: i64) -> Option<&EntityNode> {
        self.nodes.get(&id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&EntityNode> {
        let lower = name.to_lowercase();
        self.nodes.values().find(|n| n.name.to_lowercase() == lower)
    }

    pub fn neighbors_of(&self, entity_id: i64) -> Vec<(i64, String, f32)> {
        self.adj.get(&entity_id).cloned().unwrap_or_default()
    }

    /// Return only the *outgoing* (directed) relations where `entity_id` is the source.
    /// Unlike `neighbors_of`, this reads the DB directly and never returns backward-traversal
    /// entries — so `Peter parent_of Yousuf` will appear on Peter's list but NOT on Yousuf's
    /// (Yousuf's `child_of Peter` DB record appears there instead).
    /// Used by the Obsidian exporter for semantically-correct directed relation display.
    pub fn outgoing_relations(&self, entity_id: i64) -> Result<Vec<(i64, String, f32, usize)>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(RELATIONS_TABLE)?;
        let prefix: Vec<u8> = entity_id.to_le_bytes().to_vec();
        let mut out = Vec::new();
        for item in table.iter()? {
            let (k, v) = item?;
            if k.value().starts_with(&prefix) {
                if let Ok(rel) = serde_json::from_slice::<RelationRecord>(v.value()) {
                    let evid = rel.evidence_chunk_ids.len();
                    out.push((rel.dst_id, rel.relation_type, rel.strength, evid));
                }
            }
        }
        Ok(out)
    }

    /// Returns true when the relation was planted by the family-tree seed (evidence_chunk_id == 0).
    /// LLM-extracted-only relations have real chunk IDs; seeded relations always include 0.
    pub fn is_relation_seeded(&self, src_id: i64, dst_id: i64, rel_type: &str) -> bool {
        let key = relation_key(src_id, dst_id, rel_type);
        let Ok(rtxn) = self.db.begin_read() else {
            return false;
        };
        let Ok(table) = rtxn.open_table(RELATIONS_TABLE) else {
            return false;
        };
        table
            .get(key.as_slice())
            .ok()
            .flatten()
            .and_then(|v| serde_json::from_slice::<RelationRecord>(v.value()).ok())
            .map(|r| r.evidence_chunk_ids.contains(&0))
            .unwrap_or(false)
    }

    /// For each `spouse_of` pair, identify the female entity (by gender field) and search
    /// for other Person entities that share her first name but carry a different surname.
    /// Those are candidate pre-marriage (maiden name) forms of the same person.
    ///
    /// Requires `sanitize_relations` to have been run first so that `gender` fields are
    /// populated from pronoun cues in entity descriptions.
    pub fn infer_maiden_name_candidates(&self) -> Result<Vec<(i64, i64, String)>> {
        let spouse_rels: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .filter(|r| r.relation_type == "spouse_of")
                .collect()
        };

        let mut seen_pairs: std::collections::HashSet<(i64, i64)> =
            std::collections::HashSet::new();
        let mut candidates: Vec<(i64, i64, String)> = Vec::new();

        for rel in &spouse_rels {
            let pair = if rel.src_id < rel.dst_id {
                (rel.src_id, rel.dst_id)
            } else {
                (rel.dst_id, rel.src_id)
            };
            if !seen_pairs.insert(pair) {
                continue;
            }

            let a = match self.nodes.get(&rel.src_id) {
                Some(n) if n.entity_type.to_lowercase() == "person" => n,
                _ => continue,
            };
            let b = match self.nodes.get(&rel.dst_id) {
                Some(n) if n.entity_type.to_lowercase() == "person" => n,
                _ => continue,
            };

            // Identify the female entity using the gender field set by sanitize_relations.
            // Surname-sharing heuristics are not used — they fail when both spouses carry
            // the same surname (the common case when the wife takes the husband's name).
            let (female, male) = {
                let ga = a.gender.as_deref();
                let gb = b.gender.as_deref();
                if ga == Some("Female") && gb != Some("Female") {
                    (a, b)
                } else if gb == Some("Female") && ga != Some("Female") {
                    (b, a)
                } else {
                    // Gender unknown for both or same gender — skip; run sanitize first.
                    continue;
                }
            };

            // Female's first name token
            let first_name = match female.name.split_whitespace().next() {
                Some(f) if f.len() >= 3 => f.to_lowercase(),
                _ => continue,
            };

            // Male's surname token (last word)
            let male_surname = match male.name.split_whitespace().last() {
                Some(s) if s.len() >= 3 => s.to_lowercase(),
                _ => continue,
            };

            // Search for Person entities sharing the female's first name but NOT having the
            // husband's surname — these are potential pre-marriage (maiden name) forms.
            for node in self.nodes.values() {
                if node.id == female.id || node.id == male.id {
                    continue;
                }
                if node.entity_type.to_lowercase() != "person" {
                    continue;
                }
                let node_tokens: Vec<String> = node
                    .name
                    .split_whitespace()
                    .map(|w| w.to_lowercase())
                    .collect();
                let node_first = node_tokens.first().map(|s| s.as_str()).unwrap_or("");
                if node_first != first_name {
                    continue;
                }
                // Must NOT contain the husband's surname (that would be the same married form)
                if node_tokens.contains(&male_surname) {
                    continue;
                }
                let reason = format!(
                    "{} (maiden) ← first name '{}' matches {} (married name, spouse of {})",
                    node.name, first_name, female.name, male.name
                );
                candidates.push((female.id, node.id, reason));
            }
        }

        Ok(candidates)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn relation_count(&self) -> usize {
        self.adj.values().map(|v| v.len()).sum::<usize>() / 2
    }

    pub fn all_entities(&self) -> impl Iterator<Item = &EntityNode> {
        self.nodes.values()
    }

    /// Sync entity.evidence from entity_to_chunks for all in-memory nodes.
    /// During a live build session, link_chunk() updates entity_to_chunks but the
    /// EntityNode.evidence fields are not updated in-place. Call this before any
    /// code that reads entity.evidence (e.g. EC refinement, confidence scoring).
    pub fn sync_evidence(&mut self) {
        for (&eid, cids) in &self.entity_to_chunks {
            if let Some(node) = self.nodes.get_mut(&eid) {
                node.evidence = cids.clone();
            }
        }
    }

    /// Compute score_entity() for every node, write `confidence` back in-memory and
    /// persist the updated records to redb. Called once at the end of every CC build.
    pub fn score_all_confidences(&mut self) -> Result<()> {
        let ids: Vec<i64> = self.nodes.keys().copied().collect();
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            for id in &ids {
                let rels: Vec<String> = self
                    .adj
                    .get(id)
                    .map(|v| v.iter().map(|(_, r, _)| r.clone()).collect())
                    .unwrap_or_default();
                if let Some(node) = self.nodes.get_mut(id) {
                    let score = crate::scorer::score_entity(node, &rels);
                    node.confidence = score.overall;
                    let val = serde_json::to_vec(node)?;
                    t.insert(id.to_le_bytes().as_ref(), val.as_slice())?;
                }
            }
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Return the current strength of a specific relation, or None if it doesn't exist.
    /// Delete a specific relation (and its logical inverse/symmetric counterpart) from the graph.
    /// Returns true if the relation existed and was removed, false if it wasn't found.
    pub fn delete_relation(&mut self, src_id: i64, dst_id: i64, rel_type: &str) -> Result<bool> {
        let key = relation_key(src_id, dst_id, rel_type);
        let wtxn = self.db.begin_write()?;
        let mut removed = false;
        {
            let mut t = wtxn.open_table(RELATIONS_TABLE)?;
            if t.remove(key.as_slice())?.is_some() {
                removed = true;
            }
            // Also remove logical inverse (e.g. child_of ↔ parent_of)
            if let Some(&inv) = FAMILIAL_INVERSE
                .iter()
                .find(|(r, _)| *r == rel_type)
                .map(|(_, i)| i)
            {
                let inv_key = relation_key(dst_id, src_id, inv);
                t.remove(inv_key.as_slice())?;
            }
            // Also remove symmetric reverse (e.g. sibling_of, spouse_of stored both directions)
            if matches!(
                rel_type,
                "spouse_of" | "sibling_of" | "half_sibling_of" | "cousin_of"
            ) {
                let sym_key = relation_key(dst_id, src_id, rel_type);
                t.remove(sym_key.as_slice())?;
            }
        }
        wtxn.commit()?;
        if removed {
            // Update in-memory adj
            if let Some(edges) = self.adj.get_mut(&src_id) {
                edges.retain(|(nbr, rel, _)| !(*nbr == dst_id && rel == rel_type));
            }
            if let Some(edges) = self.adj.get_mut(&dst_id) {
                edges.retain(|(nbr, rel, _)| !(*nbr == src_id && rel == rel_type));
            }
        }
        Ok(removed)
    }

    pub fn get_relation_strength(&self, src_id: i64, dst_id: i64, rel_type: &str) -> Option<f32> {
        self.adj
            .get(&src_id)?
            .iter()
            .find(|(nbr, rel, _)| *nbr == dst_id && rel == rel_type)
            .map(|(_, _, s)| *s)
    }

    /// Return candidate Person entities for coreference resolution for a given chunk.
    ///
    /// Collects the entity sets for the chunk itself plus up to `window` adjacent
    /// chunks (by iterating `chunk_to_entities` for nearby chunk IDs), then returns
    /// the unique Person entities as `(name, aliases, gender)` triples.
    ///
    /// Also always includes any entity whose aliases contain "narrator", "author",
    /// or "i" — these are high-priority coref targets regardless of window.
    pub fn coref_candidates_for_chunk(
        &self,
        chunk_id: i64,
        adjacent_chunk_ids: &[i64],
    ) -> Vec<(String, Vec<String>, Option<String>)> {
        let mut seen: HashSet<i64> = HashSet::new();
        let mut candidates: Vec<(String, Vec<String>, Option<String>)> = Vec::new();

        // Always include the narrator/author entity. Force gender to Some("Male") for
        // entities whose aliases include "narrator"/"author"/"I" — the narrator in this
        // corpus is Yousuf Rassool, a man. The stored gender field may be wrong (inferred
        // from an impoverished description), so we override it here.
        for node in self.nodes.values() {
            if node.entity_type.eq_ignore_ascii_case("person")
                && node.aliases.iter().any(|a| {
                    matches!(
                        a.to_lowercase().as_str(),
                        "narrator" | "author" | "i" | "the author" | "the narrator"
                    )
                })
                && seen.insert(node.id)
            {
                candidates.push((
                    node.name.clone(),
                    node.aliases.clone(),
                    Some("Male".to_string()), // narrator is always Male in this corpus
                ));
            }
        }

        // Collect entities from this chunk + adjacent chunks
        let all_chunk_ids = std::iter::once(&chunk_id).chain(adjacent_chunk_ids.iter());
        for &cid in all_chunk_ids {
            if let Some(entity_ids) = self.chunk_to_entities.get(&cid) {
                for &eid in entity_ids {
                    if seen.contains(&eid) {
                        continue;
                    }
                    if let Some(node) = self.nodes.get(&eid) {
                        if node.entity_type.eq_ignore_ascii_case("person") {
                            seen.insert(eid);
                            candidates.push((
                                node.name.clone(),
                                node.aliases.clone(),
                                node.gender.clone(),
                            ));
                        }
                    }
                }
            }
        }
        candidates
    }

    /// Gather entity candidates of a specific type (Place, Organization, …) for coref.
    /// Returns `(name, aliases)` pairs — no gender field, unlike person candidates.
    pub fn coref_typed_candidates_for_chunk(
        &self,
        chunk_id: i64,
        adjacent_chunk_ids: &[i64],
        entity_type: &str,
    ) -> Vec<(String, Vec<String>)> {
        let mut seen: HashSet<i64> = HashSet::new();
        let mut candidates: Vec<(String, Vec<String>)> = Vec::new();
        let all_chunk_ids = std::iter::once(&chunk_id).chain(adjacent_chunk_ids.iter());
        for &cid in all_chunk_ids {
            if let Some(entity_ids) = self.chunk_to_entities.get(&cid) {
                for &eid in entity_ids {
                    if seen.contains(&eid) {
                        continue;
                    }
                    if let Some(node) = self.nodes.get(&eid) {
                        if node.entity_type.eq_ignore_ascii_case(entity_type) {
                            seen.insert(eid);
                            candidates.push((node.name.clone(), node.aliases.clone()));
                        }
                    }
                }
            }
        }
        candidates
    }

    pub fn chunks_for_entity(&self, entity_id: i64) -> &[i64] {
        self.entity_to_chunks
            .get(&entity_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Case-insensitive name match after stripping non-alphanumeric chars.
    /// Matches "J.M.H. Gool" == "JMH Gool", "Abdul Hamid (BG)" == "Abdul Hamid BG", etc.
    pub fn find_by_name_normalized(&self, name: &str) -> Option<&EntityNode> {
        let norm = normalize_name(name);
        self.nodes
            .values()
            .find(|n| normalize_name(&n.name) == norm)
    }

    /// Return all entity IDs whose normalized name (or any alias) contains `token` as a whole word.
    /// Used to augment embedding-based seed search with query name-token matching.
    pub fn find_ids_by_name_token(&self, token: &str) -> Vec<i64> {
        if token.len() < 3 {
            return vec![];
        }
        let token_lc = token.to_lowercase();
        self.nodes
            .values()
            .filter(|n| {
                let name_match = normalize_name(&n.name)
                    .split_whitespace()
                    .any(|w| w == token_lc);
                let alias_match = n
                    .aliases
                    .iter()
                    .any(|a| normalize_name(a).split_whitespace().any(|w| w == token_lc));
                name_match || alias_match
            })
            .map(|n| n.id)
            .collect()
    }

    /// Look up entity IDs by a raw (non-normalized) lowercased token.
    /// Hits the pre-built alias_token_index which stores tokens exactly as they appear
    /// in alias strings (e.g. "j.m.h." from "J.M.H. Gool"), so abbreviations with
    /// internal punctuation are found without stripping.  Returns an empty slice for
    /// tokens shorter than 2 characters.
    pub fn find_ids_by_alias_token(&self, token: &str) -> &[i64] {
        if token.len() < 2 {
            return &[];
        }
        self.alias_token_index
            .get(token)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Merge all relations from `alias_id` into `canonical_id` and delete the alias entity.
    ///
    /// After this call `alias_id` no longer exists in the graph. The in-memory `nodes`
    /// map is updated immediately (alias removed, canonical mention_count bumped) so
    /// subsequent `find_by_name` calls within the same session work correctly. Call
    /// `rebuild_in_memory()` once after a batch of merges to fully resync the adjacency list.
    pub fn merge_entity_into(&mut self, alias_id: i64, canonical_id: i64) -> Result<usize> {
        if alias_id == canonical_id {
            return Ok(0);
        }

        let alias_mention_count = self
            .nodes
            .get(&alias_id)
            .map(|n| n.mention_count)
            .unwrap_or(0);
        let alias_name = self.nodes.get(&alias_id).map(|n| n.name.clone());
        let alias_aliases = self
            .nodes
            .get(&alias_id)
            .map(|n| n.aliases.clone())
            .unwrap_or_default();
        let alias_description = self
            .nodes
            .get(&alias_id)
            .map(|n| n.description.clone())
            .unwrap_or_default();

        // ── 1. Collect relations involving alias_id ─────────────────────────
        let mut to_rewrite: Vec<RelationRecord> = vec![];
        {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            for entry in table.iter()? {
                let (_, v) = entry?;
                let rel: RelationRecord = serde_json::from_slice(v.value())?;
                if rel.src_id == alias_id || rel.dst_id == alias_id {
                    to_rewrite.push(rel);
                }
            }
        }
        let n_moved = to_rewrite.len();

        // ── 2. Rewrite relations in one transaction ─────────────────────────
        {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for rel in &to_rewrite {
                    let old_key = relation_key(rel.src_id, rel.dst_id, &rel.relation_type);
                    t.remove(old_key.as_slice())?;

                    let new_src = if rel.src_id == alias_id {
                        canonical_id
                    } else {
                        rel.src_id
                    };
                    let new_dst = if rel.dst_id == alias_id {
                        canonical_id
                    } else {
                        rel.dst_id
                    };
                    if new_src == new_dst {
                        continue; // skip self-loops
                    }
                    let new_key = relation_key(new_src, new_dst, &rel.relation_type);
                    let merged = match t.get(new_key.as_slice())? {
                        Some(v) => {
                            let mut e: RelationRecord = serde_json::from_slice(v.value())?;
                            for cid in &rel.evidence_chunk_ids {
                                if !e.evidence_chunk_ids.contains(cid) {
                                    e.evidence_chunk_ids.push(*cid);
                                }
                            }
                            e.strength = (e.evidence_chunk_ids.len() as f32 / 10.0).min(1.0);
                            e
                        }
                        None => RelationRecord {
                            src_id: new_src,
                            dst_id: new_dst,
                            relation_type: rel.relation_type.clone(),
                            strength: rel.strength,
                            evidence_chunk_ids: rel.evidence_chunk_ids.clone(),
                        },
                    };
                    t.insert(new_key.as_slice(), serde_json::to_vec(&merged)?.as_slice())?;
                }
            }
            wtxn.commit()?;
        }

        // ── 3. Delete alias entity + boost canonical mention_count + transfer chunk refs ──
        {
            let wtxn = self.db.begin_write()?;
            {
                let mut et = wtxn.open_table(ENTITIES_TABLE)?;
                let mut ec_tbl = wtxn.open_table(ENTITY_CHUNK_TABLE)?;
                let mut ce_tbl = wtxn.open_table(CHUNK_ENTITY_TABLE)?;

                // Transfer chunk references: alias → canonical
                let alias_chunk_ids: Vec<i64> = {
                    let ek = alias_id.to_le_bytes();
                    match ec_tbl.get(ek.as_ref())? {
                        Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                        None => vec![],
                    }
                };
                if !alias_chunk_ids.is_empty() {
                    // Extend canonical's chunk list
                    let canonical_ek = canonical_id.to_le_bytes();
                    let mut canonical_cids: Vec<i64> = match ec_tbl.get(canonical_ek.as_ref())? {
                        Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                        None => vec![],
                    };
                    for &cid in &alias_chunk_ids {
                        if !canonical_cids.contains(&cid) {
                            canonical_cids.push(cid);
                        }
                    }
                    ec_tbl.insert(
                        canonical_ek.as_ref(),
                        serde_json::to_vec(&canonical_cids)?.as_slice(),
                    )?;
                    // Replace alias_id with canonical_id in each chunk's entity list
                    for &cid in &alias_chunk_ids {
                        let ck = cid.to_le_bytes();
                        let mut eids: Vec<i64> = match ce_tbl.get(ck.as_ref())? {
                            Some(v) => serde_json::from_slice(v.value()).unwrap_or_default(),
                            None => vec![],
                        };
                        eids.retain(|&id| id != alias_id);
                        if !eids.contains(&canonical_id) {
                            eids.push(canonical_id);
                        }
                        ce_tbl.insert(ck.as_ref(), serde_json::to_vec(&eids)?.as_slice())?;
                    }
                }
                // Remove alias from entity-chunk index
                ec_tbl.remove(&alias_id.to_le_bytes()[..])?;

                // Remove alias entity and update canonical
                et.remove(&alias_id.to_le_bytes()[..])?;
                if let Some(canonical) = self.nodes.get(&canonical_id).cloned() {
                    let mut new_aliases = canonical.aliases.clone();
                    if let Some(ref aname) = alias_name {
                        if !new_aliases.contains(aname) {
                            new_aliases.push(aname.clone());
                        }
                    }
                    for a in &alias_aliases {
                        if !new_aliases.contains(a) {
                            new_aliases.push(a.clone());
                        }
                    }
                    let merged_description =
                        if alias_description.len() > canonical.description.len() {
                            alias_description.clone()
                        } else {
                            canonical.description.clone()
                        };
                    let updated = EntityNode {
                        mention_count: canonical.mention_count + alias_mention_count,
                        aliases: new_aliases,
                        description: merged_description,
                        ..canonical
                    };
                    et.insert(
                        &canonical_id.to_le_bytes()[..],
                        serde_json::to_vec(&updated)?.as_slice(),
                    )?;
                }
            }
            wtxn.commit()?;
        }

        // ── 4. Update in-memory state immediately ───────────────────────────
        self.nodes.remove(&alias_id);
        if let Some(node) = self.nodes.get_mut(&canonical_id) {
            node.mention_count += alias_mention_count;
            if alias_description.len() > node.description.len() {
                node.description = alias_description;
            }
        }
        // Transfer chunk refs in-memory so callers don't see stale state before rebuild_in_memory()
        let alias_mem_chunks = self.entity_to_chunks.remove(&alias_id).unwrap_or_default();
        for cid in &alias_mem_chunks {
            if let Some(eids) = self.chunk_to_entities.get_mut(cid) {
                eids.retain(|&id| id != alias_id);
                if !eids.contains(&canonical_id) {
                    eids.push(canonical_id);
                }
            }
            let ec = self.entity_to_chunks.entry(canonical_id).or_default();
            if !ec.contains(cid) {
                ec.push(*cid);
            }
        }
        // Keep entity.evidence in sync
        if let Some(node) = self.nodes.get_mut(&canonical_id) {
            for cid in alias_mem_chunks {
                if !node.evidence.contains(&cid) {
                    node.evidence.push(cid);
                }
            }
        }
        // Leave adj stale — caller should call rebuild_in_memory() after a batch.

        Ok(n_moved)
    }

    /// Infer gender from pronouns in the entity's description.
    /// Returns "Male" for he/his/him, "Female" for she/her/hers, None if ambiguous.
    pub fn infer_gender(description: &str) -> Option<String> {
        let text = description.to_lowercase();
        let male_cues = ["he ", "his ", "him ", " he,", " his,", " him,"];
        let female_cues = ["she ", "her ", "hers ", " she,", " her,", " hers,"];
        let m = male_cues.iter().filter(|&&c| text.contains(c)).count();
        let f = female_cues.iter().filter(|&&c| text.contains(c)).count();
        match (m, f) {
            (0, 0) => None,
            (m, f) if m > f => Some("Male".to_string()),
            (m, f) if f > m => Some("Female".to_string()),
            _ => None, // tie → ambiguous
        }
    }

    /// Sanitize all relations in the graph:
    /// 1. Remove familial relations where either endpoint is not a Person entity.
    /// 2. Add missing logical inverses for asymmetric familial relations.
    /// 3. Recompute relation strength from actual shared-evidence-chunk count.
    /// 4. Infer and store gender for all Person entities from their descriptions.
    /// 5. Flag (log) spouse_of pairs where inferred genders match (non-heteronormative or error).
    ///
    /// Returns (removed, added, recomputed, gendered) counts.
    pub fn sanitize_relations(&mut self) -> Result<(usize, usize, usize, usize)> {
        let mut removed = 0usize;
        let mut added = 0usize;
        let mut recomputed = 0usize;
        let mut gendered = 0usize;

        // ── Step 1: Collect all current relations from DB ──
        let all_rels: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .collect()
        };

        // ── Step 2: Remove type-invalid familial relations ──
        let mut to_delete: Vec<Vec<u8>> = Vec::new();
        let mut to_keep: Vec<RelationRecord> = Vec::new();
        for rel in &all_rels {
            if FAMILIAL_RELS.contains(&rel.relation_type.as_str()) {
                let src_person = self
                    .nodes
                    .get(&rel.src_id)
                    .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                    .unwrap_or(false);
                let dst_person = self
                    .nodes
                    .get(&rel.dst_id)
                    .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                    .unwrap_or(false);
                if !src_person || !dst_person {
                    to_delete.push(relation_key(rel.src_id, rel.dst_id, &rel.relation_type));
                    removed += 1;
                    continue;
                }
            }
            to_keep.push(rel.clone());
        }

        if !to_delete.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for k in &to_delete {
                    t.remove(k.as_slice())?;
                }
            }
            wtxn.commit()?;
        }

        // ── Step 3: Add missing inverses for asymmetric familial relations ──
        // Build a set of existing (src, dst, type) keys for O(1) lookup
        let existing: std::collections::HashSet<(i64, i64, String)> = to_keep
            .iter()
            .map(|r| (r.src_id, r.dst_id, r.relation_type.clone()))
            .collect();

        for rel in &to_keep {
            if let Some(&inverse) = FAMILIAL_INVERSE
                .iter()
                .find(|(r, _)| *r == rel.relation_type.as_str())
                .map(|(_, inv)| inv)
            {
                if !existing.contains(&(rel.dst_id, rel.src_id, inverse.to_string())) {
                    // Use each evidence chunk so strength is inherited from the forward relation
                    for &cid in &rel.evidence_chunk_ids {
                        self.upsert_relation_unchecked(rel.dst_id, rel.src_id, inverse, cid)?;
                    }
                    added += 1;
                }
            }
            // Symmetric: ensure both directions exist
            if matches!(
                rel.relation_type.as_str(),
                "spouse_of" | "sibling_of" | "half_sibling_of" | "cousin_of"
            ) && !existing.contains(&(rel.dst_id, rel.src_id, rel.relation_type.clone()))
            {
                for &cid in &rel.evidence_chunk_ids {
                    self.upsert_relation_unchecked(
                        rel.dst_id,
                        rel.src_id,
                        &rel.relation_type,
                        cid,
                    )?;
                }
                added += 1;
            }
        }

        // ── Step 4: Recompute strength from shared evidence chunks ──
        // For each relation, strength = min(1.0, shared_chunks / 10.0) where shared_chunks =
        // number of chunks that mention BOTH the src and dst entity.
        let all_rels_fresh: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .collect()
        };

        let mut strength_updates: Vec<(Vec<u8>, RelationRecord)> = Vec::new();
        for mut rel in all_rels_fresh {
            let src_chunks: std::collections::HashSet<i64> =
                self.chunks_for_entity(rel.src_id).iter().copied().collect();
            let dst_chunks: std::collections::HashSet<i64> =
                self.chunks_for_entity(rel.dst_id).iter().copied().collect();
            let shared: usize = src_chunks.intersection(&dst_chunks).count();
            let new_strength = if shared > 0 {
                (shared as f32 / 10.0).min(1.0)
            } else {
                // No shared chunks (e.g. dream-added relation) — use evidence_chunk_ids count
                (rel.evidence_chunk_ids.len() as f32 / 10.0).clamp(0.1, 1.0)
            };
            if (new_strength - rel.strength).abs() > 0.001 {
                rel.strength = new_strength;
                let key = relation_key(rel.src_id, rel.dst_id, &rel.relation_type);
                strength_updates.push((key, rel));
                recomputed += 1;
            }
        }

        if !strength_updates.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for (k, rel) in &strength_updates {
                    t.insert(k.as_slice(), serde_json::to_vec(rel)?.as_slice())?;
                }
            }
            wtxn.commit()?;
        }

        // ── Step 5: Infer and store gender for Person entities ──
        let person_ids: Vec<i64> = self
            .nodes
            .values()
            .filter(|n| n.entity_type.eq_ignore_ascii_case("person"))
            .map(|n| n.id)
            .collect();

        for id in person_ids {
            if let Some(node) = self.nodes.get(&id) {
                if node.gender.is_some() {
                    continue; // already set
                }
                if let Some(g) = Self::infer_gender(&node.description) {
                    let mut updated = node.clone();
                    updated.gender = Some(g);
                    gendered += 1;
                    self.upsert_entity(updated)?;
                }
            }
        }

        // ── Step 6: Delete suspect spouse_of pairs (both endpoints same inferred gender) ──
        // These are virtually always LLM hallucinations — the model confuses "associated with"
        // or "met" for "married to".  Pairs where gender is unknown on either side are kept.
        let spouse_rels: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .filter(|r| r.relation_type == "spouse_of")
                .collect()
        };

        let mut spouses_purged = 0usize;
        let mut suspect_keys: Vec<Vec<u8>> = Vec::new();

        // Collect unique pairs (avoid double-counting A↔B and B↔A)
        let mut seen_pairs: std::collections::HashSet<(i64, i64)> =
            std::collections::HashSet::new();
        for rel in &spouse_rels {
            let pair = if rel.src_id < rel.dst_id {
                (rel.src_id, rel.dst_id)
            } else {
                (rel.dst_id, rel.src_id)
            };
            if seen_pairs.contains(&pair) {
                continue;
            }
            seen_pairs.insert(pair);

            let ga = self
                .nodes
                .get(&rel.src_id)
                .and_then(|n| n.gender.as_deref());
            let gb = self
                .nodes
                .get(&rel.dst_id)
                .and_then(|n| n.gender.as_deref());
            if let (Some(ga), Some(gb)) = (ga, gb) {
                if ga == gb {
                    let na = self
                        .nodes
                        .get(&rel.src_id)
                        .map(|n| n.name.as_str())
                        .unwrap_or("?");
                    let nb = self
                        .nodes
                        .get(&rel.dst_id)
                        .map(|n| n.name.as_str())
                        .unwrap_or("?");
                    tracing::warn!(
                        "removing suspect spouse_of: {} ({}) ↔ {} ({}) — same gender (likely hallucination)",
                        na, ga, nb, gb
                    );
                    suspect_keys.push(relation_key(rel.src_id, rel.dst_id, "spouse_of"));
                    suspect_keys.push(relation_key(rel.dst_id, rel.src_id, "spouse_of"));
                    spouses_purged += 1;
                }
            }
        }

        // ── Also supersede half_sibling_of when sibling_of exists for same pair ──
        // If the seed confirms a full sibling relationship, the dream-added half_sibling_of
        // is always wrong for that pair.
        {
            let all_sibling_rels: Vec<RelationRecord> = {
                let rtxn = self.db.begin_read()?;
                let table = rtxn.open_table(RELATIONS_TABLE)?;
                table
                    .iter()?
                    .filter_map(|r| r.ok())
                    .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                    .filter(|r| {
                        r.relation_type == "sibling_of" || r.relation_type == "half_sibling_of"
                    })
                    .collect()
            };
            let sibling_pairs: std::collections::HashSet<(i64, i64)> = all_sibling_rels
                .iter()
                .filter(|r| r.relation_type == "sibling_of")
                .map(|r| (r.src_id.min(r.dst_id), r.src_id.max(r.dst_id)))
                .collect();
            for rel in &all_sibling_rels {
                if rel.relation_type == "half_sibling_of" {
                    let pair = (rel.src_id.min(rel.dst_id), rel.src_id.max(rel.dst_id));
                    if sibling_pairs.contains(&pair) {
                        suspect_keys.push(relation_key(rel.src_id, rel.dst_id, "half_sibling_of"));
                        spouses_purged += 1;
                    }
                }
            }
        }

        if !suspect_keys.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for k in &suspect_keys {
                    let _ = t.remove(k.as_slice());
                }
            }
            wtxn.commit()?;
            removed += spouses_purged * 2;
        }

        // ── Step 7: Remove parent_of paradoxes ──────────────────────────────────
        // If A parent_of B AND B parent_of A both exist, one must be wrong (a person
        // cannot be both parent and child of the same person).  Resolve by checking
        // descriptions for "son of / daughter of / child of <other>"; the entity whose
        // description names the other as a parent is the child — so its parent_of edge
        // pointing back at the other is the bogus one.  If descriptions are ambiguous,
        // keep the edge whose src has the higher mention_count (more-documented entity
        // is more likely the main subject whose relations were written correctly).
        let all_parent_rels: Vec<(i64, i64)> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .filter(|r| r.relation_type == "parent_of")
                .map(|r| (r.src_id, r.dst_id))
                .collect()
        };

        let parent_set: std::collections::HashSet<(i64, i64)> =
            all_parent_rels.iter().cloned().collect();
        let mut paradox_deletes: Vec<Vec<u8>> = Vec::new();
        let mut paradoxes_fixed = 0usize;

        for &(a, b) in &all_parent_rels {
            if parent_set.contains(&(b, a)) && a < b {
                // Both A parent_of B and B parent_of A — paradox.
                // Determine which is the child by checking descriptions.
                let a_desc = self
                    .nodes
                    .get(&a)
                    .map(|n| n.description.to_lowercase())
                    .unwrap_or_default();
                let b_name_lc = self
                    .nodes
                    .get(&b)
                    .map(|n| n.name.to_lowercase())
                    .unwrap_or_default();
                let b_desc = self
                    .nodes
                    .get(&b)
                    .map(|n| n.description.to_lowercase())
                    .unwrap_or_default();
                let a_name_lc = self
                    .nodes
                    .get(&a)
                    .map(|n| n.name.to_lowercase())
                    .unwrap_or_default();

                // Does A's description say it is a child of B?
                let a_is_child =
                    ["son of", "daughter of", "child of", "born to"]
                        .iter()
                        .any(|&cue| {
                            a_desc.contains(cue)
                                && (a_desc.contains(&b_name_lc)
                                    || b_name_lc
                                        .split_whitespace()
                                        .any(|tok| tok.len() >= 4 && a_desc.contains(tok)))
                        });
                // Does B's description say it is a child of A?
                let b_is_child =
                    ["son of", "daughter of", "child of", "born to"]
                        .iter()
                        .any(|&cue| {
                            b_desc.contains(cue)
                                && (b_desc.contains(&a_name_lc)
                                    || a_name_lc
                                        .split_whitespace()
                                        .any(|tok| tok.len() >= 4 && b_desc.contains(tok)))
                        });

                let (wrong_src, wrong_dst) = match (a_is_child, b_is_child) {
                    (true, false) => (a, b), // A is the child → A parent_of B is wrong
                    (false, true) => (b, a), // B is the child → B parent_of A is wrong
                    _ => {
                        // Ambiguous — keep the edge from the higher-mention entity (main subject)
                        let mc_a = self.nodes.get(&a).map(|n| n.mention_count).unwrap_or(0);
                        let mc_b = self.nodes.get(&b).map(|n| n.mention_count).unwrap_or(0);
                        if mc_a >= mc_b {
                            (b, a)
                        } else {
                            (a, b)
                        }
                    }
                };

                let na = self
                    .nodes
                    .get(&wrong_src)
                    .map(|n| n.name.as_str())
                    .unwrap_or("?");
                let nb = self
                    .nodes
                    .get(&wrong_dst)
                    .map(|n| n.name.as_str())
                    .unwrap_or("?");
                tracing::warn!(
                    "parent_of paradox: removing bogus '{na}' parent_of '{nb}' \
                     (description indicates direction is reversed)"
                );
                paradox_deletes.push(relation_key(wrong_src, wrong_dst, "parent_of"));
                // Also remove the bogus child_of in the opposite direction if it was auto-added
                paradox_deletes.push(relation_key(wrong_dst, wrong_src, "child_of"));
                paradoxes_fixed += 1;
            }
        }

        if !paradox_deletes.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for k in &paradox_deletes {
                    let _ = t.remove(k.as_slice());
                }
            }
            wtxn.commit()?;
            removed += paradoxes_fixed * 2;
        }

        // ── Step 8: Remove type-mismatched work/location relations ──────────────
        // `works_at`, `employed_by`, `staffed_by`, `works_for` must target an
        // Organization or Place — not a Person.  The LLM commonly emits
        // `works_at → Person` when the text says "worked alongside <person>".
        //
        // `located_in`, `located_at`, `lives_in`, `settled_in`, `went_to` must
        // target a Place — not a Person or Organization.
        {
            const WORK_RELS: &[&str] = &[
                "works_at",
                "employed_by",
                "staffed_by",
                "works_for",
                "worked_at",
            ];
            const LOCATION_RELS: &[&str] = &[
                "located_in",
                "located_at",
                "lives_in",
                "settled_in",
                "went_to",
                "visited",
            ];

            let all_fresh: Vec<RelationRecord> = {
                let rtxn = self.db.begin_read()?;
                let table = rtxn.open_table(RELATIONS_TABLE)?;
                table
                    .iter()?
                    .filter_map(|r| r.ok())
                    .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                    .collect()
            };

            let mut type_mismatch_keys: Vec<Vec<u8>> = Vec::new();
            for rel in &all_fresh {
                let rtype = rel.relation_type.as_str();
                if WORK_RELS.contains(&rtype) {
                    let dst_is_person = self
                        .nodes
                        .get(&rel.dst_id)
                        .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                        .unwrap_or(false);
                    if dst_is_person {
                        let src = self
                            .nodes
                            .get(&rel.src_id)
                            .map(|n| n.name.as_str())
                            .unwrap_or("?");
                        let dst = self
                            .nodes
                            .get(&rel.dst_id)
                            .map(|n| n.name.as_str())
                            .unwrap_or("?");
                        tracing::warn!(
                            "type mismatch: removing '{}' {} '{}' — target is a Person, not an Organization",
                            src, rtype, dst
                        );
                        type_mismatch_keys.push(relation_key(rel.src_id, rel.dst_id, rtype));
                        removed += 1;
                    }
                }
                if LOCATION_RELS.contains(&rtype) {
                    let dst_type = self
                        .nodes
                        .get(&rel.dst_id)
                        .map(|n| n.entity_type.to_lowercase())
                        .unwrap_or_default();
                    // Allow: Place, Event, schema:Place, schema:Event — block Person and Organization
                    let dst_is_non_place = matches!(
                        dst_type.as_str(),
                        "person" | "organization" | "schema:person" | "schema:organization"
                    );
                    if dst_is_non_place {
                        let src = self
                            .nodes
                            .get(&rel.src_id)
                            .map(|n| n.name.as_str())
                            .unwrap_or("?");
                        let dst = self
                            .nodes
                            .get(&rel.dst_id)
                            .map(|n| n.name.as_str())
                            .unwrap_or("?");
                        tracing::warn!(
                            "type mismatch: removing '{}' {} '{}' — target is {}, not a Place",
                            src,
                            rtype,
                            dst,
                            dst_type
                        );
                        type_mismatch_keys.push(relation_key(rel.src_id, rel.dst_id, rtype));
                        removed += 1;
                    }
                }
            }

            if !type_mismatch_keys.is_empty() {
                let wtxn = self.db.begin_write()?;
                {
                    let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                    for k in &type_mismatch_keys {
                        let _ = t.remove(k.as_slice());
                    }
                }
                wtxn.commit()?;
            }
        }

        // ── Step 9: Prune honorific stub entities ──────────────────────────────
        // Entities whose name is a bare honorific/title fragment ("Dr.", "Mr.",
        // "Mrs.", "Prof.", "Sir", "Lady", "Rev.") with ≤5 chars are extraction
        // noise — the LLM failed to capture the actual name.  Remove all their
        // relations and then the entity itself.
        {
            const HONORIFICS: &[&str] = &[
                "dr", "mr", "mrs", "ms", "prof", "sir", "rev", "hon", "lady", "lord",
            ];
            let stub_ids: Vec<i64> = self
                .nodes
                .values()
                .filter(|n| {
                    let trimmed = n.name.trim_end_matches('.').to_lowercase();
                    HONORIFICS.contains(&trimmed.as_str()) && n.name.len() <= 6
                })
                .map(|n| n.id)
                .collect();

            if !stub_ids.is_empty() {
                let all_rels_for_stubs: Vec<RelationRecord> = {
                    let rtxn = self.db.begin_read()?;
                    let table = rtxn.open_table(RELATIONS_TABLE)?;
                    table
                        .iter()?
                        .filter_map(|r| r.ok())
                        .filter_map(|(_, v)| {
                            serde_json::from_slice::<RelationRecord>(v.value()).ok()
                        })
                        .filter(|r| stub_ids.contains(&r.src_id) || stub_ids.contains(&r.dst_id))
                        .collect()
                };

                let wtxn = self.db.begin_write()?;
                {
                    let mut rt = wtxn.open_table(RELATIONS_TABLE)?;
                    for r in &all_rels_for_stubs {
                        let _ = rt
                            .remove(relation_key(r.src_id, r.dst_id, &r.relation_type).as_slice());
                        removed += 1;
                    }
                    let mut et = wtxn.open_table(ENTITIES_TABLE)?;
                    for &id in &stub_ids {
                        if let Some(node) = self.nodes.get(&id) {
                            tracing::warn!("pruning honorific stub entity: '{}'", node.name);
                        }
                        et.remove(&id.to_le_bytes()[..])?;
                    }
                }
                wtxn.commit()?;

                for &id in &stub_ids {
                    self.nodes.remove(&id);
                }
            }
        }

        // Rebuild adjacency to reflect all changes
        self.rebuild_in_memory()?;

        Ok((removed, added, recomputed, gendered))
    }

    /// Return (alias_id, canonical_id) pairs where both entities have the same
    /// normalized name. These are unambiguous duplicates — always safe to auto-merge.
    /// Canonical is the entity with the higher mention_count.
    pub fn find_dedup_candidates_exact(&self) -> Vec<(i64, i64)> {
        let mut by_norm: HashMap<String, Vec<i64>> = HashMap::new();
        for (&id, node) in &self.nodes {
            by_norm
                .entry(normalize_name(&node.name))
                .or_default()
                .push(id);
        }
        let mut pairs = Vec::new();
        for ids in by_norm.values() {
            if ids.len() < 2 {
                continue;
            }
            let &canonical = ids
                .iter()
                .max_by_key(|&&id| self.nodes.get(&id).map(|n| n.mention_count).unwrap_or(0))
                .unwrap();
            for &alias in ids.iter().filter(|&&id| id != canonical) {
                pairs.push((alias, canonical));
            }
        }
        pairs
    }

    /// Return (alias_id, canonical_id, sim) triples where the pair shares ≥1 significant
    /// name token and has embedding cosine similarity ≥ threshold.
    /// Sorted by similarity descending. Exact-name matches are excluded (Tier 1 handles those).
    /// Canonical = longer name; tie-break by higher mention_count.
    pub fn find_dedup_candidates(&self, threshold: f32) -> Vec<(i64, i64, f32)> {
        const DEDUP_STOP: &[&str] = &[
            // articles / prepositions
            "the",
            "and",
            "of",
            "in",
            "a",
            "an",
            "for",
            "at",
            "by",
            "to",
            // honorifics
            "dr",
            "mr",
            "mrs",
            "ms",
            "prof",
            "sir",
            // geographic generics
            "north",
            "south",
            "east",
            "west",
            "new",
            "old",
            "cape",
            "great",
            "lower",
            "upper",
            "central",
            // institutional generics (the main false-match offenders)
            "union",
            "street",
            "road",
            "avenue",
            "lane",
            "school",
            "institute",
            "college",
            "university",
            "club",
            "party",
            "movement",
            "committee",
            "association",
            "council",
            "congress",
            "league",
            "society",
            "church",
            "hall",
            "high",
            "primary",
            "secondary",
            // common auxiliaries / pronouns
            "its",
            "was",
            "his",
            "her",
        ];
        let stop = DEDUP_STOP;

        let mut token_to_ids: HashMap<String, Vec<i64>> = HashMap::new();
        for (&id, node) in &self.nodes {
            for token in normalize_name(&node.name).split_whitespace() {
                if token.len() >= 3 && !stop.contains(&token) {
                    token_to_ids.entry(token.to_string()).or_default().push(id);
                }
            }
        }

        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut candidates: Vec<(i64, i64, f32)> = Vec::new();

        for ids in token_to_ids.values() {
            for i in 0..ids.len() {
                for j in (i + 1)..ids.len() {
                    let a = ids[i];
                    let b = ids[j];
                    let key = if a < b { (a, b) } else { (b, a) };
                    if !seen.insert(key) {
                        continue;
                    }
                    let (na, nb) = match (self.nodes.get(&a), self.nodes.get(&b)) {
                        (Some(x), Some(y)) => (x, y),
                        _ => continue,
                    };
                    // Gate A: cross-type pairs are never merged (Person ≠ Place, etc.)
                    if na.entity_type != nb.entity_type {
                        continue;
                    }
                    // Gate B: explicit disambiguation markers mean distinct individuals
                    if has_disambiguation_token(&na.name) || has_disambiguation_token(&nb.name) {
                        continue;
                    }
                    if normalize_name(&na.name) == normalize_name(&nb.name) {
                        continue; // exact matches handled by Tier 1
                    }
                    // Gate C: names must be string-similar before paying the cost of
                    // embedding comparison — prevents merging on description similarity alone.
                    const JW_DEDUP_GATE: f32 = 0.60;
                    if jaro_winkler(&normalize_name(&na.name), &normalize_name(&nb.name))
                        < JW_DEDUP_GATE
                    {
                        continue;
                    }
                    let mut sim = cosine_sim_f32(&na.embedding, &nb.embedding);
                    if sim < threshold {
                        continue;
                    }
                    // Guard: shared single-token dominance causes false high similarity.
                    // Three cases:
                    //   (a) Same surname: "Helen Abrahams" / "Hassen Abrahams".
                    //       Also catches hyphenated extensions: "Gool" / "Gool-Ebrahim".
                    //   (b) Same prefix/first-name: "Auntie Annie" / "Auntie Minnie",
                    //       "Cecil Rhodes" / "Cecil Wightman".  Uses 33% threshold so
                    //       short suffixes like "Annie"/"Minnie" (dist=2, max=6) are caught.
                    //   (c) 3-token names with no shared distinctive token (> 4 chars):
                    //       "Cecil John Rhodes" / "Rev John Phillips" — "John" (4 chars)
                    //       doesn't qualify, so they cap even though last tokens differ.
                    {
                        let wa: Vec<&str> = na.name.split_whitespace().collect();
                        let wb: Vec<&str> = nb.name.split_whitespace().collect();

                        // Blob guard: a 4+-token entity is likely a multi-person
                        // extraction artifact; don't auto-merge with a short name.
                        if (wa.len() >= 4) != (wb.len() >= 4) {
                            sim = sim.min(0.96);
                        }

                        // Gender guard: "Mr X" and "Mrs X" are different people.
                        if wa.len() >= 2 && wb.len() >= 2 {
                            let male_h = ["mr", "sir"];
                            let female_h = ["mrs", "miss", "ms"];
                            let ha = wa[0].to_lowercase();
                            let hb = wb[0].to_lowercase();
                            if (male_h.contains(&ha.as_str()) && female_h.contains(&hb.as_str()))
                                || (female_h.contains(&ha.as_str())
                                    && male_h.contains(&hb.as_str()))
                            {
                                sim = sim.min(0.96);
                            }
                        }

                        if wa.len() >= 2 && wb.len() >= 2 {
                            let first_a = wa[0].to_lowercase();
                            let first_b = wb[0].to_lowercase();
                            let last_a = wa.last().unwrap().to_lowercase();
                            let last_b = wb.last().unwrap().to_lowercase();

                            // (a) Same last token or hyphenated extension → surname dominates
                            let last_matches = last_a == last_b
                                || (last_b.contains('-') && last_b.starts_with(last_a.as_str()))
                                || (last_a.contains('-') && last_a.starts_with(last_b.as_str()));
                            if last_matches {
                                // Don't cap when one side has an abbreviated/initial first token
                                // (e.g. "A.H." or "AH" vs "Abdul Hamid") — let embedding decide.
                                let a_is_abbrev = first_a.len() <= 2 || first_a.contains('.');
                                let b_is_abbrev = first_b.len() <= 2 || first_b.contains('.');
                                if !a_is_abbrev && !b_is_abbrev {
                                    let max_len = first_a.len().max(first_b.len());
                                    let dist = levenshtein_distance(&first_a, &first_b);
                                    if max_len > 2 && dist * 2 >= max_len {
                                        sim = sim.min(0.96);
                                    }
                                }
                            }

                            // (b) Same first token → prefix/first-name dominates (33% threshold)
                            if first_a == first_b {
                                let max_len = last_a.len().max(last_b.len());
                                let dist = levenshtein_distance(&last_a, &last_b);
                                if max_len > 2 && dist * 3 >= max_len {
                                    sim = sim.min(0.96);
                                }
                            }

                            // (c) Both ≥ 3 tokens, first AND last differ, no shared
                            //     distinctive middle token → cap
                            if wa.len() >= 3
                                && wb.len() >= 3
                                && first_a != first_b
                                && last_a != last_b
                            {
                                let wa_set: std::collections::HashSet<String> =
                                    wa.iter().map(|s| s.to_lowercase()).collect();
                                let wb_set: std::collections::HashSet<String> =
                                    wb.iter().map(|s| s.to_lowercase()).collect();
                                let has_shared_distinctive =
                                    wa_set.intersection(&wb_set).any(|t| t.len() > 4);
                                if !has_shared_distinctive {
                                    sim = sim.min(0.96);
                                }
                            }

                            // (d) Cross-position shared token: "Hamid Khan" / "Abdul Hamid"
                            //     share "hamid" at last_a == first_b; "Dullah Omar" /
                            //     "Omar Khayyam" share "omar" at last_a == first_b.
                            //     These are almost always different people.
                            let cross_pos = (last_a.len() > 3 && last_a == first_b)
                                || (first_a.len() > 3 && first_a == last_b);
                            if cross_pos {
                                sim = sim.min(0.96);
                            }
                        }
                    }
                    // Guard: description divergence. When both entities have rich
                    // descriptions (post-enrich) but share very few significant
                    // words, their descriptions contradict → cap to prevent auto-merge.
                    if self.dedup_desc_diverges(a, b) {
                        sim = sim.min(0.94);
                    }
                    if sim < threshold {
                        continue;
                    }
                    // Canonical = longer name; tie-break by mention_count
                    let (alias_id, canonical_id) = if na.name.len() > nb.name.len()
                        || (na.name.len() == nb.name.len() && na.mention_count >= nb.mention_count)
                    {
                        (b, a)
                    } else {
                        (a, b)
                    };
                    candidates.push((alias_id, canonical_id, sim));
                }
            }
        }

        candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        candidates
    }

    /// Tier 3: Structural name-pattern dedup.
    ///
    /// Catches duplicates that embedding similarity misses because the names live in
    /// different semantic regions (role words, honorifics, partial names, OCR noise).
    ///
    /// Returns `(alias_id, canonical_id, reason)` where `reason` is one of:
    /// - `"honorific"` — same name after stripping titles (Dr., Haji, Auntie, …)
    /// - `"subset"` — one name's words are a strict subset of the other's AND they
    ///   share ≥ 2 distinct graph neighbours
    /// - `"fuzzy"` — full-name edit distance ≤ 2 within a shared-token bucket
    ///
    /// Canonical is the entity with the longer name; tie-broken by mention_count.
    pub fn find_dedup_candidates_name_structure(&self) -> Vec<(i64, i64, &'static str)> {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut out: Vec<(i64, i64, &'static str)> = Vec::new();

        // ── A: honorific-stripped exact match ────────────────────────────────
        {
            let mut key_map: HashMap<String, Vec<i64>> = HashMap::new();
            for (&id, node) in &self.nodes {
                let key = stripped_key(&node.name);
                if !key.is_empty() {
                    key_map.entry(key.clone()).or_default().push(id);
                }
            }
            for ids in key_map.values() {
                if ids.len() < 2 {
                    continue;
                }
                let &canonical = ids
                    .iter()
                    .max_by_key(|&&id| {
                        self.nodes
                            .get(&id)
                            .map(|n| (n.mention_count, n.name.len()))
                            .unwrap_or((0, 0))
                    })
                    .unwrap();
                for &alias in ids.iter().filter(|&&id| id != canonical) {
                    let na = match self.nodes.get(&alias) {
                        Some(n) => n,
                        None => continue,
                    };
                    let nc = match self.nodes.get(&canonical) {
                        Some(n) => n,
                        None => continue,
                    };
                    if normalize_name(&na.name) == normalize_name(&nc.name) {
                        continue; // Tier 1 handles exact matches
                    }
                    // Don't merge Mr X with Mrs X — different people.
                    let leading = |n: &str| {
                        normalize_name(n)
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_string()
                    };
                    let ha = leading(&na.name);
                    let hc = leading(&nc.name);
                    let male_h = ["mr", "sir"];
                    let female_h = ["mrs", "miss", "ms"];
                    if (male_h.contains(&ha.as_str()) && female_h.contains(&hc.as_str()))
                        || (female_h.contains(&ha.as_str()) && male_h.contains(&hc.as_str()))
                    {
                        continue;
                    }
                    // Gate: never merge across entity types (Person ≠ Place, etc.)
                    if na.entity_type != nc.entity_type {
                        continue;
                    }
                    let key = ord_pair(alias, canonical);
                    if seen.insert(key) {
                        out.push((alias, canonical, "honorific"));
                    }
                }
            }
        }

        // ── A2: canonical-name matches an alias of another entity ─────────────
        // Catches "Dr. A. H. Gool" (entity) whose stripped canonical "a h gool"
        // matches alias "A.H. Gool" carried on "Abdul Hamid Gool".
        // Directional: entity E's NAME → must appear as an ALIAS of entity F.
        // This avoids the false-positive of symmetric alias-key grouping, where
        // two entities sharing a surname alias ("Gool") wrongly end up merged.
        {
            // Build alias-stripped → canonical_id reverse map (one entry per alias)
            let mut alias_key_to_id: HashMap<String, i64> = HashMap::new();
            for (&id, node) in &self.nodes {
                for alias in &node.aliases {
                    let akey = stripped_key(alias);
                    if akey.is_empty() {
                        continue;
                    }
                    // Prefer the entity with more mentions when aliases collide
                    alias_key_to_id
                        .entry(akey)
                        .and_modify(|existing| {
                            let existing_mc = self
                                .nodes
                                .get(existing)
                                .map(|n| n.mention_count)
                                .unwrap_or(0);
                            let new_mc = node.mention_count;
                            if new_mc > existing_mc {
                                *existing = id;
                            }
                        })
                        .or_insert(id);
                }
            }

            for (&eid, node) in &self.nodes {
                let ekey = stripped_key(&node.name);
                if ekey.is_empty() {
                    continue;
                }
                let Some(&canonical_id) = alias_key_to_id.get(&ekey) else {
                    continue;
                };
                if canonical_id == eid {
                    continue; // same entity
                }
                // Don't merge if the canonical is already an alias of this entity
                if node.aliases.iter().any(|a| {
                    self.nodes
                        .get(&canonical_id)
                        .map(|n| n.name == *a)
                        .unwrap_or(false)
                }) {
                    continue;
                }
                // Require that the entity's own name normalises differently from canonical
                if let Some(nc) = self.nodes.get(&canonical_id) {
                    if normalize_name(&node.name) == normalize_name(&nc.name) {
                        continue; // Tier 1 handles exact matches
                    }
                    // Gate: never merge across entity types (Person ≠ Place, etc.)
                    if node.entity_type != nc.entity_type {
                        continue;
                    }
                }
                let pair = ord_pair(eid, canonical_id);
                if seen.insert(pair) {
                    out.push((eid, canonical_id, "alias_match"));
                }
            }
        }

        // ── B: subset-name with ≥ 2 shared neighbours ────────────────────────
        {
            // word (≥ 4 chars) → entity ids that have it in their name
            let mut word_ids: HashMap<String, Vec<i64>> = HashMap::new();
            for (&id, node) in &self.nodes {
                for w in normalize_name(&node.name).split_whitespace() {
                    if w.len() >= 4 {
                        word_ids.entry(w.to_string()).or_default().push(id);
                    }
                }
            }

            // entity → set of neighbour ids (undirected)
            let nbr_sets: HashMap<i64, HashSet<i64>> = self
                .adj
                .iter()
                .map(|(&id, edges)| {
                    (
                        id,
                        edges.iter().map(|(nbr, _, _)| *nbr).collect::<HashSet<_>>(),
                    )
                })
                .collect();

            for (&id, node) in &self.nodes {
                let my_words: Vec<String> = normalize_name(&node.name)
                    .split_whitespace()
                    .filter(|w| w.len() >= 4)
                    .map(|w| w.to_string())
                    .collect();
                if my_words.is_empty() {
                    continue;
                }

                // Intersect candidate sets across all my significant words
                let mut universe: Option<HashSet<i64>> = None;
                for w in &my_words {
                    let set: HashSet<i64> = word_ids
                        .get(w.as_str())
                        .map(|v| v.iter().copied().collect())
                        .unwrap_or_default();
                    universe = Some(match universe {
                        None => set,
                        Some(prev) => prev.intersection(&set).copied().collect(),
                    });
                }
                let universe = match universe {
                    Some(u) if !u.is_empty() => u,
                    _ => continue,
                };

                let my_all_word_count = normalize_name(&node.name).split_whitespace().count();
                let my_nbrs = nbr_sets.get(&id).cloned().unwrap_or_default();

                for other_id in universe {
                    if other_id == id {
                        continue;
                    }
                    let other = match self.nodes.get(&other_id) {
                        Some(n) => n,
                        None => continue,
                    };
                    let other_word_count = normalize_name(&other.name).split_whitespace().count();
                    // Only treat `id` as the subset (other must have strictly more words)
                    if other_word_count <= my_all_word_count {
                        continue;
                    }
                    if normalize_name(&node.name) == normalize_name(&other.name) {
                        continue;
                    }
                    // Require ≥ 2 shared neighbours as an identity signal
                    let other_nbrs = nbr_sets.get(&other_id).cloned().unwrap_or_default();
                    if my_nbrs.intersection(&other_nbrs).count() < 2 {
                        continue;
                    }
                    let key = ord_pair(id, other_id);
                    if seen.insert(key) {
                        out.push((id, other_id, "subset"));
                    }
                }
            }
        }

        // ── C: edit-distance ≤ 2 within shared-token bucket ─────────────────
        {
            let mut token_ids: HashMap<String, Vec<i64>> = HashMap::new();
            for (&id, node) in &self.nodes {
                for w in normalize_name(&node.name).split_whitespace() {
                    if w.len() >= 4 {
                        token_ids.entry(w.to_string()).or_default().push(id);
                    }
                }
            }
            let mut bucket_seen: HashSet<(i64, i64)> = HashSet::new();
            for ids in token_ids.values() {
                for i in 0..ids.len() {
                    for j in (i + 1)..ids.len() {
                        let (a, b) = (ids[i], ids[j]);
                        let key = ord_pair(a, b);
                        if !bucket_seen.insert(key) || seen.contains(&key) {
                            continue;
                        }
                        let (na, nb) = match (self.nodes.get(&a), self.nodes.get(&b)) {
                            (Some(x), Some(y)) => (x, y),
                            _ => continue,
                        };
                        let norm_a = normalize_name(&na.name);
                        let norm_b = normalize_name(&nb.name);
                        if norm_a == norm_b {
                            continue;
                        }
                        // Skip pairs whose lengths differ too much to have distance ≤ 2
                        let la = norm_a.chars().count();
                        let lb = norm_b.chars().count();
                        if la.abs_diff(lb) > 2 {
                            continue;
                        }
                        if edit_distance(&norm_a, &norm_b) <= 2 {
                            let wa_v: Vec<&str> = norm_a.split_whitespace().collect();
                            let wb_v: Vec<&str> = norm_b.split_whitespace().collect();
                            // Skip when only short leading tokens differ: "ms gool" ≠ "ah gool"
                            if wa_v.len() >= 2
                                && wb_v.len() >= 2
                                && wa_v[0] != wb_v[0]
                                && wa_v[0].len() <= 3
                                && wb_v[0].len() <= 3
                                && wa_v[1..] == wb_v[1..]
                            {
                                continue;
                            }
                            // Skip when same surname but differing leading initials:
                            // "jmh gool" ≠ "a h gool" — different family members.
                            // Guard only fires when at least one name has ≥2 pre-surname
                            // tokens: "jmh" (1 token) vs "a h" (2 tokens) → max=2 ≥ 2 → guard.
                            // "ben" (1 token) vs "bm" (1 token) → max=1 < 2 → skip guard,
                            // letting edit-distance decide (abbreviation vs nickname).
                            if wa_v.len() >= 2 && wb_v.len() >= 2 {
                                let last_a = *wa_v.last().unwrap();
                                let last_b = *wb_v.last().unwrap();
                                if last_a == last_b && last_a.len() >= 4 {
                                    let pre_a = wa_v.len() - 1;
                                    let pre_b = wb_v.len() - 1;
                                    if pre_a.max(pre_b) >= 2 {
                                        let all_init_a = wa_v[..pre_a].iter().all(|t| t.len() <= 3);
                                        let all_init_b = wb_v[..pre_b].iter().all(|t| t.len() <= 3);
                                        if all_init_a && all_init_b {
                                            let mut sa: Vec<&&str> = wa_v[..pre_a].iter().collect();
                                            let mut sb: Vec<&&str> = wb_v[..pre_b].iter().collect();
                                            sa.sort();
                                            sb.sort();
                                            if sa != sb {
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            // Apply same-first-token structural guard as Tier 2:
                            // "auntie annie" / "auntie minnie" — same prefix, different suffix.
                            if wa_v.len() >= 2 && wb_v.len() >= 2 && wa_v[0] == wb_v[0] {
                                let last_a = wa_v.last().unwrap();
                                let last_b = wb_v.last().unwrap();
                                let max_len = last_a.len().max(last_b.len());
                                let dist = levenshtein_distance(last_a, last_b);
                                if max_len > 2 && dist * 3 >= max_len {
                                    continue;
                                }
                            }
                            let (alias, canonical) = if na.name.len() > nb.name.len()
                                || (na.name.len() == nb.name.len()
                                    && na.mention_count >= nb.mention_count)
                            {
                                (b, a)
                            } else {
                                (a, b)
                            };
                            let key2 = ord_pair(alias, canonical);
                            if seen.insert(key2) {
                                out.push((alias, canonical, "fuzzy"));
                            }
                        }
                    }
                }
            }
        }

        // ── D: qualification-suffix-stripped exact match ──────────────────────
        // "Ben Kies M.A" ↔ "Ben Kies",  "P.V. Tobias M.D." ↔ "P.V. Tobias"
        // Key = strip_qualifications(name) if quals present, else normalize_name(name).
        // Only index names that resolve to ≥ 2 tokens (avoids single-surname ambiguity).
        {
            let mut key_map: HashMap<String, Vec<i64>> = HashMap::new();
            for (&id, node) in &self.nodes {
                let key = match strip_qualifications(&node.name) {
                    Some(s) => s,
                    None => normalize_name(&node.name),
                };
                if key.split_whitespace().count() >= 2 {
                    key_map.entry(key).or_default().push(id);
                }
            }
            for ids in key_map.values() {
                if ids.len() < 2 {
                    continue;
                }
                // Only emit when at least one entity actually carries qualifications;
                // otherwise this is a duplicate of Tier 1 / 3A.
                let any_has_qual = ids.iter().any(|&id| {
                    self.nodes
                        .get(&id)
                        .is_some_and(|n| strip_qualifications(&n.name).is_some())
                });
                if !any_has_qual {
                    continue;
                }
                let &canonical = ids
                    .iter()
                    .max_by_key(|&&id| {
                        self.nodes
                            .get(&id)
                            .map(|n| (n.mention_count, n.name.len()))
                            .unwrap_or((0, 0))
                    })
                    .unwrap();
                for &alias in ids.iter().filter(|&&id| id != canonical) {
                    let na = match self.nodes.get(&alias) {
                        Some(n) => n,
                        None => continue,
                    };
                    let nc = match self.nodes.get(&canonical) {
                        Some(n) => n,
                        None => continue,
                    };
                    if normalize_name(&na.name) == normalize_name(&nc.name) {
                        continue; // Tier 1 handles exact matches
                    }
                    let key2 = ord_pair(alias, canonical);
                    if seen.insert(key2) {
                        out.push((alias, canonical, "qualification"));
                    }
                }
            }
        }

        // ── E: first-name-only subset ─────────────────────────────────────────
        // Catches "Cissie" → "Cissie Gool", "Fatima" → "Fatima Gool", "Zobeida" → "Zobeida Gool".
        //
        // A single-word entity A (≥4 chars, not an honorific/title/common word) is a
        // first-name alias of multi-word entity B when A's word is the first token of B's
        // name AND B is the ONLY entity in the graph whose name starts with that word.
        // The uniqueness guard prevents "Gool" (surname shared by many) from collapsing.
        //
        // No neighbour requirement — first-name-only stubs typically have sparse graphs.
        // Canonical = the longer name (entity B); alias = the single-word entity A.
        {
            // Common English words and titles that should never be treated as a first name.
            const WORD_BLOCKLIST: &[&str] = &[
                "instead",
                "even",
                "head",
                "chief",
                "prince",
                "princess",
                "premier",
                "president",
                "king",
                "queen",
                "lord",
                "lady",
                "captain",
                "major",
                "general",
                "colonel",
                "minister",
                "secretary",
                "director",
                "chairman",
                "leader",
                "speaker",
                "judge",
                "justice",
                "senator",
                "member",
            ];

            // Map: first-token (≥4 chars, non-honorific, non-blocked) → entity IDs whose name starts with it
            let mut first_token_to_ids: HashMap<String, Vec<i64>> = HashMap::new();
            for (&id, node) in &self.nodes {
                let norm = normalize_name(&node.name);
                let words: Vec<&str> = norm.split_whitespace().collect();
                if words.len() >= 2 {
                    let first = words[0];
                    if first.len() >= 4
                        && !HONORIFICS.contains(&first)
                        && !WORD_BLOCKLIST.contains(&first)
                    {
                        first_token_to_ids
                            .entry(first.to_string())
                            .or_default()
                            .push(id);
                    }
                }
            }

            for (&id, node) in &self.nodes {
                let norm = normalize_name(&node.name);
                let words: Vec<&str> = norm.split_whitespace().collect();
                if words.len() != 1 {
                    continue;
                }
                let word = words[0];
                if word.len() < 4 || HONORIFICS.contains(&word) || WORD_BLOCKLIST.contains(&word) {
                    continue;
                }
                let Some(candidates) = first_token_to_ids.get(word) else {
                    continue;
                };
                // Safety: exactly one multi-word entity starts with this first name
                if candidates.len() != 1 {
                    continue;
                }
                let canonical_id = candidates[0];
                if canonical_id == id {
                    continue;
                }
                // Must not already be covered by earlier tiers
                let pair = ord_pair(id, canonical_id);
                if seen.insert(pair) {
                    out.push((id, canonical_id, "first_name_only"));
                }
            }
        }

        out
    }

    // ── Relation-aware dedup blocking ────────────────────────────────────────

    /// Return the subset of `candidates` that MUST NOT be merged because doing
    /// so would create a structural contradiction in the family graph.
    ///
    /// **R1 — Role contradiction** (catches Nasim/Nazima, I.B. Tabata/Jane Tabata):
    /// For a given candidate pair (A, B), look at their family relations to any
    /// shared third-entity C. If A has relation R1 to C and B has relation R2 to C
    /// and R1 ≠ R2, merging A into B would give one entity two different family roles
    /// to the same person — impossible in reality.
    ///
    /// **R2 — Half-sibling disambiguation** (catches Mohamed Saaid / Mohammed Hanief):
    /// Both A and B are `child_of` the same parent P, but each also has a **different**
    /// additional parent that the other lacks. This proves they are half-siblings, not
    /// the same person.
    ///
    /// **Trusted relations**: only relations with `strength ≥ 0.1` (all seeded + multi-
    /// chunk LLM) are considered. The minimum possible strength (0.1) corresponds to
    /// a single evidence source, which is sufficient for blocking since family relations
    /// in this graph are primarily YAML-seeded ground truth.
    pub fn find_dedup_relation_blocks(&self, candidates: &[(i64, i64)]) -> HashSet<(i64, i64)> {
        let mut blocked = HashSet::new();
        for &(alias_id, canonical_id) in candidates {
            if self.dedup_block_r1(alias_id, canonical_id)
                || self.dedup_block_r2(alias_id, canonical_id)
            {
                blocked.insert(ord_pair(alias_id, canonical_id));
            }
        }
        blocked
    }

    /// Returns true when both entities have rich descriptions (≥ `MIN_DESC_LEN` chars)
    /// but share fewer than `MIN_JACCARD` of their significant word types.
    ///
    /// Used as a blocking guard in Tier 2: if descriptions clearly describe different
    /// people (different roles, different eras, different locations) we cap embedding
    /// similarity to prevent auto-merge even when name similarity is high.
    pub fn dedup_desc_diverges(&self, a: i64, b: i64) -> bool {
        const MIN_DESC_LEN: usize = 100;
        const MIN_JACCARD: f32 = 0.12; // < 12% word overlap → descriptions diverge

        let (da, db) = match (self.nodes.get(&a), self.nodes.get(&b)) {
            (Some(na), Some(nb)) => (na.description.as_str(), nb.description.as_str()),
            _ => return false,
        };
        if da.len() < MIN_DESC_LEN || db.len() < MIN_DESC_LEN {
            return false; // not rich enough to judge
        }
        let words_a: std::collections::HashSet<String> = desc_sig_words(da).collect();
        let words_b: std::collections::HashSet<String> = desc_sig_words(db).collect();
        if words_a.is_empty() || words_b.is_empty() {
            return false;
        }
        let intersect = words_a.intersection(&words_b).count();
        let union = words_a.union(&words_b).count();
        let jaccard = intersect as f32 / union as f32;
        jaccard < MIN_JACCARD
    }

    /// R3: returns true when both entities share a high-risk surname (Gool, Rassool, …)
    /// but have NO matching family relation to any shared third entity. Used to downgrade
    /// high-risk pairs from "auto-merge" to "review-only" without blocking outright.
    pub fn dedup_r3_high_risk_surname(&self, a: i64, b: i64) -> bool {
        const HIGH_RISK_SURNAMES: &[&str] = &["gool", "rassool", "abdurahman", "tabata"];

        let surname_of = |id: i64| -> Option<&'static str> {
            let name = self.nodes.get(&id).map(|n| n.name.as_str())?;
            let norm = normalize_name(name);
            let last = norm.split_whitespace().last()?.to_string();
            HIGH_RISK_SURNAMES
                .iter()
                .copied()
                .find(|&s| s == last.as_str())
        };

        let sa = surname_of(a);
        let sb = surname_of(b);
        if sa.is_none() || sa != sb {
            return false; // different surnames or not high-risk
        }

        // Check for at least one matching family relation to a shared third entity
        let a_family: HashMap<i64, HashSet<String>> = self.trusted_family_rel_map(a);
        let b_family: HashMap<i64, HashSet<String>> = self.trusted_family_rel_map(b);
        for (target, a_rels) in &a_family {
            if let Some(b_rels) = b_family.get(target) {
                // Same target, same role → matching relation evidence
                if a_rels.intersection(b_rels).next().is_some() {
                    return false; // found matching relation — not high-risk
                }
            }
        }
        // Shared high-risk surname, no matching relation → flag for R3
        true
    }

    /// R1: merging A into B would give one entity two *different* family roles to
    /// the same third entity (e.g., both `spouse_of X` and `sibling_of X`).
    fn dedup_block_r1(&self, a: i64, b: i64) -> bool {
        // Build: target_id → set of normalized relation types, from A's perspective
        let a_rels = self.trusted_family_rel_map(a);
        for (target, rel_b) in self.trusted_family_rel_iter(b) {
            if let Some(rels_a) = a_rels.get(&target) {
                for rel_a in rels_a {
                    if family_role_contradicts(rel_a, &rel_b) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// R2: both A and B are `child_of` the same parent P, but each also has a
    /// *different* additional parent → they are half-siblings, not duplicates.
    fn dedup_block_r2(&self, a: i64, b: i64) -> bool {
        let a_parents = self.trusted_parent_ids(a);
        let b_parents = self.trusted_parent_ids(b);
        if a_parents.is_empty() || b_parents.is_empty() {
            return false;
        }
        let shared_count = a_parents.intersection(&b_parents).count();
        if shared_count == 0 {
            return false;
        }
        // Have at least one shared parent. Block if both have non-overlapping additional parents.
        let a_only = a_parents.difference(&b_parents).count();
        let b_only = b_parents.difference(&a_parents).count();
        a_only > 0 && b_only > 0
    }

    /// Collect (target_id → HashSet<rel_type>) for **symmetric** family relations from `id`.
    ///
    /// Only symmetric relations (spouse_of, sibling_of, half_sibling_of, cousin_of) are used
    /// here because the in-memory adj stores both directions with the same label for all
    /// relations. Asymmetric relations (parent_of, child_of) appear in both directions in adj,
    /// causing false contradictions (an entity appears as both parent and child of another).
    /// Asymmetric relation checks (R2) must use `trusted_parent_ids` which reads from the DB.
    fn trusted_family_rel_map(&self, id: i64) -> HashMap<i64, HashSet<String>> {
        const SYMMETRIC: &[&str] = &["spouse_of", "sibling_of", "half_sibling_of", "cousin_of"];
        let mut map: HashMap<i64, HashSet<String>> = HashMap::new();
        if let Some(edges) = self.adj.get(&id) {
            for (nbr, rel, strength) in edges {
                if SYMMETRIC.contains(&rel.as_str()) && *strength >= 0.1 {
                    map.entry(*nbr).or_default().insert(rel.clone());
                }
            }
        }
        map
    }

    /// Iterator over (target_id, rel_type) for trusted symmetric family relations from `id`.
    fn trusted_family_rel_iter(&self, id: i64) -> Vec<(i64, String)> {
        const SYMMETRIC: &[&str] = &["spouse_of", "sibling_of", "half_sibling_of", "cousin_of"];
        self.adj
            .get(&id)
            .into_iter()
            .flatten()
            .filter(|(_, rel, strength)| SYMMETRIC.contains(&rel.as_str()) && *strength >= 0.1)
            .map(|(nbr, rel, _)| (*nbr, rel.clone()))
            .collect()
    }

    /// Set of entity IDs that are **parents** of `id`.
    ///
    /// Reads `child_of(id, parent)` records directly from the DB rather than from adj,
    /// because adj stores both directions of asymmetric relations with the same label,
    /// which would incorrectly include backward `child_of` entries from the inverse storage.
    pub fn trusted_parent_ids(&self, id: i64) -> HashSet<i64> {
        let Ok(rtxn) = self.db.begin_read() else {
            return HashSet::new();
        };
        let Ok(table) = rtxn.open_table(RELATIONS_TABLE) else {
            return HashSet::new();
        };
        let mut parents = HashSet::new();
        if let Ok(iter) = table.iter() {
            for entry in iter.flatten() {
                let (_, v) = entry;
                if let Ok(rel) = serde_json::from_slice::<RelationRecord>(v.value()) {
                    if rel.src_id == id && rel.relation_type == "child_of" && rel.strength >= 0.1 {
                        parents.insert(rel.dst_id);
                    }
                }
            }
        }
        parents
    }

    /// Remove non-seeded `child_of`/`parent_of` edges for entities whose
    /// parentage is established by the family-tree seed (evidence_chunk_id == 0).
    ///
    /// When the YAML seed has planted `A child_of B`, any LLM-extracted `A child_of C`
    /// for unrelated C is almost certainly a hallucination.  This pass purges those
    /// edges (and their auto-created inverses) in bulk, then rebuilds the in-memory adj.
    ///
    /// Returns the number of erroneous forward edges removed.
    pub fn purge_unseeded_parent_relations(&mut self) -> Result<usize> {
        let all_rels: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .filter(|r| r.relation_type == "child_of" || r.relation_type == "parent_of")
                .collect()
        };

        // Entities whose parentage / children are established by seed
        let mut child_of_seeded: HashSet<i64> = HashSet::new();
        let mut parent_of_seeded: HashSet<i64> = HashSet::new();
        for rel in &all_rels {
            if rel.evidence_chunk_ids.contains(&0) {
                match rel.relation_type.as_str() {
                    "child_of" => {
                        child_of_seeded.insert(rel.src_id);
                    }
                    "parent_of" => {
                        parent_of_seeded.insert(rel.src_id);
                    }
                    _ => {}
                }
            }
        }

        let mut keys_to_delete: Vec<Vec<u8>> = Vec::new();
        let mut removed = 0usize;
        for rel in &all_rels {
            if rel.evidence_chunk_ids.contains(&0) {
                continue; // seeded — keep
            }
            let purge = match rel.relation_type.as_str() {
                "child_of" => child_of_seeded.contains(&rel.src_id),
                "parent_of" => parent_of_seeded.contains(&rel.src_id),
                _ => false,
            };
            if purge {
                removed += 1;
                keys_to_delete.push(relation_key(rel.src_id, rel.dst_id, &rel.relation_type));
                // Also purge the stored inverse (parent_of ↔ child_of)
                if let Some(&inv) = FAMILIAL_INVERSE
                    .iter()
                    .find(|(r, _)| *r == rel.relation_type.as_str())
                    .map(|(_, i)| i)
                {
                    keys_to_delete.push(relation_key(rel.dst_id, rel.src_id, inv));
                }
            }
        }

        if !keys_to_delete.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for k in &keys_to_delete {
                    let _ = t.remove(k.as_slice()); // ignore not-found for inverses
                }
            }
            wtxn.commit()?;
            self.rebuild_in_memory()?;
        }

        Ok(removed)
    }

    /// Tier 4a: Unique-surname dedup.
    ///
    /// Catches "Mr Kies", "Mrs Gool", or bare "Kies" where the entity name reduces
    /// to a **single surname token** after stripping honorifics, AND exactly one
    /// other entity in the graph shares that surname as the last token of their
    /// stripped name (or any alias).  Because the surname is unambiguous in the
    /// graph, the merge is safe without any embedding or neighbour check.
    ///
    /// Returns `(alias_id, canonical_id, "unique_surname")`.
    ///
    /// Gates:
    /// - Same entity_type on both ends.
    /// - Surname token ≥ 3 characters (avoids single-initial references).
    /// - The canonical must have > 1 stripped token (it is a full name, not
    ///   another bare-surname entity).
    /// - Existing description divergence blocks the merge.
    pub fn find_dedup_candidates_unique_surname(&self) -> Vec<(i64, i64, &'static str)> {
        // ── Step 1: build surname (last stripped token) → entity_ids map ─────
        // We index every entity by the last meaningful token of every name form
        // it carries (canonical + aliases), so "Ben Kies" → "kies",
        // "Benjamin Maximilian Kies" → "kies", alias "B.M. Kies" → "kies".
        let mut surname_map: HashMap<String, Vec<i64>> = HashMap::new();
        for (&id, node) in &self.nodes {
            let mut surnames_for_entity: std::collections::HashSet<String> = Default::default();
            let all_names =
                std::iter::once(node.name.as_str()).chain(node.aliases.iter().map(|a| a.as_str()));
            for name in all_names {
                let sk = stripped_key(name);
                if let Some(last) = sk.split_whitespace().last() {
                    if last.len() >= 3 {
                        surnames_for_entity.insert(last.to_string());
                    }
                }
            }
            for surname in surnames_for_entity {
                surname_map.entry(surname).or_default().push(id);
            }
        }
        // Deduplicate entity lists (an entity can contribute the same surname via
        // multiple aliases — we only want it counted once per surname).
        for ids in surname_map.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }

        // ── Step 2: find surname-only reference entities ──────────────────────
        let mut out: Vec<(i64, i64, &'static str)> = Vec::new();
        let mut seen: HashSet<(i64, i64)> = HashSet::new();

        for (&alias_id, node) in &self.nodes {
            let sk = stripped_key(&node.name);
            let tokens: Vec<&str> = sk.split_whitespace().collect();

            // Must reduce to exactly 1 token after honorific stripping.
            if tokens.len() != 1 {
                continue;
            }
            let surname = tokens[0];
            if surname.len() < 3 {
                continue;
            }

            // Look up all entities carrying this surname.
            let candidates = match surname_map.get(surname) {
                Some(v) => v,
                None => continue,
            };

            // Collect other entities with this surname (exclude self).
            let others: Vec<i64> = candidates
                .iter()
                .copied()
                .filter(|&id| id != alias_id)
                .collect();

            // Exactly one other entity → unambiguous match.
            if others.len() != 1 {
                continue;
            }
            let canonical_id = others[0];

            let canonical = match self.nodes.get(&canonical_id) {
                Some(n) => n,
                None => continue,
            };

            // Gate: same entity type.
            if node.entity_type != canonical.entity_type {
                continue;
            }

            // Gate: canonical must be a full name (> 1 stripped token), not
            // another bare-surname entity — avoids merging two bare surnames.
            let canonical_sk = stripped_key(&canonical.name);
            if canonical_sk.split_whitespace().count() <= 1 {
                continue;
            }

            // Gate: description divergence blocks the merge.
            if self.dedup_desc_diverges(alias_id, canonical_id) {
                continue;
            }

            let key = ord_pair(alias_id, canonical_id);
            if seen.insert(key) {
                out.push((alias_id, canonical_id, "unique_surname"));
            }
        }

        out
    }

    /// Tier 4b: Middle-name-drop dedup.
    ///
    /// Catches the memoir pattern where a character is introduced with their full
    /// name including middle names ("Victor Arthur Wessels") and later referred to
    /// as first+last ("Victor Wessels") or bare first name ("Victor").
    ///
    /// Two sub-rules:
    ///
    /// **Rule B1 — first+last match**: entity A has exactly 2 stripped tokens
    /// `[First] [Last]`; entity B has ≥ 3 stripped tokens whose first and last
    /// tokens match A's.  Safe when the (first, last) pair is **unique** in the
    /// graph (only one entity carries those bookend names).
    ///
    /// **Rule B2 — bare first name**: entity A has exactly 1 stripped token that
    /// matches the *first* token of exactly one other entity with ≥ 2 stripped
    /// tokens.  More conservative: also requires the alias entity to have no
    /// description (unenriched) OR both entities share ≥ 1 graph neighbour.
    ///
    /// Returns `(alias_id, canonical_id, reason)` where reason is
    /// `"first_last_drop"` or `"bare_firstname"`.
    pub fn find_dedup_candidates_middle_drop(&self) -> Vec<(i64, i64, &'static str)> {
        let mut seen: HashSet<(i64, i64)> = HashSet::new();
        let mut out: Vec<(i64, i64, &'static str)> = Vec::new();

        // Pre-compute stripped tokens for every entity (canonical only — aliases
        // are deliberately excluded to keep the uniqueness gate tight).
        let stripped: HashMap<i64, Vec<String>> = self
            .nodes
            .iter()
            .map(|(&id, n)| {
                let tokens: Vec<String> = stripped_key(&n.name)
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                (id, tokens)
            })
            .collect();

        // ── B1: (first, last) index ───────────────────────────────────────────
        // Maps (first_token, last_token) → all entity IDs that carry those
        // bookend tokens (canonical + aliases).
        let mut fl_map: HashMap<(String, String), Vec<i64>> = HashMap::new();
        for (&id, node) in &self.nodes {
            let all_names =
                std::iter::once(node.name.as_str()).chain(node.aliases.iter().map(|a| a.as_str()));
            let mut seen_for_entity: std::collections::HashSet<(String, String)> =
                Default::default();
            for name in all_names {
                let sk = stripped_key(name);
                let toks: Vec<&str> = sk.split_whitespace().collect();
                if toks.len() < 2 {
                    continue;
                }
                let key = (toks[0].to_string(), toks[toks.len() - 1].to_string());
                if seen_for_entity.insert(key.clone()) {
                    fl_map.entry(key).or_default().push(id);
                }
            }
        }
        for ids in fl_map.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }

        // ── B1: find 2-token entities whose (first, last) is unique ───────────
        for (&alias_id, alias_toks) in &stripped {
            if alias_toks.len() != 2 {
                continue;
            }
            let first = &alias_toks[0];
            let last = &alias_toks[1];
            // Single-letter initials ("A. Gool") are too ambiguous to auto-merge.
            if first.len() < 2 || last.len() < 2 {
                continue;
            }
            let key = (first.clone(), last.clone());
            let candidates = match fl_map.get(&key) {
                Some(v) => v,
                None => continue,
            };
            // Others = entities that also carry (first, last) but have ≥ 3 stripped tokens
            // (the alias itself also appears in the map if it contributed the same pair).
            let others: Vec<i64> = candidates
                .iter()
                .copied()
                .filter(|&id| {
                    id != alias_id && stripped.get(&id).map(|t| t.len() >= 3).unwrap_or(false)
                })
                .collect();
            if others.len() != 1 {
                continue; // ambiguous or no full-name match
            }
            let canonical_id = others[0];
            let alias_node = match self.nodes.get(&alias_id) {
                Some(n) => n,
                None => continue,
            };
            let canonical_node = match self.nodes.get(&canonical_id) {
                Some(n) => n,
                None => continue,
            };
            // Gates
            if alias_node.entity_type != canonical_node.entity_type {
                continue;
            }
            if self.dedup_desc_diverges(alias_id, canonical_id) {
                continue;
            }
            let k = ord_pair(alias_id, canonical_id);
            if seen.insert(k) {
                out.push((alias_id, canonical_id, "first_last_drop"));
            }
        }

        // ── B2: bare first name ───────────────────────────────────────────────
        // Build first_token → entity IDs (for entities with ≥ 2 tokens).
        let mut first_map: HashMap<String, Vec<i64>> = HashMap::new();
        for (&id, toks) in &stripped {
            if toks.len() >= 2 {
                first_map.entry(toks[0].clone()).or_default().push(id);
            }
        }
        for ids in first_map.values_mut() {
            ids.sort_unstable();
            ids.dedup();
        }

        for (&alias_id, alias_toks) in &stripped {
            if alias_toks.len() != 1 {
                continue; // handled by Tier 4a (surname) or not applicable
            }
            let first = &alias_toks[0];
            if first.len() < 3 {
                continue;
            }
            let candidates = match first_map.get(first) {
                Some(v) => v,
                None => continue,
            };
            let others: Vec<i64> = candidates
                .iter()
                .copied()
                .filter(|&id| id != alias_id)
                .collect();
            if others.len() != 1 {
                continue; // common first name — ambiguous
            }
            let canonical_id = others[0];
            let alias_node = match self.nodes.get(&alias_id) {
                Some(n) => n,
                None => continue,
            };
            let canonical_node = match self.nodes.get(&canonical_id) {
                Some(n) => n,
                None => continue,
            };
            if alias_node.entity_type != canonical_node.entity_type {
                continue;
            }
            if self.dedup_desc_diverges(alias_id, canonical_id) {
                continue;
            }
            // Extra gate for bare first names: require at least 1 shared neighbour
            // OR the alias is unenriched (no description yet).
            let alias_unenriched = alias_node.description.is_empty();
            let shares_neighbour = {
                let an: std::collections::HashSet<i64> = self
                    .neighbors_of(alias_id)
                    .into_iter()
                    .map(|(id, _, _)| id)
                    .collect();
                self.neighbors_of(canonical_id)
                    .into_iter()
                    .any(|(id, _, _)| an.contains(&id))
            };
            if !alias_unenriched && !shares_neighbour {
                continue;
            }
            let k = ord_pair(alias_id, canonical_id);
            if seen.insert(k) {
                out.push((alias_id, canonical_id, "bare_firstname"));
            }
        }

        out
    }

    /// Tier 5 (was Tier 4): Role-pronoun neighbour-containment dedup.
    ///
    /// Targets entities whose name is a **role or pronoun description** ("Grandpa",
    /// "my uncle", "the Head", "grandmother") rather than a proper name.  Such
    /// entities often refer to a known person elsewhere in the graph.  We confirm the
    /// match by requiring that ≥ `min_containment` of the alias's neighbours also
    /// appear in the canonical's neighbour set (with ≥ `min_shared` absolute overlap).
    ///
    /// Role detection: the alias name must consist entirely of words from the
    /// honorific/role vocabulary (HONORIFICS + common relational words).  This
    /// excludes proper names, initials, and mixed names like "Uncle Ben Kies".
    ///
    /// Returns `(alias_id, canonical_id, containment)` sorted by containment desc.
    pub fn find_dedup_candidates_neighbor_containment(
        &self,
        min_containment: f32,
        min_shared: usize,
    ) -> Vec<(i64, i64, f32)> {
        // Standalone role words: meaningful on their own as a person reference.
        // Honorifics (dr, mr, etc.) are excluded — they only make sense before a name.
        const STANDALONE_ROLES: &[&str] = &[
            "grandpa",
            "grandma",
            "grandfather",
            "grandmother",
            "granddad",
            "dad",
            "father",
            "mom",
            "mother",
            "mum",
            "cousin",
            "brother",
            "sister",
            "nephew",
            "niece",
            "narrator",
            "author",
            "writer",
            "head",
            "chief",
            "she",
            "he",
            "they",
        ];
        // All words allowed in a role-entity name (role + connective filler).
        const ROLE_WORDS: &[&str] = &[
            "dr",
            "mr",
            "mrs",
            "ms",
            "miss",
            "prof",
            "professor",
            "rev",
            "reverend",
            "sir",
            "haji",
            "hajj",
            "maulvi",
            "maulana",
            "imam",
            "sheikh",
            "shaykh",
            "auntie",
            "aunt",
            "uncle",
            "oom",
            "tannie",
            "oupa",
            "grandpa",
            "grandma",
            "grandfather",
            "grandmother",
            "granddad",
            "dad",
            "father",
            "mom",
            "mother",
            "mum",
            "cousin",
            "brother",
            "sister",
            "nephew",
            "niece",
            "the",
            "a",
            "our",
            "her",
            "his",
            "my",
            "narrator",
            "author",
            "writer",
            "head",
            "chief",
            "senior",
            "she",
            "he",
            "they",
            "it",
        ];

        // Build neighbour sets, excluding hyper-popular hubs (> 25 connecting
        // entities) — they carry no identity signal in a memoir.
        const MAX_HUB: usize = 25;
        let mut nbr_to_entities: HashMap<i64, Vec<i64>> = HashMap::new();
        for (&id, edges) in &self.adj {
            for &(nbr, _, _) in edges {
                nbr_to_entities.entry(nbr).or_default().push(id);
            }
        }
        let nbr_sets: HashMap<i64, HashSet<i64>> = self
            .adj
            .iter()
            .map(|(&id, edges)| {
                let set: HashSet<i64> = edges
                    .iter()
                    .filter(|(nbr, _, _)| {
                        nbr_to_entities
                            .get(nbr)
                            .map(|v| v.len() <= MAX_HUB)
                            .unwrap_or(true)
                    })
                    .map(|(nbr, _, _)| *nbr)
                    .collect();
                (id, set)
            })
            .collect();

        let mut out: Vec<(i64, i64, f32)> = Vec::new();
        let ids: Vec<i64> = self.nodes.keys().copied().collect();

        for &a in &ids {
            let na = match self.nodes.get(&a) {
                Some(n) => n,
                None => continue,
            };
            let norm_a = normalize_name(&na.name);
            let words_a: Vec<&str> = norm_a.split_whitespace().collect();
            // Gate 1: every token must be a role word (no proper-name tokens).
            if words_a.is_empty() || !words_a.iter().all(|w| ROLE_WORDS.contains(w)) {
                continue;
            }
            // Gate 2: at least one token must be a standalone role (not just an honorific).
            if !words_a.iter().any(|w| STANDALONE_ROLES.contains(w)) {
                continue;
            }

            let nbrs_a = match nbr_sets.get(&a) {
                Some(s) if !s.is_empty() => s,
                _ => continue,
            };

            for &b in &ids {
                if b == a {
                    continue;
                }
                let nb = match self.nodes.get(&b) {
                    Some(n) => n,
                    None => continue,
                };
                let nbrs_b = match nbr_sets.get(&b) {
                    Some(s) => s,
                    None => continue,
                };
                if nbrs_b.len() <= nbrs_a.len() {
                    continue; // b must be the larger entity
                }
                let shared = nbrs_a.intersection(nbrs_b).count();
                if shared < min_shared {
                    continue;
                }
                let containment = shared as f32 / nbrs_a.len() as f32;
                if containment < min_containment {
                    continue;
                }
                if normalize_name(&na.name) == normalize_name(&nb.name) {
                    continue;
                }
                // Gate: never merge across entity types (Person ≠ Place, etc.)
                if na.entity_type != nb.entity_type {
                    continue;
                }
                if let (Some(sa), Some(sb)) = (&na.schema_type, &nb.schema_type) {
                    if sa != sb {
                        continue;
                    }
                }
                out.push((a, b, containment));
            }
        }
        out.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        out
    }

    /// Fully reload in-memory state from the database.
    /// Call once after a batch of `merge_entity_into` calls.
    /// Delete an entity from the persistent store and in-memory index.
    /// Relations that referenced this entity become stale — call `rebuild_in_memory()` after a
    /// batch of deletes.
    pub fn delete_entity(&mut self, entity_id: i64) -> Result<()> {
        let key = entity_id.to_le_bytes();
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            t.remove(key.as_ref())?;
        }
        wtxn.commit()?;
        self.nodes.remove(&entity_id);
        self.entity_to_chunks.remove(&entity_id);
        self.adj.remove(&entity_id);
        Ok(())
    }

    /// Remove any relation from the DB and in-memory adj map where either endpoint entity
    /// no longer exists in `self.nodes`. Call this after bulk entity deletion to avoid
    /// dangling edges. Returns the number of relations removed.
    pub fn prune_dangling_relations(&mut self) -> Result<usize> {
        let all_rels: Vec<RelationRecord> = {
            let rtxn = self.db.begin_read()?;
            let table = rtxn.open_table(RELATIONS_TABLE)?;
            table
                .iter()?
                .filter_map(|r| r.ok())
                .filter_map(|(_, v)| serde_json::from_slice::<RelationRecord>(v.value()).ok())
                .collect()
        };

        let mut to_delete: Vec<Vec<u8>> = Vec::new();
        for rel in &all_rels {
            if !self.nodes.contains_key(&rel.src_id) || !self.nodes.contains_key(&rel.dst_id) {
                to_delete.push(relation_key(rel.src_id, rel.dst_id, &rel.relation_type));
                // Remove from in-memory adj both directions.
                if let Some(v) = self.adj.get_mut(&rel.src_id) {
                    v.retain(|(dst, rtype, _)| *dst != rel.dst_id || rtype != &rel.relation_type);
                }
                if let Some(v) = self.adj.get_mut(&rel.dst_id) {
                    v.retain(|(dst, rtype, _)| *dst != rel.src_id || rtype != &rel.relation_type);
                }
            }
        }

        let removed = to_delete.len();
        if !to_delete.is_empty() {
            let wtxn = self.db.begin_write()?;
            {
                let mut t = wtxn.open_table(RELATIONS_TABLE)?;
                for k in &to_delete {
                    t.remove(k.as_slice())?;
                }
            }
            wtxn.commit()?;
        }
        Ok(removed)
    }

    /// Write a resolved schema.org type back to a stored entity without changing its entity_id.
    pub fn set_schema_type(&mut self, entity_id: i64, schema_type: &str) -> Result<()> {
        let node = match self.nodes.get_mut(&entity_id) {
            Some(n) => n,
            None => return Ok(()),
        };
        node.schema_type = Some(schema_type.to_string());
        let key = entity_id.to_le_bytes();
        let val = serde_json::to_vec(node)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            t.insert(key.as_ref(), val.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Directly replace an entity's prose description without going through the
    /// upsert merge logic. The entity's embedding is left unchanged (caller must
    /// re-embed separately if semantic search accuracy matters).
    pub fn set_description(&mut self, entity_id: i64, description: &str) -> Result<()> {
        let node = match self.nodes.get_mut(&entity_id) {
            Some(n) => n,
            None => return Ok(()),
        };
        node.description = description.to_string();
        let key = entity_id.to_le_bytes();
        let val = serde_json::to_vec(node)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            t.insert(key.as_ref(), val.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Store the list of source document titles for this KB. Used by dream tasks to
    /// inject exclusion rules that prevent book/work titles from being used as
    /// location or organisation targets in the knowledge graph.
    pub fn set_document_titles(&mut self, titles: &[String]) -> Result<()> {
        let json = serde_json::to_string(titles)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(METADATA_TABLE)?;
            t.insert("document_titles", json.as_str())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Persist document-level metadata from a DocSchema into the graph store.
    pub fn set_doc_metadata(
        &mut self,
        metadata: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        let json = serde_json::to_string(metadata)?;
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(METADATA_TABLE)?;
            t.insert("doc_metadata", json.as_str())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Retrieve persisted document metadata. Returns empty map if none stored.
    pub fn get_doc_metadata(&self) -> std::collections::HashMap<String, String> {
        let Ok(rtxn) = self.db.begin_read() else {
            return Default::default();
        };
        let Ok(table) = rtxn.open_table(METADATA_TABLE) else {
            return Default::default();
        };
        let Ok(Some(v)) = table.get("doc_metadata") else {
            return Default::default();
        };
        serde_json::from_str(v.value()).unwrap_or_default()
    }

    /// Retrieve the stored document titles. Returns an empty vec if none are stored.
    pub fn get_document_titles(&self) -> Vec<String> {
        let Ok(rtxn) = self.db.begin_read() else {
            return vec![];
        };
        let Ok(table) = rtxn.open_table(METADATA_TABLE) else {
            return vec![];
        };
        let Ok(Some(v)) = table.get("document_titles") else {
            return vec![];
        };
        serde_json::from_str(v.value()).unwrap_or_default()
    }

    /// Expose chunk→entity mapping for cross-link discovery in the dream loop.
    pub fn all_chunk_entity_pairs(&self) -> impl Iterator<Item = (i64, &Vec<i64>)> {
        self.chunk_to_entities.iter().map(|(&k, v)| (k, v))
    }

    /// Return the entity IDs linked to a chunk (empty slice if none).
    pub fn get_chunk_entities(&self, chunk_id: i64) -> Vec<i64> {
        self.chunk_to_entities
            .get(&chunk_id)
            .cloned()
            .unwrap_or_default()
    }

    // ── Sequence diagram: timeline store/get ──────────────────────────────────

    /// Append timeline events for an entity, merging with any already stored.
    pub fn store_timeline_events(
        &self,
        entity_id: i64,
        new_events: &[crate::sequence::TimelineEvent],
    ) -> Result<()> {
        if new_events.is_empty() {
            return Ok(());
        }
        let key = entity_id.to_le_bytes();
        let wtxn = self.db.begin_write()?;
        {
            let mut table = wtxn.open_table(TIMELINE_TABLE)?;
            let existing: Vec<crate::sequence::TimelineEvent> = table
                .get(key.as_ref())?
                .and_then(|v| serde_json::from_slice(v.value()).ok())
                .unwrap_or_default();
            let mut merged = existing;
            merged.extend_from_slice(new_events);
            // Deduplicate by (chunk_id, description)
            merged.dedup_by(|a, b| {
                a.evidence_chunk_id == b.evidence_chunk_id && a.description == b.description
            });
            let val = serde_json::to_vec(&merged)?;
            table.insert(key.as_ref(), val.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Return all timeline events for a list of entity IDs.
    pub fn get_timeline_events(&self, entity_ids: &[i64]) -> Vec<crate::sequence::TimelineEvent> {
        let rtxn = match self.db.begin_read() {
            Ok(t) => t,
            Err(_) => return vec![],
        };
        let table = match rtxn.open_table(TIMELINE_TABLE) {
            Ok(t) => t,
            Err(_) => return vec![],
        };
        let mut out = Vec::new();
        for &eid in entity_ids {
            let key = eid.to_le_bytes();
            if let Ok(Some(v)) = table.get(key.as_ref()) {
                if let Ok(evts) =
                    serde_json::from_slice::<Vec<crate::sequence::TimelineEvent>>(v.value())
                {
                    out.extend(evts);
                }
            }
        }
        out
    }

    /// Append interactions between two entities, merging with any already stored.
    /// Key is (min(from,to), max(from,to)) so direction doesn't matter for lookup.
    pub fn store_interaction(
        &self,
        interaction: &crate::sequence::SequenceInteraction,
    ) -> Result<()> {
        let lo = interaction.from_entity_id.min(interaction.to_entity_id);
        let hi = interaction.from_entity_id.max(interaction.to_entity_id);
        let mut key = [0u8; 16];
        key[..8].copy_from_slice(&lo.to_le_bytes());
        key[8..].copy_from_slice(&hi.to_le_bytes());

        let wtxn = self.db.begin_write()?;
        {
            let mut table = wtxn.open_table(INTERACTION_TABLE)?;
            let existing: Vec<crate::sequence::SequenceInteraction> = table
                .get(key.as_ref())?
                .and_then(|v| serde_json::from_slice(v.value()).ok())
                .unwrap_or_default();
            let mut merged = existing;
            merged.push(interaction.clone());
            merged
                .dedup_by(|a, b| a.evidence_chunk_id == b.evidence_chunk_id && a.label == b.label);
            let val = serde_json::to_vec(&merged)?;
            table.insert(key.as_ref(), val.as_slice())?;
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Return all interactions where either endpoint is in `entity_ids`.
    pub fn get_interactions_for(
        &self,
        entity_ids: &[i64],
    ) -> Vec<crate::sequence::SequenceInteraction> {
        let rtxn = match self.db.begin_read() {
            Ok(t) => t,
            Err(_) => return vec![],
        };
        let table = match rtxn.open_table(INTERACTION_TABLE) {
            Ok(t) => t,
            Err(_) => return vec![],
        };
        let id_set: std::collections::HashSet<i64> = entity_ids.iter().copied().collect();
        let mut out = Vec::new();
        for (_, v) in table.iter().into_iter().flatten().flatten() {
            if let Ok(interactions) =
                serde_json::from_slice::<Vec<crate::sequence::SequenceInteraction>>(v.value())
            {
                for ia in interactions {
                    if id_set.contains(&ia.from_entity_id) || id_set.contains(&ia.to_entity_id) {
                        out.push(ia);
                    }
                }
            }
        }
        out
    }

    /// Drop all timeline events and interactions (used by --reset-timeline).
    pub fn reset_timeline(&self) -> Result<()> {
        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(TIMELINE_TABLE)?;
            let keys: Vec<Vec<u8>> = t
                .iter()?
                .filter_map(|e| e.ok())
                .map(|(k, _)| k.value().to_vec())
                .collect();
            for k in keys {
                t.remove(k.as_slice())?;
            }
        }
        {
            let mut t = wtxn.open_table(INTERACTION_TABLE)?;
            let keys: Vec<Vec<u8>> = t
                .iter()?
                .filter_map(|e| e.ok())
                .map(|(k, _)| k.value().to_vec())
                .collect();
            for k in keys {
                t.remove(k.as_slice())?;
            }
        }
        wtxn.commit()?;
        Ok(())
    }

    /// Count stored timeline events and interactions (for status display).
    pub fn timeline_stats(&self) -> (usize, usize) {
        let rtxn = match self.db.begin_read() {
            Ok(t) => t,
            Err(_) => return (0, 0),
        };
        let event_count = rtxn
            .open_table(TIMELINE_TABLE)
            .ok()
            .and_then(|t| t.iter().ok().map(|iter| iter.count()))
            .unwrap_or(0);
        let interaction_count = rtxn
            .open_table(INTERACTION_TABLE)
            .ok()
            .and_then(|t| t.iter().ok().map(|iter| iter.count()))
            .unwrap_or(0);
        (event_count, interaction_count)
    }

    /// For each chunk linked to at least one entity, return (chunk_id, tag_prefix).
    ///
    /// tag_prefix is a space-separated `[Name]` sequence of all entities linked to that
    /// chunk, sorted descending by mention_count and capped at `max_tags`.  Using all
    /// linked entities (rather than just the top-1) ensures that chunks covering multiple
    /// subjects (e.g. Gandhi visiting Buitencingle) are reachable via any of their entities.
    pub fn chunk_primary_entity_names(&self) -> Vec<(i64, String)> {
        self.chunk_entity_tag_prefixes(3)
    }

    pub fn chunk_entity_tag_prefixes(&self, max_tags: usize) -> Vec<(i64, String)> {
        self.chunk_to_entities
            .iter()
            .filter_map(|(&cid, eids)| {
                let mut names: Vec<(u32, &str)> = eids
                    .iter()
                    .filter_map(|&eid| self.nodes.get(&eid))
                    .map(|n| (n.mention_count, n.name.as_str()))
                    .collect();
                if names.is_empty() {
                    return None;
                }
                names.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));
                names.truncate(max_tags);
                let prefix = names
                    .iter()
                    .map(|(_, name)| format!("[{name}]"))
                    .collect::<Vec<_>>()
                    .join(" ");
                Some((cid, prefix))
            })
            .collect()
    }

    pub fn rebuild_in_memory(&mut self) -> Result<()> {
        self.nodes.clear();
        self.adj.clear();
        self.chunk_to_entities.clear();
        self.entity_to_chunks.clear();
        self.alias_token_index.clear();
        self.rebuild()
    }

    /// Build the text string that gets embedded for an entity.
    /// Includes aliases so that abbreviation queries find the canonical entity after a merge.
    pub fn entity_embed_text(name: &str, aliases: &[String], description: &str) -> String {
        let alias_str = aliases
            .iter()
            .filter(|a| !a.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        match (alias_str.is_empty(), description.is_empty()) {
            (true, true) => name.to_string(),
            (true, false) => format!("{}: {}", name, description),
            (false, true) => format!("{} ({})", name, alias_str),
            (false, false) => format!("{} ({}): {}", name, alias_str, description),
        }
    }

    /// Re-embed all entities. Includes aliases in the embedded text so abbreviation
    /// queries resolve to canonical entities after a merge.
    pub async fn reembed_all(&mut self, embed: &crate::embedder::EmbedClient) -> Result<usize> {
        let ids: Vec<i64> = self.nodes.keys().copied().collect();
        self.reembed_by_ids(&ids, embed).await
    }

    /// Re-embed a specific subset of entities by ID. Used after alias merges to
    /// refresh only the affected canonical entities without re-embedding the full graph.
    pub async fn reembed_entities(
        &mut self,
        ids: &[i64],
        embed: &crate::embedder::EmbedClient,
    ) -> Result<usize> {
        let ids: Vec<i64> = ids
            .iter()
            .filter(|id| self.nodes.contains_key(id))
            .copied()
            .collect();
        self.reembed_by_ids(&ids, embed).await
    }

    async fn reembed_by_ids(
        &mut self,
        ids: &[i64],
        embed: &crate::embedder::EmbedClient,
    ) -> Result<usize> {
        // Re-derive description from structured fields so that placeholder values
        // ("unknown", "undefined", etc.) are stripped by description_from_fields().
        // This is a no-op for entities whose fields are empty (prose-only descriptions).
        for id in ids {
            if let Some(node) = self.nodes.get_mut(id) {
                if !node.fields.is_empty() {
                    let fresh =
                        description_from_fields(&node.name, &node.entity_type, &node.fields);
                    if !fresh.is_empty() {
                        node.description = fresh;
                    }
                }
            }
        }

        // Embed in batches of 64 — sending all entities in one request OOMs Ollama on large graphs.
        const EMBED_BATCH: usize = 64;
        let mut all_embeddings: Vec<Vec<f32>> = Vec::with_capacity(ids.len());
        for id_chunk in ids.chunks(EMBED_BATCH) {
            let texts: Vec<String> = id_chunk
                .iter()
                .map(|id| {
                    let n = &self.nodes[id];
                    Self::entity_embed_text(&n.name, &n.aliases, &n.description)
                })
                .collect();
            let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
            let mut batch_embeddings = embed.embed_batch(&text_refs).await?;
            all_embeddings.append(&mut batch_embeddings);
        }

        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            for (id, emb) in ids.iter().zip(all_embeddings) {
                if let Some(node) = self.nodes.get_mut(id) {
                    node.embedding = emb;
                    let key = id.to_le_bytes();
                    let val = serde_json::to_vec(node)?;
                    t.insert(key.as_ref(), val.as_slice())?;
                }
            }
        }
        wtxn.commit()?;
        Ok(ids.len())
    }
}

// ── LLM-based entity extraction ───────────────────────────────────────────────

/// Extract entities and relations from a chunk of text using the local LLM.
/// Returns `Ok((entities, relations))` or `Ok(([], []))` on parse failure so
/// Normalise OCR underscore artifacts in proper-noun candidates before sending to the LLM.
/// Delegates to `clean_entity_name` which handles all patterns: `J_ M_ H_`, `M_K_`, `Dr_`.
fn normalize_underscores(s: &str) -> String {
    clean_entity_name(s)
}

/// ingestion can continue without hard errors.
#[allow(clippy::too_many_arguments)]
pub async fn extract_from_text(
    text: &str,
    candidates: &[String],
    pronoun_map: &[(String, String)],
    section_note: Option<&str>,
    inference_url: &str,
    model: &str,
    entity_types: &[&str],
    no_relations: bool,
    gliner_hints: Option<&[String]>,
) -> Result<(Vec<ExtractedEntity>, Vec<ExtractedRelation>)> {
    // Skip the LLM entirely when the local pre-screener found no proper nouns.
    // Avoids inference cost on boilerplate, numeric, or table-heavy chunks.
    if candidates.is_empty() {
        tracing::debug!("no proper noun candidates — skipping LLM extraction for this chunk");
        return Ok((vec![], vec![]));
    }

    let effective_types = if entity_types.is_empty() {
        ENTITY_TYPES
    } else {
        entity_types
    };
    let entity_list = effective_types.join(", ");

    let section_context = section_note
        .map(|note| format!("DOCUMENT CONTEXT: {note}\n\n"))
        .unwrap_or_default();

    // Resolved pronouns injected as a preamble so the LLM never treats a pronoun
    // as a candidate entity name.
    let pronoun_context = if pronoun_map.is_empty() {
        String::new()
    } else {
        let pairs = pronoun_map
            .iter()
            .map(|(pron, name)| format!("'{pron}' = '{name}'"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("KNOWN COREFERENCES: {pairs}\n\n")
    };

    // Cap prevents JSON overflow failures on entity-dense passages (+7pp reliability,
    // experiments show no recall loss at this cap with window=1 chunking).
    let entity_cap = if entity_types.len() <= 3 { 25 } else { 20 };

    // Normalise OCR artifacts before presenting candidates to the LLM.
    // In this corpus underscores replace periods in initials (J_ M_ H_ → J. M. H.).
    let normalized_candidates: Vec<String> = candidates
        .iter()
        .map(|c| normalize_underscores(c))
        .collect();
    let candidates_block = normalized_candidates
        .iter()
        .map(|c| format!("- {c}"))
        .collect::<Vec<_>>()
        .join("\n");

    // GLiNER hints: high-confidence Person spans from a dedicated NER model.
    // Injected as a preamble block so the LLM treats them as validated anchors.
    let hints_block = match gliner_hints {
        Some(hints) if !hints.is_empty() => {
            let list = hints
                .iter()
                .map(|h| format!("- {h}"))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "CONFIRMED PERSON NAMES (detected by a dedicated NER model — treat these as \
                 high-confidence Person entities if they appear in the text):\n{list}\n\n"
            )
        }
        _ => String::new(),
    };

    let prompt = if no_relations {
        format!(
            "{section_context}\
             {pronoun_context}\
             You are a precise knowledge extraction engine.\n\
             The following proper noun candidates were identified in the text.\n\
             Classify each as a named entity (keep) or discard it if it is not a real entity.\n\
             For kept entities output: name, type, and structured fields.\n\
             List AT MOST {entity_cap} entities.\n\
             Return ONLY valid JSON (no markdown, no explanation):\n\
             {{\"entities\":[{{\"name\":\"...\",\"type\":\"...\",\"fields\":{{...}},\"extraction_confidence\":0.9}},...]}}\n\n\
             {hints_block}\
             Candidates:\n{candidates_block}\n\n\
             Entity types: {entity_list}\n\n\
             Field keys by entity type — include only keys whose values are explicitly stated in this passage:\n\
               Person:       birthDate, birthPlace, deathDate, nationality, occupation, \
                             affiliation, spouse, parent, sibling, child\n\
               Place:        locationType, historicalNote\n\
               Organization: foundingDate, dissolutionDate, founder, orgType\n\n\
             extraction_confidence: 1.0 = entity is explicitly named and described in the text;\
 0.5 = inferred from context or partially described; 0.2 = uncertain, might be a hallucination.\n\n\
             IMPORTANT RULES:\n\
             - Never create an entity whose name is a pronoun or generic role.\n\
             - Only keep candidates that are real proper names, organisations, or places.\n\
             - Entity names must exactly match a candidate from the list above. \
Do NOT append dates, page numbers, volume references, or citation details to entity names \
(e.g. if the candidate is \"Indian Opinion\", the entity name is \"Indian Opinion\" — \
not \"Indian Opinion Dec 29, 1906 p986\").\n\
             - Entity names must be ≤ 5 words. If a candidate contains multiple names \
separated by commas or 'and', extract each as its own entity.\n\
             - Descriptions must contain at least one specific fact (date, place, role, or \
relationship) from the text. Do not describe in generic terms.\n\
             - Field values must be verbatim quotes or direct paraphrases from THIS passage. \
Do NOT use background knowledge to fill in geographic locations, dates, or other attributes \
not stated in the text.\n\
             - NEVER extract generic family roles as entity names. \"Uncle Aity\", \
\"Auntie Cissie\", \"Granny Bibi\" are NOT valid entity names — skip them. Only extract \
proper names (first name + family name, or a well-known single name).\n\
             - If a name appears ONLY as the author of a literary work being read or cited \
(e.g. Chekhov, Dickens, Shaw, Homer, Milton, Dostoevsky, Gogol, Gorki, Zola, Steinbeck, \
Wordsworth, Browning, Jack London, Mark Twain) it is NOT an entity — skip it.\n\
             - Collective nouns (\"the servants\", \"the uncles\", \"the family\") and bare \
titles (\"the Imam\", \"the Doctor\") are NOT Person entities.\n\
             - Do NOT extract ethnic or racial group nouns as Person entities: African, Indian, \
Arab, Chinese, Bantu, Boer, Cape Malay, Coolie, Dutch, Griqua, Hindu, Irish, Japanese, \
Malay, Non-White, Pathan, Punjabi, Sikh, Turk, West Indian, Zulu, Afrikaner.\n\
             - Do NOT extract ideological or political labels: Nationalist, Socialist, \
Marxist, Nazi, Communist, Labour, Victorian, Native.\n\
             - Do NOT extract fictional characters from comics or films even if the memoir \
mentions reading/watching them: Tarzan, Flash, Buck Rogers, Buck Jones, Dandy, Globi, \
Lobo, Brick Bradford, Hopalong Cassidy, Roy Rogers, Gene Autry, Cobra Woman, Ali Baba, \
Banquo, Dorian Gray, Mephistopheles, Hunchback of Notre Dame.\n\
             - Do NOT extract common English words or sentence fragments as entity names: \
Apart, Being, Figure, Hatless, History, Just, Later, Little, Much, Now, Perhaps, \
Regrettably, Science, Several, Soon, Still, Tell, Whether, Worse.\n\
             - Do NOT fuse a fictional character with its author: \"King Lear\" and \
\"William Shakespeare\" are separate — do not output \"King Lear William Shakespeare\".\n\
             - Do NOT fuse a list of names into one entity. If the source text has \
\"A, B, C and D\" extract each as a separate entity or skip all.\n\n\
             If no candidates are real entities, return {{\"entities\":[]}}.\n\n\
             Text:\n{text}"
        )
    } else {
        let person_only = entity_types.len() == 1 && entity_types[0].eq_ignore_ascii_case("Person");
        let relation_list = if person_only {
            PERSON_RELATION_TYPES.join(", ")
        } else {
            RELATION_TYPES.join(", ")
        };
        format!(
            "{section_context}\
             {pronoun_context}\
             You are a precise knowledge extraction engine.\n\
             The following proper noun candidates were identified in the text.\n\
             Classify each as a named entity (keep) or discard it if it is not a real entity,\n\
             then extract relationships between kept entities.\n\
             List AT MOST {entity_cap} entities.\n\
             Return ONLY valid JSON (no markdown, no explanation):\n\
             {{\"entities\":[{{\"name\":\"...\",\"type\":\"...\",\"description\":\"1-2 sentences\",\"extraction_confidence\":0.9}},...],\
             \"relations\":[{{\"from\":\"entity name\",\"to\":\"entity name\",\"relation\":\"relation_type\"}},...]}}\n\n\
             {hints_block}\
             Candidates:\n{candidates_block}\n\n\
             Entity types: {entity_list}\n\
             Relation types: {relation_list}\n\n\
             extraction_confidence: 1.0 = entity is explicitly named and described in the text;\
 0.5 = inferred from context or partially described; 0.2 = uncertain, might be a hallucination.\n\n\
             IMPORTANT RULES:\n\
             - Never create an entity whose name is a pronoun or generic role.\n\
             - Only keep candidates that are real proper names, organisations, or places.\n\
             - Entity names must exactly match a candidate from the list above. \
Do NOT append dates, page numbers, volume references, or citation details to entity names.\n\
             - Entity names must be ≤ 5 words. If a candidate contains multiple names \
separated by commas or 'and', extract each as its own entity.\n\
             - Descriptions must contain at least one specific fact (date, place, role, or \
relationship) from the text. Do not describe in generic terms.\n\
             - NEVER extract generic family roles as entity names. \"Uncle Aity\", \
\"Auntie Cissie\", \"Granny Bibi\" are NOT valid entity names — skip them. Only extract \
proper names (first name + family name, or a well-known single name).\n\
             - If a name appears ONLY as the author of a literary work being read or cited \
(e.g. Chekhov, Dickens, Shaw, Homer, Dostoevsky, Gogol, Gorki, Zola, Steinbeck, \
Wordsworth, Browning, Jack London, Mark Twain) it is NOT an entity — skip it.\n\
             - Do NOT extract ethnic or racial group nouns as Person entities: African, Indian, \
Arab, Chinese, Bantu, Boer, Cape Malay, Coolie, Dutch, Griqua, Hindu, Irish, Japanese, \
Malay, Non-White, Pathan, Punjabi, Sikh, Turk, West Indian, Zulu, Afrikaner.\n\
             - Do NOT extract ideological or political labels: Nationalist, Socialist, \
Marxist, Nazi, Communist, Labour, Victorian, Native.\n\
             - Do NOT extract fictional characters from comics or films even if the memoir \
mentions reading/watching them: Tarzan, Flash, Buck Rogers, Buck Jones, Dandy, Globi, \
Lobo, Brick Bradford, Hopalong Cassidy, Roy Rogers, Gene Autry, Cobra Woman, Ali Baba, \
Banquo, Dorian Gray, Mephistopheles.\n\
             - Do NOT extract common English words or sentence fragments: Apart, Being, \
Figure, Hatless, History, Just, Later, Little, Much, Now, Perhaps, Several, Soon, Still, \
Tell, Whether, Worse.\n\
             - Do NOT fuse a list of names into one entity. Extract each name separately.\n\
             - Only assert a relation when the text EXPLICITLY STATES IT. Do not infer \
relations from two people being mentioned in the same paragraph.\n\
             - Use `spouse_of` ONLY when the text says \"married\", \"wife\", \"husband\", \
\"wed\", or \"betrothed\".\n\
             - Use `child_of`/`parent_of` ONLY when the text says \"son of\", \"daughter of\", \
\"mother of\", \"father of\", or \"born to\".\n\
             - Two people who share a common spouse are NOT `sibling_of` or `spouse_of` \
each other — use `associated_with` at most.\n\
             - Do not create relations to generic roles (\"Dad\", \"Granny\") or to a person \
who appears only as an author of a literary work.\n\
             - Do NOT fuse a fictional character with its author: keep them as separate \
entities or omit the fictional one entirely.\n\n\
             If no candidates are real entities, return {{\"entities\":[],\"relations\":[]}}.\n\n\
             Text:\n{text}"
        )
    };

    let url = format!("{}/api/chat", inference_url.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()?;

    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        // stream: true so Ollama sends headers immediately on first token.
        // .send() returns in seconds (relay latency only) instead of blocking
        // until the full generation completes, eliminating 90s send timeouts.
        "stream": true,
        "options": {
            "temperature": 0.1,
            "num_predict": 1024,
            "num_ctx": 8192,
        },
    });

    // Note: the P2P relay buffers the full response before returning headers,
    // so this timeout covers the full round-trip (relay + generation), not
    // just connection setup. 120s gives headroom for slow chunks on loaded GPUs.
    let send_result = tokio::time::timeout(
        std::time::Duration::from_secs(120),
        client.post(&url).json(&body).send(),
    )
    .await;
    let resp = match send_result {
        Ok(Ok(r)) => r,
        Ok(Err(e)) => {
            tracing::warn!("entity extraction request failed: {e}");
            return Ok((vec![], vec![]));
        }
        Err(_) => {
            tracing::warn!("entity extraction send timed out after 120s");
            return Ok((vec![], vec![]));
        }
    };

    if !resp.status().is_success() {
        tracing::warn!("entity extraction got HTTP {}", resp.status());
        return Ok((vec![], vec![]));
    }

    // Read full streaming NDJSON body: each line is {"message":{"content":"..."},"done":bool}
    let raw_text =
        match tokio::time::timeout(std::time::Duration::from_secs(120), resp.text()).await {
            Ok(Ok(t)) => t,
            Ok(Err(e)) => {
                tracing::warn!("entity extraction body read error: {e}");
                return Ok((vec![], vec![]));
            }
            Err(_) => {
                tracing::warn!("entity extraction body read timed out after 120s");
                return Ok((vec![], vec![]));
            }
        };

    // Accumulate content tokens from each streaming chunk
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

    let content = content_buf.trim();
    let cleaned = content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    match serde_json::from_str::<ExtractionPayload>(cleaned) {
        Ok(p) => {
            let entities = p
                .entities
                .into_iter()
                .map(|mut e| {
                    e.name = clean_entity_name(&e.name);
                    e
                })
                .collect();
            let relations = if no_relations {
                vec![]
            } else {
                p.relations
                    .into_iter()
                    .map(|mut r| {
                        r.from = clean_entity_name(&r.from);
                        r.to = clean_entity_name(&r.to);
                        r
                    })
                    .collect()
            };
            Ok((entities, relations))
        }
        Err(e) => {
            tracing::debug!("entity extraction JSON parse failed: {e}; raw: {cleaned:.200}");
            Ok((vec![], vec![]))
        }
    }
}

/// Fix PDF-extraction underscore artifacts in entity names extracted by the LLM.
///
/// PDF text extraction commonly produces:
///   "Dr_"       instead of "Dr."    (period after title → underscore)
///   "J_ M_"     instead of "J. M."  (spaced initials)
///   "M_K_"      instead of "M.K."   (chained initials)
///   "Wooding_s" instead of "Wooding's" (apostrophe-s → underscore-s)
///
/// Rules applied in order:
///   1. `WORD_s` at a word boundary → `WORD's`   (possessive/contraction)
///   2. `_` preceded by a letter and followed by whitespace, end, or uppercase → `.`
///   3. Remaining lone underscores → stripped
pub fn clean_entity_name(name: &str) -> String {
    // Pre-pass: normalise typographic single quotes (PDF OCR) to ASCII apostrophe,
    // then convert OCR parenthetical patterns " _Word_ " → " (Word) "
    let name_owned;
    let name = {
        let mut result: String = name
            .chars()
            .map(|c| match c {
                '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => '\'',
                other => other,
            })
            .collect();
        loop {
            let b = result.as_bytes().to_vec();
            let mut found: Option<(usize, usize)> = None;
            let mut i = 0;
            while i < b.len() {
                if b[i] == b'_' && (i == 0 || b[i - 1] == b' ') {
                    let mut j = i + 1;
                    while j < b.len() {
                        if b[j] == b'_' && j > i + 1 && (j + 1 >= b.len() || b[j + 1] == b' ') {
                            found = Some((i, j));
                            break;
                        }
                        j += 1;
                    }
                }
                if found.is_some() {
                    break;
                }
                i += 1;
            }
            match found {
                Some((open, close)) => {
                    let content = result[open + 1..close].to_string();
                    result = format!("{}({}){}", &result[..open], content, &result[close + 1..]);
                }
                None => break,
            }
        }
        name_owned = result;
        name_owned.as_str()
    };
    let mut out = String::with_capacity(name.len());
    let chars: Vec<char> = name.chars().collect();
    let n = chars.len();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        if c == '_' {
            // Rule 1: `_s` at word boundary → `'s`
            if i + 1 < n && chars[i + 1] == 's' {
                let after_s = i + 2;
                let is_boundary = after_s >= n
                    || !chars[after_s].is_alphabetic()
                    || chars[after_s].is_uppercase();
                if is_boundary {
                    out.push('\'');
                    out.push('s');
                    i += 2;
                    continue;
                }
            }
            // Rule 2: `_` preceded by a letter and followed by whitespace, end,
            // or an uppercase letter (chained initials like M_K_ → M.K.) → period
            let prev_is_alpha = out
                .chars()
                .last()
                .map(|p| p.is_alphabetic())
                .unwrap_or(false);
            let next_is_break_or_initial =
                i + 1 >= n || chars[i + 1] == ' ' || chars[i + 1].is_uppercase();
            if prev_is_alpha && next_is_break_or_initial {
                out.push('.');
                i += 1;
                continue;
            }
            // Rule 3: strip remaining underscores (don't emit anything)
            i += 1;
        } else {
            out.push(c);
            i += 1;
        }
    }
    // Collapse any double spaces created by stripping
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn relation_key(src: i64, dst: i64, rel_type: &str) -> Vec<u8> {
    let mut k = Vec::with_capacity(16 + rel_type.len());
    k.extend_from_slice(&src.to_le_bytes());
    k.extend_from_slice(&dst.to_le_bytes());
    k.extend_from_slice(rel_type.as_bytes());
    k
}

/// Jaro-Winkler string similarity in [0, 1]. Applied as a name-level gate in Tier 2
/// dedup before the more expensive cosine similarity check. Operates on char sequences.
fn jaro_winkler(s1: &str, s2: &str) -> f32 {
    let c1: Vec<char> = s1.chars().collect();
    let c2: Vec<char> = s2.chars().collect();
    let (l1, l2) = (c1.len(), c2.len());
    if l1 == 0 && l2 == 0 {
        return 1.0;
    }
    if l1 == 0 || l2 == 0 {
        return 0.0;
    }
    if c1 == c2 {
        return 1.0;
    }
    let win = (l1.max(l2) / 2).saturating_sub(1);
    let mut m1 = vec![false; l1];
    let mut m2 = vec![false; l2];
    let mut matches = 0usize;
    for i in 0..l1 {
        let lo = i.saturating_sub(win);
        let hi = (i + win + 1).min(l2);
        for j in lo..hi {
            if !m2[j] && c1[i] == c2[j] {
                m1[i] = true;
                m2[j] = true;
                matches += 1;
                break;
            }
        }
    }
    if matches == 0 {
        return 0.0;
    }
    let mut t = 0usize;
    let mut k = 0usize;
    for i in 0..l1 {
        if m1[i] {
            while !m2[k] {
                k += 1;
            }
            if c1[i] != c2[k] {
                t += 1;
            }
            k += 1;
        }
    }
    let m = matches as f64;
    let jaro = (m / l1 as f64 + m / l2 as f64 + (m - t as f64 / 2.0) / m) / 3.0;
    let prefix = c1
        .iter()
        .zip(c2.iter())
        .take(4)
        .take_while(|(a, b)| a == b)
        .count();
    (jaro + prefix as f64 * 0.1 * (1.0 - jaro)) as f32
}

/// Returns true if the name carries an explicit disambiguation marker that means it
/// refers to a distinct individual and should never be auto-merged with a similarly-
/// named entity (e.g. "John Smith (novelist)" or "John Smith III").
fn has_disambiguation_token(name: &str) -> bool {
    let t = name.trim_end();
    if t.ends_with(')') && t.rfind('(').is_some_and(|p| p > 0) {
        return true;
    }
    matches!(
        name.split_whitespace().last().unwrap_or(""),
        "II" | "III" | "IV" | "VI" | "VII" | "VIII" | "IX"
    )
}

/// Tokenise a description into significant words (≥4 chars, not stop words).
fn desc_sig_words(desc: &str) -> impl Iterator<Item = String> + '_ {
    const DESC_STOP: &[&str] = &[
        "that", "this", "with", "from", "have", "been", "were", "they", "their", "also", "well",
        "known", "very", "when", "some", "many", "most", "more", "than", "into", "about", "after",
        "which", "would", "could", "should", "other", "over",
    ];
    desc.split(|c: char| !c.is_alphabetic())
        .filter(|w| w.len() >= 4)
        .map(|w| w.to_lowercase())
        .filter(move |w| !DESC_STOP.contains(&w.as_str()))
}

fn cosine_sim_f32(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let dot: f64 = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| x as f64 * y as f64)
        .sum();
    let na: f64 = a.iter().map(|&x| x as f64 * x as f64).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|&x| x as f64 * x as f64).sum::<f64>().sqrt();
    if na == 0.0 || nb == 0.0 {
        return 0.0;
    }
    (dot / (na * nb)).clamp(-1.0, 1.0) as f32
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0usize; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            curr[j] = if a[i - 1] == b[j - 1] {
                prev[j - 1]
            } else {
                1 + prev[j - 1].min(prev[j]).min(curr[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

fn update_adj(
    adj: &mut HashMap<i64, Vec<(i64, String, f32)>>,
    from: i64,
    to: i64,
    rel: &str,
    strength: f32,
) {
    let entry = adj.entry(from).or_default();
    if let Some(pos) = entry.iter().position(|(d, r, _)| *d == to && r == rel) {
        entry[pos].2 = strength;
    } else {
        entry.push((to, rel.to_string(), strength));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_entity_name_chained_initials() {
        assert_eq!(clean_entity_name("M_K_ Gandhi"), "M.K. Gandhi");
        assert_eq!(clean_entity_name("E_S_ Reddy"), "E.S. Reddy");
        assert_eq!(clean_entity_name("J_ M_ H_ Gool"), "J. M. H. Gool");
        assert_eq!(clean_entity_name("Mrs_ Wo"), "Mrs. Wo");
        assert_eq!(clean_entity_name("Joe Rassool_s"), "Joe Rassool's");
        assert_eq!(clean_entity_name("Tykie_s"), "Tykie's");
        // Clean names pass through unchanged
        assert_eq!(clean_entity_name("Gandhi"), "Gandhi");
        assert_eq!(clean_entity_name("Goolam Gool"), "Goolam Gool");
    }

    #[test]
    fn test_clean_entity_name_unicode_quotes() {
        // U+2019 right single quotation mark (PDF apostrophe)
        assert_eq!(
            clean_entity_name("Grandpa\u{2019}s daughter"),
            "Grandpa's daughter"
        );
        assert_eq!(clean_entity_name("Granny\u{2019}s niece"), "Granny's niece");
        assert_eq!(clean_entity_name("Y\u{2019}Allah"), "Y'Allah");
        // U+2018/U+2019 as nickname delimiters
        assert_eq!(
            clean_entity_name("Pharaoh \u{2018}Cheops\u{2019}"),
            "Pharaoh 'Cheops'"
        );
        // underscore possessive still works
        assert_eq!(
            clean_entity_name("Grandpa_s daughter"),
            "Grandpa's daughter"
        );
        // parenthetical
        assert_eq!(
            clean_entity_name("Yousuf _Joe_ Rassool"),
            "Yousuf (Joe) Rassool"
        );
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("helen", "hassen"), 3);
        assert_eq!(levenshtein_distance("yousuf", "yousuf"), 0);
        assert_eq!(levenshtein_distance("hassen", "hassan"), 1);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        // "helen" vs "hassen": dist=3, max=6 → 3*2=6 >= 6 → would cap sim
        let dist = levenshtein_distance("helen", "hassen");
        let max_len = "helen".len().max("hassen".len());
        assert!(
            dist * 2 >= max_len,
            "Helen/Hassen should trigger same-surname guard"
        );
        // "hassen" vs "hassan": dist=1, max=6 → 1*2=2 < 6 → would NOT cap
        let dist2 = levenshtein_distance("hassen", "hassan");
        let max2 = "hassen".len().max("hassan".len());
        assert!(
            dist2 * 2 < max2,
            "Hassen/Hassan variant spelling should not be capped"
        );
    }
}
