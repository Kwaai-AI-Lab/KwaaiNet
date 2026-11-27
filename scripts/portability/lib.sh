#!/usr/bin/env bash
# Shared helpers for the portability toolkit.
#
# Bash 3.2 compatible on purpose. macOS ships bash 3.2 as /bin/bash, and the
# whole point of this toolkit is to run on the machine you are moving TO —
# the one you have not set up yet, where you cannot assume a modern bash.
# So: no associative arrays, no mapfile/readarray, no ${var,,}, no 'local -n'.
#
# Sourced, not executed. 'set -euo pipefail' is deliberately NOT set here:
# these are reporting tools, and a missing optional tool (no ss, no gh,
# no systemd) must produce a warning, not a dead script.

# Colour only when a human is watching.
if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
    C_RESET=$'\033[0m'
    C_OK=$'\033[32m'
    C_FAIL=$'\033[31m'
    C_WARN=$'\033[33m'
    C_INFO=$'\033[36m'
    C_DIM=$'\033[2m'
    C_BOLD=$'\033[1m'
else
    C_RESET=""
    C_OK=""
    C_FAIL=""
    C_WARN=""
    C_INFO=""
    C_DIM=""
    C_BOLD=""
fi

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0
SKIP_COUNT=0

p_ok()   { PASS_COUNT=$((PASS_COUNT + 1)); printf '%s[ ok ]%s %s\n' "$C_OK" "$C_RESET" "$*"; }
p_fail() { FAIL_COUNT=$((FAIL_COUNT + 1)); printf '%s[FAIL]%s %s\n' "$C_FAIL" "$C_RESET" "$*"; }
p_warn() { WARN_COUNT=$((WARN_COUNT + 1)); printf '%s[warn]%s %s\n' "$C_WARN" "$C_RESET" "$*"; }
p_skip() { SKIP_COUNT=$((SKIP_COUNT + 1)); printf '%s[skip]%s %s\n' "$C_DIM" "$C_RESET" "$*"; }
p_info() { printf '%s[info]%s %s\n' "$C_INFO" "$C_RESET" "$*"; }
p_dim()  { printf '%s      %s%s\n' "$C_DIM" "$*" "$C_RESET"; }
p_head() { printf '\n%s%s== %s ==%s\n' "$C_BOLD" "$C_INFO" "$*" "$C_RESET"; }
p_step() { printf '%s  ->%s %s\n' "$C_INFO" "$C_RESET" "$*"; }

# p_summary prints the tally every tool ends with.
p_summary() {
    printf '\n%s%s%d passed, %d failed, %d warnings, %d skipped%s\n' \
        "$C_BOLD" "$C_INFO" "$PASS_COUNT" "$FAIL_COUNT" "$WARN_COUNT" "$SKIP_COUNT" "$C_RESET"
}

# p_exit_code returns 0 when nothing failed.
p_exit_code() {
    if [ "$FAIL_COUNT" -gt 0 ]; then
        return 1
    fi
    return 0
}

# p_hash prints the first 12 hex chars of the sha256 of stdin.
# Prefers sha256sum (Linux), falls back to shasum (macOS), then cksum.
p_hash() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum | cut -c1-12
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 | cut -c1-12
    else
        cksum | tr -d ' ' | cut -c1-12
    fi
}

# p_secret describes a secret without disclosing it.
#
# The portability problem this solves: a node's identity key and a hivv
# node's token have to move to the new machine, but an inventory file is
# something people paste into chat. So the inventory records that the value
# is set and what it fingerprints as, never the value itself. Two
# inventories can be diffed to prove the same key moved, and that diff is
# safe to share.
p_secret() {
    # $1 = value (may be empty), $2 = label
    if [ -z "${1:-}" ]; then
        printf '%s (unset)' "$2"
    else
        printf '%s (set, sha256:%s)' "$2" "$(printf '%s' "$1" | p_hash)"
    fi
}

