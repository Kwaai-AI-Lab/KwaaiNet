# Memory reflection — 2026-11-08

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exited 0.

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

Full output omitted here for length (135 lines); every warning is in
`projects/*/plans/*.md`, not in the twelve `CLAUDE.md` files or
`GLOSSARY.md`. Almost all are `(proposed?)` references to files that were
planned but never written (e.g. `d6_family_tree.yaml`, `ontology.yaml`) — the
checker itself flags these as plans describing future work, not broken
claims about the present. Zero broken references.

## Semantic — claims now false, with file, line, and the commit responsible

Nothing found. `git log --since="36 hours ago" --stat` returned no commits —
the last commit to the repo (`e98ca373`) landed 2026-11-04, four days before
this run. There is no code change in the window to check memory files
against.

Since the window was empty, I spot-checked the two risk patterns the task
description names explicitly, against current code rather than against a
recent commit:

- `projects/kwaai-knowledge/CLAUDE.md:150` — "Do not enable *unfiltered*
  relation extraction for 8B models" already credits the Phase-4 pipeline
  (`--relation-threshold-high`) as the fix, so it is not the stale pattern
  the task warned about (a prohibition that predates its own remedy). The
  flag exists: `core/crates/kwaai-cli/src/rag_cmd.rs:2843` takes
  `relation_threshold_high: f32`. Still accurate.
- `projects/kwaai-knowledge/CLAUDE.md:156` — "Do not use `cargo build -p
  kwaai-core`" — confirmed no crate named `kwaai-core` exists, and the
  `kwaainet` binary is still built via the `kwaai-cli` package
  (`core/crates/kwaai-cli/Cargo.toml`). Still accurate.

## Accretion — what could be removed and why

- **Hardcoded peer IDs for lab machines, duplicated in two files.**
  `projects/kwaai-knowledge/CLAUDE.md:103-105,111` and
  `projects/kwaai-compute/CLAUDE.md:32-34,36` both hard-code the same three
  `12D3Koo...` peer IDs for `metro-linux`, `metro-win`, and `jerome`. This is
  exactly the pattern Step 3 calls out (hardcoded peer IDs). I can't verify
  from the repo alone whether these machines still run with the same
  identity keys — that needs checking against the live DHT or asking
  whoever administers those boxes. Worth either confirming they're current
  or pointing both files at one source of truth instead of copy-pasting IDs
  that will silently drift when a machine is re-keyed.
- **Glossary term count may be off by one.** `projects/kwaai-knowledge/
  CLAUDE.md:161` says the glossary has "63 terms." Counting bold-lead
  definitions in `GLOSSARY.md` gives 64 (excluding one false match, "**Not**
  `CLAUDE.md`," which is prose, not a term). Low confidence — I'm not sure
  the original count used the same method I did (e.g. whether the two
  entries marked "(proposed)" were meant to count). Flagging rather than
  fixing.

Nothing else stood out as finished work still being described as current,
or as a plan superseded by a later one — but I did not read all 57 plan
files end to end, only the twelve `CLAUDE.md` files and the glossary
closely, plus the checker's structural pass over the plans. A plan-by-plan
"is this superseded" review of `projects/kwaai-knowledge/plans/` (the
largest and most plan-heavy directory, 57 files) would need its own pass.

## Nothing found

No broken references, no commits in the last 36 hours, and no semantic
contradiction introduced by a recent change.
