---
title: "DreamRAG: What a Knowledge Graph Learns While It Sleeps"
authors: "Annika Vriens, Christopher Mayfield, Jourdane Hamilton, Reza Rassool"
affiliation: "Kwaai, Los Angeles, USA · {annika, chris, jourdane, reza}@kwaai.org"
note: "Builds on Vriens et al., AIAS+ 2026"
---

<!--
AIAS+ extended abstract. Length (organisers, 2026-11-25): 1-5 pages recommended, references and
appendices included, measured in the final ACM two-column proceedings layout; keep the accepted
category (Extended Abstract). Supplementary material goes separately at the ACM production stage.
Build:
  python3 projects/kwaai-knowledge/papers/aias-forgetting/build_docx.py
Figure 1 (method): figures/gen_method_diagram.py -> fig0_method.{svg,png,pdf}.
Number sources (feat/dream-forgetting worktree, tests/kwaai-knowledge/results/dream_forget/):
  Figure 2, Table 1       <run>/metrics_t0.jsonl (temperature-0 re-score) + _recall_per_size/recall_per_size.csv
                          (memory, chunks kept); runs {Legal,NIST,Manhattan_v3}_p10_b40000, Legal_p100_b40000;
                          figures/gen_binned_t0.py -> binned_t0.csv
  Baselines, Fig 2 refs   <run>/metrics_t0_extra.jsonl (2026-11-25 temperature-0 re-score,
                          eval2/pilot/rescore_t0.py --extra / --chunk-baseline): E0 chunks-only (conventional
                          RAG, 20 chunks), E0 graph+all (10 chunks, empty graph), F/R/R2 c16 graph+all
  Table 2                 per-chunk: as Table 1; probe-check and per-entity: *_cap1 / *_pairs
                          metrics_t0_extra.jsonl (per-entity re-scored with a binary that reads per-pair marks);
                          forgot counts from <run>/snapshots/F_c16.dream.json chunks_processed_total;
                          probe-check cost: plans/DreamRAG-Forgetting-plan.md, amendment 7
  Table 3, fact audit     {NIST,Legal}_p10_b40000_pairs/fact_audit/report.json
  F out of material       plans/DreamRAG-Forgetting-plan.md, amendment 5
Private corpora are never named here, and their results stay out of this text.
References added 2026-11-25 (Edge, Grattafiori, Kumaran, Laurer, Nussbaum, Packer, Robertson,
Tononi) were written from memory: check each before submission.
-->

## Abstract

Biological memory has limited capacity: it consolidates what matters and forgets raw episodes, so
it can keep learning. Retrieval-augmented generation (RAG) keeps every chunk. We simulate
consolidation and forgetting in RAG: chunks are short-term memory and an entity graph is long-term
memory. An offline "dream" loop moves facts from chunks into the graph, and the chunks it has
consolidated are then forgotten. The cost of forgetting depends on the corpus. On Supreme Court opinions,
which share one structure, loss stayed below proportional: two-thirds of memory cost a fifth of
recall. On AI-security guidance in mixed formats it hit a cliff, where the next 9% of memory cost
30% of recall, and Manhattan Project documents lost a third of their recall in their first tenth. Rules that check the graph before forgetting avoid the loss but forget
almost nothing. A fact-level audit locates the limit in the long-term representation, not the rule:
dreaming copies facts into the graph, but rarely onto the cards of the entities they describe.

## 1 Introduction & Motivation

A finite memory that keeps learning must forget. In biological cognition, the hippocampus stores
episodes, the neocortex gradually extracts their regularities, and the episode fades (McClelland et
al. 1995; Kumaran et al. 2016; Richards and Frankland 2017); sleep is when much of this
consolidation and pruning is thought to happen (Tononi and Cirelli 2014). RAG stores only grow
(Lewis et al. 2020). Graph RAG builds its graph once, at ingestion (Edge et al. 2024; Gutiérrez et
al. 2024). Systems that do forget act on conversation history or agent memory (Packer et al. 2023;
Zhong et al. 2024; Shinde 2026), not on the documents a retrieval store was built from.

The earlier DreamRAG loop moved knowledge into the graph, but the chunks still answered the
questions best (Vriens et al. 2026). If consolidation does not raise recall, its payoff has to lie
elsewhere: in what it lets a system forget, and at what cost in recall. We build that test. The
dream loop consolidates facts from chunks into an entity graph, a forgetting rule then withholds the
chunks it judges consolidated, and we measure recall as memory shrinks: the forgetting curve of a
retrieval store (Ebbinghaus 1885).

