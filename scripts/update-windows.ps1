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
$url = "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/download/v$ver/kwaainet-x86_64-pc-windows-msvc.zip"

Write-Host "Downloading..."
Invoke-WebRequest -Uri $url -OutFile $zip

if (Test-Path $tmp) { Remove-Item $tmp -Recurse -Force }
Expand-Archive -LiteralPath $zip -DestinationPath $tmp -Force

Write-Host "Stopping kwaainet..."
Stop-Process -Name kwaainet -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

Get-ChildItem -Path $tmp -Recurse -Include '*.exe' | ForEach-Object {
    $dest = Join-Path $installDir $_.Name
    Copy-Item -Path $_.FullName -Destination $dest -Force
    Write-Host "  Installed $($_.Name)"
}

Remove-Item $zip, $tmp -Recurse -Force -ErrorAction SilentlyContinue

Write-Host "Starting daemon..."
Start-Process -FilePath (Join-Path $installDir 'kwaainet.exe') -ArgumentList 'start', '--daemon' -WindowStyle Hidden
Write-Host "Done. kwaainet v$ver installed."
