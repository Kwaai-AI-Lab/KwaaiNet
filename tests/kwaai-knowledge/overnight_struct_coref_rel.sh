#!/usr/bin/env bash
# Overnight experiment: structure-aware ingestion + coref + relations + eval
# Experiment question:
#   How much do section-type boundaries + coreference + text-derived relations
#   improve recall vs the D6_person_full baseline (53.3%)?
#
# Usage: nohup bash overnight_struct_coref_rel.sh > overnight.log 2>&1 &

set -euo pipefail

REPO=/Users/rezarassool/Source/KwaaiNet
PDF="$REPO/docs/LEST WE FORGET -rev25.pdf"
DOC_SCHEMA="$REPO/tests/kwaai-knowledge/d6_doc_schema.yaml"
SEED_FILE="$REPO/tests/kwaai-knowledge/d6_family_tree.yaml"
EVAL_Q="$REPO/tests/kwaai-knowledge/d6_eval_questions.json"
RESULTS="$REPO/tests/kwaai-knowledge/results"
LOG_FILE="$REPO/tests/kwaai-knowledge/d6_experiments_log.md"
# Multi-machine resource allocation (per user):
#   metro-linux (A6000 48GB): 70b Q3_K_M for relation extraction
#   metro-win   (A6000 48GB): 8b for entity extraction (parallel with metro-linux)
#   local M4 Pro:             embedding only
METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
# Graph build uses both metro machines for parallel 8b extraction
BUILD_INFERENCE_URLS="${METRO_LINUX},${METRO_WIN}"
# Relation extraction uses metro-linux 70b Q3 only
RE_MODEL="llama3.1:70b-instruct-q3_K_M"
RE_INFERENCE_URL="$METRO_LINUX"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LABEL="D6_struct_coref_rel_${TIMESTAMP}"
OUTPUT_MD="$RESULTS/eval_${LABEL}.md"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

# ── Step 0: record baseline ─────────────────────────────────────────────────
log "Step 0: recording current graph state"
BEFORE_ENTITIES=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Entities | awk '{print $2}')
BEFORE_RELATIONS=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Relations | awk '{print $2}')
log "Before: $BEFORE_ENTITIES entities, $BEFORE_RELATIONS relations"

# ── Step 1: full rebuild with structure-aware schema ────────────────────────
log "Step 1: full rebuild (destroy → ingest → graph build → seed → dedup)"
log "  PDF:        $PDF"
log "  doc-schema: $DOC_SCHEMA"
log "  seed-file:  $SEED_FILE"
log "  entity-types: Person only (matches D6_person_full baseline)"
log "  no-relations: yes (CC+EC pass runs separately with 70b)"

kwaainet rag rebuild "$PDF" \
  --kb D6 \
  --doc-schema "$DOC_SCHEMA" \
  --seed-file "$SEED_FILE" \
  --chunk-strategy paragraph \
  --entity-types Person \
  --no-relations \
  --workers 8 \
  --model llama3.1:8b \
  --inference-urls "$BUILD_INFERENCE_URLS" \
  --yes

log "Step 1 complete"

# ── Step 2: coref pass (Tier 1 only, fast) ──────────────────────────────────
log "Step 2: coref pass (alias-match + gender-nearest, no LLM)"
kwaainet rag graph coref --kb D6 \
  --no-llm \
  --commit \
  --window 2 \
  --output "$RESULTS/coref_${LABEL}.md"
log "Step 2 complete"

# ── Step 3: relation extraction with --commit ────────────────────────────────
log "Step 3: CC+EC relation extraction (70b Q3 on metro A6000, --commit)"
kwaainet rag graph extract-relations --kb D6 \
  --inference-url "$RE_INFERENCE_URL" \
  --model "$RE_MODEL" \
  --sample 1.0 \
  --commit \
  --output "$RESULTS/extract_rel_${LABEL}.md"
log "Step 3 complete"

# ── Step 4: graph health + stats ─────────────────────────────────────────────
log "Step 4: graph stats after rebuild+coref+relations"
AFTER_ENTITIES=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Entities | awk '{print $2}')
AFTER_RELATIONS=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Relations | awk '{print $2}')
HEALTH=$(kwaainet rag graph score --kb D6 2>/dev/null | grep "Overall:" | awk '{print $2}')
log "After:  $AFTER_ENTITIES entities, $AFTER_RELATIONS relations, health=$HEALTH"

# ── Step 5: eval ─────────────────────────────────────────────────────────────
log "Step 5: full eval (40 questions, iterative mode)"
kwaainet rag eval \
  --questions "$EVAL_Q" \
  --kb D6 \
  --mode iterative \
  --output "$OUTPUT_MD"
log "Step 5 complete — results at $OUTPUT_MD"

# ── Step 6: extract key metrics and log ─────────────────────────────────────
log "Step 6: logging to $LOG_FILE"

RECALL=$(grep "Overall recall" "$OUTPUT_MD" 2>/dev/null | grep -oP '\d+\.\d+%' | head -1 || echo "?")
KW_SCORE=$(grep "Overall recall" "$OUTPUT_MD" 2>/dev/null | grep -oP '\d+/\d+' | head -1 || echo "?")

# Append to experiments log
cat >> "$LOG_FILE" << ENTRY

## $(date '+%Y-%m-%d') – $LABEL

- **Experiment:** Full rebuild with structure-aware ingestion + coref + CC/EC relations
- **Before:** $BEFORE_ENTITIES entities, $BEFORE_RELATIONS relations, **53.3%** recall (D6_person_full baseline 2026-06-04)
- **After:**  $AFTER_ENTITIES entities, $AFTER_RELATIONS relations, health=$HEALTH, **$RECALL** recall ($KW_SCORE)
- **Changes vs baseline:**
  - SectionType boundaries in chunk packing, context windows, coref adjacency, CC/EC windows
  - Coref pass (Tier 1: alias-match + gender-nearest, --no-llm, ±2 window)
  - CC+EC relation extraction committed (70b Q3 on metro A6000, --commit)
- **Eval output:** $OUTPUT_MD
- **Coref output:** $RESULTS/coref_${LABEL}.md
- **Relation output:** $RESULTS/extract_rel_${LABEL}.md

### Key delta questions
\`\`\`
$(grep -E "q09|q12|q24|q26|q32|q38|Overall" "$OUTPUT_MD" 2>/dev/null | grep "|" | head -10 || echo "see $OUTPUT_MD")
\`\`\`
ENTRY

log "Done. Experiment $LABEL complete."
log "Recall: $RECALL  ($KW_SCORE)"
log "Results: $OUTPUT_MD"
