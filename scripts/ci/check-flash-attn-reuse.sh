#!/usr/bin/env bash
# After a build, report what cudaforge did with candle-flash-attn's kernels,
# and fail if they were restored (FLASH_ATTN_KERNELS_RESTORED=true) but still
# recompiled — the environment no longer matches the run that built them.
#
#   check-flash-attn-reuse.sh <cargo target dir>
#
# Reads the build script's output file: cargo hides cargo:warning lines from
# crates.io dependencies, so they never reach the build log.
set -euo pipefail

target_dir=$1
shopt -s nullglob
outputs=("$target_dir"/release/build/candle-flash-attn-*/output)
if [ ${#outputs[@]} -eq 0 ]; then
  echo "::error::No candle-flash-attn build-script output under $target_dir/release/build"
  exit 1
fi
# The newest one, if a stale build dir is lying around.
output=$(ls -t "${outputs[@]}" | head -1)
report=$(sed -n 's/^cargo:warning=\(All kernels up-to-date.*\|Compiling [0-9]* of [0-9]* kernels\)$/\1/p' "$output")
echo "cudaforge: ${report:-no report in $output}"
echo "- flash-attn kernels: restored=${FLASH_ATTN_KERNELS_RESTORED:-false}, cudaforge: ${report:-no report}" >> "$GITHUB_STEP_SUMMARY"

if [ "${FLASH_ATTN_KERNELS_RESTORED:-false}" = true ] && [[ "$report" != "All kernels up-to-date"* ]]; then
  echo "::error::Kernels were restored but cudaforge recompiled ($report): the nvcc args, paths or HOME differ from the run that built them."
  exit 1
fi
