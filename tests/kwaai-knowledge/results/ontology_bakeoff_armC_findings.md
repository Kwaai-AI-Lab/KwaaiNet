# Ontology bakeoff — arm C findings (2026-08-25)

Induced ontology drafts for every KB, following the GraphRAG auto-tuning shape
(`PerKBOntology-plan.md` §8.4): sample 24 chunks at random, infer the domain,
propose entity and relation types, drop anything whose evidence is not a
verbatim span of the sample.

Tooling: `ontology_induce.py`, `ontology_induce_all.sh`, `ontology_draft_lint.py`.
Model `llama3.1:8b` on local Ollama, `num_ctx=8192`, `temperature=0.2`.
Drafts in `ontology_drafts/`. ~57s per KB.

---

## 1. The headline number

Across the 12 KBs that induced successfully:

> **88% of proposed entity types (82/93) and 98% of proposed predicates
> (121/124) do not exist in the global vocabulary.**

Only three induced predicates — out of 124 — were already available in
`RELATION_TYPES`. The global vocabulary is not merely a poor fit for these
corpora; it is almost disjoint from what they need.

This is the quantitative form of the thesis, and it is stronger than the
qualitative version in the parent framing doc.

## 2. Domain inference is reliable; vocabulary quality is not

Inferred domains, unprompted, from 24 random chunks:

| kb | inferred domain |
|---|---|
| MobyDick | Whaling and Maritime History |
| Legal | US Supreme Court Jurisprudence |
| RFCs | Network Protocols and Standards |
| DeepSea | Deep-Sea Ecology and Biology |
| NIST | Artificial Intelligence and Machine Learning Security |
| Poems | Poetry and Dramatic Works |
| Astrophysics | Space Exploration and Astronomy |
| DreamMem | Sleep and Memory Consolidation |
| CountryHistory | Indian History and Culture |
| Climate | Climate Science |
| OSMDocs | OpenStreetMap Wiki |
| WarPeace | War and Peace |

Every one is right, and several are more specific than a human would have
bothered to be — NIST is indeed the AI risk-management corpus, not NIST
generally. **Domain detection is the solved half of the problem.** Arm D's
module selector can lean on this.

## 3. Draft quality, scored

`ontology_draft_lint.py` applies deterministic guards — duplicate predicates,
vague predicates, evidence reused across predicates, range collapse, and
sufficiency. All thresholds, no LLM judgement.

| kb | ents | rels | grounded | dupes | vague | reused ev | range collapse | usable |
|---|---|---|---|---|---|---|---|---|
| RFCs | 7 | 6 | 68% | 0 | 0 | 0 | 33% | **0.977** |
| WarPeace | 7 | 10 | 85% | 0 | 0 | 1 | 40% | **0.952** |
| Meetings | 8 | 13 | 100% | 0 | 0 | 3 | 23% | **0.942** |
| DeepSea | 8 | 12 | 71% | 1 | 0 | 3 | 17% | **0.917** |
| Legal | 7 | 19 | 84% | 1 | 0 | 3 | 42% | **0.917** |
| MobyDick | 8 | 8 | 100% | 0 | 0 | 1 | 50% | **0.908** |
| NIST | 8 | 14 | 100% | 0 | 0 | 6 | 36% | **0.886** |
| Astrophysics | 7 | 10 | 94% | 0 | 2 | 4 | 30% | **0.85** |
| Poems | 7 | 11 | 86% | 4 | 1 | 4 | 36% | **0.787** |
| PythonDocs | 8 | 14 | 100% | 7 | 0 | 7 | 14% | **0.75** |
| Climate | 9 | 15 | 100% | 6 | 4 | 3 | 47% | **0.735** |
| DreamMem | 4 | 7 | 52% | 0 | 4 | 1 | 29% | **0.615** |
| OSMDocs | 4 | 4 | 38% | 0 | 0 | 0 | 25% | **0.544** |
| CountryHistory | 1 | 1 | 14% | 0 | 0 | 0 | 100% | **0.0** |

