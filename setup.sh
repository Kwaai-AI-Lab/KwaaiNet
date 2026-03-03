#!/bin/bash
# Cross-platform setup script for KwaaiNet
# Works on Linux and macOS

set -e

echo "=========================================="
echo "KwaaiNet Development Environment Setup"
echo "=========================================="
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    PKG_MANAGER="apt"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    PKG_MANAGER="brew"
else
    echo "❌ Unsupported OS: $OSTYPE"
    echo "This script supports Linux and macOS only."
    echo "For Windows, use setup.ps1"
    exit 1
fi

echo "Detected OS: $OS"
echo ""

# Check and install prerequisites
echo "Checking prerequisites..."
echo ""

# 1. Git
if ! command -v git &> /dev/null; then
    echo "📦 Installing Git..."
    if [ "$OS" = "linux" ]; then
        sudo apt update && sudo apt install -y git
    else
        brew install git
    fi
else
    echo "✅ Git found: $(git --version)"
fi

# 2. curl (for downloads)
if ! command -v curl &> /dev/null; then
    echo "📦 Installing curl..."
    if [ "$OS" = "linux" ]; then
        sudo apt install -y curl
    else
        echo "✅ curl is pre-installed on macOS"
    fi
else
    echo "✅ curl found"
fi

# 3. unzip (for protoc extraction)
if ! command -v unzip &> /dev/null; then
    echo "📦 Installing unzip..."
    if [ "$OS" = "linux" ]; then
        sudo apt install -y unzip
    else
        echo "✅ unzip is pre-installed on macOS"
    fi
else
    echo "✅ unzip found"
fi

# 4. Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed: $(cargo --version)"
else
    echo "✅ Rust found: $(cargo --version)"

    # Check Rust version - need 1.80+ for edition2024 support
    RUST_VERSION=$(cargo --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    MAJOR=$(echo $RUST_VERSION | cut -d. -f1)
    MINOR=$(echo $RUST_VERSION | cut -d. -f2)

    if [ "$MAJOR" -lt 1 ] || ([ "$MAJOR" -eq 1 ] && [ "$MINOR" -lt 80 ]); then
        echo "⚠️  Rust version $RUST_VERSION is too old (need 1.80+)"
        echo "📦 Updating Rust to latest stable..."
        rustup update stable
        rustup default stable
        source "$HOME/.cargo/env"
        echo "✅ Rust updated to: $(cargo --version)"
    fi
fi

# 5. Go toolchain
GO_ACTION=""
if ! command -v go &> /dev/null; then
    GO_ACTION="install"
else
    echo "✅ Go found: $(go version)"

    # Check Go version - need 1.20+
    GO_VERSION=$(go version | grep -oE 'go[0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+')
    GO_MAJOR=$(echo $GO_VERSION | cut -d. -f1)
    GO_MINOR=$(echo $GO_VERSION | cut -d. -f2)

    if [ "$GO_MAJOR" -lt 1 ] || ([ "$GO_MAJOR" -eq 1 ] && [ "$GO_MINOR" -lt 20 ]); then
        echo "⚠️  Go version $GO_VERSION is too old (need 1.20+)"
        GO_ACTION="upgrade"
    fi
fi

if [ -n "$GO_ACTION" ]; then
    echo "📦 Installing Go 1.21..."

    if [ "$OS" = "linux" ]; then
        if [ "$(uname -m)" = "aarch64" ] || [ "$(uname -m)" = "arm64" ]; then
            wget https://go.dev/dl/go1.21.13.linux-arm64.tar.gz
            sudo rm -rf /usr/local/go
            sudo tar -C /usr/local -xzf go1.21.13.linux-arm64.tar.gz
            rm go1.21.13.linux-arm64.tar.gz
        else
            wget https://go.dev/dl/go1.21.13.linux-amd64.tar.gz
            sudo rm -rf /usr/local/go
            sudo tar -C /usr/local -xzf go1.21.13.linux-amd64.tar.gz
            rm go1.21.13.linux-amd64.tar.gz
        fi

        if [ "$GO_ACTION" = "install" ]; then
            export PATH=$PATH:/usr/local/go/bin
            echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
        fi
    elif [ "$OS" = "macos" ]; then
        if [ "$(uname -m)" = "arm64" ]; then
            wget https://go.dev/dl/go1.21.13.darwin-arm64.tar.gz
            sudo tar -C /usr/local -xzf go1.21.13.darwin-arm64.tar.gz
            rm go1.21.13.darwin-arm64.tar.gz
        else
            wget https://go.dev/dl/go1.21.13.darwin-amd64.tar.gz
            sudo tar -C /usr/local -xzf go1.21.13.darwin-amd64.tar.gz
            rm go1.21.13.darwin-amd64.tar.gz
        fi

        if [ "$GO_ACTION" = "install" ]; then
            export PATH=$PATH:/usr/local/go/bin
            echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.zshrc
            echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.bashrc
        fi
    else
        echo "❌ Unsupported platform for Go installation: $OS"
        exit 1
    fi

    if [ "$GO_ACTION" = "install" ]; then
        echo "✅ Go installed: $(go version)"
    else
        echo "✅ Go upgraded to: $(go version)"
    fi
fi

echo ""
echo "=========================================="
echo "✅ Setup complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "  1. Build the project: cargo build"
echo "  2. Run tests: cargo test"
echo "  3. Run example: cargo run --example petals_visible"
echo ""
