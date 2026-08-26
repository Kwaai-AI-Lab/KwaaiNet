#!/usr/bin/env bash
# Re-run the three KBs that failed the arm C sweep. Two attempts each: the
# failures were unparseable prose (Manhattan), an HTML-fragment corpus
# (PythonDocs), and a total grounding failure (Legal) — all plausibly transient
# given induction is non-deterministic.
set -uo pipefail
cd "$(dirname "$0")"
for kb in Legal Manhattan PythonDocs; do
  for attempt in 1 2; do
    echo "--- $kb attempt $attempt"
    if python3 ontology_induce.py "$kb" 2>&1 | tail -1; then break; fi
  done
done
echo "RETRY DONE"
