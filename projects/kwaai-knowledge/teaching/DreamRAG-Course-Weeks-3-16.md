% Kwaai Intern Program — DreamRAG Track
% Graduate Cohort · Weeks 3–16
% 16 weeks · 4 hrs/week · standups Mon / Tue / Thu

## The course in one slide

**Weeks 1–2 (done):** AI and RAG foundations, baseline experiment.

**Weeks 3–16:** research. You are testing whether our diagnosis of a measured
failure is correct, and writing the paper either way.

| | |
|---|---|
| **W3–4** | Orientation · **replication gate** |
| **W5–6** | Build the instruments · first real signal |
| **W7–8** | Compression probe · induction · **mid-programme cut** |
| **W9–11** | Integration · ablations · robustness |
| **W12** | **Results frozen** |
| **W13–14** | Writing · **arXiv preprint** |
| **W15–16** | Reproducibility package · **submission** |

*C = core (everyone) · A = additional (if you're ahead)*

---

# Week 3 — Orientation to the real system

## Week 3 · Mon — Where the project actually is

**Class (C): the null result, and how we got it**

- The cross-KB baseline: 15 KBs — memoir, climate science, astrophysics, law,
  RFCs, NIST, Python docs, poetry, Moby-Dick, War and Peace.
- **Relation count does not predict retrieval quality.** Pearson **+0.081**, n=15.
  - WarPeace: **742 relations → 63.7%** recall (below median)
  - Legal: **2 relations → 87.5%** (best in set)
  - OSMDocs: **0 relations, 240 entities** → beats Astrophysics (23 rels, 6437 ents)
- Adding 114 relations to D6 moved recall **0pp**.

**Class (C): our diagnosis**

The entire relation vocabulary is:

`related_to, parent_of, child_of, spouse_of, sibling_of, half_sibling_of,`
`grandparent_of, grandchild_of, uncle_of, aunt_of, niece_of, nephew_of,`
`cousin_of, foster_parent_of`

Thirteen kinship predicates. We asked a permafrost paper who its aunt was.

## Week 3 · Mon — Homework

**Homework (C): get the system running and produce one number**

```bash
kwaainet rag list                      # see the 15 KBs
kwaainet rag graph stats --kb Legal
kwaainet rag eval --kb Legal \
  --questions tests/kwaai-knowledge/Legal/eval_questions.json \
  --inference-url http://localhost:11434 --model llama3.1:8b \
  --output my_first_eval.md
```

Post your recall number in the standup channel. Recorded baseline: **87.5%**.

**Homework (A):** pick one paper from the reading order and prepare a 5-minute
summary for Tuesday. Start with *Decision-centric rate–distortion* or
*AutoSchemaKG*.

## Week 3 · Tue — The testbed, the code, the teams

**Class (C): tour**

- Corpora → chunks → embeddings → graph → retrieval. Where each lives on disk.
- `kwaai-rag` crate: `graph.rs`, `dream.rs`, `relation_extract.rs`, `scorer.rs`.
- The eval harness and what "token-overlap recall" actually measures — and its
  weaknesses. You will replace it.

**Class (C): workstreams assigned** — teams of 2–3, each owns a paper section.

| | |
|---|---|
| **WS-A Ontology** | per-KB vocabularies, domain/range, induction |
| **WS-B Evaluation** | replication, curriculum questions, abstention |
| **WS-C Compression** | consolidation gate, rate–distortion sweeps |
| **WS-D Streaming** | replay harness, observe-only agent |

## Week 3 · Tue — Homework

**Homework (C):** reproduce **one row** of the baseline table for a KB that is
not Legal. Open a PR adding your number to `results/replication_w3.md` with the
exact command you ran.

**Homework (A):** your number will not match exactly. Explain the gap — model
version, judge nondeterminism, retrieval mode, or a real difference? Write two
sentences distinguishing *noise* from *discrepancy*.

> If you cannot state why two runs differ, you cannot claim an improvement later.

## Week 3 · Thu — Writing slot #1

**Class (C): the paper, from day one**

- Working title: *One ontology does not fit all: measuring and repairing domain
  mismatch in graph-augmented retrieval.*
- Nine sections, mapped to your four workstreams. Section 4 — the null result —
  is already collected. You are writing Sections 5–8.
- Overleaf set up today. Every Thursday advances the paper.

**Class (C): authorship policy — agreed now, not at submission**

Complete a workstream and contribute text → co-author. Ordering by
contribution, decided by the advisor, disclosed in advance.

## Week 3 · Thu — Homework

**Homework (C):** draft 3–5 sentences of Section 1 motivation *in your own
words*. Why should anyone care that a graph adds nothing across 15 domains?

**Homework (A):** add three related-work citations from the literature scan to
the shared `.bib`, each with one sentence on how it differs from what we are
doing.

> A related-work entry you cannot distinguish from your own work is one you have
> not understood yet.

---

# Week 4 — The replication gate

## Week 4 · Mon — A result you did not reproduce is a rumour

**Class (C): how last week's result was actually produced**

Honest account. Every item is a lesson you will encode in your own work:

- A flag defaulting to the literal string `"default"` made **every** LLM call
  fail. Unnoticed for hours, because failures were indistinguishable from
  "nothing to do".
- Cycles were optimised for **throughput** — completions, relations added —
  while the metric anyone cared about did not move.
- A destructive step ran on cycles that accomplished nothing, costing **14 of
  D6's 237 relations** before a guard was added.
- The first conclusion — "relations do not help" — was **too broad**. Every
  relation measured was generic or mismatched.

## Week 4 · Mon — The gate

**This week decides the shape of the paper.**

WS-B independently reproduces the cross-KB baseline from the raw corpora.

- **Pass** (within ±3pp on ≥12 of 15 KBs) → Section 4 stands, everyone proceeds.
- **Fail** → the paper becomes *why the baseline was unreproducible*. Still
  honest, still publishable, and a better lesson than building on sand.

**Homework (C):** WS-B runs the full sweep. WS-A/C/D instrument their own area
and gather the "before" numbers they will need in week 9.

**Homework (A):** **pre-register** your workstream's hypothesis — what you
expect to change, in which direction, by roughly how much. Commit it. You do not
get to revise it after seeing results.

## Week 4 · Tue — Methods before results

**Class (C): write the method before you have the outcome**

- A methods section you cannot write is a method you have not specified.
- Reviewers reject on under-specification more often than on weak results.
- Worked example: turn "we compress the KB" into something a stranger could
  re-run — what is discarded, by what rule, measured how, against what baseline.

**Class (C): what makes n=15 hard**

Small samples. Report intervals, not point estimates. Our own headline (+0.081)
is safe *because* it is a null; the entity correlation (+0.397) is **not**
established and we do not claim it. Learn the difference.

## Week 4 · Tue — Homework

**Homework (C):** draft your workstream's Methods subsection in Overleaf. Two
paragraphs minimum. No results — method only.

**Homework (A):** swap with another team. Try to re-run their method from their
text alone. Log every question you had to ask them; each one is an
under-specification they must fix.

> The peer who cannot follow your method is doing you a favour.

## Week 4 · Thu — Gate review + writing slot #2

**Class (C): WS-B presents the replication**

- Side-by-side: recorded baseline vs reproduced, all 15 KBs.
- Where it differed, and whether that is noise or discrepancy.
- **Gate decision taken in the session**, on the evidence, in front of everyone.

**Class (C): what "done" looks like**

Results frozen W12 · arXiv preprint W14 · reproducibility package W15 ·
submission W16. Working backwards, your experiments must be finished eight weeks
from today.

## Week 4 · Thu — Homework

**Homework (C):** commit your Methods draft. Create your section's results-table
skeleton — column headings and units, with no numbers in it yet.

**Homework (A):** write the caption for your main figure before the figure
exists. If the caption is vague, the experiment is vague.

## Looking ahead — weeks 5–9

| | |
|---|---|
| **W5** | Ontology v1 (pure refactor, behaviour unchanged) · question generator v1 |
| **W6** | Hand-authored legal + scientific ontologies · **first real signal** |
| **W7** | Compression probe · multi-hop questions · mid-programme cut review |
| **W8** | Grounded induction · first stream replay · Methods complete |
| **W9** | Integration — all workstreams, one KB set, one eval, first results table |

**The falsifiable bet (W7):** WarPeace's 742 `associated_with` edges should
compress **badly**; D6's typed kinship should compress **well**. If that fails,
our framing is wrong and we want to know in week 7, not week 15.

---

# Week 5 — Building the instruments

## Week 5 · Mon — What an ontology actually is

**Class (C): the formal object**

Not "a list of relation types". An ontology is a **triplet constraint space**:

1. permissible **entity types**
2. permissible **relation types**
3. a **domain/range function** — which types may sit at each end of each relation

That third part is the one we lack, and it is the one that does the work.

```yaml
relations:
  measured_by:
    domain: Phenomenon
    range:  Instrument
```

`measured_by(Person, Poem)` is now rejected **structurally — with no LLM call.**

## Week 5 · Mon — Why this is not academic

Our current sanitiser removes relations it thinks are wrong using a **gender
heuristic**. It deleted valid D6 relations because *Gadija* — a woman's name —
was recorded as Male:

```
removing suspect spouse_of: Gadija Dija Noor (Male)
  ↔ Advocate Christopher (Male) — same gender (likely hallucination)
```

A guess about names, deleting real data. Domain/range constraints are the
principled replacement: a rule the ontology *declares*, not one the code infers.

**Homework (C), all workstreams:** read `graph.rs` where `RELATION_TYPES` is
defined and the sanitiser that consumes it. Write three sentences on what it
would take to make that check declarative.

## Week 5 · Mon — Workstream tasks

| | this week's deliverable |
|---|---|
| **WS-A** | `ontology.yaml` **loading**, with today's kinship set as the `memoir` default. **Behaviour must not change.** |
| **WS-B** | Question generator v1 — tiers 1–2 (recall, paraphrase) with a grounding check |
| **WS-C** | Consolidation gate as a **measurement only**: what fraction of chunks *would be* evictable. Nothing is deleted. |
| **WS-D** | Replay harness skeleton: read a corpus, emit it as a timed stream, no consumer yet |

**Homework (A):** for your workstream, write down the one thing most likely to
make your deliverable wrong, and how you would detect it.

## Week 5 · Tue — Grounding: not hallucinating your own instruments

**Class (C): the same failure mode, in two workstreams**

- **WS-A:** ask an LLM "what relations exist in this corpus?" and it will
  produce a beautiful, plausible vocabulary the corpus cannot instantiate.
- **WS-B:** ask an LLM "generate questions about this corpus" and it will
  produce questions the corpus cannot answer.

Same disease. Same cure: **nothing enters unless the corpus can instantiate it.**

- A predicate needs **N distinct chunks** containing extractable instances.
- A question needs its expected answer **locatable in source text**.

Anything that fails the check is discarded, and *the discard rate is itself a
result worth reporting*.

## Week 5 · Tue — Behaviour-preserving refactors

**Class (C): WS-A's deliverable is a claim, not just code**

"Moving the vocabulary into a config file changes nothing" is a **hypothesis**.
Evidence required:

- Same eval scores on ≥3 KBs, before and after — within run-to-run noise.
- Graph stats identical: entity count, relation count, type distribution.
- Reviewed and merged, not left on a branch.

> If you cannot demonstrate that a refactor changed nothing, you cannot later
> claim that a change caused something.

**Homework (C):** build your deliverable. **Homework (A):** WS-A — write the
before/after comparison as a test, not a manual check.

## Week 5 · Thu — Writing slot #3

**Class (C): writing Methods when you have no results**

This week's work produces almost no numbers. It still produces two Methods
subsections:

- **§5.1 Ontology representation** — what the schema declares, what constraints
  it enforces, how extraction consumes it.
- **§6.1 Question generation** — tiers, generation procedure, grounding check,
  discard criteria.

Write the procedure so a stranger could re-implement it. Where you have not
decided, write **`TODO(decision)`** rather than vague prose — an honest gap is
findable; a vague sentence is not.

**Homework (C):** commit your subsection. **Homework (A):** review another
team's and list every question you had to ask them.

---

# Week 6 — The first real signal

## Week 6 · Mon — Authoring a domain ontology

**Class (C): worked live, on Legal**

Do **not** design in the abstract. Open five chunks and see what is asserted.

| entity types | relation types |
|---|---|
| Statute, Clause, Party, Obligation, Jurisdiction, Definition | `obliges`, `permits`, `prohibits`, `defines`, `amends`, `supersedes`, `exception_to`, `applies_to` |

Then check each predicate against the corpus: **can you find three real
instances?** If not, cut it. A vocabulary is a hypothesis about the text.

Borrow rather than invent where you can — LegalRuleML and LKIF for law, CiTO for
scholarly citation, SKOS for concept hierarchies.

## Week 6 · Mon — And the scientific ontology

| entity types | relation types |
|---|---|
| Phenomenon, Quantity, Method, Dataset, Instrument, Finding | `causes`, `correlates_with`, `measured_by`, `supports`, `contradicts`, `sample_of` |

**Homework (C):** WS-A authors both ontologies as `ontology.yaml`, each
predicate justified by three corpus instances committed alongside it.

**Homework (A):** a third ontology — **RFCs are nearly free**. RFC 2119 puts the
normative vocabulary in the source text itself: MUST, SHOULD, MAY. The ontology
is *given*, not inferred. Worth doing precisely because it is the easy case.

## Week 6 · Tue — The re-run control

**Class (C): this slide is the one to remember**

You are about to compare the old graph with a new-ontology graph. Between them
you will have changed: the vocabulary, **and** re-run extraction, **and**
possibly the model version, sampling, and chunk set.

Any of those could explain a difference. So a two-arm comparison proves nothing.

**Three arms:**

| arm | what it is |
|---|---|
| 1 | Recorded baseline (2026-08-22) |
| 2 | **Re-extract with the OLD ontology**, today's conditions |
| 3 | Re-extract with the NEW ontology, today's conditions |

**Arm 2 vs 3 is the comparison that matters.** Arm 1 vs 2 measures drift in
everything else — and if that gap is large, that is a finding too.

## Week 6 · Tue — What we predict, on the record

Registered before we look:

- **Legal** — already retrieves at 87.5% with 2 relations. A legal ontology may
  add little. **Ceiling effects are real**; say so now, not afterwards.
- **Climate** — 48.4%, near the bottom, 4 relations. If domain ontology helps
  anywhere, here.

**If both are null, that is a finding, not a failure.** It would strengthen the
paper's central claim from "the wrong ontology does not help" to "ontology
conditioning does not rescue graph-augmented RAG" — a stronger and more
interesting result.

Nobody hides a null. Register the prediction, report the outcome.

## Week 6 · Tue — Homework

**Homework (C):** WS-A runs all three arms on Legal and Climate. WS-B scores
them with the same harness and the same questions — **no changing the exam
between arms.**

**Homework (C), WS-C/WS-D:** your instruments meet real data for the first time.
Report what broke.

**Homework (A):** compute the arm-1 vs arm-2 gap and characterise it. Is our
pipeline reproducible week to week? We do not currently know.

## Week 6 · Thu — First signal, and writing slot #4

**Class (C): results presented, three arms, both KBs**

Read the table together. Questions to ask of it:

- Is the arm 2→3 difference larger than the arm 1→2 drift? If not, we have
  measured noise.
- Did entity and relation *counts* change in the direction we expected, and does
  that track recall — or does it again not?
- Did anything get **worse**? Worse is informative.

## Week 6 · Thu — Risk check before the W7 cut

**Class (C): honest status, per workstream**

Next week is the mid-programme review, where a workstream may be cut to protect
the other three. Nothing is punished; hours are finite and four in flight is
ambitious for 4 hrs/week.

State plainly: on track / at risk / blocked, and what you would need.

**Homework (C):** draft §5.2 Results for the ontology experiment — including the
null case, written as though it happened, so it is ready either way.

**Homework (A):** write the limitation paragraph for your section now. What
would a hostile reviewer say? Two KBs is a small sample; ceiling effects; single
judge model.

## Looking ahead — week 7

| | |
|---|---|
| **Mon** | Compression probe: rate–distortion sweep at R ∈ {1.0, 0.75, 0.5, 0.25} |
| **Tue** | Multi-hop questions — the tier where a graph should finally pay |
| **Thu** | **Mid-programme cut review** + writing |

**The falsifiable bet:** WarPeace's 742 `associated_with` edges should compress
**badly** — they encode nothing, so discarding text loses the answers. D6's
typed kinship should compress **well**.

If that comes out backwards, our framing is wrong, and week 7 is when we want to
learn it.

---

# Week 7 — Compression, multi-hop, and the cut

## Week 7 · Mon — Rate–distortion, in one slide

**Class (C): the formulation**

> **Maximise comprehension, subject to retaining ≤ R of the corpus.**

Sweep *R*, plot the curve. That curve says more than any single score.

The literature calls the sharper version **decision-centric** rate–distortion:
measure the loss in *downstream task quality*, not in reconstruction fidelity.
We adopt that framing.

**Why this is a quality metric, not an efficiency trick:**

> If extraction genuinely captured the facts, you can discard the text they came
> from and still answer. If it did not, you cannot.

Compressibility measures whether the graph captured anything — which relation
count demonstrably does not (r = +0.081).

## Week 7 · Mon — The consolidation gate

**Class (C): the eviction rule**

A chunk becomes evictable only once its claims are:

1. represented in the graph, **and**
2. corroborated by ≥1 independent chunk, **or** explicitly marked *irreducible*

Irreducible content — exact quotations, figures, identifiers, legal text — is
kept verbatim or not at all. The ontology should mark which entity types carry
it.

**The bet, registered now:**

| KB | edges | prediction |
|---|---|---|
| WarPeace | 742 `associated_with` | compresses **badly** — untyped edges encode nothing |
| D6 | typed kinship | compresses **well** — genealogy recitation is droppable |

If that comes out backwards, our framing is wrong. Better to learn it now.

## Week 7 · Mon — Homework

**Homework (C), WS-C:** run the sweep on D6 and WarPeace at
R ∈ {1.0, 0.75, 0.5, 0.25}. Nothing is deleted — evict from a **copy**.

**Homework (C), all:** your instruments now have to survive contact with a
second KB. Report what broke.

**Homework (A):** plot the two curves on shared axes. Which KB's curve falls off
first, and does that match the prediction?

## Week 7 · Tue — Multi-hop: where a graph should finally pay

**Class (C): the tier that matters**

Tiers 1–2 (recall, paraphrase) are answerable by vector search alone — which is
why our current eval cannot see the graph at all.

**Multi-hop requires composing two or more asserted facts.** If the graph is
carrying anything, this is where it shows.

**The circularity trap — the most important warning of the term:**

> Generate questions from the **corpus**, never from the graph.

Generate from the graph and you measure self-consistency. A confidently wrong
graph scores perfectly. The ontology may *shape* questions; the corpus is the
only ground truth.

## Week 7 · Tue — Homework

**Homework (C), WS-B:** tier-3 generation. Each question must require ≥2 facts,
each locatable in a *different* chunk, with both cited in the answer key.

**Homework (C), WS-A/C/D:** supply WS-B with three example multi-hop questions
from your KB, by hand. Hand-written examples calibrate the generator.

**Homework (A):** measure the gap. Run tier-1 and tier-3 sets against the same
KB. If tier-3 is not markedly harder, the generator is not producing genuine
multi-hop questions.

## Week 7 · Thu — Mid-programme cut review

**Class (C): honest triage**

Nine weeks remain. Four workstreams at 4 hrs/week is ambitious by design — this
session decides whether it stays that way.

Each team, five minutes: **on track / at risk / blocked**, what you would need,
and what you would cut first.

**The cut rule:** we protect the critical path — Sections 4, 5, 6 (null result,
ontology, evaluation). WS-D is explicitly the most cuttable, and being cut is
not a failure; it becomes a workshop paper or the next cohort's start.

**Homework (C):** whatever the review decided, revise your section plan to match
and commit it.

---

# Week 8 — Induction and streaming

## Week 8 · Mon — Inducing an ontology

**Class (C): the third source**

Week 6 you *declared* an ontology by hand. That does not scale to fifteen KBs,
let alone a stream. So: induce it.

1. **Sample** the corpus — a stratified handful of chunks.
2. **Propose** — ask the model what kinds of relationship hold between entities
   *in this text*.
3. **Cluster** near-duplicate proposals into candidate predicates.
4. **Ground** — a predicate is admitted only with N distinct instantiating
   chunks.
5. **Ratify** — a human accepts or rejects. Cheap to review, expensive to undo.

`AutoSchemaKG` is the published version of this loop; read it before building.

## Week 8 · Mon — Homework

**Homework (C), WS-A:** induce vocabularies for three KBs you have *not*
hand-authored. Present proposals with their grounding evidence for ratification
on Tuesday.

**Homework (A):** compare induced against hand-authored for Legal — where you
have both. Precision (proposed predicates that survive grounding) and recall
(hand-authored predicates the induction missed). **This comparison is a paper
figure.**

> Induced-vs-declared is one of the few places we can quantify how good the
> automatic path is. Do it carefully.

## Week 8 · Tue — Running to equilibrium

**Class (C): what steady state means**

Static corpus → "how good is this KB". Stream → **"what comprehension can be
*sustained* at ingest rate λ within memory M"**.

That is a capacity question, and it is the right headline number for a node with
a fixed disk.

New metrics:

- **Retention curve** — comprehension as a function of content age
- **Forgetting errors** — Type I (discarded, later needed) / Type II (retained,
  never needed)
- **Consolidation lag** — ingest → fact-in-graph
- **Eviction eligibility** — fraction passing the gate; a cheap extraction-quality
  proxy needing no eval run

## Week 8 · Tue — Homework

**Homework (C), WS-D:** first replay to equilibrium. Feed a corpus as a timed
stream into a memory-capped KB. Report where it stabilises — or that it does
not.

**Homework (A):** vary the cap. Does comprehension degrade gracefully or fall
off a cliff? A cliff would be the most interesting result of the week.

## Week 8 · Thu — Methods complete

**Class (C): the gate**

Every section's Methods subsection is finished this week. Not drafted — finished.

Test: hand it to someone outside your workstream. If they cannot re-run your
procedure from the text alone, it is not finished.

Remaining `TODO(decision)` markers are resolved now, because from week 9 you are
running integrated experiments and changing the method mid-flight invalidates
what came before.

**Homework (C):** final Methods, reviewed by another team.
**Homework (A):** write your section's *threats to validity* paragraph while the
method is fresh.

---

# Week 9 — Integration

## Week 9 · Mon — One configuration, one truth

**Class (C): why integration is not just "run everything"**

Until now each workstream measured in its own conditions. Those results are **not
comparable** and must not appear in one table.

Integration means: one KB set, one eval harness, one model, one judge, one seed
policy, one week. Everything re-measured together, even results you already have.

> Any number in the paper that was not produced under the frozen configuration
> is a liability. Reviewers find these.

**Homework (C):** agree the frozen configuration as a committed file. Every team
runs against it and nothing else from here.

## Week 9 · Tue — The first end-to-end table

**Class (C): building the results table**

Rows are conditions, columns are metrics, and **every cell must be reproducible
from a logged command**.

- Baseline (memoir ontology, no compression)
- \+ domain ontology
- \+ curriculum eval (tiers reported separately)
- \+ compression at each R
- Streaming steady state

**Class (C): figures**

Draft all of them now, however ugly. A figure you cannot draft is a result you
have not understood.

**Homework (C):** your section's rows, populated. **Homework (A):** the main
figure, in the tool you will actually use for the final version — not a sketch
you will have to redo.

## Week 9 · Thu — Writing slot: Results

**Class (C): describing results without arguing about them**

Results state what happened. Discussion argues what it means. Students merge the
two and reviewers punish it.

- "Recall increased from 48.4% to 61.2% (+12.8pp)" — Results.
- "…demonstrating that domain ontologies improve retrieval" — Discussion, and
  only if the ablation supports it.

**Homework (C):** draft your Results subsection, past tense, no interpretation.
**Homework (A):** for each number, write the one sentence a hostile reviewer
would use to dismiss it.

---

# Week 10 — Ablations

## Week 10 · Mon — What separates a paper from a demo

**Class (C): the ablation principle**

You will claim several things caused an improvement. An ablation asks, for each
one: **what happens without it?**

Without ablations you have a system that works and no idea why — which is a
demo, not a contribution.

| claimed gain | ablation |
|---|---|
| domain ontology | run with entity types only, no relation types |
| domain/range constraints | ontology with constraints disabled |
| grounding check | admit ungrounded predicates and see what enters |
| consolidation gate | evict by redundancy alone |
| curriculum tiers | score all tiers as one pooled number |

**One change at a time.** Two changes and you have learned nothing.

## Week 10 · Tue — Running them

**Homework (C):** each team runs its own ablations under the frozen
configuration. Log every command.

**Homework (A):** an ablation that *improves* the result is the most valuable
outcome available this week — it means a component is hurting. Look for it
honestly rather than hoping it does not appear.

> The most common failure in student ablations is running only the ones expected
> to confirm the design.

## Week 10 · Thu — Statistics for n=15

**Class (C): small samples, honest claims**

- Report confidence intervals, not point estimates.
- Distinguish a **null** (our r = +0.081 — safe to lean on) from an
  **underpowered non-finding** (the entity correlation at +0.397 — *not*
  established, and we do not claim it).
- Two KBs in the week-6 experiment is a very small sample. Say so in the text
  before a reviewer says it for you.

**Homework (C):** add intervals to every headline number.
**Homework (A):** a permutation test on the relation-count correlation. Does
+0.081 survive as a null under resampling?

---

# Week 11 — Robustness and negative results

## Week 11 · Mon — Would this survive a different setup?

**Class (C): the confounds we have not controlled**

Every number so far used one embedding model, one generator, one judge.

- **Judge choice** — LLM-as-judge is nondeterministic and model-dependent. If
  conclusions flip with the judge, they are about the judge.
- **Embedding model** — retrieval is carried by vectors; changing them may swamp
  every graph effect we measured.
- **Seeds and ordering** — dedup and merge are order-dependent. We saw this
  concretely: the Joseph/Samuel Rassool mis-merge.

**Homework (C):** re-run headline conditions with a second embedding model and a
second judge. Report agreement, not just scores.

## Week 11 · Tue — Negative results are results

**Class (C): the section most students omit**

We already hold several, and they are among the paper's most useful content:

- Relation count does not predict retrieval quality across 15 domains.
- Adding 114 relations to D6 moved recall 0pp.
- Untyped `associated_with` edges are the only thing extraction produced.
- Every dream result before the `--model` fix is unreliable — a whole month of
  reported outcomes invalidated by one flag default.

Reviewers trust a paper that reports what failed. More importantly, so should
you.

**Homework (C):** draft the negative-results subsection.
**Homework (A):** find one *more* negative result in your own workstream. There
is always one; it is usually being quietly ignored.

## Week 11 · Thu — Writing slot: Discussion

**Class (C): claims must map to evidence**

Every claim in the Discussion points at a specific table or figure. Write the
pointer in brackets as you draft — `(Table 3)` — and delete any claim that has
nowhere to point.

**Homework (C):** Discussion draft with every claim mapped.
**Homework (A):** the limitations paragraph. Ceiling effects, small n, single
corpus per domain, English only, one model family.

---

# Week 12 — Results frozen

## Week 12 · Mon — What "frozen" means

**Class (C): the discipline**

From today, **no new experiments** except those a reviewer would obviously
demand and the advisor approves.

Why this is not bureaucracy: experiments running into the writing period produce
papers written in a panic, numbers that disagree between sections, and figures
regenerated the night before submission. The freeze is what makes the last four
weeks calm.

**What is still permitted:** re-running an existing condition to fix a bug;
regenerating a figure; adding an interval to an existing number.

## Week 12 · Tue — Final tables and figures

**Homework (C):** every table final, every figure at publication quality —
readable in greyscale, axes labelled with units, captions that stand alone.

**Homework (A):** the caption test. Hand someone only your figure and its
caption. If they cannot say what it shows, the caption is not finished.

## Week 12 · Thu — Full draft assembly

**Class (C): reading it as one document**

Four teams wrote four sections. Today it becomes one paper.

Look for: terminology drift (the same thing named three ways), numbers that
disagree between sections, claims in the intro the results never deliver,
duplicated related work.

**Homework (C):** complete draft in Overleaf, all sections, all figures placed.
**Homework (A):** a terminology table — one agreed term per concept — and apply
it throughout.

---

# Weeks 13–14 — Writing

## Week 13 · Mon — The abstract and the funnel

**Class (C): structure**

The intro is a funnel: the field → the gap → what we did → what we found → why
it matters. Four paragraphs, rarely more.

The abstract is the paper in eight sentences, and it is what most people will
read. Write it **last**, after the results are frozen — never first.

**Our claim, plainly:** across 15 domains a shared extraction vocabulary
contributes nothing measurable to retrieval; conditioning on a per-domain
ontology changes that; and compressibility is a better diagnostic of graph
quality than any count.

**Homework (C):** abstract draft, eight sentences.
**Homework (A):** rewrite it for a reader outside the field. If you cannot, you
do not yet know what you found.

## Week 13 · Tue–Thu — Drafting and internal review

**Tue (C):** section-by-section revision against the claim map. Cut anything not
serving the claim, however much work it was. *(This is the hardest instruction
in the course.)*

**Thu (C): internal review round.** Each team reviews another's section as a
hostile reviewer: unsupported claims, missing baselines, unclear methods,
over-reach.

**Homework (C):** written review of another section, in reviewer format —
summary, strengths, weaknesses, questions.

> Reviewing badly-argued work teaches more about writing than any style guide.

## Week 14 · Mon — Related work as positioning

**Class (C): not a list**

Related work says **where you sit**, not what exists. Each entry: what they did,
and how you differ.

Position against the scan:

- Ontology-guided KG construction — we test *cross-domain* mismatch, they
  assume one domain.
- Rate–distortion memory compaction — we use compressibility as a *diagnostic*,
  not only as an objective.
- Dynamic benchmarks — we apply curriculum tiers to *knowledge bases*, not
  models.

**Homework (C):** related work complete, every entry differentiated.

## Week 14 · Tue–Thu — Final pass and arXiv

**Tue (C):** consistency pass — numbers, terminology, references, figure
callouts, author list and affiliations.

**Thu (C): arXiv preprint submitted.** Everyone has a citable artifact from
today, regardless of how long journal review takes.

**Homework (A):** draft the tweet-length and paragraph-length summaries. Being
able to compress your own contribution is the last comprehension test of the
course.

---

# Weeks 15–16 — Reproducibility and submission

## Week 15 · Mon — The fresh-machine test

**Class (C): what reproducibility actually requires**

Not "the code is on GitHub". The test:

> A stranger, on a clean machine, following only the README, reproduces the
> headline number.

Required: exact commands, pinned model versions, seeds, dataset manifests with
checksums, environment specification, and expected outputs with tolerances —
because LLM pipelines are not bit-reproducible and the tolerance must be stated.

**Homework (C):** write the README as if for a stranger. **Homework (A):** be
the stranger — run another team's package on a clean environment and log every
failure.

## Week 15 · Tue–Thu — Package and merge

**Tue (C):** code merged to `main` behind review. Not left on branches. This is
Kwaai's codebase, and the work must survive the cohort.

**Thu (C):** data and code availability statements; licence check; anonymised
artifact if the venue requires double-blind.

**Homework (A):** archive a tagged release and mint a DOI (Zenodo) so the paper
can cite an immutable artifact.

## Week 16 · Mon — Submission mechanics

**Class (C): the unglamorous part**

- Venue formatting, length limits, figure resolution.
- Cover letter: what the paper claims and why it fits this venue.
- Suggested reviewers, conflicts declared.
- **Prepare for reviews now** — list the three objections you expect, and where
  the answer already sits in the paper.

**Homework (C):** submission checklist complete.

## Week 16 · Tue — Submit

**Class (C):** submission, together, in the session.

Then: what happens next. Review takes months and will outlast the cohort.
Whoever stays on handles the response; everyone is kept informed and credited
regardless.

## Week 16 · Thu — Showcase and retrospective

**Class (C): present your workstream** — 10 minutes each: question, method,
result, what you would do differently.

**Class (C): retrospective, feeding the next cohort**

- What took longer than planned?
- Which week was most valuable? Which was wasted?
- What should the next cohort be told in week 3 that you learned in week 12?

## What you will have done

- Reproduced a published-quality result before trusting it.
- Registered predictions before measuring, and reported a null honestly.
- Designed a control that separates your effect from everything else changing.
- Ablated your own contribution to find out which part mattered.
- Written a methods section a stranger could follow.
- Shipped code into a real open-source system, reviewed and merged.
- Co-authored a paper.

Most graduate courses deliver two or three of these. Fourteen weeks at four
hours is enough for all of them, but only because the failure that motivates the
work has already been measured — and because we will not pretend it did not
happen.
