# KwaaiNet Windows installer
# Usage: irm https://raw.githubusercontent.com/Kwaai-AI-Lab/KwaaiNet/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

$repo    = "Kwaai-AI-Lab/KwaaiNet"

# Resolve the latest release version via the GitHub API so we use a versioned
# URL instead of the /releases/latest alias, which can serve stale CDN-cached
# assets for several minutes after a new release is published.
$version = (Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest").tag_name
Write-Host "Version  : $version" -ForegroundColor Cyan
$asset   = "kwaainet-$version-x86_64-pc-windows-msvc.zip"
$url     = "https://github.com/$repo/releases/download/$version/$asset"
$zip     = "$env:TEMP\kwaainet-install.zip"
$dst     = "$env:LOCALAPPDATA\kwaainet"

Write-Host "Downloading KwaaiNet..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $url -OutFile $zip

Write-Host "Installing to $dst ..." -ForegroundColor Cyan
if (Test-Path $dst) { Remove-Item $dst -Recurse -Force }
Expand-Archive $zip $dst -Force
Remove-Item $zip

# Add to PATH for this session
if ($env:Path -notlike "*$dst*") {
    $env:Path += ";$dst"
}

# Persist to user PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$dst*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$dst", "User")
    Write-Host "Added $dst to your PATH (restart terminal to take effect in new windows)" -ForegroundColor Yellow
}

# Remove stale copies from other locations (e.g. cargo install) that would shadow the new binary
$stalePaths = @(
    "$env:USERPROFILE\.cargo\bin\kwaainet.exe",
    "$env:USERPROFILE\.cargo\bin\p2pd.exe"
)
foreach ($stale in $stalePaths) {
    if (Test-Path $stale) {
        Remove-Item $stale -Force
        Write-Host "Removed stale binary: $stale" -ForegroundColor Yellow
    }
}

# Prepend install dir so it wins over any remaining PATH entries
$env:Path = "$dst;$env:Path"

Write-Host "Running kwaainet setup..." -ForegroundColor Cyan
& "$dst\kwaainet.exe" setup

Write-Host ""
Write-Host "KwaaiNet installed successfully." -ForegroundColor Green
Write-Host "Run 'kwaainet start --daemon' to join the network." -ForegroundColor Green
