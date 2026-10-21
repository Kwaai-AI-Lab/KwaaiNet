#!/usr/bin/env bash
# Patch the cargo-dist PowerShell installer to auto-detect NVIDIA GPUs and
# prefer the CUDA-enabled archive when available.  When CUDA is selected,
# bundled CUDA runtime DLLs are also installed alongside the binaries.
#
# Usage: patch-installer-cuda-ps1.sh <path-to-kwaainet-installer.ps1>

set -euo pipefail

INSTALLER="${1:?Usage: patch-installer-cuda-ps1.sh <installer-path>}"

if [ ! -f "$INSTALLER" ]; then
    echo "ERROR: installer not found: $INSTALLER"
    exit 1
fi

# Write the CUDA detection function to a temp file.
CUDA_FUNC=$(mktemp)
cat > "$CUDA_FUNC" <<'CUDA_PATCH'

# ── NVIDIA CUDA variant auto-detection (injected by CI) ──────────
$global:_cuda_selected = $false
$global:_cuda_checksum = ""

function Invoke-CudaUpgrade($download_url, [ref]$artifact_name_ref) {
    $current = $artifact_name_ref.Value
    if ($current -notlike "*x86_64-pc-windows-msvc*") { return }
    $nvidiaSmi = Get-Command "nvidia-smi" -ErrorAction SilentlyContinue
    if (-not $nvidiaSmi) { return }
    try {
        $gpuName = & nvidia-smi --query-gpu=name --format=csv,noheader 2>$null | Select-Object -First 1
    } catch { return }
    if ([string]::IsNullOrWhiteSpace($gpuName)) { return }
    # The lean archive is the binary alone; the CUDA runtime DLLs live only in
    # the -cuda-full archive. A machine installing for the first time has no
    # runtime to link against, and the CUDA binary then dies at startup with
    # 0xC0000135 (DLL not found) before printing anything. So take the full
    # archive unless cublas64_12.dll is already resolvable here — the same
    # choice install_update() makes in updater.rs, on the same file.
    $cudaRuntimePresent = $false
    $probeDirs = @()
    if ($env:KWAAINET_INSTALL_DIR) {
        $probeDirs += $env:KWAAINET_INSTALL_DIR
        $probeDirs += (Join-Path $env:KWAAINET_INSTALL_DIR "bin")
    }
    if ($env:CARGO_HOME) { $probeDirs += (Join-Path $env:CARGO_HOME "bin") }
    else { $probeDirs += (Join-Path $HOME ".cargo\bin") }
    if ($env:CUDA_PATH) { $probeDirs += (Join-Path $env:CUDA_PATH "bin") }
    $probeDirs += ($env:PATH -split ";" | Where-Object { $_ })
    foreach ($dir in $probeDirs) {
        # Not Test-Path: a PATH entry with invalid characters makes it emit a
        # non-terminating ArgumentException, which try/catch does not catch —
        # it printed a red error mid-install. File::Exists just returns false.
        try {
            $probe = [System.IO.Path]::Combine($dir.Trim('"'), "cublas64_12.dll")
        } catch { continue }
        if ([System.IO.File]::Exists($probe)) { $cudaRuntimePresent = $true; break }
    }
    if ($cudaRuntimePresent) {
        $cudaName = "kwaainet-x86_64-pc-windows-msvc-cuda.zip"
    } else {
        $cudaName = "kwaainet-x86_64-pc-windows-msvc-cuda-full.zip"
        Write-Host "No CUDA runtime on this machine - downloading the full build (~1 GB, once)"
    }
    $cudaUrl = "$download_url/$cudaName"
    try {
        Invoke-WebRequest -Uri $cudaUrl -Method Head -ErrorAction Stop | Out-Null
        Write-Host "NVIDIA GPU detected: $gpuName - using CUDA-enabled build"
        $artifact_name_ref.Value = $cudaName
        $global:_cuda_selected = $true
        # Fetch the CUDA archive hash so the installer verifies the right file.
        # On failure we set an empty string, which skips verification rather than
        # hard-failing with the CPU archive's embedded hash.
        try {
            $hashContent = (Invoke-WebRequest -Uri "$cudaUrl.sha256" -UseBasicParsing -ErrorAction Stop).Content
            $cudaHash = ($hashContent -split '[\s\r\n]+')[0].Trim().ToLower()
            if ($cudaHash -match '^[0-9a-f]{64}$') {
                $global:_cuda_checksum = $cudaHash
            }
        } catch {
            $global:_cuda_checksum = ""
        }
    } catch {
        Write-Host "CUDA archive not found for this release - falling back to CPU build"
    }
}
CUDA_PATCH

