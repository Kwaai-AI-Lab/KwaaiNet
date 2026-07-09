#!/usr/bin/env bash
# Ingest all corpus documents into their respective KBs.
# Run this before multi_corpus_graph_build.sh.
#
# Documents are expected at:
#   /Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus/<Corpus Name>/documents/
#
# Usage:
#   ./multi_corpus_ingest.sh                  # ingest all
#   ./multi_corpus_ingest.sh Manhattan Legal  # specific corpora only

set -euo pipefail
cd "$(dirname "$0")"

CORPUS_ROOT="/Volumes/WD2/Source/KwaaiNet/tests/rag-bench/Corpus"

declare -A KB_DOCS=(
  [Manhattan]="Manhattan Project/documents"
  [MobyDick]="Moby-Dick and companion works/documents"
  [Legal]="Legal Documents/documents"
  [Meetings]="Meeting Transcripts/documents"
  [PythonDocs]="Python Documentation/documents/pythondocs"
  [NIST]="documents"                  # NIST docs are in top-level documents/ folder
  [Climate]="Climate Science/documents"
  [RFCs]="Internet Standards (RFCs)/documents"
  [DeepSea]="Deep Sea Biology/documents"
  [DreamMem]="Dream-Based Memory Consolidation and Forgetting/documents"
  [Astrophysics]="Astrophysics - Space Exploration/documents"
  [CountryHistory]="Country History-Culture/documents"
  [WarPeace]="War and Peace/documents"
  [Poems]="Poems/documents"
  [OSMDocs]="OpenStreetMap Data Documentation/documents"
)

if [[ $# -gt 0 ]]; then
  TARGET_KBS=("$@")
else
  TARGET_KBS=("${!KB_DOCS[@]}")
fi

for KB in "${TARGET_KBS[@]}"; do
  DOCS_DIR="$CORPUS_ROOT/${KB_DOCS[$KB]}"
  if [[ ! -d "$DOCS_DIR" ]]; then
    echo "⚠️  $KB: documents directory not found ($DOCS_DIR), skipping"
    continue
  fi

  echo "▶ Ingesting $KB from $DOCS_DIR …"
  for f in "$DOCS_DIR"/*; do
    [[ -f "$f" ]] || continue
    EXT="${f##*.}"
    # Skip non-document files
    case "${EXT,,}" in
      pdf|txt|md|html|htm|docx|doc|rst|vtt) ;;
      *) echo "  skip: $(basename "$f")"; continue ;;
    esac
    echo "  → $(basename "$f")"
    # Try UTF-8 ingest; on encoding error, transcode via latin-1 and retry
    if ! kwaainet rag ingest --kb "$KB" --chunk-strategy paragraph "$f" 2>&1 \
         | grep -E "✓|Error|chunks"; then
      echo "    retrying with latin-1 transcode …"
      TMP=$(mktemp /tmp/ingest_XXXXX."$EXT")
      python3 -c "
import sys
data = open(sys.argv[1], 'rb').read()
open(sys.argv[2], 'w', encoding='utf-8').write(data.decode('latin-1'))
" "$f" "$TMP"
      kwaainet rag ingest --kb "$KB" --chunk-strategy paragraph "$TMP" \
        2>&1 | grep -E "✓|Error|chunks" || true
      rm -f "$TMP"
    fi
  done
  echo "✅ $KB ingested"
done
