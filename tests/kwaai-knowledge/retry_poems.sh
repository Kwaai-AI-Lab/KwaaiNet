#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "$0")"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
RESULTS="results"

# Wait for the main metro-win pipeline (PID 12599) to finish so we don't
# send concurrent requests to metro-win.
while kill -0 12599 2>/dev/null; do
  sleep 30
done

echo "[$(date '+%H:%M:%S')] === Poems : reembed-vectors (retry) ==="
TS=$(date +%Y%m%d_%H%M%S)
if kwaainet rag reembed-vectors --kb Poems --embed-url "$METRO_WIN" > "$RESULTS/reembed_Poems_retry_${TS}.log" 2>&1; then
  echo "[$(date '+%H:%M:%S')] Poems reembed done"
  EVAL_TS=$(date +%Y%m%d_%H%M%S)
  kwaainet rag eval --questions Poems/eval_questions.json --kb Poems --inference-url "$METRO_WIN" \
    --model llama3.1:8b --top-k 5 --mode vector \
    --output "$RESULTS/eval_Poems_postmigration_${EVAL_TS}.md" \
    > "$RESULTS/eval_Poems_postmigration_${EVAL_TS}.log" 2>&1
  echo "[$(date '+%H:%M:%S')] Poems eval done"
else
  echo "[$(date '+%H:%M:%S')] !! Poems reembed retry failed too, see $RESULTS/reembed_Poems_retry_${TS}.log"
fi
