# Pipeline Ordering Experiment Plan

**Status:** Ready to run  
**Script:** `tests/kwaai-knowledge/ordering_experiment_1pct.sh`  
**KB:** D6 ("District Six — Lest We Forget")  
**Current baseline:** 53.3% recall, 2049 entities, 220 relations

---

## Background

The D6 pipeline has five post-build steps that modify the graph in meaningful ways. Until now
they have been run in a fixed order that was never formally tested. Two sessions of root-cause
work on false relations (Yousuf `spouse_of` Wahida, `sibling_of` Khadija, etc.) identified
three interactions that make ordering matter:

1. **Coref needs clean entity candidates.**  
   The pronoun resolver picks the best match from the live entity list. If "Abdul Hamid Gool"
   and "A.H. Gool" are still separate nodes when coref runs, `his` may resolve to the wrong
   one. Running dedup first collapses duplicates, giving coref a clean single candidate per
   person.

2. **Coref needs correct gender.**  
   `resolve_pronouns_from_candidates` uses gender to pick between candidates. In every run so
   far, only Yousuf had `gender: Male` set in the graph, so every `his` resolved to Yousuf —
   the root cause of the false spouse/sibling relations. The seed now sets gender for all 47
   persons. Enrich adds gender for un-seeded entities from text evidence. Whether enrich should
   run before or after coref is the key open question.

3. **Extract-relations consumes stored coref resolutions.**  
   The CC+EC extraction loads stored pronoun→entity mappings from the DB when building its
   prompts. Better coref quality → fewer hallucinated relations. Enrich may also improve
   extract-relations indirectly by giving entities richer descriptions for the LLM to reason
   against.

---

## What we're testing

The five post-build steps, their purpose, and the cost of getting the order wrong:

| Step | Command | What it does | Risk if too early | Risk if too late |
| --- | --- | --- | --- | --- |
| `seed` | `graph seed` | Injects canonical names, aliases, gender, ground-truth relations from YAML | — always first | Missing gender breaks coref |
| `dedup` | `graph dedup` | Merges duplicate entity nodes | Duplicate candidates confuse coref | Coref resolves to wrong node |
| `coref` | `graph coref` | Resolves `his/her/the author` to canonical entities | Resolves against dirty list | extract-rel gets no pronoun mappings |
| `enrich` | `graph enrich-entities` | Builds descriptions + gender from chunk text | Gender used by coref may be wrong | extract-rel has no description context |
| `extract-rel` | `graph extract-relations` | CC+EC family relation extraction | Always last | — |

---

## The three orderings

All orderings start identically: `build → seed`. The graph build uses `--no-relations`
(entity extraction only) and seeds the family tree YAML. The clones branch from this
common base.

### Ordering A — Proposed

```text
seed → dedup → coref → dedup → enrich → extract-rel → dedup
```

**Rationale:**  
Dedup before coref: clean candidate list for pronoun resolution. Coref before enrich: coref
adds narrator links to chunks, which enrich can use when building descriptions. Second dedup
after coref: coref sometimes reveals hidden duplicates (two stubs both resolve to the same
narrator). Enrich last before extract-rel: descriptions are available for the LLM.

**Predicted strength:** Lowest false-relation count because coref has clean, gendered entities.  
**Predicted weakness:** Enrich runs before its output can influence coref.

---

### Ordering B — Coref before dedup (current/simpler)

```text
seed → coref → dedup → enrich → extract-rel → dedup
```

**Rationale:**  
Coref runs immediately after seed. Gender from seed is available (all 47 persons seeded),
but un-seeded entities still have `gender: null`. Dedup runs after coref, so the coref pass
may process duplicate entities as separate candidates.

**Predicted strength:** Simpler pipeline; no double-dedup cost.  
**Predicted weakness:** More pronoun misattributions because duplicate entity nodes exist
during coref (e.g., two forms of "Abdul Hamid" are separate candidates, splitting confidence).

---

### Ordering C — Enrich before coref

```text
seed → dedup → enrich → coref → extract-rel → dedup
```

**Rationale:**  
Enrich runs before coref, populating gender for un-seeded entities from text evidence. Coref
then has the fullest possible gender coverage when resolving pronouns. The question is whether
the gender extracted by enrich (LLM-based) is reliable enough to help, or whether it
introduces noise.

**Predicted strength:** Coref has the most complete gender information.  
**Predicted weakness:** Enrich quality depends on entity descriptions being clean before
gender extraction, which requires good dedup first — that ordering is satisfied here.

---

## Metrics

For each ordering we capture:

