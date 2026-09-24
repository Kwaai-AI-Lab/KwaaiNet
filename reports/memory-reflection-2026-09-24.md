# Memory reflection — 2026-09-24

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exits 0: **0 broken reference(s), 45 warning(s) across 48 files**. All 45 warnings are in `projects/*/plans/*.md` files and are file/function references the checker itself parenthesizes as `(proposed?)` — i.e. forward references to not-yet-written code in planning docs, not stale claims about what exists. None of the twelve `CLAUDE.md` files or `GLOSSARY.md` produced a warning. Nothing actionable here.

## Semantic — claims now false, with file, line, and the commit responsible

**No commits landed in the last 36 hours.** `git log --since="36 hours ago" --stat` returned nothing; the most recent commits on `main` (`5bb2ce0`, `4578271`) are from 2026-09-22 12:2x PDT, ~38–39 hours before this run — just outside the window. Since that's the closest thing to "recent," I checked those two anyway rather than report a bare zero:

- `5bb2ce0` ("run-node supervises its shard and storage children") removed `ShardManager::stop_process`/`spawn_shard_child`, replaced PID-based signaling with a lifeline pipe, and added restart-with-backoff via a new `supervisor.rs`. No CLAUDE.md or GLOSSARY.md file describes the old stop mechanism, claims children aren't restarted, or otherwise asserts anything this commit contradicts. **Nothing invalidated.**
- `4578271` only added the "Before opening a PR" section to root `CLAUDE.md` — no claims to check.
- One gap, not a false claim: `supervisor.rs` (593 lines, now central to `run-node`) appears in no file-listing table — not `CLAUDE.md`'s project map, not `projects/kwaai-platform/CLAUDE.md`'s "Key source files," not `projects/kwaai-network/CLAUDE.md`'s. These tables were already incomplete before this commit (also missing `daemon.rs`, `node.rs`, `storage.rs`) and aren't phrased as exhaustive, so this is an omission worth an update, not a correctness bug.

**Found opportunistically, predates the window, but is a live contradiction:** `projects/kwaai-network/CLAUDE.md:33` — "Fix needed: p2p relay routing for inference requests" — and `:63` — "In progress/planned: P2P relay routing for inference (route through p2p network instead of direct TCP)". This is false today: `core/crates/kwaai-cli/src/inference_mux.rs` implements `p2p://`/`mux://` inference URLs and `resolve_inference_urls()`, and both `projects/kwaai-compute/CLAUDE.md:26-28` and `projects/kwaai-knowledge/CLAUDE.md:92-100` document the same feature as already shipped, with working peer-ID examples. `projects/kwaai-network/roadmap.md:17` and `TODO.md:8` repeat the stale "not yet done" framing too.

This isn't a regression from a recent commit — both the true and false versions were written in the *same* commit, `05d69255` (2026-09-02, "kad protocol migration #167"), which created `kwaai-network/CLAUDE.md` and `kwaai-compute/CLAUDE.md` simultaneously with contradictory status for the same feature. `1a9d2d6` (2026-09-04, "docs: say what the code actually does #188") edited `kwaai-network/CLAUDE.md` afterward and fixed other stale shipped/planned claims in it, but missed this section — it was overlooked, not reconfirmed. I'm flagging it because it's a real, currently-false claim an agent would read as authoritative, even though it falls outside this run's 36-hour mandate.

## Accretion — what could be removed and why

- `projects/kwaai-network/CLAUDE.md:31-32` — `metro-linux`/`metro-win` hardcoded to "DNS broken: resolves to `192.168.1.1` (router)". One-off environment fault on specific personal machines, not durable project knowledge, and it underpins the stale "fix needed" claim above.
- `projects/kwaai-compute/CLAUDE.md:75` — points to `~/.claude/plans/cached-jingling-creek.md`. That path doesn't exist on this machine (`ls ~/.claude/plans/` → no such directory) and, being outside the repo, can never resolve for any other session or agent that reads this file.
- `projects/kwaai-storage/CLAUDE.md:40` and `:43` — reference `tests/kwaai-storage/mock-vpk-health.py` and `tests/kwaai-storage/vpk-lan-test.sh`. Verified with `test -e`: neither exists at that path. Both scripts actually live under `tests/kwaai-network/`. (The mechanical checker doesn't parse commands inside fenced code blocks, which is why this didn't surface in Step 1.)
- `projects/kwaai-trust/CLAUDE.md:56-57` — the "Key source files" table lists `kwaai-cli/src/identity.rs` twice back to back with overlapping descriptions ("`kwaainet identity` command handler" / "`kwaainet identity` handler — DID, VC import/list/verify"). One row is leftover from an edit and should be dropped.

Weak/optional, not acted on: `core/crates/kwaai-inference/CLAUDE.md`'s "Candle broadcasting rule" restates the same rule as `projects/kwaai-compute/CLAUDE.md`'s section of the same name. Both say the same true thing and the crate file already opens by pointing back to the project doc for full context, so this reads as an intentional local quick-reference rather than harmful drift — noting it, not flagging it as removable.

## Nothing found

No broken mechanical references, and the one recent commit that touched documented CLI behavior (`5bb2ce0`) didn't falsify anything currently written in memory.
