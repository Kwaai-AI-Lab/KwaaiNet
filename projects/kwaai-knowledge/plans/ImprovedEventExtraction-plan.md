# Improved Event Extraction Plan
*Save to: `projects/kwaai-knowledge/plans/ImprovedEventExtraction-plan.md`*

## Context

Timeline event extraction currently yields 26–55 events (non-deterministic) against a 197-event
baseline. The root cause is that the current LLM-first approach (`extract_temporal_events()`) runs
an 8B model over every chunk to find dates AND attribute them to entities in a single combined call.
This fails in three ways:
1. **Non-determinism**: temperature=0.0 is not deterministic across different GPU hardware — the
   same chunk produces different event counts on A6000 vs A5000 vs CPU.
2. **Prompt fragility**: the prompt has 9 rules (R1-R9) needed to prevent hallucinations, but any
   change causes catastrophic event count collapse (55→20→0 observed in v0.4.138-140).
3. **Rule-based suppression ceiling**: R2 gates events on the entity being pre-linked to the chunk,
   R4 suppresses narrator events, R5 conservatively handles birth/death — all necessary to prevent
   hallucinations but collectively miss many correct events.

**Proposed fix**: Decouple date detection (deterministic regex) from entity attribution (confidence-
scored heuristic), and use the LLM only as a fallback for genuinely ambiguous cases. This restores
determinism and removes the combined prompt's structural ceiling.

---

## Architecture: 4-Phase Pipeline

```
Per-chunk input: chunk_id, text, linked_entity_ids (from graph)
                      │
          ┌───────────▼────────────┐
Phase 1   │ scan_chunk_for_dates() │  deterministic regex — no LLM
          │  → Vec<DateMention>    │
          └───────────┬────────────┘
                      │
          ┌───────────▼──────────────────┐
Phase 2   │ attribute_dates_to_entities()│  heuristic confidence scoring
          │  → (high_conf, low_conf)     │
          └───────────┬──────────────────┘
                      │
          ┌───────────▼────────────┐
Phase 3   │ resolve_extracted()   │  existing Axioms 1–7 (reused unchanged)
          │ (existing, unchanged) │
          └───────────┬────────────┘
                      │
          ┌───────────▼──────────────────────────┐
Phase 4   │ extract_events_for_uncertain()       │  LLM only for low-confidence
          │  → Vec<RawEvent> (merged back)       │
          └──────────────────────────────────────┘
```

---

## New Data Structures (add to `sequence.rs`)

```rust
pub struct DateMention {
    pub chunk_id: i64,
    pub date_raw: String,       // exactly as found: "1884", "February 1914", "the 1880s"
    pub date_sort: String,      // normalize_date(date_raw) — ISO-padded
    pub date_confidence: f32,   // 0.9 explicit year, 0.75 month+year, 0.5 decade, 0.3 phrase
    pub char_offset: usize,     // byte offset of date in chunk text
    pub sentence: String,       // full sentence containing the date (max 300 chars)
    pub mention_type: DateMentionType,
}

pub enum DateMentionType {
    ExplicitYear,    // bare "1884"
    MonthYear,       // "February 1884", "1 April 1940"
    Decade,          // "the 1880s", "1880s"
    RelativePhrase,  // "at the turn of the century", "during the removals"
}

pub struct AttributedEvent {
    pub date_mention: DateMention,
    pub entity_id: i64,
    pub entity_name: String,
    pub description: String,           // trimmed sentence ±40 words around date
    pub event_class: String,           // same classes as existing TimelineEvent
    pub confidence: f32,               // date_confidence × attribution_confidence
    pub attribution_method: AttributionMethod,
}

pub enum AttributionMethod {
    ProximityHigh,     // entity within ±20 tokens of date
    ProximitySentence, // entity in same sentence
    SoleEntity,        // only one entity linked to chunk
    KinshipMap,        // narrator kinship phrase ("my grandfather" → JMH)
    LlmFallback,       // came from LLM call
}
```

---

## Phase 1: Deterministic Date Scanner

**New function** in `sequence.rs`:
```rust
pub fn scan_chunk_for_dates(text: &str, chunk_id: i64) -> Vec<DateMention>
```

**Detection patterns** (applied in order, first match wins per span):

| Pattern | Type | date_confidence |
|---------|------|-----------------|
| `\b(\d{1,2}\s+(?:Jan|Feb|...|December)\s+(1[7-9]\d{2}|20\d{2}))\b` | MonthYear | 0.85 |
| `\b((?:Jan|...|December)\s+(1[7-9]\d{2}|20\d{2}))\b` | MonthYear | 0.75 |
| `\b(1[7-9]\d{2}|20\d{2})\b` | ExplicitYear | 0.90 |
| `\b(1[7-9]\d0s|20\d0s)\b` | Decade | 0.50 |
| Relative phrase dict lookup | RelativePhrase | 0.30 |

