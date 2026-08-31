#!/usr/bin/env bash
# Overnight pass 2: further dream-cycle enrichment + timeline extraction for
# the 11 corpora rebuilt/dreamed/evaluated this cycle.
#
# Per KB: N more dream cycles (score after each, same --no-relations
# convention as the original rebuild pipeline to avoid spurious co-mention
# relations on graphs that may carry seeded family relations), then
# `graph timeline build` to populate lifeline/interaction event data.
#
# Usage:
#   ./overnight_dream_timeline.sh                  # all 11 corpora
#   ./overnight_dream_timeline.sh Manhattan Legal   # specific corpora only

set -uo pipefail
cd "$(dirname "$0")"

P2P_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
RESULTS="results"
PROGRESS="overnight_dream_timeline_progress.json"
DREAM_CYCLES=5
MODEL="llama3.1:8b"

ALL_KBS="Manhattan MobyDick Legal Meetings PythonDocs NIST Climate RFCs DeepSea DreamMem Astrophysics"

mkdir -p "$RESULTS"

if [[ $# -gt 0 ]]; then
  TARGET_KBS="$*"
else
  TARGET_KBS="$ALL_KBS"
fi

log() { echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"; }

write_progress() {
  local KB="$1" PHASE="$2" STATUS="$3"
  python3 - "$KB" "$PHASE" "$STATUS" "$PROGRESS" <<'PY'
import json, sys, os
kb, phase, status, path = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
data = json.load(open(path)) if os.path.exists(path) else {"kbs": {}, "current_kb": "", "current_phase": "", "status": ""}
data["current_kb"] = kb
data["current_phase"] = phase
data["status"] = status
data["kbs"].setdefault(kb, {})
data["kbs"][kb]["phase"] = phase
data["kbs"][kb]["status"] = status
json.dump(data, open(path, "w"), indent=2)
PY
}

for KB in $TARGET_KBS; do
  LOG="$RESULTS/overnight_dream_timeline_${KB}.log"
  log "=========================================="
  log "KB: $KB"
  log "=========================================="
  : > "$LOG"

  for i in $(seq 1 "$DREAM_CYCLES"); do
    log "-- dream cycle $i/$DREAM_CYCLES --"
    write_progress "$KB" "dream-$i" "running"
    kwaainet rag dream run --kb "$KB" \
      --inference-urls "$P2P_URLS" \
      --model "$MODEL" \
      --no-relations \
      --max-completions 200 \
      --workers 4 \
      >> "$LOG" 2>&1
    write_progress "$KB" "dream-$i" "done"
    kwaainet rag graph score --kb "$KB" >> "$LOG" 2>&1
  done

  log "-- timeline build --"
  write_progress "$KB" "timeline-build" "running"
  kwaainet rag graph timeline build --kb "$KB" \
    --inference-urls "$P2P_URLS" \
    --model "$MODEL" \
    --workers 4 \
    >> "$LOG" 2>&1
  write_progress "$KB" "timeline-build" "done"

  log "-- timeline stats --"
  kwaainet rag graph timeline stats --kb "$KB" >> "$LOG" 2>&1

  write_progress "$KB" "complete" "done"
  log "$KB: dream cycles + timeline complete (log: $LOG)"
done

python3 - "$PROGRESS" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path))
data["current_kb"] = "ALL"
data["status"] = "complete"
json.dump(data, open(path, "w"), indent=2)
PY

log "All requested corpora: dream cycles + timeline events complete."
