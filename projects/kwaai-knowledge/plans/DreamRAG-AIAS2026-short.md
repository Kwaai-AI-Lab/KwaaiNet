---
title: "Dream RAG: Offline Consolidation of a Retrieval Knowledge Graph, and When It Helps"
---

<!--
4-page submission text (anonymized). A précis of DreamRAG-AIAS2026-manuscript.md, the extended
version; every number traces to the evidence ledger in DreamRAG-AIAS2026-manuscript-plan.md.
Build: python3 projects/kwaai-knowledge/plans/build_manuscript_docx.py --short
Items in ⟦double brackets⟧ must be resolved before submission.
-->

## Abstract

Most retrieval-augmented generation (RAG) systems build their knowledge graph once, at ingestion. Dream
RAG adds an offline "dream" loop that rewrites the graph toward *fact density* (fewer, more complete
entities): it completes weak entities from their own evidence with an 8B model, merges duplicates and
prunes unsupported nodes. On twelve corpora of 773–15,018 passages, three to five cycles cut the entity count by
7–26% and raised mean completeness on eleven. Retrieval accuracy did not follow. On a 1,152-passage
memoir, completeness rose from 51.5% to 78.1% over 31 cycles while answer recall stayed within noise
(Spearman ρ = 0.17); across fifteen corpora completeness did not predict recall (ρ = 0.28); and two
consolidation operations reduced accuracy. Across 88 development milestones, accuracy gains came from
retrieval and curated knowledge, not from dreaming. Structural graph metrics are not a proxy for RAG
quality.

**Keywords:** retrieval-augmented generation, GraphRAG, knowledge-graph consolidation, evaluation,
negative results

## 1 Introduction

Graph-based RAG systems add an entity graph to a passage index so that retrieval can follow
relationships [Edge et al. 2024; Gutiérrez et al. 2024; Guo et al. 2024]. They build the graph at
ingestion; what extraction gets wrong (a missing type, a duplicate) stays wrong. Biological memory, by contrast, is consolidated offline [McClelland et al.
1995], and the free-energy principle [Friston 2010] frames the trade-off such a process manages:
explain the evidence with as little model complexity as possible. We use these ideas as design
heuristics, not formal models, and derive one measurable objective, **fact density**: the same evidence
represented by fewer, more complete entities.

We ask whether an offline loop that optimizes fact density improves RAG. Our contributions are (a) an
implemented consolidation loop that converges to zero model calls per cycle, (b) a multi-corpus study
showing that it improves graph structure but not retrieval accuracy, measured against an explicit noise
floor, and (c) two documented ways in which consolidation reduces accuracy.

## 2 System

**Ingestion.** Documents are split into paragraph-based passages of at most 800 characters (200
overlap), embedded (768-dimensional) and indexed for BM25. A capitalized-phrase scan proposes entity
candidates; passages without candidates skip the language model, and otherwise llama3.1:8b extracts at
most 20 typed entities per passage, with the adjacent passages as context. Relation extraction was
disabled for the multi-corpus graphs, because an 8B model extracted relations too imprecisely in earlier
experiments.

**Retrieval** fuses BM25 and dense rankings by reciprocal rank (k = 60) [Cormack et al. 2009] together
with passages of graph entities within two hops of those matching the query. If fewer than 70% of the
query's content terms are covered, a second round retrieves for the missing terms, and below 75% a third
round reformulates the query. The top 20 passages, led by one entity "fact card", go to the generator.

**Completeness.** An entity's score is the mean of three parts: its type (0 untyped, 0.4 generic, 1
specific); its content (the fraction of its type's expected fields that are filled, or a length-and-
sentence tier on its description); and its relations (the fraction of its type's expected relation
groups present). Graph completeness is the mean over entities.

**The dream loop.** Each cycle (i) scores all entities; (ii) selects those below 0.6, weakest first, up
to a budget (100–200); (iii) asks the model, given only the entity's own evidence passages, for its
type-specific fields, a description and relations; (iv) accepts a description only if it ranks in a
higher tier than the current one, or the same tier and more than 20 characters longer, and keeps a
relation only if its target already exists; (v) merges duplicates (identical normalized names; or same
type, Jaro–Winkler ≥ 0.60 and embedding cosine ≥ 0.92; or names differing only by an honorific);
(vi) prunes entities with one mention, no passages, no relations and a score below 0.3. A cycle whose
model calls all failed stops before merging. With no candidates left, a cycle makes no model calls.
Generated descriptions are not fact-checked, and hand-curated descriptions are protected only by the
tier gate in (iv).

## 3 Evaluation setup

**Corpora.** Eleven corpora span historical narrative, legal opinions, meeting transcripts, technical
documentation and standards, scientific and academic papers, and fiction (773–15,018 passages); a
twelfth (a novel) is used for the consolidation study, and a 1,152-passage single-author memoir for
cycle-by-cycle and component studies. All are English. ⟦question-set construction⟧

