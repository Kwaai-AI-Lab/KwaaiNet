# Per-KB ontology: session assessment

2026-08-25/26. Branch `feat/per-kb-ontology`, 7 commits, nothing pushed.

---

## 1. What was asked, and what happened

The session began with "let's explore specifying the knowledge schema for each
KB" and ended with an ontology driving extraction on a real corpus. Between
those, three of my own outputs were wrong and had to be withdrawn. That pattern
is the most useful thing here, so it is recorded first rather than buried.

| # | What I produced | How it was caught | What it cost |
|---|---|---|---|
| 1 | A `narrative-history` genre module giving D6 a `Vessel` type | Reza asked whether vessels are prominent in D6 | Measured: 31% of MobyDick chunks, 1% of D6 |
| 2 | A D6 ontology making `dispossessed` central | Measuring before re-running | 1.5% of corpus; the book ends in 1956, demolitions began in the 1960s |
| 3 | "23 of 25 eval questions answerable" | Re-reading the eval file | The set has **40** questions; I had truncated with `head -25` |

All three share a cause: **asserting a property of the corpus without measuring
it.** The fix that stuck was not better judgement but a method change — every
entity type in `D6.yaml` now carries the measured share of the 1152 chunks that
contains its vocabulary, and rejected types are recorded with their numbers so a
later revision cannot re-propose them.

## 2. Code

Seven commits. The crate change is separable from the data and the docs.

**`ontology.rs`** — one config type read by every consumer, with each compiled
table kept as fallback. 23 sites of compiled-in domain knowledge generalised.
Four were not merely inflexible but wrong:

- **narrator gender hardcoded `Some("Male")`** ("the narrator in this corpus is
  Yousuf Rassool, a man") — mis-gendered any corpus with a woman narrator and
  corrupted its coreference resolution.
- **the stop-list contained "Hatless"**, a named District Six character
  introduced in a chapter called "Characters of District Six" and listed in the
  book's own index, so he could never enter any graph.
- **`scorer.rs` expected `schema:Person` to carry kinship**, making `graph score`
  a memoir-conformance metric. A climate KB re-extracted under a climate
  ontology would have scored *worse for being correct*.
- **`query_understand.rs` had 34 query patterns, all kinship**, so "the founders
  of the TLSA" matched nothing.

Three separate hardcoded rules turned out to be one rule: the familial-Person
axiom, the `located_in`/`works_at` CreativeWork guard, and `FAMILIAL_INVERSE`'s
fourteen hand-written pairs are all **domain/range plus an `inverse:` field**.

**`OntologyIndex`** — compiled once per run, shared via `Arc`. Measured over
2000 sentences:

| path | naive | hoisted | indexed |
|---|---|---|---|
| trigger lookup | 184 ms | 47 ms | **38 ms** (5×) |
| classification | 46 ms | — | **3.3 ms** (14×) |

Index build is 306 µs, once per run.

**A latent bug found while checking the A/B for confounds.** `entity_cap` keyed
on the `entity_types` argument, where empty means "offer all 17 globals". Empty
has `len() == 0`, so the *widest* vocabulary received the cap 25 meant for
focused 2–3 type runs, while a caller listing the same 17 types explicitly got
20. The control arm passes none and the ontology arm passes 17, so the ontology
arm was handicapped before a single chunk was extracted. Fixed and tested.

## 3. Ontologies

Sixteen corpus ontologies plus three shared modules. Only **D6** was built from
a full reading; the other seven marked `unverified` carry the genre-module fault
that produced `Vessel` and have not been re-derived.

D6 v6: 17 entity types, 27 predicates, 187 trigger phrases, 5 axioms, 38 of 40
eval questions covered.

*(As committed, `ontologies/D6.yaml` is **v9**: 18 entity types, 33 predicates,
240 trigger/marker phrases, 71 aliases, 4 axioms. v6-v9 are revisions of the
same third derivation; the counts below describe v6 as it stood for the smoke
runs. The file's comment header says "third authored derivation", which is a
different number from `ontology.version` and used to read as "Version 3".)*

Two findings worth keeping:

- **An independent NotebookLM pass beat my recall.** It surfaced `Doctrine`
  (6.6% of chunks — more than `Community` or `SportsClub`, both of which I had
  included) and `ReligiousObservance` (5.5%). Its entity lists were accurate:
  of ~50 checked, only five returned zero and all five were formatting variants.
  It produced a *taxonomy* — no relations, no domain/range, no axioms — so the
  two are complementary rather than competing.
- **Corpus frequency is not reader demand.** Q31 asks about a mosque; mosque is
  1.4% of chunks, below the bar every other type cleared. As one `Venue` type
  with church and hall it is 11.9%. Conversely three questions ask about the
  forced removals, which are 1.3% because the memoir ends before them — that one
  is an honest limit of the corpus and is recorded, not engineered around.

## 4. The finding that outranks the ontology work

`corpus_hygiene.py` measured what fraction of each KB is usable prose:

| KB | clean | dominant defect |
|---|---|---|
| Meetings | **2.0%** | 1875/1914 chunks are WebVTT cue numbers and timestamps |
| PythonDocs | **16.1%** | raw Sphinx HTML |
| Poems | **22.3%** | 22202 chunks of `<div class="verse">` |
| CountryHistory | **25.0%** | raw HTML |
| RFCs | **30.6%** | character-spaced PDF (`L I S T  t y p i c a l l y`) |

**Nine of sixteen KBs are data-bound, not vocabulary-bound.** An ingestion
hygiene pass is higher-value than this entire plan for those nine, and
`doc_schema.rs` already has the machinery — only D6 has ever used it.

This also corrected the bakeoff design: Climate was dropped as a subject (31% of
its chunks unusable) in favour of Legal, Astrophysics and NIST.

## 4b. The A/B, run 1 — it found bugs in the code, not virtues in the ontology

120 chunks per arm, same chunks, same model, local Ollama, entity cap confound
already fixed. Archived under `tests/kwaai-knowledge/results/run1/`.

| metric | control | ontology | |
|---|---|---|---|
| escape-hatch rate | 17.3% | **5.2%** | −12.2pp |
| entities | 332 | 252 | −80 |
| relations | 277 | 194 | −83 |
| density (rel/ent) | 0.834 | 0.770 | held |
| kinship edges | 98 | **36** | −62 |

Four of five predictions passed. `Address` populated and separated from `Place`
(Cannon Street, Kloof Street, Caledon Street — the 24.2% signal that previously
had nowhere to go). `Doctrine` populated. Escape-hatch fell by more than two
thirds. Density held.

**Prediction 5 failed, and the cause was in the crate.** `extends: genealogy`
was parsed and ignored, so the extraction prompt offered a kinship-dense memoir
zero kinship predicates — the ontology arm was *worse than no ontology* at the
one thing D6 is densest in. Fixed: built-in modules resolve at load, with
inverses, symmetry, Person domain/range and trigger phrases.

**A second defect the numbers exposed.** The arm produced 88 distinct relation
types from 27 declared — `was_defenestrated_at`, `gazed_at`, `staying behind`.
The vocabulary reached the prompt and nothing checked the output against it, so
the ontology only *suggested*. `coerce_relation` now maps unknowns to the
corpus's fallback, or drops them where a corpus admits none.

**Still open after both fixes: type precision.** `Language` caught "Integral
Calculus, Latin, Mathematics, Trigonometry" — school subjects. `Legislation`
caught "Senior Certificate", "Health Department". `Venue` caught "Nash". Those
are authoring faults in my markers, not code faults, and they need a separate
pass over `D6.yaml`.

**The methodological point.** A test designed to flatter the ontology would have
reported "escape-hatch down 12 points, four of five predictions met" and stopped.
Recording a kinship prediction that the ontology could fail is what surfaced the
unimplemented `extends`. The value came from the prediction that failed.

## 4d. Runs 2 and 3 — and why none of the three answers the question

Three ontology-arm builds against one control, all on the same 120 chunks.

| arm | ents | rels | density | uninformative |
|---|---|---|---|---|
| **control** (no ontology) | 332 | 277 | 0.834 | 34.3% |
| run 1 — no `extends`, vocabulary unenforced | 252 | 194 | 0.770 | — |
| run 2 — `extends` + enforcement | 278 | 208 | 0.748 | 41.8% |
| run 3 — v7, + aliases and two missing predicates | 226 | 166 | 0.735 | **27.7%** |

Run 3 finally passes prediction 3 on the corrected metric (34.3% → 27.7%
uninformative) and holds density, `Address`, and `Doctrine`. Undeclared
predicates fell 17.0% → 2.4%, so vocabulary enforcement demonstrably works.

**But the runs also disagree with each other in a way that invalidates the
comparison.** Run 3 added only aliases and two predicates — changes that can
only *increase* edge retention — and produced 42 *fewer* relations than run 2.
Across three runs on identical input:

```
relations  166–208   mean 189, sd 21   range = 22% of mean
entities   226–278   mean 252, sd 26   range = 21% of mean

control-vs-ontology difference in relations:  32%
within-arm run-to-run range:                  22%
```

**Signal is roughly 1.5× noise at n = 1 per arm.** The relation-count gap is
marginally outside the band; density (−12%) and uninformative rate (−6.6pp) sit
well inside it. Each ontology run also changed *code* as well as randomness, so
the three are not even replicates of one another.

**Honest conclusion: this A/B cannot say whether the ontology improves D6's
graph.** What survives is only what is deterministic:

- the code defects it exposed, all fixed and unit-tested;
- the entity types only the ontology can produce — `Address` (12), `Doctrine`
  (4), `Venue`, `PoliticalOrganization` (24), `EducationalInstitution` — a type
  is in the vocabulary or it is not, and no sampling changes that;
- undeclared predicates 17.0% → 2.4%, which is enforcement, not chance.

The per-edge breakdown behind that last line is in `results/predicate_collapse.json`
(chart via `build_collapse_chart.py`), and it sharpens the reading: the control's
87 undeclared edges are almost all *nuance* — undeclared predicates that carry
real meaning — against 0 junk, while run 1 traded 83 nuance for 12 junk. So
enforcement is not removing noise; it is removing meaning and noise together,
and the residue mechanism in [`PredicateVectors-exploration.md`](./PredicateVectors-exploration.md)
§4 exists to keep the first of those recoverable.

**What would make it decidable.** Extraction runs at `temperature: 0.1`
(`graph.rs:5672`) with 2 concurrent workers, so request interleaving and
sampling both vary the output. Either pin `temperature: 0.0` with
`--workers 1` for measurement runs, or run n ≥ 3 per arm and compare means.
The first is cheaper and probably correct anyway — extraction is not a creative
task. Neither should be changed for production without its own measurement.

## 4c. A near-miss worth recording

Between the two runs, `graph clear` silently deleted the ontology — it removed
the whole database file, and the metadata table lives in it. The ontology arm
ran eleven chunks as a **second control**, and the comparison would have
reported two near-identical arms and a clean null. It was caught only by
checking for the `extraction driven by KB ontology` log line rather than
trusting the run.

Fixed as a bug: the graph is derived data, the ontology is configuration, and
clearing one must not discard the other. `--all` is the full reset.

## 5. Open, in priority order

0. **Make the A/B decidable** (§4d) — pin temperature and workers, or n ≥ 3
   per arm. Every other measurement here inherits this limit.
0b. **D6 marker precision** — `Language`, `Legislation` and `Venue` are catching
   the wrong things (§4b). Cheapest fix with a measurable effect, and it is
   ontology authoring rather than code.
1. **Ingestion hygiene for the nine data-bound KBs.** Highest value in the
   backlog and independent of everything else here.
2. **Re-derive the seven `unverified` ontologies** from their own corpora.
3. **The remaining bakeoff arms** — B and D across Legal, Astrophysics, NIST.
4. `document.rs:COMMON_WORDS` left compiled deliberately as language-generic.

## 6. Caveats on anything measured here

The 40-question eval set is token-overlap scored, saturated near 90%, and
relation count does not predict recall (Pearson +0.081 across 15 KBs). A large
move in the graph-side metrics is real signal; **a null on recall is not
evidence the ontology did not help** — it is the known limit of the instrument.
