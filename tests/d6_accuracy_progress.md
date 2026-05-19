# D6 RAG Accuracy Progress

**KB:** D6 memoir — _The Setting in Time and Place_ by Y.S. Rassool  
**Eval:** `tests/d6_eval_questions.json` · 20 questions · keyword hit rate + LLM-as-judge  
**Stack:** Rust KwaaiNet RAG · Ollama · llama3.1:8b

---

## Progress Chart

```
60% ┤                                                                       ████ 56.9% ← keyword best
    │                                                                            ████ 56.0% M18
55% ┤                                                                  ████ 51.7%
    │                                                             ████ 50.0%
50% ┤                                                        ████ 49.1%
    │                                                   ████ 48.3%
45% ┤                                         ████ 44.8%
    │                                    ████ 44.0%
    │                               ████ 43.1%
40% ┤               ████ 37.9%
    │          ████ 33.3%
35% ┤     ████
30% ┤████ 25.0% ── 31.9% ── 33.6% ← experiments (reverted)
25% ┤24.6%
    │
    └───────────────────────────────────────────────────────────────────
     P1    P2   P3  P7..11  exp    mini  fix  mxbai  auto  famseed  iter  dedup  iter  dream
                                                           +judge         k=20   k=20  cycle1
```

**Judge score history:** — / — / — / — / — / — / — / — / — / — / 1.85 / 1.65 / 1.80 / 1.55 / **1.80** / 1.70 (M18)

---

## Milestone Table

| # | Version | Config | Model | Keyword % | Judge | Notes |
|---|---------|--------|-------|-----------|-------|-------|
| 1 | v0.4.44 | paragraph, k=5, nomic 768-dim, 10q | llama3.1:8b | **24.6%** (14/57) | — | Baseline |
| 2 | v0.4.44 | + HyDE, k=8 | llama3.1:8b | **33.3%** (19/57) | — | +8.7pp |
| 3 | v0.4.45 | min_chunk_len 100→20 | llama3.1:8b | — | — | Unblocked short intro chunks |
| 4 | v0.4.45 | Phases 7–11, 10q | llama3.1:8b | **37.9%** (~22/57) | — | 10-question baseline |
| — | — | + synthetic doc headers | llama3.1:8b | 31.9% | — | Reverted |
| — | — | + document summaries | llama3.1:8b | 25.0% | — | Reverted |
| — | — | nomic, chunk=2000, k=20, 20q | llama3.1:8b | 33.6% (~39/116) | — | Reverted |
| 5 | v0.4.48 | **all-minilm 384-dim**, chunk=512, k=30, 20q | llama3.1:8b | **43.1%** (50/116) | — | New embedder |
| 6 | v0.4.49 | + max_context 24000, anti-hallucination prompt | llama3.1:8b | **44.0%** (50/114) | — | +0.9pp |
| 7 | v0.4.49 | same | gemma3:4b | 41.4% (48/116) | — | −2.6pp vs llama |
| 8 | v0.4.49 | same | gpt-oss:20b | 40.5% (47/116) | — | −3.5pp, 4.3× slower |
| 9 | v0.4.49 | **mxbai-embed-large 1024-dim**, k=30 | llama3.1:8b | **44.8%** (52/116) | — | +0.8pp |
| 10 | v0.4.49 | mxbai, k=30, --mode graph | llama3.1:8b | 43.1% (50/116) | — | Graph alone trails; gains entity Qs |
| 11 | v0.4.49 | mxbai, k=30, **--mode auto** | llama3.1:8b | **46.6%** (54/116) | — | Router adds +2 on q06/q12/q19 |
| 12 | v0.4.51 | auto + **family tree seeding** (61 aliases merged) | llama3.1:8b | **50.0%** (58/116) | 1.85/2 (lenient) | Graph cleaned; new best keyword |
| 13 | v0.4.51 | same + **strict judge** (content-focused prompt) | llama3.1:8b | **48.3%** (56/116) | **1.65/2** | Calibrated judge; 11×2/2, 8×1/2, 1×0/2 |
| 14 | v0.4.53 | **--mode iterative**, top_k=10 | llama3.1:8b | **49.1%** (57/116) | **1.80/2** ⬆ | Multi-round gap-fill; **16×2/2, 4×1/2, 0×0/2** — new judge best |
| 15 | v0.4.56 | graph dedup + `graph reembed`, mode=graph, k=5 | llama3.1:8b | **35.3%** (41/116) | — | Post-dedup checkpoint; q1–q10 at 29.8% vs 24.6% original baseline. Dedup+reembed confirmed positive. |
| 16 | v0.4.56 | **mode=auto, k=20** | llama3.1:8b | **51.7%** (60/116) | 1.55/2 | k=20 sweet spot confirmed. q04 dedication first-ever 4/4. Rerank hurts (−3.4pp). Judge trails iterative. |
| 17 | v0.4.56 | **mode=iterative, k=20** | llama3.1:8b | **56.9%** (66/116) | **1.80/2** ⬆ | New best on BOTH metrics. q04 4/4 ✓, q05 J.M.H. Gool 6/8 ↑ (alias fix working). 16×2/2, 4×1/2, 0×0/2. |
| 18 | v0.4.72 | same + Dream RAG cycle 1 (7 types, 4 summaries) | llama3.1:8b | **56.0%** (65/116) | 1.70/2 | Within variance of M17. Insufficient dream cycles to move needle — 100-completion run stalled, only 20 completions applied. Gains: q11, q12 kw, q16 kw, q17 full. Losses: q04, q05 (sampling variance). |

