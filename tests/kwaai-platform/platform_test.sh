#!/usr/bin/env bash
# =============================================================================
# kwaai-platform — integration test suite with performance metrics
# =============================================================================
#
# Tests the full lifecycle of a KwaaiNet node and measures key performance
# metrics.  Primary target: node announced to DHT in ≤10 seconds.
#
# USAGE
#   bash tests/kwaai-platform/platform_test.sh
#
#   KEEP_RUNNING=1   leave node running after tests
#   SKIP_MAP_CHECK=1 skip map.kwaai.ai visibility check (useful offline / CI)
#   TARGET_SECS=10   startup time target (default: 10)
#
# EXIT CODES
#   0  all assertions passed
#   1  one or more assertions failed

# Do NOT use set -e — grep exits 1 on no-match and we want to handle that.
set -uo pipefail

# ---------------------------------------------------------------------------
# Colour + result helpers
# ---------------------------------------------------------------------------
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
BLUE='\033[0;34m'; BOLD='\033[1m'; NC='\033[0m'

FAILURES=0
METRICS=()

pass()   { echo -e "  ${GREEN}PASS${NC}  $*"; }
fail()   { echo -e "  ${RED}FAIL${NC}  $*"; FAILURES=$((FAILURES + 1)); }
info()   { echo -e "  ${YELLOW}....${NC}  $*"; }
header() { echo -e "\n${BOLD}${BLUE}── $* ──${NC}"; }
metric() { METRICS+=("$*"); echo -e "  ${BOLD}PERF${NC}  $*"; }

# Millisecond timestamp via python3 (portable; avoids %N quirks)
now_ms() { python3 -c "import time; print(int(time.time()*1000))"; }

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------
KWAAINET_BIN="${KWAAINET_BIN:-$(which kwaainet 2>/dev/null || echo '')}"
TARGET_SECS="${TARGET_SECS:-10}"
SKIP_MAP_CHECK="${SKIP_MAP_CHECK:-}"
KEEP_RUNNING="${KEEP_RUNNING:-}"
WAS_RUNNING=0

# ---------------------------------------------------------------------------
# 0. Prerequisites
# ---------------------------------------------------------------------------
header "0. Prerequisites"

if [[ -z "$KWAAINET_BIN" || ! -x "$KWAAINET_BIN" ]]; then
    fail "kwaainet binary not found — install with: cargo build -p kwaainet --release && cp target/release/kwaainet ~/.cargo/bin/"
    exit 1
fi
pass "kwaainet binary: $KWAAINET_BIN"

KWAAINET_VERSION=$("$KWAAINET_BIN" --version 2>/dev/null || echo "unknown")
info "version: $KWAAINET_VERSION"

P2PD_BIN="$(dirname "$KWAAINET_BIN")/p2pd"
if [[ -x "$P2PD_BIN" ]]; then
    pass "p2pd binary: $P2PD_BIN"
else
    fail "p2pd not found at $P2PD_BIN — run: kwaainet setup --get-deps"
    exit 1
fi

CONFIG_FILE="$HOME/.kwaainet/config.yaml"
if [[ -f "$CONFIG_FILE" ]]; then
    pass "config file: $CONFIG_FILE"
else
    fail "config not found at $CONFIG_FILE"
    exit 1
fi

if grep -q "initial_peers" "$CONFIG_FILE" 2>/dev/null; then
    pass "bootstrap peers configured"
else
    fail "no initial_peers in config.yaml"
fi

# ---------------------------------------------------------------------------
# 1. Lifecycle — stop any running node to get a clean start
# ---------------------------------------------------------------------------
header "1. Node lifecycle — clean start"

