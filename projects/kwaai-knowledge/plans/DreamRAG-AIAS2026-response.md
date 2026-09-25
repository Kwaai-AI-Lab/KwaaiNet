# Response to Reviewers: Submission 54

> **Draft.** Items in ⟦double brackets⟧ must be resolved before submission. Section, figure and
> table numbers refer to the revised manuscript as outlined in `DreamRAG-AIAS2026-manuscript-plan.md`;
> check them against the final PDF. This file must stay free of identifying information.
> Sources for every number are in that plan's evidence ledger.

---

**Title of revised submission:** Dream RAG: Offline Consolidation of a Retrieval Knowledge Graph, and When It Helps

We thank both reviewers for their careful reading. Both noted that the original submission was an
extended abstract that did not give enough detail to judge novelty, contribution or rigour. We agree,
and the revision is a full paper. Before the point-by-point replies, we summarize three changes that
affect several comments.

**1. We narrowed the claims to the system we evaluate.** The original abstract described components
(an Ebbinghaus-style memory-strength model with long-term, short-term and dormant tiers; Bayesian
uncertainty estimation for abstention; query-likelihood reranking) and results (rank-AUC 0.66;
MRR 0.57 → 0.70; 94% of gold evidence retained after a 25% prune). Those components reflected the
architecture as planned when the abstract was written. They are not part of the system evaluated in
this paper. We have therefore removed them, together with the results the abstract associated with them.
Every result in the revision is measured on the implemented system. Section 6 lists the memory-strength
model and uncertainty-based abstention as future work. We also no longer imply a formal free-energy
model. The revision states the principle's actual role: it is a design heuristic for **fact density**.
The system removes redundant and weakly supported entities (lower complexity) and completes the
remaining ones from their evidence (better fit to the source text). The dream loop is defined
operationally (§3.4, Algorithm 1). Its effect on density is measured (§5.3).

**2. We replaced the single small corpus with a multi-corpus evaluation.** The system is now evaluated on
eleven corpora from seven domain types, ranging from 773 to 15,018 passages, and Table 1 now reports
their sizes. A secondary evaluation covers fifteen
corpora, and a single-author memoir is used to study the dream loop over time (§4.1, Table 1).

**3. We report what dreaming does and does not improve, including negative results.** The dream loop
reliably increases the fact density of the knowledge graph. Over five cycles on twelve corpora (three on
one, whose run failed on a since-fixed defect), it cut the number of entities by 7–26% (median 15%), and
mean entity completeness rose on eleven of the twelve. Almost all of the reduction came from duplicate
merging in the first cycle. In
our experiments, retrieval
accuracy did not follow. Some consolidation operations reduced it (§5.3–5.4). We think this is the most
useful finding for the community. It directly answers both reviewers' question of which component drives
performance and whether the complexity is warranted, and we have restructured the paper around it.

---

## Reviewer 1

> **R1.1** *The technical novelty is difficult to assess because many components (GraphRAG, Bayesian
> uncertainty estimation, graph completion, context optimization) already exist individually.*

We agree that most building blocks exist individually. Section 1 now states the contribution precisely:

- **(a)** an implemented offline consolidation loop for a RAG knowledge graph. It scores every entity for
  structural completeness, completes the weakest from its own evidence passages, merges duplicates and
  prunes unsupported nodes. It converges: once no candidate passes the acceptance gate, a cycle makes no
  language-model calls (§3.4, Algorithm 1).
  It optimizes the graph for fact density: fewer,
  more complete entities (§3.4–3.5, §5.3).
- **(b)** a multi-corpus study of the loop's effect on graph structure and on retrieval accuracy
  (§5.1–5.3).
- **(c)** documented failure modes of consolidation, and the safeguards they led to (§5.4, §3.5).

Bayesian uncertainty estimation and context-window optimization are no longer claimed (see general
change 1). Section 2 now positions the work against GraphRAG, HippoRAG, LightRAG, RAPTOR, Self-RAG and
MemGPT-style memory management. The distinguishing feature is that ours revises the index offline and
repeatedly, instead of building it once or revising it at query time.

> **R1.2** *It is unclear which component contributes most to the reported improvements.*

Section 5.2 and Table 2 add the controlled comparisons available for our system. Each is reported with
its number of runs and against the measured noise floor (§4.3):

