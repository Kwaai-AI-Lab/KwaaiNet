# D6 RAG Accuracy Progress

**KB:** D6 memoir — _The Setting in Time and Place_ by Y.S. Rassool  
**Eval:** `tests/d6_eval_questions.json` · 20 questions · keyword hit rate + LLM-as-judge  
**Stack:** Rust KwaaiNet RAG · Ollama · llama3.1:8b

---

## Progress Chart

```
55% ┤
    │
50% ┤                                                             ████ 50.0% ← keyword best
    │                                                        ████ 49.1% ← iterative (judge best)
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
     P1    P2   P3  P7..11  exp    mini  fix  mxbai  auto  famseed  iterative
                                                           +judge   (new best)
```

**Judge score history:** — / — / — / — / — / — / — / — / — / — / 1.85 / 1.65 / **1.80** ← new best

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

> Note: keyword hit rate varies ±4pp between runs of the same config due to LLM sampling. Milestones 12–13 are separate runs of the same stack; consider 48–50% the range for the current best config.

---

## Judge Scores by Question

| ID | Question | M13 kw | M13 judge | M14 kw | M14 judge | Δ judge |
|----|----------|--------|-----------|--------|-----------|---------|
| q01 | Who is the author? | 2/3 | 1/2 | 3/3 | **2/2** | ⬆ +1 |
| q02 | Author's children? | 3/3 | 2/2 | 3/3 | **2/2** | = |
| q03 | Author's grandchildren? | 0/6 | 1/2 | 6/6 | **2/2** | ⬆ +1 |
| q04 | Book dedication? | 1/4 | 1/2 | 0/4 | 1/2 | = |
| q05 | Who was J.M.H. Gool? | 2/8 | 1/2 | 2/8 | **2/2** | ⬆ +1 |
| q06 | Tell me about Buitencingle. | 5/8 | 2/2 | 1/8 | 1/2 | ⬇ −1 |
| q07 | Author's wife? | 1/3 | 2/2 | 1/3 | **2/2** | = |
| q08 | More about wife? | 5/6 | 2/2 | 5/6 | **2/2** | = |
| q09 | Author's grandfather? | 4/9 | 2/2 | 2/9 | **2/2** | = |
| q10 | Kloof Nek? | 2/7 | 1/2 | 5/7 | **2/2** | ⬆ +1 |
| q11 | TLSA? | 2/6 | 2/2 | 3/6 | **2/2** | = |
| q12 | Who was Cissie Gool? | 4/6 | 2/2 | 3/6 | **2/2** | = |
| q13 | All Africa Convention? | 3/6 | 1/2 | 2/6 | 1/2 | = |
| q14 | Where was District Six? | 3/6 | 2/2 | 2/6 | 1/2 | ⬇ −1 |
| q15 | Forced removals? | 3/6 | 1/2 | 3/6 | **2/2** | ⬆ +1 |
| q16 | Gandhi / Gool family? | 3/7 | 2/2 | 2/7 | **2/2** | = |
| q17 | Hewat College? | 4/5 | 2/2 | 4/5 | **2/2** | = |
| q18 | New Era Fellowship? | 4/6 | 2/2 | 4/6 | **2/2** | = |
| q19 | NEUM? | 4/6 | 2/2 | 4/6 | **2/2** | = |
| q20 | Cricket? | 1/5 | 2/2 | 2/5 | **2/2** | = |

**M13 summary:** 11×2/2, 8×1/2, 1×0/2 → judge=1.65/2  
**M14 summary:** 16×2/2, 4×1/2, 0×0/2 → judge=**1.80/2** ⬆ new best

**Net gains:** q01, q03, q05, q10, q15 improved (+1 each)  
**Regressions:** q06 (Buitencingle −1), q14 (District Six −1) — top_k=10 vs 30 likely cause

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
| High | Re-test iterative at `--top-k 20` — fix q06/q14 regressions without losing q01/q03/q05/q10 gains | +1–2pp judge |
| High | `graph dedup` command — merge "Hassen Mall"/"Hassen", other Obsidian-visible dups | Cleaner graph → better entity retrieval |
| High | Chunk metadata injection for q04 (dedication) | +1pp judge on q04 |
| Medium | `--rerank` on iterative eval — both flags exist, never combined | +1–2pp |
| Medium | 3-run eval average — reduce ±4pp variance to ±1–2pp | Diagnostic |
| Low | Fan-out entity extraction to metro nodes (v0.4.53 P2P Ollama proxy) | Faster graph rebuild |