> Note: keyword hit rate varies ±4pp between runs of the same config due to LLM sampling. Milestones 12–13 are separate runs of the same stack; consider 48–50% the range for the current best config.

---

## Judge Scores by Question

| ID | Question | M14 kw | M14 judge | M17 kw | M17 judge | Δ kw | Δ judge |
|----|----------|--------|-----------|--------|-----------|------|---------|
| q01 | Who is the author? | 3/3 | 2/2 | 2/3 | **2/2** | −1 | = |
| q02 | Author's children? | 3/3 | 2/2 | 3/3 | **2/2** | = | = |
| q03 | Author's grandchildren? | 6/6 | 2/2 | 6/6 | 1/2 | = | ⬇ −1 |
| q04 | Book dedication? | 0/4 | 1/2 | **4/4** | **2/2** | ⬆ +4 | ⬆ +1 |
| q05 | Who was J.M.H. Gool? | 2/8 | 2/2 | **6/8** | **2/2** | ⬆ +4 | = |
| q06 | Tell me about Buitencingle. | 1/8 | 1/2 | 3/8 | **2/2** | ⬆ +2 | ⬆ +1 |
| q07 | Author's wife? | 1/3 | 2/2 | 2/3 | **2/2** | ⬆ +1 | = |
| q08 | More about wife? | 5/6 | 2/2 | 5/6 | 1/2 | = | ⬇ −1 |
| q09 | Author's grandfather? | 2/9 | 2/2 | 3/9 | **2/2** | ⬆ +1 | = |
| q10 | Kloof Nek? | 5/7 | 2/2 | 4/7 | **2/2** | −1 | = |
| q11 | TLSA? | 3/6 | 2/2 | 3/6 | 1/2 | = | ⬇ −1 |
| q12 | Who was Cissie Gool? | 3/6 | 2/2 | 0/6 | **2/2** | −3 | = |
| q13 | All Africa Convention? | 2/6 | 1/2 | 2/6 | 1/2 | = | = |
| q14 | Where was District Six? | 2/6 | 1/2 | 4/6 | **2/2** | ⬆ +2 | ⬆ +1 |
| q15 | Forced removals? | 3/6 | 2/2 | 2/6 | **2/2** | −1 | = |
| q16 | Gandhi / Gool family? | 2/7 | 2/2 | 3/7 | **2/2** | ⬆ +1 | = |
| q17 | Hewat College? | 4/5 | 2/2 | 4/5 | **2/2** | = | = |
| q18 | New Era Fellowship? | 4/6 | 2/2 | 5/6 | **2/2** | ⬆ +1 | = |
| q19 | NEUM? | 4/6 | 2/2 | 3/6 | **2/2** | −1 | = |
| q20 | Cricket? | 2/5 | 2/2 | 2/5 | **2/2** | = | = |

