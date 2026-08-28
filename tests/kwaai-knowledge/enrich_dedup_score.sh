#!/usr/bin/env bash
# Post-build pipeline: enrich entity descriptions → reembed → dedup → score → eval
#
# Run AFTER a full graph build (overnight_struct_coref_rel.sh) completes.
# Uses the new description-aware dedup which activates once entities have
# rich descriptions (≥100 chars) from the enrich pass.
#
# Usage: bash enrich_dedup_score.sh [--skip-enrich] [--skip-eval]

set -euo pipefail

REPO=/Users/rezarassool/Source/KwaaiNet
EVAL_Q="$REPO/tests/kwaai-knowledge/d6_eval_questions.json"
RESULTS="$REPO/tests/kwaai-knowledge/results"
LOG_FILE="$REPO/tests/kwaai-knowledge/d6_experiments_log.md"

METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"

SKIP_ENRICH=false
SKIP_EVAL=false
for arg in "$@"; do
  case $arg in
    --skip-enrich) SKIP_ENRICH=true ;;
    --skip-eval)   SKIP_EVAL=true ;;
  esac
done

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LABEL="D6_enrich_dedup_${TIMESTAMP}"
OUTPUT_MD="$RESULTS/eval_${LABEL}.md"

log() { echo "[$(date '+%H:%M:%S')] $*"; }

# ── Step 1: enrich entity descriptions ────────────────────────────────────
if [ "$SKIP_ENRICH" = false ]; then
  log "Step 1: enrich entity descriptions (llama3.1:8b, min-mentions=2)"
  log "  Inference: ${METRO_WIN} (8b, fast)"
  kwaainet rag graph enrich-entities \
    --kb D6 \
    --model llama3.1:8b \
    --inference-urls "$METRO_WIN" \
    --workers 4 \
    --min-mentions 2 \
    --entity-types Person,Place,Organization
  log "Step 1 complete"
else
  log "Step 1: skipped (--skip-enrich)"
fi

# ── Step 2: reembed (update embeddings with enriched descriptions) ────────
log "Step 2: graph reembed (update entity embeddings post-enrich)"
kwaainet rag graph reembed --kb D6
log "Step 2 complete"

# ── Step 3: dedup with description-aware blocking ────────────────────────
log "Step 3: dedup --auto (Tier 2+3 with DESC + R1/R2/R3 blocking)"
kwaainet rag graph dedup --kb D6 --auto --auto-threshold 0.97
log "Step 3 complete"

# ── Step 4: graph score ───────────────────────────────────────────────────
log "Step 4: graph score (completion percentage)"
kwaainet rag graph score --kb D6
log "Step 4 complete"

# ── Step 5: eval ─────────────────────────────────────────────────────────
if [ "$SKIP_EVAL" = false ]; then
  log "Step 5: eval (40 questions)"
  kwaainet rag eval \
    --questions "$EVAL_Q" \
    --kb D6 \
    --mode iterative \
    --output "$OUTPUT_MD"
  log "Step 5 complete — results at $OUTPUT_MD"

  # ── Step 6: log results ─────────────────────────────────────────────────
  RECALL=$(grep "Overall recall" "$OUTPUT_MD" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+%' | head -1 || echo "?")
  KW_SCORE=$(grep "Overall recall" "$OUTPUT_MD" 2>/dev/null | grep -oE '[0-9]+/[0-9]+' | head -1 || echo "?")
  ENTITIES=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Entities | awk '{print $2}')
  RELATIONS=$(kwaainet rag graph stats --kb D6 2>/dev/null | grep Relations | awk '{print $2}')
  HEALTH=$(kwaainet rag graph score --kb D6 2>/dev/null | grep "Overall:" | awk '{print $2}')

  cat >> "$LOG_FILE" << ENTRY

## $(date '+%Y-%m-%d') – $LABEL

- **Experiment:** Enrich + description-aware dedup + rescore (post run-8)
- **Pipeline:** enrich-entities → reembed → dedup --auto → graph score → eval
- **After:**  $ENTITIES entities, $RELATIONS relations, health=$HEALTH, **$RECALL** recall ($KW_SCORE)
- **Changes:**
  - enrich-entities: LLM paragraph summaries for all entities with ≥2 mentions
  - reembed: entity embeddings updated with enriched descriptions
  - dedup --auto: description-divergence block active (Jaccard < 12% → [BLOCKED:DESC])
- **Eval output:** $OUTPUT_MD

### Key delta questions
\`\`\`
$(grep -E "q09|q12|q24|q26|q32|q38|Overall" "$OUTPUT_MD" 2>/dev/null | grep "|" | head -10 || echo "see $OUTPUT_MD")
\`\`\`
ENTRY

  log "Done. Recall: $RECALL ($KW_SCORE)  health: $HEALTH"
else
  log "Step 5: skipped (--skip-eval)"
  HEALTH=$(kwaainet rag graph score --kb D6 2>/dev/null | grep "Overall:" | awk '{print $2}')
  log "Graph health: $HEALTH"
fi
