# Plan: D6 RAG Accuracy Improvement — Phased Pipeline Upgrade

**Rename this file to `d6-rag-accuracy-improvement.md` after exiting plan mode.**

## Context

The D6 knowledge base has 466 clean Person entities (Phase 1 complete) but **zero relations**.
Without edges, `retrieve_graph_anchored()` BFS traversal does nothing — seed entities have no
neighbours to follow. The four worst-scoring eval questions share two root causes:

- **Missing edges**: q07 (wife: 0%) and q09 (grandfather: 11%) need `spouse_of`/`grandchild_of`
  edges. `resolve_author_relative()` at `retriever.rs:285` is written and waiting for them.
- **Thin descriptions**: q05 (J.M.H. Gool: 12%) and q20 (cricket: 20%) fail the quality gate
  in `inject_entity_descriptions()` at `retriever.rs:398` (≥40 chars, ≥1 sentence required).

Current baseline: **52.6% token-overlap recall, 1.80/2.0 judge score** (20-question D6 set).

**Template note**: This plan is written for D6 but is a template for any small memoir/biography KB.
Steps tagged `[corpus-specific]` require KB-specific content authoring. Steps tagged
`[system-general]` belong in the standard KwaaiNet RAG build runbook and apply to any KB.

---

## Ablation Protocol

Steps are not orthogonal (Phase 2 edges enable Step 5; descriptions enable Step 4), so true
factorial ablation is impractical. Instead: **incremental eval logging** at every step boundary.

```bash
kwaainet rag eval --kb D6 \
  --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --output tests/kwaai-knowledge/results/metrics.jsonl \
  --label "after-step-N"
```

Hold constant between measurements: same question set, same inference URL, same model.
**Decision rule**: if a step produces no measurable delta on its target question slice, stop and
diagnose before proceeding — don't stack changes on a broken foundation.

**Per-step metrics to track** (pre/post each step):
- Per-entity: `description_length`, `has_bio` (≥150 chars + ≥2 sentences), `num_relations`
- Per-relation-type: count and average `strength`
- Eval: `chunk_recall_at_{1,5,10}`, `entity_mrr`, judge score overall and per-type tag

**Mini-ablation after all steps**: run eval with reranker off, BFS weighting off, and injection
capped at 1 — to verify each step's independent contribution and support a future Dream RAG write-up.

**CI hook** (future): `kwaainet rag eval --kb D6 --quick` on a 10-question subset in CI must not
regress more than 5 points on judge score vs the last committed `metrics.jsonl` entry.
Commit a JSON summary of metrics after each milestone so the ablation timeline is reconstructable.

---

## Step 1 — Seed the Family Tree YAML  `[corpus-specific]`

**Complexity**: quick-win (YAML data + `kwaainet rag graph seed` CLI command, no code).

**What**: Create/update `tests/kwaai-knowledge/d6_family_tree.yaml` with curated ground-truth
canonical entities and relations. Seeded edges carry `evidence_chunk_id=0` (ground-truth anchor
flag) and survive `graph sanitize`. Later LLM-extracted edges for the same pairs merge onto them,
strengthening `strength` score.

**Gold-graph convention**: Every YAML entry must include `source: "curator_gold"` and a
`curator_note:` field explaining why this fact is ground-truth (e.g., "author states explicitly in
preface"). This distinguishes curated data from LLM-derived edges in all future analysis and
prevents silent drift between the seed file and corpus text.

**Minimum entries for D6**:
- Yousuf (Joe) Rassool `spouse_of` Nazima Rassool → fixes q07
- Yousuf (Joe) Rassool `grandchild_of` Joosub Maulvi Hamid Gooli → fixes q09
- Alias `"Grandfather"` on J.M.H. Gool's entry (Pass 1 of `resolve_author_relative()` at
  `retriever.rs:331` checks aliases before needing graph edges)
- Canonical names + descriptions for Cissie Gool, Abdul Hamid Gool, key Rassool children

