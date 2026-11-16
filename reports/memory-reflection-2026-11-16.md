# Memory Reflection — 2026-11-16

Scope: the 12 `CLAUDE.md` files plus `projects/kwaai-knowledge/GLOSSARY.md`, checked against
`git log --since="36 hours ago"` on the default branch (HEAD `c7dbb0d`, confirmed equal to
`origin/main` after `git fetch`).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
```
… 135 warning(s) across 57 files, **0 broken reference(s)**. Every warning is inside
`projects/kwaai-knowledge/plans/*.md` (proposed files/functions that were never built, or
`file.rs:N` line numbers the named symbol has since moved off of). None of the 135 fall in the
12 `CLAUDE.md` files or `GLOSSARY.md`. Full output was reviewed in the session; omitted here
for length since every line is a `plans/` warning of the two kinds above.

## Semantic — claims now false, with file, line, and the commit responsible

Seven commits landed in the last 36 hours, all in `kwaai-network`/`kwaai-compute`/`kwaai-platform`
territory: #302 (chunked prompt send), #299 (control-stream reconnect + shard-api error reporting),
#298 (oversize block reply no longer deregisters the shard), #292 (`head_dim` from `config.json`),
#289 (whole-model node announce fix), #273 (one node per `KWAAINET_HOME`, `--concurrent` removed),
#272 (Windows token temp file made owner-only).

Checked each against `projects/kwaai-network/CLAUDE.md`, `projects/kwaai-compute/CLAUDE.md`,
`projects/kwaai-platform/CLAUDE.md`, `core/crates/kwaai-inference/CLAUDE.md`,
`core/crates/kwaai-trust/CLAUDE.md`, `core/crates/kwaai-p2p/CLAUDE.md`, and the root `CLAUDE.md`
— grepping for `--concurrent`, lock/acquire semantics, token-file permissions, `head_dim`,
control-stream/reconnect, and chunking. **Nothing found.** None of these docs describe
implementation detail at the level these fixes touch (chunk sizes, lock FD handling, error
propagation shape, token file ACLs), so none of the seven commits contradicts a standing claim.

## Accretion — what could be removed and why

- **Stale line pointer, not from this window.** `projects/kwaai-compute/CLAUDE.md:88` and
  `core/crates/kwaai-inference/CLAUDE.md:20` both say "RoPE fix at `shard.rs:104`." The
  `broadcast_mul` fix is now at `core/crates/kwaai-inference/src/shard.rs:134`; line 104 is
  unrelated RoPE-table setup code. `git blame` traces the surrounding code to `49014ee`
  (2026-09-17), well before this reflection's 36-hour window, so this didn't happen on my
  watch — but it's live drift worth a one-line fix in both files.
- **`projects/kwaai-knowledge/plans/` (27 files) carries a large amount of unresolved
  speculative content** — dozens of "proposed" file/function references across plans like
  `DreamRAG-Eval3-plan.md`, `Phase5-TrainedClassifiers-plan.md`, `PublicRelease-plan.md`, and
  several `D6-*`/`OntologySelection`/`PerKBOntology` plans that all reference the same
  not-yet-existing `d6_family_tree.yaml` / `ontologies/D6.yaml`. This may mean several plans
  were superseded by a later one (the repeated unresolved filenames suggest convergent, possibly
  abandoned, proposals) — but I did not read each plan end-to-end to confirm which, if any, is
  dead versus simply not-yet-implemented. That would need a dedicated pass per plan (checking
  each against current `kwaai-rag` code and any successor plan that cites it) rather than a
  36-hour diff review, so I'm flagging the pattern, not asserting specific files are dead.
- Everything else checked (metro machine peer IDs/GPU inventory in
  `projects/kwaai-compute/CLAUDE.md`, `projects/kwaai-network/CLAUDE.md`; the
  `~/.claude/plans/cached-jingling-creek.md` reference in `projects/kwaai-compute/CLAUDE.md:75`)
  is infrastructure/local-path content I have no way to verify currency of from this checkout —
  noting that rather than guessing.

## Nothing found

No broken references, and no commit in the last 36 hours makes any of the 12 `CLAUDE.md` files
or `GLOSSARY.md` assert something now untrue.
