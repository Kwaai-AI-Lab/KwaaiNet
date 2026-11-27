#!/usr/bin/env bash
# github-setup.sh — give this machine a GitHub identity that can CLONE and PUSH.
#
# Moving a repo to a new machine is only half a move if the new machine can
# read GitHub but not write to it. Every piece of this script exists because
# "it worked on the old box" hid one of these:
#
#   * an SSH key that exists locally but is not REGISTERED with GitHub
#     (the clone works over https by accident; the push 403s)
#   * no 'git config user.name/user.email', so the first commit is authored
#     by "asher@ashy" and gets rejected by the fork's branch protection
#   * no 'gh' CLI, so the "add my key" step needs a browser on a machine
#     that is not the one you are on
#   * remotes named differently than the convention the org expects
#
# Modes:
#   --check    read-only report; changes nothing. This is what CI runs.
#   --apply    make the changes. Requires the ability to sudo, install
#              packages, and (for key registration) an authenticated 'gh'.
#
# Nothing here prints a secret. The private key is never echoed; only its
# fingerprint and the PUBLIC key (which is public by definition) appear.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/portability/lib.sh
. "$SCRIPT_DIR/lib.sh"

MODE="check"
KEY_TYPE="ed25519"
KEY_PATH=""
KEY_COMMENT=""
GITHUB_LOGIN="${GITHUB_LOGIN:-}"
UPSTREAM_REPO="${KWAAI_UPSTREAM_REPO:-Kwaai-AI-Lab/KwaaiNet}"
FORK_REPO="${KWAAI_FORK_REPO:-}"
REPO_PATH=""

usage() {
    cat <<'EOF'
usage: github-setup.sh [--check|--apply] [--repo PATH] [--login LOGIN]
                        [--upstream OWNER/REPO] [--fork OWNER/REPO]
                        [--key-type ed25519|rsa] [--key-path PATH]

  --check            report only; change nothing (default)
  --apply            apply the changes
  --repo PATH        the repository to configure remotes for
  --login LOGIN      GitHub login to configure (default: gh's own login)
  --upstream OWNER/REPO   the upstream remote target
  --fork OWNER/REPO       the personal fork remote target
  --key-type TYPE    ed25519 (default) or rsa
  --key-path PATH    explicit private key path
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --check) MODE="check" ;;
        --apply) MODE="apply" ;;
        --repo) shift; REPO_PATH="${1:-}" ;;
        --login) shift; GITHUB_LOGIN="${1:-}" ;;
        --upstream) shift; UPSTREAM_REPO="${1:-}" ;;
        --fork) shift; FORK_REPO="${1:-}" ;;
        --key-type) shift; KEY_TYPE="${1:-}" ;;
        --key-path) shift; KEY_PATH="${1:-}" ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
    shift
done

if [ -z "$REPO_PATH" ]; then
    REPO_PATH="$(p_repo_root "$SCRIPT_DIR")" || REPO_PATH="$(pwd)"
fi

if [ -z "$KEY_PATH" ]; then
    if [ "$KEY_TYPE" = "rsa" ]; then
        KEY_PATH="${HOME}/.ssh/id_rsa"
    else
        KEY_PATH="${HOME}/.ssh/id_ed25519"
    fi
fi
PUB_PATH="${KEY_PATH}.pub"

p_head "github identity"

# ── 1. ssh key ───────────────────────────────────────────────────────────────
if [ -f "$KEY_PATH" ]; then
    p_ok "ssh key present: $(p_home_path "$KEY_PATH")"
    if [ -f "$PUB_PATH" ]; then
        p_ok "public key present: $(p_home_path "$PUB_PATH")"
        p_dim "fingerprint: $(ssh-keygen -lf "$PUB_PATH" 2>/dev/null | awk '{print $2}')"
    else
        p_warn "public key missing — regenerate with: ssh-keygen -y -f $KEY_PATH > $PUB_PATH"
    fi
else
    p_warn "ssh key missing: $(p_home_path "$KEY_PATH")"
    if [ "$MODE" = "apply" ]; then
        mkdir -p "${HOME}/.ssh"
        chmod 700 "${HOME}/.ssh"
        # No passphrase: this key is for a machine, not a person, and an
        # automated agent cannot answer a passphrase prompt. Say so loudly.
        p_step "generating an ed25519 key with no passphrase (machine key)"
        ssh-keygen -t "$KEY_TYPE" -f "$KEY_PATH" -N "" -C "${KEY_COMMENT:-kwaainet@$(hostname)}" >/dev/null 2>&1
        chmod 600 "$KEY_PATH"
        p_ok "generated $(p_home_path "$KEY_PATH")"
    else
        p_dim "would run: ssh-keygen -t $KEY_TYPE -f $KEY_PATH -N ''"
    fi
