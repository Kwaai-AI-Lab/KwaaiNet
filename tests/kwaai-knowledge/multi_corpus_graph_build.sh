#!/usr/bin/env bash
# Graph build for all corpus KBs — run sequentially to avoid GPU overload.
# Each build uses all available GPU relay machines.
#
# Usage:
#   ./multi_corpus_graph_build.sh                  # build all
#   ./multi_corpus_graph_build.sh Manhattan Legal  # specific corpora only

set -euo pipefail

P2P_URLS="p2p://12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs,p2p://12D3KooWLMizEbViSoL4WGJUMsLVRyLccyymosX36MDKdbYgGFzE,p2p://12D3KooWDyPJBavUudh6dWitszGL2FSrEgy32SJY5qiSrATapGgd"

# Entity types per KB — tuned to each domain
declare -A KB_ENTITY_TYPES=(
  [Manhattan]="Person,Place,Organization"
  [MobyDick]="Person,Place,Organization"
  [Legal]="Person,Organization,Legislation"
  [Meetings]="Person,Organization"
  [PythonDocs]="Organization,Publication"
  [NIST]="Organization,Legislation,Publication"
  [Climate]="Organization,Legislation,Publication"
  [RFCs]="Organization,Publication"
  [DeepSea]="Person,Organization,Publication"
  [DreamMem]="Person,Organization,Publication"
  [Astrophysics]="Person,Organization,Publication"
  [CountryHistory]="Person,Place,Organization"
  [WarPeace]="Person,Place,Organization"
  [Poems]="Person,Place"
  [OSMDocs]="Organization"
)

if [[ $# -gt 0 ]]; then
  TARGET_KBS=("$@")
else
  TARGET_KBS=("${!KB_ENTITY_TYPES[@]}")
fi

for KB in "${TARGET_KBS[@]}"; do
  ET="${KB_ENTITY_TYPES[$KB]:-Person,Place,Organization}"
  echo "▶ Graph build: $KB (entity-types=$ET) …"
  kwaainet rag graph build --kb "$KB" \
    --model llama3.1:8b \
    --inference-urls "$P2P_URLS" \
    --workers 4 \
    --entity-types "$ET" \
    --no-relations \
    --graph-window 1 \
    --timeline \
    2>&1 | tee "/tmp/graph_build_${KB}.log" | tail -5
  kwaainet rag graph score --kb "$KB" 2>&1 | tail -3
  echo "✅ $KB graph built"
done