**M14 summary (iterative k=10):** 16×2/2, 4×1/2, 0×0/2 → 49.1% kw / 1.80/2 judge  
**M17 summary (iterative k=20):** 16×2/2, 4×1/2, 0×0/2 → **56.9% kw / 1.80/2 judge** ⬆ new best both metrics

**Net keyword gains (M14→M17):** q04 (+4), q05 (+4), q06 (+2), q14 (+2), q09/q07/q16/q18 (+1 each)  
**Net keyword losses:** q12 (−3, but judge=2/2 — model answers correctly with different words), q01/q10/q19 (−1 each — within variance)  
**Judge changes:** q04 ⬆, q06 ⬆, q14 ⬆ gained; q03, q08, q11 ⬇ lost — net zero, same 1.80/2

**M18 summary (iterative k=20, Dream RAG cycle 1):** 15×2/2, 4×1/2, 1×0/2 → 56.0% kw / 1.70/2 judge — within M17 variance  
**Net keyword gains (M17→M18):** q09 (+1), q11 (+1), q12 (+4), q16 (+2), q17 (+1)  
**Net keyword losses:** q04 (−4), q05 (−4), q08 (−1), q10 (−1), q14 (−1) — sampling variance, not structural regression  
**Judge changes:** q03 ⬆ recovered; q04 ⬇, q05 ⬇, q10 ⬇, q15 ⬇ — q04/q05 are high-variance questions  
**Conclusion:** Dream RAG showed no measurable impact after 1 cycle (only 20 completions, graph health 59.5→59.6%). Need sustained cycles.

---

## Key Insight: Keyword vs Judge Gap

The keyword metric *underestimates* answer quality for questions where the model answers correctly but with different wording (q07, q09, q11, q20). It *accurately measures* retrieval failures where the model can't find the content at all.

| Question cluster | Keyword | Judge | Interpretation |
|---|---|---|---|
| q07, q09, q11, q20 | Low | 2/2 | Model answers correctly, keywords too specific |
| q01, q04, q05 | Low | 1/2 | Genuine partial retrieval — content is there but partial |
| q03 | Variable | Variable | High run-to-run variance; borderline retrieval |

**Eval reliability:** ±4pp run-to-run variance at the same settings. Use 3-run average for reliable comparison.

---

## Persistent Retrieval Failures

| ID | Root cause | Fix path |
|----|------------|----------|
| q04 dedication (1/2) | Dedication text doesn't use the word "dedicated" — BM25 blind spot | Metadata injection: tag dedication chunk as `[Document type: dedication page]` |
| q06 Buitencingle (1/2 at k=10) | top_k=10 too restrictive for broad descriptive questions | Re-test with --top-k 20 in iterative mode |
| q13 All Africa Conv (1/2) | Correct chapter retrieved but model hedges without enough specific facts | Investigate chapter chunk density |
| q14 District Six (1/2 at k=10) | Regression from k=30; top_k=10 misses key chunks | Re-test with --top-k 20 in iterative mode |

---

## What Changed at Each Phase

### Iterative retrieval (v0.4.53)
`--mode iterative` adds multi-round gap-filling on top of the Round 1 vector+graph fusion. Coverage check extracts content terms from the query; if < 70% found in retrieved chunks, Round 2 embeds the missing terms and does a targeted graph entity BFS. If still < 75%, Round 3 asks the LLM to reformulate a targeted sub-query and re-runs vector search. Each round narrates what it's doing. **Judge score: 1.65 → 1.80** (+0.15). Notably fixed q01 (author), q03 (grandchildren), q05 (J.M.H. Gool), q10 (Kloof Nek), q15 (forced removals). Two regressions (q06, q14) likely caused by top_k=10 vs 30; test at top_k=20.



