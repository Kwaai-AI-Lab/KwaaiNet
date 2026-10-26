# Memory reflection — 2026-10-26

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` ran clean: **0 broken reference(s), 117 warning(s)
across 53 files**, exit code 0. Every warning is inside `projects/*/plans/*.md` — proposed
file paths for work not yet built, or line-number drift in files outside the plan itself. None
of the twelve `CLAUDE.md` files or `GLOSSARY.md` are flagged. Full tool output on request; not
reproduced here since it is unchanged in kind from prior runs and the tool already labels it
"a prompt to re-read, not a failure."

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned no commits — the last commit on this branch
is `c6da5da` (2026-10-23, ~3 days ago). There is nothing in the 36-hour window for this step to
check. No semantic claim can be tied to a commit from the window, so none is reported here.

Two false claims turned up anyway while reading for Step 3, verified against the code rather
than against a recent commit (see Accretion — both are old, not new).

## Accretion — what could be removed and why

1. **`core/crates/kwaai-storage/CLAUDE.md` claims encryption that isn't there — contradicts its
   own sibling doc.** Lines 3–4 ("multi-tenant **homomorphic-encrypted** vector storage") and
   line 15 (`src/vectors.rs` → "**Homomorphic** vector search") assert encrypted vector search.
   `core/crates/kwaai-storage/src/vectors.rs::VectorStore::upload` stores vectors with
   `f32s_to_bytes(embedding)` — plain float bytes, no encryption — and the project-level
   `projects/kwaai-storage/CLAUDE.md` says so explicitly: "vectors are plaintext f32 today —
   encryption is planned, see roadmap.md" and lists `vectors.rs` as "Cosine vector search
   (plaintext)". The crate file is the wrong one. This isn't accretion so much as a standing
   error — can't tell from `git log` when it was introduced since the crate `CLAUDE.md` and the
   file it describes haven't moved together, but it should be corrected to match the project
   doc, not removed.

2. **`projects/kwaai-network/CLAUDE.md` still lists P2P relay routing for inference as "In
   progress / planned," and the paired "Fix needed" line — it's shipped.** Both lines live in
   the file as committed at `b402540` (2026-09-10, the commit that introduced `mux://`/`p2p://`
   support into `shard_cmd.rs` in the first place) and have not been touched since — `git log`
   shows exactly one commit touching this file, its creation. `rag_cmd.rs` and `shard_cmd.rs`
   resolve `p2p://`/`mux://` URLs throughout (e.g. `rag_cmd.rs:1637-1639`, `shard_cmd.rs:1123`),
   and both `projects/kwaai-compute/CLAUDE.md` and `projects/kwaai-knowledge/CLAUDE.md` give
   working example commands that route inference through it today. The "In progress / planned"
   bullet and the "Fix needed: p2p relay routing for inference requests" line should move to
   "Shipped" or be deleted.

3. **A personal, non-repo path in `projects/kwaai-compute/CLAUDE.md`.** "In progress / planned"
   lists "Dedicated inference thread with session pool, LRU eviction (plan:
   `~/.claude/plans/cached-jingling-creek.md`)". That path doesn't exist in this checkout (nor
   would it on anyone else's machine or in a fresh session) — it's a local Claude Code plan file
   on whoever wrote the line's own disk. The mechanical checker can't catch it because it's
   outside the repo. Either inline what the plan says or drop the pointer.

4. **Duplicate row in `projects/kwaai-trust/CLAUDE.md`** — "Key source files" lists
   `kwaai-cli/src/identity.rs` twice in a row (lines 56–57) with near-identical descriptions.
   Harmless but worth collapsing into one row.

Not chased further, and flagged here as genuinely unverified rather than assumed stale: the
GLOSSARY's KB census ("Sixteen exist", "Nine of sixteen KBs are under 80%" corpus hygiene, D6's
eval set size) and the per-machine infrastructure notes (metro-linux/metro-win DNS and Ollama
bind address) all describe state outside this repo (a private local config, physical machines)
that this checkout has no way to confirm or refute.

## Nothing found

Nothing else — the rest of the twelve `CLAUDE.md` files and `GLOSSARY.md` read consistently
with each other and with the code checked (glossary term count, D6 pipeline commands, the
Phase-4 relation-extraction "do not," crate/project cross-references).
