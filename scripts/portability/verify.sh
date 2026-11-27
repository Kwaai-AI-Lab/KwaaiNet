#!/usr/bin/env bash
# verify.sh — did the move to this machine actually work?
#
# A migration without an acceptance test is a hope. This is the acceptance
# test: it answers one question per check, and prints a verdict.
#
# It is deliberately READ-ONLY by default. It inspects; it does not fix.
# A verifier that silently repairs things teaches you nothing about whether
# the move worked.
#
# Usage:
#   scripts/portability/verify.sh                    # verify this machine
#   scripts/portability/verify.sh --repo PATH        # verify a given checkout
#   scripts/portability/verify.sh --expect-node      # require a running node
#   scripts/portability/verify.sh --expect-shard-api # require the hivv door
#
# Exit status: 0 if nothing failed, 1 otherwise. Warnings do not fail.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/portability/lib.sh
. "$SCRIPT_DIR/lib.sh"

REPO_PATH=""
EXPECT_NODE=0
EXPECT_SHARD_API=0
SHARD_API_PORT="${SHARD_API_PORT:-18090}"

usage() {
    cat <<'EOF'
usage: verify.sh [--repo PATH] [--expect-node] [--expect-shard-api]

  --repo PATH          checkout to verify (default: the repo this script is in)
  --expect-node        require a KwaaiNet node to be running
  --expect-shard-api   require the OpenAI-compatible shard API to answer
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --repo) shift; REPO_PATH="${1:-}" ;;
        --expect-node) EXPECT_NODE=1 ;;
        --expect-shard-api) EXPECT_SHARD_API=1 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
    shift
done

if [ -z "$REPO_PATH" ]; then
    REPO_PATH="$(p_repo_root "$SCRIPT_DIR")" || REPO_PATH="$(pwd)"
fi

# ── 1. the checkout ──────────────────────────────────────────────────────────
p_head "checkout"
if [ -d "$REPO_PATH" ]; then
    p_ok "checkout present: $(p_home_path "$REPO_PATH")"
else
    p_fail "no checkout at $(p_home_path "$REPO_PATH")"
    p_summary
    exit 1
fi

if git -C "$REPO_PATH" rev-parse --git-dir >/dev/null 2>&1; then
    p_ok "is a git repository"
    p_dim "branch $(git -C "$REPO_PATH" branch --show-current 2>/dev/null) @ $(git -C "$REPO_PATH" rev-parse --short HEAD 2>/dev/null)"
else
    p_fail "not a git repository"
fi

if git -C "$REPO_PATH" remote get-url origin >/dev/null 2>&1; then
    p_ok "origin remote: $(git -C "$REPO_PATH" remote get-url origin 2>/dev/null)"
else
    p_fail "no origin remote — cannot pull or push"
fi

# Uncommitted work is the classic thing a clone silently leaves behind.
_dirty="$(git -C "$REPO_PATH" status --porcelain 2>/dev/null | wc -l | tr -d ' ')"
if [ "$_dirty" -eq 0 ]; then
    p_ok "working tree clean"
else
    p_warn "$_dirty uncommitted change(s) — a fresh clone would not have them:"
    git -C "$REPO_PATH" status --porcelain 2>/dev/null | head -10 | while read -r line; do
        p_dim "$line"
    done
fi

# ── 2. the build ─────────────────────────────────────────────────────────────
p_head "build"
if command -v cargo >/dev/null 2>&1; then
    p_ok "cargo present: $(cargo --version 2>/dev/null)"
    if ! command -v rustc >/dev/null 2>&1; then
        p_fail "rustc not found although cargo is — run setup.sh on this machine"
    else
        _rustver="$(rustc --version 2>/dev/null | awk '{print $2}')"
        if [ -z "$_rustver" ]; then
            p_fail "could not determine the rustc version"
        elif printf '%s' "$_rustver" | grep -qE '^1\.(8[0-9]|[9-9][0-9])'; then
            p_ok "rustc $_rustver"
        else
            p_fail "rustc $_rustver is too old (need 1.80+ for edition 2024)"
        fi
    fi
else
    p_fail "cargo not found — run setup.sh on this machine"
fi

if [ -d "$REPO_PATH/core/patches" ]; then
    _missing=0
    for p in multistream-select libp2p-kad cudarc; do
        if [ ! -d "$REPO_PATH/core/patches/$p" ]; then
            p_warn "patched dependency not fetched: $p (run core/patches/fetch-patches.sh)"
            _missing=1
        fi
    done
    [ "$_missing" -eq 0 ] && p_ok "patched dependencies fetched"
fi

# The binary: prefer a release build, fall back to debug, fall back to PATH.
_binary=""
for cand in "$REPO_PATH/core/target/release/kwaainet" "$REPO_PATH/core/target/debug/kwaainet"; do
    if [ -x "$cand" ]; then
        _binary="$cand"
        break
    fi
