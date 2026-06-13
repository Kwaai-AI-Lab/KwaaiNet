#!/usr/bin/env bash
# ordering_experiment_1pct.sh
#
# Tests 3 pipeline orderings with extract-relations at 1% sample (--sample 0.01).
# Strategy: one fresh rebuild + seed on D6 → clone to 3 KBs → run orderings in
# parallel overnight → compare false-relation counts, gender coverage, eval recall.
#
# Orderings:
#   A (proposed by Reza): seed → dedup → coref → dedup → enrich → extract-rel → dedup → eval
#   B (coref before dedup): seed → coref → dedup → enrich → extract-rel → dedup → eval
#   C (enrich before coref): seed → dedup → enrich → coref → extract-rel → dedup → eval
#
# Usage:
#   nohup bash ordering_experiment_1pct.sh > ordering_experiment.log 2>&1 &
#
# Prerequisites:
#   - kwaainet in PATH, codesigned
#   - p2pd running: kwaainet start --daemon
#   - Ollama running on metro (OLLAMA_HOST=0.0.0.0)
#   - Jerome's machine accessible via p2p (set JEROME below)

set -euo pipefail

REPO=/Users/rezarassool/Source/KwaaiNet
PDF="$REPO/docs/LEST WE FORGET -rev25.pdf"
DOC_SCHEMA="$REPO/tests/kwaai-knowledge/d6_doc_schema.yaml"
SEED_FILE="$REPO/tests/kwaai-knowledge/d6_family_tree.yaml"
EVAL_Q="$REPO/tests/kwaai-knowledge/d6_eval_questions.json"
RESULTS="$REPO/tests/kwaai-knowledge/results"
LOG_FILE="$REPO/tests/kwaai-knowledge/d6_experiments_log.md"

# ── Machine pool ──────────────────────────────────────────────────────────────
METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
JEROME="p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd"
LOCAL="http://localhost:11434"

# Use metro-linux for graph build (entity extraction), metro-win + jerome for
# parallel ordering runs, local for embedding.
BUILD_URLS="${METRO_LINUX},${METRO_WIN}"
RE_MODEL="llama3.1:8b"
ENRICH_MODEL="llama3.1:8b"
EXTRACT_SAMPLE="0.01"  # 1% — bump to 0.10 or 1.0 for 10%/100% runs

# ── Derived paths ─────────────────────────────────────────────────────────────
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
PROGRESS_DIR="$HOME/.kwaainet/rag"
RUN_START=$(date +%s)

ORDERINGS=("A" "B" "C")

get_ord_desc() {
  case "$1" in
    A) echo "seed → dedup → coref → dedup → enrich → extract-rel → dedup" ;;
    B) echo "seed → coref → dedup → enrich → extract-rel → dedup" ;;
    C) echo "seed → dedup → enrich → coref → extract-rel → dedup" ;;
  esac
}

log() { echo "[$(date '+%H:%M:%S')] $*"; }

elapsed() { echo $(( $(date +%s) - RUN_START ))s; }

write_progress() {
  local kb="$1" step="$2" status="$3" detail="${4:-}"
  printf '{"kb":"%s","step":"%s","status":"%s","detail":"%s","elapsed_secs":%d}\n' \
    "$kb" "$step" "$status" "$detail" "$(( $(date +%s) - RUN_START ))" \
    > "$PROGRESS_DIR/${kb}/progress.json"
}

# ── KB clone helpers ──────────────────────────────────────────────────────────

clone_kb() {
  # Clone SRC_KB data dir + register DST_KB in config.yaml (same tenant_id, new dir)
  local SRC_KB="$1" DST_KB="$2"
  local SRC_DIR DST_DIR
  SRC_DIR=$(python3 - <<EOF
import yaml, os
cfg = yaml.safe_load(open(os.path.expanduser('~/.kwaainet/config.yaml')))
print(cfg['rag_kbs']['${SRC_KB}']['rag_data_dir'])
EOF
)
  DST_DIR="$HOME/.kwaainet/rag/$DST_KB"
  log "  Cloning $SRC_KB → $DST_KB ($SRC_DIR → $DST_DIR)"
  rm -rf "$DST_DIR"
  cp -r "$SRC_DIR" "$DST_DIR"

  python3 - <<EOF
import yaml, os, copy
p = os.path.expanduser('~/.kwaainet/config.yaml')
cfg = yaml.safe_load(open(p))
new = copy.deepcopy(cfg['rag_kbs']['${SRC_KB}'])
new['rag_data_dir'] = '${DST_DIR}'
cfg['rag_kbs']['${DST_KB}'] = new
with open(p, 'w') as f:
    yaml.dump(cfg, f, default_flow_style=False, allow_unicode=True)
print("  Registered ${DST_KB}")
EOF
}

