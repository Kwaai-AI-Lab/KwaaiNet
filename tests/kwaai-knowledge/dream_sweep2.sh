#!/bin/bash
# Dream sweep, second pass — adaptive, after what pass 1 taught us.
#
# Pass 1 (dream_sweep.sh) ran a fixed 6 cycles per KB across all 23. Three
# things came out of it that this pass acts on:
#
#   1. **Yield tracks corpus type, not graph size.** Narrative and biographical
#      corpora (D6 memoir, CountryHistory) produce 16-31 completions a cycle;
#      scientific ones (Climate, Astrophysics) produce two and then nothing.
#      Dream's relation vocabulary is person/organisation-centric, so papers
#      about permafrost simply do not contain the relations it asks for. Those
#      KBs are not retried here — they need a domain relation schema, which is
#      a design decision, not more compute.
#
#   2. **A fixed cycle cap is wrong in both directions.** CountryHistory was
#      still adding 11 relations on cycle 6 when the cap stopped it; empty test
#      fixtures burned 12 minutes each in backoff. This pass keeps going while a
#      KB is productive and drops it the moment it is not.
#
#   3. **Empty KBs must be skipped outright**, not discovered by failing.
#
# Two bugs in pass 1's script are fixed: DEAD_STREAK is now per-KB (it was
# global, so a KB following a stalled one could be abandoned on its first
# cycle), and there is no backoff sleep when the KB has nothing to work on.
#
# Usage: dream_sweep2.sh [hours]

set -uo pipefail

K="${KWAAINET_BIN:-$HOME/.kwaainet/bin/kwaainet-dream}"
MODEL="${DREAM_MODEL:-llama3.1:8b}"
PEER="${DREAM_PEER:-12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs}"
BUDGET_HOURS="${1:-40}"
WORKERS="${DREAM_WORKERS:-2}"
MAX_COMPLETIONS="${DREAM_MAX_COMPLETIONS:-40}"
MAX_CYCLES_PER_KB="${DREAM_MAX_CYCLES:-40}"
# A cycle adding fewer than this many completions means the KB is done for now.
PRODUCTIVE_MIN="${DREAM_PRODUCTIVE_MIN:-3}"

# Narrative/biographical corpora only — see note 1 above.
CANDIDATES="${DREAM_KBS:-D6 CountryHistory Manhattan DeepSea WarPeace MobyDick Legal Poems DreamMem Meetings}"

RESULTS="${DREAM_RESULTS:-$HOME/Source/KwaaiNet/tests/kwaai-knowledge/results}"
PROGRESS="$RESULTS/dream_sweep_progress.json"
LOG="$RESULTS/dream_sweep2_$(date +%Y%m%d_%H%M%S).md"
BACKUPS="$HOME/.kwaainet/backup-native-test/dream-sweep2-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$RESULTS" "$BACKUPS"

DEADLINE=$(python3 -c "import time;print(time.time() + $BUDGET_HOURS*3600)")
started=$(python3 -c "import time;print(time.time())")
CYCLES_RUN=0; COMPLETIONS_TOTAL=0; RELATIONS_ADDED=0; KBS_DONE=0

stats() { $K rag graph stats --kb "$1" 2>/dev/null \
    | awk '/Entities:/{e=$2} /Relations:/{r=$2} END{print e+0, r+0}'; }

progress() {
    local now; now=$(python3 -c "import time;print(time.time())")
    python3 - "$1" "$2" "$3" "$now" "$4" <<PYEOF > "$PROGRESS"
import json, sys
kb, cycle, phase, now, streak = sys.argv[1], int(sys.argv[2]), sys.argv[3], float(sys.argv[4]), int(sys.argv[5])
print(json.dumps({
    "pass": 2, "kb": kb, "cycle": cycle, "phase": phase,
    "kbs_done": $KBS_DONE,
    "elapsed_s": round(now - $started, 1),
    "remaining_s": round(max(0, $DEADLINE - now), 1),
    "cycles_run": $CYCLES_RUN,
    "completions_total": $COMPLETIONS_TOTAL,
    "relations_added_total": $RELATIONS_ADDED,
    "consecutive_dead_cycles": streak,
    "log": "$LOG",
}, indent=2))
PYEOF
}