STATUS_OUT=$("$KWAAINET_BIN" status 2>/dev/null || echo "")
if echo "$STATUS_OUT" | grep -q "Status.*Running"; then
    WAS_RUNNING=1
    info "Stopping existing node for clean start..."
    "$KWAAINET_BIN" stop 2>/dev/null || true
    # Wait for clean stop (max 10s)
    for i in $(seq 1 10); do
        STATUS_CHECK=$("$KWAAINET_BIN" status 2>/dev/null || echo "")
        if ! echo "$STATUS_CHECK" | grep -q "Status.*Running"; then
            pass "node stopped cleanly (${i}s)"
            break
        fi
        sleep 1
        if [[ $i -eq 10 ]]; then
            fail "node did not stop within 10s"
        fi
    done
    # Brief pause to let sockets close
    sleep 1
else
    info "node not running — starting fresh"
fi

# ---------------------------------------------------------------------------
# 2. Startup performance
# ---------------------------------------------------------------------------
header "2. Startup performance (target ≤${TARGET_SECS}s to DHT announce)"

info "Starting node in daemon mode..."
T_START=$(now_ms)
"$KWAAINET_BIN" start --daemon
T_STARTED=$(now_ms)
STARTUP_MS=$((T_STARTED - T_START))
metric "kwaainet start --daemon returned: ${STARTUP_MS}ms"

if [[ $? -ne 0 ]]; then
    fail "kwaainet start --daemon exited non-zero"
fi

# Wait for Running status (max 15s)
T_RUNNING=""
for i in $(seq 1 30); do
    STATUS_CHECK=$("$KWAAINET_BIN" status 2>/dev/null || echo "")
    if echo "$STATUS_CHECK" | grep -q "Status.*Running"; then
        T_NOW=$(now_ms)
        RUNNING_MS=$((T_NOW - T_START))
        metric "status=Running: ${RUNNING_MS}ms from start"
        T_RUNNING="$T_NOW"
        pass "node reached Running state (${RUNNING_MS}ms)"
        break
    fi
    sleep 0.5
    if [[ $i -eq 30 ]]; then
        fail "node did not reach Running state within 15s"
    fi
done

# Wait for DHT announcement in logs (max 20s)
T_ANNOUNCED=""
for i in $(seq 1 40); do
    LOGS=$("$KWAAINET_BIN" logs 2>/dev/null || echo "")
    if echo "$LOGS" | grep -q "Announced.*bootstrap peers"; then
        T_NOW=$(now_ms)
        ANNOUNCE_MS=$((T_NOW - T_START))
        ANNOUNCE_SECS_INT=$((ANNOUNCE_MS / 1000))
        ANNOUNCE_SECS_DEC=$((ANNOUNCE_MS % 1000 / 100))
        metric "DHT announced: ${ANNOUNCE_MS}ms (${ANNOUNCE_SECS_INT}.${ANNOUNCE_SECS_DEC}s) from process start"
        T_ANNOUNCED="$T_NOW"

        if [[ $ANNOUNCE_MS -le $((TARGET_SECS * 1000)) ]]; then
            pass "startup ≤${TARGET_SECS}s ✓ announced in ${ANNOUNCE_SECS_INT}.${ANNOUNCE_SECS_DEC}s"
        else
            fail "startup exceeded ${TARGET_SECS}s target: took ${ANNOUNCE_SECS_INT}.${ANNOUNCE_SECS_DEC}s"
        fi
        # Brief pause so the remaining announcement log messages flush to disk
        # before we read logs in sections 3–5 (they appear within ~300ms of each other).
        sleep 1
        break
    fi
    sleep 0.5
    if [[ $i -eq 40 ]]; then
        fail "DHT announcement not found in logs within 20s"
    fi
done

# ---------------------------------------------------------------------------
# 3. Node info assertions
# ---------------------------------------------------------------------------
header "3. Node info"

STATUS_OUT=$("$KWAAINET_BIN" status 2>/dev/null || echo "")
FRESH_LOGS=$("$KWAAINET_BIN" logs 2>/dev/null || echo "")

if echo "$STATUS_OUT" | grep -q "Status.*Running"; then
    pass "status=Running"
else
    fail "status not Running"
fi

