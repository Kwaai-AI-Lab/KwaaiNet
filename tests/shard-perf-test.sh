#!/usr/bin/env bash
# =============================================================================
# Shard Performance Benchmark
# =============================================================================
#
# Tests the hypothesis: does inference throughput increase with more shard
# hosts (compute reduction wins over added network hops)?
#
# WHAT IT MEASURES
#   Runs N identical inference requests with --stats and parses:
#     - Prefill time (ms)
#     - Decode avg ms/tok  (the per-token forward-pass cost)
#     - Throughput (tok/s)
#     - Per-hop breakdown: which host, how many blocks, avg elapsed ms
#
# USAGE
#   # Ensure kwaainet is running and the DHT chain is reachable:
#   kwaainet start --daemon
#
#   bash tests/shard-perf-test.sh [--runs N] [--tokens T] [--filter NAME]
#
#   Examples:
#     bash tests/shard-perf-test.sh                      # default: 5 runs, 20 tokens
#     bash tests/shard-perf-test.sh --runs 10 --tokens 30
#     bash tests/shard-perf-test.sh --filter "john"      # only john's machines
#     bash tests/shard-perf-test.sh --filter "draak"     # force a specific host subset
#
# INTERPRETING RESULTS
#   If per-hop avg >> 50ms and scales with block count → compute-bound
#   If per-hop avg ≈ 50-100ms flat regardless of block count → network-bound
#
# =============================================================================

set -euo pipefail

KWAAINET="${KWAAINET:-./core/target/release/kwaainet}"
RUNS=5
MAX_TOKENS=20
NAME_FILTER=""
PROMPT="What is the capital of France? Answer in one sentence."

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✅ $*${RESET}"; }
fail() { echo -e "  ${RED}❌ $*${RESET}"; }
info() { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }
warn() { echo -e "  ${YELLOW}⚠  $*${RESET}"; }

# Parse flags
while [[ $# -gt 0 ]]; do
    case "$1" in
        --runs)   RUNS="$2"; shift 2;;
        --tokens) MAX_TOKENS="$2"; shift 2;;
        --filter) NAME_FILTER="$2"; shift 2;;
        --prompt) PROMPT="$2"; shift 2;;
        *) echo "Unknown flag: $1"; exit 1;;
    esac
done

# ---------------------------------------------------------------------------
# Pre-flight
# ---------------------------------------------------------------------------
step "Pre-flight"

if [ ! -x "$KWAAINET" ]; then
    echo -e "${RED}binary not found: $KWAAINET${RESET}"
    echo "Build with: cd core && cargo build --release -p kwaainet"
    exit 1
fi
pass "binary: $KWAAINET"

declare -a FILTER_ARGS=()
if [ -n "$NAME_FILTER" ]; then
    FILTER_ARGS=(--name-filter "$NAME_FILTER")
    info "Name filter: $NAME_FILTER"
fi

info "Runs: $RUNS  |  Max tokens: $MAX_TOKENS  |  Prompt: \"${PROMPT:0:60}...\""

# ---------------------------------------------------------------------------
# Show the chain that will be used
# ---------------------------------------------------------------------------
step "Chain coverage"

"$KWAAINET" shard chain ${FILTER_ARGS[@]+"${FILTER_ARGS[@]}"} 2>/dev/null || true

# ---------------------------------------------------------------------------
# Warm-up run (not counted)
# ---------------------------------------------------------------------------
step "Warm-up run"
info "Running warm-up (not counted in results)..."
"$KWAAINET" shard run "$PROMPT" --max-tokens 5 ${FILTER_ARGS[@]+"${FILTER_ARGS[@]}"} 2>/dev/null \
    | grep -v "^$" | tail -3 || true

# ---------------------------------------------------------------------------
# Benchmark runs
# ---------------------------------------------------------------------------
step "Benchmark ($RUNS runs, $MAX_TOKENS tokens each)"

PREFILL_TIMES=()
DECODE_AVGS=()
THROUGHPUTS=()
HOP_LINES=()

TMP_OUT=$(mktemp)

