#!/usr/bin/env bash
# Build the kwaainet .deb from an already-built payload. Runs INSIDE a
# debian:bookworm container — it needs dpkg-dev, and lintian if --lintian.
#
# Usage: build-deb.sh --payload <dir> --arch <amd64|arm64> [--outdir <dir>]
#                     [--version <deb-version>] [--lintian]
#                     [--depends <string>]  (testing only: skips shlibdeps)
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGING="${HERE}/../packaging"

PAYLOAD=""
ARCH=""
OUTDIR="$(pwd)/dist"
VERSION_OVERRIDE=""
DEPENDS_OVERRIDE=""
RUN_LINTIAN=0

while [ $# -gt 0 ]; do
    case "$1" in
        --payload) PAYLOAD="$2"; shift 2 ;;
        --arch)    ARCH="$2"; shift 2 ;;
        --outdir)  OUTDIR="$2"; shift 2 ;;
        --version) VERSION_OVERRIDE="$2"; shift 2 ;;
        --depends) DEPENDS_OVERRIDE="$2"; shift 2 ;;
        --lintian) RUN_LINTIAN=1; shift ;;
        *) echo "build-deb: unknown argument: $1" >&2; exit 1 ;;
    esac
done

[ -n "${PAYLOAD}" ] || { echo "build-deb: --payload is required" >&2; exit 1; }
[ -d "${PAYLOAD}" ] || { echo "build-deb: no such payload dir: ${PAYLOAD}" >&2; exit 1; }
for f in kwaainet p2pd; do
    [ -f "${PAYLOAD}/${f}" ] || { echo "build-deb: ${PAYLOAD}/${f} missing" >&2; exit 1; }
done

[ -n "${ARCH}" ] || ARCH="$(dpkg --print-architecture)"

# Sets UPSTREAM_VERSION and DEB_VERSION.
eval "$("${PACKAGING}/version.sh")"
[ -z "${VERSION_OVERRIDE}" ] || DEB_VERSION="${VERSION_OVERRIDE}"

STAGE="$(mktemp -d)"
trap 'rm -rf "${STAGE}"' EXIT
chmod 0755 "${STAGE}"   # mktemp gives 0700, which would become the package root

install -d -m 0755 "${STAGE}/DEBIAN"
install -d -m 0755 "${STAGE}/usr/bin"
install -d -m 0755 "${STAGE}/usr/share/man/man1"
install -d -m 0755 "${STAGE}/usr/share/doc/kwaainet"
install -d -m 0755 "${STAGE}/usr/share/lintian/overrides"
install -d -m 0755 "${STAGE}/usr/lib/kwaainet"

install -m 0755 "${PAYLOAD}/kwaainet" "${STAGE}/usr/bin/kwaainet"
# /usr/bin, not a private libexec: find_p2pd_binary() searches only next to
# the exe, a cargo target dir, and $PATH.
install -m 0755 "${PAYLOAD}/p2pd" "${STAGE}/usr/bin/p2pd"

install -m 0644 "${PACKAGING}/copyright" "${STAGE}/usr/share/doc/kwaainet/copyright"
install -m 0644 "${HERE}/lintian-overrides" "${STAGE}/usr/share/lintian/overrides/kwaainet"

gzip -9nc "${PACKAGING}/kwaainet.1" > "${STAGE}/usr/share/man/man1/kwaainet.1.gz"
chmod 0644 "${STAGE}/usr/share/man/man1/kwaainet.1.gz"

# The marker the Rust updater reads to refuse a self-update.
printf 'deb\n' > "${STAGE}/usr/lib/kwaainet/packaged"
chmod 0644 "${STAGE}/usr/lib/kwaainet/packaged"

DATE="$(LC_ALL=C date -R)"
sed -e "s|@VERSION@|${DEB_VERSION}|g" \
    -e "s|@UPSTREAM_VERSION@|${UPSTREAM_VERSION}|g" \
    -e "s|@DATE@|${DATE}|g" \
    "${HERE}/changelog.Debian.in" \
    | gzip -9nc > "${STAGE}/usr/share/doc/kwaainet/changelog.Debian.gz"