| Metric | How measured | Why it matters |
| --- | --- | --- |
| **False relations (Yousuf)** | Count lines matching `Yousuf Rassool.*spouse_of\|sibling_of` in extract-rel log, filtered against known-correct targets | Primary quality signal — the bug we fixed |
| **Coref resolutions** | Count `→ **` lines in coref output | Proxy for how many pronouns were resolved vs left ambiguous |
| **Eval recall** | `kwaainet rag eval` — 40 questions, token-overlap scoring | End-to-end RAG quality |
| **Graph health** | `graph score` Overall | Structural completeness of entity records |
| **Entities / relations** | `graph stats` | Sanity check — should be stable across orderings |

The deciding metric is **false relations for Yousuf Rassool**. All three orderings should
show improvement over the baseline (which had 6 false relations) because the CC+EC guards
(`quote_lacks_schema`, `quote_insufficient_endpoints`) are now in place. What we're measuring
is whether correct ordering provides additional signal beyond the code-level guards alone.

---

## Infrastructure

| Machine | Role in experiment | Model |
| --- | --- | --- |
| metro-linux (A6000) | Graph build + Ordering A inference | llama3.1:8b (build), llama3.1:8b (extract-rel) |
| metro-win (A5000) | Ordering B inference | llama3.1:8b |
| Jerome's machine | Ordering C inference | llama3.1:8b |
| Local M4 Pro | Embedding (all orderings) | nomic-embed-text |

Extract-relations uses `--sample 0.01` (1% ≈ 11 chunks) for the first pass. This is fast
(~2 min per ordering) and sufficient to see whether false relations are being generated.

Enrich uses `--min-mentions 1` so seeded entities with few text mentions still get gender
extracted if evidence exists in their linked chunks.

---

## Decision criteria

### From 1% → 10%

If the winning ordering (fewest false Yousuf relations) is consistent across at least 5 of
the ~11 sampled chunks, promote to `--sample 0.10`. Re-run only the winning ordering.

Change `EXTRACT_SAMPLE=0.01` → `EXTRACT_SAMPLE=0.10` in the script, or run:

```bash
kwaainet rag graph extract-relations --kb D6_ord_A \
  --inference-url $METRO_LINUX \
  --model llama3.1:8b \
  --sample 0.10 --commit \
  --output results/extract_rel_ord_A_10pct_$(date +%Y%m%d).md
```

### From 10% → 100%

If the 10% run produces precision > 80% on extracted relations (manually spot-checked), run
the full pipeline with:

- `--sample 1.0`
- `--model llama3.1:70b-instruct-q3_K_M` (better precision for full run)
- Both metro machines for throughput

### If all three orderings produce the same result

The code-level guards are sufficient; ordering doesn't matter at 1%. Skip to 10% with the
proposed Ordering A (best theoretical foundation) and measure end-to-end eval recall instead.

---

## What we're NOT testing

- **LLM model quality differences** — all orderings use llama3.1:8b to isolate the ordering
  variable. The 100% run will switch to 70b.
- **Chunk size or window variations** — fixed at 100w / `--graph-window 1` per Phase 2
  confirmed settings (CLAUDE.md).
- **Relations enabled during build** — always disabled; extract-rel is the only relation
  source. This was confirmed as correct in the root-cause analysis.

---

## Open questions

1. **Does enrich gender extraction help coref at 1% scale?**  
   The 47-person seed covers the most frequently-mentioned persons. At 1% (11 chunks), we're
   unlikely to hit chunks that mention un-seeded persons who need enrich-derived gender. The
   ordering difference between A and C may only become visible at 10%+.

2. **Is `--no-llm` coref sufficient, or does Tier 2 (LLM-assisted) matter?**  
   All three orderings use `--no-llm` for speed. If false relations persist, the next
   experiment should test `--no-llm` vs full coref with `--inference-url`.

3. **Does a second dedup pass after coref actually find new duplicates?**  
   Ordering A runs dedup twice before extract-rel; B and C run it once. If the second pass
   finds nothing, it can be dropped from the canonical pipeline.

---

## Running the experiment

```bash
cd /Users/rezarassool/Source/KwaaiNet
nohup bash tests/kwaai-knowledge/ordering_experiment_1pct.sh \
  > tests/kwaai-knowledge/ordering_experiment.log 2>&1 &
echo "PID: $!"
```

Estimated duration: ~30 min rebuild + 3 × ~25 min orderings = **~2 hours total**.

Monitor:

```bash
tail -f tests/kwaai-knowledge/ordering_experiment.log
cat ~/.kwaainet/rag/D6_ord_A/progress.json   # after clones are created
```

Results will appear in:

```text
tests/kwaai-knowledge/d6_experiments_log.md  ← comparison table
tests/kwaai-knowledge/results/eval_D6_ord*.md
tests/kwaai-knowledge/results/extract_rel_D6_ord*.md
```
