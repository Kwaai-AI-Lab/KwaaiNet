#!/usr/bin/env bash
# Unpack a cargo-dist kwaainet-<triple>.tar.xz into a flat payload directory
# holding kwaainet + p2pd. Usage: stage-payload.sh <archive.tar.xz> <outdir>
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

for f in kwaainet p2pd; do
    [ -f "${TOP}/${f}" ] || { echo "stage-payload: ${f} missing from ${ARCHIVE}" >&2; exit 1; }
done

mkdir -p "${OUTDIR}"
cp "${TOP}/kwaainet" "${TOP}/p2pd" "${OUTDIR}/"
chmod 0755 "${OUTDIR}/kwaainet" "${OUTDIR}/p2pd"

echo "stage-payload: staged kwaainet and p2pd into ${OUTDIR}"
