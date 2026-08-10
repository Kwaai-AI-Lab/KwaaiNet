#!/usr/bin/env bash
# Cross-OS stress agent for the native-p2p path.
#
# Runs on macOS, Linux and Windows (Git Bash). Drives sustained unary RPC at a
# list of peers, samples the node's resource use, and writes two files:
#
#   <out>/results.jsonl   one line per RPC and per resource sample
#   <out>/progress.json   rewritten every 15s so a supervisor can watch ETA
#
# It does not start or stop the node under test — point it at a node that is
# already running and it will hammer it from the outside, exactly as a real
# client would.
#
# Usage:
#   ./stress_agent.sh --targets targets.tsv --duration 3600 --label metro-win
#
# targets.tsv is TAB-separated: os<TAB>name<TAB>peer_id  (extra columns ignored)

set -uo pipefail

BIN=${BIN:-kwaainet}
SOCKET=${KWAAINET_SOCKET:-}
DURATION=1800
WORKERS=4
OUT=""
LABEL=""
TARGETS=""
NODE_PID=""
TIMEOUT=25

while [ $# -gt 0 ]; do
    case "$1" in
        --bin)       BIN=$2; shift 2 ;;
        --targets)   TARGETS=$2; shift 2 ;;
        --duration)  DURATION=$2; shift 2 ;;
        --workers)   WORKERS=$2; shift 2 ;;
        --out)       OUT=$2; shift 2 ;;
        --label)     LABEL=$2; shift 2 ;;
        --node-pid)  NODE_PID=$2; shift 2 ;;
        --timeout)   TIMEOUT=$2; shift 2 ;;
        *) echo "unknown flag: $1" >&2; exit 2 ;;
    esac
done

[ -n "$TARGETS" ] || { echo "--targets is required" >&2; exit 2; }
[ -f "$TARGETS" ] || { echo "no such targets file: $TARGETS" >&2; exit 2; }

# ---- platform detection -----------------------------------------------------
case "$(uname -s)" in
    Darwin)                     OSNAME=macos   ;;
    Linux)                      OSNAME=linux   ;;
    MINGW*|MSYS*|CYGWIN*)       OSNAME=windows ;;
    *)                          OSNAME=unknown ;;
esac

LABEL=${LABEL:-$(hostname 2>/dev/null || echo node)-$OSNAME}
OUT=${OUT:-./stress-$LABEL}
mkdir -p "$OUT/payloads"
RESULTS="$OUT/results.jsonl"
PROGRESS="$OUT/progress.json"
: > "$RESULTS"

# ---- millisecond clock, with fallbacks for every platform -------------------
# Git Bash has no `date +%s%3N`; macOS `date` has no %N at all. Pick once.
if date +%s%3N 2>/dev/null | grep -qE '^[0-9]+$' && [ "$(date +%s%3N)" != "$(date +%s)%3N" ]; then
    CLOCK=date
elif command -v perl >/dev/null 2>&1; then
    CLOCK=perl
elif command -v python3 >/dev/null 2>&1; then
    CLOCK=python
else
    CLOCK=seconds
fi

now_ms() {
    case $CLOCK in
        date)   date +%s%3N ;;
        perl)   perl -MTime::HiRes=time -e 'printf "%d\n", time()*1000' ;;
        python) python3 -c 'import time;print(int(time.time()*1000))' ;;
        *)      echo $(( $(date +%s) * 1000 )) ;;
    esac
}

json_escape() { printf '%s' "$1" | tr -d '\000-\037' | sed 's/\\/\\\\/g; s/"/\\"/g'; }

# ---- payload sweep ----------------------------------------------------------
# Sizes chosen to straddle the yamux window and the 1MiB the interop tier proved.
SIZES="64 4096 65536 262144 1048576"
for s in $SIZES; do
    f="$OUT/payloads/$s.bin"
    [ -f "$f" ] || head -c "$s" /dev/urandom > "$f" 2>/dev/null || \
        dd if=/dev/urandom of="$f" bs="$s" count=1 2>/dev/null
done

# ---- targets ----------------------------------------------------------------
TGT_LIST="$OUT/targets.resolved"
grep -vE '^\s*(#|$)' "$TARGETS" | awk -F'\t' 'NF>=3 {print $1"\t"$2"\t"$3}' > "$TGT_LIST"
NTARGETS=$(wc -l < "$TGT_LIST" | tr -d ' ')
[ "$NTARGETS" -gt 0 ] || { echo "targets file has no usable rows" >&2; exit 2; }

echo "stress agent: label=$LABEL os=$OSNAME targets=$NTARGETS workers=$WORKERS duration=${DURATION}s clock=$CLOCK"
echo "results -> $RESULTS"

START=$(date +%s)
END=$(( START + DURATION ))

emit() { printf '%s\n' "$1" >> "$RESULTS"; }

