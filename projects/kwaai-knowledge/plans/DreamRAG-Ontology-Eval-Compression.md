# DreamRAG: ontology, dynamic evaluation, compression

Reflection prompted by Reza, 2026-08-23, after the overnight dream sweep and the
first cross-KB retrieval baseline. Not a plan of record yet — a framing to argue
with.

## The correction that starts this

I concluded from the baseline that relation count does not predict retrieval
quality, and inferred that relations are not where the value is. The first half
is measured and stands: Pearson +0.081 across 15 KBs; WarPeace with 742
relations retrieves below median; Legal with 2 relations tops the table.

The inference was wrong, and the codebase says why. The complete relation
vocabulary is:

```
related_to, parent_of, child_of, spouse_of, sibling_of, half_sibling_of,
grandparent_of, grandchild_of, uncle_of, aunt_of, niece_of, nephew_of,
cousin_of, foster_parent_of
```

Thirteen kinship predicates and one escape hatch. That is a memoir ontology. We
applied it to climate science, astrophysics, RFCs, Python documentation, legal
statutes and poetry, and then measured that it did not help. Of course it did
not: for a permafrost paper there is nothing in that vocabulary to assert, so
extraction fell back to `associated_with` and produced edges carrying no
information.

So the experiment measured *generic and mismatched* relations, not relations.
The null result is evidence for Reza's reading — the ontology is wrong — not for
mine.

Entity types are broader but pull the same way: `Person, Organization, Place,
Event, Product, CreativeWork, SoftwareApplication, DefinedTerm, HowTo, Role`.
Nothing for a Quantity, Method, Dataset, Finding, Statute, Clause, Obligation,
Metaphor or Speaker. A physics paper has to become people and places to be
represented at all.

The precedent for the fix already exists, for exactly one KB: `d6_doc_schema.yaml`,
`d6_entity_schema.yaml`, `d6_entity_type_schemas.yaml`, and a `--doc-schema`
flag. D6 is both the best-schematised and the best-understood KB. That is not a
coincidence to explain away; it is the result to generalise.

---

## 1. Ontology

### The claim

A knowledge graph is only worth its cost when its predicates carry the domain's
actual structure of meaning. One vocabulary cannot span memoir, physics, poetry
and law, because those domains do not share a structure of meaning. Kinship is
the *right* ontology for D6 and an absurd one for RFC 2119.

### What per-domain vocabularies look like

| domain | entity types | relation types |
|---|---|---|
| memoir / biography | Person, Place, Organization, Event | kinship, `lived_in`, `worked_at`, `witnessed`, `recalls` |
| scientific paper | Phenomenon, Quantity, Method, Dataset, Instrument, Finding | `causes`, `correlates_with`, `measured_by`, `supports`, `contradicts`, `sample_of` |
| legal | Statute, Clause, Party, Obligation, Jurisdiction | `obliges`, `permits`, `prohibits`, `defines`, `amends`, `supersedes`, `exception_to` |
| standards (RFC / NIST) | Requirement, Protocol, Parameter, Role | `MUST`/`SHOULD`/`MAY` (RFC 2119 is a ready-made normative vocabulary), `depends_on`, `deprecates`, `conforms_to` |
| software docs | Module, Function, Parameter, Type, Error | `returns`, `raises`, `parameter_of`, `deprecated_by`, `example_of` |
| poetry | Image, Motif, Speaker, Form, Addressee | `metaphor_for`, `alludes_to`, `voice_of`, `addressed_to`, `recurs_in` |

Two things stand out. RFCs and NIST documents carry an explicit normative
vocabulary in the source text — MUST/SHOULD/MAY — so the ontology is *given*,
not inferred. And several domains have established vocabularies worth borrowing
rather than inventing: CiTO for scholarly citation, LegalRuleML/LKIF for law,
SKOS for concept hierarchies, schema.org's ScholarlyArticle branch.

### Where does a KB's ontology come from?

Three sources, probably all three in practice:

1. **Declared** — hand-authored per KB, as D6 already is. Highest quality,
   highest effort, right for KBs that matter.
2. **Selected** — a library of domain ontologies, chosen at `rag init` by
   corpus classification. Cheap, covers the common cases.
