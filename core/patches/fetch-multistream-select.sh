#!/usr/bin/env bash
# Fetch multistream-select 0.13.0 from crates.io and apply the slash-less
# protocol-ID patch. The patched source is used via `[patch.crates-io]` in
# core/Cargo.toml and is NOT committed — this repo carries only the patch
# file, which is the entire intentional delta against upstream.
#
# Idempotent: re-runs are no-ops unless the patch file changed.
set -euo pipefail

VERSION=0.13.0
SHA256=ea0df8e5eec2298a62b326ee4f0d7fe1a6b90a09dfcf9df37b38f947a8c42f19
DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$DIR/multistream-select"
PATCH="$DIR/multistream-select.patch"
STAMP="$DEST/.kwaai-patch-stamp"

want_stamp="$VERSION $(shasum -a 256 "$PATCH" | cut -d' ' -f1)"
if [ -f "$STAMP" ] && [ "$(cat "$STAMP")" = "$want_stamp" ]; then
    exit 0
fi

echo "fetching multistream-select $VERSION and applying the slash-less patch..."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

crate="$tmp/ms.crate"
curl -fsSL -o "$crate" \
    "https://static.crates.io/crates/multistream-select/multistream-select-$VERSION.crate"
echo "$SHA256  $crate" | shasum -a 256 -c - >/dev/null

tar -xzf "$crate" -C "$tmp"
rm -rf "$DEST"
mv "$tmp/multistream-select-$VERSION" "$DEST"
patch -p1 -d "$DEST" --no-backup-if-mismatch <"$PATCH" >/dev/null
echo "$want_stamp" >"$STAMP"
echo "patched multistream-select ready at core/patches/multistream-select"