cleanup_kb() {
  local KB="$1"
  python3 - <<EOF
import yaml, os
p = os.path.expanduser('~/.kwaainet/config.yaml')
cfg = yaml.safe_load(open(p))
removed = cfg['rag_kbs'].pop('${KB}', None)
with open(p, 'w') as f:
    yaml.dump(cfg, f, default_flow_style=False, allow_unicode=True)
if removed:
    print("  Removed ${KB} from config")
EOF
  rm -rf "$HOME/.kwaainet/rag/$KB"
  log "  Cleaned up $KB"
}

# ── Metric helpers ────────────────────────────────────────────────────────────

count_gendered_persons() {
  local KB="$1"
  python3 - <<EOF
import os, sys
sys.path.insert(0, '${REPO}/core')
try:
    import yaml
    cfg = yaml.safe_load(open(os.path.expanduser('~/.kwaainet/config.yaml')))
    kb_cfg = cfg['rag_kbs']['${KB}']
    data_dir = kb_cfg['rag_data_dir']
    import uuid
    tid = uuid.UUID(kb_cfg['tenant_id'])
    # Try to read gender stats via kwaainet stats
    import subprocess, json
    r = subprocess.run(['kwaainet', 'rag', 'graph', 'stats', '--kb', '${KB}'],
                       capture_output=True, text=True)
    for line in r.stdout.splitlines():
        if 'Gender' in line or 'gender' in line:
            print(line.strip())
            sys.exit(0)
    print("(gender stats not available in graph stats)")
except Exception as e:
    print(f"(could not count: {e})")
EOF
}

count_false_relations() {
  # Count hallucinated relations for Yousuf Rassool in an extract-relations log
  local FILE="$1"
  if [ ! -f "$FILE" ]; then echo "0 (no file)"; return; fi
  # Known-correct Yousuf relations: spouse_of Nazima, parent_of Feyruz/Reza/Zarina,
  # sibling_of Abdul/Fazil/Zain/Rasheda/Berina/Yasmin/Nasim
  local CORRECT_TARGETS="Nazima|Feyruz|Reza|Zarina|Abdul Rassool|Fazil|Zain|Rasheda|Berina|Yasmin|Nasim"
  local TOTAL WRONG
  TOTAL=$(grep -c "Yousuf Rassool.*spouse_of\|Yousuf Rassool.*sibling_of\|Yousuf Rassool.*parent_of" "$FILE" 2>/dev/null || echo 0)
  WRONG=$(grep "Yousuf Rassool.*spouse_of\|Yousuf Rassool.*sibling_of\|Yousuf Rassool.*parent_of" "$FILE" 2>/dev/null \
           | grep -vE "$CORRECT_TARGETS" | wc -l | tr -d ' ' || echo 0)
  echo "${WRONG} false / ${TOTAL} total (Yousuf)"
}

graph_stats() {
  local KB="$1"
  kwaainet rag graph stats --kb "$KB" 2>/dev/null \
    | grep -E "Entities|Relations" | awk '{printf "%s=%s ", $1, $2}' | sed 's/ $/\n/'
}

coref_resolution_count() {
  local FILE="$1"
  if [ ! -f "$FILE" ]; then echo "0"; return; fi
  grep -c "→ \*\*" "$FILE" 2>/dev/null || echo 0
}

# ── Phase 0: fresh rebuild ────────────────────────────────────────────────────

log "═══════════════════════════════════════════════════════════"
log "Phase 0: fresh rebuild of D6 (build + seed only, no dedup/coref/enrich)"
log "  PDF:         $PDF"
log "  doc-schema:  $DOC_SCHEMA"
log "  seed-file:   $SEED_FILE"
log "  build-urls:  $BUILD_URLS"
log "  sample:      $EXTRACT_SAMPLE"
log "═══════════════════════════════════════════════════════════"

