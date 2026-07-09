//! Unit tests for kwaai_rag::axiom_extract — axiomatic entity pre-classification.
//!
//! Tests classify_candidates_axiomatic, split_by_confidence, validate_with_axioms,
//! and the ChunkPath / ClassificationMethod enums. No LLM or network required.

use std::collections::HashMap;

use kwaai_rag::axiom_extract::{
    classify_candidates_axiomatic, split_by_confidence, validate_with_axioms, ClassificationMethod,
};

// ── helpers ───────────────────────────────────────────────────────────────────

fn empty_snap() -> HashMap<String, String> {
    HashMap::new()
}

fn classify_one(
    name: &str,
    snap: &HashMap<String, String>,
) -> kwaai_rag::axiom_extract::TypedCandidate {
    let mut r = classify_candidates_axiomatic(&[name.to_string()], snap, &[]);
    assert!(!r.is_empty(), "classify returned empty for {name:?}");
    r.remove(0)
}

// ── classification: honorific prefixes ────────────────────────────────────────

#[test]
fn classify_honorific_dr_is_person() {
    let c = classify_one("Dr. Smith", &empty_snap());
    assert_eq!(
        c.entity_type.as_deref(),
        Some("Person"),
        "method={:?}",
        c.method
    );
    assert!(
        matches!(c.method, ClassificationMethod::HonorificPrefix),
        "expected HonorificPrefix, got {:?}",
        c.method
    );
    assert!(
        c.composite_confidence >= 0.80,
        "composite={} should be ≥0.80",
        c.composite_confidence
    );
}

