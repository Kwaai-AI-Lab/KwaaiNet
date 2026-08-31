#!/usr/bin/env bash
# Smoke gate for the D6 A/B: does the ontology change what extraction produces?
# 40 chunks per arm, sequential, progress JSON so it can be watched.
set -uo pipefail
cd "$(dirname "$0")"
K=/Users/rezarassool/Source/KwaaiNet/core/target/ci/kwaainet
N="${N:-40}"
# Resumable: an arm with a graph already at >= N entities is treated as done, so
# a killed run can be relaunched without redoing finished work.
PROG=results/smoke_ontology_progress.json
START=$(python3 -c 'import time;print(time.time())')
note(){ python3 - "$PROG" "$1" "$2" "$START" <<'PY'
import json,sys,time
p,arm,state,start=sys.argv[1:5]
json.dump({"arm":arm,"state":state,"elapsed_s":round(time.time()-float(start),1)},open(p,"w"),indent=2)
PY
}
for arm in ctl narr; do
  KB=D6_$arm
  if [ -f "results/${arm}.done" ]; then echo "── $KB already complete, skipping"; continue; fi
  note "$arm" building
  echo "── $KB ($N chunks)"
  $K rag graph build --kb "$KB" --model llama3.1:8b \
     --inference-url http://localhost:11434 --limit "$N" --workers 2 >"results/smoke_$arm.log" 2>&1
  $K rag graph stats --kb "$KB" 2>&1 | tail -2
  touch "results/${arm}.done"
done
note done done
echo SMOKE_DONE
