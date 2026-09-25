---
title: "Dream RAG: Offline Consolidation of a Retrieval Knowledge Graph, and When It Helps"
subtitle: "AIAS+ 2026, submission 54 — revised manuscript (anonymized draft)"
---

<!--
Anonymized manuscript draft. Source of truth for the text until it is moved onto the ACM template.
Every number traces to the evidence ledger in ../../plans/DreamRAG-AIAS2026-manuscript-plan.md.
Items in ⟦double brackets⟧ must be resolved before submission.
No project, product, network, machine, author or memoir names may appear in this file.
-->

## Abstract

Most retrieval-augmented generation (RAG) systems build their index once, at ingestion. We describe
Dream RAG, an implemented RAG system with an offline "dream" loop that optimizes its knowledge graph for
*fact density*: the same evidence represented by fewer, more complete entities. During idle time, the
loop scores every
entity for structural completeness, completes the weakest from their own evidence passages with a small
(8B) language model, merges duplicates, and prunes unsupported nodes; once no candidate remains, a cycle
makes no model calls. Retrieval fuses BM25 and dense search by reciprocal rank and adds entity context
from the graph. We evaluate on eleven corpora spanning history, law, science, technical standards,
fiction and meeting transcripts (773–15,018 passages), and study the loop over time on a 1,152-passage
memoir. The loop reliably increases fact density: over five cycles on twelve corpora (three on one), it
cut the entity count by 7–26% (median 15%), almost all of it by duplicate merging in the first cycle,
and mean completeness rose on eleven of the twelve. On the memoir, completeness rose from 51.5% to 78.1%
over 31 cycles and then plateaued. Retrieval accuracy did not follow. Completeness was uncorrelated with
answer recall across cycles on the memoir (Spearman ρ = 0.17, p = 0.62, n = 11) and across fifteen
corpora (ρ = 0.28, p ≈ 0.3), and two consolidation operations measurably reduced accuracy. We quantify
the evaluation noise floor, conclude that structural graph metrics are not a valid proxy for retrieval
quality, and identify typed relation extraction as the prerequisite for consolidation to pay off.

**Keywords:** retrieval-augmented generation, GraphRAG, knowledge-graph consolidation, offline
enrichment, fact density, evaluation methodology, negative results

## 1 Introduction

Retrieval-augmented generation grounds a language model's answers in passages retrieved from a document
collection [Lewis et al. 2020]. Graph-based variants add a knowledge graph of the entities those passages
mention, so that retrieval can follow relationships a passage-level index cannot see [Edge et al. 2024;
Gutiérrez et al. 2024; Guo et al. 2025]. In almost all of these systems the index is built once. Entity
extraction runs at ingestion, and whatever it gets wrong — a missed type, an empty description, the same
person recorded under three spellings — stays wrong until the collection is re-ingested.

Biological memory does not work this way. Memories are consolidated offline, during sleep, when recent
experience is replayed, integrated with what is already known, and selectively weakened
[McClelland et al. 1995; Diekelmann and Born 2010]. The free-energy principle [Friston 2010] offers a
compact description of the trade-off such a process manages: a good model explains its evidence with as
little complexity as possible. We do not model either idea formally. We use them as design heuristics,
and they lead to one measurable objective, which we call **fact density**: the same source evidence
represented by fewer, more complete entities. Removing redundant and weakly supported entities lowers
complexity; completing the remaining ones from their evidence improves fit.

Dream RAG implements this as an offline loop over the knowledge graph of an otherwise conventional hybrid
RAG system. The question we set out to answer is whether it helps. The answer turned out to be
two-sided, and this paper reports both sides. The loop does what it is designed to do: across twelve
corpora it removes 7–26% of entities and raises structural completeness. But retrieval accuracy does
not follow, within one corpus over 31 cycles or across fifteen corpora, and two consolidation
operations made it worse.

Our contributions are:

- **(a)** an implemented offline consolidation loop for a RAG knowledge graph, with completeness-scored
  candidate selection, evidence-grounded completion, duplicate merging and pruning, which converges to
  a state in which a cycle makes no model calls (§3);
- **(b)** a multi-corpus study of the loop's effect on graph structure and on retrieval accuracy, measured
  against an explicit evaluation noise floor (§4–5);
- **(c)** documented failure modes of consolidation, and the safeguards they led to (§5.4).

