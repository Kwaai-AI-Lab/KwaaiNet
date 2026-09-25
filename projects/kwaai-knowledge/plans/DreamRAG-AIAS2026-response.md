# Response to Reviewers: Submission 54

> **Draft.** Items in ⟦double brackets⟧ must be resolved before submission. Section, figure and
> table numbers refer to the **4-page revised paper**, `DreamRAG-AIAS2026-short.md`; check them against
> the final PDF. This file must stay free of identifying information. Sources for every number are in
> the evidence ledger of `DreamRAG-AIAS2026-manuscript-plan.md`.

---

**Title of revised submission:** Dream RAG: Offline Consolidation of a Retrieval Knowledge Graph, and When It Helps

We thank both reviewers for their careful reading. Both noted that the original submission was an
extended abstract that did not give enough detail to judge novelty, contribution or rigour. We agree.
The revision is a complete paper within the four-page limit. To fit that limit we prioritized the
evaluation and the algorithm over related work and per-corpus detail. An extended version, with the
full algorithm, parameters, prompts, related work and per-corpus results, will be released with the
source code at camera-ready. Before the point-by-point replies, we summarize three changes that affect
several comments.

**1. We narrowed the claims to the system we evaluate.** The original abstract described components
(an Ebbinghaus-style memory-strength model with long-term, short-term and dormant tiers; Bayesian
uncertainty estimation for abstention; query-likelihood reranking) and results (rank-AUC 0.66;
MRR 0.57 → 0.70; 94% of gold evidence retained after a 25% prune). Those components reflected the
architecture as planned when the abstract was written. They are not part of the system evaluated in
this paper, so we have removed them together with the results the abstract associated with them. Every
result in the revision is measured on the implemented system. We also no longer imply a formal
free-energy model. The principle's actual role is as a design heuristic for **fact density** (§1): the
system removes redundant and weakly supported entities (lower complexity) and completes the remaining
ones from their evidence (better fit to the source text). The dream loop is defined operationally (§2),
and its effect on density is measured (§4).

**2. We replaced the single small corpus with a multi-corpus evaluation.** The system is now evaluated
on eleven corpora from seven domain types, ranging from 773 to 15,018 passages (§3). A twelfth corpus
joins the consolidation study, fifteen corpora enter a cross-corpus correlation, and a single-author
memoir is used to study the dream loop cycle by cycle (§3–4).

**3. We report what dreaming does and does not improve, including negative results.** The dream loop
reliably increases the fact density of the knowledge graph. Over three to five cycles on twelve
corpora, it cut the number of entities by 7–26% and raised mean completeness on eleven of the twelve;
almost all of the reduction came from duplicate merging in the first cycle. Retrieval accuracy did not
follow, and two consolidation operations reduced it (§4). We think this is the most useful finding for
the community. It answers both reviewers' question of which component drives performance and whether
the complexity is warranted, and we have built the paper around it.

---

## Reviewer 1

> **R1.1** *The technical novelty is difficult to assess because many components (GraphRAG, Bayesian
> uncertainty estimation, graph completion, context optimization) already exist individually.*

We agree that most building blocks exist individually. Section 1 now states the contribution precisely:

- **(a)** an implemented offline consolidation loop for a RAG knowledge graph. It scores every entity
  for structural completeness, completes the weakest from its own evidence passages, merges duplicates
  and prunes unsupported nodes, and it converges: once no candidate remains, a cycle makes no
  language-model calls (§2);
- **(b)** a multi-corpus study showing that the loop improves graph structure but not retrieval
  accuracy, measured against an explicit noise floor (§3–4);
- **(c)** two documented ways in which consolidation reduces accuracy (§4, Table 1).

Bayesian uncertainty estimation and context-window optimization are no longer claimed (general change
1). Section 1 positions the work against GraphRAG, HippoRAG and LightRAG: those systems build their
graph at ingestion, whereas ours revises it offline and repeatedly.

> **R1.2** *It is unclear which component contributes most to the reported improvements.*

Table 1 collects the controlled comparisons available for our system, each with its number of runs, to
be read against the noise floor in §3:

