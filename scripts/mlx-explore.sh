#!/usr/bin/env bash
# scripts/mlx-explore.sh — Automated MLX backend exploration
#
# Runs unattended: builds mlx feature, runs all mlx_ tests, logs results.
# Expected runtime: ~3-5 minutes (mlx-sys build + tests)
#
# Usage:
#   cd /Users/rezarassool/Source/KwaaiNet/core && ../scripts/mlx-explore.sh
#
# Results: ~/.kwaainet/logs/mlx-explore-<timestamp>.log

set -euo pipefail

TIMESTAMP=$(date +%Y%m%d-%H%M%S)
LOG="$HOME/.kwaainet/logs/mlx-explore-${TIMESTAMP}.log"
mkdir -p "$HOME/.kwaainet/logs"

# Ensure we're in the workspace root
if [ ! -f Cargo.toml ]; then
    cd "$(dirname "$0")/../core" 2>/dev/null || {
        echo "ERROR: Run from core/ or let the script find it"
        exit 1
    }
fi

log() { echo "[$(date +%H:%M:%S)] $*" | tee -a "$LOG"; }

log "========================================"
log "  MLX Backend Exploration"
log "  $(date)"
log "  Log: $LOG"
log "========================================"
log ""

# Check prerequisites
log "Checking prerequisites..."
if ! xcrun --find metal >/dev/null 2>&1; then
    log "ERROR: Metal compiler not found. Install Xcode and run:"
    log "  xcodebuild -downloadComponent MetalToolchain"
    exit 1
fi
log "  Metal compiler: $(xcrun --find metal)"
log "  Rust: $(rustc --version)"
log ""

# Step 1: Build
log "Step 1/3: Building kwaai-inference with mlx feature..."
if cargo build -p kwaai-inference --features mlx 2>>"$LOG"; then
    log "  Build: OK"
else
    log "  Build: FAILED (see log for details)"
    log "Full log: $LOG"
    exit 1
fi
log ""

# Step 2: Run tests
log "Step 2/3: Running MLX tests..."
log ""

# Run each test individually so we see which pass/fail
TESTS=(
    test_mlx_array_basic
    test_mlx_matmul
    test_mlx_nn_linear
    test_mlx_nn_embedding
    test_mlx_nn_rms_norm
    test_mlx_softmax
    test_mlx_silu
    test_mlx_concat
    test_mlx_eval_lazy
    test_mlx_transpose
    test_mlx_transformer_block
    test_mlx_safetensors_load
)

PASSED=0
FAILED=0
SKIPPED=0

for test in "${TESTS[@]}"; do
    log "  Running $test..."
    if cargo test -p kwaai-inference --features mlx -- "$test" --nocapture 2>&1 | tee -a "$LOG" | grep -q "test .* ok"; then
        PASSED=$((PASSED + 1))
        log "    → PASS"
    elif grep -q "SKIP" "$LOG"; then
        SKIPPED=$((SKIPPED + 1))
        log "    → SKIP"
    else
        FAILED=$((FAILED + 1))
        log "    → FAIL"
    fi
done

log ""

# Step 3: Summary
log "========================================"
log "  RESULTS"
log "========================================"
log "  Passed:  $PASSED"
log "  Failed:  $FAILED"
log "  Skipped: $SKIPPED"
log "  Total:   ${#TESTS[@]}"
log ""

# Extract [OK] and [SKIP] lines from test output
log "  Test output highlights:"
grep -E "\[(OK|SKIP|FAIL)\]" "$LOG" | sort -u | while read -r line; do
    log "    $line"
done

log ""
log "Full log: $LOG"
log "Done."
