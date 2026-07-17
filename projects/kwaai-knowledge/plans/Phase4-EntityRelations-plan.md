# Phase 4: Entity Relations — Confidence-Gated Lexical Relation Extraction

## Context

D6's knowledge graph currently extracts relations only through a coarse, all-or-nothing path: `lexical_relation_trigger()` (`graph.rs`) checks whether a chunk contains *any* of ~30 trigger phrases (family/agent/spatial), and if so hands the *entire chunk* to an open-ended LLM relation-extraction prompt with no per-candidate confidence signal. The project's own docs (`projects/kwaai-knowledge/CLAUDE.md`) already call this "Phase 4" and note 8B models get 0–17% precision on unconstrained relation extraction — which is why `--no-relations` is the production default and D6's graph currently has entities but very few relations (209 out of 653 entities after the 2026-07-16 rebuild).

The goal now is to replace that boolean gate with a graded, three-way confidence classifier — mirroring the axiomatic *entity* classifier (`axiom_extract.rs`) that already gets ~46% of entities classified without any LLM call at 0.80 threshold. High-confidence lexical relation matches get written directly to the graph; medium-confidence matches go to a narrow, cheap LLM confirm/reject/retype call (not full open extraction); low-confidence matches are dropped entirely rather than wasting an LLM call on noise. This directly extends work the codebase has already invested in — a working prototype of the exact "windowed trigger → nearest entities → triplet" algorithm already exists (`extract_kinship_interactions()` in `sequence.rs`, currently only feeding the Mermaid timeline view), and a working two-pass LLM verification pipeline already exists (`cmd_extract_relations`/`build_cc_prompt`/`build_ec_prompt` in `rag_cmd.rs`, currently a manual, family-only, `--commit`-gated review command). The job is to generalize and wire these together into an automatic pipeline, not invent new architecture from scratch.

Decisions made with the user going in:
- **Replace** the legacy boolean `lexical_relation_trigger()` gate as the automated path once the new threshold flags are set (>0); **keep** `graph extract-relations` as a separate manual review/audit command (generalized to the fuller type list), not delete it.
- **First-cut scope**: 12 relation types proven relevant by `d6_family_tree.yaml` (`spouse_of`, `parent_of`/`child_of`, `sibling_of`, `half_sibling_of`, `grandparent_of`/`grandchild_of`, `foster_parent_of`/`foster_child_of`, `member_of`, `founded`, `affiliated_with`, `associated_with`, `lived_in`) plus `works_at`. **Defer** avuncular relations (`uncle_of`/`aunt_of`/`nephew_of`/`niece_of`/`cousin_of`) — the existing manual command already tried and explicitly backed away from these for precision reasons.
- **`RelationRecord` gets 3 new fields**: `confidence`, `method`, and `source` (separate "how classified" from "which pipeline produced it").

## Architecture

### 1. Data model (`core/crates/kwaai-rag/src/graph.rs`)

Add to `RelationRecord` (line 236), following the exact backward-compatible pattern already used for `EntityNode.extraction_confidence` (`#[serde(default = "default_extraction_confidence")]`, `graph.rs` ~231):
- `confidence: f32` — `#[serde(default = "default_relation_confidence")]` → `1.0`. Preserves current behavior for every existing serialized relation (seeded/LLM-open rows are implicitly fully trusted today).
- `method: String` — `#[serde(default = "default_relation_method")]` → `"LlmOpenExtraction"`. How the candidate was classified: `"Trigger:<relation_type>"` (lexical commit tier), `"LlmVerified"` (medium tier confirmed), `"LlmOpenExtraction"` (legacy/unchanged), `"Seeded"`.
- `source: String` — `#[serde(default = "default_relation_source")]` → `"llm_open"`. Which pipeline produced it: `"axiomatic_high"`, `"axiomatic_llm_verify"`, `"llm_open"`, `"seeded"`, `"manual"` (from `cmd_extract_relations --commit`).

Update the 3 existing `RelationRecord {...}` construction sites (upsert-unchecked ~987, alias-relocation merge ~1693, entity-rename rekey ~4385) to carry these through; on repeat-evidence merges, keep `confidence = old.max(new)` and the corresponding `method`/`source`.

Add `upsert_relation_with_confidence(&mut self, src_id, dst_id, relation_type: &str, evidence_chunk_id: i64, confidence: f32, method: &str, source: &str) -> Result<()>` alongside the existing `upsert_relation()` (line 903) — same Person-only/inverse/symmetric logic, just threading the 3 new fields through. Make `upsert_relation()` a thin wrapper calling it with `(1.0, "LlmOpenExtraction", "llm_open")` so none of its ~5 existing call sites need to change.

### 2. New library module: `core/crates/kwaai-rag/src/relation_extract.rs`

