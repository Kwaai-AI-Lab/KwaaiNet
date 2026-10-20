# DreamRAG Eval v3 — plan (draft for Reza's review)

Status: **draft, 2026-09-27 night**, written while the ontology pilot ran. This is not a
pre-registration until Reza approves it. Once approved, the fixed parts are copied into
`eval2/prereg_v3.md` and committed before any cycle ≥ 1 evaluation finishes, as for v2.

## Why a v3

Eval v2 asked the right question with a broken instrument. Measured after the run (see
`OntologySelection-plan.md` and the Eval v2 deviations log):

1. **No ontology in any graph.** Extraction offered three entity types per corpus, which is how
   DeepSea came out 42% Publication and Climate 46% Organization. Dream offered five generic
   predicates, which made up 70–99% of every non-memoir corpus's arm-B edges; a third of them were
   `associated_with`. So arm B tested whether generic edges help, not whether relation completion
   does.
2. **`--graph-window 1` read unrelated chunks.** Chunks were processed in key order, where 1 in
   3,722 adjacent pairs are real neighbours. Every window was a random chunk from elsewhere in the
   KB.
3. **Two primary corpora were too dirty to measure:** Climate (68.7% clean) and DeepSea (75.2%).
   Manhattan is 54% non-prose, and two of its questions ask about documents it does not contain.
4. **Arm and hardware were confounded:** every arm-A dream ran on CPU (metro-linux) and every arm-B
   dream on GPU (metro-win).

The question is unchanged:

> Does offline dream consolidation, which raises a knowledge graph's structural completeness,
> make retrieval-augmented answers better?

## What carries over unchanged

- The **hypotheses H1–H3**.
- The **endpoints**: primary context-nugget coverage (NLI, θ = 0.5); key secondaries answer
  nugget recall and Claude-graded nDCG@20; other secondaries descriptive.
- The **analysis rules**: developed on the noise floor only, run once; the question as the unit;
  exclusions.
- The **gold method**: NotebookLM answers, then nuggets, then NLI verification, then Claude
  adjudication, then Reza's review, then freeze.
- The **judge** (`claude-opus-5`, `relevance-v1`), the **NLI model** and **llama3.1:8b** for
  extraction, dream and answers.
- The Eval v2 **driver** and its 2026-09-27 fixes: dumps read with `read_jsonl`, and guards
  against `(error:…)` answers and empty dream cycles.

## What changes

| | Eval v2 | Eval v3 |
|---|---|---|
| Corpora (primary) | Manhattan, DeepSea, Legal, Climate | **Legal, Astrophysics, NIST, Manhattan_v3**: clean, with checked gold. WarPeace and MobyDick are out: their KBs are incomplete, and complete they are too large (see Corpus completeness) |
| Secondary | D6 | D6, unchanged role (development set, never sent to the API) |
| Vocabulary | `--entity-types` (3 types), compiled predicates | each KB's **ontology**, loaded after cloning and before the build; no `--entity-types` |
| Extraction window | key order (random neighbours) | **reading order** (fixed on the branch) |
| Dream | six templates with hard-coded generic predicates | **ontology-driven completion**: the type's description, fields, and the predicates its domain admits |
| Completeness (ΔC) | compiled scorer (kinship expected on every Person) | **ontology-aware scorer**. Not comparable to v2 |
| Hardware | arm A on CPU, arm B on GPU | **one machine per corpus**, carrying both arms and all its evals |
| Snapshot checks | none | the driver asserts `kb_ontology` is present in every snapshot and records its hash in each dump record |

## Corpora and gold (status at 21:00)

| Corpus | Clean | Ontology | Gold | Eval questions |
|---|---|---|---|---|
| Legal | 96.9% | `Legal.yaml` (derived) | frozen in v2 (reused) | 20 |
| Astrophysics | 94.2% | `Astrophysics.yaml` (derived) | nuggetize → verify → adjudicate running overnight; **needs Reza's review, then freeze** | 20 |
| NIST | 90.7% | `NIST.yaml` (derived) | same | 20 |
| WarPeace | 99.9% | `WarPeace.yaml`, a thin `narrative-history` extension, **not derived** | same | 20 |
| MobyDick (reserve) | 98.8% | same as WarPeace, not derived | same | 20 |
| D6 | 99.6% | `D6.yaml` (derived) | frozen in v2 (reused) | 40 |

All four new trackers pass the check that caught DreamMem: mean question/answer vocabulary overlap
is 0.55–0.63, against DreamMem's 0.01.

