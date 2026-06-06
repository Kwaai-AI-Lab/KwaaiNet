# Pronoun Coreference Resolution — kwaai-rag

## Context

The D6 memoir is first-person narrative. The narrator (Yousuf Rassool) and family members appear far more often as pronouns and definite descriptions than as proper names:

- "I", "my", "me" → Yousuf Rassool (thousands of occurrences)
- "Grandpa", "my grandfather" → Haji Joosub Maulvi Hamid Gool
- "my mother" → Ayesha Rassool
- "his wife", "she" → Nazima Rassool
- "my brother Fazil" → Fazil Rassool

Currently only proper-name mentions create entity-chunk links. Pronoun mentions are invisible to:
- **Dream**: builds bios from entity evidence — misses pronoun-heavy narrative passages
- **Relation extraction (CC+EC)**: can't identify narrator when Yousuf isn't named in the chunk (caused the "Fazil sibling_of Ayesha" error — EC picked the wrong endpoint because Yousuf wasn't in the chunk's entity list)
- **Retrieval**: "what did the narrator do?" only hits chunks where "Yousuf" appears by name

**Goal:** a `coref` pass that resolves pronouns and definite descriptions to canonical entities, then adds chunk links via the existing `link_chunk()` mechanism — no schema changes needed.

---

## Where It Fits in the Pipeline

```
graph build (CC/EC)
  → entity extraction + proper-name chunk links

kwaainet rag graph coref --kb D6 [--sample 0.01] [--commit]
  → pronoun/description resolution → additional chunk links

dream / dedup / relation extraction
  → now sees richer entity evidence
```

Run after `graph build`, before `dream`. Can be re-run incrementally.

---

## Existing Infrastructure to Reuse

| Component | File:line | Reuse |
|-----------|-----------|-------|
| `ner::resolve_pronouns(text, gender_context)` | `src/ner.rs:186` | Extend to accept graph candidates instead of global gender snapshot |
| `refine_low_confidence_entities()` | `src/ingestion.rs:1118` | Template for LLM tier: one entity, constrained JSON question |
| `GraphStore::link_chunk(chunk_id, entity_ids)` | `src/graph.rs:892` | Write pronoun-resolved links — already deduplicates |
| `EntityNode.gender` | `src/graph.rs:220` | Gender-matching in rule tier |
| `EntityNode.aliases` | `src/graph.rs:212` | Match "Grandpa", "narrator", "the author" |
| `MetaStore::all_chunks()` | `src/meta_store.rs:132` | Iterate all chunks with text + section_name |
| `GraphStore::all_chunk_entity_pairs()` | `src/graph.rs:3187` | Candidate antecedents per chunk |
| `FAMILY_TRIGGERS` constant | `src/rag_cmd.rs:3884` | Possessive-kinship detection |

No new DB tables. No schema changes to `EntityNode`.

---

## New Command

```bash
kwaainet rag graph coref --kb D6
  [--sample 0.01]          # fraction of chunks to process
  [--output FILE]          # markdown review file (no-commit default)
  [--commit]               # write links to graph
  [--window 2]             # ±N chunks for candidate antecedent context
  [--inference-url URL]    # for LLM tier (default: localhost:11434)
  [--model MODEL]          # default: llama3.1:8b
```

Follows the same pattern as `rag graph extract-relations`.

---

## Three-Tier Pipeline (per chunk)

### Tier 0 — Candidate antecedent collection

For each chunk being processed:
1. Get the chunk's own linked entities from `chunk_to_entities`
2. Get ±`window` adjacent chunks' entity sets (prefer same `section_name`)
3. Union → `candidates: Vec<EntityNode>`, filter to `entity_type == "Person"`
4. Always include the narrator entity (any entity with alias "narrator"/"I"/"author")

### Tier 1 — Rule-based resolution (zero LLM calls, confidence 0.9)

**1a. Definite description / alias match**
- "Grandpa" → check if any candidate has "Grandpa" or "grandfather" as alias
- "the author" / "the narrator" → narrator entity
- "my mother" → entity with alias "mother" in candidate list OR entity with `parent_of` relation to narrator
- Match case-insensitively against `entity.name` and `entity.aliases`

**1b. Gender + intra-sentence nearest Person**
- If a sentence has exactly one pronoun (he/she) and exactly one candidate entity of matching gender → attach
- Extend `ner::resolve_pronouns()` to accept the per-chunk `candidates` list instead of the global gender snapshot

