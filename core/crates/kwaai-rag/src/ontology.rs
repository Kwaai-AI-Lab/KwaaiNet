//! Per-KB ontology: the knowledge schema for one corpus, loaded from YAML.
//!
//! This module exists because domain knowledge was compiled into the extractor.
//! `kwaai-rag` serves sixteen heterogeneous knowledge bases from one binary, so
//! a `const` table describing one corpus silently applies to all of them. That
//! is not a style problem — it caused real loss:
//!
//!   * a stop-list containing "Hatless", a named District Six character, so he
//!     could never enter any graph;
//!   * a narrator gender hardcoded `Some("Male")`, mis-gendering any corpus
//!     with a woman narrator and corrupting its coreference resolution;
//!   * a list of 1930s comics characters sent to the RFC and climate corpora;
//!   * a graph scorer that rewards kinship relations, making "graph score" a
//!     memoir-conformance metric rather than a quality one.
//!
//! Everything here is **additive**: every accessor falls back to the compiled
//! table when a KB has no ontology, so the fifteen KBs without one behave
//! exactly as before. See `projects/kwaai-knowledge/plans/PerKBOntology-plan.md`.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::graph::{EntityMarkers, KBEntityTypeSchema, RelationTrigger};

/// Header block: identity and inheritance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OntologyHeader {
    pub name: String,
    #[serde(default = "one")]
    pub version: u32,
    /// Name of a built-in module to inherit from, or "none".
    #[serde(default)]
    pub extends: Option<String>,
}
fn one() -> u32 {
    1
}

/// A structured metadata field expected on an entity type.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FieldDef {
    pub key: String,
    #[serde(default)]
    pub desc: String,
}

/// One entity type as declared by a corpus's ontology.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityTypeDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub anti_examples: Vec<String>,
    /// Must survive compression verbatim (exact quotations, numbers, statutes).
    #[serde(default)]
    pub irreducible: bool,
    #[serde(default)]
    pub fields: Vec<FieldDef>,
    #[serde(default)]
    pub markers: EntityMarkers,
    /// schema.org type this maps to, replacing `scorer::schema_type_for`'s
    /// compiled 20-arm match for KBs that declare one.
    #[serde(default)]
    pub schema_type: Option<String>,
    /// Measured share of corpus chunks carrying this type's vocabulary.
    /// Documentation of why the type earns its place; not consumed by code.
    #[serde(default)]
    pub evidence: Option<f32>,
}

/// One predicate, with its domain/range constraints and lexical triggers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationTypeDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub domain: Vec<String>,
    #[serde(default)]
    pub range: Vec<String>,
    #[serde(default)]
    pub inverse: Option<String>,
    #[serde(default)]
    pub symmetric: bool,
    #[serde(default)]
    pub transitive: bool,
    #[serde(default)]
    pub acyclic: bool,
    #[serde(default)]
    pub functional: bool,
    /// Lexical trigger phrases; the relation half of the corpus's lexicon.
    #[serde(default)]
    pub triggers: Vec<String>,
    /// Confidence assigned to an axiomatic match on any of `triggers`.
    #[serde(default = "default_trigger_conf")]
    pub confidence: f32,
}
fn default_trigger_conf() -> f32 {
    0.75
}

/// A declarative constraint. Generalises the compiled contradiction table,
/// `FAMILIAL_INVERSE`, and the familial-requires-Person rule.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AxiomDef {
    pub kind: String,
    #[serde(default)]
    pub pairs: Vec<Vec<String>>,
    #[serde(default)]
    pub relations: Vec<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// A document kind within one KB, with its own type subset and ingest policy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamDef {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub entity_types: Vec<String>,
    /// "skip" excludes this stream from extraction entirely.
    #[serde(default)]
    pub ingest: Option<String>,
    #[serde(default)]
    pub retention: Option<String>,
}

