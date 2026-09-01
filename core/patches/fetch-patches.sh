#!/usr/bin/env bash
# Fetch and patch every build-time patched dependency. Each per-crate script
# is checksum-pinned and idempotent; this is the one entry point callers use
# so adding a patched crate never touches setup.sh or the workflows again.
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
# Suppresses the per-crate scripts' "this is only part of it" note.
export KWAAI_FETCH_PATCHES=1
bash "$DIR/fetch-multistream-select.sh"
bash "$DIR/fetch-libp2p-kad.sh"
