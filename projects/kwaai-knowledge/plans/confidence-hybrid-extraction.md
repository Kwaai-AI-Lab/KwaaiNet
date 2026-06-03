# Confidence-Scored Hybrid Extraction: CC First Pass → EC Refinement

## Context

Chunk-centric (CC) extraction is fast but produces thin entity records for people who appear
briefly or only in one chunk. Entity-centric (EC) extraction produces richer records but is
3.8× more expensive. The right approach: run CC over everything, score every extracted entity,
then escalate only the low-confidence ones to a targeted EC refinement pass.

The existing `EntityScore` / `score_entity()` in `scorer.rs` already computes a 0–1 completeness
score (type pillar + summary pillar + relation pillar). This is the confidence signal — no LLM
prompt changes needed. After CC completes, store the score on each `EntityNode`, filter to
entities below a threshold, and run focused EC calls for just those entities.

---

## What already exists and can be reused

| Existing asset | Location | Role in this plan |
|---|---|---|
| `score_entity(node, neighbor_rels)` | `scorer.rs` | Compute per-entity confidence after CC |
| `EntityNode.evidence: Vec<i64>` | `graph.rs:228` | Chunk IDs where entity appears — used to build EC context |
| `extract_from_text()` | `graph.rs` | Reused for EC refinement calls (single candidate + hint) |
| `clean_extracted_name()` | `ingestion.rs` | Applied to EC results too |
| `entity_centric` EC drain | `ingestion.rs` | Template for the refinement drain |
| `extract_entity_centric()` | `ingestion.rs` | Reference for aggregated-context building |

---

## Implementation

### Step 1 — Add `confidence: f32` to `EntityNode`

**File:** `core/crates/kwaai-rag/src/graph.rs`

Add one field to `EntityNode`:
```rust
#[serde(default)]
pub confidence: f32,   // 0.0 = not yet scored; updated after CC build
```

Default 0.0 keeps existing serialized nodes backwards-compatible (`#[serde(default)]`).

### Step 2 — Store confidence after CC build

**File:** `core/crates/kwaai-rag/src/ingestion.rs`, in `extract_and_store_entities_pub()`,
immediately before the final `drain_handle.await`.

After the spawn loop drains, lock the store and run one scoring pass:
```rust
if chunk_batch_mode {
    let mut g = store.lock().unwrap();
    let ids: Vec<i64> = g.all_entities().map(|n| n.id).collect();
    for id in ids {
        if let Some(node) = g.nodes.get(&id) {
            let neighbor_types: Vec<String> = g.neighbors_of(id)
                .iter().map(|(_,r,_)| r.clone()).collect();
            let score = kwaai_rag::scorer::score_entity(node, &neighbor_types);
            // We need a mutable reference now
        }
        if let Some(node) = g.nodes.get_mut(&id) {
            node.confidence = score.overall;
        }
    }
    // persist to DB
}
```

This scoring pass is fast (no I/O, no LLM): it's a pure in-memory computation over the already-loaded node graph. Do it unconditionally after every CC build (adds ~1ms for D6-scale graphs).

**Persist:** After updating `.confidence` in memory, call a new helper
`graph.flush_confidence_scores()` (or inline the redb write) to persist the updated nodes.
Alternatively, reuse `graph.reembed_entities()` as a template — it already does a batch redb
write of updated nodes.

### Step 3 — Add CLI flags

**File:** `core/crates/kwaai-cli/src/cli.rs`, `GraphAction::Build`

```rust
/// After CC build, escalate entities whose confidence score is below this
/// threshold to an entity-centric refinement pass. 0.0 = disabled (default).
#[arg(long, value_name = "THRESHOLD", default_value = "0.0")]
ec_refine_threshold: f32,

/// Max entities to escalate per run (guards against runaway cost).
#[arg(long, value_name = "N", default_value = "50")]
ec_refine_budget: usize,
```

Wire through to `GraphIngestConfig`:
```rust
pub ec_refine_threshold: f32,   // 0.0 = disabled
pub ec_refine_budget: usize,    // cap on EC escalations
```

### Step 4 — Implement `refine_low_confidence_entities()`

**File:** `core/crates/kwaai-rag/src/ingestion.rs` (new async function, ~100 lines)

Called from `extract_and_store_entities_pub()` after the confidence-scoring pass,
when `cfg.ec_refine_threshold > 0.0` and `cfg.gliner_client` is not required
(EC refinement uses evidence-chunk lookup, not GLiNER Phase 1).

```rust
async fn refine_low_confidence_entities(
    chunks: &[Chunk],
    chunk_ids: &[i64],     // parallel array: chunk_ids[i] is the DB id of chunks[i]
    embed: &EmbedClient,
    cfg: &GraphIngestConfig,
)
```

**Algorithm:**

1. Build `id_to_index: HashMap<i64, usize>` mapping chunk DB id → position in `chunks[]`.

2. Lock the store, collect entities where `confidence < cfg.ec_refine_threshold` and
   `entity_type` is in `cfg.entity_types` (or all types if empty). Sort ascending by
   confidence; take the lowest `cfg.ec_refine_budget`.

