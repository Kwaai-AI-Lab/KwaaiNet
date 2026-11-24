---
title: "Forgetting to Remember: Measuring What Retrieval Memory Loses When It Forgets"
authors: "Annika Vriens, Christopher Mayfield, Jourdane Hamilton, Reza Rassool"
affiliation: "Kwaai, Los Angeles, USA · {annika, chris, jourdane, reza}@kwaai.org"
note: "Builds on Vriens et al., AIAS+ 2026"
---

<!--
AIAS+ 2-page extended abstract, following the AIAS+ template (2-page HARD limit, references
included; abstract ~100-120 words; sections 1-4 as below). Build:
  python3 projects/kwaai-knowledge/papers/aias-forgetting/build_docx.py
Figure 1: figures/gen_forgetting_curve.py <recall_per_size.csv>.
Number sources (feat/dream-forgetting worktree, tests/kwaai-knowledge/results/dream_forget/):
  recall, memory drop    _recall_per_size/recall_per_size.csv (runs *_p10_b40000{,_cap1,_pairs}, Legal_p100_b40000)
  Figure 1, recall      <run>/metrics_t0.jsonl (2026-11-23 temperature-0 re-score; figures/gen_binned_t0.py, binned_t0.csv)
  probe-check cost       plans/DreamRAG-Forgetting-plan.md, amendment 7
  fact audit             {NIST,Legal}_p10_b40000_pairs/fact_audit/{report.json,verdicts_*.jsonl}
Private corpora are never named here, and their results stay out of this text.
Items in ⟦double brackets⟧ must be resolved before submission.
-->

## Abstract

Biological memory has limited capacity: it consolidates what matters and forgets raw episodes, so
it can keep learning. Retrieval-augmented generation (RAG) keeps every passage. We simulate both in
RAG: passages are short-term memory, an entity graph long-term memory, and an offline "dream" loop
extracts facts from one into the other before passages are forgotten. The cost of forgetting depends on the corpus. Legal opinions
stayed below proportional loss: two-thirds of memory cost a fifth of recall. Security standards hit a
cliff, where the next 9% of memory cost 30% of recall, and historical documents lost a third of
recall in their first tenth. A fact-level audit traces the limit to the long-term representation,
not the forgetting rule.

## 1 Introduction & Motivation

A memory that keeps learning must forget: the hippocampus stores episodes, the neocortex extracts
their regularities, and the episode fades (McClelland et al. 1995; Richards and Frankland 2017). RAG
stores only grow (Lewis et al. 2020); graph RAG builds its graph once, at ingestion (Gutiérrez et al.
2024); and systems that do forget act on conversation history (Zhong et al. 2024; Shinde 2026). We ask
what recall forgetting costs. Our dream loop moved knowledge into the graph but the passages
still answered the questions (Vriens et al. 2026); its payoff should lie in what it lets a system forget. **Contributions:** (i) a RAG simulation of
consolidation and forgetting, scored as a forgetting curve (Ebbinghaus 1885); (ii) on three corpora, a
curve whose shape depends on the corpus: below proportional, a cliff, or an early drop; (iii) a fact-level audit locating the limit in the long-term store.

## 2 Methodology

Passages (≤ 800 characters, BM25 and dense indexes) are short-term memory; an entity graph built by
llama3.1:8b, each entity a text card, is long-term memory. Each offline dream cycle completes entities
from their evidence passages within 40,000 model tokens, for 16 cycles. Forgotten passages are
withheld, not deleted. The *per-chunk* rule forgets a passage once dreaming has read
it; the *probe-check* rule only if the graph answers three probes written from it; the *per-entity*
rule only once every entity it mentions has read it. Arm F also hides forgotten passages from dreaming;
arm R does not, which separates the cost of the rule from that of dreaming with less. We report drops from
cycle 0 in *memory* (searchable characters) and *recall* (gold facts a temperature-0 answer entails).

## 3 Key Results & Empirical Validation

**Setup.** Legal opinions (377 passages), AI security standards (399) and Manhattan Project
documents (57), the first 10% of each document, 19, 19 and 10 questions, plus the full legal
corpus (3,723). Repeat evaluations of a graph differed by ≤ 0.05 in recall; repeat dream runs by ≤ 0.31.

![**Figure 1.** Recall drop against memory drop over 16 dream cycles, arms F, R and R2 pooled and
binned by memory drop (bin means, 95% bootstrap intervals). Dashed: proportional loss.](figures/fig1_binned_t0.png){width=100%}

**Main findings.** Under the per-chunk rule, F forgot 99% and 93% of passages (66% and 49% of memory,
counting card growth). Pooling arms into memory bins, the first fifth of memory cost 0.4 (legal) and 0.5
(standards) of its share of recall. Legal opinions stayed below the diagonal: a 66% memory drop cost
21% of recall, and 26% cost 13% on the full corpus. The standards hit a cliff: the
next 9% of memory cost 30% of recall, 3.3 times its share (CI 2.6–4.3). The historical documents
lost 32% of recall in their first 10% of memory, then little more. Rules that check
the graph prevented the loss but forgot only 1–2% of passages.

**Insight.** R traced F's curve: the loss is in what the rule forgets. Atomic facts from 100 passages
per corpus (LLM-extracted, median 6) show dreaming under forgetting keeps more of them
(0.45 → 0.61 and 0.26 → 0.47; R: 0.53, 0.27), but only 0.28 and 0.19 on the card of
an entity they name. Of facts read but not kept, 44% and 60% relate no
two entities, and 20% and 26% fit no predicate.

## 4 Impact, Limitations & Conclusion

On-device AI must learn on fixed storage, and data minimisation asks systems to keep no more raw
personal text than needed; both must know what forgetting costs before deleting, which
the forgetting curve measures. Limitations: three corpora of 10–20 questions, one dream run per arm, an
unvalidated fact judge, no random-forgetting baseline. Next, we attach facts to the entities they name and stream documents into fixed storage: the
target is the long-term representation, not the rule.

## References

- Ebbinghaus, H. 1885. *Über das Gedächtnis.* Duncker & Humblot.
- Gutiérrez, B. J., et al. 2024. HippoRAG. NeurIPS 2024.
- Lewis, P., et al. 2020. Retrieval-augmented generation. NeurIPS 2020.
- McClelland, J. L., et al. 1995. Complementary learning systems. *Psychol. Rev.* 102(3), 419–457.
- Richards, B. A., Frankland, P. W. 2017. The persistence and transience of memory. *Neuron* 94(6).
- Shinde, S. S. 2026. SCM: Sleep-consolidated memory. arXiv:2604.20943.
- Vriens, A., et al. 2026. DreamRAG: A dreaming RAG architecture. AIAS+ 2026.
- Zhong, W., et al. 2024. MemoryBank: Long-term memory for LLMs. AAAI 2024.
