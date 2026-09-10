# Review Integrity Retrospective

**The September 2026 PR queue: what an AI-assisted review pass got wrong, why, and the rules proposed to stop it recurring.**

*Prepared 10 September 2026 from the session of 8–9 September. Author of the review under examination: the Claude Code agent operating on behalf of the maintainer. This document is the maintainer's to edit and circulate.*

---

## Summary

Between 8 and 9 September 2026 an AI agent reviewed all twenty open pull requests in the KwaaiNet repository at high effort, raised roughly ninety findings, posted feedback on every community PR, filed one cross-cutting issue, and merged two PRs. The contributor addressed essentially all of the findings within a day. Measured by outcome, the pass worked.

Measured by process, it did not. Across the two days the agent made twelve identifiable errors of fact or method — nearly all of them in the second day's *verification* rounds rather than the first day's reviews — and the maintainer had to prompt for re-checking three separate times before the errors surfaced. The rate of error rose as the session went on. This document inventories those errors, identifies the four patterns underneath them, and proposes nine rules for the repository's `CLAUDE.md`, ranked by how many of the incidents each would have prevented.

The single most important observation: **almost every wrong claim was a negative** — "not found", "0", "gone", "not in HEAD" — produced by a probe that did not work. Positive claims were rarely wrong. The agent trusted negatives without asking whether the tool would have found the thing had it been there.

---

## 1. What happened

### 1.1 The review pass