3. For each target entity:
   - Get `entity.evidence` (list of chunk DB ids from the graph store).
   - Map each evidence id to a chunk index via `id_to_index`. Skip ids not in the corpus
     (e.g., from an earlier ingest run on the same KB).
   - Take up to 3 distinct source indices (lowest, as in `extract_entity_centric`);
     expand each to a ±`context_window` window; deduplicate; concatenate with `\n\n[...]\n\n`.
   - Call `extract_from_text()` with `candidates = [entity.name]`, `hints = [entity.name]`,
     the entity type list, `no_relations = cfg.no_relations`.
   - Apply `clean_extracted_name()` + entity_type filter to results.
   - If any result entity passes: merge it into the existing node via `upsert_entity()`.
     Re-score and update `node.confidence`.

4. After the refinement loop, print summary:
   ```
   EC refinement: N escalated, M improved (confidence ↑ avg +0.XX)
   ```

**Matching EC entity back to the target:** The LLM might return the entity under a slightly
different name. Match on `entity_id(clean_name, type)` — if it matches the target's id,
it's the same entity. If it returns a new id, upsert separately (it's a new entity discovered
by the context; bonus recall).

**Entity-type filter:** Apply `entity_types_cfg` filter inside the refinement drain, same as
the CC drain does at line ~859 of rag_cmd.rs. Without this, non-Person entities could be
introduced into a Person-only run via EC dossier context.

**Refinement prompt:** Use a focused prompt that tells the LLM what it already knows:

```
You already have a record for the person named '{entity_name}'.
Here are additional text passages mentioning them.
Extract any new or corrected information: occupation, affiliation, nationality,
family relationships. Return ONLY the fields you can directly confirm from the text.
{"entities":[{"name":"...", "type":"Person", "fields":{...}}]}

Text:
{aggregated_context}
```

This is distinct from the CC prompt (which asks "what entities are here?") — the EC
refinement prompt says "you know this person exists; tell me more about them."

---

## Staged execution: 1% → review → 10%

### Stage 1 — 1% sanity check (12 chunks)

```bash
kwaainet rag graph build --kb D6 \
  --entity-types Person --no-relations --sample-pct 1 \
  --workers 4 --inference-urls http://localhost:11434 \
  --model llama3.1:8b --gliner-url http://127.0.0.1:9099 \
  --ec-refine-threshold 0.45 --ec-refine-budget 10
```

**What to check:**
- Every `EntityNode` has `confidence > 0.0` after build.
- `kwaainet rag graph score --kb D6` shows the same `overall` values as stored `confidence`.
- EC refinement log prints "N escalated, M improved" — inspect which entities were escalated
  and whether their field completeness actually improved.
- Manual spot-check: `kwaainet rag graph show --kb D6 "<low-confidence entity>"` before and
  after refinement to confirm richer description.

**Calibrate threshold first:** Before running refinement at 0.45, log the distribution of
`confidence` values across all ~40 entities from the CC-only build:
```bash
kwaainet rag graph score --kb D6 --top 50
```
Look at the histogram — if most entities cluster above 0.45, the threshold is too low
(over-escalates); if below, too permissive. Adjust before spending EC budget.

**Decision gate:** If ≥1 entity meaningfully improved (more fields filled, confidence ↑),
proceed to Stage 2. If EC produces no improvement or worsens entities, tune the
refinement prompt or threshold before scaling.

### Stage 2 — 10% full review (114 chunks)

```bash
kwaainet rag graph build --kb D6 \
  --entity-types Person --no-relations --sample-pct 10 \
  --workers 8 \
  --inference-urls "http://localhost:11434,mux://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs" \
  --model llama3.1:8b --gliner-url http://127.0.0.1:9099 \
  --ec-refine-threshold 0.45 --ec-refine-budget 50
```

**What to measure:**
- Entity count before vs after refinement.
- Recall@family_tree (% of `d6_family_tree.yaml` names found) — compare pure CC vs CC+EC.
- EC call count and wall-clock cost of the refinement pass.
- False positive rate on a 20-entity manual sample from the escalated set.
- Log the distribution of confidence scores pre-refinement to calibrate the threshold.

**Metrics to record in `tests/kwaai-knowledge/d6_experiments_log.md`:**
```
## YYYY-MM-DD – confidence_hybrid_10pct_v1
- CC entities: N_cc, avg confidence: X.XX
- Escalated: N_esc (confidence < 0.45), EC calls: N_calls
- Post-refinement: N_final entities, avg confidence: Y.YY
- FT recall: CC=A%, CC+EC=B% (+Cpp)
- EC cost: Ds elapsed
```

---

## Known limitations (note in implementation)

- EC entity-type leakage — add entity_type post-filter to the refinement drain
  (non-Person entities slip through; same fix needed in `extract_entity_centric()`).
- Model-verbalized confidence (optional future enhancement): add `"confidence": float` to
  the LLM prompt schema and blend into the score formula. Defer until structural scoring
  is validated on 1% and 10%.
- `entity.evidence` may be incomplete for entities extracted early before `link_chunk()` has
  run for all chunks — acceptable at these scales.
