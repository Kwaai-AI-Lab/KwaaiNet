# Memory Reflection — 2026-09-20

Reviewed at `19f288d405d07c4f4b16e87aaf3383a7558ad4dc` (origin/main).
Lookback: `git log --since="36 hours ago"` → 2 commits (b9a7343, 19f288d).

## Mechanical — checker output, verbatim

```
Checking 48 memory and plan files

  [45 warnings across 48 files — full list omitted here, all "file/function
  reference does not resolve (proposed?)" inside plan documents describing
  future work, plus three "moved" suggestions in plan docs]

0 broken reference(s), 45 warning(s) across 48 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still
says something true, then touch it or fix it.
```

Exit code 0. All 45 warnings are in `projects/*/plans/*.md` files and are
either explicitly proposed/future paths (`ontology.yaml`, `gap_analysis.py`,
etc.) or references to files in the private PHE repo. None indicate a broken
claim in an active `CLAUDE.md`.

## Semantic — claims now false, with file, line, and the commit responsible

**b9a7343** (drop the Petals bridge roadmap and its bootstrap example): the
commit's own diff deletes `core/docs/PETALS_BRIDGE_ROADMAP.md` and
`core/examples/petals_bootstrap.rs`. Grepped all 12 `CLAUDE.md` files, the
GLOSSARY, and every `plans/*.md` for `PETALS_BRIDGE_ROADMAP` and
`petals_bootstrap` — zero hits. No dangling references were left behind.

**19f288d** (`feat(cli): start flags are ephemeral...`, #228): this rewrites
how `StartOverrides` is persisted (new `run/start-args.json` written by
`daemon.rs::write_start_args`, read by `read_start_args`). Grepped all 12
`CLAUDE.md` files for `config.yaml`, `start --daemon`, `restart`,
`grpc_port`, `shard.backend`, `StartOverrides`, `run-node`, `config set`. One
real mismatch found, no others:

- `projects/kwaai-platform/CLAUDE.md`, "Config fields pattern" section
  (~line 78) states the pattern for a serialized config struct field:
  `#[serde(default)] pub field: Option<T>` + `skip_serializing_if`. The new
  `StartOverrides` struct (`core/crates/kwaai-cli/src/cli.rs:191`) is now an
  on-disk format under that same pattern's remit, but four of its fields —
  `no_gpu`, `no_relay`, `shard`, `no_contribute` (`cli.rs:211,233,238,243`) —
  are plain `bool` without `#[serde(default)]`, which serde will fail to
  deserialize once any old on-disk record lacks that field. Verified by
  reading the struct directly, not inferred.

  This is the same issue 19f288d's own commit message documents as its
  finding #1 ("StartOverrides is now an on-disk format but derives
  Deserialize without `#[serde(default)]`... adding any future boolean flag
  makes every record written by this release fail to parse"), already
  flagged there as an open, non-blocking follow-up. I'm not raising it as a
  new discovery — the commit message already tracks the code-level bug — but
  the *doc* consequence is new and worth stating plainly: as of this commit,
  `projects/kwaai-platform/CLAUDE.md`'s "Config fields pattern" is not
  actually followed by the newest on-disk struct in the crate it governs.

No CLI subcommand was renamed or deleted in either commit. No documented
default/threshold/cap value was changed. No "do not do X" guidance in any
`CLAUDE.md` was invalidated by new machinery in this window.

## Accretion — what could be removed and why

- `projects/kwaai-knowledge/CLAUDE.md` (P2P relay inference table) still
  hardcodes three p2p peer IDs (metro-linux, metro-win, jerome). This is not
  a new finding — it's already named as open in
  `projects/kwaai-knowledge/plans/MemoryIntegrity-growth-cycle.md` under
  "Still open" — and remains unaddressed. I did not attempt to verify
  whether these peer IDs are still live; that needs a `kwaainet p2p ping`
  against a running node, which this reflection has no access to run.
- Checked two plan docs whose file references the checker flagged as
  unresolved (`projects/kwaai-storage/plans/VPK-CrateIntegration-plan.md`,
  `projects/kwaai-knowledge/plans/AutoDeriveSeededFacts-plan.md`) for
  superseded/finished status. `VPK-CrateIntegration-plan.md` is marked
  "Status: draft for review" and depends on the private PHE repo, matching
  the known false-positive class the checker already special-cases
  elsewhere — not stale. `AutoDeriveSeededFacts-plan.md` has no explicit
  status marker and its "proposed" references are future work items, not
  evidence of completion or supersession — I found nothing to justify
  flagging it and am not speculating further.

## Nothing found

No broken references (mechanical). No inverted behavior, no renamed/deleted
subcommand, and no stale default/threshold value from either of the two
commits in the last 36 hours, beyond the one config-pattern mismatch noted
above.