The negative result is, we think, the most useful part for practitioners: structural graph metrics are
easy to compute and easy to improve, and in our system they did not predict whether answers got better.

## 2 Related work

⟦verify every reference against the published version before submission⟧

**Graph-augmented retrieval.** GraphRAG [Edge et al. 2024] extracts an entity graph from the corpus,
clusters it into communities and pre-computes community summaries for global, query-focused
summarization. HippoRAG [Gutiérrez et al. 2024] builds a schemaless graph and retrieves by personalized
PageRank from query entities, motivated by hippocampal indexing theory. LightRAG [Guo et al. 2025] couples
graph and vector retrieval at two levels of granularity and supports incremental updates. RAPTOR
[Sarthi et al. 2024] recursively clusters and summarizes passages into a tree. All four construct their
index structure at ingestion. They may add to it when documents arrive, but they do not revisit what was
already extracted.

**Revising at query time.** Self-RAG [Asai et al. 2024] trains the generator to decide when to retrieve
and to critique its own output. Iterative and multi-round retrieval re-query when coverage is incomplete.
These methods improve a single answer and leave the index unchanged.

**Memory management and reflection.** MemGPT [Packer et al. 2023] manages a tiered context for an agent,
paging information between a working context and external storage. Generative agents [Park et al. 2023]
periodically *reflect*, synthesizing higher-level observations from a stream of memories. These are the
closest in spirit to offline consolidation, but they operate on an agent's own experience, not on a
document index built for retrieval.

**Complementary learning systems.** The theory that fast episodic learning is followed by slow offline
integration [McClelland et al. 1995] and the role of sleep in that integration [Diekelmann and Born
2010] motivate our design. We borrow the schedule — work done offline, in idle time — and not any claim
of biological fidelity.

**Evaluating RAG.** Reference-free frameworks score faithfulness and relevance with a language model
[Es et al. 2024], and language models are widely used as judges [Zheng et al. 2023]. We use keyword
recall against gold answers plus an LLM judge, and we report the run-to-run variability of both, which
evaluations of graph RAG rarely do.

Our system differs from the graph RAG systems above in one respect that is the subject of this paper: it
revises the graph *after* ingestion, repeatedly and offline, using the graph's own structural
completeness to decide what to revise.

## 3 System

Figure 1 gives an overview. The system has three parts: ingestion builds a passage index and an entity
graph; retrieval combines both at query time; and the dream loop revises the graph offline.

### 3.1 Ingestion

Documents are split into passages by a paragraph → sentence → character cascade, with at most 800
characters per passage and 200 characters of overlap; passages shorter than 20 characters are dropped.
Each passage is embedded with a 768-dimensional text embedding model (nomic-embed-text) and indexed
for BM25.

Entities are extracted per passage in two steps. A lightweight scan first collects capitalized-phrase
candidates (at most 40 per passage). A passage with no candidates is not sent to the language model at
all, which is why construction time depends on entity density (§5.5). Otherwise an 8B model
(llama3.1:8b) extracts typed entities, restricted to a declared type vocabulary, with the adjacent
passage on either side from the same document section as context. At most 20 entities are kept per
passage (25 when three or fewer types are declared), which limits over-extraction on dense passages.

Relation extraction was disabled for the twelve multi-corpus graphs ⟦check which memoir builds used
relation extraction, and say so⟧. In earlier experiments, unconstrained
relation extraction with an 8B model was too imprecise to use ⟦cite the figure only if a result file
backs it⟧. The memoir graph additionally carries a hand-curated set of entities and family relations,
inserted as seed data.

### 3.2 Hybrid retrieval

Dense and BM25 rankings are fused by reciprocal rank fusion (RRF, k = 60) [Cormack et al. 2009], over
4 × top-k candidates. All experiments use the *iterative* mode, which proceeds in up to three rounds:

1. **Round 1** fuses hybrid passage retrieval with graph-anchored retrieval. Seed entities are the five
   nearest by embedding, plus entities whose name or alias appears in the query; the passages of
   entities within two hops of a seed are added to the pool.
2. **Coverage check.** Coverage is the fraction of the query's content terms (four or more characters,
   not stop words) that appear in the pool. If it is below 0.70, **round 2** embeds the missing terms,
   takes the six nearest entities, and adds the passages of entities within two hops.
