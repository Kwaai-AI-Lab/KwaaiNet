# D6 ontology A/B — test plan

Status: **run, and superseded by its own results.** Written 2026-08-25;
executed 2026-08-26/27. Read
[`D6-FullAB-results.md`](./D6-FullAB-results.md) for the outcome and
[`OntologySession-assessment.md`](./OntologySession-assessment.md) §4b/§4d for
the three smoke runs that preceded it.

The headline: the ontology arm scored **−3.8pp** and the twelve questions the
vocabulary was built to reach did not separate from the other twenty-eight.
Predictions 1, 2 and 4 passed; **3 and 5 failed**. Apart from the resolution
note in §1, the document below is kept as written before the run — it is the
record of what was predicted, and §4 is only worth anything because it was
written first.

Tests whether a per-corpus ontology changes what extraction produces, on the one
KB where the ontology was built from a full reading of the text.

---

## 1. Prerequisite: the ontology does not drive extraction yet — resolved

**Done on `feat/per-kb-ontology` before the run, merged as #150 on 2026-08-31.**
Both call sites now take the ontology path: `classify_candidates_with_ontology`
is called from `ingestion.rs`, and `validate_relation_axioms_with_ontology` from
`rag_cmd.rs`. `extract_from_text` reads the ontology's entity and relation
lists. The table below is what was outstanding when the plan was written:

| site | then | needed |
|---|---|---|
| `ingestion.rs` | `classify_candidates_axiomatic(...)` | `classify_candidates_with_ontology(..., ont)` |
| `rag_cmd.rs` | `validate_relation_axioms(...)` | `validate_relation_axioms_with_ontology(..., ont)` |

Plus `extract_from_text` needs the ontology's entity and relation lists, and the
Phase-4 verify prompt needs `in_scope_relation_types(ont)`.

**Running the A/B before this lands measures nothing** — both arms would use the
same vocabulary. This is roughly a day, and it is the whole risk surface: these
functions feed the confidence split that decides what commits without an LLM.

## 2. Arms

Both are copies of D6's chunk store with separate graph directories, so D6's own
340-relation graph is never touched. Same chunks, same model, same settings,
local Ollama for both — a flaky peer would confound the comparison.

| arm | KB | vocabulary |
|---|---|---|
| control | `D6_ctl` | today's global 17 entity types / 35 predicates |
| ontology | `D6_narr` | D6 v6: 17 types, 27 predicates, 187 triggers, 5 axioms |

The full run reported in [`D6-FullAB-results.md`](./D6-FullAB-results.md)
actually used **v8**, and the version committed to
`ontologies/D6.yaml` is **v9** (18 types, 33 predicates, 240 triggers, 4 axioms)
— the aliases and the `opposed_by` inverse that [`D6-FullAB-results.md`](./D6-FullAB-results.md)
§3 identified were added after this plan was written.

Note the type *counts* are nearly identical; the vocabularies are not. That is
the point — this is not a test of "more types".

## 3. Metrics

Instrument-independent first, because they need no eval run and discriminate
without waiting on a weak exam.

| metric | why |
|---|---|
| **escape-hatch rate** | share of edges that are `associated_with`/`related_to`. If Doctrine, Venue and the 187 triggers do not move this, the vocabulary did not take. Readable straight after extraction. |
| **relation-type distribution** | do the new predicates fire at all — `advocates`, `denounced_as`, `attended`, `worships_at`, `classified_as`? |
| **entity-type distribution** | do `Address`, `Doctrine`, `Community`, `Venue` get populated, and does `Address` split from `Place`? |
| **axiom demotions** | already instrumented in `RelationAxiomMetrics`; a too-narrow ontology shows up here rather than as unexplained recall loss |
| **relation density** | D6 control is 0.225 rel/ent, the densest KB in the set |
| recall on the 40-question set | corroborating only — see §5 |
| ~~graph score~~ | excluded: hardcoded to the memoir ontology |

## 4. Specific predictions, recorded before the run

Falsifiable, so the run can disconfirm rather than merely describe:

1. **`Address` populates and separates from `Place`.** Street language is 24.2%
   of chunks and the control has no Address type. If this does not happen the
   markers are not firing at all, and nothing else in the run is trustworthy.
2. **`Doctrine` populates.** `boycott` alone occurs 42 times. Q40 asks about the
   Unity Movement's boycott policy.
3. **Escape-hatch rate falls from 8.5%.** More typed predicates should absorb
   edges that currently default.
4. **Relation density does not collapse.** A narrower vocabulary rejecting
   out-of-domain triples is the ghost-prune risk. Demotions are counted, not
   silent, so a fall shows up as demotions rather than a mystery.
5. **Kinship is unharmed.** 10 of 40 questions are kinship and D6's kinship
   extraction is the best in the corpus set. Any regression here fails the run
   outright, regardless of what else improves.

## 5. What this test cannot tell us

The 40-question set is a weak instrument: token-overlap scored, saturated near
90%, and relation count does not predict recall (Pearson +0.081 across 15 KBs).
So:

- a **large** move in the graph-side metrics is real signal;
- a **null on recall is not evidence the ontology did not help** — it is the
  known limit of the exam, and the same null the dream sweep produced.

Any conclusion of the form "improves recall by N points" waits for the
curriculum eval. "Moved 60% of D6's edges out of `associated_with`" does not.

## 6. Cost

~1150 chunks per arm. Local Ollama measured slower than 8s/chunk during a
probe, so budget several hours per arm and run them sequentially and in the
background with progress JSON. A 150-chunk smoke run on both arms first, to
confirm the new predicates fire before committing to the full build.

## 7. Method note this exercise produced

**Corpus frequency is not the same as reader demand.** The v3 method ranked
types purely by measured share of chunks. The full 40-question eval set then
showed two places where that is insufficient:

- **Q31 asks about a mosque.** No type existed for it. Mosque is 1.4% of chunks
  — below the bar — but as one `Venue` type with church and hall it is 11.9%,
  above `Community` and `SportsClub`, both of which were included. Added in v6.
- **Three questions (15, 34, 39) ask about the forced removals**, which are 1.3%
  of the text because the memoir ends in 1956 and the demolitions began in the
  1960s. This one is *not* fixed by adding a type: the text genuinely does not
  contain the answer. It is an honest limit of the corpus, and it should be
  recorded rather than engineered around.

Coverage of the 40-question set under v6: **38 of 40** cleanly answerable.
The two that are not are Q4 (document metadata, not graph content) and Q15
(the corpus does not carry it).