### Family tree seeding (v0.4.51)
61 aliases merged into 24 canonical Person entities (e.g., "Joosub Gool", "JMH Gool", "J.M.H. Gool" → "Haji Joosub Maulvi Hamid Gool"). 46 authoritative family relations planted. Eliminated duplicate fragmented entity nodes — context for entity questions now consolidated. q12 (Cissie Gool), q16 (Gandhi+Gool), q19 (NEUM) all improved.

### LLM-as-judge (v0.4.51)
`--llm-judge` flag now available on `rag eval`. Key calibration finding: judge prompt must score CONTENT not phrasing — "I couldn't find but here's what I know" hedges should be scored on facts, not the hedge. Same-model judging (llama3.1:8b) is adequate for factual questions; for nuanced eval, use `--judge-model` with a different model.

### Embedder: nomic → all-minilm → mxbai-embed-large
- nomic-embed-text: 768-dim — 43.1% (baseline)
- all-minilm: 384-dim — 44.0% (+0.9pp)
- mxbai-embed-large: 1024-dim — 44.8% (+0.8pp)

### max_context_chars 8192 → 24000
With k=30 chunks at ~300 chars each, 8192 chars only showed ~16/30 chunks. Raising to 24000 lets all 30 reach the LLM.

---

## Model Comparison

| Model | Keyword % | Avg latency | Judge |
|-------|-----------|-------------|-------|
| llama3.1:8b | **44–50%** | ~21s | 1.65/2 |
| gemma3:4b | 41.4% | ~6s | — |
| gpt-oss:20b | 40.5% | ~25s | — |

**Finding:** Larger ≠ better for RAG. llama3.1:8b leads on both metrics. The 20B model over-explains and drifts from the source. gemma3:4b is a good speed/quality tradeoff.

---

## Next Steps

