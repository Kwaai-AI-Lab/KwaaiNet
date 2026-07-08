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

KB="${1:-D6}"
FAIL=0

# Send one query then exit, capture only stdout (stderr to /dev/null so
# progress lines don't appear in terminal during the test).
STDOUT=$(echo -e "Who is the author?\nexit" \
  | kwaainet rag chat --kb "$KB" --local 2>/dev/null)

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

exit $FAIL
