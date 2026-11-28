#!/usr/bin/env bash
# handoff.sh — write a self-contained, REDACTED brief for whoever performs the move.
#
# The idea, and it is the same idea this repository already applies to code
# review: the context that WROTE a thing carries its assumptions into
# reading it. A person — or an agent — who did not build this node does not
# know which files are load-bearing, which of them are secrets, or which
# failure is silent. So the brief is written by the machine that HAS the
# context, and it travels with the move.
#
# It is deliberately safe to hand to someone else: every secret in it is
# recorded as a fingerprint, never a value. The brief tells you WHAT to
# transfer and HOW to prove you transferred the right thing.
#
# Usage:
#   scripts/portability/handoff.sh --out BRIEF.md
#   scripts/portability/handoff.sh --out BRIEF.md --repo PATH
#
# Read-only with respect to the system it describes.

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/portability/lib.sh
. "$SCRIPT_DIR/lib.sh"

OUT_FILE=""
REPO_PATH=""

usage() {
    cat <<'EOF'
usage: handoff.sh --out FILE [--repo PATH]

  --out FILE   write the brief to FILE (default: stdout)
  --repo PATH  the repository to describe
EOF
}

while [ $# -gt 0 ]; do
    case "$1" in
        --out) shift; OUT_FILE="${1:-}" ;;
        --repo) shift; REPO_PATH="${1:-}" ;;
        -h|--help) usage; exit 0 ;;
        *) echo "unknown argument: $1" >&2; usage >&2; exit 2 ;;
    esac
    shift
done

if [ -z "$REPO_PATH" ]; then
    REPO_PATH="$(p_repo_root "$SCRIPT_DIR")" || REPO_PATH="$(pwd)"
fi
# Absolutise: 'basename .' is '.', which is not a name worth putting in a
# brief that is meant to identify a checkout.
REPO_PATH="$(cd "$REPO_PATH" 2>/dev/null && pwd)" || REPO_PATH="$REPO_PATH"

INV="$(mktemp)"
TPL="${SCRIPT_DIR}/brief.template.md"
trap 'rm -f "$INV"' EXIT

if ! bash "$SCRIPT_DIR/inventory.sh" --json --repo "$REPO_PATH" > "$INV" 2>/dev/null; then
    p_fail "could not produce an inventory"
    exit 1
fi

if [ ! -f "$TPL" ]; then
    p_fail "missing template: $(p_home_path "$TPL")"
    exit 1
fi

# Pull one dotted key out of the flat JSON inventory.
#
# jq when it is there; a grep/sed fallback when it is not. The fallback
# exists because the brief must be generatable on a fresh machine that has
# not had anything installed on it yet — which is exactly the machine that
# needs the brief.
jq_get() {
    if command -v jq >/dev/null 2>&1; then
        jq -r "$1 // """ "$INV" 2>/dev/null
    else
        grep -F "\"$1\": \"" "$INV" 2>/dev/null | head -1 |
            sed -e 's/^[^:]*: *"//' -e 's/",$//'
    fi
}

# The units table, built from the inventory's service keys.
build_units_table() {
    grep -o '"services\.[^"]*\.enabled": "[^"]*"' "$INV" 2>/dev/null |
        sed -e 's/"services\.//' -e 's/\.enabled": "/|/' -e 's/"$//' |
        while IFS='|' read -r unit enabled; do
            [ -n "$unit" ] || continue
            unit="$(printf '%s' "$unit" | tr -d ' ')"
            enabled="$(printf '%s' "$enabled" | tr -d ' ')"
            active="$(jq_get "services.${unit}.active")"
            execp="$(jq_get "services.${unit}.exec")"
            printf '| %s | %s | %s | `%s` |\n' "$unit" "$enabled" "$active" "$execp"
        done
}

# Render the template. Python does the substitution because a value may
# contain any character at all, including the sed replacement metacharacters
# '|' and '&'.
#
# The two values that are NOT in the inventory — the generation timestamp
# and the repo name — are passed as arguments rather than merged into the
# inventory JSON. Merging two JSON documents by hand is where the first
# draft of this script lost the timestamp and rendered an empty string.
render() {
    if ! command -v python3 >/dev/null 2>&1; then
        p_warn "python3 not found — emitting the raw template"
        cat "$TPL"
        return
    fi
    python3 - "$TPL" "$INV" "$GENERATED_AT" "$REPO_NAME" <<'PYEOF'
import json
import re
import sys

template_path, inventory_path = sys.argv[1], sys.argv[2]
generated_at, repo_name = sys.argv[3], sys.argv[4]

with open(template_path) as fh:
    template = fh.read()

try:
    with open(inventory_path) as fh:
        inventory = json.load(fh)
except Exception:
    inventory = {}

def get(key, default=""):
    value = inventory.get(key, default)
    return default if value is None else str(value)

# The unit table is assembled from the flat service keys.
units = []
for key in sorted(inventory):
    if key.startswith("services.") and key.endswith(".enabled"):
        unit = key[len("services."):-len(".enabled")]
        units.append(
            "| {} | {} | {} | `{}` |".format(
                unit,
                get(key),
                get("services.{}.active".format(unit)),
                get("services.{}.exec".format(unit)),
            )
        )

values = {
    "GENERATED_AT": generated_at,
    "REPO_NAME": repo_name,
    "HOST": get("machine.host"),
    "OS": get("machine.os"),
    "ARCH": get("machine.arch"),
    "BRANCH": get("repo.branch"),
    "HEAD": get("repo.head"),
    "HOME_PATH": get("home.0.path"),
    "IDENTITY": get("home.0.identity"),
    "CONFIG": get("home.0.config"),
    "CONFIG_KEYS": get("home.0.config_keys"),
    "UNITS_TABLE": "\n".join(units),
    "HIVV_LIB": get("hivv.lib_dir"),
    "HIVV_SHARE": get("hivv.share_dir"),
    "HIVV_IDENTITY": get("hivv.identity_files"),
    "HIVV_ENV": get("hivv.env"),
    "HIVV_VHOST": get("hivv.vhost"),
    "HIVV_SHARD_API": get("hivv.shard_api"),
    "GH_KEY": get("github.ssh_key"),
    "GH_FINGERPRINT": get("github.ssh_pub_fingerprint"),
    "GH_LOGIN": get("github.gh_user"),
    "GH_SCOPES": get("github.gh_scopes"),
    "GH_NAME": get("github.user_name"),
    "GH_EMAIL": get("github.user_email"),
}

def substitute(match):
    return values.get(match.group(1), match.group(0))

sys.stdout.write(re.sub(r"\{\{([A-Z_]+)\}\}", substitute, template))
PYEOF
}

GENERATED_AT="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
REPO_NAME="$(basename "$REPO_PATH")"

render > "${INV}.md"

if [ -n "$OUT_FILE" ]; then
    cp "${INV}.md" "$OUT_FILE"
    p_ok "wrote $(p_home_path "$OUT_FILE")"
    p_dim "$(wc -l < "$OUT_FILE" | tr -d ' ') lines"
else
    cat "${INV}.md"
fi

rm -f "$INV" "${INV}.md"
