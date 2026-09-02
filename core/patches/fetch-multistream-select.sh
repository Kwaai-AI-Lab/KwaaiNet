#!/usr/bin/env bash
# Fetch multistream-select 0.13.0 from crates.io and apply the slash-less
# protocol-ID patch. The patched source is used via `[patch.crates-io]` in
# core/Cargo.toml and is NOT committed — this repo carries only the patch
# file, which is the entire intentional delta against upstream.
#
# Idempotent: re-runs are no-ops unless the patch file changed.
set -euo pipefail

# Partial by design: this fetches one crate. Run directly it still exits 0 with
# the other patched crates missing, and the omission only surfaces later as a
# workspace parse failure — so say so. fetch-patches.sh sets the variable.
if [ -z "${KWAAI_FETCH_PATCHES:-}" ]; then
    echo "note: this script fetches only multistream-select; run core/patches/fetch-patches.sh to get every patched crate" >&2
fi

# SHA-256 front end. Stock macOS ships `shasum` (a perl script) but not
# `sha256sum`; most Linux images ship `sha256sum` and usually, but not always,
# `shasum`. Git Bash on a Windows runner reaches this script through
# `run: bash …` in release.yml, and that path runs only on a tag push — so a
# hashing problem there would surface during a release rather than in any PR
# check. Rather than establish which tool every host has, use whichever works.
#
# Probed by running them, not by `command -v`: `shasum` is a perl script, so it
# can exist on PATH and still fail (a Git Bash with a broken or absent perl is
# exactly that shape). Existence is not usability, and picking a present-but-
# broken tool would skip a working fallback — verified against a stub `shasum`
# that exits 2 with a perl error.
#
# Both accept the same `HASH  FILE` input in `-c` mode, so one shim covers the
# stamp calculation and the crate verification alike.
if echo | shasum -a 256 >/dev/null 2>&1; then
    sha256() { shasum -a 256 "$@"; }
elif echo | sha256sum >/dev/null 2>&1; then
    sha256() { sha256sum "$@"; }
else
    echo "error: need a working 'shasum' or 'sha256sum' to verify the download" >&2
    echo "       (macOS: shasum, via perl; Debian/Ubuntu: coreutils;" >&2
    echo "        Windows: Git Bash — if shasum is present but failing, perl is" >&2
    echo "        the usual cause)" >&2
    exit 1
fi

VERSION=0.13.0
SHA256=ea0df8e5eec2298a62b326ee4f0d7fe1a6b90a09dfcf9df37b38f947a8c42f19
DIR="$(cd "$(dirname "$0")" && pwd)"
DEST="$DIR/multistream-select"
PATCH="$DIR/multistream-select.patch"
STAMP="$DEST/.kwaai-patch-stamp"

want_stamp="$VERSION $(sha256 "$PATCH" | cut -d' ' -f1)"
if [ -f "$STAMP" ] && [ "$(cat "$STAMP")" = "$want_stamp" ]; then
    exit 0
fi

echo "fetching multistream-select $VERSION and applying the slash-less patch..."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

crate="$tmp/ms.crate"
curl -fsSL -o "$crate" \
    "https://static.crates.io/crates/multistream-select/multistream-select-$VERSION.crate"
echo "$SHA256  $crate" | sha256 -c - >/dev/null

tar -xzf "$crate" -C "$tmp"
rm -rf "$DEST"
mv "$tmp/multistream-select-$VERSION" "$DEST"
patch -p1 -d "$DEST" --no-backup-if-mismatch <"$PATCH" >/dev/null
echo "$want_stamp" >"$STAMP"
echo "patched multistream-select ready at core/patches/multistream-select"
