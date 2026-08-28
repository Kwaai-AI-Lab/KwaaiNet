#!/usr/bin/env bash
# Regression: the interactive relevance-marking prompt must never run when stdin is
# not a terminal.
#
# The prompt reads a line. If it runs under a pipe it consumes the *next* scripted
# question, so N piped questions yield fewer than N answers — or the session hangs
# waiting on input that will never come. Piping two questions and counting the answers
# is the direct test for the is_stdin_tty() gate.
#
# Usage: ./test_chat_piped_no_prompt.sh [KB]
set -euo pipefail
cd "$(dirname "$0")"
. ./_chat_test_lib.sh

KB="${1:-D6}"
BIN="$(resolve_bin)"
FAIL=0
OUT=$(mktemp); ERR=$(mktemp)
trap 'rm -f "$OUT" "$ERR"' EXIT

run_capped 300 "$OUT" "$ERR" $'Who is the author?\nWhere was he born?\nexit\n' -- \
  "$BIN" rag chat --kb "$KB" --local || {
    echo "FAIL: chat hung on piped stdin (the prompt likely consumed a question)"; exit 1; }

N=$(grep -c 'Assistant:' "$OUT" || true)
if [ "$N" -eq 2 ]; then
  pass "two piped questions produced exactly two answers"
else
  fail "expected 2 answers, got $N — the prompt is eating piped input"
  cat "$OUT"
fi

# The mark prompt and its help must never appear at all under a pipe.
if grep -qE '⟩|mark relevance' "$OUT" "$ERR"; then
  fail "interactive prompt rendered despite non-TTY stdin"
else
  pass "no interactive prompt under a pipe"
fi

# Regression: EOF must end the session, not spin on it.
#
# read_line reports EOF as Ok(0), not Err. The original guard tested only is_err(),
# so a pipe with no trailing `exit` looped on an empty line at full CPU forever.
# Every other case here ends its input with `exit`, which is exactly why none of
# them caught it.
OUT2=$(mktemp); ERR2=$(mktemp)
trap 'rm -f "$OUT" "$ERR" "$OUT2" "$ERR2"' EXIT

if run_capped 300 "$OUT2" "$ERR2" $'Who is the author?\n' -- \
     "$BIN" rag chat --kb "$KB" --local; then
  pass "EOF with no trailing exit ends the session"
else
  rc=$?
  if [ "$rc" -eq 124 ]; then
    fail "chat spun on EOF instead of exiting (killed at the cap)"
  else
    fail "chat exited $rc on EOF"
  fi
fi

M=$(grep -c 'Assistant:' "$OUT2" || true)
if [ "$M" -eq 1 ]; then
  pass "the question before EOF was still answered"
else
  fail "expected 1 answer before EOF, got $M"
fi

exit $FAIL
