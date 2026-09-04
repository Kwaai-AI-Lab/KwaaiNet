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

# p2pd is being removed upstream; the package must never resurrect it.
if p2pd_path="$(dpkg -L kwaainet | grep -E '/p2pd$')"; then
    fail "package ships p2pd: ${p2pd_path}"
fi
ok "ships no p2pd, as intended"

kwaainet --version || fail "kwaainet --version failed"
ok "kwaainet --version ran"

MARKER="$(cat /usr/lib/kwaainet/packaged)"
[ "${MARKER}" = "deb" ] || fail "marker is '${MARKER}', expected 'deb'"
ok "marker /usr/lib/kwaainet/packaged is 'deb'"

# No service is shipped yet: a system daemon needs a system user, /var/lib
# state and /etc config, none of which the current config layout supports.
# Ask what the package owns — /usr/lib/systemd exists on any Debian system.
if owned="$(dpkg -L kwaainet | grep -E '^/(usr/lib/systemd|etc)/')"; then
    fail "package ships a unit or /etc config: ${owned}"
fi
ok "ships no service and no /etc config, as intended"

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
[ ! -e /usr/lib/kwaainet ] || fail "/usr/lib/kwaainet survived remove"
ok "clean remove"

[ -f "${HOME}/.kwaainet/identity.key" ] || fail "remove deleted ${HOME}/.kwaainet"
ok "${HOME}/.kwaainet survived remove"

# Purge is the stronger claim: it is where a package would plausibly delete
# the identity key. The package ships no conffiles, so remove already leaves
# nothing on disk and purge must be run against the installed package.
apt-get install -y -qq "${DEB}" >/dev/null
apt-get purge -y -qq kwaainet >/dev/null
[ ! -e /usr/bin/kwaainet ] || fail "/usr/bin/kwaainet survived purge"
[ ! -e /usr/lib/kwaainet ] || fail "/usr/lib/kwaainet survived purge"
ok "clean purge"

[ -f "${HOME}/.kwaainet/identity.key" ] || fail "purge deleted ${HOME}/.kwaainet"
ok "${HOME}/.kwaainet survived purge"

echo "ALL CHECKS PASSED"
