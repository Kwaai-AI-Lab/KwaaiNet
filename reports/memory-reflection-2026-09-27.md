# Memory reflection — 2026-09-27

Scope: root `CLAUDE.md` + 11 other `CLAUDE.md` files + `projects/kwaai-knowledge/GLOSSARY.md`,
against `git log --since="36 hours ago"` on `origin/main` @ `486bcf6a`.

## Mechanical — checker output, verbatim

`scripts/check_memory_integrity.py` exists and ran clean (exit 0):

```
Checking 49 memory and plan files
  [... 46 warnings, all in projects/*/plans/*.md, all either "moved?" suggestions
       or "(proposed?)" placeholders for not-yet-written files/functions ...]
0 broken reference(s), 46 warning(s) across 49 files
```

None of the 46 warnings fall in the 12 `CLAUDE.md` files or `GLOSSARY.md` — all are in
`projects/*/plans/*.md`, and all are the checker's known-benign case (a plan describing
work not yet built). Full raw output is in this session's log; not reproduced in full here
since none of it is new or actionable.

## Semantic — claims now false, with file, line, and the commit responsible

**`CLAUDE.md:226-231` ("No `.docx` is tracked") is now false, as of commits in this window.**

The rule reads:

> **No `.docx` is tracked.** ... The one thing that may legitimately be tracked is a pandoc
> *reference template* — a `.docx` that is an input to a build, not an output of one.

`git ls-files` shows two tracked files that are builds, not templates:
`projects/kwaai-knowledge/papers/aias2026/submission54-manuscript.docx` and
`submission54-4page.docx`. Both were introduced by commit `3fb0185` ("AIAS+ 2026 Dream RAG
manuscript, response letter and figures") and touched again by five more commits in this
36-hour window (`7f976b0`, `0ef036f`, `c42dd5e`, `51b35b4`, `486bcf6`), the last of which —
"submission .docx opened in Word with 'unreadable content'" — is exactly the kind of binary-drift
failure mode the rule was written to prevent.

This looks deliberate, not accidental: `build_manuscript_docx.py`'s own docstring says
`Output: submission54-manuscript.docx ... tracked so the submission is in the repo`, because
EasyChair needs an actual `.docx` upload, not a `.md`. But `CLAUDE.md` carves out no such
exception, so an agent trusting the root file at face value would flag or delete these, or
refuse to track the next conference submission's output the same way. Recommend adding a
one-line exception at `CLAUDE.md:230-231` for submission-platform output files, next to the
existing reference-template exception. (`_easychair_template.docx` and
`projects/kwaai-knowledge/plans/_base.docx` are unaffected — both are genuine input templates
already covered by the existing exception.)

No other semantic drift found: every commit in the window touches only
`projects/kwaai-knowledge/papers/aias2026/` and `projects/kwaai-knowledge/plans/`
(paper text, figures, the build script, the built .docx files). No CLI subcommand, default,
threshold, cap, or flag changed anywhere in `core/`, so nothing to check there this cycle.

## Accretion — what could be removed and why

Nothing found. The plan most worth a second look —
`projects/kwaai-knowledge/plans/DreamRAG-AIAS2026-manuscript-plan.md` — is still live (deadline
2026-10-01, not yet reached). The checker's "(proposed?)" warnings in
`MemoryIntegrity-growth-cycle.md` (`src/hivemind.rs`, `trust.rs`, `fn_name()`,
`reports/memory-reflection-YYYY-MM-DD.md`, etc.) are that plan's own illustrative placeholders
for this reflection process, not decayed real references — no action.

## Nothing found

- No broken references (checker: 0).
- No stale/renamed CLI surface, changed defaults, or obsoleted "do not do X" guidance this cycle —
  the only commits in the window are outside `core/`.
- No accretion candidates ready to remove this cycle.