# Insert the function after the param() block.
# PowerShell requires param() to be the first non-comment statement, so we
# must inject after its closing ')' rather than at the top of the file.
PARAM_END=$(awk '/^param/{found=1} found && /^\)/{print NR; exit}' "$INSTALLER")
if [ -z "$PARAM_END" ]; then
    # There is no safe fallback: PowerShell requires param() to be the first
    # non-comment statement, so injecting anywhere else produces an installer
    # that will not parse. Fail the release instead of shipping one.
    echo "ERROR: no param() block in $INSTALLER — cannot inject the CUDA helper"
    exit 1
fi
{
    head -n "$PARAM_END" "$INSTALLER"
    cat "$CUDA_FUNC"
    tail -n +"$((PARAM_END + 1))" "$INSTALLER"
} > "${INSTALLER}.tmp"
mv "${INSTALLER}.tmp" "$INSTALLER"
rm -f "$CUDA_FUNC"

# Inject the call just before the URL is constructed from $artifact_name.
if grep -q '\$url = "\$download_url/\$artifact_name"' "$INSTALLER"; then
    sed -i '/\$url = "\$download_url\/\$artifact_name"/i\  Invoke-CudaUpgrade $download_url ([ref]$artifact_name)' "$INSTALLER"
    # After the URL is set, override the embedded CPU checksum with the CUDA one.
    # The installer embeds the CPU archive hash at build time; without this patch
    # the CUDA archive would fail hash verification and abort the install.
    sed -i '/\$url = "\$download_url\/\$artifact_name"/a\  if ($global:_cuda_selected) { if ($global:_cuda_checksum -ne "") { $checksum = $global:_cuda_checksum } else { $checksum = "" } }' "$INSTALLER"
    echo "Patched ${INSTALLER}: CUDA auto-detection and checksum override added"
else
    echo "ERROR: could not find '\$url = \"\$download_url/\$artifact_name\"' in PS1 installer"
    echo "  Download-related lines, for whoever updates this script:"
    grep -n 'download_url\|artifact_name\|downloadFile' "$INSTALLER" | head -10
    exit 1
fi

# Inject CUDA DLL collection into the Download function.
# After the bin_paths loop collects named binaries from $tmp, we add any
# extra .dll files (CUDA runtime libs) to bin_paths so they get installed
# by the existing copy loop.
# We look for the closing brace of: foreach ($bin_name in $bin_names) { ... }
# which is followed by: $lib_paths = @()
CUDA_DLL_INJECT='  if ($global:_cuda_selected) { Get-ChildItem -Path "$tmp" -Filter *.dll -Recurse -ErrorAction SilentlyContinue | Where-Object { $bin_paths -notcontains $_.FullName } | ForEach-Object { $bin_paths += $_.FullName; Write-Host "  Bundled CUDA lib: $($_.Name)" } }'

if grep -q '\$lib_paths = @()' "$INSTALLER"; then
    sed -i "/\\\$lib_paths = @()/i\\${CUDA_DLL_INJECT}" "$INSTALLER"
    echo "Patched ${INSTALLER}: CUDA DLL collection added to Download function"
else
    echo "ERROR: could not find '\$lib_paths = @()' in PS1 installer"
    exit 1
fi

# ── Verify, because `sed -i` succeeds when it matches nothing ────────────
# An unpatched installer is indistinguishable from a patched one at a glance,
# and the difference only shows up as a CPU-only binary on somebody's GPU
# machine months later. Assert every injection landed.
missing=""
for marker in \
    'function Invoke-CudaUpgrade' \
    'Invoke-CudaUpgrade $download_url ([ref]$artifact_name)' \
    'if ($global:_cuda_selected) { if ($global:_cuda_checksum' \
    'Bundled CUDA lib:'
do
    grep -qF -- "$marker" "$INSTALLER" || missing="${missing}
  ${marker}"
done
if [ -n "$missing" ]; then
    echo "ERROR: ${INSTALLER} is missing CUDA injections after patching:${missing}"
    exit 1
fi
echo "Verified ${INSTALLER}: all CUDA injections present"
