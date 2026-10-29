#!/usr/bin/env bash
# Package the Linux CUDA build as two archives, and refuse to produce either
# unless the full one runs on a host that has only an NVIDIA driver.
#
#   <label>.tar.xz       kwaainet + the CUDA runtime libraries it links (full)
#   <label>-lean.tar.xz  kwaainet only, for hosts that already have those libraries
#
# The full archive keeps the name every earlier installer and updater
# downloads, so a host without a CUDA 12 toolkit is repaired by its next
# update rather than handed another binary that cannot start (#240).
#
# The binary must carry DT_RPATH $ORIGIN (link with
# -Wl,--disable-new-dtags,-rpath,$ORIGIN). RPATH, unlike RUNPATH, also
# applies to the libraries' own dependencies: libcublas loads libcublasLt.
#
# Usage: package-cuda-linux.sh <kwaainet-binary> <label> <cuda-lib-dir> <out-dir>

set -euo pipefail

BIN="${1:?Usage: package-cuda-linux.sh <binary> <label> <cuda-lib-dir> <out-dir>}"
LABEL="${2:?missing label}"
CUDA_LIB_ARG="${3:?missing cuda lib dir}"
OUT="${4:?missing out dir}"

fail() { echo "ERROR: $*" >&2; exit 1; }

for tool in readelf ldd sha256sum tar; do
    command -v "$tool" >/dev/null || fail "$tool is not installed"
done
[ -x "$BIN" ] || fail "binary not found or not executable: $BIN"
# lib64 is a symlink to targets/x86_64-linux/lib in NVIDIA's packages, and
# `find` does not descend into a symlinked start directory. That is how every
# archive since at least v0.4.100 shipped with no libraries at all.
CUDA_LIB="$(readlink -f "$CUDA_LIB_ARG")"
[ -d "$CUDA_LIB" ] || fail "CUDA library dir not found: $CUDA_LIB_ARG"

readelf -d "$BIN" | grep -F '(RPATH)' | grep -qF '[$ORIGIN]' \
    || fail "$BIN has no DT_RPATH \$ORIGIN, so it would not find libraries beside it:
$(readelf -d "$BIN" | grep -E 'RPATH|RUNPATH' || echo '  (no RPATH or RUNPATH)')"

# ── Collect what the binary loads from the toolkit ─────────────────────
# ldd lists dependencies of dependencies too. libcuda.so.1 is the driver's,
# installed with it, and must come from the host.
deps="$(LD_LIBRARY_PATH="$CUDA_LIB" ldd "$BIN")"
missing="$(printf '%s\n' "$deps" | awk '/=> not found/ && $1 != "libcuda.so.1" {print "  " $1}')"
[ -z "$missing" ] || fail "libraries not found in $CUDA_LIB or on the system:
$missing"
cuda_libs="$(printf '%s\n' "$deps" | awk -v d="$CUDA_LIB/" 'index($3, d) == 1 {print $1 " " $3}')"
[ -n "$cuda_libs" ] || fail "$BIN loads nothing from $CUDA_LIB; is this the CUDA build?"

rm -rf "$OUT/$LABEL" "$OUT/lean"
mkdir -p "$OUT/$LABEL" "$OUT/lean/$LABEL"
cp "$BIN" "$OUT/$LABEL/kwaainet"
cp "$BIN" "$OUT/lean/$LABEL/kwaainet"
chmod 755 "$OUT/$LABEL/kwaainet" "$OUT/lean/$LABEL/kwaainet"
while read -r name path; do
    # Under the name the loader asks for, as a regular file: symlinks would
    # not survive every installer's copy.
    cp -L "$path" "$OUT/$LABEL/$name"
    chmod 755 "$OUT/$LABEL/$name"
    echo "  Bundled: $name ($(du -h "$OUT/$LABEL/$name" | cut -f1))"
done <<< "$cuda_libs"

# ── Verify the full archive resolves against itself ────────────────────
FULL_DIR="$(cd "$OUT/$LABEL" && pwd -P)"
resolved="$(env -u LD_LIBRARY_PATH ldd "$FULL_DIR/kwaainet")"
while read -r name _; do
    printf '%s\n' "$resolved" | grep -qF "$name => $FULL_DIR/$name " \
        || fail "$name does not resolve to the bundled copy:
$(printf '%s\n' "$resolved" | grep -F "$name" || echo "  $name: absent from ldd output")"
done <<< "$cuda_libs"
missing="$(printf '%s\n' "$resolved" | awk '/=> not found/ && $1 != "libcuda.so.1" {print "  " $1}')"
[ -z "$missing" ] || fail "the bundled binary still misses:
$missing"

# Start it with nothing but a stand-in for the driver: the toolkit's stub
# libcuda, which loads but answers every call with CUDA_ERROR_STUB_LIBRARY.
STUB="$CUDA_LIB/stubs/libcuda.so"
[ -f "$STUB" ] || fail "no driver stub at $STUB to run the bundled binary against"
DRIVER_DIR="$(mktemp -d)"
trap 'rm -rf "$DRIVER_DIR"' EXIT
ln -s "$STUB" "$DRIVER_DIR/libcuda.so.1"
version="$(env -u LD_LIBRARY_PATH LD_LIBRARY_PATH="$DRIVER_DIR" "$FULL_DIR/kwaainet" --version)" \
    || fail "the bundled binary does not start"
echo "  Bundled binary starts: $version"

# ── Archives ───────────────────────────────────────────────────────────
(
    cd "$OUT"
    tar -cJf "$LABEL.tar.xz" "$LABEL"
    tar -C lean -cJf "$LABEL-lean.tar.xz" "$LABEL"
    for f in "$LABEL.tar.xz" "$LABEL-lean.tar.xz"; do
        sha256sum "$f" > "$f.sha256"
        echo "  $f: $(du -h "$f" | cut -f1)"
    done
)
