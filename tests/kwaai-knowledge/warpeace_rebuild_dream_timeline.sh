#!/usr/bin/env bash
# One-shot rebuild + dream-cycle enrichment + timeline build for WarPeace.
# WarPeace was left on a legacy redb store (never migrated to SQLite) and was
# out of scope for the 11-corpus rebuild/dream/timeline pipeline run earlier.
# This does it in one pass on the current (post-UTF8-fix) binary:
#   destroy -> init -> ingest -> graph build -> score -> 5x(dream + score) -> timeline build -> timeline stats.

set -uo pipefail
cd "$(dirname "$0")"

KB="WarPeace"
DOCS_DIR="/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus/War and Peace/documents"
# Scoped to the actual War and Peace text only — the corpus directory also
# bundles unrelated companion titles (Anna Karenina, Les Misérables, Vanity
# Fair, etc.) that inflated the first pass to 37k+ chunks and a ~53hr ETA.
INGEST_FILES=("$DOCS_DIR/warandpeace.pdf")
ET="Person,Place,Organization"
P2P_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
RESULTS="results"
PROGRESS="warpeace_rebuild_dream_timeline_progress.json"
DREAM_CYCLES=5
MODEL="llama3.1:8b"

mkdir -p "$RESULTS"
LOG="$RESULTS/warpeace_rebuild_dream_timeline.log"
: > "$LOG"

log() { echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"; }

write_progress() {
  local PHASE="$1" STATUS="$2"
  python3 - "$PHASE" "$STATUS" "$PROGRESS" <<'PY'
import json, sys, os
phase, status, path = sys.argv[1], sys.argv[2], sys.argv[3]
data = json.load(open(path)) if os.path.exists(path) else {}
data["phase"] = phase
data["status"] = status
json.dump(data, open(path, "w"), indent=2)
PY
}

log "=========================================="
log "KB: $KB"
log "=========================================="

log "-- destroy (clears legacy redb store) --"
write_progress "destroy" "running"
kwaainet rag destroy --kb "$KB" -y >> "$LOG" 2>&1
write_progress "destroy" "done"

log "-- init --"
write_progress "init" "running"
kwaainet rag init --kb "$KB" >> "$LOG" 2>&1
write_progress "init" "done"

log "-- ingest --"
write_progress "ingest" "running"
for f in "${INGEST_FILES[@]}"; do
  [[ -f "$f" ]] || continue
  EXT="${f##*.}"
  EXT_LOWER=$(echo "$EXT" | tr '[:upper:]' '[:lower:]')
  case "$EXT_LOWER" in
    pdf|txt|md|html|htm|docx|doc|rst|vtt) ;;
    *) echo "  skip: $(basename "$f")" >> "$LOG"; continue ;;
  esac
  echo "  -> $(basename "$f")" >> "$LOG"
  if ! kwaainet rag ingest --kb "$KB" --chunk-strategy paragraph "$f" >> "$LOG" 2>&1; then
    echo "    retrying with latin-1 transcode …" >> "$LOG"
    TMP=$(mktemp /tmp/ingest_XXXXX."$EXT")
    python3 -c "
import sys
data = open(sys.argv[1], 'rb').read()
open(sys.argv[2], 'w', encoding='utf-8').write(data.decode('latin-1'))
" "$f" "$TMP"
    kwaainet rag ingest --kb "$KB" --chunk-strategy paragraph "$TMP" >> "$LOG" 2>&1 || true
    rm -f "$TMP"
  fi
done
write_progress "ingest" "done"

log "-- graph build (entity-types=$ET) --"
write_progress "graph-build" "running"
kwaainet rag graph build --kb "$KB" \
  --model "$MODEL" \
  --inference-urls "$P2P_URLS" \
  --workers 4 \
  --entity-types "$ET" \
  --no-relations \
  --graph-window 1 \
  >> "$LOG" 2>&1
write_progress "graph-build" "done"

log "-- initial score --"
kwaainet rag graph score --kb "$KB" >> "$LOG" 2>&1

for i in $(seq 1 "$DREAM_CYCLES"); do
  log "-- dream cycle $i/$DREAM_CYCLES --"
  write_progress "dream-$i" "running"
  kwaainet rag dream run --kb "$KB" \
    --inference-urls "$P2P_URLS" \
    --model "$MODEL" \
    --no-relations \
    --max-completions 200 \
    --workers 4 \
    >> "$LOG" 2>&1
  write_progress "dream-$i" "done"
  kwaainet rag graph score --kb "$KB" >> "$LOG" 2>&1
done

log "-- timeline build --"
write_progress "timeline-build" "running"
kwaainet rag graph timeline build --kb "$KB" \
  --inference-urls "$P2P_URLS" \
  --model "$MODEL" \
  --workers 4 \
  >> "$LOG" 2>&1
write_progress "timeline-build" "done"

log "-- timeline stats --"
kwaainet rag graph timeline stats --kb "$KB" >> "$LOG" 2>&1

write_progress "complete" "done"
log "$KB fully rebuilt, dreamed, and timelined (log: $LOG)"
