use sha2::{Digest, Sha256};

use crate::schema::{Claim, ClaimKind, EvidenceKind, EvidenceRef};

fn hash_parts(domain: &str, parts: &[&str]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(domain.as_bytes());

    for part in parts {
        let bytes = part.as_bytes();
        hasher.update((bytes.len() as u64).to_be_bytes());
        hasher.update(bytes);
    }

    hex::encode(hasher.finalize())
}

pub fn claim_id(kind: ClaimKind, subject: &str, predicate: &str, object: &str) -> String {
    let kind_label = match kind {
        ClaimKind::EntityField => "entity_field",
        ClaimKind::Relation => "relation",
    };

    hash_parts(
        "kwaai-provenance:claim:v1",
        &[kind_label, subject, predicate, object],
    )
}

pub fn evidence_id(
    kind: EvidenceKind,
    source_id: &str,
    content_hash: &str,
    chunk_id: Option<i64>,
) -> String {
    let kind_label = match kind {
        EvidenceKind::Original => "original",
        EvidenceKind::Derived => "derived",
    };

    let chunk = chunk_id
        .map(|id| id.to_string())
        .unwrap_or_else(|| "none".to_string());

    hash_parts(
        "kwaai-provenance:evidence:v1",
        &[kind_label, source_id, content_hash, &chunk],
    )
}

impl Claim {
    pub fn new(
        kind: ClaimKind,
        subject: impl Into<String>,
        predicate: impl Into<String>,
        object: impl Into<String>,
    ) -> Self {
        let subject = subject.into();
        let predicate = predicate.into();
        let object = object.into();

        let id = claim_id(kind.clone(), &subject, &predicate, &object);

        Self {
            id,
            kind,
            subject,
            predicate,
            object,
        }
    }
}

impl EvidenceRef {
    pub fn new(
        kind: EvidenceKind,
        source_id: impl Into<String>,
        content_hash: impl Into<String>,
        chunk_id: Option<i64>,
        parent_ids: Vec<String>,
    ) -> Self {
        let source_id = source_id.into();
        let content_hash = content_hash.into();

        let id = evidence_id(kind.clone(), &source_id, &content_hash, chunk_id);

        Self {
            id,
            kind,
            source_id,
            content_hash,
            chunk_id,
            parent_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_claims_have_identical_ids() {
        let a = Claim::new(ClaimKind::Relation, "Alice", "works_at", "Acme");
        let b = Claim::new(ClaimKind::Relation, "Alice", "works_at", "Acme");

        assert_eq!(a.id, b.id);
    }

    #[test]
    fn different_claims_have_different_ids() {
        let a = Claim::new(ClaimKind::Relation, "Alice", "works_at", "Acme");
        let b = Claim::new(ClaimKind::Relation, "Alice", "works_at", "Other");

        assert_ne!(a.id, b.id);
    }

    #[test]
    fn claim_kinds_are_distinct() {
        let a = Claim::new(ClaimKind::EntityField, "Alice", "works_at", "Acme");
        let b = Claim::new(ClaimKind::Relation, "Alice", "works_at", "Acme");

        assert_ne!(a.id, b.id);
    }

    #[test]
    fn evidence_identity_is_deterministic() {
        let a = EvidenceRef::new(
            EvidenceKind::Original,
            "document-1",
            "abc123",
            Some(42),
            vec![],
        );

        let b = EvidenceRef::new(
            EvidenceKind::Original,
            "document-1",
            "abc123",
            Some(42),
            vec![],
        );

        assert_eq!(a.id, b.id);
    }

    #[test]
    fn evidence_versions_have_different_ids() {
        let a = evidence_id(
            EvidenceKind::Original,
            "document-1",
            "hash-version-1",
            Some(42),
        );

        let b = evidence_id(
            EvidenceKind::Original,
            "document-1",
            "hash-version-2",
            Some(42),
        );

        assert_ne!(a, b);
    }

    #[test]
    fn hash_encoding_is_unambiguous() {
        let a = claim_id(ClaimKind::Relation, "ab", "c", "d");
        let b = claim_id(ClaimKind::Relation, "a", "bc", "d");

        assert_ne!(a, b);
    }
}
