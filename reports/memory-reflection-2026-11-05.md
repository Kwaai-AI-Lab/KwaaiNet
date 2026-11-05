# Memory reflection — 2026-11-05

Scope: 12 `CLAUDE.md` files + `projects/kwaai-knowledge/GLOSSARY.md`, against `git log --since="36 hours ago"` on `main` (HEAD `e98ca37`, 6 commits: #274, release v0.7.3, #265, #249, #264, #258).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

0 broken references. All 135 warnings are in `projects/*/plans/*.md`, not in any of the 12 `CLAUDE.md` files or `GLOSSARY.md` — none of those root/project memory files triggered a checker warning. Most plan-file warnings are either (a) line-number drift in function references, or (b) references to the private, gitignored D6 corpus files (`d6_family_tree.yaml`, `d6_doc_schema.yaml`, etc.) that the checker can't resolve from a public clone but that `projects/kwaai-knowledge/CLAUDE.md` already documents as intentionally absent.

## Semantic — claims now false, with file, line, and the commit responsible

Checked each of the 6 commits in the window against all 12 `CLAUDE.md` files and the glossary by grepping for the specific subcommands, defaults, and test/behavior names each commit touched (bearer-token/gRPC auth from the v0.7.3 release and #254, the DACL/SID comparison in #274, `ChunkDropStats`/chunk-loss accounting in #265, the CUDA archive renaming in #258, sccache scoping in #264, and the ontology/eval terms #249's own commit message flagged as questionable).

**Nothing found.** None of the 12 memory files or the glossary mention bearer tokens, gRPC auth, the Windows DACL/SID test, chunk-loss counting, the CUDA archive naming (`-cuda.tar.xz` / `-cuda-lean.tar.xz`), or sccache cache scoping — so none of them assert anything these commits invalidated. #249 itself already corrected the one live claim it touched (`projects/kwaai-knowledge/roadmap.md`, Q3 metric) in the same commit, and the "curated ontologies" overstatement it flagged lives only in the new research doc it added, not in an existing memory file.

I did not find a case of the "do not do X, now made safe" pattern in this window. `projects/kwaai-knowledge/CLAUDE.md`'s "Do not enable *unfiltered* relation extraction for 8B models" (line 150) looked like a candidate at first grep, but it already names the Phase-4 axiomatic pipeline as the sanctioned alternative and scopes the prohibition to the *unfiltered* path specifically — it is current, not stale, and no commit in this window touched that code.

## Accretion — what could be removed or fixed

- **`projects/kwaai-knowledge/plans/RAGPerformanceReport-20260712.md:130`** — "Status: code complete, not yet committed" for the redb→rusqlite WAL migration. Confirmed stale: `core/crates/kwaai-rag/Cargo.toml` has carried `rusqlite = "0.31"` for a long time, and it's used live in `meta_store.rs`, `graph.rs`, `cache.rs`, `feedback.rs`. This report predates v0.4.148's successor by many releases (main is at v0.7.3), so the line should either be updated to say when it landed or the report retired/archived — it currently reads as an open item that finished months ago. (The checker flags this same file with the same concern; I verified it's a real positive, not just drift.)
- **Hardcoded peer-ID tables duplicated verbatim** in `projects/kwaai-knowledge/CLAUDE.md` and `projects/kwaai-compute/CLAUDE.md` (metro-linux/metro-win/jerome, same three `12D3Koo...` IDs). Any peer rotation or machine swap needs two synchronized edits with no checker coverage of the duplication; worth consolidating to one source of truth (e.g. the network project doc) with the other two referencing it.
- **`projects/kwaai-compute/CLAUDE.md`** ("In progress / planned") points to `~/.claude/plans/cached-jingling-creek.md` — a path under the user's local Claude settings directory, not the repo. It isn't resolvable from a fresh clone or by another contributor/agent, and the checker can't see it either since it's outside the tree it scans.

## Nothing found

No renamed/deleted CLI subcommands, inverted behaviors, or changed defaults/thresholds among the 6 commits in this window are documented (correctly or incorrectly) in any of the 12 `CLAUDE.md` files or the glossary — so there is nothing to correct on that front this cycle.
