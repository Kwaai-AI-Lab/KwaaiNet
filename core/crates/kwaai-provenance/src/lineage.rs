//! Immutable lineage events and an append-only, in-memory lineage log.
//!
//! Design (Option A from the handoff): artifact identity (evidence/claim/entity IDs)
//! is separate from lineage identity. A derivation is recorded as a [`LineageEvent`]
//! whose ID covers *what* was derived from *what*, never the mutation state.
//! State changes (proposed -> committed | failed) are separate append-only log
//! entries, so a proposed-but-never-applied mutation can never be mistaken for
//! history.
//!
//! `EvidenceRef.parent_ids` is NOT consulted here; this log is the authoritative
//! derivation record.
//!
//! Scope: in-memory only. No persistence, snapshots, or durable quarantine yet.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Domain separator for lineage event IDs.
pub const LINEAGE_EVENT_DOMAIN: &str = "kwaai-provenance:lineage-event:v1";

/// What kind of transformation an event records.
///
/// Entity-level merges are kept distinct from claim-level rewrites: merging two
/// graph entities is not evidence that any claim about them is supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageEventKind {
    /// Original evidence -> extracted claim(s).
    Extract,
    /// Evidence -> derived evidence (summary, fact card, synthetic chunk).
    Derive,
    /// Several graph entities -> one entity (dream-cycle merge stage).
    EntityMerge,
    /// Claim -> rewritten claim (completion, sanitization).
    ClaimRewrite,
    /// Artifact(s) removed. Must have no outputs.
    Prune,
}

impl LineageEventKind {
    fn label(self) -> &'static str {
        match self {
            Self::Extract => "extract",
            Self::Derive => "derive",
            Self::EntityMerge => "entity_merge",
            Self::ClaimRewrite => "claim_rewrite",
            Self::Prune => "prune",
        }
    }
}

/// Lifecycle state of a mutation. Graph writes are not assumed atomic, so a
/// proposal only enters history once explicitly committed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationState {
    /// Recorded, not yet applied. Excluded from ancestry.
    Proposed,
    /// Applied to the graph. Part of history.
    Committed,
    /// Not applied. Never enters history.
    Failed,
}

/// An immutable derivation record. The `id` is a content hash over every other
/// field; inputs and outputs are canonicalized (sorted, de-duplicated) first so
/// that ordering does not change identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageEvent {
    /// Content hash over every other field (see [`LineageEvent::expected_id`]).
    pub id: String,
    /// Transformation type.
    pub kind: LineageEventKind,
    /// Artifact IDs consumed (evidence, claim, or entity IDs).
    pub inputs: Vec<String>,
    /// Artifact IDs produced. Empty only for `Prune`.
    pub outputs: Vec<String>,
    /// Component that produced the event, e.g. `"dream.merge"`.
    pub actor: String,
    /// Dream/consolidation cycle number, if applicable.
    pub cycle: Option<u64>,
    /// Caller-supplied sequence number; distinguishes otherwise identical events
    /// (e.g. the same prune repeated in a later run).
    pub seq: u64,
}

impl LineageEvent {
    /// Build an event, canonicalize it, compute its ID, and validate its shape.
    pub fn new<I, O>(
        kind: LineageEventKind,
        inputs: I,
        outputs: O,
        actor: &str,
        cycle: Option<u64>,
        seq: u64,
    ) -> Result<Self, LineageError>
    where
        I: IntoIterator,
        I::Item: Into<String>,
        O: IntoIterator,
        O::Item: Into<String>,
    {
        let inputs = canonical(inputs);
        let outputs = canonical(outputs);
        let id = compute_event_id(kind, &inputs, &outputs, actor, cycle, seq);
        let ev = LineageEvent {
            id,
            kind,
            inputs,
            outputs,
            actor: actor.to_string(),
            cycle,
            seq,
        };
        ev.validate()?;
        Ok(ev)
    }

    /// Recompute the ID from contents. Use this after deserializing a record.
    pub fn expected_id(&self) -> String {
        compute_event_id(
            self.kind,
            &self.inputs,
            &self.outputs,
            &self.actor,
            self.cycle,
            self.seq,
        )
    }

