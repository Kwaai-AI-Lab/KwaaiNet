#!/usr/bin/env bash
# Phase 4 relation axiomatic classifier sweep — sibling of axiomatic_sweep.sh (entities),
# same Phase A/B/C structure and log/metrics conventions, applied to
# --relation-threshold-high/--relation-threshold-low instead of --axiomatic-threshold.
#
# Phase A: (threshold_high, threshold_low) grid sweep at 1% (quick, ~2-5 min per run)
# Phase B: best pair at 10%, plus a rough precision/recall estimate against the
#          family-tree YAML's 12 ground-truth relation types (~20-30 min)
# Phase C: full rebuild with the best pair (~1.5-2h)
#
# Usage: ./relation_sweep.sh [phase_a|phase_b|phase_c] [threshold_high] [threshold_low]
# Default: runs all phases sequentially, picking thresholds automatically.
#
# Precondition: entities must already be classifiable — this always runs graph build
# with --axiomatic-threshold $ENTITY_THRESHOLD (the documented production value)
# alongside the relation thresholds, in the same command, since Phase 4 relations need
# resolved entity IDs from the same build.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
LOG="$RESULTS_DIR/relation_sweep_$(date +%Y%m%d_%H%M%S).log"
QUESTIONS="$SCRIPT_DIR/d6_eval_questions.json"
SEED_FILE="$SCRIPT_DIR/d6_family_tree.yaml"
DOC_SCHEMA="$SCRIPT_DIR/d6_doc_schema.yaml"
DOC_PATH="$REPO_ROOT/docs/LEST WE FORGET -rev25.pdf"

INFERENCE_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
MODEL="llama3.1:8b"
ENTITY_TYPES="Person,Place,Organization,Legislation,Publication"
WORKERS=4
ENTITY_THRESHOLD=0.80 # documented production value (2026-07-08 sweep) — not re-tuned here

PHASE="${1:-all}"
OVERRIDE_HIGH="${2:-}"
OVERRIDE_LOW="${3:-}"

mkdir -p "$RESULTS_DIR"

log() { echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG"; }

copy_relation_metrics() {
  local label="$1"
  # NOTE: this function's stdout is captured with $(...) by callers — never call
  # log() (which echoes to stdout) from in here, only write to "$LOG" directly, or
  # the captured metrics-file path gets corrupted with log text (see
  # pick_best_relation_thresholds for the same footgun).
  local src
  src=$(find /tmp "${TMPDIR:-/tmp}" "$HOME/.cache/temp" \
        -maxdepth 1 -name "kwaai_relation_axiomatic_metrics_*.json" \
        2>/dev/null | xargs ls -t 2>/dev/null | head -1 || true)
  if [[ -n "$src" ]]; then
    local dest="$RESULTS_DIR/relation_axiomatic_metrics_${label}_$(date +%Y%m%d_%H%M%S).json"
    cp "$src" "$dest"
    echo "[$(date '+%H:%M:%S')] Metrics copied → $dest" >> "$LOG"
    echo "$dest"
  else
    echo "[$(date '+%H:%M:%S')] WARNING: no relation metrics file found in /tmp, \$TMPDIR, or ~/.cache/temp" >> "$LOG"
    echo ""
  fi
}

print_relation_metrics() {
  local f="$1"
  if [[ -f "$f" ]]; then
    jq -r '"  thresholds:      high=\(.threshold_high) low=\(.threshold_low)
  candidates:      \(.candidates_generated)
  commit_pct:      \(.commit_pct | tostring | .[0:6])%
  drop_pct:        \(.drop_pct | tostring | .[0:6])%
  demoted_axiom:   \(.candidates_demoted_by_axiom)
  llm_confirm_pct: \((.llm_confirm_rate * 100) | tostring | .[0:6])%  (confirmed=\(.llm_confirmed) rejected=\(.llm_rejected) retyped=\(.llm_retyped))
  wall_secs:       \(.total_wall_secs | tostring | .[0:8])s
  method_breakdown: \(.method_breakdown)"' "$f" | tee -a "$LOG"
  fi
}

compare_phase_a_rel() {
  log "=== Phase A (relations) comparison ==="
  local files
  files=$(ls -t "$RESULTS_DIR"/relation_axiomatic_metrics_phA_*.json 2>/dev/null || true)
  if [[ -z "$files" ]]; then
    log "No Phase A relation metrics found"
    return
  fi
  echo "" | tee -a "$LOG"
  printf "%-6s %-6s %-10s %-10s %-14s %-10s\n" "high" "low" "commit_pct" "drop_pct" "llm_confirm" "wall_secs" | tee -a "$LOG"
  printf "%-6s %-6s %-10s %-10s %-14s %-10s\n" "----" "---" "----------" "--------" "-----------" "---------" | tee -a "$LOG"
  # shellcheck disable=SC2086
  jq -r '[.threshold_high, .threshold_low, (.commit_pct|tostring), (.drop_pct|tostring), ((.llm_confirm_rate*100)|tostring), (.total_wall_secs|tostring)] | @tsv' \
    $files 2>/dev/null | sort -k1 -rn -k2 -rn | while IFS=$'\t' read -r h l c d lc w; do
    printf "%-6s %-6s %-10s %-10s %-14s %-10s\n" "$h" "$l" "${c:0:6}%" "${d:0:6}%" "${lc:0:6}%" "${w:0:8}s"
  done | tee -a "$LOG"
  echo "" | tee -a "$LOG"
}

pick_best_relation_thresholds() {
  # Among pairs with llm_confirm_rate >= 0.7 (provisional precision floor — see
  # projects/kwaai-knowledge/plans/Phase4-EntityRelations-plan.md "Needs human decision"),
  # pick the one maximizing commit_pct. Falls back to a conservative default pair if
  # no Phase A data exists yet, or nothing clears the floor.
  local files
  files=$(ls -t "$RESULTS_DIR"/relation_axiomatic_metrics_phA_*.json 2>/dev/null || true)
  if [[ -z "$files" ]]; then
    echo "0.75 0.45"
    return
  fi
  # shellcheck disable=SC2086
  local best
  best=$(jq -s 'map(select(.llm_confirm_rate >= 0.7)) | sort_by(-.commit_pct) | .[0]
                 | if . == null then null else "\(.threshold_high) \(.threshold_low)" end' \
    $files 2>/dev/null || echo "null")
  if [[ "$best" == "null" || -z "$best" ]]; then
    # Write directly to the log file, NOT via log() — this function's stdout is
    # captured with $(...) by callers, and log() echoes to stdout too, which would
    # corrupt the captured threshold pair with this warning text.
    echo "[$(date '+%H:%M:%S')] WARNING: no threshold pair cleared the 70% LLM-confirm-rate floor — falling back to 0.75/0.45" >> "$LOG"
    echo "0.75 0.45"
  else
    echo "$best" | tr -d '"'
  fi
}

run_graph_build_rel() {
  local high="$1"
  local low="$2"
  local sample_pct="$3"
  local reset="${4:-}"

  local reset_flag=""
  [[ "$reset" == "--reset-graph" ]] && reset_flag="--reset-graph"

  local sample_flag=""
  [[ -n "$sample_pct" ]] && sample_flag="--sample-pct $sample_pct"

  # Two passes, not one: the narrator entity only gets its "narrator"/"author"
  # aliases from the YAML seed, and Phase 4's first-person pronoun resolution
  # needs that to find the narrator's entity ID (this is exactly what
  # `kwaainet rag rebuild` fixes internally by deferring relation extraction to
  # its own step 5.5b, after seeding — `graph build` alone has no such ordering,
  # so this script has to reproduce it manually). Pass 1 builds entities only
  # (no relation flags) and resets the graph; seed; pass 2 rebuilds WITHOUT
  # --reset-graph (so the seed survives) and WITH relation flags — its entity
  # re-extraction pass is cheap since axiom_extract's KnownEntity rule matches
  # everything already in the graph without an LLM call.
  log "graph build (pass 1/2, entities only): sample=${sample_pct:-100}% reset=${reset_flag:-no}"
  # --no-relations here is load-bearing, not decoration: without it, ingestion.rs's
  # `no_relations || relation_threshold_high > 0.0` guard evaluates false (this pass
  # passes neither flag), so the legacy lexical_relation_trigger()-gated full-LLM
  # relation extraction runs at full strength and pollutes the graph with exactly the
  # kind of hallucinated relations (bare-initials-as-Person entities getting
  # parent_of/spouse_of edges) that Phase 4 exists to replace. `cmd_rebuild()`'s Step 4
  # forces this same suppression internally; this script has to do it explicitly since
  # it drives `graph build` directly instead of `rag rebuild`.
  # shellcheck disable=SC2086
  kwaainet rag graph build --kb D6 \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers "$WORKERS" \
    --entity-types "$ENTITY_TYPES" \
    --graph-window 1 \
    --axiomatic-threshold "$ENTITY_THRESHOLD" \
    --no-relations \
    $reset_flag \
    $sample_flag \
    2>&1 | tee -a "$LOG"

  log "Seeding graph with family tree (so the narrator entity exists before relations run)..."
  kwaainet rag graph seed --kb D6 --file "$SEED_FILE" 2>&1 | tee -a "$LOG"

  log "graph build (pass 2/2, relations): high=$high low=$low sample=${sample_pct:-100}%"
  # shellcheck disable=SC2086
  kwaainet rag graph build --kb D6 \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers "$WORKERS" \
    --entity-types "$ENTITY_TYPES" \
    --graph-window 1 \
    --axiomatic-threshold "$ENTITY_THRESHOLD" \
    --relation-threshold-high "$high" \
    --relation-threshold-low "$low" \
    $sample_flag \
    2>&1 | tee -a "$LOG"
}

# ─── Precision/recall estimate against d6_family_tree.yaml ─────────────────
#
# Rough estimate only — the YAML is itself incomplete ground truth (not an
# exhaustive labeling of the whole corpus), so a true positive absent from
# both the corpus text and the YAML looks identical to a false positive here.
# Treat this as a directional signal, not a rigorous score.
estimate_precision_recall() {
  log "Estimating relation precision/recall against d6_family_tree.yaml (rough — see script header)..."
  python3 - "$SEED_FILE" <<'PYEOF' 2>&1 | tee -a "$LOG"
import sys
import sqlite3
import json
import glob
import os
import yaml

seed_path = sys.argv[1]
with open(seed_path) as f:
    seed = yaml.safe_load(f)
seed_triples = set()
for r in seed.get("relations", []):
    seed_triples.add((r["from"].strip().lower(), r["to"].strip().lower(), r["type"]))

data_dir = os.path.expanduser("~/.kwaainet/rag/D6")
db_candidates = glob.glob(os.path.join(data_dir, "graph-*.db"))
if not db_candidates:
    print("  No D6 graph DB found — skipping precision/recall estimate.")
    sys.exit(0)
db_path = db_candidates[0]

conn = sqlite3.connect(db_path)
names = {}
for (v,) in conn.execute("SELECT value FROM entities"):
    e = json.loads(v)
    names[e["id"]] = e["name"].strip().lower()

extracted_triples = set()
tp = fp = 0
for (v,) in conn.execute("SELECT value FROM relations"):
    rel = json.loads(v)
    if 0 in rel.get("evidence_chunk_ids", []):
        continue  # seeded relation — not a Phase 4 extraction, skip
    if rel.get("source") not in ("axiomatic_high", "axiomatic_llm_verify"):
        continue  # only score Phase 4 output, not legacy LLM-open relations
    src = names.get(rel["src_id"])
    dst = names.get(rel["dst_id"])
    if not src or not dst:
        continue
    triple = (src, dst, rel["relation_type"])
    extracted_triples.add(triple)
    if triple in seed_triples or (dst, src, rel["relation_type"]) in seed_triples:
        tp += 1
    else:
        fp += 1

fn = sum(
    1 for t in seed_triples
    if t not in extracted_triples and (t[1], t[0], t[2]) not in extracted_triples
)

precision = tp / (tp + fp) if (tp + fp) else 0.0
recall = tp / (tp + fn) if (tp + fn) else 0.0
print(f"  Phase 4 extracted (non-seeded): {len(extracted_triples)}")
print(f"  True positives (vs YAML):       {tp}")
print(f"  False positives (vs YAML):      {fp}")
print(f"  False negatives (YAML missed):  {fn}")
print(f"  Precision (rough estimate):     {precision:.1%}")
print(f"  Recall (rough estimate):        {recall:.1%}")
PYEOF
}

# ─── Phase A: (high, low) grid sweep at 1% ─────────────────────────────────

run_phase_a_rel() {
  log "=== PHASE A (relations): threshold grid sweep at 1% sample ==="
  local pairs=("0.85 0.60" "0.75 0.45" "0.65 0.30")
  [[ -n "$OVERRIDE_HIGH" && -n "$OVERRIDE_LOW" ]] && pairs=("$OVERRIDE_HIGH $OVERRIDE_LOW")

  for pair in "${pairs[@]}"; do
    read -r H L <<< "$pair"
    log "--- Phase A (relations): high=$H low=$L ---"
    run_graph_build_rel "$H" "$L" 1 "--reset-graph"
    local mfile
    mfile=$(copy_relation_metrics "phA_h${H}_l${L}")
    log "high=$H low=$L results:"
    [[ -n "$mfile" ]] && print_relation_metrics "$mfile"
    log ""
  done

  compare_phase_a_rel
}

# ─── Phase B: best pair at 10% + precision/recall estimate ─────────────────

run_phase_b_rel() {
  local pair
  if [[ -n "$OVERRIDE_HIGH" && -n "$OVERRIDE_LOW" ]]; then
    pair="$OVERRIDE_HIGH $OVERRIDE_LOW"
  else
    pair="$(pick_best_relation_thresholds)"
  fi
  read -r H L <<< "$pair"
  log "=== PHASE B (relations): best pair high=$H low=$L at 10% sample ==="

  run_graph_build_rel "$H" "$L" 10 "--reset-graph"
  local mfile
  mfile=$(copy_relation_metrics "phB_h${H}_l${L}")
  log "Phase B (relations) results (high=$H low=$L, 10% sample):"
  [[ -n "$mfile" ]] && print_relation_metrics "$mfile"

  # Seeding already happened inside run_graph_build_rel (pass 1 → seed → pass 2),
  # so the graph is ready for eval without a redundant re-seed here.
  estimate_precision_recall

  log "Running eval (10% sample — directional only)..."
  kwaainet rag eval --kb D6 --questions "$QUESTIONS" 2>&1 | tee -a "$LOG"
}

# ─── Phase C: full production rebuild ──────────────────────────────────────

run_phase_c_rel() {
  local pair
  if [[ -n "$OVERRIDE_HIGH" && -n "$OVERRIDE_LOW" ]]; then
    pair="$OVERRIDE_HIGH $OVERRIDE_LOW"
  else
    pair="$(pick_best_relation_thresholds)"
  fi
  read -r H L <<< "$pair"
  log "=== PHASE C (relations): full rebuild at high=$H low=$L ==="

  kwaainet rag rebuild "$DOC_PATH" --kb D6 \
    --model "$MODEL" \
    --inference-urls "$INFERENCE_URLS" \
    --workers "$WORKERS" \
    --entity-types "$ENTITY_TYPES" \
    --graph-window 1 \
    --timeline \
    --seed-file "$SEED_FILE" \
    --doc-schema "$DOC_SCHEMA" \
    --axiomatic-threshold "$ENTITY_THRESHOLD" \
    --relation-threshold-high "$H" \
    --relation-threshold-low "$L" \
    --yes \
    2>&1 | tee -a "$LOG"

  local mfile
  mfile=$(copy_relation_metrics "phC_h${H}_l${L}")
  log "Phase C (relations) build complete. Metrics:"
  [[ -n "$mfile" ]] && print_relation_metrics "$mfile"

  estimate_precision_recall

  log "Checking Graph Health Score (Relatio% component)..."
  kwaainet rag graph score --kb D6 2>&1 | tee -a "$LOG"

  log "Running eval after full rebuild..."
  local eval_out
  eval_out=$(kwaainet rag eval --kb D6 --questions "$QUESTIONS" 2>&1)
  echo "$eval_out" | tee -a "$LOG"

  local score
  score=$(echo "$eval_out" | grep -oE '[0-9]+(\.[0-9]+)?/[0-9]+' | tail -1 || true)
  {
    echo ""
    echo "## $(date '+%Y-%m-%d') Phase C relation thresholds=high:${H}/low:${L}"
    echo "- Eval: $score"
    echo "- Log: $LOG"
    echo "- Metrics: $mfile"
  } >> "$RESULTS_DIR/eval_log.md"
  log "Eval log updated."
}

# ─── Main dispatch ──────────────────────────────────────────────────────────

log "Relation sweep started (phase=$PHASE)"
log "Log: $LOG"
log ""

case "$PHASE" in
  phase_a) run_phase_a_rel ;;
  phase_b) run_phase_b_rel ;;
  phase_c) run_phase_c_rel ;;
  all)
    run_phase_a_rel
    compare_phase_a_rel
    run_phase_b_rel
    run_phase_c_rel
    ;;
  *)
    echo "Usage: $0 [phase_a|phase_b|phase_c|all] [threshold_high] [threshold_low]"
    exit 1
    ;;
esac

log "=== Relation sweep complete ==="
