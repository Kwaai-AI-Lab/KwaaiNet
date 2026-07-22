# Dream RAG — Reference Documentation

Dream is the autonomous graph-improvement loop in kwaai-knowledge. After initial ingestion and
graph extraction, entities vary in completeness: some have rich descriptions and relations, others
are stubs with just a name and type. Dream iterates over the lowest-scoring entities, enriches
them via LLM inference, deduplicates near-duplicates, prunes zombies, and fixes relation integrity
— all in one cycle that can be re-run until the graph score stabilises.

**Key design principles:**
- **Conservative acceptance** — LLM output is only applied if it strictly improves the entity (type not yet set, description gets longer/better-tiered, relation target already exists in graph)
- **Evidence-first** — the LLM receives the actual chunk text mentioning the entity, not just the entity name; completions are grounded in the source document
- **Load-balanced fan-out** — concurrent tokio tasks distribute work round-robin across multiple inference URLs; GPU relay machines work in parallel
- **Graceful degradation** — any LLM failure returns an empty completion; the cycle continues and records the error in the report

---

## 1. Component Diagram

Runtime components and their relationships.

```mermaid
graph TD
    CLI["rag_cmd.rs\n(cmd_dream)"]
    DC["dream.rs\n(run_dream_cycle)"]
    DT["dream_tasks.rs\n(task dispatch)"]
    GS[("GraphStore\n(redb — entities, relations,\nchunk↔entity index)")]
    MS[("MetaStore\n(redb — chunk text)")]
    EMB["EmbedClient\n(nomic-embed-text\nvia Ollama)"]
    INF["Inference Servers\n(Ollama via p2p relay\nor localhost)"]
    CH["mpsc Channel\n(fan-out / collect)"]
    RPT["dream-report-{tenant}.json"]

    CLI -->|"DreamConfig + URLs"| DC
    DC --> DT
    DC <-->|"score / read / write entities"| GS
    DC -->|"get_chunks(evidence_ids)"| MS
    DC -->|"embed_one after update"| EMB
    DT -->|"POST /v1/chat/completions"| INF
    INF -.->|EntityCompletion JSON| CH
    CH -.->|recv| DC
    DC -->|DreamReport| RPT
    CLI -->|progress callback| CLI
```

---

## 2. DFD Level 0 — Context Diagram

Dream as a black box: what flows in and what comes out.

```mermaid
graph LR
    KG[("Knowledge Graph\nEntityNodes · Relations\nchunk↔entity index")]
    CS[("Chunk Store\nChunkMeta · raw text")]
    CFG["DreamConfig\nthresholds · budget · workers"]
    INF["Inference URLs\np2p:// GPU relay"]
    KG2[("Updated Graph\nenriched entities\nmerged duplicates\npruned zombies")]
    RPT["dream-report-{tenant}.json\nscore before→after · counts · errors"]

    KG -->|"incomplete entities\n(score < threshold)"| D((Dream\nCycle))
    CS -->|"evidence text\nper entity"| D
    CFG -->|config| D
    INF -->|LLM completions| D
    D --> KG2
    D --> RPT
```

---

## 3. DFD Level 1 — 9-Step Cycle

Internal data flow through all nine steps. Steps 2–4 are the fan-out phase; they share one
numbered block because the tokio spawn, channel send, and channel receive are interleaved.

