#!/usr/bin/env bash
# Multi-corpus RAG eval harness.
# Runs kwaainet rag eval for each corpus KB and appends results to the summary.
#
# Usage:
#   ./multi_corpus_eval.sh                  # eval all corpora
#   ./multi_corpus_eval.sh Manhattan Legal  # eval specific corpora only
#
# Prereqs: KBs must already be ingested and graph-built.
# Results: tests/kwaai-knowledge/results/eval_<KB>_<timestamp>.md
#          tests/kwaai-knowledge/results/multi_corpus_summary.md

set -euo pipefail
cd "$(dirname "$0")"

RESULTS_DIR="results"
SUMMARY="$RESULTS_DIR/multi_corpus_summary.md"
mkdir -p "$RESULTS_DIR"

ALL_KBS=(
  D6
  ragbench
  Manhattan
  MobyDick
  Legal
  Meetings
  PythonDocs
  NIST
  Climate
  RFCs
  DeepSea
  DreamMem
  Astrophysics
  CountryHistory
  WarPeace
  Poems
  OSMDocs
)

# If args provided, use them; otherwise run all
if [[ $# -gt 0 ]]; then
  TARGET_KBS=("$@")
else
  TARGET_KBS=("${ALL_KBS[@]}")
fi

# Initialise summary header if missing
if [[ ! -f "$SUMMARY" ]]; then
  cat > "$SUMMARY" <<'HEADER'
# Multi-Corpus RAG Eval Summary

Baseline: D6 = 88.9% (v0.4.148 seed6)

| KB | Score | Questions | Chunk strategy | Entity types | Date | Notes |
|----|-------|-----------|----------------|--------------|------|-------|
| D6 | 88.9% | 36 | paragraph/100w | Person,Place,Org,Legislation,Publication | 2026-07-02 | Baseline, seeded family tree |
HEADER
fi

for KB in "${TARGET_KBS[@]}"; do
  # Determine question file location
  case "$KB" in
    D6)      QF="d6_eval_questions.json" ;;
    ragbench) QF="rag-bench/eval_questions.json" ;;
    *)       QF="${KB}/eval_questions.json" ;;
  esac

  if [[ ! -f "$QF" ]]; then
    echo "⚠️  $KB: question file not found ($QF), skipping"
    continue
  fi

  TS=$(date +%Y%m%d_%H%M%S)
  LOG="$RESULTS_DIR/eval_${KB}_${TS}.md"

  echo "▶ Evaluating $KB …"
  set +e
  kwaainet rag eval --kb "$KB" --questions "$QF" 2>&1 | tee "$LOG"
  EXIT=$?
  set -e

  if [[ $EXIT -ne 0 ]]; then
    echo "⚠️  $KB eval exited with code $EXIT"
    continue
  fi

  # Extract score from log (line like "  Score: 22/36 (61.1%)")
  SCORE=$(grep -oE '[0-9]+\.[0-9]+%' "$LOG" | tail -1 || echo "?%")
  TOTAL=$(grep -oE '[0-9]+/[0-9]+' "$LOG" | tail -1 || echo "?/?")

  echo "| $KB | $SCORE | $TOTAL | paragraph | — | $(date +%Y-%m-%d) | |" >> "$SUMMARY"
  echo "✅ $KB: $SCORE → $LOG"
done

echo ""
echo "Summary: $SUMMARY"
