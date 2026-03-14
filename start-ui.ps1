# Start KwaaiNet Node Dashboard (local web UI)
# Run with: powershell -ExecutionPolicy Bypass -File start-ui.ps1
# For Linux/macOS use: ./start-ui.sh

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "KwaaiNet Node Dashboard" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Check Node.js
$nodeCmd = Get-Command node -ErrorAction SilentlyContinue
if (-not $nodeCmd) {
    Write-Host "[X] Node.js not found." -ForegroundColor Red
    Write-Host "Install Node.js 18+ from https://nodejs.org/" -ForegroundColor Yellow
    exit 1
}

$nodeVersion = node -v 2>$null
Write-Host "[OK] Node: $nodeVersion" -ForegroundColor Green
Write-Host ""

# Ensure dashboard exists
$DashboardDir = Join-Path $ScriptDir "systems\node-dashboard"
if (-not (Test-Path $DashboardDir)) {
    Write-Host "[X] Dashboard not found at systems\node-dashboard" -ForegroundColor Red
    exit 1
}

Write-Host "Installing dashboard dependencies (if needed)..." -ForegroundColor Cyan
Set-Location $DashboardDir
npm install
Write-Host ""

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "Starting KwaaiNet Dashboard" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Backend:  http://127.0.0.1:3456" -ForegroundColor White
Write-Host "  Frontend: http://127.0.0.1:5173" -ForegroundColor White
Write-Host ""
Write-Host "  Opening browser: http://127.0.0.1:5173" -ForegroundColor Yellow
Write-Host "  (Close the server window or press Ctrl+C there to stop)" -ForegroundColor Gray
Write-Host ""

Start-Process -FilePath "npm" -ArgumentList "run", "dev" -WorkingDirectory $DashboardDir
Start-Sleep -Seconds 5
Start-Process "http://127.0.0.1:5173"
