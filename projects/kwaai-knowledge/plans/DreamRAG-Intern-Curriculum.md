# DreamRAG intern curriculum — weeks 3–16

Adapts the existing Kwaai intern curriculum (`Kwaai Intern Program.pdf`, the
8-week undergraduate Master Class) toward extended DreamRAG, targeting a
publishable paper plus merged code.

**Assumed parameters** — flag if wrong, they drive everything below:

- Graduate track: **16 weeks, 4 hrs/week, standups Mon/Tue/Thu**. Two weeks
  complete, so **14 remain ≈ 56 hrs per intern**.
- Undergraduates on the 8-week / 8 hrs-a-week track can be folded in; see
  *Running both cohorts*.
- Unpaid volunteers. Attrition and uneven skill are design constraints, not
  surprises.

## Why pivot, and what we are pivoting from

The existing curriculum builds toward "centralized vs decentralized RAG"
benchmarking. That is a fine undergraduate arc but a weak paper: it measures an
engineering choice, not a research question, and the answer is largely known.

Three things changed in the week of 2026-08-16:

1. We built a **cross-KB baseline** — 15 knowledge bases across memoir, science,
   law, standards, software docs and poetry, each with its own question set.
2. We got a **clean negative result**: relation count does not predict retrieval
   quality (Pearson **+0.081**, n=15). Adding 114 relations to D6 moved recall
   0pp. WarPeace with 742 relations retrieves *below median*; Legal with 2 tops
   the table at 87.5%.
3. We found **why**: the entire relation vocabulary is thirteen kinship
   predicates plus `related_to`. A memoir ontology, applied to permafrost papers
   and RFCs.

That is a publishable shape already — a measured failure with an identified
cause. The interns' work is to build and evaluate the remedy, which is a far
better research experience than re-benchmarking a known comparison.

**The pedagogical arc is preserved**: experiment-first, versioned experiments,
core/additional homework differentiation, Overleaf paper from week one of the
pivot, living `Methods.md`. Only the research question changes.

## The paper

**Working title:** *One ontology does not fit all: measuring and repairing
domain mismatch in graph-augmented retrieval*

**Thesis.** Graph-augmented RAG is routinely evaluated on a single domain. Across
15 domains with a shared extraction vocabulary, the graph contributes nothing
measurable. Conditioning extraction on a per-domain ontology recovers the
benefit, and the achievable compression ratio is a better diagnostic of graph
quality than any count of nodes or edges.

**Why it can land.** The negative result is already collected and is unusual —
most work reports gains on one corpus. A multi-domain null plus a remedy plus a
new diagnostic is a complete story. The infrastructure exists, so intern hours
go to experiments rather than plumbing.

**Structure, mapped to workstreams:**

| section | content | workstream |
|---|---|---|
| 1–2 | Intro, related work | all (shared) |
| 3 | The multi-domain testbed: 15 KBs, corpora, question sets | WS-B |
| 4 | **The null result** — already collected, needs replication | WS-B |
| 5 | Ontology conditioning: declared, selected, induced | WS-A |
| 6 | Dynamic curriculum evaluation | WS-B |
| 7 | Compression as a quality diagnostic | WS-C |
| 8 | Streaming and steady state | WS-D |
| 9 | Discussion, limitations, negative results |all |

**Venues.** Primary: *IEEE TKDE* or *ACM TOIS* — both take multi-domain
empirical work on knowledge systems. Ontology-forward alternative: *ISWC*
(conference, but high-prestige for this exact topic). **arXiv preprint at week
14 regardless**, so interns have a citable artifact even if review runs long.
A workshop paper is the fallback if a workstream fails.

**Authorship**, agreed in week 3 and written down, not left implicit: every
intern who completes a workstream and contributes text is a co-author. Ordering
by contribution, decided by the advisor, disclosed in advance. This is the
program's stated deliverable — treat it as a commitment.

## Workstreams

Four teams of 2–3. Each owns a paper section and can produce a standalone result
if the others slip — deliberate, given volunteer attrition.

**WS-A · Ontology.** Move the vocabulary out of `graph.rs` constants into a
per-KB `ontology.yaml` with entity types, relation types, and domain/range
constraints. Author two contrasting ontologies by hand (legal, scientific).
Build a grounded induction pipeline: propose predicates from a corpus sample,
require N distinct instantiating chunks before admission, human ratifies.
Re-extract and measure against the week-3 baseline.

**WS-B · Evaluation.** Replicate the null result independently (first task, and
non-negotiable — see *Replication gate*). Build a corpus-grounded question
generator with difficulty tiers (recall → paraphrase → multi-hop → synthesis →
inference). Add abstention scoring, where "I no longer hold that" is correct for
discarded content. Separate retriever from generator diagnostics.

**WS-C · Compression.** Implement the consolidation gate (a chunk is evictable
once its claims are in the graph and independently corroborated). Run
rate–distortion sweeps at retention budgets R ∈ {1.0, 0.75, 0.5, 0.25}. Test the
prediction that WarPeace's 742 `associated_with` edges compress *badly* while
D6's typed kinship compresses well — a cheap, early, falsifiable test of the
whole framing.

**WS-D · Streaming & harness.** Build a stream-replay harness: feed an existing
corpus as a timed stream into a memory-capped KB, measure steady state. Then
harness v0, **observe-only** — it reports what it would do and why, without
permission to act.

## Replication gate (week 4, hard)

Before anyone builds anything, WS-B must independently reproduce the null
result from the raw corpora. If they cannot, the paper has no Section 4 and the
plan changes.

This is the single most important week. It teaches that a result you did not
reproduce is a rumour, and it protects us from building on a finding that came
out of one long unsupervised session with known operator errors — see
*What went wrong last week*, below, which interns should read.