DAEMON_PID=$(echo "$STATUS_OUT" | grep -oE "PID[^0-9]*([0-9]+)" | grep -oE "[0-9]+" | head -1 || echo "")
if [[ -n "$DAEMON_PID" ]]; then
    pass "daemon PID: $DAEMON_PID"
else
    fail "could not read daemon PID from status"
fi

PEER_ID=$(echo "$FRESH_LOGS" | grep -oE "12D3KooW[A-Za-z0-9]+" | head -1 || echo "")
if [[ -n "$PEER_ID" ]]; then
    pass "peer ID: $PEER_ID"
else
    fail "peer ID not found in logs"
fi

if echo "$FRESH_LOGS" | grep -q "relay.*true\|Using relay: true"; then
    pass "relay mode active (NAT traversal)"
else
    info "relay not detected (node may be directly reachable)"
fi

# ---------------------------------------------------------------------------
# 4. DHT announcement correctness
# ---------------------------------------------------------------------------
header "4. DHT announcement"

FRESH_LOGS=$("$KWAAINET_BIN" logs 2>/dev/null || echo "")

BLOCKS_ANNOUNCED=$(echo "$FRESH_LOGS" | grep -oE "Announced [0-9]+ blocks" | grep -oE "[0-9]+" | tail -1 || echo "")
if [[ "${BLOCKS_ANNOUNCED:-}" == "32" ]]; then
    pass "32 blocks announced to DHT"
elif [[ -n "${BLOCKS_ANNOUNCED:-}" ]]; then
    fail "expected 32 blocks, got $BLOCKS_ANNOUNCED"
else
    fail "block announcement not found in logs"
fi

# grep -c exits 1 when count is 0, so use || true (count is already in stdout)
B1_STORED=$(echo "$FRESH_LOGS" | grep "QmQhRuhe" | { grep -c "stored" || true; })
B2_STORED=$(echo "$FRESH_LOGS" | grep "Qmd3A8N5" | { grep -c "stored" || true; })
if [[ "${B1_STORED:-0}" -gt 0 && "${B2_STORED:-0}" -gt 0 ]]; then
    pass "both bootstrap peers confirmed stores"
else
    fail "not all bootstrap peers confirmed stores (bootstrap-1: ${B1_STORED:-0}, bootstrap-2: ${B2_STORED:-0})"
fi

if echo "$FRESH_LOGS" | grep -q "_petals.models registry"; then
    pass "_petals.models registry announced"
else
    fail "_petals.models announcement not found"
fi

if echo "$FRESH_LOGS" | grep -q "VPK capability"; then
    pass "VPK capability announced to _kwaai.vpk.nodes"
else
    info "VPK capability not announced (may be disabled)"
fi

# ---------------------------------------------------------------------------
# 5. RPC handler correctness
# ---------------------------------------------------------------------------
header "5. RPC handler"

STORE_CALLS=$(echo "$FRESH_LOGS" | { grep -c "RPC DHTProtocol.rpc_store" || true; })
STORE_ERRORS=$(echo "$FRESH_LOGS" | { grep -c "RPC handler error\|decode StoreRequest\|decode Persistent" || true; })

if [[ "${STORE_CALLS:-0}" -gt 0 ]]; then
    if [[ "${STORE_ERRORS:-0}" -eq 0 ]]; then
        pass "rpc_store: $STORE_CALLS call(s), 0 errors"
    else
        fail "rpc_store errors: $STORE_ERRORS errors in $STORE_CALLS calls"
    fi
else
    info "no rpc_store calls yet (bootstrap peers may not have queried us)"
fi

FIND_CALLS=$(echo "$FRESH_LOGS" | { grep -c "RPC DHTProtocol.rpc_find" || true; })
FIND_ERRORS=$(echo "$FRESH_LOGS" | { grep -c "RPC handler error\|decode FindRequest" || true; })
if [[ "${FIND_CALLS:-0}" -gt 0 ]]; then
    FERR="${FIND_ERRORS:-0}"
    pass "rpc_find: $FIND_CALLS call(s), ${FERR} errors"
fi

