//! Policy gating over verification results.
//!
//! Verification status ("is this claim supported by its evidence?") is kept
//! separate from the policy decision ("what do we do with the mutation?").
//! Observe and Enforce go through the same [`Gate::check`] path and the same
//! [`Verifier`]; the only difference is whether the recommended decision is applied.
//!
//! Scope: `Quarantine` is a decision only. There is no durable quarantine store yet.

use crate::schema::{Claim, EvidenceRef, SupportStatus, VerificationResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// How the gate treats verification results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateMode {
    /// No verification, no enforcement. Baseline KwaaiNet behaviour.
    Disabled,
    /// Verify and record; never block.
    Observe,
    /// Verify and apply the policy decision.
    Enforce,
}

/// Policy outcome for one proposed mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    /// Apply the mutation.
    Allow,
    /// Do not apply the mutation.
    Reject,
    /// Hold the mutation for review. Decision only; no durable store yet.
    Quarantine,
}

/// Status -> decision mapping. Default: Supported->Allow, Unsupported->Reject,
/// Unresolved->Quarantine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Policy {
    /// Decision for `SupportStatus::Supported`.
    pub on_supported: Decision,
    /// Decision for `SupportStatus::Unsupported`.
    pub on_unsupported: Decision,
    /// Decision for `SupportStatus::Unresolved`.
    pub on_unresolved: Decision,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            on_supported: Decision::Allow,
            on_unsupported: Decision::Reject,
            on_unresolved: Decision::Quarantine,
        }
    }
}

impl Policy {
    /// Map a verification status to a decision.
    pub fn decide(&self, status: &SupportStatus) -> Decision {
        match status {
            SupportStatus::Supported => self.on_supported,
            SupportStatus::Unsupported => self.on_unsupported,
            SupportStatus::Unresolved => self.on_unresolved,
        }
    }
}

/// The single verification interface used by both Observe and Enforce.
pub trait Verifier {
    /// Stable identifier recorded in every [`VerificationResult`].
    fn name(&self) -> &str;
    /// Judge whether `evidence` semantically supports `claim`.
    /// Must return a result whose `claim_id` equals `claim.id`.
    fn verify(&self, claim: &Claim, evidence: &[EvidenceRef]) -> VerificationResult;
}

/// Placeholder verifier: always `Unresolved`. It never reports support, so it
/// cannot be mistaken for semantic verification. Evidence *association* alone
/// must never yield `Supported`.
#[derive(Debug, Default, Clone, Copy)]
pub struct UnresolvedVerifier;

impl Verifier for UnresolvedVerifier {
    fn name(&self) -> &str {
        "unresolved_stub"
    }
    fn verify(&self, claim: &Claim, evidence: &[EvidenceRef]) -> VerificationResult {
        VerificationResult {
            claim_id: claim.id.clone(),
            evidence_ids: evidence.iter().map(|e| e.id.clone()).collect(),
            status: SupportStatus::Unresolved,
            score: None,
            verifier: self.name().to_string(),
        }
    }
}

/// Record of one gate evaluation. Serializable for JSONL observation logs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateOutcome {
    /// Mode the gate ran in.
    pub mode: GateMode,
    /// Claim the outcome is about.
    pub claim_id: String,
    /// `None` when the gate is disabled and nothing was verified.
    pub status: Option<SupportStatus>,
    /// Verifier that produced the status. `None` when disabled.
    pub verifier: Option<String>,
    /// Evidence the verifier considered.
    pub evidence_ids: Vec<String>,
    /// What the policy says. `None` when disabled.
    pub recommended: Option<Decision>,
    /// What actually happens to the mutation. Always `Allow` unless enforcing.
    pub applied: Decision,
}

impl GateOutcome {
    /// True if, in Observe mode, enforcement would have changed the outcome.
    pub fn would_block(&self) -> bool {
        matches!(self.recommended, Some(d) if d != Decision::Allow)
    }
}