| When | What |
| --- | --- |
| 8 Sep | `gh pr list` shows 20 open PRs: 18 from the contributor, 2 from the maintainer, oldest 8 days. |
| 8 Sep | All 18 community PRs reviewed at high effort in parallel subagents. Each review built a worktree and ran the relevant test suite where a finding required it. ~90 findings. |
| 9 Sep | Reviews posted as comments on all 17 remaining community PRs. Issue #208 filed for a pattern shared by three PRs (each bypassed `add_routing_address`, an invariant the code documents). |
| 9 Sep | #192 merged (`19e96aea`). #191 merged (`4fed65fc`) after resolving three conflicts the first merge caused. |
| 9 Sep | Contributor rebases every open PR onto the new `main` and pushes fixes for the findings — 2 to 15 commits per PR — and opens two new PRs (#209, #210) derived from the review. |
| 9 Sep | Agent verifies fixes against the new heads; reviews #209 and #210; reviews #189 again after it grew from 8 to 129 lines. |
| 9 Sep | Process notes committed to `CLAUDE.md` (`952f737e`). |
| 10 Sep | Contributor opens #212 — a default written into `config.yaml` cannot later be told from a user's choice — arriving at the #189 "installed base" finding independently. |
| 10 Sep | This retrospective. |

### 1.2 What the pass got right

Recorded for calibration, not defence. The first-day reviews were substantive: they downloaded the actual v0.6.8 release tarballs to check declared `.deb` dependencies; wrote throwaway probe tests against the real `RelayManager` to reproduce a relay-slot bug four ways; checked the vendored `libp2p-kad` patch applied cleanly to the pristine crate; and read this machine's own `~/.kwaainet/config.yaml` to prove a default flip would reach no existing install. The contributor's fixes track the findings closely, and in three cases improved on the proposed remedy. The #189 "installed base" finding was independently reached by a second reviewer, and by the contributor, who opened #212 on it. A CI check written on 10 September caught a *new* instance of a defect class the review had flagged five times.

None of that excuses what follows. It bounds it.

---

## 2. The errors

Twelve, in the order they occurred. "Class" names the failure mode; section 3 groups them.

| # | The agent said | What was true | Class |
| --- | --- | --- | --- |
| 1 | Relayed the subagent findings on #209 and #210 to the maintainer as review results | None had been checked. The maintainer had to say "critically review your findings first." The agent's own persistent memory already recorded: *verify a subagent's findings before relaying; Reza will ask.* | Relaying unverified |
| 2 | `node.rs` is "gone" at #201's head | It exists, at 679 lines (down from 2,657). The claim was inferred from one grep that returned nothing. | Strong claim from a weak negative |
| 3 | `dispatch_local` is absent from every ref, *including #209 which adds it* | The shell (zsh) consumed `:c` in `git show "$ref:core/…"` as a parameter modifier. Every result was `0` because every command failed. The agent had hit and fixed this exact trap earlier the same session, then wrote the broken form again. It nearly retracted a correct HIGH-severity finding on the strength of it. | Broken probe trusted; known trap not guarded |
| 4 | Clean zeros from the same commands | `\|\| echo 0` was masking `fatal: unknown revision`. A command that never ran looked identical to one that found nothing. | Suppressed failure |
| 5 | The maintainer's `.docx` wording is "not in HEAD" | A `sed` range pattern did not match the backticks in the actual line. It was there. | Broken probe trusted |
| 6 | #189's drift finding is "exactly the fix, plus a test in `02_unit_p2p.rs`" — **in a table labelled verified** | The change to that file *weakened* an assertion into a tautology (`!swarm_listen_addrs().is_empty()` can never fail). "Test added" was inferred from a filename appearing in the diff. | Diff inference presented as code verification |
| 7 | "4/4 flagged" for the new orphaned-doc checker | Two of the four were false positives — edited function signatures, not inserted items. Declared before reading any of them. | Summary before verification |
| 8 | Working tree restored after a subagent left it on a PR branch — three times | A guard was added only after the third. Between the first and second, the maintainer's *uncommitted* edit to `CLAUDE.md` was destroyed by an agent's checkout. | Known hazard, not mitigated |
| 9 | CLAUDE.md edit "applied cleanly" | It landed on branch `pr-209-review`. `git branch --show-current` was never run before writing. | Acted without checking state |
| 10 | #169 and #164 "waiting on you, not on review" | Fourteen live findings between them, including one the contributor had raised a week earlier and which was still unaddressed verbatim. | Dismissed without looking |
| 11 | "dazwin fixed essentially all of it" | Then found that #189's fix reaches no node currently on the network, and that #202's fix introduced a fresh instance of the defect it was correcting. | Summary ahead of the checking |
| 12 | An inline Python probe to classify two checker flags | `SyntaxError`. A broken probe, written during a task about probes. | Speed |

Not counted above, because it is already recorded in `CLAUDE.md`: the first-day pass reviewed eighteen PRs and *then* merged two, one of which deleted 9,779 lines across 66 files. Every open PR had to rebase, every review went stale, and every line-number anchor in every finding became unverifiable at once.

---

## 3. The patterns underneath

### 3.1 Negatives were trusted; positives were checked

Errors 2, 3, 4, 5 and 12 are the same failure. A probe returned "nothing" and the agent believed it. In every case the probe was broken — a quoting bug, a masked error, a pattern that did not match — and would also have returned "nothing" for a case known to be positive. Not once did the agent run its probe against a known positive before trusting the negative.

This is asymmetric for a reason. A positive result carries its own evidence: the matched line is right there. A negative carries none. It says only that *this probe*, *as written*, found nothing — which is a statement about the probe as much as the code.

### 3.2 Evidence levels collapsed into one register

"Opened the file at that SHA and read it", "read the commit message", and "the subagent said so" were all written in the same voice. The `verified` badge on the shared board was meant to mark the first of these; error 6 put a filename inference under it, and the badge lost its meaning.

Errors 1, 6 and 10 are all this. So is the general shape of the second day: commit titles that *claimed* a fix were treated as evidence the fix worked.

### 3.3 Summaries were written first

"4/4", "all resolved", "essentially all" — each was written and then checked, and each was wrong. A count should be derived from a list of individual verdicts, not the list from the count. Errors 7 and 11.

### 3.4 Caught errors did not become guards

The zsh trap bit once, was fixed, and bit again forty minutes later because the fix was to the *command*, not to the *habit*. A subagent moved the working tree; it was moved back; nothing prevented the next one doing it; it happened twice more and destroyed the maintainer's work. Errors 3 and 8.

### 3.5 Why it got worse

The maintainer's word was "increased", and it is accurate. The first day's reviews each ran ten to fifteen minutes in a dedicated worktree. The second day's verifications were the agent batch-grepping fifteen PRs in one shell command and reading the result at a glance. Volume rose, care per item fell, and the errors tracked the speed. Nothing about the task changed; the agent's pace did.

---

## 4. Proposed rules for `CLAUDE.md`

Ranked by how many of the twelve incidents each would have prevented. The first four together account for ten of the twelve.

**1. Calibrate every negative.**
Before trusting a probe that returns "not found", "0", or "none", run the *same probe* against a case known to be positive. If the known positive also comes back empty, the probe is broken, not the code. A negative that has not been calibrated is not evidence.
*Prevents incidents 2, 3, 5, 12 — and 4, once the failure is visible.*

**2. Subagents never touch the main working tree.**
Every review or exploratory subagent runs in a throwaway `git worktree`. After any subagent completes, and before any file is read or edited, check `git branch --show-current` and `git status --porcelain` against what is expected. An unexpected branch stops the work until it is understood.
*Prevents incidents 8, 9, and the destroyed edit.*

**3. Three evidence levels, always labelled, never upgraded.**
Every finding carries exactly one of:

- **`read`** — the file was opened at the recorded head SHA and the claim was checked against its contents;
- **`diff`** — the claim rests on a hunk or a commit message;
- **`relayed`** — a subagent, an agent, or a person said so.

Only `read` may be called verified. A summary, table, or board may never present a finding at a higher level than it was established at.
*Prevents incidents 1, 6, 10.*

**4. A caught error becomes a guard in the same turn.**
When a trap bites — a quoting error, a moved worktree, a masked failure — the fix is not to correct the one result but to add the check that prevents recurrence, *before* continuing with the task. Fixing the command and moving on is what allowed incident 3 to repeat.
*Prevents the recurrence in 3 and the repeats in 8.*

**5. Summaries last, derived from the list.**
No "N/N", "all", or "essentially all" until every item has an individual verdict in the same message. The count is computed from the verdicts.
*Prevents 7, 11.*

**6. Verification commands fail loudly.**
No `|| echo`, `|| true`, or `2>/dev/null` on any command whose output becomes a claim. Check the exit status. A tool that cannot distinguish "not present" from "did not run" is not a tool.
*Prevents 4.*

**7. Reproduce the load-bearing claim of every relayed finding before it is posted.**
A subagent's finding is a hypothesis. Identify the single assertion that, if wrong, collapses it, and re-run that probe in the main session. This is required even when the finding turns out to be right — incident 1's findings were correct, and the process was still wrong.
*Prevents 1 as a process failure.*

**8. Cap the pass.**
No more than about five PRs per review batch before acting on them — merging, posting, or stopping to report. Quality fell visibly with volume, and a cap also enforces the merge-then-review sequence that `CLAUDE.md` already requires.
*Addresses 3.5.*

**9. Keep a running corrections ledger in the deliverable.**
Every retracted or corrected claim is added to one visible list on the shared board or report, with what was said, what was true, and how it was caught. Scattered one-line corrections hid the trend until the maintainer noticed it; a ledger makes the error rate legible to the agent as well as to the reader.
*Would have surfaced the trend before a human had to.*

---

## 5. Two questions for the maintainer

**Where these live.** The existing `## Reviewing PRs` section of `CLAUDE.md` is about how to run a review. Rules 1–7 are about the integrity of claims and apply to any verification work, not only PR review. A separate `## Verification discipline` section, referenced from the review section, would keep the two concerns distinct.

**How many.** Nine is a lot for a file that is read at the start of every session. The first four would have prevented ten of the twelve incidents. One option is to write those four as hard rules and fold 5–9 into a shorter checklist beneath them; the other is nine rules at equal weight. This is a judgement about what the file is for.

---

## Appendix A — The two environment traps, verbatim

Both produced confident-looking negatives from commands that had not worked. Both are now in `CLAUDE.md`; they are repeated here because they are the concrete form of pattern 3.1.

**zsh consumes `:c` in `$var:path`.**

```bash
# Broken under zsh: the shell reads ":c" as a modifier and the path is mangled.
git show "$ref:core/crates/kwaai-p2p/src/service.rs"
#   fatal: ambiguous argument 'abc1234ore/crates/…'

# Correct: brace the variable and quote the two halves separately.
git show "${ref}":"core/crates/kwaai-p2p/src/service.rs"
```

**`|| echo 0` hides the failure.**

```bash
# Broken: a command that fails and a command that finds nothing both print 0.
git show "$ref:path" | grep -c 'fn dispatch_local' || echo 0

# Correct: let the failure through; a fatal: is information.
git show "${ref}":"path" | grep -c 'fn dispatch_local'
```

Applied together, these two produced incident 3: every ref, including the one that demonstrably contained the function, returned `0`, and the agent came within one message of withdrawing a correct finding.

## Appendix B — The orphaned-doc checker

One of the defect classes the review found five times — a new Rust item inserted between an existing `///` block and the item it documented, so the doc silently re-attaches to the wrong function — is invisible to the compiler, to clippy, and to rustdoc. `scripts/check_orphaned_docs.py` detects it from a diff: an *added* item declaration whose preceding doc block is *unchanged* context.

Its own development on 10 September illustrates rule 5. The first run reported "4/4 flagged". Reading the four showed two were edited signatures (`refresh_server_info` gained a parameter; `build_pinned_path` was renamed), which a diff shows as `-old`/`+new` and the checker had read as insertions. After correcting for that class it reports two, both confirmed true positives by reading the code — and one of them, in #202, is a *new* instance the contributor introduced while fixing the original. Ten of the twelve PRs it passed clean have not been individually inspected; those negatives are uncalibrated and are recorded here as such.

It is not yet wired into CI.
