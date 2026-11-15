# Memory Reflection — 2026-11-15

Scope: `scripts/check_memory_integrity.py` output, plus a semantic read of
`git log --since="36 hours ago"` (3 commits, all by Darren Warner, 2026-11-14
15:44–15:45 PST: #289 announce fix, #273 single-node-per-home lock, #272
Windows token-file ACL).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files
```

All 135 warnings are in `projects/kwaai-knowledge/plans/*` and
`projects/kwaai-trust/plans/*`/`projects/kwaai-platform/plans/*`. The large
majority are "proposed?" hits on D6 corpus files (`d6_family_tree.yaml`,
`d6_doc_schema.yaml`, etc.) — these are expected false positives: D6 is a
private corpus that `projects/kwaai-knowledge/CLAUDE.md` documents as
gitignored and never committed to this repo, so the checker can't resolve
them and never will. The rest are line-number drift on functions that moved
(`graph.rs`, `rag_cmd.rs`, `ingestion.rs`, `sequence.rs`, `config.rs`) —
harmless, caught mechanically, not re-litigated here.

One warning is a real, actionable lead and is promoted to Accretion below:
`RAGPerformanceReport-20260712.md` — "status says 'not yet committed' but
core/crates/kwaai-rag/src has changed since."

No references in any of the 12 `CLAUDE.md` files or `GLOSSARY.md` are broken.

## Semantic — claims now false, with file, line, and the commit responsible

**`projects/kwaai-compute/plans/MacOllamaStopgap-plan.md`, lines 61–64:**

> Because no shard loads, `ShardManager::shard_is_ready()` stays false and
> `KwaaiNetConfig::announce_state()` (`config.rs:1059`) already returns `0`
> rather than `2`. **No announce changes needed** — the existing "online, no
> shard" state is exactly right.

This is now false, and freshly so. Commit `c48c3b8` (today, PR #289,
"fix(announce): a whole-model node is not loading a shard") changed
`announce_shard_loading()` at `core/crates/kwaai-cli/src/config.rs:1131`
specifically because a macOS whole-model (Ollama) node — exactly the node
this plan describes — was announcing `shard_loading=true` indefinitely. The
fix adds a `!ShardManager::whole_model_is_ready()` guard, with a new doc
comment that says in plain terms: "A Mac's shard process serves a whole
model and never writes the block sentinel." That is precisely the case the
plan waved off as needing no announce change.

The companion function the plan actually cites, `announce_state()`, has
also moved (now at `config.rs:1116`, not `1059`) and its own doc comment
(`config.rs:1100-1115`) independently confirms the plan's premise was wrong
even before today: it describes "Apple-silicon node[s] serv[ing] whole
models... announced JOINING for as long as they ran" as a bug that had to
be gated on `whole_model_is_ready()` too. So the plan's "no changes needed"
line has now been contradicted twice over by the announce path it name-checks,
most recently by this 36-hour-old commit.

No other change in the window touches anything asserted in a `CLAUDE.md` or
`GLOSSARY.md` file: `--concurrent` (removed from `cli.rs` by #273) and the
Windows token-file ACL behavior (changed by #272) are not documented in any
memory file, so nothing there needed invalidating. `run_node()` is mentioned
once, in `projects/kwaai-network/CLAUDE.md:21`, only as a file-map pointer
with no behavioral claim — unaffected by #273's locking change.

## Accretion — what could be removed and why

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`**
  says "Status: code complete, not yet committed" for the redb→rusqlite WAL
  migration. It has landed: `core/crates/kwaai-rag/src/meta_store.rs` imports
  `rusqlite` throughout and carries a legacy-`.redb`-detection path
  (`meta_store.rs:79-81`) that only makes sense post-migration. This predates
  the 36-hour window (file last touched 2026-09-15), so it's accretion, not
  a fresh semantic break — the status line should be updated or the section
  closed out.

- **`projects/kwaai-compute/plans/MacOllamaStopgap-plan.md`** (approved
  2026-08-21) reads as substantially implemented, not just the announce fix
  above: `shard_cmd.rs` already gates on `cfg!(target_os = "macos")` at two
  sites (lines 349, 2632). I did not verify every item in its Verification
  section (Ollama-absent degradation message, the `shard status` mode line),
  so I'm not calling it fully done — but it's a stale "no announce changes
  needed" away from being closeable, and worth a read to confirm and archive
  rather than carrying it as an open plan.

No hardcoded peer IDs, absolute paths, or finished-and-forgotten sections
were confirmed stale beyond the two items above — I did not independently
verify whether the `12D3Koo...` peer IDs in `projects/kwaai-compute/CLAUDE.md`
and `projects/kwaai-knowledge/CLAUDE.md` still name live machines; that needs
someone with access to the actual nodes, not a code read.

## Nothing found

No broken references in any `CLAUDE.md` or `GLOSSARY.md`. No other commit in
the 36-hour window invalidates a documented default, flag, or "do not"
guidance — in particular, `projects/kwaai-knowledge/CLAUDE.md`'s "do not
enable unfiltered relation extraction for 8B models" guidance remains
accurate as written (it already documents the Phase-4 axiomatic pipeline as
the escape hatch, so it is not the stale pattern the task description warns
about).
