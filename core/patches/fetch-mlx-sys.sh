#!/usr/bin/env bash
# Fetch mlx-sys 0.2.0 from crates.io and apply the embedded-metallib patch.
# It also installs mlx-metallib.patch, which cmake applies to the MLX source
# it fetches at build time. The patched source is used via `[patch.crates-io]`
# in core/Cargo.toml and is NOT committed — this repo carries only the patch file, which is the
# entire intentional delta against upstream.
#
# Idempotent: re-runs are no-ops unless the patch file changed.
set -euo pipefail

# Partial by design: this fetches one crate. Run directly it still exits 0 with
# the other patched crates missing, and the omission only surfaces later as a
# workspace parse failure — so say so. fetch-patches.sh sets the variable.
if [ -z "${KWAAI_FETCH_PATCHES:-}" ]; then
    echo "note: this script fetches only mlx-sys; run core/patches/fetch-patches.sh to get every patched crate" >&2
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

VERSION=0.2.0
SHA256=2e3bc3880111918b2d5018f845d48fd995f9901f16efc81d1fcfd2f4210b8219
DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$DIR/mlx-sys"
PATCH="$DIR/mlx-sys.patch"
MLX_PATCH="$DIR/mlx-metallib.patch"
MLX_APPLY="$DIR/mlx-apply-metallib-patch.sh"
STAMP="$DEST/.kwaai-patch-stamp"

want_stamp="$VERSION $(cat "$PATCH" "$MLX_PATCH" "$MLX_APPLY" | sha256 | cut -d' ' -f1)"
if [ -f "$STAMP" ] && [ "$(cat "$STAMP")" = "$want_stamp" ]; then
    exit 0
fi

echo "fetching mlx-sys $VERSION and applying the embedded-metallib patch..."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

crate="$tmp/mlx-sys.crate"
curl -fsSL -o "$crate" \
    "https://static.crates.io/crates/mlx-sys/mlx-sys-$VERSION.crate"
echo "$SHA256  $crate" | sha256 -c - >/dev/null

tar -xzf "$crate" -C "$tmp"
rm -rf "$DEST"
mv "$tmp/mlx-sys-$VERSION" "$DEST"
patch -p1 -d "$DEST" --no-backup-if-mismatch <"$PATCH" >/dev/null
cp "$MLX_PATCH" "$DEST/src/mlx-c/kwaai-mlx-metallib.patch"
cp "$MLX_APPLY" "$DEST/src/mlx-c/kwaai-mlx-apply.sh"
echo "$want_stamp" >"$STAMP"
echo "patched mlx-sys ready at core/patches/mlx-sys"