**Before WarPeace can be primary,** its ontology must be derived from its own text, as Manhattan's
was. The genre module gives it 14 kinship predicates, and *War and Peace* may justify some of
them, but that has to be measured. The test set (questions and gold nuggets) is used as a source
and a check (`OntologySelection-plan.md` §2.2a), and types that exist only for the test set are
flagged.

## Corpus completeness (found 21:00–21:15)

The KBs do not match their source folders (`rag-bench/Corpus/<name>/documents/`):

| Corpus | Source files | KB documents | Missing |
|---|---|---|---|
| Legal | 13 | 13 | none |
| Astrophysics | 14 | 11 | NASA Space Act 1958 (a scan), the LIGO detection paper, Space Policy Directive 1 |
| MobyDick | 14 | 12 | Lawrence, *Studies in Classic American Literature* |
| Manhattan | 13 | 12 | the Smyth Report; the Handy–Spaatz order is present only as binary junk |
| **WarPeace** | 13 | **1** | all 12 companion works (Clausewitz, Caulaincourt, Iliad, Stendhal, Hugo…). **17 of 20 questions are about them** |

**Causes, fixed on the branch:**
- HTML had no extractor and was ingested as raw source. It was also missing from
  `supported_extensions`, so folder sync skipped it.
- A scanned PDF named `.html` was read as text.

**Not fixed:** scanned PDFs without a text layer need OCR: the NASA Act and the Handy–Spaatz order.

**Consequences for Eval v3:**
- **WarPeace is out as a primary corpus.** Complete, it would be about 13 books and roughly 30k
  chunks, several days of cycle-0 build. MobyDick is 15k chunks, the same problem.
- **Manhattan comes back as `Manhattan_v3`,** re-ingested tonight with the fixes: 505 chunks, 99.4%
  prose, Smyth Report included. Its existing 109 nuggets are being re-verified against the new KB
  (`work/gold/run_manhattan_v3_gold.sh`, queued after the new-corpus gold job).
  - Handy–Spaatz is still missing (a scan), so its nuggets stay unreachable.
  - Trinity Eyewitness is, in the source file itself, a list of links: the accounts are not in
    it.
- **`Manhattan.yaml` needs a v3 revision after the pilot,** which is using v2.
  - With Smyth added, Material rises from 24.6% to 46.2% of prose chunks and Facility from 31.0%
    to 49.6%.
  - A new Process type (gaseous diffusion, electromagnetic separation, chain reaction) is at
    19.7% (41.6% of Smyth's chunks).
  - Kinship falls to 1.2%.
- **Proposed primary set: Legal, Astrophysics, NIST, Manhattan_v3,** with D6 secondary.
  - Astrophysics has 3 documents missing; their questions count as unreachable, and the count is
    reported.

## Gold status (overnight run, finished 23:39)

Nuggetize → NLI verify → Claude adjudicate: `eval2/work/gold/<KB>_adjudicated.json`. **Not frozen**:
freezing applies Reza's review of the partial and contradicted nuggets (`gold/review.py`, then
`gold/freeze.py`).

| Corpus | Nuggets | Reachable (verified + supported) | Unreachable | Partial (for review) |
|---|---|---|---|---|
| NIST | 75 | **85%** | 4 | 7 |
| Astrophysics | 111 | **77%** | 19 (mostly the 3 missing documents) | 6 |
| Manhattan_v3 | 109 | **72%** | 16 | 15 |
| Manhattan (old KB, for comparison) | 109 | 66% | 26 | 11 |
| MobyDick | 103 | 69% | 19 | 12, plus 1 contradicted |
| WarPeace | 93 | **5%** | 84 | 4 |

Re-ingesting Manhattan made 10 more nuggets reachable. WarPeace's 5% confirms that its KB, one
document of 13, cannot be evaluated.

## Ontology check against the test set (§2.2a), 2026-09-27 night

Each new corpus's gold nuggets were read against its ontology: can each fact be written with the
ontology's types and predicates? The ontology files are **unchanged**, because the pilot is testing
them as they are. Revisions happen after the pilot, with evidence shares measured as for
Manhattan.

