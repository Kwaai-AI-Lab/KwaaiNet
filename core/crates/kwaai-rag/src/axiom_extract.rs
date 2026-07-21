//! Axiomatic entity candidate classification — Phase 1-3 of the 4-phase extraction pipeline.
//!
//! Classifies proper-noun candidates using deterministic rules (lexical markers, graph lookup,
//! GLiNER hints) and assigns composite confidence scores. High-confidence candidates bypass the
//! LLM entirely; low-confidence ones fall through to `extract_from_text()` (Phase 4).
//!
//! Mirrors the architecture of `sequence.rs` (4-phase timeline pipeline) applied to entities.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Classification types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum ClassificationMethod {
    KnownEntity,
    HonorificPrefix,
    HonorificSuffix,
    OrgMarker,
    LegislationMarker,
    PublicationMarker,
    GeoMarker,
    GliNERHint,
    Unknown,
}

impl ClassificationMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::KnownEntity => "KnownEntity",
            Self::HonorificPrefix => "HonorificPrefix",
            Self::HonorificSuffix => "HonorificSuffix",
            Self::OrgMarker => "OrgMarker",
            Self::LegislationMarker => "LegislationMarker",
            Self::PublicationMarker => "PublicationMarker",
            Self::GeoMarker => "GeoMarker",
            Self::GliNERHint => "GliNERHint",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypedCandidate {
    pub name: String,
    /// `None` = type unknown → needs LLM
    pub entity_type: Option<String>,
    pub type_confidence: f32,
    pub mention_confidence: f32,
    /// type_confidence × mention_confidence
    pub composite_confidence: f32,
    pub method: ClassificationMethod,
    /// Axiomatic fields (e.g. occupation from honorific) — may be empty
    pub axiomatic_fields: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Rule tables
// ---------------------------------------------------------------------------

const HONORIFIC_PREFIXES: &[&str] = &[
    "dr", "dr.", "prof", "prof.", "rev", "rev.", "haji", "sheikh", "imam", "cllr", "cllr.", "adv",
    "adv.", "sgt", "sgt.", "capt", "capt.", "sir", "mr", "mr.", "mrs", "mrs.", "miss", "ms", "ms.",
    "brig", "brig.", "col", "col.", "gen", "gen.", "lt", "lt.",
];

const HONORIFIC_SUFFIXES: &[&str] = &["junior", "senior", "jr", "jr.", "sr", "sr."];

// Checked against the LAST word of the candidate phrase.
const ORG_MARKERS_LAST: &[&str] = &[
    "council",
    "league",
    "movement",
    "party",
    "congress",
    "institute",
    "society",
    "association",
    "brotherhood",
    "federation",
    "union",
    "committee",
    "board",
    "club",
    "order",
    "guild",
    "branch",
    "bureau",
    "commission",
    "corps",
];

// Checked against the LAST word of the candidate phrase.
const LEGISLATION_MARKERS_LAST: &[&str] = &[
    "act",
    "ordinance",
    "regulation",
    "bill",
    "amendment",
    "proclamation",
];

// Checked anywhere in the candidate phrase.
const PUBLICATION_MARKERS_ANY: &[&str] = &[
    "newspaper",
    "gazette",
    "times",
    "argus",
    "herald",
    "journal",
    "review",
    "magazine",
    "opinion",
    "chronicle",
    "press",
    "daily",
    "weekly",
    "monthly",
];

// Checked anywhere in the candidate phrase.
const GEO_MARKERS_ANY: &[&str] = &[
    "cape", "district", "street", "road", "avenue", "nek", "bay", "mountain", "town", "city",
    "valley", "flats", "island", "hill", "square", "lane", "drive", "park", "village", "quarter",
    "location", "suburb", "heights", "gardens", "point", "beach", "harbour", "river", "grove",
    "terrace", "estate", "court",
];

// ---------------------------------------------------------------------------
// Phase 1: axiomatic classification
// ---------------------------------------------------------------------------

/// Classify proper-noun candidates using deterministic rules, without any LLM call.
///
/// `entity_snapshot` maps lowercased entity names and aliases to their type — built
/// once from the GraphStore before the spawn loop so tasks never hold the graph lock.
///
/// Rules are applied in priority order; first match wins.
pub fn classify_candidates_axiomatic(
    candidates: &[String],
    entity_snapshot: &HashMap<String, String>,
    gliner_hints: &[String],
) -> Vec<TypedCandidate> {
    let gliner_set: std::collections::HashSet<&str> =
        gliner_hints.iter().map(|s| s.as_str()).collect();
    candidates
        .iter()
        .map(|name| classify_one(name, entity_snapshot, &gliner_set))
        .collect()
}

fn classify_one(
    name: &str,
    entity_snapshot: &HashMap<String, String>,
    gliner_set: &std::collections::HashSet<&str>,
) -> TypedCandidate {
    let name_lower = name.to_lowercase();
    let words: Vec<&str> = name.split_whitespace().collect();

    // ── Rule 1: known entity (canonical or alias already in graph) ──────────
    if let Some(entity_type) = entity_snapshot.get(&name_lower) {
        return known_entity(name, entity_type);
    }
    // Also try with honorific prefix stripped ("Dr. Abdurahman" → "abdurahman")
    if let Some(first) = words.first() {
        let fl = first.to_lowercase();
        let fl_stripped = fl.trim_end_matches('.');
        if (HONORIFIC_PREFIXES.contains(&fl.as_str()) || HONORIFIC_PREFIXES.contains(&fl_stripped))
            && words.len() > 1
        {
            let stripped = words[1..].join(" ").to_lowercase();
            if let Some(entity_type) = entity_snapshot.get(&stripped) {
                return known_entity(name, entity_type);
            }
        }
    }

    // ── Rule 2: honorific prefix → Person ────────────────────────────────────
    // Requires a word after the honorific — a bare "Rev" or "Dr" with nothing
    // following it is a title fragment, not a name, and must not commit as Person.
    if let Some(first) = words.first() {
        let fl = first.to_lowercase();
        let fl_stripped = fl.trim_end_matches('.');
        if words.len() > 1
            && (HONORIFIC_PREFIXES.contains(&fl.as_str())
                || HONORIFIC_PREFIXES.contains(&fl_stripped))
        {
            let mut fields = HashMap::new();
            if let Some(occ) = honorific_to_occupation(fl_stripped) {
                fields.insert("occupation".to_string(), occ.to_string());
            }
            return TypedCandidate {
                name: name.to_string(),
                entity_type: Some("Person".to_string()),
                type_confidence: 0.90,
                mention_confidence: 0.90,
                composite_confidence: 0.81,
                method: ClassificationMethod::HonorificPrefix,
                axiomatic_fields: fields,
            };
        }
    }

    // ── Rule 3: honorific suffix → Person ────────────────────────────────────
    if let Some(last) = words.last() {
        let ll = last.to_lowercase();
        if HONORIFIC_SUFFIXES.contains(&ll.as_str()) {
            return TypedCandidate {
                name: name.to_string(),
                entity_type: Some("Person".to_string()),
                type_confidence: 0.80,
                mention_confidence: 0.80,
                composite_confidence: 0.64,
                method: ClassificationMethod::HonorificSuffix,
                axiomatic_fields: HashMap::new(),
            };
        }
    }

    // ── Rule 4: legislation marker (last word) → Legislation ─────────────────
    if let Some(last) = words.last() {
        let ll = last.to_lowercase();
        if LEGISLATION_MARKERS_LAST.contains(&ll.as_str()) {
            return TypedCandidate {
                name: name.to_string(),
                entity_type: Some("Legislation".to_string()),
                type_confidence: 0.85,
                mention_confidence: 0.85,
                composite_confidence: 0.7225,
                method: ClassificationMethod::LegislationMarker,
                axiomatic_fields: HashMap::new(),
            };
        }
    }

    // ── Rule 5: org marker (last word) → Organization ────────────────────────
    if let Some(last) = words.last() {
        let ll = last.to_lowercase();
        if ORG_MARKERS_LAST.contains(&ll.as_str()) {
            return TypedCandidate {
                name: name.to_string(),
                entity_type: Some("Organization".to_string()),
                type_confidence: 0.85,
                mention_confidence: 0.85,
                composite_confidence: 0.7225,
                method: ClassificationMethod::OrgMarker,
                axiomatic_fields: HashMap::new(),
            };
        }
    }

    let words_lower: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();

    // ── Rule 6: publication marker (any word) → Publication ──────────────────
    if words_lower
        .iter()
        .any(|w| PUBLICATION_MARKERS_ANY.contains(&w.as_str()))
    {
        return TypedCandidate {
            name: name.to_string(),
            entity_type: Some("Publication".to_string()),
            type_confidence: 0.80,
            mention_confidence: 0.80,
            composite_confidence: 0.64,
            method: ClassificationMethod::PublicationMarker,
            axiomatic_fields: HashMap::new(),
        };
    }

    // ── Rule 7: geo marker (any word) → Place ────────────────────────────────
    if let Some(geo) = words_lower
        .iter()
        .find(|w| GEO_MARKERS_ANY.contains(&w.as_str()))
    {
        let mut fields = HashMap::new();
        fields.insert("locationType".to_string(), geo.clone());
        return TypedCandidate {
            name: name.to_string(),
            entity_type: Some("Place".to_string()),
            type_confidence: 0.80,
            mention_confidence: 0.80,
            composite_confidence: 0.64,
            method: ClassificationMethod::GeoMarker,
            axiomatic_fields: fields,
        };
    }

    // ── Rule 8: GLiNER hint → Person ─────────────────────────────────────────
    if gliner_set.contains(name) {
        return TypedCandidate {
            name: name.to_string(),
            entity_type: Some("Person".to_string()),
            type_confidence: 0.75,
            mention_confidence: 0.85,
            composite_confidence: 0.6375,
            method: ClassificationMethod::GliNERHint,
            axiomatic_fields: HashMap::new(),
        };
    }

    // ── Rule 9: unknown → needs LLM ──────────────────────────────────────────
    TypedCandidate {
        name: name.to_string(),
        entity_type: None,
        type_confidence: 0.0,
        mention_confidence: 0.50,
        composite_confidence: 0.0,
        method: ClassificationMethod::Unknown,
        axiomatic_fields: HashMap::new(),
    }
}

fn known_entity(name: &str, entity_type: &str) -> TypedCandidate {
    TypedCandidate {
        name: name.to_string(),
        entity_type: Some(entity_type.to_string()),
        type_confidence: 1.0,
        mention_confidence: 0.95,
        composite_confidence: 0.95,
        method: ClassificationMethod::KnownEntity,
        axiomatic_fields: HashMap::new(),
    }
}

fn honorific_to_occupation(h: &str) -> Option<&'static str> {
    match h {
        "dr" => Some("doctor"),
        "prof" => Some("professor"),
        "rev" => Some("reverend"),
        "cllr" => Some("councillor"),
        "adv" => Some("advocate"),
        "sgt" => Some("sergeant"),
        "capt" => Some("captain"),
        "imam" => Some("imam"),
        "sheikh" => Some("sheikh"),
        "haji" => Some("haji"),
        "brig" => Some("brigadier"),
        "col" => Some("colonel"),
        "gen" => Some("general"),
        "lt" => Some("lieutenant"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Phase 2: confidence threshold split
// ---------------------------------------------------------------------------

/// Split candidates into high-confidence (above threshold, no LLM needed) and
/// low-confidence (below threshold, sent to LLM Phase 4).
pub fn split_by_confidence(
    candidates: Vec<TypedCandidate>,
    threshold: f32,
) -> (Vec<TypedCandidate>, Vec<TypedCandidate>) {
    let mut high = Vec::new();
    let mut low = Vec::new();
    for c in candidates {
        if c.composite_confidence >= threshold {
            high.push(c);
        } else {
            low.push(c);
        }
    }
    (high, low)
}

// ---------------------------------------------------------------------------
// Phase 3: axiomatic validation (axioms 2–6; axiom 1 = blocklist done externally)
// ---------------------------------------------------------------------------

/// Apply consistency axioms to high-confidence candidates; demote conflicting or
/// ambiguous ones to low-confidence (composite = 0.0) so they fall through to LLM.
pub fn validate_with_axioms(candidates: Vec<TypedCandidate>, text: &str) -> Vec<TypedCandidate> {
    let mut result: Vec<TypedCandidate> = Vec::with_capacity(candidates.len());

    for mut c in candidates {
        // Axiom 2: type-marker consistency — honorific prefix + org-marker last word
        // signals conflicting classification; demote so LLM resolves it.
        if c.method == ClassificationMethod::OrgMarker {
            if let Some(first) = c.name.split_whitespace().next() {
                let fl = first.to_lowercase();
                let fl_stripped = fl.trim_end_matches('.');
                if HONORIFIC_PREFIXES.contains(&fl.as_str())
                    || HONORIFIC_PREFIXES.contains(&fl_stripped)
                {
                    c.composite_confidence = 0.0;
                    c.type_confidence = 0.0;
                    c.entity_type = None;
                    c.method = ClassificationMethod::Unknown;
                }
            }
        }

        // Axiom 4: single-word candidates with no supporting signal → too ambiguous.
        // KnownEntity and GLiNER-confirmed names are exempt.
        if c.name.split_whitespace().count() == 1
            && !matches!(
                c.method,
                ClassificationMethod::KnownEntity
                    | ClassificationMethod::HonorificPrefix
                    | ClassificationMethod::GliNERHint
            )
        {
            c.composite_confidence = 0.0;
            c.type_confidence = 0.0;
            c.entity_type = None;
            c.method = ClassificationMethod::Unknown;
        }

        // Axiom 6: low type_confidence + single mention → probably noise.
        if c.type_confidence < 0.60
            && c.method != ClassificationMethod::KnownEntity
            && text.matches(c.name.as_str()).count() <= 1
        {
            c.composite_confidence = 0.0;
        }

        result.push(c);
    }

    // Axiom 5: substring dominance — if candidate A is a strict substring of
    // candidate B in this same chunk's candidate set, demote A.
    // Collect all names first so the filter can see the full set.
    let all_names: Vec<String> = result.iter().map(|c| c.name.clone()).collect();
    result
        .into_iter()
        .map(|mut c| {
            let dominated = all_names.iter().any(|other| {
                other != &c.name && other.to_lowercase().contains(&c.name.to_lowercase())
            });
            if dominated {
                c.composite_confidence = 0.0;
                c.entity_type = None;
                c.method = ClassificationMethod::Unknown;
            }
            c
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

/// Which path a chunk took through the pipeline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChunkPath {
    /// All candidates were high-confidence — LLM call skipped entirely.
    FullAxiomatic,
    /// Mixed confidence — focused LLM call for low-confidence subset.
    FocusedLlm,
    /// All candidates low-confidence (or threshold disabled) — full LLM call.
    FullLlm,
}

/// Mutable accumulator updated by each worker task (guarded by a Mutex).
pub struct AxiomaticMetricsAccum {
    pub threshold: f32,
    pub total_chunks: usize,
    pub chunks_skipped_llm: usize,
    pub chunks_partial_llm: usize,
    pub chunks_full_llm: usize,
    pub entities_axiomatic: usize,
    pub entities_llm: usize,
    pub axio_times_ms: Vec<f64>,
    pub llm_times_ms: Vec<f64>,
    pub method_breakdown: HashMap<String, usize>,
}

impl AxiomaticMetricsAccum {
    pub fn new(threshold: f32, total_chunks: usize) -> Self {
        Self {
            threshold,
            total_chunks,
            chunks_skipped_llm: 0,
            chunks_partial_llm: 0,
            chunks_full_llm: 0,
            entities_axiomatic: 0,
            entities_llm: 0,
            axio_times_ms: Vec::new(),
            llm_times_ms: Vec::new(),
            method_breakdown: HashMap::new(),
        }
    }

    pub fn record_chunk(
        &mut self,
        path: ChunkPath,
        elapsed_ms: f64,
        n_axio: usize,
        n_llm: usize,
        methods: &[ClassificationMethod],
    ) {
        match path {
            ChunkPath::FullAxiomatic => {
                self.chunks_skipped_llm += 1;
                self.axio_times_ms.push(elapsed_ms);
            }
            ChunkPath::FocusedLlm => {
                self.chunks_partial_llm += 1;
                self.llm_times_ms.push(elapsed_ms);
            }
            ChunkPath::FullLlm => {
                self.chunks_full_llm += 1;
                self.llm_times_ms.push(elapsed_ms);
            }
        }
        self.entities_axiomatic += n_axio;
        self.entities_llm += n_llm;
        for m in methods {
            *self
                .method_breakdown
                .entry(m.as_str().to_string())
                .or_insert(0) += 1;
        }
    }

    pub fn finalise(self, wall_secs: f64) -> AxiomaticRunMetrics {
        let total_entities = self.entities_axiomatic + self.entities_llm;
        let llm_skip_pct = if self.total_chunks > 0 {
            self.chunks_skipped_llm as f64 / self.total_chunks as f64 * 100.0
        } else {
            0.0
        };
        let axiomatic_entity_pct = if total_entities > 0 {
            self.entities_axiomatic as f64 / total_entities as f64 * 100.0
        } else {
            0.0
        };

        AxiomaticRunMetrics {
            timestamp: chrono::Utc::now().to_rfc3339(),
            threshold: self.threshold,
            total_chunks: self.total_chunks,
            chunks_skipped_llm: self.chunks_skipped_llm,
            chunks_partial_llm: self.chunks_partial_llm,
            chunks_full_llm: self.chunks_full_llm,
            entities_axiomatic: self.entities_axiomatic,
            entities_llm: self.entities_llm,
            total_wall_secs: wall_secs,
            mean_ms_per_chunk_axio: mean(&self.axio_times_ms),
            p95_ms_per_chunk_axio: p95(&self.axio_times_ms),
            mean_ms_per_chunk_llm: mean(&self.llm_times_ms),
            p95_ms_per_chunk_llm: p95(&self.llm_times_ms),
            llm_skip_pct,
            axiomatic_entity_pct,
            method_breakdown: self.method_breakdown,
        }
    }
}

/// Finalised, serialisable metrics for one graph build run.
#[derive(Debug, Serialize, Deserialize)]
pub struct AxiomaticRunMetrics {
    pub timestamp: String,
    pub threshold: f32,
    pub total_chunks: usize,
    pub chunks_skipped_llm: usize,
    pub chunks_partial_llm: usize,
    pub chunks_full_llm: usize,
    pub entities_axiomatic: usize,
    pub entities_llm: usize,
    pub total_wall_secs: f64,
    pub mean_ms_per_chunk_axio: f64,
    pub p95_ms_per_chunk_axio: f64,
    pub mean_ms_per_chunk_llm: f64,
    pub p95_ms_per_chunk_llm: f64,
    pub llm_skip_pct: f64,
    pub axiomatic_entity_pct: f64,
    pub method_breakdown: HashMap<String, usize>,
}

impl AxiomaticRunMetrics {
    pub fn print_summary(&self) {
        let sep = "─".repeat(62);
        println!("\n{sep}");
        println!("  Axiomatic Extraction Summary");
        println!("{sep}");
        println!("  Threshold       : {:.2}", self.threshold);
        println!("  Total chunks    : {}", self.total_chunks);
        println!(
            "  Skipped LLM     : {} ({:.1}%)  ← no LLM call",
            self.chunks_skipped_llm, self.llm_skip_pct
        );
        let partial_pct = self.chunks_partial_llm as f64 / self.total_chunks.max(1) as f64 * 100.0;
        println!(
            "  Focused prompt  : {} ({:.1}%)  ← partial LLM",
            self.chunks_partial_llm, partial_pct
        );
        let full_pct = self.chunks_full_llm as f64 / self.total_chunks.max(1) as f64 * 100.0;
        println!(
            "  Full LLM        : {} ({:.1}%)  ← baseline path",
            self.chunks_full_llm, full_pct
        );
        let total_e = self.entities_axiomatic + self.entities_llm;
        println!("  Entities total  : {total_e}");
        println!(
            "  Axiomatic       : {} ({:.1}%)",
            self.entities_axiomatic, self.axiomatic_entity_pct
        );
        println!("  Via LLM         : {}", self.entities_llm);
        println!(
            "  Wall-clock      : {}",
            format_duration(self.total_wall_secs)
        );
        if self.mean_ms_per_chunk_axio > 0.0 {
            println!(
                "  Axio latency    : {:.3}ms mean  {:.3}ms p95",
                self.mean_ms_per_chunk_axio, self.p95_ms_per_chunk_axio
            );
        }
        if self.mean_ms_per_chunk_llm > 0.0 {
            println!(
                "  LLM latency     : {:.1}s mean  {:.1}s p95",
                self.mean_ms_per_chunk_llm / 1000.0,
                self.p95_ms_per_chunk_llm / 1000.0
            );
        }
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

    fn empty_snapshot() -> HashMap<String, String> {
        HashMap::new()
    }

    #[test]
    fn honorific_prefix_classifies_as_person() {
        let candidates = vec!["Dr. Abdurahman".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert_eq!(result[0].entity_type.as_deref(), Some("Person"));
        assert_eq!(result[0].method, ClassificationMethod::HonorificPrefix);
        assert!(result[0].composite_confidence > 0.8);
        assert_eq!(
            result[0]
                .axiomatic_fields
                .get("occupation")
                .map(|s| s.as_str()),
            Some("doctor")
        );
    }

    #[test]
    fn bare_honorific_alone_does_not_classify_as_person() {
        // A standalone "Rev" with nothing following it is a title fragment, not a
        // name — must not commit as Person via HonorificPrefix (regression: this
        // previously classified with composite_confidence 0.81, high enough to
        // auto-commit and then attract spurious relations from the legacy pipeline).
        let candidates = vec!["Rev".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert_ne!(result[0].method, ClassificationMethod::HonorificPrefix);
        assert_eq!(result[0].composite_confidence, 0.0);
    }

    #[test]
    fn org_marker_last_word() {
        let candidates = vec!["City Council".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert_eq!(result[0].entity_type.as_deref(), Some("Organization"));
        assert_eq!(result[0].method, ClassificationMethod::OrgMarker);
    }

    #[test]
    fn legislation_marker_last_word() {
        let candidates = vec!["Group Areas Act".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert_eq!(result[0].entity_type.as_deref(), Some("Legislation"));
    }

    #[test]
    fn geo_marker_any_word() {
        let candidates = vec!["District Six".to_string(), "Hanover Street".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert!(result
            .iter()
            .all(|c| c.entity_type.as_deref() == Some("Place")));
    }

    #[test]
    fn known_entity_lookup() {
        let mut snap = HashMap::new();
        snap.insert("yousuf rassool".to_string(), "Person".to_string());
        let candidates = vec!["Yousuf Rassool".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &snap, &[]);
        assert_eq!(result[0].method, ClassificationMethod::KnownEntity);
        assert!(result[0].composite_confidence > 0.9);
    }

    #[test]
    fn known_entity_with_honorific_prefix_stripped() {
        let mut snap = HashMap::new();
        snap.insert("abdurahman".to_string(), "Person".to_string());
        let candidates = vec!["Dr. Abdurahman".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &snap, &[]);
        assert_eq!(result[0].method, ClassificationMethod::KnownEntity);
    }

    #[test]
    fn unknown_candidate() {
        let candidates = vec!["Foobar".to_string()];
        let result = classify_candidates_axiomatic(&candidates, &empty_snapshot(), &[]);
        assert_eq!(result[0].method, ClassificationMethod::Unknown);
        assert_eq!(result[0].composite_confidence, 0.0);
    }

    #[test]
    fn split_by_confidence_basic() {
        let candidates = vec![
            TypedCandidate {
                name: "A".to_string(),
                entity_type: Some("Person".to_string()),
                type_confidence: 0.9,
                mention_confidence: 0.9,
                composite_confidence: 0.81,
                method: ClassificationMethod::HonorificPrefix,
                axiomatic_fields: HashMap::new(),
            },
            TypedCandidate {
                name: "B".to_string(),
                entity_type: None,
                type_confidence: 0.0,
                mention_confidence: 0.5,
                composite_confidence: 0.0,
                method: ClassificationMethod::Unknown,
                axiomatic_fields: HashMap::new(),
            },
        ];
        let (high, low) = split_by_confidence(candidates, 0.70);
        assert_eq!(high.len(), 1);
        assert_eq!(high[0].name, "A");
        assert_eq!(low.len(), 1);
        assert_eq!(low[0].name, "B");
    }

    #[test]
    fn validate_single_word_demoted() {
        let candidates = vec![TypedCandidate {
            name: "Cape".to_string(),
            entity_type: Some("Place".to_string()),
            type_confidence: 0.80,
            mention_confidence: 0.80,
            composite_confidence: 0.64,
            method: ClassificationMethod::GeoMarker,
            axiomatic_fields: HashMap::new(),
        }];
        let validated = validate_with_axioms(candidates, "some text Cape");
        assert_eq!(validated[0].composite_confidence, 0.0);
    }

    #[test]
    fn validate_substring_dominance() {
        let candidates = vec![
            TypedCandidate {
                name: "Teachers League".to_string(),
                entity_type: Some("Organization".to_string()),
                type_confidence: 0.85,
                mention_confidence: 0.85,
                composite_confidence: 0.7225,
                method: ClassificationMethod::OrgMarker,
                axiomatic_fields: HashMap::new(),
            },
            TypedCandidate {
                name: "Teachers League of South Africa".to_string(),
                entity_type: Some("Organization".to_string()),
                type_confidence: 0.85,
                mention_confidence: 0.85,
                composite_confidence: 0.7225,
                method: ClassificationMethod::OrgMarker,
                axiomatic_fields: HashMap::new(),
            },
        ];
        let validated = validate_with_axioms(candidates, "Teachers League of South Africa met");
        let shorter = validated
            .iter()
            .find(|c| c.name == "Teachers League")
            .unwrap();
        assert_eq!(shorter.composite_confidence, 0.0);
        let longer = validated
            .iter()
            .find(|c| c.name == "Teachers League of South Africa")
            .unwrap();
        assert!(longer.composite_confidence > 0.0);
    }
}
