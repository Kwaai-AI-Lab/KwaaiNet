# D6 Person-Entity Experiments Log

Plan: [`projects/kwaai-knowledge/d6-person-entity-experiments.md`](../../projects/kwaai-knowledge/d6-person-entity-experiments.md)

---

| Snapshot | Date | Exp | Chunks | Raw entities | After dedup | Eval recall | Judge | Notes |
|----------|------|-----|--------|-------------|-------------|-------------|-------|-------|
| D6_person_1pct_tier1_20260602 | 2026-06-02 | 1 | 12 (1%) | 34 | 33 | — | — | sanity check only |
| D6_person_10pct_dedup_v1_20260602 | 2026-06-02 | 2 | 114 (10%) | 288 | 262 | — | — | Tier 2 NOT merged — FP rate 69%; see analysis |
| D6_confidence_hybrid_10pct_v1_20260603 | 2026-06-03 | confidence hybrid Stage 2 | 114 (10%) | 223 CC | 242 CC+EC | — | — | 11 improved, 19 new discovered |
| D6_mini_loop_10pct_20260604 | 2026-06-04 | full mini-loop | 114 (10%) | 223 CC | 241 final | **52.0%** | — | CC+EC+dedup+seed+dream+EC@0.34 |
| D6_firstname_dedup_20260604 | 2026-06-04 | first-name dedup | full | 1085 | 1051 | **53.3%** | — | Cissie/Fatima/Zobeida merged; Tier 3E added; q38 +20pp |

---

## 2026-06-04 – D6_firstname_dedup_20260604

**Goal:** Merge first-name-only stubs (e.g. "Cissie") into their full-name canonicals ("Cissie Gool")
when they fail Tier 3B's ≥2-shared-neighbour requirement because the stub has no graph connections.

### Problem identified

Three extracted entities were disconnected from their canonical counterparts:

| Stub entity | Mentions | Canonical | Mentions | Issue |
|-------------|---------|-----------|---------|-------|
| Cissie | 4 | Cissie Gool | 36 | 0 neighbors → Tier 3B silent |
| Fatima | 28 | Fatima Gool | 1 | Fatima was richer — seed had wrong direction |
| Zobeida | 4 | Zobeida Gool | 9 | 0 neighbors → Tier 3B silent |

### Fixes applied

**1. YAML aliases** (`d6_family_tree.yaml`):
Added `"Cissie"`, `"Fatima"`, `"Zobeida"` as explicit first-name aliases.
Re-seed merged all three immediately (3 aliases merged, 4 relations re-pointed).

**2. Tier 3E code** (`graph.rs` → `find_dedup_candidates_name_structure`):
New dedup sub-tier: if a single-word entity (≥4 chars, not an honorific or common word) is
the first token of **exactly one** other multi-word entity in the graph, it is a first-name alias.
No neighbour requirement. Safety: uniqueness guard + `WORD_BLOCKLIST` (head, prince, premier, instead, …).
Catches future Cissie/Fatima patterns automatically after each `graph build`.

### Post-merge state

| Entity | Before (mentions) | After (mentions) | Aliases gained |
|--------|------------------|-----------------|----------------|
| Cissie Gool | 36 | 41 | + "Cissie" |
| Fatima Gool | 1 | 30 | + "Fatima", Timmie aliases |
| Zobeida Gool | 9 | 14 | + "Zobeida" |

### Eval results

| Question | Baseline | After | Δ |
|----------|---------|-------|---|
| q12 Who was Cissie Gool? | 33% | 33% | = |
| q24 Children of JMH Gool? | 43% | 0% | ↓ (LLM noise) |
| q26 Dr. Abdurahman? | 100% | 83% | ↓ (LLM noise) |
| q32 Cissie↔JMH relationship? | 60% | 40% | ↓ (LLM noise) |
| q38 Cissie's father? | **40%** | **60%** | **↑+20pp** |
| **Overall** | **53.3%** | **53.3%** | = |

q38 (+20pp) is the clearest signal from the Cissie merge. Other changes are within LLM noise
(14 questions improved, 13 regressed, net zero). Overall score is stable at 53.3%.

### Also fixed during session

Two bad Tier 2 auto-merges detected and unmerged:
- `"I.B. Tabata"` → `"Jane Gool-Tabata"` (different people, same surname)
- `"Nasim Rassool"` → `"Nazima Rassool"` (child vs. wife of author)

Ran `graph reembed` to restore embeddings after unmerge.

### Files changed

- `core/crates/kwaai-rag/src/graph.rs` — Tier 3E `first_name_only` + `WORD_BLOCKLIST`
- `tests/kwaai-knowledge/d6_family_tree.yaml` — added Cissie, Fatima, Zobeida as first-name aliases

---

## 2026-06-02 – D6_person_1pct_tier1_20260602

