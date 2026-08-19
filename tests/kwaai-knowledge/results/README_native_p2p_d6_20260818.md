# Native-p2p D6 validation — 2026-08-18

Read this before trusting any single report in this run set. **Two of the four
evals here scored catastrophically low for reasons that have nothing to do with
RAG accuracy**, and are kept only as evidence of the failure mode.

Harness: `../native_p2p_d6_rebuild.sh` (`PEERS=` and `WORKERS=`).
Transport: native-p2p with the V1Lazy fix (PR #107). Model: `llama3.1:8b`.
Production `D6` was never modified — the rebuild targets `D6_native`.

## The results

| Report | Graph | Recall | Verdict |
|---|---|---|---|
| `control_D6_prod_native_20260818_160106.md` | production D6 | **90.0%** (188.0/209) | **The headline.** Same graph as the 90.4% p2pd baseline, re-evaluated over native |
| `native_p2p_d6_eval_metrowin_20260818_154848.md` | fresh `D6_native` | **83.5%** (174.6/209) | Valid, clean run. Lower because a single-pass rebuild lacks the refinement production D6 has accumulated |
| `native_p2p_d6_20260818_114949_eval.md` | fresh `D6_native` | 9.6% | **Invalid.** Eval routed via `p2p://auto` to a VRAM-starved peer → 7 × 30 s timeouts → circuit breaker → 30 questions failed in ~109 ms |
| `native_p2p_d6_eval_metrowin_20260818_154708.md` | fresh `D6_native` | 0.5% | **Invalid.** Issue #108 (loopback dial) → 3 dial failures → circuit breaker latched → 37 questions failed in ~109 ms. Fixed by restarting the node |

## What it establishes

**Native transport is accuracy-neutral: 90.0% vs 90.4% on the identical graph** —
0.4pp, inside LLM nondeterminism.

**Throughput is at parity per peer.** The graph build did 1152/1152 chunks with
**zero** transport errors at 0.12 chunks/s on one peer with 2 workers; the p2pd
baseline was 0.24 chunks/s across two peers with 4 workers — the same 0.12 per
peer. Extraction was *richer*, not lossier: 2083 entities vs the baseline's 1983.

## Two traps this run walked into

**Workers are per-run, not per-peer.** 4 workers on a single peer overloads its
Ollama, which answers 502/503 instead of extracting. Dropping to 2 took errors
from 142 to zero. Keep it near 2 per peer.

**A peer can look perfectly healthy and still be unusable.** metro-linux answered
a probe in 22.5 ms and advertised 32/32 blocks VERIFIED while failing ~50% of
inference calls (reduced VRAM). Liveness checks do not catch this — only the
workload does, and it surfaces as a *RAG accuracy* regression.

**Corollary: audit completeness before believing any score.** Chunks that exhaust
their retries drop out of the graph silently. Check `chunks_done` and the entity
count against a baseline first; the `*.progress.json` files here exist for that.

## Not committed

Run logs (`*.log`) and the 1 MB coref intermediate are deliberately omitted — no
`.log` file is tracked anywhere under `results/`.