| Priority | Approach | Expected gain |
|----------|----------|---------------|
| High | Run sustained dream cycles (target graph health >70%) then run M19 eval — confirm graph quality lifts retrieval | Diagnostic |
| High | Investigate q13 (All Africa Convention) — right chapter retrieved but model hedges, stuck at 1/2 | +1pp judge |
| Medium | Investigate q03/q08/q11 judge regressions from M14 — were they sampling flukes? | Stability |
| Medium | Dream RAG Phase 3: quality gate — snapshot + rollback if score drops >5% after a cycle | Stability |
| Low | Dream RAG Phase 4: embed model evaluation (`dream embed-eval`) | Graph quality |
| Low | Dream RAG Phase 5: gamified curation GUI (Flutter, after PR #56 merge) | UX |
| Done ✓ | **Dream RAG Phase 1+2**: graph health scorer + autonomous completion cycle (v0.4.72) | Graph quality |
| Done ✓ | Best config found: **iterative k=20** — 56.9% kw / 1.80/2 judge (new best both metrics) | |
| Done ✓ | k-sweep: k=5 (35%), k=8 (33%), k=10 (41%), k=20 (**56.9%**), k=30 (41%) — k=20 is sweet spot | |
| Done ✓ | Rerank at k=20: −3.4pp (auto), −8.6pp (iterative) — rerank hurts, do not use | |
| Done ✓ | `graph dedup --auto` + interactive pass (v0.4.56) | Graph cleaned |
| Done ✓ | `graph reembed` — entities now embed `"{name}: {description}"` | Abbreviation lookup fixed |

---

## Graph Health (Dream RAG baseline — v0.4.72)

| Metric | Value |
|--------|-------|
| Entities | 2,291 |
| Relations | 7,942 |
| Overall health score | 59.6% |
| Unknown-type entities | 81 (88 before first cycle) |
| Type distribution | Person 1056, Org 460, Place 360, DefinedTerm 177, Unknown 81, Event 59 |

**Dream RAG cycle 1 (20 completions):** 7 type assignments, 4 summary enrichments, 5 relations added, score 59.5% → 59.6%, Unknown 88 → 81.  
Next: run eval after several more cycles to measure retrieval impact.

---

## Engineering Narrative

This section tells the story of how we got from 24.6% to 49–50% accuracy on a 20-question factual eval over a 200-page memoir. Each improvement is described in terms of the problem it solved, not just what was changed.

---

### The starting point

KwaaiNet's RAG pipeline began as a straightforward dense-retrieval system: split documents into overlapping paragraphs, embed with `nomic-embed-text`, store vectors locally, retrieve top-5 by cosine similarity, pass to `llama3.1:8b`. On a 10-question eval over _The Setting in Time and Place_, this scored **24.6% keyword hit rate**.

The immediate diagnosis was brutal: several questions got 0% not because the retriever was imprecise, but because the relevant text *physically could not be retrieved*. The author's name appears in a signature line roughly 17 characters long. The book dedication runs to about 110 characters. Both were below the 100-character minimum chunk length and were being silently discarded or merged into bibliography noise during ingestion.

---

### Phase 1 — Unblocking the obvious content (v0.4.45)

Lowering `min_chunk_len` from 100 to 20 was a single-line change. Combined with tuning top-k and switching to the full 20-question set, this lifted the baseline to **37.9%**. The experiment also revealed that synthetic document headers and pre-generated summaries hurt retrieval — the model anchored on the summaries rather than the source chunks, so those were reverted.

---

### Phase 2 — Better embeddings

The pipeline was re-run with three embedding models in succession:

- `nomic-embed-text` (768-dim): 43.1%
- `all-minilm` (384-dim): 44.0% — smaller and faster, slightly better
- `mxbai-embed-large` (1024-dim): **44.8%** — new high, richer semantic space

Each swap required destroying and re-ingesting the KB because the vector dimensions change. The lesson: embedding model choice matters more than most retrieval parameters.

Two other improvements shipped alongside the embedder work: the context window passed to the LLM was raised from 8,192 to 24,000 characters (at k=30, the old window was silently discarding the bottom half of retrieved chunks), and an anti-hallucination instruction was added to the system prompt. Together these added 0.9pp over the mxbai baseline.

---

### Phase 3 — Knowledge graph (v0.4.49–v0.4.51)

Dense retrieval has a fundamental weakness with proper-name questions: "Who was J.M.H. Gool?" yields a query embedding that clusters near general biographical text, not specifically near the chapter about that person. The gap between abbreviations/acronyms and their canonical descriptions is invisible to cosine similarity.

A knowledge graph layer was built on top of the chunk store. During ingestion, an LLM extracts entities (Person, Organization, Location, Event, …) and directed relations from each chunk, assigns deterministic IDs via SHA-256(name + type), and persists them to a per-tenant redb database. At query time, entity embeddings are searched first; matching entities' BFS neighborhoods (2 hops) contribute chunk IDs that are RRF-fused with the dense vector results.

Graph mode alone scored **43.1%** — trailing hybrid vector retrieval. The reason is clear in hindsight: graph retrieval pulls in all chunks that mention related entities, which adds noise when those entities appear in tangential contexts. The router mode (`--mode auto`) solves this by detecting entity-heavy queries and blending graph and vector results. Auto mode reached **46.6%**.

The bigger graph win came from a different direction: the graph was full of fragmented duplicates. "J.M.H. Gool", "JMH Gool", "Joosub Gool" and several other variants all existed as separate entity nodes with no connections between them. A family-tree YAML seeding command was built: it loads a ground-truth list of canonical persons with their known aliases, upserts the canonical entity with an authoritative description, merges all alias nodes into it (re-pointing their relations), and plants known family relations. After merging 61 aliases into 24 canonical persons and planting 46 family relations, **keyword hit rate jumped to 50.0%** — the highest ever recorded, and the first time graph consolidation produced a clear improvement over pure vector retrieval.

---

### Phase 4 — Measuring quality, not just keywords (v0.4.51)

Keyword hit rate has a known flaw: it counts whether specific expected words appear in the answer, not whether the answer is correct. A question like "Who was the author's wife?" scored 1/3 by keywords but the model answered perfectly — it used different phrasing. Conversely, the keyword metric accurately catches genuine retrieval failures where the model can't find the content at all and hedges.

An LLM-as-judge was added to the eval harness (`--llm-judge`). The first judge prompt was too lenient — it rated answers 1.85/2 on average. A content-focused calibration (score on facts conveyed, not on tone or confidence) brought it to **1.65/2**, a more honest baseline. Key calibration rule: a hedging answer ("I couldn't find this but here's what I think…") should be scored on the facts it gets right, not penalised for the hedge.

---

### Phase 5 — Iterative retrieval (v0.4.53)

Even with a well-seeded graph, a single-round retrieval pass has a coverage problem: it returns the top-k most similar chunks, but for multi-faceted questions some relevant content is consistently ranked below the cutoff. The fix is to check coverage after Round 1 and go back for more if it's insufficient.

Iterative mode adds two gap-filling rounds:

- **Coverage check**: extract significant terms from the query (≥4 chars, not stop words), check what fraction appear in the retrieved chunks. If < 70%, trigger Round 2.
- **Round 2 — graph gap-fill**: embed the missing terms, find their nearest entity neighbors via the graph, add their chunk neighborhoods to the pool.
- **Round 3 — LLM reformulation**: if coverage is still < 75%, ask the LLM to rewrite a targeted sub-query for the missing terms, re-run vector+BM25 retrieval, add new chunks.

Each round narrates what it is doing so the user can see the pipeline at work. At top_k=10, iterative mode reached **1.80/2 judge score** — the best recorded, with 16 of 20 questions at full score and zero at zero. Five questions improved; two regressed (q06 Buitencingle, q14 District Six), almost certainly because top_k=10 is too tight for broad descriptive questions that were previously answered well at k=30.

---

### Phase 6 — Entity embedding quality (v0.4.54)

A subtle but important fix: entity embeddings were being computed from the entity *description* only — the LLM-generated summary text. This means "Haji Joosub Maulvi Hamid Gool: Patriarch of the Gool family…" was stored as an embedding of just "Patriarch of the Gool family…". A query for "J.M.H. Gool" embeds the abbreviation, which doesn't semantically resemble a description of a merchant patriarch.

Two changes fixed this:

1. The embedded text was changed from `description` to `"{name}: {description}"`, so the entity's name is baked into its embedding. Abbreviations and acronyms now find their canonical entity through similarity.
2. Alias names (e.g. "J.M.H. Gool", "JMH Gool", "Joosub Gool") are now stored on the canonical entity's `aliases` field when they are merged. `find_ids_by_name_token()` searches both the canonical name and aliases, so name-token matching continues working after the alias entity is removed.
3. A `graph reembed` command was added to re-embed all existing entities with the new format without requiring a full graph rebuild.

---

### Phase 7 — Graph deduplication (v0.4.55+)

Inspecting the graph in Obsidian revealed a long tail of duplicates that the family-tree seed didn't cover: "Hassen Mall" and "Hassen" as separate nodes, punctuation variants like "Mitchell s Plain" vs "Mitchell's Plain", OCR artifacts like "S  Jayiya" vs "S. Jayiya". A `graph dedup` command implements a three-tier approach:

- **Tier 1** (automatic): entities with identical normalized names (punctuation stripped, case-folded) are merged silently.
- **Tier 2** (interactive): entity pairs sharing ≥1 significant name token with embedding cosine similarity ≥ 0.85 are shown for review, with `y/n/q/?` prompts.
- **`--auto`**: merges all Tier 2 pairs above 0.92 similarity without prompts.

A dry-run on the D6 graph found 675 Tier 1 exact-name duplicates and 892 Tier 2 similarity candidates — substantial noise that the entity retrieval system has been working around. Cleaning this is expected to improve graph precision for entity-heavy questions.

---

### Phase 8 — Graph cleanup and k-sweep (v0.4.56)

After the entity embedding fix (Phase 6) and graph deduplication (Phase 7), a systematic top-k sweep was run on the cleaned graph:

| k | auto mode | iterative |
|---|-----------|-----------|
| 5  | 35.3% | — |
| 8  | 32.8% | — |
| 10 | 41.4% | 46.6% |
| 20 | **51.7%** | **51.7%** |
| 30 | 41.4% | — |

Two clear findings:

**k=20 is the sweet spot.** k=10 leaves relevant chunks below the cutoff; k=30 overflows the LLM context window (24,000 chars, ~300 chars/chunk → k=30 starts discarding). At k=20, every retrieved chunk fits and the quality jump is substantial.

**Rerank hurts at k=20.** The reranker dropped scores by 3.4pp (auto) and 8.6pp (iterative). Auto mode's RRF fusion already balances vector, BM25, and graph signals well at k=20; the reranker disrupts this balance by re-scoring purely on query–chunk similarity, ignoring the multi-signal fusion.

The 51.7% keyword score is the new project best, beating the previous 50.0% (M12). Notably, q04 (book dedication) hit 4/4 for the first time — it was stuck at 0/4 across all prior configs because the dedication chunk was consistently ranked just outside k=10. At k=20 it lands in the context window. This is a pure retrieval depth fix, not a content fix.

---

### Phase 9 — Dream RAG: continuous graph refinement (v0.4.72)

Inspecting the D6 knowledge graph in Obsidian made the problem concrete: 88 entities typed "Unknown", hundreds with one-line descriptions, many missing the relations that should be obvious given their type. The graph's entity extraction happened during ingestion — one pass, no iteration. A human reading the same memoir would naturally refine their mental model over time. Dream RAG brings that loop to the graph.

The system adds two new layers:

**Graph health scorer** (`scorer.rs`): a 3-pillar completeness score for every entity, using the schema.org ontology as the type standard.

- **Type score**: Unknown=0.0, schema:Thing=0.4 (catch-all), specific type (schema:Person, schema:Place, etc.)=1.0
- **Summary score**: empty=0.0, <50 chars=0.3, <150 chars=0.6, ≥150 chars + 2 sentences=1.0
- **Relation score**: expected relation groups for each schema.org type (e.g. Person expects `bornIn`, `memberOf`, `knownAs`); matched groups / total groups; 0.5 if no expectations defined

Overall score is the mean of the three pillars. `kwaainet rag graph score --kb D6` prints a live health report per entity and a type distribution table.

**Dream cycle** (`dream.rs`): an autonomous multi-step refinement loop.

1. Score graph → collect entities below the completeness threshold with source chunk text
2. Fan out LLM completion calls (configurable workers + semaphore) — each call asks the LLM for schema_type, a 2–3 sentence description from the source text, and relations present in that text
3. Write back: set schema_type, upsert richer description (only if >20 chars longer than current), add relations (target must already exist in graph)
4. Auto-merge near-duplicates (embedding cosine ≥ 0.92)
5. Prune zombies: mention_count ≤ 1, no neighbors, no chunks, score < prune_threshold
6. Re-score → write `dream-report-{tenant_id}.json`

Cycle 1 on D6 (20 completions, 4 workers): 35s, 7 type assignments, 4 summary enrichments, 5 relations added, Unknown count 88 → 81, overall health 59.5% → 59.6%.

The health score improvement per cycle is small but cumulative. The hypothesis — that a cleaner, more complete graph will lift entity-heavy question performance — will be measured at M18.

---

### What the numbers mean

The jump from 24.6% to ~50% represents a doubling of keyword retrieval accuracy, but the more meaningful signal is the judge score trajectory: from unmeasured → 1.85/2 (lenient) → 1.65/2 (calibrated) → **1.80/2** (iterative). At 1.80/2, 80% of questions get a fully correct answer and none get a completely wrong one.

The remaining gap to a perfect score is concentrated in:
- **q04** (book dedication): the word "dedicated" doesn't appear in the dedication text — BM25 is blind to it
- **q06** (Buitencingle) and **q14** (District Six): top_k=10 is too tight for broad descriptive questions
- **q13** (All Africa Convention): the right chapter is retrieved but the model hedges — likely a chunk density issue

Each of these has a clear fix path. The system is past the point where the main bottleneck is retrieval architecture; it is now in the tuning and cleanup phase.
