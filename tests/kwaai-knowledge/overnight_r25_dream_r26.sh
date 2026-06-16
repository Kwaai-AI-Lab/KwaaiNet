#!/usr/bin/env bash
# Overnight pipeline: r25 (biographical expansion) → dream cycles → r26 eval
# Run after binary is built and codesigned.
set -euo pipefail

KB=D6
MODEL=llama3.1:8b
INFER=http://localhost:11434
QUESTIONS=/Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/d6_eval_questions.json
RESULTS=/Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/results
DREAM_CYCLES=5

echo "=== r25: biographical expansion eval ==="
R25="$RESULTS/eval_D6_r25_bio_t0_$(date +%Y%m%d_%H%M%S).md"
kwaainet rag eval --kb $KB --questions $QUESTIONS \
  --mode smart --summary-expansion --biographical-expansion \
  > "$R25" 2>&1
echo "r25 done: $R25"
grep "Overall recall" "$R25" || true

echo ""
echo "=== Dream cycles ($DREAM_CYCLES × 100 completions) ==="
for i in $(seq 1 $DREAM_CYCLES); do
  DREAM_LOG="$RESULTS/dream_D6_cycle${i}_$(date +%Y%m%d_%H%M%S).log"
  echo "  Cycle $i / $DREAM_CYCLES → $DREAM_LOG"
  kwaainet rag dream run --kb $KB \
    --model $MODEL \
    --inference-url $INFER \
    --no-relations \
    --dedup-threshold 0.99 \
    --max-completions 100 \
    --workers 4 \
    > "$DREAM_LOG" 2>&1
  # Check health after each cycle
  grep -E "health|Overall|completed|entities" "$DREAM_LOG" | tail -5 || true
  echo "  Cycle $i complete."
done

echo ""
echo "=== Graph score after dream ==="
kwaainet rag graph score --kb $KB 2>&1 | tail -10

echo ""
echo "=== r26: post-dream eval (smart + summary-expansion + biographical-expansion) ==="
R26="$RESULTS/eval_D6_r26_dream_t0_$(date +%Y%m%d_%H%M%S).md"
kwaainet rag eval --kb $KB --questions $QUESTIONS \
  --mode smart --summary-expansion --biographical-expansion \
  > "$R26" 2>&1
echo "r26 done: $R26"
grep "Overall recall" "$R26" || true

echo ""
echo "=== Overnight pipeline complete ==="
echo "r25: $R25"
echo "r26: $R26"
