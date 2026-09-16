# Memory reflection — 2026-09-16

Checked out at `ec3dc24` (= `origin/main`). Window: `git log --since="36 hours ago"`,
7 commits, `1c0b342`..`ec3dc24` (2026-09-14 17:19 → 2026-09-15 20:51, all times local
to the commits).

## Mechanical — checker output, verbatim

```
Checking 48 memory and plan files
...
0 broken reference(s), 46 warning(s) across 48 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says
something true, then touch it or fix it.
```

0 broken references. All 46 warnings are in `projects/*/plans/*.md` (proposed-file
references, one line-anchor drift in `MacOllamaStopgap-plan.md` at `config.rs:1059`
vs. the actual `config.rs:1043`, and a stale "not yet committed" status flag on
`RAGPerformanceReport-20260712.md`). None are in a `CLAUDE.md` or in
`GLOSSARY.md`. Full output not reproduced here since it's mechanical and
unchanged in kind from what `--self-test`/CI already sees.

## Semantic — claims now false, with file, line, and the commit responsible

**metro-linux's documented peer ID is dead.** Three files hardcode
`12D3KooWCzuhpXrZXD8aezgm4JCkCZSTgj48uDywYYdTzUhF8SHs` as metro-linux's peer ID:

- `projects/kwaai-compute/CLAUDE.md:32` (table) and `:36` (example `--inference-urls`)
- `projects/kwaai-knowledge/CLAUDE.md:34` (rebuild command) and `:98`/`:106`
  (table + second example)

Commit `ec3dc24` (`release(v0.7.1)`, 2026-09-15) verifies live, on real hardware:

> metro-linux, rebuilt after the 2026-08-25 NVMe failure and running v0.7.0, was
> located through the DHT under a new peer id — the old keypair did not survive,
> and the old id now resolves to nothing.

This ID was added on 2026-08-31 (`c3359b6`) and never touched again — i.e. it's
whatever ID was in use six days after the 2026-08-25 NVMe failure that
`docs/runbooks/metro-linux-boot-recovery.md` documents. That runbook's recovery
steps were built specifically to save the identity keypair off the failing drive
(`cp -a .../identity* ... # THE PEER ID`), so the recovery looked successful at
the time the CLAUDE.md files were written. `ec3dc24` is a later, direct
contradiction: the keypair did not in fact survive, and the "new" ID it found is
not recorded anywhere in the repo (the commit message doesn't quote it either).

Net effect: every `--inference-urls`/`p2p://` example in these two files that
includes metro-linux will fail to reach it. I did not find the replacement ID
anywhere in the tree to substitute in — someone who can query the live DHT
(`kwaainet p2p peers find` per the commit's own verification) needs to supply it
before these lines are corrected.

No other semantic breaks found from this window's six other commits — in each
case (the kad peerstore revert in #216, the `dht_server` semantics fix in #217,
the QUIC default plumbing in #189, the storage homomorphic-search correction in
#169, the integrity-checker extension in #164, and the review retrospective in
#213) the commit's own diff already updated the one memory file that described
the changed behavior (`core/patches/README.md`, `projects/kwaai-storage/CLAUDE.md`
+ 3 design docs, `docs/REVIEW_INTEGRITY_RETROSPECTIVE.md` itself). Checked for
leftover references to the removed peerstore functions
(`set_peerstore_addresses`, `refresh_peerstore`, `peer_listen_addrs`,
`vouchable_listen_addrs`), to `dht_server` semantics, and to `enable_quic`
defaults across every `CLAUDE.md`/`*.md` under `projects/` and `docs/` — none
found outside the commits that already fixed them.

## Accretion — what could be removed and why

Nothing new this cycle. Two items the mechanical pass surfaces are worth a
mention but aren't ripe for removal:

- `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130` still says
  "code complete, not yet committed" for work whose target file
  (`core/crates/kwaai-rag/src`) has since changed multiple times — pre-existing,
  already flagged by the checker, not introduced in this window.
- `projects/kwaai-knowledge/plans/_base.docx` looked like a stray tracked
  `.docx` (the `#164` commit that added it flagged this itself as unconfirmed),
  but it's genuinely a build input — `build_report_docx.py:98` loads it via
  `Document("_base.docx")` as a python-docx template. Confirmed legitimate under
  the root CLAUDE.md's "no tracked `.docx`" exception; no action needed.

## Nothing found

- No renamed/deleted CLI subcommands documented as existing.
- No inverted-behavior claims beyond the peer-ID item above.
- No "do not do X" guidance made obsolete by new machinery in this window.
