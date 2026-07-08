#!/usr/bin/env bash
# Axiomatic entity pre-classification overnight sweep
# Phase A: threshold sweep at 1% (quick, ~2-5 min per run)
# Phase B: best threshold at 10% (~20-30 min)
# Phase C: full rebuild with best threshold (~1.5-2h)
#
# Usage: ./axiomatic_sweep.sh [phase_a|phase_b|phase_c] [threshold]
# Default: runs all phases sequentially

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
LOG="$RESULTS_DIR/axiomatic_sweep_$(date +%Y%m%d_%H%M%S).log"
QUESTIONS="$SCRIPT_DIR/d6_eval_questions.json"
SEED_FILE="$SCRIPT_DIR/d6_family_tree.yaml"
DOC_SCHEMA="$SCRIPT_DIR/d6_doc_schema.yaml"
DOC_PATH="$REPO_ROOT/docs/LEST WE FORGET -rev25.pdf"

INFERENCE_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd"
MODEL="llama3.1:8b"
ENTITY_TYPES="Person,Place,Organization,Legislation,Publication"
WORKERS=4

PHASE="${1:-all}"
OVERRIDE_THRESHOLD="${2:-}"

mkdir -p "$RESULTS_DIR"

log() { echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG"; }

copy_metrics() {
  local label="$1"
  # Find the most recent metrics file — std::env::temp_dir() varies by OS:
  #   macOS: ~/.cache/temp/  Linux: /tmp/  macOS (older): $TMPDIR
  local src
  src=$(find /tmp "${TMPDIR:-/tmp}" "$HOME/.cache/temp" \
        -maxdepth 1 -name "kwaai_axiomatic_metrics_*.json" \
        2>/dev/null | xargs ls -t 2>/dev/null | head -1 || true)
  if [[ -n "$src" ]]; then
    local dest="$RESULTS_DIR/axiomatic_metrics_${label}_$(date +%Y%m%d_%H%M%S).json"
    cp "$src" "$dest"
    log "Metrics copied → $dest"
    echo "$dest"
  else
    log "WARNING: no metrics file found in /tmp, \$TMPDIR, or ~/.cache/temp"
    echo ""
  fi
}

print_metrics() {
  local f="$1"
  if [[ -f "$f" ]]; then
    jq -r '"  threshold:     \(.threshold)
  llm_skip_pct:  \(.llm_skip_pct | tostring | .[0:6])%
  axio_ent_pct:  \(.axiomatic_entity_pct | tostring | .[0:6])%
  wall_secs:     \(.total_wall_secs | tostring | .[0:8])s
  chunks total:  \(.total_chunks)
  skipped_llm:   \(.chunks_skipped_llm)
  partial_llm:   \(.chunks_partial_llm)
  full_llm:      \(.chunks_full_llm)
  method_breakdown: \(.method_breakdown)"' "$f" | tee -a "$LOG"
  fi
}

compare_phase_a() {
  log "=== Phase A comparison ==="
  local files
  files=$(ls -t "$RESULTS_DIR"/axiomatic_metrics_phA_*.json 2>/dev/null || true)
  if [[ -z "$files" ]]; then
    log "No Phase A metrics found"
    return
  fi
  echo "" | tee -a "$LOG"
  printf "%-10s %-12s %-12s %-12s\n" "threshold" "llm_skip_pct" "axio_ent_pct" "wall_secs" | tee -a "$LOG"
  printf "%-10s %-12s %-12s %-12s\n" "---------" "------------" "------------" "---------" | tee -a "$LOG"
  # shellcheck disable=SC2086
  jq -r '[.threshold, (.llm_skip_pct|tostring), (.axiomatic_entity_pct|tostring), (.total_wall_secs|tostring)] | @tsv' \
    $files 2>/dev/null | sort -k1 -n | while IFS=$'\t' read -r t s a w; do
    printf "%-10s %-12s %-12s %-12s\n" "$t" "${s:0:6}%" "${a:0:6}%" "${w:0:8}s"
  done | tee -a "$LOG"
  echo "" | tee -a "$LOG"
}

pick_best_threshold() {
  # Pick threshold with highest llm_skip_pct that still has entities_axiomatic > 0
  local files
  files=$(ls -t "$RESULTS_DIR"/axiomatic_metrics_phA_*.json 2>/dev/null || true)
  if [[ -z "$files" ]]; then
    echo "0.70"
    return
  fi
  # shellcheck disable=SC2086
  jq -s 'map(select(.entities_axiomatic > 0)) | sort_by(-.llm_skip_pct) | .[0].threshold' \
    $files 2>/dev/null || echo "0.70"
}

run_graph_build() {
  local threshold="$1"
  local sample_pct="$2"
  local reset="${3:-}"

  local reset_flag=""
  [[ "$reset" == "--reset-graph" ]] && reset_flag="--reset-graph"

  local sample_flag=""
  [[ -n "$sample_pct" ]] && sample_flag="--sample-pct $sample_pct"

  log "graph build: threshold=$threshold sample=${sample_pct:-100}% reset=${reset_flag:-no}"

  # shellcheck disable=SC2086
  kwaainet rag graph build --kb D6 \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers "$WORKERS" \
    --entity-types "$ENTITY_TYPES" \
    --no-relations \
    --graph-window 1 \
    --axiomatic-threshold "$threshold" \
    $reset_flag \
    $sample_flag \
    2>&1 | tee -a "$LOG"
}

# ─── Phase A: threshold sweep at 1% ────────────────────────────────────────

run_phase_a() {
  log "=== PHASE A: threshold sweep at 1% sample ==="
  local thresholds=(0.70 0.60 0.80)
  [[ -n "$OVERRIDE_THRESHOLD" ]] && thresholds=("$OVERRIDE_THRESHOLD")

  for T in "${thresholds[@]}"; do
    log "--- Phase A: threshold=$T ---"
    run_graph_build "$T" 1 "--reset-graph"
    local mfile
    mfile=$(copy_metrics "phA_t${T}")
    log "Threshold $T results:"
    [[ -n "$mfile" ]] && print_metrics "$mfile"
    log ""
  done

  compare_phase_a
}

# ─── Phase B: best threshold at 10% ────────────────────────────────────────

run_phase_b() {
  local T="${OVERRIDE_THRESHOLD:-$(pick_best_threshold)}"
  log "=== PHASE B: best threshold=$T at 10% sample ==="

  run_graph_build "$T" 10 "--reset-graph"
  local mfile
  mfile=$(copy_metrics "phB_t${T}")
  log "Phase B results (threshold=$T, 10% sample):"
  [[ -n "$mfile" ]] && print_metrics "$mfile"

  # Seed before eval
  log "Seeding graph with family tree..."
  kwaainet rag graph seed --kb D6 --file "$SEED_FILE" 2>&1 | tee -a "$LOG"

  log "Running eval (10% sample — directional only)..."
  kwaainet rag eval --kb D6 --questions "$QUESTIONS" 2>&1 | tee -a "$LOG"
}

# ─── Phase C: full production rebuild ──────────────────────────────────────

run_phase_c() {
  local T="${OVERRIDE_THRESHOLD:-$(pick_best_threshold)}"
  log "=== PHASE C: full rebuild at threshold=$T ==="

  kwaainet rag rebuild "$DOC_PATH" --kb D6 \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers "$WORKERS" \
    --entity-types "$ENTITY_TYPES" \
    --no-relations \
    --graph-window 1 \
    --timeline \
    --seed-file "$SEED_FILE" \
    --doc-schema "$DOC_SCHEMA" \
    --axiomatic-threshold "$T" \
    --yes \
    2>&1 | tee -a "$LOG"

  local mfile
  mfile=$(copy_metrics "phC_t${T}")
  log "Phase C build complete. Metrics:"
  [[ -n "$mfile" ]] && print_metrics "$mfile"

  log "Running eval after full rebuild..."
  local eval_out
  eval_out=$(kwaainet rag eval --kb D6 --questions "$QUESTIONS" 2>&1)
  echo "$eval_out" | tee -a "$LOG"

  # Append to eval_log.md
  # grep for "NNN.N/NNN" or "NNN/NNN" — eval outputs float numerator like "164.0/209"
  local score
  score=$(echo "$eval_out" | grep -oE '[0-9]+(\.[0-9]+)?/[0-9]+' | tail -1 || true)
  {
    echo ""
    echo "## $(date '+%Y-%m-%d') Phase C axiomatic threshold=$T"
    echo "- Eval: $score"
    echo "- Log: $LOG"
    echo "- Metrics: $mfile"
  } >> "$RESULTS_DIR/eval_log.md"
  log "Eval log updated."
}

# ─── Main dispatch ──────────────────────────────────────────────────────────

log "Axiomatic sweep started (phase=$PHASE)"
log "Log: $LOG"
log ""

case "$PHASE" in
  phase_a) run_phase_a ;;
  phase_b) run_phase_b ;;
  phase_c) run_phase_c ;;
  all)
    run_phase_a
    compare_phase_a
    run_phase_b
    run_phase_c
    ;;
  *)
    echo "Usage: $0 [phase_a|phase_b|phase_c|all] [threshold_override]"
    exit 1
    ;;
esac

log "=== Sweep complete ==="
