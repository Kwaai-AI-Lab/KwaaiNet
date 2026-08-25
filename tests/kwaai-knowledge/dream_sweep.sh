#!/bin/bash
# Dream-cycle sweep across every knowledge base, inference over native p2p.
#
# Doubles as a native-p2p soak test: every completion is a unary call over
# /kwaai/ollama-proxy/1.0.0 to a remote peer, so a 48h run is tens of thousands
# of round trips through the native stack.
#
# Three things this guards against, all learned the hard way on 2026-08-22:
#
#   1. `--model` defaults to the literal string "default", which Ollama rejects.
#      Every completion then fails, and (before the fix in this branch) failed
#      silently. Always pass a real model name.
#   2. A cycle whose completions all fail used to still dedup/prune/sanitize —
#      one such cycle destroyed 14 of D6's 237 relations in 0.4s. The guard in
#      dream.rs now blocks that, but we still back off rather than spin.
#   3. Graphs are backed up before the first cycle of each KB regardless.
#
# Progress is written as JSON so it can be monitored without stdout.
#
# Usage: dream_sweep.sh [hours] [cycles_per_kb]

set -uo pipefail

K="${KWAAINET_BIN:-$HOME/.cargo/bin/kwaainet}"
MODEL="${DREAM_MODEL:-llama3.1:8b}"
PEER="${DREAM_PEER:-12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs}"   # metro-linux
BUDGET_HOURS="${1:-48}"
CYCLES_PER_KB="${2:-6}"
# metro-linux sheds concurrent load as 502/503 when VRAM is tight, so keep the
# per-peer worker count low rather than chasing throughput.
WORKERS="${DREAM_WORKERS:-2}"
MAX_COMPLETIONS="${DREAM_MAX_COMPLETIONS:-40}"

RESULTS="${DREAM_RESULTS:-$HOME/Source/KwaaiNet/tests/kwaai-knowledge/results}"
PROGRESS="$RESULTS/dream_sweep_progress.json"
LOG="$RESULTS/dream_sweep_$(date +%Y%m%d_%H%M%S).md"
BACKUPS="$HOME/.kwaainet/backup-native-test/dream-sweep-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$RESULTS" "$BACKUPS"

DEADLINE=$(python3 -c "import time;print(time.time() + $BUDGET_HOURS*3600)")
started=$(python3 -c "import time;print(time.time())")

# D6 first, as asked; the rest after. Ordered so the KBs that matter are
# reached before any time budget runs out.
KBS=$($K rag list 2>/dev/null | grep -oE '^\s+• [A-Za-z0-9_]+' | awk '{print $2}')
ORDERED="D6 $(echo "$KBS" | grep -vx 'D6' | tr '\n' ' ')"

stats() {  # $1=kb -> "entities relations"
    $K rag graph stats --kb "$1" 2>/dev/null \
        | awk '/Entities:/{e=$2} /Relations:/{r=$2} END{print e+0, r+0}'
}

progress() {  # $1=kb $2=cycle $3=phase
    local now; now=$(python3 -c "import time;print(time.time())")
    python3 - "$1" "$2" "$3" "$now" <<PYEOF > "$PROGRESS"
import json, sys
kb, cycle, phase, now = sys.argv[1], int(sys.argv[2]), sys.argv[3], float(sys.argv[4])
started, deadline = $started, $DEADLINE
print(json.dumps({
    "kb": kb, "cycle": cycle, "phase": phase,
    "kbs_total": $(echo "$ORDERED" | wc -w | tr -d ' '),
    "kbs_done": $KBS_DONE,
    "elapsed_s": round(now - started, 1),
    "remaining_s": round(max(0, deadline - now), 1),
    "cycles_run": $CYCLES_RUN,
    "completions_total": $COMPLETIONS_TOTAL,
    "relations_added_total": $RELATIONS_ADDED,
    "consecutive_dead_cycles": $DEAD_STREAK,
    "log": "$LOG",
}, indent=2))
PYEOF
}