fi

# ── 2. gh cli ────────────────────────────────────────────────────────────────
if command -v gh >/dev/null 2>&1; then
    p_ok "gh cli present: $(gh --version 2>/dev/null | head -1)"
else
    p_warn "gh cli not installed"
    if [ "$MODE" = "apply" ]; then
        p_step "installing gh cli"
        if command -v apt-get >/dev/null 2>&1; then
            (sudo apt-get update -qq && sudo apt-get install -y -qq gh) >/dev/null 2>&1 ||
                (curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg |
                 sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg 2>/dev/null &&
                 echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" |
                 sudo tee /etc/apt/sources.list.d/github-cli.list >/dev/null &&
                 sudo apt-get update -qq && sudo apt-get install -y -qq gh) >/dev/null 2>&1
        elif command -v brew >/dev/null 2>&1; then
            brew install gh >/dev/null 2>&1
        fi
        command -v gh >/dev/null 2>&1 && p_ok "gh cli installed" || p_fail "gh cli install failed"
    else
        p_dim "would install the gh cli"
    fi
fi

# ── 3. gh authentication ─────────────────────────────────────────────────────
GH_AUTHED=0
if command -v gh >/dev/null 2>&1; then
    _login="$(gh api user --jq .login 2>/dev/null || true)"
    if [ -n "$_login" ]; then
        GH_AUTHED=1
        p_ok "gh authenticated as $_login"
        [ -z "$GITHUB_LOGIN" ] && GITHUB_LOGIN="$_login"
        p_dim "scopes: $(gh auth status 2>&1 | grep -o 'Token scopes:.*' | cut -d: -f2- | sed 's/^ *//')"
    else
        p_warn "gh is installed but not authenticated"
        if [ "$MODE" = "apply" ]; then
            p_step "starting gh device login (finish it in the browser it prints)"
            gh auth login --hostname github.com --git-protocol ssh --web 2>&1 | tail -5
            _login="$(gh api user --jq .login 2>/dev/null || true)"
            if [ -n "$_login" ]; then
                GH_AUTHED=1
                p_ok "gh authenticated as $_login"
                [ -z "$GITHUB_LOGIN" ] && GITHUB_LOGIN="$_login"
            else
                p_fail "gh auth login did not complete"
            fi
        else
            p_dim "would run: gh auth login --hostname github.com --git-protocol ssh --web"
        fi
    fi
fi

# ── 4. register the key with GitHub ─────────────────────────────────────────
if [ -f "$PUB_PATH" ]; then
    _fp="$(ssh-keygen -lf "$PUB_PATH" 2>/dev/null | awk '{print $2}')"
    # Compare the KEY BODY, not the fingerprint. 'gh ssh-key list' prints the
    # public key itself (title, type, body, date, id) and has no --json in
    # the 2.x line, so a fingerprint comparison never matches anything and
    # reports every key as unregistered. Comparing bodies is exact.
    _body="$(awk '{print $2}' "$PUB_PATH" 2>/dev/null)"
    if [ "$GH_AUTHED" -eq 1 ]; then
        _known="$(gh ssh-key list 2>/dev/null | awk '{print $3}' | grep -Fx "$_body" || true)"
        if [ -n "$_known" ]; then
            p_ok "key already registered with GitHub ($_fp)"
        elif [ "$MODE" = "apply" ]; then
            p_step "registering key ${_fp} with GitHub"
            if gh ssh-key add "$PUB_PATH" --title "${KEY_COMMENT:-kwaainet@$(hostname)}" 2>&1 | tail -2; then
                p_ok "key registered with GitHub"
            else
                p_fail "key registration failed"
            fi
        else
            p_warn "key ${_fp} is NOT registered with GitHub — pushes will fail"
            p_dim "would run: gh ssh-key add $PUB_PATH"
        fi
    else
        p_warn "cannot check key registration without an authenticated gh"
    fi
    # The public key is public. Printing it is how a human registers it by
    # hand when there is no browser on this box.
    p_info "public key (safe to paste into github.com/settings/ssh/new):"
    p_dim "$(cat "$PUB_PATH" 2>/dev/null)"
fi

# ── 5. git identity ──────────────────────────────────────────────────────────
p_head "git identity"
_name="$(git config --global user.name 2>/dev/null || true)"
_email="$(git config --global user.email 2>/dev/null || true)"
if [ -n "$_name" ] && [ -n "$_email" ]; then
    p_ok "git identity set: $_name <$_email>"
