//! Versioned claim, evidence and verification records.

use serde::{Deserialize, Serialize};

/// Version of the record schemas and ID domains in this crate.
pub const SCHEMA_VERSION: u32 = 1;

/// What a claim asserts about the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimKind {
    /// A structured field on one entity (subject = entity, predicate = field name).
    EntityField,
    /// A typed relation between two entities.
    Relation,
}

/// A subject–predicate–object assertion. `id` is derived from the other fields
/// (see [`crate::identity::claim_id`]); build with [`Claim::new`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    /// Content-derived identifier.
    pub id: String,
    /// Entity field or relation.
    pub kind: ClaimKind,
    /// Entity the claim is about. Byte-exact; not normalized.
    pub subject: String,
    /// Field name or relation type.
    pub predicate: String,
    /// Field value or target entity.
    pub object: String,
}

/// Whether evidence is source text or something produced from it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A passage from an ingested source document.
    Original,
    /// Content produced from other evidence (summary, fact card, synthetic chunk).
    Derived,
}

/// A reference to one evidence artifact. `id` is derived from kind, source,
/// content hash and chunk (see [`crate::identity::evidence_id`]); build with
/// [`EvidenceRef::new`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRef {
    /// Content-derived identifier. Does not cover `parent_ids`.
    pub id: String,
    /// Original or derived.
    pub kind: EvidenceKind,
    /// Source document or artifact identifier.
    pub source_id: String,
    /// Hash of the evidence content, supplied by the caller and not verified here.
    /// A new content version gets a new evidence ID.
    pub content_hash: String,
    /// Graph chunk ID, if any. `None` for synthetic or derived content.
    pub chunk_id: Option<i64>,
    /// Informational only. Not part of `id` and not authoritative history;
    /// derivations are recorded in [`crate::lineage::LineageLog`].
    pub parent_ids: Vec<String>,
}

/// A verifier's judgement of whether evidence semantically supports a claim.
/// Kept separate from the policy decision ([`crate::gate::Decision`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportStatus {
    /// The evidence supports the claim.
    Supported,
    /// The evidence does not support the claim.
    Unsupported,
    /// The verifier could not decide.
    Unresolved,
}

/// Output of one [`crate::gate::Verifier`] call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationResult {
    /// Claim that was verified.
    pub claim_id: String,
    /// Evidence the verifier considered.
    pub evidence_ids: Vec<String>,
    /// Verdict.
    pub status: SupportStatus,
    /// Optional verifier confidence. Not interpreted by the gate.
    pub score: Option<f64>,
    /// Name of the verifier that produced this result.
    pub verifier: String,
}
