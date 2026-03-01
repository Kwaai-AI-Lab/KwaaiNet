#!/usr/bin/env bash
# cargo-dist extra-artifact hook: build p2pd (go-libp2p-daemon fork).
#
# cargo-dist calls this script and expects the built binary to be placed at
# $CARGO_DIST_EXTRA_ARTIFACTS_DIR/p2pd  (Unix) or p2pd.exe (Windows).
#
# Environment variables set by cargo-dist:
#   CARGO_DIST_EXTRA_ARTIFACTS_DIR  — output directory for this artifact
#   CARGO_BUILD_TARGET              — Rust target triple (e.g. aarch64-apple-darwin)
#
# GOARCH mapping: We derive Go's GOARCH/GOOS from CARGO_BUILD_TARGET so that
# cross-compiled targets (e.g. building x86_64-apple-darwin on an ARM runner)
# produce the correct p2pd binary.

set -euo pipefail

TARGET="${CARGO_BUILD_TARGET:-}"
OUT_DIR="${CARGO_DIST_EXTRA_ARTIFACTS_DIR:-.}"

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

CLONE_DIR="/tmp/go-libp2p-daemon-$$"
git clone --depth 1 --branch v0.5.0.hivemind1 \
    https://github.com/learning-at-home/go-libp2p-daemon.git "${CLONE_DIR}"

cd "${CLONE_DIR}"
go build -o "${OUT_DIR}/${BINARY_NAME}" ./p2pd

echo "==> p2pd built at ${OUT_DIR}/${BINARY_NAME}"
