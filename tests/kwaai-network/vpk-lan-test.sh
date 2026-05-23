#!/usr/bin/env bash
# =============================================================================
# VPK LAN Integration Test
# =============================================================================
#
# Two-machine test for KwaaiNet VPK Phase 1 integration.
#
# SETUP
#   Machine A (Eve+Bob role) — the machine that hosts VPK
#   Machine B (Bob role)     — the second LAN machine
#
# RUN ON BOTH MACHINES FIRST
#   cargo build --release -p kwaai-cli   (or use an installed kwaainet binary)
#
# USAGE
#   # On Machine A:
#   bash tests/vpk-lan-test.sh machine-a [<machine-a-ip>]
#
#   # On Machine B:
#   bash tests/vpk-lan-test.sh machine-b
#
# =============================================================================

set -euo pipefail

KWAAINET="${KWAAINET:-kwaainet}"   # path to binary; override if not on PATH
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✅ $*${RESET}"; }
fail() { echo -e "  ${RED}❌ $*${RESET}"; exit 1; }
info() { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
warn() { echo -e "  ${YELLOW}⚠  $*${RESET}"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }

ROLE="${1:-}"
MACHINE_A_IP="${2:-}"

if [[ -z "$ROLE" ]]; then
    echo "Usage: $0 <machine-a|machine-b> [<machine-a-ip>]"
    exit 1
fi

# ---------------------------------------------------------------------------
# Shared: check binary
# ---------------------------------------------------------------------------
check_binary() {
    step "Checking kwaainet binary"
    if ! command -v "$KWAAINET" &>/dev/null; then
        # Try cargo-built binary
        KWAAINET="$SCRIPT_DIR/../core/target/release/kwaainet"
        [[ -x "$KWAAINET" ]] || fail "kwaainet not found. Build with: cargo build --release -p kwaai-cli"
    fi
    VERSION=$("$KWAAINET" --version 2>&1 | head -1)
    pass "Found: $VERSION"
}

# ---------------------------------------------------------------------------
# Machine A: enable VPK and start node
# ---------------------------------------------------------------------------
run_machine_a() {
    local ip="${MACHINE_A_IP:-$(hostname -I 2>/dev/null | awk '{print $1}')}"

    echo
    echo -e "${BOLD}╔══════════════════════════════════╗"
    echo -e "║   VPK LAN Test — Machine A (Eve) ║"
    echo -e "╚══════════════════════════════════╝${RESET}"
    echo

    check_binary

    # ── Step 1: Show identity ───────────────────────────────────────────────
    step "Step 1 — Node identity"
    "$KWAAINET" identity show
    echo
    info "Copy the Peer ID above into VPK's config.toml as 'peer_id'"

    # ── Step 2: Start mock VPK health server ────────────────────────────────
    step "Step 2 — Start mock VPK health server (port 7432)"
    if lsof -i :7432 &>/dev/null 2>&1; then
        warn "Port 7432 already in use — assuming VPK (real or mock) is already running"
    else
        info "Launching mock VPK health server in background…"
        VPK_MODE=both \
        VPK_CAPACITY_GB=512 \
        VPK_PEER_ID="$(${KWAAINET} identity show 2>/dev/null | grep 'Peer ID' | awk '{print $NF}' || echo 'unknown')" \
            python3 "${SCRIPT_DIR}/mock-vpk-health.py" &
        MOCK_PID=$!
        sleep 1
        if kill -0 "$MOCK_PID" 2>/dev/null; then
            pass "Mock server running (PID $MOCK_PID)"
        else
            fail "Mock server failed to start"
        fi
    fi

    # ── Step 3: Verify health endpoint ─────────────────────────────────────
    step "Step 3 — Verify VPK health endpoint"
    HEALTH=$(curl -sf http://localhost:7432/api/health 2>/dev/null || true)
    if [[ -n "$HEALTH" ]]; then
        pass "Health endpoint responds: $HEALTH"
    else
        fail "Health endpoint not responding on :7432"
    fi

    # ── Step 4: Enable VPK in kwaainet config ───────────────────────────────
    step "Step 4 — Enable VPK integration"
    ENDPOINT="http://${ip}:7432"
    "$KWAAINET" vpk enable --mode both --endpoint "$ENDPOINT" --port 7432
    pass "VPK enabled (endpoint: $ENDPOINT)"

    # ── Step 5: Check vpk status ────────────────────────────────────────────
    step "Step 5 — kwaainet vpk status"
    "$KWAAINET" vpk status
    pass "Status command completed"

    # ── Step 6: Start node in daemon mode ───────────────────────────────────
    step "Step 6 — Start KwaaiNet node"
    if "$KWAAINET" status 2>/dev/null | grep -q 'Running'; then
        warn "Node already running — restarting to pick up VPK config"
        "$KWAAINET" restart
    else
        "$KWAAINET" start --daemon
    fi
    pass "Node started"

    # ── Step 7: Wait for first announce cycle ───────────────────────────────
    step "Step 7 — Wait for VPK DHT announcement (35 s)"
    info "Waiting for bootstrap + initial announcement…"
    sleep 35

    # ── Step 8: Check logs ──────────────────────────────────────────────────
    step "Step 8 — Check logs for VPK announcement"
    echo
    "$KWAAINET" logs -n 60 | grep -E "(VPK|vpk)" || warn "No VPK lines in recent logs yet — may need more time"
    echo

    # Verify specific log markers
    if "$KWAAINET" logs -n 100 | grep -q "VPK healthy"; then
        pass "Log: 'VPK healthy' found"
    else
        warn "Log: 'VPK healthy' not yet seen — health check may have failed"
    fi

    if "$KWAAINET" logs -n 100 | grep -q "Announced VPK capability"; then
        pass "Log: 'Announced VPK capability' found — DHT record written"
    else
        warn "Log: 'Announced VPK capability' not yet seen — check bootstrap connectivity"
    fi

    echo
    echo -e "${BOLD}Machine A is ready.${RESET}"
    echo -e "  Machine A IP:   ${CYAN}${ip}${RESET}"
    echo -e "  VPK endpoint:   ${CYAN}${ENDPOINT}${RESET}"
    echo
    echo -e "  Now run on Machine B:"
    echo -e "    ${YELLOW}bash tests/vpk-lan-test.sh machine-b${RESET}"
    echo
}

# ---------------------------------------------------------------------------
# Machine B: start node and discover VPK peers
# ---------------------------------------------------------------------------
run_machine_b() {
    echo
    echo -e "${BOLD}╔══════════════════════════════════╗"
    echo -e "║   VPK LAN Test — Machine B (Bob) ║"
    echo -e "╚══════════════════════════════════╝${RESET}"
    echo

    check_binary

    # ── Step 1: Show identity ───────────────────────────────────────────────
    step "Step 1 — Node identity"
    "$KWAAINET" identity show

    # ── Step 2: Start node ──────────────────────────────────────────────────
    step "Step 2 — Start KwaaiNet node"
    if "$KWAAINET" status 2>/dev/null | grep -q 'Running'; then
        pass "Node already running"
    else
        "$KWAAINET" start --daemon
        pass "Node started"
    fi

    # ── Step 3: Bootstrap wait ──────────────────────────────────────────────
    step "Step 3 — Bootstrap into DHT (35 s)"
    info "Connecting to bootstrap peers…"
    sleep 35
    pass "Bootstrap wait complete"

    # ── Step 4: Discover VPK nodes ──────────────────────────────────────────
    step "Step 4 — kwaainet vpk discover"
    echo
    "$KWAAINET" vpk discover
    echo

    # ── Step 5: Interpret results ───────────────────────────────────────────
    step "Step 5 — Interpreting results"
    DISCOVER_OUT=$("$KWAAINET" vpk discover 2>&1)

    if echo "$DISCOVER_OUT" | grep -q "Found.*VPK-capable"; then
        COUNT=$(echo "$DISCOVER_OUT" | grep -oP '(?<=Found )\d+')
        pass "Discovered $COUNT VPK-capable node(s) via DHT"
    elif echo "$DISCOVER_OUT" | grep -q "No VPK-capable"; then
        warn "No VPK nodes found yet"
        info "Check that Machine A's node is running and VPK is enabled"
        info "DHT records expire after 360 s — Machine A must be running"
        info "Try again in a few minutes: kwaainet vpk discover"
    else
        info "Discover output:"
        echo "$DISCOVER_OUT"
    fi

    echo
    echo -e "${BOLD}Machine B test complete.${RESET}"
    echo
}

# ---------------------------------------------------------------------------
# Dispatch
# ---------------------------------------------------------------------------
case "$ROLE" in
    machine-a) run_machine_a ;;
    machine-b) run_machine_b ;;
    *)
        echo "Unknown role '$ROLE'. Use: machine-a or machine-b"
        exit 1
        ;;
esac