```mermaid
flowchart TD
    S1["**Step 1 — Score & Collect**\nscore_graph() over all entities\nfilter: score &lt; completeness_threshold\nsort ascending; take top max_completions\nbuild Vec&lt;WorkItem&gt; with evidence text"]

    S24["**Steps 2–4 — Fan-out LLM Completions**\nspawn N tokio tasks (semaphore: workers)\neach task calls complete_entity(evidence_text, url)\nURL selected by atomic round-robin index\nresult sent to mpsc channel\nmain loop recv() until all tasks finish"]

    S5["**Step 5 — Write Completions**\nfor each EntityCompletion:\n• schema_type → set only if currently None\n• fields → merge; recompute description;\n  re-embed(name + description)\n• relations → add only if target exists in graph"]

    S6["**Step 6 — Auto-Merge Duplicates**\nfind_dedup_candidates_exact() — same name\nfind_dedup_candidates(threshold) — cosine sim\nfind_dedup_candidates_name_structure() — honorifics\nauto-merge tiers 1–3; tier 4+ flagged only"]

    S7["**Step 7 — Prune Zombies**\ndelete if ALL true:\n• mention_count ≤ 1\n• no graph neighbours\n• no chunk evidence\n• overall_score &lt; prune_threshold"]

    S8["**Step 8 — Relation Integrity**\nsanitize_relations():\n• remove familial rels between non-Persons\n• remove same-gender spouse_of\n• add missing logical inverses\n• recompute strength from evidence count"]

    S9["**Step 9 — Final Score**\nscore_graph() → report.score_after\nwrite DreamReport JSON to data dir"]

    GS_R[("GraphStore\n(read)")]
    GS_W[("GraphStore\n(write)")]
    MS[("MetaStore\n(read)")]
    EMB["EmbedClient"]

    GS_R -->|"all entities + scores"| S1
    MS -->|"chunk text"| S1
    S1 -->|"Vec&lt;WorkItem&gt;"| S24
    S24 -->|"Vec&lt;EntityCompletion&gt;"| S5
    EMB -->|"re-embed updated description"| S5
    S5 -->|"updated entities"| GS_W
    S5 --> S6
    S6 -->|"merged entities"| GS_W
    S6 --> S7
    S7 -->|"deleted entities"| GS_W
    S7 --> S8
    S8 -->|"sanitized relations"| GS_W
    S8 --> S9
    GS_R -->|"all entities + edges"| S9
```

---

## 4. Entity Scoring

Scoring lives in `scorer.rs`. Each entity gets three independent pillar scores; `overall` is their
unweighted mean. The work queue in Step 1 targets entities with `overall < completeness_threshold`
(default 0.6), sorted worst-first.

### Pillars

| Pillar | Range | Logic |
|--------|-------|-------|
| **type_score** | 0.0 / 0.4 / 1.0 | 0.0 = Unknown (no schema type); 0.4 = `schema:Thing` (vague but valid); 1.0 = specific type |
| **summary_score** | 0.0 – 1.0 | Field-based if `expected_fields()` defined for type; otherwise description-length: empty=0.0, <50 chars=0.3, <150 chars=0.6, ≥2 sentences=1.0 |
| **relation_score** | 0.0 – 1.0 | Fraction of expected relation groups (from `expected_relation_groups()`) that have ≥1 match. Peripheral entities (mention_count ≤ 2) only need 1 group. `schema:Thing` / no expectations → neutral 0.5 |

```
overall = (type_score + summary_score + relation_score) / 3.0
```

### Expected relation groups by schema type (examples)

| Schema type | Group 1 | Group 2 | Group 3 |
|-------------|---------|---------|---------|
| `schema:Person` | family rels | works_at / founded / manages | located_in / associated_with |
| `schema:Organization` | located_in / belongs_to | founded / part_of / contains | — |
| `schema:Place` | located_in / contains / part_of | — | — |
| `schema:Event` | occurred_on / started / ended | located_in / associated_with | — |

---

## 5. Entity Relationship Diagram (ERD)

Data structures that Dream reads and writes. Arrows show foreign-key / containment relationships.