chmod 0644 "${STAGE}/usr/share/doc/kwaainet/changelog.Debian.gz"

cat > "${STAGE}/usr/share/doc/kwaainet/README.Debian" <<'README'
kwaainet for Debian
===================

Installed from a package
------------------------
This copy of kwaainet was installed by the package manager, so self-update is
turned off: /usr/lib/kwaainet/packaged marks the install as "deb". Do not run
"kwaainet update" or "kwaainet uninstall" — they would replace or delete files
dpkg owns, leaving the package database disagreeing with the filesystem. Both
commands refuse and point at apt. Upgrade and remove with apt instead.

Running a node
--------------
This package installs no service. Start a node the same way you would from a
tarball install:

    kwaainet start

The node runs as you, and its configuration, logs and identity key live in
~/.kwaainet — so each user on a machine has their own node.

"kwaainet service install" writes a systemd *user* unit to
~/.config/systemd/user/kwaainet.service if you want the node to start at login.
Note that a user unit stops when your session ends unless you enable lingering
("loginctl enable-linger $USER"), so it is not a substitute for a system
service on a headless machine. A packaged system service is planned but
deliberately not shipped yet: it needs a system user, state under /var/lib and
configuration under /etc, none of which the current config layout supports.

Your data is never removed
--------------------------
Removing or purging this package does not touch ~/.kwaainet. It holds the node
identity key — deleting it means losing the node's identity on the network.

p2pd
----
/usr/bin/p2pd is the libp2p daemon the node runs alongside itself. It is on
PATH because that is where kwaainet looks for it; it is not meant to be run
by hand.
README
chmod 0644 "${STAGE}/usr/share/doc/kwaainet/README.Debian"

# Derived, never hand-written. Only kwaainet is passed: p2pd is a static ELF
# and dpkg-shlibdeps errors on it. dpkg-shlibdeps insists on a debian/control
# relative to its cwd, so give it an empty one to read.
if [ -n "${DEPENDS_OVERRIDE}" ]; then
    # Only the CI stub payload uses this: shell scripts have no ELF to analyze.
    echo "build-deb: WARNING using --depends override, shlibdeps skipped" >&2
    DEPENDS="${DEPENDS_OVERRIDE}"
else
    mkdir -p "${STAGE}/debian"
    touch "${STAGE}/debian/control"
    DEPENDS="$(
        cd "${STAGE}" \
            && dpkg-shlibdeps -O --ignore-missing-info usr/bin/kwaainet \
            | sed 's/^shlibs:Depends=//'
    )"
    rm -rf "${STAGE}/debian"
fi
[ -n "${DEPENDS}" ] || { echo "build-deb: dpkg-shlibdeps produced no Depends" >&2; exit 1; }
echo "build-deb: Depends: ${DEPENDS}"

INSTALLED_SIZE="$(du -k -s --exclude=DEBIAN "${STAGE}" | cut -f1)"

sed -e "s|@VERSION@|${DEB_VERSION}|g" \
    -e "s|@ARCH@|${ARCH}|g" \
    -e "s|@DEPENDS@|${DEPENDS}|g" \
    -e "s|@INSTALLED_SIZE@|${INSTALLED_SIZE}|g" \
    "${HERE}/control.in" > "${STAGE}/DEBIAN/control"
chmod 0644 "${STAGE}/DEBIAN/control"

mkdir -p "${OUTDIR}"
DEB="${OUTDIR}/kwaainet_${DEB_VERSION}_${ARCH}.deb"
dpkg-deb --build --root-owner-group "${STAGE}" "${DEB}"

( cd "${OUTDIR}" && sha256sum "$(basename "${DEB}")" > "$(basename "${DEB}").sha256" )

if [ "${RUN_LINTIAN}" = "1" ]; then
    echo "build-deb: lintian ---------------------------------------------"
    lintian --no-tag-display-limit "${DEB}" || true
fi

echo "build-deb: built ${DEB}"
echo "${DEPENDS}" > "${OUTDIR}/depends.${ARCH}"
