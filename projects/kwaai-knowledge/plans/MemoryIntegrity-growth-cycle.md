# Can pai-seed's growth cycle keep our memory honest?

2026-08-26. Prompted by [`Kwaai-AI-Lab/pai-seed`](https://github.com/Kwaai-AI-Lab/pai-seed).

**Short answer: half of it, and not the half we most need.** The growth cycle is
the right model for *accretion* — documents that swell and are never pruned. Our
worse problem is *staleness*, and that has a different cause and needs a
different mechanism. Both are worth adopting; only one of them is pai-seed's.

---

## 1. What pai-seed proposes

A Personal AI grown for one Principal, structured as a tree: **roots** (model
providers), **trunk** (shared session/artifact infrastructure), **boughs** (life
domains), **branches** (current working context), **leaves** (insights and
actions, which fall and feed growth).

The engine is a per-session rhythm — **opening** (orient), **working**,
**closing** (reflect on what worked and what caused friction) — with the closing
reflection named as "the engine of growth" and never to be skipped. Governing it
is the **Bonsai Principle**: *"grow what you need to accomplish more of what you
are trying to do. No more."* Before adding a term or a process, ask whether its
absence has caused real friction, repeatedly.

Three artifacts emerge from use: a **glossary** ("the most important artifact"),
**processes** (written only after repeated friction), and **session notes**.
Renaming, restructuring and pruning are explicitly framed as growth, not failure.

## 2. Where the analogy holds

Our repository already has the shape.

| pai-seed | KwaaiNet |
|---|---|
| Roots | model providers, p2p inference peers, Ollama |
| Trunk | root `CLAUDE.md`, the build, `~/.claude/.../memory/` |
| Boughs | the six `projects/kwaai-*` domains |
| Branches | feature branches and active plans |
| Leaves | commits, plan documents, `tests/*/results/` |

Three of its claims land directly:

- **The glossary is the most important artifact.** We built one yesterday for
  independent reasons and immediately found it was the thing that made a
  reading order possible. pai-seed treats this as the central artifact rather
  than a nicety; that is the stronger position and we should adopt it.
- **Pruning is growth.** Our `CLAUDE.md` files only ever accrete. The D6 rebuild
  command in `projects/kwaai-knowledge/CLAUDE.md` carries three hardcoded p2p
  peer IDs; nothing has ever removed anything from that file.
- **Write a process only after repeated friction.** We have the inverse habit —
  the plans directory has 30+ documents, several never actioned.

## 3. Where it does not reach — and this is the important part

**A Personal AI's artifacts describe its Principal. A repository's `CLAUDE.md`
describes code.**

The Principal's vocabulary and priorities change only when the Principal
changes, and the Principal is in the room at the closing reflection. Code changes
without anyone opening the document. There is no session boundary at which a
refactor announces that it has invalidated a file map three directories away.

Measured across our twelve tracked `CLAUDE.md` files:

```
core/crates/kwaai-rag/CLAUDE.md          94 days of drift
projects/kwaai-platform/CLAUDE.md        94 days
core/crates/kwaai-p2p/CLAUDE.md          93 days
core/crates/kwaai-inference/CLAUDE.md    54 days
core/crates/kwaai-storage/CLAUDE.md      54 days
```

("Drift" = the code the file describes has been edited this many days more
recently than the file itself.)

And the drift is not merely age. **10 broken references across 5 files**:

- `core/crates/kwaai-p2p/CLAUDE.md` maps `src/network.rs` and `src/hivemind.rs`.
  Neither exists; the crate was refactored into `behaviour.rs`, `service.rs`,
  `dht_service.rs`.
- `projects/kwaai-storage/CLAUDE.md` cites `src/api/mod.rs` — the module was
  flattened to `api.rs` — and `src/mcp/server.rs`, gone.
- **The root `CLAUDE.md`, loaded into every single session, names `trust.rs` in
  its project map. There is no such file anywhere in the repository.**

Yesterday produced a semantic version of the same failure that no path check
would catch: `projects/kwaai-knowledge/CLAUDE.md` said *"Do not re-enable
relation extraction for 8B models — precision is too low."* That predates the
Phase-4 axiomatic pipeline built specifically to solve it. A contributor
following it would have avoided the pipeline this whole branch depends on.

**These files are read by agents as authoritative, so a stale claim is amplified
rather than noticed.** That is the asymmetry against a personal AI, where the
Principal reads their own glossary and would spot a term they no longer use.

## 4. What we should actually do

**Adopt the reflection ritual** for accretion, and **add verification** for
staleness. They address different failure modes and neither substitutes.

### 4a. Verification — built, and now in CI

`scripts/check_memory_integrity.py`, wired as the `memory` job in `ci.yml`. It
runs in seconds with no toolchain:

- every `` `path/file.rs` `` in a memory file must resolve, and resolve *at that
  path* (a file that moved is reported as moved);
- every `` `fn_name()` `` must exist in the crates;
- a doc untouched for 60+ days while its subject kept moving is a **warning**,
  not a failure — a prompt to re-read, not a gate.

Broken references fail the build. Drift does not, because the correct response
to drift is judgement, and a red build teaches people to silence it.

This is precisely what pai-seed cannot offer: a personal AI has no compiler to
check its glossary against. We do.

### 4b. Reflection — proposed, not built

The closing-reflection habit, scoped to what a repository can honestly sustain:

- **At the end of a session that changed code**, ask the one question the
  checker cannot: *does any memory file now claim something that is no longer
  true?* Yesterday's `entity_cap` change invalidated a table row three
  directories away, and only a human-or-agent reading caught it.
- **Prune in the same commit.** The D6 rebuild command's peer IDs, the plans
  never actioned. pai-seed's framing — that removal is growth — is the
  permission this repository lacks.
- **Let the glossary be the anchor.** New vocabulary goes there first, and
  `CLAUDE.md` files reference it rather than redefining terms locally.

### 4c. What not to adopt

The full tree taxonomy. Our `projects/` directories already *are* boughs and
renaming them buys nothing — which is itself the Bonsai Principle applied to
pai-seed itself. Take the growth cycle and the restraint; leave the vocabulary.

## 5. Outcome of the first run

All ten broken references fixed; the check now passes.

| file | was | now |
|---|---|---|
| root `CLAUDE.md` | `trust.rs` in the project map | `identity.rs`, `reputation.rs`, `reputation_cmd.rs` |
| `kwaai-p2p/CLAUDE.md` | `network.rs`, `dht.rs`, `hivemind.rs`, `protocol.rs` | the ten modules the crate actually has |
| `kwaai-network/CLAUDE.md` | same four, prefixed | same correction; hivemind wire format is its own crate |
| `kwaai-trust/CLAUDE.md` | `kwaainet trust` (score, issue, verify) | the command no longer exists — now `identity` (VCs) and `reputation` |
| `kwaai-storage/CLAUDE.md` | flagged `src/api/mod.rs`, `src/mcp/server.rs` | **checker bug, not a doc bug** — see below |

Two things the exercise taught, both of which changed the tool rather than the docs:

**Not every unresolved path is a broken claim.** kwaai-storage documents changes
needed in the *PHE repo*, under a heading that says so. Flagging those trains
people to ignore the check, which is worse than not having it. The checker now
drops sections whose heading names another repository.

**Drift is not error.** `projects/kwaai-platform/CLAUDE.md` carries 94 days of
drift and every claim in it — six display functions, `KwaaiNetConfig`, five
source files, two crates — verifies. It needed no change. Had drift been a
failure the correct response would have been to touch the file and learn nothing.
`kwaai-rag/CLAUDE.md`, same 94 days, was accurate but listed 8 of 34 modules and
none of the ones this branch added; it got a real update. The warning surfaces
both, and a human decides which is which. That division of labour is the point:
**the checker knows what is false, only a reader knows what is missing.**

## 6. The reflection half — implemented

`Nightly memory reflection — KwaaiNet`, routine `trig_01TuA53doMcSPCTNNNyqJEuc`,
`0 10 * * *` UTC = **03:00 America/Los_Angeles**, chosen because it is reliably
idle. A cloud session, so it works on a fresh GitHub checkout and never touches
a local machine.

Four steps, mirroring the split this document argues for:

1. **Mechanical** — run `check_memory_integrity.py`, record output verbatim.
2. **Semantic** — read `git log --since="36 hours ago" --stat` and ask the
   question the checker cannot: did any commit make a well-formed claim untrue?
   The prompt names the four shapes we actually hit — a removed CLI subcommand
   still documented, a changed default, a "do not do X" superseded by new
   machinery, an inverted behaviour — and requires the file, the line, and the
   commit responsible. Suspicions unverified against the code are not to be
   reported.
3. **Accretion** — pai-seed's contribution. Name what has outlived its use;
   removal is a legitimate outcome.
4. **Report only.** Writes `reports/memory-reflection-YYYY-MM-DD.md`, commits
   only that file to a dated branch, pushes. It may not edit a memory file,
   commit anything else, or open a PR.

Two deliberate constraints:

**Report-only, per pai-seed's "trust is earned gradually — begin cautiously with
all outputs reviewed."** A nightly agent editing memory files is the loop that
produced the stale claims in the first place. Autonomy can widen once the
reports prove accurate; it cannot be un-widened after a wrong "correction"
propagates into every future session.

**Brevity is instructed, not hoped for.** The prompt says a three-line "nothing
to review" is a good report and that padding trains the reader to skim — because
skimming is how ten broken references survived. An agent rewarded for looking
busy will manufacture findings, and this one is told not to.

36-hour lookback against a 24-hour cadence, so a missed night does not lose a day.

**Still open:** the human half of §4b — pruning in the same commit, and letting
the glossary be the anchor for new vocabulary. Those are habits, not automation,
and the nightly report is meant to prompt them rather than replace them.