**Framing**: Eval questions answerable only via gold-seed data are still valid. The YAML is the
human curator supplying essential context — analogous to a reference librarian annotating a
catalogue. Document this explicitly so future contributors don't interpret gold-seed-dependent
questions as "cheating."

**Eval gate**: Run eval after seeding; q07 and q09 should move. If they don't, inspect
`resolve_author_relative()` with debug logging before proceeding to Step 2.

**Generalisation note**: Every KB should have a seed YAML for known canonical entities and
relations. The *mechanism* (`family.rs:seed_family_tree()`, `source: curator_gold` convention) is
system-general and belongs in the KB setup checklist. The *content* is always corpus-specific.

---

## Step 2 — Phase 2: Relation Extraction  `[system-general]`

**Complexity**: CLI only (no code changes needed; hardened prompts already in `graph.rs:2887–3005`).

```bash
# 10% sample first — review before full run
kwaainet rag graph build --kb D6 --entity-types Person \
  --sample-pct 10 --workers 8 \
  --inference-urls "mux://12D3KooWK5j2ansdZevkb8PchZkoE92mwdjXJFkG1LvLZmf6sdrd,\
mux://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,\
mux://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE" \
  --model llama3.1:8b
kwaainet rag graph sanitize --kb D6
kwaainet rag graph export --kb D6 --output-dir tests/kwaai-knowledge/d6_p2_sample_export
# [Obsidian review] → full run only if clean
kwaainet rag graph build --kb D6 --entity-types Person --workers 8 --inference-urls "mux://..."
kwaainet rag graph dedup --kb D6 --auto-threshold 1.01
kwaainet rag graph sanitize --kb D6
kwaainet rag graph reembed --kb D6
```

**`--auto-threshold 1.01` note**: This constant skips all Tier 2 embedding-similarity merges
(nothing has similarity > 1.0). It exists because Tier 2 merges at the default 0.97 threshold
produced false positives on this KB (common surnames: Parker, Gool, Harry). Document this in
`graph.rs` dedup code as: *"1.01 = disable Tier 2; use when surname collisions outweigh
abbreviation merges. Revisit if KB grows and canonical names improve."*

**Relation trigger recall limitation**: String patterns ("wife", "son of", etc.) are conservative
and safe but will miss paraphrases ("joined in marriage", "their union", "their eldest"). This is
acceptable for Phase 2 — conservatism reduces hallucination. Log extraction misses during the
sample review as a curated "hard examples" file (`tests/kwaai-knowledge/d6_relation_hard_cases.md`)
for future Dream RAG relation-audit tasks.

**Review gate** (Obsidian acceptance criteria): No `spouse_of` between co-wives, no
`child_of → Dickens`, core Gool family tree correct. Run eval after full run; relation count
should be 300–600.

### Step 2b — Future: Relax Lexical Triggers  `[system-general]`

Once Phase 2 baseline is established and hard-example log exists:
- Option A: Domain-tuned transformer relation classifier trained on D6 hard examples
- Option B: LLM-disambiguated classifier with a richer paraphrase list but same ontological
  constraints (`PERSON_RELATION_TYPES`)

This is a research step for a future Dream RAG cycle, not part of the current operational plan.

---

## Step 3 — Dream Cycle: Description Enrichment  `[system-general]`

**Complexity**: CLI only (targeted run with completeness pre-filter).

**Pre-filter before running** — compute per-entity completeness and target only incomplete entities:

```bash
# Score the graph first; entities below threshold are dream targets
kwaainet rag graph score --kb D6
# Then run dream with a max-completions budget
kwaainet rag dream --kb D6 --max-completions 300 --workers 8 \
  --inference-urls "mux://..." --model llama3.1:8b
```

Dream targets entities where `description_length < 40` or `num_relations == 0` or
`overall_score < completeness_threshold` (0.6). Log how many entities crossed the threshold
after the run as the primary success metric for this step.

**Overwrite guard**: Dream must not overwrite non-empty curator bios from Step 1. The dream
`Biography` task at `dream_tasks.rs` should check `if entity.description.len() > 40 &&
entity.source == "curator_gold" { skip }`. Verify this guard exists before running.

