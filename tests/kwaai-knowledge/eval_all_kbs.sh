#!/bin/bash
# Cross-KB retrieval baseline: run every KB's own 20-question set and record recall.
#
# Why this and not more dream cycles: the overnight sweep added 1000+ relations
# and moved D6's recall not at all (89.5% against pre-sweep baselines of 88.5%
# and ~90.0%). Every relation dream extracts is `associated_with`, so the edges
# carry no information about *how* entities relate. Before spending more compute
# on enrichment, it is worth knowing where each corpus actually stands.
#
# Runs against local Ollama rather than p2p: this is measurement, not a soak
# test, and a 30s p2p unary ceiling (#134) would silently truncate long answers.
#
# Per-question detail lands in each KB's own report — always keep the per-question
# log, not just the headline number.
#
# Usage: eval_all_kbs.sh [kb ...]     (default: every KB with an eval set)

set -uo pipefail

K="${KWAAINET_BIN:-$HOME/.kwaainet/bin/kwaainet-dream}"
MODEL="${EVAL_MODEL:-llama3.1:8b}"
URL="${EVAL_URL:-http://localhost:11434}"
MODE="${EVAL_MODE:-iterative}"
QDIR="$HOME/Source/KwaaiNet/tests/kwaai-knowledge"
RESULTS="$QDIR/results"
STAMP=$(date +%Y%m%d_%H%M%S)
SUMMARY="$RESULTS/eval_all_kbs_$STAMP.md"
PROGRESS="$RESULTS/eval_all_kbs_progress.json"
mkdir -p "$RESULTS"

if [ $# -gt 0 ]; then KBS="$*"; else
    KBS=$(find "$QDIR" -name eval_questions.json -mindepth 2 -maxdepth 2 \
          | sed "s|$QDIR/||; s|/eval_questions.json||" | sort | tr '\n' ' ')
fi

started=$(python3 -c "import time;print(time.time())")
DONE=0; TOTAL=$(echo $KBS | wc -w | tr -d ' ')

{
    echo "# Cross-KB retrieval baseline — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo
    echo "Each KB evaluated against its own 20-question set. Model \`$MODEL\`, mode"
    echo "\`$MODE\`, local Ollama. Per-question detail is in the linked per-KB reports."
    echo
    echo "| kb | docs | entities | relations | graph score | recall | tokens | secs |"
    echo "|---|---|---|---|---|---|---|---|"
} > "$SUMMARY"

for kb in $KBS; do
    Q="$QDIR/$kb/eval_questions.json"
    [ -f "$Q" ] || { echo "| $kb | — | — | — | — | no question set | | |" >> "$SUMMARY"; continue; }

    docs=$($K rag docs --kb "$kb" 2>/dev/null | grep -c '•')
    read -r ents rels <<<"$($K rag graph stats --kb "$kb" 2>/dev/null \
        | awk '/Entities:/{e=$2} /Relations:/{r=$2} END{print e+0, r+0}')"
    score=$($K rag graph score --kb "$kb" 2>/dev/null | grep -oE 'Overall: +[0-9.]+%' | grep -oE '[0-9.]+')

    now=$(python3 -c "import time;print(time.time())")
    python3 -c "
import json
print(json.dumps({'kb':'$kb','done':$DONE,'total':$TOTAL,
                  'elapsed_s':round($now-$started,1),'summary':'$SUMMARY'}, indent=2))" > "$PROGRESS"

    report="$RESULTS/eval_${kb}_baseline_$STAMP.md"
    t0=$(python3 -c "import time;print(time.time())")
    $K rag eval --kb "$kb" --questions "$Q" --inference-url "$URL" \
        --model "$MODEL" --mode "$MODE" --output "$report" >/dev/null 2>&1
    t1=$(python3 -c "import time;print(time.time())")

    line=$(grep -oE 'Overall recall \(token-overlap\) \| [0-9.]+% \([0-9./]+\)' "$report" 2>/dev/null | head -1)
    recall=$(grep -oE '[0-9.]+%' <<<"$line" | head -1)
    toks=$(grep -oE '\([0-9./]+\)' <<<"$line" | head -1 | tr -d '()')

    printf '| %s | %s | %s | %s | %s%% | %s | %s | %s |\n' \
        "$kb" "$docs" "$ents" "$rels" "${score:--}" "${recall:-FAILED}" "${toks:--}" \
        "$(python3 -c "print(f'{$t1-$t0:.0f}')")" >> "$SUMMARY"
    DONE=$((DONE+1))
done

python3 -c "
import json
print(json.dumps({'kb':'-','done':$DONE,'total':$TOTAL,'phase':'finished',
                  'summary':'$SUMMARY'}, indent=2))" > "$PROGRESS"
echo "" >> "$SUMMARY"
echo "Completed $DONE of $TOTAL KBs." >> "$SUMMARY"