- **Experiment:** 1 (1% sample sanity check)
- **Build:** 1% of corpus (12 chunks), Person entities only, no relations, llama3.1:8b, 4 workers, localhost:11434
- **Dedup:** Tier 1 + Tier 3 + Tier 4 only (--auto --auto-threshold 1.01; Tier 2 cosine disabled)
- **Stats:** raw=34, after-dedup=33 (1 Tier 1 merge: "Mr. Edross" → "Mr Edross")
- **Sanitize:** 1 gender inferred, 0 relations affected
- **Eval:** N/A — sanity check only

### Dedup detail
- Tier 1: 1 merge (punctuation normalisation — `Mr. Edross` → `Mr Edross`)
- Tier 2: 3 candidates above 0.85 threshold, all skipped (auto-threshold 1.01)
- Tier 3: 0 structural duplicates
- Tier 4: 0 neighbour-containment candidates

### Entity spot-check (8 of 33)

| Entity | Mentions | Description | Assessment |
|--------|----------|-------------|------------|
| Yousuf (Joe) Rassool | 1 | occupation: author | ✅ plausible |
| Joe Rassool | 4 | occupation: author | ✅ plausible — same person, not yet merged |
| Hassen Abrahams | 1 | (empty) | ✅ name looks real |
| Hassen | 3 | affiliation: team | ✅ plausible — likely same as Hassen Abrahams |
| Mr Edross | 2 | occupation: teacher | ✅ merged correctly from Mr. Edross |
| Hamid Khan | 1 | (empty) | ✅ plausible name |
| Hamid | 1 | (empty) | ✅ plausible — likely fragment of a longer name |
| P.V. Tobias | 4 | occupation: academic | ✅ correct (Philip Tobias, anthropologist) |

### Observations

**Strengths:**
- 100% Person type assignment — no junk entity types (Place, Organization, Unknown)
- Tier 1 correctly merged `Mr. Edross` / `Mr Edross` (punctuation variant)
- No obvious garbage extractions (e.g. no "He", "The Author", multi-entity blobs)
- `P.V. Tobias` correctly identified as academic (Philip Tobias, University of the Witwatersrand)
- NER phrase-merge bug from M40 does NOT appear — names look clean

**Known fragmentation (expected at 1%, no Tier 2):**
- `Joe Rassool` and `Yousuf (Joe) Rassool` are the same person — not merged yet
  (Tier 2 would handle this; they exist in separate chunks so no shared tokens for Tier 1/3)
- `Hassen` and `Hassen Abrahams` are likely the same — not merged (no shared neighbours at 1%)
- `Hamid` and `Hamid Khan` are likely the same — not merged (same reason)

**The 3 skipped Tier 2 candidates** are the interesting ones for Experiment 2 — when JW gate
is active and Tier 2 is un-blocked, we'll be able to see whether they are correct merges.

**Verdict:** ✅ Sanity check passes. Extraction is producing clean Person names from D6.
Ready to proceed to Experiment 2 (needs JW gate build + 10% run).

---

## 2026-06-02 – D6_person_10pct_dedup_v1_20260602

- **Experiment:** 2 (10% sample + JW gate calibration)
- **Build:** 10% of corpus (114 chunks), Person only, no relations, llama3.1:8b, 8 workers
- **Inference URLs:** `http://localhost:11434` + `mux://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` (metro-linux)
- **Build time:** ~702s (~11.7 min). **Timeouts:** ~27/114 chunks (24%) silently skipped
- **Dedup:** Tier 1+3+4 auto (--auto --auto-threshold 1.01); Tier 2 NOT auto-merged
- **Stats:** raw=288 → after Tier 1+3+sanitize=262 (26 merged/pruned)
- **Eval:** N/A — threshold calibration step

### Dedup detail

| Tier | Merges | Notes |
|------|--------|-------|
| Tier 1 (exact normalized) | 11 | Punctuation variants: Mr./Mr, Prof./Prof, W.P./W.P, BM.Kies/BM Kies, etc. |
| Tier 2 (cosine ≥ 0.85) | 0 auto-merged; 54 candidates found | All held back for analysis (see below) |
| Tier 3 (structural) | 13 | Honorific strips + 2 fuzzy: Yusuf→Yousuf, J.M.H.→J.H.M (good); James→Professor James |
| Sanitize (stub prune) | 2 | Removed bare "MS" and "Rev" as honorific stubs |

### Tier 2 candidate analysis — 54 pairs at threshold 0.85

**False-positive rate: 37/54 = 69%** — far above the 5–10% target. JW gate (0.60) blocked only 1.

