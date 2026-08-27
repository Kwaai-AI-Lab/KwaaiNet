#!/usr/bin/env bash
set -uo pipefail
cd "$(dirname "$0")"
K=/private/tmp/claude-501/-Users-rezarassool-Source-KwaaiNet-tests-kwaai-knowledge/78a22fa9-8e6c-421a-aed2-e802084d04bf/scratchpad/pr-wt/core/target/release/kwaainet
RELAY="p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE"
S=$(date +%Y%m%d_%H%M%S)
for arm in ctl narr; do
  echo "── D6_$arm eval"
  $K rag eval --kb "D6_$arm" --questions d6_eval_questions.json \
     --model llama3.1:8b --inference-url "$RELAY" \
     > "results/eval2_${arm}_$S.md" 2>&1
  grep -oE "Overall recall \(token-overlap\) \| [0-9.]+%.*" "results/eval2_${arm}_$S.md" | tail -1
done
echo "EVALS_DONE $S"