KBS_DONE=0; CYCLES_RUN=0; COMPLETIONS_TOTAL=0; RELATIONS_ADDED=0; DEAD_STREAK=0

{
    echo "# Dream sweep — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo
    echo "- peer: \`$PEER\` (native p2p, /kwaai/ollama-proxy/1.0.0)"
    echo "- model: \`$MODEL\`  workers: $WORKERS  max-completions/cycle: $MAX_COMPLETIONS"
    echo "- budget: ${BUDGET_HOURS}h, up to $CYCLES_PER_KB cycles per KB"
    echo "- backups: \`$BACKUPS\`"
    echo
    echo "| time | kb | cycle | entities | relations | Δrel | summaries | types | rels added | secs |"
    echo "|---|---|---|---|---|---|---|---|---|---|"
} >> "$LOG"

for kb in $ORDERED; do
    now=$(python3 -c "import time;print(time.time())")
    if (( $(python3 -c "print(1 if $now >= $DEADLINE else 0)") )); then
        echo "budget exhausted before $kb" >> "$LOG"; break
    fi

    # Back up this KB's graph before touching it.
    progress "$kb" 0 "backup"
    for g in "$HOME"/.kwaainet/rag/"$kb"/graph-*.db; do
        [ -f "$g" ] && cp "$g" "$BACKUPS/$kb-$(basename "$g")" 2>/dev/null
    done

    for cycle in $(seq 1 "$CYCLES_PER_KB"); do
        now=$(python3 -c "import time;print(time.time())")
        (( $(python3 -c "print(1 if $now >= $DEADLINE else 0)") )) && break

        read -r e0 r0 <<<"$(stats "$kb")"
        progress "$kb" "$cycle" "dreaming"

        t0=$(python3 -c "import time;print(time.time())")
        out=$($K rag dream run --kb "$kb" \
                --inference-url "p2p://$PEER" \
                --model "$MODEL" \
                --workers "$WORKERS" \
                --max-completions "$MAX_COMPLETIONS" 2>&1)
        t1=$(python3 -c "import time;print(time.time())")

        read -r e1 r1 <<<"$(stats "$kb")"
        summ=$(grep -oE 'Summary completions: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$' || echo 0)
        typs=$(grep -oE 'Type completions: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$' || echo 0)
        rels=$(grep -oE 'Relations added: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$' || echo 0)
        secs=$(python3 -c "print(f'{$t1-$t0:.0f}')")

        CYCLES_RUN=$((CYCLES_RUN+1))
        COMPLETIONS_TOTAL=$((COMPLETIONS_TOTAL + summ + typs))
        RELATIONS_ADDED=$((RELATIONS_ADDED + rels))

        printf '| %s | %s | %d | %s | %s | %+d | %s | %s | %s | %s |\n' \
            "$(date -u +%H:%M:%S)" "$kb" "$cycle" "$e1" "$r1" "$((r1-r0))" \
            "$summ" "$typs" "$rels" "$secs" >> "$LOG"

        # A cycle that completed nothing means the endpoint is not answering.
        # Back off rather than burning the budget on a dead peer.
        if [ "$((summ+typs))" -eq 0 ]; then
            DEAD_STREAK=$((DEAD_STREAK+1))
            echo "  - no completions (streak $DEAD_STREAK) — backing off 120s" >> "$LOG"
            progress "$kb" "$cycle" "backoff"
            sleep 120
            [ "$DEAD_STREAK" -ge 5 ] && { echo "  - 5 dead cycles, moving to next KB" >> "$LOG"; break; }
        else
            DEAD_STREAK=0
        fi
    done
    KBS_DONE=$((KBS_DONE+1))
    progress "$kb" 0 "kb-done"
done

progress "-" 0 "finished"
{
    echo
    echo "## Totals"
    echo "- cycles: $CYCLES_RUN"
    echo "- completions: $COMPLETIONS_TOTAL"
    echo "- relations added: $RELATIONS_ADDED"
    echo "- KBs processed: $KBS_DONE"
} >> "$LOG"