3. **Induced** — sample the corpus, ask an LLM what kinds of relationship hold
   between entities in *this* text, cluster the answers, propose a vocabulary
   for human ratification. This is ontology learning, and it is the interesting
   one: it makes a new domain tractable without hand-authoring.

Induction has an obvious failure mode — an LLM will happily invent a plausible
vocabulary that the corpus does not support. Mitigation: require every proposed
predicate to be *grounded*, i.e. supported by N distinct chunks with extractable
instances, before it enters the vocabulary. A predicate that cannot be
instantiated is discarded.

### Architecture

The ontology should be a first-class, versioned, per-KB artifact — not constants
in `graph.rs`. It is read by extraction (what to look for), dream (what to
complete), retrieval (how to traverse), and evaluation (what questions are
answerable). Today it is a `const` array, which is why it is uniform.

Concretely: `~/.kwaainet/rag/<KB>/ontology.yaml`, declaring entity types with
their expected fields and relation types with their domain/range constraints.
Domain/range matters: `measured_by(Phenomenon, Instrument)` lets extraction
reject `measured_by(Person, Poem)` without an LLM call, and gives the sanitiser
something principled to check instead of the current gender heuristic that
deleted valid D6 relations because "Gadija" was recorded as Male.

---

## 2. Dynamic evaluation

### The problem with the current instrument

Twenty fixed questions per KB, scored by token overlap. Three consequences, all
of which bit us this week:

- **It saturates.** D6 sat at ~90% before and after adding 114 relations. A
  saturated metric cannot tell improvement from noise, which is exactly why the
  overnight run was inconclusive.
- **It cannot see what it does not ask.** New relations went unmeasured because
  no question probed them.
- **It has no notion of difficulty.** A KB that answers twenty easy questions
  scores the same as one that answers twenty hard ones.

Reza's framing is the right one: we are giving the final-year exam in the first
semester, and then again every semester, unchanged.

### A curriculum, not an exam

Tier questions by cognitive demand, and let the KB advance:

1. **Recall** — a single fact stated in one chunk. "Who wrote this?"
2. **Retrieval under paraphrase** — the fact is present but not in the
   question's words.
3. **Multi-hop** — requires composing two or more asserted facts. *This is the
   tier where a graph should start to pay, and where the current set has almost
   nothing.*
4. **Synthesis** — aggregate across many chunks. "How did the author's view of
   X change?"
5. **Inference** — supported by the corpus but stated nowhere in it.

The KB's score becomes a *level* plus a pass rate within it, not a single
percentage. Progress is visible as advancement, and the exam stays hard enough
to discriminate.

### Generating the exam

Generate questions from the **corpus**, never from the graph. This matters more
than it sounds: if you generate questions from the graph and then test the
graph, you measure self-consistency and will happily reward a confidently wrong
graph. The corpus is the ground truth; the graph is the thing under test.

Each generated question needs a grounding check — the expected answer must be
locatable in source text — or the exam inherits the generator's hallucinations.

Tier 3 questions can be *shaped* by the ontology without being derived from the
graph: if the ontology declares `causes(Phenomenon, Phenomenon)`, generate
multi-hop causal questions from the corpus and see whether the pipeline can
answer them. That ties evaluation to ontology, which is the point.

### Regression and forgetting

Dream mutates the graph, and this week it destroyed 14 relations in a cycle that
did no work. A dynamic eval should keep a bank of previously-passed questions and
re-ask a sample — spaced repetition, in the student analogy — so regression is
caught rather than discovered months later. This is also the instrument
compression needs.

### Calibration

Longer term, item-response theory gives the formal version of "an exam that grows
with the student": estimate question difficulty and KB ability jointly, so scores
are comparable across KBs whose question sets differ in difficulty. Our current
sets differ (58–80 expected tokens) and we have been comparing them anyway.

---

## 3. Compression

This is the least-developed of the three and, I think, the most valuable.

### The system does not currently forget anything

The stated model is short-term memory (vector store) and long-term memory
(graph). In practice both grow monotonically and nothing is ever discarded, so
they are two parallel accumulating stores rather than a memory hierarchy. Human
reading does not work this way: we retain gist, schema and a few vivid details,
and reconstruct the rest. Forgetting is not a failure of memory, it is what
makes memory generalise.

### The formulation

Treat it as rate–distortion. Define a retention budget *R* — the fraction of
source chunks kept — and ask:

> **maximise comprehension score, subject to retaining ≤ R of the corpus**

