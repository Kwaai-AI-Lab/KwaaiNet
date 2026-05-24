#!/usr/bin/env bash
# run-tests.sh — master test runner for kwaai-network
#
# Usage:
#   bash tests/kwaai-network/run-tests.sh              # unit tests only
#   bash tests/kwaai-network/run-tests.sh --all        # unit + integration + network
#   bash tests/kwaai-network/run-tests.sh --integration  # unit + integration
#   bash tests/kwaai-network/run-tests.sh --network      # unit + network
#   bash tests/kwaai-network/run-tests.sh --report       # show metrics trends only
#
# The performance metric for kwaai-network is:
#   connectivity, uptime, and robustness of the P2P network.
# Each run appends results to tests/kwaai-network/results/metrics.jsonl.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CORE="$REPO_ROOT/core"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'; NC='\033[0m'
ok()   { echo -e "${GREEN}  ✅ $1${NC}"; }
fail() { echo -e "${RED}  ❌ $1${NC}"; }
info() { echo -e "${CYAN}  ▶ $1${NC}"; }
warn() { echo -e "${YELLOW}  ⚠ $1${NC}"; }

# Parse flags
RUN_INTEGRATION=0
RUN_NETWORK=0
REPORT_ONLY=0
for arg in "$@"; do
    case "$arg" in
        --all)         RUN_INTEGRATION=1; RUN_NETWORK=1 ;;
        --integration) RUN_INTEGRATION=1 ;;
        --network)     RUN_NETWORK=1 ;;
        --report)      REPORT_ONLY=1 ;;
        *) echo "Unknown flag: $arg"; exit 1 ;;
    esac
done

echo
echo "╭───────────────────────────────────────────────────────────────╮"
echo "│          KwaaiNet P2P Network Test Suite                      │"
echo "╰───────────────────────────────────────────────────────────────╯"
echo
echo "  Metrics: tests/kwaai-network/results/metrics.jsonl"
echo "  Rule:    Never route inference over direct TCP — use p2p relay"
echo

if [[ "$REPORT_ONLY" == "1" ]]; then
    info "Metrics trend report"
    cd "$CORE"
    cargo run -q -p kwaai-network-tests --bin metrics-report
    exit 0
fi

# ── Step 1: Build (ensures p2pd binary is available for integration tests) ──
info "Step 1/4  Build network crates"
cd "$CORE"
if [[ "$RUN_INTEGRATION" == "1" ]] || [[ "$RUN_NETWORK" == "1" ]]; then
    cargo build -q -p kwaai-p2p -p kwaai-p2p-daemon -p kwaai-hivemind-dht -p kwaai-rpc -p kwaai-network-tests 2>&1 | tail -5
else
    cargo build -q -p kwaai-p2p -p kwaai-hivemind-dht -p kwaai-rpc -p kwaai-network-tests 2>&1 | tail -5
fi
ok "build complete"
echo

# ── Step 2: Unit tests (always) ──────────────────────────────────────────────
info "Step 2/4  Unit tests (01_unit_dht, 02_unit_p2p, 03_unit_rpc)"
UNIT_PASS=0
if cargo test -q -p kwaai-network-tests \
    --test 01_unit_dht \
    --test 02_unit_p2p \
    --test 03_unit_rpc \
    2>&1 | tee /tmp/kwaai-unit-test.log | grep -E "test .* ok|FAILED|error"; then
    ok "unit tests passed"
    UNIT_PASS=1
else
    fail "unit tests FAILED — see /tmp/kwaai-unit-test.log"
fi
echo

# ── Step 3: Integration tests (if enabled) ───────────────────────────────────
INTEGRATION_PASS=1
if [[ "$RUN_INTEGRATION" == "1" ]]; then
    info "Step 3/4  Integration tests (daemon + relay topology)"
    if KWAAI_INTEGRATION_TESTS=1 \
        cargo test -q -p kwaai-network-tests \
            --test 04_integration_daemon \
            --test 05_integration_relay \
            -- --test-threads=1 \
        2>&1 | tee /tmp/kwaai-integration-test.log | grep -E "test .* ok|FAILED|error"; then
        ok "integration tests passed"
        INTEGRATION_PASS=1
    else
        fail "integration tests FAILED — see /tmp/kwaai-integration-test.log"
        INTEGRATION_PASS=0
    fi
else
    warn "Step 3/4  Integration tests skipped (pass --integration to enable)"
fi
echo

# ── Step 4: Network tests (if enabled) ───────────────────────────────────────
NETWORK_PASS=1
if [[ "$RUN_NETWORK" == "1" ]]; then
    info "Step 4/4  Network tests (live bootstrap + peer count + DHT latency)"
    if KWAAI_INTEGRATION_TESTS=1 KWAAI_NETWORK_TESTS=1 \
        cargo test -q -p kwaai-network-tests \
            --test 06_network_bootstrap \
            -- --test-threads=2 \
        2>&1 | tee /tmp/kwaai-network-test.log | grep -E "test .* ok|FAILED|error"; then
        ok "network tests passed"
        NETWORK_PASS=1
    else
        fail "network tests FAILED — see /tmp/kwaai-network-test.log"
        NETWORK_PASS=0
    fi
else
    warn "Step 4/4  Network tests skipped (pass --network or --all to enable)"
fi
echo

# ── Metrics trend report ──────────────────────────────────────────────────────
info "Metrics trend (this run + history)"
cargo run -q -p kwaai-network-tests --bin metrics-report 2>/dev/null || true

# ── Summary ──────────────────────────────────────────────────────────────────
echo
echo "╭─────────────────────────────────────────────────────────────────╮"
if [[ "$UNIT_PASS" == "1" ]] && [[ "$INTEGRATION_PASS" == "1" ]] && [[ "$NETWORK_PASS" == "1" ]]; then
    echo -e "│  ${GREEN}All test tiers passed${NC}                                          │"
else
    echo -e "│  ${RED}Some tests failed — see logs above${NC}                             │"
fi
echo "╰─────────────────────────────────────────────────────────────────╯"
echo
echo "  To iterate: fix regressions, rerun, compare metrics-report."
echo "  Target: peer_count ↑, dht_get_ms ↓, relay connectivity = 100%"
echo

[[ "$UNIT_PASS" == "1" ]] && [[ "$INTEGRATION_PASS" == "1" ]] && [[ "$NETWORK_PASS" == "1" ]]
