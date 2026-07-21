# Phase 5: Trained Entity/Relation Type Classifiers

## Context

Phase 4 (`projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md`, implemented) built a graded,
rule + LLM-hybrid confidence classifier for both entity types (`axiom_extract.rs`) and relation types
(`relation_extract.rs`). Validation this week (D6, `llama3.1:8b`) surfaced a structural ceiling on that
approach:

- The only place a *trained model* enters the pipeline today is GLiNER (`gliner.rs` / `scripts/gliner_server.py`),
  and even there the integration throws away the signal that matters: `GliNERClient::person_spans()`
  filters by score internally and returns plain `Vec<String>`, so `axiom_extract.rs`'s Rule 8 collapses
  it to a boolean `gliner_set.contains(name)` and assigns a **hardcoded** confidence (0.6375) regardless
  of how confident GLiNER actually was. No rule in the cascade — axiomatic or LLM — ever produces a real
  per-type confidence *vector*; it's always a single guessed type plus a scalar.
- Rule 9 (the LLM fallback, entities) and the LLM-verify pass (Phase 4 relations, medium tier) inherit
  every weakness of unconstrained 8B-model judgment already documented this week: confidently wrong
  single-word classifications (`Mathematics`, `Streptomycin`, `1945` → Legislation/Publication),
  inconsistent verdicts on clear-cut relations (`"Adam Gool married ... Laura Heffer"` → rejected), and
  — freshly fixed but illustrative — API failures that silently looked like content-based rejections
  until `RelationVerifyOutcome::CallFailed` was split out.
- These fallbacks are also the *expensive* and *slow* part of the pipeline: every full D6 rebuild spends
  most of its ~30-90 min wall-clock on `Focused prompt`/`Full LLM` entity calls and the Phase 4 verify
  batch, round-tripping to remote GPU nodes over a p2p relay that has repeatedly shown transient
  502/503 flakiness this session.

The goal: train two small, local, fast classifiers — one for entity types, one for relation types — that
take a text window around a candidate span/trigger and output a full confidence vector across all
in-scope types (not a single guess). Slot them into the existing axiomatic cascades as **an additional
signal alongside the current rules** (per the scope decision below) — the first real per-candidate
confidence *vector* the pipeline has ever produced, and a path toward reducing LLM-fallback dependency
in a later phase once precision is proven.

**Decisions made with the user going in:**
- **Architecture**: reuse the existing embedding pipeline (`EmbedClient`, nomic-embed-text via Ollama —
  already deployed, zero new serving infra) to encode the text window, then train a small classifier
  head (multinomial logistic regression: one linear layer + softmax) on top. No new always-running
  server process, unlike the GLiNER pattern. The head is small enough (768 × N_types weight matrix) to
  run as a hand-rolled forward pass in Rust — no new ML crate dependency (`ndarray`/`nalgebra`/`linfa`
  are not currently workspace dependencies and aren't needed for a matrix this small).
- **Training data**: harvest from the existing pipeline's own output rather than build a dedicated
  annotation tool — entity labels from seed-YAML ground truth + high-confidence axiomatic commits paired
  with their evidence-chunk text; relation labels from Phase 4's own candidate/verdict trail (extending
  the logging just added for `tracing::info!` into a structured export).
  the same is a structural change: the classifier hint needs the surrounding window, so `classify_one`
  needs a text-window parameter (see §3).
- **Integration point**: land as a new rule/signal in each cascade, **alongside** existing rules —
  not replacing the LLM fallback yet. Safer rollout; sets up a future phase to actually cut LLM calls
  once precision is validated against real production traffic.

## Architecture

### 1. Training data export

