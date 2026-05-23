#!/usr/bin/env bash
# p2p-ollama-relay-test.sh
#
# Tests the /kwaai/ollama-proxy/1.0.0 P2P relay by:
#   1. Checking metro-linux is reachable via DHT
#   2. Running a 1-chunk graph build through p2p:// to verify end-to-end extraction
#
# Usage:
#   bash tests/p2p-ollama-relay-test.sh
#
# Prerequisites:
#   - kwaainet start --daemon running locally
#   - metro-linux running kwaainet start --daemon + ollama serve
#   - llama3.2:3b pulled on metro-linux

set -euo pipefail

METRO_LINUX_PEER="12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd"
METRO_WIN_PEER="12D3KooWSbrhkjVkyaKaDFzFxSdsbZcYe9wnZRCgQKB7y3WCWqQz"
TEST_KB="p2p-relay-test"
TEST_PDF="/Users/rezarassool/Source/KwaaiNet/docs/LEST WE FORGET -rev25.pdf"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
pass() { echo -e "${GREEN}  ✅ $1${NC}"; }
fail() { echo -e "${RED}  ❌ $1${NC}"; exit 1; }
info() { echo -e "${YELLOW}  ▶ $1${NC}"; }

echo "╭─────────────────────────────────────────────────────────────────────╮"
echo "│               P2P Ollama Relay Test — metro-linux                   │"
echo "╰─────────────────────────────────────────────────────────────────────╯"
echo

# ── Step 1: Check local daemon is running ─────────────────────────────────
info "Step 1/5  local daemon health"
if ! kwaainet p2p info &>/dev/null; then
    fail "local p2pd not running — start with: kwaainet start --daemon"
fi
LOCAL_PEER=$(kwaainet p2p info 2>/dev/null | grep "Peer ID:" | awk '{print $3}')
echo "  local peer: $LOCAL_PEER"
pass "daemon running"

# ── Step 2: DHT lookup for metro-linux ───────────────────────────────────
info "Step 2/5  DHT lookup for metro-linux ($METRO_LINUX_PEER)"
FIND_OUT=$(kwaainet p2p peers find "$METRO_LINUX_PEER" --timeout 15 2>&1 || true)
if echo "$FIND_OUT" | grep -q "/ip4\|/dns\|relay\|circuit"; then
    pass "metro-linux found in DHT"
    echo "$FIND_OUT" | grep -E "addr|/ip4|/dns" | head -3 | sed 's/^/    /'
else
    echo "$FIND_OUT" | head -5 | sed 's/^/    /'
    fail "metro-linux not reachable via DHT — is it online and running kwaainet start --daemon?"
fi

# ── Step 3: Dial metro-linux ──────────────────────────────────────────────
info "Step 3/5  dial metro-linux via P2P"
CONNECT_OUT=$(kwaainet p2p peers connect --peer "$METRO_LINUX_PEER" 2>&1 || kwaainet p2p peers connect --addr "/p2p/$METRO_LINUX_PEER" 2>&1 || true)
if echo "$CONNECT_OUT" | grep -qi "connect\|success\|dial"; then
    pass "connection established"
else
    echo "  (connect output: $(echo "$CONNECT_OUT" | head -2))"
    info "  continuing — relay may still work via circuit"
fi

# ── Step 4: Ollama proxy smoke test ──────────────────────────────────────
info "Step 4/5  init throw-away KB and ingest 1 chunk"
kwaainet rag destroy --kb "$TEST_KB" -y 2>/dev/null || true
kwaainet rag init --kb "$TEST_KB" --embed-model nomic-embed-text 2>&1 | tail -3

# Ingest just the first page of the PDF (use a tiny chunk size + limit)
kwaainet rag ingest "$TEST_PDF" --kb "$TEST_KB" \
    --chunk-strategy paragraph --chunk-size 400 --chunk-overlap 0 2>&1 | tail -3

pass "ingest complete"

# ── Step 5: Graph build via p2p:// ────────────────────────────────────────
info "Step 5/5  graph build --limit 3 via p2p://$METRO_LINUX_PEER"
echo
BUILD_OUT=$(kwaainet rag graph build \
    --kb "$TEST_KB" \
    --inference-urls "p2p://$METRO_LINUX_PEER" \
    --model llama3.2:3b \
    --workers 1 \
    --limit 3 \
    2>&1)

echo "$BUILD_OUT" | tail -20

if echo "$BUILD_OUT" | grep -qiE "HTTP 503|503 Service"; then
    fail "P2P relay connected but Ollama returned 503 — Ollama not running on metro-linux"
elif echo "$BUILD_OUT" | grep -qiE "HTTP 404|404 Not Found"; then
    fail "P2P relay connected, Ollama running, but model not found — run on metro-linux: ollama pull llama3.2:3b"
elif echo "$BUILD_OUT" | grep -qi "502\|bad gateway"; then
    fail "P2P relay connected but got 502 — metro-linux proxy handler may not be registered"
elif echo "$BUILD_OUT" | grep -qi "p2p.*→.*127.0.0.1" && echo "$BUILD_OUT" | grep -qiE "entities=[1-9]"; then
    pass "P2P Ollama relay working end-to-end — entities extracted via metro-linux"
elif echo "$BUILD_OUT" | grep -qi "p2p.*→.*127.0.0.1"; then
    fail "proxy tunnel established but 0 entities extracted — unexpected failure, check output above"
else
    fail "graph build failed — check output above"
fi

# ── Cleanup ────────────────────────────────────────────────────────────────
echo
info "Cleanup"
kwaainet rag destroy --kb "$TEST_KB" -y 2>/dev/null || true
pass "test KB destroyed"

echo
echo "  P2P Ollama relay is working. Use in rebuild:"
echo "  --inference-urls \"p2p://$METRO_LINUX_PEER\""
echo
