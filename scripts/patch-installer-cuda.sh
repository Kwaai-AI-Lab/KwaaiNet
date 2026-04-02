#!/usr/bin/env bash
# Patch the cargo-dist shell installer to auto-detect NVIDIA GPUs and
# prefer the CUDA-enabled archive when available.  When CUDA is selected,
# bundled CUDA runtime .so files are also installed alongside the binaries.
#
# Usage: patch-installer-cuda.sh <path-to-kwaainet-installer.sh>

set -euo pipefail

INSTALLER="${1:?Usage: patch-installer-cuda.sh <installer-path>}"

if [ ! -f "$INSTALLER" ]; then
    echo "ERROR: installer not found: $INSTALLER"
    exit 1
fi

# Write the CUDA detection function to a temp file.
CUDA_FUNC=$(mktemp)
cat > "$CUDA_FUNC" <<'CUDA_PATCH'

# ── NVIDIA CUDA variant auto-detection (injected by CI) ──────────
_cuda_selected=""
_cuda_libs_staged=""
_try_cuda_upgrade() {
    # Only applies to x86_64 linux targets (gnu or musl fallback)
    case "${_artifact_name:-}" in
        kwaainet-x86_64-unknown-linux-gnu*) ;;
        kwaainet-x86_64-unknown-linux-musl*) ;;
        *) return 0 ;;
    esac
    if ! command -v nvidia-smi >/dev/null 2>&1; then
        return 0
    fi
    _gpu_name=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)
    if [ -z "$_gpu_name" ]; then return 0; fi
    say "NVIDIA GPU detected: $_gpu_name — using CUDA-enabled build"
    _cuda_name="kwaainet-x86_64-unknown-linux-gnu-cuda.tar.xz"
    _cuda_url="${_url%/*}/${_cuda_name}"
    if curl --output /dev/null --silent --head --fail "$_cuda_url"; then
        _artifact_name="$_cuda_name"
        _url="$_cuda_url"
        _cuda_selected="1"
        _cuda_hash=$(curl -sSfL "${_url}.sha256" 2>/dev/null | awk '{print $1}')
        if [ -n "$_cuda_hash" ]; then
            _checksum_value="$_cuda_hash"
        else
            _checksum_style=""
        fi
    else
        say "CUDA archive not found for this release — falling back to CPU build"
    fi
}

# Stage: copy CUDA .so files from source dir to install temp dir.
_stage_cuda_libs() {
    local _src="$1" _staging="$2"
    if [ -z "$_cuda_selected" ]; then return 0; fi
    _cuda_libs_staged=""
    for _f in "$_src"/*.so*; do
        [ -f "$_f" ] || continue
        local _name; _name=$(basename "$_f")
        cp "$_f" "$_staging/$_name"
        chmod +x "$_staging/$_name"
        _cuda_libs_staged="$_cuda_libs_staged $_name"
        say "  $_name (CUDA runtime)"
    done
}

# Final: move staged CUDA .so files from temp to final install dir.
_finalize_cuda_libs() {
    local _staging="$1" _dest="$2"
    for _name in $_cuda_libs_staged; do
        [ -f "$_staging/$_name" ] && mv "$_staging/$_name" "$_dest/$_name"
    done
}
CUDA_PATCH

# Insert the functions right after the shebang line.
{
    head -1 "$INSTALLER"
    cat "$CUDA_FUNC"
    tail -n +2 "$INSTALLER"
} > "${INSTALLER}.tmp"
mv "${INSTALLER}.tmp" "$INSTALLER"
chmod +x "$INSTALLER"
rm -f "$CUDA_FUNC"

# 1) Inject _try_cuda_upgrade just before the download.
sed -i '/downloader "$_url" "$_file"/i\    _try_cuda_upgrade' "$INSTALLER"

# 2) Inject _stage_cuda_libs after the first bin staging loop.
#    The marker: "# Like the above, but no aliases" (right after the bin loop).
sed -i '/# Like the above, but no aliases/i\    _stage_cuda_libs "$_src_dir" "$_install_temp"' "$INSTALLER"

# 3) Inject _finalize_cuda_libs after the second bin mv loop.
#    The marker: "for _lib_name in $_libs $_staticlibs; do" (second occurrence,
#    in the final mv section). We use the line right before cleanup.
sed -i '/ignore rm -rf "$_install_temp"/i\    _finalize_cuda_libs "$_install_temp" "$_install_dir"' "$INSTALLER"

echo "Patched ${INSTALLER}: CUDA auto-detection and library install added"
