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

---

# Refinement: dynamic ingest

Reza, 2026-08-23. The framing above assumed a static corpus. It is not one. The
real case is continuous streams — email, messages, social media, news,
financial, health telemetry — arriving indefinitely into a store of finite size.

This is not an extra requirement bolted onto the three themes. It is the
constraint that makes them one problem, and it changes each of them.

## What changes

With a static corpus, compression is an optimisation you may decline. With an
unbounded stream and finite memory, **forgetting is mandatory** — the only
question is whether it is principled or accidental. Today it is neither: nothing
is discarded, so the system simply fills up and the question of what to lose is
answered by whatever breaks first.

Three consequences:

**The problem becomes steady-state, not batch.** The question stops being "how
good is this KB" and becomes "what comprehension can be *sustained* at ingest
rate λ within memory M". That is a capacity question, and it has an answer we
can measure and optimise.

**Facts acquire validity windows.** "Where does Reza work?" has a
time-dependent answer. A static corpus lets you ignore this; a stream does not.
`entity_timeline_v1` and `sequence.rs` already extract temporal events, so the
substrate exists — but the graph stores assertions without validity intervals,
so a new fact contradicting an old one is a conflict rather than a succession.

**Evaluation must distinguish two kinds of forgetting.** *Intended* forgetting
compresses detail away but leaves the gist answerable. *Unintended* forgetting
loses the fact entirely. A single recall score cannot tell them apart, and they
demand opposite responses — the first is the system working, the second is data
loss.

## The biological correspondence, precisely

Reza's framing invokes biological memory. The analogy is load-bearing in places
and decorative in others; worth separating them.

**Genuinely structural:**

- *Hippocampal → neocortical consolidation.* Recent episodic memory is replayed
  offline and integrated into semantic memory. This is exactly what a dream
  cycle does: read recent chunks, extract durable facts into the graph. The
  correspondence is close enough to borrow from — including that consolidation
  happens *offline*, on a schedule, not in the ingest path.
- *Gist versus verbatim* (fuzzy-trace theory). Humans retain gist and lose
  surface form. Chunk → summary → fact is the same gradient, and it tells us
  compression should be *tiered*, not binary keep/discard.
- *Retrieval as reinforcement.* Recalling a memory strengthens it. A fact
  retrieved often should resist eviction; one never retrieved is a candidate.
- *Interference.* Similar memories corrupt each other. We have already seen
  this concretely — the Joseph/Samuel Rassool mis-merge came from two similar
  entities colliding during dedup.

**Decorative, and we should not over-fit to it:** sleep stages, synaptic
mechanisms, dream content as narrative. The metaphor earns its keep as
consolidation-plus-forgetting; pushing it further will produce architecture
chosen for poetic fit rather than function.

## Streams are not interchangeable

The six named streams differ along axes that determine policy:

| stream | velocity | value density | structure | forget aggressively? |
|---|---|---|---|---|
| health telemetry | very high | very low per sample | numeric | yes — keep aggregates + anomalies |
| messages | high | high | semi-structured | selectively |
| email | medium | high | semi-structured | selectively, thread-aware |
| social media | high | low | unstructured | mostly |
| news | medium | low, decays fast | unstructured | aggressively, keep what proved relevant |
| financial | low | high, legally retained | structured | rarely — retention obligations |

Two things follow. **Compression is polymorphic**: summarising text and
downsampling a heart-rate series are different operations, and telemetry should
never enter the text pipeline at all — you keep hourly aggregates plus flagged
anomalies, which is compression by construction. And **retention policy is
per-stream**, not global: financial records may carry legal retention
requirements while social media is disposable, in the same KB.

This also means the ontology question compounds. A single personal KB ingesting
all six streams needs either several ontologies or one that spans them — and the
earlier open question ("does a KB have one ontology or several?") stops being
theoretical.

## Consolidation and eviction

The memory hierarchy the system claims to have needs an actual transfer rule.
The natural one:

> **A chunk becomes evictable only once its claims are represented in the graph,
> corroborated by at least one independent chunk or explicitly marked
> irreducible.**

That gate does real work. It ties eviction to extraction quality — a KB whose
extraction is poor cannot evict, so its storage grows and the pressure is
*visible* rather than silent. It also makes the earlier compressibility metric
operational: a system that cannot evict is telling you its graph is not
capturing anything.

Irreducible content needs an explicit escape hatch: exact quotations, numbers,
identifiers, legal text. Some things must be kept verbatim or not at all, and
the ontology should mark which entity types carry irreducible detail.

