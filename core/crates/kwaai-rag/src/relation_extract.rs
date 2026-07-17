//! Axiomatic relation candidate classification — the relation-side counterpart of
//! `axiom_extract.rs`'s entity classifier, replacing the coarse per-chunk boolean
//! `graph::lexical_relation_trigger()` gate with a per-candidate, three-way confidence
//! split: high-confidence lexical matches commit directly to the graph, medium-confidence
//! matches go to a narrow LLM confirm/reject/retype call, low-confidence matches are
//! dropped before ever reaching an LLM.
//!
//! The window-scan algorithm generalizes `sequence::extract_kinship_interactions()`
//! (previously kinship-only, feeding only the Mermaid timeline view) to the fuller
//! Family/Agent/Spatial trigger vocabulary, and to write real graph relations via
//! `graph::GraphStore::upsert_relation_with_confidence()`.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::graph::{family_role_contradicts, ord_pair, FAMILIAL_RELS};

// ---------------------------------------------------------------------------
// Classification types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RelationClassificationMethod {
    FamilyTrigger,
    AgentTrigger,
    SpatialTrigger,
    LlmVerified,
    Unknown,
}

impl RelationClassificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FamilyTrigger => "FamilyTrigger",
            Self::AgentTrigger => "AgentTrigger",
            Self::SpatialTrigger => "SpatialTrigger",
            Self::LlmVerified => "LlmVerified",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedRelationCandidate {
    pub subject_id: i64,
    pub object_id: i64,
    pub subject_name: String,
    pub object_name: String,
    pub relation_type: String,
    /// How specific/unambiguous the matched trigger phrase is (0.90 for multi-word
    /// family phrases, 0.70 for shorter agent/spatial phrases).
    pub trigger_confidence: f32,
    /// How unambiguous the two flanking entities were (0.95 unambiguous, 0.70 when
    /// another candidate entity sat within the tie margin on either side).
    pub proximity_confidence: f32,
    /// trigger_confidence × proximity_confidence.
    pub composite_confidence: f32,
    pub method: RelationClassificationMethod,
    /// Whether the trigger phrase required a subject/object swap (e.g. "founded by").
    pub reversed: bool,
    /// How many *other* candidate entities sat within the tie margin of the winning
    /// entity on either side of the trigger — feeds the ambiguous-window axiom.
    pub ambiguity_count: u8,
    pub source_sentence: String,
    pub chunk_id: i64,
}

/// Closed relation-type vocabulary Phase 4 operates over — the union of every
/// relation_type produced by the trigger tables below. Used both to build the LLM
/// verify prompt's whitelist and to validate `retype_as` values in its response,
/// so a "corrected" type from the LLM can never introduce an out-of-scope type.
/// Avuncular relations (uncle_of/aunt_of/nephew_of/niece_of/cousin_of) are
/// deliberately excluded — the existing manual `graph extract-relations` command
/// already tried and backed away from them for precision reasons.
pub const IN_SCOPE_RELATION_TYPES: &[&str] = &[
    "spouse_of",
    "parent_of",
    "child_of",
    "sibling_of",
    "half_sibling_of",
    "grandparent_of",
    "grandchild_of",
    "foster_parent_of",
    "foster_child_of",
    "member_of",
    "founded",
    "affiliated_with",
    "associated_with",
    "works_at",
    "lived_in",
];

// ---------------------------------------------------------------------------
// Trigger tables
// ---------------------------------------------------------------------------

/// Multi-word, high-specificity family/kinship phrases → relation_type.
/// Consolidates `sequence::KINSHIP_KEYWORDS` and the FAMILY list from
/// `graph::lexical_relation_trigger()` into one type-mapped table.
const FAMILY_RELATION_TRIGGERS: &[(&str, &str)] = &[
    ("son of", "child_of"),
    ("daughter of", "child_of"),
    ("born to", "child_of"),
    ("wife of", "spouse_of"),
    ("husband of", "spouse_of"),
    ("married to", "spouse_of"),
    ("married ", "spouse_of"), // trailing space avoids matching "unmarried"
    ("widow of", "spouse_of"),
    ("widower of", "spouse_of"),
    ("father of", "parent_of"),
    ("mother of", "parent_of"),
    ("brother of", "sibling_of"),
    ("sister of", "sibling_of"),
    ("half-brother of", "half_sibling_of"),
    ("half-sister of", "half_sibling_of"),
    ("half brother of", "half_sibling_of"),
    ("half sister of", "half_sibling_of"),
    ("grandfather of", "grandparent_of"),
    ("grandmother of", "grandparent_of"),
    ("grandson of", "grandchild_of"),
    ("granddaughter of", "grandchild_of"),
    ("foster son of", "foster_child_of"),
    ("foster daughter of", "foster_child_of"),
    ("foster father of", "foster_parent_of"),
    ("foster mother of", "foster_parent_of"),
];