else
    p_warn "git identity incomplete (name='$_name' email='$_email')"
    if [ "$MODE" = "apply" ] && [ "$GH_AUTHED" -eq 1 ]; then
        _ghname="$(gh api user --jq .name 2>/dev/null || true)"
        _ghmail="$(gh api user --jq .email 2>/dev/null || true)"
        [ -z "$_ghmail" ] && _ghmail="$(gh api user --jq .id 2>/dev/null || true)@$(gh api user --jq .login 2>/dev/null || echo users.noreply.github.com)"
        git config --global user.name "${_ghname:-$GITHUB_LOGIN}"
        git config --global user.email "${_ghmail:-users.noreply.github.com}"
        p_ok "git identity set from GitHub: $(git config --global user.name) <$(git config --global user.email)>"
    else
        p_dim "would set user.name/user.email from the authenticated GitHub account"
    fi
fi

# ── 6. remotes ───────────────────────────────────────────────────────────────
p_head "remotes"
if git -C "$REPO_PATH" rev-parse --git-dir >/dev/null 2>&1; then
    if [ -n "$FORK_REPO" ]; then
        if [ "$MODE" = "apply" ]; then
            git -C "$REPO_PATH" remote set-url origin "git@github.com:${UPSTREAM_REPO}.git" 2>/dev/null
            git -C "$REPO_PATH" remote set-url fork "git@github.com:${FORK_REPO}.git" 2>/dev/null ||
                git -C "$REPO_PATH" remote add fork "git@github.com:${FORK_REPO}.git" 2>/dev/null
            p_ok "remotes set: origin=$UPSTREAM_REPO fork=$FORK_REPO"
        else
            p_info "would set origin=$UPSTREAM_REPO fork=$FORK_REPO"
        fi
    fi
    git -C "$REPO_PATH" remote -v 2>/dev/null | awk '!seen[$1" "$2]++' | while read -r _name _url _rest; do
        p_dim "remote $_name -> $_url"
    done
else
    p_warn "not a git repository: $(p_home_path "$REPO_PATH")"
fi

# ── 7. verify ────────────────────────────────────────────────────────────────
p_head "verification"
# Pin the key. OpenSSH expands '~' through the PASSWD DATABASE, not through
# $HOME, so an unpinned probe on a box with several keys reports success for
# whichever key it happened to pick — which is not necessarily the key this
# script just created, nor the one a migration moved.
# Host-key handling depends on the mode: in check mode the probe
# must not write ~/.ssh/known_hosts (see p_ssh_host_opts), in apply mode
# accepting the host key into the real known_hosts is exactly what a
# first-time clone does anyway.
if [ "$MODE" = "apply" ]; then
    _SSH_OPTS="$(p_ssh_host_opts apply)"
else
    _SSH_OPTS="$(p_ssh_host_opts readonly)"
fi
_SSH_OPTS="$_SSH_OPTS -o IdentitiesOnly=yes"
if [ -f "$KEY_PATH" ]; then
    _SSH_OPTS="$_SSH_OPTS -i $KEY_PATH"
else
    # No key on this machine: offer nothing, so the probe reports the truth
    # instead of finding a key through passwd's idea of '~'.
    _SSH_OPTS="$_SSH_OPTS -o IdentityFile=/dev/null"
fi
_ssh_out="$(ssh $_SSH_OPTS -T git@github.com 2>&1)"
# The WHOLE output, not just its first line: ssh prints warnings and
# notices before the result, and taking head -1 reports "inconclusive" on a
# perfectly healthy key.
case "$_ssh_out" in
    *"successfully authenticated"*) p_ok "ssh authentication to github.com works" ;;
    *"Permission denied"*) p_fail "ssh authentication to github.com FAILED — the key is not registered" ;;
    *) p_warn "ssh probe inconclusive: $(printf '%s' "$_ssh_out" | head -1)" ;;
esac

GIT_SSH_COMMAND="ssh $_SSH_OPTS"
export GIT_SSH_COMMAND

if git -C "$REPO_PATH" rev-parse --git-dir >/dev/null 2>&1; then
    if git -C "$REPO_PATH" ls-remote --exit-code origin HEAD >/dev/null 2>&1; then
        p_ok "can read from origin"
    else
        p_fail "cannot read from origin"
    fi
    # --dry-run does the full authentication and permission check without
    # writing a single ref. It is the only honest way to prove PUSH works
    # without pushing.
    if git -C "$REPO_PATH" push --dry-run origin HEAD:refs/heads/portability/probe >/dev/null 2>&1; then
        p_ok "can PUSH to origin (dry run against a probe branch)"
    else
        p_fail "cannot push to origin — the token/key lacks write access"
    fi
fi

p_summary
p_exit_code
