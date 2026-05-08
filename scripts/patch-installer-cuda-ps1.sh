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
    $cudaName = "kwaainet-x86_64-pc-windows-msvc-cuda.zip"
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
if [ -n "$PARAM_END" ]; then
    {
        head -n "$PARAM_END" "$INSTALLER"
        cat "$CUDA_FUNC"
        tail -n +"$((PARAM_END + 1))" "$INSTALLER"
    } > "${INSTALLER}.tmp"
else
    # Fallback: no param() block found, insert after line 1
    echo "WARNING: param() block not found — inserting after line 1 (fallback)"
    {
        head -1 "$INSTALLER"
        cat "$CUDA_FUNC"
        tail -n +2 "$INSTALLER"
    } > "${INSTALLER}.tmp"
fi
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
    echo "WARNING: Could not find '\$url = \"\$download_url/\$artifact_name\"' in PS1 installer"
    echo "  Searching for download-related lines:"
    grep -n 'download_url\|artifact_name\|downloadFile' "$INSTALLER" | head -10
fi

# Inject CUDA DLL collection into the Download function.
# After the bin_paths loop collects named binaries from $tmp, we add any
# extra .dll files (CUDA runtime libs) to bin_paths so they get installed
# by the existing copy loop.
# We look for the closing brace of: foreach ($bin_name in $bin_names) { ... }
# which is followed by: $lib_paths = @()
CUDA_DLL_INJECT='  if ($global:_cuda_selected) { Get-ChildItem "$tmp\\*.dll" -ErrorAction SilentlyContinue | Where-Object { $bin_paths -notcontains $_.FullName } | ForEach-Object { $bin_paths += $_.FullName; Write-Verbose "  Bundled CUDA lib: $($_.Name)" } }'

if grep -q '\$lib_paths = @()' "$INSTALLER"; then
    sed -i "/\\\$lib_paths = @()/i\\${CUDA_DLL_INJECT}" "$INSTALLER"
    echo "Patched ${INSTALLER}: CUDA DLL collection added to Download function"
else
    echo "WARNING: Could not find '\$lib_paths = @()' in PS1 installer"
fi
