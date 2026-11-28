#!/usr/bin/env bash
# install-user-unit.sh — render and install a KwaaiNet systemd USER unit.
#
# The unit files that run a production box live in ~/.config/systemd/user/,
# which is not in any repository. That means a box cannot be rebuilt from
# its own source, and a unit that was fixed on one box stays fixed on that
# box only. This script takes the template in this directory and puts a
# rendered, correct-for-this-machine copy where systemd will find it.
#
# Usage:
#   bash distrib/systemd/install-user-unit.sh                 # install
#   bash distrib/systemd/install-user-unit.sh --check         # show, install nothing
#   bash distrib/systemd/install-user-unit.sh --name kwaainet-shard-api-hivv.service \
#       --home ~/.kwaainet-hivv --port 18090 --model unsloth/Llama-3.1-8B-Instruct
#
# Bash 3.2 compatible — macOS ships bash 3.2 and the box you install this
# on may be the one you have not set up yet.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

TEMPLATE="${SCRIPT_DIR}/kwaainet-shard-api.service.template"
UNIT_NAME="kwaainet-shard-api.service"
KWAAINET_HOME_DIR="${KWAAINET_HOME:-${HOME}/.kwaainet}"
PORT="18090"
MODEL="unsloth/Llama-3.1-8B-Instruct"
BINARY=""
WORKING_DIRECTORY=""
CHECK=0

usage() {
    cat <<'EOF'
usage: install-user-unit.sh [--check] [--name FILE] [--home DIR]
                             [--port N] [--model NAME] [--binary PATH]
                             [--working-dir DIR] [--no-enable]

  --check         render and print the unit; install nothing
  --name FILE     unit file name (default: kwaainet-shard-api.service)
  --home DIR      KWAAINET_HOME for the unit (default: $KWAAINET_HOME or ~/.kwaainet)
  --port N        port the shard api listens on (default: 18090)
  --model NAME    model the shard api serves (default: unsloth/Llama-3.1-8B-Instruct)
  --binary PATH   kwaainet binary (default: the release build in this repo)
  --working-dir DIR  WorkingDirectory for the unit (default: the repo root)
  --no-enable     install the unit but do not enable or start it
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --check) CHECK=1 ;;
        --name) shift; UNIT_NAME="${1:-}" ;;
        --home) shift; KWAAINET_HOME_DIR="${1:-}" ;;
        --port) shift; PORT="${1:-}" ;;
        --model) shift; MODEL="${1:-}" ;;
        --binary) shift; BINARY="${1:-}" ;;
        --working-dir) shift; WORKING_DIRECTORY="${1:-}" ;;
        --no-enable) NO_ENABLE=1 ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
    shift
done

if [ ! -f "$TEMPLATE" ]; then
    echo "missing template: $TEMPLATE" >&2
    exit 1
fi

# The binary: prefer a release build in this repo, then a debug build, then
# whatever is on PATH.
if [ -z "$BINARY" ]; then
    for cand in "$REPO_ROOT/core/target/release/kwaainet" "$REPO_ROOT/core/target/debug/kwaainet"; do
        if [ -x "$cand" ]; then
            BINARY="$cand"
            break
        fi
    done
    if [ -z "$BINARY" ] && command -v kwaainet >/dev/null 2>&1; then
        BINARY="$(command -v kwaainet)"
    fi
fi
if [ -z "$BINARY" ]; then
    echo "no kwaainet binary found — build it first (bash setup.sh) or pass --binary" >&2
    exit 1
fi

[ -n "$WORKING_DIRECTORY" ] || WORKING_DIRECTORY="$REPO_ROOT"

# The unit's log directory has to exist before systemd starts the unit, or
# the append: target fails and the unit dies on a missing directory.
LOG_DIR="${KWAAINET_HOME_DIR}/logs"

UNIT_DIR="${XDG_CONFIG_HOME:-${HOME}/.config}/systemd/user"
UNIT_PATH="${UNIT_DIR}/${UNIT_NAME}"

render() {
    sed \
        -e "s|{{PORT}}|${PORT}|g" \
        -e "s|{{MODEL}}|${MODEL}|g" \
        -e "s|{{KWAAINET_HOME}}|${KWAAINET_HOME_DIR}|g" \
        -e "s|{{BINARY}}|${BINARY}|g" \
        -e "s|{{WORKING_DIRECTORY}}|${WORKING_DIRECTORY}|g" \
        "$TEMPLATE"
}

if [ "$CHECK" -eq 1 ]; then
    echo "# ---- would write ${UNIT_PATH} ----"
    render
    echo "# ---- end ----"
    echo
    echo "checks:"
    echo "  binary       ${BINARY} $([ -x "$BINARY" ] && echo '(present)' || echo '(MISSING)')"
    echo "  home         ${KWAAINET_HOME_DIR} $([ -d "${KWAAINET_HOME_DIR}" ] && echo '(present)' || echo '(MISSING)')"
    echo "  log dir      ${LOG_DIR} $([ -d "$LOG_DIR" ] && echo '(present)' || echo '(will be created)')"
    echo "  port         ${PORT}"
    exit 0
fi

mkdir -p "$UNIT_DIR" "$LOG_DIR"
render > "$UNIT_PATH"
chmod 644 "$UNIT_PATH"
echo "wrote ${UNIT_PATH}"

if ! command -v systemctl >/dev/null 2>&1; then
    echo "systemctl not found — the unit is written but not installed"
    exit 0
fi

systemctl --user daemon-reload
echo "reloaded the user manager"

if [ "${NO_ENABLE:-0}" -eq 1 ]; then
    echo "not enabling (--no-enable)"
    exit 0
fi

systemctl --user enable --now "${UNIT_NAME}"
echo "enabled and started ${UNIT_NAME}"

# Linger is what makes a user unit survive a reboot without a login
# session. Without it the node comes back only after someone logs in.
if command -v loginctl >/dev/null 2>&1; then
    loginctl enable-linger "$(id -un)" 2>/dev/null &&
        echo "linger enabled for $(id -un) (the unit will start at boot)" ||
        echo "could not enable linger — the unit will start only after a login"
fi

echo
echo "verify it with:"
echo "  systemctl --user status ${UNIT_NAME}"
echo "  curl -s http://127.0.0.1:${PORT}/v1/models"
