#!/usr/bin/env bash
# cargo-dist extra-artifact hook: build p2pd (go-libp2p-daemon fork).
#
# cargo-dist runs this script from the workspace root (core/) and looks for
# the listed artifacts in that same directory when the script exits.
# Concretely: artifacts = ["p2pd"] means cargo-dist expects core/p2pd.
#
# If cargo-dist sets CARGO_DIST_EXTRA_ARTIFACTS_DIR, that takes priority.
#
# Environment variables set by cargo-dist:
#   CARGO_DIST_EXTRA_ARTIFACTS_DIR  — output directory for this artifact (may be unset)
#   CARGO_BUILD_TARGET              — Rust target triple (e.g. aarch64-apple-darwin)

set -euo pipefail

TARGET="${CARGO_BUILD_TARGET:-}"

# Resolve output directory to an absolute path NOW, before any directory
# changes.  cargo-dist may not set CARGO_DIST_EXTRA_ARTIFACTS_DIR; if unset
# it expects artifacts relative to its working directory (core/), i.e. CWD.
OUT_DIR="$(cd "${CARGO_DIST_EXTRA_ARTIFACTS_DIR:-.}" && pwd)"

# ── Derive GOOS / GOARCH from Rust target triple ─────────────────────────────
case "${TARGET}" in
    aarch64-apple-darwin)     GOOS=darwin  GOARCH=arm64  ;;
    x86_64-apple-darwin)      GOOS=darwin  GOARCH=amd64  ;;
    x86_64-unknown-linux-gnu) GOOS=linux   GOARCH=amd64  ;;
    aarch64-unknown-linux-gnu)GOOS=linux   GOARCH=arm64  ;;
    x86_64-pc-windows-msvc)   GOOS=windows GOARCH=amd64  ;;
    *)
        # Fallback: let Go detect the host platform
        GOOS="$(go env GOOS)"
        GOARCH="$(go env GOARCH)"
        ;;
esac

export GOOS GOARCH

BINARY_NAME="p2pd"
if [ "${GOOS}" = "windows" ]; then
    BINARY_NAME="p2pd.exe"
fi

echo "==> Building p2pd for ${GOOS}/${GOARCH} ..."

CLONE_DIR="$(mktemp -d -t go-libp2p-daemon-XXXX)"
git clone --depth 1 --branch v0.5.0.hivemind1 \
    https://github.com/learning-at-home/go-libp2p-daemon.git "${CLONE_DIR}"

# Use -C (Go 1.21+) to build without cd-ing away from OUT_DIR.
# This ensures the output lands in OUT_DIR regardless of working directory.
go build -C "${CLONE_DIR}" -o "${OUT_DIR}/${BINARY_NAME}" ./p2pd

echo "==> p2pd built at ${OUT_DIR}/${BINARY_NAME}"