**Relative phrase dictionary** (hardcoded `const RELATIVE_DATE_PHRASES` in `sequence.rs`):

| Phrase | Approx year (date_sort) | Notes |
|--------|------------------------|-------|
| "at the turn of the century" / "turn of the century" | 1900 | Cape Colony era |
| "the war years" / "during the war" / "war time" | 1941 | WW2 |
| "before the removals" | 1960 | Group Areas enforced ~1966 |
| "during the removals" | 1970 | 1966–1982 |
| "after the removals" | 1985 | post-1982 |
| "after independence" / "since independence" | 1962 | Republic 1961 |
| "before independence" | 1959 | |
| "in the early days" | 0 | too vague — skip, do not emit |
| "at that time" | 0 | too vague — skip |
| "in those days" | 0 | too vague — skip |

Phrases with year=0 are skipped entirely. Relative phrases get `date_confidence: 0.30` — virtually
all will fall below the composite threshold and go to the LLM.

**Reuses**: `normalize_date()` for `date_sort`, `parse_4digit_year()` for ExplicitYear detection.
**Sentence extraction**: split on `[.!?]\s+[A-Z]`; return sentence containing `char_offset`.

---

## Phase 2: Entity Attribution with Confidence

**New function** in `sequence.rs`:
```rust
pub fn attribute_dates_to_entities(
    mentions: Vec<DateMention>,
    chunk_entities: &[(i64, String, Vec<String>)], // (id, canonical, aliases)
    narrator_kinship: &HashMap<String, (i64, String)>,
    chunk_text: &str,
    confidence_threshold: f32,
) -> (Vec<AttributedEvent>, Vec<DateMention>)
// Returns: (events above threshold, mentions below threshold for LLM)
```

**Scoring logic** for each (DateMention, candidate entity) pair:

1. **Kinship map first**: if sentence contains a kinship phrase from `narrator_kinship` (e.g. "my
   grandfather") → attribution_confidence = 0.85, method = KinshipMap.

2. **Sole entity**: if `chunk_entities.len() == 1` and `entity_present_in_text()` → attribution_confidence = 0.75, method = SoleEntity.

3. **Proximity** (when multiple entities):
   - Entity name/alias within ±20 tokens of date char_offset → 0.85, ProximityHigh
   - Entity name/alias anywhere in same sentence → 0.60, ProximitySentence
   - Entity in chunk but not sentence → 0.35 (goes to LLM)

4. **Narrator penalty**: if narrator entity is the closest match but sentence is third-person
   (no first-person pronouns "I/my/we/our") → subtract 0.15 from attribution_confidence.

**composite confidence** = `date_mention.date_confidence × attribution_confidence`

**Decision**: composite ≥ `confidence_threshold` → emit AttributedEvent; else → LLM queue.

**Description**: use `mention.sentence`. **Event class**: rule-based from sentence keywords (same
logic as existing `extract_temporal_events()` prompt, now in pure Rust):
- "arrived/came/moved/settled" → `arrival`; "founded/established/built" → `founding`
- "died/passed away/death" → `death`; "was born/birth/born in" → `birth`
- "removed/evicted/demolished/forced" → `removal`; "married/wedded" → `marriage`
- default → `other`

---

## Phase 3: Axiomatic Validation

**Reuse `resolve_extracted()` unchanged.** Convert AttributedEvents to RawEvents before passing:

```rust
let raw_events: Vec<RawEvent> = attributed_events.iter().map(|ae| RawEvent {
    entity: ae.entity_name.clone(),
    date: Some(ae.date_mention.date_raw.clone()),
    description: ae.description.clone(),
    class: Some(ae.event_class.clone()),
}).collect();
```

All 7 existing axioms apply. No changes to `resolve_extracted()`.

---

## Phase 4: Selective LLM Fallback

**New function** in `sequence.rs`:
```rust
pub async fn extract_events_for_uncertain(
    mentions: Vec<DateMention>,
    chunk_text: &str,
    entity_names: &[String],
    pronoun_map: &[(String, String)],
    inference_url: &str,
    model: &str,
) -> Result<Vec<RawEvent>>
```

Single batched LLM call per chunk (all low-confidence mentions together), not one call per mention.
Focused prompt — no 9-rule machinery, just:

```
You are a historical event extractor. Given the passage and date references below, for each
date reference identify which entity (from the list) the event belongs to.

PASSAGE: {chunk_text}

DATE REFERENCES:
{for each mention: "- {date_raw}: \"{sentence}\""}

ENTITIES IN THIS PASSAGE: {entity_names}
COREFERENCE: {pronoun_map as "'{pronoun}' → {name}"}

For each date reference output one JSON per line:
{"date": "...", "entity": "...", "description": "...", "class": "..."}
If no entity clearly matches, output {"date": "...", "entity": null}.
Do not invent events not supported by the passage.
```

Null-entity responses are discarded. Results flow back through Phase 3 axioms.

**When no low-confidence mentions**: skip LLM entirely — the main performance win for common chunks
with a single clear entity.

---

## Integration in `rag_cmd.rs`

In `run_timeline_build()`, replace the `extract_temporal_events()` call at ~line 8326:

```rust
// Phase 1
let date_mentions = kwaai_rag::sequence::scan_chunk_for_dates(&clean_text, chunk_id);

// Phase 2
let narrator_kinship = {
    let g = graph.lock().await;
    narrator_id.map(|nid| kwaai_rag::sequence::narrator_kinship_map(nid, &g))
               .unwrap_or_default()
};
let (attributed, low_conf) = kwaai_rag::sequence::attribute_dates_to_entities(
    date_mentions, &entity_data, &narrator_kinship, &clean_text, confidence_threshold,
);

// Phase 4 (only if needed)
let llm_events = if !low_conf.is_empty() {
    kwaai_rag::sequence::extract_events_for_uncertain(
        low_conf, &clean_text, &entity_names, &pronoun_map, &infer_url, &model,
    ).await.unwrap_or_default()
} else {
    vec![]
};

// Convert and merge for Phase 3
let raw_events: Vec<RawEvent> = attributed.into_iter()
    .map(|ae| RawEvent { entity: ae.entity_name, date: Some(ae.date_mention.date_raw),
                         description: ae.description, class: Some(ae.event_class) })
    .chain(llm_events)
    .collect();
let raw_interactions = vec![]; // keep extract_kinship_interactions() for two-entity interactions
```

Keep `extract_kinship_interactions()` call unchanged — two-entity interactions are already
rule-based and unaffected.

---

## Files to Modify

| File | Change |
|------|--------|
| `core/crates/kwaai-rag/src/sequence.rs` | Add `DateMention`, `DateMentionType`, `AttributedEvent`, `AttributionMethod`; add `scan_chunk_for_dates()`, `attribute_dates_to_entities()`, `extract_events_for_uncertain()`, `RELATIVE_DATE_PHRASES` const |
| `core/crates/kwaai-cli/src/rag_cmd.rs` | Replace `extract_temporal_events()` call in `run_timeline_build()` with 4-phase pipeline; thread `confidence_threshold` through |
| `core/crates/kwaai-cli/src/cli.rs` | Add `--confidence-threshold <f32>` (default 0.6) to `Timeline Build` variant |

**Do not modify**: `resolve_extracted()`, `TimelineEvent`, `SequenceInteraction`, `RawEvent`,
`RawInteraction`, `narrator_kinship_map()`, `entity_present_in_text()`, `normalize_date()`.

---

## Confidence Threshold Tuning

Default: 0.6. Tune by running timeline build at different thresholds and comparing eval scores:

| Threshold | Expected behavior |
|-----------|------------------|
| 0.8 | Minimal LLM calls; only explicit-year + sole-entity events pass. Low coverage. |
| 0.6 (default) | Most explicit-year + proximate-entity events pass; relative phrases and multi-entity chunks go to LLM |
| 0.4 | More LLM calls; catches ambiguous multi-entity chunks deterministically |
| 0.2 | Nearly all events go through LLM — matches current architecture's behavior |

---

## Verification

```bash
# Build
cd core && cargo build -p kwaainet --release
cp core/target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet

# Rebuild timeline (default threshold 0.6, GPU relay)
kwaainet rag graph timeline build --kb D6 \
  --inference-urls "p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd" \
  --workers 4 --reset

# Verify determinism: run twice, event count must be identical
kwaainet rag graph timeline show --kb D6 | grep -c "event"  # run 1
kwaainet rag graph timeline build --kb D6 ... --reset
kwaainet rag graph timeline show --kb D6 | grep -c "event"  # run 2 — must match

# Check event count (target: >100, was 26-55)
kwaainet rag graph timeline show --kb D6

# Run eval and log
kwaainet rag eval --kb D6 --questions tests/kwaai-knowledge/d6_eval_questions.json
# Log to tests/kwaai-knowledge/results/eval_log.md
```

**Success criteria**:
- Deterministic event count on back-to-back runs (was non-deterministic)
- Event count substantially above 55 (targeting >100)
- Eval score ≥ 65.8% (must not regress from v0.4.137 baseline)