/// Text-normalisation rules, which are properties of a corpus's *source
/// format*, not of language. `normalize_underscores` was compiled in with the
/// comment "in this corpus underscores replace periods in initials" — true of
/// one PDF and applied to every document in every KB.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TextRules {
    /// Rewrite `J_ M_ H_` → `J. M. H.` (a pdf-extract glyph-mapping artifact).
    #[serde(default)]
    pub underscores_are_initials: bool,
    /// Extra corpus-specific noise words to suppress, beyond the global list.
    #[serde(default)]
    pub stop_words: Vec<String>,
    /// Definite descriptions ("the district", "my grandfather") that resolve to
    /// entities of a given type. Compiled tables are the fallback.
    #[serde(default)]
    pub definite_descriptions: HashMap<String, Vec<String>>,
}

/// The complete per-KB knowledge schema.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ontology {
    #[serde(default)]
    pub ontology: OntologyHeader,
    #[serde(default)]
    pub entity_types: Vec<EntityTypeDef>,
    #[serde(default)]
    pub relation_types: Vec<RelationTypeDef>,
    #[serde(default)]
    pub axioms: Vec<AxiomDef>,
    #[serde(default)]
    pub streams: Vec<StreamDef>,
    #[serde(default)]
    pub text_rules: TextRules,
    /// Predicate admitted when no declared relation fits. `None` means a vague
    /// edge is a bug in this corpus — correct for law, standards and science.
    #[serde(default)]
    pub fallback_predicate: Option<String>,
    /// Gender of the narrator, when the corpus states one.
    #[serde(default)]
    pub narrator_gender: Option<String>,
}

