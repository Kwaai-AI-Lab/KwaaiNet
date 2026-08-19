#!/usr/bin/env bash
# D6 rebuild over the native-p2p transport, as an end-to-end test of the
# V1Lazy negotiation fix (PR #107) under sustained real load.
#
# Why a D6 rebuild is the right test: every chunk extraction is an LLM call
# relayed over p2p, and transport on this fabric costs ~160 ms per *token*
# (see project_p2p_relay_token_latency). The pre-fix native path paid that
# twice per token (750 vs 478 ms/tok), which would stretch the recorded
# 4792.9 s graph build to roughly 125 min. So wall clock is a direct read on
# whether V1Lazy is working, and the eval score is a read on whether native
# drops or corrupts calls under load — the silent-degradation failure mode
# that once discarded up to 94% of a corpus's chunks as false successes.
#
# Builds into D6_native. The production D6 knowledge base is never touched.
#
# Usage: bash tests/kwaai-knowledge/native_p2p_d6_rebuild.sh [/path/to/kwaainet]

set -uo pipefail
cd "$(dirname "$0")/../.."

BIN="${1:-kwaainet}"
KB=D6_native
DOC="docs/LEST WE FORGET -rev25.pdf"
SEED=tests/kwaai-knowledge/d6_family_tree.yaml
SCHEMA=tests/kwaai-knowledge/d6_doc_schema.yaml
QUESTIONS=tests/kwaai-knowledge/d6_eval_questions.json
RESULTS=tests/kwaai-knowledge/results
MODEL=llama3.1:8b
# Workers are per-run, not per-peer. The 0.24 chunks/s baseline ran 4 workers
# across 2 peers — 2 in flight per Ollama. Pointing 4 at a single peer
# overloads its Ollama and it answers 502/503 instead of extracting.
WORKERS="${WORKERS:-2}"

# The third peer in the canonical D6 command (12D3KooWDyPJBav…) is omitted
# throughout: it is long gone from the DHT, and an unreachable peer in the
# round-robin costs a circuit-breaker cooldown per cycle, which would corrupt
# the timing this run exists to measure.
METRO_LINUX="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs"
METRO_WIN="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"

# Peer set. metro-win only by default: as of 2026-08-18 metro-linux runs with
# reduced VRAM, so it cannot hold a deep pipeline of concurrent requests and
# sheds load as 502/503 rather than queueing it — 98 timeouts at 30 s plus
# 47x503 and 40x502 in 15 minutes, against a single failure to metro-win.
# Its *transport* is fine (the ollama-proxy handler answered a probe in 22.5 ms
# and it still advertised 32/32 blocks VERIFIED), which is exactly why this is
# worth writing down: left in the round-robin it dropped the build to 0.07
# chunks/s and failed chunks out of the graph entirely, which reads as an
# accuracy regression while having nothing to do with the transport under test.
#   PEERS=both  -> restore the two-peer set once metro-linux has VRAM again,
#                  and keep WORKERS low enough that it is not the bottleneck
PEERS="${PEERS:-metrowin}"
case "$PEERS" in
  both)     URLS="${METRO_LINUX},${METRO_WIN}" ;;
  metrowin) URLS="${METRO_WIN}" ;;
  metrolinux) URLS="${METRO_LINUX}" ;;
  *) echo "unknown PEERS=$PEERS (want: both|metrowin|metrolinux)" >&2; exit 2 ;;
esac

TS=$(date +%Y%m%d_%H%M%S)
RUN="${RESULTS}/native_p2p_d6_${TS}"
mkdir -p "${RESULTS}"
LOG="${RUN}.log"
PROGRESS="${RUN}.progress.json"

log() { echo "[$(date '+%H:%M:%S')] $*" | tee -a "$LOG"; }