**Contributions:** (i) a RAG simulation of consolidation and forgetting, with three forgetting rules
and a control arm that separates the cost of a rule from that of dreaming with less, scored as a
forgetting curve; (ii) on three corpora, a curve whose shape depends on the corpus: below
proportional, a cliff, or an early drop; (iii) a fact-level audit of what the graph holds from a
chunk, which locates the limit in the long-term store rather than in the forgetting rule.

## 2 Methodology

![**Figure 1.** Data flow of one run. The forgetting rule (highlighted) turns the chunks dreaming has
read into forgotten chunks, which arm F also hides from its own dreaming. Each snapshot is scored
from 20 cards and 10 remembered chunks.](figures/fig0_method.png){width=100%}

**Memories and dreaming.** Short-term memory is chunks of at most 800 characters, searched by BM25
(Robertson and Zaragoza 2009) and dense embeddings (nomic-embed-text; Nussbaum et al. 2024).
Long-term memory is an entity graph of text cards, built at cycle 0 by llama3.1:8b (Grattafiori et
al. 2024) at temperature 0 from each corpus's curated ontology. Each dream cycle rewrites cards, and
proposes relations, from up to 20 evidence chunks per entity, keeping a write only if it passes an
improvement gate. A cycle has a fixed budget of 40,000 tokens whatever the corpus size, as sleep
does not lengthen after a busier day; we run 16 cycles and snapshot after cycles 1, 2, 4, 8 and 16.

**Forgetting.** Forgotten chunks are withheld, not deleted. The *per-chunk* rule forgets a chunk once
any accepted completion has read it; the *probe-check* rule only if the graph alone answers three
probes the model writes from it, checked by code (every number, half the content words); the
*per-entity* rule only once every entity the chunk mentions has been completed from it. Arm F hides
forgotten chunks from its own dreaming as well as from retrieval; arm R dreams over every chunk but
records the same marks, and R2 repeats R. If R traces F, the loss comes from what the rule forgets,
not from dreaming with less. Two references need no dreaming: the cycle-0 graph with every chunk,
and conventional RAG, the 20 best chunks with no graph.

**Corpora and measures.** Three public corpora.
**Legal**: 13 U.S. Supreme Court opinions (1803–2010), all in one consistent court-opinion
structure. **NIST**: 14 AI-security documents, mostly NIST publications with OWASP guides and agency
information sheets, in differing formats (standards, checklists, a taxonomy, guides, notes).
**Manhattan**: 12 Manhattan Project documents, also in differing formats (letters, a petition,
reports, meeting summaries, eyewitness accounts). Each run uses the first 10% of every document,
plus one run on all of Legal. Each
question's gold facts (nuggets) are tied to their supporting chunks, adjudicated and frozen before the
runs. At each snapshot llama3.1:8b answers at temperature 0 from the 20 best cards and the 10 best
remembered chunks. *Recall* is the share of gold facts the answer entails (DeBERTa-v3 NLI ≥ 0.5;
Laurer et al. 2024); *memory* is the searchable text of cards and remembered chunks; both are
reported as drops from cycle 0. Figure 2 pools F, R and R2 into 10%-wide bins of memory drop, with
95% bootstrap intervals.

## 3 Key Results & Empirical Validation

**Setup and noise.** Table 1 lists the four configurations. Re-evaluating one graph moved recall by
at most 0.05 (temperature 0, fixed seed, exact vector search); repeat dream runs moved it by up to
0.31, so differences between R and R2 are dream-to-dream variation.

**Table 1.** Configurations and the end of each run under the per-chunk rule (cycle 16). Cards: the
graph's share of memory at cycle 0. Forgot: chunks withheld. Drops are relative to cycle 0.

| Corpus | Chunks | Qs | Recall c0 | Cards | F forgot | F memory ↓ | F recall ↓ | R memory ↓ | R recall ↓ |
|---|---|---|---|---|---|---|---|---|---|
| Legal 10% | 377 | 19 | 0.81 | 31% | 99% | 66% | 13% | 48% | 20% |
| NIST 10% | 399 | 19 | 0.84 | 51% | 93% | 49% | 67% | 33% | 51% |
| Manhattan 10% | 57 | 10 | 0.76 | 53% | 82% | 25% | 43% | 9% | 34% |
| Legal 100% | 3,723 | 20 | 0.72 | 25% | 30% | 26% | 13% | 20% | 8% |