for i in $(seq 1 "$RUNS"); do
    printf "  Run %d/%d ... " "$i" "$RUNS"
    start_epoch=$(date +%s%N)

    "$KWAAINET" shard run "$PROMPT" \
        --max-tokens "$MAX_TOKENS" \
        --stats \
        ${FILTER_ARGS[@]+"${FILTER_ARGS[@]}"} \
        > "$TMP_OUT" 2>&1 || true

    end_epoch=$(date +%s%N)
    wall_ms=$(( (end_epoch - start_epoch) / 1000000 ))

    # Parse --stats output
    prefill=$(grep "Prefill:" "$TMP_OUT" | grep -oE '[0-9]+ms' | head -1 | tr -d 'ms' || echo "0")
    decode_avg=$(grep "Decode avg:" "$TMP_OUT" | grep -oE '[0-9]+ms' | head -1 | tr -d 'ms' || echo "0")
    tps=$(grep "Throughput:" "$TMP_OUT" | grep -oE '[0-9]+\.[0-9]+' | head -1 || echo "0")

    PREFILL_TIMES+=("$prefill")
    DECODE_AVGS+=("$decode_avg")
    THROUGHPUTS+=("$tps")

    # Capture per-hop lines (once — they're stable across runs with same path)
    if [ "${#HOP_LINES[@]}" -eq 0 ]; then
        while IFS= read -r line; do
            HOP_LINES+=("$line")
        done < <(grep -E '^\s+\[[0-9]\]' "$TMP_OUT" || true)
    fi

    printf "${GREEN}done${RESET}  (wall %dms, prefill=%sms, decode=%sms/tok, %.1f tok/s)\n" \
        "$wall_ms" "$prefill" "$decode_avg" "$tps"
done

rm -f "$TMP_OUT"

# ---------------------------------------------------------------------------
# Summary statistics
# ---------------------------------------------------------------------------
step "Results"

python3 - <<EOF
import statistics

prefill    = [float(x) for x in "${PREFILL_TIMES[*]}".split()]
decode_avg = [float(x) for x in "${DECODE_AVGS[*]}".split()]
tps        = [float(x) for x in "${THROUGHPUTS[*]}".split()]

def fmt(label, vals, unit):
    if not vals or all(v == 0 for v in vals):
        print(f"  {label:<22} n/a")
        return
    print(f"  {label:<22} mean={statistics.mean(vals):>7.1f}{unit}  "
          f"min={min(vals):>7.1f}  max={max(vals):>7.1f}  "
          f"stdev={statistics.stdev(vals):>5.1f}" if len(vals) > 1
          else f"  {label:<22} {vals[0]:.1f}{unit}")

print()
print(f"  {'Metric':<22} {'Value':>8}")
print("  " + "─"*50)
fmt("Prefill",        prefill,    "ms")
fmt("Decode avg",     decode_avg, "ms/tok")
fmt("Throughput",     tps,        " tok/s")
print()

if tps and all(t > 0 for t in tps):
    # Network overhead estimate: decode_avg ≈ compute + RTT_sum
    # If we had 1 hop we'd have 0 RTT → lower bound is compute only.
    # We can't separate them here but print the hypothesis framing.
    avg_decode = statistics.mean(decode_avg) if decode_avg else 0
    avg_tps    = statistics.mean(tps) if tps else 0
    print(f"  Each token takes ~{avg_decode:.0f}ms through the full chain.")
    print(f"  At {avg_tps:.2f} tok/s — per-hop breakdown below reveals where time is spent.")
EOF

# Per-hop breakdown
if [ "${#HOP_LINES[@]}" -gt 0 ]; then
    echo ""
    echo -e "  ${BOLD}Per-hop breakdown (decode avg, from last run):${RESET}"
    for line in "${HOP_LINES[@]}"; do
        echo "  $line"
    done
    echo ""
    echo -e "  ${CYAN}Interpretation:${RESET}"
    echo "    - hop ms >> 50ms and grows with block count  →  compute-bound  (sharding helps)"
    echo "    - hop ms ≈ flat 30–100ms regardless of blocks →  network-bound (sharding hurts)"
fi

# ---------------------------------------------------------------------------
# Pinned path summary
# ---------------------------------------------------------------------------
step "What this tells us"
echo ""
echo "  Run with subsets to compare hop counts:"
echo "    bash tests/shard-perf-test.sh --filter 'draak'    # fewer hops (if draak alone covers enough)"
echo "    bash tests/shard-perf-test.sh --filter 'vovin'    # force smaller first hop"
echo "    bash tests/shard-perf-test.sh                     # all hosts, widest-first (default)"
echo ""
echo "  Run with more tokens to get stable decode estimates:"
echo "    bash tests/shard-perf-test.sh --runs 3 --tokens 60"
echo ""
