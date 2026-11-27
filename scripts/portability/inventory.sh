#!/usr/bin/env bash
# inventory.sh — capture the machine-local state that does NOT live in git.
#
# A git clone gives you the code. It does not give you:
#   * the node's identity key (a node with a new identity is a NEW node —
#     new peer id, new reputation, orphaned from every peer that pinned it)
#   * the node's config.yaml (tuned ports, announce address, block ranges)
#   * the systemd units that make it start on boot and survive a crash
#   * the machine's GitHub credentials (clone AND push)
#   * the hivv.org integration that treats this box as its inference door
#
# This script reports all of that, WITHOUT DISCLOSING SECRETS. Secret values
# are recorded as "set, sha256:<12 hex>" so two inventories can be diffed to
# prove the same key moved — and that diff is safe to paste into chat.
#
# Usage:
#   scripts/portability/inventory.sh                 # human-readable report
#   scripts/portability/inventory.sh --json          # machine-readable
#   scripts/portability/inventory.sh --out FILE      # write to a file
#
# Read-only. It never writes to the system it describes.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/portability/lib.sh
. "$SCRIPT_DIR/lib.sh"

JSON_MODE=0
OUT_FILE=""
REPO_PATH=""

usage() {
    cat <<'EOF'
usage: inventory.sh [--json] [--out FILE] [--repo PATH]

  --json        emit JSON (flat, dotted keys) instead of a text report
  --out FILE    write to FILE instead of stdout
  --repo PATH   repository to inspect (default: the repo this script lives in)
  -h, --help    this text
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --json) JSON_MODE=1 ;;
        --out) shift; OUT_FILE="${1:-}" ;;
        --repo) shift; REPO_PATH="${1:-}" ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
    shift
done

if [ -z "$REPO_PATH" ]; then
    REPO_PATH="$(p_repo_root "$SCRIPT_DIR")" || REPO_PATH="$(pwd)"
fi

# Text mode writes to a file from the very first line, not from the last.
if [ "$JSON_MODE" -eq 0 ] && [ -n "$OUT_FILE" ]; then
    exec > "$OUT_FILE"
fi

TMP_OUT=""
if [ "$JSON_MODE" -eq 1 ]; then
    TMP_OUT="$(mktemp)"
fi

emit() {
    # emit <dotted.key> <value>
    _k="$1"
    _v="${2:-}"
    if [ "$JSON_MODE" -eq 1 ]; then
        printf '  "%s": "%s",\n' "$(p_json_escape "$_k")" "$(p_json_escape "$_v")" >> "$TMP_OUT"
    else
        printf '  %-30s %s\n' "$_k" "$_v"
    fi
}

section() {
    if [ "$JSON_MODE" -eq 0 ]; then
        p_head "$1"
    fi
}

# p_count counts non-empty lines on stdin. 'grep -c .' prints a 0 AND exits
# non-zero on no match, so 'grep -c . || echo 0' emits "0" twice — a bug
# that shipped in the first draft of this file.
p_count() {
    wc -l | tr -d ' '
}

# ── machine ───────────────────────────────────────────────────────────────────
section "machine"
emit "machine.host" "$(hostname 2>/dev/null || echo unknown)"
emit "machine.os" "$(uname -s 2>/dev/null || echo unknown)"
emit "machine.arch" "$(uname -m 2>/dev/null || echo unknown)"
emit "machine.kernel" "$(uname -r 2>/dev/null || echo unknown)"
emit "machine.cpus" "$( (nproc 2>/dev/null) || echo unknown)"
if command -v free >/dev/null 2>&1; then
    emit "machine.memory" "$(free -h 2>/dev/null | awk '/^Mem:/{print $2" total, "$7" available"}')"
fi
emit "machine.home" "$(p_home_path "$HOME")"

# ── toolchain ────────────────────────────────────────────────────────────────
section "toolchain"
for tool in git gh rustc cargo protoc node python3 docker; do
    if command -v "$tool" >/dev/null 2>&1; then
        emit "toolchain.$tool" "$(p_version "$tool")"
    else
        emit "toolchain.$tool" "not found"
    fi
done
if command -v rustc >/dev/null 2>&1; then
    emit "toolchain.rustc_ok" "$(rustc --version | grep -qE '1\.(8[0-9]|[9-9][0-9])' && echo yes || echo 'too old (need 1.80+)')"
fi