Long-term memory needs compaction too. Semantic memory generalises — you recall
"I have been to Paris several times", not each trip. The graph equivalent is
merging repeated episodic assertions into a single generalised one with a count
and a time span, which is both compression and a better representation.

## Distributed memory: the KwaaiNet-specific part

Finite memory per node is the constraint. But KwaaiNet is a *network* of nodes
with heterogeneous budgets, and VPK already provides encrypted multi-tenant
storage across peers. That admits something a single machine cannot do:

> **A node may forget locally what the network retains.**

Hot memory local, cold memory distributed, retrieval fanning out only when local
gist proves insufficient. Memory becomes a property of the network rather than
of the node — which is a genuinely different position from every single-machine
RAG system, and it uses the storage fabric and trust layers rather than treating
them as unrelated projects.

It also gives a concrete answer to a question the compression work would
otherwise face: what do you do with content that is low-value locally but not
worthless? You demote it to a peer rather than destroying it, and the
irreversibility problem that makes compression dangerous largely dissolves.

The trust layer decides which peers may hold which privacy class — health and
financial streams have different rules from news.

## Metrics for a steady state

The static metrics do not survive the reframe. What replaces them:

- **Sustained comprehension** — score at equilibrium under continuous ingest,
  not after a one-off build.
- **Capacity** — maximum λ sustainable at memory M holding comprehension ≥ C.
  The headline number for a constrained node.
- **Retention curve** — comprehension as a function of content age. Separates
  "recent things work" from "old things survive".
- **Forgetting errors**, as a first-class pair:
  - *Type I* — discarded something later needed. Data loss.
  - *Type II* — retained something never needed. Wasted budget.
  Optimising one alone is trivial and useless; the trade-off is the point.
- **Consolidation lag** — time from ingest to fact-in-graph. Determines how long
  the system is holding unconsolidated bulk.
- **Eviction eligibility** — fraction of chunks that pass the consolidation
  gate. A direct, cheap proxy for extraction quality that needs no eval run.

## What this does to the path forward

The earlier order — evaluation, then ontology, then compression — still holds,
because we still cannot measure. But the target changes: build the instrument
for a *stream*, not a corpus, or it will need rebuilding.

Concretely, revising the four first steps:

1. **Ontology as a per-KB artifact** — unchanged, still the mechanical unblocker,
   but the schema should carry per-stream sections and an `irreducible` flag on
   entity types from the start.
2. **Question generator** — generate against a *time-windowed* corpus, so the
   same machinery produces both recency and retention questions, and keep the
   bank so regression and forgetting errors are detectable.
3. **Two contrasting ontologies** (legal, scientific) — unchanged.
4. **Compression probe** — unchanged as a cheap early test of the framing, but
   add the consolidation gate as the eviction rule rather than picking chunks by
   redundancy alone.

And one new step, which the streaming frame makes the obvious next experiment:

5. **A synthetic stream harness.** Replay an existing corpus as a timed stream
   into a memory-capped KB, and measure the steady state: capacity, retention
   curve, forgetting errors. `rag sync --watch --interval` is already an
   incremental ingest loop, so this is closer than it looks. Without a harness
   that can actually run the system to equilibrium, every claim in this section
   stays theoretical.

## Open questions this adds

- **What is the unit of forgetting?** A chunk, a document, a thread, a time
  window? Threads and conversations argue against chunk-level eviction.
- **Does consolidation need contradiction handling?** A stream will assert
  things that conflict with earlier facts. Succession (with validity intervals)
  and contradiction (one of them is wrong) look identical without a temporal
  model.
- **Who sets the memory budget?** A per-node constant, or negotiated against
  what the network will hold on the node's behalf?
- **Can the eval bank itself be forgotten?** It grows without bound too, and an
  exam that only ever accretes will eventually cost more to run than the system
  it measures.

---

# Refinement: the agentic harness

Reza, 2026-08-23. A steady-state memory system cannot be operated by a human
typing CLI commands. It needs an agent running the loop. This is the same
"agentic harness" block in `PublicRelease-plan.md`, and DreamRAG maintenance is
its first real customer.

## Why, concretely

The evidence is this session. Running the loop by hand for twelve hours produced:

- a flag bug (`--model default`) that silently voided every completion, unnoticed
  for hours because failures were indistinguishable from no-ops;
- cycles optimised for *throughput* — cycles run, completions made, relations
  added — when the goal was comprehension, which did not move at all;
- destructive dedup and prune running on cycles that had accomplished nothing,
  costing 14 of D6's 237 relations before a guard was added;
- a conclusion ("relations do not help") that survived until a human pointed at
  the ontology.

Those are the failure modes the harness has to be designed against, not
incidental mistakes. Each maps to a requirement.

