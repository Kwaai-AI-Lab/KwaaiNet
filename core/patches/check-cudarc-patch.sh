#!/usr/bin/env bash
# Fail if cargo no longer resolves cudarc to core/patches/cudarc. An unused
# `[patch.crates-io]` entry is only a warning, so a candle bump past the
# patched major would silently drop the Jetson fix. Run from core/.
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
want="$(sed -n 's/^VERSION=//p' "$DIR/fetch-cudarc.sh")"

# cudarc is optional (CUDA features only), so resolve with every feature on.
# A path-patched package has no `source`; registry copies (ug-cuda's 0.17.x) do.
got="$(cargo metadata --format-version 1 --locked --all-features \
    | jq -r '.packages[] | select(.name == "cudarc" and .source == null) | .version')"

if [ "$got" != "$want" ]; then
    echo "error: patched cudarc $want is not in the dependency graph (resolved: '${got:-none}')." >&2
    echo "       candle now wants a different cudarc; re-pin patches/fetch-cudarc.sh and cudarc.patch." >&2
    exit 1
fi
echo "cudarc $want resolves to the patched source"