# p_home_path rewrites $HOME to ~ so an inventory is comparable between
# machines with different usernames.
#
# The tilde is BACKSLASH-ESCAPED on purpose. Bash runs tilde expansion on the
# replacement text of ${var/pat/repl}, so an unescaped '~' expands back to
# $HOME and the function becomes a no-op that silently prints the absolute
# path. That bug shipped in the first draft of this file.
p_home_path() {
    printf '%s' "${1/#$HOME/\~}"
}

# p_secret_file describes a secret FILE without disclosing it.
p_secret_file() {
    # $1 = path, $2 = label
    if [ ! -f "$1" ]; then
        printf '%s (absent)' "$2"
        return 1
    fi
    printf '%s (set, sha256:%s)' "$2" "$(p_hash < "$1")"
}

# p_preferred_ssh_key prints the private key this tool should be using.
#
# Order matters: ed25519 first (the modern default, and what
# github-setup.sh creates), then RSA for the older boxes.
p_preferred_ssh_key() {
    if [ -f "${HOME}/.ssh/id_ed25519" ]; then
        printf '%s' "${HOME}/.ssh/id_ed25519"
    elif [ -f "${HOME}/.ssh/id_rsa" ]; then
        printf '%s' "${HOME}/.ssh/id_rsa"
    else
        printf ''
    fi
}

# p_git_ssh_command prints a GIT_SSH_COMMAND value that pins the key.
#
# Without this, a git push check answers a question nobody asked: OpenSSH
# expands '~' through the PASSWD DATABASE, not through $HOME, so on a box
# with several keys it silently picks whichever one it likes first. The
# check then reports success for a key the migration never moved.
#
# -o IdentitiesOnly=yes is load-bearing: it stops ssh from offering the
# other keys in the agent and the default list.
#
# When there is NO key, IdentityFile=/dev/null makes ssh offer nothing at
# all. Without it, IdentitiesOnly=yes still falls back to the default
# identity list, which resolves '~' through passwd — so the probe would
# find a key on a machine that has none, and report a success that is
# about a different machine entirely.
#
# $1: "readonly" (default) or "apply". See p_ssh_host_opts.
p_git_ssh_command() {
    _mode="${1:-readonly}"
    _key="$(p_preferred_ssh_key)"
    printf 'ssh %s' "$(p_ssh_host_opts "$_mode")"
    if [ -n "$_key" ]; then
        printf ' -o IdentitiesOnly=yes -i %s' "$_key"
    else
        printf ' -o IdentitiesOnly=yes -o IdentityFile=/dev/null'
    fi
}

# p_ssh_host_opts prints the host-key options for an ssh invocation.
#
# "readonly" throws the host key away. This matters: with
# StrictHostKeyChecking=accept-new, the FIRST ssh connection on a machine
# with no ~/.ssh writes ~/.ssh/known_hosts. A check mode that creates a
# directory in the user's home is not a check mode, and the CI job
# "github-setup --check changes nothing" caught exactly that on macOS
# (the macOS runner has no ~/.ssh; the ubuntu one already had one, which
# is why only macOS failed).
#
# "apply" accepts the host key into the real known_hosts, which is what a
# first-time clone does anyway and is a legitimate mutation in a mode
# whose whole job is to set the machine up.
p_ssh_host_opts() {
    _mode="${1:-readonly}"
    # IdentityAgent=none is load-bearing on macOS: Apple's ssh defaults to a
    # per-user agent at ~/.ssh/agent and CREATES that file on the first
    # connection attempt. A check mode that leaves a file in the user's
    # home is not a check mode. It also removes the agent from the picture
    # entirely, which is right anyway — an agent can offer other keys, and
    # the whole point of pinning the key is that only THAT key is tested.
    if [ "$_mode" = "apply" ]; then
        printf -- '-o BatchMode=yes -o StrictHostKeyChecking=accept-new -o IdentityAgent=none'
    else
        # LogLevel=ERROR suppresses the "Permanently added ... to the list of
        # known hosts" notice that StrictHostKeyChecking=no prints on every
        # connection. Without it that notice lands on the FIRST line of the
        # probe's output and hides the actual result behind it.
        printf -- '-o BatchMode=yes -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -o LogLevel=ERROR -o IdentityAgent=none'
    fi
}

