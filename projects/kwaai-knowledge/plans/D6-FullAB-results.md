# D6 full A/B — results

2026-08-26/27. Both arms rebuilt over all 1152 chunks, then evaluated on the
40-question set. First run with `KWAAI_EXTRACTION_TEMPERATURE=0` and
`--workers 1`, so vocabulary is the only difference between arms.

---

## 1. The headline is negative

| | control | ontology (v8) | |
|---|---|---|---|
| **recall** | **59.3%** | **55.5%** | **−3.8pp** |
| entities | 2723 | 2332 | −391 |
| relations | 2366 | 2126 | −240 |
| density (rel/ent) | 0.869 | 0.912 | +0.043 |
| distinct predicate types | 377 | 39 | −338 |
| undeclared-predicate rate | 26.5% | 2.6% | −23.8pp |

**The ontology arm is worse, and the targeted questions moved no differently
from the rest** — targeted −3.4pp, everything else −3.8pp. If the vocabulary
were doing the work it was designed for, the twelve questions it was built to
reach should have separated from the other twenty-eight. They did not.

24 of 40 questions changed (11 up, 13 down). That churn dwarfs the 3.8pp
aggregate, which is the signature of a comparison dominated by noise.

Predictions from the test plan: **1 (Address populates, 110 entities) PASS ·
2 (Doctrine populates, 23) PASS · 3 (uninformative falls) FAIL · 4 (density
holds) PASS · 5 (kinship unharmed) FAIL, 764 → 590 edges.**

## 2. Both arms are ~30pp below the production build, and that is my error

Historical D6 scores 88.5%. Both arms here sit near 57%, because I ran without
three flags the production build uses:

```
--seed-file   d6_family_tree.yaml    <- 10 of 40 questions are kinship
--doc-schema  d6_doc_schema.yaml     <- skips index/appendix, stops title-as-place
--no-relations                       <- the historical configuration
```

Dropping the family-tree seed is the significant one: it supplies kinship ground
truth for a quarter of the eval set. The arm-to-arm comparison is still
controlled — identical settings, pinned sampling — but **neither arm represents
how D6 is actually built**, so the −3.8pp should not be read as "the ontology
costs 3.8 points in production".

## 3. The residue explains the kinship failure

`original_predicate`, added the same day, recorded what the extractor emitted
before coercion. Over the full run: **760 coerced edges across 474 distinct
rejected predicates.** The top of that list is not exotic vocabulary — it is
alias gaps in my own ontology:

| rejected | edges | should have been |
|---|---|---|
| `lived_with` | 13 | arguably a missing type |
| `studied_at` | 12 | alias of `attended` |
| `played_at` / `played_with` | 20 | alias of `played_for` |
| `opposed_by` | 10 | the **inverse** of `opposed`, which I declared without one |
| `taught` | 8 | alias of `taught_at` |
| `father_of` · `mother_of` | 13 | alias of `parent_of` |
| `married_to` | 5 | alias of `spouse_of` |
| `brother_of` | 5 | alias of `sibling_of` |

**23 kinship edges were lost purely to missing aliases** on `father_of`,
`mother_of`, `married_to` and `brother_of`. That is a direct, mechanical
contribution to prediction 5's failure, and it is trivially fixable.

The mechanism paid for itself within hours of being built: without it these
edges would have been indistinguishable `associated_with`, and the kinship
regression would have had no explanation.

## 4. A wasted first eval, and the check that caught it

The first eval returned 20.1% / 12.4% with an empty Sources column on every
question. I had copied the chunk store but not the vector store — `metadata.db`,
5.2 MB and all 1152 embeddings — so retrieval had nothing to search. Caught
because ~1000ms average latency is impossible for real LLM answers; the
historical run averages ~6000ms.

Worth recording as a setup requirement: **cloning a KB means the chunk store,
the vector store, and the tantivy index.** Two of the three are easy to forget
and the failure looks like a result rather than an error.

## 5. What this does and does not establish

**Established:** the ontology closes the vocabulary — undeclared predicates fall
26.5% → 2.6%, distinct types 377 → 39. New types populate at scale: 152
PoliticalOrganization, 110 Address, 69 Venue, 23 Doctrine. That is structural
and not subject to sampling noise.

**Not established:** any retrieval benefit. Three prior runs and this one all
show the ontology arm at or below control.

**Not yet tested:** the ontology against the *production* configuration, with
the family-tree seed and doc schema, where kinship is not being reconstructed
from scratch and where the alias gaps in §3 are closed. That is the honest next
experiment and it is another ~9 hours.

Before spending it, the cheap fix is §3: add the eight missing aliases and the
`opposed_by` inverse. They are named, counted, and take minutes.