    /// Structural validation, including ID integrity. Call on deserialized events.
    pub fn validate(&self) -> Result<(), LineageError> {
        if self.inputs.is_empty() {
            return Err(LineageError::NoInputs);
        }
        if self
            .inputs
            .iter()
            .chain(&self.outputs)
            .any(|a| a.is_empty())
        {
            return Err(LineageError::EmptyArtifactId);
        }
        if self.inputs != canonical(self.inputs.clone())
            || self.outputs != canonical(self.outputs.clone())
        {
            return Err(LineageError::NotCanonical);
        }
        match (self.kind, self.outputs.is_empty()) {
            (LineageEventKind::Prune, false) => return Err(LineageError::PruneWithOutputs),
            (k, true) if k != LineageEventKind::Prune => return Err(LineageError::NoOutputs),
            _ => {}
        }
        if let Some(a) = self
            .outputs
            .iter()
            .find(|o| self.inputs.binary_search(o).is_ok())
        {
            return Err(LineageError::SelfDerivation(a.clone()));
        }
        if self.id != self.expected_id() {
            return Err(LineageError::IdMismatch);
        }
        Ok(())
    }
}

/// One append-only log record. State transitions reference events by ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "entry", rename_all = "snake_case")]
pub enum LogEntry {
    /// A mutation was proposed.
    Proposed {
        /// The proposed event.
        event: LineageEvent,
    },
    /// A proposed mutation was applied.
    Committed {
        /// ID of the committed event.
        event_id: String,
    },
    /// A proposed mutation was not applied.
    Failed {
        /// ID of the failed event.
        event_id: String,
        /// Caller-supplied reason.
        reason: String,
    },
}

/// Errors from building, validating, or transitioning lineage events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineageError {
    /// Event has no inputs.
    NoInputs,
    /// Non-prune event has no outputs.
    NoOutputs,
    /// Prune event lists outputs.
    PruneWithOutputs,
    /// An input or output ID is the empty string.
    EmptyArtifactId,
    /// Inputs or outputs are not sorted and de-duplicated.
    NotCanonical,
    /// An artifact is both input and output of one event.
    SelfDerivation(String),
    /// Stored `id` does not match the recomputed content hash.
    IdMismatch,
    /// An event with this ID was already proposed.
    DuplicateEvent(String),
    /// No event with this ID has been proposed.
    UnknownEvent(String),
    /// Transition attempted from a state other than `Proposed`.
    InvalidTransition {
        /// Event whose transition was refused.
        event_id: String,
        /// State the event was in.
        from: MutationState,
    },
    /// Committing would make an output its own ancestor.
    CycleDetected {
        /// Event whose commit was refused.
        event_id: String,
        /// Output that is already an ancestor of an input.
        artifact: String,
    },
}

impl fmt::Display for LineageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LineageError {}

/// Append-only, in-memory lineage log. Only committed events form history.
#[derive(Debug, Default)]
pub struct LineageLog {
    entries: Vec<LogEntry>,
    events: BTreeMap<String, LineageEvent>,
    states: BTreeMap<String, MutationState>,
    /// artifact -> committed events that produced it
    producers: BTreeMap<String, BTreeSet<String>>,
    /// artifacts consumed by a committed prune
    pruned: BTreeSet<String>,
}

impl LineageLog {
    /// Empty log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a proposed mutation. Does not affect ancestry.
    pub fn propose(&mut self, event: LineageEvent) -> Result<String, LineageError> {
        event.validate()?;
        if self.events.contains_key(&event.id) {
            return Err(LineageError::DuplicateEvent(event.id));
        }
        let id = event.id.clone();
        self.states.insert(id.clone(), MutationState::Proposed);
        self.events.insert(id.clone(), event.clone());
        self.entries.push(LogEntry::Proposed { event });
        Ok(id)
    }

    /// Mark a proposed mutation as applied. Rejects commits that would create a cycle.
    pub fn commit(&mut self, event_id: &str) -> Result<(), LineageError> {
        self.require_proposed(event_id)?;
        let event = self.events[event_id].clone();
        for out in &event.outputs {
            // Cycle iff `out` already appears among the ancestors of any input (or is an input's ancestor chain root).
            for input in &event.inputs {
                if self.ancestors(input).contains(out) {
                    return Err(LineageError::CycleDetected {
                        event_id: event_id.to_string(),
                        artifact: out.clone(),
                    });
                }
            }
        }
        for out in &event.outputs {
            self.producers
                .entry(out.clone())
                .or_default()
                .insert(event.id.clone());
        }
        if event.kind == LineageEventKind::Prune {
            self.pruned.extend(event.inputs.iter().cloned());
        }
        self.states
            .insert(event.id.clone(), MutationState::Committed);
        self.entries
            .push(LogEntry::Committed { event_id: event.id });
        Ok(())
    }