**Post-run spot-check**: Randomly sample 10 enriched bios. Manually inspect for:
- Faithfulness to source chunks (description cites a specific verifiable fact)
- No hallucinated dates, places, or relationships
- No overwriting of Step 1 gold data

If any hallucinations are serious, add guardrails: require in-text citation evidence for each
added biographical fact before persisting.

**Q05 immediate fix**: Seed J.M.H. Gool's description directly in the Step 1 YAML
(`description:` field) — injected by `family.rs:seed_family_tree()` with no LLM call.

**Eval gate**: Run eval after dream; description-dependent questions (q05, q20) should move.
Also check that `has_bio` count increased and `description_length` distribution shifted right.

---

## Step 7 — Expand Eval Dataset  `[corpus-specific content / system-general methodology]`

**Do this before Steps 4–6** so you have a tagged question set that isolates each step's
contribution even in a sequential protocol.

**Question authoring**: Questions drafted by LLM (using entity descriptions as seed) but curated
and finalized by humans to avoid training-eval leakage, then frozen per version. Document the
version alongside each `metrics.jsonl` entry.

**Target**: 40 questions minimum (up from 20); plan to grow to 100+ over time.

**New categories** (with step-isolation tags in the JSON):
- `family_relation` — tests Step 1 seed + Step 2 edges + Step 5 BFS (q07, q09 type)
- `entity_description` — tests Step 3 dream (q05 type)
- `multi_entity` — tests Step 4 injection (q20 cricket type)
- `cross_entity` — tests Step 5 BFS ("who worked with Cissie Gool in the Teachers League?")
- `temporal_event` — tests general retrieval ("what happened to District Six in the 1960s?")
- `org_membership` — tests non-family graph edges ("which organisations did J.M.H. Gool lead?")

**Per-type metrics**: Report recall and judge score per category tag so regressions are visible
at a category level, not just as an overall average.

---

## Step 4 — Multi-Entity Description Injection  `[system-general]`

**Complexity**: easy (~30 lines, `retriever.rs:inject_entity_descriptions()` lines 398–515).

**Change**: Allow up to cap=3 synthetic chunks (up from 1). Selection heuristic:
1. Require description to pass the quality gate (≥40 chars, ≥1 sentence)
2. Among candidates, prefer **entity-type diversity** — don't inject three Person descriptions;
   prefer one Person + one Organization + one Location if available
3. If all candidates are the same entity type, pick the top 3 by description length
4. Score each synthetic chunk as `2.0 * clamp(desc_len / 300.0, 0.9, 1.1)` so description
   quality is lightly reflected in score (rather than a flat 2.0 for all)

**Eval gate**: Run eval before and after. If delta is negligible, description quality from Step 3
was insufficient — diagnose before proceeding to Step 5.

---

## Step 5 — Relation-Weighted BFS  `[system-general]`

**Complexity**: easy (2 files: `retriever.rs:214–243`, `graph.rs:879–899`).

**Changes**:
1. `bfs_neighbors(seed_ids, max_hops, rel_types: Option<&[&str]>)` — when `Some`, only traverse
   edges whose relation type is in the allowed list
2. Chunk score: `if is_seed { 1.0 } else { 0.6 * strength.clamp(0.1, 1.0) }` — tune the 0.6
   factor via grid search (see below)
3. Family-query detection: centralise as `fn is_family_query(query: &str) -> bool` with a word
   set (`["wife","husband","child","son","daughter","father","mother","grandfather","grandmother",
   "sibling","brother","sister","relative","family"]`). When true, pass
   `rel_types = Some(&["spouse_of","child_of","parent_of","grandparent_of","grandchild_of",
   "sibling_of"])` to avoid BFS drifting to `associated_with` political-figure edges

**Tuning plan**: Once Phase 2 edges exist, run a small grid search over `{0.4, 0.6, 0.8}` for
the neighbour score factor on the `family_relation` question slice. Pick the value that maximises
`entity_mrr` on that slice without degrading overall recall.