/// Membership / founding / affiliation phrases → relation_type.
const AGENT_RELATION_TRIGGERS: &[(&str, &str)] = &[
    ("member of", "member_of"),
    ("belongs to", "member_of"),
    ("belonged to", "member_of"),
    ("founded by", "founded"),
    ("co-founder of", "founded"),
    ("affiliated with", "affiliated_with"),
    ("associated with", "associated_with"),
    ("works at", "works_at"),
    ("worked at", "works_at"),
];

/// Spatial / biographical residence phrases → relation_type.
const SPATIAL_RELATION_TRIGGERS: &[(&str, &str)] = &[
    ("lived in", "lived_in"),
    ("moved to", "lived_in"),
    ("relocated to", "lived_in"),
];

/// Trigger phrases where the object precedes the subject in the sentence structure
/// (e.g. "X founded by Y" → Y founded X), generalizing `sequence::kinship_is_reversed`.
const REVERSED_TRIGGERS: &[&str] = &["founded by", "born to"];

fn find_best_trigger(
    s_lower: &str,
) -> Option<(usize, usize, &'static str, &'static str, RelationClassificationMethod, f32)> {
    let mut matches: Vec<(usize, usize, &'static str, &'static str, RelationClassificationMethod, f32)> =
        Vec::new();
    for &(phrase, rel) in FAMILY_RELATION_TRIGGERS {
        if let Some(pos) = s_lower.find(phrase) {
            matches.push((
                pos,
                pos + phrase.len(),
                phrase,
                rel,
                RelationClassificationMethod::FamilyTrigger,
                0.90,
            ));
        }
    }
    for &(phrase, rel) in AGENT_RELATION_TRIGGERS {
        if let Some(pos) = s_lower.find(phrase) {
            matches.push((
                pos,
                pos + phrase.len(),
                phrase,
                rel,
                RelationClassificationMethod::AgentTrigger,
                0.70,
            ));
        }
    }
    for &(phrase, rel) in SPATIAL_RELATION_TRIGGERS {
        if let Some(pos) = s_lower.find(phrase) {
            matches.push((
                pos,
                pos + phrase.len(),
                phrase,
                rel,
                RelationClassificationMethod::SpatialTrigger,
                0.70,
            ));
        }
    }
    // Longest phrase first (prevents e.g. "mother of" clobbering "foster mother of"),
    // then earliest position, mirroring the substring-clobber-prevention rule already
    // applied by `sequence::scan_chunk_for_dates()` for date phrases.
    matches.sort_by(|a, b| b.2.len().cmp(&a.2.len()).then(a.0.cmp(&b.0)));
    matches.into_iter().next()
}

// ---------------------------------------------------------------------------
// Phase 1: windowed candidate generation
// ---------------------------------------------------------------------------

/// How close (in characters) two candidate entity positions must be to count as
/// "equally close" for ambiguity scoring.
const TIE_MARGIN_CHARS: usize = 15;

/// Whole-word first-person pronouns, resolved to a fixed narrator entity regardless
/// of local context (unlike 3rd-person pronouns, which need gender/recency matching).
/// D6 (and memoirs generally) are written in first person, so most relation-bearing
/// sentences ("I married Wahida", "my grandfather arrived in 1884") only ever name
/// ONE person literally — without this, `positions.len() < 2` below discards nearly
/// every first-person sentence, which is why an earlier version of this classifier
/// found almost no candidates on D6's real corpus despite it being dense with family
/// narrative.
const FIRST_PERSON_PRONOUNS: &[&str] = &["i", "my", "me", "myself"];

/// Locate whole-word first-person pronoun occurrences in `text`, each resolved to
/// `narrator_id`. Byte offsets computed the same way as
/// `ner::resolve_pronouns_from_candidates` (cumulative preceding-word lengths), for
/// consistency with the other coref position sources merged into `positions` below.
fn find_narrator_pronoun_positions(text: &str, narrator_id: i64) -> Vec<(usize, usize, i64)> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut results = Vec::new();
    let mut offset = 0usize;
    for &w in &words {
        let core = w.trim_matches(|c: char| !c.is_alphanumeric());
        if FIRST_PERSON_PRONOUNS.contains(&core.to_lowercase().as_str()) {
            let start = offset + w.find(core).unwrap_or(0);
            results.push((start, start + core.len(), narrator_id));
        }
        offset += w.len() + 1; // +1 for the single space split_whitespace consumed
    }
    results
}