    /// Mark a proposed mutation as not applied. It never enters history.
    pub fn fail(&mut self, event_id: &str, reason: &str) -> Result<(), LineageError> {
        self.require_proposed(event_id)?;
        self.states
            .insert(event_id.to_string(), MutationState::Failed);
        self.entries.push(LogEntry::Failed {
            event_id: event_id.to_string(),
            reason: reason.to_string(),
        });
        Ok(())
    }

    fn require_proposed(&self, event_id: &str) -> Result<(), LineageError> {
        match self.states.get(event_id) {
            None => Err(LineageError::UnknownEvent(event_id.to_string())),
            Some(MutationState::Proposed) => Ok(()),
            Some(&from) => Err(LineageError::InvalidTransition {
                event_id: event_id.to_string(),
                from,
            }),
        }
    }

    /// Current lifecycle state of an event, if known.
    pub fn state(&self, event_id: &str) -> Option<MutationState> {
        self.states.get(event_id).copied()
    }

    /// Look up an event by ID, whatever its state.
    pub fn event(&self, event_id: &str) -> Option<&LineageEvent> {
        self.events.get(event_id)
    }

    /// The append-only record, in order. Suitable for JSONL export.
    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    /// Every artifact this one was (transitively) derived from via committed events.
    pub fn ancestors(&self, artifact: &str) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![artifact.to_string()];
        while let Some(a) = stack.pop() {
            if let Some(evs) = self.producers.get(&a) {
                for ev in evs {
                    for input in &self.events[ev].inputs {
                        if seen.insert(input.clone()) {
                            stack.push(input.clone());
                        }
                    }
                }
            }
        }
        seen
    }

    /// Ancestors with no committed producer, i.e. where recorded lineage stops.
    /// An artifact with no lineage at all is its own root.
    pub fn roots(&self, artifact: &str) -> BTreeSet<String> {
        let anc = self.ancestors(artifact);
        if anc.is_empty() {
            return BTreeSet::from([artifact.to_string()]);
        }
        anc.into_iter()
            .filter(|a| !self.producers.contains_key(a))
            .collect()
    }

    /// True if lineage reaches at least one artifact the caller identifies as
    /// original source evidence. This is *traceability*, not semantic support.
    pub fn traces_to(&self, artifact: &str, is_original: impl Fn(&str) -> bool) -> bool {
        is_original(artifact) || self.roots(artifact).iter().any(|r| is_original(r))
    }

    /// True if a committed prune consumed this artifact. Its lineage is retained.
    pub fn is_pruned(&self, artifact: &str) -> bool {
        self.pruned.contains(artifact)
    }

    /// Fraction of `artifacts` that trace to original evidence. `None` if empty.
    /// Intended for the "provenance retained after N cycles" metric.
    pub fn traceability_rate<'a>(
        &self,
        artifacts: impl IntoIterator<Item = &'a str>,
        is_original: impl Fn(&str) -> bool,
    ) -> Option<f64> {
        let (mut n, mut ok) = (0usize, 0usize);
        for a in artifacts {
            n += 1;
            if self.traces_to(a, &is_original) {
                ok += 1;
            }
        }
        (n > 0).then(|| ok as f64 / n as f64)
    }
}

fn canonical<T, S>(items: T) -> Vec<String>
where
    T: IntoIterator<Item = S>,
    S: Into<String>,
{
    let set: BTreeSet<String> = items.into_iter().map(Into::into).collect();
    set.into_iter().collect()
}

