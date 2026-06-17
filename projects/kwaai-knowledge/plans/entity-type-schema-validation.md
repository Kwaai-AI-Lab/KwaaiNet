# Entity Type Expansion with Per-KB Schema and Two-Stage Validation

> Save as: `projects/kwaai-knowledge/plans/entity-type-schema-validation.md`

## Context

The D6 eval (now 82.7%) reveals that entity extraction is noisy for non-Person types — organisations,
places, legislation, and abstract concepts are either missed or mis-classified. The naive fix would be
to hardcode D6-specific category definitions in the Rust pipeline. That would be the wrong move: this
is a general-purpose RAG system, and the pipeline must stay domain-agnostic.

The right fix has two parts:
1. **Generic infrastructure**: New entity types (`Legislation`, `Publication`) added to the pipeline
   with proper field schemas. Existing `Concept` is already present but has no expected fields.
2. **Per-KB schema configuration**: Type definitions (what counts as an Organisation in *this*
   document), examples, anti-examples, and validation rules live in KB metadata (a YAML file loaded
   into `METADATA_TABLE`), not in compiled Rust.

The 70b validation model then validates ambiguous extractions against *the KB's own definitions*,
not hardcoded rules — so it works identically for a D6 memoir, a legal corpus, or a scientific paper.

---

## What already exists (do not re-implement)

- `ENTITY_TYPES: &[&str]` in `graph.rs:43` — extensible constant slice, not a compiled enum.
  Already includes `Concept`, `Document`. Need to add `Legislation`, `Publication`.
- `extraction_confidence: f32` on `ExtractedEntity` and `EntityNode` — LLM-returned at
  extraction time (1.0 / 0.5 / 0.2). Already merged via `f32::max` on upsert.
- `ec_refine_threshold` + `ec_refine_budget` on `GraphIngestConfig` — existing second-pass
  refinement for low-confidence entities. The new validation pass is *different* (type
  correctness check, not field enrichment) but reuses the same batch dispatch pattern.
- `METADATA_TABLE` in `GraphStore` with `set_doc_metadata` / `get_doc_metadata` — the
  pattern for KB-wide configuration persistence is already there.
- `entity_types: Vec<String>` on `GraphIngestConfig` — already filters extraction output.
  Already loaded from `--entity-types` CLI flag.

---

## Design Principles

1. **Rust constants = defaults only.** `ENTITY_TYPES` lists all possible types. KBs that don't
   provide a schema extract all of them. KBs that do provide a schema restrict to their list.
2. **Domain knowledge lives in YAML, not code.** What counts as "Legislation" in D6 is in
   `tests/kwaai-knowledge/d6_entity_schema.yaml`, not in `graph.rs`.
3. **Validation uses the KB's own definitions.** The 70b prompt injects the KB schema's
   description + examples, so the same Rust code works for any domain.
4. **Graceful degradation.** No KB schema → behaves exactly as today. Validation model not
   specified → stage 1 only (existing behaviour). Every new flag is additive.

---

## Changes

### 1. New entity types in `graph.rs`

Add to `ENTITY_TYPES` constant:
```rust
"Legislation",   // Laws, acts, decrees, administrative systems
"Publication",   // Newspapers, books, pamphlets, journals
```

`Concept` is already present. Remove `Document` or keep as alias — `Publication` is more
precise for RAG retrieval (books/papers, not generic documents).

Add entries to `expected_fields()` function (currently only Person/Place/Organization have fields):

```rust
"Legislation" => vec![
    ("dateEnacted",  "Year or date the legislation was passed"),
    ("jurisdiction", "Geographic or political jurisdiction"),
    ("effect",       "Primary effect or purpose of the legislation"),
],
"Publication" => vec![
    ("publisher",     "Publisher, newspaper, or institution"),
    ("datePublished", "Year or date range of publication"),
    ("frequency",     "Daily / weekly / pamphlet / one-off"),
],
"Concept" => vec![
    ("definition", "Short definition of the concept"),
    ("domain",     "Social / political / religious / other"),
],
```

---

### 2. Per-KB entity type schema — `graph.rs` + new YAML

