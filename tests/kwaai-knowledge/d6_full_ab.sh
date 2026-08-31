#!/usr/bin/env bash
# Full D6 rebuild, both arms, then eval. ~4.5 h per arm on the p2p GPU relay.
#
#   D6_ctl  — no ontology: the global 17 entity types / 35 predicates
#   D6_narr — D6 ontology v8: 17 types, 43 predicates, 223 triggers, 5 axioms
#
# Identical everything else. Temperature pinned to 0 and a single worker so the
# arms differ only in vocabulary — three earlier runs varied by ~22% under
# sampling + worker interleaving, which was more than the effect being measured.
#
# Resumable: an arm that finished leaves a .done marker.
set -uo pipefail
cd "$(dirname "$0")"
K=/private/tmp/claude-501/-Users-rezarassool-Source-KwaaiNet-tests-kwaai-knowledge/78a22fa9-8e6c-421a-aed2-e802084d04bf/scratchpad/pr-wt/core/target/release/kwaainet
RELAY="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
Q=d6_eval_questions.json
STAMP=$(date +%Y%m%d_%H%M%S)
PROG=results/d6_full_ab_progress.json
export KWAAI_EXTRACTION_TEMPERATURE=0
START=$(python3 -c 'import time;print(time.time())')

note(){ python3 - "$PROG" "$1" "$2" "$START" <<'PY'
import json,sys,time
p,arm,state,s=sys.argv[1:5]
json.dump({"arm":arm,"state":state,"elapsed_h":round((time.time()-float(s))/3600,2)},open(p,"w"),indent=1)
PY
}

for arm in ctl narr; do
  KB=D6_$arm
  if [ -f "results/full_${arm}.done" ]; then echo "── $KB done, skipping"; continue; fi
  note "$arm" building
  echo "── $KB: graph build (1152 chunks)"
  $K rag graph clear --kb "$KB" --yes >/dev/null 2>&1     # ontology preserved
  $K rag graph build --kb "$KB" --model llama3.1:8b --inference-urls "$RELAY" \
     --workers 1 --graph-window 1 >"results/full_${arm}_build.log" 2>&1
  $K rag graph stats --kb "$KB" 2>&1 | tail -2
  touch "results/full_${arm}.done"
done

for arm in ctl narr; do
  KB=D6_$arm
  echo "── $KB: eval (40 questions)"
  $K rag eval --kb "$KB" --questions "$Q" --model llama3.1:8b \
     --inference-url "$RELAY" >"results/full_${arm}_eval_$STAMP.md" 2>&1
  grep -iE "overall|recall" "results/full_${arm}_eval_$STAMP.md" | tail -2
done
note done done
echo "FULL_AB_DONE $STAMP"