- **Retrieval strategy matters most.** On the memoir corpus, multi-round iterative retrieval scored
  57.8% keyword recall (mean of 3 runs) against 39.7% for single-pass automatic routing (1 run).
- **The effect of the graph itself is small and sometimes negative.** Turning off graph-entity context
  injection *raised* recall from 53.4% to 56.9% (1 run each), which traced to a small number of malformed
  merged entities.
- **Curated structure helps.** Adding seven curated organisation and place nodes raised recall from 54.4%
  (mean of 3; 95% CI 50.7–58.0%) to 63.1% (1 run).
- **Post-processing order matters.** Three orderings of the same steps scored 61.8%, 54.2% and 56.9%.
- **Dream cycles do not measurably change accuracy** (§5.3, R1.9).

Across fifteen corpora, the number of extracted relations did not predict recall (Pearson r = 0.08).
We attribute this in §6 to the graphs carrying almost no relations: the corpora were built without
relation extraction, which an 8B model could not do precisely enough in our earlier experiments.

> **R1.3** *There is little information about the actual algorithm, making reproducibility impossible.*

Section 3 now specifies each stage:

- chunking (paragraph-based passages of at most 800 characters, with 200 characters of overlap);
- entity extraction (NER pre-screening, then LLM extraction constrained to a declared type vocabulary,
  with a per-passage entity cap and one adjacent passage as context);
- hybrid retrieval (BM25 and dense retrieval fused by reciprocal rank, top-k = 20);
- the dream loop (Algorithm 1). It covers the completeness score, candidate selection, the budget of
  completions per cycle, the improvement gate a completion must pass to be accepted, and the scheduling
  of cycles (each run performs one cycle, repeated by an external scheduler; a converged cycle does no work).

The appendix gives all parameters and the prompts. ⟦confirm the appendix fits the page limit; otherwise
move it to supplementary material⟧ We will release the source code with the camera-ready version. It is
withheld now only to preserve anonymity.

> **R1.4** *The evaluation appears to use only a relatively small document corpus (~430 chunks),
> raising questions about scalability.*

See general change 2. The main evaluation (§5.1, Figure 2) covers eleven corpora:

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

They range from 773 to 15,018 passages. Table 1 also reports graph-construction time for each corpus.
§5.5 and our reply to R1.9 discuss scalability.

> **R1.5** *There is no comparison against recent state-of-the-art dynamic or GraphRAG systems.*

We did not re-implement GraphRAG, HippoRAG or LightRAG, and we did not run a separate static-RAG
baseline across the corpora. Section 6 states both as limitations. We think this matters less for the
revised paper than for the original abstract, because the revision does not claim that the system
outperforms other systems. Its central claim is internal: offline consolidation improves the graph's
structure without improving retrieval accuracy. That claim is tested by comparing the same system
before and after consolidation, and by correlating structural completeness with accuracy within one
corpus and across fifteen (§5.3). Section 2 compares the designs qualitatively.

> **R1.6** *The reported experimental results are limited and lack statistical significance or broader
> benchmarking.*

Section 4.3 now quantifies the evaluation noise floor before any effect is interpreted:

- two evaluations of the *same* graph differed by 7.6 points (70.2% vs 77.8%);
- ten runs of one pipeline configuration had a standard deviation of 1.6 points;
- where we have repeated runs, we report means, standard deviations and confidence intervals, plus
  Welch's t-test for the pruning comparison (t = 2.50, p = 0.055);
- the correlation between graph completeness and accuracy across dream cycles is reported with its
  p-value (§5.3).

We treat single-run differences smaller than the noise floor as inconclusive and say so in the text.
We also acknowledge (§6) that most comparisons have one to four runs, and that keyword-overlap scoring is
a coarse instrument. For that reason §5.1 also reports an LLM-judge score.

> **R1.7** *Claims regarding "dreaming" and biological inspiration appear largely conceptual rather than
> mathematically grounded.*

We agree that the original abstract overstated this. The revision does not claim a formal model of
memory consolidation or of the free-energy principle. Instead, §1 and §3.4 state what the analogy
contributed to the design. The free-energy principle trades model complexity against fit to evidence. We
use that trade-off as a heuristic, and the system optimizes a measurable counterpart, **fact density**:
the same source evidence represented by fewer, more complete entities. At ingestion, a per-passage
entity cap limits over-extraction, and passages with no named-entity candidates are not sent to the
model (§3.1). The dream loop then applies three operations:

- **Multi-tier duplicate merging** (§3.4): exact normalized names; a string-similarity gate followed by
  embedding similarity, with a type-match guard; and names that differ only by an honorific.
- **Pruning** of entities with a single mention, no evidence passages, no relations and a low
  completeness score (§3.4).
- **Evidence-grounded completion** of the least complete entities (§3.4).

Section 3.4 defines each formally: the completeness score as a function of an entity's type, description
and relations; the selection rule; the acceptance gate; and convergence, when no candidate remains
and a cycle makes no language-model calls.

Section 5.3 measures the result. Over five cycles on twelve corpora (three on one), the entity count fell
by 7–26% (median 15%), and mean completeness rose on eleven of them (Figure 3). Between 92% and 100% of
each corpus's reduction occurred in the first cycle, when duplicates are merged; later cycles raised
completeness by about 0.1–0.5 points each. Section 5.4 reports where density
optimization stops helping. Pruning entities that lacked direct evidence but were well connected removed
half the graph's relations and lowered retrieval recall. Density must therefore be pursued without
cutting connectivity.

> **R1.8** *What is the computational cost of the offline dreaming phase? How often should consolidation
> occur in practice?*

Section 5.5 and Figure 6 report cost:

- On the memoir corpus, a cycle of 100 language-model completions with four parallel workers took about
  5–6 minutes. All 31 cycles took about two hours with a single 8B model.
- On eleven other corpora, a cycle of 200 completions took 3–21 minutes (median 13). The time did not
  grow with graph size: the largest graph, about 12,000 entities, had a median of 8 minutes. The spread
  tracked load on the shared inference servers.
- Once the loop converges, a cycle costs seconds, because no candidates remain.

On frequency (§5.3, Figure 3):

- Completeness on the memoir corpus plateaued at about 78% from cycle 24 onwards.
- On the other corpora, gains fell to about 0.1 points per cycle after five cycles.

We therefore recommend running the loop after ingestion until it converges, and again only when new
documents arrive. A converged cycle makes no language-model calls and finishes in seconds, so re-running it
costs little.

> **R1.9** *Can the framework scale to millions of documents?*

We have not tested at that scale, and §6 says so. What we can report (§5.5):

- **Graph construction** ran at 0.17–0.83 passages per second on two commodity GPUs with an 8B model.
  Throughput depends on entity density as well as size, because passages with no named-entity
  candidates skip the language-model call. The largest corpus (15,018 passages) took 18 hours. It is
  the dominant cost, and it parallelizes across workers.
- **Dream-cycle cost** is bounded by the per-cycle completion budget, not by corpus size. Consolidation
  can therefore be spread over idle time on large corpora.

> **R1.10** *How are synthesized facts verified to avoid introducing hallucinations?*

Section 3.5 describes the safeguards, and §6 their limits:

1. Each completion is conditioned only on that entity's own evidence passages.
2. A completion replaces an existing description only if it passes an improvement gate on completeness
   tier and length.
3. A completion may add a relation only to an entity that already exists in the graph, so it cannot
   invent new entities.

Hand-curated descriptions have no further protection inside the loop. A merge keeps the longer
description, and a completion replaces one that it outranks. The failure in §5.4 was repaired by
re-seeding the graph, since a seeded description takes precedence when it is inserted; §6 lists the
missing in-loop protection as a limitation. We do not independently
fact-check generated descriptions, and §6 lists this as a limitation. A manual audit of the memoir graph
did find extraction errors (for example, a gender swap and conflated person names). They came from the
extraction stage, and the dream loop can propagate them. ⟦decide whether to include the audit counts⟧

> **R1.11** *Does the dreaming process ever degrade retrieval quality after repeated consolidation?*

Yes. Section 5.4 is new and reports two cases:

- **Pruning connected, weakly evidenced entities.** A separate graph-maintenance prune, run outside the
  dream loop, removed 361 entities that had no direct textual
  evidence but were linked to evidenced ones, deleting 2,972 of 6,164 relations. Keyword recall fell from a
  mean of 56.7% (4 runs) to 52.6% (3 runs; Welch t = 2.50, p = 0.055). By default, the pruning operation now
  removes only unevidenced entities that have no relations. Removing connected ones requires an
  explicit opt-in.
- **Overwriting curated descriptions.** In one cycle, generated summaries replaced hand-curated
  descriptions. Recall dropped to 155/225 keywords, compared with 166–173 in the preceding cycles. After
  re-seeding restored the curated descriptions, recall recovered to 178/225.