```mermaid
erDiagram
    EntityNode {
        i64     id                  PK  "sha256(lower(name)::type)[0..8]"
        string  name
        string  entity_type             "Person · Place · Organization · …"
        string  description             "prose summary or computed from fields"
        vec_f32 embedding               "768-dim nomic-embed-text vector"
        u32     mention_count
        i64     first_chunk_id      FK
        vec_str aliases                 "names merged into this entity"
        string  schema_type             "schema.org type (optional)"
        string  gender                  "Male · Female (optional)"
        vec_i64 evidence                "chunk IDs (populated at load)"
        f32     confidence              "structural completeness"
        f32     extraction_confidence   "LLM confidence at extraction"
    }

    FieldValue {
        string  value
        vec_i64 evidence_chunk_ids  FK
        f32     confidence              "min(1.0, evidence_count / 3.0)"
    }

    RelationRecord {
        i64     src_id              FK
        i64     dst_id              FK
        string  relation_type           "parent_of · spouse_of · works_at · …"
        f32     strength                "min(1.0, evidence_count / 10.0)"
        vec_i64 evidence_chunk_ids  FK
    }

    ChunkMeta {
        i64     id                  PK
        string  doc_name
        u32     chunk_index
        string  text                    "~100-word sentence-aligned chunk"
        string  surrounding             "adjacent context"
        u32     page_num
        string  section_name
        string  section_type
        bool    skip_extraction
    }

    WorkItem {
        i64     entity_id           FK
        string  name
        string  entity_type
        string  description
        string  evidence_text           "concatenated chunk text (doc context prepended)"
        u32     mention_count
        usize   chunk_count
        vec_i64 evidence_chunk_ids  FK
    }

    EntityCompletion {
        i64        entity_id         FK
        opt_string schema_type           "None if invalid or not improved"
        opt_string description           "None if no tier improvement"
        vec_tuple  relations             "(relation_type, target_name)[]"
        map        fields                "HashMap<String, FieldValue>"
    }

    DreamConfig {
        f32    completeness_threshold    "default 0.6"
        f32    dedup_threshold           "default 0.92"
        f32    prune_threshold           "default 0.3"
        usize  max_completions_per_cycle "default 50"
        usize  workers                   "default 4"
        bool   no_relations              "default false"
        u64    interval_secs             "default 3600 (daemon mode)"
    }

    DreamReport {
        string timestamp
        f64    duration_secs
        usize  entities_type_completed
        usize  entities_summary_completed
        usize  entities_relations_added
        usize  entities_merged
        usize  entities_pruned
        f32    score_before
        f32    score_after
        vec_str cycle_errors
    }

    EntityNode    ||--o{  FieldValue      : "fields map"
    EntityNode    ||--o{  RelationRecord  : "outgoing (src_id)"
    EntityNode    ||--o{  RelationRecord  : "incoming (dst_id)"
    EntityNode    ||--o{  ChunkMeta       : "evidence chunks"
    WorkItem      }o--||  EntityNode      : "wraps"
    WorkItem      }o--o{  ChunkMeta       : "evidence_chunk_ids"
    EntityCompletion }o--|| EntityNode    : "targets"
    EntityCompletion ||--o{ FieldValue    : "proposed field updates"
```

---

## 6. Sequence Diagram — One Full Dream Cycle

Interactions between all participants for a single `kwaainet rag dream run` invocation.
The fan-out phase (Steps 2–4) shows two parallel workers for illustration; real concurrency
is governed by the `--workers` semaphore.

```mermaid
sequenceDiagram
    actor       User
    participant CLI  as rag_cmd.rs
    participant DC   as dream.rs
    participant GS   as GraphStore
    participant MS   as MetaStore
    participant EMB  as EmbedClient
    participant INF1 as Inference URL 0
    participant INF2 as Inference URL 1
    participant CH   as mpsc Channel

    User ->> CLI : kwaainet rag dream run --kb D6 --workers 4
    CLI  ->> CLI : resolve p2p:// → TCP proxy addresses
    CLI  ->> DC  : run_dream_cycle(config, [url0, url1, …], model)

    rect rgb(240, 248, 255)
        Note over DC,GS: Step 1 — Score & Collect
        DC  ->> GS  : all_entities()
        GS -->> DC  : Vec<EntityNode>
        DC  ->> GS  : neighbors_of(id) per entity
        DC  ->> DC  : score_entity() → sort by overall asc
        DC  ->> MS  : get_chunks(evidence_ids) per entity
        MS -->> DC  : Vec<ChunkMeta>
        DC  ->> DC  : build WorkItem queue (top N = max_completions)
    end

    rect rgb(255, 248, 240)
        Note over DC,CH: Steps 2–4 — Fan-out (semaphore: workers)
        par Worker 0 (url0)
            DC  ->> CH  : spawn task[0]
            CH  ->> INF1: POST /v1/chat/completions (entity A)
            INF1 -->> CH: EntityCompletion JSON
        and Worker 1 (url1)
            DC  ->> CH  : spawn task[1]
            CH  ->> INF2: POST /v1/chat/completions (entity B)
            INF2 -->> CH: EntityCompletion JSON
        end
        DC  ->> CH  : recv() until all N results collected
        CH -->> DC  : Vec<EntityCompletion>
    end

    rect rgb(240, 255, 240)
        Note over DC,EMB: Step 5 — Write Completions
        loop for each EntityCompletion
            DC  ->> GS  : set schema_type (if currently None)
            DC  ->> GS  : merge fields (if new keys)
            DC  ->> EMB : embed_one(name + updated description)
            EMB -->> DC : Vec<f32> (768-dim)
            DC  ->> GS  : update entity (new embedding + description)
            DC  ->> GS  : add_relation (only if target entity exists)
        end
    end

    rect rgb(248, 240, 255)
        Note over DC,GS: Steps 6–8 — Maintenance
        DC  ->> GS  : find_dedup_candidates_exact()
        DC  ->> GS  : find_dedup_candidates(dedup_threshold)
        DC  ->> GS  : find_dedup_candidates_name_structure()
        DC  ->> GS  : merge(src, dst) for each candidate
        DC  ->> GS  : delete zombie entities
        DC  ->> GS  : sanitize_relations()
    end

    rect rgb(255, 255, 240)
        Note over DC,GS: Step 9 — Final Score
        DC  ->> GS  : score_graph()
        GS -->> DC  : GraphHealthReport
        DC ->> DC   : write dream-report-{tenant}.json
        DC -->> CLI : DreamReport
    end

    CLI -->> User : score 72.3% → 81.6% · 38 completions · 4 merged · 2 pruned
```