3. If coverage is still below 0.75, **round 3** asks the language model to reformulate the query and
   runs a further hybrid search.

The pool is then re-ranked with a bonus for each covered query term. One synthetic "fact card"
summarizing the best-matching graph entity is placed first. The top
20 items go to the generator.

### 3.3 Knowledge graph and the completeness score

Each entity has a name, aliases, a type, a free-text description, a set of typed fields (for example
birth date or occupation for a person), links to the passages it was extracted from, and a mention
count. The completeness of an entity is the unweighted mean of three components, each in [0, 1]:

- **Type:** 0 if untyped, 0.4 for the generic type *Thing*, 1 for any specific type.
- **Content:** for types with a field schema (a person has ten expected fields, a place two, an
  organization four), the fraction of fields filled. For other types, a tier on the description:
  0 if empty, 0.3 under 50 characters, 0.6 under 150, and 1.0 for 150 characters or more with at least
  two sentences (0.8 otherwise).
- **Relations:** the fraction of the relation groups expected for the entity's type (for a person:
  kinship, organizational and locational) with at least one relation. Types with no expected groups,
  and *Thing*, score a neutral 0.5; an entity mentioned at most twice needs only one group.

Graph completeness is the mean over all entities. The expected fields and relation groups are fixed
per type. Because the multi-corpus graphs have almost no relations, the relation component is at or
near zero for most typed entities, which caps their completeness.

### 3.4 The dream loop

One run of the loop performs one cycle (Algorithm 1). Cycles are repeated by an external scheduler; in
our experiments, a driver script ran them back to back.

```text
Algorithm 1  One dream cycle
Input: graph G, completion budget B, workers W, threshold θ = 0.6
 1  score every entity in G                                    (§3.3)
 2  C ← entities with score < θ, ascending by score, first B
 3  if C is empty: return                           (converged: no model calls)
 4  for each e in C, on W parallel workers:
 5      P ← e's linked passages (at most 20); if none, passages containing e's name (at most 10)
 6      if P is empty: skip e
 7      ask the model, given P, for e's type-specific fields, a description and relations
 8      accept the description only if it passes the improvement gate
 9      set e's type only if e has none; keep a relation only if its target exists in G
10      re-embed e
11  if no completion succeeded and some call failed: return      (failure guard)
12  merge duplicates                                              (below)
13  prune e if mentions(e) ≤ 1 and e has no passages, no relations and score < 0.3
14  apply consistency rules to kinship relations
15  re-score G
```

**Completion (lines 5–10).** The entity is routed by type to a task prompt: biography for a person,
geography for a place, profile for an organization, event or creative work, definition for a concept,
and a general task otherwise. The prompt contains only the entity's own evidence passages, prefixed
with the document's title and role. The model returns JSON at temperature 0.25.

**Improvement gate (line 8).** A description is ranked into the tiers used by the content score. A
generated description replaces the current one only if its tier is higher, or its tier is equal and it
is more than 20 characters longer.

**Duplicate merging (line 12)** uses three tiers:

1. Entities whose normalized names are identical.
2. Pairs that share a significant name token and have the same type, pass a Jaro–Winkler gate of 0.60
   on their names, and have embedding cosine similarity of at least 0.92. Names carrying a
   disambiguating suffix, such as "(film)" or "II", are never merged.
3. Names that differ only by an honorific.

A merge moves the duplicate's relations and passage links to the surviving entity, appends its name to
the aliases, sums the mention counts, and keeps the longer description.

**Convergence.** When no entity scores below θ, or none of those that do has evidence, line 3 returns
immediately. A converged cycle makes no model calls and finishes in seconds.

The per-cycle budget was 100 completions on the memoir and 200 on the other corpora, with four
workers. On the memoir, cycles 1–9 used a 3B completion model and later cycles the 8B model.

### 3.5 Safeguards

The loop has four safeguards against degrading the graph:

- Completions are conditioned only on the entity's own evidence passages.
- The improvement gate (§3.4) stops a completion from replacing a better description.
- A completion cannot create entities: relations to names not already in the graph are dropped.
- The failure guard (Algorithm 1, line 11) stops a cycle whose model calls failed from going on to
  merge and prune. We added it after a cycle against an unreachable endpoint made no completions but
  still merged entities and deleted relations.