**Protocol.** Each corpus has 20 questions with gold answers and keywords (58–116 per set). We report
answer recall (the fraction of gold keywords in the generated answer), retrieval recall (in the
retrieved passages) and an LLM judge score (0–2; the judge is the generator's model).

**Noise floor.** Two evaluations of the *same* memoir graph scored 70.2% and 77.8%; ten runs of one
configuration had SD 1.6 points. We treat single-run differences below about 5 points as inconclusive.

## 4 Results

**Across corpora**, retrieval recall ranged from 77.6% (meeting transcripts) to 94.3% (mean 88.9%),
answer recall from 68.5% to 86.1% (mean 76.5%), and judge scores from 1.20 to 1.85 of 2.

**Fact density rises.** Over five cycles on twelve corpora (three on one, whose fourth cycle hit a
since-fixed defect), entity counts fell by 7.4–26.0% (median 14.8%) and mean completeness rose on
eleven. 92–100% of each reduction came from duplicate merging in the first cycle; later cycles added
0.1–0.5 completeness points each. Merging can raise completeness mechanically, since a merged entity
inherits both sources' fields.

**Accuracy does not.** On the memoir, completeness rose from 51.5% to 78.1% and plateaued after about
two hours, while answer recall stayed at 54.1 ± 3.4% (Figure 1), uncorrelated with completeness
(Spearman ρ = 0.17, p = 0.62, n = 11). Across fifteen corpora, completeness did not predict answer
recall either (ρ = 0.28, p ≈ 0.3), nor did the number of relations (r = 0.08). At these sample sizes only a
strong relationship was detectable, but the data rule out that raising the structural score by itself
improves answers.

![**Figure 1.** Memoir, 31 dream cycles: graph completeness (a) and answer keyword recall on the 11
evaluated cycles (b). Shaded: cycles with a 3B completion model. Cycle 12 is omitted (failed
run).](figures/short1_memoir_cycles.png){width=100%}

**Consolidation can hurt** (Table 1, last two rows). A maintenance prune of 361 unevidenced but connected entities
deleted 2,972 of 6,164 relations, and in one cycle generated summaries overwrote curated descriptions;
re-seeding restored recall to 178/225 keywords. Pruning now spares connected entities unless asked.

**Table 1.** Controlled comparisons on the memoir (answer keyword recall; *n* = runs per arm).

| Change | Before | After | n |
|---|---|---|---|
| Single-pass → iterative multi-round retrieval | 39.7% | 57.8% | 1 / 3 |
| Seven curated organization/place nodes added | 54.4% (CI 50.7–58.0) | 63.1% | 3 / 1 |
| Graph-context injection turned off | 53.4% | 56.9% | 1 / 1 |
| Generic → corpus-specific ontology (40 q) | 59.3% | 55.5% | 1 / 1 |
| Dream cycles only (5 milestones, mean change) | — | −1.7 points | 5 |
| Prune connected, weakly evidenced entities | 56.7% | 52.6% (Welch p = 0.055) | 4 / 3 |
| Summaries overwrite curated descriptions | 166–173 kw | 155 of 225 kw | 1 |

**What moved accuracy during development** (Figure 2). Over 88 milestones, memoir recall rose from 24.6%
to 95.6%. This is a development history, not held-out accuracy: every milestone was tuned against the
same questions, and the question set, generation temperature and scorer each changed once. Code and configuration
changes to retrieval and query handling drove most of the rise to 74.7% (milestone 58). The five milestones that only ran dream cycles changed
recall by −4.5 to +0.9 points. The eight that only edited curated seed data all raised it (+0.4 to +8.7),
several in response to specific questions, and account for about 25 points.

**Cost.** Graph construction ran at 0.17–0.83 passages/s on two commodity GPUs (15,018 passages in
18 h), depending on entity density. A dream cycle of 200 completions took 3–21 min (median 13), with no
trend in graph size; a converged cycle takes seconds.

![**Figure 2.** Memoir answer recall across 88 development milestones. Dashed lines: changes to the
instrument. Orange: dream cycles only. Green: curated seed edits only.](figures/short2_development_history.png){width=100%}

## 5 Discussion and limitations

The loop does what it is designed to do, and it does not buy accuracy. Our explanation is that the
graphs carried almost no relations, so completeness tracked types and fields that restate what the
passages already say; hybrid, iterative retrieval over those passages is what moved answers. We expect
consolidation to pay off only with precise, typed relation extraction. Until then, fact density is
worth having where people read and maintain the graph, not as a route to better answers.

Limitations: no static-RAG or external GraphRAG baseline; before/after evaluation on one corpus only;
one to four runs per comparison; keyword-overlap scoring with a same-model judge; no fact-checking of
generated descriptions; English only, at most 15,018 passages. We recommend that evaluations of
self-improving RAG report accuracy directly, against a measured noise floor, and never infer it from
graph structure.

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
