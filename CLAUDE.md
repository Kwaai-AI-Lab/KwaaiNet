# KwaaiNet — Claude Code Instructions

## PR Triage (run at the start of every session)

Reza is responsible for merging all community pull requests. At the start of each conversation,
run `gh pr list` and surface any open PRs. If there are open PRs, call them out clearly so the
backlog doesn't grow silently.

```bash
gh pr list
```

Flag any PR that has been open longer than 7 days as overdue.

---

## Reviewing PRs

Lessons from the 2026-09-08/09 pass over 20 open PRs, where the entire review went
stale before it could be acted on.

**Don't review against a stale head.** This is the one that invalidated everything
else. Check the head SHA immediately before reviewing, and check it again before
acting on the review — posting, merging, or telling anyone a finding stands. Between
those two moments a contributor can rewrite the branch, and a fast one will. Findings
against a head that has moved are not "mostly still true"; they are unverified, and
the only honest thing to say about them is that they need re-checking.

```bash
gh pr view <n> --json headRefOid --jq .headRefOid   # before, and again after
```

The same applies to the working tree: confirm `git branch --show-current` is what you
expect before reading or editing files. Review subagents have twice left this repo's
main working tree on a PR branch, and once discarded an uncommitted edit doing it.

**Merge the churn first, then review.** Reviewing 18 PRs and *then* merging two of
them invalidated the whole pass: #191 alone deleted 9,779 lines across 66 files and
forced a rebase across the queue. Land the big structural merges, let everything
rebase onto the settled base, and review against that. Never review against a base
you are about to change.

**Anchor findings to symbols, not line numbers.** `service.rs:1653` dies on the next
rebase; `service.rs → dispatch_local, the RoutedRequest::Unary arm` survives one.
Name the function or type and quote the line; use the number only as a hint. This is
the same rot `scripts/check_memory_integrity.py` exists to catch in docs — review
comments have no such checker.

**Record the head SHA in every review.** Open with ``reviewed at `<sha>` `` so
staleness is self-evident to both sides and anyone can diff forward to see what the
fixes touched. Without it there is no way to tell a resolved finding from a moved one.

**Post incrementally.** Seventeen reviews in one batch guarantees a fast contributor
starts fixing the first PR while you are still reviewing the last.

**Verify findings before relaying them, and make the verification fail loudly.**
`grep -c ... || echo 0` cannot distinguish "not present" from "the command didn't
run" — that masked a `fatal:` twice in one session and nearly retracted a correct
finding. Drop the `|| echo 0`, check exit status, and confirm the negative case is
really negative.

**zsh eats `:c` in `$var:path`.** `git show "$ref:core/..."` silently loses the `:c`
and errors with `unknown revision`. Always brace and quote separately:

```bash
git show "${ref}":"${path}"
```

## Merging PRs

**Check scope against the merge base before merging** — confirm `git merge-base` is
`origin/main` (nothing stale) and that git and the GitHub API agree on the file count.

**`gh pr merge` refuses anything GitHub treats as a stack**, as does the plain REST
merge endpoint. Use the asynchronous endpoint and poll the returned UUID:

```bash
gh api -X PUT "repos/Kwaai-AI-Lab/KwaaiNet/pulls/<n>/merge-async" \
  -f merge_method=squash -f merge_action=direct_merge \
  -f commit_title="…(#<n>)" -f commit_message="…" -f sha=<FULL 40-CHAR SHA>
gh api "repos/Kwaai-AI-Lab/KwaaiNet/pulls/<n>/merge-async/<uuid>"
```

It requires the **full 40-character SHA**. A short SHA fails with
`"Pull request head branch was modified"`, which reads like the branch moved when it
has not — check the head before retrying.

**Do not delete a branch other PRs are stacked on.** GitHub retargets the dependent
PR to `main` on merge, which is what keeps the stack alive.

**Record findings you did not block on in the squash message**, so they live in the
history rather than only in a PR comment.

---

## Workspace

Rust workspace root: `core/`. Build the CLI binary:

```bash
cd core && cargo build -p kwaainet --release
cp core/target/release/kwaainet ~/.cargo/bin/kwaainet
codesign -s - --force ~/.cargo/bin/kwaainet  # macOS 26+ required
```

Run all tests: `cd core && cargo test`

---

## Project map

| Project | Crates | CLI files | Docs |
|---------|--------|-----------|------|
| **kwaai-trust** | kwaai-trust, kwaai-wasm | identity.rs, reputation.rs, reputation_cmd.rs | `projects/kwaai-trust/` |
| **kwaai-network** | kwaai-p2p, kwaai-p2p-daemon (control-socket protocol), kwaai-hivemind-dht, kwaai-rpc | p2p_cmd.rs, node.rs | `projects/kwaai-network/` |
| **kwaai-compute** | kwaai-inference, kwaai-compression, kwaai-distributed | shard_cmd.rs, block_rpc.rs | `projects/kwaai-compute/` |
| **kwaai-storage** | kwaai-storage | vpk.rs | `projects/kwaai-storage/` |
| **kwaai-knowledge** | kwaai-rag | rag_cmd.rs, rag_api.rs | `projects/kwaai-knowledge/` |
| **kwaai-platform** | kwaai-cli, summit-server | main.rs, config.rs, updater.rs | `projects/kwaai-platform/` |

**For domain work, see `projects/{project}/CLAUDE.md`** — each contains: scope, crate ownership, infrastructure details, build commands, current state, key files, and do-not list.

Per-crate shortcuts also exist: `core/crates/{crate}/CLAUDE.md` points back to the project.

---

## Readable renderings of docs

`.md` is the source of truth; `.docx` is for reading. When a readable copy of a
tracked document is wanted, render it to **`rendered/`, mirroring the source
path** — `docs/FOO.md` becomes `rendered/docs/FOO.docx`.

```bash
pandoc docs/FOO.md -o rendered/docs/FOO.docx --toc --toc-depth=3
```

`rendered/` is gitignored. These are build outputs, never committed and never
the thing to edit — a fix goes in the `.md` and the rendering is regenerated.

**No `.docx` is tracked.** A binary snapshot cannot be reviewed in a diff and
goes stale silently: `DreamRAG-doc.docx` had drifted to 36% of its `.md`'s word
count, missing two whole sections, and `WHITEPAPER.docx` still asserted security
properties the `.md` no longer claims. If someone wants a Word copy, regenerate
it into `rendered/`. The one thing that may legitimately be tracked is a pandoc
*reference template* — a `.docx` that is an input to a build, not an output of
one.

## Tests

`tests/{project}/` — integration and evaluation scripts per domain.
Most active: `tests/kwaai-knowledge/` — D6 eval, entity extraction experiments, family tree.
