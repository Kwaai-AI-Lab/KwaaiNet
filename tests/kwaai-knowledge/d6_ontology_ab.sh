#!/usr/bin/env bash
# D6 A/B: same chunks, same model, same settings — only the ontology differs.
#
#   D6_ctl  : today's global vocabulary (17 entity types / 35 predicates)
#   D6_narr : the authored narrative-history ontology (9 types / 27 predicates)
#
# Both are copies of D6's chunk store, so D6's own 340-relation graph is never
# touched. Local Ollama for both arms — this is measurement, and a flaky peer
# would confound the comparison.
set -uo pipefail
cd "$(dirname "$0")"
K=/Users/rezarassool/Source/KwaaiNet/core/target/ci/kwaainet
LIMIT="${LIMIT:-150}"
MODEL=llama3.1:8b
URL=http://localhost:11434
PROG=results/d6_ontology_ab_progress.json
START=$(python3 -c 'import time;print(time.time())')

note () {
  python3 - "$PROG" "$1" "$2" "$START" <<'PY'
import json,sys,time
p,arm,state,start=sys.argv[1:5]
json.dump({"arm":arm,"state":state,"elapsed_s":round(time.time()-float(start),1)},
          open(p,"w"), indent=2)
PY
}

for arm in ctl narr; do
  if [ "$arm" = ctl ]; then KB=D6_ctl; EXTRA=(); else
    KB=D6_narr
    EXTRA=(--entity-types "$(cat /tmp/d6_narr_ents.txt)"
           --relation-types "$(cat /tmp/d6_narr_rels.txt)")
  fi
  echo "=== arm $arm ($KB), limit $LIMIT ==="
  note "$arm" building
  $K rag graph clear --kb "$KB" --yes >/dev/null 2>&1
  $K rag graph build --kb "$KB" --model "$MODEL" --inference-url "$URL" \
      --limit "$LIMIT" --workers 2 "${EXTRA[@]}" 2>&1 | tail -25
  note "$arm" done
  $K rag graph stats --kb "$KB" 2>&1 | tail -2
done
echo "AB DONE"
