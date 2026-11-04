#!/bin/sh
# Run by cmake in MLX's fetched checkout; a no-op once the patch is in.
p="$(dirname "$0")/kwaai-mlx-metallib.patch"
git apply --reverse --check "$p" 2>/dev/null || git apply "$p"
