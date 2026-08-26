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
        let scoped = in_scope_relation_types(Some(&o));
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
