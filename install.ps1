# KwaaiNet Windows installer (cargo-dist generated, PowerShell variant)
#
# Usage:
#   irm https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-installer.ps1 | iex
#
# This script is a convenience wrapper.  The canonical installer is the
# cargo-dist-generated kwaainet-installer.ps1 that is uploaded as a release
# asset on every release.  It handles checksum verification and PATH setup.

$ErrorActionPreference = "Stop"

$repo    = "Kwaai-AI-Lab/KwaaiNet"

# Resolve the latest release tag so we fetch the versioned installer URL
# (avoids stale CDN caches on /releases/latest).
$version = (Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest").tag_name
Write-Host "Installing KwaaiNet $version ..." -ForegroundColor Cyan

$installerUrl = "https://github.com/$repo/releases/download/$version/kwaainet-installer.ps1"
Invoke-Expression (Invoke-RestMethod $installerUrl)
