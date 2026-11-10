# Memory Reflection — 2026-11-10

Scope: default branch (`main` @ `e98ca3733da35f6e5dea1abb70fd45f735d2fd67`).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files

0 broken reference(s), 135 warning(s) across 57 files
```

All 135 warnings are in `projects/*/plans/*.md` (forward-looking plan documents), not in
the twelve `CLAUDE.md` files or `GLOSSARY.md`. Most are `(proposed?)` file/function
references for work not yet started — expected for plans. Two are not proposals and are
carried into Accretion below since they're existing drift, not something this commit
window caused:

- `projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md`: "status says 'not
  yet committed' but `core/crates/kwaai-rag/src` has changed since — recheck whether it
  has landed"
- Several `graph.rs`/`ingestion.rs`/`rag_cmd.rs` function-location warnings (e.g. in
  `coref-pronoun-resolution.md`, `Phase4-EntityRelations-plan.md`, `Phase6-...-plan.md`)
  — line numbers drifted from refactors, not broken, no action needed.

## Semantic — claims now false, with file, line, and the commit responsible

**Nothing found.** `git log --since="36 hours ago" --stat` returns no commits — the last
commit on `main` (`e98ca37`, `test(rpc): compare the token file's ACE by SID, not by SDDL
string`, #274) landed 2026-11-04, six days before this run. There is no recent code
change for a memory file to have fallen out of sync with.

## Accretion — what could be removed and why

1. **Hardcoded peer IDs duplicated across two files.** The `metro-linux` / `metro-win` /
   `jerome` peer-ID table appears verbatim in both `projects/kwaai-compute/CLAUDE.md`
   (lines 32–36) and `projects/kwaai-knowledge/CLAUDE.md` (lines 103–111). Two copies of
   infrastructure identifiers drift independently — if a peer ID changes, one copy is
   easy to miss. Consider keeping one authoritative copy and having the other reference
   it.

2. **`RAGPerformanceReport-20260712.md` status line is stale.** Line 130 says "code
   complete, not yet committed," but `core/crates/kwaai-rag/src` has had at least three
   commits since the plan's last edit (Sept 14): #251 (2026-10-28), #259 (2026-10-29),
   #265 (2026-11-04). I did not trace which of those commits correspond to the work this
   plan describes, so I can't say whether the plan is now fully superseded or only partly
   — that needs someone who knows the RAG roadmap to check the plan's content against
   those three PRs, not just the status line.

3. **DNS-broken note for `metro-linux`/`metro-win`** (`projects/kwaai-network/CLAUDE.md`,
   lines 31–32) states both machines currently resolve to the router (192.168.1.1). I
   have no way to verify live network state of physical machines from this session, so
   I'm flagging it as unverified rather than stale — worth a human check next time either
   machine is touched.

Not flagging for removal: the four `**In progress / planned**` sections (kwaai-compute,
kwaai-network, kwaai-storage, kwaai-trust) and the 57-file `plans/` directory's volume in
general. Confirming any specific planned item has landed, or any specific plan is fully
superseded, requires tracing it against merged PRs one by one — out of proportion for a
nightly pass with no commits to anchor it to. Worth a dedicated pass, not a nightly one.

## Nothing found

No commits in the last 36 hours, so Semantic is empty as noted above. The mechanical
checker found zero broken references.