| Corpus | Fit | Gaps found |
|---|---|---|
| **Astrophysics** | poor on the document questions | **Documents are the actors again**: Ertel's history, Launius's analysis, the Apollo 204 Review Board report, the Senate review, Roos, Wright and Gaudi. The ontology has no Document/Publication type and no authorship, `covers` or `investigates`-as-a-document relations. Also missing: Legislation/Policy (the Space Act "established" NASA; SPD-1 "directs" it), an astronomical-object type (M87, the Coma cluster, GW150914), a detection-method type (radial velocity, transit photometry), an artefact type (the Golden Record), and `died_in` (the Apollo 1 crew) |
| **NIST** | good | Missing: `Person` (q13 asks for NIST AI 100-2's authors), `Tactic` (ATLAS's Reconnaissance), authorship ("co-authored by NSA, CISA, FBI…"), `modeled_after` (ATLAS and ATT&CK), `profile_of` (600-1 and AI RMF; 800-218A and SSDF). Publication fields for date, draft/version and TLP marking |
| Manhattan | checked while deriving v2 | Needs a v3 for `Manhattan_v3`'s added technical content: a Process type, with evidence re-measured (see above) |
| **Legal** | good | Missing: `upholds` (McConnell upheld the soft-money limits; Korematsu upheld Exclusion Order No. 34), for statutes and orders, where `affirms` covers only lower-court cases. An executive-order type. A decision-date field on Case (q10 asks which case came first) |

**Completed 2026-09-28** (`ade6245e`, Reza's decision 1). Each gap above was measured on the
corpus's prose chunks and added with its evidence share, or rejected with its numbers:

| Ontology | Types | Relations | Main additions |
|---|---|---|---|
| Legal v2 | 14 | 25 | Legislature (20.8%), Organization, Person, ExecutiveOrder, HistoricalEvent; upholds, enacted_by, amends, joins, nullifies. `Party` is now a role |
| Astrophysics v2 | 22 | 22 | Document (18.9%), AstronomicalObject (14.5%), Phenomenon, Method, Artefact, Legislation; observed, evidence_for, covers, authored |
| NIST v2 | 13 | 18 | Tactic (12.9%), Person; derived_from, profile_of, achieves, authored; Publication date, version and marking fields |
| Manhattan v3 | 13 | 24 | Process (19.9%); developed, uses_process. Evidence re-measured on Manhattan_v3 |

Correction: Legal's `Case` already had `dateDecided`. The earlier check missed it.

The pattern: **a corpus of documents about documents needs `Document` and authorship at its
centre.** Of the corpora that have eval questions, Manhattan, Astrophysics and NIST all do. That
argues for making them part of the backbone, as schema.org's CreativeWork with author, about and
dateCreated (`OntologySelection-plan.md` §2.1). Each corpus should not have to rediscover them.

## Hardware and schedule

metro-linux is CPU-only, and metro-win is the GPU. One machine per corpus:

| Machine | Corpora | c00 build | Trajectory (2 arms × 12 cycles + 14 evals) |
|---|---|---|---|
| metro-win (GPU) | Legal, then Astrophysics, NIST | Legal starts automatically after the pilot **if it passes** (`results/eval3/after_pilot.sh`); ~11 h | ~5 h each |
| Mac (Metal) | D6, then WarPeace | D6 starts with Legal on the same condition; ~9 h (Eval v2) | ~6 h |
| metro-linux | none for now | **down at 20:53**: HTTP 502/503 on every call. Its Legal build was stopped after 4 chunks and 0 entities | — |

That is about two days end to end, so **it does not fit the Oct 1 revision**. Reza decides the
paper framing (`OntologySelection-plan.md` §7.4).

A reduced set that could report sooner: Legal and D6 on cycles {0, 2, 8}, both arms, labelled as
reduced.

## Gates before any trajectory starts

1. **The ontology pilot passes** (`results/ontology_pilot/report.md`). If it fails, v3 waits for a
   revised design.
2. **Phase 1 is complete** for everything Eval v3 exercises:
   - dream and scorer (done 2026-09-27);
   - `rag eval --mode iterative`: checked 2026-09-27, and it does not use the kinship
     patterns. Iterative mode calls `understand_query_rule` only to detect temporal questions
     (for sequence retrieval). The retriever calls the full `understand_query` only in
     `prepend`/`replace` graph modes, and Eval v2 ran `graph_mode=inject`. No change needed.
3. **Reza reviews:**
   - the new gold (review sheet);
   - WarPeace's derived ontology;
   - this plan.
4. **The driver** loads and asserts ontologies, records their hashes, and pins one machine per
   corpus. `build.py` passes no `--entity-types`.
5. **A 3-question smoke run** per corpus, through the whole chain (build → dream 1 cycle → eval →
   score), from a fresh clone.

## Open for Reza

- The corpus set: WarPeace or MobyDick as the fourth, or both.
- Whether Manhattan is re-ingested (from `Corpus_Final_Review`: the Smyth Report and the
  Handy–Spaatz order are missing, and the Einstein page needs its text, not its CSS) and brought
  back.
- The Oct 1 paper framing.