- **Retrieval strategy matters most.** Multi-round iterative retrieval scored 57.8% keyword recall
  (mean of 3 runs) against 39.7% for single-pass retrieval (1 run).
- **Curated structure helps.** Adding seven curated organization and place nodes raised recall from
  54.4% (mean of 3; 95% CI 50.7–58.0%) to 63.1% (1 run).
- **Automatically extracted graph structure contributes little.** Turning off graph-context injection
  *raised* recall from 53.4% to 56.9%, and a corpus-specific ontology scored 55.5% against 59.3% for the
  generic one (1 run each; both within noise).
- **Dream cycles do not measurably change accuracy.** The five development milestones whose only change
  was running dream cycles moved recall by −1.7 points on average (−4.5 to +0.9).

Figure 2 adds the development history of the system: 88 milestones, with the changes to the
instrument marked. Code and configuration changes to retrieval and query handling drove most of the
rise to 74.7%; curated seed edits account for about 25 points; dream-only milestones for none. Across
fifteen corpora, neither graph completeness (ρ = 0.28) nor the number of relations (r = 0.08) predicted
recall (§4). Section 5 attributes this to the graphs carrying almost no relations: the corpora were
built without relation extraction, which an 8B model could not do precisely enough in our earlier
experiments.

> **R1.3** *There is little information about the actual algorithm, making reproducibility impossible.*

Section 2 now specifies each stage with its parameters:

- **chunking**: paragraph-based passages of at most 800 characters, with 200 characters of overlap;
- **entity extraction**: a candidate scan that sends only passages with candidates to the model, then
  typed extraction with at most 20 entities per passage and the adjacent passages as context;
- **retrieval**: BM25 and dense retrieval fused by reciprocal rank (k = 60), graph expansion within two
  hops, coverage-triggered second and third rounds (thresholds 0.70 and 0.75), top 20 passages;
- **the completeness score**: the mean of a type, a content and a relation component, each defined;
- **the dream loop**: selection (score below 0.6, weakest first, budget 100–200), evidence-only
  completion, the acceptance gate, the three merge tiers with their thresholds (Jaro–Winkler 0.60,
  cosine 0.92), the prune rule, the failure guard and convergence.

The prompts and the complete configuration did not fit the page limit. They will be in the extended
version released with the source code at camera-ready; the code is withheld now only to preserve
anonymity.

> **R1.4** *The evaluation appears to use only a relatively small document corpus (~430 chunks),
> raising questions about scalability.*

See general change 2. The eleven corpora of the main evaluation are, for the reviewers' reference:

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

They range from 773 to 15,018 passages (§3). Section 4 ("Cost") and our reply to R1.9 discuss
scalability.

> **R1.5** *There is no comparison against recent state-of-the-art dynamic or GraphRAG systems.*

We did not re-implement GraphRAG, HippoRAG or LightRAG, and we did not run a separate static-RAG
baseline across the corpora. Section 5 states both as limitations. We think this matters less for the
revised paper than for the original abstract, because the revision does not claim that the system
outperforms other systems. Its central claim is internal: offline consolidation improves the graph's
structure without improving retrieval accuracy. That claim is tested by comparing the same system
before and after consolidation, and by correlating structural completeness with accuracy within one
corpus and across fifteen (§4, Figure 1).

> **R1.6** *The reported experimental results are limited and lack statistical significance or broader
> benchmarking.*

Section 3 now quantifies the evaluation noise floor before any effect is interpreted:

- two evaluations of the *same* graph differed by 7.6 points (70.2% vs 77.8%);
- ten runs of one configuration had a standard deviation of 1.6 points.