mkdir -p "$RESULTS"
write_progress "D6" "0-rebuild" "running" "fresh rebuild"

# Destroy + ingest + graph build (entity extraction only, no dedup/coref)
kwaainet rag rebuild "$PDF" \
  --kb D6 \
  --doc-schema "$DOC_SCHEMA" \
  --chunk-strategy paragraph \
  --entity-types Person,Place,Organization \
  --no-relations \
  --workers 4 \
  --model llama3.1:8b \
  --inference-urls "$BUILD_URLS" \
  --yes

# Seed family tree BEFORE cloning so all clones have canonical entities + ground-truth relations
log "Seeding family tree..."
kwaainet rag graph seed --kb D6 --file "$SEED_FILE"

log "Post-seed stats: $(graph_stats D6)"
write_progress "D6" "0-rebuild" "complete" "post-seed: $(graph_stats D6)"

# ── Phase 1: clone post-seed state to 3 KBs ──────────────────────────────────

log ""
log "Phase 1: cloning D6 → D6_ord_A, D6_ord_B, D6_ord_C"

# Clean up any stale clones from previous runs
for ORD in "${ORDERINGS[@]}"; do
  cleanup_kb "D6_ord_${ORD}" 2>/dev/null || true
done

for ORD in "${ORDERINGS[@]}"; do
  clone_kb "D6" "D6_ord_${ORD}"
done

log "Clones ready."

# ── Phase 2: run orderings ─────────────────────────────────────────────────────

