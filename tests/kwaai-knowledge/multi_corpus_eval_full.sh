#!/usr/bin/env bash
# Multi-corpus RAG eval harness — "full" variant with the decomposed
# retrieval/generation scoring plus LLM-judge, matching the methodology in
# projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md
# ("token-overlap + semantic (cosine similarity; low=0.30, high=0.85)").
#
# Unlike multi_corpus_eval.sh's plain run, this adds:
#   --semantic-score --semantic-low 0.30 --semantic-high 0.85   (real ret=/gen= split)
#   --llm-judge                                                  (0/1/2 judge score)
#
# Usage:
#   ./multi_corpus_eval_full.sh                  # eval all listed corpora
#   ./multi_corpus_eval_full.sh Manhattan Legal   # eval specific corpora only

set -uo pipefail
cd "$(dirname "$0")"

RESULTS_DIR="results"
SUMMARY="$RESULTS_DIR/multi_corpus_eval_full_summary.md"
PROGRESS="multi_corpus_eval_full_progress.json"
mkdir -p "$RESULTS_DIR"

ALL_KBS="Manhattan MobyDick Legal Meetings PythonDocs NIST Climate RFCs DeepSea DreamMem Astrophysics"

if [[ $# -gt 0 ]]; then
  TARGET_KBS="$*"
else
  TARGET_KBS="$ALL_KBS"
fi

if [[ ! -f "$SUMMARY" ]]; then
  cat > "$SUMMARY" <<'HEADER'
# Multi-Corpus RAG Eval Summary — Full (retrieval + generation + LLM judge)

Method: token-overlap + semantic (cosine; low=0.30, high=0.85) + --llm-judge
Compare against projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md

| KB | Retrieval | Generation | Judge (avg/2) | Date |
|----|-----------|------------|---------------|------|
HEADER
fi

log() { echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*"; }

write_progress() {
  local KB="$1" STATUS="$2"
  python3 - "$KB" "$STATUS" "$PROGRESS" <<'PY'
import json, sys, os
kb, status, path = sys.argv[1], sys.argv[2], sys.argv[3]
data = json.load(open(path)) if os.path.exists(path) else {"kbs": {}, "current_kb": "", "status": ""}
data["current_kb"] = kb
data["status"] = status
data["kbs"].setdefault(kb, {})
data["kbs"][kb]["status"] = status
json.dump(data, open(path, "w"), indent=2)
PY
}

for KB in $TARGET_KBS; do
  QF="${KB}/eval_questions.json"
  if [[ ! -f "$QF" ]]; then
    log "⚠️  $KB: question file not found ($QF), skipping"
    write_progress "$KB" "skip-no-questions"
    continue
  fi

  TS=$(date +%Y%m%d_%H%M%S)
  OUT="$RESULTS_DIR/eval_${KB}_full_${TS}.md"

  log "▶ Evaluating $KB (full: semantic-score + llm-judge) …"
  write_progress "$KB" "running"

  STDOUT_LOG="$RESULTS_DIR/eval_${KB}_full_${TS}.stdout.log"
  kwaainet rag eval \
    --kb "$KB" \
    --questions "$QF" \
    --semantic-score \
    --semantic-low 0.30 \
    --semantic-high 0.85 \
    --llm-judge \
    --output "$OUT" \
    > "$STDOUT_LOG" 2>&1
  EXIT=$?

  if [[ $EXIT -ne 0 ]]; then
    log "⚠️  $KB eval exited with code $EXIT — see $OUT"
    write_progress "$KB" "failed"
    continue
  fi

  # With --output, the markdown report only holds per-question Q&A — no
  # aggregate line. The aggregate only prints to stdout, and only as a
  # generation-only "hit rate" (e.g. "✅ Report written to ... (69.7% hit rate
  # judge=1.70/2, 9707ms avg)"). Retrieval has no aggregate at all in either
  # place — only per-question "ret=X/Y" lines — so compute it ourselves.
  GEN=$(grep -oE "\([0-9.]+% hit rate" "$STDOUT_LOG" | tail -1 | grep -oE "[0-9.]+%" || echo "?")
  JUDGE=$(grep -oE "judge=[0-9.]+/2" "$STDOUT_LOG" | tail -1 | grep -oE "[0-9.]+/2" || echo "?")
  RET=$(python3 - "$STDOUT_LOG" <<'PY'
import re, sys
text = open(sys.argv[1]).read()
matches = re.findall(r'ret=([0-9.]+)/([0-9.]+)', text)
num = sum(float(a) for a, b in matches)
den = sum(float(b) for a, b in matches)
print(f"{100*num/den:.1f}%" if den else "?")
PY
)

  echo "| $KB | $RET | $GEN | $JUDGE | $(date +%Y-%m-%d) |" >> "$SUMMARY"
  write_progress "$KB" "done"
  log "✅ $KB: ret=$RET gen=$GEN judge=$JUDGE → $OUT"
done

python3 - "$PROGRESS" <<'PY'
import json, sys
path = sys.argv[1]
data = json.load(open(path))
data["current_kb"] = "ALL"
data["status"] = "complete"
json.dump(data, open(path, "w"), indent=2)
PY

log "Full eval complete. Summary: $SUMMARY"
