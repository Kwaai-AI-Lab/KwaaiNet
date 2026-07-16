#!/usr/bin/env bash
# Re-embed vectors (SQLite backend) + eval for the metro-linux group.
# Does NOT touch the knowledge graph - chunk/graph data already migrated.
#
# KBs: WarPeace, CountryHistory, RFCs, DreamMem, DeepSea, Manhattan
#
# Usage: bash tests/kwaai-knowledge/reembed_pipeline_metrolinux.sh

set -uo pipefail
cd "$(dirname "$0")"

METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
RESULTS="results"
PROGRESS="reembed_metrolinux_progress.json"

mkdir -p "$RESULTS"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

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

for KB in WarPeace CountryHistory RFCs DreamMem DeepSea; do
  log "=== $KB : reembed-vectors ==="
  write_progress "$KB" "reembed" "running"
  TS=$(date +%Y%m%d_%H%M%S)
  if kwaainet rag reembed-vectors --kb "$KB" --embed-url "$METRO_LINUX" > "$RESULTS/reembed_${KB}_${TS}.log" 2>&1; then
    write_progress "$KB" "reembed" "done"
  else
    log "  !! reembed failed for $KB, see $RESULTS/reembed_${KB}_${TS}.log"
    write_progress "$KB" "reembed" "failed"
    continue
  fi

  log "=== $KB : eval ==="
  write_progress "$KB" "eval" "running"
  EVAL_TS=$(date +%Y%m%d_%H%M%S)
  if kwaainet rag eval \
      --questions "$KB/eval_questions.json" \
      --kb "$KB" \
      --inference-url "$METRO_LINUX" \
      --model llama3.1:8b \
      --top-k 5 \
      --mode vector \
      --output "$RESULTS/eval_${KB}_postmigration_${EVAL_TS}.md" \
      > "$RESULTS/eval_${KB}_postmigration_${EVAL_TS}.log" 2>&1; then
    write_progress "$KB" "eval" "done"
  else
    log "  !! eval failed for $KB, see $RESULTS/eval_${KB}_postmigration_${EVAL_TS}.log"
    write_progress "$KB" "eval" "failed"
  fi
done

write_progress "ALL" "complete" "done"
log "metro-linux group complete."
