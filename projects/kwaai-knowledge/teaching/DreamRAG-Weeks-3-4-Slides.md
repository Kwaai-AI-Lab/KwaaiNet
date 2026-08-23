% Kwaai Intern Program — DreamRAG Track
% Weeks 3–4 · Graduate Cohort
% 4 hrs/week · standups Mon / Tue / Thu

## The pivot, in one slide

Weeks 1–2 taught RAG and built a baseline. Weeks 3–16 do research.

**What changed:** we measured our own system across 15 knowledge bases and it
failed — and we think we know why.

- You are not re-running a known comparison.
- You are testing whether our diagnosis is correct.
- The result — either way — is the paper.

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
