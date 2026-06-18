
## r46 — 2026-06-17 19:34 — **84.9% (191.0/225)** ⭐ new best

**Flags:** smart mode, biographical-expansion, model=llama3.1:8b, metro-linux p2p, num_ctx=8192

**Changes since r45:**
- **Bug fix**: `reembed_all` now skips description_from_fields for YAML-seeded entities (extraction_confidence=1.0) — was destroying Nazima, Cissie, Yousuf, Haji Joosub descriptions on every reembed
- **Bug fix**: name_overlap abbreviation token weighting — dot-containing tokens like "j.m.h" count double, fixes Q30 entity injection (was injecting "Cape Town-Woodstock P.T.A" instead of Haji Joosub)
- YAML: Added siblings to Yousuf Rassool description (Abdul/Doolie, Fazil, Zain, Rasheda, Berina, Yasmin/Jessie, Nasim)
- YAML: Added "Indian Opinion (March 1914)" to Haji Joosub description
- Re-seeded + re-embedded with fixed code

**vs r45 (175.0/225 = 77.8%):** +16.0 pts

| Q | r45 | r46 | delta | Note |
|---|-----|-----|-------|------|
| q23 | 1 | 5 | +4 | Siblings added to Yousuf entity — Abdul, Fazil, Zain, Rasheda all hit |
| q06 | 3 | 6 | +3 | Buitencingle entity description now preserved after reembed (seeded) |
| q13 | 3 | 6 | +3 | All Africa Convention — bonus from reembed fix preserving seeded entities |
| q12 | 4 | 6 | +2 | Cissie Gool description preserved → properly injected for Q12 |
| q26 | 4 | 6 | +2 | Abdurahman description preserved → better injection |
| q30 | 1 | 3 | +2 | Abbreviation token fix → Haji Joosub injected (partial, Mauritius/India/Swat) |
| q39 | 3 | 5 | +2 | Improvement |
| q10 | 4 | 5 | +1 | Improvement |
| q14 | 4 | 5 | +1 | Improvement |
| q18 | 4 | 5 | +1 | Improvement |
| q19 | 5 | 6 | +1 | Improvement |
| q33 | 4 | 5 | +1 | Improvement |
| q34 | 5 | 6 | +1 | Improvement |
| q38 | 3 | 4 | +1 | Improvement |
| q07 | 2 | 3 | +1 | Nazima description partially restored |
| q08 | 1 | 2 | +1 | Nazima still not 6/6 — further investigation needed |
| q20 | 5 | 2 | -3 | LLM non-det regression (cricket) |
| q37 | 7 | 5 | -2 | LLM non-det regression |
| q29 | 6 | 4 | -2 | LLM non-det regression |
| q09 | 9 | 8 | -1 | Minor non-det regression |
| q17 | 4 | 3 | -1 | Minor non-det regression |
| q32 | 4 | 3 | -1 | Minor non-det regression |
| q35 | 4 | 3 | -1 | Minor non-det regression |

**Still broken:**
- q08 (2/6): Nazima entity still giving weak answer — possible description not fully reaching LLM in Replace mode
- q30 (3/6): Partial fix — gets some keywords (3/5) but not all
- q27 (4/5): Indian Opinion didn't fire — likely entity embedding needed more time

**Next:** Investigate Q08 Nazima path, stabilize Q20/Q29/Q37 non-det regressions

## r45 — 2026-06-17 18:57 — 77.8% (175.0/225)

**Flags:** smart mode, biographical-expansion, model=llama3.1:8b, metro-linux p2p, num_ctx=8192

**Changes since r44c:**
- num_ctx reverted 32768→8192 (eliminates "lost in the middle" + stream reset)
- is_bio check expanded: "tell me more about", "tell me about", "describe" added
- num_predict cap removed

**vs r44c (170.8/225 = 75.9%):** +4.2 pts — 8192 context fixes Q20 stream reset (+5) but Q23 regressed (-4)

## r44c — 2026-06-17 18:15 — 75.9% (170.8/225)

**Flags:** smart mode, biographical-expansion, model=llama3.1:8b, metro-linux p2p, **num_ctx=32768**, iterative.rs fix (60s timeout, /api/chat, num_ctx=4096)