**Debug mode**: Add `--trace-bfs` flag that logs which entities and relation types were traversed
for each query to stderr. Use during dev and eval analysis; off by default in production.

**Eval gate**: Compare graph-anchored retrieval before/after on the `family_relation` and
`cross_entity` question slices specifically.

---

## Step 6 — Activate Listwise Reranker  `[system-general]`

**Complexity**: easy (wire existing `reranker.rs:rerank_chunks()` into `retrieve_iterative()`,
~10 lines, gate behind `use_reranker: bool` config flag, default false).

**Crossed evaluation** after Steps 1–5:
- Run eval with reranker on vs off, same graph and descriptions
- Report per-category (not just overall): `family_relation`, `entity_description`, `cross_entity`
- Check for per-category regressions — reranking can overfit to spurious patterns on some types
- Log initial top-k vs reranked top-k for a 5-question sample to inspect qualitative behaviour

**Latency trade-off**: Adds ~3–5s per query (one extra LLM call). If judge improvement is small
(< 5 points), leave disabled and revisit with a lighter reranker model.

---

## Execution Order

| Step | Action | Scope | Complexity | Eval Gate |
|------|--------|-------|------------|-----------|
| 1 | Seed family tree YAML | corpus-specific | quick-win | Run eval; q07/q09 must move |
| 7 | Expand eval dataset (do early) | corpus-content/general-method | data only | New tagged baseline committed |
| 2 | Phase 2 relation extraction | system-general | CLI only | Run eval; relation count 300–600 |
| 3 | Dream cycle enrichment | system-general | CLI only | Run eval; has_bio count up, q05 moves |
| 4 | Multi-entity injection | system-general | easy (~30 lines) | Run eval; if no delta, diagnose S3 |
| 5 | Relation-weighted BFS | system-general | easy (2 files) | Run eval on family_relation slice |
| 6 | Activate reranker | system-general | easy (wire existing) | Crossed eval with/without |

**Decision rule**: No measurable delta on the target question slice → stop and diagnose before
the next step. Don't stack improvements on a broken foundation.

---

## Files Modified

| File | Step | Change |
|------|------|--------|
| `tests/kwaai-knowledge/d6_family_tree.yaml` | 1 | Gold-standard entities + relations + `source: curator_gold` |
| `tests/kwaai-knowledge/d6_eval_questions.json` | 7 | 40+ questions with per-type tags |
| `tests/kwaai-knowledge/d6_relation_hard_cases.md` | 2 | Curated paraphrase misses for future 2b |
| `src/retriever.rs` `inject_entity_descriptions()` | 4 | N=3 diverse injections, score by desc quality |
| `src/retriever.rs` `retrieve_graph_anchored()` | 5 | Strength-weighted scores, family-query filter |
| `src/retriever.rs` `is_family_query()` | 5 | Centralised family keyword detection |
| `src/graph.rs` `bfs_neighbors()` | 5 | Optional `rel_types` filter param |
| `src/retriever.rs` `retrieve_iterative()` | 6 | Wire reranker post Round 2, behind flag |

No new crates. No schema changes. No breaking CLI changes (all new flags are optional).

---

## Verification

1. After Step 1: `kwaainet rag graph show --kb D6 "Yousuf (Joe) Rassool"` — confirm `spouse_of`
   Nazima Rassool, `grandchild_of` Joosub Maulvi Hamid Gooli; run eval
2. After Step 7: Commit new question set; confirm per-type tag coverage is adequate
3. After Step 2 full run: `kwaainet rag graph stats --kb D6` — 300–600 relations; run eval
4. After Step 3: Per-entity `has_bio` count up; spot-check 10 bios for hallucinations; run eval
5. After Steps 4–5: Run eval on `family_relation` and `multi_entity` slices
6. After Step 6: Crossed eval with/without `--use-reranker`; per-category breakdown; commit result

**Persistent eval record**: Commit a `metrics.jsonl` line after each milestone so the ablation
timeline is reconstructable. Include `label`, `step`, `timestamp`, `entity_count`,
`relation_count`, `recall_at_{1,5,10}`, `judge_score`, and per-type breakdowns.
