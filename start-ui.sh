#!/bin/bash
# Start KwaaiNet Node Dashboard (local web UI)
# Works on Linux and macOS. For Windows use: .\start-ui.ps1

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=========================================="
echo "KwaaiNet Node Dashboard"
echo "=========================================="
echo ""

# Check Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found."
    echo "Install Node.js 18+ from https://nodejs.org/ or run: ./setup.sh (then install Node if needed)."
    exit 1
fi

NODE_VERSION=$(node -v 2>/dev/null || true)
echo "✅ Node: $NODE_VERSION"
echo ""

# Ensure dashboard deps are installed
DASHBOARD_DIR="$SCRIPT_DIR/systems/node-dashboard"
if [ ! -d "$DASHBOARD_DIR" ]; then
    echo "❌ Dashboard not found at systems/node-dashboard"
    exit 1
fi

echo "Installing dashboard dependencies (if needed)..."
cd "$DASHBOARD_DIR"
npm install
echo ""

echo "=========================================="
echo "Starting KwaaiNet Dashboard"
echo "=========================================="
echo ""
echo "  Backend:  http://127.0.0.1:3456"
echo "  Frontend: http://127.0.0.1:5173"
echo ""
echo "  Opening browser: http://127.0.0.1:5173"
echo "  (Press Ctrl+C to stop)"
echo ""

npm run dev &
DEV_PID=$!
sleep 5
if command -v xdg-open &> /dev/null; then
    xdg-open "http://127.0.0.1:5173" 2>/dev/null || true
elif command -v open &> /dev/null; then
    open "http://127.0.0.1:5173" 2>/dev/null || true
fi
wait $DEV_PID
