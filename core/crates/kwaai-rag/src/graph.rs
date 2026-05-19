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

// ── Core types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityNode {
    /// Deterministic: sha256(name.lower() + "::" + entity_type)[..8] as i64 LE.
    pub id: i64,
    pub name: String,
    pub entity_type: String,
    /// 1–2 sentence LLM-generated summary.
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
    pub description: String,
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
            Some(existing) => EntityNode {
                id: node.id,
                name: existing.name.clone(),
                entity_type: existing.entity_type.clone(),
                description: if node.description.len() > existing.description.len() {
                    node.description.clone()
                } else {
                    existing.description.clone()
                },
                embedding: if node.description.len() > existing.description.len() {
                    node.embedding.clone()
                } else {
                    existing.embedding.clone()
                },
                mention_count: existing.mention_count + 1,
                first_chunk_id: existing.first_chunk_id,
                aliases: existing.aliases.clone(),
                schema_type: existing.schema_type.clone().or(node.schema_type.clone()),
            },
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

        // Update in-memory adjacency (both directions).
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
                    let merged_description = if alias_description.len() > canonical.description.len() {
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
        for cid in alias_mem_chunks {
            if let Some(eids) = self.chunk_to_entities.get_mut(&cid) {
                eids.retain(|&id| id != alias_id);
                if !eids.contains(&canonical_id) {
                    eids.push(canonical_id);
                }
            }
            let ec = self.entity_to_chunks.entry(canonical_id).or_default();
            if !ec.contains(&cid) {
                ec.push(cid);
            }
        }
        // Leave adj stale — caller should call rebuild_in_memory() after a batch.

        Ok(n_moved)
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
        let stop: &[&str] = &[
            "the", "and", "of", "in", "a", "an", "for", "at", "by", "to", "dr", "mr", "mrs", "ms",
            "prof", "sir",
        ];

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
    inference_url: &str,
    model: &str,
) -> Result<(Vec<ExtractedEntity>, Vec<ExtractedRelation>)> {
    let entity_list = ENTITY_TYPES.join(", ");
    let relation_list = RELATION_TYPES.join(", ");

    let prompt = format!(
        "You are a precise knowledge extraction engine.\n\
         Extract named entities and relationships from the text below.\n\
         Return ONLY valid JSON matching this schema (no markdown, no explanation):\n\
         {{\"entities\":[{{\"name\":\"...\",\"type\":\"...\",\"description\":\"1-2 sentences\"}},...],\
         \"relations\":[{{\"from\":\"entity name\",\"to\":\"entity name\",\"relation\":\"relation_type\"}},...]}}\n\n\
         Entity types: {entity_list}\n\
         Relation types: {relation_list}\n\n\
         IMPORTANT RULES:\n\
         - Never create an entity whose name is a pronoun or generic role: \
           do NOT use names like \"I\", \"me\", \"my\", \"he\", \"she\", \"they\", \
           \"narrator\", \"author\", \"writer\", \"the author\", \"the narrator\", \
           \"the writer\", \"speaker\", \"subject\".\n\
         - If the text uses \"I\" or \"the author\" to refer to a named person, \
           use that person's actual name as the entity name instead.\n\
         - Only extract entities that have a real proper name or a specific \
           organisation/place/event name.\n\n\
         If no clear entities exist, return {{\"entities\":[],\"relations\":[]}}.\n\n\
         Text:\n{text}"
    );

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
        std::time::Duration::from_secs(30),
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
            tracing::warn!("entity extraction send timed out after 30s");
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
        Ok(p) => Ok((p.entities, p.relations)),
        Err(e) => {
            tracing::debug!("entity extraction JSON parse failed: {e}; raw: {cleaned:.200}");
            Ok((vec![], vec![]))
        }
    }
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
