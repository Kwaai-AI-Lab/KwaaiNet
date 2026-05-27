# KwaaiNet Windows updater — run with:
#   irm https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/main/scripts/update-windows.ps1 | iex

$ErrorActionPreference = 'Stop'

$api  = 'https://api.github.com/repos/Kwaai-AI-Lab/KwaaiNet/releases/latest'
$info = Invoke-RestMethod -Uri $api -Headers @{ 'User-Agent' = 'kwaainet-updater' }
$ver  = $info.tag_name.TrimStart('v')

$installDir = (Get-Command kwaainet -ErrorAction SilentlyContinue)?.Source |
              ForEach-Object { Split-Path $_ } |
              Select-Object -First 1
if (-not $installDir) {
    $installDir = Join-Path $env:USERPROFILE '.cargo\bin'
}

Write-Host "Installing kwaainet v$ver to $installDir"

$zip = Join-Path $env:TEMP 'kwaainet-update.zip'
$tmp = Join-Path $env:TEMP 'kwaainet-upd-extract'

# Prefer the CUDA-enabled zip on NVIDIA GPU machines.
$cpuUrl  = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v$ver/kwaainet-x86_64-pc-windows-msvc.zip"
$cudaUrl = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v$ver/kwaainet-x86_64-pc-windows-msvc-cuda.zip"

$hasGpu = (nvidia-smi --query-gpu=name --format=csv,noheader 2>$null) -ne $null
$isCuda = $false
$url    = $cpuUrl

if ($hasGpu) {
    try {
        $resp = Invoke-WebRequest -Uri $cudaUrl -Method Head -UseBasicParsing -ErrorAction Stop
        if ($resp.StatusCode -eq 200) {
            Write-Host "NVIDIA GPU detected — downloading CUDA build..."
            $url    = $cudaUrl
            $isCuda = $true
        }
    } catch {
        Write-Host "NVIDIA GPU detected but CUDA build for v$ver isn't published yet."
        Write-Host "Installing CPU build — run this script again later for GPU support."
    }
}

Write-Host "Downloading..."
Invoke-WebRequest -Uri $url -OutFile $zip

if (Test-Path $tmp) { Remove-Item $tmp -Recurse -Force }
Expand-Archive -LiteralPath $zip -DestinationPath $tmp -Force

Write-Host "Stopping kwaainet..."
Stop-Process -Name kwaainet,p2pd -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# For CUDA zips include *.dll so bundled CUDA runtime DLLs are installed.
$include = if ($isCuda) { @('*.exe', '*.dll') } else { @('*.exe') }
Get-ChildItem -Path $tmp -Recurse -Include $include | ForEach-Object {
    $dest = Join-Path $installDir $_.Name
    Copy-Item -Path $_.FullName -Destination $dest -Force
    Write-Host "  Installed $($_.Name)"
}

Remove-Item $zip, $tmp -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "Starting daemon..."
Start-Process -FilePath (Join-Path $installDir 'kwaainet.exe') -ArgumentList 'start', '--daemon' -WindowStyle Hidden
Write-Host "Done. kwaainet v$ver installed."