run_ordering() {
  local ORD="$1" KB="D6_ord_$1"
  local TS; TS=$(date +%Y%m%d_%H%M%S)
  local LABEL="D6_ord${ORD}_${TS}"
  local COREF_OUT="$RESULTS/coref_${LABEL}.md"
  local EXTRACT_OUT="$RESULTS/extract_rel_${LABEL}.md"
  local EVAL_OUT="$RESULTS/eval_${LABEL}.md"

  eval "ORD_COREF_FILE_${ORD}=\"\${COREF_OUT}\""
  eval "ORD_EXTRACT_FILE_${ORD}=\"\${EXTRACT_OUT}\""
  eval "ORD_EVAL_FILE_${ORD}=\"\${EVAL_OUT}\""

  log ""
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  log "Ordering $ORD — $(get_ord_desc "$ORD")"
  log "  KB: $KB   Label: $LABEL"
  log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  # Pick an inference machine per ordering to spread load:
  # A → metro-linux, B → metro-win, C → jerome
  local INF_URL
  case "$ORD" in
    A) INF_URL="$METRO_LINUX" ;;
    B) INF_URL="$METRO_WIN" ;;
    C) INF_URL="$JEROME" ;;
  esac

  # ── Ordering A: seed → dedup → coref → dedup → enrich → extract-rel → dedup ──
  if [ "$ORD" = "A" ]; then
    write_progress "$KB" "dedup-1" "running"
    log "$KB step: dedup (Tier 1+2)"
    kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
    kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

    write_progress "$KB" "coref" "running"
    log "$KB step: coref (no-llm)"
    kwaainet rag graph coref --kb "$KB" --no-llm --commit --window 2 --output "$COREF_OUT"
    log "$KB coref: $(coref_resolution_count "$COREF_OUT") resolutions"

    write_progress "$KB" "dedup-2" "running"
    log "$KB step: dedup pass 2"
    kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
    kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

    write_progress "$KB" "enrich" "running"
    log "$KB step: enrich-entities (descriptions + gender)"
    kwaainet rag graph enrich-entities --kb "$KB" \
      --model "$ENRICH_MODEL" \
      --inference-urls "$INF_URL" \
      --workers 4 \
      --min-mentions 1 \
      --entity-types Person,Place,Organization
  fi

  # ── Ordering B: seed → coref → dedup → enrich → extract-rel → dedup ──────────
  if [ "$ORD" = "B" ]; then
    write_progress "$KB" "coref" "running"
    log "$KB step: coref (no-llm)"
    kwaainet rag graph coref --kb "$KB" --no-llm --commit --window 2 --output "$COREF_OUT"
    log "$KB coref: $(coref_resolution_count "$COREF_OUT") resolutions"

    write_progress "$KB" "dedup" "running"
    log "$KB step: dedup"
    kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
    kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

    write_progress "$KB" "enrich" "running"
    log "$KB step: enrich-entities (descriptions + gender)"
    kwaainet rag graph enrich-entities --kb "$KB" \
      --model "$ENRICH_MODEL" \
      --inference-urls "$INF_URL" \
      --workers 4 \
      --min-mentions 1 \
      --entity-types Person,Place,Organization
  fi

  # ── Ordering C: seed → dedup → enrich → coref → extract-rel → dedup ──────────
  if [ "$ORD" = "C" ]; then
    write_progress "$KB" "dedup" "running"
    log "$KB step: dedup"
    kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
    kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

    write_progress "$KB" "enrich" "running"
    log "$KB step: enrich-entities (descriptions + gender)"
    kwaainet rag graph enrich-entities --kb "$KB" \
      --model "$ENRICH_MODEL" \
      --inference-urls "$INF_URL" \
      --workers 4 \
      --min-mentions 1 \
      --entity-types Person,Place,Organization

    write_progress "$KB" "coref" "running"
    log "$KB step: coref (no-llm)"
    kwaainet rag graph coref --kb "$KB" --no-llm --commit --window 2 --output "$COREF_OUT"
    log "$KB coref: $(coref_resolution_count "$COREF_OUT") resolutions"
  fi

  # ── Extract-relations (1%) — common to all orderings ─────────────────────────
  write_progress "$KB" "extract-rel" "running" "sample=$EXTRACT_SAMPLE"
  log "$KB step: extract-relations (sample=$EXTRACT_SAMPLE, model=$RE_MODEL)"
  kwaainet rag graph extract-relations --kb "$KB" \
    --inference-url "$INF_URL" \
    --model "$RE_MODEL" \
    --sample "$EXTRACT_SAMPLE" \
    --commit \
    --output "$EXTRACT_OUT"

  local FALSE_RELS; FALSE_RELS=$(count_false_relations "$EXTRACT_OUT")
  log "$KB extract-relations complete — false relations: $FALSE_RELS"
  log "$KB extract-relations output: $EXTRACT_OUT"

  # ── Final dedup + score ───────────────────────────────────────────────────────
  write_progress "$KB" "dedup-final" "running"
  log "$KB step: final dedup"
  kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
  HEALTH=$(kwaainet rag graph score --kb "$KB" 2>/dev/null | grep "Overall:" | awk '{print $2}' || echo "?")

  # ── Eval ──────────────────────────────────────────────────────────────────────
  write_progress "$KB" "eval" "running" "40 questions"
  log "$KB step: eval (40 questions)"
  kwaainet rag eval --kb "$KB" \
    --questions "$EVAL_Q" \
    --mode iterative \
    --output "$EVAL_OUT"

  RECALL=$(grep "Overall recall" "$EVAL_OUT" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+%' | head -1 || echo "?")
  KW_SCORE=$(grep "Overall recall" "$EVAL_OUT" 2>/dev/null | grep -oE '[0-9]+/[0-9]+' | head -1 || echo "?")
  STATS=$(graph_stats "$KB")

  write_progress "$KB" "done" "complete" \
    "recall=${RECALL} health=${HEALTH} false_rels=${FALSE_RELS}"

  log "$KB DONE — recall=$RECALL ($KW_SCORE)  health=$HEALTH  stats=$STATS"
  log "$KB false-relations: $FALSE_RELS"
}

# ── Run all orderings sequentially ────────────────────────────────────────────
# Parallelism across machines is handled at the inference level (each ordering
# uses a different machine). Running sequentially avoids SQLite contention on
# the local embedding model and keeps the log readable.

for ORD in "${ORDERINGS[@]}"; do
  run_ordering "$ORD"
done

# ── Phase 3: comparison summary ───────────────────────────────────────────────

log ""
log "═══════════════════════════════════════════════════════════"
log "EXPERIMENT SUMMARY"
log "═══════════════════════════════════════════════════════════"

SUMMARY_TS=$(date '+%Y-%m-%d %H:%M')

