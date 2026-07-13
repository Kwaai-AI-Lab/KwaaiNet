#!/usr/bin/env bash
# Rebuild KBs that are on redb (pre-SQLite-migration stores).
# The new binary (v0.5.1+) ignores .redb files and creates fresh .db files.
#
# KBs: Manhattan, Meetings, Climate, Legal, NIST, PythonDocs, RFCs, MobyDick
# Order: shortest to longest build time.
#
# Usage: bash tests/kwaai-knowledge/redb_migration_pipeline.sh

set -euo pipefail
cd "$(dirname "$0")"

METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
INFERENCE_URLS="${METRO_LINUX},${METRO_WIN}"
MODEL="llama3.1:8b"
CORPUS_ROOT="/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus"
RESULTS="results"
PROGRESS="redb_migration_progress.json"

mkdir -p "$RESULTS"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

write_progress() {
  local KB="$1" PHASE="$2" STATUS="$3" SCORE="${4:-}"
  python3 - "$KB" "$PHASE" "$STATUS" "$SCORE" "$PROGRESS" <<'PY'
import json, sys, os
kb, phase, status, score, path = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5]
data = json.load(open(path)) if os.path.exists(path) else {"corpora": {}, "current_kb": "", "current_phase": "", "status": ""}
data["current_kb"] = kb
data["current_phase"] = phase
data["status"] = status
if kb not in data["corpora"]:
    data["corpora"][kb] = {}
data["corpora"][kb]["phase"] = phase
data["corpora"][kb]["status"] = status
if score:
    data["corpora"][kb]["score"] = score
json.dump(data, open(path, "w"), indent=2)
PY
}

get_docs_dir() {
  local KB="$1"
  case "$KB" in
    Manhattan)   echo "$CORPUS_ROOT/Manhattan Project/documents" ;;
    Meetings)    echo "$CORPUS_ROOT/Meeting Transcripts/documents" ;;
    Climate)     echo "$CORPUS_ROOT/Climate Science/documents" ;;
    Legal)       echo "$CORPUS_ROOT/Legal Documents/documents" ;;
    NIST)        echo "$CORPUS_ROOT/documents" ;;
    PythonDocs)  echo "$CORPUS_ROOT/Python Documentation/documents/pythondocs" ;;
    RFCs)        echo "$CORPUS_ROOT/Internet Standards (RFCs)/documents" ;;
    MobyDick)    echo "$CORPUS_ROOT/Moby-Dick and companion works/documents" ;;
    *) echo "" ;;
  esac
}

get_entity_types() {
  local KB="$1"
  case "$KB" in
    Manhattan)   echo "Person,Place,Organization" ;;
    Meetings)    echo "Person,Organization" ;;
    Climate)     echo "Person,Place,Organization,Publication" ;;
    Legal)       echo "Person,Place,Organization,Legislation,Publication" ;;
    NIST)        echo "Organization,Publication,Legislation" ;;
    PythonDocs)  echo "Organization,Publication" ;;
    RFCs)        echo "Organization,Publication,Legislation" ;;
    MobyDick)    echo "Person,Place" ;;
    *)           echo "Person,Place,Organization" ;;
  esac
}

for KB in Manhattan Meetings Climate Legal NIST PythonDocs RFCs MobyDick; do
  DOCS_DIR=$(get_docs_dir "$KB")
  ET=$(get_entity_types "$KB")
  QF="${KB}/eval_questions.json"
  TS=$(date +%Y%m%d_%H%M%S)
  LOG="$RESULTS/eval_${KB}_sqlite_${TS}.md"

  log "==============================="
  log "=== $KB ==="
  log "==============================="

  # --- INGEST ---
  if [[ -z "$DOCS_DIR" ]] || [[ ! -d "$DOCS_DIR" ]]; then
    log "⚠️  $KB: documents dir not found ($DOCS_DIR), skipping"
    write_progress "$KB" "ingest" "skipped"
    continue
  fi

  log "[$KB] Phase 1/3: ingest from $DOCS_DIR"
  write_progress "$KB" "ingest" "running"

  for f in "$DOCS_DIR"/*; do
    [[ -f "$f" ]] || continue
    EXT="${f##*.}"
    case "${EXT}" in
      pdf|PDF|txt|TXT|md|MD|html|HTML|htm|HTM|docx|DOCX|doc|DOC|rst|RST|vtt|VTT) ;;
      *) log "  skip: $(basename "$f")"; continue ;;
    esac
    log "  → $(basename "$f")"
    set +e
    OUT=$(kwaainet rag ingest --kb "$KB" --chunk-strategy paragraph "$f" 2>&1)
    EXIT=$?
    set -e
    echo "$OUT" | grep -E "✓|Error|chunks|ingested" || echo "$OUT" | tail -2
  done

  write_progress "$KB" "ingest" "done"
  log "[$KB] Ingest done"

  # --- GRAPH BUILD ---
  log "[$KB] Phase 2/3: graph build (entity-types=$ET)"
  write_progress "$KB" "graph_build" "running"
  set +e
  kwaainet rag graph build --kb "$KB" \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers 4 \
    --entity-types "$ET" \
    --no-relations \
    --graph-window 1 \
    --timeline \
    --reset-graph \
    2>&1 | tee "/tmp/graph_build_${KB}.log" | tail -10
  BUILD_EXIT=$?
  set -e
  if [[ $BUILD_EXIT -ne 0 ]]; then
    log "⚠️  $KB: graph build exited $BUILD_EXIT — continuing to eval anyway"
  fi
  set +e
  kwaainet rag graph score --kb "$KB" 2>&1 | tail -5
  set -e
  write_progress "$KB" "graph_build" "done"
  log "[$KB] Graph build done"

  # --- EVAL ---
  if [[ ! -f "$QF" ]]; then
    log "⚠️  $KB: question file not found ($QF), skipping eval"
    write_progress "$KB" "eval" "skipped"
    continue
  fi

  log "[$KB] Phase 3/3: eval (--semantic-score)"
  write_progress "$KB" "eval" "running"
  set +e
  kwaainet rag eval --kb "$KB" \
    --questions "$QF" \
    --inference-url "$METRO_LINUX" \
    --semantic-score \
    --semantic-low 0.3 \
    --semantic-high 0.85 \
    2>&1 | tee "$LOG"
  EVAL_EXIT=$?
  set -e

  SCORE=$(grep -oE 'Retrieval:[[:space:]]*[0-9.]+%|Overall:[[:space:]]*[0-9.]+%' "$LOG" 2>/dev/null | head -1 || echo "?")
  log "[$KB] Score: $SCORE (eval exit=$EVAL_EXIT)"
  write_progress "$KB" "eval" "done" "$SCORE"
  log "✅ $KB complete → $LOG"
done

write_progress "ALL" "complete" "done"
log "=== All redb-migration corpora complete ==="
