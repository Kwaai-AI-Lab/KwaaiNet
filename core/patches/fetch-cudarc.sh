#!/usr/bin/env bash
# Fetch cudarc 0.19.7 from crates.io and apply the sync-allocation opt-out
# patch. The patched source is used via `[patch.crates-io]` in core/Cargo.toml
# and is NOT committed — this repo carries only the patch file, which is the
# entire intentional delta against upstream.
#
# Idempotent: re-runs are no-ops unless the patch file changed.
set -euo pipefail

# Partial by design: this fetches one crate. Run directly it still exits 0 with
# the other patched crates missing, and the omission only surfaces later as a
# workspace parse failure — so say so. fetch-patches.sh sets the variable.
if [ -z "${KWAAI_FETCH_PATCHES:-}" ]; then
    echo "note: this script fetches only cudarc; run core/patches/fetch-patches.sh to get every patched crate" >&2
fi

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

VERSION=0.19.7
SHA256=1cea5f10a99e025c1b44ae2354c2d8326b25ddbd0baf76bde8e55cfd4018a2cc
DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$DIR/cudarc"
PATCH="$DIR/cudarc.patch"
STAMP="$DEST/.kwaai-patch-stamp"

want_stamp="$VERSION $(sha256 "$PATCH" | cut -d' ' -f1)"
if [ -f "$STAMP" ] && [ "$(cat "$STAMP")" = "$want_stamp" ]; then
    exit 0
fi

echo "fetching cudarc $VERSION and applying the sync-allocation opt-out patch..."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

crate="$tmp/cudarc.crate"
curl -fsSL -o "$crate" \
    "https://static.crates.io/crates/cudarc/cudarc-$VERSION.crate"
echo "$SHA256  $crate" | sha256 -c - >/dev/null

tar -xzf "$crate" -C "$tmp"
rm -rf "$DEST"
mv "$tmp/cudarc-$VERSION" "$DEST"
patch -p1 -d "$DEST" --no-backup-if-mismatch <"$PATCH" >/dev/null
echo "$want_stamp" >"$STAMP"
echo "patched cudarc ready at core/patches/cudarc"
