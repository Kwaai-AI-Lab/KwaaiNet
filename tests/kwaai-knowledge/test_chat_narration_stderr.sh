#!/usr/bin/env bash
# Mirror-image of test_chat_stdout_clean.sh.
#
# That test asserts retrieval narration never reaches stdout. On its own, the cheapest
# way to satisfy it is to delete the narration entirely — which would silently remove
# the whole point of the chat UX. This test asserts the narration *is* present on
# stderr, so the pair pins the split in both directions.
#
# Usage: ./test_chat_narration_stderr.sh [KB]
set -euo pipefail
cd "$(dirname "$0")"
. ./_chat_test_lib.sh

KB="${1:-D6}"
BIN="$(resolve_bin)"
FAIL=0
OUT=$(mktemp); ERR=$(mktemp)
trap 'rm -f "$OUT" "$ERR"' EXIT

run_capped 240 "$OUT" "$ERR" $'Who is the author?\nexit\n' -- \
  "$BIN" rag chat --kb "$KB" --local || {
    echo "FAIL: chat did not terminate within the deadline"; exit 1; }

if grep -qE '○ Round|○ Final' "$ERR"; then
  pass "retrieval round markers present on stderr"
else
  fail "no retrieval narration on stderr — the chatty layer is missing"
  head -20 "$ERR"
fi

if grep -q "Here's what we know" "$ERR"; then
  pass "findings narration present on stderr"
else
  fail "no findings narration on stderr"
fi

if grep -qE '┌|│' "$ERR"; then
  pass "findings table rendered on stderr"
else
  fail "no findings table on stderr"
fi

exit $FAIL