**Baselines.** Without forgetting, the graph does not beat conventional RAG. The 20 best chunks
with no graph scored between 0.09 below and 0.12 above the cycle-0 system (20 cards with the 10
best chunks): lower on Legal 10% and Manhattan, level on NIST, and higher on Legal 100%. The cards do add to a half-size chunk context: with the graph emptied, the same
10 chunks lost 0.11–0.16 of recall in three configurations, though not on NIST (+0.04).
After 16 cycles with every chunk still remembered, recall had moved by −0.13 to +0.04 against
cycle 0, with no consistent direction and inside the run-to-run variation. Dreaming neither raised
nor degraded recall; what it changes is what can be forgotten.

![**Figure 2.** Recall drop against memory drop over 16 dream cycles, arms F, R and R2 pooled and
binned by memory drop (bin means, 95% bootstrap intervals). Dashed: proportional loss. Dotted:
conventional RAG, 20 chunks and no graph. Hollow, left of 0: each arm at cycle 16 with nothing
forgotten.](figures/fig1_binned_t0.png){width=100%}

**Three curve shapes.** Legal stayed below the diagonal: a 66% memory drop cost 21% of recall, and
on Legal 100% 26% cost 13% (a single point, not drawn). With arms pooled into memory bins, the first fifth of
memory cost 0.4 of its share of recall for Legal and 0.5 for NIST. NIST then hit a cliff: the next 9% of memory cost 30% of recall, 3.3 times its share (CI 2.6–4.3), and
loss stayed roughly proportional after it. Manhattan lost 32% of its recall in the
first 10% of memory, then little more.

**The forgetting rule, not dreaming with less.** R traced F's curve (Table 1): on NIST R lost 51% of recall at a 33% memory drop without hiding a chunk from its dreams, so the loss comes from
what the per-chunk rule releases. F had forgotten almost every chunk it could by cycle 8 (cycle 4 on Manhattan) and left most of its later budget unspent, which is why its curves flatten.

**Long-term memory is not compact.** The graph's cards already made up 25–53% of memory at cycle 0
(Table 1), so forgetting every chunk could cut memory by at most 47–75%. Dreaming then grew the
cards: on Manhattan F's cards grew 26% and R's 61%, and R's memory drop *shrank* from
26% at cycle 1 to 9% at cycle 16 even as it released more chunks.

**Table 2.** Forgetting rules, 10% slices, arm F at cycle 16: chunks forgotten and recall from graph
plus remembered chunks (cycle 0: 0.81 Legal, 0.84 NIST). Each rule is a separate dream run, so
differences of a few points between rules that forget almost nothing are dream-run variation.

| Rule | Legal forgot | Legal recall | NIST forgot | NIST recall |
|---|---|---|---|---|
| per-chunk | 373 / 377 | 0.71 | 370 / 399 | 0.28 |
| probe-check | 5 / 377 | 0.76 | 7 / 399 | 0.85 |
| per-entity | 9 / 377 | 0.83 | 1 / 399 | 0.88 |

**Rules that check the graph forget almost nothing.** The probe-check rule removed NIST's loss, but it also refused Legal, where forgetting had cost little: the graph answered
only 15–25% of probes in either corpus, so the check does not tell the corpora apart. It also cost
35–107k tokens per cycle on top of the 40k dreaming budget. The per-entity rule forgot 9 Legal chunks
and one NIST chunk; in NIST a chunk mentions a median of 12 entities, and by cycle 16 F had been
completed from a median 23% of them.

**What the graph holds of a chunk.** An LLM split 100 chunks per corpus (seed 0) into at most 12
atomic facts each (median 6), with subject, object and the ontology predicate that fits, or none; a
fact is held when one card contains every number and half its content words (Table 3, per-entity runs).

**Table 3.** Fact audit, 100 chunks per corpus. Held: share of facts a card holds. Named card: held
on the card of an entity the fact names. Reached, not held: facts whose entities dreaming had read
from the chunk, yet no card holds.

