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
    "endorses",
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
            ("addressLocality", "city, district or suburb"),
            ("addressRegion", "province or region"),
            ("addressCountry", "country"),
            (
                "locationType",
                "type of place (district, city, country, neighbourhood)",
            ),
            ("historicalNote", "historical significance or period"),
        ],
        "Organization" => &[
            ("foundingDate", "year or period when founded"),
            (
                "dissolutionDate",
                "year or period when dissolved, if applicable",
            ),
            ("location", "city or country of headquarters or main office"),
            ("founder", "founder name"),
            (
                "orgType",
                "type of organization (school, mosque, political party, etc.)",
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
                .filter(|fv| !fv.value.is_empty())
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
fn ord_pair(a: i64, b: i64) -> (i64, i64) {
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
                // Recompute description from merged fields, or keep the longer prose description.
                let computed =
                    description_from_fields(&existing.name, &existing.entity_type, &merged_fields);
                let best_desc = if !computed.is_empty() {
                    computed
                } else if node.description.len() > existing.description.len() {
                    node.description.clone()
                } else {
                    existing.description.clone()
                };
                let best_emb = if best_desc == existing.description {
                    existing.embedding.clone()
                } else {
                    node.embedding.clone()
                };
                EntityNode {
                    id: node.id,
                    name: existing.name.clone(),
                    entity_type: existing.entity_type.clone(),
                    description: best_desc,
                    embedding: best_emb,
                    mention_count: existing.mention_count + 1,
                    first_chunk_id: existing.first_chunk_id,
                    aliases: existing.aliases.clone(),
                    schema_type: existing.schema_type.clone().or(node.schema_type.clone()),
                    gender: existing.gender.clone().or(node.gender.clone()),
                    evidence: existing.evidence.clone(),
                    fields: merged_fields,
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
        if FAMILIAL_RELS.contains(&relation_type) {
            let src_is_person = self
                .nodes
                .get(&src_id)
                .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                .unwrap_or(false);
            let dst_is_person = self
                .nodes
                .get(&dst_id)
                .map(|n| n.entity_type.eq_ignore_ascii_case("person"))
                .unwrap_or(false);
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
    pub fn outgoing_relations(&self, entity_id: i64) -> Result<Vec<(i64, String, f32)>> {
        let rtxn = self.db.begin_read()?;
        let table = rtxn.open_table(RELATIONS_TABLE)?;
        let prefix: Vec<u8> = entity_id.to_le_bytes().to_vec();
        let mut out = Vec::new();
        for item in table.iter()? {
            let (k, v) = item?;
            if k.value().starts_with(&prefix) {
                if let Ok(rel) = serde_json::from_slice::<RelationRecord>(v.value()) {
                    out.push((rel.dst_id, rel.relation_type, rel.strength));
                }
            }
        }
        Ok(out)
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
                    if normalize_name(&na.name) == normalize_name(&nb.name) {
                        continue; // exact matches handled by Tier 1
                    }
                    let sim = cosine_sim_f32(&na.embedding, &nb.embedding);
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
                    key_map.entry(key).or_default().push(id);
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
                    let key = ord_pair(alias, canonical);
                    if seen.insert(key) {
                        out.push((alias, canonical, "honorific"));
                    }
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

        out
    }

    /// Tier 4: Role-pronoun neighbour-containment dedup.
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
        Ok(())
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

    pub fn rebuild_in_memory(&mut self) -> Result<()> {
        self.nodes.clear();
        self.adj.clear();
        self.chunk_to_entities.clear();
        self.entity_to_chunks.clear();
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
        let texts: Vec<String> = ids
            .iter()
            .map(|id| {
                let n = &self.nodes[id];
                Self::entity_embed_text(&n.name, &n.aliases, &n.description)
            })
            .collect();

        let text_refs: Vec<&str> = texts.iter().map(|s| s.as_str()).collect();
        let embeddings = embed.embed_batch(&text_refs).await?;

        let wtxn = self.db.begin_write()?;
        {
            let mut t = wtxn.open_table(ENTITIES_TABLE)?;
            for (id, emb) in ids.iter().zip(embeddings) {
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
/// ingestion can continue without hard errors.
pub async fn extract_from_text(
    text: &str,
    candidates: &[String],
    pronoun_map: &[(String, String)],
    section_note: Option<&str>,
    inference_url: &str,
    model: &str,
    entity_types: &[&str],
    no_relations: bool,
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

    let candidates_block = candidates
        .iter()
        .map(|c| format!("- {c}"))
        .collect::<Vec<_>>()
        .join("\n");

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
             {{\"entities\":[{{\"name\":\"...\",\"type\":\"...\",\"fields\":{{...}}}},...]}}\n\n\
             Candidates:\n{candidates_block}\n\n\
             Entity types: {entity_list}\n\n\
             Field keys by entity type — include only keys whose values appear in the text:\n\
               Person:       birthDate, birthPlace, deathDate, nationality, occupation, \
                             affiliation, spouse, parent, sibling, child\n\
               Place:        addressLocality, addressRegion, addressCountry, locationType, \
                             historicalNote\n\
               Organization: foundingDate, dissolutionDate, location, founder, orgType\n\n\
             IMPORTANT RULES:\n\
             - Never create an entity whose name is a pronoun or generic role.\n\
             - Only keep candidates that are real proper names, organisations, or places.\n\
             - Omit any field whose value is not clearly stated in the text.\n\n\
             If no candidates are real entities, return {{\"entities\":[]}}.\n\n\
             Text:\n{text}"
        )
    } else {
        let relation_list = RELATION_TYPES.join(", ");
        format!(
            "{section_context}\
             {pronoun_context}\
             You are a precise knowledge extraction engine.\n\
             The following proper noun candidates were identified in the text.\n\
             Classify each as a named entity (keep) or discard it if it is not a real entity,\n\
             then extract relationships between kept entities.\n\
             List AT MOST {entity_cap} entities.\n\
             Return ONLY valid JSON (no markdown, no explanation):\n\
             {{\"entities\":[{{\"name\":\"...\",\"type\":\"...\",\"description\":\"1-2 sentences\"}},...],\
             \"relations\":[{{\"from\":\"entity name\",\"to\":\"entity name\",\"relation\":\"relation_type\"}},...]}}\n\n\
             Candidates:\n{candidates_block}\n\n\
             Entity types: {entity_list}\n\
             Relation types: {relation_list}\n\n\
             IMPORTANT RULES:\n\
             - Never create an entity whose name is a pronoun or generic role.\n\
             - Only keep candidates that are real proper names, organisations, or places.\n\n\
             If no candidates are real entities, return {{\"entities\":[],\"relations\":[]}}.\n\n\
             Text:\n{text}"
        )
    };

    let url = format!(
        "{}/v1/chat/completions",
        inference_url.trim_end_matches('/')
    );
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()?;

    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.1,
        "max_tokens": 1024,
    });

    let send_result = tokio::time::timeout(
        std::time::Duration::from_secs(90),
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
            tracing::warn!("entity extraction send timed out after 90s");
            return Ok((vec![], vec![]));
        }
    };

    if !resp.status().is_success() {
        tracing::warn!("entity extraction got HTTP {}", resp.status());
        return Ok((vec![], vec![]));
    }

    let v: serde_json::Value =
        match tokio::time::timeout(std::time::Duration::from_secs(120), resp.json()).await {
            Ok(Ok(v)) => v,
            Ok(Err(e)) => {
                tracing::warn!("entity extraction parse error: {e}");
                return Ok((vec![], vec![]));
            }
            Err(_) => {
                tracing::warn!("entity extraction body read timed out after 120s");
                return Ok((vec![], vec![]));
            }
        };

    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("{}");
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
///   "Dr_"      instead of "Dr."   (period after title/initial → underscore)
///   "J_ M_"    instead of "J. M." (initials lose their periods)
///   "Wooding_s" instead of "Wooding's" (apostrophe-s → underscore-s)
///
/// Rules applied in order:
///   1. `WORD_s` at a word boundary → `WORD's`   (possessive/contraction)
///   2. `LETTER_` followed by space, end, or another initial pattern → `LETTER.`
///   3. Remaining lone underscores → stripped
fn clean_entity_name(name: &str) -> String {
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
            // Rule 2: single letter before `_`, followed by space/end/next initial
            let prev_is_single_letter = out
                .chars()
                .last()
                .map(|p| p.is_alphabetic())
                .unwrap_or(false)
                && !out.is_empty()
                && {
                    // check the previous char was preceded by a space or start
                    let ob: &[u8] = out.as_bytes();
                    ob.len() == 1 || ob[ob.len() - 2] == b' ' || ob[ob.len() - 2] == b'.'
                };
            let next_is_space_or_end = i + 1 >= n || chars[i + 1] == ' ';
            if prev_is_single_letter && next_is_space_or_end {
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