Register with `mod relation_extract;` in `lib.rs` next to the existing `mod axiom_extract;`. Mirrors `axiom_extract.rs`'s shape:

- **`RelationClassificationMethod` enum**: `FamilyTrigger`, `AgentTrigger`, `SpatialTrigger`, `LlmVerified`, `Unknown` — grouped by trigger category (not per-word), matching how the entity classifier groups dozens of honorifics under one `HonorificPrefix` variant. `.as_str()` method for metrics.
- **`TypedRelationCandidate` struct**: `subject_id/object_id: i64`, `subject_name/object_name: String`, `relation_type: String`, `trigger_confidence: f32`, `proximity_confidence: f32`, `composite_confidence: f32` (= product of the two, same multiplicative shape as the entity classifier), `method: RelationClassificationMethod`, `reversed: bool`, `source_sentence: String`, `chunk_id: i64`.
- **Consolidated trigger tables**: merge the currently-divergent `graph.rs::lexical_relation_trigger()` FAMILY/AGENT/SPATIAL lists, `sequence.rs::KINSHIP_KEYWORDS` (phrase→relation_type pairs — reuse this exact shape), and `rag_cmd.rs::FAMILY_TRIGGERS` (keep its trailing-space tricks like `" uncle "` to avoid matching "Auntie" in index entries) into one flat `RELATION_TRIGGERS: &[(&str, &str)]` = `(trigger_phrase, relation_type)`, covering the 13 in-scope types. Add `REVERSED_TRIGGERS: &[&str]` generalizing `kinship_is_reversed()` (e.g. `"founded by"`, `"built by"`, `"born to"`).
- **`classify_relation_candidates(chunk_id, text, known_entities: &[(i64, String, Vec<String>)]) -> Vec<TypedRelationCandidate>`**: generalizes `extract_kinship_interactions()` (`sequence.rs:777`) — same sentence-splitting + nearest-entity-before/after-keyword logic — but (a) scans the merged trigger table (not kinship-only), matching **longest phrase first** to prevent substring clobber (same rule `scan_chunk_for_dates()` already applies for dates), (b) computes an explicit ambiguity signal (how many *other* candidate entities sit within a small tie margin on each side) instead of just picking the closest, (c) assigns `trigger_confidence` per category (FAMILY-tier phrases ≈0.90, AGENT/SPATIAL ≈0.70 — matches the graph.rs comment that AGENT/SPATIAL triggers alone are noisier) and `proximity_confidence` (0.95 unambiguous, 0.70 some ambiguity resolved). **Any window/offset arithmetic must use the char-boundary-safe snapping pattern already proven in `extract_rc_windows()` (`rag_cmd.rs:6501`, verified directly) — never raw byte-offset slicing like `scan_chunk_for_dates()`'s `saturating_sub`/direct-index approach, which is the exact bug class that caused two non-fatal panics during the 2026-07-16 D6 rebuild.**
- **`validate_relation_axioms(candidates, snapshot: &RelationAxiomSnapshot) -> Vec<TypedRelationCandidate>`**: demotion-based (composite→0.0), mirroring `validate_with_axioms()`. Axioms: (1) familial type requires both endpoints Person per the graph snapshot (pre-check of what `upsert_relation` enforces anyway, avoids wasted LLM-verify calls); (2) ambiguous-window rejection (>1 equally-close candidate on either side); (3) contradiction with an existing trusted relation on the same entity pair (reuse the R1 dedup guard's role-contradiction predicate — factor it into a shared `pub(crate)` function called by both the dedup guard and this axiom, rather than duplicating the table).
- **`split_relations_by_confidence(candidates, threshold_high, threshold_low) -> (commit, verify, drop)`**: three-way partition (the deliberate deviation from the entity classifier's two-way split — relation candidate pairs scale combinatorially with entities-per-window, so a hard low-end drop is a needed cost/precision safeguard entities didn't need). If `threshold_low > threshold_high`, collapse to `threshold_low = threshold_high` (empty verify band) rather than error.
- **`schema_org_property_for(relation_type: &str) -> Option<&'static str>`**: documentation-only mapping mirroring `scorer.rs::schema_type_for()`, never consumed by storage/dedup logic. Document known asymmetries in comments (e.g. `founded`'s direction is inverted relative to schema.org's `founder` property) rather than "fixing" them.
- **`RelationAxiomaticMetricsAccum`/`RelationAxiomaticRunMetrics`/`print_summary()`**: mirrors `AxiomaticMetricsAccum` (axiom_extract.rs). Fields: `threshold_high/low`, `total_chunks`, `candidates_generated/committed_high/sent_llm_medium/dropped_low/demoted_by_axiom`, `llm_confirmed/rejected/retyped`, latency vectors, `method_breakdown`. Derived: `llm_confirm_rate` (the key new precision signal — how often a medium-confidence lexical guess survives LLM scrutiny), `commit_pct`, `drop_pct`. JSON dump to `{temp_dir}/kwaai_relation_axiomatic_metrics_<timestamp>.json`, same convention as the entity metrics file.

### 3. LLM verification for "medium" candidates (`core/crates/kwaai-cli/src/rag_cmd.rs`)

Generalize rather than rewrite the existing CC/EC machinery:
- Skip the CC pass entirely for Phase 4 candidates (the lexical classifier already found and scored the qualifying sentence — CC's whole job is redundant here).
- New `build_relation_verify_prompt(candidates: &[TypedRelationCandidate]) -> String` — batches multiple candidates per call (unlike the current one-quote-per-call EC path), each listed as `{subject_name} [{guessed_relation_type}] {object_name} — from: "{source_sentence}"`, asking for `confirm` / `reject` / `retype:<type>` per candidate (retype restricted to the same closed 13-type vocabulary, never free-form).
- Reuse `call_llm_for_relations()` (`rag_cmd.rs:6727`) verbatim for the HTTP call (it already has the working 120s hard-timeout wrapper for p2p-proxy hangs).
- New `parse_relation_verify_response()` mirrors `parse_relation_response()`'s JSON-extraction pattern.
- Confirmed/retyped → `upsert_relation_with_confidence(..., confidence: 0.75, method: "LlmVerified", source: "axiomatic_llm_verify")` (0.75 is a starting constant, tune in Phase B of the sweep below). Rejected → discarded, counted in `llm_rejected` (feeds `llm_confirm_rate`).
- Generalize `build_ec_prompt()`'s hardcoded 5-type whitelist to accept the fuller 13-type list, for use by both the new automatic pipeline and the existing manual `graph extract-relations` command.

### 4. Pipeline wiring (`core/crates/kwaai-cli/src/rag_cmd.rs`, `core/crates/kwaai-rag/src/ingestion.rs`)

Relations need both endpoint entity IDs fully resolved, which is only guaranteed once the *entire* corpus's entity extraction has finished (the entity `entity_snapshot` is built once before the run and never updated mid-run — the same reason `extract_kinship_interactions()` already runs from `run_timeline_build()` *after* `extract_and_store_entities_pub()` returns, not inline in the per-chunk loop).

New orchestration function `extract_relations_axiomatic()` in `rag_cmd.rs`, inserted in `GraphAction::Build`'s handler right after `extract_and_store_entities_pub()` completes and **before** the existing `if timeline { run_timeline_build(...) }` block:
- Per chunk: build `known_entities` via `graph.get_chunk_entities(chunk_id)` resolved through `graph.get_entity(id)` — reuse the exact pattern already proven at `run_timeline_build` (rag_cmd.rs ~8709-8717).
- Call `classify_relation_candidates()` → `validate_relation_axioms()` → `split_relations_by_confidence()`.
- Commit the `commit` tier inline via `upsert_relation_with_confidence(..., source: "axiomatic_high")`.
- Batch the `verify` tier across chunks into `verify_relation_candidates_llm()`, using the same `tokio::sync::Semaphore`+`workers` pattern already used in `extract_and_store_entities_pub`'s spawn loop.
- Print metrics summary; dump JSON.

Gate: only runs when `relation_threshold_high > 0.0` (new `GraphIngestConfig` field). When active, force `effective_no_relations = true` unconditionally for every `extract_from_text()` call in `ingestion.rs` (~line 658), overriding whatever `lexical_relation_trigger()` would have returned — this fully replaces the legacy chunk-level boolean gate for the automated path. When `relation_threshold_high == 0.0` (default), behavior is byte-for-byte unchanged — same non-breaking-default convention `axiomatic_threshold == 0.0` already uses for entities.

### 5. Dedup-guard trust (`core/crates/kwaai-rag/src/graph.rs`, ~3180-3370)

`trusted_family_rel_map()`/`trusted_parent_ids()` currently gate purely on `strength >= 0.1` (calibrated against an assumption that family relations are primarily seeded/LLM-open — both far more precise than a lexical-only match). Once `confidence`/`method`/`source` exist, add a second filter: a relation is "trusted" for R1/R2 dedup contradiction-guard purposes only if `source != "axiomatic_high"` OR `confidence >= 0.85` (a stricter, hardcoded floor independent of whatever `--relation-threshold-high` the user configured — a permissive commit threshold for writing relations shouldn't automatically also lower the bar for blocking merges). `trusted_parent_ids()` already deserializes full `RelationRecord`s per row — trivial to add the check there. `trusted_family_rel_map()` reads from the in-memory `adj` (which doesn't carry the new fields) — add a point lookup against the `relations` table only for entries that already passed `strength >= 0.1` (small set, not a hot path); don't change `adj`'s tuple shape, which is read/written throughout `graph.rs`.

### 6. CLI flags (`core/crates/kwaai-cli/src/cli.rs`)

Add to both `RagAction::Rebuild` (~1365) and `GraphAction::Build` (~1815), alongside `axiomatic_threshold`:
```
--relation-threshold-high <FLOAT>   # default 0.0 (disabled). Commit directly above this.
--relation-threshold-low <FLOAT>    # default 0.0. Drop entirely below this (only when high > 0).
```
Thread through `rag_cmd.rs` exactly like `axiomatic_threshold` (destructure → `GraphIngestConfig` field → all 3 construction sites, defaulting untouched sites to `0.0`).

### 7. Test/rollout methodology

Extend `tests/kwaai-knowledge/axiomatic_sweep.sh` with a parallel Phase A/B/C for relations (reuse its `log()`/`copy_metrics()`/`print_metrics()` helpers, just retarget the glob to `relation_axiomatic_metrics_*`):
- **Phase A** (`--sample-pct 1`, `--reset-graph`, entities extracted first at the already-chosen entity threshold): sweep a small grid of `(threshold_high, threshold_low)` pairs, e.g. `(0.85,0.60)`, `(0.75,0.45)`, `(0.65,0.30)`. `pick_best_relation_thresholds()` selects the pair maximizing `commit_pct` subject to `llm_confirm_rate >= 0.7` (a provisional precision floor — revisit once real Phase A data exists, unlike the entity classifier's already-tuned constants).
- **Phase B** (`--sample-pct 10`): run the chosen pair, seed with `d6_family_tree.yaml`, then compute a rough precision/recall estimate by diffing newly-extracted (non-seeded, `evidence_chunk_ids` not containing 0) relations against the YAML's 12-type ground truth (`true_positive`/`false_positive`/`false_negative` by triple match, either direction). Document clearly that this is a rough estimate, not rigorous eval — the YAML itself isn't exhaustive ground truth for the whole corpus.
- **Phase C** (full rebuild, 100%): `rag rebuild ... --relation-threshold-high <T> --relation-threshold-low <T> --yes`, full `rag eval`, plus a before/after check of the Graph Health Score's `Relatio%` component (`scorer.rs`, needs zero code changes — it already reads whatever relations exist). Append to `tests/kwaai-knowledge/results/eval_log.md` in the existing format, with `llm_confirm_rate`/precision-recall added.

## Files to create/modify

**Create:**
- `core/crates/kwaai-rag/src/relation_extract.rs` — classifier, trigger tables, axioms, metrics (§2).

**Modify:**
- `core/crates/kwaai-rag/src/graph.rs` — `RelationRecord` fields + defaults (§1), `upsert_relation_with_confidence()` (§1), dedup-guard trust filter (§5).
- `core/crates/kwaai-rag/src/ingestion.rs` — `GraphIngestConfig` new fields, `effective_no_relations` override (§4).
- `core/crates/kwaai-cli/src/rag_cmd.rs` — `extract_relations_axiomatic()`, `verify_relation_candidates_llm()`, generalized `build_ec_prompt()`/whitelist, CLI wiring, call-site insertion (§3, §4, §6).
- `core/crates/kwaai-cli/src/cli.rs` — two new flags on `Rebuild` and `GraphAction::Build` (§6).
- `tests/kwaai-knowledge/axiomatic_sweep.sh` — Phase A/B/C extension for relations (§7).
- `core/crates/kwaai-rag/src/lib.rs` — register new module.

## Verification

1. **Unit tests** in `relation_extract.rs` (mirror `axiom.rs`'s style): trigger matching + reversed-trigger swap, ambiguous-window demotion, three-way split boundary behavior (including the `threshold_low > threshold_high` collapse case), confidence multiplication.
2. **`cargo test -p kwaai-rag`** and **`cargo build -p kwaainet --release`** must pass; reinstall + codesign per project convention before any CLI testing.
3. **Phase A/B/C sweep** on D6 (§7) — confirms the pipeline runs end-to-end at 1%/10%/100% scale, produces sane metrics JSON, and doesn't regress `rag eval`'s score versus the current 90.4% post-rebuild baseline.
4. **Manual spot-check**: after Phase C, run `kwaainet rag graph export --kb D6` and inspect a sample of newly-`axiomatic_high`/`axiomatic_llm_verify`-sourced relations in the Obsidian export for obvious errors, plus confirm `rag graph score`'s `Relatio%` component improved versus the pre-Phase-4 baseline (39.6% overall / 209 relations, from the 2026-07-16 rebuild).
5. **Regression check**: run with `--relation-threshold-high 0.0` (default) and confirm output is byte-for-byte identical to current behavior — the legacy path must remain fully intact when the feature is off.
