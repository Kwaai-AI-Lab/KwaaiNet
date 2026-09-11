#!/usr/bin/env bash
# Unpack a cargo-dist kwaainet-<triple>.tar.xz into a flat payload directory
# holding kwaainet. Usage: stage-payload.sh <archive.tar.xz> <outdir>
set -euo pipefail

ARCHIVE="${1:?usage: stage-payload.sh <archive.tar.xz> <outdir>}"
OUTDIR="${2:?usage: stage-payload.sh <archive.tar.xz> <outdir>}"

[ -f "${ARCHIVE}" ] || { echo "stage-payload: no such archive: ${ARCHIVE}" >&2; exit 1; }

TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

tar -xJf "${ARCHIVE}" -C "${TMP}"

# The archive holds exactly one top-level directory; flatten it away.
TOP="$(find "${TMP}" -mindepth 1 -maxdepth 1 -type d)"
[ -d "${TOP}" ] || { echo "stage-payload: expected one top-level dir in ${ARCHIVE}" >&2; exit 1; }

[ -f "${TOP}/kwaainet" ] || { echo "stage-payload: kwaainet missing from ${ARCHIVE}" >&2; exit 1; }

# Only kwaainet is staged; anything else in the archive (README, a p2pd from
# a pre-0.6.9 archive) is ignored rather than rejected.
mkdir -p "${OUTDIR}"
cp "${TOP}/kwaainet" "${OUTDIR}/"
chmod 0755 "${OUTDIR}/kwaainet"

echo "stage-payload: staged kwaainet into ${OUTDIR}"