impl Ontology {
    pub fn from_yaml(src: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(src)?)
    }

    pub fn is_empty(&self) -> bool {
        self.entity_types.is_empty() && self.relation_types.is_empty()
    }

    /// Entity type names, for the extraction prompt.
    pub fn entity_type_names(&self) -> Vec<&str> {
        self.entity_types.iter().map(|e| e.name.as_str()).collect()
    }

    /// Predicate names, for the extraction prompt and dream validation.
    pub fn relation_type_names(&self) -> Vec<&str> {
        self.relation_types.iter().map(|r| r.name.as_str()).collect()
    }

    pub fn entity_type(&self, name: &str) -> Option<&EntityTypeDef> {
        self.entity_types
            .iter()
            .find(|e| e.name.eq_ignore_ascii_case(name))
    }

    pub fn relation_type(&self, name: &str) -> Option<&RelationTypeDef> {
        self.relation_types
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case(name))
    }

    /// Flatten every declared trigger phrase into the matcher's shape.
    /// Phrases are lowercased once here so the hot path never re-allocates.
    pub fn triggers(&self) -> Vec<RelationTrigger> {
        let mut out = Vec::new();
        for r in &self.relation_types {
            for phrase in &r.triggers {
                out.push(RelationTrigger {
                    phrase: phrase.trim().to_lowercase(),
                    relation_type: r.name.clone(),
                    confidence: r.confidence,
                    reversed: is_reversed_phrase(phrase),
                });
            }
        }
        out
    }

    /// Bridge to the existing per-KB entity schema storage, so the ontology can
    /// drive the prompt and validation paths that already read `KBEntityTypeSchema`.
    pub fn to_kb_schemas(&self) -> Vec<KBEntityTypeSchema> {
        self.entity_types
            .iter()
            .map(|e| KBEntityTypeSchema {
                name: e.name.clone(),
                description: e.description.clone(),
                examples: e.examples.clone(),
                anti_examples: e.anti_examples.clone(),
                fields: e.fields.iter().map(|f| f.key.clone()).collect(),
                markers: e.markers.clone(),
            })
            .collect()
    }

    /// Domain/range check. Generalises the compiled
    /// "familial relations require two Person endpoints" rule to any predicate.
    /// Unknown endpoints pass, matching `upsert_relation`'s existing leniency:
    /// an entity absent from the graph cannot be the wrong type.
    pub fn relation_endpoints_valid(
        &self,
        relation: &str,
        subject_type: Option<&str>,
        object_type: Option<&str>,
    ) -> bool {
        let Some(def) = self.relation_type(relation) else {
            return true; // undeclared predicate — not ours to judge
        };
        let ok = |allowed: &[String], actual: Option<&str>| match actual {
            None => true,
            Some(t) => {
                allowed.is_empty() || allowed.iter().any(|a| a.eq_ignore_ascii_case(t))
            }
        };
        ok(&def.domain, subject_type) && ok(&def.range, object_type)
    }

    /// Inverse predicate to auto-store, replacing `FAMILIAL_INVERSE`.
    pub fn inverse_of(&self, relation: &str) -> Option<&str> {
        self.relation_type(relation)?.inverse.as_deref()
    }

    pub fn is_symmetric(&self, relation: &str) -> bool {
        self.relation_type(relation).is_some_and(|r| r.symmetric)
    }

    /// Contradiction pairs from the axiom block, in both orders.
    pub fn contradicts(&self, a: &str, b: &str) -> bool {
        self.axioms
            .iter()
            .filter(|ax| ax.kind == "contradiction")
            .flat_map(|ax| ax.pairs.iter())
            .any(|p| {
                p.len() == 2
                    && ((p[0].eq_ignore_ascii_case(a) && p[1].eq_ignore_ascii_case(b))
                        || (p[0].eq_ignore_ascii_case(b) && p[1].eq_ignore_ascii_case(a)))
            })
    }

    /// A predicate is functional unless an axiom declares otherwise.
    /// The `non_functional` kind exists because D6's narrator had a grandfather
    /// with two simultaneous wives: defaulting `spouse_of` to functional would
    /// silently discard half a household the book devotes a chapter to.
    pub fn is_functional(&self, relation: &str) -> bool {
        if self
            .axioms
            .iter()
            .filter(|ax| ax.kind == "non_functional")
            .any(|ax| ax.relations.iter().any(|r| r.eq_ignore_ascii_case(relation)))
        {
            return false;
        }
        self.relation_type(relation).is_some_and(|r| r.functional)
    }

    /// Streams marked `ingest: skip` — their chunks never reach extraction.
    pub fn skipped_streams(&self) -> Vec<&str> {
        self.streams
            .iter()
            .filter(|s| s.ingest.as_deref() == Some("skip"))
            .map(|s| s.name.as_str())
            .collect()
    }

    /// Expected relation groups for the scorer, derived from what the ontology
    /// actually declares about a type rather than from a compiled kinship
    /// assumption. Each group is the set of predicates whose domain admits this
    /// type; an entity scores when it carries at least one from each group.
    pub fn expected_relations_for(&self, entity_type: &str) -> Vec<String> {
        self.relation_types
            .iter()
            .filter(|r| {
                r.domain.is_empty()
                    || r.domain.iter().any(|d| d.eq_ignore_ascii_case(entity_type))
            })
            .map(|r| r.name.clone())
            .collect()
    }

    /// Field keys expected on a type, replacing `graph::expected_fields`'s
    /// compiled Person/Place/Organization/Legislation/Publication table.
    pub fn fields_for(&self, entity_type: &str) -> Vec<(&str, &str)> {
        self.entity_type(entity_type)
            .map(|e| {
                e.fields
                    .iter()
                    .map(|f| (f.key.as_str(), f.desc.as_str()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Type a candidate from the declared markers alone, no LLM call.
    /// Returns the best match across all types, most specific rule winning.
    pub fn classify_candidate(&self, candidate: &str) -> Option<(&str, f32)> {
        self.entity_types
            .iter()
            .filter_map(|e| {
                e.markers
                    .match_kind(candidate)
                    .map(|m| (e.name.as_str(), m.confidence()))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
    }
}

/// Trigger phrases where the object precedes the subject ("X founded by Y").
/// Passive and attributive constructions, recognised by shape rather than by a
/// hand-maintained list, so an ontology author gets this for free.
fn is_reversed_phrase(phrase: &str) -> bool {
    let p = phrase.trim().to_lowercase();
    p.ends_with(" by")
        || p.ends_with(" as")
        || p.ends_with(" a")
        || p.starts_with("born to")
}

#[cfg(test)]
mod tests {
    use super::*;

    const D6_FRAGMENT: &str = r#"
ontology:
  name: district-six-memoir
  version: 5
  extends: genealogy
narrator_gender: Male
entity_types:
  - name: Address
    evidence: 24.2
    markers:
      last: [street, road, cingle]
      any: [buitencingle]
    fields:
      - { key: occupant, desc: "who lived there" }
  - name: Person
    examples: [Hatless]
    markers:
      prefix: [haji, aboeta]
relation_types:
  - name: lived_at
    domain: [Person]
    range: [Address]
    confidence: 0.85
    triggers: ["lived at", "resided at"]
  - name: spouse_of
    symmetric: true
    functional: true
  - name: enacted_by
    triggers: ["passed by"]
axioms:
  - kind: non_functional
    relations: [spouse_of]
  - kind: contradiction
    pairs: [[attended, taught_at]]
streams:
  - name: verse
  - name: apparatus
    ingest: skip
text_rules:
  underscores_are_initials: true
  stop_words: [Beano]
"#;

    fn ont() -> Ontology {
        Ontology::from_yaml(D6_FRAGMENT).expect("fragment must parse")
    }

    #[test]
    fn parses_the_real_ontology_shape() {
        let o = ont();
        assert_eq!(o.ontology.name, "district-six-memoir");
        assert_eq!(o.ontology.version, 5);
        assert_eq!(o.narrator_gender.as_deref(), Some("Male"));
        assert!(o.text_rules.underscores_are_initials);
        assert_eq!(o.entity_type("Address").unwrap().evidence, Some(24.2));
    }

    #[test]
    fn triggers_flatten_with_reversal_detected_by_shape() {
        let t = ont().triggers();
        let lived = t.iter().find(|x| x.phrase == "lived at").unwrap();
        assert_eq!(lived.relation_type, "lived_at");
        assert_eq!(lived.confidence, 0.85);
        assert!(!lived.reversed);
        // "passed by" ends in " by" → object precedes subject
        assert!(t.iter().find(|x| x.phrase == "passed by").unwrap().reversed);
    }

    /// The polygamy case: an axiom must beat the field on the predicate.
    #[test]
    fn non_functional_axiom_overrides_a_functional_predicate() {
        let o = ont();
        assert!(o.relation_type("spouse_of").unwrap().functional);
        assert!(!o.is_functional("spouse_of"), "axiom must win");
        assert!(o.is_symmetric("spouse_of"));
    }

    #[test]
    fn domain_range_generalises_the_familial_person_rule() {
        let o = ont();
        assert!(o.relation_endpoints_valid("lived_at", Some("Person"), Some("Address")));
        assert!(!o.relation_endpoints_valid("lived_at", Some("Place"), Some("Address")));
        // unknown endpoints pass, matching upsert_relation's leniency
        assert!(o.relation_endpoints_valid("lived_at", None, Some("Address")));
        // an undeclared predicate is not ours to judge
        assert!(o.relation_endpoints_valid("mystery_of", Some("Place"), None));
    }

    #[test]
    fn contradiction_is_order_independent() {
        let o = ont();
        assert!(o.contradicts("attended", "taught_at"));
        assert!(o.contradicts("taught_at", "attended"));
        assert!(!o.contradicts("attended", "lived_at"));
    }

    #[test]
    fn markers_classify_without_an_llm() {
        let o = ont();
        let (t, conf) = o.classify_candidate("Chapel Street").unwrap();
        assert_eq!(t, "Address");
        assert!(conf > 0.8);
        assert_eq!(o.classify_candidate("Haji Joosub").unwrap().0, "Person");
        assert!(o.classify_candidate("something ordinary").is_none());
    }

    #[test]
    fn skipped_streams_are_named() {
        assert_eq!(ont().skipped_streams(), vec!["apparatus"]);
    }

    #[test]
    fn expected_relations_come_from_declared_domains_not_kinship() {
        let o = ont();
        let p = o.expected_relations_for("Person");
        assert!(p.contains(&"lived_at".to_string()));
        // Address is not in lived_at's domain, so it must not be expected there
        assert!(!o.expected_relations_for("Address").contains(&"lived_at".to_string()));
    }

    #[test]
    fn fields_come_from_the_ontology() {
        assert_eq!(ont().fields_for("Address"), vec![("occupant", "who lived there")]);
        assert!(ont().fields_for("Nonexistent").is_empty());
    }

    #[test]
    fn an_empty_ontology_is_inert_so_existing_kbs_are_unaffected() {
        let o = Ontology::default();
        assert!(o.is_empty());
        assert!(o.triggers().is_empty());
        assert!(o.classify_candidate("Chapel Street").is_none());
        assert!(o.relation_endpoints_valid("anything", Some("X"), Some("Y")));
        assert!(o.expected_relations_for("Person").is_empty());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::scorer::{expected_relation_groups_for, schema_type_for_ontology};

    /// The whole point of the exercise: a climate corpus must not be scored on
    /// whether its entities have kinship relations. Before the ontology path,
    /// `expected_relation_groups` gave every `schema:Person` a kinship group, so
    /// `graph score` measured memoir-conformance rather than quality.
    #[test]
    fn scorer_expectations_follow_the_corpus_not_kinship() {
        let climate = Ontology::from_yaml(
            r#"
ontology: { name: climate }
entity_types:
  - name: Phenomenon
    schema_type: "schema:Thing"
relation_types:
  - name: measured_by
    domain: [Phenomenon]
    range: [Instrument]
  - name: causes
    domain: [Phenomenon]
    range: [Phenomenon]
"#,
        )
        .unwrap();

        let groups = expected_relation_groups_for("Phenomenon", "schema:Thing", Some(&climate));
        let flat: Vec<&String> = groups.iter().flatten().collect();
        assert!(flat.iter().any(|r| *r == "measured_by"));
        assert!(
            !flat.iter().any(|r| r.contains("spouse") || r.contains("parent")),
            "a climate ontology must never expect kinship"
        );

        // Without an ontology the compiled groups still apply, unchanged.
        let legacy = expected_relation_groups_for("Person", "schema:Person", None);
        assert!(legacy.iter().flatten().any(|r| r == "spouse_of"));
    }

    /// A type the ontology declares but does not map must not score as Unknown —
    /// the compiled map only knows types that happened to appear in one corpus.
    #[test]
    fn declared_types_are_never_scored_as_unknown() {
        let o = Ontology::from_yaml(
            "ontology: { name: t }\nentity_types:\n  - name: TippingElement\n",
        )
        .unwrap();
        assert_eq!(schema_type_for_ontology("TippingElement", None), None);
        assert_eq!(
            schema_type_for_ontology("TippingElement", Some(&o)),
            Some("schema:Thing")
        );
    }

    /// The underscore→initials rewrite is one PDF's artifact. A corpus that uses
    /// underscores meaningfully (code docs) must keep them.
    #[test]
    fn text_rules_gate_the_underscore_rewrite() {
        use crate::document::clean_pdf_text_with_rules;
        let memoir = Ontology::from_yaml(
            "ontology: { name: m }\ntext_rules: { underscores_are_initials: true }\n",
        )
        .unwrap();
        let docs = Ontology::from_yaml("ontology: { name: d }\n").unwrap();

        assert_eq!(clean_pdf_text_with_rules("J_ M_ H_ Gool", Some(&memoir)), "J. M. H. Gool");
        assert!(
            clean_pdf_text_with_rules("call max_workers now", Some(&docs)).contains('_'),
            "a KB that does not opt in must keep its underscores"
        );
        // no ontology at all → historical behaviour preserved
        assert_eq!(clean_pdf_text_with_rules("J_ M_ H_ Gool", None), "J. M. H. Gool");
    }

    /// The Phase-4 verify prompt must offer the corpus's own predicates.
    #[test]
    fn in_scope_types_come_from_the_ontology() {
        use crate::relation_extract::in_scope_relation_types;
        let o = Ontology::from_yaml(
            "ontology: { name: l }\nrelation_types:\n  - name: overrules\n  - name: cites\n",
        )
        .unwrap();
        let idx = OntologyIndex::build(o);
        let scoped = in_scope_relation_types(Some(&idx));
        assert_eq!(scoped, vec!["overrules", "cites"]);
        assert!(!scoped.iter().any(|r| r == "spouse_of"));
        assert!(in_scope_relation_types(None).iter().any(|r| r == "spouse_of"));
    }

    /// Query patterns must reach beyond kinship, or "the founders of X" and
    /// "the author of RFC 8446" match nothing.
    #[test]
    fn query_patterns_extend_past_family_roles() {
        use crate::query_understand::query_patterns_from_ontology;
        let o = Ontology::from_yaml(
            r#"
ontology: { name: l }
relation_types:
  - name: founded
    triggers: ["founder of"]
"#,
        )
        .unwrap();
        let pats = query_patterns_from_ontology(Some(&o));
        assert!(pats.iter().any(|(p, r, _)| p.contains("founder of") && r == "founded"));
        // kinship patterns survive
        assert!(pats.iter().any(|(p, _, _)| p.starts_with("children of")));
    }
}

// ---------------------------------------------------------------------------
// Compiled index
// ---------------------------------------------------------------------------

/// An ontology prepared for the extraction hot path.
///
/// `Ontology` is the on-disk shape; this is the form the per-chunk loop wants.
/// Building it once per run and sharing it via `Arc` matters: the naive path
/// rebuilds 187 trigger `String`s for every sentence examined, which measured
/// 3× slower than hoisting, and hoisting alone still leaves a full linear scan
/// per sentence.
///
/// Two structural wins over a plain `Vec`:
///   * triggers are sorted by phrase length descending, so the **first** match
///     is the longest match and the scan can stop there — no `max_by_key` over
///     the whole table. This preserves the "half-brother of beats brother of"
///     rule the compiled matcher relies on.
///   * exact, last-word and prefix markers become hash lookups instead of
///     linear scans over every type's vector.
#[derive(Debug, Clone, Default)]
pub struct OntologyIndex {
    /// Sorted by `phrase.len()` descending. First hit wins.
    triggers: Vec<RelationTrigger>,
    /// Closed-vocabulary term → (entity type, confidence).
    exact: HashMap<String, (String, f32)>,
    /// Last word → (entity type, confidence).
    last: HashMap<String, (String, f32)>,
    /// First word → (entity type, confidence).
    prefix: HashMap<String, (String, f32)>,
    /// Substring markers, the only rule that still needs a scan.
    any: Vec<(String, String, f32)>,
    /// The ontology this was built from.
    ontology: Ontology,
}

impl OntologyIndex {
    pub fn build(ont: Ontology) -> Self {
        use crate::graph::MarkerMatch;
        let mut triggers = ont.triggers();
        // Longest first, so a scan can stop at its first hit.
        triggers.sort_by(|a, b| b.phrase.len().cmp(&a.phrase.len()));

        let mut exact = HashMap::new();
        let mut last = HashMap::new();
        let mut prefix = HashMap::new();
        let mut any: Vec<(String, String, f32)> = Vec::new();
        for e in &ont.entity_types {
            let ty = e.name.clone();
            // First declaration wins on collision, so ontology order is
            // meaningful and a later type cannot silently steal a marker.
            for m in &e.markers.exact {
                exact
                    .entry(m.to_lowercase())
                    .or_insert_with(|| (ty.clone(), MarkerMatch::Exact.confidence()));
            }
            for m in &e.markers.last {
                last.entry(m.to_lowercase())
                    .or_insert_with(|| (ty.clone(), MarkerMatch::Last.confidence()));
            }
            for m in &e.markers.prefix {
                prefix
                    .entry(m.to_lowercase())
                    .or_insert_with(|| (ty.clone(), MarkerMatch::Prefix.confidence()));
            }
            for m in &e.markers.any {
                any.push((m.to_lowercase(), ty.clone(), MarkerMatch::Any.confidence()));
            }
        }
        Self { triggers, exact, last, prefix, any, ontology: ont }
    }

    pub fn ontology(&self) -> &Ontology {
        &self.ontology
    }

    pub fn is_empty(&self) -> bool {
        self.ontology.is_empty()
    }

    /// Longest trigger phrase occurring in `sentence_lower`, or `None`.
    /// Stops at the first hit because the table is length-sorted.
    pub fn find_trigger(&self, sentence_lower: &str) -> Option<&RelationTrigger> {
        self.triggers
            .iter()
            .find(|t| sentence_lower.contains(t.phrase.as_str()))
    }

    /// Type a candidate by marker, most specific rule first.
    pub fn classify(&self, candidate: &str) -> Option<(&str, f32)> {
        let c = candidate.trim().to_lowercase();
        if c.is_empty() {
            return None;
        }
        if let Some((t, conf)) = self.exact.get(&c) {
            return Some((t.as_str(), *conf));
        }
        let words: Vec<&str> = c.split_whitespace().collect();
        let strip = |w: &str| w.trim_matches(|ch: char| !ch.is_alphanumeric()).to_string();
        if let Some(w) = words.last() {
            if let Some((t, conf)) = self.last.get(&strip(w)) {
                return Some((t.as_str(), *conf));
            }
        }
        if let Some(w) = words.first() {
            if let Some((t, conf)) = self.prefix.get(&strip(w)) {
                return Some((t.as_str(), *conf));
            }
        }
        self.any
            .iter()
            .find(|(m, _, _)| c.contains(m.as_str()))
            .map(|(_, t, conf)| (t.as_str(), *conf))
    }
}

#[cfg(test)]
mod index_tests {
    use super::*;

    fn idx(y: &str) -> OntologyIndex {
        OntologyIndex::build(Ontology::from_yaml(y).unwrap())
    }

    /// Length-descending order must preserve longest-match-wins, which is what
    /// keeps "half-brother of" from being swallowed by "brother of".
    #[test]
    fn first_hit_is_the_longest_match() {
        let i = idx(r#"
ontology: { name: t }
relation_types:
  - name: sibling_of
    triggers: ["brother of"]
  - name: half_sibling_of
    triggers: ["half-brother of"]
"#);
        let got = i.find_trigger("he was the half-brother of ahmed").unwrap();
        assert_eq!(got.relation_type, "half_sibling_of");
        let got = i.find_trigger("he was the brother of ahmed").unwrap();
        assert_eq!(got.relation_type, "sibling_of");
        assert!(i.find_trigger("nothing relevant here").is_none());
    }

    /// Marker specificity must survive the hash-map rewrite.
    #[test]
    fn marker_precedence_is_preserved() {
        let i = idx(r#"
ontology: { name: t }
entity_types:
  - name: RacialClassification
    markers: { exact: [malay] }
  - name: Address
    markers: { last: [street], any: [chapel] }
  - name: Person
    markers: { prefix: [haji] }
"#);
        assert_eq!(i.classify("Malay").unwrap().0, "RacialClassification");
        assert_eq!(i.classify("Chapel Street").unwrap().0, "Address");
        assert_eq!(i.classify("Haji Joosub").unwrap().0, "Person");
        assert!(i.classify("ordinary words").is_none());
        // exact must outrank any
        assert!(i.classify("Malay").unwrap().1 > i.classify("chapel lane").map_or(0.0, |x| x.1));
    }

    /// Declaration order decides marker collisions, so an ontology author can
    /// reason about precedence instead of discovering it.
    #[test]
    fn first_declaration_wins_a_marker_collision() {
        let i = idx(r#"
ontology: { name: t }
entity_types:
  - name: Address
    markers: { last: [hall] }
  - name: Venue
    markers: { last: [hall] }
"#);
        assert_eq!(i.classify("Memorial Hall").unwrap().0, "Address");
    }

    #[test]
    fn an_empty_ontology_indexes_to_nothing() {
        let i = OntologyIndex::default();
        assert!(i.is_empty());
        assert!(i.find_trigger("anything at all").is_none());
        assert!(i.classify("anything").is_none());
    }
}

#[cfg(test)]
mod wiring_tests {
    use super::*;
    use crate::axiom_extract::{classify_candidates_with_ontology, ClassificationMethod};
    use std::collections::HashMap;

    fn d6_like() -> OntologyIndex {
        OntologyIndex::build(
            Ontology::from_yaml(
                r#"
ontology: { name: d6 }
entity_types:
  - name: Address
    markers: { last: [street, cingle] }
  - name: Doctrine
    markers: { exact: [boycott] }
relation_types:
  - name: lived_at
    domain: [Person]
    range: [Address]
    confidence: 0.85
    triggers: ["lived at"]
"#,
            )
            .unwrap(),
        )
    }

    /// The end the whole exercise is for: "Chapel Street" is typed `Address` by
    /// the corpus's own marker, with no LLM call. Under the compiled tables it
    /// would be `Location` at best — GEO_MARKERS_ANY does not distinguish a
    /// street from a settlement, which is why the corpus's strongest signal
    /// (24.2% of chunks) had nowhere to land.
    #[test]
    fn ontology_markers_type_a_candidate_without_an_llm() {
        let idx = d6_like();
        let snap: HashMap<String, String> = HashMap::new();
        let got = classify_candidates_with_ontology(
            &["Chapel Street".to_string(), "boycott".to_string()],
            &snap,
            &[],
            Some(&idx),
        );
        let addr = got.iter().find(|c| c.name == "Chapel Street").unwrap();
        assert_eq!(addr.entity_type.as_deref(), Some("Address"));
        assert_eq!(addr.method, ClassificationMethod::OntologyMarker);
        let doc = got.iter().find(|c| c.name == "boycott").unwrap();
        assert_eq!(doc.entity_type.as_deref(), Some("Doctrine"));
        // exact marker is the most specific rule, so it must score highest
        assert!(doc.type_confidence > addr.type_confidence);
    }

    /// Passing `None` must reproduce the pre-ontology path exactly — this is the
    /// guarantee that the other fifteen KBs are untouched.
    #[test]
    fn without_an_ontology_classification_is_byte_for_byte_unchanged() {
        let snap: HashMap<String, String> = HashMap::new();
        let cands = vec!["Chapel Street".to_string(), "Haji Joosub".to_string()];
        let with_none = classify_candidates_with_ontology(&cands, &snap, &[], None);
        let legacy = crate::axiom_extract::classify_candidates_axiomatic(&cands, &snap, &[]);
        assert_eq!(with_none.len(), legacy.len());
        for (a, b) in with_none.iter().zip(legacy.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(a.entity_type, b.entity_type);
            assert_eq!(a.method, b.method);
            assert_eq!(a.composite_confidence, b.composite_confidence);
        }
    }

    /// A known graph entity must still beat a lexical marker: the graph is
    /// evidence, the marker is a guess.
    #[test]
    fn a_known_entity_outranks_an_ontology_marker() {
        let idx = d6_like();
        let mut snap = HashMap::new();
        snap.insert("chapel street".to_string(), "Place".to_string());
        let got =
            classify_candidates_with_ontology(&["Chapel Street".to_string()], &snap, &[], Some(&idx));
        assert_eq!(got[0].entity_type.as_deref(), Some("Place"));
        assert_eq!(got[0].method, ClassificationMethod::KnownEntity);
    }

    #[test]
    fn triggers_reach_the_relation_matcher() {
        use crate::relation_extract::find_trigger_with_ontology;
        let idx = d6_like();
        let (_, _, phrase, rel, method, conf) =
            find_trigger_with_ontology("he lived at chapel street", Some(&idx)).unwrap();
        assert_eq!(phrase, "lived at");
        assert_eq!(rel, "lived_at");
        assert_eq!(conf, 0.85);
        assert_eq!(
            method,
            crate::relation_extract::RelationClassificationMethod::OntologyTrigger
        );
    }
}