done
if [ -z "$_binary" ] && command -v kwaainet >/dev/null 2>&1; then
    _binary="$(command -v kwaainet)"
fi
if [ -n "$_binary" ]; then
    p_ok "kwaainet binary: $(p_home_path "$_binary")"
    _ver="$("$_binary" --version 2>&1 | head -1)"
    if [ -n "$_ver" ]; then
        p_ok "binary runs: $_ver"
    else
        p_fail "binary does not run"
    fi
else
    p_fail "no kwaainet binary — the workspace has not been built on this machine"
fi

# ── 3. the node's own state ──────────────────────────────────────────────────
p_head "node state"
# The home must EXIST, not merely be listed. p_kwaainet_homes always prints
# candidate paths, so testing the list would always pass.
_home_found=0
for home in $(p_kwaainet_homes); do
    [ -n "$home" ] || continue
    if [ -d "$home" ]; then
        _home_found=1
        p_dim "home: $(p_home_path "$home")"
    fi
done
if [ "$_home_found" -eq 1 ]; then
    p_ok "at least one KWAAINET_HOME exists"
else
    p_fail "no KWAAINET_HOME exists — run 'kwaainet setup' to create one"
fi

# The identity is the node. Check the one that matters most.
_home="${KWAAINET_HOME:-${HOME}/.kwaainet}"
if [ -f "$_home/identity.key" ]; then
    p_ok "identity key present ($(p_secret_file "$_home/identity.key" "identity.key"))"
else
    p_fail "identity key absent in $(p_home_path "$_home") — this is a NEW node, not a moved one"
fi
if [ -f "$_home/config.yaml" ]; then
    p_ok "config.yaml present"
else
    p_fail "config.yaml absent in $(p_home_path "$_home")"
fi

# ── 4. services ─────────────────────────────────────────────────────────────
p_head "services"
if command -v systemctl >/dev/null 2>&1; then
    _u="$(systemctl --user list-unit-files --no-pager --no-legend 2>/dev/null |
          awk '{print $1}' | grep -E 'kwaainet|hivv' || true)"
    if [ -n "$_u" ]; then
        for unit in $_u; do
            _en="$(systemctl --user is-enabled "$unit" 2>/dev/null || echo unknown)"
            _ac="$(systemctl --user is-active "$unit" 2>/dev/null || echo unknown)"
            if [ "$_en" = "enabled" ] && [ "$_ac" = "active" ]; then
                p_ok "$unit: $_en, $_ac"
            else
                p_warn "$unit: $_en, $_ac"
            fi
        done
    else
        p_warn "no kwaainet/hivv systemd user units found"
    fi
else
    p_skip "systemd not available (not Linux) — services not checked"
fi

# ── 5. github ────────────────────────────────────────────────────────────────
p_head "github"
# The git checks pin the SSH key on purpose — see p_git_ssh_command. An
# unpinned check answers "can THIS box reach GitHub", which on a box with
# several keys is not the same question as "does the key we moved work".
GIT_SSH_COMMAND="$(p_git_ssh_command readonly)"
export GIT_SSH_COMMAND
if git -C "$REPO_PATH" ls-remote --exit-code origin HEAD >/dev/null 2>&1; then
    p_ok "can read from origin"
else
    p_fail "cannot read from origin"
fi
# --dry-run does the full authentication and permission check without
# writing a single ref. It is the only honest way to prove PUSH works
# without pushing.
if git -C "$REPO_PATH" push --dry-run origin HEAD:refs/heads/portability/probe >/dev/null 2>&1; then
    p_ok "can push to origin (dry run)"
else
    p_fail "cannot push to origin"
fi

# ── 6. the running node ─────────────────────────────────────────────────────
p_head "runtime"
if [ "$EXPECT_NODE" -eq 1 ]; then
    if p_port_open 31337 || p_port_open 8000; then
        p_ok "a node is listening"
    else
        p_fail "no node listening on the p2p port"
    fi
else
    if p_port_open 31337 || p_port_open 8000; then
        p_ok "a node is listening"
    else
        p_skip "no node listening (pass --expect-node to require it)"
    fi
fi

if [ "$EXPECT_SHARD_API" -eq 1 ]; then
    if p_port_open "$SHARD_API_PORT"; then
        p_ok "shard API listening on $SHARD_API_PORT"
        _models="$(curl -s --max-time 5 "http://127.0.0.1:${SHARD_API_PORT}/v1/models" 2>/dev/null | head -c 200)"
        if [ -n "$_models" ]; then
            p_ok "shard API answers /v1/models"
        else
            p_warn "shard API port is open but /v1/models did not answer"
        fi
    else
        p_fail "shard API not listening on $SHARD_API_PORT"
    fi
else
    if p_port_open "$SHARD_API_PORT"; then
        p_ok "shard API listening on $SHARD_API_PORT"
    else
        p_skip "shard API not listening (pass --expect-shard-api to require it)"
    fi
fi

p_summary
p_exit_code
