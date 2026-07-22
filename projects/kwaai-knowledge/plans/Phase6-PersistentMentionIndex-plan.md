# Phase 6: Persistent Per-Chunk Mention Index

## Context

Validating Phase 4's relation extraction against real D6 candidates today surfaced a structural
inefficiency the user caught directly while watching a sweep run: **entities are effectively
re-extracted every time relations run.** Two distinct things are happening under that one observation:

1. **Test-script level** (`relation_sweep.sh`'s two-pass pattern): pass 2 re-invokes `graph build`'s
   full entity pipeline a second time. It mostly short-circuits via `KnownEntity` matching (cheap), but
   it's wasted work and a small consistency risk on every sweep run.
2. **Architectural, and the real target of this phase**: `relation_extract.rs::classify_relation_candidates`
   independently re-scans each chunk's raw text *every time relations run* — literal name/alias matching,
   the surname-drop fallback, definite-description resolution (`ner::resolve_definite_descriptions`),
   third-person pronoun resolution (`ner::resolve_pronouns_from_candidates`), and first-person narrator
   pronoun resolution (`find_narrator_pronoun_positions`) are all redone from scratch, using none of what
   entity extraction already determined about that same chunk.

This duplication isn't just wasteful — it's a source of inconsistency. Entity extraction (Step 4) commits
entities using the axiomatic classifier and (for genuinely new candidates) an LLM call. Relation
extraction later re-derives *where in the text* those same entities are mentioned using a separate,
narrower heuristic (nearest-nearest-name/pronoun scan) that has no knowledge of what extraction already
resolved. Several of today's real bugs traced directly to this gap: the "Zain Rassool" vs. "Abdul Hamid"
mismatch (`"Abdul Hamid, a decade into his marriage with Cissie..."`) happened because the relation
classifier's own position-finding grabbed the wrong candidate from a wider window, not because entity
extraction got anything wrong. Phase 5's planned classifiers (trained on "a window of text around a
trigger word") will need exactly this same mention-to-entity mapping too — building it once, correctly,
is better than three different consumers each deriving their own approximation.

**The fix**: persist a per-chunk mention index — every noun, alias, pronoun, and definite-description
span resolved to an entity ID — as a byproduct of entity extraction itself, when the full context for
that chunk is freshest. Relation extraction (and later, Phase 5) then read this index instead of
re-deriving it.

## Architecture

### 1. New persisted structure (`core/crates/kwaai-rag/src/graph.rs`)

Add `chunk_mentions`, following the exact existing convention for `chunk_entity`/`entity_chunk`
(`graph.rs:635-638`): a plain key→JSON-value table, `key = chunk_id.to_le_bytes()`,
`value = serde_json::to_vec(&Vec<MentionSpan>)`. `CREATE TABLE IF NOT EXISTS chunk_mentions (key BLOB
NOT NULL PRIMARY KEY, value BLOB NOT NULL) WITHOUT ROWID;` added to the same schema block — no separate
migration mechanism needed, `IF NOT EXISTS` already handles both fresh and existing DBs.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionSpan {
    pub entity_id: i64,
    pub start: usize,      // byte offset into the chunk's text, char-boundary safe
    pub end: usize,
    pub kind: MentionKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MentionKind {
    Literal,          // exact name/alias match
    SurnameDropped,    // truncated-name fallback match (relation_extract.rs's existing logic)
    Pronoun,           // third-person "he"/"she", gender-matched
    DefiniteDescription, // "my wife", "the author", etc.
    NarratorPronoun,   // bare first-person "I"/"my"/"me"
}
```

New `GraphStore` methods `write_chunk_mentions(chunk_id, &[MentionSpan])` / `get_chunk_mentions(chunk_id)
-> Vec<MentionSpan>`, mirroring `link_chunk`/`get_chunk_entities`'s existing read/write shape exactly.

### 2. Shared mention-resolution function (new: `core/crates/kwaai-rag/src/mentions.rs`)

Extract the position-finding logic currently inline in `relation_extract.rs::classify_relation_candidates`
(the literal-name/alias loop with surname-drop fallback, the overlap-resolution dedup, and the three coref
calls) into one reusable function:

```rust
pub fn resolve_chunk_mentions(
    chunk_text: &str,
    known_entities: &[(i64, String, Vec<String>)],
    person_candidates: &[(String, Vec<String>, Option<String>)],
    narrator: Option<(i64, &str)>,
) -> Vec<MentionSpan>
```

This is a refactor, not new logic — the literal/surname-drop matching, `ner::resolve_definite_descriptions`/
`resolve_pronouns_from_candidates`/`find_narrator_pronoun_positions` calls, and the existing
overlap-resolution dedup (the "same entity redundant" / "same span genuine ambiguity" / "different span
containment" distinction already fixed earlier this session) all move here unchanged. Both entity
extraction (new caller) and relation extraction (existing caller, now reading instead of deriving) use it.

### 3. Wire into entity extraction (`core/crates/kwaai-rag/src/ingestion.rs`)

Both `extract_and_store_entities_pub` (`ingestion.rs:423`) and `extract_entity_centric`
(`ingestion.rs:1021`) call `graph.link_chunk(chunk_id, &entity_ids_for_chunk)` right after committing a
chunk's entities — `entity_ids_for_chunk` is already fully resolved at that point. Add immediately after:
resolve those IDs to `(id, name, aliases)` triples (same shape `known_entities` already takes elsewhere),
build `person_candidates`/`narrator` the same way `extract_relations_axiomatic` already does in
`rag_cmd.rs` (via `g.coref_candidates_for_chunk`), call `resolve_chunk_mentions`, and
`graph.write_chunk_mentions(chunk_id, &mentions)`.

### 4. Simplify relation extraction (`core/crates/kwaai-rag/src/relation_extract.rs`)

`classify_relation_candidates` stops taking `known_entities`/`person_candidates`/`narrator` and re-deriving
positions; it takes `chunk_mentions: &[MentionSpan]` (fetched once via the new
`graph.get_chunk_mentions(chunk_id)`, same call-site pattern as today's `get_chunk_entities`) and goes
straight to trigger-phrase scanning + nearest-mention-before/after lookup against the pre-built list. This
removes essentially all of the position-finding code from this function — it becomes purely "scan for
trigger phrases, look up nearest mention on each side" once the index exists.

### 5. Rollout / existing-KB compatibility

New ingestion runs get the index automatically. For already-ingested KBs (D6, mid-experimentation right
now), two options — don't build the backfill path unless it's actually needed:
- Cheapest: a full `--reset-graph` rebuild naturally populates it (we're already doing more of these to
  validate Phase 4 thresholds).
- If backfilling matters before then: a `kwaainet rag graph build-mention-index --kb X` command that
  iterates existing chunks + their already-linked entities and just calls `resolve_chunk_mentions` without
  any LLM call (cheap — it's applying already-proven matching logic against entities the graph already
  knows about).

## Files to create/modify

**Create:**
- `core/crates/kwaai-rag/src/mentions.rs` — `MentionSpan`/`MentionKind`, `resolve_chunk_mentions()`
  (relocated from `relation_extract.rs`, not reinvented).

**Modify:**
- `core/crates/kwaai-rag/src/graph.rs` — new `chunk_mentions` table + `write_chunk_mentions`/
  `get_chunk_mentions`.
- `core/crates/kwaai-rag/src/ingestion.rs` — call the new resolver + writer at both `link_chunk` sites.
- `core/crates/kwaai-rag/src/relation_extract.rs` — `classify_relation_candidates` reads the index
  instead of re-deriving positions; delete the now-dead inline position-finding code.
- `core/crates/kwaai-cli/src/rag_cmd.rs::extract_relations_axiomatic` — fetch
  `graph.get_chunk_mentions(cid)` instead of building `known_entities`/`person_candidates` fresh per call.
- `core/crates/kwaai-rag/src/lib.rs` — register `mod mentions;`.

## Verification

1. **Unit tests** for `resolve_chunk_mentions` — port the existing position-finding tests currently in
   `relation_extract.rs` (literal match, surname-drop, definite-description, pronoun, narrator-pronoun,
   the ambiguity-collision case) over to `mentions.rs` unchanged in intent, since the logic itself isn't
   changing, only its location and caller.
2. **`classify_relation_candidates` tests** — same relation-type/trigger tests as today, now constructing
   a `Vec<MentionSpan>` fixture directly instead of `known_entities`/`person_candidates`/`narrator`.
3. **Regression check on the specific bugs this targets**: re-run the "Abdul Hamid"/"Zain Rassool"
   sentence from today's 10% D6 run and confirm the mention index correctly attributes "Abdul Hamid" (not
   a wrong, further-away entity) as the nearest mention before "marriage with."
4. **1%/10%/100% staged validation** (same methodology used throughout this session) — confirm relation
   candidate counts/quality are at least as good as before the refactor, and that entity-extraction wall
   time doesn't regress meaningfully (the new per-chunk mention resolution is pure Rust string matching,
   no additional LLM calls).

## Implementation notes (post-hoc, written after building)

**Deviation from §3 (wire into entity extraction):** the index is built in
`rag_cmd.rs::extract_relations_axiomatic`, right before relation classification — not at `ingestion.rs`'s
`link_chunk` call sites as originally planned. Reason: entity IDs can be renamed/merged by seeding/dedup
steps that run *after* Step 4 (entity extraction) but *before* Step 5.5b (relations). Building the index at
`link_chunk` time risked baking in entity IDs that seeding/dedup later invalidates; building it at
relation-extraction time — which already runs post-seed/dedup — sidesteps that staleness risk entirely.
`ingestion.rs` was not touched.

**The "Abdul Hamid" mismatch had two layered causes, not one:**
1. `known_entities` was scoped to only the current chunk's own linked entities, so an entity linked to an
   adjacent chunk wasn't a resolution candidate at all. Fixed by widening `extract_relations_axiomatic`'s
   `known_entities` to draw from ±3 adjacent chunks (`COREF_WINDOW` 2 → 3) — safe here (unlike the
   `run_timeline_build` "Gandhi in a cricket chunk" LLM-fabrication risk documented elsewhere in
   `rag_cmd.rs`) because this pathway is deterministic substring matching, not an LLM given a free-choice
   name list.
2. The real root cause, found after the widening alone didn't fix the target sentence and briefly
   introduced a new false positive: `ner::resolve_pronouns_from_candidates` picked whichever gender-matching
   candidate was *last in the caller's list* (`.rev().find()`), not whichever was textually nearest the
   pronoun. In `"Abdul Hamid, ... his marriage with Cissie..."`, "his" could resolve to any Male candidate in
   the window — not necessarily Abdul Hamid — and because it sat closer to the trigger than the correct
   literal "Abdul Hamid" match, it won `classify_relation_candidates`'s `before_best` closest-wins
   comparison. Fixed by making `resolve_pronouns_from_candidates` rank gender-matching candidates by actual
   text proximity (same `rfind`-based pattern `resolve_place_pronouns_from_candidates` already used for
   spatial pronouns), falling back to list order only when no gender-matching candidate is named anywhere
   in the text before the pronoun. Regression test:
   `relation_extract.rs::pronoun_resolves_to_nearest_literal_antecedent_not_list_order`.

Validated via the 1%/10%/100% staged methodology: 10% rerun after the proximity fix shows the target
sentence's subject correctly resolves to "Abdul Hamid Gool" (previously 4 different wrong entities across
successive runs), and the false positive introduced by the widening alone
(`Ahmed Abdurahman member_of Congress Movement`) is now correctly rejected by LLM-verify instead of
silently committed.