#[test]
fn classify_honorific_haji_is_person() {
    let c = classify_one("Haji Joosub", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Person"));
    assert!(matches!(c.method, ClassificationMethod::HonorificPrefix));
}

#[test]
fn classify_honorific_cllr_is_person() {
    let c = classify_one("Cllr. Adams", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Person"));
    assert!(matches!(c.method, ClassificationMethod::HonorificPrefix));
}

#[test]
fn classify_honorific_prof_is_person() {
    let c = classify_one("Prof. Rassool", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Person"));
    assert!(matches!(c.method, ClassificationMethod::HonorificPrefix));
}

#[test]
fn classify_honorific_sheikh_is_person() {
    let c = classify_one("Sheikh Ahmed", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Person"));
    assert!(matches!(c.method, ClassificationMethod::HonorificPrefix));
}

// ── classification: organization markers ─────────────────────────────────────

#[test]
fn classify_org_marker_council_is_organization() {
    let c = classify_one("Cape Town Council", &empty_snap());
    assert_eq!(
        c.entity_type.as_deref(),
        Some("Organization"),
        "method={:?}",
        c.method
    );
    assert!(matches!(c.method, ClassificationMethod::OrgMarker));
}

#[test]
fn classify_org_marker_league_is_organization() {
    let c = classify_one("Cricket League", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Organization"));
    assert!(matches!(c.method, ClassificationMethod::OrgMarker));
}

#[test]
fn classify_org_marker_congress_is_organization() {
    let c = classify_one("African National Congress", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Organization"));
    assert!(matches!(c.method, ClassificationMethod::OrgMarker));
}

// ── classification: geo markers ───────────────────────────────────────────────

#[test]
fn classify_geo_marker_mountain_is_place() {
    let c = classify_one("Table Mountain", &empty_snap());
    assert_eq!(
        c.entity_type.as_deref(),
        Some("Place"),
        "method={:?}",
        c.method
    );
    assert!(matches!(c.method, ClassificationMethod::GeoMarker));
}

#[test]
fn classify_geo_marker_district_is_place() {
    let c = classify_one("District Six", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Place"));
    assert!(matches!(c.method, ClassificationMethod::GeoMarker));
}

// ── classification: legislation markers ──────────────────────────────────────

#[test]
fn classify_legislation_act_at_end_is_legislation() {
    let c = classify_one("Group Areas Act", &empty_snap());
    assert_eq!(
        c.entity_type.as_deref(),
        Some("Legislation"),
        "method={:?}",
        c.method
    );
    assert!(matches!(c.method, ClassificationMethod::LegislationMarker));
}

#[test]
fn classify_legislation_ordinance_is_legislation() {
    let c = classify_one("Cape Municipalities Ordinance", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Legislation"));
    assert!(matches!(c.method, ClassificationMethod::LegislationMarker));
}

// ── classification: publication markers ──────────────────────────────────────

#[test]
fn classify_publication_times_is_publication() {
    let c = classify_one("Cape Times", &empty_snap());
    assert_eq!(
        c.entity_type.as_deref(),
        Some("Publication"),
        "method={:?}",
        c.method
    );
    assert!(matches!(c.method, ClassificationMethod::PublicationMarker));
}

#[test]
fn classify_publication_argus_is_publication() {
    let c = classify_one("Cape Argus", &empty_snap());
    assert_eq!(c.entity_type.as_deref(), Some("Publication"));
    assert!(matches!(c.method, ClassificationMethod::PublicationMarker));
}

// ── classification: known entity lookup ──────────────────────────────────────

#[test]
fn classify_known_entity_from_snapshot() {
    let mut snap = HashMap::new();
    snap.insert("joe rassool".to_string(), "Person".to_string());
    let c = classify_one("Joe Rassool", &snap);
    assert_eq!(c.entity_type.as_deref(), Some("Person"));
    assert!(matches!(c.method, ClassificationMethod::KnownEntity));
    assert!(
        c.composite_confidence >= 0.90,
        "KnownEntity composite={} should be ≥0.90",
        c.composite_confidence
    );
}

#[test]
fn classify_known_entity_beats_other_rules() {
    // "Cape Town Council" would normally be OrgMarker, but if it's known → KnownEntity wins
    let mut snap = HashMap::new();
    snap.insert("cape town council".to_string(), "Organization".to_string());
    let c = classify_one("Cape Town Council", &snap);
    assert!(
        matches!(c.method, ClassificationMethod::KnownEntity),
        "got {:?}",
        c.method
    );
}

// ── classification: unknown fallthrough ───────────────────────────────────────

#[test]
fn classify_unknown_candidate_method_is_unknown() {
    let c = classify_one("George", &empty_snap());
    assert!(
        matches!(c.method, ClassificationMethod::Unknown),
        "got {:?}",
        c.method
    );
}

#[test]
fn classify_empty_candidates_returns_empty() {
    let result = classify_candidates_axiomatic(&[], &empty_snap(), &[]);
    assert!(result.is_empty());
}

#[test]
fn classify_multiple_candidates_all_classified() {
    let names = vec![
        "Dr. Smith".to_string(),
        "Cape Town Council".to_string(),
        "Table Mountain".to_string(),
    ];
    let result = classify_candidates_axiomatic(&names, &empty_snap(), &[]);
    assert_eq!(result.len(), 3, "should return one result per candidate");
}

// ── confidence split ──────────────────────────────────────────────────────────

#[test]
fn split_empty_input_returns_empty_pairs() {
    let (hi, lo) = split_by_confidence(vec![], 0.80);
    assert!(hi.is_empty());
    assert!(lo.is_empty());
}

#[test]
fn split_t080_honorific_goes_high() {
    // HonorificPrefix composite (0.81) is ≥ T=0.80 → high_conf
    let candidates = classify_candidates_axiomatic(&["Dr. Smith".to_string()], &empty_snap(), &[]);
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert_eq!(
        hi.len(),
        1,
        "HonorificPrefix should be high-conf at T=0.80; lo={lo:?}"
    );
    assert!(lo.is_empty());
}

#[test]
fn split_t080_org_marker_goes_low() {
    // OrgMarker composite (0.7225) is < T=0.80 → low_conf
    let candidates =
        classify_candidates_axiomatic(&["Cape Town Council".to_string()], &empty_snap(), &[]);
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert_eq!(
        lo.len(),
        1,
        "OrgMarker should be low-conf at T=0.80; hi={hi:?}"
    );
    assert!(hi.is_empty());
}

#[test]
fn split_known_entity_always_high_at_t080() {
    let mut snap = HashMap::new();
    snap.insert("joe rassool".to_string(), "Person".to_string());
    let candidates = classify_candidates_axiomatic(&["Joe Rassool".to_string()], &snap, &[]);
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert_eq!(
        hi.len(),
        1,
        "KnownEntity should be high-conf at T=0.80; lo={lo:?}"
    );
    assert!(lo.is_empty());
}

#[test]
fn split_unknown_goes_low() {
    let candidates = classify_candidates_axiomatic(&["George".to_string()], &empty_snap(), &[]);
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert!(
        hi.is_empty(),
        "Unknown candidate should not be high-conf at T=0.80"
    );
    assert_eq!(lo.len(), 1);
}

#[test]
fn split_mixed_batch_splits_correctly() {
    let mut snap = HashMap::new();
    snap.insert("joe rassool".to_string(), "Person".to_string());
    // Joe Rassool → KnownEntity (high), Cape Town Council → OrgMarker (low at T=0.80)
    let candidates = classify_candidates_axiomatic(
        &["Joe Rassool".to_string(), "Cape Town Council".to_string()],
        &snap,
        &[],
    );
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert_eq!(hi.len(), 1, "one high-conf (KnownEntity)");
    assert_eq!(lo.len(), 1, "one low-conf (OrgMarker)");
}

#[test]
fn split_at_t060_org_marker_goes_high() {
    // At a lower threshold (0.60), OrgMarker composite (0.7225) should be high
    let candidates =
        classify_candidates_axiomatic(&["Cape Town Council".to_string()], &empty_snap(), &[]);
    let (hi, lo) = split_by_confidence(candidates, 0.60);
    assert_eq!(
        hi.len(),
        1,
        "OrgMarker should be high-conf at T=0.60; lo={lo:?}"
    );
    assert!(lo.is_empty());
}

#[test]
fn split_preserves_total_count() {
    let names: Vec<String> = vec!["Dr. Smith", "Cape Town Council", "Table Mountain", "George"]
        .into_iter()
        .map(str::to_string)
        .collect();
    let original_count = names.len();
    let candidates = classify_candidates_axiomatic(&names, &empty_snap(), &[]);
    let classified_count = candidates.len();
    let (hi, lo) = split_by_confidence(candidates, 0.80);
    assert_eq!(
        hi.len() + lo.len(),
        classified_count,
        "split must not lose candidates (original={original_count})"
    );
}

// ── validation ────────────────────────────────────────────────────────────────

#[test]
fn validate_empty_candidates_returns_empty() {
    let result = validate_with_axioms(vec![], "some text here");
    assert!(result.is_empty());
}

#[test]
fn validate_known_entity_passes_through() {
    let mut snap = HashMap::new();
    snap.insert("joe rassool".to_string(), "Person".to_string());
    let candidates = classify_candidates_axiomatic(&["Joe Rassool".to_string()], &snap, &[]);
    let validated = validate_with_axioms(candidates, "Joe Rassool was present at the meeting.");
    assert_eq!(validated.len(), 1, "KnownEntity should survive validation");
}

#[test]
fn validate_honorific_person_passes_through() {
    let candidates = classify_candidates_axiomatic(&["Dr. Smith".to_string()], &empty_snap(), &[]);
    let validated = validate_with_axioms(candidates, "Dr. Smith attended the event.");
    assert_eq!(
        validated.len(),
        1,
        "HonorificPrefix should survive validation"
    );
}

#[test]
fn validate_geo_marker_passes_through() {
    let candidates =
        classify_candidates_axiomatic(&["Table Mountain".to_string()], &empty_snap(), &[]);
    let validated =
        validate_with_axioms(candidates, "Table Mountain is visible from the city bowl.");
    assert_eq!(validated.len(), 1);
}

#[test]
fn validate_legislation_marker_passes_through() {
    let candidates =
        classify_candidates_axiomatic(&["Group Areas Act".to_string()], &empty_snap(), &[]);
    let validated = validate_with_axioms(
        candidates,
        "The Group Areas Act was enforced throughout the city.",
    );
    assert_eq!(validated.len(), 1);
}

#[test]
fn validate_does_not_increase_candidate_count() {
    let candidates = classify_candidates_axiomatic(
        &[
            "Dr. Smith".to_string(),
            "Cape Town Council".to_string(),
            "Table Mountain".to_string(),
        ],
        &empty_snap(),
        &[],
    );
    let original_len = candidates.len();
    let validated = validate_with_axioms(
        candidates,
        "Dr. Smith met with Cape Town Council near Table Mountain.",
    );
    assert!(
        validated.len() <= original_len,
        "validate must not add candidates (got {} from {original_len})",
        validated.len()
    );
}
