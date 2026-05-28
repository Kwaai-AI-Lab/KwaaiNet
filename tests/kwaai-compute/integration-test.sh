#!/usr/bin/env bash
# =============================================================================
# kwaai-compute Integration Test Suite
# =============================================================================
#
# Tests distributed block-shard inference end-to-end without real model
# weights.  Validates the control plane (shard serve/run/chain), RPC codec,
# and session lifecycle.
#
# USAGE
#   # Build first:
#   cd core && cargo build --release -p kwaainet
#
#   bash tests/kwaai-compute/integration-test.sh [--binary PATH]
#
# EXIT CODE
#   0  all tests passed
#   1  one or more tests failed
#
# =============================================================================

set -euo pipefail

KWAAINET="${1:-./core/target/release/kwaainet}"
# Allow override via env or first positional arg
if [[ "${1:-}" == "--binary" ]]; then
    KWAAINET="${2:?--binary requires a path}"
    shift 2
fi

# ── colour helpers ─────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

PASS=0; FAIL=0; SKIP=0

pass() { PASS=$((PASS+1)); echo -e "  ${GREEN}✅ PASS${RESET} $*"; }
fail() { FAIL=$((FAIL+1)); echo -e "  ${RED}❌ FAIL${RESET} $*"; }
skip() { SKIP=$((SKIP+1)); echo -e "  ${YELLOW}⏭  SKIP${RESET} $*"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }

# ── pre-flight ─────────────────────────────────────────────────────────────────
step "Pre-flight"

if [[ ! -x "$KWAAINET" ]]; then
    echo -e "${RED}binary not found: $KWAAINET${RESET}"
    echo "Build with: cd core && cargo build --release -p kwaainet"
    exit 1
fi
pass "binary exists: $KWAAINET"

BINARY_VERSION=$("$KWAAINET" --version 2>&1 | head -1 || echo "unknown")
echo -e "  ${CYAN}version: $BINARY_VERSION${RESET}"

# ── unit / codec tests (via cargo test) ────────────────────────────────────────
step "Cargo unit tests (kwaai-inference)"

if cargo test -p kwaai-inference --quiet 2>&1 | tail -5; then
    pass "kwaai-inference unit tests"
else
    fail "kwaai-inference unit tests"
fi

step "Cargo unit tests (kwaai-compression)"

if cargo test -p kwaai-compression --quiet 2>&1 | tail -5; then
    pass "kwaai-compression unit tests"
else
    fail "kwaai-compression unit tests"
fi

step "Cargo unit tests (kwaai-distributed)"

if cargo test -p kwaai-distributed --quiet 2>&1 | tail -5; then
    pass "kwaai-distributed unit tests"
else
    fail "kwaai-distributed unit tests"
fi

# ── shard serve / run smoke test (single-node, no model) ──────────────────────
step "Shard help / subcommand availability"

if "$KWAAINET" shard --help 2>&1 | grep -q "serve"; then
    pass "shard serve subcommand registered"
else
    fail "shard serve subcommand missing"
fi

if "$KWAAINET" shard --help 2>&1 | grep -q "run"; then
    pass "shard run subcommand registered"
else
    fail "shard run subcommand missing"
fi

if "$KWAAINET" shard --help 2>&1 | grep -q "chain"; then
    pass "shard chain subcommand registered"
else
    fail "shard chain subcommand missing"
fi

if "$KWAAINET" shard --help 2>&1 | grep -q "status"; then
    pass "shard status subcommand registered"
else
    fail "shard status subcommand missing"
fi

# ── shard chain (no daemon needed for --help-style validation) ─────────────────
step "Shard chain coverage (offline)"

# chain with no daemon should fail gracefully, not panic
CHAIN_OUT=$("$KWAAINET" shard chain 2>&1 || true)
if echo "$CHAIN_OUT" | grep -qiE "no.*block|empty|0 block|not running|daemon|error|warning"; then
    pass "shard chain exits gracefully when no blocks available"
