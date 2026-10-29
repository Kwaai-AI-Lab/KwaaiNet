#!/usr/bin/env bash
# Restore candle-flash-attn's compiled kernels from the newest unexpired
# artifact named $FLASH_ATTN_KERNELS, uploaded by cuda-kernel-cache.yml on one
# of the branches given as arguments.
#
#   fetch-flash-attn-kernels.sh main [other-branch...]
#
# An artifact, not the Actions cache: the repo's cache sits at its 10 GB limit
# and evicts within hours, and a cache saved from a tag is invisible to the
# next tag. Artifacts are kept 90 days and readable from any run.
#
# Never fails the job: no kernels just means a full nvcc compile. Writes
# FLASH_ATTN_KERNELS_RESTORED=true|false to $GITHUB_ENV.
# Needs: gh, GH_TOKEN (actions: read), GITHUB_REPOSITORY, FLASH_ATTN_KERNELS,
# CANDLE_FLASH_ATTN_BUILD_DIR (from setup-cuda-manylinux.sh).
set -uo pipefail

restored=false
finish() {
  echo "FLASH_ATTN_KERNELS_RESTORED=$restored" >> "$GITHUB_ENV"
  exit 0
}

workflow=.github/workflows/cuda-kernel-cache.yml

# The release links whatever this restores, so provenance matters more than
# the branch name. A fork PR's run reports its bare branch name — a fork's
# "main" looks like ours — and runs the PR's own copy of the workflow. So an
# artifact is only trusted when:
#   - its run is in this repository, not a fork (head repo == repo);
#   - the run is cuda-kernel-cache.yml, not another workflow;
#   - for main, the run wasn't a pull_request (push, schedule or dispatch).
trusted() { # <run id> <branch>
  local run
  run=$(gh api "repos/$GITHUB_REPOSITORY/actions/runs/$1" --jq '"\(.event)\t\(.path)"') || return 1
  local event=${run%%$'\t'*} path=${run#*$'\t'}
  [ "$path" = "$workflow" ] || return 1
  if [ "$2" = main ] && [ "$event" = pull_request ]; then return 1; fi
  return 0
}

# Newest first: "<run id>\t<branch>" per unexpired same-repo artifact.
# Branches are matched here in bash, not spliced into the jq filter.
if ! candidates=$(gh api "repos/$GITHUB_REPOSITORY/actions/artifacts?name=$FLASH_ATTN_KERNELS&per_page=100" \
    --jq '[.artifacts[] | select(.expired | not)
           | select(.workflow_run.head_repository_id == .workflow_run.repository_id)]
          | sort_by(.created_at) | reverse | .[]
          | "\(.workflow_run.id)\t\(.workflow_run.head_branch)"'); then
  echo "::warning::Could not list kernel artifacts (the token needs actions: read); building flash-attn from scratch."
  finish
fi
run_id=
while IFS=$'\t' read -r id branch; do
  for want in "$@"; do
    if [ "$branch" = "$want" ] && trusted "$id" "$branch"; then run_id=$id; break 2; fi
  done
done <<< "$candidates"
if [ -z "$run_id" ]; then
  echo "::notice::No artifact $FLASH_ATTN_KERNELS from branch(es) $*; building flash-attn from scratch."
  finish
fi

if ! gh run download "$run_id" -R "$GITHUB_REPOSITORY" -n "$FLASH_ATTN_KERNELS" -D "$CANDLE_FLASH_ATTN_BUILD_DIR"; then
  echo "::warning::Downloading $FLASH_ATTN_KERNELS from run $run_id failed; building flash-attn from scratch."
  # A partial download could leave the index next to a truncated object,
  # which cudaforge would reuse: it checks that objects exist, not their content.
  find "$CANDLE_FLASH_ATTN_BUILD_DIR" -mindepth 1 -delete
  finish
fi
# Without the cudaforge index every kernel recompiles, which would look like
# a restore that silently did nothing.
for f in .cudaforge_cache.json libflashattention.a; do
  if [ ! -s "$CANDLE_FLASH_ATTN_BUILD_DIR/$f" ]; then
    echo "::warning::Artifact from run $run_id has no $f; building flash-attn from scratch."
    find "$CANDLE_FLASH_ATTN_BUILD_DIR" -mindepth 1 -delete
    finish
  fi
done
echo "Restored $FLASH_ATTN_KERNELS from run $run_id ($(du -sh "$CANDLE_FLASH_ATTN_BUILD_DIR" | cut -f1))"
restored=true
finish
