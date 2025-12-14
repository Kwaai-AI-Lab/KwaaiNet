# Cross-platform setup script for KwaaiNet (Windows)
# Run with: powershell -ExecutionPolicy Bypass -File setup.ps1

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "KwaaiNet Development Environment Setup" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Check if running as Administrator
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "[!] Warning: Not running as Administrator" -ForegroundColor Yellow
    Write-Host "Some installations may require elevation." -ForegroundColor Yellow
    Write-Host ""
}

# Check for winget (Windows Package Manager)
$hasWinget = Get-Command winget -ErrorAction SilentlyContinue

if (-not $hasWinget) {
    Write-Host "[X] winget not found!" -ForegroundColor Red
    Write-Host "Please install App Installer from Microsoft Store or install tools manually:" -ForegroundColor Yellow
    Write-Host "  - Git: https://git-scm.com/download/win" -ForegroundColor Yellow
    Write-Host "  - Rust: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "  - Go: https://golang.org/dl/" -ForegroundColor Yellow
    exit 1
}

Write-Host "Checking prerequisites..." -ForegroundColor Cyan
Write-Host ""

# 1. Git
try {
    $gitVersion = git --version 2>$null
    Write-Host "[OK] Git found: $gitVersion" -ForegroundColor Green
} catch {
    Write-Host "[=>] Installing Git..." -ForegroundColor Yellow
    winget install --id Git.Git -e --source winget
}

# 2. Rust toolchain
try {
    $cargoVersion = cargo --version 2>$null
    Write-Host "[OK] Rust found: $cargoVersion" -ForegroundColor Green

    # Check Rust version - need 1.80+
    $version = ($cargoVersion -split ' ')[1]
    $parts = $version -split '\.'
    $major = [int]$parts[0]
    $minor = [int]$parts[1]

    if ($major -lt 1 -or ($major -eq 1 -and $minor -lt 80)) {
        Write-Host "[!] Rust version $version is too old (need 1.80+)" -ForegroundColor Yellow
        Write-Host "[=>] Updating Rust to latest stable..." -ForegroundColor Yellow
        rustup update stable
        rustup default stable
        Write-Host "[OK] Rust updated to: $(cargo --version)" -ForegroundColor Green
    }
} catch {
    Write-Host "[=>] Installing Rust toolchain..." -ForegroundColor Yellow
    Write-Host "Downloading rustup-init.exe..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y
    Remove-Item "$env:TEMP\rustup-init.exe"

    # Reload PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

    Write-Host "[OK] Rust installed: $(cargo --version)" -ForegroundColor Green
}

# 3. Go toolchain
try {
    $goVersion = go version 2>$null
    Write-Host "[OK] Go found: $goVersion" -ForegroundColor Green

    # Check Go version - need 1.20+
    if ($goVersion -match 'go(\d+)\.(\d+)') {
        $goMajor = [int]$matches[1]
        $goMinor = [int]$matches[2]

        if ($goMajor -lt 1 -or ($goMajor -eq 1 -and $goMinor -lt 20)) {
            Write-Host "[!] Go version is too old (need 1.20+)" -ForegroundColor Yellow
            Write-Host "[=>] Installing Go 1.21..." -ForegroundColor Yellow
            winget install --id GoLang.Go -e --source winget
        }
    }
} catch {
    Write-Host "[=>] Installing Go 1.21..." -ForegroundColor Yellow
    winget install --id GoLang.Go -e --source winget

    # Reload PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

Write-Host ""
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "[OK] Setup complete!" -ForegroundColor Green
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Build the project: cargo build" -ForegroundColor White
Write-Host "  2. Run tests: cargo test" -ForegroundColor White
Write-Host "  3. Run example: cargo run --example petals_visible" -ForegroundColor White
Write-Host ""
Write-Host "Note: You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
Write-Host ""