Two protections are *absent*, and §5.4 and §6 return to them. Generated descriptions are not
fact-checked. And hand-curated descriptions have no protection inside the loop beyond the improvement
gate: a merge keeps the longer description, and a completion replaces one that it outranks. A curated
description takes precedence only when seed data is re-inserted.

## 4 Experimental setup

### 4.1 Corpora

Table 1 lists the corpora. Eleven form the main evaluation (§5.1). They cover seven domain types:
historical narrative, legal opinions, conversational meeting transcripts, technical documentation and
standards, scientific literature, academic papers and literary fiction, and range from 773 to 15,018
passages. A twelfth, a single novel of 6,451 passages, is used only in the consolidation study (§5.3)
and the cost analysis (§5.5). A 1,152-passage single-author memoir, not otherwise part of the
multi-corpus evaluation, is used to study the dream loop over time and for component comparisons
(§5.2–5.4), because it is the only corpus on which we evaluated retrieval after individual dream cycles.
The cross-corpus correlation in §5.3 uses fifteen corpora: the eleven above, the novel, and three
more (map-data documentation, a national history and a poetry collection) ⟦add to Table 1 or a
footnote⟧. All corpora are in English.

**Table 1.** Corpora. Passages are paragraph-based chunks of at most 800 characters, with 200 characters of overlap. Entities: after graph construction
→ after the dream cycles. Build: wall-clock hours for entity extraction (8B model, four workers, two
commodity GPUs). Every corpus has 20 questions; *kw* is the total number of gold keywords.

| Corpus | Domain type | Passages | Entities | Dream cycles | Build (h) | kw |
|---|---|---:|---:|---:|---:|---:|
| Manhattan Project | historical narrative | 773 | 866 → 736 | 3ᵃ | 0.9 | 79 |
| Deep-sea biology | scientific literature | 1,886 | 2,657 → 2,374 | 5 | 3.0 | 58 |
| Meeting transcripts | conversational | 1,914 | 797 → 729 | 5 | 1.1 | 59 |
| Sleep & memory papers | academic papers | 2,254 | 3,221 → 2,983 | 5 | 3.2 | 67 |
| Climate science | scientific literature | 2,747 | 2,532 → 2,006 | 5 | 3.3 | 62 |
| Legal opinions | legal | 3,723 | 3,523 → 2,962 | 5 | 3.9 | 80 |
| Python documentation | technical documentation | 3,783 | 1,233 → 999 | 5 | 1.3 | 63 |
| AI security standards | technical standards | 3,934 | 7,019 → 5,193 | 5 | 4.7 | 72 |
| Astrophysics | scientific literature | 5,638 | 7,550 → 6,453 | 5 | 7.9 | 75 |
| Internet RFCs | technical standards | 6,544 | 2,369 → 1,964 | 5 | 3.4 | 78 |
| *War and Peace*ᵇ | literary fiction | 6,451 | 4,194 → 3,689 | 5 | 8.9 | 80 |
| *Moby-Dick* & companions | literary fiction | 15,018 | 13,995 → 12,186 | 5 | 18.0 | 78 |
| Memoirᶜ | single-author memoir | 1,152 | ⟦from the 2026-05-20 build⟧ | 31 | ⟦⟧ | 116 |

ᵃ The fourth cycle failed on a text-encoding defect, since fixed. ᵇ Used for Figures 3 and 6 only; not
in the eleven-corpus evaluation of Figure 2. ᶜ Used for the dream-over-time study (§5.3) and the
component comparisons (Table 2).
⟦Domain labels are my reading of the corpus names; check them against the source documents.⟧

### 4.2 Protocol

Each corpus has 20 questions with gold answers and a list of gold keywords per question (58–80 keywords
per corpus; 116 for the memoir set). ⟦how the question sets were constructed — who wrote them, and how⟧

For each question the system retrieves the top 20 passages with iterative hybrid retrieval (§3.2) and
generates an answer with llama3.1:8b. We report three measures:

- **Retrieval recall**: gold keywords found in the concatenated retrieved passages;
- **Answer recall**: gold keywords found in the generated answer;
- **Judge score**: a rating of the answer against the gold answer by an LLM judge at temperature 0:
  2 if all key facts are present (wording may differ), 1 if some are present but some are missing or
  wrong, and 0 if none are. The judge is the same 8B model as the generator.

