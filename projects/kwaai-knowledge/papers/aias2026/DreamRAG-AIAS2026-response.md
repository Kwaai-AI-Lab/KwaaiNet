# Response to Reviewers: Submission 54

<!--
Internal note, not part of the letter: delete this comment before pasting or converting the letter.
Section, figure and table numbers refer to the 4-page revised paper, DreamRAG-AIAS2026-short.md;
check them against the final PDF. Sources: the evidence ledger of
../../plans/DreamRAG-AIAS2026-manuscript-plan.md (multi-corpus and development numbers) and
../../plans/D6-NoSeed-Dream-plan.md (the consolidation trial's pre-registration and results).
-->

---

**Title of revised submission:** Dream RAG: Offline Consolidation of Passage Memory into a Knowledge Graph

We thank both reviewers for their careful reading. Both noted that the original submission did not give
enough detail to judge novelty, contribution or rigour. We agree. The revision is an extended abstract
built around a stated hypothesis and a pre-registered experiment designed to test it. To fit four
pages, it gives the hypothesis, the setup and the results priority over related work and per-corpus
detail. An extended version will be released with the source code at camera-ready. It will contain the
full algorithm, parameters, prompts, related work, per-corpus results and the controlled comparisons
cited below. Before the point-by-point replies, we summarize four changes that affect several comments.

**1. We narrowed the claims to the system we evaluate.** The original abstract described components
(an Ebbinghaus-style memory-strength model with long-term, short-term and dormant tiers; Bayesian
uncertainty estimation for abstention; query-likelihood reranking) and results (rank-AUC 0.66;
MRR 0.57 → 0.70; 94% of gold evidence retained after a 25% prune). Those components reflected the
architecture as planned when the abstract was written. They are not part of the system evaluated here,
so we have removed them together with the results associated with them. Every result in the revision is
measured on the implemented system; the multi-corpus results (§4.1) predate the retrieval fixes
described in §2. We no longer imply a formal free-energy model; the principle serves
as a design heuristic for **fact density** (§1).

**2. We state the hypothesis and test it directly.** The biological analogy makes a specific claim. A
fast, capacity-limited store is consolidated offline into a slower structured one, after which the
fast store is no longer needed. In RAG terms, the passage index is short-term memory and the graph is
long-term memory. Section 1 separates two hypotheses:
- **H1 (transfer):** after consolidation, the graph *alone* supports more of the facts needed to
  answer.
- **H2 (hybrid benefit):** consolidation improves retrieval while the passage index remains in use.

Our earlier evaluations, and most graph-RAG evaluations, test only H2.

**3. We ran a pre-registered, controlled trial** (§3.2).
- **Corpus:** a memoir with no curated seed data.
- **Ablation:** two arms, descriptions-only against relation completion.
- **Baseline:** single-pass passage retrieval with no graph.
- **Measure:** retrieval scored as coverage of gold answer facts (nuggets), with paired bootstrap
  intervals.
- **Pass rule:** fixed in advance.

**Result:** H2 fails and H1 passes (§4.2–4.3, Table 1, Figure 1). With the passage index in use, 24
cycles moved coverage by +1.0 points (95% CI −2.0 to +5.3). From the graph alone, the same cycles raised
it from 14.0% to 32.3% (+18.3, CI +8.7 to +29.6).

**4. We kept the multi-corpus evaluation** (§3.1, §4.1). Twelve corpora of 773–15,018 passages from
the domains listed in §3.1 were consolidated, and fifteen enter a cross-corpus correlation. Consolidation
improved graph structure on eleven of the twelve, while answer accuracy under hybrid retrieval did not
follow. That is the H2 result, observed at scale.

---

## Reviewer 1

> **R1.1** *The technical novelty is difficult to assess because many components (GraphRAG, Bayesian
> uncertainty estimation, graph completion, context optimization) already exist individually.*

We agree that most building blocks exist individually. The contribution is now stated as a hypothesis
and its test (§1):

- **(a)** an offline consolidation loop for a RAG knowledge graph, which completes weak entities from
  their own evidence, merges duplicates and prunes unsupported nodes (§2);
- **(b)** a separation of two claims that graph-RAG evaluations usually conflate: transfer into the
  graph (H1) and benefit to hybrid retrieval (H2);
- **(c)** a pre-registered trial showing that consolidation transfers knowledge into the graph while
  giving the hybrid system almost nothing, and why (§4.2–4.3, §5).

Bayesian uncertainty estimation and context-window optimization are no longer claimed (general change
1). GraphRAG, HippoRAG and LightRAG build their graph at ingestion; ours revises it offline and
repeatedly (§1).

> **R1.2** *It is unclear which component contributes most to the reported improvements.*

The trial isolates the two components of the dream loop, each against the same cycle-0 graph
(§3.2, Table 1):

- **Consolidation itself** drives the transfer. From the graph alone, coverage rose from 14.0% to
  31.0% in arm A (+17.0, CI +7.7 to +28.1) and 32.3% in arm B (+18.3, CI +8.7 to +29.6).
- **Relation completion** made no detectable difference by cycle 24 (B − A = +1.3, CI −7.8 to +8.9).
  Arm B was ahead after one cycle (33.2% against 25.7%, one run each), but that is untested, and the
  trial cannot separate relations from better descriptions.
- **Consolidation adds nothing detectable while the passage index is available.** Hybrid coverage
  stayed at 56.4–58.5% in every arm and cycle. Single-pass passage retrieval with no graph reached
  52.8%; that gap also includes the hybrid mode's extra retrieval rounds.

The extended version adds the controlled comparisons from the system's development. For example,
multi-round iterative retrieval scored 57.8% keyword recall against 39.7% for single-pass, and turning
off graph-context injection moved recall within noise. Across 88 development milestones, retrieval and
curated knowledge moved accuracy, and dream-only milestones did not.

> **R1.3** *There is little information about the actual algorithm, making reproducibility impossible.*

Section 2 defines ingestion, the completeness score, the dream loop and both retrieval modes. The
parameters that did not fit are in the extended version, with the prompts and configuration:
- passages of at most 800 characters with 200 of overlap, and at most 20 entities per passage (25
  when three or fewer entity types are declared);
- reciprocal-rank fusion with k = 60, and coverage thresholds of 0.70 and 0.75 for the second and
  third retrieval rounds;
- a selection threshold of 0.6 and a budget of 100–200 completions per cycle;
- the acceptance gate, the three merge tiers (Jaro–Winkler 0.60, cosine 0.92) and the prune rule.

The trial's setup (§3.2) is fully specified: the slice, the arms, the cycle budget, the gold
construction, the NLI model and threshold, the bootstrap, and the pre-registered rule. The code is
withheld now only to preserve anonymity.

> **R1.4** *The evaluation appears to use only a relatively small document corpus (~430 chunks),
> raising questions about scalability.*

See general change 4. The twelve corpora of the consolidation study are, for the reviewers'
reference:
- Manhattan Project history
- legal opinions
- meeting transcripts
- Python documentation
- AI security standards
- climate science
- internet standards (RFCs)
- deep-sea biology
- academic papers on sleep and memory
- astrophysics
- *Moby-Dick* and companion works
- a novel

They range from 773 to 15,018 passages (§3.1). The controlled trial deliberately uses a small slice
(116 passages of a 1,152-passage memoir), so that both arms could run 24 cycles with evaluation at nine
checkpoints. Section 5 names scaling the trial to the whole memoir and to the other corpora as the
first next step.

> **R1.5** *There is no comparison against recent state-of-the-art dynamic or GraphRAG systems.*

We did not re-implement GraphRAG, HippoRAG or LightRAG, and §5 lists this among the next steps. The
revision's claims are internal: the graph alone improves with consolidation (H1), and the hybrid
system does not (H2). Both are tested within one system, against its own cycle-0 graph and a
vector-only baseline (Table 1). Comparing external systems is part of the next step on the evaluation
instrument in §5.

> **R1.6** *The reported experimental results are limited and lack statistical significance or broader
> benchmarking.*

The revision states each test in advance and reports intervals (§3.2):
- **Pre-registration.** Arm B passes if its change in coverage has a 95% interval above zero and
  larger than the cycle-0 retest spread. This was registered for H2 before the trial, and for H1 before
  any graph-only evaluation.
- **Paired intervals.** Every change is paired per question against cycle 0, with a bootstrap
  interval (5,000 resamples).
- **Retest.** After the fixes in §2, repeated hybrid runs on the same graph give identical prompts
  (retest spread 0). Graph-only runs return the same cards, with field lines that can reorder; their
  coverage retest spread was 0.65 points.
- **Retrieval, not wording.** Coverage scores whether the retrieved context entails each gold fact,
  independently of how the answer is phrased.

The multi-corpus results (§4.1) come with their correlations (ρ = 0.17 over 31 cycles on one corpus;
ρ = 0.28 across fifteen). Section 5 notes the remaining limits: one trial corpus, a single run per
intermediate checkpoint, and one 8B model throughout.

> **R1.7** *Claims regarding "dreaming" and biological inspiration appear largely conceptual rather than
> mathematically grounded.*

We agree that the original abstract overstated this. The revision uses the analogy only for what it
predicts, and tests that prediction.
- **Consolidation.** Complementary learning systems [McClelland et al. 1995] predict that offline
  consolidation moves what a fast store holds into a structured one. H1 tests this, and it passes
  (§4.3).
- **The free-energy principle** is used only as a design heuristic, not as a model: *fact density*,
  the same evidence held by fewer, more complete entities. Section 4.1 measures it: entity counts fell
  by 7–26% and completeness rose on eleven of twelve corpora.

The analogy also names what is still missing. The fast store is capacity-limited and is released after
consolidation. Our passage index is neither, and §5 proposes making it so ("Forgetting") once graph
coverage approaches that of the passage index.

> **R1.8** *What is the computational cost of the offline dreaming phase? How often should consolidation
> occur in practice?*

A dream cycle is bounded by its completion budget, not by corpus size. On the full corpora, a cycle of
200 completions took 3–21 minutes (median 13); a converged cycle takes seconds. In the trial, most of
the transfer arrived in the first cycle (§4.3), and graph completeness plateaued within about a dozen
cycles. We therefore recommend running the loop after ingestion until it converges, and again when new
documents arrive. Graph-construction and cycle costs per corpus are in the extended version.

> **R1.9** *Can the framework scale to millions of documents?*

We have not tested at that scale; our largest corpus has 15,018 passages, and §5 lists scale as the
first next step. Graph
construction ran at 0.17–0.83 passages per second on two commodity GPUs, and took 18 hours for the
largest corpus. It is the dominant cost. Dream-cycle cost is bounded by the per-cycle budget, so
consolidation can be spread over idle time.

> **R1.10** *How are synthesized facts verified to avoid introducing hallucinations?*

Section 2 describes the first three safeguards; the fourth is in the extended version:
1. Each completion sees only the entity's own evidence passages.
2. Relations may target only entities that already exist.
3. Prose descriptions are kept, with the generated field summary appended.
4. A cycle whose model calls all failed stops before merging.

Generated content is not independently fact-checked; the extended version discusses this. The
trial's coverage measure checks, by entailment, whether the consolidated graph states gold facts. It
does not check what else the graph states.

> **R1.11** *Does the dreaming process ever degrade retrieval quality after repeated consolidation?*

In the trial, no: hybrid coverage stayed within +1.0 to +2.1 points of cycle 0 through 24 cycles in
both arms (Figure 1a). During development we did observe two degradations, both since fixed.
- Pruning connected but weakly evidenced entities deleted about half of a graph's relations and
  lowered recall (56.7% to 52.6%, p = 0.055).
- In one cycle, generated summaries overwrote curated descriptions. The loop now keeps prose and
  appends the field summary (§2).

The extended version documents both.

---

## Reviewer 2

> **R2.1** *A larger and more diverse evaluation corpus. ~430 chunks from a single domain is a very
> limited benchmark … does Dream RAG perform similarly across diverse domains, languages, document types,
> or larger corpora?*

See general change 4 and our reply to R1.4. The twelve corpora span historical narrative, legal
opinions, meeting transcripts, technical documentation and standards, scientific papers and fiction
(§3.1). Consolidation behaved
consistently across them: entity counts fell and completeness rose on eleven of twelve (§4.1). All
corpora are in English; §5 states this, and we make no multilingual claims. The controlled trial uses
one corpus, and extending it across the corpora is the first next step (§5).

> **R2.2** *Explicit baseline comparisons. The reported metrics (MRR 0.57 → 0.70, Rank-AUC 0.66) are
> presented without comparison to standard RAG baselines.*

We have removed those metrics (general change 1). The trial now includes the requested baselines
(Table 1):
- **Static RAG:** single-pass BM25 and dense retrieval with no graph, at 52.8% coverage. It is not
  exactly like-for-like, because the hybrid mode also runs extra retrieval rounds.
- **Hybrid retrieval before consolidation:** the cycle-0 graph, at 56.4%.
- **The graph alone before consolidation:** at 14.0%.

Each consolidated condition is compared, paired per question, with its own cycle-0 baseline. External
GraphRAG systems are not compared (see R1.5).

> **R2.3** *A clearer articulation of which architectural components are driving the observed gains …
> it is difficult to assess whether the complexity is warranted.*

We agree this was the central gap. Our answer, from the trial (§4, Table 1) and the multi-corpus
study (§4.1):

- **Consolidation works as a transfer mechanism in this trial.** It more than doubled what the graph
  alone can support. Relation completion made no detectable difference by the end.
- **It does not yet pay off for hybrid RAG.** The passage index already supplies most of what the
  graph learns: with the consolidated graph, hybrid coverage was no better than with the cycle-0
  graph (+1.0 points, CI −2.0 to +5.3), so the added complexity buys no accuracy while the index
  remains.
- **The complexity is warranted where the passage index cannot be kept.** That is the case when it
  must be pruned or forgotten, or where the graph itself is the product. The graph alone currently
  reaches about half of the passage index's coverage (32% against 56%), so §5 makes closing that gap,
  and then evicting passages, the next steps.

---

## Summary of changes

| Change | Location |
|---|---|
| Hypothesis stated: transfer (H1) vs hybrid benefit (H2) | §1 |
| Planned-but-unimplemented components removed, with their associated results | Abstract, §1 |
| Dream loop, completeness and both retrieval modes defined; determinism fixes noted | §2 |
| Twelve-corpus consolidation study and fifteen-corpus correlation | §3.1, §4.1 |
| Pre-registered controlled trial: two arms, vector-only baseline, paired bootstrap CIs | §3.2, §4.2–4.3, Table 1, Figure 1 |
| Graph-only retrieval, which tests transfer directly | §2, §4.3 |
| Next steps: scale, entity resolution, answer use, forgetting, instrument and baselines | §5 |
| Algorithm parameters, prompts, per-corpus results, development history, cost detail | extended version (camera-ready) |
