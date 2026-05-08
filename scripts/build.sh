#!/bin/bash
# Auto-detect GPU capabilities and build kwaainet with the right features.
# Usage:
#   ./scripts/build.sh              # release build, auto-detect GPU
#   ./scripts/build.sh --debug      # debug build, auto-detect GPU
#   ./scripts/build.sh --no-gpu     # force CPU-only build
#   ./scripts/build.sh --install    # build + cargo install

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CORE_DIR="$SCRIPT_DIR/../core"

RELEASE="--release"
INSTALL=false
FORCE_NO_GPU=false

for arg in "$@"; do
    case "$arg" in
        --debug)   RELEASE="" ;;
        --install) INSTALL=true ;;
        --no-gpu)  FORCE_NO_GPU=true ;;
    esac
done

# ── GPU auto-detection ────────────────────────────────────────────────
detect_gpu_features() {
    if [ "$FORCE_NO_GPU" = true ]; then
        echo "ℹ️  GPU detection skipped (--no-gpu)"
        return
    fi

    # CUDA — works on Linux and Windows (nvidia-smi is on PATH for both)
    if command -v nvidia-smi &> /dev/null; then
        GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)
        echo "✅ NVIDIA GPU detected: $GPU_NAME"

        # Find nvcc
        if command -v nvcc &> /dev/null; then
            NVCC_VER=$(nvcc --version 2>/dev/null | grep 'release' | sed 's/.*release //' | sed 's/,.*//')
            echo "✅ CUDA toolkit found: $NVCC_VER"
            CARGO_FEATURES="--features cuda,flash-attn"
            return
        fi

        # Search common CUDA install paths (Linux)
        for cuda_dir in /usr/local/cuda /usr/local/cuda-*; do
            if [ -x "$cuda_dir/bin/nvcc" ]; then
                echo "✅ Found nvcc at $cuda_dir/bin/nvcc"
                export PATH="$cuda_dir/bin:$PATH"
                CARGO_FEATURES="--features cuda,flash-attn"
                return
            fi
        done

        # Search common CUDA install paths (Windows via MSYS/Git Bash)
        for cuda_dir in /c/Program\ Files/NVIDIA\ GPU\ Computing\ Toolkit/CUDA/v*; do
            if [ -x "$cuda_dir/bin/nvcc.exe" ]; then
                echo "✅ Found nvcc at $cuda_dir/bin/nvcc.exe"
                export PATH="$cuda_dir/bin:$PATH"
                CARGO_FEATURES="--features cuda,flash-attn"
                return
            fi
        done

        echo "⚠️  NVIDIA GPU found but CUDA toolkit (nvcc) is not installed."
        echo "   Building for CPU only. Install the CUDA toolkit to enable GPU support."
    else
        echo "ℹ️  No NVIDIA GPU detected — building for CPU only."
    fi
}

CARGO_FEATURES=""
detect_gpu_features

# ── Build ─────────────────────────────────────────────────────────────
echo ""
echo "Building kwaainet $RELEASE $CARGO_FEATURES"
echo ""

cd "$CORE_DIR"
cargo build $RELEASE -p kwaainet $CARGO_FEATURES

if [ "$INSTALL" = true ]; then
    echo ""
    echo "Installing kwaainet…"
    cargo install --path crates/kwaai-cli $CARGO_FEATURES --force
fi

echo ""
echo "✅ Build complete."
if [ -n "$CARGO_FEATURES" ]; then
    echo "   GPU support: CUDA + Flash Attention enabled"
else
    echo "   GPU support: CPU only"
fi
