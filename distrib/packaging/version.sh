#!/usr/bin/env bash
# Emits UPSTREAM_VERSION and DEB_VERSION as shell assignments on stdout.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

UPSTREAM_VERSION="${KWAAINET_VERSION:-}"

if [ -z "${UPSTREAM_VERSION}" ]; then
    # `release.yml` sets tag from github.ref_name, which is a *branch* under
    # workflow_dispatch, so only take a tag that looks like a version.
    TAG="${GITHUB_REF_NAME:-}"
    if [ -z "${TAG}" ] && command -v git >/dev/null 2>&1; then
        TAG="$(git -C "${REPO_ROOT}" describe --tags --exact-match 2>/dev/null || true)"
    fi
    case "${TAG}" in
        v[0-9]*) UPSTREAM_VERSION="${TAG#v}" ;;
        [0-9]*)  UPSTREAM_VERSION="${TAG}" ;;
    esac
fi

if [ -z "${UPSTREAM_VERSION}" ]; then
    UPSTREAM_VERSION="$(
        awk '/^\[workspace\.package\]/{f=1;next} /^\[/{f=0} f && /^version *=/{gsub(/[" ]/,"");sub(/^version=/,"");print;exit}' \
            "${REPO_ROOT}/core/Cargo.toml"
    )"
fi

[ -n "${UPSTREAM_VERSION}" ] || { echo "version.sh: could not determine version" >&2; exit 1; }

# A bare `-` is illegal in a Debian version carrying a revision, and `~` also
# sorts a prerelease *before* the final release (0.7.0~rc.1 < 0.7.0).
DEB_VERSION="${UPSTREAM_VERSION//-/\~}-1"

echo "UPSTREAM_VERSION=${UPSTREAM_VERSION}"
echo "DEB_VERSION=${DEB_VERSION}"