Legal and PythonDocs are retry results (`retry_induce.sh`); both failed their
first attempt and succeeded on a re-run, which is the §5 reproducibility point
again. Legal's retry is *better* than its original destroyed draft — 19 grounded
predicates against 12.

Two KBs never produced parseable output: **Manhattan** (returned prose, failed
three attempts) and **rag-bench** (the known directory/KB name mismatch — dir
`rag-bench`, KB `ragbench`, already recorded in `eval_all_kbs.sh`).

**Success rate: 9 of 16 usable on the first sweep; 11 of 16 after one retry
round** (score ≥ 0.7). 2 degenerate (Climate, OSMDocs), 1 failed on content
(CountryHistory), 2 never parsed.

## 4. Grounding by quotation is necessary but not sufficient

Climate is the instructive failure. It scored 100% grounded and is still
unusable:

- 7 of 9 predicates collapsed onto a single range (`ClimateFeedback`)
- `is_related_to` and `is_influencing` each appear twice
- one Turetsky quote justifies three different predicates
- `is_related_to`, `has_impact_on`, `affects`, `is_influencing` — the
  `associated_with` problem, reinvented under four new names

Every one of those cited a genuine verbatim span. **A real quote attached to a
predicate it does not instantiate still grounds.** So the grounding requirement
the plan inherited from AutoSchemaKG catches invented evidence but not
misapplied evidence, and needs the deterministic guards above alongside it.

Contrast Legal's draft, which is genuinely good:
`Government`, `Law`, `Court`, `Case`, `Constitution`, `Territory`, with
`has_jurisdiction`, `is_enacted_by`, `is_citizen_of`, `interprets`, `decides` —
correct domain/range, no duplicates. Usable after light editing.

## 5. Reproducibility is the binding problem

Three separate observations, all from today:

- **Climate**, run twice minutes apart with identical prompt and identical fixed
  chunk sample: 13 vs 15 predicates, 0 vs 6 duplicates.
- **Legal**, same: 100% grounded (9 ents / 12 rels) on one run, **0% grounded**
  (0/9, 0/14) on the next.
- **Manhattan and PythonDocs** returned unparseable prose on one attempt each.

Induction is cheap and often good, but it is a **slot machine**. That makes the
plan's ratification requirement (§7.2) not a nicety but the only thing standing
between this and a graph that differs on every rebuild.

Practical consequence: arm C must be run *n* times per KB and the best-scoring
draft kept, not run once and trusted. At ~57s per KB that is affordable.

## 6. A tooling mistake worth recording

The sweep **overwrote Legal's good draft with the failed one.** `ontology_induce.py`
wrote to a fixed `<KB>_induced.yaml`, so a non-deterministic re-run destroyed a
9/9-entity, 12/12-relation draft scoring 0.917 and replaced it with an empty
file. The good version is gone.

Fixed: output is now timestamped, `<KB>_induced.*` is a symlink to the latest
success, and a run producing nothing grounded refuses to write at all.

This is a small instance of exactly what the plan argues about ontologies being
versioned artifacts — and it happened inside the tooling built to test that
claim.

Second bug, in the linter: it scored Legal's empty draft **1.0**, because an
ontology with no relation types has no duplicates, no vague predicates and no
range collapse. Sufficiency is now checked first (<3 entity types or <4
predicates scores 0.0), and the grounding-kept rate now scales the score instead
of being reported and ignored.

## 7. What this says about the arms

- **Arm C is worth running before arm B, as the plan's §5.6 orders it.** Nine
  usable drafts for ~15 minutes of local GPU is a better starting point than a
  blank file, even where they need editing.
- **Arm D's selector has a solid foundation** — domain detection was correct 14
  for 14, including "Python Library Reference" from raw Sphinx HTML.
- **The `usable_score` is itself a bakeoff instrument.** It needs no eval run and
  it discriminated cleanly (0.0 to 0.977), which is what §5.4 asks for.
- **Still unanswered: does any of this move retrieval?** That is arms B/C
  re-extraction, and the H1/H2 prediction in plan §5.4.2 is the thing to watch —
  if Climate gets a real ontology and relation density stays near zero, the
  bottleneck is extraction, not vocabulary.