{
    echo "# Dream sweep, pass 2 — $(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo
    echo "Adaptive: cycles a KB while it stays productive (>= $PRODUCTIVE_MIN completions),"
    echo "up to $MAX_CYCLES_PER_KB. Narrative corpora only — see script header for why."
    echo
    echo "- peer: \`$PEER\` (native p2p)   model: \`$MODEL\`   workers: $WORKERS"
    echo "- backups: \`$BACKUPS\`"
    echo
    echo "| time | kb | cycle | entities | relations | Δrel | summaries | rels added | secs |"
    echo "|---|---|---|---|---|---|---|---|---|"
} >> "$LOG"

for kb in $CANDIDATES; do
    now=$(python3 -c "import time;print(time.time())")
    (( $(python3 -c "print(1 if $now >= $DEADLINE else 0)") )) && { echo "budget exhausted before $kb" >> "$LOG"; break; }

    read -r e0 _ <<<"$(stats "$kb")"
    if [ "${e0:-0}" -eq 0 ]; then
        echo "- skipped \`$kb\` — empty graph" >> "$LOG"
        continue
    fi

    progress "$kb" 0 "backup" 0
    for g in "$HOME"/.kwaainet/rag/"$kb"/graph-*.db; do
        [ -f "$g" ] && cp "$g" "$BACKUPS/$kb-$(basename "$g")" 2>/dev/null
    done

    streak=0   # per-KB, deliberately: pass 1 carried this across KBs
    for cycle in $(seq 1 "$MAX_CYCLES_PER_KB"); do
        now=$(python3 -c "import time;print(time.time())")
        (( $(python3 -c "print(1 if $now >= $DEADLINE else 0)") )) && break

        read -r _ r0 <<<"$(stats "$kb")"
        progress "$kb" "$cycle" "dreaming" "$streak"

        t0=$(python3 -c "import time;print(time.time())")
        out=$($K rag dream run --kb "$kb" --inference-url "p2p://$PEER" --model "$MODEL" \
                --workers "$WORKERS" --max-completions "$MAX_COMPLETIONS" 2>&1)
        t1=$(python3 -c "import time;print(time.time())")

        read -r e1 r1 <<<"$(stats "$kb")"
        summ=$(grep -oE 'Summary completions: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$'); summ=${summ:-0}
        typs=$(grep -oE 'Type completions: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$'); typs=${typs:-0}
        rels=$(grep -oE 'Relations added: +[0-9]+' <<<"$out" | grep -oE '[0-9]+$'); rels=${rels:-0}
        did=$((summ + typs))

        CYCLES_RUN=$((CYCLES_RUN+1)); COMPLETIONS_TOTAL=$((COMPLETIONS_TOTAL+did)); RELATIONS_ADDED=$((RELATIONS_ADDED+rels))
        printf '| %s | %s | %d | %s | %s | %+d | %s | %s | %s |\n' \
            "$(date -u +%H:%M:%S)" "$kb" "$cycle" "$e1" "$r1" "$((r1-r0))" "$summ" "$rels" \
            "$(python3 -c "print(f'{$t1-$t0:.0f}')")" >> "$LOG"

        if [ "$did" -lt "$PRODUCTIVE_MIN" ]; then
            streak=$((streak+1))
            if [ "$did" -eq 0 ]; then
                # Distinguish "nothing left to do" from "peer is down": only the
                # latter is worth waiting for.
                if $K p2p probe --peer "$PEER" --proto /kwaai/ollama-proxy/1.0.0 --count 1 --timeout 15 >/dev/null 2>&1; then
                    echo "  - \`$kb\` exhausted (peer healthy) — next KB" >> "$LOG"
                    break
                fi
                echo "  - peer not answering — waiting 300s" >> "$LOG"
                progress "$kb" "$cycle" "peer-down" "$streak"
                sleep 300
            fi
            [ "$streak" -ge 3 ] && { echo "  - \`$kb\` below threshold 3x — next KB" >> "$LOG"; break; }
        else
            streak=0
        fi
    done
    KBS_DONE=$((KBS_DONE+1))
done

progress "-" 0 "finished" 0
{
    echo
    echo "## Totals"
    echo "- cycles: $CYCLES_RUN"
    echo "- completions: $COMPLETIONS_TOTAL"
    echo "- relations added: $RELATIONS_ADDED"
    echo "- KBs processed: $KBS_DONE"
} >> "$LOG"