Where we have repeated runs, Table 1 reports means, a confidence interval and a significance test (the
pruning comparison, p = 0.055, Welch's t = 2.50). The correlations between completeness and accuracy
are reported with their p-values (§4). We treat single-run differences below about 5 points as
inconclusive, and we note (§4) that at our sample sizes only a strong correlation would have been
detectable. Section 5 acknowledges that most comparisons have one to four runs and that
keyword-overlap scoring is a coarse instrument, which is why §4 also reports an LLM-judge score.

> **R1.7** *Claims regarding "dreaming" and biological inspiration appear largely conceptual rather than
> mathematically grounded.*

We agree that the original abstract overstated this. The revision does not claim a formal model of
memory consolidation or of the free-energy principle. Section 1 states what the analogy contributed to
the design. The free-energy principle trades model complexity against fit to evidence; we use that
trade-off as a heuristic, and the system optimizes a measurable counterpart, **fact density**: the same
source evidence represented by fewer, more complete entities. At ingestion, a per-passage entity cap
limits over-extraction (§2). The dream loop then applies three operations, each defined in §2:

- **duplicate merging** in three tiers: identical normalized names; the same type with Jaro–Winkler
  similarity ≥ 0.60 and embedding cosine ≥ 0.92; and names differing only by an honorific;
- **pruning** of entities with a single mention, no evidence passages, no relations and a completeness
  score below 0.3;
- **evidence-grounded completion** of the least complete entities, accepted only through the
  improvement gate.

Section 4 measures the result. The entity count fell by 7–26%, and mean completeness rose on eleven of
twelve corpora. 92–100% of each reduction occurred in the first cycle, when duplicates are merged;
later cycles raised completeness by about 0.1–0.5 points each. Section 4 also reports where density
optimization stops helping: pruning entities that lacked direct evidence but were connected removed
half the graph's relations and lowered recall. Density must therefore be pursued without cutting
connectivity.

> **R1.8** *What is the computational cost of the offline dreaming phase? How often should consolidation
> occur in practice?*

Section 4 ("Cost") reports that a dream cycle of 200 language-model completions took 3–21 minutes
(median 13) with no trend in graph size, and that a converged cycle takes seconds because no candidates
remain. On frequency (§4): almost all of the density gain arrives in the first cycle, later cycles add
0.1–0.5 completeness points each, and on the memoir completeness plateaued after about two hours of
cycles (Figure 1a). We therefore recommend running the loop after ingestion until it converges, and
again when new documents arrive. Because a converged cycle makes no model calls, re-running it costs
little.

> **R1.9** *Can the framework scale to millions of documents?*

We have not tested at that scale, and §5 says so; our largest corpus has 15,018 passages. What we can
report (§4, "Cost"):

- **Graph construction** ran at 0.17–0.83 passages per second on two commodity GPUs with an 8B model,
  and took 18 hours for the largest corpus. Throughput depends on entity density as well as size,
  because passages with no entity candidates skip the language model. It is the dominant cost.
- **Dream-cycle cost** is bounded by the per-cycle completion budget, not by corpus size, so
  consolidation can be spread over idle time on large corpora.

> **R1.10** *How are synthesized facts verified to avoid introducing hallucinations?*

Section 2 describes the safeguards, and §5 their limits:

1. Each completion is conditioned only on that entity's own evidence passages.
2. A completion replaces an existing description only if it passes an improvement gate: a higher
   description tier, or the same tier and more than 20 characters longer.
3. A completion may add a relation only to an entity that already exists in the graph, so it cannot
   invent entities.
4. A cycle whose model calls all failed stops before merging and pruning.

We do not independently fact-check generated descriptions, and hand-curated descriptions are protected
only by the gate in item 2. Section 2 states both, and §5 lists fact-checking as a limitation. The
curated-description failure in §4 was repaired by re-seeding the curated data, not prevented.

> **R1.11** *Does the dreaming process ever degrade retrieval quality after repeated consolidation?*

Yes. Section 4 and the last two rows of Table 1 report two cases:

- **Pruning connected, weakly evidenced entities.** A maintenance prune of 361 entities that had no
  direct textual evidence but were linked to evidenced ones deleted 2,972 of 6,164 relations. Recall
  fell from a mean of 56.7% (4 runs) to 52.6% (3 runs; p = 0.055). Pruning now leaves connected
  entities alone unless explicitly asked.
- **Overwriting curated descriptions.** In one cycle, generated summaries replaced curated
  descriptions and recall fell to 155/225 keywords, from 166–173 in the preceding cycles. Re-seeding
  the curated data restored 178/225.

We also show (Figure 1) that across 31 cycles on the memoir, graph completeness rose from 51.5% to
78.1% while accuracy stayed within its noise band (Spearman ρ = 0.17, p = 0.62). We therefore caution
against using structural graph metrics as a proxy for retrieval quality.

---

## Reviewer 2

> **R2.1** *A larger and more diverse evaluation corpus. ~430 chunks from a single domain is a very
> limited benchmark … does Dream RAG perform similarly across diverse domains, languages, document types,
> or larger corpora?*

See general change 2 and our reply to R1.4. The eleven corpora span seven domain types (§3):
historical narrative, legal opinions, conversational transcripts, technical documentation and
standards, scientific literature, academic papers, and literary fiction.

**Performance is not uniform** (§4). Retrieval recall ranged from 77.6% (meeting transcripts) to 94.3%,
answer recall from 68.5% to 86.1%, and judge scores from 1.20 to 1.85 out of 2. All corpora are in
English; §5 states this as a limitation, and we make no multilingual claims.

> **R2.2** *Explicit baseline comparisons. The reported metrics (MRR 0.57 → 0.70, Rank-AUC 0.66) are
> presented without comparison to standard RAG baselines.*

We have removed those metrics; general change 1 explains why. We did not run a like-for-like
static-RAG baseline (BM25 and dense retrieval fused by reciprocal rank, with no graph and no dream loop)
across the corpora, and §5 lists it as a limitation. Because the revision no longer claims an
improvement over standard RAG, its conclusions do not depend on that comparison. What we can report
are controlled comparisons within the system (Table 1). The largest is retrieval strategy:
multi-round iterative retrieval over the hybrid index scored 57.8% keyword recall (mean of 3 runs)
against 39.7% for single-pass retrieval (1 run). Section 5 also notes that we did not re-implement
external GraphRAG systems (see R1.5).

> **R2.3** *A clearer articulation of which architectural components are driving the observed gains …
> it is difficult to assess whether the complexity is warranted.*

We agree this was the central gap. Table 1 gives the component comparisons (summarized in our reply to
R1.2), §4 isolates the dream loop within one corpus and across fifteen, and Figure 2 shows which kinds
of change moved accuracy over the system's development. Our answer is direct:

- **Retrieval accuracy** is driven mainly by hybrid lexical-plus-dense retrieval with multi-round
  iterative retrieval.
- **Curated knowledge** gave clear gains, although on questions it was curated against, which Figure 2
  makes explicit.
- **Automatically extracted graph structure, and its offline consolidation,** did not improve accuracy
  measurably with an 8B extraction model, and some consolidation steps harmed it.

On current evidence, the added complexity is warranted for graph quality, inspectability and the
maintenance of curated knowledge, not for retrieval accuracy (§5). Section 5 identifies why: the
multi-corpus graphs were built without relation extraction and carry almost no relations, so the graph
adds little that the passages do not already say. Precise, typed relation extraction is the
prerequisite for consolidation to improve retrieval. Query decomposition and context-window
optimization, listed in the original abstract, are no longer presented as contributions, because we
have no isolated measurement of them.

---

## Summary of changes

| Change | Location |
|---|---|
| Extended abstract replaced by a complete four-page paper | whole paper |
| Planned-but-unimplemented components removed, with their associated results (memory-strength tiers, Bayesian abstention, query-likelihood reranking) | Abstract, §1 |
| Contribution restated and positioned against graph RAG systems | §1 |
| Algorithm and parameters specified | §2 |
| Eleven-corpus evaluation, a twelve-corpus consolidation study and a fifteen-corpus correlation | §3, §4 |
| Noise floor measured; comparisons reported with run counts | §3, Table 1 |
| Dream loop: structure vs accuracy | §4, Figure 1 |
| Failure modes of consolidation | §4, Table 1 |
| Development history: what moved accuracy | §4, Figure 2 |
| Cost, scheduling and scalability | §4 |
| Limitations: no static-RAG or external GraphRAG baselines, one before/after corpus, few runs, keyword metric with a same-model judge, no fact-checking, English only | §5 |
