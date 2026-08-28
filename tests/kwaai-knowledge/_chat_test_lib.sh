#!/usr/bin/env bash
# Shared helpers for the rag chat regression tests.
#
# macOS ships no coreutils `timeout`, and every one of these tests needs a hard
# deadline: the failure mode they guard against (an interactive prompt eating piped
# input) manifests as a hang, which without a deadline stalls CI instead of failing it.

# run_capped <seconds> <stdout-file> <stderr-file> <input> -- <cmd...>
run_capped() {
  local secs="$1" out="$2" err="$3" input="$4"; shift 5   # shift past the "--"
  printf '%s' "$input" | "$@" >"$out" 2>"$err" &
  local pid=$!
  local waited=0
  while kill -0 "$pid" 2>/dev/null; do
    if [ "$waited" -ge "$secs" ]; then
      kill -9 "$pid" 2>/dev/null
      wait "$pid" 2>/dev/null
      return 124
    fi
    sleep 1
    waited=$((waited + 1))
  done
  wait "$pid"
}

# Resolve the binary under test: KWAAINET_BIN, else a local debug build, else PATH.
resolve_bin() {
  if [ -n "${KWAAINET_BIN:-}" ]; then echo "$KWAAINET_BIN"; return; fi
  local dbg="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../core" && pwd)/target/debug/kwaainet"
  if [ -x "$dbg" ]; then echo "$dbg"; else echo "kwaainet"; fi
}

pass() { echo "PASS: $1"; }
fail() { echo "FAIL: $1"; FAIL=1; }
