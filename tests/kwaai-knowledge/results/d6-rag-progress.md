# D6 RAG Accuracy Improvement — Progress Log

Plan: [`projects/kwaai-knowledge/d6-rag-accuracy-improvement.md`](../../../projects/kwaai-knowledge/d6-rag-accuracy-improvement.md)  
Metrics: [`results/metrics.jsonl`](metrics.jsonl)

---

## Eval Timeline

| Label | Step | Date | Entities | Relations | Questions | Recall | Judge | Notes |
|-------|------|------|----------|-----------|-----------|--------|-------|-------|
| baseline-20q | 0 | 2026-05-29 | 466 | 0 | 20 | 52.6% | 1.80/2 | Pre-seed, no relations, no descriptions |
| after-step-1-seed-no-desc | 1 | 2026-06-02 | 463 | 140 | 20 | 54.3% | 1.60/2 | Family tree seeded, no descriptions |
| after-step-1-descriptions | 1 | 2026-06-02 | 463 | 140 | 40 | 56.9% | 1.73/2 | 9 entity descriptions added |
| after-step-2-relations | 2 | 2026-06-02 | 1325 | 2783 | 40 | 56.4% | 1.65/2 | **REGRESSION** — LLM relation extraction |

---

## Step 0 — Baseline (2026-05-29)

- 466 entities, 0 relations, 0 descriptions
- 20-question eval: **52.6% recall, 1.80/2 judge**
- Eval file: `eval_round6_20260529` (external)

---

## Step 1 — Family Tree Seed + Descriptions (2026-06-02)

### 1a. Family tree YAML seeded

- File: `tests/kwaai-knowledge/d6_family_tree.yaml`
- 34 persons, 71 relations → 70 planted (1 duplicate skipped)
- 140 bidirectional edges in graph
- Entity count dropped from 466 → 463 (alias merges during seed)
- 20q eval post-seed: **54.3% recall, 1.60/2 judge**

### 1b. Descriptions added for 9 key entities

Entities updated with `source: curator_gold` descriptions:
J.M.H. Gool, Nazima Rassool, Yousuf (Joe) Rassool, Cissie Gool, Abdul Hamid Gool,
Abdurahman, Bibi Gool, Wahida Gool, Gandhi

- Eval expanded to 40 questions
- **40q eval: 56.9% recall, 1.73/2 judge**
- Eval file: `eval_after_step1_with_descriptions_20260602.md`

### Key findings from Step 1 eval

| Question | Before | After | Status |
|----------|--------|-------|--------|
| q07 (wife) | 0/3 kw, 1/2 j | 2/3 kw, 2/2 j | ✅ FIXED |
| q09 (grandfather) | — | 2/9 kw, 2/2 j | ✅ Judge passes |
| q26 (Abdurahman) | — | 6/6 kw, 2/2 j | ✅ Description injection working |
| q27 (Gandhi/Gool) | — | 5/5 kw, 2/2 j | ✅ Working |

**Known bug after Step 1:** q05, q12, q24, q30, q32 inject wrong Gool-family entity
(`Bibi Gool` or `Wahida Gool` instead of `J.M.H. Gool` / `Cissie Gool`).
Cause: "Gool" token matches multiple entities; wrong one ranks highest.

---

## Step 7 (Early) — Expanded Eval to 40 Questions (2026-06-02)

Added 20 new questions covering:
- `family_relation` — tests seed + BFS (q07, q09 type)
- `entity_description` — tests dream enrichment (q05 type)
- `multi_entity` — tests injection cap (q20 cricket type)
- `cross_entity` — tests BFS traversal
- `temporal_event` — tests general retrieval
- `org_membership` — tests non-family graph edges

---

## Step 2 — LLM Relation Extraction (2026-06-02)

### 2a. 10% sample run

- 114 chunks processed, llama3.1:8b, 3-type entities
- Sample precision assessment: ~65% overall (parent/child ~85%, spouse_of ~55%)
- Above 50% gate → proceeded to full run
- Known false positives: `Dullah Omar ↔ Hans Friederichs` (political co-occurrence, not spouse)

### 2b. Full run

- 1136 chunks, ~113 min on metro-linux (A6000) + metro-win (A5000)
- Raw output: 1405 entities, 2719 relations
- After dedup (--auto-threshold 1.01, Tier 2 disabled to avoid surname collisions):
  77 merges → **1328 entities**
- After sanitize (removed same-gender spouse_of, parent_of paradoxes):
  **1325 entities, 2783 relations**
- After reembed: complete

### 2c. Step 2 eval result — REGRESSION

- **40q eval: 56.4% recall, 1.65/2 judge** (down from 56.9%, 1.73/2)
- Eval file: `eval_after_step2_20260602.md`

| Question | Step 1 | Step 2 | Change |
|----------|--------|--------|--------|
| q26 (Abdurahman) | 6/6 kw, 2/2 j | 3/6 kw, 1/2 j | ❌ Regressed — wrong entity injected |
| q27 (Gandhi/Gool) | 5/5 kw, 2/2 j | 2/5 kw, 2/2 j | ❌ Regressed |
| q07 (wife) | 2/3 kw, 2/2 j | 2/3 kw, 2/2 j | ✅ Held |
| q35 (Hassen Mall) | — | 3/4 kw, 2/2 j | ✅ Improved |

**Root cause:** Entity pool tripled (463 → 1325). Wrong entities now outrank correct ones
in injection. `[Graph: Goolam Gool]` injected for q26 (Abdurahman) and q25 (I.B. Tabata).

---

## Decision: Backtrack — Perfect Dedup Before Relations (2026-06-02)

**Rationale:** Relations cannot help if the wrong entity is being injected. The entity pool
must be clean and dedup must be sound before relation extraction adds value.

### Archived graph

Current graph (1325 entities, 2783 relations) archived to:
`tests/kwaai-knowledge/archive/d6_graph_step2_20260602/` (Obsidian export, 1325 entity files)

Can be re-imported: `kwaainet rag graph import --input-dir tests/kwaai-knowledge/archive/d6_graph_step2_20260602 --kb D6`

### Revised plan

1. **Reset → entity-only rebuild** (`--no-relations`, same optimal settings) — IN PROGRESS
2. **Audit dedup quality** — inspect top similar pairs, tune Tier 2 threshold within-type
3. **Re-seed family tree** with `d6_family_tree.yaml`
4. **Implement Step 4** (injection cap N=3 + entity-type diversity) — establish clean baseline
5. **Re-attempt relation extraction** only after entity pool is clean and injection works

---

## In Progress

- `graph build --kb D6 --no-relations --reset-graph --graph-window 1 --workers 4 --model llama3.1:8b` started 2026-06-02 ~16:25 PDT
  — Expected: ~40 min, ~1136 chunks, target ~463 entities

---

## Reference

### Optimal build settings (confirmed 2026-05-21 + Step 1)

```bash
kwaainet rag graph build \
  --kb D6 \
  --entity-types Person,Place,Organization \
  --no-relations \
  --reset-graph \
  --graph-window 1 \
  --workers 4 \
  --inference-urls "mux://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,mux://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE" \
  --model llama3.1:8b
```

### Dedup notes

- `--auto-threshold 1.01` disables Tier 2 embedding merges (avoids Gool/Parker/Harry surname collisions)
- Next attempt: try 0.93–0.95 within-type-only merging and inspect top-50 similar pairs first
- Sanitize removes: same-gender `spouse_of`, reversed `parent_of`, type mismatches, stub honorific entities

### Remote inference

- metro-linux: `mux://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` (A6000)
- metro-win: `mux://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE` (A5000)
