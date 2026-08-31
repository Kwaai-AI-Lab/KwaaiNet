#!/usr/bin/env bash
# Post-rebuild pipeline for D6 after full graph build with 6 entity types.
# Run after: kwaainet rag graph build --kb D6 ... --reset-graph ...
#
# Usage: bash tests/kwaai-knowledge/post_rebuild_pipeline.sh

set -euo pipefail

KB=D6
YAML=tests/kwaai-knowledge/d6_family_tree.yaml
METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
INFERENCE_URLS="${METRO_LINUX},${METRO_WIN}"
MODEL=llama3.1:8b
RESULTS=tests/kwaai-knowledge/results
TS=$(date +%Y%m%d_%H%M%S)

log() { echo "[$(date '+%H:%M:%S')] $*"; }

log "=== Post-rebuild pipeline: D6 Phase 3 ==="
log "Timestamp: ${TS}"

# Step 1: Seed family tree (canonical entities + aliases + relations)
log "Step 1/6: graph seed"
kwaainet rag graph seed --kb "${KB}" --file "${YAML}"

# Step 2: Dedup (merge duplicates that emerged from 6-type extraction)
log "Step 2/6: graph dedup (auto)"
kwaainet rag graph dedup --kb "${KB}" --auto

# Step 3: Coref (pronoun/definite-description resolution)
log "Step 3/6: graph coref (no-llm, rule-based only)"
kwaainet rag graph coref --kb "${KB}" \
  --output "${RESULTS}/coref_D6_phase3_${TS}.md" \
  --commit

# Step 4: Enrich entities (descriptions + gender)
log "Step 4/6: graph enrich-entities"
kwaainet rag graph enrich-entities --kb "${KB}" \
  --inference-urls "${INFERENCE_URLS}" \
  --model "${MODEL}" \
  --workers 4 \
  --min-mentions 1

# Step 5: Score
log "Step 5/6: graph score"
kwaainet rag graph score --kb "${KB}" 2>&1 | grep -E "Overall:|Unknown type:"

# Step 6: Eval
log "Step 6/6: eval"
kwaainet rag eval --kb "${KB}" \
  --questions tests/kwaai-knowledge/d6_eval_questions.json \
  --output "${RESULTS}/eval_D6_phase3_${TS}.md"

log "=== Pipeline complete. Results in ${RESULTS}/eval_D6_phase3_${TS}.md ==="
