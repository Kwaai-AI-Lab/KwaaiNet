# Eval v2 pre-registration

Committed before any dream-trajectory evaluation (cycle ≥ 1) had finished. The git commit time is
the evidence. Everything below is fixed for the confirmatory analysis. Anything decided later is
labelled **exploratory** in the paper.

Plan: `projects/kwaai-knowledge/plans/DreamRAG-Eval2-plan.md`. Its deviations log records every
change made before this file was committed.

## Question

Does offline dream consolidation, which raises a knowledge graph's structural completeness, make
retrieval-augmented answers better?

## Design (fixed)

- **Corpora.** Primary: Manhattan, DeepSea, Legal and Climate, 20 questions each, 80 in total.
  Secondary: D6 (a memoir, 40 questions). D6 is reported separately as a development set and is
  never sent to the API.
- **Cycle 0.** A fresh graph build per corpus (`graph build --reset-graph --no-relations
  --graph-window 1`, llama3.1:8b, the per-KB entity types). Chunks are unchanged.
- **Arms.** Both arms start from the same cycle-0 snapshot.
  - **A:** `rag dream run --no-relations`.
  - **B:** `rag dream run` with relations on.
  - Both use `--max-completions 200` and llama3.1:8b, with the default thresholds.
- **Snapshots evaluated.** Cycles 0, 1, 2, 4, 8 and 12, per arm.
- **Evaluation.** Each eval runs `rag eval --mode iterative -k 20` (llama3.1:8b, temperature 0) on
  the original question files, pinned to one machine per corpus. Cycle 0 is evaluated three times
  (the noise floor) and once with `--mode vector` (graph-free baseline).
- **Completeness.** `rag graph score --json` Overall, plus the type, summary and relation pillars.

## Gold

- NotebookLM reference answers from the QA-Tracker files, split into nuggets by Claude
  (`gold/nuggetize.py`, prompt `nuggetize-v1`).
- Each nugget is verified against the corpus by NLI (`gold/verify.py`): verified if entailment is
  at least 0.90, with passages at 0.70 or above recorded as gold passages. Borderline and
  unsupported nuggets go to Claude adjudication and then author review.
- Nuggets still unsupported after review are **unreachable** and are excluded from the primary
  endpoint. Their count is reported.
- The gold is frozen (`work/gold/<KB>_gold.json`) before the trajectory analysis runs.

## Endpoints

**Primary: context nugget coverage.** For each question, the fraction of its reachable nuggets
entailed by at least one passage that reached the prompt (`in_prompt`). This is NLI
`MoritzLaurer/DeBERTa-v3-large-mnli-fever-anli-ling-wanli` over markup-cleaned passages, with
350-token windows and a stride of 128, maximised over passages. A nugget counts as entailed when its
probability is at least θ.
- θ = 0.5, fixed now.
- A calibration against about 150 author-labelled pairs is reported. If it shows θ = 0.5 is badly
  off, the calibrated θ is reported alongside as a sensitivity result. The primary result stays at
  θ = 0.5.

**Key secondary endpoints** (Holm-corrected together with the primary):
1. **Answer nugget recall.** The fraction of reachable nuggets that the generated answer entails
   (NLI, θ = 0.5).
2. **Claude-graded nDCG@20.** Grades 0/1/2 from `judge/relevance.py` (`claude-opus-5`, prompt
   `relevance-v1`). The ideal ranking uses the pooled judged set per question.

**Other secondary endpoints** (descriptive):
- faithfulness (answer sentences entailed by the prompt's passages);
- contradiction rate;
- P@5 and P@20;
- gold-passage recall@20;
- retrieved-set Jaccard against cycle 0;
- the legacy keyword recall.

## Hypotheses

- **H1 (arm B).** The mean per-question change in coverage from cycle 0 to cycle 12 is greater than
  0. One-sided paired test on 80 questions: Wilcoxon signed-rank, α = 0.05, Holm-adjusted.
- **H2 (dose-response, each arm).** The mixed model `Y_qs = β·ΔC_s + u_q + γ_corpus + ε`, where Y is
  coverage, ΔC is the change in completeness from the corpus's own cycle 0 in percentage points, u_q
  is a random intercept per question and γ is a corpus fixed effect. We report β with a 95% CI from
  2,000 bootstrap resamples of questions within corpus. A sensitivity analysis uses ΔC restricted to
  the type and summary pillars.
- **H3 (arm A, equivalence).** TOST on the mean per-question change in coverage from cycle 0 to
  cycle 12, with bounds ±δ.
  - δ = max(4 coverage points, 2 × the test–retest SD of per-question coverage measured from the
    three cycle-0 repeats).
  - δ is computed from the noise-floor data, before the trajectory data are analysed.

## Analysis rules

- The analysis code is developed on the noise-floor runs (cycle-0 repeats) only, then run once on
  the trajectories.
- Units:
  - The unit of analysis is the question, paired across snapshots.
  - The paper states that there are only about ten trajectories.
  - Per-corpus Spearman values are descriptive only.
- Exclusions:
  - A question with no reachable nugget is excluded from nugget endpoints.
  - An eval that did not complete all its questions is re-run, not partially used.
- **Minimum viable result (if compute runs short):** Manhattan and Legal on cycles {0, 2, 8}, both
  arms, with the noise floor, the primary endpoint and answer recall, plus H1–H3 on that subset.
  This is labelled as the reduced set.

## Known limitations, stated in advance

- There is one cycle-0 build per corpus, so the variance of graph construction is not measured.
- The completeness score uses compiled per-type expectations, not the per-KB ontology.
- Two corpora were excluded for data defects:
  - PythonDocs, which is mostly HTML markup;
  - DreamMem, whose tracker has misaligned answers.
- D6 has been tuned against its own questions throughout development.
