---
title: "Dream RAG: Offline Consolidation of Passage Memory into a Knowledge Graph"
---

<!--
4-page extended abstract (anonymized). Structure (2026-09-29): hypothesis, experimental setup,
preliminary results (hybrid and graph-only retrieval), next steps. The extended version is
DreamRAG-AIAS2026-manuscript.md.
Sources: multi-corpus and memoir-cycle numbers trace to the evidence ledger in
../../plans/DreamRAG-AIAS2026-manuscript-plan.md; the consolidation trial (§3.2, §4.2, §4.3) to
tests/kwaai-knowledge/results/d6_noseed_s10/report.json and report_graph-only.json, with its
pre-registration in ../../plans/D6-NoSeed-Dream-plan.md.
Build: python3 projects/kwaai-knowledge/papers/aias2026/build_manuscript_docx.py --short
Items in ⟦double brackets⟧ must be resolved before submission.
-->

## Abstract

Complementary-learning-systems theory holds that a fast, capacity-limited store is consolidated
offline into a slower structured one, after which the fast store can be released. We test the
analogous claim for retrieval-augmented generation (RAG): that an offline "dream" loop transfers what
a passage index holds into an entity graph. Dream RAG completes weak entities from their own evidence
with an 8B model, merges duplicates and prunes unsupported nodes. On twelve corpora, three to five
cycles reduced the entity count by 7–26% and raised graph completeness on eleven. However, with the passage index still
in use, answer accuracy did not follow. In a pre-registered trial on a memoir, with no curated seed data, 24 cycles
left hybrid (passage + graph) retrieval unchanged: coverage of gold answer facts moved +0.010 (95% CI
−0.020 to +0.053). With the passages withheld, so that only the graph can answer, the same cycles
more than doubled coverage, from 14.0% to 32.3% (+0.183, CI +0.087 to +0.296), with most of the
gain in the first cycle. In this trial, consolidation transferred knowledge into the graph, but while the
passage index remained, hybrid retrieval did not show the gain; the graph alone reached 57% of hybrid
coverage.

**Keywords:** retrieval-augmented generation, GraphRAG, memory consolidation, knowledge graphs,
evaluation

## 1 Hypothesis

Graph-based RAG adds an entity graph to a passage index so that retrieval can follow relationships
[Edge et al. 2024; Gutiérrez et al. 2024; Guo et al. 2025]. The graph is built once, at ingestion.
Biological memory works differently. In complementary learning systems [McClelland et al. 1995], a
fast episodic store (the hippocampus) is replayed offline into a slow structured one (the neocortex).
Once consolidation is complete, the fast store is no longer needed to recall what was consolidated.
The free-energy principle [Friston 2010] suggests what consolidation should optimize: explain the
same evidence with less model complexity. We use this as a design heuristic, *fact density* (the same
evidence held by fewer, more complete entities), not as a formal model.

In RAG terms, the passage index is short-term memory and the graph is long-term memory. We test two
hypotheses:

- **H1 (transfer).** After consolidation, the graph *alone* supports more of the facts needed to
  answer questions about the corpus.
- **H2 (hybrid benefit).** Consolidation improves retrieval when the passage index remains available
  alongside the graph.

Most graph-RAG evaluations, including our own earlier ones, test only H2. If the passage index
already answers most questions, H2 can fail even when H1 holds.

## 2 System

**Ingestion.** Documents are split into passages of at most 800 characters, embedded and indexed for
BM25. llama3.1:8b extracts typed entities from each passage, with its neighbours as context. Relation
extraction is off at ingestion, because an 8B model extracted relations too imprecisely in earlier
experiments.

**The dream loop.** Each cycle scores every entity for completeness, which averages three parts: its
type (untyped, generic or specific), how many of its type's expected fields and how much description
it has, and how many of its type's expected relations are present. The cycle then asks the model,
given only an entity's own evidence passages, for its missing fields and description, and optionally
for relations to entities that already exist; keeps prose descriptions and refreshes a one-line field
summary; merges duplicates by name, string similarity and embedding; and prunes isolated, unevidenced
entities.