# Any unexpected WARN/ERROR lines in logs (VPK startup retries are benign)
WARN_LINES=$(echo "$FRESH_LOGS" | grep -E " WARN | ERROR " | grep -v "VPK not ready\|Persistent connection closed" || echo "")
if [[ -z "$WARN_LINES" ]]; then
    pass "no unexpected warnings or errors in logs"
else
    WARN_COUNT=$(echo "$WARN_LINES" | { grep -c "." || true; })
    info "warnings in logs (${WARN_COUNT:-0}):"
    echo "$WARN_LINES" | head -5 | while IFS= read -r line; do
        echo "      $line"
    done
fi

# ---------------------------------------------------------------------------
# 6. Map visibility (optional)
# ---------------------------------------------------------------------------
header "6. Map visibility"

if [[ -n "$SKIP_MAP_CHECK" ]]; then
    info "skipped (SKIP_MAP_CHECK set)"
elif [[ -z "${PEER_ID:-}" ]]; then
    info "skipped — peer ID unknown"
else
    TARGET_PEER="${MAP_PEER_ID:-$PEER_ID}"
    info "checking map.kwaai.ai for $TARGET_PEER ..."
    info "(map crawls DHT every ~60s — polling up to 90s)"

    MAP_FOUND=0
    MAP_START=$(now_ms)
    MAP_ENDPOINTS=(
        "https://map.kwaai.ai/api/v1/nodes"
        "https://map.kwaai.ai/api/v1/state"
    )

    for _elapsed in $(seq 0 5 90); do
        for endpoint in "${MAP_ENDPOINTS[@]}"; do
            RESP=$(curl -sf --max-time 8 "$endpoint" 2>/dev/null || echo "")
            if [[ -n "$RESP" ]] && echo "$RESP" | grep -q "$TARGET_PEER"; then
                MAP_NOW=$(now_ms)
                MAP_MS=$((MAP_NOW - MAP_START))
                MAP_SECS_INT=$((MAP_MS / 1000))
                MAP_SECS_DEC=$((MAP_MS % 1000 / 100))
                metric "map.kwaai.ai visibility: ${MAP_MS}ms (${MAP_SECS_INT}.${MAP_SECS_DEC}s) after starting check"
                pass "node visible on map.kwaai.ai (${MAP_SECS_INT}.${MAP_SECS_DEC}s)"
                MAP_FOUND=1
                break 2
            fi
        done
        if [[ $_elapsed -lt 90 ]]; then
            sleep 5
        fi
    done

    if [[ "$MAP_FOUND" -eq 0 ]]; then
        info "node not yet visible via map API (may appear shortly — map crawls every ~60s)"
        info "verify manually: https://map.kwaai.ai"
    fi
fi

# ---------------------------------------------------------------------------
# 7. Cleanup
# ---------------------------------------------------------------------------
header "7. Cleanup"

if [[ -n "$KEEP_RUNNING" ]]; then
    info "leaving node running (KEEP_RUNNING set)"
elif [[ "$WAS_RUNNING" -eq 0 ]]; then
    info "stopping node (was not running before test)"
    "$KWAAINET_BIN" stop 2>/dev/null || true
    pass "node stopped"
else
    info "leaving node running (was running before test)"
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo "══════════════════════════════════════════════════════"
echo -e "${BOLD}  Performance Metrics${NC}"
echo "══════════════════════════════════════════════════════"
if [[ ${#METRICS[@]} -eq 0 ]]; then
    echo "  (none recorded)"
else
    for m in "${METRICS[@]}"; do
        echo "  • $m"
    done
fi
echo ""
echo "══════════════════════════════════════════════════════"
if [[ $FAILURES -eq 0 ]]; then
    echo -e "  ${GREEN}${BOLD}All tests passed.${NC}"
    echo "══════════════════════════════════════════════════════"
    exit 0
else
    echo -e "  ${RED}${BOLD}$FAILURES test(s) failed.${NC}"
    echo "══════════════════════════════════════════════════════"
    exit 1
fi