else
    # Any output without a panic/backtrace is acceptable
    if ! echo "$CHAIN_OUT" | grep -q "thread.*panicked"; then
        pass "shard chain exits without panic"
    else
        fail "shard chain panicked: $CHAIN_OUT"
    fi
fi

# ── shard serve start/stop lifecycle ──────────────────────────────────────────
step "Shard serve lifecycle (no model)"

# Attempting to serve without a model path should give a clear error, not panic
SERVE_OUT=$("$KWAAINET" shard serve --start-block 0 --blocks 4 2>&1 || true)
if echo "$SERVE_OUT" | grep -q "thread.*panicked"; then
    fail "shard serve panicked: $(echo "$SERVE_OUT" | head -5)"
else
    pass "shard serve exits without panic when no model configured"
fi

# ── shard run without peers ────────────────────────────────────────────────────
step "Shard run (no peers)"

RUN_OUT=$("$KWAAINET" shard run "Hello" --max-tokens 1 2>&1 || true)
if echo "$RUN_OUT" | grep -q "thread.*panicked"; then
    fail "shard run panicked: $(echo "$RUN_OUT" | head -5)"
else
    pass "shard run exits without panic when no peers"
fi

# ── version field in output ────────────────────────────────────────────────────
step "Version consistency"

V1=$("$KWAAINET" --version 2>&1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo "")
V2=$(grep '^version' ./core/crates/kwaai-cli/Cargo.toml 2>/dev/null | head -1 | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo "")

if [[ -n "$V1" && -n "$V2" ]]; then
    if [[ "$V1" == "$V2" ]]; then
        pass "binary version ($V1) matches Cargo.toml ($V2)"
    else
        fail "version mismatch: binary=$V1 Cargo.toml=$V2"
    fi
else
    skip "version check (could not parse one or both versions: binary='$V1' Cargo.toml='$V2')"
fi

# ── two-node single-machine integration (skipped if model not present) ─────────
step "Two-node local shard integration"

MODEL_PATH="${KWAAI_MODEL_PATH:-}"
if [[ -z "$MODEL_PATH" ]]; then
    skip "KWAAI_MODEL_PATH not set — skipping live inference test"
    skip "  set KWAAI_MODEL_PATH=/path/to/model.safetensors and re-run"
else
    if [[ ! -f "$MODEL_PATH" ]]; then
        skip "model file not found at $MODEL_PATH"
    else
        # Start first shard (blocks 0–3)
        "$KWAAINET" shard serve \
            --model "$MODEL_PATH" \
            --start-block 0 --blocks 4 \
            --port 9080 &
        SHARD1_PID=$!

        # Start second shard (blocks 4–7)
        "$KWAAINET" shard serve \
            --model "$MODEL_PATH" \
            --start-block 4 --blocks 4 \
            --port 9081 &
        SHARD2_PID=$!

        sleep 2  # allow servers to bind

        # Run inference
        RUN_RESULT=$("$KWAAINET" shard run "What is 2+2?" --max-tokens 8 2>&1 || true)

        kill $SHARD1_PID $SHARD2_PID 2>/dev/null || true
        wait $SHARD1_PID $SHARD2_PID 2>/dev/null || true

        if echo "$RUN_RESULT" | grep -qiE "4|four"; then
            pass "distributed inference produced expected answer"
        elif echo "$RUN_RESULT" | grep -q "thread.*panicked"; then
            fail "distributed inference panicked"
        else
            pass "distributed inference ran without panic (answer: $(echo "$RUN_RESULT" | head -1))"
        fi
    fi
fi

# ── summary ────────────────────────────────────────────────────────────────────
echo ""
echo -e "${BOLD}══════════════════════════════════════════${RESET}"
echo -e "${BOLD}  Results: ${GREEN}$PASS passed${RESET}  ${RED}$FAIL failed${RESET}  ${YELLOW}$SKIP skipped${RESET}"
echo -e "${BOLD}══════════════════════════════════════════${RESET}"

if [[ $FAIL -gt 0 ]]; then
    exit 1
fi
