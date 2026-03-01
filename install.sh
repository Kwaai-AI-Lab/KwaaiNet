#!/usr/bin/env bash
# KwaaiNet one-command installer (cargo-dist generated, shell variant)
#
# Usage:
#   curl --proto '=https' --tlsv1.2 -LsSf \
#     https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.sh | sh
#
# This script is a convenience wrapper.  The canonical installer is the
# cargo-dist-generated kwaainet-installer.sh that is uploaded as a release
# asset on every release.  It handles all platform detection, checksum
# verification, and PATH setup automatically.

set -euo pipefail

REPO="Kwaai-AI-Lab/KwaaiNet"

# Resolve the latest release tag so we fetch the versioned installer URL
# (avoids stale CDN caches on /releases/latest).
VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
  | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
if [ -z "${VERSION}" ]; then
  echo "Error: could not determine latest release version from GitHub API." >&2
  exit 1
fi

INSTALLER_URL="https://github.com/${REPO}/releases/download/${VERSION}/kwaainet-installer.sh"
echo "Installing KwaaiNet ${VERSION} ..."
curl --proto '=https' --tlsv1.2 -LsSf "${INSTALLER_URL}" | sh