A keyword of one or two tokens matches if all its tokens occur; a longer one matches if at least half
do. In the multi-corpus evaluation (§5.1) a keyword also earns partial credit by embedding similarity:
its score is the larger of the token match and the cosine similarity to the answer (or, for retrieval,
to the most similar retrieved passage) mapped linearly from [0.30, 0.85] onto [0, 1]. Recall is pooled
over all keywords in a question set, not averaged per question.

The memoir experiments (§5.2–5.4) report answer recall by token overlap only, and were run at different
times over several months of development; each comparison in Table 2 holds everything but the named
factor fixed, and states its question set and number of runs.

### 4.3 The noise floor

Before interpreting any difference we measured how much the evaluation varies on its own. Two evaluations
of the *same* memoir graph, with no change in between, scored 158 and 175 of 225 keywords (70.2% vs
77.8%), a difference of 7.6 points. Ten runs of one pipeline configuration had a mean of 62.9% and a
standard deviation of 1.6 points; code changed between some of those runs, so they are not pure
repeats. Three repeated runs of another configuration gave 54.4% ± 1.5 (95% CI 50.7–58.0%).

We therefore treat single-run differences below about 5 points as
inconclusive, report the number of runs next to every comparison, and use a significance test where we
have repeated runs.

## 5 Results

### 5.1 Performance across corpora

Figure 2 reports the three measures for the eleven corpora, all evaluated in one session with the
same configuration, after three to five dream cycles. Retrieval recall ranged from 77.6% (meeting transcripts) to 94.3% (AI security standards),
mean 88.9%. Answer recall ranged from 68.5% (meeting transcripts) to 86.1% (*Moby-Dick* and companions),
mean 76.5%, and judge scores from 1.20 to 1.85 of 2, mean 1.61.

Two patterns stand out. First, answer recall trails retrieval recall on every corpus, by 12.4 points on
average: the retriever finds evidence that the 8B generator then fails to use or paraphrases away.
Second, conversational transcripts are hardest on all three measures. We suspect this is because their facts are
spread across turns and speakers, so a passage of at most 800 characters often carries only part of an exchange.

### 5.2 Component comparisons

Table 2 collects the controlled comparisons available on the memoir corpus.

**Table 2.** Controlled comparisons on the memoir corpus. Answer keyword recall; llama3.1:8b generator;
*n* = evaluation runs per arm. The noise floor (§4.3): two runs on one graph differed by 7.6 points;
ten runs of pipeline A had SD 1.6. Differences smaller than these are inconclusive.

| Factor | Arm A | Arm B | n (A / B) | Question set | Reading |
|---|---|---|---|---|---|
| Retrieval routing | iterative multi-round 57.8% | single-pass automatic 39.7% | 3 / 1 | 20 q, 116 kw | large; outside noise |
| Curated seed nodes (+7 org/place) | without 54.4% (95% CI 50.7–58.0) | with 63.1% | 3 / 1 | 20 q, 116 kw | B above A's CI |
| Post-processing order | A 61.8% · B 54.2% · C 56.9% | — | 1 each (A: 10 runs, 62.9 ± 1.6) | 20 q, 116 kw | A best |
| Graph-entity context injection | on 53.4% | off 56.9% | 1 / 1 | 20 q, 116 kw | inconclusive; "on" had a merge defect |
| Domain ontology (same 17 types) | global predicates 59.3% | memoir-specific 55.5% | 1 / 1 | 40 q, 209 kw | inconclusive |
| Dream cycles (31) | completeness 51.5 → 78.1% | recall 54.1 ± 3.4% over 11 evaluated cycles | — | 20 q, 116 kw | uncorrelated (ρ = 0.17, p = 0.62) |

**Retrieval strategy matters most.** Multi-round iterative retrieval, which re-queries for query terms
not yet covered, scored 57.8% keyword recall (mean of 3 runs) against 39.7% for single-pass automatic
routing (1 run). This is the only difference in Table 2 far outside the noise floor on its own.

**Curated structure helps.** Adding seven hand-curated organization and place nodes raised recall from
54.4% (mean of 3; 95% CI 50.7–58.0%) to 63.1% (1 run), above the upper end of the interval.

**Post-processing order matters.** Three orderings of the same graph post-processing steps scored 61.8%,
54.2% and 56.9%.

