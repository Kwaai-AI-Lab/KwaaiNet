#!/usr/bin/env bash
# N-arm bake-off runner for competing p2p stack implementations.
#
# Runs several nodes side by side on one host and interleaves identical work
# across them, so network and remote-GPU drift cannot systematically favour one
# arm. Designed for three arms — the Go p2pd stack as the *control*, plus two
# candidate native implementations — but takes any number.
#
# It measures four things, in descending order of how much they should matter:
#
#   1. VISIBILITY  which peers each arm can see at all, versus the control.
#      This is the check that catches silent degradation: an arm that quietly
#      sees a smaller network than p2pd looks healthy on every other metric.
#   2. REACHABILITY per-peer unary success, versus the control.
#   3. RESOURCES   RSS/FD/thread drift over the run.
#   4. LATENCY     per-call cost. Reported, but the weakest signal — see
#      BAKEOFF.md on why performance must not decide this.
#
# It starts and stops nothing. Bring the nodes up yourself, then point it at
# them; it loads them from outside exactly as a real client would.
#
# Usage:
#   ./bakeoff.sh --arms arms.tsv --targets targets.tsv --rounds 40
#
# arms.tsv    TAB-separated: label<TAB>socket<TAB>binary<TAB>node_pid
#             `socket` is a path (unix) or multiaddr (Windows), or the literal
#             word `default` to use the platform default socket.
#             `node_pid` may be `-` to skip resource sampling for that arm.
# targets.tsv TAB-separated: os<TAB>name<TAB>peer_id   (extra columns ignored)

set -uo pipefail

ARMS=""
TARGETS=""
ROUNDS=20
OUT=""
TIMEOUT=25
SIZES="64 4096 65536 262144 1048576"

while [ $# -gt 0 ]; do
    case "$1" in
        --arms)     ARMS=$2; shift 2 ;;
        --targets)  TARGETS=$2; shift 2 ;;
        --rounds)   ROUNDS=$2; shift 2 ;;
        --out)      OUT=$2; shift 2 ;;
        --timeout)  TIMEOUT=$2; shift 2 ;;
        --sizes)    SIZES=$2; shift 2 ;;
        *) echo "unknown flag: $1" >&2; exit 2 ;;
    esac
done

[ -n "$ARMS" ] && [ -f "$ARMS" ]       || { echo "--arms <file> required" >&2; exit 2; }
[ -n "$TARGETS" ] && [ -f "$TARGETS" ] || { echo "--targets <file> required" >&2; exit 2; }

case "$(uname -s)" in
    Darwin) OSNAME=macos ;;
    Linux)  OSNAME=linux ;;
    MINGW*|MSYS*|CYGWIN*) OSNAME=windows ;;
    *)      OSNAME=unknown ;;
esac

OUT=${OUT:-./bakeoff-$(date +%Y%m%d-%H%M%S)}
mkdir -p "$OUT/payloads"
RESULTS="$OUT/results.jsonl"
PROGRESS="$OUT/progress.json"
: > "$RESULTS"

# ---- portable millisecond clock (Git Bash has no %3N; macOS has no %N) ------
if command -v perl >/dev/null 2>&1; then CLOCK=perl
elif date +%s%3N 2>/dev/null | grep -qE '^[0-9]+$'; then CLOCK=date
elif command -v python3 >/dev/null 2>&1; then CLOCK=python
else CLOCK=seconds; fi

now_ms() {
    case $CLOCK in
        perl)   perl -MTime::HiRes=time -e 'printf "%d\n", time()*1000' ;;
        date)   date +%s%3N ;;
        python) python3 -c 'import time;print(int(time.time()*1000))' ;;
        *)      echo $(( $(date +%s) * 1000 )) ;;
    esac
}

esc() { printf '%s' "$1" | tr -d '\000-\037' | sed 's/\\/\\\\/g; s/"/\\"/g'; }
emit() { printf '%s\n' "$1" >> "$RESULTS"; }

# ---- load arms and targets --------------------------------------------------
ARM_LABEL=(); ARM_SOCK=(); ARM_BIN=(); ARM_PID=()
while IFS=$'\t' read -r label sock bin pid; do
    case "$label" in ''|\#*) continue ;; esac
    ARM_LABEL+=("$label"); ARM_SOCK+=("$sock")
    ARM_BIN+=("${bin:-kwaainet}"); ARM_PID+=("${pid:--}")