# ── repository ───────────────────────────────────────────────────────────────
section "repository"
emit "repo.path" "$(p_home_path "$REPO_PATH")"
if git -C "$REPO_PATH" rev-parse --git-dir >/dev/null 2>&1; then
    emit "repo.branch" "$(git -C "$REPO_PATH" branch --show-current 2>/dev/null)"
    emit "repo.head" "$(git -C "$REPO_PATH" rev-parse --short HEAD 2>/dev/null)"
    emit "repo.head_date" "$(git -C "$REPO_PATH" log -1 --format=%cI 2>/dev/null)"
    emit "repo.remotes" "$(git -C "$REPO_PATH" remote -v 2>/dev/null | awk '{print $1"="$2}' | sort -u | tr '\n' ' ')"
    emit "repo.dirty_files" "$(git -C "$REPO_PATH" status --porcelain 2>/dev/null | p_count)"
    # The NAMES of the dirty files: work that a clone will not bring with it.
    emit "repo.dirty_paths" "$(git -C "$REPO_PATH" status --porcelain 2>/dev/null | awk '{print $2}' | tr '\n' ' ')"
    emit "repo.stashes" "$(git -C "$REPO_PATH" stash list 2>/dev/null | p_count)"
    emit "repo.stash_entries" "$(git -C "$REPO_PATH" stash list 2>/dev/null | cut -d: -f1 | tr '\n' ' ')"
    emit "repo.unpushed" "$(git -C "$REPO_PATH" log --oneline '@{u}..HEAD' 2>/dev/null | p_count)"
    # Branch list matters: a branch that exists only on this machine is
    # work that a clone will not bring with it.
    emit "repo.local_branches" "$(git -C "$REPO_PATH" branch --format='%(refname:short)' 2>/dev/null | tr '\n' ' ')"
    emit "repo.local_only_branches" "$(git -C "$REPO_PATH" branch --format='%(refname:short)' 2>/dev/null |
        while read -r b; do
            git -C "$REPO_PATH" show-ref --verify --quiet "refs/remotes/origin/$b" || printf '%s ' "$b"
        done)"
else
    emit "repo.state" "not a git repository"
fi

# ── kwaainet homes (the state that is NOT in git) ───────────────────────────
section "kwaainet homes"
# The index keeps the two homes from colliding on the same JSON key: a box
# running the hivv integration has BOTH ~/.kwaainet and ~/.kwaainet-hivv, and
# an unindexed key would silently keep only the last one.
_i=0
p_kwaainet_homes | while read -r home; do
    [ -n "$home" ] || continue
    if [ ! -d "$home" ]; then
        emit "home.$_i.path" "$(p_home_path "$home")"
        emit "home.$_i.state" "absent"
        _i=$((_i + 1))
        continue
    fi
    emit "home.$_i.path" "$(p_home_path "$home")"
    emit "home.$_i.state" "present"

    cfg="$home/config.yaml"
    if [ -f "$cfg" ]; then
        emit "home.$_i.config" "present"
        # Keys only. A config's values can include tokens.
        emit "home.$_i.config_keys" "$(p_yaml_keys "$cfg" | tr '\n' ' ')"
        _port="$(p_yaml_value "$cfg" port)"
        [ -n "$_port" ] && emit "home.$_i.config_port" "$_port"
        _model="$(p_yaml_value "$cfg" model)"
        [ -n "$_model" ] && emit "home.$_i.config_model" "$_model"
    else
        emit "home.$_i.config" "absent"
    fi

    idf="$home/identity.key"
    if [ -f "$idf" ]; then
        # The identity IS the node. A node whose identity key does not move
        # is a new node with a new peer id and a fresh reputation.
        emit "home.$_i.identity" "$(p_secret_file "$idf" "identity.key")"
        emit "home.$_i.identity_bytes" "$(wc -c < "$idf" | tr -d ' ')"
    else
        emit "home.$_i.identity" "absent"
    fi

    [ -f "$home/reputation.json" ] && emit "home.$_i.reputation" "present"
    [ -d "$home/run" ] && emit "home.$_i.run_dir" "present"
    [ -d "$home/logs" ] && emit "home.$_i.logs_dir" "present"
    _i=$((_i + 1))
done

