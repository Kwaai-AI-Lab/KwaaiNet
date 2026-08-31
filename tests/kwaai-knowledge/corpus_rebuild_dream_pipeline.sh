#!/usr/bin/env bash
# Full rebuild + dream-cycle pipeline for the 15 non-D6 benchmark corpora.
# Per KB, sequentially (avoid GPU overload): destroy -> init -> ingest -> graph build -> score -> N dream cycles -> score.
#
# Note: macOS ships bash 3.2 (no bash 4+ available on this machine), so this
# avoids `declare -A` associative arrays in favor of `case` lookups.
#
# Usage:
#   ./corpus_rebuild_dream_pipeline.sh                  # all 15 corpora
#   ./corpus_rebuild_dream_pipeline.sh Manhattan Legal   # specific corpora only

set -uo pipefail
cd "$(dirname "$0")"

CORPUS_ROOT="/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus"
# Third peer (12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd) dropped —
# confirmed unreachable (routing: not found, circuit breaker opened) and absent
# from the current shard-chain peer list.
# metro-win (12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE) dropped mid-run
# and stayed absent from the DHT peer list for about a week; a simultaneous
# metro-linux blip overlapping with metro-win's outage triggered a real bug (both
# peers circuit-broken at once → graph build silently discarded up to 94% of a
# corpus's chunks as false "successes"), now fixed in
# graph.rs::extract_request_with_retry (retries with backoff spanning the circuit
# breaker's cooldown, returns a real Err instead of a silent empty success once
# genuinely exhausted). metro-win is back online and confirmed stable — restoring
# both peers now that the underlying bug is fixed either way.
P2P_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
RESULTS="results"
PROGRESS="corpus_rebuild_dream_progress.json"
DREAM_CYCLES=5

ALL_KBS="Manhattan MobyDick Legal Meetings PythonDocs NIST Climate RFCs DeepSea DreamMem Astrophysics CountryHistory WarPeace Poems OSMDocs"

kb_docs_subpath() {
  case "$1" in
    Manhattan)      echo "Manhattan Project/documents" ;;
    MobyDick)       echo "Moby-Dick and companion works/documents" ;;
    Legal)          echo "Legal Documents/documents" ;;
    Meetings)       echo "Meeting Transcripts/documents" ;;
    PythonDocs)     echo "Python Documentation/documents/pythondocs" ;;
    NIST)           echo "documents" ;;
    Climate)        echo "Climate Science/documents" ;;
    RFCs)           echo "Internet Standards (RFCs)/documents" ;;
    DeepSea)        echo "Deep Sea Biology/documents" ;;
    DreamMem)       echo "Dream-Based Memory Consolidation and Forgetting/documents" ;;
    Astrophysics)   echo "Astrophysics - Space Exploration/documents" ;;
    CountryHistory) echo "Country History-Culture/documents" ;;
    WarPeace)       echo "War and Peace/documents" ;;
    Poems)          echo "Poems/documents" ;;
    OSMDocs)        echo "OpenStreetMap Data Documentation/documents" ;;
    *)              echo "" ;;
  esac
}

kb_entity_types() {
  case "$1" in
    Manhattan)      echo "Person,Place,Organization" ;;
    MobyDick)       echo "Person,Place,Organization" ;;
    Legal)          echo "Person,Organization,Legislation" ;;
    Meetings)       echo "Person,Organization" ;;
    PythonDocs)     echo "Organization,Publication" ;;
    NIST)           echo "Organization,Legislation,Publication" ;;
    Climate)        echo "Organization,Legislation,Publication" ;;
    RFCs)           echo "Organization,Publication" ;;
    DeepSea)        echo "Person,Organization,Publication" ;;
    DreamMem)       echo "Person,Organization,Publication" ;;
    Astrophysics)   echo "Person,Organization,Publication" ;;
    CountryHistory) echo "Person,Place,Organization" ;;
    WarPeace)       echo "Person,Place,Organization" ;;
    Poems)          echo "Person,Place" ;;
    OSMDocs)        echo "Organization" ;;
    *)              echo "Person,Place,Organization" ;;
  esac
}

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
  SUBPATH="$(kb_docs_subpath "$KB")"
  ET="$(kb_entity_types "$KB")"
  DOCS_DIR="$CORPUS_ROOT/$SUBPATH"
  LOG="$RESULTS/rebuild_dream_${KB}.log"

  log "=========================================="
  log "KB: $KB"
  log "=========================================="

  if [[ -z "$SUBPATH" || ! -d "$DOCS_DIR" ]]; then
    log "  !! documents dir not found ($DOCS_DIR), skipping $KB"
    write_progress "$KB" "skip" "no-docs"
    continue
  fi

  : > "$LOG"

  log "-- destroy --"
  write_progress "$KB" "destroy" "running"
  kwaainet rag destroy --kb "$KB" -y >> "$LOG" 2>&1
  write_progress "$KB" "destroy" "done"

  log "-- init --"
  write_progress "$KB" "init" "running"
  kwaainet rag init --kb "$KB" >> "$LOG" 2>&1
  write_progress "$KB" "init" "done"

  log "-- ingest --"
  write_progress "$KB" "ingest" "running"
  for f in "$DOCS_DIR"/*; do
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
  write_progress "$KB" "ingest" "done"

  log "-- graph build (entity-types=$ET) --"
  write_progress "$KB" "graph-build" "running"
  kwaainet rag graph build --kb "$KB" \
    --model llama3.1:8b \
    --inference-urls "$P2P_URLS" \
    --workers 4 \
    --entity-types "$ET" \
    --no-relations \
    --graph-window 1 \
    >> "$LOG" 2>&1
  write_progress "$KB" "graph-build" "done"

  log "-- initial score --"
  kwaainet rag graph score --kb "$KB" >> "$LOG" 2>&1

  for i in $(seq 1 "$DREAM_CYCLES"); do
    log "-- dream cycle $i/$DREAM_CYCLES --"
    write_progress "$KB" "dream-$i" "running"
    kwaainet rag dream run --kb "$KB" \
      --inference-urls "$P2P_URLS" \
      --model llama3.1:8b \
      --no-relations \
      --max-completions 200 \
      --workers 4 \
      >> "$LOG" 2>&1
    write_progress "$KB" "dream-$i" "done"
    kwaainet rag graph score --kb "$KB" >> "$LOG" 2>&1
  done

  write_progress "$KB" "complete" "done"
  log "$KB fully rebuilt + dreamed (log: $LOG)"
done

write_progress "ALL" "complete" "done"
log "All requested corpora rebuilt and dreamed."
