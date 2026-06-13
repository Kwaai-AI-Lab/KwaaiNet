#!/usr/bin/env bash
# ordering_experiment_10pct.sh
#
# Promotes the winning Ordering A to 10% extract-relations sample.
# Skips the full D6 rebuild — clones from existing D6 (post-seed state).
#
# Ordering A: seed → dedup → coref → dedup → enrich → extract-rel(10%) → dedup → eval
#
# Usage:
#   nohup bash ordering_experiment_10pct.sh > ordering_experiment_10pct.log 2>&1 &
#
# Prerequisites:
#   - kwaainet 0.4.96+ (has call_llm retry fix)
#   - D6 KB exists and is post-seed (run ordering_experiment_1pct.sh first)
#   - p2pd running: kwaainet start --daemon

set -euo pipefail

REPO=/Users/rezarassool/Source/KwaaiNet
SEED_FILE="$REPO/tests/kwaai-knowledge/d6_family_tree.yaml"
EVAL_Q="$REPO/tests/kwaai-knowledge/d6_eval_questions.json"
RESULTS="$REPO/tests/kwaai-knowledge/results"
LOG_FILE="$REPO/tests/kwaai-knowledge/d6_experiments_log.md"

# ── Machine pool ──────────────────────────────────────────────────────────────
METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
LOCAL="http://localhost:11434"

INF_URL="$METRO_LINUX"
RE_MODEL="llama3.1:8b"
ENRICH_MODEL="llama3.1:8b"
EXTRACT_SAMPLE="0.10"

# ── Derived ───────────────────────────────────────────────────────────────────
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LABEL="D6_ordA_10pct_${TIMESTAMP}"
KB="D6_10pct_A"
COREF_OUT="$RESULTS/coref_${LABEL}.md"
EXTRACT_OUT="$RESULTS/extract_rel_${LABEL}.md"
EVAL_OUT="$RESULTS/eval_${LABEL}.md"
RUN_START=$(date +%s)

log() { echo "[$(date '+%H:%M:%S')] $*"; }
elapsed() { echo $(( $(date +%s) - RUN_START ))s; }

mkdir -p "$RESULTS"

# ── Clone D6 → D6_10pct_A ─────────────────────────────────────────────────────

log "Cloning D6 → $KB"

# Clean up stale clone
python3 - <<EOF
import yaml, os
p = os.path.expanduser('~/.kwaainet/config.yaml')
cfg = yaml.safe_load(open(p))
cfg['rag_kbs'].pop('${KB}', None)
with open(p, 'w') as f:
    yaml.dump(cfg, f, default_flow_style=False, allow_unicode=True)
EOF
rm -rf "$HOME/.kwaainet/rag/$KB"

SRC_DIR=$(python3 - <<EOF
import yaml, os
cfg = yaml.safe_load(open(os.path.expanduser('~/.kwaainet/config.yaml')))
print(cfg['rag_kbs']['D6']['rag_data_dir'])
EOF
)
DST_DIR="$HOME/.kwaainet/rag/$KB"

cp -r "$SRC_DIR" "$DST_DIR"

python3 - <<EOF
import yaml, os, copy
p = os.path.expanduser('~/.kwaainet/config.yaml')
cfg = yaml.safe_load(open(p))
new = copy.deepcopy(cfg['rag_kbs']['D6'])
new['rag_data_dir'] = '${DST_DIR}'
cfg['rag_kbs']['${KB}'] = new
with open(p, 'w') as f:
    yaml.dump(cfg, f, default_flow_style=False, allow_unicode=True)
print("  Registered ${KB}")
EOF

log "Clone ready: $KB"

# ── Ordering A: dedup → coref → dedup → enrich → extract-rel → dedup → eval ──

log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
log "Ordering A (10%) — seed → dedup → coref → dedup → enrich → extract-rel → dedup"
log "  KB:     $KB"
log "  Label:  $LABEL"
log "  INF:    $INF_URL"
log "  Sample: $EXTRACT_SAMPLE"
log "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Seed is already applied in D6 base — but re-seed is idempotent and ensures
# any new YAML entries (from future edits) are also applied.
log "Re-seeding family tree (idempotent)..."
kwaainet rag graph seed --kb "$KB" --file "$SEED_FILE"

log "Step: dedup (Tier 1+2)"
kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

log "Step: coref (no-llm)"
kwaainet rag graph coref --kb "$KB" --no-llm --commit --window 2 --output "$COREF_OUT"
COREF_CNT=$(grep -c "→ \*\*" "$COREF_OUT" 2>/dev/null || echo 0)
log "  coref: $COREF_CNT resolutions"

log "Step: dedup pass 2"
kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
kwaainet rag graph score --kb "$KB" > /dev/null 2>&1 || true