/// Scan `text` for relation trigger phrases and, for each one found, locate the
/// nearest known entity before and after it to form a subject-relation-object
/// candidate triple. Generalizes `sequence::extract_kinship_interactions()`:
/// same sentence-splitting + nearest-entity-before/after-keyword algorithm, but
/// covering the full Family/Agent/Spatial trigger vocabulary (not kinship-only),
/// picking the longest matching phrase per sentence, and scoring a composite
/// confidence instead of returning a bare boolean/triple.
///
/// `known_entities` is `(entity_id, canonical_name, aliases)`, exactly as accepted
/// by `extract_kinship_interactions`. Only the first trigger match per sentence is
/// used — same one-relation-per-clause assumption as the function this generalizes.
///
/// `person_candidates` is `(name, aliases, gender)`, exactly as produced by
/// `GraphStore::coref_candidates_for_chunk()` — passed straight into
/// `ner::resolve_definite_descriptions`/`ner::resolve_pronouns_from_candidates` so
/// phrases like "my wife", "the author", "he"/"she" can resolve to a specific known
/// entity even when that entity's canonical name never appears literally in the
/// sentence. `narrator` (id, name) additionally resolves bare first-person pronouns
/// ("I", "my", "me") — the one case the two `ner` functions above don't cover, since
/// first-person always resolves to the same fixed entity rather than needing
/// gender/alias matching.
///
/// Every coref-resolved match is flagged internally and caps this candidate's
/// `proximity_confidence` lower than a literal name/alias match would — an extra
/// inference step is a real (if modest) precision risk worth reflecting in the score.
pub fn classify_relation_candidates(
    chunk_id: i64,
    text: &str,
    known_entities: &[(i64, String, Vec<String>)],
    person_candidates: &[(String, Vec<String>, Option<String>)],
    narrator: Option<(i64, &str)>,
) -> Vec<TypedRelationCandidate> {
    let name_by_id: HashMap<i64, &str> = known_entities
        .iter()
        .map(|(id, name, _)| (*id, name.as_str()))
        .collect();
    let id_by_name_lower: HashMap<String, i64> = known_entities
        .iter()
        .map(|(id, name, _)| (name.to_lowercase(), *id))
        .collect();

    let mut results: Vec<TypedRelationCandidate> = Vec::new();
    let mut seen: HashSet<(i64, i64, String)> = HashSet::new();

    let sentences: Vec<&str> = text
        .split(['.', '?', '!', ';'])
        .map(str::trim)
        .filter(|s| s.len() >= 12)
        .collect();

    for sentence in sentences {
        let s_lower = sentence.to_lowercase();

        let Some((kw_pos, kw_end, kw, relation_type, method, trigger_confidence)) =
            find_best_trigger(&s_lower)
        else {
            continue;
        };
        // Reversal is resolved once, correctly, right here (structural, not
        // probabilistic) — no separate validation pass is needed downstream.
        let reversed = REVERSED_TRIGGERS.contains(&kw);

        // Map each known entity to its first occurrence position in this sentence.
        // (start, end, id, via_coref)
        let mut positions: Vec<(usize, usize, i64, bool)> = Vec::new();
        for (id, name, aliases) in known_entities {
            let candidates =
                std::iter::once(name.as_str()).chain(aliases.iter().map(String::as_str));
            for tok in candidates {
                if tok.len() < 3 || tok.eq_ignore_ascii_case("I") {
                    continue;
                }
                if let Some(pos) = s_lower.find(&tok.to_lowercase()) {
                    positions.push((pos, pos + tok.len(), *id, false));
                    break;
                }
            }
        }

        // Coref: definite descriptions ("my wife", "the author") + 3rd-person pronouns
        // ("he"/"she", gender-matched to the nearest candidate).
        for res in crate::ner::resolve_definite_descriptions(sentence, person_candidates)
            .into_iter()
            .chain(crate::ner::resolve_pronouns_from_candidates(
                sentence,
                person_candidates,
            ))
        {
            if let Some(&id) = id_by_name_lower.get(&res.entity_name.to_lowercase()) {
                positions.push((res.offset, res.offset + res.surface.len(), id, true));
            }
        }

        // Coref: bare first-person pronouns ("I", "my", "me") → the narrator.
        if let Some((narrator_id, _)) = narrator {
            positions.extend(
                find_narrator_pronoun_positions(sentence, narrator_id)
                    .into_iter()
                    .map(|(s, e, id)| (s, e, id, true)),
            );
        }

        // Resolve overlapping spans by keeping the longest/most-specific match —
        // e.g. "my wife" (a definite-description resolution) fully contains "my"
        // (a bare first-person-pronoun match); without this, whichever happened to
        // be pushed last would win regardless of specificity. Mirrors the
        // longest-trigger-wins rule `find_best_trigger` already applies above.
        positions.sort_by_key(|(start, end, _, _)| std::cmp::Reverse(end - start));
        let mut deduped: Vec<(usize, usize, i64, bool)> = Vec::with_capacity(positions.len());
        for pos @ (start, end, _, _) in positions {
            let overlaps = deduped.iter().any(|&(rs, re, _, _)| start < re && rs < end);
            if !overlaps {
                deduped.push(pos);
            }
        }
        let positions = deduped;

        if positions.len() < 2 {
            continue;
        }

        let before: Vec<&(usize, usize, i64, bool)> =
            positions.iter().filter(|(_, end, _, _)| *end <= kw_pos).collect();
        let after: Vec<&(usize, usize, i64, bool)> =
            positions.iter().filter(|(start, _, _, _)| *start >= kw_end).collect();

        let before_best = before.iter().max_by_key(|(start, _, _, _)| *start);
        let after_best = after.iter().min_by_key(|(start, _, _, _)| *start);

        let (Some(&&(before_start, _, a_id, a_coref)), Some(&&(after_start, _, b_id, b_coref))) =
            (before_best, after_best)
        else {
            continue;
        };
        if a_id == b_id {
            continue;
        }

        let (sub_id, obj_id) = if reversed { (b_id, a_id) } else { (a_id, b_id) };
        let key = (sub_id, obj_id, relation_type.to_string());
        if !seen.insert(key) {
            continue;
        }

        let before_ties = before
            .iter()
            .filter(|(start, _, id, _)| *id != a_id && before_start.saturating_sub(*start) <= TIE_MARGIN_CHARS)
            .count();
        let after_ties = after
            .iter()
            .filter(|(start, _, id, _)| *id != b_id && start.saturating_sub(after_start) <= TIE_MARGIN_CHARS)
            .count();
        let ambiguity_count = (before_ties + after_ties) as u8;
        let via_coref = a_coref || b_coref;
        // 0.50 floor is defensive only — `positions.len() < 2` above already rules
        // out the zero-candidate-on-a-side case in practice. Coref-resolved matches
        // are capped lower at every ambiguity level to reflect the extra inference step.
        let proximity_confidence = match (ambiguity_count == 0, via_coref) {
            (true, false) => 0.95,
            (true, true) => 0.80,
            (false, false) => 0.70,
            (false, true) => 0.60,
        };

        results.push(TypedRelationCandidate {
            subject_id: sub_id,
            object_id: obj_id,
            subject_name: name_by_id.get(&sub_id).map(|s| s.to_string()).unwrap_or_default(),
            object_name: name_by_id.get(&obj_id).map(|s| s.to_string()).unwrap_or_default(),
            relation_type: relation_type.to_string(),
            trigger_confidence,
            proximity_confidence,
            composite_confidence: trigger_confidence * proximity_confidence,
            method,
            reversed,
            ambiguity_count,
            source_sentence: sentence.to_string(),
            chunk_id,
        });
    }

    results
}