/// Errors from [`Gate::check`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateError {
    /// Verifier returned a result for a different claim.
    ClaimMismatch {
        /// The claim passed to the verifier.
        expected: String,
        /// The claim ID the verifier returned.
        got: String,
    },
}

impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for GateError {}

/// Mode plus policy. Cheap to copy; holds no state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gate {
    /// Disabled, Observe, or Enforce.
    pub mode: GateMode,
    /// Status to decision mapping.
    pub policy: Policy,
}

impl Gate {
    /// Gate with the default [`Policy`].
    pub fn new(mode: GateMode) -> Self {
        Gate {
            mode,
            policy: Policy::default(),
        }
    }

    /// Gate with a custom [`Policy`].
    pub fn with_policy(mode: GateMode, policy: Policy) -> Self {
        Gate { mode, policy }
    }

    /// Callers can skip building evidence at all when this is false.
    pub fn should_verify(&self) -> bool {
        self.mode != GateMode::Disabled
    }

    /// Verify (unless disabled) and decide. Same path for Observe and Enforce.
    pub fn check(
        &self,
        claim: &Claim,
        evidence: &[EvidenceRef],
        verifier: &dyn Verifier,
    ) -> Result<GateOutcome, GateError> {
        if !self.should_verify() {
            return Ok(GateOutcome {
                mode: self.mode,
                claim_id: claim.id.clone(),
                status: None,
                verifier: None,
                evidence_ids: Vec::new(),
                recommended: None,
                applied: Decision::Allow,
            });
        }
        let result = verifier.verify(claim, evidence);
        if result.claim_id != claim.id {
            return Err(GateError::ClaimMismatch {
                expected: claim.id.clone(),
                got: result.claim_id,
            });
        }
        Ok(self.decide(&result))
    }

