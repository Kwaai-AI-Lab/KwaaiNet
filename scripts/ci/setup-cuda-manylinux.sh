#!/usr/bin/env bash
# Toolchain for the Linux CUDA build inside quay.io/pypa/manylinux_2_28_x86_64.
#
# Shared by release.yml (build-cuda-artifacts) and cuda-kernel-cache.yml, and
# it must stay shared: candle-flash-attn's kernel cache (cudaforge) is only
# reused when the nvcc arguments, CUDA version and absolute paths match the
# run that saved it. Two copies of this setup would drift and silently turn
# every release back into a ~55-minute kernel compile.
#
# Writes to $GITHUB_ENV / $GITHUB_PATH, so later steps see the toolchain and:
#   CUDA_COMPUTE_CAP             GPU arch the kernels are built for
#   CANDLE_FLASH_ATTN_BUILD_DIR  where candle-flash-attn keeps its objects
#   FLASH_ATTN_KERNELS           artifact name for that directory
set -euo pipefail

CUDA_VERSION=12.6
CUDA_COMPUTE_CAP="${CUDA_COMPUTE_CAP:-80}"
SCCACHE_VERSION=v0.8.1
# Fixed and absolute: cudaforge records each object's absolute path and
# recompiles any kernel whose recorded path differs.
FLASH_ATTN_BUILD_DIR=/tmp/flash-attn-build

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
echo "$HOME/.cargo/bin" >> "$GITHUB_PATH"

# GCC 12 — manylinux ships GCC 14 but CUDA 12.6 nvcc only supports up to GCC 13
dnf install -y -q gcc-toolset-12-gcc gcc-toolset-12-gcc-c++
echo "NVCC_CCBIN=/opt/rh/gcc-toolset-12/root/usr/bin/gcc" >> "$GITHUB_ENV"
echo "CC=/opt/rh/gcc-toolset-12/root/usr/bin/gcc" >> "$GITHUB_ENV"
echo "CXX=/opt/rh/gcc-toolset-12/root/usr/bin/g++" >> "$GITHUB_ENV"

# CUDA toolkit (RHEL 8 repo)
dnf install -y -q 'dnf-command(config-manager)'
dnf config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/rhel8/x86_64/cuda-rhel8.repo
dnf install -y -q "cuda-toolkit-${CUDA_VERSION/./-}" openssl-devel perl-IPC-Cmd
echo "/usr/local/cuda-${CUDA_VERSION}/bin" >> "$GITHUB_PATH"
echo "CUDA_COMPUTE_CAP=$CUDA_COMPUTE_CAP" >> "$GITHUB_ENV"

# gh CLI — fetches the kernel artifact (scripts/ci/fetch-flash-attn-kernels.sh)
# (manylinux container doesn't include gh; install from the official RPM repo)
curl -fsSL https://cli.github.com/packages/rpm/gh-cli.repo -o /etc/yum.repos.d/gh-cli.repo
dnf install -y -q gh

# sccache for faster incremental Rust compilation
curl -sSfL "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/sccache-${SCCACHE_VERSION}-x86_64-unknown-linux-musl.tar.gz" \
  | tar -xzf - --strip-components=1 -C /usr/local/bin "sccache-${SCCACHE_VERSION}-x86_64-unknown-linux-musl/sccache"
chmod +x /usr/local/bin/sccache
echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
# GHA cache service is not available inside containers — use local disk cache
echo "SCCACHE_DIR=/tmp/sccache" >> "$GITHUB_ENV"

# candle-flash-attn kernel cache. The build script compiles into
# CANDLE_FLASH_ATTN_BUILD_DIR instead of OUT_DIR when it is set (and the
# directory exists), and cudaforge skips every kernel whose content, GPU arch
# and nvcc args match its .cudaforge_cache.json. The nvcc args include
# -I$HOME/.cudaforge/..., so HOME must match too (/github/home in containers).
mkdir -p "$FLASH_ATTN_BUILD_DIR"
echo "CANDLE_FLASH_ATTN_BUILD_DIR=$FLASH_ATTN_BUILD_DIR" >> "$GITHUB_ENV"
flash_attn_version=$(awk '/^name = "candle-flash-attn"$/ { getline; gsub(/version = |"/, ""); print }' core/Cargo.lock)
if [ -z "$flash_attn_version" ] || [ "$(printf '%s\n' "$flash_attn_version" | wc -l)" -ne 1 ]; then
  echo "::error::expected exactly one candle-flash-attn in core/Cargo.lock, got: ${flash_attn_version:-none}" >&2
  exit 1
fi
# The object file names come from std's DefaultHasher, which isn't
# guaranteed stable across Rust releases, so the pinned toolchain is part of
# the name.
rust_channel=$(sed -n 's/^channel = "\(.*\)"$/\1/p' rust-toolchain.toml)
if [ -z "$rust_channel" ]; then
  echo "::error::no channel in rust-toolchain.toml" >&2
  exit 1
fi
# Bump v1 to discard every saved kernel artifact: cudaforge doesn't see the
# nvcc version or included headers, so a change there needs a fresh build.
echo "FLASH_ATTN_KERNELS=flash-attn-kernels-v1-${flash_attn_version}-sm${CUDA_COMPUTE_CAP}-cuda${CUDA_VERSION}-gcc12-rust${rust_channel}-manylinux_2_28" >> "$GITHUB_ENV"