// ---------------------------------------------------------------------------
// Phase 2: axiom validation
// ---------------------------------------------------------------------------

/// Snapshot of graph state needed to validate relation candidates, built once
/// before a run so the axiom pass never needs to hold the graph lock — mirrors
/// `axiom_extract`'s `entity_snapshot` precedent.
#[derive(Default)]
pub struct RelationAxiomSnapshot {
    /// Entity ID → entity_type (e.g. "Person"), for the familial-Person-only axiom.
    pub entity_types: HashMap<i64, String>,
    /// Ordered (min_id, max_id) entity pair → set of existing trusted relation
    /// types already asserted between them, for the contradiction axiom.
    pub trusted_relations: HashMap<(i64, i64), HashSet<String>>,
}

/// Apply consistency axioms to relation candidates; demote conflicting or ambiguous
/// ones to zero confidence (never dropping the candidate itself — a demoted entry
/// still shows up in metrics as axiom-demoted rather than silently vanishing),
/// mirroring `axiom_extract::validate_with_axioms`'s demotion-based design.
pub fn validate_relation_axioms(
    candidates: Vec<TypedRelationCandidate>,
    snapshot: &RelationAxiomSnapshot,
) -> Vec<TypedRelationCandidate> {
    candidates
        .into_iter()
        .map(|mut c| {
            // Axiom 1: familial relation types require both endpoints to be Person.
            // Pre-checks what `upsert_relation` enforces anyway, avoiding a wasted
            // LLM-verify call on a candidate that would be silently dropped at commit.
            if FAMILIAL_RELS.contains(&c.relation_type.as_str()) {
                let subject_ok = snapshot
                    .entity_types
                    .get(&c.subject_id)
                    .map(|t| t.eq_ignore_ascii_case("person"))
                    .unwrap_or(true); // unresolved entity → assume OK, same as upsert_relation
                let object_ok = snapshot
                    .entity_types
                    .get(&c.object_id)
                    .map(|t| t.eq_ignore_ascii_case("person"))
                    .unwrap_or(true);
                if !subject_ok || !object_ok {
                    c.composite_confidence = 0.0;
                    c.method = RelationClassificationMethod::Unknown;
                }
            }

            // Axiom 2: ambiguous window — more than one equally-close candidate
            // entity crowding the pick on either side means the automatic choice
            // is unsafe, even though `classify_relation_candidates` already picked
            // the closest one and scored it down to 0.70 proximity.
            if c.ambiguity_count >= 2 {
                c.composite_confidence = 0.0;
                c.method = RelationClassificationMethod::Unknown;
            }

            // Axiom 3: contradiction with an existing trusted relation on the same
            // pair (e.g. graph already trusts spouse_of A-B; this candidate guesses
            // sibling_of A-B) — reuses the same contradiction table as the R1 dedup
            // guard rather than duplicating it.
            if let Some(existing_types) = snapshot.trusted_relations.get(&ord_pair(c.subject_id, c.object_id)) {
                let contradicts = existing_types
                    .iter()
                    .any(|existing| family_role_contradicts(&c.relation_type, existing));
                if contradicts {
                    c.composite_confidence = 0.0;
                    c.method = RelationClassificationMethod::Unknown;
                }
            }

            c
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Phase 3: three-way confidence split
// ---------------------------------------------------------------------------

/// Split candidates into commit (write directly, no LLM), verify (send to the
/// narrow LLM confirm/reject/retype pass), and drop (discarded, never reaching
/// any LLM). Unlike the entity classifier's two-way split, relations get a hard
/// low-end cutoff: candidate subject-relation-object triples scale combinatorially
/// with entities-per-window, so dropping the noisiest tier outright is a needed
/// precision/cost safeguard entities didn't need.
///
/// If `threshold_low > threshold_high` (misconfiguration), collapses to
/// `threshold_low = threshold_high` — an empty verify band (everything commits or
/// drops) rather than panicking or silently reordering the two thresholds.
pub fn split_relations_by_confidence(
    candidates: Vec<TypedRelationCandidate>,
    threshold_high: f32,
    threshold_low: f32,
) -> (Vec<TypedRelationCandidate>, Vec<TypedRelationCandidate>, Vec<TypedRelationCandidate>) {
    let threshold_low = threshold_low.min(threshold_high);
    let mut commit = Vec::new();
    let mut verify = Vec::new();
    let mut drop = Vec::new();
    for c in candidates {
        if c.composite_confidence >= threshold_high {
            commit.push(c);
        } else if c.composite_confidence >= threshold_low {
            verify.push(c);
        } else {
            drop.push(c);
        }
    }
    (commit, verify, drop)
}

// ---------------------------------------------------------------------------
// schema.org documentation mapping (interop/docs only — never consumed by
// storage, dedup, or scorer logic; internal snake_case relation_type strings
// remain the source of truth everywhere else).
// ---------------------------------------------------------------------------

/// Best-effort schema.org property name for a relation_type, for documentation and
/// future interop only. Several of our types have no exact schema.org equivalent
/// (returns `None`) or a direction/granularity mismatch noted in the match arms —
/// these are documented, not "fixed", since fixing them would mean either renaming
/// widely-used internal strings or diverging from schema.org's own conventions.
pub fn schema_org_property_for(relation_type: &str) -> Option<&'static str> {
    match relation_type {
        // schema.org models parent/child as the same "parent" property read in
        // opposite directions; we model it as two internal types + auto-inverse.
        "parent_of" | "child_of" => Some("schema:parent"),
        "spouse_of" => Some("schema:spouse"),
        // schema.org has no half-sibling distinction; both map to the same property.
        "sibling_of" | "half_sibling_of" => Some("schema:sibling"),
        "member_of" => Some("schema:memberOf"),
        // schema.org's "founder" points Organization→Person; our "founded" points
        // Person→Organization — the inverse direction. Intentional, documented here.
        "founded" => Some("schema:founder"),
        "affiliated_with" => Some("schema:affiliation"),
        "lived_in" => Some("schema:homeLocation"),
        "works_at" => Some("schema:worksFor"),
        // No exact schema.org property — internal-only extensions.
        "grandparent_of" | "grandchild_of" | "foster_parent_of" | "foster_child_of"
        | "associated_with" => None,
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Mutable accumulator updated as relation candidates are classified and (for the
/// medium tier) sent through LLM verification. Mirrors `axiom_extract::AxiomaticMetricsAccum`.
pub struct RelationAxiomaticMetricsAccum {
    pub threshold_high: f32,
    pub threshold_low: f32,
    pub total_chunks: usize,
    pub candidates_generated: usize,
    pub candidates_committed_high: usize,
    pub candidates_sent_llm_medium: usize,
    pub candidates_dropped_low: usize,
    pub candidates_demoted_by_axiom: usize,
    pub llm_confirmed: usize,
    pub llm_rejected: usize,
    pub llm_retyped: usize,
    pub axio_times_ms: Vec<f64>,
    pub llm_verify_times_ms: Vec<f64>,
    pub method_breakdown: HashMap<String, usize>,
}

impl RelationAxiomaticMetricsAccum {
    pub fn new(threshold_high: f32, threshold_low: f32, total_chunks: usize) -> Self {
        Self {
            threshold_high,
            threshold_low,
            total_chunks,
            candidates_generated: 0,
            candidates_committed_high: 0,
            candidates_sent_llm_medium: 0,
            candidates_dropped_low: 0,
            candidates_demoted_by_axiom: 0,
            llm_confirmed: 0,
            llm_rejected: 0,
            llm_retyped: 0,
            axio_times_ms: Vec::new(),
            llm_verify_times_ms: Vec::new(),
            method_breakdown: HashMap::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_chunk(
        &mut self,
        generated: usize,
        committed: usize,
        sent_llm: usize,
        dropped: usize,
        demoted: usize,
        elapsed_ms: f64,
        methods: &[RelationClassificationMethod],
    ) {
        self.candidates_generated += generated;
        self.candidates_committed_high += committed;
        self.candidates_sent_llm_medium += sent_llm;
        self.candidates_dropped_low += dropped;
        self.candidates_demoted_by_axiom += demoted;
        self.axio_times_ms.push(elapsed_ms);
        for m in methods {
            *self.method_breakdown.entry(m.as_str().to_string()).or_insert(0) += 1;
        }
    }

    pub fn record_llm_verify(&mut self, confirmed: usize, rejected: usize, retyped: usize, elapsed_ms: f64) {
        self.llm_confirmed += confirmed;
        self.llm_rejected += rejected;
        self.llm_retyped += retyped;
        self.llm_verify_times_ms.push(elapsed_ms);
    }

    pub fn finalise(self, wall_secs: f64) -> RelationAxiomaticRunMetrics {
        let commit_pct = if self.candidates_generated > 0 {
            self.candidates_committed_high as f64 / self.candidates_generated as f64 * 100.0
        } else {
            0.0
        };
        let drop_pct = if self.candidates_generated > 0 {
            self.candidates_dropped_low as f64 / self.candidates_generated as f64 * 100.0
        } else {
            0.0
        };
        let llm_total = self.llm_confirmed + self.llm_rejected;
        let llm_confirm_rate = if llm_total > 0 {
            self.llm_confirmed as f64 / llm_total as f64
        } else {
            0.0
        };

        RelationAxiomaticRunMetrics {
            timestamp: chrono::Utc::now().to_rfc3339(),
            threshold_high: self.threshold_high,
            threshold_low: self.threshold_low,
            total_chunks: self.total_chunks,
            candidates_generated: self.candidates_generated,
            candidates_committed_high: self.candidates_committed_high,
            candidates_sent_llm_medium: self.candidates_sent_llm_medium,
            candidates_dropped_low: self.candidates_dropped_low,
            candidates_demoted_by_axiom: self.candidates_demoted_by_axiom,
            llm_confirmed: self.llm_confirmed,
            llm_rejected: self.llm_rejected,
            llm_retyped: self.llm_retyped,
            total_wall_secs: wall_secs,
            mean_ms_per_chunk_axio: mean(&self.axio_times_ms),
            p95_ms_per_chunk_axio: p95(&self.axio_times_ms),
            mean_ms_per_llm_verify_call: mean(&self.llm_verify_times_ms),
            p95_ms_per_llm_verify_call: p95(&self.llm_verify_times_ms),
            commit_pct,
            drop_pct,
            llm_confirm_rate,
            method_breakdown: self.method_breakdown,
        }
    }
}

/// Finalised, serialisable metrics for one relation-extraction run.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationAxiomaticRunMetrics {
    pub timestamp: String,
    pub threshold_high: f32,
    pub threshold_low: f32,
    pub total_chunks: usize,
    pub candidates_generated: usize,
    pub candidates_committed_high: usize,
    pub candidates_sent_llm_medium: usize,
    pub candidates_dropped_low: usize,
    pub candidates_demoted_by_axiom: usize,
    pub llm_confirmed: usize,
    pub llm_rejected: usize,
    pub llm_retyped: usize,
    pub total_wall_secs: f64,
    pub mean_ms_per_chunk_axio: f64,
    pub p95_ms_per_chunk_axio: f64,
    pub mean_ms_per_llm_verify_call: f64,
    pub p95_ms_per_llm_verify_call: f64,
    pub commit_pct: f64,
    pub drop_pct: f64,
    /// Of the candidates sent to LLM verification, the fraction confirmed (not
    /// rejected) — the key precision signal for re-tuning threshold_low/high.
    pub llm_confirm_rate: f64,
    pub method_breakdown: HashMap<String, usize>,
}

impl RelationAxiomaticRunMetrics {
    pub fn print_summary(&self) {
        let sep = "─".repeat(62);
        println!("\n{sep}");
        println!("  Relation Axiomatic Extraction Summary");
        println!("{sep}");
        println!(
            "  Thresholds      : high={:.2}  low={:.2}",
            self.threshold_high, self.threshold_low
        );
        println!("  Total chunks    : {}", self.total_chunks);
        println!("  Candidates      : {}", self.candidates_generated);
        println!(
            "  Committed (high): {} ({:.1}%)  ← no LLM call",
            self.candidates_committed_high, self.commit_pct
        );
        println!(
            "  Sent to LLM     : {}",
            self.candidates_sent_llm_medium
        );
        println!(
            "  Dropped (low)   : {} ({:.1}%)  ← never reached an LLM",
            self.candidates_dropped_low, self.drop_pct
        );
        println!(
            "  Demoted by axiom: {}",
            self.candidates_demoted_by_axiom
        );
        if self.llm_confirmed + self.llm_rejected > 0 {
            println!(
                "  LLM confirm rate: {:.1}%  (confirmed={} rejected={} retyped={})",
                self.llm_confirm_rate * 100.0,
                self.llm_confirmed,
                self.llm_rejected,
                self.llm_retyped
            );
        }
        println!("  Wall-clock      : {}", format_duration(self.total_wall_secs));
        if !self.method_breakdown.is_empty() {
            let mut methods: Vec<_> = self.method_breakdown.iter().collect();
            methods.sort_by(|a, b| b.1.cmp(a.1));
            let breakdown = methods
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("  ");
            println!("  Methods         : {breakdown}");
        }
        println!("{sep}\n");
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.iter().sum::<f64>() / v.len() as f64
}

fn p95(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let mut sorted = v.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = ((sorted.len() as f64 * 0.95) as usize).min(sorted.len() - 1);
    sorted[idx]
}

fn format_duration(secs: f64) -> String {
    let h = (secs / 3600.0) as u64;
    let m = ((secs % 3600.0) / 60.0) as u64;
    let s = (secs % 60.0) as u64;
    if h > 0 {
        format!("{h}h {m:02}m {s:02}s")
    } else if m > 0 {
        format!("{m}m {s:02}s")
    } else {
        format!("{s}s")
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn entities(list: &[(i64, &str)]) -> Vec<(i64, String, Vec<String>)> {
        list.iter().map(|(id, name)| (*id, name.to_string(), vec![])).collect()
    }

    #[test]
    fn family_trigger_produces_correct_triplet() {
        let known = entities(&[(1, "Cissie Gool"), (2, "Abdul Hamid Gool")]);
        let candidates = classify_relation_candidates(
            10,
            "Cissie Gool was the wife of Abdul Hamid Gool for many years.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        let c = &candidates[0];
        assert_eq!(c.relation_type, "spouse_of");
        assert_eq!(c.subject_id, 1);
        assert_eq!(c.object_id, 2);
        assert_eq!(c.method, RelationClassificationMethod::FamilyTrigger);
        assert!(c.composite_confidence > 0.8);
    }

    #[test]
    fn reversed_trigger_swaps_subject_and_object() {
        let known = entities(&[(1, "New Era Fellowship"), (2, "Ben Kies")]);
        let candidates = classify_relation_candidates(
            11,
            "The New Era Fellowship was founded by Ben Kies in that decade.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        let c = &candidates[0];
        assert_eq!(c.relation_type, "founded");
        assert!(c.reversed);
        // Ben Kies (2) founded New Era Fellowship (1) — subject/object swapped.
        assert_eq!(c.subject_id, 2);
        assert_eq!(c.object_id, 1);
    }

    #[test]
    fn longest_phrase_wins_over_substring() {
        let known = entities(&[(1, "Cissie Gool"), (2, "Halima Gool")]);
        let candidates = classify_relation_candidates(
            12,
            "Cissie Gool was the foster mother of Halima Gool.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].relation_type, "foster_parent_of");
    }

    #[test]
    fn ambiguous_window_is_demoted_by_axiom() {
        // Three candidate entities all clustered within the tie margin on the same
        // side of the trigger — none should be picked with confidence.
        let known = entities(&[(1, "Ann"), (2, "Ivy"), (3, "Amy"), (4, "Zoe")]);
        let candidates = classify_relation_candidates(
            13,
            "Ann Ivy Amy was the mother of Zoe in that year.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        assert!(candidates[0].ambiguity_count >= 2);
        let validated = validate_relation_axioms(candidates, &RelationAxiomSnapshot::default());
        assert_eq!(validated[0].composite_confidence, 0.0);
    }

    #[test]
    fn familial_type_requires_both_person() {
        let known = entities(&[(1, "Cissie Gool"), (2, "District Six")]);
        let candidates = classify_relation_candidates(
            14,
            "Cissie Gool was the mother of District Six in this sentence.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        let mut snapshot = RelationAxiomSnapshot::default();
        snapshot.entity_types.insert(1, "Person".to_string());
        snapshot.entity_types.insert(2, "Place".to_string());
        let validated = validate_relation_axioms(candidates, &snapshot);
        assert_eq!(validated[0].composite_confidence, 0.0);
        assert_eq!(validated[0].method, RelationClassificationMethod::Unknown);
    }

    #[test]
    fn contradiction_with_existing_trusted_relation_demotes() {
        let known = entities(&[(1, "Cissie Gool"), (2, "Abdul Hamid Gool")]);
        let candidates = classify_relation_candidates(
            15,
            "Cissie Gool was the sister of Abdul Hamid Gool according to this account.",
            &known,
            &[],
            None,
        );
        assert_eq!(candidates.len(), 1);
        let mut snapshot = RelationAxiomSnapshot::default();
        snapshot
            .trusted_relations
            .insert(ord_pair(1, 2), HashSet::from(["spouse_of".to_string()]));
        let validated = validate_relation_axioms(candidates, &snapshot);
        assert_eq!(validated[0].composite_confidence, 0.0);
    }

    #[test]
    fn first_person_narrator_pronoun_resolves_missing_second_entity() {
        // The general first-person-memoir failure case this fix addresses: only one
        // person is literally named in the sentence, the other side is a bare
        // first-person pronoun ("I"/"my"/"me") referring to whoever is narrating.
        let known = entities(&[(1, "Jane Doe")]);
        let candidates = classify_relation_candidates(
            20,
            "I married Jane Doe in the winter of that year.",
            &known,
            &[],
            Some((99, "John Smith")),
        );
        assert_eq!(candidates.len(), 1);
        let c = &candidates[0];
        assert_eq!(c.relation_type, "spouse_of");
        assert!(c.subject_id == 99 || c.object_id == 99, "narrator must appear as an endpoint");
        assert!(c.subject_id == 1 || c.object_id == 1, "Jane Doe must appear as an endpoint");
        // Coref-resolved endpoint caps proximity_confidence below a pure literal match.
        assert!(c.proximity_confidence <= 0.80);
    }

    #[test]
    fn without_narrator_first_person_sentence_yields_nothing() {
        // Confirms the gap this fix closes: with no narrator supplied, a sentence
        // naming only one person can never produce a candidate (positions.len() < 2).
        let known = entities(&[(1, "Jane Doe")]);
        let candidates = classify_relation_candidates(
            21,
            "I married Jane Doe in the winter of that year.",
            &known,
            &[],
            None,
        );
        assert!(candidates.is_empty());
    }

    #[test]
    fn definite_description_resolves_kinship_phrase() {
        // "my wife" resolves via ner::resolve_definite_descriptions against an alias,
        // without the wife's canonical name ever appearing literally in the sentence.
        let known = entities(&[(1, "Jane Doe"), (2, "Mary Johnson")]);
        let person_candidates = vec![(
            "Jane Doe".to_string(),
            vec!["wife".to_string()],
            Some("Female".to_string()),
        )];
        let candidates = classify_relation_candidates(
            22,
            "My wife was the daughter of Mary Johnson in this account.",
            &known,
            &person_candidates,
            Some((99, "John Smith")),
        );
        assert_eq!(candidates.len(), 1);
        let c = &candidates[0];
        assert_eq!(c.relation_type, "child_of");
        assert!(c.subject_id == 1 || c.object_id == 1, "Jane Doe (\"my wife\") must appear as an endpoint");
        assert!(c.subject_id == 2 || c.object_id == 2, "Mary Johnson must appear as an endpoint");
    }

    #[test]
    fn split_three_way_basic() {
        let mk = |name: &str, conf: f32| TypedRelationCandidate {
            subject_id: 1,
            object_id: 2,
            subject_name: name.to_string(),
            object_name: "B".to_string(),
            relation_type: "member_of".to_string(),
            trigger_confidence: conf,
            proximity_confidence: 1.0,
            composite_confidence: conf,
            method: RelationClassificationMethod::AgentTrigger,
            reversed: false,
            ambiguity_count: 0,
            source_sentence: String::new(),
            chunk_id: 0,
        };
        let candidates = vec![mk("high", 0.9), mk("medium", 0.6), mk("low", 0.2)];
        let (commit, verify, drop) = split_relations_by_confidence(candidates, 0.80, 0.45);
        assert_eq!(commit.len(), 1);
        assert_eq!(commit[0].subject_name, "high");
        assert_eq!(verify.len(), 1);
        assert_eq!(verify[0].subject_name, "medium");
        assert_eq!(drop.len(), 1);
        assert_eq!(drop[0].subject_name, "low");
    }

    #[test]
    fn split_collapses_when_low_exceeds_high() {
        let mk = |conf: f32| TypedRelationCandidate {
            subject_id: 1,
            object_id: 2,
            subject_name: "A".to_string(),
            object_name: "B".to_string(),
            relation_type: "member_of".to_string(),
            trigger_confidence: conf,
            proximity_confidence: 1.0,
            composite_confidence: conf,
            method: RelationClassificationMethod::AgentTrigger,
            reversed: false,
            ambiguity_count: 0,
            source_sentence: String::new(),
            chunk_id: 0,
        };
        // Misconfigured: low (0.90) > high (0.70) — should collapse to an empty
        // verify band rather than misclassify or panic.
        let candidates = vec![mk(0.9), mk(0.75), mk(0.3)];
        let (commit, verify, drop) = split_relations_by_confidence(candidates, 0.70, 0.90);
        assert_eq!(commit.len(), 2);
        assert!(verify.is_empty());
        assert_eq!(drop.len(), 1);
    }

    #[test]
    fn schema_org_mapping_documents_known_gaps() {
        assert_eq!(schema_org_property_for("spouse_of"), Some("schema:spouse"));
        assert_eq!(schema_org_property_for("grandparent_of"), None);
    }
}
