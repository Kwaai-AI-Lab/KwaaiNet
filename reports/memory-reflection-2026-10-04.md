# Memory reflection — 2026-10-04

Reviewed at commit `97d0b1e235f4d2bd9dbf8645013e288e3d026ad9` (origin/main).

## Mechanical — checker output, verbatim

```
Checking 51 memory and plan files
...
0 broken reference(s), 65 warning(s) across 51 files
```

All 65 warnings are in `projects/*/plans/*.md` (proposed-but-unwritten files/functions,
and a handful of moved paths the checker already resolves with a "moved?" suggestion).
None of the 12 `CLAUDE.md` files or `GLOSSARY.md` produced a warning or a broken
reference. Full per-file list available by re-running
`python3 scripts/check_memory_integrity.py`; omitted here since Accretion below
covers the two items worth a human look.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returns nothing: the most recent commit
(`97d0b1e`, 2026-10-01 23:31 UTC) is ~58 hours before this run. **No commits landed
in the review window**, so there is nothing a recent change could have invalidated.

Reading the memory files directly (not commit-triggered, but verified against code
and history) turned up two claims that are false right now:

**1. `core/crates/kwaai-storage/CLAUDE.md:3` and `:15`, and
`projects/kwaai-knowledge/GLOSSARY.md:37`** still assert VPK vector storage is
homomorphically encrypted —

- `core/crates/kwaai-storage/CLAUDE.md:3`: "multi-tenant homomorphic-encrypted vector storage"
- `core/crates/kwaai-storage/CLAUDE.md:15` (key-files table): "Homomorphic vector search"
- `GLOSSARY.md:37`: "**VPK** — the encrypted multi-tenant storage fabric Eve nodes run."

This was true-sounding prose that commit `1c0b342` (`docs(storage): stop claiming
homomorphic search ships (#169)`, 2026-09-14) deliberately corrected — but only in
four files: `projects/kwaai-storage/CLAUDE.md`, `design/data-flows.md`,
`design/overview.md`, `roadmap.md`. It verified against `kwaai-storage/src/vectors.rs`
(`Hnsw<'static, f32, DistCosine>` over plain `Vec<f32>` — confirmed again just now by
reading `vectors.rs`) that the host stores opaque plaintext and ranks with cosine
similarity, no encryption. The crate-level `CLAUDE.md` and the glossary entry say the
opposite and were never touched by that fix — they still read as if #169 never
happened.

Knock-on effect: `projects/kwaai-storage/CLAUDE.md`'s own "Do not" section (line 90)
— "Do not store unencrypted vectors for `eve`/`bob` modes" — was also left standing
by `1c0b342`, so the same file now admits ("Shipped: ... plaintext f32 vectors
today", line 57) and forbids the same fact two sections apart.

**2. `projects/kwaai-network/CLAUDE.md:62-65`** lists under "In progress / planned":

> P2P relay routing for inference (route through p2p network instead of direct TCP)

This shipped. `ollama_proxy.rs` (added `96235a3`, 2026-05-12, "libp2p Ollama proxy —
fan-out entity extraction over P2P fabric") and `inference_mux.rs` (added `e637fc0`,
2026-05-28, "concurrent GPU inference multiplexer over p2p stream") implement exactly
this, and `projects/kwaai-compute/CLAUDE.md:22-28` and
`projects/kwaai-knowledge/CLAUDE.md:88-94` both already document the `p2p://PEER_ID`
/ `mux://PEER_ID` scheme as shipped, preferred infrastructure, with live peer IDs and
working example commands. Three memory files describe the same feature and two call
it done.

`projects/kwaai-network/CLAUDE.md` was last edited by `1a9d2d6` (`docs: say what the
code actually does (#188)`, 2026-09-04) — an audit built for exactly this purpose,
four months after the feature shipped. Its own commit message says every change was
"verified against the code, not against neighbouring prose," but the diff only
touched the `p2p status`→`p2p info` rename and a DHT field drop; the stale
"Fix needed: p2p relay routing for inference requests" line at `:33` and the planned-
work list at `:62-65` were in scope and untouched.

## Accretion — what could be removed and why

- **`projects/kwaai-compute/CLAUDE.md:75`** points to
  `~/.claude/plans/cached-jingling-creek.md` as the plan for the dedicated-inference-
  thread work. That path is a local, unversioned Claude Code session artifact —
  it does not exist in this checkout (`ls` confirms) and can't exist for any other
  contributor's machine. It's a dangling reference by construction; if the plan's
  content still matters, move it into `projects/kwaai-compute/plans/` under the
  repo's normal convention, otherwise drop the pointer.

- **`projects/kwaai-trust/CLAUDE.md:56-57`** lists `kwaai-cli/src/identity.rs` twice
  in the "Key source files" table, with two different one-line descriptions
  ("command handler" vs. "handler — DID, VC import/list/verify"). Harmless but
  worth collapsing into one row.

- **Root `CLAUDE.md`'s "No `.docx` is tracked" rule** (lines 226-232) states the only
  legitimate exception is a pandoc reference template. Two files don't fit that:
  `projects/kwaai-knowledge/papers/aias2026/submission54-4page.docx` and
  `submission54-manuscript.docx` are tracked, and they're build *outputs* of
  `build_manuscript_docx.py` (an EasyChair conference submission requirement), not
  inputs. They look like intentional, legitimate tracked files — EasyChair needs the
  binary — but the rule as written doesn't name this second exception, so a future
  reader hits an apparent contradiction. Worth either stating the exception
  explicitly or confirming these shouldn't be tracked either.

## Nothing found

- No broken or drifted references in any of the 12 `CLAUDE.md` files or
  `GLOSSARY.md` — the mechanical checker's 65 warnings are entirely confined to
  `projects/*/plans/*.md` forward-looking/proposed references, which is what that
  checker expects of a plan document.
- No commits in the 36-hour review window, so no fresh commit-caused staleness to
  report — the two Semantic findings above predate the window and were caught only
  by direct reading, as Step 3 permits.