## What the loop actually is

Sense → decide → act → verify, with escalation:

**Sense.** The steady-state metrics: capacity, retention curve, forgetting
errors (Type I/II), consolidation lag, eviction eligibility, ontology coverage.
Note these must exist before the harness can be built — which is the third
independent argument for doing evaluation first.

**Decide.** Identify the binding constraint. Storage near budget → consolidate
and evict. Retention curve sagging at 30 days → consolidation is lossy, not
storage. Ontology coverage falling as a stream drifts → propose predicates.
Comprehension flat while storage climbs → extraction is failing, stop ingesting
until it is fixed.

**Act.** Invoke primitives, nearly all of which already exist as CLI commands:
`rag sync`, `graph build`, `extract-relations`, `dream run`, `summarize`, `eval`.
Missing: `evict`, `ontology propose/apply`, `questions generate`.

**Verify.** Every action carries a predicted metric movement, and the harness
checks it. An action that does not move its metric is reverted or escalated —
not repeated. Tonight's guard (do not mutate when nothing completed) is a
primitive instance of exactly this.

## The split that keeps it honest

The word "agent" invites building an LLM that does everything. That would be
worse, and tonight shows why: every bug I hit was in *orchestration* — shell
quoting, awk column indices, a global counter that should have been per-KB,
`pkill` patterns matching their own process. An LLM is not better at those than
a control loop; it is worse, because it is nondeterministic about them.

So:

**Deterministic control loop** owns scheduling, thresholds, budgets, retries,
snapshots, invariants. Boring, testable, and where the reliability lives.

**LLM agent** is called only for judgements that are genuinely semantic:

- proposing ontology extensions from drifting content;
- deciding whether a new fact *succeeds* an old one or *contradicts* it;
- judging whether a compression lost meaning, when the metric is ambiguous;
- generating and grading curriculum questions;
- diagnosing *why* a metric moved, as opposed to detecting that it did.

If a decision can be expressed as a threshold, it should be. The agent is for
the residue.

## Safety properties, derived from the failure modes above

1. **Goal is comprehension per unit storage.** Never throughput. A harness that
   reports "2,000 completions" while recall is flat has learned the wrong
   objective — as I did.
2. **No destructive action without a verified hypothesis.** Predict the metric
   movement, snapshot, act, verify, roll back on failure. Eviction demotes to a
   peer rather than deleting, so most actions are reversible by construction.
3. **Halt on regression, do not continue.** A retention-curve drop is a stop
   condition, not a data point to average away.
4. **Bounded budget.** Compute, storage delta, and wall-clock per cycle. Tonight
   consumed ~2,000 remote completions on work of no measured value; an agent must
   not be able to do that unsupervised.
5. **Report the metric, not the activity.** The status a human sees is
   comprehension, capacity and forgetting errors — never cycle counts.

## Where a human stays in the loop

- **Ontology ratification.** Induced predicates are proposals. A grounded
  proposal with example instances is cheap for a human to accept or reject, and
  wrong ontologies are expensive to unwind.
- **Contradiction resolution** when succession versus error cannot be settled
  from timestamps.
- **Eviction beyond a threshold**, or of anything marked irreducible.
- **Budget increases.** The agent may ask; it may not grant.

## The network dimension

Each node runs its own harness over its own memory, but they are not
independent: a node forgetting locally depends on a peer retaining. That implies
a protocol — retention commitments between peers, and a way to discover that a
commitment has lapsed before relying on it. Trust tier decides which peers may
hold which privacy class.

This is the point where the RAG work, the storage fabric and the trust graph
stop being three projects. The harness is what makes a personal knowledge system
self-maintaining, and self-maintenance across a constrained network is the thing
KwaaiNet is actually for.

## Path forward, revised again

The order is unchanged — instrument, then ontology, then compression — but the
harness is not a fifth phase after them. It is the thing that runs them, and it
should be built incrementally alongside:

1. **Metrics first** (already the plan). The harness cannot sense without them.
2. **Harness v0: observe only.** Run the loop with acting disabled. It reports
   what it *would* do and why. This is cheap, it validates the decision policy
   against a human's judgement, and it would have caught tonight's "optimising
   the wrong objective" on day one.
3. **Harness v1: act on reversible operations** — consolidate, summarise,
   re-eval, propose. Destructive operations stay manual.
4. **Harness v2: eviction with demote-to-peer**, once retention commitments
   exist and the rate–distortion curve for a KB is known.

Observe-only first is the important one. An agent that can explain its intended
actions before it has permission to take them is both the safest starting point
and the fastest way to find out whether the decision policy is any good.
