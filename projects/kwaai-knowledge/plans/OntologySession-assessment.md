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

## 5. Open, in priority order

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