**1c. Possessive-kinship identification**
- "my brother Fazil" with narrator known → identify Fazil in candidates, emit `sibling_of(narrator, Fazil)`
- "his wife Nazima" with known subject → emit `spouse_of(subject, Nazima)`
- Reuse `FAMILY_TRIGGERS` from `rag_cmd.rs` for trigger-word detection

### Tier 2 — LLM-assisted for uncertain cases (EC refinement pattern, confidence 0.7)

For unresolved pronouns where Tier 1 confidence < 0.7, build a constrained coref prompt:

```
In the following passage, who does "[pronoun]" refer to?

Candidates:
  - Yousuf (Joe) Rassool — the author and narrator of this memoir
  - Ayesha Rassool — his mother (also known as Lallie)
  - Haji Joosub Maulvi Hamid Gool — his maternal grandfather

Passage:
"[2–3 sentence window around the pronoun]"

Answer with the EXACT canonical name from the list, or "none".
Return JSON: {"referent": "Canonical Name"} or {"referent": "none"}
```

Validate JSON, ensure referent is in candidate list, discard otherwise.

### Tier 3 — Graph write (--commit only)

For each resolved `(chunk_id, entity_id, confidence)`:
1. `graph.link_chunk(chunk_id, &[entity_id])` — adds chunk to entity's evidence list
2. If kinship inferred: `graph.upsert_relation(from_id, to_id, rel_type, chunk_id)` — increases relation strength

**Do NOT increment `mention_count`** — that tracks proper-name mentions; coref is softer evidence.

---

## Review File Format

```markdown
## Chunk 47 / 1136

**Section:** Chapter Three — Buitencingle
**Candidates (±2 chunks):**
  - Yousuf Rassool (narrator)  mentions=172
  - Haji Joosub Maulvi Hamid Gooli  mentions=104

**Tier 1 resolutions:**
  - "my grandfather" → Haji Joosub Maulvi Hamid Gooli  ✅ alias match  conf=0.90
  - "he" → Haji Joosub Maulvi Hamid Gooli  ✅ gender+nearest  conf=0.90

**Tier 2 (LLM):**
  - "the old man" → Haji Joosub Maulvi Hamid Gooli  conf=0.70

**Links to commit:** chunk 47 → [Haji Joosub Maulvi Hamid Gooli]
**Relations to commit:** none

---
```

---

## Files to Modify

| File | Change |
|------|--------|
| `core/crates/kwaai-cli/src/cli.rs` | Add `GraphAction::Coref { sample, output, commit, window, inference_url, model }` |
| `core/crates/kwaai-cli/src/rag_cmd.rs` | Add `cmd_coref()` + dispatch; ~300 lines following extract-relations pattern |
| `core/crates/kwaai-rag/src/ner.rs` | Extend `resolve_pronouns()` to accept graph candidate list; add `resolve_definite_descriptions()` |
| `core/crates/kwaai-rag/src/graph.rs` | Add `coref_candidates_for_chunk(chunk_id, window) -> Vec<EntityNode>` helper |

---

## Connection to CC+EC Relation Extraction

The coref pass directly fixes the narrator-context problem in relation extraction:

- **Current**: "my brother Fazil" → CC quotes "brother Fazil" → EC can't find narrator in entity list → picks wrong endpoint (Ayesha instead of Yousuf)
- **After coref**: chunks with "my/I/me" are now linked to Yousuf Rassool → Yousuf appears in the chunk entity list → EC correctly extracts `Yousuf Rassool sibling_of Fazil Rassool`

The coref pass should run before `extract-relations` for best results.

---

## Rollout

| Stage | Command | Goal |
|-------|---------|------|
| 1% dry-run | `--sample 0.01 --output review_1pct.md` | Manually check ~11 chunks |
| 10% dry-run | `--sample 0.10 --output review_10pct.md` | Rule vs LLM accuracy split |
| Full commit | `--commit` | Once precision confirmed from sample |

**Measurable after commit:**
- `kwaainet rag graph show --kb D6 "Yousuf Rassool"`: `entity_to_chunks` count increases significantly (from ~172 to 400+)
- Eval: questions about narrator activities, siblings, parents improve (more chunk evidence in retrieval)
- CC+EC: narrator in chunk entity list → correct sibling/parent extraction

---

## Phase 2 (deferred)

- Add `PRONOUN_LINK_TABLE` (chunk_id, entity_id, mention_type, confidence) for Dream to prefer proper-name evidence over pronoun-only chunks
- Extend to Organization and Place entities (not just Person)
- Run inter-chapter coref for entities mentioned by name in one chapter and only as pronouns in subsequent chapters
