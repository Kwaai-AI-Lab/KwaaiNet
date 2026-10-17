# Memory Reflection — 2026-10-17

Scope: root `CLAUDE.md`, the 11 other `CLAUDE.md` files (5 `core/crates/*`, 6 `projects/*`),
and `projects/kwaai-knowledge/GLOSSARY.md`. Checked out `main` at `913e0bc4` (fast-forwarded
from `d46bb4aa` at session start).

## Mechanical — checker output, verbatim

`python3 scripts/check_memory_integrity.py` exists and ran clean:

```
Checking 51 memory and plan files
...
0 broken reference(s), 101 warning(s) across 51 files
```

All 101 warnings land in `projects/*/plans/*.md` (forward-looking plan documents, mostly
references to not-yet-created files marked "(proposed?)", plus a handful of
function/line drift in plan docs). None of the 101 warnings touch any of the 12
`CLAUDE.md` files or `GLOSSARY.md` — the checker found those clean. Full warning list is
in the raw run output (not reproduced here to keep this report short); none of it bears
on this run's semantic/accretion review below, which is scoped to the `CLAUDE.md` files
and the glossary per this task's brief.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returned **no commits**. The last commit on `main`
is `913e0bc4` (2026-10-14), which is outside the window. Nothing to check here — there is
no new commit that could have invalidated a memory-file claim since the last run.

## Accretion — what could be removed and why

**Stale line reference to the RoPE broadcast fix, in two files.** Both
`core/crates/kwaai-inference/CLAUDE.md:20` and `projects/kwaai-compute/CLAUDE.md:88` say
the fix is "`shard.rs:104`: `x1.broadcast_mul(&cos4)` not `x1 * cos4`". Line 104 in
`core/crates/kwaai-inference/src/shard.rs` is now `let sin4 = sin` — unrelated code. The
actual `x1.broadcast_mul(&cos4)` call is at line 108–109. `git log` on that file shows its
last change was commit `13515c23` (2026-09-10, PR #193), over a month ago, so this drifted
well before this run's 36-hour window — it's pre-existing rot, not something last night's
commits caused. The checker doesn't catch it because the warning format it looks for is
"function X is defined at line Y," and this is an inline code-snippet-to-line-number claim,
not a function reference. Worth fixing in both files (point at the function/snippet rather
than a bare line number, as the root `CLAUDE.md`'s own "anchor findings to symbols, not
line numbers" rule for PR reviews already argues for), or dropping the line number
entirely.

**A user-local plan path in `projects/kwaai-compute/CLAUDE.md:75`.** "(plan:
`~/.claude/plans/cached-jingling-creek.md`)" points outside the repo, into a path that
doesn't exist in this container (confirmed: no `~/.claude/plans/` directory here) and
can't exist for any other contributor or session — it was presumably a local plan-mode
artifact on whoever wrote this line's machine. It resolves for nobody who reads this file
cold. Worth replacing with an in-repo plan doc reference or removing if the plan it points
to no longer matters.

I did not find evidence of other accretion candidates (superseded plan documents, finished
work still described as in-progress, other hardcoded paths) in the 12 `CLAUDE.md` files or
the glossary this pass. The `projects/kwaai-network/CLAUDE.md` and
`projects/kwaai-compute/CLAUDE.md` peer-ID tables (metro-linux/metro-win/jerome) and the
"DNS broken" / "GPU swapped 2026-08" notes are infrastructure facts I have no way to verify
from the repo alone (no network access to the machines in question) — flagging that they
exist and are the kind of claim that goes stale silently, but I can't confirm or deny them
from here.

## Nothing found

No broken mechanical references in the 12 `CLAUDE.md` files or `GLOSSARY.md`. No commits
in the last 36 hours, so no semantic drift to check against new code.
