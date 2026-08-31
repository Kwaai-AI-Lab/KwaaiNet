#!/usr/bin/env bash
# Regression test: kwaainet rag chat must not emit retrieval progress (○ Round,
# ○ Coverage, ○ Done) on stdout. These lines go to stderr only; stdout is
# reserved for the chat interface ("You:" / "Assistant:") so that piping or
# redirecting stdout never feeds progress lines back into stdin.
#
# Bug: rag chat would read its own stdout progress lines as user input,
# creating an infinite query loop. Fix: on_status closure in cmd_chat uses
# eprintln! not println!.
#
# Usage:
#   ./test_chat_stdout_clean.sh [--kb D6]
#
# Requires: kwaainet binary, D6 knowledge base already built.

set -euo pipefail
cd "$(dirname "$0")"
. ./_chat_test_lib.sh

KB="${1:-D6}"
BIN="$(resolve_bin)"
FAIL=0

# Send one query then exit, capture only stdout (stderr discarded so progress lines
# don't appear in the terminal during the test). The deadline matters: the regression
# this guards against can present as a hang rather than as bad output.
OUT=$(mktemp); ERR=$(mktemp)
trap 'rm -f "$OUT" "$ERR"' EXIT
run_capped 240 "$OUT" "$ERR" $'Who is the author?\nexit\n' -- \
  "$BIN" rag chat --kb "$KB" --local || {
    echo "FAIL: chat did not terminate within the deadline"; exit 1; }
STDOUT=$(cat "$OUT")

# Check: no retrieval round markers on stdout
if echo "$STDOUT" | grep -qE '○ Round|○ Coverage|○ Done|○ Gap|○ No new'; then
  echo "FAIL: retrieval progress lines found on stdout:"
  echo "$STDOUT" | grep -E '○ Round|○ Coverage|○ Done|○ Gap|○ No new'
  FAIL=1
else
  echo "PASS: no retrieval progress lines on stdout"
fi

# Check: assistant response IS on stdout
if ! echo "$STDOUT" | grep -q "Assistant:"; then
  echo "FAIL: no Assistant: line found on stdout (chat may have failed to run)"
  FAIL=1
else
  echo "PASS: Assistant response present on stdout"
fi

# The chatty layer added on top of the original invariant must also stay off stdout:
# the findings table, the "Here's what we know" narration, and the relevance prompt.
if echo "$STDOUT" | grep -qE "Here's what we know|as evidenced by|⟩|┌|├|└"; then
  echo "FAIL: chat UX narration leaked onto stdout:"
  echo "$STDOUT" | grep -nE "Here's what we know|as evidenced by|⟩|┌|├|└"
  FAIL=1
else
  echo "PASS: no findings table or mark prompt on stdout"
fi

# Piped stdout must carry no ANSI either — it is consumed by scripts, not a terminal.
if printf '%s' "$STDOUT" | LC_ALL=C grep -q "$(printf '\033')"; then
  echo "FAIL: ANSI escape sequences on piped stdout"
  FAIL=1
else
  echo "PASS: no ANSI escapes on piped stdout"
fi

exit $FAIL
