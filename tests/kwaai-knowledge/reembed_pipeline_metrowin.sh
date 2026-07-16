#!/usr/bin/env bash
# Re-embed vectors (SQLite backend) + eval for the metro-win group.
# Does NOT touch the knowledge graph - chunk/graph data already migrated.
#
# KBs: Poems, MobyDick, Astrophysics, PythonDocs, Climate, Meetings, OSMDocs, NIST, Legal
#
# Usage: bash tests/kwaai-knowledge/reembed_pipeline_metrowin.sh

set -uo pipefail
cd "$(dirname "$0")"

METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
RESULTS="results"
PROGRESS="reembed_metrowin_progress.json"

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

for KB in Poems MobyDick Astrophysics PythonDocs Climate Meetings OSMDocs NIST Legal; do
  log "=== $KB : reembed-vectors ==="
  write_progress "$KB" "reembed" "running"
  TS=$(date +%Y%m%d_%H%M%S)
  if kwaainet rag reembed-vectors --kb "$KB" --embed-url "$METRO_WIN" > "$RESULTS/reembed_${KB}_${TS}.log" 2>&1; then
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
      --inference-url "$METRO_WIN" \
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
log "metro-win group complete."