**Automatically extracted graph context is small and sometimes negative.** Turning off the injection of
graph-entity context into the retrieved set *raised* recall from 53.4% to 56.9% (1 run each); the
injected context in that run included malformed entities produced by a merge defect. A
corpus-specific ontology, with the same entity types and a narrower relation vocabulary, scored 55.5%
against 59.3% for the generic one (1 run each, 40 questions). Both differences are within the noise
floor.

We did not run a static RAG baseline — hybrid retrieval with the graph and the dream loop disabled —
across the corpora. We return to this in §6.

### 5.3 Dreaming: fact density rises, accuracy does not

**Fact density.** Figure 3 shows the effect of five dream cycles on twelve corpora (three on the
Manhattan Project corpus, whose fourth cycle failed on a text-encoding defect since fixed). The entity
count fell on every corpus, by 7.4–26.0% (median 14.8%), and mean completeness rose on eleven of the
twelve; on the AI security standards corpus it fell from 38.8% to 38.6%. Between 92% and 100% of each
corpus's reduction happened in the first cycle, when accumulated duplicates are merged. Later cycles
removed few entities and raised completeness by about 0.1–0.5 points each. Figure 4 extends the series
to ten cycles: the two smallest graphs gained 4.4–4.6 points and flattened by the fifth cycle, and most
others gained 1–1.5 points in a slow, steady climb.

Merging can raise the completeness score mechanically, because a merged entity inherits the union of
its sources' fields. The density gain is therefore real, but part of the completeness gain may be
bookkeeping rather than new information.

**Accuracy over cycles.** Figure 5 follows the memoir graph over 31 cycles. Completeness rose from
51.5% to 78.1% and plateaued from cycle 24, after about two hours of consolidation. Answer recall,
measured on eleven of those cycles, stayed at 54.1 ± 3.4% — within the noise floor throughout — and was
uncorrelated with completeness (Spearman ρ = 0.17, p = 0.62; Pearson r = 0.25, p = 0.46). Restricting
to the cycles run with the 8B completion model leaves five points and no trend (ρ = −0.26).

**Accuracy across corpora.** As an independent check we correlated completeness with answer recall
across fifteen corpora evaluated with one configuration on the same day. The correlation was weak and
not significant (Spearman ρ = 0.28, Pearson r = 0.25, p ≈ 0.3, n = 15). The number of relations did not
predict recall either (Pearson r = 0.08). The eleven-corpus round of §5.1 agrees: completeness against
retrieval recall gave ρ = 0.05 (p = 0.89). Cross-corpus correlations are confounded by domain and
question difficulty, so they cannot show that completeness *doesn't* matter; they show that it is not
a usable predictor of accuracy.

With n = 11 cycles and n = 15 corpora, only a strong relationship (|ρ| ≳ 0.6) would have reached
significance. A modest effect could be hidden in the noise. What the data excludes is the assumption
that improving the graph's structural score will, by itself, improve answers.

### 5.4 When consolidation hurts

Two consolidation operations reduced accuracy on the memoir corpus (Table 3).

**Table 3.** When consolidation reduced accuracy (memoir corpus).

| Operation | Graph change | Recall before → after | n | Safeguard now in the system |
|---|---|---|---|---|
| Pruning unevidenced entities *with* relations | 1,905 → 1,544 entities; 6,164 → 3,192 relations | 56.7% → 52.6% (Welch t = 2.50, p = 0.055) | 4 / 3 | prune removes only unevidenced entities with no relations; connected ones need an explicit opt-in |
| Overwriting curated descriptions (cycle 6) | generated summaries replaced hand-curated descriptions | 166–173 → 155 of 225 kw; 178 after re-seeding | 1 per cycle | re-seeding restores them (a seeded description takes precedence on insertion); inside the loop, only the improvement gate |

**Pruning connected, weakly evidenced entities.** A separate graph-maintenance prune, run outside the
dream loop, removed 361 entities that had no
direct textual evidence but were linked to evidenced ones. It deleted 2,972 of the graph's 6,164
relations. Keyword recall fell from a mean of 56.7% (4 runs) to 52.6% (3 runs; Welch's t = 2.50,
p = 0.055). Taken one at a time, the pruned entities looked like noise; together, they connected the entities
that did have evidence. The pruning operation now removes only unevidenced entities that have
no relations; removing connected ones requires an explicit opt-in.