# p_version prints the first line of a tool's version output, or "not found".
p_version() {
    _cmd="$1"
    _flag="${2:---version}"
    if ! command -v "$_cmd" >/dev/null 2>&1; then
        printf 'not found'
        return 1
    fi
    "$_cmd" "$_flag" 2>&1 | head -1
}

# p_port_open reports whether something is listening on a TCP port.
# Uses ss (Linux), netstat (both), then lsof (macOS).
p_port_open() {
    _port="$1"
    if command -v ss >/dev/null 2>&1; then
        ss -ltn 2>/dev/null | awk '{print $4}' | grep -qE "[:.]${_port}$"
        return $?
    fi
    if command -v netstat >/dev/null 2>&1; then
        netstat -ltn 2>/dev/null | awk '{print $4}' | grep -qE "[:.]${_port}$"
        return $?
    fi
    if command -v lsof >/dev/null 2>&1; then
        lsof -nP -iTCP:"$_port" -sTCP:LISTEN >/dev/null 2>&1
        return $?
    fi
    return 1
}

# p_json_escape escapes a string for inclusion in a JSON document.
p_json_escape() {
    printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' | tr '\n' ' '
}

# p_repo_root walks up from a directory until it finds a .git.
p_repo_root() {
    _d="$(cd "${1:-.}" 2>/dev/null && pwd)" || return 1
    while [ -n "$_d" ] && [ "$_d" != "/" ]; do
        if [ -e "$_d/.git" ]; then
            printf '%s' "$_d"
            return 0
        fi
        _d="$(dirname "$_d")"
    done
    return 1
}

# p_kwaainet_homes prints every KWAAINET_HOME this box knows about.
# The env var wins; the two conventional locations are always included so a
# box running the hivv integration (which uses its own home) is fully
# inventoried rather than half-inventoried.
p_kwaainet_homes() {
    if [ -n "${KWAAINET_HOME:-}" ]; then
        printf '%s\n' "$KWAAINET_HOME"
    fi
    printf '%s\n' "$HOME/.kwaainet"
    printf '%s\n' "$HOME/.kwaainet-hivv"
}

# p_yaml_keys prints the top-level keys of a YAML file without printing any
# values. An inventory can then show what a config is CONFIGURED to do
# without leaking a token that happens to live in it.
p_yaml_keys() {
    _f="$1"
    [ -f "$_f" ] || return 1
    if command -v python3 >/dev/null 2>&1; then
        python3 - "$_f" <<'PYEOF' 2>/dev/null && return 0
import sys
try:
    import yaml
except ImportError:
    yaml = None
path = sys.argv[1]
if yaml is not None:
    with open(path) as fh:
        data = yaml.safe_load(fh) or {}
    if isinstance(data, dict):
        for k in sorted(data):
            print(k)
else:
    with open(path) as fh:
        for line in fh:
            if not line.strip() or line.lstrip().startswith('#'):
                continue
            if line[0] not in ' \t-':
                key = line.split(':', 1)[0].strip()
                if key:
                    print(key)
PYEOF
    fi
    # No python (or python failed): crude but sufficient — top-level keys only.
    grep -E '^[A-Za-z_][A-Za-z0-9_]*:' "$_f" 2>/dev/null | cut -d: -f1
}

# p_yaml_value prints ONE value from a YAML file by top-level key.
# Used only for values known to be non-secret (a port, a path).
p_yaml_value() {
    _f="$1"
    _k="$2"
    [ -f "$_f" ] || return 1
    grep -E "^${_k}:" "$_f" 2>/dev/null | head -1 | cut -d: -f2- |
        sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//' -e 's/^"//' -e 's/"$//'
}