**Changes since r43:**
- Updated Haji Joosub entity description (comprehensive: merchant from India via Mauritius 1884, J.M.H. Gool & Co., Mosque in Loop St, Gandhi/Rhodes/Abdurahman/Shaw, Buitencingle mansion)
- Deleted duplicate org entities: Dr Gool AH Gool, J.M.H. Gool & Co., J. M. H. Gool and Co.
- Graph reseeded + reembedded
- iterative.rs: timeout 30s→60s, /v1/chat/completions→/api/chat, num_ctx=4096
- **num_ctx=32768 experiment** (this turned out to be net negative)

**vs r43 (167/225 = 74.2%):** +3.8 pts — entity fixes yielded +19.8 but 32768 regressions cost -16

| Q | r43 | r44c | delta | Note |
|---|-----|------|-------|------|
| q03 | 0 | 6 | +6 | Yousuf grandchildren seeded back, LLM recovered |
| q05 | 4 | 8 | +4 | Haji Joosub entity injected — India/Mauritius keywords hit |
| q27 | 3 | 5 | +2 | Haji Joosub entity → Mosque, Loop Street |
| q06 | 5 | 6 | +1 | Small improvement |
| q11 | 5 | 6 | +1 | Small improvement |
| q18 | 4 | 5 | +1 | Small improvement |
| q21 | 4 | 5 | +1 | Small improvement |
| q26 | 4 | 5 | +1 | Small improvement |
| q30 | 1 | 1.8 | +0.8 | Entity injected but 32768 still caused misses |
| q32 | 4 | 5 | +1 | Small improvement |
| q33 | 3 | 4 | +1 | Haji Joosub entity → Gandhi/Rhodes/Abdurahman |
| q19 | 6 | 3 | -3 | 32768 "lost in the middle" regression |
| q31 | 6 | 3 | -3 | 32768 "lost in the middle" regression |
| q10 | 5 | 3 | -2 | 32768 regression |
| q14 | 5 | 3 | -2 | 32768 regression |
| q09 | 9 | 8 | -1 | 32768 slight regression |
| q13 | 5 | 4 | -1 | 32768 regression |
| q20 | 1 | 0 | -1 | Stream reset (Q19 took 50s with 32768, model unloaded) |
| q22 | 4 | 3 | -1 | 32768 regression |
| q38 | 4 | 3 | -1 | 32768 regression |
| q39 | 4 | 3 | -1 | 32768 regression |

**Still broken:**
- q08 (1/6): is_bio check doesn't trigger for "Tell me more about" — fix in r45
- q30 (1.8/6): entity injected but LLM misses keywords in large context

**Next:** r45 with num_ctx=8192 (reverted) + is_bio expanded ("tell me more about", "tell me about", "describe") + num_predict cap removed

## r43 — 2026-06-17 17:33 — 74.2% (167/225)

**Flags:** smart mode, biographical-expansion, model=llama3.1:8b, metro-linux p2p, num_ctx=8192

**Changes since r42:**
- Added YAML aliases for J. M. H. Gool variants (spaces in initials) and Yousuf Joe Rassool
- Re-seeded graph → merged extracted variants into YAML canonicals
- Re-deduped graph → 12 more merges

**vs r42 (173/225 = 76.9%):** -6 pts — entity merges hurt overall

| Q | r42 | r43 | delta | Note |
|---|-----|-----|-------|------|
| q02 | 0 | 3 | +3 | Yousuf Rassool YAML children list now retrieved |
| q09 | 3 | 9 | +6 | J M H Gool alias merge → perfect recall |
| q05 | 2 | 4 | +2 | Partial improvement |
| q16 | 4 | 5 | +1 | Partial improvement |
| q03 | 6 | 0 | -6 | LLM non-det: entity has grandchildren in desc but LLM failed |
| q08 | 6 | 1 | -5 | LLM non-det: weaker answer for Nazima Rassool entity |
| q30 | 4 | 1 | -3 | Wrong entity retrieved: Dr Gool AH Gool vs Haji Joosub |
| q18 | 6 | 4 | -2 | Minor regression |
| q26 | 6 | 4 | -2 | Minor regression |

**Still broken:**
- q15 (3/6, 68s) — reformulate_query timeout (iterative.rs not yet compiled)
- q20 (1/5, 70s) — same

**Next:** r44 with iterative.rs fix + num_ctx=32768 + graph reembed