    /// Decide from an existing verification result (e.g. replaying a log).
    pub fn decide(&self, result: &VerificationResult) -> GateOutcome {
        // Rebuild the status by value so this works whatever derives SupportStatus has.
        let status = match &result.status {
            SupportStatus::Supported => SupportStatus::Supported,
            SupportStatus::Unsupported => SupportStatus::Unsupported,
            SupportStatus::Unresolved => SupportStatus::Unresolved,
        };
        let (recommended, applied) = match self.mode {
            GateMode::Disabled => (None, Decision::Allow),
            GateMode::Observe => (Some(self.policy.decide(&status)), Decision::Allow),
            GateMode::Enforce => {
                let d = self.policy.decide(&status);
                (Some(d), d)
            }
        };
        GateOutcome {
            mode: self.mode,
            claim_id: result.claim_id.clone(),
            status: (self.mode != GateMode::Disabled).then_some(status),
            verifier: (self.mode != GateMode::Disabled).then(|| result.verifier.clone()),
            evidence_ids: result.evidence_ids.clone(),
            recommended,
            applied,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ClaimKind;
    use std::cell::Cell;

    fn claim() -> Claim {
        Claim::new(ClaimKind::Relation, "alice", "works_at", "kwaai")
    }

    /// Returns a fixed status and counts calls.
    struct Fixed {
        status: fn() -> SupportStatus,
        calls: Cell<u32>,
        wrong_claim: bool,
    }

    impl Fixed {
        fn new(status: fn() -> SupportStatus) -> Self {
            Fixed {
                status,
                calls: Cell::new(0),
                wrong_claim: false,
            }
        }
    }

    impl Verifier for Fixed {
        fn name(&self) -> &str {
            "fixed"
        }
        fn verify(&self, claim: &Claim, _e: &[EvidenceRef]) -> VerificationResult {
            self.calls.set(self.calls.get() + 1);
            VerificationResult {
                claim_id: if self.wrong_claim {
                    "other".into()
                } else {
                    claim.id.clone()
                },
                evidence_ids: vec!["ev1".into()],
                status: (self.status)(),
                score: Some(0.9),
                verifier: "fixed".into(),
            }
        }
    }

    fn sup() -> SupportStatus {
        SupportStatus::Supported
    }
    fn uns() -> SupportStatus {
        SupportStatus::Unsupported
    }
    fn unr() -> SupportStatus {
        SupportStatus::Unresolved
    }

    #[test]
    fn disabled_skips_verifier_and_allows() {
        let v = Fixed::new(uns);
        let out = Gate::new(GateMode::Disabled)
            .check(&claim(), &[], &v)
            .unwrap();
        assert_eq!(v.calls.get(), 0);
        assert_eq!(out.applied, Decision::Allow);
        assert_eq!(out.recommended, None);
        assert_eq!(out.status, None);
        assert!(!out.would_block());
    }

    #[test]
    fn observe_records_but_never_blocks() {
        for (s, want) in [
            (sup as fn() -> SupportStatus, Decision::Allow),
            (uns, Decision::Reject),
            (unr, Decision::Quarantine),
        ] {
            let v = Fixed::new(s);
            let out = Gate::new(GateMode::Observe)
                .check(&claim(), &[], &v)
                .unwrap();
            assert_eq!(v.calls.get(), 1);
            assert_eq!(out.recommended, Some(want));
            assert_eq!(out.applied, Decision::Allow);
            assert_eq!(out.would_block(), want != Decision::Allow);
        }
    }

    #[test]
    fn enforce_applies_default_policy() {
        for (s, want) in [
            (sup as fn() -> SupportStatus, Decision::Allow),
            (uns, Decision::Reject),
            (unr, Decision::Quarantine),
        ] {
            let out = Gate::new(GateMode::Enforce)
                .check(&claim(), &[], &Fixed::new(s))
                .unwrap();
            assert_eq!(out.applied, want);
            assert_eq!(out.recommended, Some(want));
        }
    }

    #[test]
    fn observe_and_enforce_agree_on_recommendation() {
        for s in [sup as fn() -> SupportStatus, uns, unr] {
            let o = Gate::new(GateMode::Observe)
                .check(&claim(), &[], &Fixed::new(s))
                .unwrap();
            let e = Gate::new(GateMode::Enforce)
                .check(&claim(), &[], &Fixed::new(s))
                .unwrap();
            assert_eq!(o.recommended, e.recommended);
            assert_eq!(o.status, e.status);
        }
    }

    #[test]
    fn custom_policy() {
        let p = Policy {
            on_unresolved: Decision::Reject,
            ..Policy::default()
        };
        let out = Gate::with_policy(GateMode::Enforce, p)
            .check(&claim(), &[], &Fixed::new(unr))
            .unwrap();
        assert_eq!(out.applied, Decision::Reject);
    }

    #[test]
    fn verifier_claim_mismatch_is_an_error() {
        let mut v = Fixed::new(sup);
        v.wrong_claim = true;
        let err = Gate::new(GateMode::Enforce)
            .check(&claim(), &[], &v)
            .unwrap_err();
        assert!(matches!(err, GateError::ClaimMismatch { .. }));
    }

    #[test]
    fn stub_verifier_never_reports_support() {
        let out = Gate::new(GateMode::Enforce)
            .check(&claim(), &[], &UnresolvedVerifier)
            .unwrap();
        assert_eq!(out.status, Some(SupportStatus::Unresolved));
        assert_eq!(out.applied, Decision::Quarantine);
        assert_eq!(out.verifier.as_deref(), Some("unresolved_stub"));
    }

    #[test]
    fn outcome_serializes_to_jsonl() {
        let out = Gate::new(GateMode::Observe)
            .check(&claim(), &[], &Fixed::new(uns))
            .unwrap();
        let line = serde_json::to_string(&out).unwrap();
        assert!(line.contains("\"mode\":\"observe\""));
        assert!(line.contains("\"recommended\":\"reject\""));
        assert!(line.contains("\"applied\":\"allow\""));
        assert!(!line.contains('\n'));
    }
}