**Retrieval.** *Hybrid* retrieval fuses BM25 and dense rankings by reciprocal rank [Cormack et al.
2009] with the passages of graph entities within two hops of the query's entities, and adds up to
two further rounds when query terms are uncovered. *Graph-only* retrieval returns up to 20 entity
cards (name, aliases, every relation, fields and description) and no passage text. The cards come
from the entities matching the query, then their neighbours, then the nearest entities by embedding.
Before the trial we fixed three defects that made hybrid retrieval non-deterministic or let graph
traversal flood the prompt. With the fixes, repeated hybrid runs on the same graph give identical
prompts; graph-only runs return the same cards, though field lines within a card may reorder.

## 3 Experimental setup

### 3.1 Consolidation across corpora

Twelve English corpora of 773–15,018 passages (historical narrative, legal opinions, meeting
transcripts, technical documentation, scientific papers, fiction) were each consolidated for three to
five cycles. Eleven of them (48,214 passages in all) have 20 questions each with gold keywords; answer
recall is the fraction of gold keywords that appear in the generated answer.

### 3.2 A pre-registered consolidation trial

**Corpus.** A 1,152-passage single-author memoir. Its graph is built from the first 10% (116
passages, in reading order) with *no curated seed data*. Our earlier memoir graphs were seeded with a
hand-written family tree and descriptions that state some answers outright. Search in the hybrid
mode still covers all 1,152 passages.

**Arms.** From the same cycle-0 graph, arm A dreams on descriptions and types only, and arm B also
completes relations. Each arm runs 24 cycles of up to 200 model calls. All inference runs locally on
llama3.1:8b.

**Gold.** We use the 31 questions whose authored answers have at least one fact supported in the
slice. These give 108 *nuggets* (atomic facts). 53 nuggets were verified by NLI entailment of at
least 0.9 against a passage. The other 55 were adjudicated by the local model and confirmed by the author.

**Measures.** *Coverage* (primary) is the fraction of a question's nuggets entailed (NLI ≥ 0.5,
DeBERTa-v3-large) by at least one retrieved passage or card; it scores retrieval, independently of the
generated answer. *Answer recall* applies the same test to the answer. Changes are paired per question against cycle 0, with 95% bootstrap intervals (5,000 resamples).
Evaluations run at cycles 0, 1, 2, 4, 8, 12, 16, 20 and 24 for hybrid retrieval, and at 0, 1, 4, 12
and 24 for graph-only retrieval, with repeats at cycles 0 and 24.

**Pre-registration.** A test counts as passed if arm B's change in coverage from cycle 0 to cycle 24
has a 95% interval above zero and exceeds the cycle-0 retest spread. We registered this for H2 before the trial
started, and for H1 after the hybrid result but before any graph-only evaluation.

## 4 Preliminary results

**4.1 Across corpora, structure improves but hybrid accuracy does not** (H2; measured before the
defect fixes in §2). Three to five cycles on twelve corpora cut entity counts
by 7.4–26.0% (median 14.8%) and raised mean completeness on eleven; 92–100% of each reduction came
from merging duplicates in the first cycle. Accuracy did not follow. On a seeded build of the memoir,
completeness rose from 51.5% to 78.1% over 31 cycles, while answer recall stayed at 54.1 ± 3.4%
(Spearman ρ = 0.17, n = 11). Across fifteen corpora, completeness did not predict answer recall
(ρ = 0.28).

**4.2 Hybrid retrieval does not benefit** (H2, trial; Table 1, Figure 1a). Arm B grew from 0 to 497
relations, and its completeness rose from 42.2% to 75.4%. Hybrid coverage barely moved: +0.010 for
arm B (CI −0.020 to +0.053) and +0.021 for arm A (CI 0.000 to +0.048). The pre-registered test
fails. Arm B's graph did change the prompt once, at cycle 1, bringing about two more slice passages
per question. After that the prompts stopped changing, although relations kept growing. Single-pass
passage retrieval with no graph (BM25 + dense) reached 52.8%, so the passage index already carried
most of the answerable facts.

