#!/usr/bin/env bash
# =============================================================================
# kwaainet uninstall — integration test
# =============================================================================
#
# Verifies that `kwaainet uninstall --yes` removes:
#   1. ~/.kwaainet/ data directory
#   2. The kwaainet binary
#   3. The p2pd binary (when present alongside kwaainet)
#
# The test installs both binaries into a scratch directory added to PATH so
# the binary can unlink itself without touching the source build output.
#
# USAGE
#   # From the repo root — build first, then run:
#   cargo build -p kwaai-cli
#   bash tests/uninstall-test.sh
#
#   # Or point at a pre-built binary:
#   KWAAINET_BIN=./path/to/kwaainet bash tests/uninstall-test.sh
#
# EXIT CODES
#   0  all assertions passed
#   1  one or more assertions failed

set -euo pipefail

# ---------------------------------------------------------------------------
# Colour helpers
# ---------------------------------------------------------------------------
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
pass() { echo -e "  ${GREEN}PASS${NC}  $*"; }
fail() { echo -e "  ${RED}FAIL${NC}  $*"; FAILURES=$((FAILURES + 1)); }
info() { echo -e "  ${YELLOW}....${NC}  $*"; }

FAILURES=0

# ---------------------------------------------------------------------------
# Locate the kwaainet binary
# ---------------------------------------------------------------------------
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
KWAAINET_BIN="${KWAAINET_BIN:-$REPO_ROOT/core/target/debug/kwaainet}"

if [[ ! -x "$KWAAINET_BIN" ]]; then
    echo "ERROR: kwaainet binary not found at $KWAAINET_BIN"
    echo "Build it first: cd core && cargo build -p kwaai-cli"
    exit 1
fi

echo "=============================================="
echo " kwaainet uninstall — integration test"
echo "=============================================="
echo "  Binary: $KWAAINET_BIN"
echo ""

# ---------------------------------------------------------------------------
# Set up an isolated HOME so we don't touch the real ~/.kwaainet
# ---------------------------------------------------------------------------
SCRATCH_HOME="$(mktemp -d)"
SCRATCH_BIN="$SCRATCH_HOME/bin"
mkdir -p "$SCRATCH_BIN"

# On Windows (MSYS/Cygwin) binaries have .exe extensions
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" || "$OSTYPE" == "win32" ]]; then
    EXE=".exe"
else
    EXE=""
fi

# Override HOME for the duration of the test
export HOME="$SCRATCH_HOME"
export PATH="$SCRATCH_BIN:$PATH"

# Cleanup scratch dir on exit (even on failure)
trap 'rm -rf "$SCRATCH_HOME"' EXIT

# ---------------------------------------------------------------------------
# Seed a fake installation
# ---------------------------------------------------------------------------
info "Seeding fake installation..."

# Copy binaries into the scratch bin dir
cp "$KWAAINET_BIN" "$SCRATCH_BIN/kwaainet${EXE}"
# Fake p2pd (an empty executable is enough; use .exe on Windows)
printf '#!/bin/sh\necho "p2pd fake"\n' > "$SCRATCH_BIN/p2pd${EXE}"
chmod +x "$SCRATCH_BIN/kwaainet${EXE}" "$SCRATCH_BIN/p2pd${EXE}"

# Create the data directory with representative files
KWAAINET_DIR="$SCRATCH_HOME/.kwaainet"
mkdir -p "$KWAAINET_DIR/run" "$KWAAINET_DIR/logs" "$KWAAINET_DIR/credentials"
echo "model: llama3.1:8b"        > "$KWAAINET_DIR/config.yaml"
echo "fake-ed25519-bytes"         > "$KWAAINET_DIR/identity.key"
echo '{"running":false}'         > "$KWAAINET_DIR/run/kwaainet.status"
echo '{"samples":0}'             > "$KWAAINET_DIR/run/monitor.json"
echo "log line 1"                > "$KWAAINET_DIR/logs/kwaainet.log"
echo '{"type":"SummitAttendeeVC"}' > "$KWAAINET_DIR/credentials/test-vc.json"

info "Fake ~/.kwaainet created at $KWAAINET_DIR"

# ---------------------------------------------------------------------------
# Run uninstall (--yes skips the confirmation prompt)
# ---------------------------------------------------------------------------
echo ""
info "Running: kwaainet uninstall --yes"
echo "----------------------------------------------"
"$SCRATCH_BIN/kwaainet" uninstall --yes
echo "----------------------------------------------"
echo ""

# ---------------------------------------------------------------------------
# Assertions
# ---------------------------------------------------------------------------

# 1. Data directory removed
if [[ ! -d "$KWAAINET_DIR" ]]; then
    pass "~/.kwaainet/ directory removed"
else
    fail "~/.kwaainet/ still exists"
    ls -la "$KWAAINET_DIR" 2>/dev/null || true
fi

# 2. kwaainet binary removed
if [[ ! -f "$SCRATCH_BIN/kwaainet${EXE}" ]]; then
    pass "kwaainet binary removed"
else
    fail "kwaainet binary still exists at $SCRATCH_BIN/kwaainet${EXE}"
fi

# 3. p2pd binary removed
if [[ ! -f "$SCRATCH_BIN/p2pd${EXE}" ]]; then
    pass "p2pd binary removed"
else
    fail "p2pd binary still exists at $SCRATCH_BIN/p2pd${EXE}"
fi

# ---------------------------------------------------------------------------
# --keep-data variant: data should survive
# ---------------------------------------------------------------------------
echo ""
info "--- Testing --keep-data flag ---"

# Re-seed
cp "$KWAAINET_BIN" "$SCRATCH_BIN/kwaainet${EXE}"
printf '#!/bin/sh\n' > "$SCRATCH_BIN/p2pd${EXE}"
chmod +x "$SCRATCH_BIN/kwaainet${EXE}" "$SCRATCH_BIN/p2pd${EXE}"
mkdir -p "$KWAAINET_DIR/run"
echo "model: llama3.1:8b" > "$KWAAINET_DIR/config.yaml"

info "Running: kwaainet uninstall --yes --keep-data"
echo "----------------------------------------------"
"$SCRATCH_BIN/kwaainet${EXE}" uninstall --yes --keep-data
echo "----------------------------------------------"
echo ""

if [[ -d "$KWAAINET_DIR" ]]; then
    pass "--keep-data: ~/.kwaainet/ preserved"
else
    fail "--keep-data: ~/.kwaainet/ was removed (should have been kept)"
fi

if [[ ! -f "$SCRATCH_BIN/kwaainet${EXE}" ]]; then
    pass "--keep-data: kwaainet binary still removed"
else
    fail "--keep-data: kwaainet binary was not removed"
fi

# ---------------------------------------------------------------------------
# Result
# ---------------------------------------------------------------------------
echo ""
echo "=============================================="
if [[ $FAILURES -eq 0 ]]; then
    echo -e "  ${GREEN}All tests passed.${NC}"
    echo "=============================================="
    exit 0
else
    echo -e "  ${RED}$FAILURES test(s) failed.${NC}"
    echo "=============================================="
    exit 1
fi