done < "$ARMS"
NARMS=${#ARM_LABEL[@]}
[ "$NARMS" -ge 2 ] || { echo "need at least 2 arms (a control and a candidate)" >&2; exit 2; }

TGT="$OUT/targets.resolved"
grep -vE '^\s*(#|$)' "$TARGETS" | awk -F'\t' 'NF>=3 {print $1"\t"$2"\t"$3}' > "$TGT"
NTGT=$(wc -l < "$TGT" | tr -d ' ')
[ "$NTGT" -gt 0 ] || { echo "no usable targets" >&2; exit 2; }

for s in $SIZES; do
    f="$OUT/payloads/$s.bin"
    [ -f "$f" ] || head -c "$s" /dev/urandom > "$f" 2>/dev/null || \
        dd if=/dev/urandom of="$f" bs="$s" count=1 2>/dev/null
done

echo "bake-off: $NARMS arms x $NTGT targets, $ROUNDS rounds, host=$OSNAME clock=$CLOCK"
printf '  arms:'; for a in "${ARM_LABEL[@]}"; do printf ' %s' "$a"; done; echo
echo "  results -> $RESULTS"
echo "  NOTE: the first arm listed is treated as the control by scorecard.py"

# Record the arm order so the scorecard knows which arm is the control without
# being told again — a mislabelled control silently inverts every verdict.
#
# Also record each arm's *own* peer id. Arms running side by side see each other
# as ordinary peers but never see themselves, so without this the visibility
# comparison reports every arm as "missing" every other arm — a false veto that
# buries the real ones.
# Run one CLI invocation against arm $1, remaining args passed through.
# Defined before first use — the meta capture below calls it.
arm_run() {
    local i=$1; shift
    local sock=${ARM_SOCK[$i]} bin=${ARM_BIN[$i]}
    if [ "$sock" = "default" ]; then
        env -u KWAAINET_SOCKET "$bin" "$@" 2>&1
    else
        KWAAINET_SOCKET="$sock" "$bin" "$@" 2>&1
    fi
}

_arms_json=""; _self_json=""; _unknown=0
for i in $(seq 0 $((NARMS-1))); do
    self=$(arm_run "$i" p2p info | sed 's/\x1b\[[0-9;]*m//g' \
           | grep -oE '(12D3Koo[A-Za-z0-9]+|Qm[A-Za-z0-9]{44})' | head -1)
    [ -n "$_arms_json" ] && { _arms_json="$_arms_json,"; _self_json="$_self_json,"; }
    _arms_json="$_arms_json\"$(esc "${ARM_LABEL[$i]}")\""
    _self_json="$_self_json\"$(esc "${self:-unknown}")\""
    if [ -n "$self" ]; then
        echo "  arm ${ARM_LABEL[$i]} peer id: $self"
    else
        echo "  arm ${ARM_LABEL[$i]} peer id: UNKNOWN — is the node up on ${ARM_SOCK[$i]}?"
        _unknown=$(( _unknown + 1 ))
    fi
done
if [ "$_unknown" -gt 0 ]; then
    # Without every arm's own id the visibility comparison produces false vetoes,
    # which is worse than not running: it discredits the real findings.
    echo "ERROR: $_unknown arm(s) did not answer 'p2p info'. Fix the arms file or" >&2
    echo "       bring the nodes up before running — refusing to produce a" >&2
    echo "       scorecard that would report false visibility vetoes." >&2
    exit 3
fi
emit "{\"kind\":\"meta\",\"t\":$(date +%s),\"host_os\":\"$OSNAME\",\"rounds\":$ROUNDS,\"arms\":[$_arms_json],\"self_peers\":[$_self_json],\"control\":\"$(esc "${ARM_LABEL[0]}")\",\"targets\":$NTGT,\"sizes\":\"$(esc "$SIZES")\"}"

START=$(date +%s)

# ---- 1. VISIBILITY: which peers does each arm see at all? -------------------
# The silent-degradation check. Run every round so a slow-converging arm is
# distinguishable from one that never converges.
probe_visibility() {
    local round=$1 i out ids n
    for i in $(seq 0 $((NARMS-1))); do
        out=$(arm_run "$i" p2p peers list | sed 's/\x1b\[[0-9;]*m//g')
        ids=$(printf '%s' "$out" | grep -oE '(12D3Koo[A-Za-z0-9]+|Qm[A-Za-z0-9]{44})' \
              | sort -u | tr '\n' ',' | sed 's/,$//')
        n=$(printf '%s' "$ids" | tr ',' '\n' | grep -c . )
        emit "{\"kind\":\"visibility\",\"t\":$(date +%s),\"round\":$round,\"arm\":\"$(esc "${ARM_LABEL[$i]}")\",\"n_peers\":${n:-0},\"peers\":\"$ids\"}"
    done
}

# ---- 3. RESOURCES -----------------------------------------------------------
probe_resources() {
    local round=$1 i pid rss fds threads
    for i in $(seq 0 $((NARMS-1))); do
        pid=${ARM_PID[$i]}
        [ "$pid" = "-" ] && continue
        if ! kill -0 "$pid" 2>/dev/null; then
            emit "{\"kind\":\"resource\",\"t\":$(date +%s),\"round\":$round,\"arm\":\"$(esc "${ARM_LABEL[$i]}")\",\"alive\":false}"
            continue
        fi
        rss=$(ps -o rss= -p "$pid" 2>/dev/null | tr -d ' '); rss=${rss:-0}
        if [ -d "/proc/$pid/fd" ]; then
            fds=$(ls "/proc/$pid/fd" 2>/dev/null | wc -l | tr -d ' ')
            threads=$(awk '/^Threads:/{print $2}' "/proc/$pid/status" 2>/dev/null)
        elif command -v lsof >/dev/null 2>&1; then
            fds=$(lsof -p "$pid" 2>/dev/null | wc -l | tr -d ' ')
            threads=$(ps -M "$pid" 2>/dev/null | tail -n +2 | wc -l | tr -d ' ')
        fi
        emit "{\"kind\":\"resource\",\"t\":$(date +%s),\"round\":$round,\"arm\":\"$(esc "${ARM_LABEL[$i]}")\",\"alive\":true,\"rss_kb\":${rss:-0},\"fds\":${fds:-0},\"threads\":${threads:-0}}"
    done
}

# ---- 2 + 4. REACHABILITY and LATENCY ----------------------------------------
# Interleaved at the innermost loop: every arm makes the same call to the same
# peer back to back, so a drift in that peer's load hits all arms alike. Arm
# order rotates per round so no arm is permanently first (and therefore always
# paying the cold-path cost).
probe_rpc() {
    local round=$1 size payload line os name peer i idx t0 t1 ms out ok err
    # one payload size per round, cycled — keeps rounds short while still
    # sweeping the range across the run
    size=$(echo $SIZES | tr ' ' '\n' | awk -v n=$(( (round - 1) % $(echo $SIZES | wc -w) + 1 )) 'NR==n')
    payload="$OUT/payloads/$size.bin"
    while IFS=$'\t' read -r os name peer; do
        [ -z "$peer" ] && continue
        for idx in $(seq 0 $((NARMS-1))); do
            i=$(( (idx + round) % NARMS ))
            t0=$(now_ms)
            out=$(arm_run "$i" p2p peers send --peer "$peer" --payload-bin "$payload" --timeout "$TIMEOUT")
            t1=$(now_ms); ms=$(( t1 - t0 ))
            if printf '%s' "$out" | grep -qiE 'Response|\bok\b'; then
                ok=true; err=""
            else
                ok=false
                err=$(printf '%s' "$out" | sed 's/\x1b\[[0-9;]*m//g' \
                      | grep -oiE 'missing relay peer id|remote does not support protocol [^ ]*|timeout|timed out|no addresses|not connected|unexpected peer id|dial [a-z ]*failed|connection refused' \
                      | head -1)
                [ -n "$err" ] || err=$(printf '%s' "$out" | tr '\n' ' ' | cut -c1-100)
            fi
            emit "{\"kind\":\"rpc\",\"t\":$(date +%s),\"round\":$round,\"arm\":\"$(esc "${ARM_LABEL[$i]}")\",\"target\":\"$(esc "$name")\",\"target_os\":\"$os\",\"peer\":\"$peer\",\"bytes\":$size,\"ms\":$ms,\"ok\":$ok,\"err\":\"$(esc "$err")\"}"
        done
    done < "$TGT"
}

for round in $(seq 1 "$ROUNDS"); do
    probe_visibility "$round"
    probe_rpc "$round"
    probe_resources "$round"
    now=$(date +%s); elapsed=$(( now - START ))
    if [ "$round" -gt 0 ]; then per=$(( elapsed / round )); else per=0; fi
    cat > "$PROGRESS" <<EOF
{"round":$round,"rounds":$ROUNDS,"elapsed_s":$elapsed,
 "eta_s":$(( per * (ROUNDS - round) )),"arms":$NARMS,"targets":$NTGT,
 "rpc_total":$(grep -c '"kind":"rpc"' "$RESULTS" 2>/dev/null || echo 0)}
EOF
    echo "  round $round/$ROUNDS  elapsed ${elapsed}s"
done

echo "done -> $RESULTS"
echo "now run:  python3 scorecard.py $RESULTS"