Sweep *R* and plot the curve per KB. That single curve is more informative than
any static score, and it directly operationalises the DreamRAG thesis.

### Why this is the metric we have been missing

Relation count is a bad proxy for graph quality — measured this week at r=+0.081
against the outcome. Compressibility is a good one, and for a principled reason:

> **If extraction genuinely captured the facts, you can discard the text they
> came from and still answer questions. If it did not, you cannot.**

So compressibility *is* a measure of extraction quality, in a way that counting
edges never can be. It also gives a falsifiable prediction to test the whole
framing:

- WarPeace, with 742 `associated_with` edges, should compress **badly** — the
  edges encode nothing, so discarding text loses the answers.
- D6, with typed kinship relations, should tolerate discarding genealogical
  recitation and still answer family questions.

If that prediction fails, this framing is wrong and we should know early.

### What to discard

Candidate signals, cheapest first:

- **Redundancy** — near-duplicate chunk embeddings; keep one exemplar.
- **Coverage** — every claim in the chunk is asserted in the graph with
  independent evidence elsewhere.
- **Derivability** — an LLM given the graph plus retained chunks can reconstruct
  the chunk's claims. Expensive but the most faithful test.
- **Salience** — never retrieved for any question across the eval bank. Weakest
  signal, and dangerous: it optimises for the exam rather than comprehension.

`summarize` (HiRAG window and section summaries) is already a compression
primitive in the codebase and is the obvious first tier: replace *n* chunks with
their summary, keep the summary, measure the loss.

### Guardrails

Discarding source text is irreversible in a way nothing else here is, and this
week's dream cycles already destroyed data while reporting success. Compression
must be tiered rather than destructive — cold-store what is dropped, keep the
manifest, make re-ingestion a single command — until the rate–distortion curve
for a KB is actually known.

---

## How the three interlock

They are one argument, not three:

- **Ontology** determines what can be extracted, which determines what can be
  discarded, which determines what questions are answerable.
- **Evaluation** is the instrument that makes the other two decidable. Without
  it, a new ontology is an aesthetic preference and compression is a gamble.
- **Compression** is the forcing function that proves the ontology works. If you
  cannot forget anything, the graph is not carrying anything — which is exactly
  what this week's null result was telling us.

That implies a dependency order, and it is not the order we would naturally pick:

**Evaluation first.** We currently cannot measure whether a change helps — that
is the root cause of an inconclusive overnight run, and it will make every
ontology experiment equally inconclusive. Build the curriculum and the generator
before touching extraction.

**Ontology second**, on one or two contrasting KBs rather than all fifteen. Legal
and Climate are the natural pair: both currently near-zero relations, one
retrieving well (87.5%) and one badly (48.4%), with completely different
structures of meaning.

**Compression third**, once there is an instrument to bound the loss and an
ontology worth compressing against.

---

## Concrete first steps

1. **Make the ontology a per-KB artifact.** Move the vocabulary out of
   `graph.rs` constants into `ontology.yaml`, with D6's existing schemas as the
   first instance and the current kinship set as the `memoir` default. Mechanical,
   unblocks everything else.
2. **Build the question generator** — corpus-grounded, tiered, with a grounding
   check. Validate it by regenerating D6's set and confirming it reproduces
   comparable scores to the hand-written one.
3. **Author two contrasting ontologies** (legal, scientific) and re-extract those
   two KBs. Measure with the new instrument, against the baseline recorded on
   2026-08-22.
4. **Run the compression probe** on D6 and WarPeace at R = 0.75, 0.5, 0.25 to
   test the prediction above. This is cheap and would tell us early whether the
   framing holds.

## Open questions worth arguing about

- **Does a KB have one ontology or several?** A memoir contains legal documents
  and letters. Per-document-type ontology within a KB is more faithful and much
  more complex.
- **Is `associated_with` ever legitimate?** It may be honest for genuinely
  unstructured association, in which case the bug is that it is the *default*
  rather than that it exists.
- **What is comprehension, operationally?** Token-overlap recall is a weak
  proxy. An LLM judge is better and less reproducible. This choice determines
  everything downstream, including the compression curve.
- **Does compression help retrieval, or only cost?** Removing redundant chunks
  may *improve* precision by reducing near-duplicate competition in the vector
  search. Worth measuring, not assuming.