log "Step: enrich-entities (descriptions + gender, retry-enabled)"
kwaainet rag graph enrich-entities --kb "$KB" \
  --model "$ENRICH_MODEL" \
  --inference-urls "$INF_URL" \
  --workers 4 \
  --min-mentions 1 \
  --entity-types Person,Place,Organization

log "Step: extract-relations (sample=$EXTRACT_SAMPLE, model=$RE_MODEL)"
kwaainet rag graph extract-relations --kb "$KB" \
  --inference-url "$INF_URL" \
  --model "$RE_MODEL" \
  --sample "$EXTRACT_SAMPLE" \
  --commit \
  --output "$EXTRACT_OUT"

KNOWN_CORRECT="Nazima|Feyruz|Reza|Zarina|Abdul Rassool|Fazil|Zain|Rasheda|Berina|Yasmin|Nasim"
FALSE_RELS=$(grep "Yousuf Rassool.*spouse_of\|Yousuf Rassool.*sibling_of\|Yousuf Rassool.*parent_of" \
  "$EXTRACT_OUT" 2>/dev/null | grep -vE "$KNOWN_CORRECT" | wc -l | tr -d ' ' || echo 0)
log "  extract-relations complete — false relations: $FALSE_RELS"

log "Step: final dedup"
kwaainet rag graph dedup --kb "$KB" --auto --auto-threshold 0.97
HEALTH=$(kwaainet rag graph score --kb "$KB" 2>/dev/null | grep "Overall:" | awk '{print $2}' || echo "?")
STATS=$(kwaainet rag graph stats --kb "$KB" 2>/dev/null \
  | grep -E "Entities|Relations" | awk '{printf "%s=%s ", $1, $2}' | sed 's/ $//')

log "Step: eval (40 questions)"
kwaainet rag eval --kb "$KB" \
  --questions "$EVAL_Q" \
  --mode iterative \
  --output "$EVAL_OUT"

RECALL=$(grep "Overall recall" "$EVAL_OUT" 2>/dev/null | grep -oE '[0-9]+\.[0-9]+%' | head -1 || echo "?")
KW_SCORE=$(grep "Overall recall" "$EVAL_OUT" 2>/dev/null | grep -oE '[0-9]+/[0-9]+' | head -1 || echo "?")

log "DONE — recall=$RECALL ($KW_SCORE)  health=$HEALTH  stats=$STATS"
log "false-relations: $FALSE_RELS"
log "Total elapsed: $(elapsed)"

# ── Append to experiments log ─────────────────────────────────────────────────

SUMMARY_TS=$(date '+%Y-%m-%d %H:%M')
ENRICH_UPDATED=$(grep "Enrich complete" "$EVAL_OUT" 2>/dev/null \
  | grep -oE '[0-9]+ updated' | head -1 || \
  grep -oE 'Enrich complete.*updated' /dev/null 2>/dev/null || echo "?")

{
  echo ""
  echo "## ${SUMMARY_TS} — Ordering A 10% extract-relations"
  echo ""
  echo "- **Sample:** \`--sample ${EXTRACT_SAMPLE}\`"
  echo "- **RE model:** ${RE_MODEL}"
  echo "- **Enrich model:** ${ENRICH_MODEL}"
  echo "- **Machine:** metro-linux (A6000)"
  echo "- **Ordering:** seed → dedup → coref → dedup → enrich → extract-rel → dedup"
  echo ""
  echo "| Metric | Value |"
  echo "| --- | --- |"
  echo "| Recall | $RECALL ($KW_SCORE) |"
  echo "| Graph health | $HEALTH |"
  echo "| False Yousuf relations | $FALSE_RELS |"
  echo "| Coref resolutions | $COREF_CNT |"
  echo "| Stats | $STATS |"
  echo ""
  echo "### Key delta questions"
  echo ""
  echo "\`\`\`"
  grep -E "q09|q12|q24|q26|q32|q38|Overall" "$EVAL_OUT" 2>/dev/null | grep "|" | head -10 \
    || echo "(see $EVAL_OUT)"
  echo "\`\`\`"
  echo ""
  echo "### Yousuf Rassool relations extracted"
  echo ""
  echo "\`\`\`"
  grep "Yousuf Rassool.*spouse_of\|Yousuf Rassool.*sibling_of\|Yousuf Rassool.*parent_of" \
    "$EXTRACT_OUT" 2>/dev/null | head -20 || echo "(none found)"
  echo "\`\`\`"
  echo ""
  echo "### Files"
  echo "- Extract: \`${EXTRACT_OUT}\`"
  echo "- Eval: \`${EVAL_OUT}\`"
  echo "- Coref: \`${COREF_OUT}\`"
} >> "$LOG_FILE"

log "Summary appended to $LOG_FILE"
log ""
log "Next: if recall > 65% and false rels = 0, run at 100%:"
log "  Set EXTRACT_SAMPLE=1.0 and RE_MODEL=llama3.1:70b-instruct-q3_K_M"
