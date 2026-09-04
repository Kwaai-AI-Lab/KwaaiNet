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
[ -f "${PAYLOAD}/kwaainet" ] || { echo "build-deb: ${PAYLOAD}/kwaainet missing" >&2; exit 1; }

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

install -m 0644 "${HERE}/README.Debian" "${STAGE}/usr/share/doc/kwaainet/README.Debian"

# Derived, never hand-written. dpkg-shlibdeps insists on a debian/control
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
