# Memory reflection — 2026-11-20

Scope: `HEAD` at `460c69b6f71763644734c92251b961806f69d720` (= `origin/main` as of this run).

## Mechanical — checker output, verbatim

```
Checking 57 memory and plan files
...
0 broken reference(s), 135 warning(s) across 57 files

Drift is a prompt to re-read, not a failure. Reflect on whether the doc still says something true, then touch it or fix it.
```

All 135 warnings are pre-existing: line-number drift in `graph.rs`/`ingestion.rs`/etc. function
references, and "proposed?" file references in `kwaai-knowledge` plan docs (e.g. `d6_family_tree.yaml`,
`ontology.yaml`) that were never meant to resolve yet. Spot-checked several against the current tree —
no new breakage, nothing that looks like a file that used to exist and was deleted. 0 broken references.

## Semantic — claims now false, with file, line, and the commit responsible

`git log --since="36 hours ago" --stat` returns exactly one commit:

- `460c69b` — "fix(build): kwaai-rpc's protoc unzip, the second half of #306 (#307)" — touches only
  `core/crates/kwaai-p2p-daemon/build.rs` and `core/crates/kwaai-rpc/build.rs`. It extends the
  `Expand-Archive` → `ZipFile` workaround (from the prior commit, `9df1e15`, #306) to `kwaai-rpc`'s
  build script, adds a clean-slate `remove_dir_all` before re-extracting protoc, and checks that the
  extracted binary actually exists before declaring success.

Checked every memory file for a reference to protoc extraction, `build.rs`, `Expand-Archive`, or
`kwaai-rpc`'s build process: there is none. `CLAUDE.md`, `projects/kwaai-network/CLAUDE.md`, and
`projects/kwaai-compute/CLAUDE.md` mention `kwaai-rpc` only as a crate name / RPC-protocol owner, never
its build mechanics. Nothing to invalidate.

Nothing found — no memory-file claim was made untrue by this commit.

## Accretion — what could be removed and why

Not assessed in depth this run. Only one commit landed in the window and it is unrelated to any
documented claim, so there is no fresh accretion signal to chase here. The checker's standing list of
"proposed?" file references in `kwaai-knowledge/plans/*.md` is long-standing noise (ideas documents
describing files not yet written) rather something this run's window changed — worth a dedicated pass,
but outside a 36-hour diff review.

## Nothing found

Checker passed clean (0 broken references). Exactly one commit landed in the last 36 hours, a narrow
build-script fix with no corresponding claim in any CLAUDE.md or the glossary. Nothing to report.
