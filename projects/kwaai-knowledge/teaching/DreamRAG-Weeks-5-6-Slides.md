% Kwaai Intern Program — DreamRAG Track
% Weeks 5–6 · Graduate Cohort
% Building the instruments, then the first real signal

## Where we are

Weeks 3–4 established the ground: you reproduced the null result and wrote your
methods before having results.

Weeks 5–6 build. All four workstreams run in parallel from here.

- **W5** — build the instruments. Nothing should change behaviour yet.
- **W6** — the first measurement that could tell us we are wrong.

Each session: one shared concept (everyone), then split by workstream.

*C = core · A = additional*

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