| | Legal | NIST |
|---|---|---|
| Facts (relational share) | 623 (51%) | 572 (60%) |
| Held: cycle 0 → R c16 → F c16 | 0.26 → 0.27 → 0.47 | 0.45 → 0.53 → 0.61 |
| Held on a named card, F c16 | 0.19 | 0.28 |
| Held facts copied verbatim, F c16 | 39% | 48% |
| Chunks with ≥ 80% of facts held, F c16 | 20 | 29 |
| Reached, not held: relate no two entities | 60% | 44% |
| Reached, not held: fit no predicate | 26% | 20% |

Dreaming under forgetting kept more facts, since withholding read chunks sends completions to unread
ones, but mostly in the wrong place: only 0.19 and 0.28 sat on the card of an entity they name, the
card retrieval would show, and up to half were chunk sentences copied onto other entities' cards. Of
facts read but not kept, most relate no two entities (requirements, conditions, numbers,
definitions) and a fifth to a quarter fit no declared predicate: an entity card has no place for
them. A fact-based rule could forget the 20–29 chunks per hundred whose facts are held, but only if
retrieval can find those facts where they were written.

## 4 Impact, Limitations & Conclusion

**Impact.** On-device AI must learn on fixed storage, and data minimisation asks systems to keep no
more raw personal text than they need. Both have to know what forgetting costs before deleting, and
the forgetting curve measures it. Our results say the cost is not a property of the method alone:
the same rule was nearly free on one corpus and steep on another, so a deployment would need to
measure its own curve before trusting any rule.

**Limitations.** Three corpora with 10–20 questions each, on 10% slices plus one corpus of 3,723
chunks, far from millions of documents; one dream run per arm (two for R) when repeat runs differed
by up to 0.31; an unvalidated lexical fact judge (a stricter NLI judge holds 0.31 and 0.46 under F);
no random-forgetting baseline; gold facts concentrated in document openings. Consolidation did not
raise recall, so the payoff is memory saved, measured as text, not storage or query compute. Why the curves differ between corpora is open: Legal, whose documents share one consistent
structure, stayed below proportional loss, while NIST and Manhattan, whose documents differ in
format, did not (Figure 2). Consistently structured text may benefit most from dreaming, but three
corpora cannot separate format from redundancy or from how questions spread over documents.

**Conclusion.** Forgetting raw chunks after consolidation can be cheap, but whether it is depends on
the corpus, and rules that make it safe today do so by forgetting almost nothing. The limit we find
is the long-term representation, not the rule: dreaming writes facts to the wrong cards and has no
place for facts that are not relations between entities. Next, we attach each fact to the entities
it names, give non-relational facts a place on the card, and stream documents into fixed storage,
where forgetting is forced rather than chosen.

## References

- Ebbinghaus, H. 1885. *Über das Gedächtnis.* Duncker & Humblot.
- Edge, D., et al. 2024. From local to global: A graph RAG approach to query-focused summarization. arXiv:2404.16130.
- Grattafiori, A., et al. 2024. The Llama 3 herd of models. arXiv:2407.21783.
- Gutiérrez, B. J., et al. 2024. HippoRAG. NeurIPS 2024.
- Kumaran, D., Hassabis, D., McClelland, J. L. 2016. What learning systems do intelligent agents need? *Trends Cogn. Sci.* 20(7), 512–534.
- Laurer, M., et al. 2024. Less annotating, more classifying. *Political Analysis* 32(1), 84–100.
- Lewis, P., et al. 2020. Retrieval-augmented generation. NeurIPS 2020.
- McClelland, J. L., et al. 1995. Complementary learning systems. *Psychol. Rev.* 102(3), 419–457.
- Nussbaum, Z., et al. 2024. Nomic Embed: Training a reproducible long context text embedder. arXiv:2402.01613.
- Packer, C., et al. 2023. MemGPT: Towards LLMs as operating systems. arXiv:2310.08560.
- Richards, B. A., Frankland, P. W. 2017. The persistence and transience of memory. *Neuron* 94(6).
- Robertson, S., Zaragoza, H. 2009. The probabilistic relevance framework: BM25 and beyond. *Found. Trends Inf. Retr.* 3(4), 333–389.
- Shinde, S. S. 2026. SCM: Sleep-consolidated memory. arXiv:2604.20943.
- Tononi, G., Cirelli, C. 2014. Sleep and the price of plasticity. *Neuron* 81(1), 12–34.
- Vriens, A., et al. 2026. DreamRAG: A dreaming RAG architecture. AIAS+ 2026.
- Zhong, W., et al. 2024. MemoryBank: Long-term memory for LLMs. AAAI 2024.