**Overwriting curated descriptions.** In one cycle, generated summaries replaced hand-curated entity
descriptions during merges. Recall dropped to 155 of 225 keywords, from 166–173 in the preceding five
cycles. After the curated descriptions were restored by re-seeding the graph, recall recovered to 178
of 225. Inside the loop, curated descriptions are still protected only by the improvement gate: a merge
keeps the longer of two descriptions, and a completion replaces one that it outranks (§3.5, §6).

Both failures have the same shape. Each operation improved a structural measure — fewer weak entities,
more complete descriptions — while removing information that retrieval depended on.

### 5.5 Cost and scale

**Graph construction** is the dominant cost (Figure 6a). With an 8B extraction model, four workers and
two commodity GPUs, throughput ranged from 0.17 to 0.83 passages per second, and the largest corpus
(15,018 passages) took 18 hours. Throughput depends on entity density as well as size, because passages
with no named-entity candidates skip the language-model call: two corpora of similar size (3,783 and
3,934 passages) took 1.3 and 4.7 hours.

**Consolidation** is bounded by the per-cycle completion budget, not by corpus size (Figure 6b). On the
memoir, a cycle of 100 completions with four workers took about 5–6 minutes, and 31 cycles took about two
hours. On eleven other corpora, a cycle of 200 completions took 3.0–21.1 minutes (median 13.0), with no
trend in graph size: the largest graph, about 12,000 entities, had a median of 8.3 minutes. The spread
tracked the order in which corpora ran overnight, which points to load on the shared inference servers.
A converged cycle makes no model calls and completes in seconds.

In practice we recommend running the loop after ingestion until it converges, and again when new
documents arrive. We have not tested corpora beyond 15,018 passages.

### 5.6 What moved accuracy during development

Figure 7 shows answer recall on the memoir across 88 development milestones. It rises from 24.6% to
95.6%, but three things keep it from being a measure of the final system's accuracy. All 88 milestones
were developed against the same questions they are scored on, so none of the rise is held-out. The
instrument changed three times: the question set grew from 20 to 40 questions, generation moved to
temperature 0, and the scorer began giving partial credit for nearby years. And from 89.8% onward, every gain
but one came from editing the hand-curated seed data, several times in response to specific questions.

Read with those caveats, the figure still says what drove recall. Code and configuration changes to
retrieval, query handling and ingestion drove the early rise, to 74.7% by milestone M58. The five
milestones whose only change was running dream cycles moved recall by −4.5, +0.9, −0.9, −0.4 and −3.5
points (mean −1.7), within the noise floor; one of them, re-evaluated on the same graph, moved by
7.6 points on its own. The eight milestones whose only change was to curated seed data all raised recall
(+0.4 to +8.7 points, mean +3.2), and together account for 25 points of the curve. Curated knowledge is effective, but on questions it was curated
against, it measures how well the curation anticipated them, not how well the system generalizes.

## 6 Discussion and limitations

**Why structure did not buy accuracy.** The multi-corpus graphs were built without relation
extraction, because unconstrained relation extraction with an 8B model was too imprecise to use in our
earlier experiments ⟦cite the precision figure only if a result file backs it⟧. After construction they
had no relations, and after five dream cycles at most ten, all added by completions. A third of the
completeness score measures relations, so on these graphs completeness mostly tracks entity types and
descriptive fields. Those fields restate what the entity's passages already say, and retrieval benefits
only from information the passages do not already surface. In our system the evidence passages remain the main carrier of answerable
facts, and hybrid, iterative retrieval over them is what moved accuracy. We expect consolidation to pay
off only once extraction produces typed, verifiable relations, and we identify that as the
prerequisite for future work.

**When consolidation is still worth running.** Fact density is valuable in itself where a graph is read
by people: fewer duplicates, complete descriptions and consistent types make the graph inspectable and
make curated knowledge maintainable. On current evidence, that — not retrieval accuracy — is what the
added complexity buys.

**Limitations.**

- *No static baseline and no external systems.* We did not run hybrid retrieval with the graph and the
  dream loop disabled across the corpora, and we did not re-implement GraphRAG, HippoRAG or LightRAG.
  The paper therefore makes no claim of improvement over standard RAG or over other graph RAG systems.
- *Single runs.* Most comparisons have one to four runs per arm; the noise floor (§4.3) is large
  relative to several of the effects we report.