# Phase telemetry, so the run can be watched without stdout access.
stamp() {
  local phase="$1" status="$2" started="$3"
  local now; now=$(date +%s)
  cat > "$PROGRESS" <<JSON
{"run":"${TS}","kb":"${KB}","transport":"native-p2p","phase":"${phase}",
 "status":"${status}","phase_elapsed_s":$((now - started)),
 "total_elapsed_s":$((now - RUN_START)),"updated_at":"$(date -u +%Y-%m-%dT%H:%M:%SZ)"}
JSON
}

RUN_START=$(date +%s)
log "=== D6 rebuild over native-p2p — run ${TS} ==="
log "binary:   ${BIN} ($("$BIN" --version 2>&1))"
log "kb:       ${KB}  (production D6 untouched)"
log "peers:    ${PEERS} (${URLS})"
log "workers:  ${WORKERS}"
log "baseline: 1152 chunks / 4792.9 s / 0.24 chunks per s / eval 90.4% (189.0 of 209)"

# --- 1. rebuild: destroy -> init -> ingest -> graph build -> seed -> reembed -> dedup -> score
P_START=$(date +%s); stamp rebuild running $P_START
log "-- phase 1/4: rag rebuild --"
"$BIN" rag rebuild "$DOC" --kb "$KB" \
  --model "$MODEL" \
  --inference-urls "$URLS" \
  --workers "$WORKERS" \
  --entity-types "Person,Place,Organization,Legislation,Publication" \
  --no-relations \
  --graph-window 1 \
  --timeline \
  --axiomatic-threshold 0.80 \
  --seed-file "$SEED" \
  --doc-schema "$SCHEMA" \
  --yes >> "$LOG" 2>&1
RC=$?
REBUILD_S=$(( $(date +%s) - P_START ))
stamp rebuild "exit_${RC}" $P_START
log "-- rebuild finished rc=${RC} in ${REBUILD_S}s --"
[ $RC -ne 0 ] && { log "!! rebuild failed — stopping before eval, results would be meaningless"; exit $RC; }

# --- 2. coref (rule-based) — part of the pipeline that produced the 90.4% baseline
P_START=$(date +%s); stamp coref running $P_START
log "-- phase 2/4: graph coref --"
"$BIN" rag graph coref --kb "$KB" --output "${RUN}_coref.md" --commit >> "$LOG" 2>&1
log "-- coref rc=$? in $(( $(date +%s) - P_START ))s --"

# --- 3. enrich entities (descriptions + gender) — also over p2p
P_START=$(date +%s); stamp enrich running $P_START
log "-- phase 3/4: graph enrich-entities --"
"$BIN" rag graph enrich-entities --kb "$KB" \
  --inference-urls "$URLS" --model "$MODEL" --workers "$WORKERS" --min-mentions 1 >> "$LOG" 2>&1
ENRICH_S=$(( $(date +%s) - P_START ))
log "-- enrich rc=$? in ${ENRICH_S}s --"

log "-- graph score --"
"$BIN" rag graph score --kb "$KB" 2>&1 | tee -a "$LOG" | grep -E "Overall:|Unknown type:" || true
"$BIN" rag graph stats --kb "$KB" 2>&1 | tee -a "$LOG" | grep -E "Entities:|Relations:" || true

# --- 4. eval — per-question results are written to the report, always kept
P_START=$(date +%s); stamp eval running $P_START
log "-- phase 4/4: eval --"
"$BIN" rag eval --kb "$KB" --questions "$QUESTIONS" --output "${RUN}_eval.md" >> "$LOG" 2>&1
EVAL_S=$(( $(date +%s) - P_START ))
stamp eval done $P_START

TOTAL_S=$(( $(date +%s) - RUN_START ))
log "=== complete in ${TOTAL_S}s (rebuild ${REBUILD_S}s, enrich ${ENRICH_S}s, eval ${EVAL_S}s) ==="
grep -E "Overall recall" "${RUN}_eval.md" 2>/dev/null | tee -a "$LOG"
log "report:   ${RUN}_eval.md"
log "log:      ${LOG}"
