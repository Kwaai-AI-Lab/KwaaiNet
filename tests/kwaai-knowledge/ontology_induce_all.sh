#!/usr/bin/env bash
# Arm C sweep: induce an ontology draft for every KB with an eval question set.
# Writes progress JSON so the run can be monitored without stdout access.
set -uo pipefail
cd "$(dirname "$0")"
PROG="results/ontology_induce_progress.json"
KBS=$(find . -name eval_questions.json -mindepth 2 -maxdepth 2 | sed 's|\./||; s|/eval_questions.json||' | sort)
TOTAL=$(echo "$KBS" | wc -w | tr -d ' '); DONE=0
START=$(python3 -c 'import time;print(time.time())')
for kb in $KBS; do
  OUT=$(python3 ontology_induce.py "$kb" 2>&1 | tail -1)
  DONE=$((DONE+1))
  python3 - "$PROG" "$DONE" "$TOTAL" "$kb" "$START" "$OUT" <<'PY'
import json,sys,time
p,d,t,kb,start,out=sys.argv[1:7]
d,t,start=int(d),int(t),float(start)
el=time.time()-start
json.dump({"done":d,"total":t,"last_kb":kb,"last_result":out,
           "elapsed_s":round(el,1),
           "eta_s":round(el/d*(t-d),1) if d else None}, open(p,"w"), indent=2)
PY
  echo "[$DONE/$TOTAL] $OUT"
done
echo "ALL DONE"
