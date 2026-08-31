#!/usr/bin/env bash
# Reproducible A/B: pins sampling temperature and uses a single worker, removing
# both sources of run-to-run variance.
#
# Why: three ontology-arm builds over identical chunks varied by 22% in relation
# count, against a 32% difference between the arms being compared. Signal was
# barely above noise, so the comparison could not decide anything.
#
# Slower by design — budget roughly 2x the normal per-chunk cost.
#
#   REPS=3 N=60 ./repro_ab.sh
set -uo pipefail
cd "$(dirname "$0")"
K=/Users/rezarassool/Source/KwaaiNet/core/target/ci/kwaainet
N="${N:-60}"; REPS="${REPS:-3}"
export KWAAI_EXTRACTION_TEMPERATURE=0
for rep in $(seq 1 "$REPS"); do
  for arm in ctl narr; do
    KB=D6_$arm
    $K rag graph clear --kb "$KB" --yes >/dev/null 2>&1   # ontology is preserved
    echo "── rep $rep / $KB ($N chunks, temp=0, workers=1)"
    $K rag graph build --kb "$KB" --model llama3.1:8b \
       --inference-url http://localhost:11434 --limit "$N" --workers 1 \
       >"results/repro_${arm}_r${rep}.log" 2>&1
    $K rag graph stats --kb "$KB" 2>&1 | tail -2
  done
  python3 compare_ab.py D6_ctl D6_narr > "results/repro_compare_r${rep}.txt" 2>&1
done
echo REPRO_DONE
