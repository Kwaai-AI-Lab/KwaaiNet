#!/usr/bin/env bash
# KwaaiNet contributor cleanup script.
#
# Removes build artifacts, leftover Go clone dirs, and stale installed
# binaries so contributors start from a known-clean state.
#
# Usage:
#   ./scripts/clean.sh              # interactive — confirm each step
#   ./scripts/clean.sh --all        # remove everything without prompting
#   ./scripts/clean.sh --dry-run    # show what would be removed, do nothing
#
# Flags can be combined:  ./scripts/clean.sh --all --dry-run

set -euo pipefail

# ── Argument parsing ──────────────────────────────────────────────────────────
ALL=false
DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --all)     ALL=true ;;
        --dry-run) DRY_RUN=true ;;
        -h|--help)
            sed -n '2,14p' "$0" | sed 's/^# \?//'
            exit 0
            ;;
        *) echo "Unknown flag: $arg  (use --all, --dry-run, or --help)" >&2; exit 1 ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# ── Helpers ───────────────────────────────────────────────────────────────────
green()  { printf '\033[0;32m%s\033[0m\n' "$*"; }
yellow() { printf '\033[0;33m%s\033[0m\n' "$*"; }
bold()   { printf '\033[1m%s\033[0m\n' "$*"; }

confirm() {
    # confirm <description>
    # Returns 0 (proceed) when --all is set or user answers y/Y.
    if $ALL; then return 0; fi
    printf '%s [y/N] ' "$1"
    read -r reply
    case "$reply" in y|Y) return 0 ;; *) return 1 ;; esac
}

remove() {
    # remove <path> [<path> ...]
    for p in "$@"; do
        if [ -e "$p" ] || [ -L "$p" ]; then
            if $DRY_RUN; then
                yellow "  [dry-run] would remove: $p"
            else
                rm -rf "$p"
                green "  removed: $p"
            fi
        fi
    done
}

# ── 1. Rust build artifacts (core/target/) ────────────────────────────────────
bold "==> Rust build artifacts  (core/target/)"
TARGET_DIR="$REPO_ROOT/core/target"
if [ -d "$TARGET_DIR" ]; then
    SIZE=$(du -sh "$TARGET_DIR" 2>/dev/null | cut -f1)
    if confirm "    Remove $TARGET_DIR  ($SIZE)?"; then
        remove "$TARGET_DIR"
    else
        yellow "    skipped"
    fi
else
    echo "    nothing to remove"
fi

# ── 2. Stale Go clone dirs in /tmp ───────────────────────────────────────────
bold "==> Stale go-libp2p-daemon clone dirs  (/tmp/go-libp2p-daemon-*)"
# shellcheck disable=SC2086
GO_CLONES=$(ls -d /tmp/go-libp2p-daemon-* 2>/dev/null || true)
if [ -n "$GO_CLONES" ]; then
    echo "$GO_CLONES"
    if confirm "    Remove these dirs?"; then
        # Use eval-free loop to handle each path
        while IFS= read -r dir; do
            remove "$dir"
        done <<< "$GO_CLONES"
    else
        yellow "    skipped"
    fi
else
    echo "    nothing to remove"
fi

# ── 3. Manually installed binaries ───────────────────────────────────────────
bold "==> Manually installed binaries  (/usr/local/bin)"
MANUAL_BINS=()
for bin in /usr/local/bin/kwaainet /usr/local/bin/p2pd; do
    [ -f "$bin" ] && MANUAL_BINS+=("$bin")
done
if [ ${#MANUAL_BINS[@]} -gt 0 ]; then
    printf '    %s\n' "${MANUAL_BINS[@]}"
    if confirm "    Remove these (may require sudo)?"; then
        sudo rm -f "${MANUAL_BINS[@]}" && green "  removed"
    else
        yellow "    skipped"
    fi
else
    echo "    nothing to remove"
fi

# ── 4. cargo-installed binary ─────────────────────────────────────────────────
bold "==> cargo-installed binary  (~/.cargo/bin/kwaainet)"
CARGO_BIN="${HOME}/.cargo/bin/kwaainet"
if [ -f "$CARGO_BIN" ]; then
    echo "    $CARGO_BIN"
    if confirm "    Remove?"; then
        remove "$CARGO_BIN"
    else
        yellow "    skipped"
    fi
else
    echo "    nothing to remove"
fi

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
if $DRY_RUN; then
    yellow "Dry run complete — nothing was changed."
else
    green "Clean complete."
    echo ""
    echo "To rebuild from source:"
    echo "  cd core && cargo build --release -p kwaainet"
fi