# ---- worker: round-robin over targets x payload sizes -----------------------
worker() {
    local wid=$1 i=0
    local sizes_arr line os name pid size payload t0 t1 ms out ok err
    while [ "$(date +%s)" -lt "$END" ]; do
        # stagger workers across the target list so they do not all hit one peer
        line=$(awk -v n=$(( (i * WORKERS + wid) % NTARGETS + 1 )) 'NR==n' "$TGT_LIST")
        os=$(printf '%s' "$line" | cut -f1)
        name=$(printf '%s' "$line" | cut -f2)
        pid=$(printf '%s' "$line" | cut -f3)
        size=$(echo $SIZES | tr ' ' '\n' | awk -v n=$(( i % 5 + 1 )) 'NR==n')
        payload="$OUT/payloads/$size.bin"

        t0=$(now_ms)
        out=$("$BIN" p2p peers send --peer "$pid" --payload-bin "$payload" \
                     --timeout "$TIMEOUT" 2>&1)
        t1=$(now_ms)
        ms=$(( t1 - t0 ))

        if printf '%s' "$out" | grep -qiE 'Response|\bok\b'; then
            ok=true; err=""
        else
            ok=false
            err=$(printf '%s' "$out" \
                  | grep -oiE 'remote does not support protocol [^ ]*|timeout|timed out|no addresses|not connected|dial [a-z ]*failed|connection refused|[a-z ]*error[^,]*' \
                  | head -1)
            [ -n "$err" ] || err=$(printf '%s' "$out" | tr '\n' ' ' | cut -c1-120)
        fi

        emit "{\"t\":$(date +%s),\"kind\":\"rpc\",\"agent\":\"$(json_escape "$LABEL")\",\"agent_os\":\"$OSNAME\",\"target\":\"$(json_escape "$name")\",\"target_os\":\"$os\",\"peer\":\"$pid\",\"bytes\":$size,\"ms\":$ms,\"ok\":$ok,\"err\":\"$(json_escape "$err")\"}"
        i=$(( i + 1 ))
    done
}

# ---- sampler: node resource use over time -----------------------------------
sampler() {
    local rss threads fds
    while [ "$(date +%s)" -lt "$END" ]; do
        if [ -n "$NODE_PID" ] && kill -0 "$NODE_PID" 2>/dev/null; then
            rss=$(ps -o rss= -p "$NODE_PID" 2>/dev/null | tr -d ' '); rss=${rss:-0}
            if [ -d "/proc/$NODE_PID/fd" ]; then
                fds=$(ls "/proc/$NODE_PID/fd" 2>/dev/null | wc -l | tr -d ' ')
                threads=$(awk '/^Threads:/{print $2}' "/proc/$NODE_PID/status" 2>/dev/null)
            elif command -v lsof >/dev/null 2>&1; then
                fds=$(lsof -p "$NODE_PID" 2>/dev/null | wc -l | tr -d ' ')
                threads=$(ps -M "$NODE_PID" 2>/dev/null | tail -n +2 | wc -l | tr -d ' ')
            fi
            emit "{\"t\":$(date +%s),\"kind\":\"sample\",\"agent\":\"$(json_escape "$LABEL")\",\"agent_os\":\"$OSNAME\",\"rss_kb\":${rss:-0},\"fds\":${fds:-0},\"threads\":${threads:-0},\"alive\":true}"
        elif [ -n "$NODE_PID" ]; then
            emit "{\"t\":$(date +%s),\"kind\":\"sample\",\"agent\":\"$(json_escape "$LABEL")\",\"agent_os\":\"$OSNAME\",\"alive\":false}"
        fi
        sleep 15
    done
}

# ---- progress: so a supervisor can watch speed and ETA ----------------------
progress() {
    local now elapsed total ok fail pct rate eta
    while [ "$(date +%s)" -lt "$END" ]; do
        now=$(date +%s); elapsed=$(( now - START ))
        total=$(grep -c '"kind":"rpc"' "$RESULTS" 2>/dev/null); total=${total:-0}
        ok=$(grep -c '"ok":true' "$RESULTS" 2>/dev/null); ok=${ok:-0}
        fail=$(( total - ok ))
        # bash evaluates both arms of `? :`, so guard the divisors with `if`
        if [ "$DURATION" -gt 0 ]; then pct=$(( elapsed * 100 / DURATION )); else pct=0; fi
        if [ "$elapsed" -gt 0 ]; then rate=$(( total / elapsed )); else rate=0; fi
        eta=$(( END - now ))
        cat > "$PROGRESS" <<EOF
{"label":"$LABEL","os":"$OSNAME","started":$START,"elapsed_s":$elapsed,
 "duration_s":$DURATION,"pct":$pct,"eta_s":$eta,
 "rpc_total":$total,"rpc_ok":$ok,"rpc_fail":$fail,"rpc_per_s":$rate}
EOF
        sleep 15
    done
}

w=0
while [ "$w" -lt "$WORKERS" ]; do worker "$w" & w=$(( w + 1 )); done
sampler &
progress &
wait

# ---- final summary ----------------------------------------------------------
TOTAL=$(grep -c '"kind":"rpc"' "$RESULTS" 2>/dev/null); TOTAL=${TOTAL:-0}
OK=$(grep -c '"ok":true' "$RESULTS" 2>/dev/null); OK=${OK:-0}
echo "done: $OK/$TOTAL ok over ${DURATION}s -> $RESULTS"
cat > "$PROGRESS" <<EOF
{"label":"$LABEL","os":"$OSNAME","started":$START,"elapsed_s":$DURATION,
 "duration_s":$DURATION,"pct":100,"eta_s":0,
 "rpc_total":$TOTAL,"rpc_ok":$OK,"rpc_fail":$(( TOTAL - OK )),"finished":true}
EOF