## Week by week

Standups Mon/Tue/Thu, 10–15 min, then lab. **C** = core, **A** = additional for
faster interns — carried over from the existing deck.

**W3 — Orientation to the real system.** Read the plan docs and the literature
scan. Set up KwaaiNet, run `rag eval` on one KB, reproduce one number from the
baseline table. Teams and workstreams assigned; authorship policy agreed;
Overleaf skeleton with all nine section headings created and committed.
*(A: read one primary paper from the reading order and present it in 5 min.)*

**W4 — Replication gate.** WS-B reproduces the cross-KB baseline end to end.
Others instrument their area and write their Methods subsection *first* — before
results exist, so the method is designed rather than rationalised.
**Gate: baseline reproduced within ±3pp, or replan.**

**W5 — Ontology v1 and question generator v1.** WS-A ships `ontology.yaml`
loading with the current kinship set as the `memoir` default (behaviour
unchanged — a pure refactor, merged and reviewed). WS-B ships tier-1/2 question
generation with a grounding check. WS-C ships the consolidation gate as a
*measurement* only: what fraction of chunks would be evictable. WS-D ships the
replay harness skeleton.

**W6 — First real signal.** WS-A hand-authors the legal and scientific
ontologies and re-extracts those two KBs. **First interesting measurement of
the program**: does a domain-appropriate ontology change recall on Legal and
Climate? *(A: a third ontology — RFC 2119 normative vocabulary is nearly free,
since MUST/SHOULD/MAY is in the source text.)*

**W7 — Compression probe.** WS-C runs the R-sweep on D6 and WarPeace and tests
the falsifiable prediction. WS-B adds multi-hop questions — the tier where a
graph should finally pay. Mid-programme review: which workstreams are on track,
what gets cut.

**W8 — Induction.** WS-A builds grounded predicate induction and proposes
vocabularies for three unseen KBs; humans ratify. WS-D runs the first
stream-replay to equilibrium. **Draft Methods complete for all sections.**

**W9 — Integration.** All four workstreams run against the same KB set with the
same eval. First end-to-end results table. Figures drafted, however ugly.

**W10 — Ablations.** The part that separates a paper from a demo. For each
claimed gain, what happens without it? Ontology without domain/range
constraints; induced versus hand-authored; compression without the
consolidation gate. *(A: statistical treatment — n=15 is small, so report
confidence intervals and resist over-claiming, exactly as the baseline analysis
does.)*

**W11 — Robustness and negative results.** Re-run with a second embedding model
and a second judge model. Record what did *not* work — a section reviewers value
and students routinely omit.

**W12 — Results frozen.** No new experiments after this week except those
reviewers would obviously demand. Full results tables and final figures.

**W13–14 — Writing.** Each team drafts its section; advisor edits for one voice.
Related work completed against the reading list. **arXiv preprint submitted at
the end of W14.**

**W15 — Reproducibility package.** Scripts, seeds, exact commands, dataset
manifests, environment. A reviewer must be able to re-run the headline number.
Code merged to `main` behind review, not left on branches.

**W16 — Submission and showcase.** Journal submission. Each intern presents
their workstream. Retrospective feeding the next cohort.

## Running both cohorts

Undergraduates (8 weeks, 8 hrs/week) join at W5 and run to W12, doing the
existing weeks 1–4 material compressed into their first fortnight. They take the
*measurement* tasks — running sweeps, scoring, building question sets — which
are genuinely useful, teach the method, and do not block the critical path.
Graduates take design and analysis. Both are co-authors under the same policy.

## What went wrong last week — required reading

Interns should read this; it is the honest version of how the result was
produced, and every item is a lesson the curriculum encodes.

- A flag defaulting to the literal string `"default"` made every LLM call fail.
  It went unnoticed for hours because failures were silently indistinguishable
  from "nothing to do". **Lesson: instrument failure paths before running
  anything unattended.**
- Cycles were optimised for throughput — completions, relations added — while
  the metric anyone cared about did not move. **Lesson: state the objective
  metric before starting, and report it, not activity.**
- A destructive step ran on cycles that accomplished nothing, costing 14 of
  D6's 237 relations. **Lesson: snapshot before mutation; verify before
  destroying.**
- The first conclusion — "relations do not help" — was wrong, because every
  relation measured was generic or mismatched. **Lesson: a null result bounds
  what you measured, not what is true.**
- Several orchestration bugs (shell quoting, awk column indices twice, a counter
  that should have been per-KB, a `pkill` matching its own process). **Lesson:
  the boring parts break most; test the harness, not just the science.**

## Risks

| risk | mitigation |
|---|---|
| Attrition — unpaid volunteers over 14 weeks | Workstreams produce standalone results; no single point of failure |
| Replication fails at W4 | Paper pivots to "why the baseline was unreproducible", which is still publishable and more honest |
| Ontology work shows no gain | That is a *finding*, not a failure — it would strengthen the null result into a stronger claim. Say so up front so nobody hides it |
| Scope creep into the harness | WS-D is observe-only by design and explicitly cuttable to a workshop paper |
| 56 hrs/intern is not enough | W7 mid-programme review exists to cut a workstream rather than fail four |
| Undergraduate skill gap | Measurement tasks are real contributions and are on the non-critical path |

## What interns actually learn

Worth stating, since it is a teaching programme and not just labour: how to
reproduce a result before trusting it; how to design an experiment whose
negative outcome is still informative; ablation as the difference between a
demo and a paper; that reporting activity instead of outcomes is the commonest
self-deception in applied ML; and how to write a methods section before you have
results, which is the fastest way to discover your method is underspecified.
