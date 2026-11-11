//! Native provenance primitives for KwaaiNet.
//!
//! This crate is independent of kwaai-rag. It defines versioned
//! claim/evidence records and deterministic identifiers, an append-only
//! lineage log ([`lineage`]), and a policy gate ([`gate`]) that keeps
//! verification status separate from the decision applied to a mutation.
//!
//! Nothing here verifies meaning: evidence *association* is not *support*.
//! The only bundled verifier, [`gate::UnresolvedVerifier`], never reports
//! `Supported`.
//!
//! # Example
//!
//! ```
//! use kwaai_provenance::gate::{Decision, Gate, GateMode, UnresolvedVerifier};
//! use kwaai_provenance::lineage::{LineageEvent, LineageEventKind, LineageLog};
//! use kwaai_provenance::{Claim, ClaimKind};
//!
//! let claim = Claim::new(ClaimKind::Relation, "alice", "works_at", "kwaai");
//!
//! // Lineage: record that the claim was extracted from an original chunk.
//! let mut log = LineageLog::new();
//! let event = LineageEvent::new(
//!     LineageEventKind::Extract,
//!     ["orig:chunk-1"],
//!     [claim.id.as_str()],
//!     "ingest",
//!     None,
//!     0,
//! )?;
//! let id = log.propose(event)?;
//! assert!(!log.traces_to(&claim.id, |a| a.starts_with("orig:"))); // proposed only
//! log.commit(&id)?;
//! assert!(log.traces_to(&claim.id, |a| a.starts_with("orig:")));
//!
//! // Gate: Observe records what Enforce would do, but never blocks.
//! let out = Gate::new(GateMode::Observe).check(&claim, &[], &UnresolvedVerifier)?;
//! assert_eq!(out.recommended, Some(Decision::Quarantine));
//! assert_eq!(out.applied, Decision::Allow);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#![warn(missing_docs)]

pub mod gate;
pub mod identity;
pub mod lineage;
pub mod schema;

pub use identity::{claim_id, evidence_id};
pub use schema::{
    Claim, ClaimKind, EvidenceKind, EvidenceRef, SupportStatus, VerificationResult, SCHEMA_VERSION,
};