# ── services ────────────────────────────────────────────────────────────────
section "services"
if command -v systemctl >/dev/null 2>&1; then
    _units="$(systemctl --user list-unit-files --no-pager --no-legend 2>/dev/null |
              awk '{print $1}' | grep -E 'kwaainet|hivv|llama' || true)"
    if [ -z "$_units" ]; then
        emit "services.systemd_user" "none found"
    else
        for u in $_units; do
            emit "services.$u.enabled" "$(systemctl --user is-enabled "$u" 2>/dev/null || echo unknown)"
            emit "services.$u.active" "$(systemctl --user is-active "$u" 2>/dev/null || echo unknown)"
            # The ExecStart's absolute paths are the things that must exist
            # on the new box. They are paths, not secrets.
            _exec="$(systemctl --user show "$u" -p ExecStart --value 2>/dev/null |
                     grep -o 'argv\[\]=[^;]*' | head -1 | cut -d= -f2-)"
            [ -n "$_exec" ] && emit "services.$u.exec" "$(p_home_path "$_exec")"
            _env="$(systemctl --user show "$u" -p Environment --value 2>/dev/null | head -1)"
            if [ -n "$_env" ]; then
                # Environment can carry tokens. Keep the KEYS, drop the values.
                emit "services.$u.env_keys" "$(printf '%s' "$_env" | tr ' ' '\n' | grep '=' | cut -d= -f1 | tr '\n' ' ')"
            fi
        done
    fi
    # Unit files outside the repo are themselves state that must move.
    _unitdir="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"
    if [ -d "$_unitdir" ]; then
        emit "services.unit_dir" "$(p_home_path "$_unitdir")"
        _unitfiles=""
        for _uf in "$_unitdir"/*.service; do
            [ -e "$_uf" ] || continue
            _unitfiles="$_unitfiles $(basename "$_uf")"
        done
        emit "services.unit_files" "$(printf '%s' "$_unitfiles" | sed 's/^ //')"
    fi
else
    emit "services.systemd_user" "systemctl not available (not Linux?)"
    # macOS equivalent: user LaunchAgents.
    _agents="${HOME}/Library/LaunchAgents"
    if [ -d "$_agents" ]; then
        emit "services.launch_agents" "$(ls -1 "$_agents" 2>/dev/null | tr '\n' ' ')"
    fi
fi

# ── ports ────────────────────────────────────────────────────────────────────
section "ports"
for port in 8000 8090 18090 31337; do
    if p_port_open "$port"; then
        emit "ports.$port" "listening"
    else
        emit "ports.$port" "closed"
    fi
done

# ── github access ───────────────────────────────────────────────────────────
section "github access"
if [ -f "${HOME}/.ssh/id_ed25519" ]; then
    emit "github.ssh_key" "id_ed25519 present"
    if [ -f "${HOME}/.ssh/id_ed25519.pub" ]; then
        emit "github.ssh_pub_fingerprint" "$(ssh-keygen -lf "${HOME}/.ssh/id_ed25519.pub" 2>/dev/null | awk '{print $2" "$4}' || echo unknown)"
    fi
elif [ -f "${HOME}/.ssh/id_rsa" ]; then
    emit "github.ssh_key" "id_rsa present"
else
    emit "github.ssh_key" "none"
fi
if command -v gh >/dev/null 2>&1; then
    emit "github.gh_user" "$(gh api user --jq .login 2>/dev/null || echo 'not authenticated')"
    # 'gh auth status' writes to STDERR, not stdout. Without the 2>&1 this
    # line silently reported no scopes on a box that had a full token.
    emit "github.gh_scopes" "$(gh auth status 2>&1 | grep -o 'Token scopes:.*' | cut -d: -f2- | sed 's/^ *//')"
else
    emit "github.gh_user" "gh not installed"
fi
if command -v git >/dev/null 2>&1; then
    emit "github.user_name" "$(git config --global user.name 2>/dev/null || echo unset)"
    emit "github.user_email" "$(git config --global user.email 2>/dev/null || echo unset)"
fi

# ── hivv (the "live help" integration) ─────────────────────────────────────
section "hivv coupling"
for hv in "${HOME}/.local/lib/hivv" "${HOME}/.local/share/hivv"; do
    # The two directories share a basename, so they need distinct labels.
    _label="lib"
    case "$hv" in
        *share*) _label="share" ;;
    esac
    if [ -d "$hv" ]; then
        emit "hivv.${_label}_dir" "present"
    else
        emit "hivv.${_label}_dir" "absent"
    fi
done
if [ -d "${HOME}/.local/share/hivv/identity" ]; then
    emit "hivv.identity_files" "$(ls -1 "${HOME}/.local/share/hivv/identity" 2>/dev/null | tr '\n' ' ')"
fi
if [ -f "${HOME}/.local/share/hivv/hivv.env" ]; then
    emit "hivv.env" "$(p_secret_file "${HOME}/.local/share/hivv/hivv.env" "hivv.env")"
fi
if [ -d "/var/www/vhosts/hivv.org" ]; then
    emit "hivv.vhost" "/var/www/vhosts/hivv.org"
elif [ -d "${HOME}/www/vhosts/hivv.org" ]; then
    emit "hivv.vhost" "$(p_home_path "${HOME}/www/vhosts/hivv.org")"
else
    emit "hivv.vhost" "not found"
fi
# The shard API is the door hivv.org knocks on. If it is not listening, the
# hive is up but has no local answer to give.
if p_port_open 18090; then
    emit "hivv.shard_api" "listening on 18090"
else
    emit "hivv.shard_api" "not listening on 18090"
fi

# ── output ───────────────────────────────────────────────────────────────────
if [ "$JSON_MODE" -eq 1 ]; then
    _body="$(sed -e '$ s/,$//' "$TMP_OUT")"
    _doc="{
$_body
}
"
    if [ -n "$OUT_FILE" ]; then
        printf '%s' "$_doc" > "$OUT_FILE"
        echo "wrote $OUT_FILE"
    else
        printf '%s' "$_doc"
    fi
    rm -f "$TMP_OUT"
else
    p_summary
fi