**New struct** (add to `graph.rs` near `EntityNode`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KBEntityTypeSchema {
    pub name: String,
    pub description: String,          // Used verbatim in validation prompt
    #[serde(default)]
    pub examples: Vec<String>,        // Positive examples for 70b prompt
    #[serde(default)]
    pub anti_examples: Vec<String>,   // Counter-examples for 70b prompt
    #[serde(default)]
    pub fields: Vec<String>,          // Expected field names (overrides expected_fields())
}
```

**New METADATA_TABLE key** `"kb_entity_schemas"`: `Vec<KBEntityTypeSchema>` serialized as JSON.

**New methods on `GraphStore`**:
```rust
pub fn set_kb_entity_schemas(&mut self, schemas: &[KBEntityTypeSchema]) -> Result<()>
pub fn get_kb_entity_schemas(&self) -> Vec<KBEntityTypeSchema>
```

**New CLI command**: `kwaainet rag graph schema`
- `set --kb <KB> --file <yaml>` — load schemas from YAML into METADATA_TABLE
- `show --kb <KB>` — print current KB schemas

This command is the D6-specific knowledge injection point. The schema YAML (e.g.
`tests/kwaai-knowledge/d6_entity_schema.yaml`) defines what Organization/Legislation/etc.
mean *for this document*. The Rust pipeline has no knowledge of D6.

---

### 3. Two-stage validation pass — `ingestion.rs`

New field on `GraphIngestConfig`:
```rust
pub validation_model: Option<String>,      // e.g. "llama3.1:70b"; None = no validation
pub validation_confidence_floor: f32,       // default 0.7; entities below this get validated
pub validation_budget: usize,              // max entities to validate per run; default 200
```

**New function** `validate_entities()` called after CC extraction, before scoring:
```rust
async fn validate_entities(
    entities: &mut Vec<EntityNode>,
    schemas: &[KBEntityTypeSchema],
    inference_url: &str,
    model: &str,
    confidence_floor: f32,
    budget: usize,
) -> Result<ValidationStats>
```

**Algorithm:**
1. Collect entities where `extraction_confidence < confidence_floor`, up to `budget`
2. Group by entity type
3. For each group, look up its `KBEntityTypeSchema` (skip types with no schema)
4. Build validation prompt (one batch per type, up to 20 entities per LLM call):
   ```
   You are validating entity classifications for a knowledge base.

   Definition of [Type]: [schema.description]
   Examples: [schema.examples joined with ", "]
   Counter-examples (these are NOT [Type]): [schema.anti_examples joined with ", "]

   For each entity below, answer "yes" if it is truly a [Type] or "no" if it is
   misclassified. Return JSON: [{"name": "...", "valid": true/false, "reason": "..."}]

   Entities to validate:
   [list of {name, source_snippet}]
   ```
5. Entities that fail → set `extraction_confidence = 0.1` (they will be pruned by
   the existing dedup/scoring pipeline or filtered during retrieval)
6. Entities that pass → set `extraction_confidence = max(existing, 0.85)`

**Why not delete failed entities immediately?** The dedup pipeline and dream cycles may
later find supporting evidence. Setting confidence low achieves filtering without
permanent data loss.

**New CLI flags** on `rag graph build`:
```
--validation-model <MODEL>       # 70b or any model; enables stage 2
--validation-floor <FLOAT>       # default 0.7; entities below this are validated
--validation-budget <INT>        # default 200; max per run
```

---

### 4. Temporal events for new entity types — `sequence.rs`

The sequence diagram plan (prior plan) is unchanged. The `extract_temporal_events()` function
already receives `entity_names: &[String]` — new entity types are automatically included
when present in the chunk's linked entities. No code change needed for timeline coverage.

The only addition: include `Legislation` and `Publication` in the `event_class` vocabulary:
```
"enactment" | "repeal" | "amendment"   (for Legislation)
"founding"  | "closure" | "issue"       (for Publication)
```

---

### 5. D6 KB schema YAML (KB-specific, not code)

New file: `tests/kwaai-knowledge/d6_entity_schema.yaml`

```yaml
# Entity type definitions for the D6 "Lest We Forget" knowledge base.
# These are the domain-specific definitions used by the 70b validation model.
# DO NOT put D6-specific knowledge in Rust source — only here.

entity_type_schemas:

  - name: Organization
    description: >-
      A formal group of people organized for a shared political, professional,
      or social purpose, with a recognized name and ongoing membership.
    examples: [NEUM, TLSA, AAC, NEF, ANC, Anti-CAD, Communist Party, TEPA,
               "Western Province Indian Cricket Union", "Kismet Cricket Club"]
    anti_examples: ["Cape Coloured community", "the teachers", "the boycotters",
                    "the Unity Movement tradition", "non-Europeans"]

  - name: Place
    description: >-
      A specific, named geographic location: a street address, building,
      neighbourhood, city, province, or country that can appear on a map.
    examples: ["District Six", "Buitencingle Street", "Cape Town", "Mauritius",
               "India", "Durban", "Woodstock", "Cape Flats", "Natal"]
    anti_examples: ["the south", "the community", "the ghetto", "poor areas"]

  - name: Legislation
    description: >-
      A law, act, decree, or formal administrative/regulatory system with
      legal force, enacted by a government or governing body.
    examples: ["Group Areas Act", "Population Registration Act", "Law 3 of 1885",
               "Coloured Advisory Council", "Bantu Education Act"]
    anti_examples: ["apartheid", "racial policy", "segregation", "the system",
                    "pass laws in general"]

  - name: Publication
    description: >-
      A named newspaper, journal, book, pamphlet, or other published work
      with a distinct title and circulation.
    examples: ["The Torch", "Indian Opinion", "The Cape Argus", "The Guardian",
               "The Awakening of a People", "300 Years", "The Role of the Missionaries"]
    anti_examples: ["a letter", "a speech", "correspondence", "a report", "an article"]

  - name: Concept
    description: >-
      An abstract idea, social category, racial classification, or ideological
      position that shapes the text's argument but is not a physical entity.
    examples: ["non-collaboration", "boycott weapon", "Herrenvolk", "Jaht",
               "Quisling", "Coloured identity", "Unity Movement tradition"]
    anti_examples: ["NEUM", "District Six", "apartheid law",
                    "the government"]  # those are Org/Place/Legislation
```

---

## Files to create / modify

| File | Change |
|------|--------|
| `kwaai-rag/src/graph.rs` | Add `Legislation`, `Publication` to `ENTITY_TYPES`; add `expected_fields()` entries; add `KBEntityTypeSchema` struct; add `set/get_kb_entity_schemas()` |
| `kwaai-rag/src/ingestion.rs` | Add `validation_model`, `validation_confidence_floor`, `validation_budget` to `GraphIngestConfig`; add `validate_entities()` function; call it after CC extraction |
| `kwaai-rag/src/sequence.rs` | Add `Legislation`/`Publication` event classes; otherwise unchanged from prior plan |
| `kwaai-cli/src/cli.rs` | Add `--validation-model`, `--validation-floor`, `--validation-budget` flags to `rag graph build`; add `schema` subcommand to `rag graph` |
| `kwaai-cli/src/rag_cmd.rs` | Wire new flags into `GraphIngestConfig`; dispatch `schema set/show` |
| `tests/kwaai-knowledge/d6_entity_schema.yaml` | New file — D6-specific type definitions |

---

## Verification

```bash
# 1. Build
cd core && cargo build -p kwaainet --release
cp target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet

# 2. Load D6 entity schema into KB
kwaainet rag graph schema set --kb D6 --file tests/kwaai-knowledge/d6_entity_schema.yaml
kwaainet rag graph schema show --kb D6  # verify round-trip

# 3. Rebuild D6 with new entity types + validation
kwaainet rag graph build --kb D6 \
  --model llama3.1:8b \
  --inference-urls "p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE" \
  --workers 4 \
  --entity-types Person,Place,Organization,Legislation,Publication,Concept \
  --no-relations \
  --reset-graph \
  --graph-window 1 \
  --validation-model llama3.1:70b \
  --validation-floor 0.7

# 4. Re-seed YAML (always after graph build)
kwaainet rag graph seed --kb D6 --file tests/kwaai-knowledge/d6_family_tree.yaml

# 5. Score and inspect
kwaainet rag graph score --kb D6
kwaainet rag graph score --kb D6 --type Legislation  # inspect new type quality

# 6. Eval (expect q34/q15 to improve from Legislation type, q36 stable)
kwaainet rag eval --kb D6 \
  --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --mode smart --summary-expansion --biographical-expansion
```

---

## What this does NOT do (out of scope)

- **Relation extraction for 8b models** — still disabled; 0–17% precision unchanged
- **Retrieval-time entity type filtering** — types are for extraction quality, not query routing
- **Hardcoding D6 categories in Rust** — the schema YAML is the domain boundary
- **Changing chunk size or graph build parameters** — Phase 2 results stand
