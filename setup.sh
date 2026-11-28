#!/bin/bash
# Cross-platform setup script for KwaaiNet
# Works on Linux and macOS
#
#   ./setup.sh            install what is missing, build, install the binary
#   ./setup.sh --check    report what is missing and what WOULD be installed
#                         without changing anything on this machine
#   ./setup.sh --help     this text
#
# --check exists because "it built fine on my machine" is not a claim you
# can test. It is the hook CI uses to prove that a FRESH machine can
# bootstrap: the job runs --check on a clean runner, on every supported
# platform, and a red build means the setup path itself has rotted.

set -e

CHECK=0

for arg in "$@"; do
    case "$arg" in
        --check|--dry-run) CHECK=1 ;;
        -h|--help)
            cat <<'USAGE'
usage: setup.sh [--check|--dry-run] [--help]

  (no flag)   install what is missing, build, and install the kwaainet binary
  --check     report what is missing and what WOULD be installed, without
              changing anything on this machine
  --help      this text

--check is the hook CI uses to prove that a FRESH machine can bootstrap.
USAGE
            exit 0
            ;;
        *)
            echo "Unknown argument: $arg" >&2
            echo "Usage: $0 [--check|--dry-run] [--help]" >&2
            exit 2
            ;;
    esac
done

# run_or_report <description> <command...>
#
# In --check mode nothing is executed; the command is described instead.
# In normal mode the command runs and its failure still fails the script.
run_or_report() {
    _desc="$1"
    shift
    if [ "$CHECK" -eq 1 ]; then
        echo "   would run ($_desc): $*"
        return 0
    fi
    "$@"
}

echo "=========================================="
if [ "$CHECK" -eq 1 ]; then
    echo "KwaaiNet Development Environment CHECK"
    echo "(no changes will be made to this machine)"
else
    echo "KwaaiNet Development Environment Setup"
fi
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

# 0. Patched dependencies — cargo cannot parse the workspace without them.
#    See core/patches/README.md.
if [ "$CHECK" -eq 1 ]; then
    _missing_patches=""
    for _p in multistream-select libp2p-kad cudarc; do
        if [ ! -d "$(dirname "$0")/core/patches/$_p" ]; then
            _missing_patches="$_missing_patches $_p"
        fi
    done
    if [ -n "$_missing_patches" ]; then
        echo "📦 Patched dependencies NOT fetched:$_missing_patches"
        echo "   would run: bash core/patches/fetch-patches.sh"
    else
        echo "✅ Patched dependencies fetched"
    fi
else
    bash "$(dirname "$0")/core/patches/fetch-patches.sh"
fi

# 1. Git
if ! command -v git &> /dev/null; then
    echo "📦 Installing Git..."
    if [ "$OS" = "linux" ]; then
        run_or_report "install git" sudo apt update && run_or_report "install git" sudo apt install -y git
    else
        run_or_report "install git" brew install git
    fi
else
    echo "✅ Git found: $(git --version)"
fi

# 2. curl (for downloads)
if ! command -v curl &> /dev/null; then
    echo "📦 Installing curl..."
    if [ "$OS" = "linux" ]; then
        run_or_report "install curl" sudo apt install -y curl
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
        run_or_report "install unzip" sudo apt install -y unzip
    else
        echo "✅ unzip is pre-installed on macOS"
    fi
else
    echo "✅ unzip found"
fi

# 4. protoc (Protocol Buffers compiler)
# Used at build time by both kwaai-p2p-daemon and kwaai-rpc. Each crate's
# build.rs has a download-on-miss fallback, but a system protoc is faster
# (no per-clean-build download) and works in offline / sandboxed
# environments where the fallback can't reach github.com.
if ! command -v protoc &> /dev/null; then
    echo "📦 Installing protoc..."
    if [ "$OS" = "linux" ]; then
        run_or_report "install protoc" sudo apt install -y protobuf-compiler
    else
        run_or_report "install protoc" brew install protobuf
    fi
    echo "✅ protoc installed: $(protoc --version 2>/dev/null || echo '(not installed in --check mode)')"
else
    echo "✅ protoc found: $(protoc --version)"
fi

# 5. Rust toolchain
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust toolchain..."
    if [ "$CHECK" -eq 1 ]; then
        echo "   would run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
        echo "   would run: rustup update stable && rustup default stable"
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo "✅ Rust installed: $(cargo --version)"
    fi
else
    echo "✅ Rust found: $(cargo --version)"

    # Check Rust version - need 1.80+ for edition2024 support
    RUST_VERSION=$(cargo --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
    MAJOR=$(echo $RUST_VERSION | cut -d. -f1)
    MINOR=$(echo $RUST_VERSION | cut -d. -f2)

    if [ "$MAJOR" -lt 1 ] || ([ "$MAJOR" -eq 1 ] && [ "$MINOR" -lt 80 ]); then
        echo "⚠️  Rust version $RUST_VERSION is too old (need 1.80+)"
        echo "📦 Updating Rust to latest stable..."
        if [ "$CHECK" -eq 1 ]; then
            echo "   would run: rustup update stable && rustup default stable"
        else
            rustup update stable
            rustup default stable
            source "$HOME/.cargo/env"
            echo "✅ Rust updated to: $(cargo --version)"
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
if [ "$CHECK" -eq 1 ]; then
    echo "Checking the build plan..."
else
    echo "Building KwaaiNet..."
fi
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

if [ "$CHECK" -eq 1 ]; then
    echo "   would run: cargo build --release -p kwaainet --manifest-path $CORE_DIR/Cargo.toml $CARGO_FEATURES"
    echo "   would run: cargo install --path $CORE_DIR/crates/kwaai-cli $CARGO_FEATURES --force"
    echo ""
    echo "=========================================="
    echo "✅ CHECK complete — nothing was installed."
    echo "=========================================="
    echo ""
    echo "  Run './setup.sh' (no --check) to perform the setup."
    echo ""
    exit 0
fi

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
echo "  Run 'kwaainet benchmark' to measure throughput."
echo ""