- *Before/after evaluation on one corpus.* Only the memoir was evaluated after individual dream cycles;
  the other corpora contribute structural data and a single post-consolidation evaluation.
- *Metric.* Keyword overlap is a coarse instrument, and the LLM judge is the same 8B model as the
  generator.
- *Curated descriptions are not protected inside the loop.* Beyond the improvement gate, nothing stops
  a merge or a completion from replacing a hand-curated description with a longer generated one; the
  failure in §5.4 was repaired by re-seeding, not prevented.
- *No fact-checking of generated descriptions.* Completions are conditioned on each entity's own
  evidence and gated (§3.5), but not independently verified. A manual audit of the memoir graph found
  extraction errors, such as a swapped gender and conflated person names, which consolidation can
  propagate. ⟦decide whether to include the audit counts⟧
- *Scale and language.* All corpora are in English, and the largest has 15,018 passages.

## 7 Conclusion

An offline consolidation loop can reliably make a RAG knowledge graph denser and structurally more
complete, at a cost bounded by its completion budget. In our system, across twelve corpora, that did not
translate into better answers, and two consolidation operations made answers worse. Structural graph
metrics should not be used as a proxy for retrieval quality. Evaluations of consolidating or
self-improving RAG systems should report accuracy directly, against a measured noise floor. The step
we expect to change the outcome is typed relation extraction.

## Appendix: parameters and prompts

⟦from §3 facts; prompts from dream_tasks.rs⟧

## References

⟦Cormack, Edge, Friston, Guo, Gutiérrez and McClelland verified 2026-09-25; verify the rest⟧

- Asai, A., Wu, Z., Wang, Y., Sil, A., Hajishirzi, H. 2024. Self-RAG: Learning to retrieve, generate,
  and critique through self-reflection. ICLR 2024.
- Cormack, G. V., Clarke, C. L. A., Büttcher, S. 2009. Reciprocal rank fusion outperforms Condorcet and
  individual rank learning methods. In Proc. SIGIR '09, 758–759. doi:10.1145/1571941.1572114
- Diekelmann, S., Born, J. 2010. The memory function of sleep. Nature Reviews Neuroscience 11, 114–126.
- Edge, D., Trinh, H., Cheng, N., et al. 2024. From local to global: A graph RAG approach to
  query-focused summarization. arXiv:2404.16130.
- Es, S., James, J., Espinosa-Anke, L., Schockaert, S. 2024. RAGAS: Automated evaluation of retrieval
  augmented generation. EACL 2024 (demonstrations).
- Friston, K. 2010. The free-energy principle: a unified brain theory? Nat. Rev. Neurosci. 11, 127–138.
  doi:10.1038/nrn2787
- Guo, Z., Xia, L., Yu, Y., Ao, T., Huang, C. 2025. LightRAG: Simple and fast retrieval-augmented
  generation. In Findings of EMNLP 2025, 10746–10761.
- Gutiérrez, B. J., Shu, Y., Gu, Y., Yasunaga, M., Su, Y. 2024. HippoRAG: Neurobiologically inspired
  long-term memory for large language models. In NeurIPS 2024.
- Lewis, P., et al. 2020. Retrieval-augmented generation for knowledge-intensive NLP tasks. NeurIPS 2020.
- McClelland, J. L., McNaughton, B. L., O'Reilly, R. C. 1995. Why there are complementary learning
  systems in the hippocampus and neocortex: Insights from the successes and failures of connectionist
  models of learning and memory. Psychol. Rev. 102(3), 419–457. doi:10.1037/0033-295X.102.3.419
- Packer, C., et al. 2023. MemGPT: Towards LLMs as operating systems. arXiv:2310.08560.
- Park, J. S., et al. 2023. Generative agents: Interactive simulacra of human behavior. UIST 2023.
- Robertson, S., Zaragoza, H. 2009. The probabilistic relevance framework: BM25 and beyond. Foundations
  and Trends in Information Retrieval 3(4), 333–389.
- Sarthi, P., Abdullah, S., Tuli, A., Khanna, S., Goldie, A., Manning, C. D. 2024. RAPTOR: Recursive
  abstractive processing for tree-organized retrieval. ICLR 2024.
- Zheng, L., et al. 2023. Judging LLM-as-a-judge with MT-Bench and Chatbot Arena. NeurIPS 2023
  (Datasets and Benchmarks).