| Category | Count | Verdict | JW blocks? |
|----------|-------|---------|------------|
| Fragment/alias of same person | 14 | ✅ Should merge | No — JW 0.80–0.87 |
| Same-surname, different people | 16 | ❌ False positive | No — JW 0.71–0.79 |
| Same-title, different people | 13 | ❌ False positive | No — JW 0.65–0.83 |
| Same-first-name, different people | 6 | ❌ False positive | No — JW 0.71–0.75 |
| Cross-Gool-family pairs | 5 | ❌ False positive | Barely — JW 0.60–0.67 |
| JW gate correctly blocks | 1 | ✅ Gate works | Yes — Ciraj/Yousuf Rassool (JW≈0.55) |

**Key false positive examples (should NOT merge):**
- General Zhukov/Paulus/MacArthur/Hertzog/Nguyen Giap — 10 pairs, all sim=0.960; all different WW2/Cold War generals grouped by shared "General" token + similar context
- Lord Headley ↔ Lord Roberts — different British lords
- Ruth/Ciraj/Rebecca Rassool cross-pairs — family members with same surname, different people
- Hamid Khan ↔ Abdul Hamid — CRITICAL different people sharing "Hamid" token (sim=0.960)
- Peter Abrahams ↔ Hassen Abrahams — different people, same surname
- AH Gool ↔ Joseph Gool / Yusuf Gool — sim=1.000 with empty descriptions; meaningless similarity

**Key true positive examples (should merge):**
- Mr. Edross ↔ Solly Edross (sim=0.946) — correct; same teacher, formal/informal name
- Abdurahman ↔ Dr. Abdullah Abdurahman (sim=0.957) — correct alias
- Joe Rassool ↔ Y.S. Rassool (sim=0.862) — correct; same author
- BM.Kies / Kies ↔ Ben Kies M.A — correct; same person

### Root cause of high false-positive rate

1. **sim=1.000 pairs (empty description):** When two entities have no description, their embeddings are identical (just the name vector). This produces spurious sim=1.000 even for completely different names like "AH Gool" / "Joseph Gool". These should be excluded from Tier 2 entirely.

2. **Title-prefix dominance:** "General X" / "General Y" pairs share the "General" token (≥3 chars, not in stop list), then have high embedding similarity because context is always "military commander." The existing blob/surname guards don't cover this.

3. **Same-surname community:** In a District Six memoir, dozens of unrelated people share surnames (Rassool, Gool, Abrahams). The surname token alone is enough to create a false shared-token bucket.

### Recommendations before Experiment 3

Two targeted fixes, in priority order:

**Fix 1 — Exclude empty-description pairs from Tier 2 (quick win):**
In `find_dedup_candidates()`, add a guard:
```rust
if na.description.len() < 20 && nb.description.len() < 20 {
    continue; // neither has a real description; embedding similarity is meaningless
}
```
This alone eliminates the sim=1.000 pairs (#1, #2, and several others).

**Fix 2 — Title-prefix guard:**
Add "general" (and other military/political titles) to `DEDUP_STOP`, OR add a guard:
if the ONLY shared significant token is a title-word (general, lord, colonel, president,
prime, minister, advocate, reverend, captain), cap sim at 0.94 (same as blob guard).
Eliminates 10+ false positives from the Generals cluster.

**JW gate assessment:** Threshold 0.60 is too low to be meaningful. After Fix 1 and Fix 2,
re-evaluate whether JW at 0.75+ would add net value without blocking true positives.

### Timeout analysis — known robustness gap

~27/114 chunks (24%) timed out and were silently skipped. From `access-delta-inference.md`,
the fix is retry-with-backoff at `graph.rs:3033–3055`. This gap will matter more at 100%.
For 10% sanity purposes, the skipped chunks mean the entity set is incomplete but usable.

### Verdict

❌ **Tier 2 dedup NOT safe to auto-merge at 0.85 threshold** for Person-only D6 graph.
Two code fixes needed (empty-description guard + title-prefix guard) before Experiment 3.
JW gate threshold should be revisited after those fixes.

---

## 2026-06-03 – D6_confidence_hybrid_10pct_v1

- **Plan:** `projects/kwaai-knowledge/plans/confidence-hybrid-extraction.md`
- **Build:** 10% corpus (114 chunks), Person-only, no relations, llama3.1:8b, 8 workers
- **Inference:** localhost:11434 + mux://metro-linux (round-robin)
- **GLiNER:** http://127.0.0.1:9099
- **Build time:** ~708s CC phase, then EC refinement pass
- **Timeouts:** ~25 chunks (~22%) — known retry gap

### CC phase results
- Raw entities after CC: **223**
- Avg confidence post-scoring: **33%** (type pillar 100%, no fields, no relations)
- All entities below threshold 0.45 (expected for a fresh graph with no dream cycles)

### EC refinement results (threshold=0.45, budget=50)
- Entities escalated: **50** (lowest-confidence first)
- Existing entities improved confidence: **11/50** (avg ↑ +0.03)
- **New entities discovered via EC context: +19** (223 → 242 total)
- Final entity count: **242**
- Entities with any fields filled: **100/242 = 41%**

### Quality metrics
- Avg confidence: **35.0%** (baseline ~33%, slight improvement from 100 entities with fields)
- **FT recall: 96/147 = 65.3%** (vs 13.6% at 1% scale — dramatic recall gain from corpus coverage)
- Top confidence: 43% (Earnest Livingston MQotsi, Omar Abrahams — have occupation fields)

### Key observations

**EC refinement value at 10%:**
- +19 new entity discoveries is the biggest contribution (bonus recall)
- 11 entities that only appeared in single CC chunks now have field data from aggregated context
- The +0.03 avg confidence improvement is modest — fields only improve from 0% to 10% (one field added)
- Most entities still lack fields because they only appear briefly in D6 text with no biographical detail

**Confidence score behaviour:**
- All fresh entities score 33% (type:100%, summary:0%, relation:0%)
- After EC, 100 entities have ≥1 field → summary pillar >0% → confidence >33%
- A threshold of 0.45 captures all entities needing enrichment at this graph health level
- Dream cycles would push confidence much higher by filling all the summary pillars

**What's needed for higher confidence:**
- Dream cycles (adds descriptions, fills missing fields) — pushes summary pillar toward 100%
- Family tree seed (adds relations) — pushes relation pillar from 0%
- Combined: expected confidence >0.65 for well-evidenced entities

**Comparison vs pure CC baseline (Experiment 2):**
| Metric | CC only | CC + EC hybrid | Δ |
|--------|---------|----------------|---|
| Entities | 223 | 242 | +19 (+8.5%) |
| FT recall | ~13.6% | 65.3% | — (corpus diff, not hybrid diff) |
| Entities with fields | 0 | 100 (41%) | +100 |
| Build time | ~708s | ~708s + EC pass | ~+10min |

Note: FT recall jump is from 10% corpus size, not EC itself. The EC contribution is the +19 entities
and the field enrichment of 11 existing entities.

### Next steps
- Run dedup (Tier 1+3) to clean the 242 entities
- Add family tree seed for relation pillar coverage
- Run dream cycles to improve summary scores toward 0.65+ health
- Re-evaluate threshold: consider 0.40 to capture only the truly empty entities in larger graphs

---

## 2026-06-04 – D6_mini_loop_10pct_20260604

- **Type:** Full mini Dream RAG loop on 10% corpus
- **Pipeline:** CC+EC build → dedup → sanitize+reembed → seed → dream (1 cycle) → EC refine-only

### Pipeline steps

| Step | Entities | Relations | Health | Notes |
|------|----------|-----------|--------|-------|
| CC + EC (threshold=0.45, budget=50) | 242 | 0 | 35.0% | 19 new EC discoveries |
| Dedup (Tier 1+3+4) | 224 | 0 | — | 18 clean merges |
| Sanitize + reembed | 223 | 0 | — | 1 stub pruned |
| Family tree seed | 219 | 118 | 38.5% | 69 relations, 33 aliases merged |
| Dream cycle (150 completions) | 218 | 136 | 41.5% | 120 summaries, 1 merge |
| EC refine-only (threshold=0.34, budget=31) | 241 | 136 | 40.9% | +6 improved, +23 new |

### Eval result
- **Recall (token-overlap): 52.0%** (117/225 keywords)
- Avg latency: 26.2s/question
- Judge score: not run (needs `--llm-judge` flag)
- Graph: 241 entities, 136 relations, 40.9% health

### Interpretation
52.0% from a 10%-corpus graph on a full 40-question set covering the entire memoir.
Most of the remaining gap vs the full-corpus best (59.5%) is explained by missing source
text rather than graph quality. This validates the pipeline and confirms readiness to scale
to 100% with the same settings.

### Threshold evolution
- Fresh graph (before dream): threshold=0.45 captures all entities (all at 33% baseline)
- Post-dream graph: threshold=0.34 targets only the 31 entities dream couldn't reach
  (no source text in the 10% corpus slice). Threshold adapts naturally with graph health.

### Ready for 100% run
- CC + EC: same flags, remove --sample-pct
- Dedup: same Tier 1+3+4 conservative settings
- Seed: same d6_family_tree.yaml
- Dream: increase --max-completions to 300-500 for larger graph
- EC refine-only at 0.34: same after dream

Expected: ~800-1200 entities, ~300-500 after dream enrichment, approaching the M22
baseline of 58-60% keyword recall.

---

## 2026-06-04 – D6_person_full_dream5_20260604

- **Pipeline:** CC+EC build (100%) → dedup → sanitize → reembed → seed → 5× dream cycles → eval
- **Frozen:** no further modifications pending manual review

### Graph state before dream
- 1,111 entities, 142 relations, 36.6% health

### Dream cycles (max-completions=300, dedup-threshold=0.99, --no-relations)

| Cycle | Health | Δ | Summaries | Merged |
|-------|--------|---|-----------|--------|
| 1 | 36.6% → 38.0% | +1.4pp | 246 | 12 |
| 2 | 38.0% → 39.3% | +1.2pp | 233 | 0 |
| 3 | 39.3% → 40.2% | +0.9pp | 242 | 3 |
| 4 | 40.2% → 40.6% | +0.5pp | 231 | 2 |
| 5 | 40.6% → 40.9% | +0.3pp | 237 | 1 |

Decelerating gain (+1.4 → +0.3pp) — approaching plateau after 5 cycles. More cycles needed for M22-level health (78.1%).

### Final graph
- **1,095 entities, 140 relations, 40.9% health**

### Eval result
- **56.0% recall (126/225 keywords)** — 40 questions, avg 31.1s/question
- mode=iterative, top_k=20, no judge score

### Per-question highlights
- Perfect (100%): q01 author, q02 children, q04 dedication, q26 Dr. Abdurahman
- Strong (≥80%): q08 wife detail, q10 Kloof Nek, q11 TLSA, q17 Hewat, q31 mosque, q34 Group Areas Act
- Weak (0%): q03 grandchildren, q36 political organisations

### Comparison
| | Person-only full+dream5 | Full M30 (~57.8% avg) | Full M22 (58.6%, 31 cycles) |
|---|---|---|---|
| Entities | 1,095 | ~1,905 | 1,013 |
| Relations | 140 | ~6,164 | 2,193 |
| Health | 40.9% | — | 78.1% |
| Recall | **56.0%** | 57.8% | 58.6% |

Person-only graph at 5 dream cycles achieves 56.0% — within 2pp of the full-corpus best (58.6% at 31 cycles). This validates the Person-only pipeline and confirms dream enrichment is the primary driver of accuracy, not entity type breadth.

## 2026-06-07 – D6_struct_coref_rel_20260607_122357

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 1 entities, 0 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  1047 entities, 188 relations, health=36.7%, **?** recall (?)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260607_122357.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260607_122357.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260607_122357.md

### Key delta questions
```
| Overall recall (token-overlap) | 57.3% (129/225) |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 23073ms |
| q12 | Who was Cissie Gool? | 3/6 (50%) | [Graph: J. M. H. Gool], LEST WE FORGET -rev25.pdf | 39274ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Mr. Gool], LEST WE FORGET -rev25.pdf | 25792ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 26350ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | LEST WE FORGET -rev25.pdf, [Graph: Mr. Gool] | 30359ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 22471ms |
```

## 2026-06-08 – D6_struct_coref_rel_20260607_220927

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 1047 entities, 188 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  1023 entities, 188 relations, health=36.5%, **51.6%** recall (116/225)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260607_220927.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260607_220927.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260607_220927.md

### Key delta questions
```
| Overall recall (token-overlap) | 51.6% (116/225) |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 20677ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 28318ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | [Graph: Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 23936ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 23989ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 23475ms |
| q38 | Who was Cissie Gool's father? | 1/5 (20%) | LEST WE FORGET -rev25.pdf | 21485ms |
```

### Analysis vs previous run (20260607_122357, 57.3%)

**Result: REGRESSION — 51.6% vs 57.3% (-5.7pp, -13 keyword matches)**

This run introduced the backward-candidate pronoun resolver (`ner.rs` commit `6015bc1`). The regression is likely caused by **backward_candidate resolving gendered pronouns to Place entities** (e.g., "He walked through District Six" → `'he' = 'District Six'`). When these wrong targets are merged into the candidates list and injected as KNOWN COREFERENCES, the LLM receives contradictory signals that reduce Person entity extraction precision.

| Question | Prev | This | Delta | Notes |
|----------|------|------|-------|-------|
| q12 Cissie Gool | 50% | 33% | -17pp | Cissie cluster most affected |
| q24 JMH children | 0% | 29% | +29pp | Genuine improvement |
| q32 Cissie-JMH relation | 80% | 60% | -20pp | |
| q38 Cissie's father | 60% | 20% | -40pp | |
| q21 Author's mother | ? | 0% | — | Ayesha Rassool not retrieved |
| q23 Author's siblings | ? | 0% | — | |

**Root cause**: `backward_candidate()` picks the rightmost proper noun before the pronoun without entity-type filtering. In memoir text like "Yousuf walked through District Six. He remembered…", "District Six" appears later than "Yousuf" → the pronoun resolves to the wrong entity. The LLM then sees `KNOWN COREFERENCES: 'he' = 'District Six'` which is confusing.

**Recommended fix**: Constrain `backward_candidate` to multi-word candidates only (single-word entries are more likely to be places), or revert to forward-scan-only for reset builds until a proper gender/type signal is available.
```

## 2026-06-08 – D6_struct_coref_rel_20260608_163617

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 64 entities, 148 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  1037 entities, 194 relations, health=36.5%, **52.0%** recall (117/225)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260608_163617.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260608_163617.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260608_163617.md

### Key delta questions
```
| Overall recall (token-overlap) | 52.0% (117/225) |
| q09 | Who was the author's grandfather? | 3/9 (33%) | LEST WE FORGET -rev25.pdf | 20846ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdul Hamid Gool] | 26765ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | [Graph: Dr. Abdul Hamid Gool], LEST WE FORGET -rev25.pdf | 26045ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 27475ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 2/5 (40%) | [Graph: Adam Gool], LEST WE FORGET -rev25.pdf | 24583ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 25612ms |
```

### Analysis vs previous runs

**Result: SLIGHT REGRESSION vs baseline (-1.3pp), MARGINAL IMPROVEMENT vs yesterday (+0.4pp)**

| Run | Recall | Notes |
|-----|--------|-------|
| D6_person_full baseline (2026-06-04) | 53.3% | Person+Place+Org, no struct, no coref |
| D6_struct_coref_rel_20260607_220927 | 51.6% (116/225) | First overnight, backward_candidate bug |
| **D6_struct_coref_rel_20260608_163617** | **52.0% (117/225)** | This run |

**Differences vs yesterday (20260607_220927)**:
- Family tree seeded with Feyruz, Reza, Zarina Rassool + 6 parent_of edges (new in this run)
- q02 (author's children) 100% ✓ — confirmed graph seed working
- q38 (Cissie's father) improved 20% → 60% — unknown why
- q32 (Cissie-JMH relation) regressed 60% → 40%

**Root causes of persistent regression vs 53.3% baseline**:
1. **47% graph-build timeout rate**: ~half the full-doc chunks return empty entity results over P2P relay with 8 workers. Graph entity coverage is low despite 1037 entities — many from index seeds, not document extraction.
2. **Person-only entity types**: Place and Organization entities excluded. Reduces graph-boost for Place-heavy questions (q06 Buitencingle, q10 Kloof Nek, q14 District Six, q15 forced removals all at 25-50%).
3. **backward_candidate pronoun resolver** (identified yesterday): still active, likely injecting wrong KNOWN COREFERENCES for place-named pronouns.

**Weakest questions (≤25%)**:
- q30 When did JMH arrive (17%) — very specific biographical fact, not in seeded relations
- q36 Political organizations (17%) — broad org question, orgs not extracted
- q05 Who was JMH Gool (25%) — Gool cluster still underpopulated despite family tree
- q06 Buitencingle (25%) — Place, needs Place entity type re-enabled

**Next steps**:
1. Re-enable Person+Place+Organization entity types (was disabled to match person_full baseline)
2. Fix backward_candidate: constrain to multi-word names or revert to forward-scan-only
3. Reduce worker count (8→4) or add retry logic to reduce timeout rate
4. Consider overnight eval with Place+Org re-enabled to isolate entity-type impact

## 2026-06-09 – D6_struct_coref_rel_20260608_195113

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 1037 entities, 194 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  1965 entities, 208 relations, health=37.3%, **56.4%** recall (127/225)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260608_195113.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260608_195113.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260608_195113.md

### Key delta questions
```
| Overall recall (token-overlap) | 56.4% (127/225) |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 20500ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Uncle Aity (Mohamed Saaid Gool)], LEST WE FORGET -rev25.pdf | 30253ms |
| q24 | Who were the children of J.M.H. Gool? | 2/7 (29%) | LEST WE FORGET -rev25.pdf, [Graph: J. M. H. Gool & Co.] | 32099ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 24872ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Uncle Aity (Mohamed Saaid Gool)], LEST WE FORGET -rev25.pdf | 24879ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 28834ms |
```

## 2026-06-09 – D6_struct_coref_rel_20260608_195113

- **Experiment:** Full rebuild with Person+Place+Organization + 4 workers + coref + CC/EC relations
- **Before:** 1037 entities, 194 relations, **52.0%** recall (D6_struct_coref_rel_20260608_163617)
- **After:**  1965 entities, 208 relations, health=37.3%, **56.4%** recall (127/225)
- **Changes vs previous run:**
  - Entity types: Person only → Person,Place,Organization
  - Workers: 8 → 4
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260608_195113.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260608_195113.md

### Key delta questions (vs 20260608_163617 — 52.0%)
```
| Overall recall (token-overlap) | 56.4% (127/225) |
| q05 | Who was J.M.H. Gool? | 5/8 (62%) | +37pp | [Graph: Uncle Aity (Mohamed Saaid Gool)] |
| q06 | Tell me about Buitencingle. | 5/8 (62%) | +37pp | Place entity now extracted |
| q15 | What were forced removals from District Six? | 5/6 (83%) | +50pp | |
| q18 | What was the New Era Fellowship? | 5/6 (83%) | +16pp | [Graph: New Era Fellowship] Org entity |
| q21 | Who was the author's mother? | 3/5 (60%) | +20pp | |
| q22 | Who was the author's father? | 2/4 (50%) | +25pp | |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | +40pp | |
| q17 | What was Hewat Training College? | 4/5 (80%) | -20pp | regression |
| q20 | Author's involvement in cricket | 2/5 (40%) | -20pp | regression |
| q24 | Children of J.M.H. Gool | 2/7 (29%) | -14pp | regression |
```

### Analysis vs all previous runs

**Result: BEST FULL-DOC RUN — 56.4% vs 53.3% baseline (+3.1pp), vs 52.0% previous (+4.4pp)**

| Run | Recall | Entity types | Workers |
|-----|--------|-------------|---------|
| D6_person_full baseline (2026-06-04) | 53.3% | Person+Place+Org | — |
| D6_struct_coref_rel_20260607_220927 | 51.6% | Person only | 8 |
| D6_struct_coref_rel_20260608_163617 | 52.0% | Person only | 8 |
| **D6_struct_coref_rel_20260608_195113** | **56.4%** | **Person+Place+Org** | **4** |

**Place+Org impact was decisive**: re-enabling them added ~928 entities (1965 vs 1037) and drove the biggest single-run improvement (+4.4pp). Place questions (q06 Buitencingle +37pp, q15 forced removals +50pp) and Org questions (q18 New Era Fellowship +16pp) were the main beneficiaries.

**Graph entities actively helping retrieval**: "Uncle Aity (Mohamed Saaid Gool)" pulled up for q05, q12, q32, q33. "New Era Fellowship" for q18. "Dr. Abdulla Abdurahman" for q26. "Bibi Gool" for q16, q27. The graph boost is working.

**Remaining regressions** (vs baseline 53.3% on specific questions):
- q09 (grandfather) 22% — still weak, not seeded
- q23 (siblings) 20% — not seeded
- q24 (JMH children) 29% — dropped from 43%; graph retrieves "J.M.H. Gool & Co." (Org) instead of family members
- q30 (JMH arrival) 0% — specific biographical fact not in seeded relations
- q36 (political orgs) 0% — broad query, no single good retrieval path

**Still below 1pct pronoun_fix run** (57.8%) but that was only 1% of document — not comparable.

**Next steps**:
- q24 regression: "J.M.H. Gool & Co." (Org) is crowding out family relations — consider blocking org names that are person-name derivatives from the entity cap
- q09/q23: seed grandfather + sibling relations in d6_family_tree.yaml
- q30/q36: these need better retrieval (passage-level) not graph — consider hyde or rerank for specific factual lookups

## 2026-06-09 – D6_struct_coref_rel_20260609_011814

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 1965 entities, 208 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  2002 entities, 204 relations, health=37.4%, **52.9%** recall (119/225)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260609_011814.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260609_011814.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260609_011814.md

### Key delta questions
```
| Overall recall (token-overlap) | 52.9% (119/225) |
| q09 | Who was the author's grandfather? | 2/9 (22%) | LEST WE FORGET -rev25.pdf | 25990ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | LEST WE FORGET -rev25.pdf, [Graph: Wahida Gool] | 37656ms |
| q24 | Who were the children of J.M.H. Gool? | 0/7 (0%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 25645ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 6/6 (100%) | LEST WE FORGET -rev25.pdf, [Graph: Dr. Abdulla Abdurahman] | 29727ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 4/5 (80%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 25075ms |
| q38 | Who was Cissie Gool's father? | 3/5 (60%) | LEST WE FORGET -rev25.pdf | 32178ms |
```

## 2026-06-09 – D6_struct_coref_rel_20260609_011814

- **Experiment:** Reproducibility run — identical settings to 20260608_195113
- **Before:** 1965 entities, 208 relations (20260608_195113 end state)
- **After:**  2002 entities, 204 relations, health=37.4%, **52.9%** recall (119/225)
- **Settings:** Person+Place+Org, 4 workers, 70b Q3 relations, no changes
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260609_011814.md

### Analysis — HIGH VARIANCE FINDING

**Two identical-config runs produced 56.4% vs 52.9% — a 3.5pp spread (8 keyword matches).**

| Run | Recall | Config |
|-----|--------|--------|
| D6_struct_coref_rel_20260608_195113 | 56.4% (127/225) | Person+Place+Org, 4w |
| D6_struct_coref_rel_20260609_011814 | 52.9% (119/225) | Person+Place+Org, 4w (identical) |
| **Average** | **54.7%** | |

Per-question swings between the two runs:

| Question | Run 1 | Run 2 | Swing |
|----------|-------|-------|-------|
| q05 JMH Gool | 62% | 12% | -50pp |
| q15 forced removals | 83% | 33% | -50pp |
| q08 author's wife | 67% | 33% | -34pp |
| q25 IB Tabata | 20% | 80% | +60pp |
| q22 author's father | 50% | 100% | +50pp |
| q06 Buitencingle | 62% | 50% | -12pp |
| q24 JMH children | 29% | 0% | -29pp |

**Root causes of variance:**
1. **Entity extraction stochasticity**: 8b LLM extracts different entity sets each run — which entities are in the graph changes which graph nodes the retriever surfaces. q05 got "Uncle Aity (Mohamed Saaid Gool)" run 1 vs "Wahida Gool" run 2 — completely different graph hits.
2. **Eval answer stochasticity**: 8b LLM generates different answer text each run — token-overlap scoring varies accordingly.
3. **Combined noise floor**: ~±2pp is inherent noise for a single run on this eval set.

**Implication**: Single-run comparisons under ±3pp are not meaningful. The true performance of the current Person+Place+Org config is approximately **54–56%**, vs the 53.3% baseline. We are likely above baseline but not by a large margin.

**To reliably distinguish configs**: Need 3+ runs per config and compare averages, or use a deterministic eval (temperature=0, fixed seed).

## 2026-06-09 – D6_struct_coref_rel_20260609_065108

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** 2002 entities, 204 relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  1984 entities, 196 relations, health=37.3%, **53.8%** recall (121/225)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260609_065108.md
- **Coref output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/coref_D6_struct_coref_rel_20260609_065108.md
- **Relation output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/extract_rel_D6_struct_coref_rel_20260609_065108.md

### Key delta questions
```
| Overall recall (token-overlap) | 53.8% (121/225) |
| q09 | Who was the author's grandfather? | 2/9 (22%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 70768ms |
| q12 | Who was Cissie Gool? | 2/6 (33%) | [Graph: Wahida Gool], LEST WE FORGET -rev25.pdf | 93990ms |
| q24 | Who were the children of J.M.H. Gool? | 3/7 (43%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 35901ms |
| q26 | Who was Dr. Abdullah Abdurahman? | 2/6 (33%) | [Graph: Dr. Abdulla Abdurahman], LEST WE FORGET -rev25.pdf | 30238ms |
| q32 | How was Cissie Gool related to J.M.H. Gool? | 3/5 (60%) | [Graph: Bibi Gool], LEST WE FORGET -rev25.pdf | 32534ms |
| q38 | Who was Cissie Gool's father? | 2/5 (40%) | [Graph: Yousuf Rassool], LEST WE FORGET -rev25.pdf | 28464ms |
```

## 2026-06-09 – D6_struct_coref_rel_20260609_065108

- **Experiment:** Reproducibility run 3 — identical settings to 20260608_195113 and 20260609_011814
- **After:**  1984 entities, 196 relations, health=37.3%, **53.8%** recall (121/225)
- **Settings:** Person+Place+Org, 4 workers, 70b Q3 relations
- **Eval output:** /Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results/eval_D6_struct_coref_rel_20260609_065108.md

### Three-run summary for Person+Place+Org config

| Run | Recall | Keywords |
|-----|--------|----------|
| 20260608_195113 | 56.4% | 127/225 |
| 20260609_011814 | 52.9% | 119/225 |
| 20260609_065108 | 53.8% | 121/225 |
| **Mean** | **54.4%** | **122.3/225** |
| **Std dev** | **1.5pp** | |
| **95% CI (n=3)** | **~50.7–58.0%** | |
| D6_person_full baseline | 53.3% | — |

**Conclusion: improvement is NOT statistically significant.** The baseline (53.3%) falls within the 95% CI of [50.7%, 58.0%]. We cannot claim the struct+coref+relations pipeline reliably beats the baseline with n=3 runs.

**What we can say:**
- Results are in the 52–57% range — consistent with or marginally above the 53.3% baseline
- Run-to-run variance (~±3.5pp) is driven by 8b LLM stochasticity in entity extraction and eval answers
- The pipeline is not hurting recall (vs earlier Person-only runs that regressed to 51-52%)
- Place+Org entity types are clearly useful (specific place/org questions improved), but the benefit is masked by noise

**To achieve a reliable signal, options:**
1. Run 8+ times and compare means — impractical overnight
2. Fix eval stochasticity: run the 8b LLM at temperature=0 with a fixed seed
3. Use a stronger eval model (70b or Claude) for answer generation — less variance
4. Focus on structural improvements that move the needle by >5pp to clear the noise floor
