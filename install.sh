#!/usr/bin/env bash
# KwaaiNet one-command installer
# Downloads the pre-built binary for your platform, installs it, and starts a node.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/main/install.sh | bash
#
# What it does:
#   1. Detect platform (OS + arch)
#   2. Download kwaainet + p2pd from the latest GitHub release
#   3. Install both to /usr/local/bin
#   4. kwaainet setup   — generate identity + write config
#   5. kwaainet benchmark — calibrate tok/s for DHT announcement
#   6. kwaainet start --daemon — join the network

set -euo pipefail

REPO="Kwaai-AI-Lab/KwaaiNet"
BASE_URL="https://github.com/${REPO}/releases/latest/download"

# ── platform detection ────────────────────────────────────────────────────────

OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}-${ARCH}" in
  Darwin-arm64)   TARGET="aarch64-apple-darwin" ;;
  Darwin-x86_64)  TARGET="x86_64-apple-darwin" ;;
  Linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
  Linux-amd64)    TARGET="x86_64-unknown-linux-gnu" ;;
  *)
    echo "Unsupported platform: ${OS}-${ARCH}"
    echo "Windows users: see the README for PowerShell install instructions."
    exit 1
    ;;
esac

ARCHIVE="kwaainet-${TARGET}.tar.gz"
URL="${BASE_URL}/${ARCHIVE}"

echo "=== KwaaiNet Installer ==="
echo "Platform : ${OS} / ${ARCH}"
echo "Target   : ${TARGET}"
echo ""

# ── download & install ────────────────────────────────────────────────────────

echo "Downloading ${ARCHIVE} ..."
curl -fsSL "${URL}" | tar -xz -C /tmp

echo "Installing kwaainet and p2pd to /usr/local/bin ..."
sudo mv /tmp/kwaainet /tmp/p2pd /usr/local/bin/
sudo chmod +x /usr/local/bin/kwaainet /usr/local/bin/p2pd

echo ""
echo "kwaainet $(kwaainet --version 2>/dev/null || echo '(installed)')"
echo ""

# ── setup ─────────────────────────────────────────────────────────────────────

echo "=== Step 1/3: Setup ==="
kwaainet setup

echo ""

# ── benchmark ─────────────────────────────────────────────────────────────────

echo "=== Step 2/3: Benchmark (calibrate tok/s) ==="
kwaainet benchmark

echo ""

# ── start node ────────────────────────────────────────────────────────────────

echo "=== Step 3/3: Starting node ==="
kwaainet start --daemon

echo ""
echo "=== Done ==="
echo "Your node is joining the network. Run 'kwaainet status' to check."
echo "It will appear on https://map.kwaai.ai within ~60 seconds."