---

## 7. LLM Completion Call — What the Model Receives and Returns

Each `complete_entity()` call posts to `/v1/chat/completions` (OpenAI-compatible, temperature=0.25,
max_tokens=700). The prompt structure:

```
[system]
You are a knowledge-graph enrichment assistant. Given entity information and source text,
output a JSON object with schema_type, description, relations, and fields.

[user]
ENTITY: {name} ({entity_type})
CURRENT DESCRIPTION: {description}
CURRENT SCHEMA TYPE: {schema_type or "none"}
CURRENT FIELDS: {fields as key: value}

DOCUMENT CONTEXT:
{doc_name} — {section_name} — {section_type}
---
EVIDENCE TEXT:
{evidence_text}  ← actual chunk text from source document

Return JSON: {schema_type, description, relations: [{type, target}], fields: {key: value}}
```

**Validation before acceptance:**

| Output field | Accepted if |
|--------------|-------------|
| `schema_type` | In the 14-type whitelist AND entity currently has `None` |
| `description` | Moves to a higher length tier OR is ≥20 chars longer in the same tier |
| `relations[]` | `relation_type` in `RELATION_TYPES`; target entity already exists in graph; `no_relations=false` |
| `fields{}` | Any new key-value pair; description recomputed from `description_from_fields()` after merge |

**Schema type whitelist (14 types):** Person, Organization, Place, Event, Product, CreativeWork,
SoftwareApplication, DefinedTerm, HowTo, Role, QuantitativeValue, Statement, Date, Thing

---

## 8. Deduplication Strategy

Three candidate-finding strategies run in sequence; all auto-merge in the same pass:

| Tier | Strategy | Auto-merge? |
|------|----------|-------------|
| 1 — Exact | Same canonical name (case-insensitive) | Yes |
| 2 — Fuzzy | Entity embedding cosine similarity > `dedup_threshold` (default 0.92) | Yes |
| 3 — Structural | Honorific variant ("Dr. Abdulla" ↔ "Abdulla") or name subset | Yes |
| 4+ — Subset/fuzzy overlap | Partial name inclusion, lower similarity | Flag only — manual review |

On merge: the lower-score entity's chunks, aliases, relations, and fields are transferred to the
higher-score entity; the source entity is deleted.

---

## 9. Configuration Reference

### `DreamConfig` / `kwaainet rag dream run` flags

| Flag | Default | Effect | Tuning guidance |
|------|---------|--------|-----------------|
| `--threshold` | 0.6 | Entities with `overall < threshold` become candidates | Lower → more work per cycle; raise if cycle takes too long |
| `--dedup-threshold` | 0.92 | Cosine similarity above this triggers auto-merge | Raise to be more conservative; 0.95+ rarely auto-merges |
| `--prune-threshold` | 0.3 | Score below this + isolated → deleted | Lower to keep more stubs; raise to prune aggressively |
| `--max-completions` | 50 | Total LLM budget for the cycle | Increase for a bigger single-pass improvement |
| `--workers` | 4 | Max concurrent inference tasks (semaphore) | Match to `OLLAMA_NUM_PARALLEL` on the GPU machine |
| `--no-relations` | off | Disable relation extraction | **Recommended on** for 8B models (precision 0–17%) |
| `--relation-summary` | off | Cross-cutting mode (see below) — replaces threshold-based selection | Use for a one-time comprehensive resummarization pass |
| `--inference-urls` | config | Comma-separated `p2p://` URLs | Use all available GPU relays for parallel load balancing |
| `--model` | default | Ollama model name | `llama3.1:8b` is the tested model |

