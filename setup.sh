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

    if [ "$GO_MAJOR" -lt 1 ] || ([ "$GO_MAJOR" -eq 1 ] && [ "$GO_MINOR" -lt 22 ]); then
        echo "⚠️  Go version $GO_VERSION is too old (need 1.22+)"
        GO_ACTION="upgrade"
    fi
fi

if [ -n "$GO_ACTION" ]; then
    GO_VERSION="1.22.12"
    echo "📦 Installing Go ${GO_VERSION}..."

    case "$OSTYPE" in
        linux-gnu*)
            GO_OS="linux"
            GO_ARCH="amd64"
            [ "$(uname -m)" = "aarch64" ] && GO_ARCH="arm64"
            ;;
        darwin*)
            GO_OS="darwin"
            GO_ARCH="amd64"
            [ "$(uname -m)" = "arm64" ] && GO_ARCH="arm64"
            ;;
        *)
            echo "❌ Unsupported platform for Go installation: $OSTYPE"
            exit 1
            ;;
    esac

    wget "https://go.dev/dl/go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz"
    sudo rm -rf /usr/local/go
    sudo tar -C /usr/local -xzf "go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz"
    rm "go${GO_VERSION}.${GO_OS}-${GO_ARCH}.tar.gz"

    export PATH=/usr/local/go/bin:$PATH
    RC_UPDATED=false
    if [[ "$SHELL" == */zsh ]]; then
        grep -qxF 'export PATH=/usr/local/go/bin:$PATH' ~/.zshrc  || { echo 'export PATH=/usr/local/go/bin:$PATH' >> ~/.zshrc;  RC_UPDATED=true; }
    else
        grep -qxF 'export PATH=/usr/local/go/bin:$PATH' ~/.bashrc || { echo 'export PATH=/usr/local/go/bin:$PATH' >> ~/.bashrc; RC_UPDATED=true; }
    fi

    if [ "$GO_ACTION" = "install" ]; then
        echo "✅ Go installed: $(go version)"
    else
        echo "✅ Go upgraded to: $(go version)"
    fi

    if [ "$RC_UPDATED" = true ]; then
        if [[ "$SHELL" == */zsh ]]; then
            echo "⚠️  Run 'source ~/.zshrc' to update your PATH for Go."
        else
            echo "⚠️  Run 'source ~/.bashrc' to update your PATH for Go."
        fi
    fi
fi

# 6. NVIDIA CUDA toolkit (Linux only, when GPU is detected)
CARGO_FEATURES=""
if [ "$OS" = "linux" ] && command -v nvidia-smi &> /dev/null; then
    GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)
    echo "✅ NVIDIA GPU detected: $GPU_NAME"

    if command -v nvcc &> /dev/null; then
        echo "✅ CUDA toolkit found: $(nvcc --version | grep 'release' | sed 's/.*release //' | sed 's/,.*//')"
        CARGO_FEATURES="--features cuda"
    else
        echo "⚠️  CUDA toolkit not found — GPU detected but nvcc is missing."

        # Try common CUDA install paths before prompting
        for cuda_dir in /usr/local/cuda /usr/local/cuda-*; do
            if [ -x "$cuda_dir/bin/nvcc" ]; then
                echo "✅ Found nvcc at $cuda_dir/bin/nvcc"
                export PATH="$cuda_dir/bin:$PATH"
                RC_LINE="export PATH=$cuda_dir/bin:\$PATH"
                if [[ "$SHELL" == */zsh ]]; then
                    grep -qxF "$RC_LINE" ~/.zshrc 2>/dev/null || echo "$RC_LINE" >> ~/.zshrc
                else
                    grep -qxF "$RC_LINE" ~/.bashrc 2>/dev/null || echo "$RC_LINE" >> ~/.bashrc
                fi
                echo "✅ Added $cuda_dir/bin to PATH"
                CARGO_FEATURES="--features cuda"
                break
            fi
        done

        if [ -z "$CARGO_FEATURES" ]; then
            echo ""
            echo "  To enable GPU acceleration, install the CUDA toolkit:"
            echo "    # RHEL/Rocky/Alma:"
            echo "    sudo dnf config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/x86_64/cuda-rhel8.repo"
            echo "    sudo dnf install -y cuda-toolkit-12-6"
            echo ""
            echo "    # Ubuntu/Debian:"
            echo "    sudo apt install -y nvidia-cuda-toolkit"
            echo ""
            echo "  Then re-run this script."
        fi
    fi
else
    echo "ℹ️  No NVIDIA GPU detected — building for CPU only."
fi

echo ""
echo "=========================================="
echo "Building KwaaiNet..."
echo "=========================================="
echo ""

# Resolve the workspace directory relative to this script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CORE_DIR="$SCRIPT_DIR/core"

if [ ! -f "$CORE_DIR/Cargo.toml" ]; then
    echo "❌ Cannot find core/Cargo.toml — run this script from the repo root."
    exit 1
fi

if [ -n "$CARGO_FEATURES" ]; then
    echo "🔨 Building with CUDA GPU acceleration..."
    echo "   cargo build --release -p kwaainet $CARGO_FEATURES"
else
    echo "🔨 Building for CPU..."
    echo "   cargo build --release -p kwaainet"
fi
echo ""

cargo build --release -p kwaainet --manifest-path "$CORE_DIR/Cargo.toml" $CARGO_FEATURES

echo ""
echo "📦 Installing kwaainet..."
cargo install --path "$CORE_DIR/crates/kwaai-cli" $CARGO_FEATURES --force

echo ""
echo "=========================================="
echo "✅ Setup complete!"
echo "=========================================="
echo ""
if [ -n "$CARGO_FEATURES" ]; then
    echo "  Installed: kwaainet (with CUDA GPU support)"
else
    echo "  Installed: kwaainet (CPU only)"
fi
echo ""
echo "  Run 'kwaainet setup' to create config dirs and identity."
echo "  Run 'kwaainet setup --get-deps' to download p2pd if needed."
echo "  Run 'kwaainet benchmark' to measure throughput."
echo ""