**4.3 The graph alone improves** (H1; Table 1, Figure 1b). With passages withheld, cycle-0 cards
covered 14.0% of nuggets; the retest spread was 0.65 points. Consolidation raised this to 32.3% in
arm B (+0.183, CI +0.087 to +0.296) and 31.0% in arm A (+0.170, CI +0.077 to +0.281). The
pre-registered test passes. Most of the gain arrives in the first cycle (arm B 33.2%, arm A 25.7%;
one run each). By cycle 24 the arms were indistinguishable (B − A = +0.013, CI −0.078 to +0.089), so
this trial cannot separate relation completion from better descriptions. Answer recall from
graph-only context showed no detectable change (arm B 14.4% → 18.8%, within the 7.2-point cycle-0
retest spread).

**Table 1.** Consolidation trial: coverage of gold nuggets (31 questions, 108 nuggets). Δ is the
paired change from cycle 0 with a 95% bootstrap interval.

| Retrieval | Cycle 0 | A, cycle 24 | B, cycle 24 | B: Δ (95% CI) |
|---|---|---|---|---|
| Hybrid | 56.4% | 58.5% | 57.5% | +1.0 (−2.0, +5.3) |
| Graph only | 14.0% | 31.0% | 32.3% | +18.3 (+8.7, +29.6) |
| Passages only, single pass | 52.8% | — | — | — |

![**Figure 1.** Coverage of gold nuggets across dream cycles for the two arms: (a) with the passage
index available; (b) from the graph alone, with passages withheld.](figures/short3_consolidation_trial.png){width=100%}

## 5 Discussion and next steps

In this trial, H1 holds and H2 does not. Consolidation moved answerable knowledge into the graph, and
the move was fast. But while the passage index remains, it already supplies most of what the graph learns, so
the hybrid system gains almost nothing. This is consistent with our multi-corpus results, and it
suggests that graph-RAG evaluations should test the graph alone as well as the hybrid. Since the transfer is still only half complete
(32% against 56% coverage), the passage index cannot yet be released.

**Next steps.** (1) *Scale and breadth:* repeat the trial on the whole memoir and the eleven other
corpora, with more runs per checkpoint. (2) *Entity resolution without curation:* without seed data
one person appears as several entities; we will measure how merging during dreaming affects graph-only
coverage. (3) *From facts to answers:* answer recall rose less than coverage; we will test whether
deduplicated, relevance-ordered cards let an 8B model use what it retrieves. (4) *Forgetting:* as graph
coverage approaches the passage store's, evict passages progressively and measure what is lost,
making short-term memory capacity-limited as the analogy requires. (5) *Instrument:* NLI coverage
misses some adjudicated facts that are in the prompt; we will add judged relevance on public corpora,
with static-RAG and GraphRAG baselines.

Limitations: one corpus for the trial, and 10% of it; one run per checkpoint, with repeats only at
the ends; a single 8B model for extraction, dreaming, generation and adjudication; English only.

## References

<!-- Verified 2026-09-25 against arXiv, DBLP, ACL Anthology, ACM DL, Nature, PubMed. -->

- Cormack, G. V., Clarke, C. L. A., Büttcher, S. 2009. Reciprocal rank fusion outperforms Condorcet and
  individual rank learning methods. In Proc. SIGIR '09, 758–759.
- Edge, D., et al. 2024. From local to global: A graph RAG approach to
  query-focused summarization. arXiv:2404.16130.
- Friston, K. 2010. The free-energy principle: a unified brain theory? Nat. Rev. Neurosci. 11, 127–138.
- Guo, Z., et al. 2025. LightRAG: Simple and fast retrieval-augmented
  generation. In Findings of EMNLP 2025, 10746–10761.
- Gutiérrez, B. J., et al. 2024. HippoRAG: Neurobiologically inspired
  long-term memory for large language models. In NeurIPS 2024.
- McClelland, J. L., McNaughton, B. L., O'Reilly, R. C. 1995. Why there are complementary learning
  systems in the hippocampus and neocortex: Insights from the successes and failures of connectionist
  models of learning and memory. Psychol. Rev. 102(3), 419–457.