### `--relation-summary` mode

A cross-cutting alternative to the score-threshold selection in Step 1, added for "resummarize
every well-connected entity from all its evidence, not just a capped sample." When set:

- **Selection**: every entity with `neighbors_of(id).len() >= 1`, **excluding** YAML-seeded
  entities (`extraction_confidence >= 1.0` — their descriptions are curated ground truth and must
  never be auto-resummarized). Sorted by relation count descending, so the most well-connected
  entities are covered first within `--max-completions`.
- **Evidence**: every chunk associated with the entity, uncapped (the normal path caps at 20
  chunks per entity) — `dream_tasks::run_full_summary_task` (`DreamTaskKind::FullSummary`).
- **Map-reduce**: chunks are grouped into ~6 000-char batches; each batch is summarized down to
  only the facts about the entity ("map"); if more than one batch exists, the batch summaries are
  combined into one final description ("reduce").
- **Write-back**: the resulting description always replaces the existing one
  (`EntityCompletion.force_description = true`), bypassing both the normal "must move up a summary
  tier or be +20 chars longer" gate and field-derived-description precedence — the whole point is a
  comprehensive resummarization, not an incremental nudge. Schema_type/relations/fields are never
  touched by this task, to avoid handing the LLM a large concatenated blob alongside a free-choice
  relation ask (the same hallucination risk pattern documented elsewhere in this codebase for
  large-context relation prompts).

---

## 10. Pipeline Integration Map

Where Dream sits in the full kwaai-knowledge pipeline:

```
kwaainet rag ingest          ← PDF → chunks → embeddings → MetaStore + HNSW index
         ↓
kwaainet rag graph build     ← LLM entity/relation extraction per chunk → GraphStore
         ↓
kwaainet rag graph score     ← EntityScore per entity; print worst offenders
         ↓
kwaainet rag dream run  ×N   ← iterative enrichment until score stabilises
         ↓
kwaainet rag graph seed      ← optional: inject trusted family-tree YAML
         ↓
kwaainet rag eval            ← score against eval questions
```

**Dream reads (never modifies):**
- MetaStore — chunk text and metadata
- HNSW vector index — chunk embeddings (entity embeddings are stored separately in GraphStore)

**Dream modifies:**
- GraphStore — entity fields, embeddings, schema types, descriptions, relations
- `dream-report-{tenant_id}.json` — written to the data directory after every cycle

**Dream does not touch:**
- Timeline events (`TIMELINE_TABLE`) — managed by `kwaainet rag graph timeline build`
- Sequence interactions (`INTERACTION_TABLE`) — extracted during graph build
- Chunk text / page numbers — read-only; never re-chunked by dream

---

## 11. Key Source Files

| File | Role |
|------|------|
| `core/crates/kwaai-rag/src/dream.rs` | `run_dream_cycle()`, `complete_entity()`, `DreamConfig`, `DreamReport`, `WorkItem`, `EntityCompletion` |
| `core/crates/kwaai-rag/src/dream_tasks.rs` | Task-type dispatch: Biography (Person), General; constructs task-specific prompts |
| `core/crates/kwaai-rag/src/scorer.rs` | `score_entity()`, `score_graph()`, `EntityScore`, `GraphHealthReport`, `expected_relation_groups()` |
| `core/crates/kwaai-rag/src/graph.rs` | `GraphStore`, `EntityNode`, `RelationRecord`, `FieldValue`; all graph read/write |
| `core/crates/kwaai-rag/src/meta_store.rs` | `MetaStore`, `ChunkMeta`; chunk text storage |
| `core/crates/kwaai-rag/src/embedder.rs` | `EmbedClient`; `embed_one()`, `embed_batch()` |
| `core/crates/kwaai-cli/src/rag_cmd.rs` | `cmd_dream()`: URL resolution, progress callback, report printing |
