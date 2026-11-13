# Memory reflection — 2026-11-13

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
... (135 warnings, listed in full below)
0 broken reference(s), 135 warning(s) across 57 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 135 warnings fall in `projects/*/plans/*.md` — proposed/future
file paths not yet created, or symbol line-numbers that drifted a few hundred lines from
refactoring (e.g. `config.rs:1059` vs. actual `1116`). None land in the twelve `CLAUDE.md`
files or `GLOSSARY.md`, which the checker left clean.

Full output saved for reference: 57 files scanned, warnings concentrated in
`projects/kwaai-knowledge/plans/` (expected — these plans cite D6 corpus files that are
gitignored/private and schema files proposed but not yet landed).

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returns **zero commits**. The last commit to the
repo (`e98ca37`, "test(rpc): compare the token file's ACE by SID, not by SDDL string",
#274) landed 2026-11-04, nine days ago. There is nothing in the last 36 hours for this
step to check against.

One item outside that window, surfaced by the checker and worth recording since I
verified it while here: `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`
says

> **Status**: code complete, not yet committed.

about the `redb` → `rusqlite`/WAL migration. That migration has landed: `kwaai-rag`'s
`Cargo.toml` depends on `rusqlite 0.31`, and `meta_store.rs`/`graph.rs` now only reference
`redb` in legacy-migration detection code (`if ...exists() { warn "Legacy redb store
detected... run kwaainet rag rebuild to migrate" }`). This predates the 36-hour window —
I can't name the landing commit without a wider log search I didn't run — but the plan's
own status line is now stale and should say the migration shipped.

## Accretion — what could be removed and why

- **Hardcoded peer-ID table duplicated verbatim in two files**:
  `projects/kwaai-compute/CLAUDE.md:32-36` and `projects/kwaai-knowledge/CLAUDE.md:101-111`
  both hardcode the same three peer IDs (metro-linux, metro-win, jerome) with GPU labels.
  Any rotation of one machine's identity means editing two files with no cross-check
  between them — exactly the accretion risk called out in the background (hardcoded peer
  IDs/paths). Candidate fix: one canonical infra table (e.g. in the root `CLAUDE.md` or a
  shared infra doc) that both project files link to.

- **Possible drift between that table and actual machine status — unverified, flagging
  rather than asserting**: `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md`
  (2026-07-12) records jerome as "❌ Offline — 148+ consecutive routing failures; excluded
  from pipeline" and metro-linux as "A6000 48GB", while both `CLAUDE.md` tables above list
  jerome without any offline caveat and give metro-linux as "A5000 24GB". I have no way to
  check current peer reachability or hardware from this container, so I don't know which
  side is current — only that the two disagree and nothing in the repo reconciles them. To
  resolve: run `kwaainet p2p` connectivity check against jerome's peer ID, and confirm
  metro-linux's actual GPU with whoever operates it.

## Nothing found

Nothing else. No renamed/deleted CLI subcommands, no default/threshold/flag drift, and
no inverted-behavior claims to report for Step 2 — there were no commits to check them
against. No other accretion candidates (superseded plan docs, finished-work sections)
found beyond the two items above; I did not do an exhaustive read of all 57 plan files
to confirm their "proposed" file warnings are still intentional, since that volume of
reading wasn't warranted by anything the mechanical pass or the 36-hour commit log
raised.
