#!/usr/bin/env bash
# Install/verify/remove smoke test for the kwaainet .deb. Runs INSIDE a
# Debian-family container.
# Usage: test-install.sh <path-to.deb> [--expect-update-gate]
set -euo pipefail

DEB="${1:?usage: test-install.sh <kwaainet_*.deb> [--expect-update-gate]}"
[ -f "${DEB}" ] || { echo "test-install: no such file: ${DEB}" >&2; exit 1; }
EXPECT_UPDATE_GATE=0
if [ "${2:-}" = "--expect-update-gate" ]; then EXPECT_UPDATE_GATE=1; fi

fail() { echo "FAIL: $*" >&2; exit 1; }
ok()   { echo "ok: $*"; }

export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null
apt-get install -y -qq "${DEB}" >/dev/null

# Stand in for pre-existing user data, to prove removal leaves it alone.
mkdir -p "${HOME}/.kwaainet"
echo "identity-key-stand-in" > "${HOME}/.kwaainet/identity.key"

command -v kwaainet >/dev/null || fail "kwaainet not on PATH"
ok "kwaainet on PATH at $(command -v kwaainet)"

# find_p2pd_binary() searches $PATH, so this is the property that matters.
command -v p2pd >/dev/null || fail "p2pd not on PATH"
ok "p2pd on PATH at $(command -v p2pd)"

kwaainet --version || fail "kwaainet --version failed"
ok "kwaainet --version ran"

MARKER="$(cat /usr/lib/kwaainet/packaged)"
[ "${MARKER}" = "deb" ] || fail "marker is '${MARKER}', expected 'deb'"
ok "marker /usr/lib/kwaainet/packaged is 'deb'"

[ -f /etc/default/kwaainet ] || fail "/etc/default/kwaainet missing"
grep -q '^KWAAINET_NO_AUTO_UPDATE=1$' /etc/default/kwaainet \
    || fail "KWAAINET_NO_AUTO_UPDATE=1 not set in /etc/default/kwaainet"
ok "/etc/default/kwaainet present with KWAAINET_NO_AUTO_UPDATE=1"

dpkg-query -W -f='${Conffiles}\n' kwaainet | grep -q '/etc/default/kwaainet' \
    || fail "/etc/default/kwaainet is not registered as a conffile"
ok "/etc/default/kwaainet registered as a conffile"

[ -f /usr/lib/systemd/user/kwaainet.service ] || fail "user unit missing"
grep -q '^ExecStart=/usr/bin/kwaainet run-node$' /usr/lib/systemd/user/kwaainet.service \
    || fail "unit ExecStart is not /usr/bin/kwaainet run-node"
ok "systemd user unit installed with the expected ExecStart"

# Minimized images (ubuntu:22.04) tell dpkg to drop man pages and most docs,
# so those files are legitimately absent there.
excluded() { grep -rqs "^path-exclude=$1" /etc/dpkg/dpkg.cfg.d/ /etc/dpkg/dpkg.cfg; }

if excluded "/usr/share/man/\*"; then
    ok "man page skipped: dpkg path-exclude drops /usr/share/man on this image"
elif command -v man >/dev/null 2>&1; then
    man -w kwaainet >/dev/null 2>&1 || fail "man -w kwaainet did not resolve"
    ok "man -w kwaainet resolves"
else
    [ -f /usr/share/man/man1/kwaainet.1.gz ] || fail "man page missing"
    ok "man page present (man(1) not installed, checked the path)"
fi

# copyright survives the exclusion via dpkg's own path-include.
[ -f /usr/share/doc/kwaainet/copyright ] || fail "copyright missing"
[ -f /usr/share/doc/kwaainet/changelog.Debian.gz ] || fail "changelog missing"
if excluded "/usr/share/doc/\*"; then
    ok "doc files present (README.Debian dropped by dpkg path-exclude)"
else
    [ -f /usr/share/doc/kwaainet/README.Debian ] || fail "README.Debian missing"
    ok "doc files present"
fi

# The marker is a filesystem contract between these scripts and
# updater::packaged_install(). If either side moves, a packaged install
# silently starts self-updating again — so assert the behaviour, not the file.
if [ "${EXPECT_UPDATE_GATE}" = 1 ]; then
    OUT="$(kwaainet update 2>&1)" || fail "kwaainet update exited non-zero"
    grep -q 'apt install --only-upgrade kwaainet' <<<"${OUT}" \
        || fail "kwaainet update did not defer to apt; got: ${OUT}"
    ok "kwaainet update defers to apt"
    grep -q 'apt remove kwaainet' <<<"$(yes n | kwaainet uninstall 2>&1 || true)" \
        || fail "kwaainet uninstall did not defer to apt"
    ok "kwaainet uninstall defers to apt"
else
    ok "update-gate assertions skipped (--expect-update-gate not given)"
fi

apt-get remove -y -qq kwaainet >/dev/null
[ ! -e /usr/bin/kwaainet ] || fail "/usr/bin/kwaainet survived remove"
[ ! -e /usr/bin/p2pd ] || fail "/usr/bin/p2pd survived remove"
ok "clean remove"

# remove keeps conffiles by design; purge is what must clear them.
[ -f /etc/default/kwaainet ] || fail "remove deleted the conffile (should keep it)"
ok "conffile retained by remove"

[ -f "${HOME}/.kwaainet/identity.key" ] || fail "remove deleted ${HOME}/.kwaainet"
ok "${HOME}/.kwaainet survived remove"

apt-get purge -y -qq kwaainet >/dev/null
[ ! -f /etc/default/kwaainet ] || fail "purge left the conffile behind"
[ ! -e /usr/lib/kwaainet ] || fail "purge left /usr/lib/kwaainet behind"
ok "clean purge"

# The identity key must outlive even a purge: losing it loses the node.
[ -f "${HOME}/.kwaainet/identity.key" ] || fail "purge deleted ${HOME}/.kwaainet"
ok "${HOME}/.kwaainet survived purge"

echo "ALL CHECKS PASSED"
