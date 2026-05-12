# D6 RAG Accuracy Progress

**KB:** D6 memoir — _The Setting in Time and Place_ by Y.S. Rassool  
**Eval:** `tests/d6_eval_questions.json` · 20 questions · keyword hit rate + LLM-as-judge  
**Stack:** Rust KwaaiNet RAG · Ollama · llama3.1:8b

---

## Progress Chart

```
55% ┤
    │
50% ┤                                                             ████ 50.0% ← run-to-run best
    │                                                        ████ 48.3% ← judge run (stable)
45% ┤                                              ████ 44.8%
    │                                         ████ 44.0%
    │                                    ████ 43.1%
40% ┤                    ████ 37.9%
    │               ████ 33.3%
35% ┤          ████
30% ┤     ████ 25.0% ── 31.9% ── 33.6% ← experiments (reverted)
25% ┤████ 24.6%
    │
    └───────────────────────────────────────────────────────────────────
     P1    P2   P3  P7..11  exp    mini  fix  mxbai  auto  famseed+judge
```

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

> Note: keyword hit rate varies ±4pp between runs of the same config due to LLM sampling. Milestones 12–13 are separate runs of the same stack; consider 48–50% the range for the current best config.

---

## Judge Scores by Question (Milestone 13 — calibrated judge)

| ID | Question | Keywords | Judge | Assessment |
|----|----------|----------|-------|------------|
| q01 | Who is the author? | 2/3 | **1/2** | Retrieves Joe Rassool but not Y.S. Rassool attribution |
| q02 | Author's children? | 3/3 | **2/2** | Solid |
| q03 | Author's grandchildren? | 0/6 | **1/2** | High run variance — correct in some runs, not others |
| q04 | Book dedication? | 1/4 | **1/2** | Finds dedication target but misses names |
| q05 | Who was J.M.H. Gool? | 2/8 | **1/2** | Gets 2 wives but misses merchant/Gujarat/Buitencingle context |
| q06 | Tell me about Buitencingle. | 5/8 | **2/2** | Solid |
| q07 | Author's wife? | 1/3 | **2/2** | Correct; keyword miss is phrasing not content |
| q08 | More about wife? | 5/6 | **2/2** | Solid |
| q09 | Author's grandfather? | 4/9 | **2/2** | Correct; keyword miss is phrasing |
| q10 | Kloof Nek? | 2/7 | **1/2** | Partial — gets location, misses social history |
| q11 | TLSA? | 2/6 | **2/2** | Correct; keywords too specific |
| q12 | Who was Cissie Gool? | 4/6 | **2/2** | Solid — family seeding helped |
| q13 | All Africa Convention? | 3/6 | **1/2** | Partial |
| q14 | Where was District Six? | 3/6 | **2/2** | Solid |
| q15 | Forced removals? | 3/6 | **1/2** | Retrieves context but hedges on detail |
| q16 | Gandhi / Gool family? | 3/7 | **2/2** | Solid — graph relations helped |
| q17 | Hewat College? | 4/5 | **2/2** | Solid |
| q18 | New Era Fellowship? | 4/6 | **2/2** | Solid |
| q19 | NEUM? | 4/6 | **2/2** | Solid — graph helped |
| q20 | Cricket? | 1/5 | **2/2** | Correct content; keywords too specific |

**Summary:** 11 questions fully correct (2/2), 8 partial (1/2), 1 zero (0/2)

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
| q01 author (1/2) | intro.docx is written in 3rd person — Y.S. Rassool not identified as the narrator | Metadata injection: prepend `[Author: Y.S. Rassool]` to intro chunks |
| q04 dedication (1/2) | Dedication text doesn't use the word "dedicated" — BM25 blind spot | Metadata injection: tag dedication chunk as `[Document type: dedication page]` |
| q05 JMH Gool (1/2) | Entity merged + family tree planted but description keywords sparse | Re-run graph build for family chapters with new family relation types |
| q03 grandchildren (variable) | Content exists in intro but borderline retrieval — sometimes found, sometimes not | Investigate intro chunk sizes and context window |

---

## What Changed at Each Phase

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
| High | Graph refinement: re-run `graph build` with new family relation types for family-dense chapters | +2–5pp on q05/q09/q16 |
| High | Chunk metadata injection for q01 (author) + q04 (dedication) | +2–4pp |
| Medium | `--rerank` and `--understand` on eval — both flags exist, never measured | +1–3pp each |
| Medium | 3-run eval average — reduce ±4pp variance to ±1–2pp for reliable comparison | Diagnostic |
| Low | HyDE blending (alpha=0.5) — prevents factoid regression if HyDE adopted | Defensive |
