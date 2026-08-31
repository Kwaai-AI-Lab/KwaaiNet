#!/usr/bin/env bash
# Fetch libp2p-kad 0.48.0 from crates.io and apply the multi-protocol-names
# patch. The patched source is used via `[patch.crates-io]` in core/Cargo.toml
# and is NOT committed — this repo carries only the patch file, which is the
# entire intentional delta against upstream.
#
# Idempotent: re-runs are no-ops unless the patch file changed.
set -euo pipefail

# SHA-256 front end — probed by running, not `command -v`; see
# fetch-multistream-select.sh for why.
if echo | shasum -a 256 >/dev/null 2>&1; then
    sha256() { shasum -a 256 "$@"; }
elif echo | sha256sum >/dev/null 2>&1; then
    sha256() { sha256sum "$@"; }
else
    echo "error: need a working 'shasum' or 'sha256sum' to verify the download" >&2
    exit 1
fi

VERSION=0.48.0
SHA256=13d3fd632a5872ec804d37e7413ceea20588f69d027a0fa3c46f82574f4dee60
DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$DIR/libp2p-kad"
PATCH="$DIR/libp2p-kad.patch"
STAMP="$DEST/.kwaai-patch-stamp"

want_stamp="$VERSION $(sha256 "$PATCH" | cut -d' ' -f1)"
if [ -f "$STAMP" ] && [ "$(cat "$STAMP")" = "$want_stamp" ]; then
    exit 0
fi

echo "fetching libp2p-kad $VERSION and applying the multi-protocol-names patch..."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

crate="$tmp/kad.crate"
curl -fsSL -o "$crate" \
    "https://static.crates.io/crates/libp2p-kad/libp2p-kad-$VERSION.crate"
echo "$SHA256  $crate" | sha256 -c - >/dev/null

tar -xzf "$crate" -C "$tmp"
rm -rf "$DEST"
mv "$tmp/libp2p-kad-$VERSION" "$DEST"
patch -p1 -d "$DEST" --no-backup-if-mismatch <"$PATCH" >/dev/null
echo "$want_stamp" >"$STAMP"
echo "patched libp2p-kad ready at core/patches/libp2p-kad"