{
  echo ""
  echo "## ${SUMMARY_TS} — Ordering Experiment (1% extract-relations)"
  echo ""
  echo "- **Sample:** \`--sample ${EXTRACT_SAMPLE}\`"
  echo "- **RE model:** ${RE_MODEL}"
  echo "- **Enrich model:** ${ENRICH_MODEL}"
  echo "- **Machines:** metro-linux (A), metro-win (B), jerome (C)"
  echo ""
  echo "| Ordering | Steps | Recall | Health | False rels (Yousuf) | Coref resolutions |"
  echo "|----------|-------|--------|--------|---------------------|-------------------|"

  for ORD in "${ORDERINGS[@]}"; do
    SUM_KB="D6_ord_${ORD}"
    eval "SUM_EXTRACT=\"\${ORD_EXTRACT_FILE_${ORD}:-}\""
    eval "SUM_EVAL=\"\${ORD_EVAL_FILE_${ORD}:-}\""
    eval "SUM_COREF=\"\${ORD_COREF_FILE_${ORD}:-}\""
    SUM_RECALL=$(grep "Overall recall" "$SUM_EVAL" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+%' | head -1 || echo "?")
    SUM_KW=$(grep "Overall recall" "$SUM_EVAL" 2>/dev/null | grep -oE '[0-9]+/[0-9]+' | head -1 || echo "?")
    SUM_HEALTH=$(kwaainet rag graph score --kb "$SUM_KB" 2>/dev/null | grep "Overall:" | awk '{print $2}' || echo "?")
    SUM_FALSE=$(count_false_relations "$SUM_EXTRACT")
    SUM_COREF_CNT=$(coref_resolution_count "$SUM_COREF")
    echo "| **$ORD** | $(get_ord_desc "$ORD") | $SUM_RECALL ($SUM_KW) | $SUM_HEALTH | $SUM_FALSE | $SUM_COREF_CNT |"
  done

  echo ""
  echo "### Key delta questions (q09=grandchildren, q24=Cissie, q32=family)"
  for ORD in "${ORDERINGS[@]}"; do
    eval "SUM_EVAL=\"\${ORD_EVAL_FILE_${ORD}:-}\""
    echo ""
    echo "#### Ordering $ORD"
    echo "\`\`\`"
    grep -E "q09|q12|q24|q26|q32|q38|Overall" "$SUM_EVAL" 2>/dev/null | grep "|" | head -10 \
      || echo "(see $SUM_EVAL)"
    echo "\`\`\`"
  done

  echo ""
  echo "### Extract-relations detail"
  for ORD in "${ORDERINGS[@]}"; do
    eval "SUM_EXTRACT=\"\${ORD_EXTRACT_FILE_${ORD}:-}\""
    echo ""
    echo "#### Ordering $ORD — Yousuf Rassool relations extracted"
    echo "\`\`\`"
    grep "Yousuf Rassool.*spouse_of\|Yousuf Rassool.*sibling_of\|Yousuf Rassool.*parent_of" \
      "$SUM_EXTRACT" 2>/dev/null | head -20 || echo "(none found)"
    echo "\`\`\`"
  done

  echo ""
  echo "### Files"
  for ORD in "${ORDERINGS[@]}"; do
    eval "SUM_EXTRACT=\"\${ORD_EXTRACT_FILE_${ORD}:-?}\""
    eval "SUM_EVAL=\"\${ORD_EVAL_FILE_${ORD}:-?}\""
    echo "- Ordering $ORD: \`${SUM_EXTRACT}\` / \`${SUM_EVAL}\`"
  done
} >> "$LOG_FILE"

log "Summary appended to $LOG_FILE"
log "Total elapsed: $(elapsed)"
log ""
log "Next steps:"
log "  • If Ordering A (proposed) has fewest false rels → promote to 10%:"
log "    bash ordering_experiment_1pct.sh  (with EXTRACT_SAMPLE=0.10)"
log "  • Then 100%: EXTRACT_SAMPLE=1.0 + RE_MODEL=llama3.1:70b-instruct-q3_K_M"
log ""
log "KBs D6_ord_A/B/C preserved for inspection. Clean up with:"
log "  for ORD in A B C; do cleanup_kb D6_ord_\$ORD; done"