We also show (§5.3, Figure 5) that across 31 cycles, graph completeness rose from 51.5% to 78.1% while
accuracy stayed within its noise band. Completeness and accuracy were uncorrelated (Spearman ρ = 0.17,
p = 0.62). We therefore caution against using structural graph metrics as a proxy for retrieval quality.

---

## Reviewer 2

> **R2.1** *A larger and more diverse evaluation corpus. ~430 chunks from a single domain is a very
> limited benchmark … does Dream RAG perform similarly across diverse domains, languages, document types,
> or larger corpora?*

See general change 2 and our reply to R1.4. Section 5.1 and Figure 2 report retrieval recall, answer
recall and an LLM-judge score for eleven corpora across seven domain types:

- historical narrative
- legal
- conversational transcripts
- technical documentation and standards
- scientific literature
- academic papers
- literary fiction

**Performance is not uniform.** Retrieval recall ranged from 77.6% (meeting transcripts) to 94.3% (AI
security standards), and judge scores from 1.20 to 1.85 out of 2. Section 5.1 discusses why
conversational transcripts are hardest. All corpora are in English. We state this as a limitation in §6
and do not claim multilingual results.

> **R2.2** *Explicit baseline comparisons. The reported metrics (MRR 0.57 → 0.70, Rank-AUC 0.66) are
> presented without comparison to standard RAG baselines.*

We have removed those metrics; general change 1 explains why. We did not run a like-for-like
static-RAG baseline (BM25 and dense retrieval fused by reciprocal rank, with no graph and no dream loop)
across the corpora, and §6 lists it as a limitation and the first item of future work. Because the
revision no longer claims an improvement over standard RAG, the paper does not depend on that
comparison. What we can report are controlled comparisons within the system (§5.2, Table 2). The largest
is retrieval strategy: on the memoir corpus, multi-round iterative retrieval over the hybrid index
scored 57.8% keyword recall (mean of 3 runs) against 39.7% for single-pass automatic routing (1 run).
Section 6 also notes that we did not re-implement external GraphRAG systems (see R1.5).

> **R2.3** *A clearer articulation of which architectural components are driving the observed gains …
> it is difficult to assess whether the complexity is warranted.*

We agree this was the central gap. Section 5.2 and Table 2 give the component comparisons (summarized in
our reply to R1.2). Section 5.3 isolates the dream loop, and adds a second test across fifteen corpora: graph completeness
did not predict recall (Spearman ρ = 0.28, p ≈ 0.3). Our answer is direct:

- **Retrieval accuracy** is driven mainly by hybrid lexical-plus-dense retrieval and by multi-round
  iterative retrieval.
- **Curated structure** gave a clear gain.
- **Automatically extracted graph structure, and its offline consolidation,** did not improve accuracy
  measurably with an 8B extraction model, and some consolidation steps harmed it.

On current evidence, the added complexity is warranted for graph quality, inspectability and the
maintenance of curated knowledge, not for retrieval accuracy. Section 6 identifies why: the multi-corpus graphs were built without relation extraction and carry
almost no relations, so the graph adds little that the passages do not already say. Precise, typed relation extraction is the stated prerequisite for consolidation to
improve retrieval. Query decomposition and context-window optimization, listed in the original abstract,
are no longer presented as contributions, because we have no isolated measurement of them.

---

## Summary of changes

| Change | Location |
|---|---|
| Extended abstract replaced by a full paper | whole manuscript |
| Planned-but-unimplemented components removed, with their associated results (memory-strength tiers, Bayesian abstention, query-likelihood reranking) | Abstract, §1, §6 |
| Contribution restated; related work added | §1, §2 |
| Algorithm, parameters and prompts specified | §3, Algorithm 1, Appendix |
| Eleven-corpus evaluation (fifteen for the secondary analysis) | §4.1, Table 1, §5.1, Figure 2 |
| Component comparisons with run counts and noise floor | §4.3, §5.2, Table 2 |
| Dream loop: structure vs accuracy | §5.3, Figures 3 and 5 |
| Failure modes of consolidation | §5.4, Table 3 |
| Cost, scheduling and scalability | §5.5, Figure 6 |
| Limitations: no static-RAG or external GraphRAG baselines, English only, single runs, keyword metric, no fact-checking | §6 |