/// Length-prefixed SHA-256, kept local so this module does not depend on the
/// exact signature of `identity::hash_parts`. Inputs/outputs are framed with
/// their counts so `[a,b] -> [c]` and `[a] -> [b,c]` cannot collide.
fn compute_event_id(
    kind: LineageEventKind,
    inputs: &[String],
    outputs: &[String],
    actor: &str,
    cycle: Option<u64>,
    seq: u64,
) -> String {
    let mut h = Sha256::new();
    let mut put = |b: &[u8]| {
        h.update((b.len() as u64).to_be_bytes());
        h.update(b);
    };
    put(LINEAGE_EVENT_DOMAIN.as_bytes());
    put(kind.label().as_bytes());
    put(&(inputs.len() as u64).to_be_bytes());
    for i in inputs {
        put(i.as_bytes());
    }
    put(&(outputs.len() as u64).to_be_bytes());
    for o in outputs {
        put(o.as_bytes());
    }
    put(actor.as_bytes());
    match cycle {
        Some(c) => put(format!("cycle:{c}").as_bytes()),
        None => put(b"cycle:none"),
    }
    put(&seq.to_be_bytes());
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use LineageEventKind::*;

    fn ev(kind: LineageEventKind, i: &[&str], o: &[&str], seq: u64) -> LineageEvent {
        LineageEvent::new(
            kind,
            i.iter().copied(),
            o.iter().copied(),
            "test",
            Some(1),
            seq,
        )
        .unwrap()
    }

    fn committed(log: &mut LineageLog, e: LineageEvent) {
        let id = log.propose(e).unwrap();
        log.commit(&id).unwrap();
    }

    #[test]
    fn event_id_is_order_independent_and_deterministic() {
        let a = ev(EntityMerge, &["e1", "e2"], &["e3"], 0);
        let b = ev(EntityMerge, &["e2", "e1", "e1"], &["e3"], 0);
        assert_eq!(a.id, b.id);
        assert_eq!(a.id.len(), 64);
    }

    #[test]
    fn event_id_separates_kind_actor_cycle_seq_and_framing() {
        let base = ev(Derive, &["a", "b"], &["c"], 0);
        assert_ne!(base.id, ev(ClaimRewrite, &["a", "b"], &["c"], 0).id);
        assert_ne!(base.id, ev(Derive, &["a", "b"], &["c"], 1).id);
        assert_ne!(
            base.id,
            LineageEvent::new(Derive, ["a", "b"], ["c"], "other", Some(1), 0)
                .unwrap()
                .id
        );
        assert_ne!(
            base.id,
            LineageEvent::new(Derive, ["a", "b"], ["c"], "test", None, 0)
                .unwrap()
                .id
        );
        assert_ne!(base.id, ev(Derive, &["a"], &["b", "c"], 0).id);
    }

    #[test]
    fn shape_validation() {
        assert_eq!(
            LineageEvent::new(Derive, Vec::<&str>::new(), ["x"], "t", None, 0),
            Err(LineageError::NoInputs)
        );
        assert_eq!(
            LineageEvent::new(Derive, ["a"], Vec::<&str>::new(), "t", None, 0),
            Err(LineageError::NoOutputs)
        );
        assert_eq!(
            LineageEvent::new(Prune, ["a"], ["b"], "t", None, 0),
            Err(LineageError::PruneWithOutputs)
        );
        assert_eq!(
            LineageEvent::new(Derive, ["a"], ["a"], "t", None, 0),
            Err(LineageError::SelfDerivation("a".into()))
        );
        assert_eq!(
            LineageEvent::new(Derive, [""], ["a"], "t", None, 0),
            Err(LineageError::EmptyArtifactId)
        );
        assert!(LineageEvent::new(Prune, ["a"], Vec::<&str>::new(), "t", None, 0).is_ok());
    }

    #[test]
    fn tampered_or_noncanonical_record_fails_validation() {
        let mut e = ev(Derive, &["a"], &["b"], 0);
        e.outputs = vec!["z".into()];
        assert_eq!(e.validate(), Err(LineageError::IdMismatch));
        let mut e = ev(Derive, &["a", "b"], &["c"], 0);
        e.inputs = vec!["b".into(), "a".into()];
        assert_eq!(e.validate(), Err(LineageError::NotCanonical));
    }

    #[test]
    fn proposed_and_failed_events_do_not_enter_history() {
        let mut log = LineageLog::new();
        let p = log.propose(ev(Extract, &["src"], &["claim"], 0)).unwrap();
        assert!(log.ancestors("claim").is_empty());
        log.fail(&p, "graph write failed").unwrap();
        assert!(log.ancestors("claim").is_empty());
        assert_eq!(log.state(&p), Some(MutationState::Failed));
    }

    #[test]
    fn state_transitions_are_one_way() {
        let mut log = LineageLog::new();
        let id = log.propose(ev(Extract, &["src"], &["c"], 0)).unwrap();
        log.commit(&id).unwrap();
        assert!(matches!(
            log.commit(&id),
            Err(LineageError::InvalidTransition {
                from: MutationState::Committed,
                ..
            })
        ));
        assert!(matches!(
            log.fail(&id, "x"),
            Err(LineageError::InvalidTransition { .. })
        ));
        assert_eq!(
            log.commit("nope"),
            Err(LineageError::UnknownEvent("nope".into()))
        );
        assert!(matches!(
            log.propose(ev(Extract, &["src"], &["c"], 0)),
            Err(LineageError::DuplicateEvent(_))
        ));
    }

    #[test]
    fn ancestry_through_merge_and_rewrite_reaches_original() {
        let mut log = LineageLog::new();
        committed(&mut log, ev(Extract, &["orig:1"], &["claim:a"], 0));
        committed(&mut log, ev(Extract, &["orig:2"], &["claim:b"], 1));
        committed(
            &mut log,
            ev(EntityMerge, &["claim:a", "claim:b"], &["claim:m"], 2),
        );
        committed(&mut log, ev(ClaimRewrite, &["claim:m"], &["claim:r"], 3));
        let anc = log.ancestors("claim:r");
        assert!(anc.contains("orig:1") && anc.contains("orig:2") && anc.contains("claim:m"));
        assert_eq!(
            log.roots("claim:r"),
            BTreeSet::from(["orig:1".to_string(), "orig:2".to_string()])
        );
        assert!(log.traces_to("claim:r", |a| a.starts_with("orig:")));
    }

    #[test]
    fn multiple_derivation_paths_to_same_artifact() {
        let mut log = LineageLog::new();
        committed(&mut log, ev(Derive, &["orig:1"], &["sum"], 0));
        committed(&mut log, ev(Derive, &["orig:2"], &["sum"], 1));
        assert_eq!(log.roots("sum").len(), 2);
    }

    #[test]
    fn lineage_that_stops_at_synthetic_input_is_not_traceable() {
        // Mirrors a dream-generated relation whose only "evidence" is synthetic (e.g. fallback id 0).
        let mut log = LineageLog::new();
        committed(
            &mut log,
            ev(ClaimRewrite, &["synthetic:0"], &["claim:x"], 0),
        );
        assert!(!log.traces_to("claim:x", |a| a.starts_with("orig:")));
        assert!(!log.traces_to("claim:never-seen", |a| a.starts_with("orig:")));
        assert!(log.traces_to("orig:9", |a| a.starts_with("orig:")));
    }

    #[test]
    fn commit_rejects_cycles() {
        let mut log = LineageLog::new();
        committed(&mut log, ev(Derive, &["a"], &["b"], 0));
        committed(&mut log, ev(Derive, &["b"], &["c"], 1));
        let back = log.propose(ev(Derive, &["c"], &["a"], 2)).unwrap();
        assert!(matches!(
            log.commit(&back),
            Err(LineageError::CycleDetected { .. })
        ));
        assert_eq!(log.state(&back), Some(MutationState::Proposed));
        assert!(!log.ancestors("a").contains("c"));
    }

    #[test]
    fn prune_is_recorded_without_erasing_history() {
        let mut log = LineageLog::new();
        committed(&mut log, ev(Extract, &["orig:1"], &["claim:a"], 0));
        committed(
            &mut log,
            LineageEvent::new(
                Prune,
                ["claim:a"],
                Vec::<&str>::new(),
                "dream.prune",
                Some(2),
                1,
            )
            .unwrap(),
        );
        assert!(log.is_pruned("claim:a"));
        assert!(log.traces_to("claim:a", |a| a.starts_with("orig:")));
    }

    #[test]
    fn traceability_rate() {
        let mut log = LineageLog::new();
        committed(&mut log, ev(Extract, &["orig:1"], &["c1"], 0));
        committed(&mut log, ev(ClaimRewrite, &["synthetic:0"], &["c2"], 1));
        let r = log.traceability_rate(["c1", "c2"], |a| a.starts_with("orig:"));
        assert_eq!(r, Some(0.5));
        assert_eq!(log.traceability_rate(Vec::<&str>::new(), |_| true), None);
    }

    #[test]
    fn log_entries_round_trip_as_jsonl() {
        let mut log = LineageLog::new();
        let id = log.propose(ev(Extract, &["orig:1"], &["c"], 0)).unwrap();
        log.commit(&id).unwrap();
        let jsonl: Vec<String> = log
            .entries()
            .iter()
            .map(|e| serde_json::to_string(e).unwrap())
            .collect();
        assert!(jsonl[0].contains("\"entry\":\"proposed\""));
        assert!(jsonl[1].contains("\"entry\":\"committed\""));
        let back: Vec<LogEntry> = jsonl
            .iter()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(back, log.entries());
        if let LogEntry::Proposed { event } = &back[0] {
            event.validate().unwrap();
        }
    }
}
