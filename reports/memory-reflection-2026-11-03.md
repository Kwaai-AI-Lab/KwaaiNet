# Memory Reflection — 2026-11-03

Checked out `main` at `ee626f3f0c4e41822e5877ce2b8cfd5ffd8fb32c` (2026-10-29 20:02:35 -0700).
This is the first run of this reflection; no prior `reports/memory-reflection-*.md` or
`memory-reflection/*` branch exists to compare against.

## Mechanical — checker output, verbatim

```
Checking 56 memory and plan files
...
0 broken reference(s), 134 warning(s) across 56 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 134 warnings are in `projects/kwaai-knowledge/plans/*.md` and
`projects/kwaai-trust/plans/*.md`, and are the checker's "(proposed?)" / "moved?" class —
plan documents naming files that don't exist yet, or naming a function/line that has since
moved within the same file. None of the warnings land in the twelve `CLAUDE.md` files or in
`GLOSSARY.md`, so nothing here is contaminating what every session loads by default. Full
verbatim output was 56-file long; truncated above for brevity, available by re-running
`python3 scripts/check_memory_integrity.py`.

Separately verified the root `CLAUDE.md` project-map table: all 13 listed crate directories
under `core/crates/` exist as named.

## Semantic — claims now false, with file, line, and the commit responsible

**Nothing found.** `git log --since="36 hours ago" --stat` returned no commits — the most
recent commit on `main` is `ee626f3` from 4 days ago (2026-10-29). There is no code change
in the review window for a memory file to have fallen out of sync with. The 7-day window
has 13 commits; if the intent was a 7-day (not 36-hour) look-back, say so and I'll run that
pass separately — I did not expand scope on my own since the task fixed the window.

## Accretion — what could be removed and why

**Hardcoded peer IDs, unverified (flagging, not confirming stale).** Two files hardcode the
same three dev-machine peer IDs as literal examples:

- `projects/kwaai-compute/CLAUDE.md:32-36` — `metro-linux`, `metro-win`, `jerome`
- `projects/kwaai-knowledge/CLAUDE.md:39,103-111` — same three, duplicated

This is exactly the accretion shape called out in the task brief (hardcoded peer IDs tied to
specific machines). I have no way from this container to confirm whether `metro-linux`,
`metro-win`, and `jerome` are still live nodes on the network, so I am not reporting them as
stale — only as a standing risk: if any of these three machines has been retired or
re-keyed, both copies go wrong silently and in sync (same error, two places). To actually
confirm, someone would need to either ping those peer IDs on the live network or ask whoever
maintains the metro-linux/metro-win/jerome boxes whether they're still running. Worth
deduplicating to one source of truth regardless of whether the values are currently correct.

No other candidates met the bar this run: no absolute user/home paths found in any `CLAUDE.md`
or `GLOSSARY.md`, and the "Current state" sections in `kwaai-compute`, `kwaai-network`,
`kwaai-storage`, and `kwaai-trust` CLAUDE.md describe shipped functionality I had no changed
code to check them against this run (see Semantic section — nothing landed in 36h to test
them against). I'm not confident enough to call any of those stale or current on this pass.

## Nothing found

Semantic section is clean — no commits in the lookback window. Mechanical section is clean —
0 broken references in any of the twelve `CLAUDE.md` files or `GLOSSARY.md`.
