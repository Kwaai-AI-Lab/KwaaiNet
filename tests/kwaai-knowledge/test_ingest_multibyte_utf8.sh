#!/usr/bin/env bash
# Regression test: ingest must not panic on documents whose multi-byte UTF-8
# characters (curly quotes, em dashes, etc.) straddle the 4000-byte preview
# boundary used by auto-detect-schema.
#
# Before the fix (rag_cmd.rs:603), ingesting a file with a U+2019 RIGHT SINGLE
# QUOTATION MARK at byte offset ~3998-4001 caused:
#   thread 'main' panicked at: byte index 4000 is not a char boundary
#
# The fix uses floor_char_boundary() to snap to the nearest valid boundary.

set -euo pipefail

KB="__test_utf8_$$"
TMP_FILE="$(mktemp /tmp/test_utf8_XXXXX.txt)"

cleanup() {
    kwaainet rag destroy --kb "$KB" --yes 2>/dev/null || true
    rm -f "$TMP_FILE"
}
trap cleanup EXIT

# Build a file whose curly-quote lands exactly at byte 4000.
# ASCII filler (1 byte each) × 3998 bytes, then a 3-byte UTF-8 curly quote,
# then more text — total length straddles 4000.
python3 -c "
import sys
body = 'x' * 3998 + '’' + ' tail text ' * 50
sys.stdout.buffer.write(body.encode('utf-8'))
" > "$TMP_FILE"

kwaainet rag init --kb "$KB" --embed-model nomic-embed-text 2>/dev/null

# This must NOT panic (exit code 101 before fix).
if kwaainet rag ingest --kb "$KB" "$TMP_FILE" 2>&1 | grep -q "is not a char boundary"; then
    echo "FAIL: char boundary panic still present"
    exit 1
fi

echo "PASS: multi-byte UTF-8 at byte 4000 boundary ingested without panic"