**Entity side** — new one-off tool (Python script, run against an existing KB's SQLite files directly,
matching the ad-hoc query pattern used throughout this session's investigations):
`scripts/export_entity_training_data.py --kb D6 > entity_training.jsonl`
- Walk `entities` table; for each entity with `extraction_confidence >= 0.8` (covers seeded
  ground-truth at 1.0 and axiomatically-classified entities via KnownEntity/HonorificPrefix/OrgMarker/etc.
  — the trustworthy, non-LLM-fallback methods), resolve its evidence via `chunk_entity`/`entity_chunk`
  and the paired chunk-store DB.
- For each (entity, chunk) pair, locate the entity name's span in the chunk text (literal match, falling
  back to the surname-drop matching already proven in `relation_extract.rs::classify_relation_candidates`)
  and slice a window — the containing sentence plus one neighbor on each side (mirrors the
  `graph-window`/`COREF_WINDOW` sizing already used elsewhere; don't invent a new window convention).
- Emit `{"window": "...", "entity_type": "Person", "confidence": 1.0, "source_method": "Seeded"}` per line.
- Explicitly **exclude** Rule 9 (LLM fallback / `Full LLM`) entities from positive training labels —
  this session's own findings (Publication/Legislation noise) show that path isn't a trustworthy label
  source. The known-garbage entities identified this session (`Mathematics`, `Streptomycin`, `1945`,
  `Boer War`, etc.) are useful, though: keep them as an explicit hard-negative fixture for manual
  eval of the trained model, not as training labels themselves (their "true" label is ambiguous/garbage,
  not a real type).

**Relation side** — extend the logging added to `rag_cmd.rs::extract_relations_axiomatic()` /
`verify_relation_candidates_llm()` this session. Those `tracing::info!` calls already have every field
needed; add a parallel structured writer (`--export-training-data <path>` flag on `graph build`, or a
dedicated `--relation-training-export` path) that appends one JSON line per candidate at *every* tier:
`{"window": source_sentence, "subject": ..., "object": ..., "guessed_type": ..., "trigger_confidence":
..., "proximity_confidence": ..., "outcome": "AutoCommitted"|"Confirmed"|"Retyped"|"Rejected"|"Dropped"}`
(never `CallFailed` — those aren't real labels, see the `RelationVerifyOutcome::CallFailed` fix).
`AutoCommitted`/`Confirmed`/`Retyped` (using the *retyped* type, not the original guess) → positive;
`Rejected`/`Dropped` → negative.

**Known scope limitation, state it plainly rather than discover it late**: Phase 4's real yield on D6 has
been tiny all session (single-digit to low-double-digit candidates per full 100% rebuild). The relation
training set will start very small. Two mitigations, both deferred to "if v1 needs more data" rather than
blocking v1:
- Accumulate across multiple rebuild runs (the export is additive/appendable, not per-run).
- Reuse the gap-analysis methodology already built this session (`gap_analysis.py`/`gap_analysis2.py`
  pattern: cross-reference `d6_family_tree.yaml`'s seeded relations against corpus text via proximity,
  independent of whether a lexical trigger phrase matched) as a second labeled-positive source — a
  seeded relation whose two entities co-occur near each other in text is a positive example even when
  Phase 4's own trigger scan missed it, which is exactly the "EVIDENCE_NO_TRIGGER" bucket that
  methodology already identifies.

### 2. Offline training (`scripts/train_entity_classifier.py`, `scripts/train_relation_classifier.py`)

- Input: the JSONL exports above. Encode each `window` via the same embedding model already used
  everywhere in this pipeline — call Ollama's `/api/embed` directly (same endpoint `EmbedClient` uses)
  to keep the training-time encoder byte-for-byte identical to inference-time.
- Train a multinomial logistic regression (`sklearn.linear_model.LogisticRegression`, or a small
  hand-rolled numpy softmax-regression if avoiding the sklearn dependency matters) mapping the 768-dim
  embedding to a probability vector over the in-scope type vocabulary (5 entity types; the 15
  `IN_SCOPE_RELATION_TYPES` already defined in `relation_extract.rs` for relations — reuse that constant
  as the label vocabulary, don't redefine it).
- Report train/held-out accuracy and a per-type confusion matrix — with the relation set this small,
  treat the first pass as exploratory/diagnostic, not a go/no-go gate.
- Serialize the trained weights to a small JSON file: `{"labels": [...], "weights": [[...]], "bias": [...]}`.
  Store under a new `tests/kwaai-knowledge/models/` directory (data artifact, not source — same tier as
  the seed YAML/doc-schema files already living in that directory), e.g.
  `entity_type_classifier_v1.json`, `relation_type_classifier_v1.json`.

### 3. Rust inference modules

**New**: `core/crates/kwaai-rag/src/entity_type_classifier.rs` and
`core/crates/kwaai-rag/src/relation_type_classifier.rs`. Same shape for both:
- `TypeClassifier::load(path: &Path) -> Result<Self>` — parses the JSON weight file once at startup
  (mirrors `EmbedClient::probe_dim()`'s startup-validation pattern).
- `async fn classify(&self, embed: &EmbedClient, window: &str) -> Option<Vec<(String, f32)>>` — embeds
  the window (one more `EmbedClient` call — already async, already used per-chunk elsewhere), does the
  hand-rolled dot-product + softmax forward pass, returns the full `(type_name, confidence)` vector
  sorted descending. Soft-fail like `GliNERClient` (embedding-call failure → `None` + `tracing::warn!`,
  never hard-fails the pipeline) — same reasoning: this is an advisory signal, not a hard dependency.

**Entity-side plumbing gap to call out explicitly**: today's axiom rules are context-free —
`classify_one(name, entity_snapshot, gliner_set)` only ever sees the candidate string, never the
surrounding chunk text. This classifier fundamentally needs the window, so this is a real signature
change, not just a new rule:
- `classify_candidates_axiomatic(candidates, entity_snapshot, gliner_hints, chunk_text)` gains a
  `chunk_text: &str` parameter, threaded down into a new Rule (after Rule 8/GLiNER, before Rule 9/Unknown)
  that slices a window around the candidate's position in `chunk_text` (reuse the literal-then-surname-
  drop matching already proven in `relation_extract.rs`, not raw byte-offset slicing — that bug class is
  exactly what caused the sequence.rs panics documented from the 2026-07-16/2026-07-20 rebuilds) and
  calls the new classifier.
- Update the two call sites in `ingestion.rs` (`extract_and_store_entities_pub`, `extract_entity_centric`)
  to pass `text`/`chunk` through — both already have the chunk text in scope at the call site.

**Relation side has no plumbing gap** — `TypedRelationCandidate` already carries `source_sentence`, so
the new signal slots in cleanly between `validate_relation_axioms()` and `split_relations_by_confidence()`
in `relation_extract.rs`: blend the classifier's confidence for the candidate's `relation_type` into
`composite_confidence` (e.g. geometric mean with the existing `trigger_confidence × proximity_confidence`
product — avoid a plain average, which would let one weak factor be masked by a strong one).

### 4. Config / CLI wiring

Mirror the `--gliner-url` pattern but with local file paths instead of a URL (no server to point at):
`--entity-classifier-model <path>` / `--relation-classifier-model <path>` on `GraphAction::Build` and
`RagAction::Rebuild`. `None` (flag omitted) → classifier rule skipped entirely, byte-for-byte unchanged
behavior — same non-breaking-default convention `axiomatic_threshold`/`relation_threshold_high` already
use.

## Files to create/modify

**Create:**
- `scripts/export_entity_training_data.py`, `scripts/train_entity_classifier.py`
- `scripts/train_relation_classifier.py` (shares most of the training-loop code with the entity trainer;
  a shared `scripts/classifier_train_common.py` if the duplication gets annoying, not before)
- `core/crates/kwaai-rag/src/entity_type_classifier.rs`, `relation_type_classifier.rs`
- `tests/kwaai-knowledge/models/` (trained weight JSON files, git-tracked like the seed/schema YAML)

**Modify:**
- `core/crates/kwaai-cli/src/rag_cmd.rs` — structured relation-training-data export (extends the
  logging added this session in `extract_relations_axiomatic()`/`verify_relation_candidates_llm()`);
  new CLI flag wiring.
- `core/crates/kwaai-rag/src/axiom_extract.rs` — `classify_candidates_axiomatic`/`classify_one` gain
  `chunk_text` parameter; new rule calling the entity classifier.
- `core/crates/kwaai-rag/src/ingestion.rs` — thread chunk text through the two call sites; construct
  `EntityTypeClassifier`/`RelationTypeClassifier` from the new CLI paths (same place `GliNERClient` is
  constructed today, `rag_cmd.rs:2993-2996` and its ingestion.rs consumption).
- `core/crates/kwaai-rag/src/relation_extract.rs` — blend classifier confidence into `composite_confidence`.
- `core/crates/kwaai-cli/src/cli.rs` — two new flags.
- `core/crates/kwaai-rag/src/lib.rs` — register the two new modules.

## Verification

1. **Export + train on existing D6 data as the first real dataset** — current graph already has
   seeded ground truth and axiomatically-classified entities; run the export scripts against it before
   writing any Rust inference code, so the label vocabulary/window format gets validated against real
   data early rather than assumed.
2. **Offline metrics**: train/held-out accuracy + confusion matrix per type for both classifiers.
   Explicitly expect the relation classifier's dataset to be small/noisy at v1 (see scope limitation
   above) — report honestly, don't force a target number.
3. **Unit tests** for the Rust inference modules: forward-pass math against a small hand-computed
   weight matrix (verify softmax normalization, not just "it runs"); soft-fail behavior when the
   embedding call fails.
4. **1%/10%/100% staged validation** (same methodology used throughout this session) on D6 with the new
   rule active: confirm axiomatic coverage % (entities resolved without any LLM call) increases, spot-
   check a sample of newly-classifier-committed entities/relations for precision, and confirm the
   `--entity-classifier-model`/`--relation-classifier-model` flags omitted → output byte-for-byte
   identical to current behavior (regression check, same convention as every other optional threshold
   flag in this pipeline).
